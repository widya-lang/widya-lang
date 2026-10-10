// ============================================================================
// Integration Test: Pure Sovereign Windows PE32+ (64-bit .exe) Direct Emitter
// ============================================================================
// Menguji struktur MS-DOS Header, PE Signature, COFF Header (AMD64),
// PE32+ Optional Header, dan Section Headers (.text & .data) tanpa MSVC / MinGW.
// ============================================================================

use std::fs;
use widya::compiler::{
    PeConfig, PurePeBuilder, DEFAULT_FILE_ALIGNMENT, DEFAULT_IMAGE_BASE,
    DEFAULT_SECTION_ALIGNMENT, DOS_MAGIC, IMAGE_FILE_MACHINE_AMD64,
    IMAGE_SUBSYSTEM_WINDOWS_CUI, IMAGE_SUBSYSTEM_WINDOWS_GUI, PE32_PLUS_MAGIC,
    PE_SIGNATURE,
};

#[test]
fn test_dos_header_and_pe_signature() {
    let config = PeConfig::baru_console();
    let exe_bytes = PurePeBuilder::bangun_exe(&config);

    // 1. Verifikasi ukuran minimal biner PE
    assert!(exe_bytes.len() >= 512);

    // 2. MS-DOS Header: "MZ"
    assert_eq!(&exe_bytes[0..2], &DOS_MAGIC);

    // 3. e_lfanew pada offset 0x3C menunjuk ke 128 (0x80)
    let e_lfanew = u32::from_le_bytes(exe_bytes[0x3C..0x40].try_into().unwrap());
    assert_eq!(e_lfanew, 0x80);

    // 4. PE Signature pada offset 0x80: "PE\0\0"
    let pe_sig_offset = e_lfanew as usize;
    assert_eq!(&exe_bytes[pe_sig_offset..pe_sig_offset + 4], &PE_SIGNATURE);
}

#[test]
fn test_coff_and_optional_header_64() {
    let config = PeConfig::baru_console();
    let exe_bytes = PurePeBuilder::bangun_exe(&config);

    // COFF Header mulai setelah PE Signature (offset 0x84)
    let coff_offset = 0x84;
    let machine = u16::from_le_bytes(exe_bytes[coff_offset..coff_offset + 2].try_into().unwrap());
    assert_eq!(machine, IMAGE_FILE_MACHINE_AMD64, "Machine harus x86_64 / AMD64");

    // Optional Header mulai pada offset 0x84 + 20 = 0x98
    let opt_offset = coff_offset + 20;
    let magic = u16::from_le_bytes(exe_bytes[opt_offset..opt_offset + 2].try_into().unwrap());
    assert_eq!(magic, PE32_PLUS_MAGIC, "Optional header magic harus PE32+ (0x020B)");

    // ImageBase (offset opt_offset + 24)
    let image_base = u64::from_le_bytes(exe_bytes[opt_offset + 24..opt_offset + 32].try_into().unwrap());
    assert_eq!(image_base, DEFAULT_IMAGE_BASE);

    // Section Alignment (offset opt_offset + 32)
    let sec_align = u32::from_le_bytes(exe_bytes[opt_offset + 32..opt_offset + 36].try_into().unwrap());
    assert_eq!(sec_align, DEFAULT_SECTION_ALIGNMENT);

    // File Alignment (offset opt_offset + 36)
    let file_align = u32::from_le_bytes(exe_bytes[opt_offset + 36..opt_offset + 40].try_into().unwrap());
    assert_eq!(file_align, DEFAULT_FILE_ALIGNMENT);

    // Subsystem Console (offset opt_offset + 68)
    let subsystem = u16::from_le_bytes(exe_bytes[opt_offset + 68..opt_offset + 70].try_into().unwrap());
    assert_eq!(subsystem, IMAGE_SUBSYSTEM_WINDOWS_CUI);

    // Uji versi GUI
    let gui_config = PeConfig::baru_gui();
    let gui_exe = PurePeBuilder::bangun_exe(&gui_config);
    let gui_subsystem = u16::from_le_bytes(gui_exe[opt_offset + 68..opt_offset + 70].try_into().unwrap());
    assert_eq!(gui_subsystem, IMAGE_SUBSYSTEM_WINDOWS_GUI);
}

#[test]
fn test_sections_text_and_data() {
    let custom_code = vec![
        0x48, 0x31, 0xC0, // xor rax, rax
        0x48, 0xFF, 0xC0, // inc rax
        0xC3,             // ret
    ];
    let custom_data = b"WIDYA_SOVEREIGN_WINDOWS_DATA_STRING\0";

    let config = PeConfig::baru_console()
        .dengan_kode(&custom_code)
        .dengan_data(custom_data);

    let exe_bytes = PurePeBuilder::bangun_exe(&config);

    // Seluruh berkas harus memiliki ukuran yang selaras dengan FileAlignment (512 bytes)
    assert_eq!(exe_bytes.len() % DEFAULT_FILE_ALIGNMENT as usize, 0);

    // Periksa keberadaan header section .text dan .data
    let exe_str = String::from_utf8_lossy(&exe_bytes);
    assert!(exe_str.contains(".text"));
    assert!(exe_str.contains(".data"));
    assert!(exe_str.contains("WIDYA_SOVEREIGN_WINDOWS_DATA_STRING"));
}

#[test]
fn test_export_exe_file() {
    let config = PeConfig::baru_console();
    let temp_dir = std::env::temp_dir().join("widya_pe_export_test");
    let target_exe = temp_dir.join("aplikasi_mandiri.exe");

    let out_path = PurePeBuilder::ekspor_exe(&config, &target_exe)
        .expect("Ekspor berkas EXE harus berhasil");

    assert_eq!(out_path, target_exe);
    assert!(target_exe.exists());

    let meta = fs::metadata(&target_exe).unwrap();
    assert!(meta.len() >= 512);
    assert_eq!(meta.len() % 512, 0);

    // Pembersihan
    let _ = fs::remove_file(&target_exe);
    let _ = fs::remove_dir(&temp_dir);
}
