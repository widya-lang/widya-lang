use serde::{Deserialize, Serialize};

/// ============================================================================
/// MANAJEMEN RISIKO PROYEK (RISK REGISTER & MATRIKS 5X5 - ISO 31000 & PMBOK)
/// Penilaian Probabilitas, Dampak, Tingkat Keparahan, dan Strategi Mitigasi
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TingkatKeparahanRisiko {
    Rendah,     // Skor 1 - 4 (Warna Hijau)
    Sedang,     // Skor 5 - 9 (Warna Kuning)
    Tinggi,     // Skor 10 - 14 (Warna Oranye)
    Ekstrem,    // Skor 15 - 25 (Warna Merah Kritis)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StrategiResponRisiko {
    Mitigasi,   // Mitigate: Kurangi kemungkinan/dampak
    Hindari,    // Avoid: Ubah rencana proyek untuk eliminasi ancaman
    Transfer,   // Transfer: Alihkan risiko (Asuransi, Garansi Vendor, Subkontrak)
    Terima,     // Accept: Terima risiko dengan dana kontinjensi
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ButirRisikoProyek {
    pub id_risiko: String,          // e.g. "RSK-01"
    pub deskripsi_ancaman: String,
    pub kategori_risiko: String,    // "Teknis", "Biaya", "Jadwal", "Regulasi", "Vendor"
    pub probabilitas_1_sd_5: u32,   // 1 (Sangat Jarang) - 5 (Hampir Pasti)
    pub dampak_1_sd_5: u32,         // 1 (Tidak Signifikan) - 5 (Bencana/Fatal)
    pub skor_risiko: u32,           // Probabilitas x Dampak (1 - 25)
    pub level_risiko: TingkatKeparahanRisiko,
    pub strategi_respon: StrategiResponRisiko,
    pub rencana_aksi_mitigasi: String,
    pub pemilik_risiko: String,
    pub alokasi_biaya_kontinjensi_rp: f64,
}

pub struct ManajemenRisikoEngine {
    pub daftar_risiko: Vec<ButirRisikoProyek>,
}

impl ManajemenRisikoEngine {
    pub fn new() -> Self {
        Self {
            daftar_risiko: Vec::new(),
        }
    }

    pub fn identifikasi_risiko(
        &mut self,
        id: &str,
        deskripsi: &str,
        kategori: &str,
        probabilitas: u32,
        dampak: u32,
        strategi: StrategiResponRisiko,
        mitigasi: &str,
        pemilik: &str,
        kontinjensi: f64,
    ) -> ButirRisikoProyek {
        let skor = probabilitas * dampak;
        let level = match skor {
            1..=4 => TingkatKeparahanRisiko::Rendah,
            5..=9 => TingkatKeparahanRisiko::Sedang,
            10..=14 => TingkatKeparahanRisiko::Tinggi,
            _ => TingkatKeparahanRisiko::Ekstrem,
        };

        let risiko = ButirRisikoProyek {
            id_risiko: id.to_string(),
            deskripsi_ancaman: deskripsi.to_string(),
            kategori_risiko: kategori.to_string(),
            probabilitas_1_sd_5: probabilitas,
            dampak_1_sd_5: dampak,
            skor_risiko: skor,
            level_risiko: level,
            strategi_respon: strategi,
            rencana_aksi_mitigasi: mitigasi.to_string(),
            pemilik_risiko: pemilik.to_string(),
            alokasi_biaya_kontinjensi_rp: kontinjensi,
        };

        self.daftar_risiko.push(risiko.clone());
        risiko
    }

    pub fn total_dana_cadangan_kontinjensi(&self) -> f64 {
        self.daftar_risiko.iter().map(|r| r.alokasi_biaya_kontinjensi_rp).sum()
    }
}
