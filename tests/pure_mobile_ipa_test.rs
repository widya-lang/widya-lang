// ============================================================================
// Integration Test: Pure Sovereign iOS IPA Bundler & Mach-O ARM64 Emitter
// ============================================================================
// Menguji struktur direktori Payload/App.app, Mach-O 64-bit ARM64, Info.plist,
// _CodeSignature/CodeResources, embedded.mobileprovision, dan ekspor .ipa
// 100% mandiri tanpa macOS atau Xcode.
// ============================================================================

use std::fs;
use widya::graphics::{
    CodeSignatureBuilder, InfoPlistBuilder, IpaConfig, MachO64Builder,
    ProvisioningProfileBuilder, PureIpaBuilder, CPU_TYPE_ARM64, LC_MAIN,
    LC_SEGMENT_64, MH_EXECUTE, MH_MAGIC_64,
};

#[test]
fn test_macho_64_arm64_header_and_load_commands() {
    let arm64_macho = MachO64Builder::bangun_arm64_executable(&[]);

    // 1. Verifikasi panjang minimal Mach-O
    assert!(arm64_macho.len() >= 32 + 72 + 24);

    // 2. Mach-O 64-bit Header (32 bytes)
    let magic = u32::from_le_bytes(arm64_macho[0..4].try_into().unwrap());
    assert_eq!(magic, MH_MAGIC_64, "Mach-O magic harus 0xFEEDFACF");

    let cputype = u32::from_le_bytes(arm64_macho[4..8].try_into().unwrap());
    assert_eq!(cputype, CPU_TYPE_ARM64, "CPU type harus ARM64 (0x0100000C)");

    let filetype = u32::from_le_bytes(arm64_macho[12..16].try_into().unwrap());
    assert_eq!(filetype, MH_EXECUTE, "File type harus MH_EXECUTE (2)");

    let ncmds = u32::from_le_bytes(arm64_macho[16..20].try_into().unwrap());
    assert_eq!(ncmds, 2, "Jumlah load commands harus 2 (LC_SEGMENT_64 + LC_MAIN)");

    // 3. Load Command 1: LC_SEGMENT_64 (__TEXT) pada offset 32
    let cmd1 = u32::from_le_bytes(arm64_macho[32..36].try_into().unwrap());
    assert_eq!(cmd1, LC_SEGMENT_64);
    let segname = &arm64_macho[40..46];
    assert_eq!(segname, b"__TEXT");

    // 4. Load Command 2: LC_MAIN pada offset 32 + 72 = 104
    let cmd2 = u32::from_le_bytes(arm64_macho[104..108].try_into().unwrap());
    assert_eq!(cmd2, LC_MAIN);

    // 5. Verifikasi instruksi ARM64 (mov x0, #0; ret)
    let code_offset = 32 + 72 + 24;
    assert_eq!(&arm64_macho[code_offset..code_offset + 4], &[0x00, 0x00, 0x80, 0xD2]);
    assert_eq!(&arm64_macho[code_offset + 4..code_offset + 8], &[0xC0, 0x03, 0x5F, 0xD6]);
}

#[test]
fn test_apple_info_plist_xml_and_binary() {
    let config = IpaConfig::baru("id.widya.mobile.demo", "WidyaDemo")
        .dengan_display_name("Widya Demo App")
        .dengan_versi("2.6.0", "42");

    // 1. XML Info.plist
    let xml = InfoPlistBuilder::bangun_xml_plist(&config);
    assert!(xml.contains("<key>CFBundleIdentifier</key>"));
    assert!(xml.contains("<string>id.widya.mobile.demo</string>"));
    assert!(xml.contains("<key>CFBundleDisplayName</key>"));
    assert!(xml.contains("<string>Widya Demo App</string>"));
    assert!(xml.contains("<key>CFBundleShortVersionString</key>"));
    assert!(xml.contains("<string>2.6.0</string>"));
    assert!(xml.contains("<key>CFBundleVersion</key>"));
    assert!(xml.contains("<string>42</string>"));
    assert!(xml.contains("<string>arm64</string>"));

    // 2. Binary Plist (bplist00)
    let bplist = InfoPlistBuilder::bangun_binary_plist(&config);
    assert!(bplist.len() > 40);
    assert_eq!(&bplist[0..8], b"bplist00");
}

