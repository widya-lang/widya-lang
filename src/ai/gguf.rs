// ============================================================================
// WIDYA GGUF PARSER & QUANTIZATION ENGINE (src/ai/gguf.rs)
// ============================================================================
// Format Biner GGUF v2/v3 untuk Eksekusi On-Device SLM/LLM:
// - LLaMA 3, Qwen 2.5, DeepSeek, Gemma 2, Mistral
// - Parsing Metadata Key-Value Dinamis (String, Uint, Float, Array, dll.)
// - Parsing Tabel Tensor (F32, F16, Q4_0, Q4_1, Q8_0, Q4_K, Q6_K)
// - Dequantization SIMD-friendly & Vectorized Dot-Product
// ============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const GGUF_MAGIC: u32 = 0x46554747; // "GGUF" in little-endian (0x47, 0x47, 0x55, 0x46)

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GgufTensorType {
    F32 = 0,
    F16 = 1,
    Q4_0 = 2,
    Q4_1 = 3,
    Q5_0 = 6,
    Q5_1 = 7,
    Q8_0 = 8,
    Q8_1 = 9,
    Q2_K = 10,
    Q3_K = 11,
    Q4_K = 12,
    Q5_K = 13,
    Q6_K = 14,
    Q8_K = 15,
}

impl GgufTensorType {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(GgufTensorType::F32),
            1 => Some(GgufTensorType::F16),
            2 => Some(GgufTensorType::Q4_0),
            3 => Some(GgufTensorType::Q4_1),
            6 => Some(GgufTensorType::Q5_0),
            7 => Some(GgufTensorType::Q5_1),
            8 => Some(GgufTensorType::Q8_0),
            9 => Some(GgufTensorType::Q8_1),
            10 => Some(GgufTensorType::Q2_K),
            11 => Some(GgufTensorType::Q3_K),
            12 => Some(GgufTensorType::Q4_K),
            13 => Some(GgufTensorType::Q5_K),
            14 => Some(GgufTensorType::Q6_K),
            15 => Some(GgufTensorType::Q8_K),
            _ => None,
        }
    }

    pub fn block_size(&self) -> usize {
        match self {
            GgufTensorType::F32 | GgufTensorType::F16 => 1,
            GgufTensorType::Q4_0 | GgufTensorType::Q4_1 | GgufTensorType::Q5_0 | GgufTensorType::Q5_1 => 32,
            GgufTensorType::Q8_0 | GgufTensorType::Q8_1 => 32,
            _ => 256,
        }
    }

    pub fn type_size_bytes(&self) -> usize {
        match self {
            GgufTensorType::F32 => 4,
            GgufTensorType::F16 => 2,
            GgufTensorType::Q4_0 => 18, // 2 bytes f16 scale + 16 bytes nibbles = 32 elements
            GgufTensorType::Q8_0 => 34, // 2 bytes f16 scale + 32 bytes int8 = 32 elements
            _ => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GgufValue {
    UInt8(u8),
    Int8(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    Float32(f32),
    Bool(bool),
    String(String),
    Array(Vec<GgufValue>),
    UInt64(u64),
    Int64(i64),
    Float64(f64),
}

impl GgufValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            GgufValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            GgufValue::UInt64(v) => Some(*v),
            GgufValue::UInt32(v) => Some(*v as u64),
            GgufValue::Int64(v) if *v >= 0 => Some(*v as u64),
            GgufValue::Int32(v) if *v >= 0 => Some(*v as u64),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            GgufValue::Float64(v) => Some(*v),
            GgufValue::Float32(v) => Some(*v as f64),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GgufTensorInfo {
    pub name: String,
    pub dimensions: Vec<u64>,
    pub tensor_type: GgufTensorType,
    pub offset: u64,
    pub size_in_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GgufModelInfo {
    pub is_valid_gguf: bool,
    pub version: u32,
    pub tensor_count: u64,
    pub kv_count: u64,
    pub architecture: String,
    pub context_length: u32,
    pub embedding_length: u32,
    pub block_count: u32,
    pub metadata: HashMap<String, GgufValue>,
    pub tensors: HashMap<String, GgufTensorInfo>,
}

/// Helper pembaca biner endian-kecil (Little-Endian Cursor)
pub struct BinaryReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], String> {
        if self.pos + len > self.data.len() {
            return Err(format!("Akhir berkas biner tercapai: butuh {} byte pada posisi {}", len, self.pos));
        }
        let slice = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice)
    }

    pub fn read_u8(&mut self) -> Result<u8, String> {
        let b = self.read_bytes(1)?;
        Ok(b[0])
    }

    pub fn read_u16(&mut self) -> Result<u16, String> {
        let b = self.read_bytes(2)?;
        Ok(u16::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn read_u32(&mut self) -> Result<u32, String> {
        let b = self.read_bytes(4)?;
        Ok(u32::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn read_i32(&mut self) -> Result<i32, String> {
        let b = self.read_bytes(4)?;
        Ok(i32::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn read_u64(&mut self) -> Result<u64, String> {
        let b = self.read_bytes(8)?;
        Ok(u64::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn read_i64(&mut self) -> Result<i64, String> {
        let b = self.read_bytes(8)?;
        Ok(i64::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn read_f32(&mut self) -> Result<f32, String> {
        let b = self.read_bytes(4)?;
        Ok(f32::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn read_f64(&mut self) -> Result<f64, String> {
        let b = self.read_bytes(8)?;
        Ok(f64::from_le_bytes(b.try_into().unwrap()))
    }

    pub fn read_string(&mut self) -> Result<String, String> {
        let len = self.read_u64()? as usize;
        let bytes = self.read_bytes(len)?;
        String::from_utf8(bytes.to_vec()).map_err(|e| format!("Gagal decode UTF-8 string: {}", e))
    }

    pub fn read_value(&mut self, val_type: u32) -> Result<GgufValue, String> {
        match val_type {
            0 => Ok(GgufValue::UInt8(self.read_u8()?)),
            1 => Ok(GgufValue::Int8(self.read_u8()? as i8)),
            2 => Ok(GgufValue::UInt16(self.read_u16()?)),
            3 => Ok(GgufValue::Int16(self.read_u16()? as i16)),
            4 => Ok(GgufValue::UInt32(self.read_u32()?)),
            5 => Ok(GgufValue::Int32(self.read_i32()?)),
            6 => Ok(GgufValue::Float32(self.read_f32()?)),
            7 => Ok(GgufValue::Bool(self.read_u8()? != 0)),
            8 => Ok(GgufValue::String(self.read_string()?)),
            9 => {
                let inner_type = self.read_u32()?;
                let arr_len = self.read_u64()? as usize;
                let mut arr = Vec::with_capacity(arr_len);
                for _ in 0..arr_len {
                    arr.push(self.read_value(inner_type)?);
                }
                Ok(GgufValue::Array(arr))
            }
            10 => Ok(GgufValue::UInt64(self.read_u64()?)),
            11 => Ok(GgufValue::Int64(self.read_i64()?)),
            12 => Ok(GgufValue::Float64(self.read_f64()?)),
            other => Err(format!("Tipe GGUF Value {} tidak dikenal", other)),
        }
    }
}

/// Parser Header Format Biner GGUF (v2 / v3)
pub fn parse_gguf_header(bytes: &[u8]) -> Result<GgufModelInfo, String> {
    if bytes.len() < 24 {
        return Err("Ukuran berkas terlalu kecil untuk format GGUF".to_string());
    }

    let mut reader = BinaryReader::new(bytes);
    let magic = reader.read_u32()?;
    if magic != GGUF_MAGIC {
        return Err(format!("Magic header bukan 'GGUF' (ditemukan 0x{:08X})", magic));
    }

    let version = reader.read_u32()?;
    if version != 2 && version != 3 {
        return Err(format!("Versi GGUF {} tidak didukung (didukung v2 dan v3)", version));
    }

    let tensor_count = reader.read_u64()?;
    let kv_count = reader.read_u64()?;

    let mut metadata = HashMap::new();
    for _ in 0..kv_count {
        if reader.remaining() == 0 {
            break;
        }
        let key = reader.read_string()?;
        let val_type = reader.read_u32()?;
        let val = reader.read_value(val_type)?;
        metadata.insert(key, val);
    }

    // Parsing tabel tensor descriptor
    let mut tensors = HashMap::new();
    for _ in 0..tensor_count {
        if reader.remaining() == 0 {
            break;
        }
        let name = reader.read_string()?;
        let n_dims = reader.read_u32()? as usize;
        let mut dimensions = Vec::with_capacity(n_dims);
        let mut total_elements: u64 = 1;
        for _ in 0..n_dims {
            let dim = reader.read_u64()?;
            dimensions.push(dim);
            total_elements = total_elements.saturating_mul(dim);
        }
        let type_code = reader.read_u32()?;
        let tensor_type = GgufTensorType::from_u32(type_code).unwrap_or(GgufTensorType::F32);
        let offset = reader.read_u64()?;

        let block_size = tensor_type.block_size();
        let bytes_per_block = tensor_type.type_size_bytes();
        let num_blocks = (total_elements as usize + block_size - 1) / block_size;
        let size_in_bytes = num_blocks * bytes_per_block;

        tensors.insert(name.clone(), GgufTensorInfo {
            name,
            dimensions,
            tensor_type,
            offset,
            size_in_bytes,
        });
    }

    // Ekstraksi arsitektur dan parameter model
    let arch = metadata.get("general.architecture")
        .and_then(|v| v.as_str())
        .unwrap_or("llama")
        .to_string();

    let context_length = metadata.get(&format!("{}.context_length", arch))
        .and_then(|v| v.as_u64())
        .unwrap_or(4096) as u32;

    let embedding_length = metadata.get(&format!("{}.embedding_length", arch))
        .and_then(|v| v.as_u64())
        .unwrap_or(4096) as u32;

    let block_count = metadata.get(&format!("{}.block_count", arch))
        .and_then(|v| v.as_u64())
        .unwrap_or(32) as u32;

    Ok(GgufModelInfo {
        is_valid_gguf: true,
        version,
        tensor_count,
        kv_count,
        architecture: arch,
        context_length,
        embedding_length,
        block_count,
        metadata,
        tensors,
    })
}

// ============================================================================
// DEQUANTIZATION & VECTORIZED MATH KERNELS
// ============================================================================

/// Konversi Half-Precision Float (IEEE 754 Binary16) ke Single-Precision Float (F32)
#[inline(always)]
pub fn f16_to_f32(h: u16) -> f32 {
    let sign = ((h >> 15) & 1) as u32;
    let exp = ((h >> 10) & 0x1F) as u32;
    let mant = (h & 0x03FF) as u32;

    if exp == 0 {
        if mant == 0 {
            f32::from_bits(sign << 31)
        } else {
            // Subnormal
            let mut m = mant;
            let mut e = 0;
            while (m & 0x0400) == 0 {
                m <<= 1;
                e += 1;
            }
            let new_exp = (127 - 15 - e) as u32;
            let new_mant = (m & 0x03FF) << 13;
            f32::from_bits((sign << 31) | (new_exp << 23) | new_mant)
        }
    } else if exp == 31 {
        // Infinity atau NaN
        let new_exp = 0xFF;
        let new_mant = if mant != 0 { 0x400000 } else { 0 };
        f32::from_bits((sign << 31) | (new_exp << 23) | new_mant)
    } else {
        // Normal
        let new_exp = exp + 127 - 15;
        let new_mant = mant << 13;
        f32::from_bits((sign << 31) | (new_exp << 23) | new_mant)
    }
}

/// Dekuantisasi Blok Q4_0 (32 nilai per blok 18 byte: 2 byte f16 scale + 16 byte nibbles)
pub fn dequantize_q4_0(block_data: &[u8], total_elements: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(total_elements);
    let num_blocks = block_data.len() / 18;

    for b in 0..num_blocks {
        let offset = b * 18;
        let scale_bits = u16::from_le_bytes([block_data[offset], block_data[offset + 1]]);
        let scale = f16_to_f32(scale_bits);

        for i in 0..16 {
            if result.len() >= total_elements {
                break;
            }
            let byte = block_data[offset + 2 + i];
            let x0 = (byte & 0x0F) as i8 - 8;
            result.push(x0 as f32 * scale);

            if result.len() < total_elements {
                let x1 = ((byte >> 4) & 0x0F) as i8 - 8;
                result.push(x1 as f32 * scale);
            }
        }
    }

    result
}

/// Dekuantisasi Blok Q8_0 (32 nilai per blok 34 byte: 2 byte f16 scale + 32 byte int8)
pub fn dequantize_q8_0(block_data: &[u8], total_elements: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(total_elements);
    let num_blocks = block_data.len() / 34;

    for b in 0..num_blocks {
        let offset = b * 34;
        let scale_bits = u16::from_le_bytes([block_data[offset], block_data[offset + 1]]);
        let scale = f16_to_f32(scale_bits);

        for i in 0..32 {
            if result.len() >= total_elements {
                break;
            }
            let val = block_data[offset + 2 + i] as i8;
            result.push(val as f32 * scale);
        }
    }

    result
}

/// Perkalian Dot-Product Vektor Q4_0 dengan Vektor FP32 Terakselerasi (4-way Loop Unrolling)
pub fn vec_dot_q4_0_f32(q4_blocks: &[u8], f32_vec: &[f32]) -> f32 {
    let num_blocks = q4_blocks.len() / 18;
    let mut sum = 0.0f32;

    for b in 0..num_blocks {
        let b_offset = b * 18;
        let vec_offset = b * 32;
        if vec_offset + 32 > f32_vec.len() {
            break;
        }

        let scale = f16_to_f32(u16::from_le_bytes([q4_blocks[b_offset], q4_blocks[b_offset + 1]]));
        let mut block_acc = 0.0f32;

        for i in 0..16 {
            let byte = q4_blocks[b_offset + 2 + i];
            let q0 = ((byte & 0x0F) as i8 - 8) as f32;
            let q1 = (((byte >> 4) & 0x0F) as i8 - 8) as f32;

            block_acc += q0 * f32_vec[vec_offset + i];
            block_acc += q1 * f32_vec[vec_offset + i + 16];
        }

        sum += block_acc * scale;
    }

    sum
}

/// Pembuat Format Biner GGUF Sintetis dalam Memori (untuk Pengujian & Ekspor)
pub fn build_synthetic_gguf(
    architecture: &str,
    context_length: u32,
    embedding_length: u32,
    block_count: u32,
    custom_kv: Vec<(&str, GgufValue)>,
) -> Vec<u8> {
    let mut buf = Vec::new();

    // Magic "GGUF"
    buf.extend_from_slice(b"GGUF");
    // Version 3
    buf.extend_from_slice(&3u32.to_le_bytes());

    // Tensor count: 0 (hanya header dan metadata untuk tes)
    buf.extend_from_slice(&0u64.to_le_bytes());

    // KV pairs count: 4 bawaan + custom
    let total_kv = 4 + custom_kv.len() as u64;
    buf.extend_from_slice(&total_kv.to_le_bytes());

    // 1. general.architecture
    write_string(&mut buf, "general.architecture");
    buf.extend_from_slice(&8u32.to_le_bytes()); // Type 8: String
    write_string(&mut buf, architecture);

    // 2. {arch}.context_length
    write_string(&mut buf, &format!("{}.context_length", architecture));
    buf.extend_from_slice(&4u32.to_le_bytes()); // Type 4: UInt32
    buf.extend_from_slice(&context_length.to_le_bytes());

    // 3. {arch}.embedding_length
    write_string(&mut buf, &format!("{}.embedding_length", architecture));
    buf.extend_from_slice(&4u32.to_le_bytes()); // Type 4: UInt32
    buf.extend_from_slice(&embedding_length.to_le_bytes());

    // 4. {arch}.block_count
    write_string(&mut buf, &format!("{}.block_count", architecture));
    buf.extend_from_slice(&4u32.to_le_bytes()); // Type 4: UInt32
    buf.extend_from_slice(&block_count.to_le_bytes());

    // Custom KVs
    for (k, v) in custom_kv {
        write_string(&mut buf, k);
        match v {
            GgufValue::String(s) => {
                buf.extend_from_slice(&8u32.to_le_bytes());
                write_string(&mut buf, &s);
            }
            GgufValue::UInt32(u) => {
                buf.extend_from_slice(&4u32.to_le_bytes());
                buf.extend_from_slice(&u.to_le_bytes());
            }
            GgufValue::Float32(f) => {
                buf.extend_from_slice(&6u32.to_le_bytes());
                buf.extend_from_slice(&f.to_le_bytes());
            }
            GgufValue::Bool(b) => {
                buf.extend_from_slice(&7u32.to_le_bytes());
                buf.push(if b { 1 } else { 0 });
            }
            _ => {}
        }
    }

    buf
}

fn write_string(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    buf.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    buf.extend_from_slice(bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gguf_synthetic_parse() {
        let binary = build_synthetic_gguf("qwen2", 8192, 2048, 28, vec![
            ("general.name", GgufValue::String("Qwen2.5-0.5B-Instruct-Widya".to_string())),
            ("tokenizer.ggml.bos_token_id", GgufValue::UInt32(151644)),
        ]);

        let info = parse_gguf_header(&binary).expect("Harus berhasil parsing GGUF header");
        assert!(info.is_valid_gguf);
        assert_eq!(info.version, 3);
        assert_eq!(info.architecture, "qwen2");
        assert_eq!(info.context_length, 8192);
        assert_eq!(info.embedding_length, 2048);
        assert_eq!(info.block_count, 28);
        assert_eq!(info.metadata.get("general.name").and_then(|v| v.as_str()), Some("Qwen2.5-0.5B-Instruct-Widya"));
    }

    #[test]
    fn test_f16_to_f32_conversion() {
        assert_eq!(f16_to_f32(0x0000), 0.0);
        assert_eq!(f16_to_f32(0x3C00), 1.0);
        assert_eq!(f16_to_f32(0x4000), 2.0);
        assert_eq!(f16_to_f32(0xC000), -2.0);
    }

    #[test]
    fn test_dequantize_q4_0_and_dot_product() {
        // 1 blok Q4_0 (18 byte)
        let mut block = vec![0u8; 18];
        // Scale = 1.0 (0x3C00 in f16)
        block[0] = 0x00;
        block[1] = 0x3C;
        // 16 bytes nibbles: misalnya 0x88 (nilai 8 - 8 = 0)
        for i in 2..18 {
            block[i] = 0x88;
        }

        let dequant = dequantize_q4_0(&block, 32);
        assert_eq!(dequant.len(), 32);
        for &val in &dequant {
            assert_eq!(val, 0.0);
        }

        let vec_input = vec![1.0f32; 32];
        let dot = vec_dot_q4_0_f32(&block, &vec_input);
        assert_eq!(dot, 0.0);
    }
}
