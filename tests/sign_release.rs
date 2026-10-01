use std::fs;
use std::path::Path;
use widya::desktop::{buat_tanda_tangan_ed25519, verifikasi_tanda_tangan_ed25519};

#[test]
fn generate_official_release_signature() {
    let sha_path = Path::new("dist/releases/v1.0.0/SHA256SUMS.txt");
    if !sha_path.exists() {
        return;
    }

    let sha_content = fs::read(sha_path).expect("Gagal membaca SHA256SUMS.txt");

    // Vendor Official Release Signing Key (32-byte Ed25519 Private Seed)
    let vendor_private_key = "a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90";

    let (sig_hex, pub_hex) = buat_tanda_tangan_ed25519(&sha_content, vendor_private_key)
        .expect("Penandatanganan Ed25519 gagal");

    // Verifikasi validitas tanda tangan
    let verified = verifikasi_tanda_tangan_ed25519(&sha_content, &sig_hex, &pub_hex)
        .expect("Verifikasi tanda tangan gagal");
    assert!(verified, "Tanda tangan harus valid");

    // Simpan tanda tangan dan kunci publik
    fs::write("dist/releases/v1.0.0/SHA256SUMS.sig", &sig_hex).expect("Gagal menulis SHA256SUMS.sig");
    fs::write("dist/releases/v1.0.0/widya_vendor_pubkey.hex", &pub_hex).expect("Gagal menulis widya_vendor_pubkey.hex");

    println!("✅ Sukses menandatangani SHA256SUMS.txt dengan Ed25519!");
    println!("🔑 Kunci Publik Vendor : {}", pub_hex);
    println!("🔏 Tanda Tangan Digital: {}", sig_hex);
}
