// ============================================================================
// WIDYA INTEGRATION TEST: ENTERPRISE MICROSERVICE BLUEPRINT
// ============================================================================
// Menguji integrasi end-to-end dari:
// 1. Enterprise Database Connection Pool (widyabase::enterprise_pool)
// 2. Prometheus Metrics & OpenTelemetry Tracing
// 3. Post-Quantum Cryptography (ML-DSA-87 Signing & Verification)
// 4. Actor Supervision Tree & Isolated Panic Boundaries
// ============================================================================

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use widya::kripto::PqcEngine;
use widya::observability::prometheus_exporter::PrometheusRegistry;
use widya::observability::tracing::Tracer;
use widya::observability::ObservabilityConfig;
use widya::stdlib::actor_supervision::{
    ActorMessage, RestartPolicy, Supervisor, SupervisionStrategy,
};
use widyabase::enterprise_pool::{EnterpriseConnectionPool, MockConnectionFactory, PoolConfig};

#[test]
fn test_enterprise_microservice_full_pipeline() {
    // ------------------------------------------------------------------------
    // 1. Inisialisasi Enterprise Connection Pool
    // ------------------------------------------------------------------------
    let config = PoolConfig {
        max_connections: 5,
        min_connections: 2,
        connection_timeout: Duration::from_millis(500),
        idle_timeout: Duration::from_secs(60),
        max_lifetime: Duration::from_secs(300),
        health_check_on_checkout: true,
        health_check_query: "PILIH 1;".to_string(),
    };

    let is_healthy = Arc::new(Mutex::new(true));
    let factory = Arc::new(MockConnectionFactory::new(is_healthy));
    let pool = EnterpriseConnectionPool::new(config, factory)
        .expect("Gagal membuat Enterprise Connection Pool");

    let metrics_init = pool.metrics();
    assert_eq!(metrics_init.idle_connections, 2);

    // Ambil koneksi dan jalankan query simulasi
    {
        let mut conn = pool.acquire().expect("Gagal mengambil koneksi");
        let res = conn.execute("INSERT INTO audit_log (status) VALUES ('DIPROSES');");
        assert!(res.is_ok(), "Query harus sukses dieksekusi");
    }
    assert_eq!(pool.metrics().idle_connections, 2);

    // ------------------------------------------------------------------------
    // 2. Inisialisasi Pengawasan Aktor Latar Belakang (Panic-Isolated Actor)
    // ------------------------------------------------------------------------
    let audit_count = Arc::new(AtomicUsize::new(0));
    let audit_count_clone = audit_count.clone();

    let supervisor = Supervisor::new(
        "microservice-supervisor",
        SupervisionStrategy::OneForOne,
        RestartPolicy::default(),
    );

    let audit_worker = supervisor.spawn_actor("audit-worker", move || {
        let counter = audit_count_clone.clone();
        Box::new(move |_ctx, msg| {
            if let ActorMessage::Binary(data) = msg {
                counter.fetch_add(1, Ordering::SeqCst);
                if data == b"PANIC_TEST" {
                    panic!("Simulasi crash pada aktor audit");
                }
            }
            Ok(None)
        })
    });

    // ------------------------------------------------------------------------
    // 3. Autentikasi Payload dengan Tanda Tangan Pasca-Kuantum (ML-DSA-87)
    // ------------------------------------------------------------------------
    let keypair = PqcEngine::generate_mldsa87_keypair("service_master_seed");
    let payload = b"POST /api/v1/transaksi {\"rekening\": \"ID-9921\", \"jumlah\": 5000000}";
    let signature = PqcEngine::sign_mldsa(payload, &keypair.private_key_hex);

    // Verifikasi tanda tangan
    let is_valid = PqcEngine::verify_mldsa(payload, &signature, &keypair.public_key_hex);
    assert!(is_valid, "Verifikasi tanda tangan pasca-kuantum harus sukses");

    // ------------------------------------------------------------------------
    // 4. Observabilitas: Distributed Tracing & Prometheus Metrics
    // ------------------------------------------------------------------------
    let obs_config = ObservabilityConfig::default();
    let tracer = Tracer::new(&obs_config).expect("Gagal membuat Tracer");
    let span = tracer.start_span("handle_transaction");

    let registry = PrometheusRegistry::new();
    registry.register_counter("http_requests_total", "Total request HTTP", &[("handler", "transaksi"), ("status", "200")], 1.0);
    registry.collect_from_db_pool("main_db", &pool.metrics());

    // Kirim event audit ke aktor
    let _ = audit_worker.tell(ActorMessage::Binary(payload.to_vec()));
    let dispatched = supervisor.dispatch_once(audit_worker.id).expect("Gagal dispatch");
    assert!(dispatched);
    assert_eq!(audit_count.load(Ordering::SeqCst), 1);

    // Akhiri span pelacakan
    tracer.end_span(span.id, widya::observability::tracing::SpanStatus::Ok).expect("Gagal mengakhiri span");

    // Verifikasi ekspor metrik Prometheus
    let metrics_text = registry.render();
    assert!(metrics_text.contains("http_requests_total"));
    assert!(metrics_text.contains("widyabase_pool_active_connections"));

    // ------------------------------------------------------------------------
    // 5. Verifikasi Panic Boundary Aktor (Fault Tolerance)
    // ------------------------------------------------------------------------
    // Kirim pesan panic ke worker
    let _ = audit_worker.tell(ActorMessage::Binary(b"PANIC_TEST".to_vec()));
    let panicked = supervisor.dispatch_once(audit_worker.id).expect("Gagal dispatch panic");
    assert!(panicked, "Aktor harus berhasil dipulihkan dari panic");

    // Kirim pesan normal berikutnya: aktor harus sudah dipulihkan dan melayani pesan
    let _ = audit_worker.tell(ActorMessage::Binary(b"NORMAL_MESSAGE".to_vec()));
    let handled_again = supervisor.dispatch_once(audit_worker.id).expect("Gagal dispatch pesan berikutnya");
    assert!(handled_again);
    assert_eq!(audit_count.load(Ordering::SeqCst), 3);
}

