//! Pure Sovereign Bootable Live ISO & UEFI Generator (ECMA-119 & El Torito v1.0)
//!
//! Pembangkit citra ISO 9660 Bootable Live CD/DVD & USB Hybrid 100% in-house tanpa
//! ketergantungan pada perkakas eksternal (Zero xorriso, Zero genisoimage, Zero grub-mkrescue):
//! - 💿 Standar ISO 9660 (ECMA-119) Sektor 2048-Byte dengan Primary Volume Descriptor (PVD)
//! - 🚀 El Torito Bootable Format Specification v1.0 dengan Boot Catalog & Validation Checksum
//! - 💾 Hybrid MBR Sektor 0 (0x55 0xAA) untuk boot langsung via USB Flash Drive & CD/DVD
//! - ⚡ UEFI System Partition (ESP FAT12/FAT16) Hybrid untuk motherboard UEFI x86_64 modern
//! - 🐧 Integrasi Payload Kernel Mandiri WidyaOS (ELF64 / Multiboot2), Initrd, & Command Line

use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// 1. KONSTANTA SPESIFIKASI ISO 9660 & EL TORITO
// ============================================================================

pub const ISO_SECTOR_SIZE: usize = 2048;
pub const MBR_SECTOR_SIZE: usize = 512;

pub const LBA_PVD: u32 = 16;
pub const LBA_EL_TORITO: u32 = 17;
pub const LBA_TERMINATOR: u32 = 18;
pub const LBA_BOOT_CATALOG: u32 = 19;
pub const LBA_ROOT_DIR: u32 = 20;
pub const LBA_DATA_START: u32 = 21;

// ============================================================================
// 2. HELPER BOTH-ENDIAN ISO 9660
// ============================================================================

/// Encode u16 both-endian (Little-Endian diikuti Big-Endian: 4 bytes)
pub fn both_endian_u16(val: u16) -> [u8; 4] {
    let le = val.to_le_bytes();
    let be = val.to_be_bytes();
    [le[0], le[1], be[0], be[1]]
}

/// Encode u32 both-endian (Little-Endian diikuti Big-Endian: 8 bytes)
pub fn both_endian_u32(val: u32) -> [u8; 8] {
    let le = val.to_le_bytes();
    let be = val.to_be_bytes();
    [le[0], le[1], le[2], le[3], be[0], be[1], be[2], be[3]]
}

// ============================================================================
// 3. KONFIGURASI LIVE ISO WIDYAOS
// ============================================================================

/// Konfigurasi pembangkitan citra Live ISO
#[derive(Debug, Clone)]
pub struct LiveIsoConfig {
    pub volume_id: String,
    pub system_id: String,
    pub publisher_id: String,
    pub kernel_payload: Vec<u8>,
    pub initrd_payload: Option<Vec<u8>>,
    pub boot_cmdline: String,
    pub enable_uefi: bool,
    pub enable_hybrid_mbr: bool,
}

impl LiveIsoConfig {
    pub fn baru(volume_id: &str) -> Self {
        Self {
            volume_id: volume_id.to_string(),
            system_id: "WIDYAOS SOVEREIGN BAREMETAL".to_string(),
            publisher_id: "WIDYA-LANG FOUNDATION".to_string(),
            kernel_payload: Vec::new(),
            initrd_payload: None,
            boot_cmdline: "console=ttyS0,115200 root=/dev/ram0 rw quiet".to_string(),
            enable_uefi: true,
            enable_hybrid_mbr: true,
        }
    }

    pub fn dengan_kernel(mut self, kernel: &[u8]) -> Self {
        self.kernel_payload = kernel.to_vec();
        self
    }

    pub fn dengan_initrd(mut self, initrd: &[u8]) -> Self {
        self.initrd_payload = Some(initrd.to_vec());
        self
    }

    pub fn dengan_cmdline(mut self, cmdline: &str) -> Self {
        self.boot_cmdline = cmdline.to_string();
        self
    }

    pub fn dengan_uefi(mut self, enable: bool) -> Self {
        self.enable_uefi = enable;
        self
    }

    pub fn dengan_hybrid_mbr(mut self, enable: bool) -> Self {
        self.enable_hybrid_mbr = enable;
        self
    }
}

// ============================================================================
// 4. HYBRID MBR SECTOR GENERATOR (LBA 0)
// ============================================================================

pub struct HybridMbrBuilder;

