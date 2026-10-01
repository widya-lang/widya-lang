//! Modul Pelindung & Anonymizer Data Pribadi (UU PDP No. 27 Tahun 2022)
//! Menyediakan Dynamic Masking, Tokenisasi Deterministik, dan Audit Sanitasi Data.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JenisDataPdp {
    Nik,
    NomorHp,
    NomorRekening,
    NamaLengkap,
    AlamatEmail,
    RekamMedis,
}

pub struct UuPdpShield;

impl UuPdpShield {
    /// Masking NIK 16 digit: Menampilkan hanya 4 digit terakhir (contoh: "************0001")
    pub fn mask_nik(nik: &str) -> String {
        let s = nik.trim();
        if s.len() <= 4 {
            return "****".to_string();
        }
        let stars = "*".repeat(s.len() - 4);
        format!("{}{}", stars, &s[s.len() - 4..])
    }

    /// Masking Nomor Telepon: Menyembunyikan 4 digit tengah (contoh: "0812****7890")
    pub fn mask_nomor_hp(hp: &str) -> String {
        let s = hp.trim();
        if s.len() < 8 {
            return "****".to_string();
        }
        let prefix = &s[..4];
        let suffix = &s[s.len() - 4..];
        format!("{}****{}", prefix, suffix)
    }

    /// Masking Rekening Bank: Menampilkan 4 digit awal dan 2 digit akhir (contoh: "5210****41")
    pub fn mask_nomor_rekening(rekening: &str) -> String {
        let s = rekening.trim();
        if s.len() <= 6 {
            return "****".to_string();
        }
        let prefix = &s[..4];
        let suffix = &s[s.len() - 2..];
        format!("{}****{}", prefix, suffix)
    }

    /// Masking Email: (contoh: "b***i@widya.id")
    pub fn mask_email(email: &str) -> String {
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() {
            return "***@***.***".to_string();
        }
        let user = parts[0];
        let domain = parts[1];
        if user.len() <= 2 {
            return format!("*@{}", domain);
        }
        let first = &user[..1];
        let last = &user[user.len() - 1..];
        format!("{}***{}@{}", first, last, domain)
    }
}
