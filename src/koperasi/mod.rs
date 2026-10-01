pub mod apotek_desa;
pub mod klinik_desa;
pub mod logistik;
pub mod perkantoran;
pub mod pergudangan;
pub mod simpan_pinjam;
pub mod warung_sembako;

pub use apotek_desa::{GolonganObat, ItemObatApotek, UnitApotekDesa};
pub use klinik_desa::{RekamMedisDesa, StatusGiziBalita, UnitKlinikDesa};
pub use logistik::{JenisKendaraanLogistik, StatusPengiriman, SuratJalanElektronik, UnitLogistikDesa};
pub use perkantoran::{AlokasiShuAnggota, AnggotaKoperasi, LaporanShuKoperasi, StatusKeanggotaan, UnitPerkantoranKoperasi};
pub use pergudangan::{JenisKomoditasGudang, ResiGudangSertifikat, TelemetriSensorSiloGudang, UnitPergudanganDesa};
pub use simpan_pinjam::{JenisAkadPembiayaan, PinjamanAnggota, RekeningTabungan, StatusPinjaman, UnitSimpanPinjam};
pub use warung_sembako::{ItemSembako, KategoriKomoditasSembako, StrukTransaksiSembako, UnitWarungSembako};

/// ============================================================================
/// KOPERASI DESA / KELURAHAN MERAH PUTIH (7 UNIT USAHA TERPADU)
/// ============================================================================

pub struct KoperasiMerahPutihHub {
    pub nama_koperasi: String,
    pub desa_kelurahan: String,
    pub unit_perkantoran: UnitPerkantoranKoperasi,
    pub unit_simpan_pinjam: UnitSimpanPinjam,
    pub unit_warung_sembako: UnitWarungSembako,
    pub unit_klinik_desa: UnitKlinikDesa,
    pub unit_apotek_desa: UnitApotekDesa,
    pub unit_pergudangan: UnitPergudanganDesa,
    pub unit_logistik: UnitLogistikDesa,
}

impl KoperasiMerahPutihHub {
    pub fn new(nama: &str, no_badan_hukum: &str, desa: &str) -> Self {
        Self {
            nama_koperasi: nama.to_string(),
            desa_kelurahan: desa.to_string(),
            unit_perkantoran: UnitPerkantoranKoperasi::new(nama, no_badan_hukum, desa),
            unit_simpan_pinjam: UnitSimpanPinjam::new(),
            unit_warung_sembako: UnitWarungSembako::new(),
            unit_klinik_desa: UnitKlinikDesa::new(&format!("Klinik Pratama {}", nama)),
            unit_apotek_desa: UnitApotekDesa::new(&format!("Apotek {}", nama), "SIA/3301/2026/01"),
            unit_pergudangan: UnitPergudanganDesa::new(&format!("Gudang Komoditas {}", nama), 1000.0),
            unit_logistik: UnitLogistikDesa::new(),
        }
    }
}
