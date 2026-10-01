use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// ============================================================================
/// Quantum State Vector Simulator Native (H, X, Y, Z, RZ, CX / CNOT, Measure)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub fn one() -> Self {
        Self { re: 1.0, im: 0.0 }
    }

    pub fn norm_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn add(&self, other: Complex64) -> Complex64 {
        Complex64::new(self.re + other.re, self.im + other.im)
    }

    pub fn mul(&self, other: Complex64) -> Complex64 {
        Complex64::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }

    pub fn scale(&self, s: f64) -> Complex64 {
        Complex64::new(self.re * s, self.im * s)
    }
}

pub struct QuantumStateVector {
    pub num_qubits: usize,
    pub state: Vec<Complex64>,
}

impl QuantumStateVector {
    pub fn new(num_qubits: usize) -> Self {
        assert!(num_qubits >= 1 && num_qubits <= 24, "Qubits must be between 1 and 24");
        let dim = 1 << num_qubits;
        let mut state = vec![Complex64::zero(); dim];
        state[0] = Complex64::one(); // Initialize in state |00...0>
        Self { num_qubits, state }
    }

    /// Gerbang Hadamard (H)
    pub fn apply_hadamard(&mut self, target: usize) {
        let inv_sqrt2 = 1.0 / (2.0_f64).sqrt();
        let stride = 1 << target;
        let dim = 1 << self.num_qubits;

        for i in (0..dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx0 = i + j;
                let idx1 = idx0 + stride;
                let v0 = self.state[idx0];
                let v1 = self.state[idx1];

                self.state[idx0] = v0.add(v1).scale(inv_sqrt2);
                self.state[idx1] = v0.add(v1.scale(-1.0)).scale(inv_sqrt2);
            }
        }
    }

    /// Gerbang Pauli-X (Bit Flip / NOT)
    pub fn apply_x(&mut self, target: usize) {
        let stride = 1 << target;
        let dim = 1 << self.num_qubits;

        for i in (0..dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx0 = i + j;
                let idx1 = idx0 + stride;
                self.state.swap(idx0, idx1);
            }
        }
    }

    /// Gerbang Pauli-Z (Phase Flip)
    pub fn apply_z(&mut self, target: usize) {
        let stride = 1 << target;
        let dim = 1 << self.num_qubits;

        for i in (0..dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx1 = i + j + stride;
                self.state[idx1] = self.state[idx1].scale(-1.0);
            }
        }
    }

    /// Gerbang Controlled-NOT (CNOT / CX)
    pub fn apply_cx(&mut self, control: usize, target: usize) {
        let dim = 1 << self.num_qubits;
        let c_mask = 1 << control;
        let t_mask = 1 << target;

        for i in 0..dim {
            if (i & c_mask) != 0 && (i & t_mask) == 0 {
                let paired = i | t_mask;
                self.state.swap(i, paired);
            }
        }
    }

    /// Gerbang Rotasi Phase RZ(theta)
    pub fn apply_rz(&mut self, target: usize, theta: f64) {
        let stride = 1 << target;
        let dim = 1 << self.num_qubits;
        let half_t = theta / 2.0;
        let phase_0 = Complex64::new(half_t.cos(), -half_t.sin());
        let phase_1 = Complex64::new(half_t.cos(), half_t.sin());

        for i in (0..dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx0 = i + j;
                let idx1 = idx0 + stride;
                self.state[idx0] = self.state[idx0].mul(phase_0);
                self.state[idx1] = self.state[idx1].mul(phase_1);
            }
        }
    }

    /// Dapatkan probabilitas semua basis keadaan |0...0> hingga |1...1>
    pub fn get_probabilities(&self) -> Vec<f64> {
        self.state.iter().map(|c| c.norm_sq()).collect()
    }
}
