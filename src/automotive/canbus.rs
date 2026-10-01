use serde::{Deserialize, Serialize};

/// Frame Pesan Protokol CAN Bus (Controller Area Network 2.0B / CAN-FD)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanFrame {
    pub id: u32,             // 11-bit standard atau 29-bit extended ID
    pub is_extended: bool,
    pub is_rtr: bool,        // Remote Transmission Request
    pub dlc: u8,             // Data Length Code (0..8 byte)
    pub data: [u8; 8],
}

impl CanFrame {
    pub fn new_standard(id: u16, data: &[u8]) -> Self {
        let mut d = [0u8; 8];
        let len = data.len().min(8);
        d[0..len].copy_from_slice(&data[0..len]);
        Self {
            id: (id & 0x7FF) as u32,
            is_extended: false,
            is_rtr: false,
            dlc: len as u8,
            data: d,
        }
    }

    pub fn new_extended(id: u32, data: &[u8]) -> Self {
        let mut d = [0u8; 8];
        let len = data.len().min(8);
        d[0..len].copy_from_slice(&data[0..len]);
        Self {
            id: id & 0x1FFFFFFF,
            is_extended: true,
            is_rtr: false,
            dlc: len as u8,
            data: d,
        }
    }
}

/// Filter Mask Hardware untuk Penerimaan Selektif Frame CAN Bus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanFilter {
    pub id: u32,
    pub mask: u32,
}

impl CanFilter {
    pub fn new(id: u32, mask: u32) -> Self {
        Self { id, mask }
    }

    /// Mengecek apakah frame CAN lolos filter hardware mask
    pub fn matches(&self, frame: &CanFrame) -> bool {
        (frame.id & self.mask) == (self.id & self.mask)
    }
}
