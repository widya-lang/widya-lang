use serde::{Deserialize, Serialize};

/// Titik Data Sensor Agroindustri & Smart City
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgroSensorTelemetry {
    pub device_id: String,
    pub timestamp_epoch: u64,
    pub soil_moisture_percent: f64, // Kelembaban tanah (0.0 .. 100.0%)
    pub temperature_celsius: f64,    // Suhu udara
    pub soil_ph: f64,                // pH tanah (0.0 .. 14.0)
    pub ambient_humidity_percent: f64,
    pub battery_level_percent: f64,
}

impl AgroSensorTelemetry {
    pub fn is_irrigation_needed(&self) -> bool {
        self.soil_moisture_percent < 30.0 && self.temperature_celsius > 28.0
    }
}

/// Geofencing Perimeter Check untuk Pemantauan Alat Berat / Ternak / Drone
pub fn periksa_geofence(lat: f64, lon: f64, center_lat: f64, center_lon: f64, radius_meter: f64) -> bool {
    let d_lat = (lat - center_lat).to_radians();
    let d_lon = (lon - center_lon).to_radians();
    let a = (d_lat / 2.0).sin().powi(2)
        + center_lat.to_radians().cos() * lat.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let distance_meter = 6371000.0 * c; // Radius bumi 6.371 km

    distance_meter <= radius_meter
}
