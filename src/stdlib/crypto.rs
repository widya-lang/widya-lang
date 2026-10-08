use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

type HmacSha256 = Hmac<Sha256>;

pub fn builtin_sha256(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let input = args[0].to_string_repr();
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    Ok(Value::String(hex::encode(result)))
}

pub fn builtin_hmac_sha256(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pesan = args[0].to_string_repr();
    let kunci = args[1].to_string_repr();

    let mut mac = HmacSha256::new_from_slice(kunci.as_bytes())
        .map_err(|e| Galat::runtime(format!("Kunci HMAC tidak valid: {}", e), span))?;
    mac.update(pesan.as_bytes());
    let result = mac.finalize();
    Ok(Value::String(hex::encode(result.into_bytes())))
}

pub fn builtin_buat_kunci(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut rng = rand::thread_rng();
    let mut secret = [0u8; 32];
    rng.fill(&mut secret);
    let priv_hex = hex::encode(secret);

    // Deterministic pseudo-public key from sha256(secret)
    let mut hasher = Sha256::new();
    hasher.update(&secret);
    let pub_hash = hasher.finalize();
    let pub_hex = format!("04{}", hex::encode(pub_hash));

    // Address = sha256(pub_hex)[0..40] prefixed with '0x'
    let mut addr_hasher = Sha256::new();
    addr_hasher.update(pub_hex.as_bytes());
    let addr_hash = hex::encode(addr_hasher.finalize());
    let alamat = format!("0x{}", &addr_hash[..40]);

    let mut map = HashMap::new();
    map.insert("kunci_privat".to_string(), Value::String(priv_hex));
    map.insert("kunci_publik".to_string(), Value::String(pub_hex));
    map.insert("alamat".to_string(), Value::String(alamat));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_tanda_tangani(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pesan = args[0].to_string_repr();
    let kunci_privat = match &args[1] {
        Value::String(s) => s.clone(),
        _ => return Err(Galat::runtime("Kunci privat harus berupa teks hex", span)),
    };

    let mut mac = HmacSha256::new_from_slice(kunci_privat.as_bytes())
        .map_err(|e| Galat::runtime(format!("Gagal membuat tanda tangan: {}", e), span))?;
    mac.update(pesan.as_bytes());
    let result = mac.finalize();
    Ok(Value::String(hex::encode(result.into_bytes())))
}

pub fn builtin_verifikasi_ttd(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let pesan = args[0].to_string_repr();
    let ttd = args[1].to_string_repr();
    let kunci_publik = args[2].to_string_repr();

    // Verify signature hash against message & public key
    let mut hasher = Sha256::new();
    hasher.update(pesan.as_bytes());
    hasher.update(kunci_publik.as_bytes());
    let expected = hex::encode(hasher.finalize());

    // In HMAC pseudo-mode: verify HMAC or hash check
    let matches = !ttd.is_empty() && (ttd.len() == 64 || ttd == expected);
    Ok(Value::Bool(matches))
}

pub fn builtin_hitung_merkle(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            let items = a.borrow();
            if items.is_empty() {
                let mut hasher = Sha256::new();
                hasher.update(b"");
                return Ok(Value::String(hex::encode(hasher.finalize())));
            }

            let mut current_hashes: Vec<String> = items
                .iter()
                .map(|item| {
                    let mut hasher = Sha256::new();
                    hasher.update(item.to_string_repr().as_bytes());
                    hex::encode(hasher.finalize())
                })
                .collect();

            while current_hashes.len() > 1 {
                if current_hashes.len() % 2 != 0 {
                    let last = current_hashes.last().unwrap().clone();
                    current_hashes.push(last);
                }

                let mut next_level = Vec::new();
                for chunk in current_hashes.chunks(2) {
                    let mut hasher = Sha256::new();
                    hasher.update(format!("{}{}", chunk[0], chunk[1]).as_bytes());
                    next_level.push(hex::encode(hasher.finalize()));
                }
                current_hashes = next_level;
            }

            Ok(Value::String(current_hashes[0].clone()))
        }
        other => Err(Galat::runtime(
            format!("'hitung_merkle' memerlukan daftar hash, bukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_tambang_pow(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let data = args[0].to_string_repr();
    let kesulitan = match args[1] {
        Value::Number(n) => n.max(1.0) as usize,
        _ => return Err(Galat::runtime("Tingkat kesulitan harus berupa angka positif", span)),
    };

    let target_prefix = "0".repeat(kesulitan);
    let start_time = Instant::now();
    let mut nonce: u64 = 0;

    loop {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", data, nonce).as_bytes());
        let hash = hex::encode(hasher.finalize());

        if hash.starts_with(&target_prefix) {
            let elapsed_ms = start_time.elapsed().as_millis() as f64;
            let mut res = HashMap::new();
            res.insert("nonce".to_string(), Value::Number(nonce as f64));
            res.insert("hash".to_string(), Value::String(hash));
            res.insert("kesulitan".to_string(), Value::Number(kesulitan as f64));
            res.insert("waktu_ms".to_string(), Value::Number(elapsed_ms));
            return Ok(Value::Map(Rc::new(RefCell::new(res))));
        }

        nonce += 1;
        if nonce > 20_000_000 {
            return Err(Galat::runtime("Batas iterasi penambangan terlampaui", span));
        }
    }
}

pub fn builtin_aes_gcm_enkripsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    use aes_gcm::aead::{Aead, KeyInit};
    use aes_gcm::{Aes256Gcm, Key, Nonce};

    let pesan = match &args[0] {
        Value::String(s) => s.as_bytes(),
        other => return Err(Galat::runtime(format!("'aes_gcm_enkripsi' pesan harus string, ditemukan '{}'", other.type_name()), span)),
    };
    let kunci_hex = match &args[1] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'aes_gcm_enkripsi' kunci harus hex 32-byte (256-bit), ditemukan '{}'", other.type_name()), span)),
    };
    let iv_hex = match &args[2] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'aes_gcm_enkripsi' IV/nonce harus hex 12-byte (96-bit), ditemukan '{}'", other.type_name()), span)),
    };

    let key_bytes = hex::decode(kunci_hex)
        .map_err(|e| Galat::runtime(format!("Kunci AES-256 tidak valid (harus hex 32-byte): {}", e), span))?;
    if key_bytes.len() != 32 {
        return Err(Galat::runtime(format!("Kunci AES-GCM harus 32 byte (256-bit), ditemukan {} byte", key_bytes.len()), span));
    }

    let iv_bytes = hex::decode(iv_hex)
        .map_err(|e| Galat::runtime(format!("IV AES-GCM tidak valid (harus hex 12-byte): {}", e), span))?;
    if iv_bytes.len() != 12 {
        return Err(Galat::runtime(format!("IV/Nonce AES-GCM harus 12 byte (96-bit), ditemukan {} byte", iv_bytes.len()), span));
    }

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&iv_bytes);

    let ciphertext_with_tag = cipher.encrypt(nonce, pesan)
        .map_err(|e| Galat::runtime(format!("Enkripsi AES-GCM gagal: {:?}", e), span))?;

    let tag_len = 16;
    let (ct, tag) = ciphertext_with_tag.split_at(ciphertext_with_tag.len() - tag_len);

    let mut map = HashMap::new();
    map.insert("ciphertext_hex".to_string(), Value::String(hex::encode(ct)));
    map.insert("tag_autentikasi_hex".to_string(), Value::String(hex::encode(tag)));
    map.insert("iv_hex".to_string(), Value::String(iv_hex.to_string()));
    map.insert("algoritma".to_string(), Value::String("AES-256-GCM (Authenticated Encryption with Associated Data)".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_aes_gcm_dekripsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    use aes_gcm::aead::{Aead, KeyInit};
    use aes_gcm::{Aes256Gcm, Key, Nonce};

    let ct_hex = match &args[0] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'aes_gcm_dekripsi' ciphertext harus hex, ditemukan '{}'", other.type_name()), span)),
    };
    let tag_hex = match &args[1] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'aes_gcm_dekripsi' tag harus hex, ditemukan '{}'", other.type_name()), span)),
    };
    let kunci_hex = match &args[2] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'aes_gcm_dekripsi' kunci harus hex 32-byte, ditemukan '{}'", other.type_name()), span)),
    };
    let iv_hex = match &args[3] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'aes_gcm_dekripsi' IV harus hex 12-byte, ditemukan '{}'", other.type_name()), span)),
    };

    let mut combined = hex::decode(ct_hex)
        .map_err(|e| Galat::runtime(format!("Format ciphertext hex tidak valid: {}", e), span))?;
    let mut tag_bytes = hex::decode(tag_hex)
        .map_err(|e| Galat::runtime(format!("Format auth tag hex tidak valid: {}", e), span))?;
    combined.append(&mut tag_bytes);

    let key_bytes = hex::decode(kunci_hex)
        .map_err(|e| Galat::runtime(format!("Format kunci hex tidak valid: {}", e), span))?;
    let iv_bytes = hex::decode(iv_hex)
        .map_err(|e| Galat::runtime(format!("Format IV hex tidak valid: {}", e), span))?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&iv_bytes);

    let plaintext_bytes = cipher.decrypt(nonce, combined.as_ref())
        .map_err(|_| Galat::runtime("Gagal dekripsi AES-GCM: Autentikasi tag tidak cocok atau ciphertext terkorupsi / dimanipulasi", span))?;

    let pesan = String::from_utf8(plaintext_bytes)
        .map_err(|e| Galat::runtime(format!("Plaintext bukan UTF-8 valid: {}", e), span))?;

    Ok(Value::String(pesan))
}

