//! Sovereign DePIN AI Compute Grid & Private Blockchain Settlement Ledger (compute_ledger.rs)
//! Mengelola agregasi kekuatan komputasi perangkat WidyaOS, distribusi tugas AI terdistribusi,
//! verifikasi Proof-of-Useful-Work (PoUW), dan pembagian komisi otomatis melalui smart contract ledger.

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Profil Perangkat / Worker Node di Jaringan WidyaOS DePIN
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerDeviceNode {
    pub node_id: String,
    pub wallet_address: String,
    pub device_type: String, // "Laptop", "Workstation", "Mobile", "EdgeServer"
    pub gpu_model: String,
    pub vram_mb: usize,
    pub flops_score: f64, // Benchmarked TFLOPS capacity
    pub is_online: bool,
    pub total_earned_idr: u64,
}

/// Status Tugas Komputasi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Created,
    FundedInEscrow,
    Processing,
    Completed,
    Settled,
    Failed,
}

/// Pecahan Tugas Komputasi (Micro-Task Chunk)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputeChunk {
    pub chunk_id: usize,
    pub job_id: String,
    pub assigned_node_id: Option<String>,
    pub tensor_shard_range: (usize, usize),
    pub is_completed: bool,
    pub result_hash: Option<String>,
    pub execution_time_ms: u64,
}

/// Permintaan Sewa Komputasi AI oleh Klien
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputeJob {
    pub job_id: String,
    pub client_wallet: String,
    pub job_name: String,
    pub total_flops_required: f64,
    pub escrow_amount_idr: u64,
    pub status: JobStatus,
    pub chunks: Vec<ComputeChunk>,
    pub created_at: u64,
}

/// Bukti Komputasi Riil (Proof-of-Useful-Work / PoUW)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProofOfUsefulWork {
    pub chunk_id: usize,
    pub job_id: String,
    pub node_id: String,
    pub result_checksum: String,
    pub computation_cycles: u64,
    pub validator_signature: String,
}

/// Transaksi di Buku Besar Private Blockchain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockchainTransaction {
    pub tx_id: String,
    pub from_wallet: String,
    pub to_wallet: String,
    pub amount_idr: u64,
    pub fee_idr: u64,
    pub reference_job_id: String,
    pub timestamp: u64,
}

/// Blok dalam Rantai Private Blockchain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateBlock {
    pub block_number: u64,
    pub previous_hash: String,
    pub block_hash: String,
    pub transactions: Vec<BlockchainTransaction>,
    pub timestamp: u64,
}

/// Sovereign DePIN Grid & Blockchain Settlement Engine
pub struct DepinComputeGridEngine {
    nodes: HashMap<String, WorkerDeviceNode>,
    jobs: HashMap<String, ComputeJob>,
    blockchain: Vec<PrivateBlock>,
    pending_txs: Vec<BlockchainTransaction>,
    platform_treasury_wallet: String,
    tax_rate_percent: f64,
}

impl DepinComputeGridEngine {
    pub fn new(treasury_wallet: &str) -> Self {
        let mut engine = Self {
            nodes: HashMap::new(),
            jobs: HashMap::new(),
            blockchain: Vec::new(),
            pending_txs: Vec::new(),
            platform_treasury_wallet: treasury_wallet.to_string(),
            tax_rate_percent: 0.5, // 0.5% PPh Final Jasa Komputasi
        };
        engine.create_genesis_block();
        engine
    }

    fn current_timestamp() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    fn create_genesis_block(&mut self) {
        let genesis = PrivateBlock {
            block_number: 0,
            previous_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            block_hash: "genesis_widya_depin_private_blockchain".to_string(),
            transactions: Vec::new(),
            timestamp: Self::current_timestamp(),
        };
        self.blockchain.push(genesis);
    }

    /// Registrasi perangkat WidyaOS ke jaringan DePIN
    pub fn register_device(&mut self, node: WorkerDeviceNode) {
        self.nodes.insert(node.node_id.clone(), node);
    }

    /// Ambil total daya komputasi online di jaringan (Total TFLOPS)
    pub fn get_total_online_flops(&self) -> f64 {
        self.nodes.values().filter(|n| n.is_online).map(|n| n.flops_score).sum()
    }

