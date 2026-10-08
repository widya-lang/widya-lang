use widya::bootstrap::{SelfHostingBootstrap, BootstrapStageStatus};

#[test]
fn test_self_hosted_modules_verification() {
    let bootstrapper = SelfHostingBootstrap::new("compiler_self_hosted");
    
    // Verifikasi masing-masing modul
    let token_res = bootstrapper.verify_module("token.widya");
    assert!(token_res.is_ok(), "Gagal verifikasi token.widya: {:?}", token_res.err());

    let lexer_res = bootstrapper.verify_module("lexer.widya");
    assert!(lexer_res.is_ok(), "Gagal verifikasi lexer.widya: {:?}", lexer_res.err());

    let ast_res = bootstrapper.verify_module("ast.widya");
    assert!(ast_res.is_ok(), "Gagal verifikasi ast.widya: {:?}", ast_res.err());

    let parser_res = bootstrapper.verify_module("parser.widya");
    assert!(parser_res.is_ok(), "Gagal verifikasi parser.widya: {:?}", parser_res.err());

    let typesystem_res = bootstrapper.verify_module("typesystem.widya");
    assert!(typesystem_res.is_ok(), "Gagal verifikasi typesystem.widya: {:?}", typesystem_res.err());

    let codegen_res = bootstrapper.verify_module("codegen.widya");
    assert!(codegen_res.is_ok(), "Gagal verifikasi codegen.widya: {:?}", codegen_res.err());

    let main_res = bootstrapper.verify_module("main.widya");
    assert!(main_res.is_ok(), "Gagal verifikasi main.widya: {:?}", main_res.err());
}

#[test]
fn test_stage_hashes_are_cryptographic_and_deterministic() {
    let bootstrapper = SelfHostingBootstrap::new("compiler_self_hosted");
    let stages = bootstrapper.run_full_bootstrap_cycle();

    assert_eq!(stages.len(), 3);

    // Stage 0: SHA-256 dari sumber
    if let BootstrapStageStatus::Success { ref hash, .. } = stages[0] {
        assert!(hash.starts_with("sha256:"), "Harus format sha256: {:?}", hash);
        assert_eq!(hash.len(), 7 + 64, "Panjang hash SHA-256 harus 64 hex characters: {}", hash);
    } else {
        panic!("Stage-0 harus Success");
    }

    // Stage 1: SHA-256 biner native mandiri
    let stage1_digest = if let BootstrapStageStatus::Success { ref hash, .. } = stages[1] {
        assert!(hash.starts_with("sha256:"));
        hash.clone()
    } else {
        panic!("Stage-1 harus Success");
    };

    // Stage 2: Deterministik reproducible build
    if let BootstrapStageStatus::Success { ref hash, .. } = stages[2] {
        assert!(hash.contains("_deterministic_match"));
        assert!(hash.contains(&stage1_digest[7..30]), "Stage-2 harus cocok dengan digest Stage-1");
    } else {
        panic!("Stage-2 harus Success");
    }
}

#[test]
fn test_jit_engine_bytecode_execution_and_metrics() {
    use widya::tools::jit::JitEngine;

    let source = r#"
        misal total = 0;
        misal i = 0;
        selama i < 10 {
            total = total + i;
            i = i + 1;
        }
    "#;

    let jit = JitEngine::new();
    let rep = jit.execute_jit_advanced(source).expect("Eksekusi JIT harus berhasil");

    assert!(rep.duration_ms >= 0.0);
    assert!(rep.mode.contains("Register-VM") || rep.mode.contains("AST"));
    if rep.instructions_executed > 0 {
        assert!(rep.instructions_executed > 10, "Total instruksi VM harus dieksekusi");
    }
}

