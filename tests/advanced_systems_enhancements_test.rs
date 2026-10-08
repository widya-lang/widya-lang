use widya::ebpf::{EbpfEmitter, EbpfSafetyReport, EbpfVerifier};
use widya::timetravel_debugger::TimeTravelDebugger;
use widya::zerocopy::{ArrowColumnarBatch, ZeroCopyHeader, ZEROCOPY_MAGIC};
use widya::wpm::{MerkleTransparencyLog, SlsaProvenance};
use widya::dap::DapEngine;
use widya::lsp_enhanced::{WidyaLsp, Position, Range, Diagnostic, DiagnosticSeverity};
use std::collections::HashMap;

#[test]
fn test_ebpf_xdp_packet_filter_emission() {
    let mut emitter = EbpfEmitter::new();
    emitter.add_xdp_rule("TCP", 80, "XDP_DROP");
    emitter.add_xdp_rule("TCP", 443, "XDP_PASS");
    emitter.add_xdp_rule("UDP", 53, "XDP_PASS");

    let c_code = emitter.emit_xdp_c();
    assert!(c_code.contains("SEC(\"xdp\")"));
    assert!(c_code.contains("bpf_ntohs(tcp->dest) == 80"));
    assert!(c_code.contains("return XDP_DROP;"));
    assert!(c_code.contains("bpf_ntohs(tcp->dest) == 443"));
    assert!(c_code.contains("return XDP_PASS;"));
    assert!(c_code.contains("events_ringbuf SEC(\".maps\")"));
}

#[test]
fn test_timetravel_debugger_delta_and_reverse_watchpoint() {
    let mut dbg = TimeTravelDebugger::new();

    // Step 0: Initial state
    let mut vars_0 = HashMap::new();
    vars_0.insert("saldo".to_string(), "1000".to_string());
    vars_0.insert("status".to_string(), "AKTIF".to_string());
    dbg.record_step(10, vars_0, 1000);

    // Step 1: Saldo dikurangi
    let mut vars_1 = HashMap::new();
    vars_1.insert("saldo".to_string(), "500".to_string());
    vars_1.insert("status".to_string(), "AKTIF".to_string());
    dbg.record_step(11, vars_1, 1050);

    // Step 2: Saldo menjadi negatif
    let mut vars_2 = HashMap::new();
    vars_2.insert("saldo".to_string(), "-200".to_string());
    vars_2.insert("status".to_string(), "DEFISIT".to_string());
    dbg.record_step(12, vars_2, 1100);

    assert_eq!(dbg.total_steps(), 3);
    assert_eq!(dbg.current_snapshot().unwrap().variables.get("saldo"), Some(&"-200".to_string()));

    // 1. Reverse Watchpoint: Mundur ke titik terakhir mutasi "saldo" sebelum step 2 -> harus step 1
    let snap_mut = dbg.rewind_to_last_mutation("saldo").expect("Found mutation");
    assert_eq!(snap_mut.step_index, 1);
    assert_eq!(snap_mut.variables.get("saldo"), Some(&"500".to_string()));

    // 2. Reverse Conditional Watchpoint: Mundur ke titik di mana saldo == 1000
    let snap_pred = dbg.rewind_until_predicate(|vars| vars.get("saldo") == Some(&"1000".to_string())).expect("Found condition");
    assert_eq!(snap_pred.step_index, 0);
    assert_eq!(snap_pred.variables.get("status"), Some(&"AKTIF".to_string()));
}

#[test]
fn test_zerocopy_wire_protocol_and_arrow_alignment() {
    let header = ZeroCopyHeader::new(101, 1, 0);
    let payload = b"WIDYA_SOVEREIGN_DMA_PAYLOAD_DATA";

    // Serialisasi dengan CRC32 otomatis
    let serialized = header.serialize_to_buffer(payload);
    assert!(serialized.len() > payload.len());

    // Deserialisasi zero-copy langsung dari slice mentah
    let (parsed_header, parsed_payload) = ZeroCopyHeader::from_raw_slice(&serialized).expect("Parse slice succeeded");
    assert_eq!(parsed_header.magic, ZEROCOPY_MAGIC);
    assert_eq!(parsed_header.message_id, 101);
    assert_eq!(parsed_header.sequence, 1);
    assert_eq!(parsed_payload, payload);

    // Verifikasi Arrow Columnar Record Batch struct
    let arrow_batch = ArrowColumnarBatch {
        row_count: 1000,
        column_count: 8,
        null_bitmap_offset: 64,
        values_offset: 128,
        buffer_length_bytes: 8192,
    };
    assert_eq!(arrow_batch.row_count, 1000);
    assert_eq!(arrow_batch.column_count, 8);
}