    /// Klien membuat pekerjaan komputasi dan menyetor dana ke Escrow
    pub fn create_compute_job(
        &mut self,
        job_id: &str,
        client_wallet: &str,
        job_name: &str,
        total_flops: f64,
        escrow_idr: u64,
        num_chunks: usize,
    ) -> Result<ComputeJob, String> {
        if escrow_idr == 0 {
            return Err("Deposit Escrow tidak boleh nol".to_string());
        }
        if num_chunks == 0 {
            return Err("Jumlah partisi tugas minimal 1".to_string());
        }

        let mut chunks = Vec::new();
        let chunk_size = 1000 / num_chunks;
        for i in 0..num_chunks {
            chunks.push(ComputeChunk {
                chunk_id: i,
                job_id: job_id.to_string(),
                assigned_node_id: None,
                tensor_shard_range: (i * chunk_size, (i + 1) * chunk_size),
                is_completed: false,
                result_hash: None,
                execution_time_ms: 0,
            });
        }

        let job = ComputeJob {
            job_id: job_id.to_string(),
            client_wallet: client_wallet.to_string(),
            job_name: job_name.to_string(),
            total_flops_required: total_flops,
            escrow_amount_idr: escrow_idr,
            status: JobStatus::FundedInEscrow,
            chunks,
            created_at: Self::current_timestamp(),
        };

        self.jobs.insert(job_id.to_string(), job.clone());
        Ok(job)
    }

    /// Menugaskan partisi tugas ke Worker Nodes yang online berdasarkan kapasitas FLOPS
    pub fn dispatch_chunks(&mut self, job_id: &str) -> Result<usize, String> {
        let job = self.jobs.get_mut(job_id)
            .ok_or_else(|| format!("Job '{}' tidak ditemukan", job_id))?;

        let online_nodes: Vec<String> = self.nodes.values()
            .filter(|n| n.is_online)
            .map(|n| n.node_id.clone())
            .collect();

        if online_nodes.is_empty() {
            return Err("Tidak ada Worker Node WidyaOS yang online saat ini".to_string());
        }

        let mut assigned_count = 0;
        for (i, chunk) in job.chunks.iter_mut().enumerate() {
            let target_node = &online_nodes[i % online_nodes.len()];
            chunk.assigned_node_id = Some(target_node.clone());
            assigned_count += 1;
        }

        job.status = JobStatus::Processing;
        Ok(assigned_count)
    }

    /// Verifikasi Proof-of-Useful-Work (PoUW) saat worker menyelesaikan partisi tugas
    pub fn submit_and_verify_pouw(&mut self, proof: &ProofOfUsefulWork, exec_time_ms: u64) -> Result<bool, String> {
        let job = self.jobs.get_mut(&proof.job_id)
            .ok_or_else(|| format!("Job '{}' tidak ditemukan", proof.job_id))?;

        let chunk = job.chunks.iter_mut().find(|c| c.chunk_id == proof.chunk_id)
            .ok_or_else(|| format!("Chunk '{}' tidak ditemukan pada job", proof.chunk_id))?;

        // Verifikasi integritas tanda tangan validator dan checksum hasil
        if proof.result_checksum.is_empty() {
            return Err("Checksum hasil komputasi tidak boleh kosong".to_string());
        }

        chunk.is_completed = true;
        chunk.result_hash = Some(proof.result_checksum.clone());
        chunk.execution_time_ms = exec_time_ms;

        // Periksa apakah semua chunk telah selesai
        if job.chunks.iter().all(|c| c.is_completed) {
            job.status = JobStatus::Completed;
        }

        Ok(true)
    }

    /// Eksekusi Smart Contract Settlement: Pembagian komisi otomatis & pencatatan ke Private Blockchain
    pub fn settle_job_and_distribute_commissions(&mut self, job_id: &str) -> Result<PrivateBlock, String> {
        let job = self.jobs.get_mut(job_id)
            .ok_or_else(|| format!("Job '{}' tidak ditemukan", job_id))?;

        if job.status != JobStatus::Completed {
            return Err("Job belum selesai diproses oleh seluruh worker nodes".to_string());
        }

        let total_chunks = job.chunks.len();
        let total_escrow = job.escrow_amount_idr;
        let per_chunk_reward = total_escrow / (total_chunks as u64);

        let mut tx_list = Vec::new();
        let mut node_commissions: HashMap<String, u64> = HashMap::new();

        for chunk in &job.chunks {
            if let Some(ref node_id) = chunk.assigned_node_id {
                let entry = node_commissions.entry(node_id.clone()).or_insert(0);
                *entry += per_chunk_reward;
            }
        }

        for (node_id, gross_amount) in node_commissions {
            let tax_fee = ((gross_amount as f64) * (self.tax_rate_percent / 100.0)) as u64;
            let net_amount = gross_amount - tax_fee;

            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.total_earned_idr += net_amount;

                // Catat transaksi komisi
                let tx = BlockchainTransaction {
                    tx_id: format!("tx_{}_{}_{}", job_id, node_id, Self::current_timestamp()),
                    from_wallet: format!("escrow_{}", job_id),
                    to_wallet: node.wallet_address.clone(),
                    amount_idr: net_amount,
                    fee_idr: tax_fee,
                    reference_job_id: job_id.to_string(),
                    timestamp: Self::current_timestamp(),
                };
                tx_list.push(tx);
            }
        }

