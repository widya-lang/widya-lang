pub mod agile_scrum;
pub mod evm_metrics;
pub mod risk_register;
pub mod wbs_cpm_pert;

pub use agile_scrum::{PrioritasMoscow, SprintScrum, StatusKanban, UserStory};
pub use evm_metrics::{EvmEngine, HasilAnalisisEvm, TitikKurvaS};
pub use risk_register::{
    ButirRisikoProyek, ManajemenRisikoEngine, StrategiResponRisiko, TingkatKeparahanRisiko,
};
pub use wbs_cpm_pert::{CpmPertEngine, EstimasiDurasiPert, TipeDependensi, WbsAktivitas};

/// ============================================================================
/// HUB UTAMA MANAJEMEN PROYEK (PMBOK, PRINCE2, AGILE & EVM)
/// ============================================================================

pub struct SistemManajemenProyekHub {
    pub id_proyek: String,
    pub nama_proyek: String,
    pub manajer_proyek: String,
    pub cpm_engine: CpmPertEngine,
    pub risk_engine: ManajemenRisikoEngine,
    pub daftar_sprint: Vec<SprintScrum>,
    pub titik_kurva_s: Vec<TitikKurvaS>,
}

impl SistemManajemenProyekHub {
    pub fn new(id: &str, nama: &str, pm: &str) -> Self {
        Self {
            id_proyek: id.to_string(),
            nama_proyek: nama.to_string(),
            manajer_proyek: pm.to_string(),
            cpm_engine: CpmPertEngine::new(nama),
            risk_engine: ManajemenRisikoEngine::new(),
            daftar_sprint: Vec::new(),
            titik_kurva_s: Vec::new(),
        }
    }
}
