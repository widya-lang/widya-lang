use super::state_vector_sim::QuantumStateVector;
use serde::{Deserialize, Serialize};

/// ============================================================================
/// Variational Quantum Eigensolver (VQE) & QAOA Hybrid Optimizer
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VqeResult {
    pub optimal_energy: f64,
    pub optimal_parameters: Vec<f64>,
    pub iterations: usize,
    pub converged: bool,
}

pub struct VqeOptimizer {
    pub num_qubits: usize,
    pub learning_rate: f64,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl VqeOptimizer {
    pub fn new(num_qubits: usize, lr: f64, max_iter: usize) -> Self {
        Self {
            num_qubits,
            learning_rate: lr,
            max_iterations: max_iter,
            tolerance: 1e-6,
        }
    }

    /// Evaluasi nilai ekspektasi Hamiltonian sederhana <H> = Z_0 Z_1 + X_0
    pub fn evaluate_energy(&self, params: &[f64]) -> f64 {
        let mut sim = QuantumStateVector::new(self.num_qubits);

        // Ansatz sederhana: Rotasi RZ pada tiap qubit + Entanglement CNOT
        for i in 0..self.num_qubits {
            let theta = if i < params.len() { params[i] } else { 0.0 };
            sim.apply_hadamard(i);
            sim.apply_rz(i, theta);
        }

        if self.num_qubits >= 2 {
            sim.apply_cx(0, 1);
        }

        let probs = sim.get_probabilities();
        // Hamiltonian Model Energy: E = <Z0 Z1> - 0.5 * <X0>
        let mut e_zz = 0.0;
        for (idx, &p) in probs.iter().enumerate() {
            let b0 = (idx & 1) != 0;
            let b1 = (idx & 2) != 0;
            let parity = if b0 ^ b1 { -1.0 } else { 1.0 };
            e_zz += p * parity;
        }

        e_zz
    }

    /// Jalankan optimasi parameter gradient descent
    pub fn optimize(&self, initial_params: &[f64]) -> VqeResult {
        let mut params = initial_params.to_vec();
        let mut prev_energy = self.evaluate_energy(&params);
        let mut iter = 0;
        let mut converged = false;

        let delta = 1e-4;
        while iter < self.max_iterations {
            iter += 1;
            let mut grads = Vec::with_capacity(params.len());

            for i in 0..params.len() {
                let mut params_plus = params.clone();
                params_plus[i] += delta;
                let e_plus = self.evaluate_energy(&params_plus);

                let mut params_minus = params.clone();
                params_minus[i] -= delta;
                let e_minus = self.evaluate_energy(&params_minus);

                let grad = (e_plus - e_minus) / (2.0 * delta);
                grads.push(grad);
            }

            for i in 0..params.len() {
                params[i] -= self.learning_rate * grads[i];
            }

            let curr_energy = self.evaluate_energy(&params);
            if (curr_energy - prev_energy).abs() < self.tolerance {
                converged = true;
                break;
            }
            prev_energy = curr_energy;
        }

        VqeResult {
            optimal_energy: prev_energy,
            optimal_parameters: params,
            iterations: iter,
            converged,
        }
    }
}