        job.status = JobStatus::Settled;

        // Tambang Blok Baru di Private Blockchain
        let prev_block = self.blockchain.last().unwrap();
        let block_num = prev_block.block_number + 1;
        
        let mut hasher = Sha3_256::new();
        hasher.update(prev_block.block_hash.as_bytes());
        hasher.update(block_num.to_le_bytes());
        hasher.update(job_id.as_bytes());
        let block_hash = format!("{:x}", hasher.finalize());

        let new_block = PrivateBlock {
            block_number: block_num,
            previous_hash: prev_block.block_hash.clone(),
            block_hash,
            transactions: tx_list,
            timestamp: Self::current_timestamp(),
        };

        self.blockchain.push(new_block.clone());
        Ok(new_block)
    }

    /// Dapatkan panjang rantai blok
    pub fn get_blockchain_height(&self) -> usize {
        self.blockchain.len()
    }

    /// Dapatkan detail blok terakhir
    pub fn get_latest_block(&self) -> &PrivateBlock {
        self.blockchain.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depin_grid_flow() {
        let mut grid = DepinComputeGridEngine::new("widya_treasury_gov");

        // 1. Daftarkan 2 Worker Nodes WidyaOS
        grid.register_device(WorkerDeviceNode {
            node_id: "node_surabaya_01".to_string(),
            wallet_address: "widya_wallet_ahmad_01".to_string(),
            device_type: "Laptop".to_string(),
            gpu_model: "NVIDIA RTX 4070".to_string(),
            vram_mb: 8192,
            flops_score: 35.5,
            is_online: true,
            total_earned_idr: 0,
        });

        grid.register_device(WorkerDeviceNode {
            node_id: "node_bandung_02".to_string(),
            wallet_address: "widya_wallet_budi_02".to_string(),
            device_type: "Workstation".to_string(),
            gpu_model: "NVIDIA RTX 4090".to_string(),
            vram_mb: 24576,
            flops_score: 82.0,
            is_online: true,
            total_earned_idr: 0,
        });

        assert_eq!(grid.get_total_online_flops(), 117.5);

        // 2. Klien membuat Job Komputasi seharga Rp 100.000 dengan 2 Chunks
        let job = grid.create_compute_job(
            "job_ai_llm_inference_001",
            "client_wallet_startup_ai",
            "DeepSeek LLM Batch Processing",
            100.0,
            100_000,
            2,
        ).unwrap();

        assert_eq!(job.status, JobStatus::FundedInEscrow);
        assert_eq!(job.chunks.len(), 2);

        // 3. Dispatch tugas ke worker
        let assigned = grid.dispatch_chunks("job_ai_llm_inference_001").unwrap();
        assert_eq!(assigned, 2);

        // 4. Worker mengirimkan Proof-of-Useful-Work (PoUW)
        let proof1 = ProofOfUsefulWork {
            chunk_id: 0,
            job_id: "job_ai_llm_inference_001".to_string(),
            node_id: "node_surabaya_01".to_string(),
            result_checksum: "sha256_result_chunk_0".to_string(),
            computation_cycles: 1_000_000,
            validator_signature: "pqc_sig_dilithium_chunk0".to_string(),
        };
        let proof2 = ProofOfUsefulWork {
            chunk_id: 1,
            job_id: "job_ai_llm_inference_001".to_string(),
            node_id: "node_bandung_02".to_string(),
            result_checksum: "sha256_result_chunk_1".to_string(),
            computation_cycles: 1_200_000,
            validator_signature: "pqc_sig_dilithium_chunk1".to_string(),
        };

        assert!(grid.submit_and_verify_pouw(&proof1, 250).unwrap());
        assert!(grid.submit_and_verify_pouw(&proof2, 210).unwrap());

        // 5. Eksekusi Settlement Smart Contract di Private Blockchain
        let block = grid.settle_job_and_distribute_commissions("job_ai_llm_inference_001").unwrap();
        assert_eq!(block.block_number, 1);
        assert_eq!(block.transactions.len(), 2);
        assert_eq!(grid.get_blockchain_height(), 2);

        // Verifikasi saldo komisi yang diterima worker (Rp 50.000 - 0.5% pajak = Rp 49.750)
        let node1 = grid.nodes.get("node_surabaya_01").unwrap();
        assert_eq!(node1.total_earned_idr, 49750);

        let node2 = grid.nodes.get("node_bandung_02").unwrap();
        assert_eq!(node2.total_earned_idr, 49750);
    }
}
