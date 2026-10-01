use serde::{Deserialize, Serialize};

/// ============================================================================
/// Molecular Mechanics Force Field Evaluator (Lennard-Jones + Coulomb)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceFieldParameters {
    pub epsilon_kcal_mol: f64, // Potential well depth
    pub sigma_angstrom: f64,   // Van der Waals distance
    pub charge_elementary: f64,
}

pub struct MolecularMechanicsEngine;

impl MolecularMechanicsEngine {
    /// Evaluasi Lennard-Jones 12-6 Potential: V_LJ(r) = 4*eps * [ (sig/r)^12 - (sig/r)^6 ]
    pub fn lennard_jones_potential(r: f64, eps: f64, sigma: f64) -> f64 {
        if r <= 1e-4 {
            return 1e6; // Strong repulsive core
        }
        let sr = sigma / r;
        let sr6 = sr.powi(6);
        let sr12 = sr6 * sr6;
        4.0 * eps * (sr12 - sr6)
    }

    /// Evaluasi Energi Elektrostatik Coulomb: V_coul(r) = q1 * q2 / (4 * pi * eps_0 * r)
    pub fn coulomb_potential(r: f64, q1: f64, q2: f64) -> f64 {
        if r <= 1e-4 {
            return 1e6;
        }
        let coulomb_constant = 332.0637; // kcal * Angstrom / (mol * e^2)
        (coulomb_constant * q1 * q2) / r
    }
}
