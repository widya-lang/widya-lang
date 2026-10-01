use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// Smart Livestock Farming (ISO 11784 / ISO 11785 RFID & Health Telemetry)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LivestockSpecies {
    BovineCattle,  // Sapi Potong / Sapi Perah
    OvineSheep,    // Domba
    CaprineGoat,   // Kambing
    PoultryChicken,// Ayam Broiler / Layer
    EquineHorse,   // Kuda
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EstrusHealthStatus {
    Normal,
    SuspectedEstrus, // Deteksi Birahi (Peningkatan mobilitas drastis)
    HeatStress,      // Stres panas lingkungan tinggi (THI > 72)
    RuminationDrop,  // Penurunan waktu memamah biak (indikasi sakit/asidosis)
    Quarantined,     // Karantina penyakit mulut dan kuku (PMK)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimalIdTag {
    pub country_code: u16,   // ISO 3166 code (e.g., 360 for Indonesia)
    pub national_id: u64,    // 12-digit ISO 11784 electronic animal ID
    pub species: LivestockSpecies,
    pub breed: String,       // e.g. "Sapi Limousin", "Sapi Bali", "Sapi FH (Friesian Holstein)"
    pub date_of_birth_iso: String,
    pub mother_id: Option<u64>,
    pub father_id: Option<u64>,
    pub halal_certified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivestockHealthTelemetry {
    pub animal_id: u64,
    pub body_temperature_c: f64,
    pub rumination_minutes_daily: u32, // Normal: 450 - 550 min/day
    pub daily_step_count: u32,
    pub weight_kg: f64,
    pub daily_milk_yield_liters: Option<f64>,
    pub ambient_temp_c: f64,
    pub relative_humidity_pct: f64,
    pub status: EstrusHealthStatus,
}

pub struct PrecisionLivestockManager {
    pub animals: HashMap<u64, AnimalIdTag>,
    pub telemetry_history: HashMap<u64, Vec<LivestockHealthTelemetry>>,
}

impl PrecisionLivestockManager {
    pub fn new() -> Self {
        Self {
            animals: HashMap::new(),
            telemetry_history: HashMap::new(),
        }
    }

    /// Mendaftarkan ternak baru sesuai standar ISO 11784/11785
    pub fn register_animal(
        &mut self,
        national_id: u64,
        species: LivestockSpecies,
        breed: &str,
        dob: &str,
    ) -> AnimalIdTag {
        let tag = AnimalIdTag {
            country_code: 360, // Indonesia
            national_id,
            species,
            breed: breed.to_string(),
            date_of_birth_iso: dob.to_string(),
            mother_id: None,
            father_id: None,
            halal_certified: true,
        };
        self.animals.insert(national_id, tag.clone());
        self.telemetry_history.insert(national_id, Vec::new());
        tag
    }

    /// Menghitung Temperature-Humidity Index (THI) untuk mendeteksi Heat Stress pada ternak
    /// Rumus NRC: THI = (1.8 * T + 32) - (0.55 - 0.0055 * RH) * (1.8 * T - 26)
    pub fn calculate_thi(temp_c: f64, rh_pct: f64) -> (f64, &'static str) {
        let thi = (1.8 * temp_c + 32.0) - (0.55 - 0.0055 * rh_pct) * (1.8 * temp_c - 26.0);
        let category = if thi < 72.0 {
            "Comfort / No Stress"
        } else if thi < 79.0 {
            "Mild Heat Stress"
        } else if thi < 89.0 {
            "Moderate Heat Stress"
        } else {
            "Severe Heat Stress (Critical Danger)"
        };
        (thi, category)
    }

    /// Evaluasi kondisi biometrik real-time & deteksi anomali kesehatan / birahi
    pub fn ingest_telemetry(
        &mut self,
        animal_id: u64,
        body_temp: f64,
        rumination_mins: u32,
        steps: u32,
        weight: f64,
        milk: Option<f64>,
        amb_temp: f64,
        rh_pct: f64,
    ) -> Result<EstrusHealthStatus, String> {
        let (thi, _) = Self::calculate_thi(amb_temp, rh_pct);

        let mut status = EstrusHealthStatus::Normal;

        // Deteksi birahi: lonjakan langkah >2.5x dari baseline rata-rata (misal >4000 langkah)
        if steps > 4000 {
            status = EstrusHealthStatus::SuspectedEstrus;
        } else if rumination_mins < 300 || body_temp > 39.8 {
            status = EstrusHealthStatus::RuminationDrop;
        } else if thi >= 79.0 {
            status = EstrusHealthStatus::HeatStress;
        }

        let record = LivestockHealthTelemetry {
            animal_id,
            body_temperature_c: body_temp,
            rumination_minutes_daily: rumination_mins,
            daily_step_count: steps,
            weight_kg: weight,
            daily_milk_yield_liters: milk,
            ambient_temp_c: amb_temp,
            relative_humidity_pct: rh_pct,
            status,
        };

        if let Some(history) = self.telemetry_history.get_mut(&animal_id) {
            history.push(record);
            Ok(status)
        } else {
            Err(format!("Hewan dengan Tag ID {} belum terdaftar", animal_id))
        }
    }

    /// Hitung Average Daily Gain (ADG) laju pertumbuhan bobot (kg/hari)
    pub fn calculate_adg(&self, animal_id: u64) -> Option<f64> {
        if let Some(history) = self.telemetry_history.get(&animal_id) {
            if history.len() >= 2 {
                let first = history.first().unwrap();
                let last = history.last().unwrap();
                let days = (history.len() - 1) as f64;
                let delta_weight = last.weight_kg - first.weight_kg;
                return Some(delta_weight / days.max(1.0));
            }
        }
        None
    }
}
