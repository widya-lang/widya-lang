// ============================================================================
// WIDYA MEGA FRONTIERS INTEGRATION TEST (tests/mega_frontiers_test.rs)
// ============================================================================
// Menguji secara terintegrasi kelima pilar baru:
// 1. AI-Native & Local GGUF/SLM Inference Engine
// 2. Post-Quantum Cryptography (ML-KEM, ML-DSA, ZKP)
// 3. National Fintech Engine (BI-FAST, ISO 20022, Proxy Resolution)
// 4. WASI Component Model (Preview 2, WIT, Serverless Edge)
// 5. Widya Interactive Notebook (.wnb) & Cross-Platform DevEx Builder
// ============================================================================

use widya::ai::{
    build_synthetic_gguf, dequantize_q4_0, parse_gguf_header, vec_dot_q4_0_f32,
    GgufValue, ModelGenerationConfig, SovereignInferenceEngine,
};
use widya::kripto::{PqcEngine, ZkProof};
use widya::fintech::{BiFastRouter, BiFastStatus, BiFastTransferRequest, ProxyType};
use widya::compiler::decimal::Desimal;
use widya::wasm_component::{
    EdgeHttpRequest, ServerlessEdgeRunner, WasmComponentEmitter, WitInterface, WitType,
};
use widya::notebook::{
    BuildTarget, CellExecutionStatus, DevexBuilder, NotebookExecutor, WidyaNotebook,
};
use std::collections::HashMap;

