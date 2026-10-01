use serde::{Deserialize, Serialize};

/// ============================================================================
/// INDEKSASI INTERNASIONAL BEREPUTASI (SCOPUS, WEB OF SCIENCE, DOAJ)
/// Quartile Q1-Q4, CiteScore, SJR, Impact Factor (JIF) & DOAJ Seal
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScopusQuartile {
    Q1, // Top 25% Jurnal di Bidangnya (Kategori Paling Bergengsi)
    Q2, // Top 25% - 50%
    Q3, // Top 50% - 75%
    Q4, // Bottom 25%
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WebOfScienceIndex {
    ScienceCitationIndexExpanded, // SCIE
    SocialSciencesCitationIndex,  // SSCI
    ArtsAndHumanitiesCitationIndex, // AHCI
    EmergingSourcesCitationIndex, // ESCI
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetrikScopusJurnal {
    pub nama_jurnal: String,
    pub scopus_source_id: String,
    pub issn: String,
    pub bidang_subjek: String,      // e.g. "Computer Science (miscellaneous)"
    pub quartile: ScopusQuartile,
    pub citescore: f64,             // e.g. 8.4
    pub scimago_sjr: f64,           // e.g. 1.85
    pub snip: f64,                  // e.g. 2.15
    pub persentil_peringkat: f64,   // e.g. 94.0%
    pub penerbit_internasional: String, // e.g. "Elsevier", "IEEE", "Springer Nature"
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetrikWebOfScienceJurnal {
    pub nama_jurnal: String,
    pub wos_accession_id: String,
    pub indeks_koleksi: WebOfScienceIndex,
    pub journal_impact_factor_jif: f64, // e.g. 4.62
    pub journal_citation_indicator_jci: f64,
    pub kategori_wos: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusDoajIndex {
    pub terdaftar_doaj: bool,
    pub doaj_seal_awarded: bool,    // Penghargaan tertinggi transparansi Open Access
    pub apc_amount_usd: f64,        // 0.0 jika Diamond Open Access (Gratis)
    pub lisensi_resmi: String,
}
