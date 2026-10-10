//! Pure Sovereign Zero-Dependency Crystallography & XRD Engine (pure_materials.rs)
//!
//! 100% In-House Implementation:
//! - 7 Bravais Crystal Systems (Cubic, Tetragonal, Orthorhombic, Hexagonal, Rhombohedral, Monoclinic, Triclinic)
//! - Universal Metric Tensor, Reciprocal Metric Tensor, and Unit Cell Volume
//! - Interplanar d-Spacing (d_hkl) for all Miller indices (h, k, l)
//! - X-Ray Powder Diffraction (XRD) Simulation based on Bragg's Law (Cu-Ka1 radiation)
//! - Structure Factor & Lorentz-Polarization Factor Intensity Synthesis
//! - Continuous 1D Diffractogram Spectrum Generator (Gaussian Peak Broadening)
//! - Direct Interoperability Bridge with `CrystalLattice`

use serde::{Deserialize, Serialize};

// ============================================================================
// 1. BRAVAIS CRYSTAL SYSTEMS & LATTICE PARAMETERS
// ============================================================================

/// Klasifikasi 7 Sistem Kristal Bravais
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrystalSystem {
    Cubic,
    Tetragonal,
    Orthorhombic,
    Hexagonal,
    Rhombohedral,
    Monoclinic,
    Triclinic,
}

/// Atom Basis dalam Sel Satuan Kristal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasisAtom {
    pub element: String,
    pub fractional_coord: [f64; 3], // [x, y, z] dalam fraksi kisi [0.0 .. 1.0]
    pub atomic_number: u32,         // Z (misal: Si=14, Fe=26, O=8, Li=3)
}

impl BasisAtom {
    pub fn new(element: &str, atomic_number: u32, x: f64, y: f64, z: f64) -> Self {
        Self {
            element: element.to_string(),
            fractional_coord: [x, y, z],
            atomic_number,
        }
    }
}

/// Sel Satuan Kristal Murni (Pure Unit Cell)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PureUnitCell {
    pub crystal_system: CrystalSystem,
    pub a: f64,         // Panjang kisi a dalam Ångström (Å)
    pub b: f64,         // Panjang kisi b dalam Ångström (Å)
    pub c: f64,         // Panjang kisi c dalam Ångström (Å)
    pub alpha_deg: f64, // Sudut alpha (antara b dan c) dalam derajat
    pub beta_deg: f64,  // Sudut beta (antara a dan c) dalam derajat
    pub gamma_deg: f64, // Sudut gamma (antara a dan b) dalam derajat
    pub atoms: Vec<BasisAtom>,
}

impl PureUnitCell {
    /// Konstruktor Kisi Kubik (misal: Silicon diamond cubic, Garam NaCl, Fe BCC/FCC)
    pub fn new_cubic(a: f64, atoms: Vec<BasisAtom>) -> Self {
        Self {
            crystal_system: CrystalSystem::Cubic,
            a,
            b: a,
            c: a,
            alpha_deg: 90.0,
            beta_deg: 90.0,
            gamma_deg: 90.0,
            atoms,
        }
    }

    /// Konstruktor Kisi Tetragonal (misal: Rutil TiO2, Indium)
    pub fn new_tetragonal(a: f64, c: f64, atoms: Vec<BasisAtom>) -> Self {
        Self {
            crystal_system: CrystalSystem::Tetragonal,
            a,
            b: a,
            c,
            alpha_deg: 90.0,
            beta_deg: 90.0,
            gamma_deg: 90.0,
            atoms,
        }
    }

    /// Konstruktor Kisi Ortorombik (misal: Katoda Baterai LFP LiFePO4, Olivine)
    pub fn new_orthorhombic(a: f64, b: f64, c: f64, atoms: Vec<BasisAtom>) -> Self {
        Self {
            crystal_system: CrystalSystem::Orthorhombic,
            a,
            b,
            c,
            alpha_deg: 90.0,
            beta_deg: 90.0,
            gamma_deg: 90.0,
            atoms,
        }
    }

