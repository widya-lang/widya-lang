use serde::{Deserialize, Serialize};

/// ============================================================================
/// Molecular Dynamics Trajectory (DCD / PDB) Parser & RMSD Analysis
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryFrame {
    pub step_number: u64,
    pub time_picoseconds: f64,
    pub atomic_coordinates: Vec<[f64; 3]>,
}

pub struct TrajectoryAnalyzer {
    pub num_atoms: usize,
    pub frames: Vec<TrajectoryFrame>,
}

impl TrajectoryAnalyzer {
    pub fn new(num_atoms: usize) -> Self {
        Self {
            num_atoms,
            frames: Vec::new(),
        }
    }

    pub fn add_frame(&mut self, step: u64, time_ps: f64, coords: Vec<[f64; 3]>) {
        self.frames.push(TrajectoryFrame {
            step_number: step,
            time_picoseconds: time_ps,
            atomic_coordinates: coords,
        });
    }

    /// Hitung Root-Mean-Square Deviation (RMSD) terhadap frame referensi (Frame 0)
    pub fn compute_rmsd_profile(&self) -> Vec<f64> {
        if self.frames.is_empty() {
            return Vec::new();
        }

        let ref_coords = &self.frames[0].atomic_coordinates;
        let mut rmsd_list = Vec::with_capacity(self.frames.len());

        for frame in &self.frames {
            let mut sum_sq_diff = 0.0;
            for (c1, c2) in frame.atomic_coordinates.iter().zip(ref_coords.iter()) {
                sum_sq_diff += (c1[0] - c2[0]).powi(2) + (c1[1] - c2[1]).powi(2) + (c1[2] - c2[2]).powi(2);
            }
            let rmsd = (sum_sq_diff / (self.num_atoms.max(1) as f64)).sqrt();
            rmsd_list.push(rmsd);
        }

        rmsd_list
    }
}
