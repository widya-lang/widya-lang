pub mod bku_transaksi;
pub mod pks_banper_apbn;
pub mod rkas_anggaran;
pub mod siplah_integrasi;
pub mod spj_markas;
pub mod sumber_dana;

pub use bku_transaksi::{
    BukuKasUmumSekolah, JenisTransaksiBku, MetodePembayaranBku, RincianPajakTransaksi,
    TransaksiBkuEntry,
};
pub use pks_banper_apbn::{
    JenisBanperApbn, OtoritasPengesahanApbn, PerjanjianKerjaSamaBanper, PihakKeduaPenerima,
    PihakPertamaPPK, StatusPksBanper, TerminPenyaluranBanper,
};
pub use rkas_anggaran::{
    DokumenRkasTahunan, ItemKegiatanRkas, JenisRevisiAnggaran, KelompokBelanjaAkuntansi,
    LogRevisiRkas, SandinganItemBeforeAfter, StandarNasionalPendidikan,
};
pub use siplah_integrasi::{
    BastDigitalSiplahPermen15, BuktiPembayaranSiplah, DokumenNegosiasiHarga,
    DokumenPerbandinganHarga, DokumenRencanaBelanja, ItemRencanaBelanja, ItemSuratPesanan,
    MetodePbjSiplah, PenawaranCalonPenyedia, RiwayatTawarMenawar, SiplahPbjEngine,
    SuratPesananSiplahPermen15, UjiKualitasFisikBarang,
};
pub use spj_markas::{
    LaporanSpjMarkasBulanan, ManajemenMarkasDinas, RekapRealisasiPerSumberDana,
};
pub use sumber_dana::{
    JenisSumberDana, RekeningBankSekolah, StatusPencairanDana, SumberDanaAlokasi,
};

/// ============================================================================
/// HUB UTAMA ARKAS (APLIKASI RENCANA KEGIATAN & ANGGARAN SEKOLAH)
/// ============================================================================

pub struct ArkasSekolahEngine {
    pub npsn_sekolah: String,
    pub nama_sekolah: String,
    pub tahun_anggaran: u32,
    pub rekening_bank: RekeningBankSekolah,
    pub daftar_sumber_dana: Vec<SumberDanaAlokasi>,
    pub daftar_pks_banper: Vec<PerjanjianKerjaSamaBanper>,
    pub rkas_tahunan: Option<DokumenRkasTahunan>,
    pub buku_kas_umum: Vec<BukuKasUmumSekolah>,
    pub pbj_siplah: SiplahPbjEngine,
}

impl ArkasSekolahEngine {
    pub fn new(
        npsn: &str,
        nama_sekolah: &str,
        tahun: u32,
        rekening: RekeningBankSekolah,
    ) -> Self {
        Self {
            npsn_sekolah: npsn.to_string(),
            nama_sekolah: nama_sekolah.to_string(),
            tahun_anggaran: tahun,
            rekening_bank: rekening,
            daftar_sumber_dana: Vec::new(),
            daftar_pks_banper: Vec::new(),
            rkas_tahunan: None,
            buku_kas_umum: Vec::new(),
            pbj_siplah: SiplahPbjEngine::new("Mitra Resmi SIPLah Kemendikdasmen RI"),
        }
    }


    /// Tambah alokasi sumber dana (BOSP Reguler, BOS Kinerja, BOSDA, Banper APBN, dll.)
    pub fn tambah_sumber_dana(&mut self, alokasi: SumberDanaAlokasi) {
        self.daftar_sumber_dana.push(alokasi);
    }

    /// Tambah PKS Banper APBN yang telah disepakati dengan PPK Kementerian
    pub fn tambah_pks_banper(&mut self, pks: PerjanjianKerjaSamaBanper) {
        self.daftar_pks_banper.push(pks);
    }

    /// Tetapkan RKAS Tahunan
    pub fn set_rkas(&mut self, rkas: DokumenRkasTahunan) {
        self.rkas_tahunan = Some(rkas);
    }

    /// Ambil total dana cair semua sumber dana
    pub fn total_dana_cair_keseluruhan(&self) -> f64 {
        self.daftar_sumber_dana.iter().map(|s| s.jumlah_dana_cair_rp).sum()
    }

    /// Ambil sisa saldo kas semua sumber dana
    pub fn total_sisa_saldo_keseluruhan(&self) -> f64 {
        self.daftar_sumber_dana.iter().map(|s| s.sisa_saldo_rp).sum()
    }
}
