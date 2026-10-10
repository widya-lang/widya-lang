//! Pure Sovereign Zero-Dependency Neuromorphic SNN & BCI EEG Engine (pure_neuro.rs)
//!
//! 100% In-House Implementation:
//! - Spiking Neural Network (SNN) with Leaky Integrate-and-Fire (LIF) Dynamics
//! - Spike-Timing-Dependent Plasticity (STDP) Hebbian Synaptic Learning (LTP & LTD)
//! - EEG Spectral Power Band Decomposition (Delta, Theta, Alpha, Beta, Gamma) via Discrete Fourier Transform
//! - Brain-Computer Interface (BCI) Motor Imagery Lateralization Classifier

use std::f64::consts::PI;

// ============================================================================
// 1. NEUROMORPHIC SPIKING NEURAL NETWORK (SNN) & STDP LEARNING
// ============================================================================

/// Parameter Sinapsis dengan Aturan Pembelajaran STDP
#[derive(Debug, Clone)]
pub struct StdpSynapse {
    pub pre_neuron_idx: usize,
    pub post_neuron_idx: usize,
    pub weight: f64,
    pub min_weight: f64,
    pub max_weight: f64,
    pub a_plus: f64,  // LTP learning rate factor
    pub a_minus: f64, // LTD learning rate factor
    pub tau_plus: f64,
    pub tau_minus: f64,
    pub last_pre_spike_time: Option<f64>,
    pub last_post_spike_time: Option<f64>,
}

impl StdpSynapse {
    pub fn new(pre: usize, post: usize, initial_weight: f64) -> Self {
        Self {
            pre_neuron_idx: pre,
            post_neuron_idx: post,
            weight: initial_weight,
            min_weight: 0.0,
            max_weight: 2.0,
            a_plus: 0.05,
            a_minus: 0.06,
            tau_plus: 20.0,  // 20 ms
            tau_minus: 20.0, // 20 ms
            last_pre_spike_time: None,
            last_post_spike_time: None,
        }
    }

    /// Terapkan penyesuaian bobot STDP saat terjadi spike pre-sinaptik
    pub fn on_pre_spike(&mut self, current_time: f64) {
        self.last_pre_spike_time = Some(current_time);
        if let Some(t_post) = self.last_post_spike_time {
            let dt = current_time - t_post; // dt > 0 (post fired before pre -> LTD)
            if dt > 0.0 && dt < 100.0 {
                let delta_w = -self.a_minus * (-dt / self.tau_minus).exp();
                self.weight = (self.weight + delta_w).clamp(self.min_weight, self.max_weight);
            }
        }
    }

    /// Terapkan penyesuaian bobot STDP saat terjadi spike post-sinaptik
    pub fn on_post_spike(&mut self, current_time: f64) {
        self.last_post_spike_time = Some(current_time);
        if let Some(t_pre) = self.last_pre_spike_time {
            let dt = current_time - t_pre; // dt > 0 (pre fired before post -> LTP)
            if dt > 0.0 && dt < 100.0 {
                let delta_w = self.a_plus * (-dt / self.tau_plus).exp();
                self.weight = (self.weight + delta_w).clamp(self.min_weight, self.max_weight);
            }
        }
    }
}

/// Neuron Leaky Integrate-and-Fire (LIF) Presisi Tinggi
#[derive(Debug, Clone)]
pub struct PureLifNeuron {
    pub v_membrane: f64,
    pub v_rest: f64,
    pub v_threshold: f64,
    pub v_reset: f64,
    pub tau_membrane: f64,
    pub refractory_steps: usize,
    pub current_refractory: usize,
}

impl PureLifNeuron {
    pub fn new(v_rest: f64, v_threshold: f64, v_reset: f64, tau: f64) -> Self {
        Self {
            v_membrane: v_rest,
            v_rest,
            v_threshold,
            v_reset,
            tau_membrane: tau,
            refractory_steps: 2,
            current_refractory: 0,
        }
    }

    /// Pembaruan dinamika membran per step waktu dt (ms)
    pub fn step(&mut self, dt: f64, i_synaptic: f64) -> bool {
        if self.current_refractory > 0 {
            self.current_refractory -= 1;
            self.v_membrane = self.v_reset;
            return false;
        }

        let decay = (-dt / self.tau_membrane).exp();
        self.v_membrane = self.v_rest + (self.v_membrane - self.v_rest) * decay + i_synaptic;

        if self.v_membrane >= self.v_threshold {
            self.v_membrane = self.v_reset;
            self.current_refractory = self.refractory_steps;
            true // Spike ditembakkan!
        } else {
            false
        }
    }
}

/// SNN Network dengan Topologi Multi-Neuron & STDP Learning
pub struct SnnNetwork {
    pub neurons: Vec<PureLifNeuron>,
    pub synapses: Vec<StdpSynapse>,
    pub current_time_ms: f64,
}

impl SnnNetwork {
    pub fn new(num_neurons: usize) -> Self {
        let neurons = (0..num_neurons)
            .map(|_| PureLifNeuron::new(-70.0, -50.0, -75.0, 10.0))
            .collect();
        Self {
            neurons,
            synapses: Vec::new(),
            current_time_ms: 0.0,
        }
    }

