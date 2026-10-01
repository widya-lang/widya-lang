use crate::automotive::canbus::CanFrame;
use serde::{Deserialize, Serialize};

/// Data Telemetri Kendaraan Otomotif / EV Standar OBD-II (On-Board Diagnostics)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obd2Telemetry {
    pub engine_rpm: f64,             // PID 0x0C: ((A * 256) + B) / 4
    pub vehicle_speed_kmh: u8,       // PID 0x0D: A km/h
    pub coolant_temp_celsius: i16,   // PID 0x05: A - 40 C
    pub throttle_position_pct: f64,  // PID 0x11: (A * 100) / 255 %
    pub fuel_level_pct: f64,         // PID 0x2F: (A * 100) / 255 %
    pub ev_battery_soc_pct: f64,     // EV State of Charge
}

/// Dekoder Respons CAN Bus OBD-II Standar (CAN ID 0x7E8)
pub fn dekode_obd2_can_frame(frame: &CanFrame) -> Option<(&'static str, f64)> {
    if frame.id != 0x7E8 && frame.id != 0x7E9 {
        return None;
    }

    // Format OBD-II: [Length, Mode, PID, DataA, DataB, ...]
    let mode = frame.data[1];
    let pid = frame.data[2];

    if mode != 0x41 { // Response untuk Show Current Data (Mode 01)
        return None;
    }

    match pid {
        0x0C => { // Engine RPM
            let a = frame.data[3] as u32;
            let b = frame.data[4] as u32;
            let rpm = ((a * 256) + b) as f64 / 4.0;
            Some(("ENGINE_RPM", rpm))
        }
        0x0D => { // Vehicle Speed
            let speed = frame.data[3] as f64;
            Some(("VEHICLE_SPEED_KMH", speed))
        }
        0x05 => { // Coolant Temperature
            let temp = (frame.data[3] as i16) - 40;
            Some(("COOLANT_TEMP_C", temp as f64))
        }
        0x11 => { // Throttle Position
            let pct = (frame.data[3] as f64 * 100.0) / 255.0;
            Some(("THROTTLE_PCT", pct))
        }
        0x2F => { // Fuel Level
            let pct = (frame.data[3] as f64 * 100.0) / 255.0;
            Some(("FUEL_LEVEL_PCT", pct))
        }
        _ => None,
    }
}
