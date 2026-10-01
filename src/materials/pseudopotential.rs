use serde::{Deserialize, Serialize};

/// ============================================================================
/// Norm-Conserving & Ultrasoft Pseudopotential Model
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PseudopotentialElement {
    pub symbol: String,
    pub atomic_number: u8,
    pub valence_electrons: f64,
    pub core_cutoff_radius_bohr: f64,
    pub functional_type: String, // "PBE", "LDA", "SCAN"
}

impl PseudopotentialElement {
    pub fn new_silicon_pbe() -> Self {
        Self {
            symbol: "Si".to_string(),
            atomic_number: 14,
            valence_electrons: 4.0,
            core_cutoff_radius_bohr: 1.8,
            functional_type: "PBE".to_string(),
        }
    }

    pub fn new_lithium_pbe() -> Self {
        Self {
            symbol: "Li".to_string(),
            atomic_number: 3,
            valence_electrons: 1.0,
            core_cutoff_radius_bohr: 2.1,
            functional_type: "PBE".to_string(),
        }
    }
}