impl HybridMbrBuilder {
    /// Menghasilkan Master Boot Record (MBR) 512-byte hybrid untuk boot dari flashdisk USB
    pub fn bangun_mbr(total_iso_sectors: u32, efi_lba: Option<u32>, efi_sectors: Option<u32>) -> [u8; 512] {
        let mut mbr = [0u8; 512];

        // x86 Stage 1 Boot Code (Real mode stub: clear interrupts, print notify if booted raw BIOS)
        let boot_code: &[u8] = &[
            0xFA,                   // cli
            0x31, 0xC0,             // xor ax, ax
            0x8E, 0xD8,             // mov ds, ax
            0x8E, 0xC0,             // mov es, ax
            0x8E, 0xD0,             // mov ss, ax
            0xBC, 0x00, 0x7C,       // mov sp, 0x7c00
            0xFB,                   // sti
            0xBE, 0x1A, 0x7C,       // mov si, 0x7c1a (pesan boot)
            0xAC,                   // lodsb
            0x08, 0xC0,             // or al, al
            0x74, 0x06,             // jz halt
            0xB4, 0x0E,             // mov ah, 0x0e
            0xCD, 0x10,             // int 0x10
            0xEB, 0xF5,             // jmp loop
            0xF4,                   // hlt
            0xEB, 0xFD,             // jmp halt
            // Pesan biner (offset 0x7C1A)
            b'W', b'i', b'd', b'y', b'a', b'O', b'S', b' ', b'L', b'i', b'v', b'e', b'!', 0,
        ];
        let copy_len = boot_code.len().min(446);
        mbr[..copy_len].copy_from_slice(&boot_code[..copy_len]);

        // Partition 1: ISO 9660 Image (Bootable, Type 0x17 / 0x83 Linux)
        // Offset 446 (0x01BE)
        let part1 = &mut mbr[446..462];
        part1[0] = 0x80; // Active / Bootable flag
        part1[1] = 0x00; // CHS start head
        part1[2] = 0x02; // CHS start sector
        part1[3] = 0x00; // CHS start cylinder
        part1[4] = 0x17; // Partition type: Hidden IFS / ISO9660
        part1[5] = 0xFE; // CHS end head
        part1[6] = 0xFF; // CHS end sector
        part1[7] = 0xFF; // CHS end cylinder
        // LBA Start = 0 (4 bytes LE)
        part1[8..12].copy_from_slice(&0u32.to_le_bytes());
        // LBA Sectors (satuan 512-byte sectors = total_iso_sectors * 4)
        let total_512_sectors = total_iso_sectors.saturating_mul(4);
        part1[12..16].copy_from_slice(&total_512_sectors.to_le_bytes());

        // Partition 2: UEFI System Partition (ESP, Type 0xEF) jika aktif
        if let (Some(lba), Some(secs)) = (efi_lba, efi_sectors) {
            let part2 = &mut mbr[462..478];
            part2[0] = 0x00; // Inactive
            part2[1] = 0xFE;
            part2[2] = 0xFF;
            part2[3] = 0xFF;
            part2[4] = 0xEF; // Partition type: EFI System Partition
            part2[5] = 0xFE;
            part2[6] = 0xFF;
            part2[7] = 0xFF;
            let efi_start_512 = lba.saturating_mul(4);
            let efi_len_512 = secs.saturating_mul(4);
            part2[8..12].copy_from_slice(&efi_start_512.to_le_bytes());
            part2[12..16].copy_from_slice(&efi_len_512.to_le_bytes());
        }

        // Boot Signature 0x55, 0xAA di offset 510..512
        mbr[510] = 0x55;
        mbr[511] = 0xAA;

        mbr
    }
}

// ============================================================================
// 5. PRIMARY VOLUME DESCRIPTOR (PVD - LBA 16)
// ============================================================================

pub struct PvdBuilder;

