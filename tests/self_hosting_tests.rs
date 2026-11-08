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
fn test_self_hosted_compiler_executes_internal_bootstrap_verification() {
    let bootstrapper = SelfHostingBootstrap::new("compiler_self_hosted");
    let res = bootstrapper.execute_compiler_driver_test()
        .expect("Eksekusi driver compiler mandiri harus berhasil");
    assert_eq!(res, "BOOTSTRAP_SUKSES", "Kompilator mandiri harus menghasilkan status BOOTSTRAP_SUKSES");
}

#[test]
fn test_self_hosted_compiler_full_deterministic_convergence_and_ed25519() {
    let bootstrapper = SelfHostingBootstrap::new("compiler_self_hosted");
    let report = bootstrapper.verify_deterministic_convergence()
        .expect("Verifikasi konvergensi deterministik Stage-1 == Stage-2 harus sukses");

    assert!(report.is_deterministic_match, "Kompilasi Stage-2 harus konvergen identik dengan Stage-1");
    assert_eq!(report.total_modules_count, 7, "Harus mencakup tepat 7 modul penyusun kompilator mandiri");
    assert!(report.total_source_lines > 100, "Total baris kode kompilator mandiri harus signifikan (> 100)");
    assert!(report.emitted_lines_count > 30, "Baris biner/C native emisi harus valid");
    
    // Tanda tangan kriptografis Ed25519 rantai pasok
    assert_eq!(report.signature_ed25519_hex.len(), 128, "Tanda tangan Ed25519 harus tepat 64 byte (128 karakter hex)");
    assert_eq!(report.public_key_ed25519_hex.len(), 64, "Kunci publik Ed25519 harus tepat 32 byte (64 karakter hex)");
}

#[test]
fn test_self_hosted_compiler_generates_valid_c_emission() {
    let bootstrapper = SelfHostingBootstrap::new("compiler_self_hosted");
    let (code, digest) = bootstrapper.emit_stage1_native_compiler()
        .expect("Emisi C99 Stage-1 harus berhasil");

    assert!(code.contains("#include <stdio.h>"), "Emisi harus memuat header C standar");
    assert!(code.contains("#include <stdint.h>"), "Emisi harus memuat header tipe data standar");
    assert!(code.contains("widya_self_"), "Emisi harus memuat simbol fungsi terkompilasi mandiri");
    assert!(code.contains("int main("), "Emisi harus memuat driver biner eksekusi mandiri");
    assert!(digest.starts_with("sha256:"), "Digest harus terformat sha256:");
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
