//! Native Tensor Engine untuk Komputasi AI & Operasi Linear Berkinerja Tinggi
//! Mendukung N-Dimensional Tensor, Matmul, SwiGLU/GELU, Softmax, LayerNorm, dan Cosine Similarity.

use serde::{Deserialize, Serialize};

/// Tensor N-Dimensi untuk Komputasi Linear AI & Deep Learning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tensor {
    pub shape: Vec<usize>,
    pub data: Vec<f32>,
}

impl Tensor {
    pub fn new(shape: Vec<usize>, data: Vec<f32>) -> Self {
        let expected_len: usize = shape.iter().product();
        assert_eq!(data.len(), expected_len, "Ukuran data tidak cocok dengan shape tensor");
        Self { shape, data }
    }

    pub fn zeros(shape: Vec<usize>) -> Self {
        let len: usize = shape.iter().product();
        Self {
            shape,
            data: vec![0.0; len],
        }
    }

    pub fn ones(shape: Vec<usize>) -> Self {
        let len: usize = shape.iter().product();
        Self {
            shape,
            data: vec![1.0; len],
        }
    }

    pub fn from_slice(shape: Vec<usize>, slice: &[f32]) -> Self {
        Self::new(shape, slice.to_vec())
    }

    /// Perkalian Matriks 2D (Matrix Multiplication: [M, K] x [K, N] -> [M, N])
    pub fn matmul(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape.len() != 2 || other.shape.len() != 2 {
            return Err("Matmul hanya mendukung tensor 2D".to_string());
        }
        let m = self.shape[0];
        let k1 = self.shape[1];
        let k2 = other.shape[0];
        let n = other.shape[1];

        if k1 != k2 {
            return Err(format!("Dimensi matriks tidak cocok: [{}, {}] x [{}, {}]", m, k1, k2, n));
        }

        let mut out_data = vec![0.0f32; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0f32;
                for k in 0..k1 {
                    sum += self.data[i * k1 + k] * other.data[k * n + j];
                }
                out_data[i * n + j] = sum;
            }
        }

        Ok(Tensor::new(vec![m, n], out_data))
    }

    /// Penjumlahan Element-wise
    pub fn add(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!("Shape tidak sama untuk add: {:?} vs {:?}", self.shape, other.shape));
        }
        let out_data: Vec<f32> = self.data.iter().zip(&other.data).map(|(a, b)| a + b).collect();
        Ok(Tensor::new(self.shape.clone(), out_data))
    }

    /// Perkalian Skalar
    pub fn scale(&self, scalar: f32) -> Tensor {
        let out_data: Vec<f32> = self.data.iter().map(|&x| x * scalar).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Aktivasi ReLU
    pub fn relu(&self) -> Tensor {
        let out_data: Vec<f32> = self.data.iter().map(|&x| x.max(0.0)).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Aktivasi GELU (Gaussian Error Linear Unit)
    pub fn gelu(&self) -> Tensor {
        const SQRT_2_OVER_PI: f32 = 0.7978845608;
        let out_data: Vec<f32> = self.data.iter().map(|&x| {
            0.5 * x * (1.0 + (SQRT_2_OVER_PI * (x + 0.044715 * x.powi(3))).tanh())
        }).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Aktivasi SiLU / Swish (x * sigmoid(x)) - Inti dari arsitektur LLaMA / Qwen
    pub fn silu(&self) -> Tensor {
        let out_data: Vec<f32> = self.data.iter().map(|&x| {
            x / (1.0 + (-x).exp())
        }).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Layer Normalization (RMSNorm / LayerNorm)
    pub fn rms_norm(&self, weight: &Tensor, eps: f32) -> Result<Tensor, String> {
        if self.shape.is_empty() {
            return Err("Tensor kosong".to_string());
        }
        let last_dim = *self.shape.last().unwrap();
        if weight.data.len() != last_dim {
            return Err("Weight shape tidak cocok dengan dimensi terakhir tensor".to_string());
        }

        let num_rows = self.data.len() / last_dim;
        let mut out_data = vec![0.0f32; self.data.len()];

        for row in 0..num_rows {
            let start = row * last_dim;
            let end = start + last_dim;
            let slice = &self.data[start..end];

            // Mean square
            let ms: f32 = slice.iter().map(|&x| x * x).sum::<f32>() / last_dim as f32;
            let rrms = 1.0 / (ms + eps).sqrt();

            for i in 0..last_dim {
                out_data[start + i] = slice[i] * rrms * weight.data[i];
            }
        }

        Ok(Tensor::new(self.shape.clone(), out_data))
    }

    /// Softmax untuk Probabilitas Token LLM
    pub fn softmax(&self) -> Tensor {
        let max_val = self.data.iter().cloned().fold(f32::MIN, f32::max);
        let exp_vals: Vec<f32> = self.data.iter().map(|&x| (x - max_val).exp()).collect();
        let sum_exp: f32 = exp_vals.iter().sum();

        let out_data: Vec<f32> = if sum_exp > 1e-6 {
            exp_vals.iter().map(|&x| x / sum_exp).collect()
        } else {
            exp_vals
        };

        Tensor::new(self.shape.clone(), out_data)
    }

    /// Cosine Similarity antara dua vektor embedding 1D
    pub fn cosine_similarity(&self, other: &Tensor) -> f32 {
        let dot: f32 = self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum();
        let norm_a: f32 = self.data.iter().map(|a| a * a).sum::<f32>().sqrt();
        let norm_b: f32 = other.data.iter().map(|b| b * b).sum::<f32>().sqrt();

        if norm_a > 1e-6 && norm_b > 1e-6 {
            dot / (norm_a * norm_b)
        } else {
            0.0
        }
    }
}
