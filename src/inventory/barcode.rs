// ============================================================================
// WIDYA INVENTORY & SUPPLY CHAIN - BARCODE ENGINE (src/inventory/barcode.rs)
// Mendukung Standar EAN-13, Code-128, dan GS1-128 Application Identifiers (AI)
// ============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hasil validasi dan informasi barcode EAN-13
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ean13Barcode {
    pub raw: String,
    pub country_code: String,
    pub manufacturer_code: String,
    pub product_code: String,
    pub check_digit: u8,
    pub is_valid: bool,
}

impl Ean13Barcode {
    /// Menghitung check digit EAN-13 dari 12 digit pertama
    pub fn calculate_checksum(digits12: &str) -> Option<u8> {
        if digits12.len() < 12 || !digits12.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }

        let mut sum = 0;
        for (i, ch) in digits12.chars().take(12).enumerate() {
            let val = ch.to_digit(10)? as u32;
            if i % 2 == 0 {
                sum += val; // Bobot 1 untuk posisi ganjil (1-indexed)
            } else {
                sum += val * 3; // Bobot 3 untuk posisi genap (1-indexed)
            }
        }

        let remainder = sum % 10;
        let check = if remainder == 0 { 0 } else { 10 - remainder };
        Some(check as u8)
    }

    /// Membuat barcode EAN-13 lengkap dari 12 digit dasar
    pub fn from_12_digits(base: &str) -> Result<Self, String> {
        if base.len() != 12 || !base.chars().all(|c| c.is_ascii_digit()) {
            return Err("Input harus tepat 12 digit angka".to_string());
        }

        let check = Self::calculate_checksum(base).ok_or("Gagal menghitung checksum")?;
        let full = format!("{}{}", base, check);
        Self::parse(&full)
    }

    /// Mem-parse dan memvalidasi barcode EAN-13 13 digit
    pub fn parse(raw: &str) -> Result<Self, String> {
        let clean = raw.trim();
        if clean.len() != 13 || !clean.chars().all(|c| c.is_ascii_digit()) {
            return Err("Format barcode EAN-13 harus tepat 13 digit numerik".to_string());
        }

        let expected_check = Self::calculate_checksum(&clean[..12]).ok_or("Checksum error")?;
        let actual_check = clean.chars().last().unwrap().to_digit(10).unwrap() as u8;

        let is_valid = expected_check == actual_check;
        if !is_valid {
            return Err(format!(
                "Check digit EAN-13 tidak valid! Tertera: {}, Ekspektasi: {}",
                actual_check, expected_check
            ));
        }

        let country_code = clean[..3].to_string(); // Contoh: 899 untuk Indonesia
        let manufacturer_code = clean[3..7].to_string();
        let product_code = clean[7..12].to_string();

        Ok(Self {
            raw: clean.to_string(),
            country_code,
            manufacturer_code,
            product_code,
            check_digit: actual_check,
            is_valid,
        })
    }
}

/// Struktur Data GS1-128 dengan Application Identifiers (AI)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gs1Barcode {
    pub raw: String,
    pub gtin: Option<String>,         // AI (01): Global Trade Item Number
    pub batch_lot: Option<String>,    // AI (10): Nomor Batch / Lot Produksi
    pub expiry_date: Option<String>,  // AI (17): Tanggal Kedaluwarsa (YYMMDD)
    pub serial_number: Option<String>,// AI (21): Nomor Seri Satuan
    pub attributes: HashMap<String, String>,
}

impl Gs1Barcode {
    /// Mengurai string GS1 berkode kurung (contoh: "(01)08991234567891(10)LOT-2026(17)261231(21)SER-0081")
    pub fn parse(raw: &str) -> Self {
        let mut attributes = HashMap::new();
        let mut cur = raw.trim();

        while let Some(open) = cur.find('(') {
            if let Some(close) = cur[open..].find(')') {
                let ai = &cur[open + 1..open + close];
                let remainder = &cur[open + close + 1..];

                // Temukan akhir nilai (hingga '(' berikutnya atau akhir string)
                let val_end = remainder.find('(').unwrap_or(remainder.len());
                let val = remainder[..val_end].trim();

                attributes.insert(ai.to_string(), val.to_string());
                cur = &cur[open + close + 1 + val_end..];
            } else {
                break;
            }
        }

        let gtin = attributes.get("01").cloned();
        let batch_lot = attributes.get("10").cloned();
        let expiry_date = attributes.get("17").cloned();
        let serial_number = attributes.get("21").cloned();

        Self {
            raw: raw.to_string(),
            gtin,
            batch_lot,
            expiry_date,
            serial_number,
            attributes,
        }
    }
}