    /// Konstruktor Kisi Heksagonal (misal: Grafena/Grafit, Seng, Titanium)
    pub fn new_hexagonal(a: f64, c: f64, atoms: Vec<BasisAtom>) -> Self {
        Self {
            crystal_system: CrystalSystem::Hexagonal,
            a,
            b: a,
            c,
            alpha_deg: 90.0,
            beta_deg: 90.0,
            gamma_deg: 120.0,
            atoms,
        }
    }

    /// Konstruktor Kisi Monoklinik
    pub fn new_monoclinic(a: f64, b: f64, c: f64, beta_deg: f64, atoms: Vec<BasisAtom>) -> Self {
        Self {
            crystal_system: CrystalSystem::Monoclinic,
            a,
            b,
            c,
            alpha_deg: 90.0,
            beta_deg,
            gamma_deg: 90.0,
            atoms,
        }
    }

    /// Konstruktor Kisi Umum / Triklinik
    pub fn new_triclinic(
        a: f64,
        b: f64,
        c: f64,
        alpha_deg: f64,
        beta_deg: f64,
        gamma_deg: f64,
        atoms: Vec<BasisAtom>,
    ) -> Self {
        Self {
            crystal_system: CrystalSystem::Triclinic,
            a,
            b,
            c,
            alpha_deg,
            beta_deg,
            gamma_deg,
            atoms,
        }
    }

    /// Hitung Volume Sel Satuan (Unit Cell Volume) dalam Å³
    /// V = a * b * c * sqrt(1 - cos²α - cos²β - cos²γ + 2*cosα*cosβ*cosγ)
    pub fn volume(&self) -> f64 {
        let alpha = self.alpha_deg.to_radians();
        let beta = self.beta_deg.to_radians();
        let gamma = self.gamma_deg.to_radians();

        let ca = alpha.cos();
        let cb = beta.cos();
        let cg = gamma.cos();

        let term = 1.0 - ca * ca - cb * cb - cg * cg + 2.0 * ca * cb * cg;
        let factor = if term > 0.0 { term.sqrt() } else { 0.0 };

        self.a * self.b * self.c * factor
    }

    /// Hitung Jarak Antar-Bidang (Interplanar Spacing d_hkl) dalam Ångström
    pub fn d_spacing(&self, h: i32, k: i32, l: i32) -> Option<f64> {
        if h == 0 && k == 0 && l == 0 {
            return None;
        }

        let h_f = h as f64;
        let k_f = k as f64;
        let l_f = l as f64;

        match self.crystal_system {
            CrystalSystem::Cubic => {
                let denom = (h_f * h_f + k_f * k_f + l_f * l_f).sqrt();
                if denom > 1e-9 {
                    Some(self.a / denom)
                } else {
                    None
                }
            }
            CrystalSystem::Tetragonal => {
                let inv_d2 = (h_f * h_f + k_f * k_f) / (self.a * self.a) + (l_f * l_f) / (self.c * self.c);
                if inv_d2 > 1e-9 {
                    Some(1.0 / inv_d2.sqrt())
                } else {
                    None
                }
            }
            CrystalSystem::Orthorhombic => {
                let inv_d2 = (h_f * h_f) / (self.a * self.a)
                    + (k_f * k_f) / (self.b * self.b)
                    + (l_f * l_f) / (self.c * self.c);
                if inv_d2 > 1e-9 {
                    Some(1.0 / inv_d2.sqrt())
                } else {
                    None
                }
            }
            CrystalSystem::Hexagonal => {
                let inv_d2 = (4.0 / 3.0) * (h_f * h_f + h_f * k_f + k_f * k_f) / (self.a * self.a)
                    + (l_f * l_f) / (self.c * self.c);
                if inv_d2 > 1e-9 {
                    Some(1.0 / inv_d2.sqrt())
                } else {
                    None
                }
            }
            _ => {
                // Metode Matriks Metrik Resiprokal Universal untuk Monoklinik, Rhombohedral, Triklinik
                let alpha = self.alpha_deg.to_radians();
                let beta = self.beta_deg.to_radians();
                let gamma = self.gamma_deg.to_radians();

                let ca = alpha.cos();
                let cb = beta.cos();
                let cg = gamma.cos();

                // Matriks Metrik Ruang Riil G
                let g11 = self.a * self.a;
                let g22 = self.b * self.b;
                let g33 = self.c * self.c;
                let g12 = self.a * self.b * cg;
                let g13 = self.a * self.c * cb;
                let g23 = self.b * self.c * ca;

                let det_g = g11 * (g22 * g33 - g23 * g23)
                    - g12 * (g12 * g33 - g23 * g13)
                    + g13 * (g12 * g23 - g22 * g13);

                if det_g.abs() < 1e-9 {
                    return None;
                }

                // Invers Matriks Metrik (Matriks Metrik Resiprokal G*)
                let inv_g11 = (g22 * g33 - g23 * g23) / det_g;
                let inv_g22 = (g11 * g33 - g13 * g13) / det_g;
                let inv_g33 = (g11 * g22 - g12 * g12) / det_g;
                let inv_g12 = (g13 * g23 - g12 * g33) / det_g;
                let inv_g13 = (g12 * g23 - g13 * g22) / det_g;
                let inv_g23 = (g12 * g13 - g23 * g11) / det_g;

                let inv_d2 = h_f * h_f * inv_g11
                    + k_f * k_f * inv_g22
                    + l_f * l_f * inv_g33
                    + 2.0 * h_f * k_f * inv_g12
                    + 2.0 * h_f * l_f * inv_g13
                    + 2.0 * k_f * l_f * inv_g23;

                if inv_d2 > 1e-9 {
                    Some(1.0 / inv_d2.sqrt())
                } else {
                    None
                }
            }
        }
    }
}