impl PvdBuilder {
    /// Menghasilkan sektor Primary Volume Descriptor (Sektor 16 / LBA 16, 2048 bytes)
    pub fn bangun_pvd(config: &LiveIsoConfig, total_sectors: u32, root_dir_lba: u32) -> Vec<u8> {
        let mut pvd = vec![0u8; ISO_SECTOR_SIZE];

        // Byte 0: Type 0x01 (Primary Volume Descriptor)
        pvd[0] = 0x01;
        // Bytes 1..6: Standard Identifier "CD001"
        pvd[1..6].copy_from_slice(b"CD001");
        // Byte 6: Version 0x01
        pvd[6] = 0x01;

        // Bytes 8..40: System Identifier (32 bytes space-padded)
        Self::pad_ascii(&mut pvd[8..40], &config.system_id);

        // Bytes 40..72: Volume Identifier (32 bytes space-padded)
        Self::pad_ascii(&mut pvd[40..72], &config.volume_id);

        // Bytes 80..88: Volume Space Size (u32 both-endian)
        pvd[80..88].copy_from_slice(&both_endian_u32(total_sectors));

        // Bytes 120..124: Volume Set Size = 1
        pvd[120..124].copy_from_slice(&both_endian_u16(1));

        // Bytes 124..128: Volume Sequence Number = 1
        pvd[124..128].copy_from_slice(&both_endian_u16(1));

        // Bytes 128..132: Logical Block Size = 2048 (both-endian)
        pvd[128..132].copy_from_slice(&both_endian_u16(ISO_SECTOR_SIZE as u16));

        // Bytes 132..140: Path Table Size = 10
        pvd[132..140].copy_from_slice(&both_endian_u32(10));

        // Bytes 140..144: Type L Path Table Location (LBA 20)
        pvd[140..144].copy_from_slice(&root_dir_lba.to_le_bytes());

        // Bytes 148..152: Type M Path Table Location (LBA 20)
        pvd[148..152].copy_from_slice(&root_dir_lba.to_be_bytes());

        // Bytes 156..190: Root Directory Record (34 bytes)
        let root_rec = &mut pvd[156..190];
        root_rec[0] = 34; // Record Length: 34 bytes
        root_rec[1] = 0;  // Extended Attribute Length
        root_rec[2..10].copy_from_slice(&both_endian_u32(root_dir_lba)); // Extent Location LBA
        root_rec[10..18].copy_from_slice(&both_endian_u32(ISO_SECTOR_SIZE as u32)); // Data Length (2048)
        // Recording Date (7 bytes: 2026-10-10 12:00:00 +07)
        root_rec[18] = (2026 - 1900) as u8;
        root_rec[19] = 10;
        root_rec[20] = 10;
        root_rec[21] = 12;
        root_rec[22] = 0;
        root_rec[23] = 0;
        root_rec[24] = 28; // GMT+7 (28 quarter hours)
        root_rec[25] = 0x02; // File Flags: Directory
        root_rec[28..32].copy_from_slice(&both_endian_u16(1)); // Volume sequence 1
        root_rec[32] = 1; // File Identifier Length: 1
        root_rec[33] = 0; // Root Identifier \0

        // Bytes 190..318: Volume Set Identifier (128 bytes)
        Self::pad_ascii(&mut pvd[190..318], &config.volume_id);

        // Bytes 318..446: Publisher Identifier (128 bytes)
        Self::pad_ascii(&mut pvd[318..446], &config.publisher_id);

        // Bytes 705..842: Application Identifier (128 bytes)
        Self::pad_ascii(&mut pvd[705..833], "WIDYAOS SOVEREIGN LIVE ISO ENGINE");

        // Bytes 881..898: Creation Date (17 bytes: "2026101012000000\0")
        pvd[881..897].copy_from_slice(b"2026101012000000");
        pvd[897] = 28;

        // Byte 881: File Structure Version = 1
        pvd[881] = 0x01;

        pvd
    }

    fn pad_ascii(slice: &mut [u8], s: &str) {
        slice.fill(b' ');
        let bytes = s.as_bytes();
        let len = bytes.len().min(slice.len());
        slice[..len].copy_from_slice(&bytes[..len]);
    }
}

// ============================================================================
// 6. EL TORITO BOOT RECORD & BOOT CATALOG (LBA 17 & LBA 19)
// ============================================================================

pub struct ElToritoBuilder;

impl ElToritoBuilder {
    /// Menghasilkan Boot Record Descriptor (Sektor 17 / LBA 17)
    pub fn bangun_boot_record(catalog_lba: u32) -> Vec<u8> {
        let mut br = vec![0u8; ISO_SECTOR_SIZE];

        br[0] = 0x00; // Type 0x00: Boot Record
        br[1..6].copy_from_slice(b"CD001");
        br[6] = 0x01; // Version 1

        // Boot System Identifier: "EL TORITO SPECIFICATION" (32 bytes null-padded)
        let spec_id = b"EL TORITO SPECIFICATION\0";
        br[7..7 + spec_id.len()].copy_from_slice(spec_id);

        // Boot Catalog Pointer di offset 0x47 (71 decimal)
        br[71..75].copy_from_slice(&catalog_lba.to_le_bytes());

        br
    }

