use serde::{Deserialize, Serialize};

/// ============================================================================
/// Real-Time DSP EEG Band Power Extractor (Delta, Theta, Alpha, Beta, Gamma)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EegBandPowers {
    pub delta_0_5_4hz: f64,  // Deep sleep
    pub theta_4_8hz: f64,    // Drowsiness, meditation
    pub alpha_8_13hz: f64,   // Relaxed focus, eyes closed
    pub beta_13_30hz: f64,   // Active thinking, problem solving
    pub gamma_30_100hz: f64, // High-level cognitive binding
    pub engagement_index: f64, // Beta / (Alpha + Theta)
}

pub struct EegFilterEngine {
    pub sample_rate_hz: f64,
}

impl EegFilterEngine {
    pub fn new(srate: f64) -> Self {
        Self {
            sample_rate_hz: srate,
        }
    }

    /// Estimasi band power spectrum menggunakan Discrete Fourier Transform / Welch sederhana
    pub fn extract_band_powers(&self, signal_window: &[f64]) -> EegBandPowers {
        let n = signal_window.len();
        if n == 0 {
            return EegBandPowers {
                delta_0_5_4hz: 0.0,
                theta_4_8hz: 0.0,
                alpha_8_13hz: 0.0,
                beta_13_30hz: 0.0,
                gamma_30_100hz: 0.0,
                engagement_index: 0.0,
            };
        }

        let mut delta_p = 0.0;
        let mut theta_p = 0.0;
        let mut alpha_p = 0.0;
        let mut beta_p = 0.0;
        let mut gamma_p = 0.0;

        let num_freqs = n / 2;
        let freq_resolution = self.sample_rate_hz / (n as f64);

        for k in 1..num_freqs {
            let freq = (k as f64) * freq_resolution;
            let mut re = 0.0;
            let mut im = 0.0;

            for (t, &val) in signal_window.iter().enumerate() {
                let angle = 2.0 * std::f64::consts::PI * (k as f64) * (t as f64) / (n as f64);
                re += val * angle.cos();
                im -= val * angle.sin();
            }

            let power = (re * re + im * im) / (n as f64);

            if (0.5..4.0).contains(&freq) {
                delta_p += power;
            } else if (4.0..8.0).contains(&freq) {
                theta_p += power;
            } else if (8.0..13.0).contains(&freq) {
                alpha_p += power;
            } else if (13.0..30.0).contains(&freq) {
                beta_p += power;
            } else if (30.0..100.0).contains(&freq) {
                gamma_p += power;
            }
        }

        let denom = alpha_p + theta_p;
        let engagement = if denom > 1e-6 { beta_p / denom } else { 0.0 };

        EegBandPowers {
            delta_0_5_4hz: delta_p,
            theta_4_8hz: theta_p,
            alpha_8_13hz: alpha_p,
            beta_13_30hz: beta_p,
            gamma_30_100hz: gamma_p,
            engagement_index: engagement,
        }
    }
}