// ============================================================================
// 2. X-RAY POWDER DIFFRACTION (XRD) ENGINE
// ============================================================================

/// Karakteristik Garis Puncak Difraksi XRD (Bragg Peak)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XrdPeak {
    pub hkl: (i32, i32, i32),
    pub two_theta_deg: f64, // Sudut hamburan 2θ dalam derajat
    pub d_spacing_angstrom: f64,
    pub relative_intensity: f64, // Intensitas relatif ternormalisasi [0.0 .. 100.0]
    pub f_structure_factor_sq: f64,
}

pub struct XrdSimulator {
    pub wavelength_angstrom: f64, // Panjang gelombang radiasi sinar-X (Cu-Kα1 = 1.54056 Å)
    pub two_theta_min_deg: f64,
    pub two_theta_max_deg: f64,
}

impl Default for XrdSimulator {
    fn default() -> Self {
        Self {
            wavelength_angstrom: 1.54056, // Cu-Kα1 baku industri
            two_theta_min_deg: 10.0,
            two_theta_max_deg: 90.0,
        }
    }
}

impl XrdSimulator {
    pub fn new(wavelength_angstrom: f64, two_theta_min_deg: f64, two_theta_max_deg: f64) -> Self {
        Self {
            wavelength_angstrom,
            two_theta_min_deg,
            two_theta_max_deg,
        }
    }

