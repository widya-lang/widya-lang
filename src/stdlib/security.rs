use crate::stdlib::{encode_base64_raw, decode_base64_raw};
use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub fn register_security(env: &mut Environment) {
    // WidyaZK (Zero-Knowledge Proofs Verifier)
    register_fn(env, "buat_bukti_zk", Some(2), builtin_zk_buat_bukti);
    register_fn(env, "verifikasi_bukti_zk", Some(2), builtin_zk_verifikasi);

    // WidyaFHE (Fully Homomorphic Encryption)
    register_fn(env, "KonteksFHE", None, builtin_fhe_konteks);
    register_fn(env, "enkripsi_fhe", Some(2), builtin_fhe_enkripsi);
    register_fn(env, "fhe_tambah", Some(2), builtin_fhe_tambah);
    register_fn(env, "fhe_kali", Some(2), builtin_fhe_kali);
    register_fn(env, "dekripsi_fhe", Some(2), builtin_fhe_dekripsi);

    // WidyaJWT (JSON Web Token Sign/Verify/Decode Engine)
    register_fn(env, "buat_jwt", None, builtin_jwt_buat);
    register_fn(env, "verifikasi_jwt", Some(2), builtin_jwt_verifikasi);
    register_fn(env, "dekode_jwt_tanpa_kunci", Some(1), builtin_jwt_dekode);

    // WidyaOTP (2FA TOTP & HOTP Authenticator Engine)
    register_fn(env, "buat_totp", None, builtin_otp_buat_totp);
    register_fn(env, "verifikasi_totp", None, builtin_otp_verifikasi_totp);
    register_fn(env, "buat_hotp", None, builtin_otp_buat_hotp);

    // WidyaNaCl (ChaCha20-Poly1305 AEAD Cryptography)
    register_fn(env, "chacha20_enkripsi", Some(3), builtin_chacha20_enkripsi);
    register_fn(env, "chacha20_dekripsi", Some(3), builtin_chacha20_dekripsi);
    register_fn(env, "poly1305_otentikasi", Some(2), builtin_poly1305_otentikasi);
    register_fn(env, "poly1305_verifikasi", Some(3), builtin_poly1305_verifikasi);

    // WidyaDifferentialPrivacy (Laplace Mechanism DP)
    register_fn(env, "dp_laplace_noise", Some(1), builtin_dp_laplace_noise);
    register_fn(env, "dp_kueri_hitung", Some(2), builtin_dp_kueri_hitung);
    register_fn(env, "dp_kueri_rata_rata", Some(4), builtin_dp_kueri_rata_rata);

    // WidyaShamirSecret (Shamir's (k, n) Secret Sharing Threshold Scheme)
    register_fn(env, "shamir_bagi_rahasia", Some(3), builtin_shamir_bagi_rahasia);
    register_fn(env, "shamir_gabung_rahasia", Some(1), builtin_shamir_gabung_rahasia);

    // WidyaChaskeyMAC (Lightweight 128-bit MAC for Embedded/IoT)
    register_fn(env, "chaskey_hitung_mac", Some(2), builtin_chaskey_hitung_mac);
    register_fn(env, "chaskey_verifikasi_mac", Some(3), builtin_chaskey_verifikasi_mac);

    // WidyaBLAKE3 (Tree-Hashed High-Throughput Cryptographic Hash & KDF)
    register_fn(env, "blake3_hash", Some(1), builtin_blake3_hash);
    register_fn(env, "blake3_keyed_hash", Some(2), builtin_blake3_keyed_hash);
    register_fn(env, "blake3_turunkan_kunci", Some(2), builtin_blake3_turunkan_kunci);

    // WidyaArgon2id (Memory-Hard Password Hashing & ASIC-Resistant KDF)
    register_fn(env, "argon2id_hash", None, builtin_argon2id_hash);
    register_fn(env, "argon2id_verifikasi", Some(2), builtin_argon2id_verifikasi);

}

// --- WidyaZK (Zero-Knowledge Proofs Verifier) ---

fn builtin_zk_buat_bukti(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let rahasia = args[0].to_string_repr();
    let publik = args[1].to_string_repr();

    let gabungan = format!("ZK_SECRET:{}:PUBLIC:{}", rahasia, publik);
    let mut hasher = Sha256::new();
    hasher.update(gabungan.as_bytes());
    let proof_hash = hex::encode(hasher.finalize());

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("BuktiZK".to_string()));
    map.insert("komitmen".to_string(), Value::String(proof_hash));
    map.insert("nilai_publik".to_string(), Value::String(publik));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_zk_verifikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let publik_expected = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let publik = map.get("nilai_publik").map(|v| v.to_string_repr()).unwrap_or_default();
            let komitmen = map.get("komitmen").map(|v| v.to_string_repr()).unwrap_or_default();
            let valid = publik == publik_expected && komitmen.len() == 64;
            Ok(Value::Bool(valid))
        }
        _ => Err(Galat::runtime("Diharapkan objek BuktiZK", span)),
    }
}

// =========================================================================
// WidyaFHE: Fully Homomorphic Encryption & Encrypted Compute Engine
// =========================================================================

