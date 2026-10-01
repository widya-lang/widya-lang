use super::sumber_dana::JenisSumberDana;
use serde::{Deserialize, Serialize};

/// ============================================================================
/// PELAPORAN SPJ & SINKRONISASI KE SISTEM MARKAS DINAS / KEMENTERIAN
/// Rekonsiliasi Realisasi Belanja, Sisa Kas (SiLPA) & Pengesahan BKU
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RekapRealisasiPerSumberDana {
    pub sumber_dana: JenisSumberDana,
    pub total_penerimaan_rp: f64,
    pub total_belanja_rp: f64,
    pub sisa_kas_rp: f64,
    pub persentase_serapan: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaporanSpjMarkasBulanan {
    pub nomor_rekap_spj: String,
    pub npsn_sekolah: String,
    pub tahun_anggaran: u32,
    pub bulan_laporan: u32,
    pub total_penerimaan_semua_sumber_rp: f64,
    pub total_pengeluaran_semua_sumber_rp: f64,
    pub total_sisa_kas_bank_rp: f64,
    pub total_sisa_kas_tunai_rp: f64,
    pub rekap_per_sumber_dana: Vec<RekapRealisasiPerSumberDana>,
    pub status_verifikasi_markas: String, // "MENUNGGU_VERIFIKASI", "TERVALIDASI_DINAS", "PERLU_REVISI"
    pub tanggal_sinkronisasi: String,
    pub catatan_auditor_dinas: Option<String>,
}

pub struct ManajemenMarkasDinas {
    pub nama_dinas_pendidikan: String,
    pub daftar_laporan_masuk: Vec<LaporanSpjMarkasBulanan>,
}

impl ManajemenMarkasDinas {
    pub fn new(nama_dinas: &str) -> Self {
        Self {
            nama_dinas_pendidikan: nama_dinas.to_string(),
            daftar_laporan_masuk: Vec::new(),
        }
    }

    /// Terima sinkronisasi data SPJ dari ARKAS Sekolah
    pub fn terima_sinkronisasi_arkas(&mut self, laporan: LaporanSpjMarkasBulanan) {
        self.daftar_laporan_masuk.push(laporan);
    }

    /// Verifikasi dan validasi laporan di portal MARKAS
    pub fn validasi_laporan_sekolah(&mut self, no_rekap: &str, valid: bool, catatan: &str) -> bool {
        if let Some(laporan) = self.daftar_laporan_masuk.iter_mut().find(|l| l.nomor_rekap_spj == no_rekap) {
            if valid {
                laporan.status_verifikasi_markas = "TERVALIDASI_DINAS".to_string();
            } else {
                laporan.status_verifikasi_markas = "PERLU_REVISI".to_string();
            }
            laporan.catatan_auditor_dinas = Some(catatan.to_string());
            true
        } else {
            false
        }
    }
}
