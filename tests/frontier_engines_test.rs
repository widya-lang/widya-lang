//! Integration Test Suite: Frontier Engines (AI Tensor & Quantization, 3D Audio & Physics, Zero-Copy, CAN Bus, ZK & Raft)

use std::collections::HashMap;
use widya::ai::{Tensor, QuantizedTensor, QuantType};
use widya::multimedia::audio::{AudioBuffer, AudioFrame};
use widya::game3d::physics3d::{AABB, SphereCollider};
use widya::game3d::math3d::Vec3;
use widya::zerocopy::{ZeroCopyHeader, ZEROCOPY_MAGIC};
use widya::automotive::canbus::{CanFrame, CanFilter};
use widya::rtos::PidController;
use widya::zk::{ZkEngine, R1csConstraint};
use widya::widyabase::consensus::{RaftNode, RaftRole};

#[test]
fn test_frontier_ai_tensor_and_quantization() {
    // 1. Uji Matmul & RMSNorm
    let t1 = Tensor::new(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]);
    let t2 = Tensor::new(vec![2, 2], vec![2.0, 0.0, 1.0, 2.0]);
    let matmul_res = t1.matmul(&t2).expect("Matmul harus berhasil");
    assert_eq!(matmul_res.data, vec![4.0, 4.0, 10.0, 8.0]);

    let weight = Tensor::new(vec![2], vec![1.0, 1.0]);
    let norm_res = t1.rms_norm(&weight, 1e-5).expect("RMSNorm harus berhasil");
    assert_eq!(norm_res.shape, vec![2, 2]);

    // 2. Uji Kuantisasi INT8 & INT4
    let weights_fp32 = Tensor::new(vec![4], vec![-1.5, 0.0, 1.5, 3.0]);
    let q8 = QuantizedTensor::quantize_int8(&weights_fp32, 4);
    assert_eq!(q8.quant_type, QuantType::Int8);
    let deq8 = q8.dequantize();
    assert!((deq8.data[3] - 3.0).abs() < 0.1, "Dequantize INT8 harus mendekati nilai asli");

    let q4 = QuantizedTensor::quantize_int4(&weights_fp32, 4);
    assert_eq!(q4.quant_type, QuantType::Int4);
    let deq4 = q4.dequantize();
    assert_eq!(deq4.shape, vec![4]);
}

#[test]
fn test_frontier_spatial_audio_and_3d_physics() {
    // 1. Uji Pembangkitan Gelombang Sinus dan 3D Spatial Audio
    let mut audio = AudioBuffer::generate_sine(44100, 440.0, 0.1, 0.8);
    audio.apply_3d_spatial((0.0, 0.0, 0.0), (10.0, 0.0, 0.0)); // Sumber di sebelah kanan
    let first_frame = audio.frames[10];
    assert!(first_frame.right > first_frame.left, "Kanal kanan harus lebih keras karena sumber di kanan");

    // 2. Uji Tumbukan Fisika 3D (Sphere & AABB)
    let aabb = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 2.0));
    let sphere = SphereCollider::new(Vec3::new(1.0, 1.0, 1.0), 0.5);
    assert!(sphere.intersects_aabb(&aabb), "Sphere di dalam AABB harus bertumbukan");
}

#[test]
fn test_frontier_zerocopy_networking() {
    let header = ZeroCopyHeader::new(101, 1, 16);
    let payload = b"PAYLOAD_RAW_DATA";
    let buffer = header.serialize_to_buffer(payload);

    let (parsed_hdr, parsed_payload) = ZeroCopyHeader::from_raw_slice(&buffer).expect("Parse ZeroCopy harus sukses");
    assert_eq!(parsed_hdr.magic, ZEROCOPY_MAGIC);
    assert_eq!(parsed_hdr.message_id, 101);
    assert_eq!(parsed_payload, b"PAYLOAD_RAW_DATA");
}

#[test]
fn test_frontier_automotive_canbus_and_pid() {
    // 1. Uji CAN Bus Hardware Mask Matching
    let frame = CanFrame::new_standard(0x123, &[0x01, 0x02, 0x03, 0x04]);
    let filter_pass = CanFilter::new(0x120, 0x7F0); // Masking 4 bit bawah
    let filter_fail = CanFilter::new(0x200, 0x700);

    assert!(filter_pass.matches(&frame), "Filter 0x120/0x7F0 harus menerima frame 0x123");
    assert!(!filter_fail.matches(&frame), "Filter 0x200/0x700 harus menolak frame 0x123");

    // 2. Uji PID Controller loop
    let mut pid = PidController::new(1.0, 0.1, 0.05);
    let output = pid.compute(100.0, 80.0, 0.01);
    assert!(output > 0.0, "PID harus menghasilkan nilai koreksi positif");
}

#[test]
fn test_frontier_zk_proofs_and_raft_consensus() {
    // 1. Uji Zero-Knowledge Proof (R1CS Constraint A * B = C)
    let mut public = HashMap::new();
    public.insert("c".to_string(), 12);

    let mut witness = HashMap::new();
    witness.insert("a".to_string(), 3);
    witness.insert("b".to_string(), 4);

    let constraints = vec![R1csConstraint {
        a_var: "a".to_string(),
        b_var: "b".to_string(),
        c_var: "c".to_string(),
    }];

    let proof = ZkEngine::generate_proof(public, witness, &constraints).expect("Generate ZK proof harus sukses");
    assert!(ZkEngine::verify_proof(&proof), "Verifikasi ZK proof harus valid");

    // 2. Uji Raft Distributed State Machine Consensus
    let node = RaftNode::new(1);
    assert_eq!(node.get_role(), RaftRole::Follower);

    let (node_id, term) = node.start_election();
    assert_eq!(node_id, 1);
    assert_eq!(term, 1);
    assert_eq!(node.get_role(), RaftRole::Candidate);

    node.become_leader();
    assert_eq!(node.get_role(), RaftRole::Leader);

    let entry_idx = node.append_entry(b"INSERT KEY VALUE".to_vec());
    assert_eq!(entry_idx, 1);
}