    /// Simulasi Puncak Bragg Difraksi Sinar-X Serbuk (Powder XRD)
    pub fn calculate_peaks(&self, cell: &PureUnitCell, max_index: i32) -> Vec<XrdPeak> {
        let mut raw_peaks = Vec::new();
        let max_intensity_ref = 1e-12f64;

        for h in 0..=max_index {
            for k in 0..=max_index {
                for l in 0..=max_index {
                    if h == 0 && k == 0 && l == 0 {
                        continue;
                    }

                    if let Some(d) = cell.d_spacing(h, k, l) {
                        // Hukum Bragg: λ = 2d sin(θ) => sin(θ) = λ / (2d)
                        let sin_theta = self.wavelength_angstrom / (2.0 * d);
                        if sin_theta <= 0.0 || sin_theta >= 1.0 {
                            continue; // Di luar jangkauan difraksi geometris
                        }

                        let theta_rad = sin_theta.asin();
                        let two_theta_deg = 2.0 * theta_rad.to_degrees();

                        if two_theta_deg < self.two_theta_min_deg || two_theta_deg > self.two_theta_max_deg {
                            continue;
                        }

                        // Faktor Struktur F_hkl = sum_j f_j * exp(2πi * (h*xj + k*yj + l*zj))
                        let mut f_real = 0.0;
                        let mut f_imag = 0.0;

                        if cell.atoms.is_empty() {
                            // Model kristal kisi titik sederhana
                            f_real = 1.0;
                        } else {
                            for atom in &cell.atoms {
                                let phase = 2.0
                                    * std::f64::consts::PI
                                    * (h as f64 * atom.fractional_coord[0]
                                        + k as f64 * atom.fractional_coord[1]
                                        + l as f64 * atom.fractional_coord[2]);
                                // Faktor hamburan atom mendekati nomor atom Z
                                let fj = atom.atomic_number as f64;
                                f_real += fj * phase.cos();
                                f_imag += fj * phase.sin();
                            }
                        }

                        let f_sq = f_real * f_real + f_imag * f_imag;
                        if f_sq < 1e-4 {
                            continue; // Puncak padam karena interferensi destruktif (systematic absence)
                        }

                        // Faktor Polarisasi Lorentz: Lp(θ) = (1 + cos²(2θ)) / (sin²θ * cosθ)
                        let cos_2theta = (2.0 * theta_rad).cos();
                        let sin_sq_theta = theta_rad.sin() * theta_rad.sin();
                        let cos_theta = theta_rad.cos();
                        let lp = if sin_sq_theta > 1e-6 && cos_theta > 1e-6 {
                            (1.0 + cos_2theta * cos_2theta) / (sin_sq_theta * cos_theta)
                        } else {
                            1.0
                        };

                        // Multiplisitas simetri perkiraan (powder multi-plane equivalence)
                        let non_zero_count = (if h != 0 { 1 } else { 0 })
                            + (if k != 0 { 1 } else { 0 })
                            + (if l != 0 { 1 } else { 0 });
                        let multiplicity = match non_zero_count {
                            1 => 6.0,
                            2 => 12.0,
                            _ => 24.0,
                        };

                        let intensity = f_sq * lp * multiplicity;

                        raw_peaks.push(XrdPeak {
                            hkl: (h, k, l),
                            two_theta_deg,
                            d_spacing_angstrom: d,
                            relative_intensity: intensity,
                            f_structure_factor_sq: f_sq,
                        });
                    }
                }
            }
        }

        // Urutkan berdasarkan 2θ menaik
        raw_peaks.sort_by(|a, b| a.two_theta_deg.partial_cmp(&b.two_theta_deg).unwrap());

        // Normalisasi intensitas ke skala [0.0 .. 100.0]
        let max_intensity = raw_peaks
            .iter()
            .map(|p| p.relative_intensity)
            .fold(max_intensity_ref, f64::max);

        for peak in &mut raw_peaks {
            peak.relative_intensity = (peak.relative_intensity / max_intensity) * 100.0;
        }

        raw_peaks
    }

    /// Sintesis Spektrum Kontinu 1D XRD Diffractogram dengan Profil Puncak Gaussian
    pub fn generate_diffractogram_spectrum(
        &self,
        peaks: &[XrdPeak],
        fwhm_deg: f64,
        step_deg: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        let fwhm = if fwhm_deg > 0.01 { fwhm_deg } else { 0.15 };
        let step = if step_deg > 0.001 { step_deg } else { 0.05 };
        let sigma = fwhm / (2.0 * (2.0 * 2.0f64.ln()).sqrt());

        let num_points = ((self.two_theta_max_deg - self.two_theta_min_deg) / step).ceil() as usize + 1;
        let mut x_vals = Vec::with_capacity(num_points);
        let mut y_vals = vec![0.0; num_points];

        for i in 0..num_points {
            let x = self.two_theta_min_deg + (i as f64) * step;
            x_vals.push(x);
        }

        for peak in peaks {
            let p_x = peak.two_theta_deg;
            let p_y = peak.relative_intensity;

            // Gaussian window sekitar +/- 4 sigma
            let window = 4.0 * sigma;
            let min_idx = (((p_x - window - self.two_theta_min_deg) / step).floor() as isize).max(0) as usize;
            let max_idx = (((p_x + window - self.two_theta_min_deg) / step).ceil() as usize).min(num_points);

            for i in min_idx..max_idx {
                let dx = x_vals[i] - p_x;
                let val = p_y * (-0.5 * (dx / sigma).powi(2)).exp();
                y_vals[i] += val;
            }
        }

        (x_vals, y_vals)
    }
}

