use widya::graphics::pure_apk::*;
use std::path::Path;

#[test]
fn test_android_binary_xml_generation() {
    let manifest = ApkManifest::baru("org.widya.silat.app", "ArenaSilatMobile")
        .dengan_versi(42, "2.6.0")
        .dengan_sdk(24, 34)
        .dengan_activity(".ArenaSilatMainActivity")
        .tambah_izin("android.permission.CAMERA")
        .tambah_izin("android.permission.BLUETOOTH");

    let axml_bytes = AxmlGenerator::bangun_axml(&manifest);

    // 1. Validasi AXML Magic Header (RES_XML_TYPE = 0x00080003)
    assert!(axml_bytes.len() >= 8);
    assert_eq!(&axml_bytes[0..4], &[0x03, 0x00, 0x08, 0x00]);

    // 2. Validasi Ukuran Berkas Total
    let file_size = u32::from_le_bytes([axml_bytes[4], axml_bytes[5], axml_bytes[6], axml_bytes[7]]) as usize;
    assert_eq!(file_size, axml_bytes.len());

    // 3. Validasi String Pool Chunk (0x0001)
    let pool_type = u16::from_le_bytes([axml_bytes[8], axml_bytes[9]]);
    assert_eq!(pool_type, 0x0001);

    // 4. Periksa keberadaan nama paket dan activity dalam format UTF-16LE
    let u16_pkg: Vec<u8> = "org.widya.silat.app".encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
    let contains_pkg = axml_bytes.windows(u16_pkg.len()).any(|w| w == u16_pkg.as_slice());
    assert!(contains_pkg, "AXML harus memuat nama paket dalam UTF-16");
}

#[test]
fn test_dalvik_dex_v035_generation() {
    let manifest = ApkManifest::baru("org.widya.mobile.app", "WidyaApp");
    let dex_bytes = DexGenerator::bangun_dex(&manifest);

    // 1. Validasi Magic Header (dex\n035\0)
    assert!(dex_bytes.len() >= 112);
    assert_eq!(&dex_bytes[0..8], b"dex\n035\0");

    // 2. Validasi Header Size = 112 (0x70)
    let header_size = u32::from_le_bytes([dex_bytes[36], dex_bytes[37], dex_bytes[38], dex_bytes[39]]);
    assert_eq!(header_size, 112);

    // 3. Validasi Endian Constant = 0x12345678
    let endian_tag = u32::from_le_bytes([dex_bytes[40], dex_bytes[41], dex_bytes[42], dex_bytes[43]]);
    assert_eq!(endian_tag, 0x12345678);

    // 4. Validasi Checksum Adler-32
    let chk_expected = DexGenerator::adler32(&dex_bytes[12..]);
    let chk_actual = u32::from_le_bytes([dex_bytes[8], dex_bytes[9], dex_bytes[10], dex_bytes[11]]);
    assert_eq!(chk_actual, chk_expected, "Checksum Adler-32 harus cocok");

    // 5. Periksa string class target ada di dalam DEX
    let dex_str = String::from_utf8_lossy(&dex_bytes);
    assert!(dex_str.contains("Lorg/widya/mobile/MainActivity;"));
    assert!(dex_str.contains("Landroid/app/Activity;"));
}

#[test]
fn test_resources_arsc_generation() {
    let manifest = ApkManifest::baru("org.widya.silat.app", "ArenaSilat");
    let arsc_bytes = ArscGenerator::bangun_arsc(&manifest);

    // 1. Validasi Magic Header RES_TABLE_TYPE (0x0002)
    assert!(arsc_bytes.len() >= 12);
    assert_eq!(&arsc_bytes[0..2], &[0x02, 0x00]);

    // 2. Validasi Total Chunk Size
    let chunk_size = u32::from_le_bytes([arsc_bytes[4], arsc_bytes[5], arsc_bytes[6], arsc_bytes[7]]) as usize;
    assert_eq!(chunk_size, arsc_bytes.len());

    // 3. Validasi Package Count = 1
    let pkg_count = u32::from_le_bytes([arsc_bytes[8], arsc_bytes[9], arsc_bytes[10], arsc_bytes[11]]);
    assert_eq!(pkg_count, 1);
}

#[test]
fn test_apk_v1_signature_generation() {
    let files = vec![
        ("AndroidManifest.xml", b"AXML_DATA" as &[u8]),
        ("classes.dex", b"DEX_DATA" as &[u8]),
    ];

    let (manifest_mf, cert_sf, cert_rsa) = ApkSigner::bangun_signatures(&files);

    let mf_str = String::from_utf8_lossy(&manifest_mf);
    assert!(mf_str.contains("Manifest-Version: 1.0"));
    assert!(mf_str.contains("Name: AndroidManifest.xml"));
    assert!(mf_str.contains("SHA-256-Digest:"));

    let sf_str = String::from_utf8_lossy(&cert_sf);
    assert!(sf_str.contains("Signature-Version: 1.0"));
    assert!(sf_str.contains("SHA-256-Digest-Manifest:"));

    assert!(cert_rsa.starts_with(&[0x30, 0x82, 0x01, 0x20]));
    assert!(cert_rsa.windows(b"WIDYA_SOVEREIGN_CERTIFICATE_BLOCK".len()).any(|w| w == b"WIDYA_SOVEREIGN_CERTIFICATE_BLOCK"));
}

#[test]
fn test_pure_apk_builder_and_zip_container() {
    let dummy_native_so = vec![0x7F, b'E', b'L', b'F', 0x02, 0x01, 0x01, 0x00]; // ELF64 header
    let manifest = ApkManifest::baru("org.widya.silat.app", "ArenaSilat")
        .dengan_versi(1, "2.6.0")
        .dengan_native_lib("arm64-v8a", &dummy_native_so);

    let apk_bytes = PureApkBuilder::bangun_apk(&manifest).expect("Harus sukses membuat APK");

    // 1. Validasi Magic ZIP PKWARE ("PK\x03\x04")
    assert!(apk_bytes.len() > 100);
    assert_eq!(&apk_bytes[0..4], b"PK\x03\x04");

    // 2. Verifikasi berkas-berkas penting ada dalam arsip APK
    let apk_str = String::from_utf8_lossy(&apk_bytes);
    assert!(apk_str.contains("AndroidManifest.xml"));
    assert!(apk_str.contains("classes.dex"));
    assert!(apk_str.contains("resources.arsc"));
    assert!(apk_str.contains("lib/arm64-v8a/libwidya_app.so"));
    assert!(apk_str.contains("META-INF/MANIFEST.MF"));
    assert!(apk_str.contains("META-INF/CERT.SF"));
    assert!(apk_str.contains("META-INF/CERT.RSA"));
}

#[test]
fn test_export_apk_file() {
    let manifest = ApkManifest::baru("org.widya.arena", "ArenaSilatApp")
        .dengan_versi(10, "2.6.0");

    let target_dir = Path::new("/tmp/widya_apk_export_test");
    let apk_path = PureApkBuilder::ekspor_berkas_apk(&manifest, target_dir).expect("Ekspor APK harus sukses");

    assert!(apk_path.exists());
    assert!(apk_path.to_string_lossy().ends_with(".apk"));
    let meta = std::fs::metadata(&apk_path).expect("Metadata terbaca");
    assert!(meta.len() > 0);

    let _ = std::fs::remove_dir_all(target_dir);
}
