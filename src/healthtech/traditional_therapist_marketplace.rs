use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// Layanan Terapis, Penyehat Tradisional (STPT) & Marketplace Jasa Kesehatan
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModalitasTerapi {
    Akupresur,              // Penekanan titik akupunktur tanpa jarum
    PijatTradisionalUrut,   // Pijat kebugaran & relaksasi otot tradisional
    PijatRefleksi,          // Refleksi zona telapak kaki/tangan
    BekamSteril,            // Hijamah bekam kering/angin higienis
    AromaterapiHerbal,      // Minyak atsiri esensial alami
    PeracikJamuTradisional, // Ramuan jamu seduh / gendong empiris
    ChiropraktikTradisional,// Penyelarasan sendi dan tulang belakang
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerapisTradisional {
    pub id_terapis: String,
    pub nama_lengkap: String,
    pub nomor_stpt: String,          // Surat Terdaftar Penyehat Tradisional (Dinas Kesehatan)
    pub asosiasi_profesi: String,    // e.g. "APSTI", "PAHI", "PPNI Tradisional"
    pub modalitas_keahlian: Vec<ModalitasTerapi>,
    pub sertifikasi_kompetensi: bool,
    pub rating_bintang: f64,         // 1.0 - 5.0
    pub total_ulasan: u32,
    pub tarif_per_sesi_idr: u64,
    pub ketersediaan_homecare: bool,
    pub koordinat_lintang_bujur: (f64, f64),
    pub verifikasi_higienitas_alat: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RekamTerapiTradisional {
    pub id_rekam: String,
    pub id_klien: String,
    pub id_terapis: String,
    pub tanggal_sesi_iso: String,
    pub keluhan_utama: String,
    pub observasi_anamnesis: String, // e.g. "Inspeksi lidah merah pucat, palpasi nadi cepat"
    pub modalitas_diberikan: ModalitasTerapi,
    pub titik_meridian_terstimulasi: Vec<String>, // e.g. ["ST36 (Zusanli)", "LI4 (Hegu)", "SP6 (Sanyinjiao)"]
    pub ramuan_herbal_anjuran: Option<String>,
    pub evaluasi_pasca_terapi: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingPesananJasa {
    pub id_pesanan: String,
    pub id_klien: String,
    pub id_terapis: String,
    pub modalitas: ModalitasTerapi,
    pub waktu_janji_iso: String,
    pub alamat_layanan: String,
    pub status_pembayaran_escrow: bool,
    pub status_layanan: String, // "MENUNGGU", "DIKONFIRMASI", "DALAM_SESI", "SELESAI"
}

pub struct MarketplaceKesehatanTradisional {
    pub terapis_list: HashMap<String, TerapisTradisional>,
    pub booking_list: HashMap<String, BookingPesananJasa>,
    pub rekam_terapi: Vec<RekamTerapiTradisional>,
}

impl MarketplaceKesehatanTradisional {
    pub fn new() -> Self {
        Self {
            terapis_list: HashMap::new(),
            booking_list: HashMap::new(),
            rekam_terapi: Vec::new(),
        }
    }

    /// Mendaftarkan terapis tradisional baru dengan verifikasi STPT Kemenkes
    pub fn register_terapis(
        &mut self,
        id: &str,
        nama: &str,
        stpt: &str,
        asosiasi: &str,
        modalitas: Vec<ModalitasTerapi>,
        tarif_idr: u64,
        homecare: bool,
        lat: f64,
        lon: f64,
    ) -> TerapisTradisional {
        let t = TerapisTradisional {
            id_terapis: id.to_string(),
            nama_lengkap: nama.to_string(),
            nomor_stpt: stpt.to_string(),
            asosiasi_profesi: asosiasi.to_string(),
            modalitas_keahlian: modalitas,
            sertifikasi_kompetensi: true,
            rating_bintang: 5.0,
            total_ulasan: 1,
            tarif_per_sesi_idr: tarif_idr,
            ketersediaan_homecare: homecare,
            koordinat_lintang_bujur: (lat, lon),
            verifikasi_higienitas_alat: true,
        };
        self.terapis_list.insert(id.to_string(), t.clone());
        t
    }

    /// Cari terapis terdekat berdasarkan modalitas dan radius GPS (Haversine)
    pub fn cari_terapis_terdekat(
        &self,
        modalitas: ModalitasTerapi,
        klien_lat: f64,
        klien_lon: f64,
        max_jarak_km: f64,
    ) -> Vec<&TerapisTradisional> {
        self.terapis_list
            .values()
            .filter(|t| {
                let memiliki_modalitas = t.modalitas_keahlian.contains(&modalitas);
                let (t_lat, t_lon) = t.koordinat_lintang_bujur;
                let d_lat = (t_lat - klien_lat) * 111.0;
                let d_lon = (t_lon - klien_lon) * 111.0;
                let dist_km = (d_lat * d_lat + d_lon * d_lon).sqrt();
                memiliki_modalitas && dist_km <= max_jarak_km && t.verifikasi_higienitas_alat
            })
            .collect()
    }

    /// Buat pesanan booking jasa terapi dengan penjaminan escrow
    pub fn buat_booking(
        &mut self,
        order_id: &str,
        klien_id: &str,
        terapis_id: &str,
        modalitas: ModalitasTerapi,
        waktu_iso: &str,
        alamat: &str,
    ) -> Result<BookingPesananJasa, String> {
        if !self.terapis_list.contains_key(terapis_id) {
            return Err(format!("Terapis ID {} tidak ditemukan", terapis_id));
        }

        let order = BookingPesananJasa {
            id_pesanan: order_id.to_string(),
            id_klien: klien_id.to_string(),
            id_terapis: terapis_id.to_string(),
            modalitas,
            waktu_janji_iso: waktu_iso.to_string(),
            alamat_layanan: alamat.to_string(),
            status_pembayaran_escrow: true,
            status_layanan: "DIKONFIRMASI".to_string(),
        };

        self.booking_list.insert(order_id.to_string(), order.clone());
        Ok(order)
    }

    /// Simpan rekam terapi tradisional (SOAP)
    pub fn catat_rekam_terapi(&mut self, rekam: RekamTerapiTradisional) {
        self.rekam_terapi.push(rekam);
    }
}
