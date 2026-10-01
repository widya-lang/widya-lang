use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// ============================================================================
/// WBS (WORK BREAKDOWN STRUCTURE) & PENJADWALAN CPM / PERT JALUR KRITIS
/// Standar PMI PMBOK & ISO 21500 untuk Analisis Jalur Kritis Proyek
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TipeDependensi {
    FinishToStart, // FS: Aktivitas B baru bisa mulai setelah Aktivitas A selesai
    StartToStart,  // SS: Aktivitas B mulai bersamaan Aktivitas A
    FinishToFinish,// FF: Aktivitas B selesai bersamaan Aktivitas A
    StartToFinish, // SF: Aktivitas B selesai setelah Aktivitas A mulai
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WbsAktivitas {
    pub id_aktivitas: String,       // e.g. "1.1.1"
    pub nama_tugas: String,
    pub durasi_hari: f64,          // Durasi (atau Expected Duration PERT)
    pub dependensi_pendahulu: Vec<String>, // List ID aktivitas pendahulu
    pub biaya_estimasi_rp: f64,
    pub penanggung_jawab: String,
    
    // Hasil Perhitungan CPM (Forward & Backward Pass)
    pub early_start: f64,          // ES
    pub early_finish: f64,         // EF = ES + Durasi
    pub late_start: f64,           // LS = LF - Durasi
    pub late_finish: f64,          // LF
    pub total_slack_float: f64,    // Float = LS - ES = LF - EF
    pub adalah_jalur_kritis: bool, // True jika Total Float == 0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstimasiDurasiPert {
    pub durasi_optimis_hari: f64,   // (o)
    pub durasi_paling_mungkin: f64, // (m)
    pub durasi_pesimis_hari: f64,  // (p)
}

impl EstimasiDurasiPert {
    /// Menghitung Expected Duration TE = (o + 4m + p) / 6 (Distribusi Beta PERT)
    pub fn hitung_expected_duration(&self) -> f64 {
        (self.durasi_optimis_hari + (4.0 * self.durasi_paling_mungkin) + self.durasi_pesimis_hari) / 6.0
    }

    /// Menghitung Varians Durasi Var = ((p - o) / 6)^2
    pub fn hitung_varians(&self) -> f64 {
        let deviasi = (self.durasi_pesimis_hari - self.durasi_optimis_hari) / 6.0;
        deviasi * deviasi
    }
}

pub struct CpmPertEngine {
    pub nama_proyek: String,
    pub daftar_tugas: Vec<WbsAktivitas>,
    pub total_durasi_proyek_hari: f64,
    pub daftar_id_jalur_kritis: Vec<String>,
}

impl CpmPertEngine {
    pub fn new(nama_proyek: &str) -> Self {
        Self {
            nama_proyek: nama_proyek.to_string(),
            daftar_tugas: Vec::new(),
            total_durasi_proyek_hari: 0.0,
            daftar_id_jalur_kritis: Vec::new(),
        }
    }

    pub fn tambah_aktivitas(&mut self, tugas: WbsAktivitas) {
        self.daftar_tugas.push(tugas);
    }

    /// Menjalankan kalkulasi CPM: Forward Pass, Backward Pass, dan Deteksi Jalur Kritis
    pub fn hitung_jalur_kritis_cpm(&mut self) -> Result<f64, String> {
        let n = self.daftar_tugas.len();
        if n == 0 {
            return Ok(0.0);
        }

        // 1. Forward Pass (Kalkulasi ES dan EF)
        // Topological forward evaluation
        for _ in 0..n {
            for i in 0..n {
                let pendahulu = self.daftar_tugas[i].dependensi_pendahulu.clone();
                let durasi = self.daftar_tugas[i].durasi_hari;

                let max_ef_pendahulu = if pendahulu.is_empty() {
                    0.0
                } else {
                    let mut max_ef: f64 = 0.0;
                    for p_id in &pendahulu {
                        if let Some(p) = self.daftar_tugas.iter().find(|t| &t.id_aktivitas == p_id) {
                            if p.early_finish > max_ef {
                                max_ef = p.early_finish;
                            }
                        }
                    }
                    max_ef
                };

                self.daftar_tugas[i].early_start = max_ef_pendahulu;
                self.daftar_tugas[i].early_finish = max_ef_pendahulu + durasi;
            }
        }

        let total_durasi = self.daftar_tugas.iter().map(|t| t.early_finish).fold(0.0, f64::max);
        self.total_durasi_proyek_hari = total_durasi;

        // Inisialisasi LF dengan total durasi
        for t in &mut self.daftar_tugas {
            t.late_finish = total_durasi;
            t.late_start = total_durasi - t.durasi_hari;
        }

        // 2. Backward Pass (Kalkulasi LF, LS dan Total Float)
        for _ in 0..n {
            for i in 0..n {
                let id_curr = self.daftar_tugas[i].id_aktivitas.clone();
                let durasi = self.daftar_tugas[i].durasi_hari;

                // Cari semua aktivitas penerus (successor)
                let penerus: Vec<&WbsAktivitas> = self.daftar_tugas.iter()
                    .filter(|t| t.dependensi_pendahulu.contains(&id_curr))
                    .collect();

                let min_ls_penerus = if penerus.is_empty() {
                    total_durasi
                } else {
                    penerus.iter().map(|s| s.late_start).fold(f64::INFINITY, f64::min)
                };

                self.daftar_tugas[i].late_finish = min_ls_penerus;
                self.daftar_tugas[i].late_start = min_ls_penerus - durasi;
                self.daftar_tugas[i].total_slack_float = min_ls_penerus - durasi - self.daftar_tugas[i].early_start;
                self.daftar_tugas[i].adalah_jalur_kritis = self.daftar_tugas[i].total_slack_float.abs() < 1e-4;
            }
        }

        self.daftar_id_jalur_kritis = self.daftar_tugas.iter()
            .filter(|t| t.adalah_jalur_kritis)
            .map(|t| t.id_aktivitas.clone())
            .collect();

        Ok(total_durasi)
    }
}