// ============================================================================
// 3. BRIDGES DENGAN SUBSISTEM KRISTALOGRAFI / DFT LEGACY
// ============================================================================

impl From<PureUnitCell> for crate::materials::dft_solver::CrystalLattice {
    fn from(cell: PureUnitCell) -> Self {
        let mut elements = Vec::with_capacity(cell.atoms.len());
        let mut positions = Vec::with_capacity(cell.atoms.len());

        for a in cell.atoms {
            elements.push(a.element);
            positions.push(a.fractional_coord);
        }

        crate::materials::dft_solver::CrystalLattice {
            a_angstrom: cell.a,
            b_angstrom: cell.b,
            c_angstrom: cell.c,
            alpha_deg: cell.alpha_deg,
            beta_deg: cell.beta_deg,
            gamma_deg: cell.gamma_deg,
            atom_elements: elements,
            atomic_positions_fractional: positions,
        }
    }
}

impl From<crate::materials::dft_solver::CrystalLattice> for PureUnitCell {
    fn from(lattice: crate::materials::dft_solver::CrystalLattice) -> Self {
        let mut atoms = Vec::with_capacity(lattice.atom_elements.len());
        for (i, elem) in lattice.atom_elements.into_iter().enumerate() {
            let pos = if i < lattice.atomic_positions_fractional.len() {
                lattice.atomic_positions_fractional[i]
            } else {
                [0.0, 0.0, 0.0]
            };
            // Default atomic number estimasi
            let z = match elem.to_uppercase().as_str() {
                "H" => 1,
                "LI" => 3,
                "C" => 6,
                "N" => 7,
                "O" => 8,
                "SI" => 14,
                "P" => 15,
                "FE" => 26,
                _ => 14,
            };
            atoms.push(BasisAtom::new(&elem, z, pos[0], pos[1], pos[2]));
        }

        // Tentukan sistem kristal dari sudut dan sisi
        let eps = 1e-4;
        let is_cubic_angles = (lattice.alpha_deg - 90.0).abs() < eps
            && (lattice.beta_deg - 90.0).abs() < eps
            && (lattice.gamma_deg - 90.0).abs() < eps;

        let system = if is_cubic_angles {
            if (lattice.a_angstrom - lattice.b_angstrom).abs() < eps
                && (lattice.b_angstrom - lattice.c_angstrom).abs() < eps
            {
                CrystalSystem::Cubic
            } else if (lattice.a_angstrom - lattice.b_angstrom).abs() < eps {
                CrystalSystem::Tetragonal
            } else {
                CrystalSystem::Orthorhombic
            }
        } else if (lattice.alpha_deg - 90.0).abs() < eps
            && (lattice.beta_deg - 90.0).abs() < eps
            && (lattice.gamma_deg - 120.0).abs() < eps
        {
            CrystalSystem::Hexagonal
        } else if (lattice.alpha_deg - 90.0).abs() < eps && (lattice.gamma_deg - 90.0).abs() < eps {
            CrystalSystem::Monoclinic
        } else {
            CrystalSystem::Triclinic
        };

        PureUnitCell {
            crystal_system: system,
            a: lattice.a_angstrom,
            b: lattice.b_angstrom,
            c: lattice.c_angstrom,
            alpha_deg: lattice.alpha_deg,
            beta_deg: lattice.beta_deg,
            gamma_deg: lattice.gamma_deg,
            atoms,
        }
    }
}
