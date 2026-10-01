pub mod bibliometrics;
pub mod ojs_article;
pub mod scopus_wos_index;
pub mod sinta_arjuna;

pub use bibliometrics::{BibliometricEngine, RiwayatPublikasiSitasi};
pub use ojs_article::{
    ArtikelJurnalIlmiah, PenulisJurnal, ReviewerRecord, StatusReviewArtikel,
};
pub use scopus_wos_index::{
    MetrikScopusJurnal, MetrikWebOfScienceJurnal, ScopusQuartile, StatusDoajIndex,
    WebOfScienceIndex,
};
pub use sinta_arjuna::{AkreditasiJurnalArjuna, PeringkatSinta, ProfilPenelitiSinta};

/// ============================================================================
/// HUB UTAMA JURNAL ILMIAH, SINTA & INDEKSASI INTERNASIONAL
/// ============================================================================

pub struct SistemJurnalIlmiahHub {
    pub nama_portal: String,
    pub daftar_artikel: Vec<ArtikelJurnalIlmiah>,
    pub daftar_jurnal_sinta: Vec<AkreditasiJurnalArjuna>,
    pub daftar_jurnal_scopus: Vec<MetrikScopusJurnal>,
    pub daftar_peneliti: Vec<ProfilPenelitiSinta>,
}

impl SistemJurnalIlmiahHub {
    pub fn new() -> Self {
        Self {
            nama_portal: "Portal Publikasi Ilmiah Widya, SINTA & Scopus Hub".to_string(),
            daftar_artikel: Vec::new(),
            daftar_jurnal_sinta: Vec::new(),
            daftar_jurnal_scopus: Vec::new(),
            daftar_peneliti: Vec::new(),
        }
    }
}
