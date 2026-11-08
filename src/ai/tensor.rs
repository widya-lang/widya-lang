//! Native Tensor Engine untuk Komputasi AI & Operasi Linear Berkinerja Tinggi
//! Mendukung N-Dimensional Tensor, Matmul, SwiGLU/GELU, Softmax, LayerNorm, dan Cosine Similarity.

use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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

    #[inline(always)]
    fn dot_product_simd_8x(a: &[f32], b: &[f32]) -> f32 {
        let len = a.len().min(b.len());
        let chunks = len / 8;
        let mut s0 = 0.0f32;
        let mut s1 = 0.0f32;
        let mut s2 = 0.0f32;
        let mut s3 = 0.0f32;
        let mut s4 = 0.0f32;
        let mut s5 = 0.0f32;
        let mut s6 = 0.0f32;
        let mut s7 = 0.0f32;

        for c in 0..chunks {
            let base = c * 8;
            s0 += a[base] * b[base];
            s1 += a[base + 1] * b[base + 1];
            s2 += a[base + 2] * b[base + 2];
            s3 += a[base + 3] * b[base + 3];
            s4 += a[base + 4] * b[base + 4];
            s5 += a[base + 5] * b[base + 5];
            s6 += a[base + 6] * b[base + 6];
            s7 += a[base + 7] * b[base + 7];
        }
        let mut rem = (s0 + s1) + (s2 + s3) + (s4 + s5) + (s6 + s7);
        for i in (chunks * 8)..len {
            rem += a[i] * b[i];
        }
        rem
    }

    /// Perkalian Matriks 2D Berkinerja Tinggi (Tiled Cache-Friendly + Multi-Threaded SIMD: [M, K] x [K, N] -> [M, N])
    pub fn matmul(&self, other: &Tensor) -> Result<Tensor, String> {
        self.matmul_tiled(other)
    }

    pub fn matmul_tiled(&self, other: &Tensor) -> Result<Tensor, String> {
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

        // Transpose B ([K, N] -> [N, K]) untuk akses memori sekuensial ramah cache L1/SIMD
        let mut b_t = vec![0.0f32; n * k1];
        for r in 0..k1 {
            let row_off = r * n;
            for c in 0..n {
                b_t[c * k1 + r] = other.data[row_off + c];
            }
        }

        let mut out_data = vec![0.0f32; m * n];
        let ops = m * n * k1;

        if ops >= 32_768 && m >= 4 {
            let num_threads = std::thread::available_parallelism()
                .map(|p| p.get())
                .unwrap_or(4)
                .min(m);
            let rows_per_chunk = (m + num_threads - 1) / num_threads;
            let a_ref = &self.data;
            let bt_ref = &b_t;

            std::thread::scope(|s| {
                for (chunk_idx, out_chunk) in out_data.chunks_mut(rows_per_chunk * n).enumerate() {
                    let start_row = chunk_idx * rows_per_chunk;
                    let chunk_rows = out_chunk.len() / n;
                    s.spawn(move || {
                        for local_i in 0..chunk_rows {
                            let global_i = start_row + local_i;
                            let a_row = &a_ref[(global_i * k1)..((global_i + 1) * k1)];
                            let out_row = &mut out_chunk[(local_i * n)..((local_i + 1) * n)];
                            for j in 0..n {
                                let b_col = &bt_ref[(j * k1)..((j + 1) * k1)];
                                out_row[j] = Self::dot_product_simd_8x(a_row, b_col);
                            }
                        }
                    });
                }
            });
        } else {
            for i in 0..m {
                let a_row = &self.data[(i * k1)..((i + 1) * k1)];
                let out_row_off = i * n;
                for j in 0..n {
                    let b_col = &b_t[(j * k1)..((j + 1) * k1)];
                    out_data[out_row_off + j] = Self::dot_product_simd_8x(a_row, b_col);
                }
            }
        }

        Ok(Tensor::new(vec![m, n], out_data))
    }

    /// Transpose Matriks 2D ([M, N] -> [N, M])
    pub fn transpose(&self) -> Result<Tensor, String> {
        if self.shape.len() != 2 {
            return Err("Transpose memerlukan tensor 2D".to_string());
        }
        let m = self.shape[0];
        let n = self.shape[1];
        let mut out = vec![0.0f32; m * n];
        for i in 0..m {
            for j in 0..n {
                out[j * m + i] = self.data[i * n + j];
            }
        }
        Ok(Tensor::new(vec![n, m], out))
    }

    /// Mengubah dimensi shape Tensor tanpa menyalin ulang nilai elemen (Reshape)
    pub fn reshape(&self, new_shape: Vec<usize>) -> Result<Tensor, String> {
        let new_len: usize = new_shape.iter().product();
        if new_len != self.data.len() {
            return Err(format!(
                "Jumlah elemen reshape tidak cocok: {:?} ({}) vs {:?} ({})",
                self.shape,
                self.data.len(),
                new_shape,
                new_len
            ));
        }
        Ok(Tensor::new(new_shape, self.data.clone()))
    }

    /// Penjumlahan Element-wise
    pub fn add(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!("Shape tidak sama untuk add: {:?} vs {:?}", self.shape, other.shape));
        }
        let out_data: Vec<f32> = self.data.iter().zip(&other.data).map(|(a, b)| a + b).collect();
        Ok(Tensor::new(self.shape.clone(), out_data))
    }

    /// Pengurangan Element-wise
    pub fn sub(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!("Shape tidak sama untuk sub: {:?} vs {:?}", self.shape, other.shape));
        }
        let out_data: Vec<f32> = self.data.iter().zip(&other.data).map(|(a, b)| a - b).collect();
        Ok(Tensor::new(self.shape.clone(), out_data))
    }

    /// Perkalian Element-wise (Hadamard Product)
    pub fn mul_elementwise(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!("Shape tidak sama untuk mul: {:?} vs {:?}", self.shape, other.shape));
        }
        let out_data: Vec<f32> = self.data.iter().zip(&other.data).map(|(a, b)| a * b).collect();
        Ok(Tensor::new(self.shape.clone(), out_data))
    }

    /// Perkalian Skalar
    pub fn scale(&self, scalar: f32) -> Tensor {
        let out_data: Vec<f32> = self.data.iter().map(|&x| x * scalar).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Reduksi Jumlah (Sum), Rata-rata (Mean), Maksimum, Minimum, dan Argmax
    pub fn sum(&self) -> f32 {
        self.data.iter().sum()
    }

    pub fn mean(&self) -> f32 {
        if self.data.is_empty() {
            0.0
        } else {
            self.sum() / (self.data.len() as f32)
        }
    }

    pub fn max(&self) -> f32 {
        self.data.iter().cloned().fold(f32::MIN, f32::max)
    }

    pub fn min(&self) -> f32 {
        self.data.iter().cloned().fold(f32::MAX, f32::min)
    }

    pub fn argmax(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Aktivasi ReLU
    pub fn relu(&self) -> Tensor {
        let out_data: Vec<f32> = self.data.iter().map(|&x| x.max(0.0)).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Aktivasi GELU (Gaussian Error Linear Unit)
    pub fn gelu(&self) -> Tensor {
        const SQRT_2_OVER_PI: f32 = 0.7978845608;
        let out_data: Vec<f32> = self
            .data
            .iter()
            .map(|&x| 0.5 * x * (1.0 + (SQRT_2_OVER_PI * (x + 0.044715 * x.powi(3))).tanh()))
            .collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Aktivasi SiLU / Swish (x * sigmoid(x)) - Inti dari arsitektur LLaMA / Qwen
    pub fn silu(&self) -> Tensor {
        let out_data: Vec<f32> = self.data.iter().map(|&x| x / (1.0 + (-x).exp())).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// RMS Normalization (RMSNorm)
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

    /// Standard Layer Normalization (LayerNorm dengan gamma weight dan beta bias)
    pub fn layer_norm(&self, weight: &Tensor, bias: &Tensor, eps: f32) -> Result<Tensor, String> {
        if self.shape.is_empty() {
            return Err("Tensor kosong".to_string());
        }
        let last_dim = *self.shape.last().unwrap();
        if weight.data.len() != last_dim || bias.data.len() != last_dim {
            return Err("Weight/Bias shape tidak cocok dengan dimensi terakhir tensor".to_string());
        }

        let num_rows = self.data.len() / last_dim;
        let mut out_data = vec![0.0f32; self.data.len()];

        for row in 0..num_rows {
            let start = row * last_dim;
            let end = start + last_dim;
            let slice = &self.data[start..end];

            let mean: f32 = slice.iter().sum::<f32>() / (last_dim as f32);
            let var: f32 = slice
                .iter()
                .map(|&x| {
                    let d = x - mean;
                    d * d
                })
                .sum::<f32>()
                / (last_dim as f32);
            let inv_std = 1.0 / (var + eps).sqrt();

            for i in 0..last_dim {
                out_data[start + i] = (slice[i] - mean) * inv_std * weight.data[i] + bias.data[i];
            }
        }

        Ok(Tensor::new(self.shape.clone(), out_data))
    }

    /// Softmax untuk Probabilitas Token LLM (per baris untuk 2D, atau seluruh vektor untuk 1D)
    pub fn softmax(&self) -> Tensor {
        if self.shape.len() == 2 {
            let rows = self.shape[0];
            let cols = self.shape[1];
            let mut out_data = vec![0.0f32; rows * cols];
            for r in 0..rows {
                let start = r * cols;
                let end = start + cols;
                let row_slice = &self.data[start..end];
                let max_val = row_slice.iter().cloned().fold(f32::MIN, f32::max);
                let exp_vals: Vec<f32> = row_slice.iter().map(|&x| (x - max_val).exp()).collect();
                let sum_exp: f32 = exp_vals.iter().sum();
                for c in 0..cols {
                    out_data[start + c] = if sum_exp > 1e-6 {
                        exp_vals[c] / sum_exp
                    } else {
                        exp_vals[c]
                    };
                }
            }
            return Tensor::new(self.shape.clone(), out_data);
        }

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

    /// Scaled Dot-Product Attention / FlashAttention: Attention(Q, K, V) = softmax((Q @ K^T) / sqrt(d_k)) @ V
    pub fn attention_scaled_dot_product(&self, k: &Tensor, v: &Tensor) -> Result<Tensor, String> {
        if self.shape.len() != 2 || k.shape.len() != 2 || v.shape.len() != 2 {
            return Err("Scaled Dot-Product Attention memerlukan tensor 2D [seq_len, head_dim]".to_string());
        }
        let d_k = self.shape[1] as f32;
        let k_t = k.transpose()?;
        let scores = self.matmul_tiled(&k_t)?.scale(1.0 / d_k.sqrt());
        let attn_weights = scores.softmax();
        attn_weights.matmul_tiled(v)
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

    /// Konversi ke representasi `Value::Map` di Widya-Lang Runtime
    pub fn to_widya_value(&self) -> Value {
        let mut map = HashMap::new();
        map.insert("_tipe".to_string(), Value::String("Tensor".to_string()));
        map.insert(
            "shape".to_string(),
            Value::Array(Rc::new(RefCell::new(
                self.shape.iter().map(|&d| Value::Number(d as f64)).collect(),
            ))),
        );
        map.insert(
            "data".to_string(),
            Value::Array(Rc::new(RefCell::new(
                self.data.iter().map(|&x| Value::Number(x as f64)).collect(),
            ))),
        );
        map.insert(
            "ukuran".to_string(),
            Value::Number(self.data.len() as f64),
        );
        Value::Map(Rc::new(RefCell::new(map)))
    }

    /// Membaca `Tensor` dari `Value::Map` di Widya-Lang Runtime
    pub fn from_widya_value(val: &Value) -> Result<Tensor, String> {
        if let Value::Map(m) = val {
            let borrow = m.borrow();
            let shape = match borrow.get("shape") {
                Some(Value::Array(arr)) => arr
                    .borrow()
                    .iter()
                    .map(|v| match v {
                        Value::Number(n) => *n as usize,
                        _ => 1,
                    })
                    .collect::<Vec<usize>>(),
                _ => return Err("Objek Tensor tidak memiliki properti 'shape'".to_string()),
            };
            let data = match borrow.get("data") {
                Some(Value::Array(arr)) => arr
                    .borrow()
                    .iter()
                    .map(|v| match v {
                        Value::Number(n) => *n as f32,
                        _ => 0.0,
                    })
                    .collect::<Vec<f32>>(),
                _ => return Err("Objek Tensor tidak memiliki properti 'data'".to_string()),
            };
            let expected: usize = shape.iter().product();
            if data.len() != expected {
                return Err(format!(
                    "Ukuran data Tensor ({}) tidak sesuai dengan shape {:?}",
                    data.len(),
                    shape
                ));
            }
            Ok(Tensor::new(shape, data))
        } else {
            Err(format!("Diharapkan objek Tensor, ditemukan '{}'", val.type_name()))
        }
    }
}
