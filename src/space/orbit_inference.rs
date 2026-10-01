use serde::{Deserialize, Serialize};

/// ============================================================================
/// On-Orbit Edge AI Compute Pipeline for Earth Observation & SAR Satellites
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetDetectionResult {
    pub label: String,
    pub confidence: f32,
    pub bounding_box_lat_lon: [f64; 4], // [min_lat, min_lon, max_lat, max_lon]
    pub anomaly_score: f32,
}

pub struct OnOrbitInferenceEngine {
    pub satellite_id: String,
    pub sensor_mode: String, // e.g. "SAR_STRIPMAP", "MULTISPECTRAL_OPTICAL"
    pub max_vram_mb: u32,
}

impl OnOrbitInferenceEngine {
    pub fn new(sat_id: &str, sensor_mode: &str, max_vram_mb: u32) -> Self {
        Self {
            satellite_id: sat_id.to_string(),
            sensor_mode: sensor_mode.to_string(),
            max_vram_mb,
        }
    }

    /// Melakukan inferensi deteksi objek kapal/kebakaran hutan langsung di orbit
    pub fn detect_anomalies(&self, frame_pixels: &[u8], width: u32, height: u32) -> Vec<TargetDetectionResult> {
        let mut results = Vec::new();
        // Simulasi thresholding & feature extraction terakselerasi NPU orbit
        let total_energy: u64 = frame_pixels.iter().map(|&p| p as u64).sum();
        let avg_intensity = (total_energy as f32) / (frame_pixels.len() as f32);

        if avg_intensity > 120.0 {
            results.push(TargetDetectionResult {
                label: "Thermal Anomaly / Forest Wildfire Hotspot".to_string(),
                confidence: 0.94,
                bounding_box_lat_lon: [-2.5489, 111.0345, -2.5100, 111.0800],
                anomaly_score: 0.88,
            });
        } else {
            results.push(TargetDetectionResult {
                label: "Vessel Detection / Maritime Traffic".to_string(),
                confidence: 0.89,
                bounding_box_lat_lon: [5.8900, 95.3200, 5.9100, 95.3400],
                anomaly_score: 0.12,
            });
        }

        results
    }
}
