use serde::{Deserialize, Serialize};

/// ============================================================================
/// STANDAR SINTA (SCIENCE AND TECHNOLOGY INDEX) & ARJUNA KEMENDIKTISAINTEK RI
/// Akreditasi Jurnal Ilmiah Nasional (SINTA 1 s.d 6) & Metrik Profil Dosen/Peneliti
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PeringkatSinta {
    Sinta1, // Akreditasi A / Terindeks Scopus / WoS Utama (Nilai 85 - 100)
    Sinta2, // Akreditasi B / Jurnal Nasional Bereputasi Tinggi (Nilai 70 - 85)
    Sinta3, // Akreditasi Jurnal Nasional Terakreditasi (Nilai 60 - 70)
    Sinta4, // Akreditasi Jurnal Nasional Terakreditasi (Nilai 50 - 60)
    Sinta5, // Akreditasi Jurnal Nasional Tingkat Awal (Nilai 40 - 50)
    Sinta6, // Akreditasi Jurnal Nasional Rintisan (Nilai 30 - 40)
    NonSinta,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AkreditasiJurnalArjuna {
    pub nama_jurnal: String,
    pub e_issn: String,
    pub nomor_sk_akreditasi: String,
    pub peringkat_sinta: PeringkatSinta,
    pub masa_berlaku_mulai_tahun: u32,
    pub masa_berlaku_sampai_tahun: u32,
    pub nilai_evaluasi_arjuna: f64,
    pub penerbit_lembaga: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfilPenelitiSinta {
    pub sinta_id: String,
    pub nidn_nidk: String,
    pub nama_dosen: String,
    pub perguruan_tinggi: String,
    pub program_studi: String,
    pub sinta_score_overall: f64,
    pub sinta_score_3yr: f64,
    pub sinta_score_productivity: f64,
    pub total_sitasi_google_scholar: u32,
    pub h_index_google_scholar: u32,
    pub scopus_author_id: Option<String>,
    pub total_dokumen_scopus: u32,
    pub total_sitasi_scopus: u32,
    pub h_index_scopus: u32,
    pub wos_researcher_id: Option<String>,
    pub garuda_id: Option<String>,
}

impl ProfilPenelitiSinta {
    /// Hitung ulang skor produktivitas 3 tahun terakhir
    pub fn perbarui_skor_sinta(&mut self, publikasi_q1_scopus: u32, publikasi_sinta2: u32) {
        // Pembobotan SINTA: Q1 Scopus = 40 poin, SINTA 2 = 25 poin, sitasi Scopus = 4 poin
        let skor_tambahan = (publikasi_q1_scopus as f64 * 40.0) + (publikasi_sinta2 as f64 * 25.0);
        self.sinta_score_3yr += skor_tambahan;
        self.sinta_score_overall += skor_tambahan;
    }
}