    pub fn connect(&mut self, pre: usize, post: usize, initial_weight: f64) {
        self.synapses.push(StdpSynapse::new(pre, post, initial_weight));
    }

    /// Jalankan 1 step simulasi (dt ms) dengan stimulus eksternal pada neuron
    pub fn step(&mut self, dt: f64, external_currents: &[f64]) -> Vec<bool> {
        self.current_time_ms += dt;
        let mut spikes = vec![false; self.neurons.len()];
        let mut synaptic_currents = vec![0.0f64; self.neurons.len()];

        // Kumpulkan arus eksternal
        for (i, &curr) in external_currents.iter().enumerate() {
            if i < synaptic_currents.len() {
                synaptic_currents[i] += curr;
            }
        }

        // Hitung perubahan status masing-masing neuron
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let fired = neuron.step(dt, synaptic_currents[i]);
            spikes[i] = fired;
        }

        // Terapkan penembakan spike ke sinapsis dan perbarui bobot STDP
        for syn in &mut self.synapses {
            let pre_idx = syn.pre_neuron_idx;
            let post_idx = syn.post_neuron_idx;

            if spikes[pre_idx] {
                syn.on_pre_spike(self.current_time_ms);
            }
            if spikes[post_idx] {
                syn.on_post_spike(self.current_time_ms);
            }
        }

        spikes
    }
}

// ============================================================================
// 2. EEG SPECTRAL DECOMPOSITION & BCI MOTOR IMAGERY
// ============================================================================

/// Estimasi Daya Spektral per Pita Frekuensi Gelombang Otak
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PureEegBandPower {
    pub delta: f64, // 0.5 - 4 Hz
    pub theta: f64, // 4.0 - 8 Hz
    pub alpha: f64, // 8.0 - 13 Hz (Mu rhythm)
    pub beta: f64,  // 13.0 - 30 Hz
    pub gamma: f64, // 30.0 - 50 Hz
    pub total_power: f64,
}

pub struct PureBciEngine;

impl PureBciEngine {
    /// Hitung dekomposisi pita frekuensi EEG via Transformasi Fourier Diskrit (DFT)
    pub fn compute_band_powers(signal: &[f64], sample_rate_hz: f64) -> PureEegBandPower {
        let n = signal.len();
        if n == 0 || sample_rate_hz <= 0.0 {
            return PureEegBandPower {
                delta: 0.0, theta: 0.0, alpha: 0.0, beta: 0.0, gamma: 0.0, total_power: 0.0,
            };
        }

        let mut delta_p = 0.0;
        let mut theta_p = 0.0;
        let mut alpha_p = 0.0;
        let mut beta_p  = 0.0;
        let mut gamma_p = 0.0;

        let half_n = n / 2;
        let freq_resolution = sample_rate_hz / (n as f64);

        for k in 1..half_n {
            let freq = (k as f64) * freq_resolution;
            if freq > 50.0 {
                break;
            }

            // DFT koefisien X[k]
            let mut re = 0.0;
            let mut im = 0.0;
            let angle_step = 2.0 * PI * (k as f64) / (n as f64);

            for (t, &val) in signal.iter().enumerate() {
                let angle = angle_step * (t as f64);
                re += val * angle.cos();
                im -= val * angle.sin();
            }

            let power = (re * re + im * im) / ((n * n) as f64);

            if freq >= 0.5 && freq < 4.0 {
                delta_p += power;
            } else if freq >= 4.0 && freq < 8.0 {
                theta_p += power;
            } else if freq >= 8.0 && freq < 13.0 {
                alpha_p += power;
            } else if freq >= 13.0 && freq < 30.0 {
                beta_p += power;
            } else if freq >= 30.0 && freq <= 50.0 {
                gamma_p += power;
            }
        }

        let total = delta_p + theta_p + alpha_p + beta_p + gamma_p;

        PureEegBandPower {
            delta: delta_p,
            theta: theta_p,
            alpha: alpha_p,
            beta: beta_p,
            gamma: gamma_p,
            total_power: total,
        }
    }

    /// Klasifikasi Niat Gerak Motorik (Motor Imagery):
    /// Mengukur desinkronisasi pita Alpha/Mu pada elektroda sensorimotor C3 (kiri) vs C4 (kanan).
    /// - Saat membayangkan tangan kanan bergerak: Ritme Mu di C3 terdesinkronisasi (menurun) -> Lateralization Index > 0.
    /// - Saat membayangkan tangan kiri bergerak: Ritme Mu di C4 terdesinkronisasi (menurun) -> Lateralization Index < 0.
    pub fn classify_motor_imagery(c3_alpha_power: f64, c4_alpha_power: f64) -> MotorImageryClass {
        let sum = c3_alpha_power + c4_alpha_power;
        if sum <= 1e-9 {
            return MotorImageryClass::Rest;
        }

        // Asimetri C4 vs C3 (ERD Index): (P_C4 - P_C3) / (P_C4 + P_C3)
        let erd_index = (c4_alpha_power - c3_alpha_power) / sum;

        if erd_index > 0.15 {
            MotorImageryClass::RightHand
        } else if erd_index < -0.15 {
            MotorImageryClass::LeftHand
        } else {
            MotorImageryClass::Rest
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorImageryClass {
    Rest,
    LeftHand,
    RightHand,
}
