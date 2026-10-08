use std::collections::HashMap;
use std::fs;
use widya::dap::{DapEngine, DebuggerState};
use widya::lsp_enhanced::{DiagnosticSeverity, Position, WidyaLsp};
use widya::wpm::{DependencyResolver, InMemoryRegistry, VersionReq, Wpm, WpmConfig};

#[test]
fn test_pubgrub_multi_transitive_diamond_resolution() {
    let mut registry = InMemoryRegistry::new();

    // Setup dependency graph:
    // Root -> WebApp (1.0.0)
    // WebApp -> Router (^1.0.0), Auth (^2.0.0)
    // Router (1.0.0) -> Crypto (>=1.0.0, <2.0.0)
    // Auth (2.0.0)   -> Crypto (^1.5.0)
    // Crypto versions: 1.0.0, 1.4.0, 1.5.2, 1.9.0, 2.0.0
    // Solver must resolve to Crypto@1.9.0 (highest compatible with both Router and Auth)

    registry
        .add_package_version(
            "webapp",
            "1.0.0",
            vec![("router", "^1.0.0"), ("auth", "^2.0.0")],
            "sha_webapp_100",
        )
        .unwrap();

    registry
        .add_package_version(
            "router",
            "1.0.0",
            vec![("crypto", ">=1.0.0, <2.0.0")],
            "sha_router_100",
        )
        .unwrap();

    registry
        .add_package_version(
            "auth",
            "2.0.0",
            vec![("crypto", "^1.5.0")],
            "sha_auth_200",
        )
        .unwrap();

    for v in &["1.0.0", "1.4.0", "1.5.2", "1.9.0", "2.0.0"] {
        registry
            .add_package_version("crypto", v, vec![], &format!("sha_crypto_{}", v))
            .unwrap();
    }

    let solver = DependencyResolver::new(&registry);
    let mut root_reqs = HashMap::new();
    root_reqs.insert("webapp".to_string(), VersionReq::parse("=1.0.0").unwrap());

    let resolved = solver.resolve(&root_reqs).expect("Resolusi diamond dependency harus berhasil!");

    assert_eq!(resolved.get("webapp").unwrap().version, "1.0.0");
    assert_eq!(resolved.get("router").unwrap().version, "1.0.0");
    assert_eq!(resolved.get("auth").unwrap().version, "2.0.0");
    // Crypto harus 1.9.0 karena memenuhi ^1.5.0 (auth) dan >=1.0.0, <2.0.0 (router)
    assert_eq!(resolved.get("crypto").unwrap().version, "1.9.0");
}

#[test]
fn test_pubgrub_incompatible_conflict_explanation() {
    let mut registry = InMemoryRegistry::new();

    // App -> PkgA (^1.0.0), PkgB (^1.0.0)
    // PkgA -> DepX (=1.0.0)
    // PkgB -> DepX (=2.0.0)
    // DepX versions: 1.0.0, 2.0.0
    // Harus menghasilkan galat konflik yang jelas dan menyebutkan konflik DepX

    registry
        .add_package_version("pkga", "1.0.0", vec![("depx", "=1.0.0")], "sha_a")
        .unwrap();

    registry
        .add_package_version("pkgb", "1.0.0", vec![("depx", "=2.0.0")], "sha_b")
        .unwrap();

    registry
        .add_package_version("depx", "1.0.0", vec![], "sha_depx1")
        .unwrap();
    registry
        .add_package_version("depx", "2.0.0", vec![], "sha_depx2")
        .unwrap();

    let solver = DependencyResolver::new(&registry);
    let mut root_reqs = HashMap::new();
    root_reqs.insert("pkga".to_string(), VersionReq::parse("^1.0.0").unwrap());
    root_reqs.insert("pkgb".to_string(), VersionReq::parse("^1.0.0").unwrap());

    let err = solver.resolve(&root_reqs).expect_err("Harus terjadi konflik dependensi!");
    assert!(
        err.contains("Konflik dependensi") || err.contains("depx"),
        "Pesan error harus menjelaskan konflik dependensi: {}",
        err
    );
}