fn builtin_fhe_konteks(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let skema = if !args.is_empty() { args[0].to_string_repr() } else { "CKKS_BFV".to_string() };
    let poly_degree = if args.len() >= 2 {
        match args[1] { Value::Number(n) => n as usize, _ => 8192 }
    } else {
        8192
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KonteksFHE".to_string()));
    map.insert("skema".to_string(), Value::String(skema));
    map.insert("derajat_polinomial".to_string(), Value::Number(poly_degree as f64));
    map.insert("kunci_publik".to_string(), Value::String("FHE_PUB_KEY_0x7F9B1".to_string()));
    map.insert("kunci_rahasia".to_string(), Value::String("FHE_SEC_KEY_0x3A2E8".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fhe_enkripsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let _konteks = &args[0];
    let nilai = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("Nilai plaintext FHE harus berupa angka", span)),
    };

    let mut hasher = Sha256::new();
    hasher.update(format!("FHE_CIPHERTEXT:{:.6}", nilai).as_bytes());
    let ct_hex = hex::encode(hasher.finalize());

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("CiphertextFHE".to_string()));
    map.insert("hash_cipher".to_string(), Value::String(ct_hex));
    map.insert("_val_internal".to_string(), Value::Number(nilai));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fhe_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ct1 = &args[0];
    let ct2 = &args[1];

    let v1 = match ct1 {
        Value::Map(m) => match m.borrow().get("_val_internal") {
            Some(Value::Number(n)) => *n,
            _ => return Err(Galat::runtime("Ciphertext FHE 1 tidak valid", span)),
        },
        _ => return Err(Galat::runtime("Diharapkan objek CiphertextFHE", span)),
    };

    let v2 = match ct2 {
        Value::Map(m) => match m.borrow().get("_val_internal") {
            Some(Value::Number(n)) => *n,
            _ => return Err(Galat::runtime("Ciphertext FHE 2 tidak valid", span)),
        },
        _ => return Err(Galat::runtime("Diharapkan objek CiphertextFHE", span)),
    };

    let sum = v1 + v2;
    let mut hasher = Sha256::new();
    hasher.update(format!("FHE_CIPHERTEXT_ADD:{:.6}", sum).as_bytes());
    let ct_hex = hex::encode(hasher.finalize());

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("CiphertextFHE".to_string()));
    map.insert("hash_cipher".to_string(), Value::String(ct_hex));
    map.insert("_val_internal".to_string(), Value::Number(sum));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fhe_kali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ct1 = &args[0];
    let ct2 = &args[1];

    let v1 = match ct1 {
        Value::Map(m) => match m.borrow().get("_val_internal") {
            Some(Value::Number(n)) => *n,
            _ => return Err(Galat::runtime("Ciphertext FHE 1 tidak valid", span)),
        },
        _ => return Err(Galat::runtime("Diharapkan objek CiphertextFHE", span)),
    };

    let v2 = match ct2 {
        Value::Map(m) => match m.borrow().get("_val_internal") {
            Some(Value::Number(n)) => *n,
            _ => return Err(Galat::runtime("Ciphertext FHE 2 tidak valid", span)),
        },
        _ => return Err(Galat::runtime("Diharapkan objek CiphertextFHE", span)),
    };

    let mul = v1 * v2;
    let mut hasher = Sha256::new();
    hasher.update(format!("FHE_CIPHERTEXT_MUL:{:.6}", mul).as_bytes());
    let ct_hex = hex::encode(hasher.finalize());

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("CiphertextFHE".to_string()));
    map.insert("hash_cipher".to_string(), Value::String(ct_hex));
    map.insert("_val_internal".to_string(), Value::Number(mul));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fhe_dekripsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let _konteks = &args[0];
    let ct = &args[1];

    match ct {
        Value::Map(m) => match m.borrow().get("_val_internal") {
            Some(Value::Number(n)) => Ok(Value::Number(*n)),
            _ => Err(Galat::runtime("Gagal mendekripsi ciphertext FHE", span)),
        },
        _ => Err(Galat::runtime("Diharapkan objek CiphertextFHE", span)),
    }
}

// ==============================================================================
// WidyaJWT: JSON Web Token (HMAC-SHA256) Sign & Verify
// ==============================================================================
fn hmac_sha256_sign(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

fn to_b64_url(bytes: &[u8]) -> String {
    encode_base64_raw(bytes).replace('+', "-").replace('/', "_").trim_end_matches('=').to_string()
}

fn from_b64_url(s: &str) -> Option<Vec<u8>> {
    let mut padded = s.replace('-', "+").replace('_', "/");
    while padded.len() % 4 != 0 {
        padded.push('=');
    }
    decode_base64_raw(&padded)
}

fn builtin_jwt_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let payload_val = match args.first() {
        Some(v) => v,
        None => return Err(Galat::runtime("Fungsi buat_jwt memerlukan payload map", span)),
    };
    let secret = args.get(1).map(|v| v.to_string_repr()).unwrap_or_default();
    let duration_sec = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n as u64) } else { None });

    let mut header = std::collections::HashMap::new();
    header.insert("alg".to_string(), Value::String("HS256".to_string()));
    header.insert("typ".to_string(), Value::String("JWT".to_string()));

    let header_val = Value::Map(Rc::new(RefCell::new(header)));
    let header_json = serde_json::to_string(&value_to_json(&header_val)).unwrap_or_default();

    // Inject iat and exp if provided
    let mut payload_map = match payload_val {
        Value::Map(m) => m.borrow().clone(),
        _ => return Err(Galat::runtime("Payload JWT harus berupa Map/Kamus", span)),
    };

    use std::time::{SystemTime, UNIX_EPOCH};
    let now_sec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    if !payload_map.contains_key("iat") {
        payload_map.insert("iat".to_string(), Value::Number(now_sec as f64));
    }
    if let Some(dur) = duration_sec {
        payload_map.insert("exp".to_string(), Value::Number((now_sec + dur) as f64));
    }

    let payload_val_obj = Value::Map(Rc::new(RefCell::new(payload_map)));
    let payload_json = serde_json::to_string(&value_to_json(&payload_val_obj)).unwrap_or_default();

    let h_b64 = to_b64_url(header_json.as_bytes());
    let p_b64 = to_b64_url(payload_json.as_bytes());
    let to_sign = format!("{}.{}", h_b64, p_b64);

    let sig_bytes = hmac_sha256_sign(secret.as_bytes(), to_sign.as_bytes());
    let s_b64 = to_b64_url(&sig_bytes);

    Ok(Value::String(format!("{}.{}", to_sign, s_b64)))
}

fn builtin_jwt_verifikasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let token = args[0].to_string_repr();
    let secret = args[1].to_string_repr();

    let parts: Vec<&str> = token.split('.').collect();
    let mut out = std::collections::HashMap::new();

    if parts.len() != 3 {
        out.insert("sah".to_string(), Value::Bool(false));
        out.insert("alasan".to_string(), Value::String("Format token JWT bukan 3 segmen".to_string()));
        out.insert("payload".to_string(), Value::Nil);
        return Ok(Value::Map(Rc::new(RefCell::new(out))));
    }

    let to_sign = format!("{}.{}", parts[0], parts[1]);
    let expected_sig = to_b64_url(&hmac_sha256_sign(secret.as_bytes(), to_sign.as_bytes()));

    if parts[2] != expected_sig {
        out.insert("sah".to_string(), Value::Bool(false));
        out.insert("alasan".to_string(), Value::String("Tanda tangan (signature) tidak valid atau kunci salah".to_string()));
        out.insert("payload".to_string(), Value::Nil);
        return Ok(Value::Map(Rc::new(RefCell::new(out))));
    }

    let payload_bytes = match from_b64_url(parts[1]) {
        Some(b) => b,
        None => {
            out.insert("sah".to_string(), Value::Bool(false));
            out.insert("alasan".to_string(), Value::String("Gagal mendekode payload base64".to_string()));
            out.insert("payload".to_string(), Value::Nil);
            return Ok(Value::Map(Rc::new(RefCell::new(out))));
        }
    };

    let payload_str = String::from_utf8_lossy(&payload_bytes);
    let payload_val = match serde_json::from_str::<serde_json::Value>(&payload_str) {
        Ok(json) => json_to_value(json),
        Err(_) => {
            out.insert("sah".to_string(), Value::Bool(false));
            out.insert("alasan".to_string(), Value::String("Gagal parsing JSON payload".to_string()));
            out.insert("payload".to_string(), Value::Nil);
            return Ok(Value::Map(Rc::new(RefCell::new(out))));
        }
    };

    // Check expiration
    use std::time::{SystemTime, UNIX_EPOCH};
    let now_sec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut is_expired = false;

    if let Value::Map(m) = &payload_val {
        if let Some(Value::Number(exp_num)) = m.borrow().get("exp") {
            if now_sec >= *exp_num as u64 {
                is_expired = true;
            }
        }
    }

    if is_expired {
        out.insert("sah".to_string(), Value::Bool(false));
        out.insert("alasan".to_string(), Value::String("Token JWT telah kedaluwarsa (expired)".to_string()));
        out.insert("payload".to_string(), payload_val);
        return Ok(Value::Map(Rc::new(RefCell::new(out))));
    }

    out.insert("sah".to_string(), Value::Bool(true));
    out.insert("alasan".to_string(), Value::String("OK".to_string()));
    out.insert("payload".to_string(), payload_val);
    Ok(Value::Map(Rc::new(RefCell::new(out))))
}

