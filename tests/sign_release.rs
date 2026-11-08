use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use widya::desktop::{buat_tanda_tangan_ed25519, verifikasi_tanda_tangan_ed25519};
use widya::office::{DocumentConverter, MarkdownDocument};

fn compute_sha256(path: &Path) -> Option<String> {
    if !path.exists() {
        return None;
    }
    let data = fs::read(path).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    Some(hex::encode(hasher.finalize()))
}

#[test]
fn generate_documentation_pdfs_and_sign_official_release() {
    // 1. Render DOKUMENTASI_PILAR_KEMANDIRIAN.md ke PDF dengan mesin baru
    if Path::new("DOKUMENTASI_PILAR_KEMANDIRIAN.md").exists() {
        let md_str = fs::read_to_string("DOKUMENTASI_PILAR_KEMANDIRIAN.md").expect("Baca pilar md");
        let md = MarkdownDocument::parse(&md_str);
        let pdf = DocumentConverter::markdown_to_pdf(&md);
        let bytes = pdf.generate_pdf_bytes();
        fs::write("DOKUMENTASI_PILAR_KEMANDIRIAN.pdf", &bytes).unwrap();
        fs::write("dist/releases/v0.1.0/DOKUMENTASI_PILAR_KEMANDIRIAN.pdf", &bytes).unwrap();
        fs::write("www/downloads/DOKUMENTASI_PILAR_KEMANDIRIAN.pdf", &bytes).unwrap();
        println!("✅ Sukses menghasilkan DOKUMENTASI_PILAR_KEMANDIRIAN.pdf ({:.1} KB)", bytes.len() as f32 / 1024.0);

        // Render juga ke EPUB 3.2 buku digital berdaulat
        let epub = DocumentConverter::markdown_to_epub(&md, Some("Pilar Kemandirian Widya-Lang"), Some("Komite Kedaulatan Widya"));
        let epub_bytes = epub.export_to_epub_bytes();
        fs::write("DOKUMENTASI_PILAR_KEMANDIRIAN.epub", &epub_bytes).unwrap();
        fs::write("dist/releases/v0.1.0/DOKUMENTASI_PILAR_KEMANDIRIAN.epub", &epub_bytes).unwrap();
        fs::write("www/downloads/DOKUMENTASI_PILAR_KEMANDIRIAN.epub", &epub_bytes).unwrap();
        println!("✅ Sukses menghasilkan DOKUMENTASI_PILAR_KEMANDIRIAN.epub ({:.1} KB)", epub_bytes.len() as f32 / 1024.0);
    }

    // 2. Render DOKUMENTASI_WPM.md ke PDF & EPUB dengan mesin baru
    if Path::new("DOKUMENTASI_WPM.md").exists() {
        let md_str = fs::read_to_string("DOKUMENTASI_WPM.md").expect("Baca wpm md");
        let md = MarkdownDocument::parse(&md_str);
        let pdf = DocumentConverter::markdown_to_pdf(&md);
        let bytes = pdf.generate_pdf_bytes();
        fs::write("DOKUMENTASI_WPM.pdf", &bytes).unwrap();
        fs::write("dist/releases/v0.1.0/DOKUMENTASI_WPM.pdf", &bytes).unwrap();
        fs::write("www/downloads/DOKUMENTASI_WPM.pdf", &bytes).unwrap();
        println!("✅ Sukses menghasilkan DOKUMENTASI_WPM.pdf ({:.1} KB)", bytes.len() as f32 / 1024.0);

        let epub = DocumentConverter::markdown_to_epub(&md, Some("Panduan Widya Package Manager (WPM)"), Some("Tim Pengembang Widya"));
        let epub_bytes = epub.export_to_epub_bytes();
        fs::write("DOKUMENTASI_WPM.epub", &epub_bytes).unwrap();
        fs::write("dist/releases/v0.1.0/DOKUMENTASI_WPM.epub", &epub_bytes).unwrap();
        fs::write("www/downloads/DOKUMENTASI_WPM.epub", &epub_bytes).unwrap();
        println!("✅ Sukses menghasilkan DOKUMENTASI_WPM.epub ({:.1} KB)", epub_bytes.len() as f32 / 1024.0);
    }

    // 2.5 Perbarui SHA256SUMS.txt di dist/releases/v0.1.0 dan www/downloads
    let release_files = vec![
        "DOKUMENTASI_PILAR_KEMANDIRIAN.epub",
        "DOKUMENTASI_PILAR_KEMANDIRIAN.pdf",
        "DOKUMENTASI_WPM.epub",
        "DOKUMENTASI_WPM.pdf",
        "widya-lang-0.1.0.vsix",
        "widya-v0.1.0-x86_64-linux.tar.gz",
    ];

    let mut sums = String::new();
    for f in &release_files {
        let p = Path::new("dist/releases/v0.1.0").join(f);
        if let Some(hash) = compute_sha256(&p) {
            sums.push_str(&format!("{}  {}\n", hash, f));
        }
    }

    if !sums.is_empty() {
        fs::write("dist/releases/v0.1.0/SHA256SUMS.txt", &sums).unwrap();
        fs::write("www/downloads/SHA256SUMS.txt", &sums).unwrap();
        println!("✅ SHA256SUMS.txt v0.1.0 berhasil diperbarui secara deterministik.");
    }

    // 3. Tanda tangani Ed25519
    let targets = vec![
        ("dist/releases/v0.1.0/SHA256SUMS.txt", "dist/releases/v0.1.0/SHA256SUMS.sig", "dist/releases/v0.1.0/widya_vendor_pubkey.hex"),
        ("www/downloads/SHA256SUMS.txt", "www/downloads/SHA256SUMS.sig", "www/downloads/widya_vendor_pubkey.hex"),
    ];

    for (sha_path_str, sig_path_str, pub_path_str) in targets {
        let sha_path = Path::new(sha_path_str);
        if !sha_path.exists() {
            continue;
        }

        let sha_content = fs::read(sha_path).expect("Gagal membaca SHA256SUMS.txt");

        // Vendor Official Release Signing Key (32-byte Ed25519 Private Seed)
        let vendor_private_key = "a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90";

        let (sig_hex, pub_hex) = buat_tanda_tangan_ed25519(&sha_content, vendor_private_key)
            .expect("Penandatanganan Ed25519 gagal");

        let verified = verifikasi_tanda_tangan_ed25519(&sha_content, &sig_hex, &pub_hex)
            .expect("Verifikasi tanda tangan gagal");
        assert!(verified, "Tanda tangan harus valid");

        fs::write(sig_path_str, &sig_hex).expect("Gagal menulis SHA256SUMS.sig");
        fs::write(pub_path_str, &pub_hex).expect("Gagal menulis widya_vendor_pubkey.hex");

        println!("✅ Sukses menandatangani {} dengan Ed25519!", sha_path_str);
    }
}