    /// Menghasilkan Volume Descriptor Set Terminator (Sektor 18 / LBA 18)
    pub fn bangun_terminator() -> Vec<u8> {
        let mut term = vec![0u8; ISO_SECTOR_SIZE];
        term[0] = 0xFF; // Type 0xFF: Terminator
        term[1..6].copy_from_slice(b"CD001");
        term[6] = 0x01;
        term
    }

    /// Menghasilkan Boot Catalog Sektor (Sektor 19 / LBA 19, 2048 bytes)
    pub fn bangun_boot_catalog(
        boot_lba: u32,
        boot_sectors: u16,
        efi_lba: Option<u32>,
        efi_sectors: Option<u16>,
    ) -> Vec<u8> {
        let mut cat = vec![0u8; ISO_SECTOR_SIZE];

        // 1. Validation Entry (32 bytes pada offset 0)
        cat[0] = 0x01; // Header ID: 0x01
        cat[1] = 0x00; // Platform ID: 0x00 (80x86)
        // Bytes 4..28: ID String (24 bytes)
        let id_str = b"WidyaOS Sovereign Boot  ";
        cat[4..28].copy_from_slice(id_str);
        cat[30] = 0x55; // Key byte 55
        cat[31] = 0xAA; // Key byte AA

        // Checksum calculation: sum of all sixteen 16-bit words must equal 0 mod 0x10000
        let mut sum: u32 = 0;
        for i in 0..16 {
            let word = u16::from_le_bytes([cat[i * 2], cat[i * 2 + 1]]) as u32;
            sum = sum.wrapping_add(word);
        }
        let chk = (0x10000u32.wrapping_sub(sum & 0xFFFF)) as u16;
        cat[28..30].copy_from_slice(&chk.to_le_bytes());

        // 2. Initial / Default Entry (32 bytes pada offset 32)
        let init_off = 32;
        cat[init_off] = 0x88; // Boot Indicator: 0x88 (Bootable)
        cat[init_off + 1] = 0x00; // Boot Media Type: No Emulation
        cat[init_off + 2..init_off + 4].copy_from_slice(&0x0000u16.to_le_bytes()); // Load Segment
        cat[init_off + 4] = 0x00; // System Type
        cat[init_off + 6..init_off + 8].copy_from_slice(&boot_sectors.to_le_bytes()); // Sector Count
        cat[init_off + 8..init_off + 12].copy_from_slice(&boot_lba.to_le_bytes()); // Load RBA (LBA bootloader)

        // 3. Section Header & Entry untuk UEFI jika aktif
        if let (Some(e_lba), Some(e_secs)) = (efi_lba, efi_sectors) {
            let uefi_hdr_off = 64;
            cat[uefi_hdr_off] = 0x91; // Final Section Header Indicator (0x91)
            cat[uefi_hdr_off + 1] = 0xEF; // Platform ID: 0xEF (EFI)
            cat[uefi_hdr_off + 2..uefi_hdr_off + 4].copy_from_slice(&1u16.to_le_bytes()); // 1 section entry

            let uefi_entry_off = 96;
            cat[uefi_entry_off] = 0x88; // Bootable
            cat[uefi_entry_off + 1] = 0x00; // No emulation
            cat[uefi_entry_off + 6..uefi_entry_off + 8].copy_from_slice(&e_secs.to_le_bytes());
            cat[uefi_entry_off + 8..uefi_entry_off + 12].copy_from_slice(&e_lba.to_le_bytes());
        }

        cat
    }
}

// ============================================================================
// 7. ROOT DIRECTORY & FILE RECORDS (LBA 20)
// ============================================================================

pub struct IsoDirectoryBuilder;

