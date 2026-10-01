use serde::{Deserialize, Serialize};

/// ============================================================================
/// Spiking Neural Network (SNN) Leaky Integrate-and-Fire (LIF) Model
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifNeuron {
    pub membrane_potential: f64,
    pub resting_potential: f64,
    pub threshold_potential: f64,
    pub reset_potential: f64,
    pub decay_tau: f64,
    pub refractory_period_steps: u32,
    pub current_refractory: u32,
    pub total_spikes_fired: u64,
}

impl LifNeuron {
    pub fn new() -> Self {
        Self {
            membrane_potential: -70.0,
            resting_potential: -70.0,
            threshold_potential: -50.0,
            reset_potential: -75.0,
            decay_tau: 0.95,
            refractory_period_steps: 2,
            current_refractory: 0,
            total_spikes_fired: 0,
        }
    }

    /// Step simulasi satu interval waktu (misal 1ms) dengan arus sinaptik masuk
    pub fn step(&mut self, input_current: f64) -> bool {
        if self.current_refractory > 0 {
            self.current_refractory -= 1;
            self.membrane_potential = self.reset_potential;
            return false;
        }

        // Leaky Integration: V[t+1] = V_rest + (V[t] - V_rest)*decay + I_syn
        self.membrane_potential = self.resting_potential
            + (self.membrane_potential - self.resting_potential) * self.decay_tau
            + input_current;

        // Spike Triggering
        if self.membrane_potential >= self.threshold_potential {
            self.membrane_potential = self.reset_potential;
            self.current_refractory = self.refractory_period_steps;
            self.total_spikes_fired += 1;
            true
        } else {
            false
        }
    }
}
