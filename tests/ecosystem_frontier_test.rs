//! Integration Test Suite: Complete Ecosystem Frontier Engines (Agentic, WPM Resolver, UI Engine, Security Linter, Formal Verifier)

use std::collections::HashMap;
use widya::ai::agent_runtime::{NativeAgent, ToolCall, ToolResult, ToolRegistry, AgentExecutor, MultiAgentBus};
use widya::wpm_resolver::{SemVer, PackageSpec, DependencyResolver};
use widya::ui_engine::{UiNode, UiStateStore, ReactiveUiRenderer};
use widya::security_linter::{SecurityLinter, VulnerabilitySeverity};
use widya::formal_verifier::{NumericInterval, FunctionContract, FormalVerifier};

#[test]
fn test_native_multi_agent_runtime() {
    // 1. Tool Registry & Tool Execution
    let mut tools = ToolRegistry::new();
    tools.register("cek_saldo_bifast", |args| {
        let no_rek = args.get("rekening").cloned().unwrap_or_default();
        ToolResult {
            success: true,
            output: format!("Saldo Rekening {} adalah Rp 50.000.000,00", no_rek),
        }
    });

    let agent = NativeAgent {
        name: "AgenFinansial".to_string(),
        model_id: "qwen-2.5-7b".to_string(),
        system_prompt: "Analisis perbankan".to_string(),
        tools: vec!["cek_saldo_bifast".to_string()],
        memory_context: Vec::new(),
    };

    let mut executor = AgentExecutor::new(agent, tools);

    let mut args = HashMap::new();
    args.insert("rekening".to_string(), "1234567890".to_string());
    let tool_call = ToolCall {
        tool_name: "cek_saldo_bifast".to_string(),
        arguments: args,
    };

    let obs = executor.step("Cek saldo nasabah", Some(tool_call));
    assert!(obs.contains("50.000.000"), "Observasi agen harus memuat hasil eksekusi tool");

    // 2. MultiAgent Communication Bus
    let mut bus = MultiAgentBus::new();
    bus.send_message("AgenQA", "Harap uji modul transfer");
    let inbox = bus.receive_messages("AgenQA");
    assert_eq!(inbox, vec!["Harap uji modul transfer"]);
}

#[test]
fn test_wpm_dependency_resolver() {
    let mut resolver = DependencyResolver::new();
    resolver.register_package("widya-fintech", vec![SemVer::new(1, 0, 0), SemVer::new(1, 2, 0)]);
    resolver.register_package("widya-crypto", vec![SemVer::new(2, 0, 0), SemVer::new(2, 1, 0)]);

    let root_deps = vec![
        PackageSpec {
            name: "widya-fintech".to_string(),
            required_version: SemVer::new(1, 1, 0),
            dependencies: vec![("widya-crypto".to_string(), SemVer::new(2, 0, 0))],
        }
    ];

    let graph = resolver.resolve(&root_deps).expect("Resolusi dependensi harus berhasil");
    assert_eq!(graph.packages.get("widya-fintech"), Some(&SemVer::new(1, 2, 0)));
    assert_eq!(graph.packages.get("widya-crypto"), Some(&SemVer::new(2, 1, 0)));
    assert!(!graph.checksum_sha256.is_empty());
}

#[test]
fn test_reactive_ui_engine() {
    let ui_tree = UiNode::Container {
        padding: 16.0,
        background_hex: "#FFFFFF".to_string(),
        child: Box::new(UiNode::Column {
            spacing: 8.0,
            children: vec![
                UiNode::Text {
                    content: "Dashboard".to_string(),
                    font_size: 24.0,
                    color_hex: "#000000".to_string(),
                },
                UiNode::Button {
                    label: "Submit".to_string(),
                    on_click_id: "BTN_SUBMIT".to_string(),
                }
            ],
        }),
    };

    assert_eq!(ReactiveUiRenderer::count_nodes(&ui_tree), 4);

    let mut state = UiStateStore::new();
    let handled = ReactiveUiRenderer::handle_event(&ui_tree, "BTN_SUBMIT", &mut state);
    assert!(handled);
    assert_eq!(state.get("last_clicked"), Some(&"BTN_SUBMIT".to_string()));
    assert!(state.is_dirty());
}

#[test]
fn test_security_linter_and_uupdp() {
    let bad_code = r#"
        let query = "SELECT * FROM users WHERE id = " + user_input;
        let api_key = "AIzaSyD-TEST_SECRET_KEY_1234567890";
        tulis_berkas("log.txt", "NIK Pasien: " + nik_pasien);
    "#;

    let findings = SecurityLinter::scan_source(bad_code);
    assert_eq!(findings.len(), 3);
    assert_eq!(findings[0].severity, VulnerabilitySeverity::Critical); // SQLi
    assert_eq!(findings[1].severity, VulnerabilitySeverity::High);     // Secret
    assert_eq!(findings[2].rule_id, "WIDYA-PDP-001");                 // UU PDP
}

#[test]
fn test_formal_verification_engine() {
    // 1. Keamanan Divide-by-Zero
    let safe_contract = FunctionContract {
        function_name: "bagi_pajak".to_string(),
        divisor_interval: Some(NumericInterval::new(1, 100)), // 1 s/d 100 (Tidak ada 0)
        max_expected_output: 1000,
    };
    assert!(FormalVerifier::verify_division_safety(&safe_contract).is_ok());

    let unsafe_contract = FunctionContract {
        function_name: "bagi_berbahaya".to_string(),
        divisor_interval: Some(NumericInterval::new(-5, 5)), // -5 s/d 5 (Memuat 0!)
        max_expected_output: 1000,
    };
    assert!(FormalVerifier::verify_division_safety(&unsafe_contract).is_err());

    // 2. Range Safety
    let interval_res = NumericInterval::new(10, 50).mul(&NumericInterval::new(2, 4)); // Range: 20 s/d 200
    assert!(FormalVerifier::verify_range_safety(&interval_res, &safe_contract).is_ok());
}
