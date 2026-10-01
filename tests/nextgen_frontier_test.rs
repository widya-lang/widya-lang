//! Integration Test Suite: Next-Gen Sovereign Frontier (WSSH Quantum-Safe, Self-Hosting, Time-Travel, WasmGrid, Agent Market)

use std::collections::HashMap;
use widya::kripto::pqc::PqcEngine;
use widya::kripto::wssh::WsshSecureSession;
use widya::bootstrap_selfhost::{SelfHostBootstrapper, SelfHostAst, SelfHostToken};
use widya::timetravel_debugger::TimeTravelDebugger;
use widya::wasm_grid::{WasmGridModule, WasmGridScheduler};
use widya::ai::agent_market::{AgentSkill, AgentSkillMarket};

#[test]
fn test_wssh_quantum_safe_session() {
    let client_kp = PqcEngine::generate_dilithium5_keypair("ENTROPY_TEST_SEED_12345");
    let (handshake_pkt, _kem_secret) = WsshSecureSession::client_initiate_handshake(&client_kp);

    let (mut session, kem_ciphertext) = WsshSecureSession::server_accept_handshake(
        &handshake_pkt,
        &client_kp.public_key_hex,
    ).expect("Handshake WSSH harus sukses");

    assert!(session.is_authenticated);
    assert_eq!(kem_ciphertext.len(), 1568);

    // Uji Enkripsi dan Dekripsi Frame
    let plaintext = b"RAHASIA_NEGARA_POST_QUANTUM_DATA";
    let ciphertext = session.encrypt_frame(plaintext);
    assert_ne!(ciphertext, plaintext);

    let decrypted = session.decrypt_frame(&ciphertext).expect("Dekripsi frame harus berhasil");
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_self_hosting_compiler_bootstrapper() {
    let src = "fungsi hitung_pajak variabel nominal kembalikan 100";
    let tokens = SelfHostBootstrapper::tokenize(src);

    assert_eq!(tokens[0], SelfHostToken::KataKunci("fungsi".to_string()));
    assert_eq!(tokens[1], SelfHostToken::Pengenal("hitung_pajak".to_string()));

    let ast = vec![
        SelfHostAst::DeklarasiFungsi {
            nama: "cetak_laporan".to_string(),
            parameter: vec!["id_laporan".to_string()],
            tubuh: vec![
                SelfHostAst::DeklarasiVariabel {
                    nama: "status".to_string(),
                    nilai: "200".to_string(),
                }
            ],
        }
    ];

    let c_code = SelfHostBootstrapper::emit_stage1_c(&ast);
    assert!(c_code.contains("void cetak_laporan_widya("));
    assert!(c_code.contains("int64_t status = 200;"));
}

#[test]
fn test_timetravel_debugger_reverse_execution() {
    let mut debugger = TimeTravelDebugger::new();

    let mut v1 = HashMap::new();
    v1.insert("saldo".to_string(), "1000".to_string());
    debugger.record_step(10, v1, 100);

    let mut v2 = HashMap::new();
    v2.insert("saldo".to_string(), "2000".to_string());
    debugger.record_step(11, v2, 200);

    let mut v3 = HashMap::new();
    v3.insert("saldo".to_string(), "3000".to_string());
    debugger.record_step(12, v3, 300);

    assert_eq!(debugger.current_snapshot().unwrap().variables.get("saldo"), Some(&"3000".to_string()));

    // 1. Step Backward
    let prev = debugger.step_backward().expect("Harus bisa step back");
    assert_eq!(prev.variables.get("saldo"), Some(&"2000".to_string()));

    // 2. Step Backward lagi
    let orig = debugger.step_backward().expect("Harus bisa step back ke awal");
    assert_eq!(orig.variables.get("saldo"), Some(&"1000".to_string()));

    // 3. Step Forward
    let fwd = debugger.step_forward().expect("Harus bisa step forward");
    assert_eq!(fwd.variables.get("saldo"), Some(&"2000".to_string()));
}

#[test]
fn test_wasm_grid_serverless_scheduler() {
    let mut grid = WasmGridScheduler::new();

    grid.register_module(WasmGridModule {
        module_id: "mod-nlp-inference".to_string(),
        bytecode_hash: "sha256_hash_abc123".to_string(),
        memory_limit_mb: 64,
        timeout_ms: 1000,
    });

    grid.add_worker("worker-edge-jakarta-01", 2);
    grid.add_worker("worker-edge-surabaya-01", 2);

    let res1 = grid.dispatch("mod-nlp-inference").expect("Dispatch 1 harus sukses");
    assert!(res1.contains("Dispatched"));

    let res2 = grid.dispatch("mod-nlp-inference").expect("Dispatch 2 harus sukses");
    assert!(res2.contains("Dispatched"));
}

#[test]
fn test_autonomous_agent_skill_market() {
    let mut market = AgentSkillMarket::new();
    market.set_agent_balance("AgenPembeli", 50000);
    market.set_agent_balance("AgenPakarPajak", 0);

    market.register_skill(AgentSkill {
        skill_id: "hitung_ppn_otomatis".to_string(),
        provider_agent_id: "AgenPakarPajak".to_string(),
        description: "Menghitung PPN 11% dan e-Faktur".to_string(),
        price_per_call_idr: 5000,
    });

    let settlement = market.invoke_skill("AgenPembeli", "hitung_ppn_otomatis").expect("Pemanggilan skill harus berhasil");
    assert!(settlement.settled);
    assert_eq!(settlement.amount_idr, 5000);

    assert_eq!(market.get_balance("AgenPembeli"), 45000);
    assert_eq!(market.get_balance("AgenPakarPajak"), 5000);
}
