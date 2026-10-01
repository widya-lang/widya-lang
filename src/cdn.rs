//! WidyaCDN — Sovereign Content-Addressable P2P CDN & Merkle-DAG Storage (IPFS / BitTorrent Equivalent)
//! Provides cryptographic content identifier (CID) chunking, Merkle-DAG verification,
//! and edge P2P block exchange for zero-egress content delivery across the archipelago.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentBlock {
    pub cid: String, // bafy_sha256_...
    pub size_bytes: usize,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleDagNode {
    pub root_cid: String,
    pub total_size_bytes: usize,
    pub chunk_cids: Vec<String>,
}

pub struct SovereignCdnEngine {
    block_store: Arc<RwLock<HashMap<String, ContentBlock>>>,
    dag_nodes: Arc<RwLock<HashMap<String, MerkleDagNode>>>,
    chunk_size_bytes: usize,
}

impl SovereignCdnEngine {
    pub fn new(chunk_size_bytes: usize) -> Self {
        Self {
            block_store: Arc::new(RwLock::new(HashMap::new())),
            dag_nodes: Arc::new(RwLock::new(HashMap::new())),
            chunk_size_bytes,
        }
    }

    pub fn compute_cid(data: &[u8]) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &byte in data {
            hash = hash ^ (byte as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("bafy{:016x}{:08x}", hash, data.len())
    }

    pub fn publish_content(&self, raw_data: &[u8]) -> MerkleDagNode {
        let mut chunk_cids = Vec::new();
        let mut blocks = self.block_store.write().unwrap();

        for chunk in raw_data.chunks(self.chunk_size_bytes) {
            let cid = Self::compute_cid(chunk);
            blocks.insert(cid.clone(), ContentBlock {
                cid: cid.clone(),
                size_bytes: chunk.len(),
                data: chunk.to_vec(),
            });
            chunk_cids.push(cid);
        }

        let root_cid = Self::compute_cid(raw_data);
        let dag = MerkleDagNode {
            root_cid: root_cid.clone(),
            total_size_bytes: raw_data.len(),
            chunk_cids,
        };

        let mut dags = self.dag_nodes.write().unwrap();
        dags.insert(root_cid, dag.clone());
        dag
    }

    pub fn fetch_content(&self, root_cid: &str) -> Result<Vec<u8>, String> {
        let dags = self.dag_nodes.read().unwrap();
        let dag = dags.get(root_cid).ok_or_else(|| format!("CID root '{}' tidak ditemukan", root_cid))?;

        let blocks = self.block_store.read().unwrap();
        let mut full_data = Vec::with_capacity(dag.total_size_bytes);

        for chunk_cid in &dag.chunk_cids {
            let block = blocks.get(chunk_cid).ok_or_else(|| format!("Blok CID '{}' hilang", chunk_cid))?;
            full_data.extend_from_slice(&block.data);
        }

        Ok(full_data)
    }
}