#[test]
fn test_wpm_vendor_manifest_and_airgapped_resolution() {
    let temp_dir = std::env::temp_dir().join(format!("widya_wpm_devex_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let wpm = Wpm::new(WpmConfig::default());
    let proj = wpm.init("test_offline_app", &temp_dir).unwrap();

    // Tambahkan dependensi eksternal ke proyek
    wpm.add_dependency(&proj, "widya_offline_math", "1.0.0").unwrap();

    let vendor_manifest = wpm.vendor(&proj, None).unwrap();
    assert_eq!(vendor_manifest.packages.len(), 1);
    assert_eq!(vendor_manifest.packages[0].name, "widya_offline_math");

    let is_valid = wpm.verify_vendor(&proj.join("vendor")).unwrap();
    assert!(is_valid);

    // Sekarang simulasikan runtime interpreter mengimpor dari direktori vendor
    let vendor_lib_path = proj.join("vendor").join("widya_offline_math").join("src").join("lib.wya");
    assert!(vendor_lib_path.exists());

    // Bersihkan temporary directory
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_lsp_diagnostics_and_completions() {
    // 1. LSP Diagnostics mendeteksi syntax error
    let invalid_code = "fungsi rusak( { cetak(1);";
    let diags = WidyaLsp::get_diagnostics(invalid_code);
    assert!(!diags.is_empty(), "LSP harus mendeteksi sintaks yang rusak!");
    assert_eq!(diags[0].severity, DiagnosticSeverity::Error);

    // 2. LSP Diagnostics kode valid tidak memiliki error
    let valid_code = "fungsi jumlah(a, b) { kembalikan a + b; }";
    let valid_diags = WidyaLsp::get_diagnostics(valid_code);
    assert!(valid_diags.is_empty(), "Kode yang benar tidak boleh menghasilkan galat diagnostik!");

    // 3. LSP Completions
    let lsp = WidyaLsp::new();
    let pos = Position { line: 1, character: 5 };
    let completions = lsp.get_completions(pos, valid_code);
    assert!(!completions.is_empty(), "LSP harus mengembalikan daftar completion!");
    let has_fungsi = completions.iter().any(|c| c.label == "fungsi" || c.label == "kembalikan");
    assert!(has_fungsi, "Daftar completion harus mencakup kata kunci Widya!");
}

#[test]
fn test_dap_engine_breakpoints_and_timetravel() {
    let mut dap = DapEngine::new();
    assert_eq!(dap.state, DebuggerState::Initialized);

    // Set breakpoint di baris 5 dan 10
    let bps = dap.set_breakpoints("main.wya", vec![5, 10]);
    assert_eq!(bps.len(), 2);
    assert_eq!(bps[0].line, 5);
    assert_eq!(bps[1].line, 10);

    // Step over
    dap.current_line = 4;
    dap.step_over();
    assert_eq!(dap.current_line, 5);

    // DAP Capabilities
    let caps = dap.get_dap_capabilities();
    assert_eq!(caps["supportsConfigurationDoneRequest"], true);
    assert_eq!(caps["supportsStepBack"], true);
}

#[test]
fn test_devex_bilingual_formatter_bidirectional_roundtrip() {
    use widya::tools::formatter::{format_code_with_style, KeywordStyle};

    let nusantara_src = r#"// Header komentar
fungsi hitung_saldo(mutabel saldo: int, mutabel mutasi: int) -> int {
    jika mutasi > 0 {
        cetak("Mutasi masuk: " + "string aman");
        kembalikan saldo + mutasi;
    } selain {
        cetak("Mutasi keluar");
        kembalikan saldo - mutasi;
    }
}
"#;

    // Convert to Internasional
    let intl = format_code_with_style(nusantara_src, Some(KeywordStyle::Internasional));
    assert!(intl.contains("fn hitung_saldo("));
    assert!(intl.contains("mut saldo: int"));
    assert!(intl.contains("if mutasi > 0"));
    assert!(intl.contains("print(\"Mutasi masuk: \" + \"string aman\")"));
    assert!(intl.contains("return saldo + mutasi;"));
    assert!(intl.contains("else {"));

    // Convert back to Nusantara
    let back_nusantara = format_code_with_style(&intl, Some(KeywordStyle::Nusantara));
    assert!(back_nusantara.contains("fungsi hitung_saldo("));
    assert!(back_nusantara.contains("mutabel saldo: int"));
    assert!(back_nusantara.contains("jika mutasi > 0"));
    assert!(back_nusantara.contains("cetak(\"Mutasi masuk: \" + \"string aman\")"));
    assert!(back_nusantara.contains("kembalikan saldo + mutasi;"));
}

#[test]
fn test_devex_security_and_pdp_linter_integration() {
    use widya::security_linter::SecurityLinter;

    let vulnerable_src = r#"
fungsi proses_transaksi(input_user: teks) {
    misal api_key = "AKIA1234567890123456";
    misal nik_pelanggan = "3171012345678901";
    cetak("Data NIK: " + nik_pelanggan);
    misal query = "SELECT * FROM users WHERE nama = '" + input_user + "'";
    system("rm -rf " + input_user);
    selama benar {
        misal buffer = [];
        buffer.tambah(1);
    }
}
"#;

    let findings = SecurityLinter::scan_source(vulnerable_src);
    assert!(!findings.is_empty());

    let codes: Vec<&str> = findings.iter().map(|i| i.rule_id.as_str()).collect();
    // Harus mendeteksi hardcoded secret, PDP NIK, SQL Injection, Command Injection, Memory Leak
    assert!(codes.contains(&"WIDYA-SEC-002"), "Deteksi Hardcoded Secret");
    assert!(codes.contains(&"WIDYA-PDP-001"), "Deteksi NIK UU PDP");
    assert!(codes.contains(&"WIDYA-SEC-001"), "Deteksi SQL Injection");
    assert!(codes.contains(&"WIDYA-SEC-003"), "Deteksi Command Injection");
    assert!(codes.contains(&"WIDYA-MEM-001"), "Deteksi Unbounded Growth Memory Leak");
}

#[test]
fn test_devex_wasm_playground_pipeline() {
    use widya::wasm_playground::WasmPlayground;

    let code = r#"
fungsi kuadrat(x) {
    kembalikan x * x;
}
kuadrat(5);
"#;
    let res = WasmPlayground::evaluate(code);
    assert!(res.success);
    assert!(res.ast_json.is_some());
    assert!(res.bytecode_disassembly.is_some());
    let disasm = res.bytecode_disassembly.unwrap();
    assert!(disasm.contains("LoadConst") || disasm.contains("Return") || disasm.contains("Mul") || disasm.contains("Call"));
}

#[test]
fn test_devex_editor_grammars_and_queries_exist() {
    use std::path::Path;

    // Tree-sitter grammar
    assert!(Path::new("editors/tree-sitter-widya/grammar.js").exists());
    assert!(Path::new("editors/tree-sitter-widya/queries/highlights.scm").exists());
    assert!(Path::new("editors/tree-sitter-widya/queries/indents.scm").exists());

    // Neovim queries
    assert!(Path::new("editors/neovim/queries/widya/highlights.scm").exists());
    assert!(Path::new("editors/neovim/queries/widya/indents.scm").exists());

    // Helix configuration
    assert!(Path::new("editors/helix/languages.toml").exists());
    assert!(Path::new("editors/helix/runtime/queries/widya/highlights.scm").exists());

    // JetBrains plugin
    assert!(Path::new("editors/jetbrains/META-INF/plugin.xml").exists());
    assert!(Path::new("editors/jetbrains/src/main/resources/syntaxes/widya.tmLanguage.json").exists());
}

#[test]
fn test_wpm_official_catalog_21_sovereign_packages() {
    use widya::wpm::WpmRegistryServer;

    let server = WpmRegistryServer::new(8095);
    let all_pkgs = server.search("");

    // Pastikan katalog memiliki minimal 21 paket resmi berdaulat
    assert!(all_pkgs.len() >= 21, "Katalog resmi WPM harus memuat minimal 21 paket, ditemukan: {}", all_pkgs.len());

    let required_packages = [
        "widya-math", "widya-http", "widya-crypto", "widya-ui", "widya-ai",
        "widya-sql", "widya-office", "widya-pqc", "widya-bifast", "widya-arkas",
        "widya-siplah", "widya-wasi", "widya-canbus", "widya-scada", "widya-fhir",
        "widya-quantum", "widya-i18n", "widya-test", "widya-json", "widya-grpc", "widya-log",
    ];

    for name in &required_packages {
        let pkg = server.get_package(name);
        assert!(pkg.is_some(), "Paket resmi '{}' harus terdaftar dalam katalog WPM!", name);
        let p = pkg.unwrap();
        assert!(!p.description.is_empty(), "Deskripsi paket '{}' tidak boleh kosong", name);
        assert_eq!(p.author, "Tim Inti Widya");
        assert_eq!(p.license, "MIT");

        // Periksa rilis versi dan tanda tangan kriptografis Ed25519
        let ver_info = p.versions.get(&p.latest_version);
        assert!(ver_info.is_some(), "Versi rilis terbaru paket '{}' harus ada dalam registry", name);
        let v = ver_info.unwrap();
        assert_eq!(v.checksum_sha256.len(), 64, "Checksum SHA256 paket '{}' harus valid 64 karakter hex", name);
        assert!(v.signature.is_some(), "Paket '{}' harus ditandatangani secara kriptografis dengan Ed25519", name);
    }
}

#[test]
fn test_wpm_security_auditor_detection_and_tamper_proofing() {
    use widya::wpm::{Wpm, WpmRegistryServer, WpmSecurityAuditor, VulnerabilitySeverity};

    let server = WpmRegistryServer::new(8096);

    // 1. Uji skenario paket rentan (widya-http < 1.1.0 terpasang versi 1.0.0)
    let mut installed = HashMap::new();
    installed.insert("widya-http".to_string(), "1.0.0".to_string());
    installed.insert("widya-math".to_string(), "1.0.0".to_string());

    let mut checksums = HashMap::new();
    let math_ck = Wpm::calculate_sha256(b"widya-math-1.0.0");
    let http_ck = Wpm::calculate_sha256(b"widya-http-1.0.0");
    checksums.insert("widya-math".to_string(), math_ck.clone());
    checksums.insert("widya-http".to_string(), http_ck);

    let report = WpmSecurityAuditor::audit_packages(&installed, &checksums, &server);
    assert_eq!(report.total_packages_scanned, 2);
    assert!(!report.is_clean(), "Laporan tidak boleh clean karena ada widya-http v1.0.0 rentan");
    assert!(report.total_vulnerabilities_found >= 1);

    let http_result = report.package_results.iter().find(|p| p.package_name == "widya-http").unwrap();
    assert!(!http_result.vulnerabilities.is_empty());
    assert_eq!(http_result.vulnerabilities[0].advisory_id, "WSA-2026-001");
    assert_eq!(http_result.vulnerabilities[0].severity, VulnerabilitySeverity::Medium);

    // 2. Uji tamper checksum (pemalsuan berkas)
    let mut tampered_checksums = HashMap::new();
    tampered_checksums.insert("widya-math".to_string(), "deadbeef1234567890abcdefdeadbeef1234567890abcdefdeadbeef12345678".to_string());
    let tamper_report = WpmSecurityAuditor::audit_packages(&installed, &tampered_checksums, &server);
    assert!(!tamper_report.all_checksums_valid, "Audit harus mendeteksi manipulasi checksum!");

    // 3. Uji rantai pasok bersih dan mutakhir (widya-http 1.2.0, widya-math 1.0.0)
    let mut clean_installed = HashMap::new();
    clean_installed.insert("widya-http".to_string(), "1.2.0".to_string());
    clean_installed.insert("widya-math".to_string(), "1.0.0".to_string());

    let mut clean_checksums = HashMap::new();
    clean_checksums.insert("widya-math".to_string(), math_ck);
    clean_checksums.insert("widya-http".to_string(), Wpm::calculate_sha256(b"widya-http-1.2.0"));

    let clean_report = WpmSecurityAuditor::audit_packages(&clean_installed, &clean_checksums, &server);
    assert!(clean_report.is_clean(), "Paket mutakhir tanpa celah harus menghasilkan audit clean");
    assert_eq!(clean_report.total_vulnerabilities_found, 0);
    assert!(clean_report.all_checksums_valid);
    assert!(clean_report.all_signatures_verified);
}

#[test]
fn test_wpm_outdated_checker_and_semver_types() {
    use widya::wpm::{WpmRegistryServer, WpmOutdatedChecker};

    let server = WpmRegistryServer::new(8097);

    let mut installed = HashMap::new();
    installed.insert("widya-http".to_string(), "1.0.0".to_string()); // Latest is 1.2.0 -> Minor
    installed.insert("widya-office".to_string(), "1.9.0".to_string()); // Latest is 2.5.0 -> Major
    installed.insert("widya-math".to_string(), "1.0.0".to_string());   // Latest is 1.0.0 -> Up to date

    let outdated = WpmOutdatedChecker::check_outdated(&installed, &server);
    assert_eq!(outdated.len(), 2, "Harus menemukan 2 paket usang");

    let http_out = outdated.iter().find(|o| o.name == "widya-http").unwrap();
    assert_eq!(http_out.current_version, "1.0.0");
    assert_eq!(http_out.latest_version, "1.2.0");
    assert_eq!(http_out.update_type, "Minor");

    let office_out = outdated.iter().find(|o| o.name == "widya-office").unwrap();
    assert_eq!(office_out.current_version, "1.9.0");
    assert_eq!(office_out.latest_version, "2.5.0");
    assert_eq!(office_out.update_type, "Major");

    // Paket mutakhir tidak boleh muncul di daftar usang
    assert!(outdated.iter().all(|o| o.name != "widya-math"));
}

#[test]
fn test_wpm_registry_http_security_advisories_api() {
    use widya::wpm::WpmRegistryServer;

    let server = WpmRegistryServer::new(8098);

    // 1. GET /api/v1/security/advisories
    let (status, ctype, body) = server.handle_http_request("GET /api/v1/security/advisories HTTP/1.1\r\n\r\n");
    assert_eq!(status, 200);
    assert_eq!(ctype, "application/json");
    assert!(body.contains("WSA-2026-001"));
    assert!(body.contains("WSA-2026-002"));
    assert!(body.contains("WSA-2026-003"));

    // 2. GET /api/v1/security/advisories/widya-crypto
    let (status_pkg, _, body_pkg) = server.handle_http_request("GET /api/v1/security/advisories/widya-crypto HTTP/1.1\r\n\r\n");
    assert_eq!(status_pkg, 200);
    assert!(body_pkg.contains("WSA-2026-002"));
    assert!(body_pkg.contains("poly1305"));

    // 3. GET /api/v1/security/advisories/widya-math (tidak ada advisory)
    let (status_clean, _, body_clean) = server.handle_http_request("GET /api/v1/security/advisories/widya-math HTTP/1.1\r\n\r\n");
    assert_eq!(status_clean, 200);
    assert_eq!(body_clean.trim(), "[]");
}

#[test]
fn test_wpm_cli_audit_and_outdated_project_flow() {
    use widya::wpm::{Wpm, WpmConfig, WpmRegistryServer};

    let temp_dir = std::env::temp_dir().join(format!("test_wpm_cli_flow_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let wpm = Wpm::new(WpmConfig::default());
    let _ = wpm.init("audit_test_project", &temp_dir).unwrap();

    // Tambah dependensi mutakhir
    wpm.add_dependency(&temp_dir, "widya-math", "1.0.0").unwrap();

    let server = WpmRegistryServer::new(8099);

    // Audit proyek
    let report = wpm.audit_project(&temp_dir, &server).expect("Audit proyek harus berhasil dijalankan");
    assert_eq!(report.total_packages_scanned, 1);
    assert!(report.is_clean(), "widya-math 1.0.0 harus lolos audit keamanan");

    // Periksa usang
    let outdated = wpm.check_outdated_project(&temp_dir, &server).expect("Pemeriksaan usang harus berhasil");
    assert!(outdated.is_empty(), "widya-math 1.0.0 tidak boleh ditandai usang");

    let _ = fs::remove_dir_all(&temp_dir);
}


