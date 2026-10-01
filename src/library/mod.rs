pub mod interop_global_z3950;
pub mod marc21_dublin_core;
pub mod perpustakaan_daerah;
pub mod perpustakaan_sekolah;
pub mod perpustakaan_tinggi;

pub use interop_global_z3950::{
    IiifManifest, InterlibraryLoanRequest, PerpustakaanInternasionalHub, StatusInterlibraryLoan,
    Z3950QueryRequest, Z3950QueryResponse,
};
pub use marc21_dublin_core::{DublinCoreMetadata, KlasifikasiDdc, Marc21Field, Marc21Record};
pub use perpustakaan_daerah::{
    BuktiSerahSimpanKckr, JenisKoleksiDaerah, LayananPerpustakaanKeliling, NaskahKunoManuskrip,
    PerpustakaanDaerahEngine, StatusKonservasiManuskrip,
};
pub use perpustakaan_sekolah::{
    AnggotaPerpustakaanSekolah, JenisKoleksiSekolah, KoleksiBukuSekolah, PerpustakaanSekolahEngine,
    TransaksiPeminjamanSekolah,
};
pub use perpustakaan_tinggi::{
    JenisKaryaIlmiah, KaryaIlmiahRepository, PerpustakaanTinggiEngine, ResponOaiPmhRecord,
};

/// ============================================================================
/// HUB TERPADU PERPUSTAKAAN (SEKOLAH, DAERAH, TINGGI, INTERNASIONAL)
/// ============================================================================

pub struct SistemPerpustakaanTerpaduHub {
    pub nama_ekosistem: String,
    pub perpus_sekolah_list: Vec<PerpustakaanSekolahEngine>,
    pub perpus_daerah_list: Vec<PerpustakaanDaerahEngine>,
    pub perpus_tinggi_list: Vec<PerpustakaanTinggiEngine>,
    pub interop_internasional: PerpustakaanInternasionalHub,
}

impl SistemPerpustakaanTerpaduHub {
    pub fn new() -> Self {
        Self {
            nama_ekosistem: "Ekosistem Perpustakaan Terpadu Nasional & Global".to_string(),
            perpus_sekolah_list: Vec::new(),
            perpus_daerah_list: Vec::new(),
            perpus_tinggi_list: Vec::new(),
            interop_internasional: PerpustakaanInternasionalHub::new(),
        }
    }
}
