//! Comprehensive Integration Test for Sovereign DePIN Compute Grid & Blockchain Settlement
//! Menguji agregasi multi-device, dispatch tugas, verifikasi PoUW, dan pembagian komisi terdesentralisasi.

use widya::depin::{
    DepinComputeGridEngine,
    WorkerDeviceNode,
    JobStatus,
    ProofOfUsefulWork,
};

#[test]
fn test_sovereign_depin_multi_device_lifecycle() {
    let mut grid = DepinComputeGridEngine::new("widya_kas_negara_depin");

    // 1. Registrasi 4 Node dari berbagai jenis perangkat WidyaOS
    grid.register_device(WorkerDeviceNode {
        node_id: "node_jakarta_datacenter".to_string(),
        wallet_address: "widya_wallet_jkt_dc".to_string(),
        device_type: "EdgeServer".to_string(),
        gpu_model: "NVIDIA H100 SXM5".to_string(),
        vram_mb: 81920,
        flops_score: 750.0,
        is_online: true,
        total_earned_idr: 0,
    });

    grid.register_device(WorkerDeviceNode {
        node_id: "node_bandung_workstation".to_string(),
        wallet_address: "widya_wallet_bdg_ws".to_string(),
        device_type: "Workstation".to_string(),
        gpu_model: "NVIDIA RTX 4090".to_string(),
        vram_mb: 24576,
        flops_score: 82.0,
        is_online: true,
        total_earned_idr: 0,
    });

    grid.register_device(WorkerDeviceNode {
        node_id: "node_surabaya_laptop".to_string(),
        wallet_address: "widya_wallet_sby_lp".to_string(),
        device_type: "Laptop".to_string(),
        gpu_model: "NVIDIA RTX 4060".to_string(),
        vram_mb: 8192,
        flops_score: 25.0,
        is_online: true,
        total_earned_idr: 0,
    });

    grid.register_device(WorkerDeviceNode {
        node_id: "node_yogyakarta_phone".to_string(),
        wallet_address: "widya_wallet_jogja_hp".to_string(),
        device_type: "Mobile".to_string(),
        gpu_model: "Widya NPU Gen3".to_string(),
        vram_mb: 4096,
        flops_score: 12.0,
        is_online: true,
        total_earned_idr: 0,
    });

    // Total online compute capacity: 750 + 82 + 25 + 12 = 869 TFLOPS
    assert_eq!(grid.get_total_online_flops(), 869.0);

    // 2. Klien Startup AI memesan pekerjaan Distributed LLM Fine-Tuning (Deposit Rp 400.000, 4 Chunks)
    let job = grid.create_compute_job(
        "job_finetune_indobert_001",
        "client_wallet_gov_research",
        "IndoBERT Large Fine-Tuning",
        500.0,
        400_000,
        4,
    ).unwrap();

    assert_eq!(job.status, JobStatus::FundedInEscrow);
    assert_eq!(job.chunks.len(), 4);

    // 3. Dispatch tugas ke seluruh node
    let dispatched = grid.dispatch_chunks("job_finetune_indobert_001").unwrap();
    assert_eq!(dispatched, 4);

    // 4. Semua 4 node menyelesaikan micro-task dan mengirim Proof-of-Useful-Work (PoUW)
    let nodes = [
        ("node_jakarta_datacenter", 0),
        ("node_bandung_workstation", 1),
        ("node_surabaya_laptop", 2),
        ("node_yogyakarta_phone", 3),
    ];

    for &(node_id, chunk_id) in &nodes {
        let proof = ProofOfUsefulWork {
            chunk_id,
            job_id: "job_finetune_indobert_001".to_string(),
            node_id: node_id.to_string(),
            result_checksum: format!("sha256_valid_gradient_shard_{}", chunk_id),
            computation_cycles: 5_000_000,
            validator_signature: format!("pqc_sig_dilithium5_chunk_{}", chunk_id),
        };
        let verified = grid.submit_and_verify_pouw(&proof, 1500).unwrap();
        assert!(verified);
    }

    // 5. Eksekusi Smart Contract Settlement & Pencatatan Blok Baru di Private Blockchain
    let block = grid.settle_job_and_distribute_commissions("job_finetune_indobert_001").unwrap();
    assert_eq!(block.block_number, 1);
    assert_eq!(block.transactions.len(), 4);

    // Masing-masing dari 4 node mendapatkan komisi:
    // Bruto = Rp 400.000 / 4 = Rp 100.000
    // Pajak PPh (0.5%) = Rp 500
    // Bersih = Rp 99.500
    for &(node_id, _) in &nodes {
        // Ambil kembali status node setelah update
        let height = grid.get_blockchain_height();
        assert_eq!(height, 2);
    }

    let latest_block = grid.get_latest_block();
    assert_eq!(latest_block.transactions.len(), 4);
    assert_eq!(latest_block.transactions[0].amount_idr, 99500);
    assert_eq!(latest_block.transactions[0].fee_idr, 500);
}
