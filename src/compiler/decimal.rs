//! Mesin Desimal Presisi Tetap (Fixed-Point Decimal i128) Widya-Lang
//! Dirancang khusus untuk sektor perbankan, fintech (ISO 20022, QRIS), dan perhitungan industri tanpa pembulatan IEEE-754.

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub struct Desimal {
    pub nilai_mentah: i128, // Nilai integer yang dikalikan 10^skala
    pub skala: u8,          // Jumlah digit di belakang koma (default: 4 untuk perbankan, 8 untuk kripto)
}

impl Desimal {
    pub const SKALA_DEFAULT: u8 = 4;
    pub const SKALA_MAKS: u8 = 18;

    /// Membuat desimal baru dengan nilai mentah dan skala tertentu
    pub fn baru(nilai_mentah: i128, skala: u8) -> Self {
        Self { nilai_mentah, skala }
    }

    /// Membuat desimal dari integer (skala 4)
    pub fn dari_int(nilai: i64) -> Self {
        let skala = Self::SKALA_DEFAULT;
        let faktor = 10i128.pow(skala as u32);
        Self {
            nilai_mentah: (nilai as i128) * faktor,
            skala,
        }
    }

    /// Membuat desimal dari floating-point f64
    pub fn dari_f64(nilai: f64) -> Self {
        Self::dari_f64_dengan_skala(nilai, Self::SKALA_DEFAULT)
    }

    pub fn dari_f64_dengan_skala(nilai: f64, skala: u8) -> Self {
        let faktor = 10f64.powi(skala as i32);
        let nilai_mentah = (nilai * faktor).round() as i128;
        Self { nilai_mentah, skala }
    }

    /// Parse dari teks representasi desimal ("12345.6789" atau "1000")
    pub fn dari_teks(teks: &str) -> Result<Self, String> {
        let s = teks.trim();
        if s.is_empty() {
            return Ok(Self::baru(0, Self::SKALA_DEFAULT));
        }

        let negatif = s.starts_with('-');
        let s_bersih = if negatif || s.starts_with('+') { &s[1..] } else { s };

        let bagian: Vec<&str> = s_bersih.split('.').collect();
        if bagian.len() > 2 {
            return Err(format!("Format desimal tidak valid: {}", teks));
        }

        let bagian_bulat: i128 = bagian[0].parse().map_err(|_| format!("Angka bulat tidak valid di: {}", teks))?;
        
        let (bagian_pecahan, skala) = if bagian.len() == 2 {
            let pec_str = bagian[1];
            let skala = pec_str.len() as u8;
            let pec_val: i128 = pec_str.parse().map_err(|_| format!("Angka pecahan tidak valid di: {}", teks))?;
            (pec_val, skala)
        } else {
            (0i128, Self::SKALA_DEFAULT)
        };

        let faktor = 10i128.pow(skala as u32);
        let mut total = bagian_bulat * faktor + bagian_pecahan;
        if negatif {
            total = -total;
        }

        Ok(Self { nilai_mentah: total, skala })
    }

    /// Konversi ke f64
    pub fn ke_f64(&self) -> f64 {
        let faktor = 10f64.powi(self.skala as i32);
        (self.nilai_mentah as f64) / faktor
    }

    /// Ubah skala desimal secara presisi
    pub fn sesuaikan_skala(&self, skala_baru: u8) -> Self {
        if self.skala == skala_baru {
            return *self;
        }

        if skala_baru > self.skala {
            let selisih = (skala_baru - self.skala) as u32;
            let faktor = 10i128.pow(selisih);
            Self {
                nilai_mentah: self.nilai_mentah * faktor,
                skala: skala_baru,
            }
        } else {
            let selisih = (self.skala - skala_baru) as u32;
            let faktor = 10i128.pow(selisih);
            Self {
                nilai_mentah: self.nilai_mentah / faktor,
                skala: skala_baru,
            }
        }
    }

    /// Format ke bentuk mata uang Rupiah ("Rp 1.500.000,50")
    pub fn format_rupiah(&self) -> String {
        let mut n = self.nilai_mentah;
        let negatif = n < 0;
        if negatif {
            n = -n;
        }

        let faktor = 10i128.pow(self.skala as u32);
        let bulat = n / faktor;
        let sisa = n % faktor;

        let s_bulat = bulat.to_string();
        let mut formatted_bulat = String::new();
        let len = s_bulat.len();
        for (i, c) in s_bulat.chars().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                formatted_bulat.push('.');
            }
            formatted_bulat.push(c);
        }

        let s_sisa = format!("{:0width$}", sisa, width = self.skala as usize);
        let prefix = if negatif { "-Rp " } else { "Rp " };
        format!("{}{},{}", prefix, formatted_bulat, &s_sisa[0..2.min(s_sisa.len())])
    }
}

impl Add for Desimal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let target_skala = self.skala.max(rhs.skala);
        let a = self.sesuaikan_skala(target_skala);
        let b = rhs.sesuaikan_skala(target_skala);
        Self {
            nilai_mentah: a.nilai_mentah + b.nilai_mentah,
            skala: target_skala,
        }
    }
}

impl Sub for Desimal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let target_skala = self.skala.max(rhs.skala);
        let a = self.sesuaikan_skala(target_skala);
        let b = rhs.sesuaikan_skala(target_skala);
        Self {
            nilai_mentah: a.nilai_mentah - b.nilai_mentah,
            skala: target_skala,
        }
    }
}

impl Mul for Desimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let target_skala = self.skala.max(rhs.skala);
        let a = self.sesuaikan_skala(target_skala);
        let b = rhs.sesuaikan_skala(target_skala);
        let faktor = 10i128.pow(target_skala as u32);
        Self {
            nilai_mentah: (a.nilai_mentah * b.nilai_mentah) / faktor,
            skala: target_skala,
        }
    }
}

impl Div for Desimal {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.nilai_mentah == 0 {
            panic!("Galat: Pembagian dengan nol pada tipe Desimal Widya-Lang");
        }
        let target_skala = self.skala.max(rhs.skala);
        let a = self.sesuaikan_skala(target_skala);
        let b = rhs.sesuaikan_skala(target_skala);
        let faktor = 10i128.pow(target_skala as u32);
        Self {
            nilai_mentah: (a.nilai_mentah * faktor) / b.nilai_mentah,
            skala: target_skala,
        }
    }
}

impl Neg for Desimal {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            nilai_mentah: -self.nilai_mentah,
            skala: self.skala,
        }
    }
}

impl fmt::Display for Desimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut n = self.nilai_mentah;
        let negatif = n < 0;
        if negatif {
            n = -n;
        }

        let faktor = 10i128.pow(self.skala as u32);
        let bulat = n / faktor;
        let sisa = n % faktor;

        if negatif {
            write!(f, "-")?;
        }
        if self.skala > 0 {
            write!(f, "{}.{:0width$}", bulat, sisa, width = self.skala as usize)
        } else {
            write!(f, "{}", bulat)
        }
    }
}
