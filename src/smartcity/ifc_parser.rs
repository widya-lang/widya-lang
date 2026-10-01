use serde::{Deserialize, Serialize};

/// ============================================================================
/// Building Information Modeling (BIM ISO 19650 / IFC 4.3) Parser & Spatial Query
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IfcElementType {
    IfcWall,
    IfcBeam,
    IfcColumn,
    IfcSlab,
    IfcDoor,
    IfcWindow,
    IfcSpace,
    IfcBridge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfcBuildingElement {
    pub global_id: String,
    pub element_type: IfcElementType,
    pub name: String,
    pub volume_m3: f64,
    pub material: String,
    pub bounding_box_min: [f64; 3], // [x, y, z] in meters
    pub bounding_box_max: [f64; 3],
}

pub struct BimModel {
    pub project_name: String,
    pub elements: Vec<IfcBuildingElement>,
}

impl BimModel {
    pub fn new(project: &str) -> Self {
        Self {
            project_name: project.to_string(),
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, id: &str, el_type: IfcElementType, name: &str, vol: f64, mat: &str) {
        self.elements.push(IfcBuildingElement {
            global_id: id.to_string(),
            element_type: el_type,
            name: name.to_string(),
            volume_m3: vol,
            material: mat.to_string(),
            bounding_box_min: [0.0, 0.0, 0.0],
            bounding_box_max: [5.0, 0.3, 3.0],
        });
    }

    /// Hitung total volume beton dan material
    pub fn calculate_total_material_volume(&self, material_filter: &str) -> f64 {
        self.elements
            .iter()
            .filter(|e| e.material.to_lowercase().contains(&material_filter.to_lowercase()))
            .map(|e| e.volume_m3)
            .sum()
    }
}
