//! Comprehensive Enterprise Industrial Stack Integration Tests for Widya-Lang
//! Menguji OpenTelemetry, Prometheus Exporter, gRPC/Protobuf, CycloneDX SBOM, FIPS/HSM, Parquet/Arrow, dan Transpiler.

use widya::apm::{W3cTraceContext, PrometheusExporter};
use widya::interop::GrpcProtobufEngine;
use widya::security_tools::{SbomGenerator, HsmPkcs11Manager};
use widya::widyabase::{ParquetArrowConnector, ParquetCompression};
use widya::tools::transpiler::{CodeTranspiler, SourceLanguage};

#[test]
fn test_opentelemetry_and_prometheus_telemetry() {
    // 1. OpenTelemetry W3C TraceContext
    let trace_id = "4bf92f3577b34da6a3ce929d0e0e4736";
    let span_id = "00f067aa0ba902b7";
    let ctx = W3cTraceContext::new(trace_id, span_id);
    let header = ctx.to_traceparent_header();
    assert_eq!(header, "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01");

    let parsed = W3cTraceContext::from_traceparent_header(&header).unwrap();
    assert_eq!(parsed.trace_id, trace_id);
    assert_eq!(parsed.parent_id, span_id);

    // 2. Prometheus metrics exporter
    let mut prom = PrometheusExporter::new();
    prom.inc_counter("widya_transactions_total", 1000.0, "Total transaksi");
    prom.set_gauge("widya_memory_usage_mb", 256.0, "Penggunaan memori MB");
    let metrics_text = prom.render_metrics();

    assert!(metrics_text.contains("# TYPE widya_transactions_total counter"));
    assert!(metrics_text.contains("widya_transactions_total 1000"));
    assert!(metrics_text.contains("# TYPE widya_memory_usage_mb gauge"));
    assert!(metrics_text.contains("widya_memory_usage_mb 256"));
}

#[test]
fn test_grpc_protobuf_wire_and_framing() {
    // 1. Protobuf Varint
    let val = 150u64;
    let encoded = GrpcProtobufEngine::encode_varint(val);
    let (decoded, len) = GrpcProtobufEngine::decode_varint(&encoded).unwrap();
    assert_eq!(decoded, 150);
    assert_eq!(len, encoded.len());

    // 2. gRPC frame
    let msg = b"RPC_REQUEST_PAYLOAD";
    let frame = GrpcProtobufEngine::frame_grpc_message(msg, false);
    assert_eq!(frame.len(), 5 + msg.len());

    let (comp, payload) = GrpcProtobufEngine::unframe_grpc_message(&frame).unwrap();
    assert!(!comp);
    assert_eq!(payload, msg);
}

#[test]
fn test_sbom_cyclonedx_and_hsm_fips_compliance() {
    // 1. CycloneDX SBOM
    let deps = vec![
        ("widya-kripto", "1.0.0", "MIT", "d41d8cd98f00b204e9800998ecf8427e"),
        ("widya-widyabase", "1.0.0", "Apache-2.0", "c4ca4238a0b923820dcc509a6f75849b"),
    ];
    let sbom = SbomGenerator::generate_cyclonedx_json("core_banking_engine", "2.0.0", deps);
    assert!(sbom.contains("CycloneDX"));
    assert!(sbom.contains("core_banking_engine"));
    assert!(sbom.contains("widya-kripto"));

    // 2. FIPS 140-3 & HSM Token Session
    assert!(HsmPkcs11Manager::is_fips_approved_algorithm("ML-DSA-87"));
    assert!(HsmPkcs11Manager::is_fips_approved_algorithm("AES-256-GCM"));
    assert!(!HsmPkcs11Manager::is_fips_approved_algorithm("RC4"));

    let session = HsmPkcs11Manager::open_session(1, "123456");
    assert!(session.is_ok());
    assert!(session.unwrap().contains("hsm_session_slot_1"));
}

#[test]
fn test_parquet_and_arrow_bigdata() {
    // 1. Parquet metadata
    let pq_meta = ParquetArrowConnector::create_parquet_metadata(
        &["nik", "nama", "saldo"],
        500_000,
        ParquetCompression::Snappy,
    );
    assert_eq!(pq_meta.num_rows, 500_000);
    assert_eq!(pq_meta.row_groups[0].columns.len(), 3);
    assert_eq!(pq_meta.row_groups[0].columns[0].compression, ParquetCompression::Snappy);

    // 2. Arrow IPC RecordBatch memory alignment
    let arrow_batch = ParquetArrowConnector::create_arrow_record_batch(
        vec![("transaksi_id", "Int64"), ("keterangan", "Utf8")],
        10_000,
    );
    assert_eq!(arrow_batch.length, 10_000);
    assert_eq!(arrow_batch.memory_alignment, 64);
    for offset in arrow_batch.buffer_offsets {
        assert_eq!(offset % 64, 0);
    }
}

#[test]
fn test_code_migration_transpiler() {
    // Python to Widya
    let py_src = "def proses_transaksi(id, nominal):\n    if nominal > 0:\n        return True\n    return False\n";
    let py_res = CodeTranspiler::transpile_python(py_src);
    assert_eq!(py_res.source_lang, SourceLanguage::Python);
    assert!(py_res.transpiled_code.contains("fungsi proses_transaksi(id, nominal):"));
    assert!(py_res.transpiled_code.contains("return benar"));
    assert!(py_res.transpiled_code.contains("return salah"));

    // JavaScript to Widya
    let js_src = "const status = true;\nfunction periksa() {\n    return null;\n}\n";
    let js_res = CodeTranspiler::transpile_javascript(js_src);
    assert_eq!(js_res.source_lang, SourceLanguage::JavaScript);
    assert!(js_res.transpiled_code.contains("tetap status = benar;"));
    assert!(js_res.transpiled_code.contains("fungsi periksa() {"));
    assert!(js_res.transpiled_code.contains("return nihil;"));
}
