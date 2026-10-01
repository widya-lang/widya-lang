//! Sovereign AI Economy & Frontier Web3 Integration Test
//! Menguji Dynamic Spot Auction, Digital Rupiah CBDC, Hardware TEE Attestation, Autonomous AI Agent Hosting, dan DePIN DAO Governance.

use widya::depin::{
    SpotComputeAuctionEngine, ComputeOrder, OrderType,
    DepinDaoEngine, GovernanceAction, ProposalStatus,
};
use widya::web3::cbdc_bridge::{CbdcCrossChainGateway, DigitalRupiahAccount, CbdcType};
use widya::security_tools::{TeeAttestationEngine, TeePlatformType};
use widya::ai::autonomous_hosting::AutonomousAgentHostManager;

#[test]
fn test_spot_market_double_auction() {
    let mut market = SpotComputeAuctionEngine::new(20.0); // Base Rp 20

    // Klien menawar Rp 30/TFLOPS untuk 100 TFLOPS
    market.place_order(ComputeOrder {
        order_id: "bid_corp_01".to_string(),
        participant_wallet: "wallet_enterprise_klien".to_string(),
        order_type: OrderType::Bid,
        tflops_amount: 100.0,
        price_per_tflops_sec_idr: 30.0,
        timestamp: 1720000000,
    });

    // Worker menawarkan Rp 24/TFLOPS untuk 100 TFLOPS
    market.place_order(ComputeOrder {
        order_id: "ask_node_01".to_string(),
        participant_wallet: "wallet_gpu_worker".to_string(),
        order_type: OrderType::Ask,
        tflops_amount: 100.0,
        price_per_tflops_sec_idr: 24.0,
        timestamp: 1720000000,
    });

    let trades = market.match_orders();
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].matched_tflops, 100.0);
    assert_eq!(trades[0].settlement_price_idr, 27.0); // (30 + 24) / 2 = 27.0
}

#[test]
fn test_digital_rupiah_cbdc_and_cross_chain() {
    let mut bridge = CbdcCrossChainGateway::new(16_000);

    bridge.register_cbdc_account(DigitalRupiahAccount {
        account_id: "bi_cbdc_acc_pln".to_string(),
        nik_or_npwp: "01.000.000.0-000.000".to_string(),
        cbdc_type: CbdcType::WholesaleCBDC,
        balance_idr: 100_000_000,
        is_kyc_verified: true,
    });

    bridge.register_cbdc_account(DigitalRupiahAccount {
        account_id: "bi_cbdc_acc_miner".to_string(),
        nik_or_npwp: "3171000000000001".to_string(),
        cbdc_type: CbdcType::RetailCBDC,
        balance_idr: 0,
        is_kyc_verified: true,
    });

    // Transfer Rp 20.000.000
    let tx = bridge.transfer_digital_rupiah("bi_cbdc_acc_pln", "bi_cbdc_acc_miner", 20_000_000).unwrap();
    assert!(tx.starts_with("cbdc_tx_"));
    assert_eq!(bridge.get_cbdc_balance("bi_cbdc_acc_pln").unwrap(), 80_000_000);
    assert_eq!(bridge.get_cbdc_balance("bi_cbdc_acc_miner").unwrap(), 20_000_000);
}

#[test]
fn test_hardware_tee_remote_attestation() {
    let code_binary = b"SOVEREIGN_ENCLAVE_AI_INFERENCE_CODE_V1";
    let confidential_data = b"CONFIDENTIAL_MEDICAL_RECORDS_DATA";

    let report = TeeAttestationEngine::generate_attestation_report(
        TeePlatformType::IntelSgx,
        code_binary,
        confidential_data,
    );

    assert!(report.is_valid);
    assert!(report.mrenclave_hash.starts_with("mrenclave_"));
    assert!(report.user_data_hash.starts_with("userdata_"));

    let verified = TeeAttestationEngine::verify_report(&report, "").unwrap();
    assert!(verified);
}

#[test]
fn test_autonomous_ai_agent_self_funding() {
    let mut host = AutonomousAgentHostManager::new(10.0); // 10% platform fee

    // Agent deployed with Rp 50.000 (biaya Rp 25.000/jam -> 2 jam hosting)
    host.deploy_agent("agent_lawyer", "Sovereign Legal AI", 50_000, 25_000);

    // Agent sells consultation contract for Rp 100.000
    let net = host.earn_service_revenue("agent_lawyer", 100_000).unwrap();
    assert_eq!(net, 90_000); // 100k - 10k fee

    let agent = host.get_agent("agent_lawyer").unwrap();
    // 2 jam awal + (90.000 / 25.000 = 3.6 jam) = 5.6 jam
    assert!((agent.prepaid_hours_remaining - 5.6).abs() < 0.01);
}

#[test]
fn test_depin_dao_governance_execution() {
    let mut dao = DepinDaoEngine::new(500.0); // 500 weight quorum

    let pid = dao.create_proposal(
        "Tingkatkan Penalti Slashing Menjadi 30%",
        "wallet_validator_01",
        GovernanceAction::UpdateSlashingPenaltyPercent(30.0),
        7200,
        1720000000,
    );

    // Node voting
    assert!(dao.cast_vote(pid, "wallet_datacenter_jakarta", true, 400.0).is_ok());
    assert!(dao.cast_vote(pid, "wallet_datacenter_surabaya", true, 200.0).is_ok());

    let final_status = dao.finalize_proposal(pid).unwrap();
    assert_eq!(final_status, ProposalStatus::Passed);
}
