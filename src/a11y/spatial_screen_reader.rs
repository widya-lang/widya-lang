use serde::{Deserialize, Serialize};

/// ============================================================================
/// AI Multimodal Screen Reader with 3D Spatial Audio Coordinate Guidance
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialAudioCue {
    pub spoken_text: String,
    pub azimuth_deg: f64,   // -90 (far left) to +90 (far right)
    pub elevation_deg: f64, // -45 (bottom of screen) to +45 (top of screen)
    pub distance_meters: f64,
}

pub struct SpatialScreenReader {
    pub reading_speed_wpm: u32,
    pub pitch: f64,
}

impl SpatialScreenReader {
    pub fn new() -> Self {
        Self {
            reading_speed_wpm: 220,
            pitch: 1.0,
        }
    }

    /// Petakan elemen UI visual (tombol, input, link) ke koordinat spasial audio 3D
    pub fn describe_ui_element(&self, label: &str, role: &str, screen_x_pct: f64, screen_y_pct: f64) -> SpatialAudioCue {
        let azimuth = (screen_x_pct - 0.5) * 120.0; // -60 to +60 deg
        let elevation = (0.5 - screen_y_pct) * 60.0; // +30 to -30 deg
        let text = format!("{}: {}", role, label);

        SpatialAudioCue {
            spoken_text: text,
            azimuth_deg: azimuth,
            elevation_deg: elevation,
            distance_meters: 1.0,
        }
    }
}
