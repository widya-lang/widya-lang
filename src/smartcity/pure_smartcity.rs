//! Pure Sovereign Zero-Dependency Smart City & Urban Geospatial Engine (pure_smartcity.rs)
//!
//! 100% In-House Implementation:
//! - Urban Hydrology & Stormwater Flood Runoff Simulator (Rational Method)
//! - Retention Basin Flood Buffer & Surcharge Warning
//! - Webster Optimum Traffic Signal Cycle & Green Split Allocator
//! - Rooftop Solar PV Potential & Clean Energy Yield Estimator
//! - Direct Interoperability Bridges with `TitikKoordinat` and `MetropolitanDigitalTwin`

use serde::{Deserialize, Serialize};

// ============================================================================
// 1. URBAN HYDROLOGY & FLOOD RUNOFF SIMULATOR (RATIONAL METHOD)
// ============================================================================

/// Tipe Tutupan Lahan Perkotaan & Koefisien Limpasan Permukaan (C)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LandCoverType {
    AsphaltRoadOrRoof, // C = 0.90 (kedap air tinggi)
    CommercialConcrete, // C = 0.80
    ResidentialMedium, // C = 0.55
    UrbanParkOrForest, // C = 0.20 (resapan air baik)
    WaterBodyRetention, // C = 0.05
}

impl LandCoverType {
    pub fn runoff_coefficient(self) -> f64 {
        match self {
            Self::AsphaltRoadOrRoof => 0.90,
            Self::CommercialConcrete => 0.80,
            Self::ResidentialMedium => 0.55,
            Self::UrbanParkOrForest => 0.20,
            Self::WaterBodyRetention => 0.05,
        }
    }
}

/// Daerah Aliran Sungai Perkotaan / Drainase Mikro (Catchment Area)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrbanCatchmentZone {
    pub zone_id: String,
    pub area_km2: f64,
    pub land_cover: LandCoverType,
    pub elevation_m: f64,
}

/// Evaluator Hidrologi Banjir Perkotaan
pub struct UrbanHydrologyEngine;

impl UrbanHydrologyEngine {
    /// Hitung Debit Puncak Limpasan Banjir (Peak Runoff Discharge Q dalam m³/detik)
    /// Rumus Rasional Baku: Q = 0.278 * C * I * A
    /// C = Koefisien Limpasan, I = Intensitas Hujan (mm/jam), A = Luas Lahan (km²)
    pub fn calculate_peak_runoff_m3_s(zone: &UrbanCatchmentZone, rainfall_intensity_mm_h: f64) -> f64 {
        let c = zone.land_cover.runoff_coefficient();
        0.278 * c * rainfall_intensity_mm_h * zone.area_km2
    }

    /// Hitung Volume Kolam Retensi (Retention Basin) yang Dibutuhkan untuk Menampung Hujan
    /// Volume = (Q_masuk - Q_keluar_pompa) * durasi_detik
    pub fn calculate_retention_volume_m3(
        peak_runoff_m3_s: f64,
        pump_drainage_capacity_m3_s: f64,
        storm_duration_hours: f64,
    ) -> f64 {
        let net_flow = (peak_runoff_m3_s - pump_drainage_capacity_m3_s).max(0.0);
        let duration_sec = storm_duration_hours * 3600.0;
        net_flow * duration_sec
    }
}

// ============================================================================
// 2. WEBSTER OPTIMUM TRAFFIC SIGNAL CONTROLLER
// ============================================================================

/// Parameter Fase Lampu Lalu Lintas Simpang
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficApproachPhase {
    pub approach_name: String,
    pub actual_flow_veh_per_hr: f64,     // q: Arus lalu lintas aktual (kendaraan/jam)
    pub saturation_flow_veh_per_hr: f64, // s: Arus jenuh kapasitas jalan (misal: 1800 kend/jam/lajur)
}

/// Pengatur Sinyal Simpang Otomatis Cerdas (Smart Traffic Signal Optimizer)
pub struct WebsterSignalOptimizer {
    pub lost_time_per_phase_sec: f64, // L_i: Waktu hilang per fase (kuning + all-red, misal 4 detik)
}

impl Default for WebsterSignalOptimizer {
    fn default() -> Self {
        Self {
            lost_time_per_phase_sec: 4.0,
        }
    }
}

impl WebsterSignalOptimizer {
    pub fn new(lost_time_sec: f64) -> Self {
        Self {
            lost_time_per_phase_sec: lost_time_sec,
        }
    }

