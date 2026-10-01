//! Zero-Copy Binary Serialization & Wire Protocol Engine (zerocopy.rs)
//! Memungkinkan pembacaan dan penulisan struktur data langsung dari buffer memori (DMA/Ring-Buffer) tanpa alokasi heap atau parsing CPU berulang.

use std::mem;

/// Header Paket Zero-Copy dengan Magic, Message ID, Sequence, dan Length
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroCopyHeader {
    pub magic: u32,        // 0x57445941 ("WDYA")
    pub message_id: u16,   // ID Jenis Pesan
    pub sequence: u32,     // Nomor Urut Transmisi
    pub payload_len: u32,  // Panjang Data Payload
    pub checksum: u32,     // Checksum Integritas CRC32
}

pub const ZEROCOPY_MAGIC: u32 = 0x57445941;

impl ZeroCopyHeader {
    pub fn new(message_id: u16, sequence: u32, payload_len: u32) -> Self {
        Self {
            magic: ZEROCOPY_MAGIC,
            message_id,
            sequence,
            payload_len,
            checksum: 0,
        }
    }

    /// Validasi integritas header langsung dari slice byte mentah tanpa alokasi
    pub fn from_raw_slice(bytes: &[u8]) -> Result<(&Self, &[u8]), String> {
        let header_size = mem::size_of::<Self>();
        if bytes.len() < header_size {
            return Err("Buffer lebih kecil dari ukuran ZeroCopyHeader".to_string());
        }

        let header = unsafe { &*(bytes.as_ptr() as *const Self) };
        if header.magic != ZEROCOPY_MAGIC {
            return Err(format!("Magic number tidak valid: 0x{:X}", header.magic));
        }

        let total_expected = header_size + header.payload_len as usize;
        if bytes.len() < total_expected {
            return Err(format!("Ukuran paket tidak lengkap: butuh {} byte, ada {} byte", total_expected, bytes.len()));
        }

        let payload = &bytes[header_size..total_expected];
        Ok((header, payload))
    }

    /// Serialisasi langsung ke buffer biner
    pub fn serialize_to_buffer(&self, payload: &[u8]) -> Vec<u8> {
        let header_size = mem::size_of::<Self>();
        let mut out = Vec::with_capacity(header_size + payload.len());

        let header_bytes = unsafe {
            std::slice::from_raw_parts(self as *const Self as *const u8, header_size)
        };
        out.extend_from_slice(header_bytes);
        out.extend_from_slice(payload);
        out
    }
}
