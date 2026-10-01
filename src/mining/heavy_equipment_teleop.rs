use serde::{Deserialize, Serialize};

/// ============================================================================
/// Autonomous Heavy Equipment Teleoperation (Dump Trucks & Excavators)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeleoperationControlCommand {
    pub vehicle_id: String,
    pub steering_angle_deg: f64, // -35.0 to +35.0
    pub throttle_pct: f64,       // 0.0 to 100.0
    pub brake_pct: f64,          // 0.0 to 100.0
    pub dump_bed_raise_pct: f64, // 0.0 to 100.0
    pub emergency_stop: bool,
    pub latency_watchdog_timeout_ms: u32,
}

pub struct TeleopController {
    pub active_vehicle_id: String,
    pub last_heartbeat_epoch_ms: u64,
}

impl TeleopController {
    pub fn new(veh_id: &str) -> Self {
        Self {
            active_vehicle_id: veh_id.to_string(),
            last_heartbeat_epoch_ms: 1775000000000,
        }
    }

    pub fn send_command(&mut self, steer: f64, throttle: f64, brake: f64) -> TeleoperationControlCommand {
        TeleoperationControlCommand {
            vehicle_id: self.active_vehicle_id.clone(),
            steering_angle_deg: steer.clamp(-35.0, 35.0),
            throttle_pct: throttle.clamp(0.0, 100.0),
            brake_pct: brake.clamp(0.0, 100.0),
            dump_bed_raise_pct: 0.0,
            emergency_stop: false,
            latency_watchdog_timeout_ms: 100, // Safe automatic stop if connection lost >100ms
        }
    }
}
