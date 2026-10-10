// ============================================================================
// Integration Test: Pure Sovereign Bootable Live ISO & UEFI Generator
// ============================================================================
// Menguji ISO 9660 (ECMA-119), El Torito v1.0, Hybrid MBR, dan UEFI ESP FAT12
// tanpa xorriso / genisoimage / grub-mkrescue.
// ============================================================================

use std::fs;
use std::path::PathBuf;
use widya::os::iso::{
    both_endian_u16, both_endian_u32, ElToritoBuilder, HybridMbrBuilder,
    LiveIsoConfig, PureLiveIsoBuilder, UefiEspBuilder,
    ISO_SECTOR_SIZE, LBA_BOOT_CATALOG, LBA_DATA_START, LBA_EL_TORITO,
    LBA_PVD, LBA_ROOT_DIR, LBA_TERMINATOR,
};

#[test]
fn test_both_endian_encoding() {
    // u16: 0x1234 -> LE [0x34, 0x12], BE [0x12, 0x34]
    let enc16 = both_endian_u16(0x1234);
    assert_eq!(enc16, [0x34, 0x12, 0x12, 0x34]);

    // u32: 0x12345678 -> LE [0x78, 0x56, 0x34, 0x12], BE [0x12, 0x34, 0x56, 0x78]
    let enc32 = both_endian_u32(0x12345678);
    assert_eq!(enc32, [0x78, 0x56, 0x34, 0x12, 0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn test_hybrid_mbr_structure() {
    let total_sectors = 100u32;
    let efi_lba = Some(30u32);
    let efi_sectors = Some(720u32);

    let mbr = HybridMbrBuilder::bangun_mbr(total_sectors, efi_lba, efi_sectors);
    assert_eq!(mbr.len(), 512);

    // Boot signature di byte 510-511 harus 0x55, 0xAA
    assert_eq!(mbr[510], 0x55);
    assert_eq!(mbr[511], 0xAA);

    // Partisi 1: ISO 9660 Image (offset 446)
    let p1 = &mbr[446..462];
    assert_eq!(p1[0], 0x80, "Partisi 1 harus active/bootable");
    assert_eq!(p1[4], 0x17, "Tipe partisi 1 harus 0x17 (ISO 9660 / IFS)");

    // Partisi 2: EFI System Partition (offset 462)
    let p2 = &mbr[462..478];
    assert_eq!(p2[0], 0x00, "Partisi 2 non-bootable MBR");
    assert_eq!(p2[4], 0xEF, "Tipe partisi 2 harus 0xEF (EFI System Partition)");

    let p2_start_lba = u32::from_le_bytes(p2[8..12].try_into().unwrap());
    assert_eq!(p2_start_lba, 30 * 4, "LBA 512-byte harus sesuai dengan sektor ISO 2048-byte * 4");
}

#[test]
fn test_uefi_esp_fat12_image() {
    let dummy_loader = b"MZ\x90\x00\x03\x00\x00\x00Pure WidyaOS UEFI BOOTX64.EFI Stub";
    let esp = UefiEspBuilder::bangun_esp_fat_image(dummy_loader);

    // Ukuran ESP adalah 64 KB (128 sektor 512 byte = 32 sektor ISO 2048 byte)
    assert_eq!(esp.len(), 64 * 1024);

    // Boot sector signature
    assert_eq!(esp[510], 0x55);
    assert_eq!(esp[511], 0xAA);

    // BPB (BIOS Parameter Block) checks
    let bytes_per_sector = u16::from_le_bytes([esp[11], esp[12]]);
    assert_eq!(bytes_per_sector, 512);
    let sectors_per_cluster = esp[13];
    assert_eq!(sectors_per_cluster, 1);
    let reserved_sectors = u16::from_le_bytes([esp[14], esp[15]]);
    assert_eq!(reserved_sectors, 1);
    let num_fats = esp[16];
    assert_eq!(num_fats, 2);

    // Periksa bahwa entri berkas BOOTX64 EFI ada di root dir
    let esp_str = String::from_utf8_lossy(&esp);
    assert!(esp_str.contains("BOOTX64"), "ESP FAT12 harus memuat BOOTX64");
    assert!(esp_str.contains("EFI"), "ESP FAT12 harus memuat ekstensi EFI");
}

#[test]
fn test_el_torito_boot_catalog_validation() {
    let kernel_lba = 21;
    let kernel_sectors = 4;
    let efi_lba = Some(30);
    let efi_sectors = Some(720);

    let catalog = ElToritoBuilder::bangun_boot_catalog(
        kernel_lba,
        kernel_sectors,
        efi_lba,
        efi_sectors,
    );
    assert_eq!(catalog.len(), ISO_SECTOR_SIZE);

    // Entry 0: Validation Entry (32 bytes)
    assert_eq!(catalog[0], 0x01, "Header ID harus 0x01");
    assert_eq!(catalog[1], 0x00, "Platform ID x86 BIOS");
    assert_eq!(catalog[0x1E], 0x55, "Key byte 0x55");
    assert_eq!(catalog[0x1F], 0xAA, "Key byte 0xAA");

    // Checksum verification: Sum of sixteen 16-bit words must equal 0 (mod 65536)
    let mut sum: u32 = 0;
    for i in (0..32).step_by(2) {
        let word = u16::from_le_bytes([catalog[i], catalog[i + 1]]) as u32;
        sum = (sum + word) & 0xFFFF;
    }
    assert_eq!(sum, 0, "El Torito Validation Checksum harus 0 (mod 0x10000)");

    // Entry 1: Initial/Default Boot Entry (32 bytes, offset 32)
    let init_entry = &catalog[32..64];
    assert_eq!(init_entry[0], 0x88, "Boot Indicator: 0x88 (Bootable)");
    assert_eq!(init_entry[1], 0x00, "Boot Media: 0x00 (No Emulation)");
    let load_rba = u32::from_le_bytes(init_entry[8..12].try_into().unwrap());
    assert_eq!(load_rba, kernel_lba, "Load RBA harus menunjuk ke kernel LBA");

    // Section Header untuk UEFI (offset 64..96)
    let uefi_hdr = &catalog[64..96];
    assert_eq!(uefi_hdr[0], 0x91, "Section Header Terminating (0x91)");
    assert_eq!(uefi_hdr[1], 0xEF, "Platform ID EFI (0xEF)");

    // Section Entry untuk UEFI (offset 96..128)
    let uefi_entry = &catalog[96..128];
    assert_eq!(uefi_entry[0], 0x88, "UEFI Boot Indicator: 0x88");
    let uefi_rba = u32::from_le_bytes(uefi_entry[8..12].try_into().unwrap());
    assert_eq!(uefi_rba, 30, "Load RBA UEFI harus menunjuk ke EFI LBA");
}

#[test]
fn test_live_iso_generation_and_descriptors() {
    let dummy_kernel = b"\x7FELF\x02\x01\x01\x00WidyaOS Sovereign Microkernel Baremetal Payload v2.6.0";
    let dummy_initrd = b"CPIO_RAMFS_ARCHIVE_MANDIRI_SOVEREIGN_ROOT_FS";

    let config = LiveIsoConfig::baru("WIDYAOS_TEST_ISO")
        .dengan_kernel(dummy_kernel)
        .dengan_initrd(dummy_initrd)
        .dengan_cmdline("console=tty0 root=/dev/ram0 rw quiet")
        .dengan_uefi(true)
        .dengan_hybrid_mbr(true);

    let iso_bytes = PureLiveIsoBuilder::bangun_live_iso(&config)
        .expect("Pembangkitan Live ISO harus berhasil");

    // 1. Ukuran ISO harus kelipatan sektor 2048 byte
    assert_eq!(iso_bytes.len() % ISO_SECTOR_SIZE, 0);
    assert!(iso_bytes.len() >= (LBA_DATA_START as usize + 32) * ISO_SECTOR_SIZE);

    // 2. Sektor 0: Hybrid MBR
    assert_eq!(iso_bytes[510], 0x55);
    assert_eq!(iso_bytes[511], 0xAA);

    // 3. Sektor 16: Primary Volume Descriptor (PVD)
    let pvd_offset = (LBA_PVD as usize) * ISO_SECTOR_SIZE;
    assert_eq!(iso_bytes[pvd_offset], 0x01, "Tipe deskriptor PVD = 0x01");
    assert_eq!(&iso_bytes[pvd_offset + 1..pvd_offset + 6], b"CD001", "Standard ID = CD001");
    assert_eq!(iso_bytes[pvd_offset + 6], 0x01, "PVD Version = 0x01");

    // Volume ID verification
    let vol_id_slice = &iso_bytes[pvd_offset + 40..pvd_offset + 72];
    let vol_id_str = String::from_utf8_lossy(vol_id_slice);
    assert!(vol_id_str.starts_with("WIDYAOS_TEST_ISO"));

    // 4. Sektor 17: Boot Record Descriptor (El Torito)
    let br_offset = (LBA_EL_TORITO as usize) * ISO_SECTOR_SIZE;
    assert_eq!(iso_bytes[br_offset], 0x00, "Tipe deskriptor Boot Record = 0x00");
    assert_eq!(&iso_bytes[br_offset + 1..br_offset + 6], b"CD001");
    assert!(iso_bytes[br_offset + 7..br_offset + 39].starts_with(b"EL TORITO SPECIFICATION"));
    let cat_lba = u32::from_le_bytes(iso_bytes[br_offset + 0x47..br_offset + 0x4B].try_into().unwrap());
    assert_eq!(cat_lba, LBA_BOOT_CATALOG);

    // 5. Sektor 18: Volume Descriptor Set Terminator
    let term_offset = (LBA_TERMINATOR as usize) * ISO_SECTOR_SIZE;
    assert_eq!(iso_bytes[term_offset], 0xFF, "Terminator type = 0xFF");
    assert_eq!(&iso_bytes[term_offset + 1..term_offset + 6], b"CD001");

    // 6. Sektor 20: Root Directory Table
    let root_offset = (LBA_ROOT_DIR as usize) * ISO_SECTOR_SIZE;
    // Entri '.' dan '..'
    let dot_len = iso_bytes[root_offset] as usize;
    assert!(dot_len >= 34);
    assert_eq!(iso_bytes[root_offset + 33], 0x00, "Identifier '.' adalah 0x00");

    let dotdot_offset = root_offset + dot_len;
    let dotdot_len = iso_bytes[dotdot_offset] as usize;
    assert!(dotdot_len >= 34);
    assert_eq!(iso_bytes[dotdot_offset + 33], 0x01, "Identifier '..' adalah 0x01");

    // 7. Verifikasi Keberadaan Berkas KERNEL.BIN, INITRD.IMG, EFI.IMG, BOOT.CFG
    let iso_slice_str = String::from_utf8_lossy(&iso_bytes[root_offset..root_offset + ISO_SECTOR_SIZE]);
    assert!(iso_slice_str.contains("KERNEL.BIN;1"));
    assert!(iso_slice_str.contains("INITRD.IMG;1"));
    assert!(iso_slice_str.contains("EFI.IMG;1"));
    assert!(iso_slice_str.contains("BOOT.CFG;1"));
}

#[test]
fn test_iso_export_to_file() {
    let dummy_kernel = b"WIDYA_MICROKERNEL_ELF64_TEST_PAYLOAD";
    let config = LiveIsoConfig::baru("WIDYAOS_EXPORT_TEST")
        .dengan_kernel(dummy_kernel)
        .dengan_uefi(false) // Uji tanpa UEFI agar ukuran file lebih ramping
        .dengan_hybrid_mbr(true);

    let temp_dir = std::env::temp_dir().join("widya_iso_test_suite");
    let target_iso = temp_dir.join("widya_test_export.iso");

    let exported_path = PureLiveIsoBuilder::ekspor_live_iso(&config, &target_iso)
        .expect("Ekspor ISO fisik harus berhasil");

    assert_eq!(exported_path, target_iso);
    assert!(target_iso.exists());

    let metadata = fs::metadata(&target_iso).unwrap();
    assert!(metadata.len() > 0);
    assert_eq!(metadata.len() % ISO_SECTOR_SIZE as u64, 0);

    // Pembersihan berkas uji
    let _ = fs::remove_file(&target_iso);
    let _ = fs::remove_dir(&temp_dir);
}