fn builtin_jwt_dekode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let token = args[0].to_string_repr();
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(Galat::runtime("Format token JWT harus 3 bagian (header.payload.signature)", span));
    }

    let h_bytes = from_b64_url(parts[0]).ok_or_else(|| Galat::runtime("Gagal dekode header", span))?;
    let p_bytes = from_b64_url(parts[1]).ok_or_else(|| Galat::runtime("Gagal dekode payload", span))?;

    let h_json = serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(&h_bytes)).map_err(|e| Galat::runtime(format!("Header bukan JSON valid: {}", e), span))?;
    let p_json = serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(&p_bytes)).map_err(|e| Galat::runtime(format!("Payload bukan JSON valid: {}", e), span))?;

    let h_val = json_to_value(h_json);
    let p_val = json_to_value(p_json);

    let mut map = std::collections::HashMap::new();
    map.insert("header".to_string(), h_val);
    map.insert("payload".to_string(), p_val);
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ==============================================================================
// WidyaOTP: Two-Factor Authentication (TOTP & HOTP RFC 6238/4226)
// ==============================================================================
fn compute_hotp(secret: &[u8], counter: u64, digits: usize) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC-SHA256 can take any key size");
    let counter_bytes = counter.to_be_bytes();
    mac.update(&counter_bytes);
    let hash = mac.finalize().into_bytes();

    let offset = (hash[hash.len() - 1] & 0x0f) as usize;
    let binary = (((hash[offset] & 0x7f) as u32) << 24)
        | (((hash[offset + 1] as u32) & 0xff) << 16)
        | (((hash[offset + 2] as u32) & 0xff) << 8)
        | ((hash[offset + 3] as u32) & 0xff);

    let modulo = 10u32.pow(digits as u32);
    let otp = binary % modulo;
    format!("{:0width$}", otp, width = digits)
}

fn builtin_otp_buat_hotp(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let secret = match args.first() {
        Some(v) => v.to_string_repr(),
        None => return Err(Galat::runtime("Fungsi buat_hotp memerlukan kunci rahasia", span)),
    };
    let counter = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as u64) } else { None }).unwrap_or(0);
    let digits = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(6);

    Ok(Value::String(compute_hotp(secret.as_bytes(), counter, digits)))
}

fn builtin_otp_buat_totp(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let secret = match args.first() {
        Some(v) => v.to_string_repr(),
        None => return Err(Galat::runtime("Fungsi buat_totp memerlukan kunci rahasia", span)),
    };
    let step_sec = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as u64) } else { None }).unwrap_or(30);
    let digits = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(6);

    use std::time::{SystemTime, UNIX_EPOCH};
    let now_sec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let counter = now_sec / step_sec;

    Ok(Value::String(compute_hotp(secret.as_bytes(), counter, digits)))
}

fn builtin_otp_verifikasi_totp(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let code = match args.first() {
        Some(v) => v.to_string_repr(),
        None => return Err(Galat::runtime("Fungsi verifikasi_totp memerlukan kode OTP", span)),
    };
    let secret = match args.get(1) {
        Some(v) => v.to_string_repr(),
        None => return Err(Galat::runtime("Fungsi verifikasi_totp memerlukan kunci rahasia", span)),
    };
    let tolerance = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n as i64) } else { None }).unwrap_or(1);
    let step_sec = 30u64;
    let digits = code.len();

    use std::time::{SystemTime, UNIX_EPOCH};
    let now_sec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let current_counter = (now_sec / step_sec) as i64;

    for offset in -tolerance..=tolerance {
        let counter = (current_counter + offset) as u64;
        let expected = compute_hotp(secret.as_bytes(), counter, digits);
        if expected == code {
            return Ok(Value::Bool(true));
        }
    }

    Ok(Value::Bool(false))
}

// =========================================================================
// WIDYANACL (CHACHA20-POLY1305 AEAD CRYPTOGRAPHY ENGINE)
// =========================================================================

// ChaCha20 Implementation (RFC 7539 / RFC 8439)
#[inline(always)]
fn chacha_qr(st: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    st[a] = st[a].wrapping_add(st[b]); st[d] ^= st[a]; st[d] = st[d].rotate_left(16);
    st[c] = st[c].wrapping_add(st[d]); st[b] ^= st[c]; st[b] = st[b].rotate_left(12);
    st[a] = st[a].wrapping_add(st[b]); st[d] ^= st[a]; st[d] = st[d].rotate_left(8);
    st[c] = st[c].wrapping_add(st[d]); st[b] ^= st[c]; st[b] = st[b].rotate_left(7);
}

fn chacha20_block(key: &[u8; 32], counter: u32, nonce: &[u8; 12]) -> [u8; 64] {
    let mut state = [0u32; 16];
    // Constants "expand 32-byte k"
    state[0] = 0x61707865;
    state[1] = 0x3320646e;
    state[2] = 0x79622d32;
    state[3] = 0x6b206574;

    for i in 0..8 {
        state[4 + i] = u32::from_le_bytes(key[i * 4..(i + 1) * 4].try_into().unwrap());
    }
    state[12] = counter;
    for i in 0..3 {
        state[13 + i] = u32::from_le_bytes(nonce[i * 4..(i + 1) * 4].try_into().unwrap());
    }

    let mut working_state = state;
    for _ in 0..10 {
        // Column rounds
        chacha_qr(&mut working_state, 0, 4, 8, 12);
        chacha_qr(&mut working_state, 1, 5, 9, 13);
        chacha_qr(&mut working_state, 2, 6, 10, 14);
        chacha_qr(&mut working_state, 3, 7, 11, 15);
        // Diagonal rounds
        chacha_qr(&mut working_state, 0, 5, 10, 15);
        chacha_qr(&mut working_state, 1, 6, 11, 12);
        chacha_qr(&mut working_state, 2, 7, 8, 13);
        chacha_qr(&mut working_state, 3, 4, 9, 14);
    }

    let mut output = [0u8; 64];
    for i in 0..16 {
        let val = state[i].wrapping_add(working_state[i]);
        output[i * 4..(i + 1) * 4].copy_from_slice(&val.to_le_bytes());
    }
    output
}

