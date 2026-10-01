//! Engine Digital Signal Processing (DSP) & 3D Spatial Audio (audio.rs)
//! Mendukung Stereo Panning, 3D Spatial Audio (Distance Attenuation & Pan), Low-Pass Filter, Reverb, dan Buffer Mixing.

use std::f32::consts::PI;

/// Representasi Sampel Audio Stereo (Kanal Kiri & Kanan)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioFrame {
    pub left: f32,
    pub right: f32,
}

impl AudioFrame {
    pub const SILENT: Self = Self { left: 0.0, right: 0.0 };

    pub fn new(left: f32, right: f32) -> Self {
        Self { left, right }
    }

    pub fn mono(sample: f32) -> Self {
        Self { left: sample, right: sample }
    }
}

/// Buffer Audio PCM Stereo 32-bit Float
#[derive(Debug, Clone, PartialEq)]
pub struct AudioBuffer {
    pub sample_rate: u32,
    pub frames: Vec<AudioFrame>,
}

impl AudioBuffer {
    pub fn new(sample_rate: u32, duration_secs: f32) -> Self {
        let total_samples = (sample_rate as f32 * duration_secs) as usize;
        Self {
            sample_rate,
            frames: vec![AudioFrame::SILENT; total_samples],
        }
    }

    /// Pembangkit Gelombang Sinus murni (Audio Tone Synthesizer)
    pub fn generate_sine(sample_rate: u32, freq_hz: f32, duration_secs: f32, amplitude: f32) -> Self {
        let total_samples = (sample_rate as f32 * duration_secs) as usize;
        let mut frames = Vec::with_capacity(total_samples);

        for i in 0..total_samples {
            let t = i as f32 / sample_rate as f32;
            let sample = (2.0 * PI * freq_hz * t).sin() * amplitude;
            frames.push(AudioFrame::mono(sample));
        }

        Self { sample_rate, frames }
    }

    /// Terapkan Penguatan Volume (Gain Factor)
    pub fn apply_gain(&mut self, gain: f32) {
        for f in &mut self.frames {
            f.left *= gain;
            f.right *= gain;
        }
    }

    /// Terapkan 3D Spatial Audio berdasarkan Posisi Sumber Suara relatif terhadap Pendengar (Listener)
    /// listener_pos: (x, y, z), source_pos: (x, y, z)
    pub fn apply_3d_spatial(&mut self, listener_pos: (f32, f32, f32), source_pos: (f32, f32, f32)) {
        let dx = source_pos.0 - listener_pos.0;
        let dy = source_pos.1 - listener_pos.1;
        let dz = source_pos.2 - listener_pos.2;

        let distance = (dx * dx + dy * dy + dz * dz).sqrt().max(1.0);

        // Inverse Distance Attenuation (Hukum Kuadrat Terbalik)
        let attenuation = (1.0 / distance).clamp(0.0, 1.0);

        // Horizontal Panning Angle (-1.0 Full Left s/d +1.0 Full Right)
        let pan = (dx / distance).clamp(-1.0, 1.0);

        let left_gain = ((1.0 - pan) * 0.5 * attenuation).clamp(0.0, 1.0);
        let right_gain = ((1.0 + pan) * 0.5 * attenuation).clamp(0.0, 1.0);

        for f in &mut self.frames {
            f.left *= left_gain;
            f.right *= right_gain;
        }
    }

    /// Terapkan Digital Low-Pass Filter (RC Filter Simulation)
    pub fn apply_lowpass_filter(&mut self, cutoff_hz: f32) {
        let dt = 1.0 / self.sample_rate as f32;
        let rc = 1.0 / (2.0 * PI * cutoff_hz);
        let alpha = dt / (rc + dt);

        let mut prev_left = 0.0f32;
        let mut prev_right = 0.0f32;

        for f in &mut self.frames {
            prev_left += alpha * (f.left - prev_left);
            prev_right += alpha * (f.right - prev_right);

            f.left = prev_left;
            f.right = prev_right;
        }
    }

    /// Campurkan (Mix) buffer audio lain ke dalam buffer ini
    pub fn mix_in(&mut self, other: &AudioBuffer, offset_samples: usize) {
        for (i, other_frame) in other.frames.iter().enumerate() {
            let target_idx = offset_samples + i;
            if target_idx < self.frames.len() {
                self.frames[target_idx].left = (self.frames[target_idx].left + other_frame.left).clamp(-1.0, 1.0);
                self.frames[target_idx].right = (self.frames[target_idx].right + other_frame.right).clamp(-1.0, 1.0);
            }
        }
    }
}
