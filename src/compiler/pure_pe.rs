//! Pure Sovereign Windows PE32+ (64-bit .exe) Direct Emitter (pure_pe.rs)
//!
//! Pembangkit biner eksekusi Windows 64-bit (PE32+ / AMD64) 100% mandiri
//! tanpa ketergantungan pada MSVC link.exe, MinGW, maupun LLVM lld:
//! - 💾 MS-DOS Header ("MZ") & DOS Stub Program pada offset 0x00
//! - 🚀 PE Signature ("PE\0\0") pada offset e_lfanew (0x80)
//! - 📑 COFF File Header: IMAGE_FILE_MACHINE_AMD64 (0x8664)
//! - ⚡ PE32+ Optional Header: Magic 0x020B, ImageBase 0x140000000, Subsystem Windows CUI/GUI
//! - 📦 Tabel Seksi Standar: .text (kode mesin x86_64 r-x) dan .data (data terinisialisasi rw-)
//! - 🪟 Ekspor berkas biner fisik .exe yang dapat langsung dieksekusi di Windows x86_64

use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// 1. KONSTANTA SPESIFIKASI WINDOWS PE / COFF
// ============================================================================

pub const DOS_MAGIC: [u8; 2] = [0x4D, 0x5A]; // "MZ"
pub const PE_SIGNATURE: [u8; 4] = [b'P', b'E', 0x00, 0x00]; // "PE\0\0"
pub const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664; // x86_64
pub const PE32_PLUS_MAGIC: u16 = 0x020B; // 64-bit Optional Header

pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: u16 = 2;
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: u16 = 3; // Console Application

pub const IMAGE_SCN_CNT_CODE: u32 = 0x00000020;
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 0x00000040;
pub const IMAGE_SCN_MEM_EXECUTE: u32 = 0x20000000;
pub const IMAGE_SCN_MEM_READ: u32 = 0x40000000;
pub const IMAGE_SCN_MEM_WRITE: u32 = 0x80000000;

pub const DEFAULT_IMAGE_BASE: u64 = 0x0000000140000000;
pub const DEFAULT_SECTION_ALIGNMENT: u32 = 0x1000; // 4096 bytes
pub const DEFAULT_FILE_ALIGNMENT: u32 = 0x200; // 512 bytes

// ============================================================================
// 2. KONFIGURASI EMITTER PE32+
// ============================================================================

#[derive(Debug, Clone)]
pub struct PeConfig {
    pub is_console: bool,
    pub image_base: u64,
    pub code_bytes: Vec<u8>,
    pub data_bytes: Vec<u8>,
}

impl PeConfig {
    pub fn baru_console() -> Self {
        Self {
            is_console: true,
            image_base: DEFAULT_IMAGE_BASE,
            code_bytes: Vec::new(),
            data_bytes: Vec::new(),
        }
    }

    pub fn baru_gui() -> Self {
        Self {
            is_console: false,
            image_base: DEFAULT_IMAGE_BASE,
            code_bytes: Vec::new(),
            data_bytes: Vec::new(),
        }
    }

    pub fn dengan_kode(mut self, code: &[u8]) -> Self {
        self.code_bytes = code.to_vec();
        self
    }

    pub fn dengan_data(mut self, data: &[u8]) -> Self {
        self.data_bytes = data.to_vec();
        self
    }
}

// ============================================================================
// 3. PURE PE32+ BUILDER
// ============================================================================

pub struct PurePeBuilder;

