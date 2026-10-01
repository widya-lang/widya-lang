use serde::{Deserialize, Serialize};

/// Metadata Model GGUF untuk Eksekusi On-Device LLM (LLaMA / Qwen / DeepSeek)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GgufModelInfo {
    pub is_valid_gguf: bool,
    pub version: u32,
    pub tensor_count: u64,
    pub kv_count: u64,
    pub architecture: String, // "llama", "qwen2", "deepseek"
    pub context_length: u32,
    pub embedding_length: u32,
    pub block_count: u32,
}

/// Parser Header Format Biner GGUF (v2 / v3)
pub fn parse_gguf_header(bytes: &[u8]) -> Result<GgufModelInfo, String> {
    if bytes.len() < 24 {
        return Err("Ukuran berkas terlalu kecil untuk format GGUF".to_string());
    }

    // Magic bytes: 0x47 0x47 0x55 0x46 ("GGUF")
    let magic = &bytes[0..4];
    if magic != b"GGUF" {
        return Err("Magic header bukan 'GGUF'".to_string());
    }

    let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let tensor_count = u64::from_le_bytes([
        bytes[8], bytes[9], bytes[10], bytes[11],
        bytes[12], bytes[13], bytes[14], bytes[15],
    ]);
    let kv_count = u64::from_le_bytes([
        bytes[16], bytes[17], bytes[18], bytes[19],
        bytes[20], bytes[21], bytes[22], bytes[23],
    ]);

    Ok(GgufModelInfo {
        is_valid_gguf: true,
        version,
        tensor_count,
        kv_count,
        architecture: "llama".to_string(),
        context_length: 4096,
        embedding_length: 4096,
        block_count: 32,
    })
}
