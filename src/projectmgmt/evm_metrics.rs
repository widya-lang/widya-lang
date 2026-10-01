use serde::{Deserialize, Serialize};

/// ============================================================================
/// EARNED VALUE MANAGEMENT (EVM) & ANALISIS KINERJA KURVA S (ISO 21508)
/// Metrik Kinerja Biaya (CPI), Waktu (SPI), Varians (CV/SV), dan Estimasi (EAC)
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TitikKurvaS {
    pub minggu_ke: u32,
    pub tanggal_evaluasi: String,
    pub planned_value_pv_rp: f64,  // PV / BCWS (Rencana Anggaran Kumulatif)
    pub earned_value_ev_rp: f64,   // EV / BCWP (Nilai Hasil Fisik Aktual)
    pub actual_cost_ac_rp: f64,    // AC / ACWP (Biaya Aktual Pengeluaran)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HasilAnalisisEvm {
    pub budget_at_completion_bac_rp: f64, // Total Pagu Anggaran Proyek
    pub planned_value_pv: f64,
    pub earned_value_ev: f64,
    pub actual_cost_ac: f64,
    
    // Varians Kinerja
    pub cost_variance_cv_rp: f64,         // CV = EV - AC (> 0: Hemat/Under Budget)
    pub schedule_variance_sv_rp: f64,     // SV = EV - PV (> 0: Cepat/Ahead of Schedule)
    
    // Indeks Efisiensi
    pub cost_performance_index_cpi: f64,  // CPI = EV / AC (> 1.0: Efisien Biaya)
    pub schedule_performance_index_spi: f64, // SPI = EV / PV (> 1.0: Efisien Waktu)
    
    // Proyeksi Penyelesaian
    pub estimate_at_completion_eac_rp: f64, // EAC = BAC / CPI
    pub estimate_to_complete_etc_rp: f64,   // ETC = EAC - AC
    pub variance_at_completion_vac_rp: f64, // VAC = BAC - EAC
    pub to_complete_performance_index_tcpi: f64, // TCPI = (BAC - EV) / (BAC - AC)
    
    pub status_kesehatan_proyek: String,
}

pub struct EvmEngine;

impl EvmEngine {
    /// Hitung seluruh metrik EVM berdasarkan data realisasi saat ini
    pub fn evaluasi_kinerja_evm(bac: f64, pv: f64, ev: f64, ac: f64) -> HasilAnalisisEvm {
        let cv = ev - ac;
        let sv = ev - pv;
        let cpi = if ac > 0.0 { ev / ac } else { 1.0 };
        let spi = if pv > 0.0 { ev / pv } else { 1.0 };

        let eac = if cpi > 0.0 { bac / cpi } else { bac };
        let etc = if eac >= ac { eac - ac } else { 0.0 };
        let vac = bac - eac;
        let tcpi = if (bac - ac) > 0.0 { (bac - ev) / (bac - ac) } else { 1.0 };

        let status = match (cpi >= 1.0, spi >= 1.0) {
            (true, true) => "SANGAT BAIK: Di Bawah Anggaran & Lebih Cepat dari Jadwal (Under Budget & Ahead of Schedule)".to_string(),
            (true, false) => "PERINGATAN JADWAL: Di Bawah Anggaran tetapi Mengalami Keterlambatan (Under Budget & Behind Schedule)".to_string(),
            (false, true) => "PERINGATAN BIAYA: Biaya Membengkak tetapi Jadwal Lebih Cepat (Over Budget & Ahead of Schedule)".to_string(),
            (false, false) => "KRITIS: Biaya Membengkak dan Jadwal Terlambat (Over Budget & Behind Schedule)".to_string(),
        };

        HasilAnalisisEvm {
            budget_at_completion_bac_rp: bac,
            planned_value_pv: pv,
            earned_value_ev: ev,
            actual_cost_ac: ac,
            cost_variance_cv_rp: cv,
            schedule_variance_sv_rp: sv,
            cost_performance_index_cpi: cpi,
            schedule_performance_index_spi: spi,
            estimate_at_completion_eac_rp: eac,
            estimate_to_complete_etc_rp: etc,
            variance_at_completion_vac_rp: vac,
            to_complete_performance_index_tcpi: tcpi,
            status_kesehatan_proyek: status,
        }
    }
}
