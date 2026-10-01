use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Representasi Pesan Standar Finansial ISO 8583 (Perbankan, ATM, Switching, EDC)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Iso8583Message {
    pub mti: String, // Message Type Identifier (4 Digit, e.g. "0200", "0210", "0800", "0810")
    pub fields: BTreeMap<u32, String>, // Field 2 hingga 128
}

impl Default for Iso8583Message {
    fn default() -> Self {
        Self {
            mti: "0200".to_string(),
            fields: BTreeMap::new(),
        }
    }
}

impl Iso8583Message {
    pub fn new(mti: &str) -> Self {
        Self {
            mti: mti.to_string(),
            fields: BTreeMap::new(),
        }
    }

    pub fn set_field(&mut self, field_num: u32, value: &str) -> &mut Self {
        self.fields.insert(field_num, value.to_string());
        self
    }

    pub fn get_field(&self, field_num: u32) -> Option<&String> {
        self.fields.get(&field_num)
    }

    /// Menghasilkan Primary Bitmap (64-bit) dan Secondary Bitmap jika ada field > 64
    pub fn generate_bitmap_hex(&self) -> String {
        let max_field = self.fields.keys().max().copied().unwrap_or(0);
        let has_secondary = max_field > 64;

        let num_bits = if has_secondary { 128 } else { 64 };
        let mut bits = vec![false; num_bits];

        if has_secondary {
            bits[0] = true; // Bit 1 menandakan keberadaan secondary bitmap
        }

        for &field in self.fields.keys() {
            if field >= 1 && (field as usize) <= num_bits {
                bits[(field - 1) as usize] = true;
            }
        }

        // Konversi boolean array ke Hex String (4 bit per hex char)
        let mut hex_str = String::new();
        for chunk in bits.chunks(4) {
            let mut val = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit {
                    val |= 1 << (3 - i);
                }
            }
            hex_str.push_str(&format!("{:X}", val));
        }

        hex_str
    }

    /// Melakukan serialisasi ISO 8583 ke string mentah (ASCII/Hex Stream)
    pub fn pack(&self) -> Result<String, String> {
        let mut packed = String::new();
        if self.mti.len() != 4 {
            return Err("MTI harus tepat 4 karakter numerik".to_string());
        }
        packed.push_str(&self.mti);

        let bitmap = self.generate_bitmap_hex();
        packed.push_str(&bitmap);

        for (&field_num, val) in &self.fields {
            let field_formatted = format_iso_field(field_num, val)?;
            packed.push_str(&field_formatted);
        }

        Ok(packed)
    }

    /// Melakukan parsing string mentah ISO 8583
    pub fn unpack(raw: &str) -> Result<Self, String> {
        if raw.len() < 20 {
            return Err("Pesan ISO 8583 terlalu pendek (minimal 20 karakter)".to_string());
        }

        let mti = raw[0..4].to_string();
        let primary_bitmap_hex = &raw[4..20];
        
        // Parse primary bitmap
        let mut active_fields = Vec::new();
        let mut has_secondary = false;

        let mut bit_idx = 1;
        for c in primary_bitmap_hex.chars() {
            let nibble = u8::from_str_radix(&c.to_string(), 16)
                .map_err(|e| format!("Karakter bitmap tidak valid: {}", e))?;
            for i in (0..4).rev() {
                let is_set = (nibble & (1 << i)) != 0;
                if bit_idx == 1 && is_set {
                    has_secondary = true;
                } else if bit_idx > 1 && is_set {
                    active_fields.push(bit_idx);
                }
                bit_idx += 1;
            }
        }

        let mut cursor = 20;
        if has_secondary {
            if raw.len() < 36 {
                return Err("Pesan terpotong saat membaca Secondary Bitmap".to_string());
            }
            let secondary_bitmap_hex = &raw[20..36];
            cursor = 36;
            for c in secondary_bitmap_hex.chars() {
                let nibble = u8::from_str_radix(&c.to_string(), 16)
                    .map_err(|e| format!("Karakter secondary bitmap tidak valid: {}", e))?;
                for i in (0..4).rev() {
                    let is_set = (nibble & (1 << i)) != 0;
                    active_fields.push(bit_idx);
                    bit_idx += 1;
                }
            }
        }

        let mut fields = BTreeMap::new();
        for field_num in active_fields {
            let (val, next_cursor) = parse_iso_field(field_num, raw, cursor)?;
            fields.insert(field_num, val);
            cursor = next_cursor;
        }

        Ok(Iso8583Message { mti, fields })
    }
}