impl PurePeBuilder {
    /// Menghasilkan berkas biner utuh Windows PE32+ (.exe) ke dalam byte buffer
    pub fn bangun_exe(config: &PeConfig) -> Vec<u8> {
        // Kode default: xor eax, eax; ret (0x31, 0xC0, 0xC3) jika tidak disediakan
        let code = if config.code_bytes.is_empty() {
            vec![0x31, 0xC0, 0xC3]
        } else {
            config.code_bytes.clone()
        };

        let data = &config.data_bytes;

        // Hitung perataan ukuran berkas (512 bytes) dan memori (4096 bytes)
        let file_align = DEFAULT_FILE_ALIGNMENT;
        let sec_align = DEFAULT_SECTION_ALIGNMENT;

        let code_raw_size = ((code.len() as u32 + file_align - 1) / file_align) * file_align;
        let code_virt_size = ((code.len() as u32 + sec_align - 1) / sec_align) * sec_align;

        let data_raw_size = if !data.is_empty() {
            ((data.len() as u32 + file_align - 1) / file_align) * file_align
        } else {
            0
        };
        let data_virt_size = if !data.is_empty() {
            ((data.len() as u32 + sec_align - 1) / sec_align) * sec_align
        } else {
            0
        };

        let num_sections = if data.is_empty() { 1u16 } else { 2u16 };

        // Ukuran header: DOS Header (64) + DOS Stub (64) + PE Sig (4) + COFF (20) + Optional (240) + Sections (40 * n)
        let headers_size_raw = 64 + 64 + 4 + 20 + 240 + (num_sections as usize * 40);
        let headers_file_size = ((headers_size_raw as u32 + file_align - 1) / file_align) * file_align;

        let text_file_offset = headers_file_size;
        let text_rva = sec_align; // 0x1000

        let data_file_offset = text_file_offset + code_raw_size;
        let data_rva = text_rva + code_virt_size;

        let size_of_image = if data.is_empty() {
            text_rva + code_virt_size
        } else {
            data_rva + data_virt_size
        };

        let mut pe = Vec::with_capacity(data_file_offset as usize + data_raw_size as usize);

        // 1. MS-DOS Header (64 bytes)
        pe.extend_from_slice(&DOS_MAGIC); // e_magic = "MZ" (0..2)
        pe.extend_from_slice(&[0u8; 58]);  // Isian DOS header standar
        let e_lfanew = 128u32; // Offset ke PE Header (0x80)
        pe.extend_from_slice(&e_lfanew.to_le_bytes()); // Offset 0x3C

        // 2. MS-DOS Stub Program (64 bytes: offset 0x40..0x80)
        let dos_stub: [u8; 64] = [
            0x0E, 0x1F, 0xBA, 0x0E, 0x00, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x01, 0x4C, 0xCD, 0x21,
            b'T', b'h', b'i', b's', b' ', b'p', b'r', b'o', b'g', b'r', b'a', b'm', b' ', b'c',
            b'a', b'n', b'n', b'o', b't', b' ', b'b', b'e', b' ', b'r', b'u', b'n', b' ', b'i',
            b'n', b' ', b'D', b'O', b'S', b' ', b'm', b'o', b'd', b'e', b'.', b'\r', b'\r', b'\n',
            b'$', 0, 0, 0, 0, 0, 0, 0,
        ];
        pe.extend_from_slice(&dos_stub);

        // 3. PE Signature (4 bytes pada offset 0x80)
        pe.extend_from_slice(&PE_SIGNATURE); // "PE\0\0"

        // 4. COFF File Header (20 bytes)
        pe.extend_from_slice(&IMAGE_FILE_MACHINE_AMD64.to_le_bytes()); // Machine: x86_64
        pe.extend_from_slice(&num_sections.to_le_bytes()); // NumberOfSections
        let timestamp = 1775822400u32; // Deterministic 2026 timestamp
        pe.extend_from_slice(&timestamp.to_le_bytes());
        pe.extend_from_slice(&0u32.to_le_bytes()); // PointerToSymbolTable
        pe.extend_from_slice(&0u32.to_le_bytes()); // NumberOfSymbols
        let size_of_opt_hdr = 240u16; // Ukuran Optional Header PE32+
        pe.extend_from_slice(&size_of_opt_hdr.to_le_bytes());
        // Characteristics: EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE (0x0022)
        pe.extend_from_slice(&0x0022u16.to_le_bytes());

        // 5. PE32+ Optional Header (240 bytes)
        // Standard fields:
        pe.extend_from_slice(&PE32_PLUS_MAGIC.to_le_bytes()); // Magic = 0x020B (PE32+)
        pe.push(14); // MajorLinkerVersion
        pe.push(0);  // MinorLinkerVersion
        pe.extend_from_slice(&code_raw_size.to_le_bytes()); // SizeOfCode
        pe.extend_from_slice(&data_raw_size.to_le_bytes()); // SizeOfInitializedData
        pe.extend_from_slice(&0u32.to_le_bytes()); // SizeOfUninitializedData
        pe.extend_from_slice(&text_rva.to_le_bytes()); // AddressOfEntryPoint (RVA 0x1000)
        pe.extend_from_slice(&text_rva.to_le_bytes()); // BaseOfCode

        // Windows-specific fields:
        pe.extend_from_slice(&config.image_base.to_le_bytes()); // ImageBase
        pe.extend_from_slice(&sec_align.to_le_bytes()); // SectionAlignment
        pe.extend_from_slice(&file_align.to_le_bytes()); // FileAlignment
        pe.extend_from_slice(&6u16.to_le_bytes()); // MajorOSVersion
        pe.extend_from_slice(&0u16.to_le_bytes()); // MinorOSVersion
        pe.extend_from_slice(&0u16.to_le_bytes()); // MajorImageVersion
        pe.extend_from_slice(&0u16.to_le_bytes()); // MinorImageVersion
        pe.extend_from_slice(&6u16.to_le_bytes()); // MajorSubsystemVersion
        pe.extend_from_slice(&0u16.to_le_bytes()); // MinorSubsystemVersion
        pe.extend_from_slice(&0u32.to_le_bytes()); // Win32VersionValue
        pe.extend_from_slice(&size_of_image.to_le_bytes()); // SizeOfImage
        pe.extend_from_slice(&headers_file_size.to_le_bytes()); // SizeOfHeaders
        pe.extend_from_slice(&0u32.to_le_bytes()); // CheckSum

        let subsystem = if config.is_console {
            IMAGE_SUBSYSTEM_WINDOWS_CUI
        } else {
            IMAGE_SUBSYSTEM_WINDOWS_GUI
        };
        pe.extend_from_slice(&subsystem.to_le_bytes()); // Subsystem
        // DllCharacteristics: DYNAMIC_BASE | NX_COMPAT | TERMINAL_SERVER_AWARE (0x8160)
        pe.extend_from_slice(&0x8160u16.to_le_bytes());

        // Stack & Heap Reserves:
        pe.extend_from_slice(&0x00100000u64.to_le_bytes()); // SizeOfStackReserve (1MB)
        pe.extend_from_slice(&0x00001000u64.to_le_bytes()); // SizeOfStackCommit (4KB)
        pe.extend_from_slice(&0x00100000u64.to_le_bytes()); // SizeOfHeapReserve (1MB)
        pe.extend_from_slice(&0x00001000u64.to_le_bytes()); // SizeOfHeapCommit (4KB)
        pe.extend_from_slice(&0u32.to_le_bytes()); // LoaderFlags
        pe.extend_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes

        // 16 Data Directories (16 * 8 = 128 bytes, all zero for standalone stub):
        pe.extend_from_slice(&[0u8; 128]);

        // 6. Section Headers (40 bytes each)
        // Seksi 1: .text
        let mut text_hdr = [0u8; 40];
        text_hdr[..5].copy_from_slice(b".text");
        text_hdr[8..12].copy_from_slice(&(code.len() as u32).to_le_bytes()); // VirtualSize
        text_hdr[12..16].copy_from_slice(&text_rva.to_le_bytes()); // VirtualAddress
        text_hdr[16..20].copy_from_slice(&code_raw_size.to_le_bytes()); // SizeOfRawData
        text_hdr[20..24].copy_from_slice(&text_file_offset.to_le_bytes()); // PointerToRawData
        let text_chars = IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_READ;
        text_hdr[36..40].copy_from_slice(&text_chars.to_le_bytes());
        pe.extend_from_slice(&text_hdr);

        // Seksi 2: .data (jika ada data)
        if !data.is_empty() {
            let mut data_hdr = [0u8; 40];
            data_hdr[..5].copy_from_slice(b".data");
            data_hdr[8..12].copy_from_slice(&(data.len() as u32).to_le_bytes());
            data_hdr[12..16].copy_from_slice(&data_rva.to_le_bytes());
            data_hdr[16..20].copy_from_slice(&data_raw_size.to_le_bytes());
            data_hdr[20..24].copy_from_slice(&data_file_offset.to_le_bytes());
            let data_chars = IMAGE_SCN_CNT_INITIALIZED_DATA | IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE;
            data_hdr[36..40].copy_from_slice(&data_chars.to_le_bytes());
            pe.extend_from_slice(&data_hdr);
        }

        // Pad header ke headers_file_size
        let header_pad = headers_file_size as usize - pe.len();
        pe.extend(std::iter::repeat(0).take(header_pad));

        // 7. Muatan Seksi .text
        pe.extend_from_slice(&code);
        let text_pad = code_raw_size as usize - code.len();
        pe.extend(std::iter::repeat(0x90).take(text_pad)); // Pad dengan NOP (0x90)

        // 8. Muatan Seksi .data jika ada
        if !data.is_empty() {
            pe.extend_from_slice(data);
            let data_pad = data_raw_size as usize - data.len();
            pe.extend(std::iter::repeat(0).take(data_pad));
        }

        pe
    }

    /// Ekspor biner .exe langsung ke berkas penyimpanan fisik
    pub fn ekspor_exe(config: &PeConfig, target_path: &Path) -> Result<PathBuf, String> {
        let exe_bytes = Self::bangun_exe(config);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Gagal membuat direktori {:?}: {}", parent, e))?;
        }
        fs::write(target_path, &exe_bytes)
            .map_err(|e| format!("Gagal menulis berkas EXE ke {:?}: {}", target_path, e))?;
        Ok(target_path.to_path_buf())
    }
}