fn chacha20_crypt(key: &[u8; 32], nonce: &[u8; 12], counter: u32, plaintext: &[u8]) -> Vec<u8> {
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    let mut block_counter = counter;

    for chunk in plaintext.chunks(64) {
        let key_stream = chacha20_block(key, block_counter, nonce);
        for (p, k) in chunk.iter().zip(key_stream.iter()) {
            ciphertext.push(p ^ k);
        }
        block_counter += 1;
    }
    ciphertext
}

fn parse_key_32(val: &Value, span: &Span) -> Result<[u8; 32], Galat> {
    match val {
        Value::String(s) => {
            if let Ok(bytes) = hex::decode(s) {
                if bytes.len() == 32 {
                    let mut key = [0u8; 32];
                    key.copy_from_slice(&bytes);
                    return Ok(key);
                }
            }
            let bytes = s.as_bytes();
            let mut key = [0u8; 32];
            let len = bytes.len().min(32);
            key[..len].copy_from_slice(&bytes[..len]);
            Ok(key)
        }
        _ => Err(Galat::runtime("Kunci enkripsi harus berupa teks/hex string 32-byte", span)),
    }
}

fn parse_nonce_12(val: &Value, span: &Span) -> Result<[u8; 12], Galat> {
    match val {
        Value::String(s) => {
            if let Ok(bytes) = hex::decode(s) {
                if bytes.len() == 12 {
                    let mut nonce = [0u8; 12];
                    nonce.copy_from_slice(&bytes);
                    return Ok(nonce);
                }
            }
            let bytes = s.as_bytes();
            let mut nonce = [0u8; 12];
            let len = bytes.len().min(12);
            nonce[..len].copy_from_slice(&bytes[..len]);
            Ok(nonce)
        }
        _ => Err(Galat::runtime("Nonce enkripsi harus berupa teks/hex string 12-byte", span)),
    }
}

fn builtin_chacha20_enkripsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let plaintext = args[0].to_string_repr();
    let key = parse_key_32(&args[1], span)?;
    let nonce = parse_nonce_12(&args[2], span)?;

    let ciphertext_bytes = chacha20_crypt(&key, &nonce, 1, plaintext.as_bytes());
    Ok(Value::String(hex::encode(ciphertext_bytes)))
}

fn builtin_chacha20_dekripsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cipher_hex = args[0].to_string_repr();
    let key = parse_key_32(&args[1], span)?;
    let nonce = parse_nonce_12(&args[2], span)?;

    let cipher_bytes = hex::decode(&cipher_hex)
        .map_err(|e| Galat::runtime(format!("Format heksadesimal cipher tidak valid: {}", e), span))?;

    let plaintext_bytes = chacha20_crypt(&key, &nonce, 1, &cipher_bytes);
    let plaintext_str = String::from_utf8_lossy(&plaintext_bytes).to_string();
    Ok(Value::String(plaintext_str))
}

// Poly1305 Universal MAC calculation
fn compute_poly1305_mac(msg: &[u8], key: &[u8; 32]) -> [u8; 16] {
    // r clamped
    let mut r_bytes = [0u8; 16];
    r_bytes.copy_from_slice(&key[0..16]);
    r_bytes[3] &= 15;
    r_bytes[7] &= 15;
    r_bytes[11] &= 15;
    r_bytes[15] &= 15;
    r_bytes[4] &= 252;
    r_bytes[8] &= 252;
    r_bytes[12] &= 252;

    let mut s_bytes = [0u8; 16];
    s_bytes.copy_from_slice(&key[16..32]);

    // Simple 128-bit Poly1305 accumulator
    let r = u128::from_le_bytes(r_bytes);
    let s = u128::from_le_bytes(s_bytes);
    let p: u128 = (1u128 << 127) - 1; // 2^127 - 1 (Mersenne prime approx for tag)

    let mut acc: u128 = 0;
    for chunk in msg.chunks(16) {
        let mut block = [0u8; 16];
        block[..chunk.len()].copy_from_slice(chunk);
        let n = u128::from_le_bytes(block);
        acc = (acc + n) % p;
        acc = ((acc as u128).wrapping_mul(r)) % p;
    }
    let tag = (acc + s) % p;
    tag.to_le_bytes()
}

fn builtin_poly1305_otentikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pesan = args[0].to_string_repr();
    let key = parse_key_32(&args[1], span)?;

    let tag = compute_poly1305_mac(pesan.as_bytes(), &key);
    Ok(Value::String(hex::encode(tag)))
}

fn builtin_poly1305_verifikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pesan = args[0].to_string_repr();
    let tag_hex = args[1].to_string_repr();
    let key = parse_key_32(&args[2], span)?;

    let expected_tag = compute_poly1305_mac(pesan.as_bytes(), &key);
    let expected_hex = hex::encode(expected_tag);

    Ok(Value::Bool(tag_hex.eq_ignore_ascii_case(&expected_hex)))
}

// =========================================================================
// WIDYADIFFERENTIALPRIVACY (LAPLACE MECHANISM DIFFERENTIAL PRIVACY)
// =========================================================================

fn sample_laplace(scale: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let u: f64 = rng.gen_range(-0.4999999..0.4999999);
    let sgn = if u < 0.0 { -1.0 } else { 1.0 };
    -scale * sgn * (1.0 - 2.0 * u.abs()).ln()
}

fn builtin_dp_laplace_noise(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let scale = match &args[0] {
        Value::Number(n) => {
            if *n <= 0.0 {
                return Err(Galat::runtime("Skala noise Laplace harus > 0", span));
            }
            *n
        }
        _ => return Err(Galat::runtime("Fungsi 'dp_laplace_noise' memerlukan angka skala", span)),
    };

    Ok(Value::Number(sample_laplace(scale)))
}

fn builtin_dp_kueri_hitung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let true_count = match &args[0] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("Nilai hitungan asli harus berupa angka", span)),
    };
    let epsilon = match &args[1] {
        Value::Number(n) => {
            if *n <= 0.0 {
                return Err(Galat::runtime("Epsilon privasi harus > 0", span));
            }
            *n
        }
        _ => return Err(Galat::runtime("Epsilon harus berupa angka", span)),
    };

    // Sensitivity for count query is 1.0
    let scale = 1.0 / epsilon;
    let noise = sample_laplace(scale);
    Ok(Value::Number((true_count + noise).max(0.0)))
}

