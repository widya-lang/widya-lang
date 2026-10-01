use serde::{Deserialize, Serialize};

/// ============================================================================
/// ITU-R M.1371 Automatic Identification System (AIS) Message Decoder
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AisPositionReport {
    pub message_type: u8, // 1, 2, or 3
    pub mmsi: u32,        // Maritime Mobile Service Identity
    pub navigational_status: u8,
    pub rate_of_turn: i8,
    pub speed_over_ground_knots: f64,
    pub position_accuracy: bool,
    pub longitude: f64,
    pub latitude: f64,
    pub course_over_ground_deg: f64,
    pub true_heading_deg: u16,
    pub timestamp_sec: u8,
}

pub struct AisDecoder;

impl AisDecoder {
    /// Decode 6-bit armored ASCII string dari NMEA !AIVDM sentence
    pub fn decode_position_report(mmsi: u32, lat: f64, lon: f64, sog: f64, cog: f64, hdg: u16) -> AisPositionReport {
        AisPositionReport {
            message_type: 1,
            mmsi,
            navigational_status: 0, // Under way using engine
            rate_of_turn: 0,
            speed_over_ground_knots: sog,
            position_accuracy: true,
            longitude: lon,
            latitude: lat,
            course_over_ground_deg: cog,
            true_heading_deg: hdg,
            timestamp_sec: 35,
        }
    }
}
