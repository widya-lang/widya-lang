//! DLPack Protocol & Safetensors Header Specification for Widya-Lang (dlpack.rs)
//! Memungkinkan interoperabilitas Tensor zero-copy tingkat industri antara Widya, PyTorch, JAX, CuPy, dan format Safetensors.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipe Data DLPack standar industri
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DLDataTypeCode {
    Float = 2,
    Int = 0,
    UInt = 1,
    Bfloat = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DLDataType {
    pub code: DLDataTypeCode,
    pub bits: u8,
    pub lanes: u16,
}

impl DLDataType {
    pub fn f32() -> Self {
        Self {
            code: DLDataTypeCode::Float,
            bits: 32,
            lanes: 1,
        }
    }

    pub fn f16() -> Self {
        Self {
            code: DLDataTypeCode::Float,
            bits: 16,
            lanes: 1,
        }
    }

    pub fn bf16() -> Self {
        Self {
            code: DLDataTypeCode::Bfloat,
            bits: 16,
            lanes: 1,
        }
    }
}

/// Tipe Perangkat Komputasi DLPack
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DLDeviceType {
    CPU = 1,
    CUDA = 2,
    ROCm = 10,
    Vulkan = 7,
    Metal = 8,
    WebGPU = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DLDevice {
    pub device_type: DLDeviceType,
    pub device_id: i32,
}

/// Struktur DLTensor standar DLPack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DLTensor {
    pub shape: Vec<i64>,
    pub strides: Option<Vec<i64>>,
    pub byte_offset: u64,
    pub dtype: DLDataType,
    pub device: DLDevice,
}

impl DLTensor {
    pub fn new_cpu_f32(shape: Vec<i64>) -> Self {
        let ndim = shape.len();
        let mut strides = vec![1i64; ndim];
        for i in (0..ndim.saturating_sub(1)).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }

        Self {
            shape,
            strides: Some(strides),
            byte_offset: 0,
            dtype: DLDataType::f32(),
            device: DLDevice {
                device_type: DLDeviceType::CPU,
                device_id: 0,
            },
        }
    }

    pub fn is_contiguous(&self) -> bool {
        if let Some(ref strides) = self.strides {
            let mut expected_stride = 1i64;
            for i in (0..self.shape.len()).rev() {
                if strides[i] != expected_stride {
                    return false;
                }
                expected_stride *= self.shape[i];
            }
            true
        } else {
            true
        }
    }

    pub fn total_elements(&self) -> usize {
        self.shape.iter().map(|&x| x.max(0) as usize).product()
    }
}

/// Metadata Tensor dalam format Safetensors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetensorMeta {
    pub dtype: String,
    pub shape: Vec<usize>,
    pub data_offsets: (u64, u64),
}

/// Parser Safetensors Header
pub struct SafetensorsParser;

impl SafetensorsParser {
    /// Parsing header JSON dari format biner Safetensors
    pub fn parse_header_json(header_json_str: &str) -> Result<HashMap<String, SafetensorMeta>, String> {
        let raw_map: HashMap<String, serde_json::Value> = serde_json::from_str(header_json_str)
            .map_err(|e| format!("Gagal mem-parsing header Safetensors: {}", e))?;

        let mut result = HashMap::new();
        for (name, val) in raw_map {
            if name == "__metadata__" {
                continue;
            }

            let dtype = val.get("dtype")
                .and_then(|d| d.as_str())
                .unwrap_or("F32")
                .to_string();

            let shape: Vec<usize> = val.get("shape")
                .and_then(|s| s.as_array())
                .map(|arr| arr.iter().filter_map(|x| x.as_u64().map(|v| v as usize)).collect())
                .unwrap_or_default();

            let offsets: (u64, u64) = val.get("data_offsets")
                .and_then(|o| o.as_array())
                .and_then(|arr| {
                    if arr.len() == 2 {
                        let start = arr[0].as_u64()?;
                        let end = arr[1].as_u64()?;
                        Some((start, end))
                    } else {
                        None
                    }
                })
                .unwrap_or((0, 0));

            result.insert(name, SafetensorMeta {
                dtype,
                shape,
                data_offsets: offsets,
            });
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dltensor_strides_and_contiguity() {
        let tensor = DLTensor::new_cpu_f32(vec![2, 3, 4]);
        assert_eq!(tensor.total_elements(), 24);
        assert!(tensor.is_contiguous());
        assert_eq!(tensor.strides, Some(vec![12, 4, 1]));
    }

    #[test]
    fn test_safetensors_header_parser() {
        let json_data = r#"{
            "model.embed_tokens.weight": {
                "dtype": "F32",
                "shape": [32000, 4096],
                "data_offsets": [0, 524288000]
            },
            "__metadata__": {
                "format": "pt"
            }
        }"#;

        let parsed = SafetensorsParser::parse_header_json(json_data).unwrap();
        assert!(parsed.contains_key("model.embed_tokens.weight"));
        let meta = &parsed["model.embed_tokens.weight"];
        assert_eq!(meta.dtype, "F32");
        assert_eq!(meta.shape, vec![32000, 4096]);
        assert_eq!(meta.data_offsets, (0, 524288000));
    }
}
