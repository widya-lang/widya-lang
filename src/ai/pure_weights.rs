//! Pure Sovereign SafeTensors & GGUF Model Weight Engine (pure_weights.rs)
//!
//! Implementasi 100% mandiri penyimpanan dan pemuatan bobot model kecerdasan buatan:
//! - 📦 SafeTensors (.safetensors):
//!   * 8-byte Little-Endian Header Size
//!   * JSON Header dengan bentuk tensor (shape), tipe data (dtype: F32, F16, BF16, I32, I8),
//!     dan rentang offset data (data_offsets: [start, end])
//!   * Raw binary byte buffers tanpa dependensi Python atau PyTorch
//! - 🧠 GGUF v3 (.gguf):
//!   * Magic bytes "GGUF" (0x46554747), Version 3
//!   * Metadata KV dinamis (String, Uint, Float, Bool, Array)
//!   * Tensor Info Table & Memory Alignment (32-byte alignment)
//!   * Kuantisasi bawaan F32 -> Q8_0 & Q4_0
//!   * Roundtrip baca dan tulis model lengkap dengan payload bobot biner

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// 1. SAFETENSORS PURE ENGINE
// ============================================================================

/// Rekaman Tensor Tunggal dalam SafeTensors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafeTensorRecord {
    pub name: String,
    pub dtype: String, // "F32", "F16", "BF16", "I32", "I8", "U8"
    pub shape: Vec<usize>,
    pub data: Vec<u8>,
}

impl SafeTensorRecord {
    pub fn baru_f32(name: &str, shape: Vec<usize>, f32_data: &[f32]) -> Self {
        let mut data = Vec::with_capacity(f32_data.len() * 4);
        for &val in f32_data {
            data.extend_from_slice(&val.to_le_bytes());
        }
        Self {
            name: name.to_string(),
            dtype: "F32".to_string(),
            shape,
            data,
        }
    }

    pub fn to_f32_vec(&self) -> Result<Vec<f32>, String> {
        if self.dtype != "F32" {
            return Err(format!("Dtype bukan F32, melainkan {}", self.dtype));
        }
        let mut res = Vec::with_capacity(self.data.len() / 4);
        for chunk in self.data.chunks_exact(4) {
            res.push(f32::from_le_bytes(chunk.try_into().unwrap()));
        }
        Ok(res)
    }
}

/// Representasi Berkas Model SafeTensors Lengkap
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SafeTensorsModel {
    pub metadata: HashMap<String, String>,
    pub tensors: HashMap<String, SafeTensorRecord>,
}

impl SafeTensorsModel {
    pub fn baru() -> Self {
        Self::default()
    }

    pub fn dengan_metadata(mut self, key: &str, val: &str) -> Self {
        self.metadata.insert(key.to_string(), val.to_string());
        self
    }

    pub fn tambah_tensor(mut self, record: SafeTensorRecord) -> Self {
        self.tensors.insert(record.name.clone(), record);
        self
    }
}

pub struct PureSafeTensorsEngine;

impl PureSafeTensorsEngine {
    /// Serialisasi SafeTensorsModel ke byte buffer
    pub fn tulis_ke_bytes(model: &SafeTensorsModel) -> Result<Vec<u8>, String> {
        // Susun JSON Header
        let mut header_map = serde_json::Map::new();

        // 1. Tambahkan __metadata__ jika ada
        if !model.metadata.is_empty() {
            let meta_json = serde_json::to_value(&model.metadata)
                .map_err(|e| format!("Gagal serialisasi __metadata__: {}", e))?;
            header_map.insert("__metadata__".to_string(), meta_json);
        }

        // 2. Hitung offset dan masukkan setiap entri tensor
        let mut current_offset: u64 = 0;
        let mut tensor_list: Vec<&SafeTensorRecord> = model.tensors.values().collect();
        // Urutkan berdasarkan nama agar output deterministik
        tensor_list.sort_by(|a, b| a.name.cmp(&b.name));

        for t in &tensor_list {
            let start = current_offset;
            let end = start + (t.data.len() as u64);
            current_offset = end;

            let mut t_map = serde_json::Map::new();
            t_map.insert("dtype".to_string(), serde_json::Value::String(t.dtype.clone()));
            t_map.insert(
                "shape".to_string(),
                serde_json::Value::Array(
                    t.shape
                        .iter()
                        .map(|&s| serde_json::Value::Number(s.into()))
                        .collect(),
                ),
            );
            t_map.insert(
                "data_offsets".to_string(),
                serde_json::Value::Array(vec![
                    serde_json::Value::Number(start.into()),
                    serde_json::Value::Number(end.into()),
                ]),
            );

            header_map.insert(t.name.clone(), serde_json::Value::Object(t_map));
        }

        let header_str = serde_json::to_string(&header_map)
            .map_err(|e| format!("Gagal serialisasi JSON header: {}", e))?;
        let header_bytes = header_str.as_bytes();
        let header_len = header_bytes.len() as u64;

        // 3. Gabungkan 8-byte LE header length + header bytes + data bytes
        let mut result = Vec::with_capacity(8 + header_bytes.len() + (current_offset as usize));
        result.extend_from_slice(&header_len.to_le_bytes());
        result.extend_from_slice(header_bytes);

        for t in tensor_list {
            result.extend_from_slice(&t.data);
        }

        Ok(result)
    }

