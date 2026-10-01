use serde::{Deserialize, Serialize};

/// ============================================================================
/// ISOBUS Task Controller (TC-GEO / TC-SC) Variable Rate Application (VRA)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrescriptionGridCell {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
    pub target_nitrogen_kg_ha: f64,
    pub target_irrigation_mm: f64,
}

pub struct TaskControllerGeo {
    pub prescription_map: Vec<PrescriptionGridCell>,
}

impl TaskControllerGeo {
    pub fn new() -> Self {
        Self {
            prescription_map: Vec::new(),
        }
    }

    pub fn add_zone(&mut self, min_lat: f64, max_lat: f64, min_lon: f64, max_lon: f64, n_kg_ha: f64, water_mm: f64) {
        self.prescription_map.push(PrescriptionGridCell {
            min_lat,
            max_lat,
            min_lon,
            max_lon,
            target_nitrogen_kg_ha: n_kg_ha,
            target_irrigation_mm: water_mm,
        });
    }

    /// Query target pemupukan / penyemprotan otomatis berdasarkan GPS traktor saat ini
    pub fn query_rate_for_position(&self, lat: f64, lon: f64) -> (f64, f64) {
        for cell in &self.prescription_map {
            if lat >= cell.min_lat && lat <= cell.max_lat && lon >= cell.min_lon && lon <= cell.max_lon {
                return (cell.target_nitrogen_kg_ha, cell.target_irrigation_mm);
            }
        }
        (100.0, 20.0) // Nilai default nominal
    }
}