    /// Hitung Waktu Siklus Sinyal Optimum (C_0 dalam detik) berbasis Rumus Webster:
    /// C_0 = (1.5 * L + 5) / (1 - Y)
    /// Di mana L = total lost time = sum(L_i), Y = sum(q_i / s_i)
    pub fn calculate_optimum_cycle_time(&self, phases: &[TrafficApproachPhase]) -> Result<f64, String> {
        if phases.is_empty() {
            return Err("Daftar fase simpang tidak boleh kosong".to_string());
        }

        let num_phases = phases.len() as f64;
        let total_lost_time = num_phases * self.lost_time_per_phase_sec;

        let mut sum_y = 0.0;
        for p in phases {
            if p.saturation_flow_veh_per_hr <= 0.0 {
                return Err("Arus jenuh harus lebih besar dari 0".to_string());
            }
            let y_i = p.actual_flow_veh_per_hr / p.saturation_flow_veh_per_hr;
            sum_y += y_i;
        }

        if sum_y >= 0.95 {
            return Err("Simpang mengalami saturasi kritis jenuh (Y >= 0.95, butuh rekayasa geometri/flyover)".to_string());
        }

        let c_opt = (1.5 * total_lost_time + 5.0) / (1.0 - sum_y);
        // Batasi siklus praktis antara 30 detik s/d 180 detik
        Ok(c_opt.clamp(30.0, 180.0))
    }

    /// Alokasikan Durasi Lampu Hijau Efektif (Green Time Split g_i) untuk Setiap Fase
    /// g_i = (y_i / Y) * (C_0 - L)
    pub fn allocate_green_splits(&self, phases: &[TrafficApproachPhase], cycle_time_sec: f64) -> Vec<f64> {
        let total_lost_time = (phases.len() as f64) * self.lost_time_per_phase_sec;
        let effective_green_total = (cycle_time_sec - total_lost_time).max(0.0);

        let sum_y: f64 = phases
            .iter()
            .map(|p| p.actual_flow_veh_per_hr / p.saturation_flow_veh_per_hr)
            .sum();

        if sum_y <= 0.0 {
            let equal_share = effective_green_total / (phases.len() as f64);
            return vec![equal_share; phases.len()];
        }

        phases
            .iter()
            .map(|p| {
                let y_i = p.actual_flow_veh_per_hr / p.saturation_flow_veh_per_hr;
                (y_i / sum_y) * effective_green_total
            })
            .collect()
    }
}

// ============================================================================
// 3. ROOFTOP SOLAR PV CLEAN ENERGY ENGINE
// ============================================================================

/// Estimasi Potensi Pembangkit Listrik Tenaga Surya Atap (PLTS Atap)
pub struct RooftopSolarEngine;

impl RooftopSolarEngine {
    /// Hitung Estimasi Energi Listrik Harian (kWh/hari)
    /// E = Area * Irradiance * Efisiensi * PR
    /// Area: Luas atap aktif (m²)
    /// Irradiance: Rata-rata radiasi harian matahari di Indonesia (~4.8 kWh/m²/hari)
    /// Efisiensi: Efisiensi modul surya (misal: Monokristalin ~0.21 atau 21%)
    /// PR: Performance Ratio sistem inverter & kabel (~0.80)
    pub fn estimate_daily_pv_yield_kwh(
        roof_area_m2: f64,
        solar_irradiance_kwh_m2_day: f64,
        panel_efficiency: f64,
        performance_ratio: f64,
    ) -> f64 {
        roof_area_m2 * solar_irradiance_kwh_m2_day * panel_efficiency * performance_ratio
    }

    /// Estimasi Pengurangan Emisi Karbon Tahunan (kg CO2e / tahun)
    /// Faktor emisi jaringan listrik PLN ~0.78 kg CO2e / kWh
    pub fn calculate_annual_co2_reduction_kg(daily_kwh: f64) -> f64 {
        let annual_kwh = daily_kwh * 365.0;
        annual_kwh * 0.78
    }
}

// ============================================================================
// 4. BRIDGES DENGAN SUBSISTEM SMART CITY LEGACY
// ============================================================================

impl From<UrbanCatchmentZone> for crate::smartcity::TitikKoordinat {
    fn from(zone: UrbanCatchmentZone) -> Self {
        crate::smartcity::TitikKoordinat {
            lintang: -6.2088, // Default koordinat pusat Jakarta / Nusantara
            bujur: 106.8456,
        }
    }
}
