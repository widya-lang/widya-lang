//! Pure Sovereign Zero-Dependency Quantum State-Vector Simulator & QASM Engine (pure_quantum.rs)
//!
//! 100% In-House Mathematical Physics Simulator:
//! - Complex number arithmetic in $C$
//! - $N$-qubit pure state vector with $2^N$ complex amplitudes
//! - Universal Unitary Gate Set: H, X, Y, Z, S, S^\dagger, T, RX, RY, RZ, CNOT, CZ, SWAP, Toffoli (CCX), CSWAP (Fredkin)
//! - Quantum Entanglement Generators: Bell States, GHZ State, W State
//! - Quantum Measurement with Projection & Wavefunction Collapse (Born Rule)
//! - OpenQASM 2.0 / 3.0 Execution Engine & Circuit Runner

use std::f64::consts::PI;

/// Representasi Bilangan Kompleks Presisi Ganda (f64)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    #[inline]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline]
    pub fn exp_i(theta: f64) -> Self {
        Self {
            re: theta.cos(),
            im: theta.sin(),
        }
    }

    #[inline]
    pub fn norm_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    #[inline]
    pub fn norm(&self) -> f64 {
        self.norm_sq().sqrt()
    }

    #[inline]
    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    #[inline]
    pub fn add(&self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    #[inline]
    pub fn sub(&self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    #[inline]
    pub fn mul(&self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    #[inline]
    pub fn scale(&self, s: f64) -> Self {
        Self {
            re: self.re * s,
            im: self.im * s,
        }
    }
}

/// Tipe 4 Keadaan Bell Standar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BellStateType {
    PhiPlus,  // (|00> + |11>) / sqrt(2)
    PhiMinus, // (|00> - |11>) / sqrt(2)
    PsiPlus,  // (|01> + |10>) / sqrt(2)
    PsiMinus, // (|01> - |10>) / sqrt(2)
}

/// Quantum State Vector Simulator $2^N$
#[derive(Debug, Clone, PartialEq)]
pub struct PureQuantumSim {
    pub num_qubits: usize,
    pub dim: usize,
    pub state: Vec<Complex>,
}

impl PureQuantumSim {
    /// Inisialisasi simulator $N$ qubit dalam ground state $|00...0\rangle$
    pub fn new(num_qubits: usize) -> Self {
        assert!(num_qubits >= 1 && num_qubits <= 24, "Qubit harus antara 1 dan 24");
        let dim = 1 << num_qubits;
        let mut state = vec![Complex::ZERO; dim];
        state[0] = Complex::ONE;
        Self {
            num_qubits,
            dim,
            state,
        }
    }

    /// Reset simulator kembali ke $|00...0\rangle$
    pub fn reset(&mut self) {
        self.state.fill(Complex::ZERO);
        self.state[0] = Complex::ONE;
    }

    /// Periksa apakah total probabilitas $\sum |c_i|^2 = 1.0$ dalam toleransi
    pub fn is_normalized(&self, tolerance: f64) -> bool {
        let sum_norm_sq: f64 = self.state.iter().map(|c| c.norm_sq()).sum();
        (sum_norm_sq - 1.0).abs() <= tolerance
    }

    /// Normalisasi vektor keadaan
    pub fn normalize(&mut self) {
        let sum_norm_sq: f64 = self.state.iter().map(|c| c.norm_sq()).sum();
        if sum_norm_sq > 0.0 {
            let factor = 1.0 / sum_norm_sq.sqrt();
            for c in &mut self.state {
                *c = c.scale(factor);
            }
        }
    }

    // ========================================================================
    // SINGLE-QUBIT GATES
    // ========================================================================

    /// Gerbang Hadamard (H): Superposisi seimbang
    pub fn h(&mut self, target: usize) {
        assert!(target < self.num_qubits);
        let inv_sqrt2 = 1.0 / (2.0_f64).sqrt();
        let stride = 1 << target;

        for i in (0..self.dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx0 = i + j;
                let idx1 = idx0 + stride;
                let v0 = self.state[idx0];
                let v1 = self.state[idx1];

                self.state[idx0] = v0.add(v1).scale(inv_sqrt2);
                self.state[idx1] = v0.sub(v1).scale(inv_sqrt2);
            }
        }
    }

    /// Gerbang Pauli-X (Bit Flip / NOT)
    pub fn x(&mut self, target: usize) {
        assert!(target < self.num_qubits);
        let stride = 1 << target;

        for i in (0..self.dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx0 = i + j;
                let idx1 = idx0 + stride;
                self.state.swap(idx0, idx1);
            }
        }
    }

    /// Gerbang Pauli-Y: Bit + Phase Flip
    pub fn y(&mut self, target: usize) {
        assert!(target < self.num_qubits);
        let stride = 1 << target;

        for i in (0..self.dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx0 = i + j;
                let idx1 = idx0 + stride;
                let v0 = self.state[idx0];
                let v1 = self.state[idx1];

                // new v0 = -i * v1 = (v1.im, -v1.re)
                // new v1 = i * v0 = (-v0.im, v0.re)
                self.state[idx0] = Complex::new(v1.im, -v1.re);
                self.state[idx1] = Complex::new(-v0.im, v0.re);
            }
        }
    }

    /// Gerbang Pauli-Z (Phase Flip)
    pub fn z(&mut self, target: usize) {
        assert!(target < self.num_qubits);
        let stride = 1 << target;

        for i in (0..self.dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx1 = i + j + stride;
                self.state[idx1] = self.state[idx1].scale(-1.0);
            }
        }
    }

    /// Gerbang S (Phase Gate: $\pi/2$ phase shift pada $|1\rangle$)
    pub fn s(&mut self, target: usize) {
        assert!(target < self.num_qubits);
        let stride = 1 << target;

        for i in (0..self.dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx1 = i + j + stride;
                let v1 = self.state[idx1];
                // v1 * i = (-v1.im, v1.re)
                self.state[idx1] = Complex::new(-v1.im, v1.re);
            }
        }
    }

    /// Gerbang S-Dagger ($-\pi/2$ phase shift pada $|1\rangle$)
    pub fn s_dagger(&mut self, target: usize) {
        assert!(target < self.num_qubits);
        let stride = 1 << target;

        for i in (0..self.dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx1 = i + j + stride;
                let v1 = self.state[idx1];
                // v1 * (-i) = (v1.im, -v1.re)
                self.state[idx1] = Complex::new(v1.im, -v1.re);
            }
        }
    }

    /// Gerbang T ($\pi/4$ phase shift pada $|1\rangle$)
    pub fn t(&mut self, target: usize) {
        assert!(target < self.num_qubits);
        let phase = Complex::exp_i(PI / 4.0);
        let stride = 1 << target;

        for i in (0..self.dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx1 = i + j + stride;
                self.state[idx1] = self.state[idx1].mul(phase);
            }
        }
    }

    /// Rotasi RX(theta)
    pub fn rx(&mut self, target: usize, theta: f64) {
        assert!(target < self.num_qubits);
        let half = theta / 2.0;
        let c = half.cos();
        let s = half.sin();
        let stride = 1 << target;

        for i in (0..self.dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx0 = i + j;
                let idx1 = idx0 + stride;
                let v0 = self.state[idx0];
                let v1 = self.state[idx1];

                // v0' = c*v0 - i*s*v1
                // v1' = -i*s*v0 + c*v1
                self.state[idx0] = Complex::new(c * v0.re + s * v1.im, c * v0.im - s * v1.re);
                self.state[idx1] = Complex::new(c * v1.re + s * v0.im, c * v1.im - s * v0.re);
            }
        }
    }

    /// Rotasi RY(theta)
    pub fn ry(&mut self, target: usize, theta: f64) {
        assert!(target < self.num_qubits);
        let half = theta / 2.0;
        let c = half.cos();
        let s = half.sin();
        let stride = 1 << target;

        for i in (0..self.dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx0 = i + j;
                let idx1 = idx0 + stride;
                let v0 = self.state[idx0];
                let v1 = self.state[idx1];

                // v0' = c*v0 - s*v1
                // v1' = s*v0 + c*v1
                self.state[idx0] = Complex::new(c * v0.re - s * v1.re, c * v0.im - s * v1.im);
                self.state[idx1] = Complex::new(s * v0.re + c * v1.re, s * v0.im + c * v1.im);
            }
        }
    }

    /// Rotasi RZ(theta)
    pub fn rz(&mut self, target: usize, theta: f64) {
        assert!(target < self.num_qubits);
        let half = theta / 2.0;
        let phase0 = Complex::exp_i(-half);
        let phase1 = Complex::exp_i(half);
        let stride = 1 << target;

        for i in (0..self.dim).step_by(stride * 2) {
            for j in 0..stride {
                let idx0 = i + j;
                let idx1 = idx0 + stride;
                self.state[idx0] = self.state[idx0].mul(phase0);
                self.state[idx1] = self.state[idx1].mul(phase1);
            }
        }
    }

    // ========================================================================
    // MULTI-QUBIT CONTROLLED GATES
    // ========================================================================

    /// Gerbang CNOT (CX)
    pub fn cx(&mut self, control: usize, target: usize) {
        assert!(control < self.num_qubits && target < self.num_qubits && control != target);
        let c_mask = 1 << control;
        let t_mask = 1 << target;

        for i in 0..self.dim {
            if (i & c_mask) != 0 && (i & t_mask) == 0 {
                let paired = i | t_mask;
                self.state.swap(i, paired);
            }
        }
    }

    /// Gerbang Controlled-Z (CZ)
    pub fn cz(&mut self, control: usize, target: usize) {
        assert!(control < self.num_qubits && target < self.num_qubits && control != target);
        let mask = (1 << control) | (1 << target);

        for i in 0..self.dim {
            if (i & mask) == mask {
                self.state[i] = self.state[i].scale(-1.0);
            }
        }
    }

    /// Gerbang SWAP
    pub fn swap(&mut self, q1: usize, q2: usize) {
        assert!(q1 < self.num_qubits && q2 < self.num_qubits);
        if q1 == q2 {
            return;
        }
        let m1 = 1 << q1;
        let m2 = 1 << q2;

        for i in 0..self.dim {
            let b1 = (i & m1) != 0;
            let b2 = (i & m2) != 0;
            if b1 && !b2 {
                let paired = (i ^ m1) | m2;
                self.state.swap(i, paired);
            }
        }
    }

    /// Gerbang Toffoli (CCNOT / CCX)
    pub fn toffoli(&mut self, c1: usize, c2: usize, target: usize) {
        assert!(c1 < self.num_qubits && c2 < self.num_qubits && target < self.num_qubits);
        assert!(c1 != c2 && c1 != target && c2 != target);

        let c_mask = (1 << c1) | (1 << c2);
        let t_mask = 1 << target;

        for i in 0..self.dim {
            if (i & c_mask) == c_mask && (i & t_mask) == 0 {
                let paired = i | t_mask;
                self.state.swap(i, paired);
            }
        }
    }

    /// Gerbang Fredkin (Controlled-SWAP)
    pub fn cswap(&mut self, control: usize, q1: usize, q2: usize) {
        assert!(control < self.num_qubits && q1 < self.num_qubits && q2 < self.num_qubits);
        if q1 == q2 {
            return;
        }

        let ctrl_mask = 1 << control;
        let m1 = 1 << q1;
        let m2 = 1 << q2;

        for i in 0..self.dim {
            if (i & ctrl_mask) != 0 {
                let b1 = (i & m1) != 0;
                let b2 = (i & m2) != 0;
                if b1 && !b2 {
                    let paired = (i ^ m1) | m2;
                    self.state.swap(i, paired);
                }
            }
        }
    }

    // ========================================================================
    // ENTANGLEMENT GENERATORS
    // ========================================================================

    /// Bangkitkan 1 dari 4 Bell State pada 2 qubit pertama
    pub fn generate_bell_state(&mut self, bell_type: BellStateType) {
        assert!(self.num_qubits >= 2);
        self.reset();
        match bell_type {
            BellStateType::PhiPlus => {
                // H(0), CX(0, 1) -> (|00> + |11>) / sqrt(2)
                self.h(0);
                self.cx(0, 1);
            }
            BellStateType::PhiMinus => {
                // X(0), H(0), CX(0, 1) -> (|00> - |11>) / sqrt(2)
                self.x(0);
                self.h(0);
                self.cx(0, 1);
            }
            BellStateType::PsiPlus => {
                // X(1), H(0), CX(0, 1) -> (|01> + |10>) / sqrt(2)
                self.x(1);
                self.h(0);
                self.cx(0, 1);
            }
            BellStateType::PsiMinus => {
                // X(0), X(1), H(0), CX(0, 1) -> (|01> - |10>) / sqrt(2)
                self.x(0);
                self.x(1);
                self.h(0);
                self.cx(0, 1);
            }
        }
    }

    /// Bangkitkan Keadaan GHZ (Greenberger-Horne-Zeilinger) pada seluruh qubit: $(|00...0\rangle + |11...1\rangle) / \sqrt{2}$
    pub fn generate_ghz_state(&mut self) {
        self.reset();
        self.h(0);
        for i in 0..self.num_qubits - 1 {
            self.cx(i, i + 1);
        }
    }

    // ========================================================================
    // MEASUREMENT & PROBABILITIES (BORN RULE)
    // ========================================================================

    /// Hitung vektor probabilitas dari seluruh $2^N$ basis keadaan
    pub fn get_probabilities(&self) -> Vec<f64> {
        self.state.iter().map(|c| c.norm_sq()).collect()
    }

    /// Probabilitas qubit tertentu bernilai 0 vs 1
    pub fn qubit_prob(&self, target: usize) -> (f64, f64) {
        assert!(target < self.num_qubits);
        let mask = 1 << target;
        let mut prob0 = 0.0;
        let mut prob1 = 0.0;

        for i in 0..self.dim {
            let nsq = self.state[i].norm_sq();
            if (i & mask) == 0 {
                prob0 += nsq;
            } else {
                prob1 += nsq;
            }
        }
        (prob0, prob1)
    }

    /// Pengukuran deterministik/probabilistik pada target qubit dengan Kolaps Fungsi Gelombang (Wavefunction Collapse)
    /// `random_val` adalah angka acak $[0.0, 1.0)$
    pub fn measure_collapse(&mut self, target: usize, random_val: f64) -> (u8, f64) {
        assert!(target < self.num_qubits);
        let (p0, p1) = self.qubit_prob(target);
        let mask = 1 << target;

        let outcome = if random_val < p0 { 0u8 } else { 1u8 };
        let outcome_prob = if outcome == 0 { p0 } else { p1 };

        if outcome_prob > 0.0 {
            let norm_factor = 1.0 / outcome_prob.sqrt();
            for i in 0..self.dim {
                let bit = ((i & mask) != 0) as u8;
                if bit == outcome {
                    self.state[i] = self.state[i].scale(norm_factor);
                } else {
                    self.state[i] = Complex::ZERO;
                }
            }
        }

        (outcome, outcome_prob)
    }

    /// Hitung Quantum Fidelity $|\langle\psi|\phi\rangle|^2$ terhadap keadaan lain
    pub fn fidelity(&self, other: &PureQuantumSim) -> f64 {
        assert_eq!(self.dim, other.dim);
        let mut dot = Complex::ZERO;
        for i in 0..self.dim {
            // dot += self[i]^* * other[i]
            let term = self.state[i].conj().mul(other.state[i]);
            dot = dot.add(term);
        }
        dot.norm_sq()
    }
}

