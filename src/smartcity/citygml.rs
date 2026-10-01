use serde::{Deserialize, Serialize};

/// ============================================================================
/// CityGML 3.0 Metropolitan 3D City & Disaster Evacuation Modeling
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityGmlBuildingLod2 {
    pub gml_id: String,
    pub measured_height_m: f64,
    pub roof_type: String, // "Flat", "Gabled", "Hipped"
    pub ground_surface_coords: Vec<(f64, f64)>, // (lat, lon)
    pub occupant_capacity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloodEvacuationRoute {
    pub shelter_name: String,
    pub shelter_capacity: u32,
    pub current_occupancy: u32,
    pub elevation_above_sea_m: f64,
    pub safe_waypoints: Vec<(f64, f64)>,
}

pub struct MetropolitanDigitalTwin {
    pub city_name: String,
    pub buildings: Vec<CityGmlBuildingLod2>,
    pub evacuation_shelters: Vec<FloodEvacuationRoute>,
}

impl MetropolitanDigitalTwin {
    pub fn new(city: &str) -> Self {
        Self {
            city_name: city.to_string(),
            buildings: Vec::new(),
            evacuation_shelters: Vec::new(),
        }
    }

    pub fn add_shelter(&mut self, name: &str, cap: u32, elevation_m: f64, waypoints: Vec<(f64, f64)>) {
        self.evacuation_shelters.push(FloodEvacuationRoute {
            shelter_name: name.to_string(),
            shelter_capacity: cap,
            current_occupancy: 0,
            elevation_above_sea_m: elevation_m,
            safe_waypoints: waypoints,
        });
    }

    /// Evaluasi tempat penampungan evakuasi banjir teraman dengan elevasi tertinggi
    pub fn find_safest_shelters(&self, flood_level_m: f64) -> Vec<&FloodEvacuationRoute> {
        self.evacuation_shelters
            .iter()
            .filter(|s| s.elevation_above_sea_m > flood_level_m + 2.0 && s.current_occupancy < s.shelter_capacity)
            .collect()
    }
}