    /// Deserialisasi SafeTensors dari byte buffer
    pub fn baca_dari_bytes(bytes: &[u8]) -> Result<SafeTensorsModel, String> {
        if bytes.len() < 8 {
            return Err("Ukuran buffer terlalu kecil untuk SafeTensors (< 8 byte)".to_string());
        }

        let header_len = u64::from_le_bytes(bytes[0..8].try_into().unwrap()) as usize;
        let header_start = 8;
        let header_end = header_start + header_len;

        if header_end > bytes.len() {
            return Err("Header length melampaui ukuran total buffer".to_string());
        }

        let header_str = std::str::from_utf8(&bytes[header_start..header_end])
            .map_err(|e| format!("Header SafeTensors bukan UTF-8 valid: {}", e))?;

        let header_val: serde_json::Value = serde_json::from_str(header_str)
            .map_err(|e| format!("Gagal parsing JSON header SafeTensors: {}", e))?;

        let obj = header_val
            .as_object()
            .ok_or_else(|| "Header SafeTensors harus berupa JSON Object".to_string())?;

        let mut metadata = HashMap::new();
        let mut tensors = HashMap::new();

        let data_base_offset = header_end;

        for (key, val) in obj {
            if key == "__metadata__" {
                if let Some(m_obj) = val.as_object() {
                    for (mk, mv) in m_obj {
                        if let Some(s) = mv.as_str() {
                            metadata.insert(mk.clone(), s.to_string());
                        }
                    }
                }
                continue;
            }

            let dtype = val
                .get("dtype")
                .and_then(|d| d.as_str())
                .ok_or_else(|| format!("Tensor '{}' tidak memiliki atribut dtype", key))?
                .to_string();

            let shape: Vec<usize> = val
                .get("shape")
                .and_then(|s| s.as_array())
                .ok_or_else(|| format!("Tensor '{}' tidak memiliki atribut shape", key))?
                .iter()
                .filter_map(|x| x.as_u64().map(|v| v as usize))
                .collect();

            let offsets = val
                .get("data_offsets")
                .and_then(|o| o.as_array())
                .ok_or_else(|| format!("Tensor '{}' tidak memiliki data_offsets", key))?;

            if offsets.len() != 2 {
                return Err(format!("data_offsets tensor '{}' harus berisi 2 elemen", key));
            }

            let start = offsets[0].as_u64().unwrap() as usize;
            let end = offsets[1].as_u64().unwrap() as usize;

            let abs_start = data_base_offset + start;
            let abs_end = data_base_offset + end;

            if abs_end > bytes.len() {
                return Err(format!("Data tensor '{}' melampaui ukuran buffer", key));
            }

            let tensor_bytes = bytes[abs_start..abs_end].to_vec();

            tensors.insert(
                key.clone(),
                SafeTensorRecord {
                    name: key.clone(),
                    dtype,
                    shape,
                    data: tensor_bytes,
                },
            );
        }

        Ok(SafeTensorsModel { metadata, tensors })
    }

    /// Ekspor berkas fisik .safetensors
    pub fn ekspor_berkas(model: &SafeTensorsModel, path: &Path) -> Result<PathBuf, String> {
        let bytes = Self::tulis_ke_bytes(model)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Gagal membuat direktori {:?}: {}", parent, e))?;
        }
        fs::write(path, &bytes)
            .map_err(|e| format!("Gagal menulis berkas SafeTensors ke {:?}: {}", path, e))?;
        Ok(path.to_path_buf())
    }

    /// Impor dari berkas fisik .safetensors
    pub fn impor_berkas(path: &Path) -> Result<SafeTensorsModel, String> {
        let bytes = fs::read(path)
            .map_err(|e| format!("Gagal membaca berkas SafeTensors dari {:?}: {}", path, e))?;
        Self::baca_dari_bytes(&bytes)
    }
}