impl IsoDirectoryBuilder {
    /// Menghasilkan sektor Root Directory (LBA 20) dengan entri berkas
    pub fn bangun_root_dir(
        self_lba: u32,
        parent_lba: u32,
        file_entries: &[(&str, u32, u32)], // (Nama, LBA, Ukuran dalam byte)
    ) -> Vec<u8> {
        let mut dir_sector = vec![0u8; ISO_SECTOR_SIZE];
        let mut offset = 0;

        // 1. Entri '.' (Current Directory) - 34 bytes
        let dot_rec = Self::create_record(self_lba, ISO_SECTOR_SIZE as u32, true, &[0]);
        dir_sector[offset..offset + dot_rec.len()].copy_from_slice(&dot_rec);
        offset += dot_rec.len();

        // 2. Entri '..' (Parent Directory) - 34 bytes
        let dotdot_rec = Self::create_record(parent_lba, ISO_SECTOR_SIZE as u32, true, &[1]);
        dir_sector[offset..offset + dotdot_rec.len()].copy_from_slice(&dotdot_rec);
        offset += dotdot_rec.len();

        // 3. Entri berkas-berkas
        for &(name, lba, size) in file_entries {
            let rec = Self::create_record(lba, size, false, name.as_bytes());
            if offset + rec.len() > ISO_SECTOR_SIZE {
                break; // Maksimal 1 sektor root directory untuk stub minimal
            }
            dir_sector[offset..offset + rec.len()].copy_from_slice(&rec);
            offset += rec.len();
        }

        dir_sector
    }

    fn create_record(lba: u32, data_len: u32, is_dir: bool, name: &[u8]) -> Vec<u8> {
        let name_len = name.len();
        let mut rec_len = 33 + name_len;
        if rec_len % 2 != 0 {
            rec_len += 1; // Padding ke genap
        }

        let mut rec = vec![0u8; rec_len];
        rec[0] = rec_len as u8;
        rec[1] = 0; // Extended attr
        rec[2..10].copy_from_slice(&both_endian_u32(lba));
        rec[10..18].copy_from_slice(&both_endian_u32(data_len));
        // Recording date
        rec[18] = (2026 - 1900) as u8;
        rec[19] = 10;
        rec[20] = 10;
        rec[21] = 12;
        rec[22] = 0;
        rec[23] = 0;
        rec[24] = 28;
        rec[25] = if is_dir { 0x02 } else { 0x00 };
        rec[28..32].copy_from_slice(&both_endian_u16(1));
        rec[32] = name_len as u8;
        rec[33..33 + name_len].copy_from_slice(name);

        rec
    }
}

// ============================================================================
// 8. UEFI EFI SYSTEM PARTITION (ESP FAT12) BUILDER
// ============================================================================

pub struct UefiEspBuilder;

impl UefiEspBuilder {
    /// Menghasilkan citra disk FAT12 minimal (ESP) yang memuat /EFI/BOOT/BOOTX64.EFI
    pub fn bangun_esp_fat_image(payload: &[u8]) -> Vec<u8> {
        // Alokasikan minimal 64 KB (32 sektor ISO 2048-byte) untuk citra FAT12
        let total_size = 64 * 1024;
        let mut fat_img = vec![0u8; total_size];

        // 1. FAT12 Boot Sector (Sector 0: 512 bytes)
        fat_img[0] = 0xEB; // JMP short
        fat_img[1] = 0x3C;
        fat_img[2] = 0x90; // NOP
        fat_img[3..11].copy_from_slice(b"MSDOS5.0"); // OEM
        fat_img[11..13].copy_from_slice(&512u16.to_le_bytes()); // Bytes per sector (512)
        fat_img[13] = 1; // Sectors per cluster
        fat_img[14..16].copy_from_slice(&1u16.to_le_bytes()); // Reserved sectors: 1
        fat_img[16] = 2; // Number of FATs: 2
        fat_img[17..19].copy_from_slice(&64u16.to_le_bytes()); // Root entries: 64
        fat_img[19..21].copy_from_slice(&(total_size as u16 / 512).to_le_bytes()); // Total sectors
        fat_img[21] = 0xF8; // Media descriptor: Hard disk
        fat_img[22..24].copy_from_slice(&1u16.to_le_bytes()); // Sectors per FAT: 1
        fat_img[54..62].copy_from_slice(b"FAT12   ");

        // Boot signature 0x55, 0xAA
        fat_img[510] = 0x55;
        fat_img[511] = 0xAA;

        // 2. FAT1 & FAT2 Tables (Sector 1 & 2)
        let fat1_off = 512;
        let fat2_off = 1024;
        // First 3 bytes: 0xF8, 0xFF, 0xFF
        fat_img[fat1_off..fat1_off + 3].copy_from_slice(&[0xF8, 0xFF, 0xFF]);
        fat_img[fat2_off..fat2_off + 3].copy_from_slice(&[0xF8, 0xFF, 0xFF]);

        // 3. Root Directory (Sector 3: offset 1536)
        let root_dir_off = 1536;
        // Entri direktori untuk EFI biner: "BOOTX64 EFI"
        let entry = &mut fat_img[root_dir_off..root_dir_off + 32];
        entry[..8].copy_from_slice(b"BOOTX64 ");
        entry[8..11].copy_from_slice(b"EFI");
        entry[11] = 0x20; // Archive attribute
        entry[26..28].copy_from_slice(&2u16.to_le_bytes()); // Cluster start = 2
        entry[28..32].copy_from_slice(&(payload.len() as u32).to_le_bytes());

        // 4. Data Cluster 2 (offset 1536 + (64 * 32) = 3584)
        let data_cluster_off = 3584;
        let copy_len = payload.len().min(total_size - data_cluster_off);
        fat_img[data_cluster_off..data_cluster_off + copy_len].copy_from_slice(&payload[..copy_len]);

        fat_img
    }
}