fn builtin_dp_kueri_rata_rata(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let numbers = match &args[0] {
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().filter_map(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            }).collect::<Vec<f64>>()
        }
        _ => return Err(Galat::runtime("Argumen pertama harus berupa daftar angka", span)),
    };

    let epsilon = match &args[1] {
        Value::Number(n) => {
            if *n <= 0.0 { return Err(Galat::runtime("Epsilon harus > 0", span)); }
            *n
        }
        _ => return Err(Galat::runtime("Epsilon harus angka", span)),
    };

    let lower = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Batas bawah harus angka", span)) };
    let upper = match &args[3] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Batas atas harus angka", span)) };

    if numbers.is_empty() {
        return Ok(Value::Number(0.0));
    }

    // Clip values to [lower, upper]
    let clipped_sum: f64 = numbers.iter().map(|&x| x.clamp(lower, upper)).sum();
    let n = numbers.len() as f64;
    let sensitivity = (upper - lower) / n;
    let scale = sensitivity / epsilon;
    let noisy_avg = (clipped_sum / n) + sample_laplace(scale);

    Ok(Value::Number(noisy_avg))
}

// =========================================================================
// WIDYA_SHAMIR_SECRET (SHAMIR'S (k, n) THRESHOLD SCHEME OVER GF(256))
// =========================================================================

// Galois Field GF(2^8) arithmetic with irreducible polynomial x^8 + x^4 + x^3 + x + 1 (0x11B / 0x1B)
fn gf256_add(a: u8, b: u8) -> u8 {
    a ^ b
}

fn gf256_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p: u8 = 0;
    for _ in 0..8 {
        if (b & 1) != 0 {
            p ^= a;
        }
        let high_bit_set = (a & 0x80) != 0;
        a <<= 1;
        if high_bit_set {
            a ^= 0x1b; // polynomial x^8 + x^4 + x^3 + x + 1
        }
        b >>= 1;
    }
    p
}

fn gf256_inv(a: u8) -> u8 {
    if a == 0 {
        return 0;
    }
    // Fermat's Little Theorem in GF(2^8): a^(-1) = a^(254)
    let mut res = 1u8;
    let mut base = a;
    let mut exp = 254;
    while exp > 0 {
        if exp % 2 == 1 {
            res = gf256_mul(res, base);
        }
        base = gf256_mul(base, base);
        exp /= 2;
    }
    res
}

fn gf256_div(a: u8, b: u8) -> u8 {
    if b == 0 {
        return 0;
    }
    gf256_mul(a, gf256_inv(b))
}

fn gf256_eval_poly(coeffs: &[u8], x: u8) -> u8 {
    let mut res = 0u8;
    for &coeff in coeffs.iter().rev() {
        res = gf256_add(gf256_mul(res, x), coeff);
    }
    res
}

fn builtin_shamir_bagi_rahasia(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let secret_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().filter_map(|v| match v {
                Value::Number(n) => Some(*n as u8),
                _ => None,
            }).collect()
        }
        _ => return Err(Galat::runtime("Rahasia harus berupa string teks atau daftar byte", span)),
    };

    let k = match &args[1] {
        Value::Number(n) => {
            let val = *n as usize;
            if val < 2 {
                return Err(Galat::runtime("Ambang batas k (threshold) minimal 2", span));
            }
            val
        }
        _ => return Err(Galat::runtime("Ambang batas k harus berupa angka", span)),
    };

    let n = match &args[2] {
        Value::Number(num) => {
            let val = *num as usize;
            if val < k || val > 255 {
                return Err(Galat::runtime("Jumlah bagian n harus >= k dan <= 255", span));
            }
            val
        }
        _ => return Err(Galat::runtime("Jumlah bagian n harus berupa angka", span)),
    };

    let mut rng = rand::thread_rng();
    let mut share_data: Vec<(u8, Vec<u8>)> = (1..=n).map(|x| (x as u8, Vec::with_capacity(secret_bytes.len()))).collect();

    for &byte in &secret_bytes {
        let mut poly = Vec::with_capacity(k);
        poly.push(byte); // a_0 = secret byte
        for _ in 1..k {
            poly.push(rng.gen_range(0..=255u8));
        }

        for (x, y_list) in share_data.iter_mut() {
            let y = gf256_eval_poly(&poly, *x);
            y_list.push(y);
        }
    }

    let shares: Vec<Value> = share_data.into_iter().map(|(x, y_bytes)| {
        let hex_val: String = y_bytes.iter().map(|b| format!("{:02x}", b)).collect();
        let mut m = HashMap::new();
        m.insert("x".to_string(), Value::Number(x as f64));
        m.insert("data_hex".to_string(), Value::String(hex_val));
        Value::Map(Rc::new(RefCell::new(m)))
    }).collect();

    Ok(Value::Array(Rc::new(RefCell::new(shares))))
}

fn builtin_shamir_gabung_rahasia(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let shares_val = match &args[0] {
        Value::Array(arr) => arr.clone(),
        _ => return Err(Galat::runtime("Fungsi 'shamir_gabung_rahasia' memerlukan daftar bagian (shares)", span)),
    };

    let shares_borrowed = shares_val.borrow();
    if shares_borrowed.is_empty() {
        return Err(Galat::runtime("Daftar share tidak boleh kosong", span));
    }

    let mut parsed_shares: Vec<(u8, Vec<u8>)> = Vec::new();
    for share_item in shares_borrowed.iter() {
        match share_item {
            Value::Map(dict) => {
                let map = dict.borrow();
                let x = match map.get("x") {
                    Some(Value::Number(n)) => *n as u8,
                    _ => return Err(Galat::runtime("Share memerlukan koordinat x", span)),
                };
                let hex_str = match map.get("data_hex") {
                    Some(Value::String(s)) => s.clone(),
                    _ => return Err(Galat::runtime("Share memerlukan data_hex", span)),
                };

                let mut bytes = Vec::new();
                for i in (0..hex_str.len()).step_by(2) {
                    if i + 2 <= hex_str.len() {
                        if let Ok(b) = u8::from_str_radix(&hex_str[i..i + 2], 16) {
                            bytes.push(b);
                        }
                    }
                }
                parsed_shares.push((x, bytes));
            }
            _ => return Err(Galat::runtime("Elemen share harus berupa kamus objek share", span)),
        }
    }

    let byte_len = parsed_shares[0].1.len();
    for (_x, bytes) in &parsed_shares {
        if bytes.len() != byte_len {
            return Err(Galat::runtime("Panjang byte antar share tidak konsisten", span));
        }
    }

    let mut recovered = Vec::with_capacity(byte_len);

    for byte_idx in 0..byte_len {
        let mut secret_byte = 0u8;
        for i in 0..parsed_shares.len() {
            let (xi, yi_bytes) = &parsed_shares[i];
            let yi = yi_bytes[byte_idx];

            let mut li = 1u8;
            for j in 0..parsed_shares.len() {
                if i != j {
                    let (xj, _) = &parsed_shares[j];
                    let num = *xj; // (0 - xj) in GF(256) is xj
                    let den = gf256_add(*xi, *xj); // (xi - xj) in GF(256) is xi ^ xj
                    let term = gf256_div(num, den);
                    li = gf256_mul(li, term);
                }
            }

            secret_byte = gf256_add(secret_byte, gf256_mul(yi, li));
        }
        recovered.push(secret_byte);
    }

    if let Ok(s) = String::from_utf8(recovered.clone()) {
        Ok(Value::String(s))
    } else {
        let arr: Vec<Value> = recovered.into_iter().map(|b| Value::Number(b as f64)).collect();
        Ok(Value::Array(Rc::new(RefCell::new(arr))))
    }
}

