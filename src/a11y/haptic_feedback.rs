use serde::{Deserialize, Serialize};

/// ============================================================================
/// Variable Frequency Neural Haptic Actuator Driver (Tactile Guidance)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HapticPattern {
    SubtleClick,
    SuccessDoublePulse,
    WarningVibration,
    ErrorHeavyBuzz,
    EdgeBoundaryWarning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticImpulse {
    pub frequency_hz: f64,
    pub amplitude_pct: f64,
    pub duration_ms: u32,
}

pub struct HapticController;

impl HapticController {
    pub fn trigger_pattern(pattern: HapticPattern) -> HapticImpulse {
        match pattern {
            HapticPattern::SubtleClick => HapticImpulse {
                frequency_hz: 180.0,
                amplitude_pct: 30.0,
                duration_ms: 25,
            },
            HapticPattern::SuccessDoublePulse => HapticImpulse {
                frequency_hz: 220.0,
                amplitude_pct: 60.0,
                duration_ms: 120,
            },
            HapticPattern::WarningVibration => HapticImpulse {
                frequency_hz: 140.0,
                amplitude_pct: 75.0,
                duration_ms: 250,
            },
            HapticPattern::ErrorHeavyBuzz => HapticImpulse {
                frequency_hz: 90.0,
                amplitude_pct: 100.0,
                duration_ms: 400,
            },
            HapticPattern::EdgeBoundaryWarning => HapticImpulse {
                frequency_hz: 280.0,
                amplitude_pct: 85.0,
                duration_ms: 80,
            },
        }
    }
}
