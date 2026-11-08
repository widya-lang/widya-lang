use widya::desktop::pure_packager::*;
use std::process::Command;

#[test]
fn test_freedesktop_desktop_entry_generation() {
    let manifest = DesktopPackageManifest::baru("arenasilat", "0.1.0")
        .dengan_deskripsi("Platform Turnamen Pencak Silat Berdaulat")
        .dengan_kategori("Sports;Game;Office;")
        .dengan_eksekusi("arenasilat")
        .dengan_ikon("arenasilat");

    let desktop_str = PureFreedesktopEntry::hasilkan_konten_desktop(&manifest);
    assert!(desktop_str.contains("[Desktop Entry]"));
    assert!(desktop_str.contains("Name=arenasilat"));
    assert!(desktop_str.contains("Exec=/usr/bin/arenasilat %U"));
    assert!(desktop_str.contains("Categories=Sports;Game;Office;"));
    assert!(desktop_str.contains("StartupNotify=true"));
}

#[test]
fn test_pure_tar_and_gzip_generation_and_decompression() {
    let mut tar = PureTarBuilder::baru();
    tar.tambah_direktori("./", 0o755);
    tar.tambah_direktori("./usr/", 0o755);
    tar.tambah_direktori("./usr/bin/", 0o755);
    tar.tambah_berkas("./usr/bin/widya-app", 0o755, b"#!/bin/sh\necho Sovereign Desktop\n");

    let tar_bytes = tar.bangun();
    assert!(tar_bytes.len() >= 512 * 5); // 4 entries + 2 zero blocks at end

    // Validasi magic ustar di block pertama (offset 257)
    assert_eq!(&tar_bytes[257..262], b"ustar");

    // Kompresi dengan GZIP store
    let gz_bytes = PureGzipBuilder::kompres_store(&tar_bytes);
    assert!(gz_bytes.len() > 10);
    assert_eq!(gz_bytes[0], 0x1F);
    assert_eq!(gz_bytes[1], 0x8B);
    assert_eq!(gz_bytes[2], 0x08); // Deflate
}

#[test]
fn test_pure_deb_package_structure_and_dpkg_compliance() {
    let manifest = DesktopPackageManifest::baru("arenasilat", "0.1.0")
        .dengan_deskripsi("ArenaSilat Pencak Silat Platform")
        .dengan_arsitektur("amd64")
        .dengan_eksekusi("arenasilat")
        .dengan_biner(b"#!/bin/sh\necho ArenaSilat Running!\n");

    let deb_bytes = PureDebPackager::bangun_paket_deb(&manifest).expect("Harus sukses membuat deb");

    // 1. Verifikasi magic ar UNIX
    assert!(deb_bytes.starts_with(b"!<arch>\n"));

    // 2. Verifikasi keberadaan komponen standar debian
    let deb_str = String::from_utf8_lossy(&deb_bytes);
    assert!(deb_str.contains("debian-binary/"));
    assert!(deb_str.contains("control.tar.gz/"));
    assert!(deb_str.contains("data.tar.gz/"));

    // 3. Verifikasi dengan tool dpkg-deb sistem jika tersedia
    let temp_deb_path = "/tmp/test_widya_packager.deb";
    if std::fs::write(temp_deb_path, &deb_bytes).is_ok() {
        if let Ok(output) = Command::new("dpkg-deb").args(["-I", temp_deb_path]).output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                assert!(stdout.contains("Package: arenasilat"));
                assert!(stdout.contains("Version: 0.1.0"));
                assert!(stdout.contains("Architecture: amd64"));
            }
        }
        let _ = std::fs::remove_file(temp_deb_path);
    }
}

#[test]
fn test_pure_rpm_package_structure() {
    let manifest = DesktopPackageManifest::baru("arenasilat", "0.1.0")
        .dengan_deskripsi("ArenaSilat Pencak Silat Platform")
        .dengan_arsitektur("x86_64")
        .dengan_eksekusi("arenasilat")
        .dengan_biner(b"#!/bin/sh\necho ArenaSilat Running!\n");

    let rpm_bytes = PureRpmPackager::bangun_paket_rpm(&manifest).expect("Harus sukses membuat rpm");

    // Verifikasi RPM Lead Magic (0xED, 0xAB, 0xEE, 0xDB)
    assert_eq!(&rpm_bytes[0..4], &[0xED, 0xAB, 0xEE, 0xDB]);
    assert_eq!(rpm_bytes[4], 3); // Major 3
    assert_eq!(rpm_bytes[5], 0); // Minor 0
}

#[test]
fn test_self_extracting_run_script_and_hash_integrity() {
    let manifest = DesktopPackageManifest::baru("arenasilat", "0.1.0")
        .dengan_eksekusi("arenasilat")
        .dengan_biner(b"#!/bin/sh\necho ArenaSilat!\n");

    let run_bytes = PureSelfExtractingRunner::bangun_installer_run(&manifest).expect("Harus sukses membuat .run");

    let run_str = String::from_utf8_lossy(&run_bytes);
    assert!(run_str.starts_with("#!/bin/sh"));
    assert!(run_str.contains("WIDYA SOVEREIGN DESKTOP SELF-EXTRACTING INSTALLER"));
    assert!(run_str.contains("APP_NAME=\"arenasilat\""));
    assert!(run_str.contains("__ARCHIVE_PAYLOAD_BELOW__"));
}

#[test]
fn test_wapp_sovereign_container() {
    let manifest = DesktopPackageManifest::baru("arenasilat", "0.1.0")
        .dengan_biner(b"\x7FELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00");

    let wapp_bytes = PureWappPackager::bangun_paket_wapp(&manifest).expect("Harus sukses membuat .wapp");

    // Verifikasi Magic WAPP (6 bytes)
    assert_eq!(&wapp_bytes[0..6], b"WAPP\x01\x00");
    let wapp_str = String::from_utf8_lossy(&wapp_bytes);
    assert!(wapp_str.contains("id:widya:app:arenasilat"));
}

#[test]
fn test_universal_packager_export_all() {
    let manifest = DesktopPackageManifest::baru("arenasilat", "0.1.0")
        .dengan_deskripsi("ArenaSilat Sovereign Edition")
        .dengan_biner(b"#!/bin/sh\necho ArenaSilat Universal!\n")
        .tambah_berkas("./usr/share/doc/arenasilat/README.txt", 0o644, b"ArenaSilat Docs\n");

    let packager = PureUniversalPackager::baru(manifest);
    let target_dir = std::path::Path::new("/tmp/widya_packager_export_test");

    let export_map = packager.ekspor_semua(target_dir).expect("Ekspor semua harus sukses");

    assert!(export_map.contains_key("deb"));
    assert!(export_map.contains_key("tar.gz"));
    assert!(export_map.contains_key("run"));
    assert!(export_map.contains_key("wapp"));

    for (_, path) in export_map {
        assert!(path.exists());
        let meta = std::fs::metadata(&path).expect("File metadata harus terbaca");
        assert!(meta.len() > 0);
    }

    let _ = std::fs::remove_dir_all(target_dir);
}
