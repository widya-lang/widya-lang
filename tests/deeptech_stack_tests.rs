use std::collections::HashMap;
use widya::unikernel::{UnikernelCompiler, UnikernelConfig};
use widya::zk::{ZkEngine, R1csConstraint};
use widya::cdn::SovereignCdnEngine;
use widya::rtos::{PidController, DroneAutopilotEngine};
use widya::timetravel::{TimeTravelDebugger, DebugVariable};

#[test]
fn test_unikernel_multiboot_image_generation() {
    let config = UnikernelConfig {
        image_name: "widya-node-unikernel.bin".to_string(),
        target_arch: "x86_64".to_string(),
        heap_size_mb: 32,
        enable_network_virtio: true,
        enable_block_virtio: true,
    };

    let sample_bytecode = vec![0x90, 0x90, 0xC3]; // NOP, NOP, RET
    let image = UnikernelCompiler::build_unikernel_image(&config, &sample_bytecode).unwrap();

    assert_eq!(image.multiboot_header_magic, UnikernelCompiler::MULTIBOOT2_MAGIC);
    assert!(image.total_size_bytes > 16);
    assert_eq!(image.entry_point_address, 0x100010);
    assert_eq!(image.memory_map_sections.len(), 4);

    let qemu_cmd = UnikernelCompiler::generate_qemu_run_command(&image);
    assert!(qemu_cmd.contains("qemu-system-x86_64"));
    assert!(qemu_cmd.contains("virtio-net-pci"));
}

#[test]
fn test_zk_snark_proof_generation_and_zkml_verification() {
    // Proving x * y = z (where x and y are private witness, z is public output)
    let mut public_inputs = HashMap::new();
    public_inputs.insert("z".to_string(), 42);
    public_inputs.insert("output_class".to_string(), 7);

    let mut private_witness = HashMap::new();
    private_witness.insert("x".to_string(), 6);
    private_witness.insert("y".to_string(), 7);

    let constraints = vec![
        R1csConstraint {
            a_var: "x".to_string(),
            b_var: "y".to_string(),
            c_var: "z".to_string(),
        }
    ];

    let proof = ZkEngine::generate_proof(public_inputs, private_witness, &constraints).unwrap();
    assert!(ZkEngine::verify_proof(&proof));

    // Test ZKML Inference Proof Verification
    assert!(ZkEngine::verify_zkml_inference("sha256_model_medical_ai_v1", 7, &proof));
    assert!(!ZkEngine::verify_zkml_inference("sha256_model_medical_ai_v1", 99, &proof));
}

#[test]
fn test_cdn_content_addressable_merkle_dag_chunking() {
    let cdn = SovereignCdnEngine::new(64); // 64-byte chunks for testing

    let sample_data = b"Sovereign Nusantara Decentralized Content Delivery Network with Merkle-DAG Deduplication and Zero Egress Cost across all Indonesian Archipelagos!";
    let dag = cdn.publish_content(sample_data);

    assert!(dag.root_cid.starts_with("bafy"));
    assert!(dag.chunk_cids.len() > 1);
    assert_eq!(dag.total_size_bytes, sample_data.len());

    // Fetch and reconstruct
    let fetched = cdn.fetch_content(&dag.root_cid).unwrap();
    assert_eq!(fetched, sample_data);
}

#[test]
fn test_rtos_mavlink_canbus_and_pid_controller() {
    // 1. PID Controller for Drone Altitude Loop
    let mut pid = PidController::new(1.2, 0.05, 0.1);
    let target_altitude = 100.0;
    let current_altitude = 80.0;
    let control_signal = pid.compute(target_altitude, current_altitude, 0.01);
    assert!(control_signal > 0.0); // positive thrust required

    // 2. MAVLink Protocol Heartbeat Packet
    let heartbeat = DroneAutopilotEngine::encode_mavlink_heartbeat(42);
    assert_eq!(heartbeat[0], 0xFD); // MAVLink v2 magic byte
    assert_eq!(heartbeat[5], 42);   // System ID
}

#[test]
fn test_timetravel_debugger_snapshots_and_reversal() {
    let mut debugger = TimeTravelDebugger::new();

    // Frame 1
    let mut vars1 = HashMap::new();
    vars1.insert("saldo".to_string(), DebugVariable {
        name: "saldo".to_string(),
        var_type: "u64".to_string(),
        value_repr: "100000".to_string(),
    });
    debugger.record_snapshot(12, "proses_transaksi", vars1);

    // Frame 2
    let mut vars2 = HashMap::new();
    vars2.insert("saldo".to_string(), DebugVariable {
        name: "saldo".to_string(),
        var_type: "u64".to_string(),
        value_repr: "50000".to_string(),
    });
    debugger.record_snapshot(15, "proses_transaksi", vars2);

    assert_eq!(debugger.total_recorded_frames(), 2);
    assert_eq!(debugger.current_frame().unwrap().variables.get("saldo").unwrap().value_repr, "50000");

    // Step Backward in Time (Reversible Debugging)
    let prev_frame = debugger.step_backward().unwrap();
    assert_eq!(prev_frame.line_number, 12);
    assert_eq!(prev_frame.variables.get("saldo").unwrap().value_repr, "100000");

    // Step Forward in Time
    let next_frame = debugger.step_forward().unwrap();
    assert_eq!(next_frame.line_number, 15);
    assert_eq!(next_frame.variables.get("saldo").unwrap().value_repr, "50000");
}