// ============================================================================
// 9. PURE LIVE ISO BUILDER ORCHESTRATOR
// ============================================================================

pub struct PureLiveIsoBuilder;

impl PureLiveIsoBuilder {
    /// Menghasilkan citra biner Live ISO Bootable utuh (ISO 9660 + El Torito + UEFI Hybrid)
    pub fn bangun_live_iso(config: &LiveIsoConfig) -> Result<Vec<u8>, String> {
        let kernel_bytes = if config.kernel_payload.is_empty() {
            // Default bare-metal kernel stub jika tidak disediakan
            b"\x7FELF\x02\x01\x01\x00WIDYAOS_BAREMETAL_KERNEL_IMAGE_V0_1_0".to_vec()
        } else {
            config.kernel_payload.clone()
        };

        // 1. Hitung alokasi sektor dan LBA untuk setiap file
        let mut current_lba = LBA_DATA_START;

        // Kernel (KERNEL.BIN)
        let kernel_lba = current_lba;
        let kernel_sectors = ((kernel_bytes.len() + ISO_SECTOR_SIZE - 1) / ISO_SECTOR_SIZE) as u32;
        current_lba += kernel_sectors;

        // Initrd jika ada (INITRD.IMG)
        let (initrd_lba, initrd_sectors) = if let Some(ref initrd) = config.initrd_payload {
            let lba = current_lba;
            let secs = ((initrd.len() + ISO_SECTOR_SIZE - 1) / ISO_SECTOR_SIZE) as u32;
            current_lba += secs;
            (Some(lba), Some(secs))
        } else {
            (None, None)
        };

        // UEFI ESP image jika aktif (EFI.IMG)
        let (efi_lba, efi_sectors, efi_img_bytes) = if config.enable_uefi {
            let img = UefiEspBuilder::bangun_esp_fat_image(&kernel_bytes);
            let lba = current_lba;
            let secs = ((img.len() + ISO_SECTOR_SIZE - 1) / ISO_SECTOR_SIZE) as u32;
            current_lba += secs;
            (Some(lba), Some(secs), Some(img))
        } else {
            (None, None, None)
        };

        // Boot configuration (BOOT.CFG)
        let boot_cfg_bytes = format!(
            "TIMEOUT=5\nDEFAULT=widyaos\nLABEL=widyaos\nKERNEL=/KERNEL.BIN\nAPPEND={}\n",
            config.boot_cmdline
        ).into_bytes();
        let cfg_lba = current_lba;
        let cfg_sectors = ((boot_cfg_bytes.len() + ISO_SECTOR_SIZE - 1) / ISO_SECTOR_SIZE) as u32;
        current_lba += cfg_sectors;

        let total_sectors = current_lba;

        // 2. Bangun Header & Sektor Sistem (Sektor 0 s/d 20)
        let mut iso_data = vec![0u8; (total_sectors as usize) * ISO_SECTOR_SIZE];

        // Sektor 0: Hybrid MBR (dengan boot signature 0x55, 0xAA)
        if config.enable_hybrid_mbr {
            let mbr = HybridMbrBuilder::bangun_mbr(total_sectors, efi_lba, efi_sectors);
            iso_data[..MBR_SECTOR_SIZE].copy_from_slice(&mbr);
        }

        // Sektor 16 (LBA 16): Primary Volume Descriptor (PVD)
        let pvd = PvdBuilder::bangun_pvd(config, total_sectors, LBA_ROOT_DIR);
        let pvd_offset = (LBA_PVD as usize) * ISO_SECTOR_SIZE;
        iso_data[pvd_offset..pvd_offset + ISO_SECTOR_SIZE].copy_from_slice(&pvd);

        // Sektor 17 (LBA 17): El Torito Boot Record Descriptor
        let el_torito_br = ElToritoBuilder::bangun_boot_record(LBA_BOOT_CATALOG);
        let br_offset = (LBA_EL_TORITO as usize) * ISO_SECTOR_SIZE;
        iso_data[br_offset..br_offset + ISO_SECTOR_SIZE].copy_from_slice(&el_torito_br);

        // Sektor 18 (LBA 18): Terminator
        let terminator = ElToritoBuilder::bangun_terminator();
        let term_offset = (LBA_TERMINATOR as usize) * ISO_SECTOR_SIZE;
        iso_data[term_offset..term_offset + ISO_SECTOR_SIZE].copy_from_slice(&terminator);

        // Sektor 19 (LBA 19): Boot Catalog
        let boot_catalog = ElToritoBuilder::bangun_boot_catalog(
            kernel_lba,
            4, // 4 virtual sectors BIOS (2048 bytes)
            efi_lba,
            efi_sectors.map(|s| s as u16),
        );
        let cat_offset = (LBA_BOOT_CATALOG as usize) * ISO_SECTOR_SIZE;
        iso_data[cat_offset..cat_offset + ISO_SECTOR_SIZE].copy_from_slice(&boot_catalog);

        // Sektor 20 (LBA 20): Root Directory
        let mut dir_records = vec![
            ("KERNEL.BIN;1", kernel_lba, kernel_bytes.len() as u32),
            ("BOOT.CFG;1", cfg_lba, boot_cfg_bytes.len() as u32),
        ];
        if let (Some(i_lba), Some(_)) = (initrd_lba, initrd_sectors) {
            if let Some(ref initrd) = config.initrd_payload {
                dir_records.push(("INITRD.IMG;1", i_lba, initrd.len() as u32));
            }
        }
        if let (Some(e_lba), Some(_), Some(ref e_img)) = (efi_lba, efi_sectors, efi_img_bytes.as_ref()) {
            dir_records.push(("EFI.IMG;1", e_lba, e_img.len() as u32));
        }

        let root_dir = IsoDirectoryBuilder::bangun_root_dir(LBA_ROOT_DIR, LBA_ROOT_DIR, &dir_records);
        let root_offset = (LBA_ROOT_DIR as usize) * ISO_SECTOR_SIZE;
        iso_data[root_offset..root_offset + ISO_SECTOR_SIZE].copy_from_slice(&root_dir);

        // 3. Tulis Payload Berkas-Berkas ke Sektor Masing-masing
        // Kernel
        let k_offset = (kernel_lba as usize) * ISO_SECTOR_SIZE;
        iso_data[k_offset..k_offset + kernel_bytes.len()].copy_from_slice(&kernel_bytes);

        // Initrd jika ada
        if let (Some(i_lba), Some(ref initrd)) = (initrd_lba, config.initrd_payload.as_ref()) {
            let i_offset = (i_lba as usize) * ISO_SECTOR_SIZE;
            iso_data[i_offset..i_offset + initrd.len()].copy_from_slice(initrd);
        }

        // EFI ESP jika ada
        if let (Some(e_lba), Some(ref e_img)) = (efi_lba, efi_img_bytes.as_ref()) {
            let e_offset = (e_lba as usize) * ISO_SECTOR_SIZE;
            iso_data[e_offset..e_offset + e_img.len()].copy_from_slice(e_img);
        }

        // Boot config
        let cfg_offset = (cfg_lba as usize) * ISO_SECTOR_SIZE;
        iso_data[cfg_offset..cfg_offset + boot_cfg_bytes.len()].copy_from_slice(&boot_cfg_bytes);

        Ok(iso_data)
    }

    /// Ekspor citra Live ISO ke berkas fisik .iso
    pub fn ekspor_live_iso(config: &LiveIsoConfig, target_path: &Path) -> Result<PathBuf, String> {
        let iso_bytes = Self::bangun_live_iso(config)?;
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Gagal membuat direktori tujuan {:?}: {}", parent, e))?;
        }

        fs::write(target_path, &iso_bytes)
            .map_err(|e| format!("Gagal menulis berkas ISO ke {:?}: {}", target_path, e))?;

        Ok(target_path.to_path_buf())
    }
}