#[test]
fn test_wpm_slsa_l3_and_merkle_transparency_log() {
    // 1. SLSA Level 3 Provenance Verification
    let dummy_hash = "a".repeat(64);
    let slsa = SlsaProvenance::new_level3("https://github.com/widya/core", "abcdef123456", &dummy_hash);
    assert!(slsa.verify());
    assert_eq!(slsa.slsa_level, 3);
    assert!(slsa.signed_by_pqc);

    // 2. Merkle Tree Transparency Log
    let mut merkle = MerkleTransparencyLog::new();
    merkle.append_entry("hash_pkg_alpha_1.0.0");
    merkle.append_entry("hash_pkg_beta_2.1.0");
    merkle.append_entry("hash_pkg_gamma_3.0.0");

    assert_eq!(merkle.total_entries(), 3);
    let root = merkle.compute_root();
    assert_eq!(root.len(), 64); // SHA-256 Hex length
}

#[test]
fn test_ebpf_safety_verifier() {
    let mut emitter = EbpfEmitter::new();
    emitter.add_xdp_rule("TCP", 80, "XDP_DROP");
    let c_code = emitter.emit_xdp_c();

    let report = EbpfVerifier::verify_c_code(&c_code);
    assert!(report.is_safe);
    assert!(!report.has_unbounded_loops);
    assert!(report.memory_access_bounded);
    assert!(report.license_valid);
    assert!(report.estimated_instructions <= report.max_allowed_instructions);

    // Kasus tidak aman: kode tanpa lisensi dan ada infinite loop
    let unsafe_c = "int test() { while(1) { } return 0; }";
    let report_unsafe = EbpfVerifier::verify_c_code(unsafe_c);
    assert!(!report_unsafe.is_safe);
    assert!(report_unsafe.has_unbounded_loops);
    assert!(!report_unsafe.license_valid);
}

#[test]
fn test_dap_time_travel_step_back_and_rewind() {
    let mut dap = DapEngine::new();
    let caps = dap.get_dap_capabilities();
    assert_eq!(caps["supportsStepBack"], true);

    // Step 0
    let mut vars_0 = HashMap::new();
    vars_0.insert("x".to_string(), "10".to_string());
    dap.record_execution_step(1, vars_0, 100);

    // Step 1
    let mut vars_1 = HashMap::new();
    vars_1.insert("x".to_string(), "20".to_string());
    dap.record_execution_step(2, vars_1, 200);

    // Step 2
    let mut vars_2 = HashMap::new();
    vars_2.insert("x".to_string(), "30".to_string());
    dap.record_execution_step(3, vars_2, 300);

    assert_eq!(dap.current_line, 3);
    assert_eq!(dap.local_variables.get("x").unwrap().value, "30");

    // 1. Step Back -> kembali ke step 1 (x = 20)
    let snap_back = dap.step_back().expect("Step back succeeded");
    assert_eq!(snap_back.instruction_pointer, 2);
    assert_eq!(dap.current_line, 2);
    assert_eq!(dap.local_variables.get("x").unwrap().value, "20");

    // 2. Rewind to mutation -> mundur ke step 0 (x = 10)
    let snap_rewind = dap.rewind_to_mutation("x").expect("Rewind succeeded");
    assert_eq!(snap_rewind.instruction_pointer, 1);
    assert_eq!(dap.current_line, 1);
    assert_eq!(dap.local_variables.get("x").unwrap().value, "10");
}

#[test]
fn test_lsp_code_actions_quick_fixes() {
    let lsp = WidyaLsp::new();
    let uri = "file:///app/main.wya";

    // 1. Quick Fix untuk kata kunci usang 'var' -> 'misal'
    let src1 = "var total = 100\n";
    let range1 = Range {
        start: Position { line: 1, character: 0 },
        end: Position { line: 1, character: 3 },
    };
    let actions1 = lsp.get_code_actions(uri, range1, &[], src1);
    assert!(!actions1.is_empty());
    assert!(actions1[0].title.contains("misal"));
    let edit1 = actions1[0].edit.as_ref().unwrap();
    assert_eq!(edit1.changes.get(uri).unwrap()[0].new_text, "misal");

    // 2. Quick Fix untuk typo 'fungi' -> 'fungsi'
    let src2 = "fungi hitung() { kembalikan 1; }\n";
    let range2 = Range {
        start: Position { line: 1, character: 0 },
        end: Position { line: 1, character: 5 },
    };
    let actions2 = lsp.get_code_actions(uri, range2, &[], src2);
    assert!(!actions2.is_empty());
    assert!(actions2[0].title.contains("fungsi"));

    // 3. Quick Fix untuk borrow checker diagnostic (tambahkan 'mutabel')
    let src3 = "misal saldo = 500\n";
    let range3 = Range {
        start: Position { line: 1, character: 0 },
        end: Position { line: 1, character: 15 },
    };
    let diags = vec![Diagnostic {
        range: range3,
        message: "Galat: Tidak dapat memutasi karena sedang dipinjam secara imutabel".to_string(),
        severity: DiagnosticSeverity::Error,
        code: None,
        source: Some("widya-borrow-checker".to_string()),
    }];
    let actions3 = lsp.get_code_actions(uri, range3, &diags, src3);
    assert!(!actions3.is_empty());
    assert!(actions3[0].title.contains("mutabel"));
    let edit3 = actions3[0].edit.as_ref().unwrap();
    assert_eq!(edit3.changes.get(uri).unwrap()[0].new_text, "mutabel ");
}
