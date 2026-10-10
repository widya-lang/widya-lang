// ============================================================================
// Integration Test: Pure Sovereign Post-Quantum Cryptography (NIST FIPS 203/204)
// ============================================================================
// Menguji ML-KEM-768 (Kyber) & ML-DSA-87 (Dilithium) serta ZKP
// murni dengan aritmatika polinomial Ring R_q tanpa pustaka eksternal.
// ============================================================================

use widya::kripto::{PqcEngine, PqcKeyPair, PqcSignature};

#[test]
fn test_mlkem768_key_encapsulation_mechanism() {
    let alice_keys = PqcEngine::generate_mlkem768_keypair("ENTROPI_ALICE_KYBER_768_SEED");
    assert!(alice_keys.public_key_hex.starts_with("mlkem_pk_"));
    assert!(alice_keys.private_key_hex.starts_with("mlkem_sk_"));

    // Bob melakukan enkapsulasi menggunakan kunci publik Alice
    let encap = PqcEngine::encapsulate_mlkem768(&alice_keys.public_key_hex)
        .expect("Enkapsulasi ML-KEM oleh Bob harus berhasil");
    assert!(encap.ciphertext_hex.starts_with("mlkem_ct_"));
    assert_eq!(encap.shared_secret_hex.len(), 64); // 32 bytes hex = 64 karakter

    // Alice melakukan dekapsulasi menggunakan kunci privatnya
    let alice_shared_secret = PqcEngine::decapsulate_mlkem768(&encap.ciphertext_hex, &alice_keys.private_key_hex)
        .expect("Dekapsulasi ML-KEM oleh Alice harus berhasil");

    // Kedua pihak harus memiliki kunci rahasia bersama yang 100% IDENTIK
    assert_eq!(alice_shared_secret, encap.shared_secret_hex);

    // Kunci privat yang berbeda TIDAK boleh menghasilkan shared secret yang sama
    let eve_keys = PqcEngine::generate_mlkem768_keypair("ENTROPI_EVE_ATTACKER_SEED");
    let eve_shared_secret = PqcEngine::decapsulate_mlkem768(&encap.ciphertext_hex, &eve_keys.private_key_hex)
        .expect("Dekapsulasi Eve selesai");
    assert_ne!(eve_shared_secret, encap.shared_secret_hex);
}

#[test]
fn test_mldsa87_lattice_digital_signatures() {
    let signer_keys = PqcEngine::generate_mldsa87_keypair("ENTROPI_SIGNER_DILITHIUM_SEED");
    let document = b"SURAT_KEPUTUSAN_KEDAULATAN_DIGITAL_REPUBLIK_INDONESIA_2026";

    // 1. Tanda tangani dokumen dengan kunci privat
    let sig = PqcEngine::sign_mldsa(document, &signer_keys.private_key_hex);
    assert!(sig.signature_hex.starts_with("mldsa_sig_"));

    // 2. Verifikasi keabsahan dengan kunci publik
    let valid = PqcEngine::verify_mldsa(document, &sig, &signer_keys.public_key_hex);
    assert!(valid, "Tanda tangan asli harus valid terverifikasi");

    // 3. Verifikasi dokumen yang dimanipulasi (Tamper Detection) HARUS GAGAL
    let tampered_document = b"SURAT_KEPUTUSAN_PALSU_REPUBLIK_INDONESIA_2026";
    let tampered_valid = PqcEngine::verify_mldsa(tampered_document, &sig, &signer_keys.public_key_hex);
    assert!(!tampered_valid, "Dokumen yang diubah HARUS gagal diverifikasi!");

    // 4. Verifikasi dengan kunci publik orang lain HARUS GAGAL
    let other_keys = PqcEngine::generate_mldsa87_keypair("ENTROPI_ORANG_LAIN_SEED");
    let other_valid = PqcEngine::verify_mldsa(document, &sig, &other_keys.public_key_hex);
    assert!(!other_valid, "Kunci publik yang salah HARUS ditolak!");

    // 5. Tanda tangan yang korup HARUS GAGAL
    let mut corrupted_sig = sig.clone();
    corrupted_sig.signature_hex = "mldsa_sig_0000000000000000".to_string();
    assert!(!PqcEngine::verify_mldsa(document, &corrupted_sig, &signer_keys.public_key_hex));
}

#[test]
fn test_zero_knowledge_proof_identity() {
    let umur_asli = 22u64;
    let syarat_min = 17u64;
    let blinding = "FAKTOR_BLINDING_KEDAULATAN_8899";

    // Buktikan umur >= 17 tanpa membocorkan angka 22
    let zkp = PqcEngine::prove_zk_threshold("usia_ktp", umur_asli, syarat_min, blinding)
        .expect("Pembuatan ZKP harus berhasil");

    assert_eq!(zkp.attribute_name, "usia_ktp");
    assert_eq!(zkp.threshold, 17);
    assert!(PqcEngine::verify_zk_proof(&zkp), "Verifikator harus menerima bukti ZKP");

    // Jika nilai tidak memenuhi syarat, harus error
    let umur_anak = 14u64;
    let gagal = PqcEngine::prove_zk_threshold("usia_ktp", umur_anak, syarat_min, blinding);
    assert!(gagal.is_err());
}
