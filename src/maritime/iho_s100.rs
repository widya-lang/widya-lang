use serde::{Deserialize, Serialize};

/// ============================================================================
/// IHO S-100 Universal Hydrographic Data Model (S-101 ENC / S-102 Bathymetry)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthContour {
    pub depth_meters: f64,
    pub line_coordinates: Vec<(f64, f64)>, // (lat, lon)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationalHazard {
    pub hazard_id: String,
    pub name: String,
    pub hazard_type: String, // "Wreck", "Obstruction", "Restricted Area"
    pub latitude: f64,
    pub longitude: f64,
    pub clear_depth_meters: f64,
}

pub struct S101ElectronicChart {
    pub dataset_name: String,
    pub scale_range: (u32, u32),
    pub contours: Vec<DepthContour>,
    pub hazards: Vec<NavigationalHazard>,
}

impl S101ElectronicChart {
    pub fn new(name: &str) -> Self {
        Self {
            dataset_name: name.to_string(),
            scale_range: (10000, 50000),
            contours: Vec::new(),
            hazards: Vec::new(),
        }
    }

    /// Verifikasi apakah rute kapal otonom melintasi zona bahaya kedalaman / rintangan navigasi
    pub fn check_route_safety(&self, draft_meters: f64, waypoints: &[(f64, f64)]) -> Result<bool, String> {
        for (lat, lon) in waypoints {
            for hazard in &self.hazards {
                let dist_sq = (lat - hazard.latitude).powi(2) + (lon - hazard.longitude).powi(2);
                if dist_sq < 0.0001 && hazard.clear_depth_meters < (draft_meters + 1.5) {
                    return Err(format!("Peringatan Bahaya Navigasi IHO S-101: Rute mendekati '{}' dengan kedalaman tidak aman {:.1}m", hazard.name, hazard.clear_depth_meters));
                }
            }
        }
        Ok(true)
    }
}
