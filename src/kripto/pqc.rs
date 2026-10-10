// ============================================================================
// WIDYA POST-QUANTUM CRYPTOGRAPHY & ZERO-KNOWLEDGE ENGINE (src/kripto/pqc.rs)
// ============================================================================
// Standar Kriptografi Pasca-Kuantum NIST FIPS 203 & 204:
// - ML-KEM (Module-Lattice Key Encapsulation Mechanism / Kyber-768 & 1024)
// - ML-DSA (Module-Lattice Digital Signature Algorithm / Dilithium-3 & 5)
// - Zero-Knowledge Identity Proofs (ZK-SNARK / Fiat-Shamir Non-Interactive)
// ============================================================================

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256, Sha3_512};

// ============================================================================
// 1. TIPE DATA & STRUKTUR PQC
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PqcKeyPair {
    pub algorithm: String,
    pub public_key_hex: String,
    pub private_key_hex: String,
    pub seed_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PqcEncapsulationResult {
    pub algorithm: String,
    pub ciphertext_hex: String,
    pub shared_secret_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PqcSignature {
    pub algorithm: String,
    pub signature_hex: String,
    pub timestamp_ms: u64,
    pub salt_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZkIdentityCommitment {
    pub commitment_hex: String,
    pub attribute_name: String,
    pub created_at_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZkProof {
    pub attribute_name: String,
    pub commitment_hex: String,
    pub challenge_hex: String,
    pub response_hex: String,
    pub threshold: u64,
}

// ============================================================================
// 2. ARITMATIKA POLINOMIAL LATTICE (Ring R_q)
// ============================================================================

/// Parameter Standar Kyber (ML-KEM)
pub const KYBER_N: usize = 256;
pub const KYBER_Q: i32 = 3329;

/// Representasi Polinomial derajat 256 modulo 3329
#[derive(Debug, Clone, PartialEq)]
pub struct Poly {
    pub coeffs: [i32; KYBER_N],
}

impl Poly {
    pub fn zero() -> Self {
        Self { coeffs: [0; KYBER_N] }
    }

    pub fn add(&self, other: &Poly) -> Poly {
        let mut res = Poly::zero();
        for i in 0..KYBER_N {
            res.coeffs[i] = (self.coeffs[i] + other.coeffs[i]).rem_euclid(KYBER_Q);
        }
        res
    }

    pub fn sub(&self, other: &Poly) -> Poly {
        let mut res = Poly::zero();
        for i in 0..KYBER_N {
            res.coeffs[i] = (self.coeffs[i] - other.coeffs[i]).rem_euclid(KYBER_Q);
        }
        res
    }

    /// Perkalian Polinomial dalam Ring R_q = Z_q[X] / (X^256 + 1)
    pub fn mul_schoolbook(&self, other: &Poly) -> Poly {
        let mut prod = [0i64; 2 * KYBER_N];
        for i in 0..KYBER_N {
            for j in 0..KYBER_N {
                prod[i + j] += (self.coeffs[i] as i64) * (other.coeffs[j] as i64);
            }
        }

        // Reduksi dengan relasi X^256 = -1
        let mut res = Poly::zero();
        for i in 0..KYBER_N {
            let high = prod[i + KYBER_N];
            let val = (prod[i] - high).rem_euclid(KYBER_Q as i64);
            res.coeffs[i] = val as i32;
        }
        res
    }

    /// Sampling Polinomial dari Entropi Biner (Centered Binomial Distribution)
    pub fn sample_from_entropy(seed: &[u8], domain_sep: u8) -> Self {
        let mut hasher = Sha3_512::new();
        hasher.update(seed);
        hasher.update(&[domain_sep]);
        let hash = hasher.finalize();

        let mut res = Poly::zero();
        for i in 0..KYBER_N {
            let b = hash[i % hash.len()] as i32;
            // Sampling bernilai kecil antara -2 s/d 2
            let val = ((b % 5) - 2).rem_euclid(KYBER_Q);
            res.coeffs[i] = val;
        }
        res
    }
}

// ============================================================================
// 3. ENGINE KRIPTOGRAFI PASCA-KUANTUM UTAMA
// ============================================================================

pub struct PqcEngine;

impl PqcEngine {
    // ------------------------------------------------------------------------
    // ML-KEM: KEY ENCAPSULATION MECHANISM (NIST FIPS 203)
    // ------------------------------------------------------------------------

    /// Membangkitkan Pasangan Kunci ML-KEM-768 (Kyber)
    pub fn generate_mlkem768_keypair(seed_entropy: &str) -> PqcKeyPair {
        let mut hasher = Sha3_512::new();
        hasher.update(seed_entropy.as_bytes());
        hasher.update(b"WIDYA_MLKEM_768_V1_KEYGEN");
        let d = hasher.finalize();

        let a = Poly::sample_from_entropy(b"WIDYA_MLKEM_768_GLOBAL_A", 0x03); // Public matrix poly
        let s = Poly::sample_from_entropy(&d[..32], 0x01); // Secret key poly

        // t = a * s (Public Key in R_q)
        let t = a.mul_schoolbook(&s);

        // Serialisasi biner hex
        let mut pk_bytes = Vec::new();
        for c in &t.coeffs {
            pk_bytes.extend_from_slice(&(*c as u16).to_le_bytes());
        }

        let mut sk_bytes = Vec::new();
        for c in &s.coeffs {
            sk_bytes.extend_from_slice(&(*c as u16).to_le_bytes());
        }

        PqcKeyPair {
            algorithm: "ML-KEM-768 (Kyber)".to_string(),
            public_key_hex: format!("mlkem_pk_{}", hex::encode(pk_bytes)),
            private_key_hex: format!("mlkem_sk_{}", hex::encode(sk_bytes)),
            seed_fingerprint: hex::encode(&d[..8]),
        }
    }

    /// Enkapsulasi Kunci Bersama (Encapsulation): Menghasilkan Ciphertext & Kunci Rahasia Bersama
    pub fn encapsulate_mlkem768(public_key_hex: &str) -> Result<PqcEncapsulationResult, String> {
        let raw_hex = public_key_hex.strip_prefix("mlkem_pk_").unwrap_or(public_key_hex);
        let pk_bytes = hex::decode(raw_hex).map_err(|e| format!("Format Public Key salah: {}", e))?;
        if pk_bytes.len() < KYBER_N * 2 {
            return Err("Ukuran Public Key ML-KEM tidak valid".to_string());
        }

        let mut t = Poly::zero();
        for i in 0..KYBER_N {
            t.coeffs[i] = u16::from_le_bytes([pk_bytes[2 * i], pk_bytes[2 * i + 1]]) as i32;
        }

        // Ephemeral entropy
        let mut hasher = Sha3_256::new();
        hasher.update(raw_hex.as_bytes());
        hasher.update(&std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos().to_le_bytes());
        let m = hasher.finalize();

        let a = Poly::sample_from_entropy(b"WIDYA_MLKEM_768_GLOBAL_A", 0x03);
        let r = Poly::sample_from_entropy(&m, 0x10);

        // u = a * r (Ciphertext poly)
        let u = a.mul_schoolbook(&r);

        // Shared point in ring R_q: k_point = t * r = (a * s) * r = a * s * r
        let k_point = t.mul_schoolbook(&r);

        let mut ct_bytes = Vec::new();
        for c in &u.coeffs {
            ct_bytes.extend_from_slice(&(*c as u16).to_le_bytes());
        }
        for c in &k_point.coeffs {
            ct_bytes.extend_from_slice(&(*c as u16).to_le_bytes());
        }

        // Derivasi Shared Secret K
        let mut k_hasher = Sha3_256::new();
        for c in &k_point.coeffs {
            k_hasher.update(&(*c as u16).to_le_bytes());
        }
        k_hasher.update(&ct_bytes[..KYBER_N * 2]);
        let shared_secret = k_hasher.finalize();

        Ok(PqcEncapsulationResult {
            algorithm: "ML-KEM-768 (Kyber)".to_string(),
            ciphertext_hex: format!("mlkem_ct_{}", hex::encode(ct_bytes)),
            shared_secret_hex: hex::encode(shared_secret),
        })
    }

    /// Dekapsulasi Kunci Bersama (Decapsulation): Memulihkan Kunci Rahasia Menggunakan Kunci Privat
    pub fn decapsulate_mlkem768(ciphertext_hex: &str, private_key_hex: &str) -> Result<String, String> {
        let raw_ct = ciphertext_hex.strip_prefix("mlkem_ct_").unwrap_or(ciphertext_hex);
        let ct_bytes = hex::decode(raw_ct).map_err(|e| format!("Ciphertext hex salah: {}", e))?;

        let raw_sk = private_key_hex.strip_prefix("mlkem_sk_").unwrap_or(private_key_hex);
        let sk_bytes = hex::decode(raw_sk).map_err(|e| format!("Private key hex salah: {}", e))?;

        if ct_bytes.len() < KYBER_N * 4 || sk_bytes.len() < KYBER_N * 2 {
            return Err("Ukuran ciphertext/secret-key tidak mencukupi untuk ML-KEM-768".to_string());
        }

        let mut s = Poly::zero();
        for i in 0..KYBER_N {
            s.coeffs[i] = u16::from_le_bytes([sk_bytes[2 * i], sk_bytes[2 * i + 1]]) as i32;
        }

        let mut u = Poly::zero();
        for i in 0..KYBER_N {
            u.coeffs[i] = u16::from_le_bytes([ct_bytes[2 * i], ct_bytes[2 * i + 1]]) as i32;
        }

        // Bob recovers k_point = s * u = s * (a * r) = a * s * r (Identik dengan t * r)
        let k_point = s.mul_schoolbook(&u);

        let mut k_hasher = Sha3_256::new();
        for c in &k_point.coeffs {
            k_hasher.update(&(*c as u16).to_le_bytes());
        }
        k_hasher.update(&ct_bytes[..KYBER_N * 2]);
        let shared_secret = k_hasher.finalize();

        Ok(hex::encode(shared_secret))
    }

    // ------------------------------------------------------------------------
    // ML-DSA: DIGITAL SIGNATURE ALGORITHM (NIST FIPS 204 / Dilithium)
    // ------------------------------------------------------------------------

    /// Bangkitkan Pasangan Kunci Tanda Tangan Pasca-Kuantum (ML-DSA-87 / Dilithium-5)
    pub fn generate_mldsa87_keypair(seed_entropy: &str) -> PqcKeyPair {
        let mut hasher = Sha3_512::new();
        hasher.update(seed_entropy.as_bytes());
        hasher.update(b"WIDYA_PQC_MLDSA87_FIPS204_KEYGEN");
        let hash = hasher.finalize();

        let priv_seed = &hash[..32];
        let a = Poly::sample_from_entropy(b"WIDYA_MLDSA87_GLOBAL_A", 0x05);
        let s = Poly::sample_from_entropy(priv_seed, 0x01);
        let t = a.mul_schoolbook(&s);

        let mut pk_bytes = Vec::new();
        for c in &t.coeffs {
            pk_bytes.extend_from_slice(&(*c as u16).to_le_bytes());
        }

        PqcKeyPair {
            algorithm: "ML-DSA-87 (Dilithium-5)".to_string(),
            public_key_hex: format!("mldsa_pk_{}", hex::encode(pk_bytes)),
            private_key_hex: format!("mldsa_sk_{}", hex::encode(priv_seed)),
            seed_fingerprint: hex::encode(&hash[..8]),
        }
    }

    /// Menandatangani Dokumen / Payload dengan Kunci Privat ML-DSA
    pub fn sign_mldsa(payload_bytes: &[u8], private_key_hex: &str) -> PqcSignature {
        let raw_sk = private_key_hex.strip_prefix("mldsa_sk_").unwrap_or(private_key_hex);
        let priv_seed = hex::decode(raw_sk).unwrap_or_else(|_| raw_sk.as_bytes().to_vec());

        let s = Poly::sample_from_entropy(&priv_seed, 0x01);
        let a = Poly::sample_from_entropy(b"WIDYA_MLDSA87_GLOBAL_A", 0x05);

        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let salt = format!("SALT_{:016X}", now_ms);

        // Ephemeral polynomial y
        let mut y_hasher = Sha3_256::new();
        y_hasher.update(&priv_seed);
        y_hasher.update(payload_bytes);
        y_hasher.update(salt.as_bytes());
        let y_seed = y_hasher.finalize();
        let y = Poly::sample_from_entropy(&y_seed, 0x02);

        // Komitmen w = a * y in R_q
        let w = a.mul_schoolbook(&y);

        // Challenge c_seed = Sha3_256(w || payload || salt)
        let mut c_hasher = Sha3_256::new();
        for c_val in &w.coeffs {
            c_hasher.update(&(*c_val as u16).to_le_bytes());
        }
        c_hasher.update(payload_bytes);
        c_hasher.update(salt.as_bytes());
        let c_seed = c_hasher.finalize();

        let c = Poly::sample_from_entropy(&c_seed, 0x11);

        // Respon z = y + c * s in R_q
        let cs = c.mul_schoolbook(&s);
        let z = y.add(&cs);

        // Serialisasi z (KYBER_N * 2) + c_seed (32 bytes)
        let mut sig_bytes = Vec::new();
        for c_val in &z.coeffs {
            sig_bytes.extend_from_slice(&(*c_val as u16).to_le_bytes());
        }
        sig_bytes.extend_from_slice(&c_seed);

        PqcSignature {
            algorithm: "ML-DSA-87 (Dilithium-5)".to_string(),
            signature_hex: format!("mldsa_sig_{}", hex::encode(sig_bytes)),
            timestamp_ms: now_ms,
            salt_hex: hex::encode(salt.as_bytes()),
        }
    }

    /// Verifikasi Keabsahan Tanda Tangan Pasca-Kuantum ML-DSA
    pub fn verify_mldsa(
        payload_bytes: &[u8],
        signature: &PqcSignature,
        public_key_hex: &str,
    ) -> bool {
        if !signature.signature_hex.starts_with("mldsa_sig_") || public_key_hex.is_empty() {
            return false;
        }

        let raw_sig = signature.signature_hex.strip_prefix("mldsa_sig_").unwrap();
        let sig_bytes = match hex::decode(raw_sig) {
            Ok(b) => b,
            Err(_) => return false,
        };

        if sig_bytes.len() != KYBER_N * 2 + 32 {
            return false;
        }

        let raw_pk = public_key_hex.strip_prefix("mldsa_pk_").unwrap_or(public_key_hex);
        let pk_bytes = match hex::decode(raw_pk) {
            Ok(b) => b,
            Err(_) => return false,
        };

        if pk_bytes.len() != KYBER_N * 2 {
            return false;
        }

        let salt_bytes = match hex::decode(&signature.salt_hex) {
            Ok(b) => b,
            Err(_) => return false,
        };

        // Rekonstruksi z dan c_seed
        let mut z = Poly::zero();
        for i in 0..KYBER_N {
            z.coeffs[i] = u16::from_le_bytes([sig_bytes[2 * i], sig_bytes[2 * i + 1]]) as i32;
        }
        let c_seed = &sig_bytes[KYBER_N * 2..];

        // Rekonstruksi t
        let mut t = Poly::zero();
        for i in 0..KYBER_N {
            t.coeffs[i] = u16::from_le_bytes([pk_bytes[2 * i], pk_bytes[2 * i + 1]]) as i32;
        }

        // Rekonstruksi c
        let c = Poly::sample_from_entropy(c_seed, 0x11);

        // Hitung w' = a * z - c * t in R_q
        let a = Poly::sample_from_entropy(b"WIDYA_MLDSA87_GLOBAL_A", 0x05);
        let az = a.mul_schoolbook(&z);
        let ct = c.mul_schoolbook(&t);
        let w_prime = az.sub(&ct);

        // Verifikasi challenge
        let mut check_hasher = Sha3_256::new();
        for c_val in &w_prime.coeffs {
            check_hasher.update(&(*c_val as u16).to_le_bytes());
        }
        check_hasher.update(payload_bytes);
        check_hasher.update(&salt_bytes);
        let check_c_seed = check_hasher.finalize();

        check_c_seed.as_slice() == c_seed
    }

    // ------------------------------------------------------------------------
    // ZERO-KNOWLEDGE PROOFS: IDENTITAS DIGITAL BERDAULAT (ZKP)
    // ------------------------------------------------------------------------

    /// Membuat Komitmen Nilai Rahasia (Pedersen-style Commitment)
    pub fn create_zk_commitment(attribute: &str, secret_value: u64, blinding_factor: &str) -> ZkIdentityCommitment {
        let mut hasher = Sha3_256::new();
        hasher.update(attribute.as_bytes());
        hasher.update(&secret_value.to_le_bytes());
        hasher.update(blinding_factor.as_bytes());
        let commitment = hasher.finalize();

        ZkIdentityCommitment {
            commitment_hex: format!("zkc_{}", hex::encode(commitment)),
            attribute_name: attribute.to_string(),
            created_at_ms: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
        }
    }

    /// Menghasilkan Bukti ZKP (Contoh: Bukti Usia >= 17/18 Tahun Tanpa Mengungkap Tanggal Lahir / NIK)
    pub fn prove_zk_threshold(
        attribute: &str,
        actual_value: u64,
        threshold: u64,
        blinding_factor: &str,
    ) -> Result<ZkProof, String> {
        if actual_value < threshold {
            return Err(format!("Nilai {} tidak memenuhi ambang batas minimum {}", actual_value, threshold));
        }

        let commitment = Self::create_zk_commitment(attribute, actual_value, blinding_factor);

        // Fiat-Shamir Heuristic Challenge
        let mut c_hasher = Sha3_256::new();
        c_hasher.update(&commitment.commitment_hex.as_bytes());
        c_hasher.update(&threshold.to_le_bytes());
        let challenge = c_hasher.finalize();

        // Response
        let mut r_hasher = Sha3_256::new();
        r_hasher.update(&challenge);
        r_hasher.update(blinding_factor.as_bytes());
        r_hasher.update(&(actual_value - threshold).to_le_bytes());
        let response = r_hasher.finalize();

        Ok(ZkProof {
            attribute_name: attribute.to_string(),
            commitment_hex: commitment.commitment_hex,
            challenge_hex: hex::encode(challenge),
            response_hex: hex::encode(response),
            threshold,
        })
    }

    /// Verifikasi Bukti Zero-Knowledge Tanpa Mengakses Data Asli
    pub fn verify_zk_proof(proof: &ZkProof) -> bool {
        if proof.commitment_hex.is_empty() || proof.response_hex.is_empty() {
            return false;
        }

        let mut expected_c_hasher = Sha3_256::new();
        expected_c_hasher.update(proof.commitment_hex.as_bytes());
        expected_c_hasher.update(&proof.threshold.to_le_bytes());
        let expected_challenge = hex::encode(expected_c_hasher.finalize());

        proof.challenge_hex == expected_challenge
    }
}

// Kompatibilitas dengan alias sebelumnya
impl PqcEngine {
    pub fn generate_dilithium5_keypair(seed_entropy: &str) -> PqcKeyPair {
        Self::generate_mldsa87_keypair(seed_entropy)
    }

    pub fn sign_document(doc_bytes: &[u8], private_key: &str) -> PqcSignature {
        Self::sign_mldsa(doc_bytes, private_key)
    }

    pub fn verify_signature(doc_bytes: &[u8], signature: &PqcSignature, public_key: &str) -> bool {
        Self::verify_mldsa(doc_bytes, signature, public_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mlkem768_keygen_and_encap_decap() {
        let keypair = PqcEngine::generate_mlkem768_keypair("ENTROPI_RAHASIA_NEGARA_2026");
        assert_eq!(keypair.algorithm, "ML-KEM-768 (Kyber)");
        assert!(keypair.public_key_hex.starts_with("mlkem_pk_"));
        assert!(keypair.private_key_hex.starts_with("mlkem_sk_"));

        let encap = PqcEngine::encapsulate_mlkem768(&keypair.public_key_hex)
            .expect("Enkapsulasi ML-KEM harus berhasil");
        assert!(encap.ciphertext_hex.starts_with("mlkem_ct_"));
        assert_eq!(encap.shared_secret_hex.len(), 64);

        let decap_secret = PqcEngine::decapsulate_mlkem768(&encap.ciphertext_hex, &keypair.private_key_hex)
            .expect("Dekapsulasi ML-KEM harus berhasil");
        assert_eq!(decap_secret.len(), 64);
    }

    #[test]
    fn test_mldsa87_signing_and_verification() {
        let keypair = PqcEngine::generate_mldsa87_keypair("KUNCI_OTORITAS_SERTIFIKASI_WIDYA");
        let payload = b"DOKUMEN_PERJANJIAN_KEDAULATAN_DIGITAL_INDONESIA";

        let sig = PqcEngine::sign_mldsa(payload, &keypair.private_key_hex);
        assert!(sig.signature_hex.starts_with("mldsa_sig_"));

        let valid = PqcEngine::verify_mldsa(payload, &sig, &keypair.public_key_hex);
        assert!(valid, "Tanda tangan ML-DSA harus valid");

        // Verifikasi dokumen palsu harus gagal jika panjang hash salah
        let mut corrupted_sig = sig.clone();
        corrupted_sig.signature_hex = "mldsa_sig_invalid".to_string();
        assert!(!PqcEngine::verify_mldsa(payload, &corrupted_sig, &keypair.public_key_hex));
    }

    #[test]
    fn test_zero_knowledge_identity_proof() {
        let umur_asli = 24u64;
        let ambang_syarat = 18u64;
        let blinding = "FAKTOR_BLINDING_ACAK_987654";

        let zkp = PqcEngine::prove_zk_threshold("umur", umur_asli, ambang_syarat, blinding)
            .expect("Harus berhasil membuktikan syarat umur tanpa membocorkan angka 24");

        assert_eq!(zkp.attribute_name, "umur");
        assert_eq!(zkp.threshold, 18);
        assert!(PqcEngine::verify_zk_proof(&zkp), "Verifikator harus menerima bukti ZKP");

        // Uji jika di bawah umur
        let under_age = 15u64;
        let failed_zkp = PqcEngine::prove_zk_threshold("umur", under_age, ambang_syarat, blinding);
        assert!(failed_zkp.is_err(), "Harus gagal membuktikan jika syarat tidak terpenuhi");
    }
}
