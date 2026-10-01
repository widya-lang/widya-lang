pub mod dapur_sppg;
pub mod gizi_akg;
pub mod penerima_manfaat;
pub mod siplah_supplier_marketplace;
pub mod yayasan_bantuan;

pub use dapur_sppg::{BatchMasakSppg, DapurSentralSppg, PasokanBahanBakuLokal};
pub use gizi_akg::{KelompokSasaranMbg, KomposisiGiziPorsi, MenuHarianMbg, StandarGiziEngine};
pub use penerima_manfaat::{
    LogPresensiPorsiHarian, ManajemenPenerimaManfaat, SiswaPenerimaManfaat, StatusImtSiswa,
};
pub use siplah_supplier_marketplace::{
    BastDigitalSIPLah, KategoriBahanPangan, LegalitasPenyedia, PenyediaBahanMakanan,
    ProdukPanganSIPLah, SIPLahPanganEngine, StatusPesananSIPLah, StatusVerifikasiPenyedia,
    SuratPesananSIPLah, UjiKualitasTerimaPangan,
};
pub use yayasan_bantuan::{
    JenisLembagaPenerima, ManajemenBantuanYayasan, SpjBuktiBelanjaDigital, YayasanPenerimaBantuan,
};

/// ============================================================================
/// HUB TERPADU PROGRAM MAKAN BERGIZI GRATIS (MBG), SPPG & YAYASAN PENERIMA
/// ============================================================================

pub struct ProgramMbgNationalHub {
    pub nama_program: String,
    pub dapur_sppg_list: Vec<DapurSentralSppg>,
    pub yayasan_manajemen: ManajemenBantuanYayasan,
    pub penerima_manfaat: ManajemenPenerimaManfaat,
    pub marketplace_siplah: SIPLahPanganEngine,
}

impl ProgramMbgNationalHub {
    pub fn new() -> Self {
        Self {
            nama_program: "Program Nasional Makan Bergizi Gratis (MBG) & Dapur SPPG".to_string(),
            dapur_sppg_list: Vec::new(),
            yayasan_manajemen: ManajemenBantuanYayasan::new(),
            penerima_manfaat: ManajemenPenerimaManfaat::new(),
            marketplace_siplah: SIPLahPanganEngine::new("SIPLah Pangan MBG & Kemendikdasmen RI"),
        }
    }
}

