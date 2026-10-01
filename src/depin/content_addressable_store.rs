//! Decentralized Model Weight Store & Content-Addressable Storage (content_addressable_store.rs)
//! Menyimpan dan mendistribusikan bobot model AI (GGUF / Safetensors) secara terdesentralisasi
//! menggunakan hash konten (CID), pemecahan chunk paralel, dan verifikasi integritas data.

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;

/// Metadata Bongkahan Bobot Model (Model Chunk)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelChunkMeta {
    pub chunk_index: usize,
    pub chunk_cid: String, // Content Identifier (SHA3-256)
    pub byte_size: usize,
    pub provider_nodes: Vec<String>,
}

/// Manifest Model AI Terdesentralisasi
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributedModelManifest {
    pub model_id: String,
    pub root_cid: String,
    pub total_size_bytes: usize,
    pub format: String, // "GGUF", "SAFETENSORS"
    pub chunks: Vec<ModelChunkMeta>,
}

/// Engine Content-Addressable Storage untuk Bobot AI
pub struct ContentAddressableStore {
    models: HashMap<String, DistributedModelManifest>, // root_cid -> manifest
}

impl ContentAddressableStore {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    /// Hitung SHA3-256 CID untuk data biner
    pub fn calculate_cid(data: &[u8]) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        format!("cid_{:x}", hasher.finalize())
    }

    /// Pecah berkas model besar menjadi chunks dan buat manifest terdistribusi
    pub fn publish_model(
        &mut self,
        model_name: &str,
        raw_bytes: &[u8],
        chunk_size: usize,
        initial_provider_node: &str,
    ) -> DistributedModelManifest {
        let total_size = raw_bytes.len();
        let mut chunks = Vec::new();

        let num_chunks = (total_size + chunk_size - 1) / chunk_size;
        for i in 0..num_chunks {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(total_size);
            let chunk_data = &raw_bytes[start..end];
            let cid = Self::calculate_cid(chunk_data);

            chunks.push(ModelChunkMeta {
                chunk_index: i,
                chunk_cid: cid,
                byte_size: end - start,
                provider_nodes: vec![initial_provider_node.to_string()],
            });
        }

        let root_cid = Self::calculate_cid(raw_bytes);
        let manifest = DistributedModelManifest {
            model_id: model_name.to_string(),
            root_cid: root_cid.clone(),
            total_size_bytes: total_size,
            format: "SAFETENSORS".to_string(),
            chunks,
        };

        self.models.insert(root_cid, manifest.clone());
        manifest
    }

    /// Daftarkan node baru yang berhasil men-cache sebuah chunk
    pub fn register_chunk_provider(&mut self, root_cid: &str, chunk_index: usize, provider_node: &str) -> Result<bool, String> {
        let manifest = self.models.get_mut(root_cid)
            .ok_or_else(|| format!("Model dengan CID '{}' tidak ditemukan", root_cid))?;

        let chunk = manifest.chunks.iter_mut().find(|c| c.chunk_index == chunk_index)
            .ok_or_else(|| format!("Chunk index {} tidak ditemukan", chunk_index))?;

        if !chunk.provider_nodes.iter().any(|p| p == provider_node) {
            chunk.provider_nodes.push(provider_node.to_string());
        }

        Ok(true)
    }

    pub fn get_manifest(&self, root_cid: &str) -> Option<&DistributedModelManifest> {
        self.models.get(root_cid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_addressable_model_distribution() {
        let mut cas = ContentAddressableStore::new();

        // Simulasi bobot model 10 KB dipecah per 3 KB
        let model_data = vec![0x42u8; 10_000];
        let manifest = cas.publish_model("DeepSeek-7B-Quant", &model_data, 3000, "node_primary_server");

        assert_eq!(manifest.chunks.len(), 4); // 3000 + 3000 + 3000 + 1000 = 10000
        assert_eq!(manifest.total_size_bytes, 10_000);
        assert!(manifest.root_cid.starts_with("cid_"));

        // Node di Bandung men-cache chunk index 0
        assert!(cas.register_chunk_provider(&manifest.root_cid, 0, "node_bandung_worker").is_ok());

        let updated = cas.get_manifest(&manifest.root_cid).unwrap();
        assert_eq!(updated.chunks[0].provider_nodes.len(), 2);
    }
}