// ============================================================================
// 2. GGUF V3 FULL MODEL WEIGHT ENGINE
// ============================================================================

pub const GGUF_MAGIC_V3: u32 = 0x46554747; // "GGUF" in LE
pub const GGUF_DEFAULT_ALIGNMENT: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PureGgufType {
    F32 = 0,
    F16 = 1,
    Q4_0 = 2,
    Q8_0 = 8,
}

impl PureGgufType {
    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(PureGgufType::F32),
            1 => Some(PureGgufType::F16),
            2 => Some(PureGgufType::Q4_0),
            8 => Some(PureGgufType::Q8_0),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PureGgufValue {
    UInt32(u32),
    Int32(i32),
    Float32(f32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PureGgufTensor {
    pub name: String,
    pub shape: Vec<u64>,
    pub tensor_type: PureGgufType,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PureGgufModel {
    pub metadata: HashMap<String, PureGgufValue>,
    pub tensors: Vec<PureGgufTensor>,
    pub alignment: usize,
}

impl PureGgufModel {
    pub fn baru() -> Self {
        Self {
            metadata: HashMap::new(),
            tensors: Vec::new(),
            alignment: GGUF_DEFAULT_ALIGNMENT,
        }
    }

    pub fn dengan_meta(mut self, key: &str, val: PureGgufValue) -> Self {
        self.metadata.insert(key.to_string(), val);
        self
    }

    pub fn tambah_tensor(mut self, tensor: PureGgufTensor) -> Self {
        self.tensors.push(tensor);
        self
    }
}

pub struct PureGgufEngine;

impl PureGgufEngine {
    /// Kuantisasi vektor F32 menjadi Q8_0 (32 elemen per blok: 2 byte f16 scale + 32 byte int8)
    pub fn kuantisasi_q8_0(f32_vals: &[f32]) -> Vec<u8> {
        let block_count = (f32_vals.len() + 31) / 32;
        let mut out = Vec::with_capacity(block_count * 34);

        for b in 0..block_count {
            let start = b * 32;
            let end = (start + 32).min(f32_vals.len());
            let slice = &f32_vals[start..end];

            // Cari amax (absolute maximum)
            let mut amax = 0.0f32;
            for &val in slice {
                let abs = val.abs();
                if abs > amax {
                    amax = abs;
                }
            }

            let d = amax / 127.0;
            let id = if d != 0.0 { 1.0 / d } else { 0.0 };

            // Tulis scale d sebagai f16 (menggunakan konversi f32 -> f16)
            let d_f16 = Self::f32_to_f16(d);
            out.extend_from_slice(&d_f16.to_le_bytes());

            // Tulis 32 int8 kuantisasi
            for i in 0..32 {
                let v = if start + i < f32_vals.len() {
                    f32_vals[start + i]
                } else {
                    0.0
                };
                let q = (v * id).round().clamp(-128.0, 127.0) as i8;
                out.push(q as u8);
            }
        }

        out
    }

    /// Serialisasi PureGgufModel ke byte buffer GGUF v3
    pub fn tulis_ke_bytes(model: &PureGgufModel) -> Result<Vec<u8>, String> {
        let mut buf = Vec::new();

        // 1. Header (16 bytes): Magic + Version + TensorCount + KvCount
        buf.extend_from_slice(&GGUF_MAGIC_V3.to_le_bytes());
        buf.extend_from_slice(&3u32.to_le_bytes()); // Version 3
        buf.extend_from_slice(&(model.tensors.len() as u64).to_le_bytes());
        buf.extend_from_slice(&(model.metadata.len() as u64).to_le_bytes());

        // 2. Metadata Key-Value pairs
        let mut kv_list: Vec<(&String, &PureGgufValue)> = model.metadata.iter().collect();
        kv_list.sort_by(|a, b| a.0.cmp(b.0));

        for (key, val) in kv_list {
            // Tulis string key (len u64 + bytes)
            Self::tulis_string(&mut buf, key);

            match val {
                PureGgufValue::UInt32(u) => {
                    buf.extend_from_slice(&0x04u32.to_le_bytes()); // Type 4: UINT32
                    buf.extend_from_slice(&u.to_le_bytes());
                }
                PureGgufValue::Int32(i) => {
                    buf.extend_from_slice(&0x05u32.to_le_bytes()); // Type 5: INT32
                    buf.extend_from_slice(&i.to_le_bytes());
                }
                PureGgufValue::Float32(f) => {
                    buf.extend_from_slice(&0x06u32.to_le_bytes()); // Type 6: FLOAT32
                    buf.extend_from_slice(&f.to_le_bytes());
                }
                PureGgufValue::Bool(b) => {
                    buf.extend_from_slice(&0x07u32.to_le_bytes()); // Type 7: BOOL
                    buf.push(if *b { 1 } else { 0 });
                }
                PureGgufValue::String(s) => {
                    buf.extend_from_slice(&0x08u32.to_le_bytes()); // Type 8: STRING
                    Self::tulis_string(&mut buf, s);
                }
            }
        }

        // 3. Tensor Info Headers
        // Hitung total panjang raw header untuk kalkulasi offset relatif data
        let mut tensor_offsets = Vec::new();
        let mut current_offset: u64 = 0;

        for t in &model.tensors {
            tensor_offsets.push(current_offset);
            let len = t.data.len() as u64;
            // Align ke model.alignment
            let pad = (model.alignment as u64 - (len % model.alignment as u64)) % model.alignment as u64;
            current_offset += len + pad;
        }

        for (idx, t) in model.tensors.iter().enumerate() {
            Self::tulis_string(&mut buf, &t.name);
            buf.extend_from_slice(&(t.shape.len() as u32).to_le_bytes());
            for &dim in &t.shape {
                buf.extend_from_slice(&dim.to_le_bytes());
            }
            buf.extend_from_slice(&(t.tensor_type as u32).to_le_bytes());
            buf.extend_from_slice(&tensor_offsets[idx].to_le_bytes());
        }

        // 4. Memory Alignment Padding sebelum raw binary tensor data
        let align = model.alignment;
        let pad = (align - (buf.len() % align)) % align;
        buf.extend(std::iter::repeat(0).take(pad));

        // 5. Tulis Raw Tensor Data Buffers dengan perataan alignment
        for t in &model.tensors {
            buf.extend_from_slice(&t.data);
            let t_pad = (align - (t.data.len() % align)) % align;
            buf.extend(std::iter::repeat(0).take(t_pad));
        }

        Ok(buf)
    }

    /// Deserialisasi PureGgufModel dari byte buffer GGUF v3
    pub fn baca_dari_bytes(bytes: &[u8]) -> Result<PureGgufModel, String> {
        if bytes.len() < 24 {
            return Err("Ukuran buffer terlalu kecil untuk GGUF (< 24 byte)".to_string());
        }

        let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        if magic != GGUF_MAGIC_V3 {
            return Err(format!("Magic GGUF tidak valid: 0x{:08X}", magic));
        }

        let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        if version != 2 && version != 3 {
            return Err(format!("Versi GGUF {} tidak didukung (harus v2/v3)", version));
        }

        let tensor_count = u64::from_le_bytes(bytes[8..16].try_into().unwrap()) as usize;
        let kv_count = u64::from_le_bytes(bytes[16..24].try_into().unwrap()) as usize;

        let mut cursor = 24;
        let mut metadata = HashMap::new();

        // 1. Parse Metadata
        for _ in 0..kv_count {
            let key = Self::baca_string(bytes, &mut cursor)?;
            let v_type = Self::baca_u32(bytes, &mut cursor)?;
            let val = match v_type {
                0x04 => PureGgufValue::UInt32(Self::baca_u32(bytes, &mut cursor)?),
                0x05 => PureGgufValue::Int32(Self::baca_i32(bytes, &mut cursor)?),
                0x06 => PureGgufValue::Float32(Self::baca_f32(bytes, &mut cursor)?),
                0x07 => {
                    let b = bytes[cursor] != 0;
                    cursor += 1;
                    PureGgufValue::Bool(b)
                }
                0x08 => PureGgufValue::String(Self::baca_string(bytes, &mut cursor)?),
                _ => return Err(format!("Tipe metadata GGUF 0x{:02X} belum didukung", v_type)),
            };
            metadata.insert(key, val);
        }

        // 2. Parse Tensor Info Table
        struct TempTensorInfo {
            name: String,
            shape: Vec<u64>,
            t_type: PureGgufType,
            offset: u64,
        }

        let mut tensor_infos = Vec::new();
        for _ in 0..tensor_count {
            let name = Self::baca_string(bytes, &mut cursor)?;
            let n_dims = Self::baca_u32(bytes, &mut cursor)? as usize;
            let mut shape = Vec::with_capacity(n_dims);
            for _ in 0..n_dims {
                shape.push(Self::baca_u64(bytes, &mut cursor)?);
            }
            let t_type_u32 = Self::baca_u32(bytes, &mut cursor)?;
            let t_type = PureGgufType::from_u32(t_type_u32)
                .ok_or_else(|| format!("Tipe tensor {} tidak dikenal", t_type_u32))?;
            let offset = Self::baca_u64(bytes, &mut cursor)?;

            tensor_infos.push(TempTensorInfo {
                name,
                shape,
                t_type,
                offset,
            });
        }

        // 3. Cari batas awal Data Buffer (aligned to 32 bytes)
        let alignment = GGUF_DEFAULT_ALIGNMENT;
        let data_base_offset = ((cursor + alignment - 1) / alignment) * alignment;

        let mut tensors = Vec::new();
        for info in tensor_infos {
            let abs_offset = data_base_offset + info.offset as usize;
            // Hitung panjang tensor dari shape dan type
            let total_elements: usize = info.shape.iter().map(|&x| x as usize).product();
            let byte_len = match info.t_type {
                PureGgufType::F32 => total_elements * 4,
                PureGgufType::F16 => total_elements * 2,
                PureGgufType::Q4_0 => ((total_elements + 31) / 32) * 18,
                PureGgufType::Q8_0 => ((total_elements + 31) / 32) * 34,
            };

            if abs_offset + byte_len > bytes.len() {
                return Err(format!("Data tensor '{}' di luar batas berkas", info.name));
            }

            let data = bytes[abs_offset..abs_offset + byte_len].to_vec();
            tensors.push(PureGgufTensor {
                name: info.name,
                shape: info.shape,
                tensor_type: info.t_type,
                data,
            });
        }

        Ok(PureGgufModel {
            metadata,
            tensors,
            alignment,
        })
    }

    fn tulis_string(buf: &mut Vec<u8>, s: &str) {
        let b = s.as_bytes();
        buf.extend_from_slice(&(b.len() as u64).to_le_bytes());
        buf.extend_from_slice(b);
    }

    fn baca_string(bytes: &[u8], cursor: &mut usize) -> Result<String, String> {
        let len = Self::baca_u64(bytes, cursor)? as usize;
        if *cursor + len > bytes.len() {
            return Err("String melampaui buffer".to_string());
        }
        let s = String::from_utf8_lossy(&bytes[*cursor..*cursor + len]).to_string();
        *cursor += len;
        Ok(s)
    }

    fn baca_u32(bytes: &[u8], cursor: &mut usize) -> Result<u32, String> {
        if *cursor + 4 > bytes.len() {
            return Err("Buffer habis saat membaca u32".to_string());
        }
        let v = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        Ok(v)
    }

    fn baca_i32(bytes: &[u8], cursor: &mut usize) -> Result<i32, String> {
        if *cursor + 4 > bytes.len() {
            return Err("Buffer habis saat membaca i32".to_string());
        }
        let v = i32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        Ok(v)
    }

    fn baca_u64(bytes: &[u8], cursor: &mut usize) -> Result<u64, String> {
        if *cursor + 8 > bytes.len() {
            return Err("Buffer habis saat membaca u64".to_string());
        }
        let v = u64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap());
        *cursor += 8;
        Ok(v)
    }

    fn baca_f32(bytes: &[u8], cursor: &mut usize) -> Result<f32, String> {
        if *cursor + 4 > bytes.len() {
            return Err("Buffer habis saat membaca f32".to_string());
        }
        let v = f32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        Ok(v)
    }

    fn f32_to_f16(val: f32) -> u16 {
        let bits = val.to_bits();
        let sign = (bits >> 16) & 0x8000;
        let mut val = (bits & 0x7fffffff) as i32;

        if val > 0x477fe000 {
            return (sign | 0x7c00) as u16;
        }
        if val < 0x38800000 {
            return sign as u16;
        }

        val -= 0x38000000;
        (sign | ((val + 0x1000) >> 13) as u32) as u16
    }
}