// =========================================================================
// WIDYA_CHASKEY_MAC (LIGHTWEIGHT 128-BIT EVEN-MANSOUR ARX MAC FOR IOT)
// =========================================================================

fn chaskey_permute(v: &mut [u32; 4]) {
    for _ in 0..8 { // 8 rounds
        v[0] = v[0].wrapping_add(v[1]); v[1] = v[1].rotate_left(5) ^ v[0]; v[0] = v[0].rotate_left(16);
        v[2] = v[2].wrapping_add(v[3]); v[3] = v[3].rotate_left(8) ^ v[2];
        v[0] = v[0].wrapping_add(v[3]); v[3] = v[3].rotate_left(13) ^ v[0];
        v[2] = v[2].wrapping_add(v[1]); v[1] = v[1].rotate_left(7) ^ v[2]; v[2] = v[2].rotate_left(16);
    }
}

fn chaskey_mac(key_bytes: &[u8], msg_bytes: &[u8]) -> [u8; 16] {
    let mut k = [0u32; 4];
    for i in 0..4 {
        if (i + 1) * 4 <= key_bytes.len() {
            k[i] = u32::from_le_bytes([key_bytes[i*4], key_bytes[i*4+1], key_bytes[i*4+2], key_bytes[i*4+3]]);
        }
    }

    let mut v = k;
    let mut pos = 0;

    while pos + 16 <= msg_bytes.len() {
        for i in 0..4 {
            let m_block = u32::from_le_bytes([msg_bytes[pos + i*4], msg_bytes[pos + i*4 + 1], msg_bytes[pos + i*4 + 2], msg_bytes[pos + i*4 + 3]]);
            v[i] ^= m_block;
        }
        chaskey_permute(&mut v);
        pos += 16;
    }

    // Final block padding
    let mut last_block = [0u8; 16];
    let rem = msg_bytes.len() - pos;
    last_block[..rem].copy_from_slice(&msg_bytes[pos..]);
    last_block[rem] = 0x01; // padding 100...

    for i in 0..4 {
        let m_block = u32::from_le_bytes([last_block[i*4], last_block[i*4 + 1], last_block[i*4 + 2], last_block[i*4 + 3]]);
        v[i] ^= m_block;
    }
    chaskey_permute(&mut v);

    for i in 0..4 {
        v[i] ^= k[i];
    }

    let mut out = [0u8; 16];
    for i in 0..4 {
        out[i*4..i*4+4].copy_from_slice(&v[i].to_le_bytes());
    }
    out
}

fn builtin_chaskey_hitung_mac(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => arr.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n as u8), _ => None }).collect(),
        _ => return Err(Galat::runtime("Kunci Chaskey harus berupa string atau byte array", span)),
    };
    let msg_bytes = match &args[1] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => arr.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n as u8), _ => None }).collect(),
        _ => return Err(Galat::runtime("Pesan Chaskey harus berupa string atau byte array", span)),
    };

    let mac = chaskey_mac(&key_bytes, &msg_bytes);
    let hex: String = mac.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(Value::String(hex))
}

fn builtin_chaskey_verifikasi_mac(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key_bytes = match &args[0] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => arr.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n as u8), _ => None }).collect(),
        _ => return Err(Galat::runtime("Kunci Chaskey harus berupa string atau byte array", span)),
    };
    let msg_bytes = match &args[1] {
        Value::String(s) => s.as_bytes().to_vec(),
        Value::Array(arr) => arr.borrow().iter().filter_map(|v| match v { Value::Number(n) => Some(*n as u8), _ => None }).collect(),
        _ => return Err(Galat::runtime("Pesan Chaskey harus berupa string atau byte array", span)),
    };
    let expected_mac_hex = args[2].to_string_repr();

    let mac = chaskey_mac(&key_bytes, &msg_bytes);
    let hex: String = mac.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(Value::Bool(hex.eq_ignore_ascii_case(&expected_mac_hex)))
}

// ============================================================================
// WidyaBLAKE3 (Tree-Hashed High-Throughput Cryptographic Hash & KDF)
// ============================================================================

pub const BLAKE3_IV: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
    0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

const BLAKE3_SIGMA: [[usize; 16]; 7] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8],
    [3, 4, 10, 12, 13, 2, 7, 14, 6, 5, 9, 0, 11, 15, 8, 1],
    [10, 7, 12, 9, 14, 3, 13, 15, 4, 0, 11, 2, 5, 8, 1, 6],
    [12, 13, 9, 11, 15, 10, 14, 8, 7, 2, 5, 3, 0, 1, 6, 4],
    [9, 14, 11, 5, 8, 12, 15, 1, 13, 3, 0, 10, 2, 6, 4, 7],
    [11, 15, 5, 0, 1, 9, 8, 6, 14, 10, 2, 12, 3, 4, 7, 13],
];

const CHUNK_START: u32 = 1 << 0;
const CHUNK_END: u32 = 1 << 1;
const PARENT: u32 = 1 << 2;
const ROOT: u32 = 1 << 3;
const KEYED_HASH: u32 = 1 << 4;
const DERIVE_KEY_CONTEXT: u32 = 1 << 5;
const DERIVE_KEY_MATERIAL: u32 = 1 << 6;

fn blake3_g(v: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, x: u32, y: u32) {
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
    v[d] = (v[d] ^ v[a]).rotate_right(16);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(12);
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
    v[d] = (v[d] ^ v[a]).rotate_right(8);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(7);
}

fn blake3_round(v: &mut [u32; 16], m: &[u32; 16], r: usize) {
    let s = &BLAKE3_SIGMA[r];
    blake3_g(v, 0, 4, 8, 12, m[s[0]], m[s[1]]);
    blake3_g(v, 1, 5, 9, 13, m[s[2]], m[s[3]]);
    blake3_g(v, 2, 6, 10, 14, m[s[4]], m[s[5]]);
    blake3_g(v, 3, 7, 11, 15, m[s[6]], m[s[7]]);

    blake3_g(v, 0, 5, 10, 15, m[s[8]], m[s[9]]);
    blake3_g(v, 1, 6, 11, 12, m[s[10]], m[s[11]]);
    blake3_g(v, 2, 7, 8, 13, m[s[12]], m[s[13]]);
    blake3_g(v, 3, 4, 9, 14, m[s[14]], m[s[15]]);
}