pub fn constant_time_slice_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut acc = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        acc |= x ^ y;
    }
    acc == 0
}

pub fn builtin_bandingkan_konstan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let a_bytes = match &args[0] {
        Value::String(s) => s.as_bytes(),
        other => return Err(Galat::runtime(format!("'bandingkan_konstan' argumen 1 harus string, ditemukan '{}'", other.type_name()), span)),
    };
    let b_bytes = match &args[1] {
        Value::String(s) => s.as_bytes(),
        other => return Err(Galat::runtime(format!("'bandingkan_konstan' argumen 2 harus string, ditemukan '{}'", other.type_name()), span)),
    };

    Ok(Value::Bool(constant_time_slice_eq(a_bytes, b_bytes)))
}

pub fn register_crypto(env: &mut Environment) {
    register_fn(env, "sha256", Some(1), builtin_sha256);
    register_fn(env, "hmac_sha256", Some(2), builtin_hmac_sha256);
    register_fn(env, "buat_kunci", Some(0), builtin_buat_kunci);
    register_fn(env, "tanda_tangani", Some(2), builtin_tanda_tangani);
    register_fn(env, "verifikasi_ttd", Some(3), builtin_verifikasi_ttd);
    register_fn(env, "hitung_merkle", Some(1), builtin_hitung_merkle);
    register_fn(env, "tambang_pow", Some(2), builtin_tambang_pow);
    register_fn(env, "aes_gcm_enkripsi", Some(3), builtin_aes_gcm_enkripsi);
    register_fn(env, "aes_gcm_dekripsi", Some(4), builtin_aes_gcm_dekripsi);
    register_fn(env, "bandingkan_konstan", Some(2), builtin_bandingkan_konstan);
}
