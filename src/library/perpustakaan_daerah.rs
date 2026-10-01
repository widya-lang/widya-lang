use super::marc21_dublin_core::{DublinCoreMetadata, KlasifikasiDdc};
use serde::{Deserialize, Serialize};

/// ============================================================================
/// SISTEM PERPUSTAKAAN DAERAH (DISPUSSIPDA & PERPUSNAS RI)
/// UU Wajib Serah Simpan KCKR, Naskah Kuno/Manuskrip Nusantara, & Perpus Keliling
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JenisKoleksiDaerah {
    DepositKaryaCetakKaryaRekam, // Wajib Serah Simpan Penerbit Lokal (UU 13/2018)
    NaskahKunoManuskrip,         // Daun Lontar, Daluang, Kertas Eropa, Aksara Tradisional
    KoleksiUmumReferensi,        // Buku Umum, Statistik Daerah (BPS), Monografi
    KoleksiDepositKhususDaerah,  // Babad, Sejarah Lokal, Cerita Budaya Daerah
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusKonservasiManuskrip {
    SangatBaik,
    PerluRestorasiFisik,
    TerkonservasiDigitalTinggi,
    DalamProsesFumigasi,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NaskahKunoManuskrip {
    pub id_manuskrip: String,
    pub judul_naskah: String,
    pub bahasa: String,             // "Jawa Kuno", "Sunda Kuna", "Melayu Kuno", "Arab Pegon", "Bali"
    pub aksara: String,             // "Hanacaraka", "Pegon", "Kawi", "Lontara"
    pub media_tulis: String,        // "Daun Lontar", "Kertas Daluang Pohon Saeh", "Kulit Kayu"
    pub perkiraan_abad: String,     // "Abad ke-16 Masehi", "Tahun 1745 M"
    pub asal_daerah: String,        // "Keraton Surakarta", "Banyumas", "Cirebon", "Singaraja"
    pub kondisi_fisik: StatusKonservasiManuskrip,
    pub resolusi_scan_dpi: u32,     // 600 DPI Master Archival
    pub uri_repositori_iiif: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuktiSerahSimpanKckr {
    pub nomor_tanda_terima: String,
    pub nama_penerbit: String,
    pub alamat_penerbit: String,
    pub judul_karya: String,
    pub isbn_issn: String,
    pub tanggal_serah: String,
    pub jumlah_salinan_diserahkan: u32, // UU 13/2018: 2 eksemplar ke Perpusnas, 1 ke Perpusda
    pub status_deposit_resmi: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayananPerpustakaanKeliling {
    pub id_armada: String,
    pub nomor_polisi: String,
    pub wilayah_rute_desa: String,
    pub jumlah_koleksi_dibawa: u32,
    pub jumlah_pengunjung_terlayani: u32,
    pub tanggal_kegiatan: String,
}

pub struct PerpustakaanDaerahEngine {
    pub nama_lembaga: String,       // e.g. "Dinas Arsip dan Perpustakaan Daerah Kabupaten Banyumas"
    pub tingkat_wilayah: String,     // "Kabupaten/Kota", "Provinsi"
    pub daftar_kckr: Vec<BuktiSerahSimpanKckr>,
    pub koleksi_manuskrip: Vec<NaskahKunoManuskrip>,
    pub rute_keliling: Vec<LayananPerpustakaanKeliling>,
}

impl PerpustakaanDaerahEngine {
    pub fn new(nama_lembaga: &str, tingkat: &str) -> Self {
        Self {
            nama_lembaga: nama_lembaga.to_string(),
            tingkat_wilayah: tingkat.to_string(),
            daftar_kckr: Vec::new(),
            koleksi_manuskrip: Vec::new(),
            rute_keliling: Vec::new(),
        }
    }

    /// Catat serah simpan karya cetak / rekam dari penerbit lokal sesuai UU 13/2018
    pub fn terima_deposit_kckr(
        &mut self,
        no_terima: &str,
        penerbit: &str,
        alamat: &str,
        judul: &str,
        isbn: &str,
        tgl: &str,
        salinan: u32,
    ) -> BuktiSerahSimpanKckr {
        let bukti = BuktiSerahSimpanKckr {
            nomor_tanda_terima: no_terima.to_string(),
            nama_penerbit: penerbit.to_string(),
            alamat_penerbit: alamat.to_string(),
            judul_karya: judul.to_string(),
            isbn_issn: isbn.to_string(),
            tanggal_serah: tgl.to_string(),
            jumlah_salinan_diserahkan: salinan,
            status_deposit_resmi: true,
        };
        self.daftar_kckr.push(bukti.clone());
        bukti
    }

    /// Tambahkan preservasi naskah kuno nusantara
    pub fn daftarkan_manuskrip(&mut self, manuskrip: NaskahKunoManuskrip) {
        self.koleksi_manuskrip.push(manuskrip);
    }

    /// Catat operasional perpustakaan keliling ke desa
    pub fn catat_layanan_keliling(&mut self, layanan: LayananPerpustakaanKeliling) {
        self.rute_keliling.push(layanan);
    }
}