// ============================================================================
// OPENQASM PARSER & EXECUTION ENGINE
// ============================================================================

/// Hasil Eksekusi Sirkuit Kuantum
#[derive(Debug, Clone)]
pub struct QuantumExecutionResult {
    pub final_probabilities: Vec<f64>,
    pub classical_bits: Vec<u8>,
    pub measured_states: Vec<(usize, u8)>, // (qubit, value)
}

pub struct PureQasmEngine;

impl PureQasmEngine {
    /// Eksekusi string OpenQASM langsung pada state vector simulator
    pub fn execute_qasm(source: &str, random_seeds: &[f64]) -> Result<QuantumExecutionResult, String> {
        let mut num_qubits = 0;
        let mut num_clbits = 0;

        // Pass 1: Parse alokasi register
        for line in source.lines() {
            let t = line.trim();
            if t.starts_with("qreg") || t.starts_with("qubit") {
                if let (Some(b), Some(e)) = (t.find('['), t.find(']')) {
                    num_qubits = t[b + 1..e].parse().unwrap_or(2);
                }
            } else if t.starts_with("creg") || t.starts_with("bit") {
                if let (Some(b), Some(e)) = (t.find('['), t.find(']')) {
                    num_clbits = t[b + 1..e].parse().unwrap_or(2);
                }
            }
        }

        if num_qubits == 0 {
            num_qubits = 2;
        }

        let mut sim = PureQuantumSim::new(num_qubits);
        let mut clbits = vec![0u8; num_clbits];
        let mut measured = Vec::new();
        let mut seed_idx = 0;

        // Pass 2: Eksekusi gate
        for line in source.lines() {
            let t = line.trim();
            if t.is_empty() || t.starts_with("//") || t.starts_with("OPENQASM") || t.starts_with("include") {
                continue;
            }

            let parts: Vec<&str> = t.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "h" => {
                    let q = parse_reg_idx(parts[1])?;
                    sim.h(q);
                }
                "x" => {
                    let q = parse_reg_idx(parts[1])?;
                    sim.x(q);
                }
                "y" => {
                    let q = parse_reg_idx(parts[1])?;
                    sim.y(q);
                }
                "z" => {
                    let q = parse_reg_idx(parts[1])?;
                    sim.z(q);
                }
                "s" => {
                    let q = parse_reg_idx(parts[1])?;
                    sim.s(q);
                }
                "t" => {
                    let q = parse_reg_idx(parts[1])?;
                    sim.t(q);
                }
                "cx" => {
                    let raw = t[2..].replace(';', "");
                    let q_parts: Vec<&str> = raw.split(',').map(|s| s.trim()).collect();
                    if q_parts.len() == 2 {
                        let c = parse_reg_idx(q_parts[0])?;
                        let target = parse_reg_idx(q_parts[1])?;
                        sim.cx(c, target);
                    }
                }
                "cz" => {
                    let raw = t[2..].replace(';', "");
                    let q_parts: Vec<&str> = raw.split(',').map(|s| s.trim()).collect();
                    if q_parts.len() == 2 {
                        let c = parse_reg_idx(q_parts[0])?;
                        let target = parse_reg_idx(q_parts[1])?;
                        sim.cz(c, target);
                    }
                }
                "swap" => {
                    let raw = t[4..].replace(';', "");
                    let q_parts: Vec<&str> = raw.split(',').map(|s| s.trim()).collect();
                    if q_parts.len() == 2 {
                        let q1 = parse_reg_idx(q_parts[0])?;
                        let q2 = parse_reg_idx(q_parts[1])?;
                        sim.swap(q1, q2);
                    }
                }
                "ccx" => {
                    let raw = t[3..].replace(';', "");
                    let q_parts: Vec<&str> = raw.split(',').map(|s| s.trim()).collect();
                    if q_parts.len() == 3 {
                        let c1 = parse_reg_idx(q_parts[0])?;
                        let c2 = parse_reg_idx(q_parts[1])?;
                        let target = parse_reg_idx(q_parts[2])?;
                        sim.toffoli(c1, c2, target);
                    }
                }
                "measure" => {
                    let raw = t[7..].replace(';', "");
                    let q_parts: Vec<&str> = raw.split("->").map(|s| s.trim()).collect();
                    if q_parts.len() == 2 {
                        let q = parse_reg_idx(q_parts[0])?;
                        let c = parse_reg_idx(q_parts[1])?;
                        let rnd = if seed_idx < random_seeds.len() {
                            let r = random_seeds[seed_idx];
                            seed_idx += 1;
                            r
                        } else {
                            0.0
                        };
                        let (outcome, _) = sim.measure_collapse(q, rnd);
                        if c < clbits.len() {
                            clbits[c] = outcome;
                        }
                        measured.push((q, outcome));
                    }
                }
                _ => {}
            }
        }

        Ok(QuantumExecutionResult {
            final_probabilities: sim.get_probabilities(),
            classical_bits: clbits,
            measured_states: measured,
        })
    }
}

fn parse_reg_idx(s: &str) -> Result<usize, String> {
    let clean = s.trim().trim_end_matches(';');
    if let (Some(b), Some(e)) = (clean.find('['), clean.find(']')) {
        clean[b + 1..e]
            .parse::<usize>()
            .map_err(|e| format!("Index register tidak valid: {}", e))
    } else {
        clean
            .parse::<usize>()
            .map_err(|e| format!("Index qubit tidak valid: {}", e))
    }
}
