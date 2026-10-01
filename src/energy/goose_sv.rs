use serde::{Deserialize, Serialize};

/// ============================================================================
/// IEC 61850 GOOSE & Sampled Values (SV) High-Speed Tripping Engine (<3ms)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GooseControlBlock {
    pub gocb_ref: String,     // e.g. "Substation1/LLN0$GO$gcb01"
    pub app_id: u16,          // Hex AppID (0x0001 - 0x3FFF)
    pub dat_set: String,      // Dataset Reference
    pub go_id: String,        // GOOSE ID
    pub conf_rev: u32,        // Configuration Revision
    pub state_num: u32,       // StNum (Increments on state change)
    pub sequence_num: u32,    // SqNum (Increments on retransmission)
    pub trip_signal: bool,    // Breaker Trip Command
    pub time_allowed_to_live_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampledValueMeasurement {
    pub sv_id: String,
    pub current_phase_a: f64,
    pub current_phase_b: f64,
    pub current_phase_c: f64,
    pub current_neutral: f64,
    pub voltage_phase_a: f64,
    pub voltage_phase_b: f64,
    pub voltage_phase_c: f64,
    pub smp_count: u16,       // Sample count in cycle (e.g., 0-3999 for 80 samples/cycle)
}

impl GooseControlBlock {
    pub fn new_trip(gocb: &str, app_id: u16, trip: bool, st_num: u32) -> Self {
        Self {
            gocb_ref: gocb.to_string(),
            app_id,
            dat_set: format!("{}/DataSet1", gocb),
            go_id: "TRIP_GCB".to_string(),
            conf_rev: 1,
            state_num: st_num,
            sequence_num: 0,
            trip_signal: trip,
            time_allowed_to_live_ms: 2000,
        }
    }
}
