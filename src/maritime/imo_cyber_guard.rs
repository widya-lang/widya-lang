use serde::{Deserialize, Serialize};

/// ============================================================================
/// IMO MSC.428(98) Maritime Cybersecurity Shield & GNSS Spoofing Detector
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaritimeCyberThreatReport {
    pub is_spoofing_detected: bool,
    pub confidence_score: f64,
    pub estimated_position_drift_km: f64,
    pub anomaly_reasons: Vec<String>,
}

pub struct ImoCyberGuard {
    pub vessel_mmsi: u32,
    pub last_known_lat: f64,
    pub last_known_lon: f64,
    pub max_physically_possible_speed_knots: f64,
}

impl ImoCyberGuard {
    pub fn new(mmsi: u32) -> Self {
        Self {
            vessel_mmsi: mmsi,
            last_known_lat: -6.10,
            last_known_lon: 106.88,
            max_physically_possible_speed_knots: 35.0, // Commercial container/cargo ship top limit
        }
    }

    /// Deteksi anomali lonjakan koordinat GNSS yang mustahil (Teleportation / Jamming attack)
    pub fn verify_gnss_fix(
        &mut self,
        new_lat: f64,
        new_lon: f64,
        delta_time_sec: f64,
    ) -> MaritimeCyberThreatReport {
        let lat_km = (new_lat - self.last_known_lat).abs() * 111.0;
        let lon_km = (new_lon - self.last_known_lon).abs() * 111.0;
        let dist_km = (lat_km * lat_km + lon_km * lon_km).sqrt();

        let apparent_speed_knots = if delta_time_sec > 0.0 {
            (dist_km / (delta_time_sec / 3600.0)) * 0.539957
        } else {
            0.0
        };

        let mut reasons = Vec::new();
        let is_spoofed = if apparent_speed_knots > self.max_physically_possible_speed_knots {
            reasons.push(format!("Kecepatan semu {:.1} knots melampaui batas fisik kapal {:.1} knots", apparent_speed_knots, self.max_physically_possible_speed_knots));
            true
        } else {
            self.last_known_lat = new_lat;
            self.last_known_lon = new_lon;
            false
        };

        MaritimeCyberThreatReport {
            is_spoofing_detected: is_spoofed,
            confidence_score: if is_spoofed { 0.96 } else { 0.05 },
            estimated_position_drift_km: dist_km,
            anomaly_reasons: reasons,
        }
    }
}