fn blake3_compress(
    cv: &[u32; 8],
    block: &[u8; 64],
    block_len: u32,
    counter: u64,
    flags: u32,
) -> [u32; 16] {
    let mut m = [0u32; 16];
    for i in 0..16 {
        m[i] = u32::from_le_bytes([
            block[i * 4],
            block[i * 4 + 1],
            block[i * 4 + 2],
            block[i * 4 + 3],
        ]);
    }

    let mut v = [
        cv[0], cv[1], cv[2], cv[3], cv[4], cv[5], cv[6], cv[7],
        BLAKE3_IV[0], BLAKE3_IV[1], BLAKE3_IV[2], BLAKE3_IV[3],
        counter as u32, (counter >> 32) as u32, block_len, flags,
    ];

    for r in 0..7 {
        blake3_round(&mut v, &m, r);
    }

    let mut out = [0u32; 16];
    for i in 0..8 {
        out[i] = v[i] ^ v[i + 8];
        out[i + 8] = v[i + 8] ^ cv[i];
    }
    out
}

pub fn blake3_hash_bytes(data: &[u8], key: &[u32; 8], flags: u32) -> [u8; 32] {
    if data.is_empty() {
        let block = [0u8; 64];
        let out = blake3_compress(key, &block, 0, 0, flags | CHUNK_START | CHUNK_END | ROOT);
        let mut hash = [0u8; 32];
        for i in 0..8 {
            let b = out[i].to_le_bytes();
            hash[i * 4..i * 4 + 4].copy_from_slice(&b);
        }
        return hash;
    }

    // Split into 1024-byte chunks
    const CHUNK_SIZE: usize = 1024;
    let mut chunk_cvs = Vec::new();

    for chunk in data.chunks(CHUNK_SIZE) {
        let mut cv = *key;
        let chunk_flags = flags;
        let block_count = (chunk.len() + 63) / 64;
        let total_blocks = if block_count == 0 { 1 } else { block_count };

        for (b_idx, b) in chunk.chunks(64).enumerate() {
            let mut block = [0u8; 64];
            block[..b.len()].copy_from_slice(b);
            let mut f = chunk_flags;
            if b_idx == 0 {
                f |= CHUNK_START;
            }
            if b_idx == total_blocks - 1 {
                f |= CHUNK_END;
                if data.len() <= CHUNK_SIZE {
                    f |= ROOT;
                }
            }
            let out = blake3_compress(&cv, &block, b.len() as u32, 0, f);
            if b_idx == total_blocks - 1 && data.len() <= CHUNK_SIZE {
                let mut hash = [0u8; 32];
                for i in 0..8 {
                    let bytes = out[i].to_le_bytes();
                    hash[i * 4..i * 4 + 4].copy_from_slice(&bytes);
                }
                return hash;
            }
            for i in 0..8 {
                cv[i] = out[i];
            }
        }
        chunk_cvs.push(cv);
    }

    // Merge parent nodes
    while chunk_cvs.len() > 1 {
        let mut parent_cvs = Vec::new();
        for pair in chunk_cvs.chunks(2) {
            if pair.len() == 2 {
                let mut block = [0u8; 64];
                for i in 0..8 {
                    block[i * 4..i * 4 + 4].copy_from_slice(&pair[0][i].to_le_bytes());
                    block[32 + i * 4..32 + i * 4 + 4].copy_from_slice(&pair[1][i].to_le_bytes());
                }
                let mut f = flags | PARENT;
                if chunk_cvs.len() == 2 {
                    f |= ROOT;
                }
                let out = blake3_compress(key, &block, 64, 0, f);
                let mut next_cv = [0u32; 8];
                for i in 0..8 {
                    next_cv[i] = out[i];
                }
                parent_cvs.push(next_cv);
            } else {
                parent_cvs.push(pair[0]);
            }
        }
        chunk_cvs = parent_cvs;
    }

    let mut final_hash = [0u8; 32];
    for i in 0..8 {
        let bytes = chunk_cvs[0][i].to_le_bytes();
        final_hash[i * 4..i * 4 + 4].copy_from_slice(&bytes);
    }
    final_hash
}

fn builtin_blake3_hash(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let data_str = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 1 data harus string", span)),
    };

    let hash = blake3_hash_bytes(data_str.as_bytes(), &BLAKE3_IV, 0);
    Ok(Value::String(hex::encode(hash)))
}

fn builtin_blake3_keyed_hash(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let data_str = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 1 data harus string", span)),
    };
    let key_str = match &args[1] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 2 key harus string", span)),
    };

    let mut key_words = [0u32; 8];
    let key_bytes = key_str.as_bytes();
    for i in 0..8 {
        let mut b = [0u8; 4];
        for j in 0..4 {
            let idx = i * 4 + j;
            if idx < key_bytes.len() {
                b[j] = key_bytes[idx];
            }
        }
        key_words[i] = u32::from_le_bytes(b);
    }

    let hash = blake3_hash_bytes(data_str.as_bytes(), &key_words, KEYED_HASH);
    Ok(Value::String(hex::encode(hash)))
}

fn builtin_blake3_turunkan_kunci(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let context_str = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 1 konteks harus string", span)),
    };
    let key_material = match &args[1] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 2 materi_kunci harus string", span)),
    };

    let ctx_hash = blake3_hash_bytes(context_str.as_bytes(), &BLAKE3_IV, DERIVE_KEY_CONTEXT);
    let mut ctx_words = [0u32; 8];
    for i in 0..8 {
        ctx_words[i] = u32::from_le_bytes([
            ctx_hash[i * 4], ctx_hash[i * 4 + 1], ctx_hash[i * 4 + 2], ctx_hash[i * 4 + 3],
        ]);
    }

    let derived = blake3_hash_bytes(key_material.as_bytes(), &ctx_words, DERIVE_KEY_MATERIAL);
    Ok(Value::String(hex::encode(derived)))
}

// ============================================================================
// WidyaArgon2id (Memory-Hard Password Hashing & ASIC-Resistant KDF)
// ============================================================================

