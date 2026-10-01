/// Pure Rust Zero-Dependency ZIP Package Builder (Store / Uncompressed method)
/// Kompatibel 100% dengan spesifikasi ZIP PKWARE & OpenXML (ECMA-376)
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Kalkulasi IEEE 802.3 CRC32 standar
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xED_B8_83_20;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

#[derive(Debug, Clone)]
pub struct ZipEntry {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct ZipPackage {
    pub entries: Vec<ZipEntry>,
}

impl ZipPackage {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_file(&mut self, path: &str, content: &[u8]) {
        self.entries.push(ZipEntry {
            name: path.replace('\\', "/"),
            data: content.to_vec(),
        });
    }

    pub fn add_file_str(&mut self, path: &str, content: &str) {
        self.add_file(path, content.as_bytes());
    }

    /// Menghasilkan byte biner berkas .ZIP / .DOCX / .XLSX / .PPTX
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut central_dir = Vec::new();
        let mut local_offsets = Vec::new();

        // 1. Tulis Local File Headers & Data
        for entry in &self.entries {
            let offset = out.len() as u32;
            local_offsets.push(offset);

            let name_bytes = entry.name.as_bytes();
            let crc = crc32(&entry.data);
            let size = entry.data.len() as u32;

            // Local File Header Signature: 0x04034b50 ("PK\x03\x04")
            out.extend_from_slice(&0x04034b50u32.to_le_bytes());
            out.extend_from_slice(&10u16.to_le_bytes()); // Version needed to extract (1.0)
            out.extend_from_slice(&0u16.to_le_bytes());  // General purpose bit flag
            out.extend_from_slice(&0u16.to_le_bytes());  // Compression method (0 = Store)
            out.extend_from_slice(&0x4b21u16.to_le_bytes()); // Last mod file time
            out.extend_from_slice(&0x5ceeu16.to_le_bytes()); // Last mod file date
            out.extend_from_slice(&crc.to_le_bytes());       // CRC-32
            out.extend_from_slice(&size.to_le_bytes());      // Compressed size
            out.extend_from_slice(&size.to_le_bytes());      // Uncompressed size
            out.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes()); // File name length
            out.extend_from_slice(&0u16.to_le_bytes());      // Extra field length
            out.extend_from_slice(name_bytes);               // File name
            out.extend_from_slice(&entry.data);              // File data

            // Central Directory Header
            // Signature: 0x02014b50 ("PK\x01\x02")
            central_dir.extend_from_slice(&0x02014b50u32.to_le_bytes());
            central_dir.extend_from_slice(&20u16.to_le_bytes());     // Version made by (DOS / 2.0)
            central_dir.extend_from_slice(&10u16.to_le_bytes());     // Version needed to extract (1.0)
            central_dir.extend_from_slice(&0u16.to_le_bytes());      // General purpose bit flag
            central_dir.extend_from_slice(&0u16.to_le_bytes());      // Compression method (0 = Store)
            central_dir.extend_from_slice(&0x4b21u16.to_le_bytes()); // Last mod time
            central_dir.extend_from_slice(&0x5ceeu16.to_le_bytes()); // Last mod date
            central_dir.extend_from_slice(&crc.to_le_bytes());       // CRC-32
            central_dir.extend_from_slice(&size.to_le_bytes());      // Compressed size
            central_dir.extend_from_slice(&size.to_le_bytes());      // Uncompressed size
            central_dir.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes()); // File name length
            central_dir.extend_from_slice(&0u16.to_le_bytes());      // Extra field length
            central_dir.extend_from_slice(&0u16.to_le_bytes());      // File comment length
            central_dir.extend_from_slice(&0u16.to_le_bytes());      // Disk number start
            central_dir.extend_from_slice(&0u16.to_le_bytes());      // Internal file attributes
            central_dir.extend_from_slice(&0x00000020u32.to_le_bytes()); // External file attributes (MS-DOS Archive)
            central_dir.extend_from_slice(&offset.to_le_bytes());    // Relative offset of local header
            central_dir.extend_from_slice(name_bytes);               // File name
        }

        let central_dir_offset = out.len() as u32;
        let central_dir_size = central_dir.len() as u32;
        out.extend_from_slice(&central_dir);

        // End of Central Directory Record (EOCD)
        // Signature: 0x06054b50 ("PK\x05\x06")
        out.extend_from_slice(&0x06054b50u32.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // Number of this disk
        out.extend_from_slice(&0u16.to_le_bytes()); // Disk with start of central directory
        out.extend_from_slice(&(self.entries.len() as u16).to_le_bytes()); // Total entries on this disk
        out.extend_from_slice(&(self.entries.len() as u16).to_le_bytes()); // Total entries
        out.extend_from_slice(&central_dir_size.to_le_bytes());   // Size of central directory
        out.extend_from_slice(&central_dir_offset.to_le_bytes()); // Offset of start of central directory
        out.extend_from_slice(&0u16.to_le_bytes()); // ZIP file comment length

        out
    }

    /// Simpan paket ke berkas fisik (.docx, .xlsx, .pptx, .zip)
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let bytes = self.to_bytes();
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}
