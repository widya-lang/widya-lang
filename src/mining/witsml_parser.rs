use serde::{Deserialize, Serialize};

/// ============================================================================
/// Energistics WITSML 2.1 (Wellsite Information Transfer Standard Markup Schema)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryStation {
    pub measured_depth_m: f64,   // MD
    pub true_vertical_depth_m: f64, // TVD
    pub inclination_deg: f64,
    pub azimuth_deg: f64,
    pub dogleg_severity_deg_30m: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellboreLogCurve {
    pub mnemonic: String, // e.g. "GR" (Gamma Ray), "RHOB" (Density), "NPHI" (Neutron Porosity)
    pub unit: String,     // e.g. "gAPI", "g/cm3", "v/v"
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitsmlWellboreReport {
    pub well_name: String,
    pub operator: String,
    pub total_depth_m: f64,
    pub trajectory: Vec<TrajectoryStation>,
    pub log_curves: Vec<WellboreLogCurve>,
}

impl WitsmlWellboreReport {
    pub fn new(well_name: &str, op: &str) -> Self {
        Self {
            well_name: well_name.to_string(),
            operator: op.to_string(),
            total_depth_m: 0.0,
            trajectory: Vec::new(),
            log_curves: Vec::new(),
        }
    }

    pub fn add_station(&mut self, md: f64, tvd: f64, inc: f64, azi: f64) {
        if md > self.total_depth_m {
            self.total_depth_m = md;
        }
        self.trajectory.push(TrajectoryStation {
            measured_depth_m: md,
            true_vertical_depth_m: tvd,
            inclination_deg: inc,
            azimuth_deg: azi,
            dogleg_severity_deg_30m: 1.2,
        });
    }
}