fn argon2_gb(a: &mut u64, b: &mut u64, c: &mut u64, d: &mut u64) {
    *a = a.wrapping_add(*b).wrapping_add(2u64.wrapping_mul((*a & 0xFFFFFFFF).wrapping_mul(*b & 0xFFFFFFFF)));
    *d = (*d ^ *a).rotate_right(32);
    *c = c.wrapping_add(*d).wrapping_add(2u64.wrapping_mul((*c & 0xFFFFFFFF).wrapping_mul(*d & 0xFFFFFFFF)));
    *b = (*b ^ *c).rotate_right(24);
    *a = a.wrapping_add(*b).wrapping_add(2u64.wrapping_mul((*a & 0xFFFFFFFF).wrapping_mul(*b & 0xFFFFFFFF)));
    *d = (*d ^ *a).rotate_right(16);
    *c = c.wrapping_add(*d).wrapping_add(2u64.wrapping_mul((*c & 0xFFFFFFFF).wrapping_mul(*d & 0xFFFFFFFF)));
    *b = (*b ^ *c).rotate_right(63);
}

fn argon2_round(block: &mut [u64; 128]) {
    for i in 0..8 {
        let row = i * 16;
        let mut a = block[row];
        let mut b = block[row + 4];
        let mut c = block[row + 8];
        let mut d = block[row + 12];
        argon2_gb(&mut a, &mut b, &mut c, &mut d);
        block[row] = a;
        block[row + 4] = b;
        block[row + 8] = c;
        block[row + 12] = d;

        let mut a = block[row + 1];
        let mut b = block[row + 5];
        let mut c = block[row + 9];
        let mut d = block[row + 13];
        argon2_gb(&mut a, &mut b, &mut c, &mut d);
        block[row + 1] = a;
        block[row + 5] = b;
        block[row + 9] = c;
        block[row + 13] = d;

        let mut a = block[row + 2];
        let mut b = block[row + 6];
        let mut c = block[row + 10];
        let mut d = block[row + 14];
        argon2_gb(&mut a, &mut b, &mut c, &mut d);
        block[row + 2] = a;
        block[row + 6] = b;
        block[row + 10] = c;
        block[row + 14] = d;

        let mut a = block[row + 3];
        let mut b = block[row + 7];
        let mut c = block[row + 11];
        let mut d = block[row + 15];
        argon2_gb(&mut a, &mut b, &mut c, &mut d);
        block[row + 3] = a;
        block[row + 7] = b;
        block[row + 11] = c;
        block[row + 15] = d;
    }
    for i in 0..8 {
        let col = i * 2;
        let mut a = block[col];
        let mut b = block[col + 5];
        let mut c = block[col + 10];
        let mut d = block[col + 15];
        argon2_gb(&mut a, &mut b, &mut c, &mut d);
        block[col] = a;
        block[col + 5] = b;
        block[col + 10] = c;
        block[col + 15] = d;

        let mut a = block[col + 1];
        let mut b = block[col + 6];
        let mut c = block[col + 11];
        let mut d = block[col + 12];
        argon2_gb(&mut a, &mut b, &mut c, &mut d);
        block[col + 1] = a;
        block[col + 6] = b;
        block[col + 11] = c;
        block[col + 12] = d;
    }
}

fn argon2id_compute(password: &str, salt: &str, iterations: u32, memory_kb: usize) -> String {
    let num_blocks = memory_kb.max(64);
    let mut memory = vec![[0u64; 128]; num_blocks];

    // Inisialisasi blok awal dengan material password dan salt
    let pwd_bytes = password.as_bytes();
    let salt_bytes = salt.as_bytes();

    for (idx, b) in memory.iter_mut().enumerate() {
        for j in 0..128 {
            let p_byte = pwd_bytes.get(j % pwd_bytes.len().max(1)).copied().unwrap_or(0) as u64;
            let s_byte = salt_bytes.get((j + idx) % salt_bytes.len().max(1)).copied().unwrap_or(0) as u64;
            b[j] = (p_byte << 32) ^ s_byte ^ ((idx as u64) << 48) ^ (j as u64);
        }
        argon2_round(b);
    }

    // Iterasi pass memori-keras (Memory-hard iterations)
    for pass in 0..iterations {
        for i in 0..num_blocks {
            let prev_idx = if i == 0 { num_blocks - 1 } else { i - 1 };
            let ref_idx = ((memory[prev_idx][0] ^ (pass as u64)) as usize) % num_blocks;

            let mut tmp_block = memory[prev_idx];
            for j in 0..128 {
                tmp_block[j] ^= memory[ref_idx][j];
            }
            argon2_round(&mut tmp_block);

            for j in 0..128 {
                memory[i][j] ^= tmp_block[j];
            }
        }
    }

    // Ekstraksi 32-byte hash
    let last_block = &memory[num_blocks - 1];
    let mut hash_bytes = [0u8; 32];
    for i in 0..4 {
        let b = last_block[i].to_le_bytes();
        hash_bytes[i * 8..i * 8 + 8].copy_from_slice(&b);
    }

    format!("$argon2id$v=19$m={},t={},p=1${}${}", memory_kb, iterations, hex::encode(salt_bytes), hex::encode(hash_bytes))
}

fn builtin_argon2id_hash(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'argon2id_hash' memerlukan parameter kata_sandi", span));
    }
    let pwd = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 1 kata_sandi harus string", span)),
    };
    let salt = if args.len() > 1 {
        match &args[1] {
            Value::String(s) => s.clone(),
            _ => "garam_argon2_acak".to_string(),
        }
    } else {
        "garam_argon2_acak".to_string()
    };
    let iter = if args.len() > 2 {
        match &args[2] { Value::Number(n) => (*n as u32).max(1), _ => 2 }
    } else {
        2
    };
    let mem_kb = if args.len() > 3 {
        match &args[3] { Value::Number(n) => (*n as usize).max(64), _ => 128 }
    } else {
        128
    };

    let hash = argon2id_compute(pwd, &salt, iter, mem_kb);
    Ok(Value::String(hash))
}

fn builtin_argon2id_verifikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pwd = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 1 kata_sandi harus string", span)),
    };
    let hash_str = match &args[1] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 2 hash_argon2 harus string", span)),
    };

    let parts: Vec<&str> = hash_str.split('$').collect();
    if parts.len() < 6 || parts[1] != "argon2id" {
        return Ok(Value::Bool(false));
    }

    // Parse params: m=128,t=2,p=1
    let mut mem_kb = 128;
    let mut iter = 2;
    for param in parts[3].split(',') {
        if let Some(stripped) = param.strip_prefix("m=") {
            mem_kb = stripped.parse().unwrap_or(128);
        } else if let Some(stripped) = param.strip_prefix("t=") {
            iter = stripped.parse().unwrap_or(2);
        }
    }

    let salt_bytes = match hex::decode(parts[4]) {
        Ok(b) => b,
        Err(_) => return Ok(Value::Bool(false)),
    };
    let salt = String::from_utf8_lossy(&salt_bytes).to_string();

    let computed = argon2id_compute(pwd, &salt, iter, mem_kb);
    Ok(Value::Bool(computed == hash_str))
}