#[test]
fn test_mega_frontiers_five_pillars_integration() {
    // ========================================================================
    // PILAR 1: AI-NATIVE & LOCAL GGUF/SLM INFERENCE ENGINE
    // ========================================================================
    let gguf_bin = build_synthetic_gguf("llama", 4096, 4096, 32, vec![
        ("general.name", GgufValue::String("Widya-SLM-1B-Sovereign".to_string())),
    ]);
    let gguf_info = parse_gguf_header(&gguf_bin).expect("Harus sukses parsing GGUF");
    assert_eq!(gguf_info.architecture, "llama");
    assert_eq!(gguf_info.context_length, 4096);

    let mut q4_block = vec![0u8; 18];
    q4_block[0] = 0x00;
    q4_block[1] = 0x3C; // Scale 1.0 (f16)
    for i in 2..18 { q4_block[i] = 0x88; } // nibbles: 8 - 8 = 0
    let dequant = dequantize_q4_0(&q4_block, 32);
    assert_eq!(dequant.len(), 32);

    let mut slm = SovereignInferenceEngine::new("Widya-SLM-1B", 4096);
    let prompt = slm.format_chatml_prompt("Anda adalah AI Nasional.", "Jelaskan Widya-Lang");
    let mut tokens = Vec::new();
    let reply = slm.generate_streaming(&prompt, &ModelGenerationConfig::default(), |t| {
        tokens.push(t.to_string());
        true
    });
    assert!(!reply.is_empty());
    assert!(!tokens.is_empty());

    // ========================================================================
    // PILAR 2: POST-QUANTUM CRYPTOGRAPHY & ZERO-KNOWLEDGE PROOFS
    // ========================================================================
    // 2A. ML-KEM-768 Kyber Key Encapsulation
    let kem_keys = PqcEngine::generate_mlkem768_keypair("ENTROPI_KEDAULATAN_2026");
    let encap = PqcEngine::encapsulate_mlkem768(&kem_keys.public_key_hex).expect("Enkap ML-KEM sukses");
    let decap_secret = PqcEngine::decapsulate_mlkem768(&encap.ciphertext_hex, &kem_keys.private_key_hex).expect("Dekap sukses");
    assert_eq!(decap_secret.len(), 64);

    // 2B. ML-DSA-87 Dilithium Signature
    let dsa_keys = PqcEngine::generate_mldsa87_keypair("ENTROPI_SERTIFIKAT_NASIONAL");
    let payload = b"DOKUMEN_TRANSAKSI_INTERNASIONAL_BERDAULAT";
    let sig = PqcEngine::sign_mldsa(payload, &dsa_keys.private_key_hex);
    assert!(PqcEngine::verify_mldsa(payload, &sig, &dsa_keys.public_key_hex));

    // 2C. Zero-Knowledge Proof (Bukti NIK terdaftar tanpa membocorkan NIK)
    let umur = 25u64;
    let ambang = 17u64;
    let zkp = PqcEngine::prove_zk_threshold("usia_pemilih", umur, ambang, "SALT_ACAK_987").expect("ZKP sukses");
    assert!(PqcEngine::verify_zk_proof(&zkp));

    // ========================================================================
    // PILAR 3: NATIONAL FINTECH ENGINE (BI-FAST & ISO 20022)
    // ========================================================================
    let bifast = BiFastRouter::new();
    let proxy_target = bifast.resolve_proxy("081234567890").expect("Proksi HP harus ditemukan");
    assert_eq!(proxy_target.account_holder_name, "Dr. Budi Santoso");

    let req_transfer = BiFastTransferRequest {
        end_to_end_id: "BIFAST-20261007-999".to_string(),
        sender_account: "8888999900".to_string(),
        sender_name: "Ahmad Dahlan".to_string(),
        sender_bank_bic: "WIDYIDJA".to_string(),
        target_proxy_or_account: "081234567890".to_string(),
        amount: Desimal::dari_int(2_500_000),
        purpose_code: "TRANSFER".to_string(),
    };

    let receipt = bifast.execute_transfer(req_transfer).expect("Setelmen BI-FAST harus sukses");
    assert_eq!(receipt.status, BiFastStatus::AcceptedSettled);
    assert_eq!(receipt.status_code, "ACTC");
    assert_eq!(receipt.total_fee, Desimal::dari_int(2500));
    assert!(receipt.iso_pacs008_xml.contains("2500000"));

    // ========================================================================
    // PILAR 4: WASI COMPONENT MODEL (PREVIEW 2) & SERVERLESS EDGE
    // ========================================================================
    let iface = WitInterface::wasi_http_incoming_handler();
    let wit_text = iface.emit_wit_file();
    assert!(wit_text.contains("package wasi:http@0.2.0;"));

    let emitter = WasmComponentEmitter::new(iface, Vec::new());
    let comp_bytes = emitter.emit_component_binary();
    assert_eq!(&comp_bytes[0..4], &[0x00, 0x61, 0x73, 0x6D]); // \0asm
    assert_eq!(&comp_bytes[4..8], &[0x0D, 0x00, 0x01, 0x00]); // Component layer version

    let edge_req = EdgeHttpRequest {
        method: "GET".to_string(),
        uri: "/edge/health".to_string(),
        headers: HashMap::new(),
        body: Vec::new(),
    };
    let edge_res = ServerlessEdgeRunner::execute_handler(&comp_bytes, edge_req).expect("Eksekusi serverless edge sukses");
    assert_eq!(edge_res.status, 200);

    // ========================================================================
    // PILAR 5: WIDYA INTERACTIVE NOTEBOOK (.wnb) & DEVEX BUILDER
    // ========================================================================
    let mut notebook = WidyaNotebook::new("Riset Kedaulatan Digital", "Tim Arsitek Widya");
    notebook.add_markdown_cell("cell_1", "# Analisis Ketahanan Siber Nasional");
    notebook.add_code_cell("cell_2", "misal cadangan_devisa = 150000000;");
    notebook.add_sql_cell("cell_3", "PILIH * DARI cadangan_devisa;");
    notebook.add_chart_cell("cell_4", "line", vec![(1.0, 10.0), (2.0, 25.0), (3.0, 45.0)]);

    let mut executor = NotebookExecutor::new();
    executor.execute_all(&mut notebook);

    assert_eq!(notebook.cells[1].status, CellExecutionStatus::Success);
    assert!(notebook.cells[1].output_text.as_ref().unwrap().contains("cadangan_devisa"));

    let json_export = notebook.to_wnb_json().expect("Ekspor .wnb berhasil");
    assert!(json_export.contains("Riset Kedaulatan Digital"));

    let html_report = notebook.export_standalone_html();
    assert!(html_report.contains("Widya Notebook"));

    // Uji DevEx Builder
    let apk = DevexBuilder::build_project("WidyaSuperApp", BuildTarget::AndroidApk).expect("Build APK sukses");
    assert_eq!(apk.target, BuildTarget::AndroidApk);
    assert!(apk.output_file_name.ends_with(".apk"));
    assert!(apk.is_signed);

    let wasm_edge = DevexBuilder::build_project("WidyaEdgeWorker", BuildTarget::WasmEdgeServerless).expect("Build WASM sukses");
    assert_eq!(wasm_edge.target, BuildTarget::WasmEdgeServerless);
    assert!(wasm_edge.output_file_name.ends_with(".wasm"));
}

