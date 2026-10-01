use serde::{Deserialize, Serialize};

/// ============================================================================
/// NMEA 2000 (IEC 61162-3) CAN Bus Parameter Group Number (PGN) Decoder
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nmea2000Pgn {
    SystemTime = 126992,
    VesselHeading = 127250,
    Speed = 128259,
    PositionRapidUpdate = 129025,
    CogSogRapidUpdate = 129026,
    GnssPositionData = 129029,
    WindData = 130306,
    EngineParameters = 127488,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VesselNavigationState {
    pub latitude: f64,
    pub longitude: f64,
    pub speed_over_ground_knots: f64,
    pub course_over_ground_deg: f64,
    pub heading_true_deg: f64,
    pub wind_speed_knots: f64,
    pub wind_angle_deg: f64,
}

impl VesselNavigationState {
    pub fn default() -> Self {
        Self {
            latitude: -6.1012, // Pelabuhan Tanjung Priok
            longitude: 106.8834,
            speed_over_ground_knots: 14.5,
            course_over_ground_deg: 45.0,
            heading_true_deg: 44.8,
            wind_speed_knots: 12.0,
            wind_angle_deg: 30.0,
        }
    }
}

pub fn parse_nmea2000_position_rapid(payload: &[u8]) -> Result<(f64, f64), String> {
    if payload.len() < 8 {
        return Err("Payload PGN 129025 terlalu pendek (< 8 bytes)".to_string());
    }

    let lat_raw = i32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    let lon_raw = i32::from_le_bytes([payload[4], payload[5], payload[6], payload[7]]);

    let lat = (lat_raw as f64) * 1e-7;
    let lon = (lon_raw as f64) * 1e-7;

    Ok((lat, lon))
}