#[test]
fn test_code_signature_code_resources() {
    let dummy_info = b"INFO_PLIST_RAW_DATA";
    let dummy_asset = b"PNG_IMAGE_SAMPLE_BYTES";

    let resources_xml = CodeSignatureBuilder::bangun_code_resources(&[
        ("Info.plist", dummy_info),
        ("AppIcon.png", dummy_asset),
    ]);

    assert!(resources_xml.contains("<key>Info.plist</key>"));
    assert!(resources_xml.contains("<key>AppIcon.png</key>"));
    assert!(resources_xml.contains("<key>hash2</key>"));
}

#[test]
fn test_embedded_mobileprovision() {
    let config = IpaConfig::baru("id.widya.sovereign", "WidyaSovereign")
        .dengan_team("Nusantara Digital Sovereign");

    let prov_xml = ProvisioningProfileBuilder::bangun_mobileprovision(&config);
    assert!(prov_xml.contains("<key>AppIDName</key>"));
    assert!(prov_xml.contains("<string>WidyaSovereign</string>"));
    assert!(prov_xml.contains("<key>TeamName</key>"));
    assert!(prov_xml.contains("<string>Nusantara Digital Sovereign</string>"));
    assert!(prov_xml.contains("<key>application-identifier</key>"));
}

#[test]
fn test_full_ipa_generation_and_zip_container() {
    let icon_data = b"\x89PNG\r\n\x1a\nPure Widya iOS Icon PNG Bytes";
    let config = IpaConfig::baru("id.widya.app", "WidyaSovereign")
        .dengan_display_name("Widya Sovereign iOS")
        .dengan_versi("2.6.0", "1")
        .tambah_asset("AppIcon.png", icon_data);

    let ipa_bytes = PureIpaBuilder::bangun_ipa(&config)
        .expect("Pembangkitan paket IPA harus berhasil");

    // 1. Verifikasi Magic Header ZIP: PK\x03\x04
    assert!(ipa_bytes.len() >= 100);
    assert_eq!(&ipa_bytes[0..4], &[0x50, 0x4B, 0x03, 0x04]);

    // 2. Verifikasi keberadaan berkas-berkas iOS penting di dalam arsip ZIP
    let ipa_str = String::from_utf8_lossy(&ipa_bytes);
    assert!(ipa_str.contains("Payload/WidyaSovereign.app/WidyaSovereign"));
    assert!(ipa_str.contains("Payload/WidyaSovereign.app/Info.plist"));
    assert!(ipa_str.contains("Payload/WidyaSovereign.app/embedded.mobileprovision"));
    assert!(ipa_str.contains("Payload/WidyaSovereign.app/_CodeSignature/CodeResources"));
    assert!(ipa_str.contains("Payload/WidyaSovereign.app/AppIcon.png"));
}

#[test]
fn test_export_ipa_file() {
    let config = IpaConfig::baru("id.widya.export", "ExportTestApp");
    let temp_dir = std::env::temp_dir().join("widya_ipa_export_test");
    let target_ipa = temp_dir.join("ExportTestApp.ipa");

    let out_path = PureIpaBuilder::ekspor_ipa(&config, &target_ipa)
        .expect("Ekspor IPA ke berkas harus berhasil");

    assert_eq!(out_path, target_ipa);
    assert!(target_ipa.exists());

    let meta = fs::metadata(&target_ipa).unwrap();
    assert!(meta.len() > 0);

    // Pembersihan
    let _ = fs::remove_file(&target_ipa);
    let _ = fs::remove_dir(&temp_dir);
}
