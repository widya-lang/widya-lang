// ============================================================================
// WIDYA INVENTORY & SUPPLY CHAIN - RFID ENGINE (src/inventory/rfid.rs)
// Mendukung Standar EPC Gen2 (ISO/IEC 18000-6C) SGTIN-96 & Gate Telemetri
// ============================================================================

use serde::{Deserialize, Serialize};

/// Nilai Filter EPC Tag (Tipe Objek Kemasan)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EpcFilterValue {
    AllOthers = 0,
    PointOfSale = 1,
    FullCase = 2,
    Reserved = 3,
    InnerPack = 4,
    UnitLoadPallet = 6,
}

impl EpcFilterValue {
    pub fn from_u8(val: u8) -> Self {
        match val {
            1 => Self::PointOfSale,
            2 => Self::FullCase,
            4 => Self::InnerPack,
            6 => Self::UnitLoadPallet,
            _ => Self::AllOthers,
        }
    }
}

/// Struktur Tag RFID EPC Gen2 SGTIN-96 (96-bit Electronic Product Code)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpcSgtin96 {
    pub header: u8,              // 8-bit: 0x30 untuk SGTIN-96
    pub filter: EpcFilterValue,  // 3-bit: Tipe kemasan
    pub partition: u8,           // 3-bit: Pembagian bit Prefix vs Item
    pub company_prefix: u64,     // Identitas Perusahaan GS1
    pub item_reference: u32,     // Nomor Kode Item Produk
    pub serial_number: u64,      // 38-bit: Nomor Seri Unik Tiap Unit
    pub raw_hex: String,         // 24-karakter Hexadecimal (96 bit)
}

impl EpcSgtin96 {
    /// Membuat dan mengenkode EPC SGTIN-96 menjadi heksadesimal 24 karakter
    pub fn new(filter: EpcFilterValue, company_prefix: u64, item_ref: u32, serial: u64) -> Self {
        let header = 0x30u8; // SGTIN-96 standard header
        let partition = 5u8; // Standar partisi 5: Company Prefix 24 bit, Item Reference 20 bit

        // Susun 96-bit buffer:
        // [Header: 8b][Filter: 3b][Partition: 3b][Company: 24b][Item: 20b][Serial: 38b] = 96 bit = 12 byte
        let mut bits: u128 = 0;
        bits |= (header as u128) << 88;
        bits |= ((filter as u8 & 0x07) as u128) << 85;
        bits |= ((partition & 0x07) as u128) << 82;
        bits |= ((company_prefix & 0xFFFFFF) as u128) << 58;
        bits |= ((item_ref & 0xFFFFF) as u128) << 38;
        bits |= (serial & 0x3FFFFFFFFF) as u128;

        let hex_str = format!("{:024X}", bits);

        Self {
            header,
            filter,
            partition,
            company_prefix,
            item_reference: item_ref,
            serial_number: serial,
            raw_hex: hex_str,
        }
    }

    /// Mendekode tag heksadesimal 24-karakter EPC SGTIN-96
    pub fn decode(hex_str: &str) -> Result<Self, String> {
        let clean = hex_str.trim().trim_start_matches("0x").trim_start_matches("0X");
        if clean.len() != 24 {
            return Err("Panjang EPC SGTIN-96 harus tepat 24 karakter heksadesimal (96 bit)".to_string());
        }

        let bits = u128::from_str_radix(clean, 16)
            .map_err(|e| format!("Gagal mem-parse heksadesimal: {}", e))?;

        let header = ((bits >> 88) & 0xFF) as u8;
        if header != 0x30 {
            return Err(format!("Header EPC bukan SGTIN-96 (diharapkan 0x30, didapat 0x{:02X})", header));
        }

        let filter_val = ((bits >> 85) & 0x07) as u8;
        let partition = ((bits >> 82) & 0x07) as u8;
        let company_prefix = ((bits >> 58) & 0xFFFFFF) as u64;
        let item_reference = ((bits >> 38) & 0xFFFFF) as u32;
        let serial_number = (bits & 0x3FFFFFFFFF) as u64;

        Ok(Self {
            header,
            filter: EpcFilterValue::from_u8(filter_val),
            partition,
            company_prefix,
            item_reference,
            serial_number,
            raw_hex: clean.to_uppercase(),
        })
    }
}

/// Telemetri Pembacaan Gate RFID Otomatis (Portal Masuk / Keluar Gudang)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfidGateEvent {
    pub gate_id: String,           // Misal: "GATE-MASUK-01" atau "GATE-DOCK-A"
    pub tag_epc_hex: String,       // EPC Hex
    pub antenna_port: u8,          // Port antena (1-4)
    pub rssi_dbm: f32,             // Sinyal kekuatan (-80 s/d -30 dBm)
    pub read_count: u32,           // Akumulasi hit bacaan dalam jendela waktu
    pub timestamp_ms: u64,         // Unix epoch time
    pub direction: GateDirection,  // Arah pergerakan
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateDirection {
    MasukGudang,   // Inbound / Gate-In
    KeluarGudang,  // Outbound / Gate-Out
    PindahLokasi,  // Relokasi Internal
}
