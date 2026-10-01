//! Modul Kuantisasi Bobot Tensor AI (INT8 & INT4 Q4_0)
//! Memungkinkan eksekusi model LLM dengan efisiensi memori tinggi di perangkat Edge & Server.

use crate::ai::tensor::Tensor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantType {
    Int8,
    Int4,
    Fp16,
}

/// Tensor Terkuantisasi dengan Skala (Scale) dan Zero-Point per Blok
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizedTensor {
    pub shape: Vec<usize>,
    pub quant_type: QuantType,
    pub block_size: usize,
    pub scales: Vec<f32>,
    pub zero_points: Vec<i8>,
    pub quantized_data: Vec<u8>,
}

impl QuantizedTensor {
    /// Kuantisasi FP32 Tensor ke INT8 simetris/asimetris
    pub fn quantize_int8(tensor: &Tensor, block_size: usize) -> Self {
        let mut scales = Vec::new();
        let mut zero_points = Vec::new();
        let mut quantized_data = Vec::with_capacity(tensor.data.len());

        let chunks = tensor.data.chunks(block_size);
        for chunk in chunks {
            let min_val = chunk.iter().cloned().fold(f32::MAX, f32::min);
            let max_val = chunk.iter().cloned().fold(f32::MIN, f32::max);

            let scale = if (max_val - min_val).abs() > 1e-7 {
                (max_val - min_val) / 255.0
            } else {
                1.0
            };

            let zero_point = (-min_val / scale).round().clamp(0.0, 255.0) as i8;

            scales.push(scale);
            zero_points.push(zero_point);

            for &val in chunk {
                let q = ((val - min_val) / scale).round().clamp(0.0, 255.0) as u8;
                quantized_data.push(q);
            }
        }

        Self {
            shape: tensor.shape.clone(),
            quant_type: QuantType::Int8,
            block_size,
            scales,
            zero_points,
            quantized_data,
        }
    }

    /// Kuantisasi FP32 Tensor ke INT4 (2 nilai 4-bit dipadatkan dalam 1 byte u8)
    pub fn quantize_int4(tensor: &Tensor, block_size: usize) -> Self {
        let mut scales = Vec::new();
        let mut zero_points = Vec::new();
        let mut quantized_data = Vec::with_capacity((tensor.data.len() + 1) / 2);

        let chunks = tensor.data.chunks(block_size);
        for chunk in chunks {
            let max_abs = chunk.iter().cloned().fold(0.0f32, |acc, x| acc.max(x.abs()));
            let scale = if max_abs > 1e-7 { max_abs / 7.0 } else { 1.0 };
            scales.push(scale);
            zero_points.push(0);

            // Padatkan tiap 2 nilai 4-bit (0..15) ke 1 byte
            for pair in chunk.chunks(2) {
                let q0 = ((pair[0] / scale).round().clamp(-7.0, 7.0) + 7.0) as u8;
                let q1 = if pair.len() > 1 {
                    ((pair[1] / scale).round().clamp(-7.0, 7.0) + 7.0) as u8
                } else {
                    7
                };
                let packed = (q0 & 0x0F) | ((q1 & 0x0F) << 4);
                quantized_data.push(packed);
            }
        }

        Self {
            shape: tensor.shape.clone(),
            quant_type: QuantType::Int4,
            block_size,
            scales,
            zero_points,
            quantized_data,
        }
    }

    /// Rekonstruksi (Dequantize) kembali ke FP32 Tensor
    pub fn dequantize(&self) -> Tensor {
        let mut out_data = Vec::new();

        match self.quant_type {
            QuantType::Int8 => {
                let total_elements: usize = self.shape.iter().product();
                let mut data_idx = 0;
                for (block_idx, &scale) in self.scales.iter().enumerate() {
                    let zp = self.zero_points[block_idx] as f32;
                    let count = self.block_size.min(total_elements - data_idx);
                    for _ in 0..count {
                        let q = self.quantized_data[data_idx] as f32;
                        let val = (q - zp) * scale;
                        out_data.push(val);
                        data_idx += 1;
                    }
                }
            }
            QuantType::Int4 => {
                let total_elements: usize = self.shape.iter().product();
                let mut byte_idx = 0;
                for (block_idx, &scale) in self.scales.iter().enumerate() {
                    let count = self.block_size.min(total_elements - out_data.len());
                    let mut elements_in_block = 0;
                    while elements_in_block < count && byte_idx < self.quantized_data.len() {
                        let packed = self.quantized_data[byte_idx];
                        byte_idx += 1;

                        let q0 = (packed & 0x0F) as f32 - 7.0;
                        out_data.push(q0 * scale);
                        elements_in_block += 1;

                        if elements_in_block < count {
                            let q1 = ((packed >> 4) & 0x0F) as f32 - 7.0;
                            out_data.push(q1 * scale);
                            elements_in_block += 1;
                        }
                    }
                }
            }
            QuantType::Fp16 => {
                // Passthrough
            }
        }

        Tensor::new(self.shape.clone(), out_data)
    }
}
