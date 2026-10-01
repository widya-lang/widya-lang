//! Complete Sovereign DePIN & Distributed AI Ecosystem Integration Test
//! Menguji Staking/Slashing, Federated Learning, P2P NAT/Gossip, Thermal Protection, dan Content-Addressable Storage.

use widya::depin::{
    StakingSlashingManager, SlashingReason,
    ThermalBatteryGuard, DeviceHardwareState, ComputeThrottlePolicy,
    ContentAddressableStore,
};
use widya::ai::federated_learning::{FederatedLearningCoordinator, ClientWeightUpdate};
use widya::meshnet::{P2pNatTraversal, IceCandidate, IceCandidateType, GossipProtocolEngine, GossipMessage};

#[test]
fn test_staking_and_slashing_lifecycle() {
    let mut manager = StakingSlashingManager::new(50_000, 20.0);
    manager.deposit_stake("node_semarang_01", 100_000);

    // Lock stake for job
    assert!(manager.lock_stake_for_job("node_semarang_01", 40_000).is_ok());
    assert_eq!(manager.get_account("node_semarang_01").unwrap().available_stake(), 60_000);

    // Node commits timeout fault -> execute slashing
    let slashed = manager.execute_slash("node_semarang_01", "job_ai_001", SlashingReason::TaskTimeout, 1720000000).unwrap();
    assert_eq!(slashed, 20_000); // 20% of 100k
    assert_eq!(manager.get_account("node_semarang_01").unwrap().staked_balance_idr, 80_000);
}

#[test]
fn test_federated_learning_privacy_aggregation() {
    let initial_weights = vec![0.5, 1.5, 2.5];
    let mut fed_coord = FederatedLearningCoordinator::new(initial_weights, 0.5);

    // Two clinics submitting perturbed model updates
    let mut w_clinic_a = vec![0.6, 1.6, 2.6];
    FederatedLearningCoordinator::apply_differential_privacy(&mut w_clinic_a, 0.5);

    let mut w_clinic_b = vec![0.4, 1.4, 2.4];
    FederatedLearningCoordinator::apply_differential_privacy(&mut w_clinic_b, 0.5);

    assert!(fed_coord.submit_client_update(ClientWeightUpdate {
        client_id: "klinik_jakarta".to_string(),
        round_id: 1,
        sample_count: 500,
        weights: w_clinic_a,
        noise_applied: true,
    }).is_ok());

    assert!(fed_coord.submit_client_update(ClientWeightUpdate {
        client_id: "klinik_surabaya".to_string(),
        round_id: 1,
        sample_count: 500,
        weights: w_clinic_b,
        noise_applied: true,
    }).is_ok());

    let global_round_2 = fed_coord.aggregate_fedavg(2).unwrap();
    assert_eq!(fed_coord.current_round(), 2);
    assert!((global_round_2[0] - 0.5).abs() < 0.1);
}

#[test]
fn test_p2p_nat_traversal_and_gossip() {
    // 1. NAT Traversal
    let loc = vec![IceCandidate { candidate_type: IceCandidateType::ServerReflexive, ip_address: "103.1.2.3".to_string(), port: 5000, priority: 100 }];
    let rem = vec![IceCandidate { candidate_type: IceCandidateType::ServerReflexive, ip_address: "180.4.5.6".to_string(), port: 6000, priority: 100 }];
    let pair = P2pNatTraversal::select_best_candidate(&loc, &rem).unwrap();
    assert_eq!(pair.0.ip_address, "103.1.2.3");
    assert_eq!(pair.1.ip_address, "180.4.5.6");

    // 2. Gossip dissemination
    let mut gossip = GossipProtocolEngine::new(3);
    gossip.add_peer("node_1");
    gossip.add_peer("node_2");
    gossip.add_peer("node_3");

    let targets = gossip.disseminate(GossipMessage {
        message_id: "job_broadcast_01".to_string(),
        topic: "compute_jobs".to_string(),
        payload: b"job_details".to_vec(),
        hop_count: 0,
    }).unwrap();
    assert_eq!(targets.len(), 3);
}

#[test]
fn test_thermal_battery_protection_guard() {
    let guard = ThermalBatteryGuard::new(48.0, 25);

    // Device overheating (50°C) -> Emergency Pause
    let hot_device = DeviceHardwareState {
        device_id: "phone_01".to_string(),
        battery_percentage: 80,
        is_charging: true,
        temperature_celsius: 50.0,
        is_user_interacting: false,
    };
    assert_eq!(guard.evaluate_throttle_policy(&hot_device), ComputeThrottlePolicy::EmergencyPause);

    // Device on low battery (20%) unplugged -> Emergency Pause
    let low_bat = DeviceHardwareState {
        battery_percentage: 20,
        is_charging: false,
        temperature_celsius: 36.0,
        ..hot_device.clone()
    };
    assert_eq!(guard.evaluate_throttle_policy(&low_bat), ComputeThrottlePolicy::EmergencyPause);
}

#[test]
fn test_decentralized_content_addressable_store() {
    let mut cas = ContentAddressableStore::new();
    let model_weights = vec![0xABu8; 15_000]; // 15 KB model
    let manifest = cas.publish_model("IndoLLM-Mini-GGUF", &model_weights, 5000, "node_origin_server");

    assert_eq!(manifest.chunks.len(), 3); // 5000 + 5000 + 5000 = 15000
    assert!(manifest.root_cid.starts_with("cid_"));

    assert!(cas.register_chunk_provider(&manifest.root_cid, 1, "node_caching_worker").is_ok());
    let fetched = cas.get_manifest(&manifest.root_cid).unwrap();
    assert_eq!(fetched.chunks[1].provider_nodes.len(), 2);
}
