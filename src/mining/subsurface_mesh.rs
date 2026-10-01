use serde::{Deserialize, Serialize};

/// ============================================================================
/// Underground Wireless Mesh & Toxic Gas Safety Interlock (CO, CH4, H2S)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineGasTelemetry {
    pub sensor_node_id: String,
    pub tunnel_sector: String,
    pub carbon_monoxide_ppm: f64,   // CO (Limit: 25 ppm)
    pub methane_lower_explosive_limit_pct: f64, // CH4 % LEL (Limit: 20% LEL)
    pub hydrogen_sulfide_ppm: f64,  // H2S (Limit: 10 ppm)
    pub oxygen_pct: f64,            // O2 (Safe: 19.5% - 23.5%)
    pub is_evacuation_triggered: bool,
}

pub struct UndergroundMeshSafety {
    pub max_co_ppm: f64,
    pub max_ch4_lel: f64,
    pub max_h2s_ppm: f64,
    pub min_o2_pct: f64,
}

impl UndergroundMeshSafety {
    pub fn new() -> Self {
        Self {
            max_co_ppm: 25.0,
            max_ch4_lel: 10.0,
            max_h2s_ppm: 10.0,
            min_o2_pct: 19.5,
        }
    }

    /// Evaluasi apakah alarm evakuasi darurat bawah tanah harus diaktifkan secara otomatis
    pub fn evaluate_gas_thresholds(&self, telem: &mut MineGasTelemetry) -> bool {
        let co_danger = telem.carbon_monoxide_ppm > self.max_co_ppm;
        let ch4_danger = telem.methane_lower_explosive_limit_pct > self.max_ch4_lel;
        let h2s_danger = telem.hydrogen_sulfide_ppm > self.max_h2s_ppm;
        let o2_danger = telem.oxygen_pct < self.min_o2_pct;

        if co_danger || ch4_danger || h2s_danger || o2_danger {
            telem.is_evacuation_triggered = true;
            true
        } else {
            telem.is_evacuation_triggered = false;
            false
        }
    }
}
