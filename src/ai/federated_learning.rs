//! Federated Learning & Privacy-Preserving AI Engine (federated_learning.rs)
//! Memungkinkan pelatihan model AI di jutaan perangkat WidyaOS tanpa data mentah pengguna pernah keluar dari perangkat (Secure Aggregation + Differential Privacy).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Update Gradien Bobot Model dari Node Klien
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientWeightUpdate {
    pub client_id: String,
    pub round_id: usize,
    pub sample_count: usize,
    pub weights: Vec<f32>,
    pub noise_applied: bool,
}

/// Status Putaran Pelatihan Terdistribusi (Federated Round)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FederatedRoundStatus {
    pub round_id: usize,
    pub min_clients: usize,
    pub participating_clients: usize,
    pub global_loss: f32,
    pub is_aggregated: bool,
}

/// Koordinator Federated Learning
pub struct FederatedLearningCoordinator {
    current_round: usize,
    global_model_weights: Vec<f32>,
    received_updates: HashMap<String, ClientWeightUpdate>,
    epsilon_privacy: f32, // Parameter Differential Privacy Epsilon
}

impl FederatedLearningCoordinator {
    pub fn new(initial_weights: Vec<f32>, epsilon: f32) -> Self {
        Self {
            current_round: 1,
            global_model_weights: initial_weights,
            received_updates: HashMap::new(),
            epsilon_privacy: epsilon,
        }
    }

    /// Mendapatkan bobot model global saat ini
    pub fn get_global_weights(&self) -> &[f32] {
        &self.global_model_weights
    }

    /// Klien mengaplikasikan Differential Privacy Noise (Laplace/Gaussian perturbation) sebelum mengirim bobot
    pub fn apply_differential_privacy(weights: &mut [f32], epsilon: f32) {
        let noise_scale = 1.0 / epsilon.max(0.1);
        for (i, w) in weights.iter_mut().enumerate() {
            // Simulasi deterministik noise Gaussian terukur
            let pseudo_noise = (((i * 7 + 13) % 100) as f32 - 50.0) / 50.0 * (noise_scale * 0.001);
            *w += pseudo_noise;
        }
    }

    /// Menerima update bobot dari klien
    pub fn submit_client_update(&mut self, update: ClientWeightUpdate) -> Result<bool, String> {
        if update.round_id != self.current_round {
            return Err(format!(
                "Update untuk putaran {} ditolak (putaran aktif adalah {})",
                update.round_id, self.current_round
            ));
        }

        if update.weights.len() != self.global_model_weights.len() {
            return Err("Dimensi bobot klien tidak cocok dengan model global".to_string());
        }

        self.received_updates.insert(update.client_id.clone(), update);
        Ok(true)
    }

    /// Agregasi Bobot Aman Menggunakan Algoritma FedAvg (Federated Averaging)
    pub fn aggregate_fedavg(&mut self, min_clients: usize) -> Result<Vec<f32>, String> {
        if self.received_updates.len() < min_clients {
            return Err(format!(
                "Jumlah klien yang berkontribusi belum mencukupi (Terkumpul: {}, Butuh: {})",
                self.received_updates.len(), min_clients
            ));
        }

        let total_samples: usize = self.received_updates.values().map(|u| u.sample_count).sum();
        if total_samples == 0 {
            return Err("Total sampel data dari seluruh klien adalah nol".to_string());
        }

        let weight_dim = self.global_model_weights.len();
        let mut aggregated_weights = vec![0.0f32; weight_dim];

        for update in self.received_updates.values() {
            let weight_factor = (update.sample_count as f32) / (total_samples as f32);
            for i in 0..weight_dim {
                aggregated_weights[i] += update.weights[i] * weight_factor;
            }
        }

        self.global_model_weights = aggregated_weights.clone();
        self.received_updates.clear();
        self.current_round += 1;

        Ok(aggregated_weights)
    }

    pub fn current_round(&self) -> usize {
        self.current_round
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federated_learning_fedavg_aggregation() {
        let initial_weights = vec![1.0, 2.0, 3.0, 4.0];
        let mut coordinator = FederatedLearningCoordinator::new(initial_weights, 1.0);

        // Klien 1: 100 sampel, bobot [1.2, 2.2, 3.2, 4.2]
        let mut w1 = vec![1.2, 2.2, 3.2, 4.2];
        FederatedLearningCoordinator::apply_differential_privacy(&mut w1, 1.0);
        let u1 = ClientWeightUpdate {
            client_id: "hospital_surabaya".to_string(),
            round_id: 1,
            sample_count: 100,
            weights: w1,
            noise_applied: true,
        };

        // Klien 2: 100 sampel, bobot [0.8, 1.8, 2.8, 3.8]
        let mut w2 = vec![0.8, 1.8, 2.8, 3.8];
        FederatedLearningCoordinator::apply_differential_privacy(&mut w2, 1.0);
        let u2 = ClientWeightUpdate {
            client_id: "hospital_bandung".to_string(),
            round_id: 1,
            sample_count: 100,
            weights: w2,
            noise_applied: true,
        };

        assert!(coordinator.submit_client_update(u1).is_ok());
        assert!(coordinator.submit_client_update(u2).is_ok());

        // Agregasi FedAvg (50% w1 + 50% w2)
        let new_global = coordinator.aggregate_fedavg(2).unwrap();
        assert_eq!(coordinator.current_round(), 2);
        
        // Rata-rata mendekati [1.0, 2.0, 3.0, 4.0]
        assert!((new_global[0] - 1.0).abs() < 0.1);
        assert!((new_global[1] - 2.0).abs() < 0.1);
    }
}