#[test]
fn test_mega_frontiers_contoh_scripts() {
    let contoh_files = [
        "contoh/210_ai_native_gguf_slm_inference.wya",
        "contoh/211_post_quantum_cryptography_dan_zkp.wya",
        "contoh/212_national_fintech_bifast_iso20022.wya",
        "contoh/213_wasi_component_serverless_edge.wya",
        "contoh/214_interactive_notebook_dan_crossplatform_packager.wya",
        "contoh/215_automotive_canbus_obd2_telemetry.wya",
        "contoh/216_telecom_oran_upf_slicing_sla.wya",
        "contoh/217_iiot_scada_opcua_modbus_tsn.wya",
        "contoh/218_healthtech_fhir_dicom_genomics.wya",
        "contoh/219_quantum_computing_qir_vqe_sim.wya",
    ];

    for file_path in &contoh_files {
        let code = std::fs::read_to_string(file_path)
            .unwrap_or_else(|e| panic!("Gagal membaca {}: {}", file_path, e));
        let result = widya::jalankan(&code);
        assert!(result.is_ok(), "Gagal mengeksekusi {}: {:?}", file_path, result.err());
    }
}

#[test]
fn test_mega_frontiers_pillars_six_to_ten_integration() {
    // ========================================================================
    // PILAR 6: OTOMOTIF, ROBOTIKA OTONOM, & AVIONIK (CAN BUS / OBD-II)
    // ========================================================================
    let std_frame = widya::automotive::CanFrame::new_standard(0x7E8, &[0x04, 0x41, 0x0C, 0x1F, 0x40, 0x00, 0x00, 0x00]);
    assert_eq!(std_frame.id, 0x7E8);
    assert!(!std_frame.is_extended);
    assert_eq!(std_frame.dlc, 8);

    let ext_frame = widya::automotive::CanFrame::new_extended(0x18DAF110, &[0x02, 0x01, 0x00]);
    assert!(ext_frame.is_extended);
    assert_eq!(ext_frame.dlc, 3);

    let filter = widya::automotive::CanFilter::new(0x7E8, 0x7F8);
    assert!(filter.matches(&std_frame));

    let decoded_rpm = widya::automotive::dekode_obd2_can_frame(&std_frame);
    assert_eq!(decoded_rpm, Some(("ENGINE_RPM", 2000.0)));

    let speed_frame = widya::automotive::CanFrame::new_standard(0x7E8, &[0x03, 0x41, 0x0D, 0x64, 0x00, 0x00, 0x00, 0x00]);
    let decoded_speed = widya::automotive::dekode_obd2_can_frame(&speed_frame);
    assert_eq!(decoded_speed, Some(("VEHICLE_SPEED_KMH", 100.0)));

    // ========================================================================
    // PILAR 7: TELEKOMUNIKASI 5G/6G & OPEN-RAN EDGE (UPF / GTP-U / SLICING)
    // ========================================================================
    let gtp_pkt = widya::telecom::GtpPacket::new_t_pdu(1001, 9, vec![0x45, 0x00, 0x00, 0x28]);
    let gtp_bytes = gtp_pkt.serialize_bytes();
    assert!(!gtp_bytes.is_empty());
    assert_eq!(gtp_pkt.header.message_type, widya::telecom::GtpMessageType::TPdu);

    let mut upf = widya::telecom::OpenRanUpf::new("UPF-JKT-01", "10.0.0.1");
    let session_id = upf.create_session("001010123456789", "internet", 9, 1000);
    assert!(session_id >= 1000);
    let fwd_ok = upf.forward_packet(session_id, 1420);
    assert!(fwd_ok.is_ok());

    let report = upf.generate_throughput_report();
    assert_eq!(report.total_active_sessions, 1);
    assert_eq!(report.total_packets_forwarded, 1);

    let mut slice_mgr = widya::telecom::NetworkSliceManager::new();
    let is_urllc_ok = slice_mgr.evaluate_sla(2, 0.8, 0.000005, 50.0, 10);
    assert!(is_urllc_ok);

    let mut mec = widya::telecom::MecPlatform::new("MEC-PLATFORM-01", "id-jkt-1");
    mec.register_app("APP-VAR-01", "VideoAnalytics", "edge-zone-a", 8, 16384);
    assert_eq!(mec.apps.len(), 1);

    // ========================================================================
    // PILAR 8: MANUFAKTUR CERDAS, IOT INDUSTRI, & PROTOKOL SCADA/ICS
    // ========================================================================
    let test_crc_payload = [0x01, 0x03, 0x00, 0x00, 0x00, 0x0A];
    let crc16 = widya::iiot::calculate_crc16_modbus(&test_crc_payload);
    assert_ne!(crc16, 0);

    let mb_frame = widya::iiot::ModbusTcpFrame::new_read_holding_registers(1, 1, 100, 5);
    let mb_bytes = mb_frame.to_bytes();
    assert_eq!(mb_bytes.len(), 12);
    let parsed_mb = widya::iiot::ModbusTcpFrame::parse_bytes(&mb_bytes).expect("Modbus frame harus valid");
    assert_eq!(parsed_mb.function_code, widya::iiot::ModbusFunctionCode::ReadHoldingRegisters);
    assert_eq!(parsed_mb.address, 100);

    let mut opc = widya::iiot::OpcUaServer::new("opc.tcp://127.0.0.1:4840", "WidyaPlcServer");
    opc.add_variable(1, "TurbineSpeed", "KecepatanTurbin", widya::iiot::OpcUaDataType::Float, "3000.5");
    let read_var = opc.read_variable(1, "TurbineSpeed").expect("Variabel OPC UA harus ditemukan");
    assert_eq!(read_var.value_string, "3000.5");

    let mut tas = widya::iiot::TsnTimeAwareShaper::new("eth0", 100_000);
    tas.add_gate_slot(0b00000001, 20_000); // slot 1: queue 0 open
    tas.add_gate_slot(0b11111110, 80_000); // slot 2: queues 1-7 open
    assert!(tas.is_queue_open(0, 10_000));
    assert!(!tas.is_queue_open(1, 10_000));
    assert!(tas.is_queue_open(1, 30_000));

    // ========================================================================
    // PILAR 9: BIOINFORMATIKA, KESEHATAN DIGITAL, & STANDAR MEDIS INTERNASIONAL
    // ========================================================================
    let fhir_patient = widya::healthtech::FhirPatient::new_satusehat(
        "3171012345670001",
        "Ahmad Santoso",
        "male",
        "1990-05-15",
        "081234567890",
    );
    assert_eq!(fhir_patient.resource_type, "Patient");
    assert_eq!(fhir_patient.identifier[0].value, "3171012345670001");

    let mut synth_dicom = vec![0u8; 132];
    synth_dicom[128..132].copy_from_slice(b"DICM");
    let dicom_res = widya::healthtech::parse_dicom_bytes(&synth_dicom).expect("DICOM sintetis harus valid");
    assert!(dicom_res.is_valid_dicom);

    let aligner = widya::healthtech::SequenceAligner::new();
    let alignment = aligner.align_dna("ACACACTA", "AGCACACA");
    assert!(alignment.max_score > 0);
    assert!(!alignment.aligned_query.is_empty());

    let vcf_line = "chr1\t123456\trs789\tA\tG\t99.0\tPASS\tINFO";
    let variant = widya::healthtech::VcfParser::parse_vcf_line(vcf_line).expect("VCF harus ter-parse");
    assert_eq!(variant.chromosome, "chr1");
    assert_eq!(variant.position_bp, 123456);

    // ========================================================================
    // PILAR 10: QUANTUM COMPUTING & QUANTUM HYBRID ALGORITHMS (QIR/QASM)
    // ========================================================================
    let qasm_src = "OPENQASM 3.0;\nqreg q[2];\ncreg c[2];\nh q[0];\ncx q[0], q[1];\nmeasure q[0] -> c[0];\n";
    let circuit = widya::quantum::QuantumCircuit::parse_qasm(qasm_src).expect("QASM parsing harus sukses");
    assert_eq!(circuit.num_qubits, 2);
    assert_eq!(circuit.gates.len(), 3);

    let qir = widya::quantum::QirEmitter::emit_llvm_qir(&circuit, "widya_bell_state");
    assert!(qir.contains("@__quantum__qis__h__body"));
    assert!(qir.contains("@__quantum__qis__cnot__body"));

    let mut qsim = widya::quantum::QuantumStateVector::new(2);
    qsim.apply_hadamard(0);
    qsim.apply_cx(0, 1);
    let probs = qsim.get_probabilities();
    assert!((probs[0] - 0.5).abs() < 1e-4);
    assert!((probs[3] - 0.5).abs() < 1e-4);

    let vqe = widya::quantum::VqeOptimizer::new(2, 0.05, 10);
    let energy = vqe.evaluate_energy(&[0.5, 0.5]);
    assert!(energy.is_finite());
}