/// Helper pemformatan field ISO 8583
fn format_iso_field(field_num: u32, val: &str) -> Result<String, String> {
    match field_num {
        2 => { // LLVAR n..19 (PAN)
            Ok(format!("{:02}{}", val.len(), val))
        }
        3 => { // n 6 (Processing Code)
            Ok(format!("{:0>6}", val))
        }
        4 => { // n 12 (Amount)
            Ok(format!("{:0>12}", val))
        }
        7 => { // n 10 (Transmission Date Time MMDDhhmmss)
            Ok(format!("{:0>10}", val))
        }
        11 => { // n 6 (STAN)
            Ok(format!("{:0>6}", val))
        }
        12 => { // n 6 (Time Local hhmmss)
            Ok(format!("{:0>6}", val))
        }
        13 => { // n 4 (Date Local MMDD)
            Ok(format!("{:0>4}", val))
        }
        14 => { // n 4 (Expiry YYMM)
            Ok(format!("{:0>4}", val))
        }
        18 => { // n 4 (MCC)
            Ok(format!("{:0>4}", val))
        }
        37 => { // an 12 (RRN)
            Ok(format!("{: <12}", val))
        }
        38 => { // an 6 (Auth Code)
            Ok(format!("{: <6}", val))
        }
        39 => { // an 2 (Response Code)
            Ok(format!("{: <2}", val))
        }
        41 => { // ans 8 (Terminal ID)
            Ok(format!("{: <8}", val))
        }
        42 => { // ans 15 (Merchant ID)
            Ok(format!("{: <15}", val))
        }
        48 => { // LLLVAR ans..999 (Additional Data)
            Ok(format!("{:03}{}", val.len(), val))
        }
        49 => { // an 3 (Currency Code)
            Ok(format!("{:0>3}", val))
        }
        70 => { // n 3 (Network Management Code)
            Ok(format!("{:0>3}", val))
        }
        _ => { // Default LLVAR untuk field umum lainnya
            if val.len() < 100 {
                Ok(format!("{:02}{}", val.len(), val))
            } else {
                Ok(format!("{:03}{}", val.len(), val))
            }
        }
    }
}

/// Helper parsing field ISO 8583 dari stream raw
fn parse_iso_field(field_num: u32, raw: &str, cursor: usize) -> Result<(String, usize), String> {
    if cursor >= raw.len() {
        return Err(format!("Data terpotong sebelum Field {}", field_num));
    }

    match field_num {
        2 => { // LLVAR n..19
            if cursor + 2 > raw.len() { return Err("Field 2 panjang terpotong".to_string()); }
            let len: usize = raw[cursor..cursor + 2].parse().map_err(|_| "Gagal parse panjang F2")?;
            let start = cursor + 2;
            let end = start + len;
            if end > raw.len() { return Err("Field 2 isi terpotong".to_string()); }
            Ok((raw[start..end].to_string(), end))
        }
        3 => { // n 6
            let end = cursor + 6;
            if end > raw.len() { return Err("Field 3 terpotong".to_string()); }
            Ok((raw[cursor..end].to_string(), end))
        }
        4 => { // n 12
            let end = cursor + 12;
            if end > raw.len() { return Err("Field 4 terpotong".to_string()); }
            Ok((raw[cursor..end].to_string(), end))
        }
        7 => { // n 10
            let end = cursor + 10;
            if end > raw.len() { return Err("Field 7 terpotong".to_string()); }
            Ok((raw[cursor..end].to_string(), end))
        }
        11 | 12 => { // n 6
            let end = cursor + 6;
            if end > raw.len() { return Err(format!("Field {} terpotong", field_num)); }
            Ok((raw[cursor..end].to_string(), end))
        }
        13 | 14 | 18 => { // n 4
            let end = cursor + 4;
            if end > raw.len() { return Err(format!("Field {} terpotong", field_num)); }
            Ok((raw[cursor..end].to_string(), end))
        }
        37 => { // an 12
            let end = cursor + 12;
            if end > raw.len() { return Err("Field 37 terpotong".to_string()); }
            Ok((raw[cursor..end].trim().to_string(), end))
        }
        38 => { // an 6
            let end = cursor + 6;
            if end > raw.len() { return Err("Field 38 terpotong".to_string()); }
            Ok((raw[cursor..end].trim().to_string(), end))
        }
        39 => { // an 2
            let end = cursor + 2;
            if end > raw.len() { return Err("Field 39 terpotong".to_string()); }
            Ok((raw[cursor..end].trim().to_string(), end))
        }
        41 => { // ans 8
            let end = cursor + 8;
            if end > raw.len() { return Err("Field 41 terpotong".to_string()); }
            Ok((raw[cursor..end].trim().to_string(), end))
        }
        42 => { // ans 15
            let end = cursor + 15;
            if end > raw.len() { return Err("Field 42 terpotong".to_string()); }
            Ok((raw[cursor..end].trim().to_string(), end))
        }
        48 => { // LLLVAR
            if cursor + 3 > raw.len() { return Err("Field 48 panjang terpotong".to_string()); }
            let len: usize = raw[cursor..cursor + 3].parse().map_err(|_| "Gagal parse panjang F48")?;
            let start = cursor + 3;
            let end = start + len;
            if end > raw.len() { return Err("Field 48 isi terpotong".to_string()); }
            Ok((raw[start..end].to_string(), end))
        }
        49 | 70 => { // n 3
            let end = cursor + 3;
            if end > raw.len() { return Err(format!("Field {} terpotong", field_num)); }
            Ok((raw[cursor..end].trim().to_string(), end))
        }
        _ => { // Generic LLVAR fallback
            if cursor + 2 > raw.len() { return Err(format!("Field {} panjang terpotong", field_num)); }
            let len: usize = raw[cursor..cursor + 2].parse().unwrap_or(0);
            let start = cursor + 2;
            let end = start + len;
            if end > raw.len() { return Err(format!("Field {} isi terpotong", field_num)); }
            Ok((raw[start..end].to_string(), end))
        }
    }
}
