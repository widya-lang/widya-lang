// ============================================================================
// WIDYA INDUSTRIAL EMPIRICAL BENCHMARK SUITE (tests/industrial_benchmarks_test.rs)
// ============================================================================
// Mengukur performa empiris dari seluruh pilar industri:
// 1. Pilar 1: Actor Supervision Message Throughput & Panic Recovery Latency
// 2. Pilar 2: WASI Preview 2 Microsecond Cold-Start Snapshot Restoration Latency
// 3. Pilar 3: Hybrid Post-Quantum TLS 1.3 Key Exchange (X25519 + ML-KEM-768) Throughput
// 4. Pilar 4: Model Context Protocol (MCP v2024-11-05) Tool Call Dispatch Latency
// 5. Enterprise Connection Pool High-Contention Acquisition Latency
// ============================================================================

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde_json::json;
use widya::ai::mcp::{McpRequest, McpServer};
use widya::kripto::hsm_pkcs11::HybridPqcEngine;
use widya::kripto::PqcEngine;
use widya::stdlib::actor_supervision::{
    ActorMessage, RestartPolicy, Supervisor, SupervisionStrategy,
};
use widya::wasm_component::EdgeSnapshotEngine;
use widyabase::enterprise_pool::{EnterpriseConnectionPool, MockConnectionFactory, PoolConfig};

#[test]
fn test_benchmark_actor_concurrency_throughput() {
    println!("\n--- [BENCHMARK 1] Actor Supervision Concurrency Throughput ---");
    let supervisor = Supervisor::new(
        "bench-supervisor",
        SupervisionStrategy::OneForOne,
        RestartPolicy::default(),
    );

    let counter = Arc::new(AtomicUsize::new(0));
    let c_clone = counter.clone();

    let worker = supervisor.spawn_actor("throughput-worker", move || {
        let cnt = c_clone.clone();
        Box::new(move |_ctx, msg| {
            if let ActorMessage::Binary(_) = msg {
                cnt.fetch_add(1, Ordering::SeqCst);
            }
            Ok(None)
        })
    });

    let iterations = 10_000;
    let payload = b"BENCHMARK_PAYLOAD_32_BYTES_SIZE!".to_vec();

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = worker.tell(ActorMessage::Binary(payload.clone()));
        let _ = supervisor.dispatch_once(worker.id);
    }
    let elapsed = start.elapsed();

    assert_eq!(counter.load(Ordering::SeqCst), iterations);
    let throughput = (iterations as f64) / elapsed.as_secs_f64();
    println!("  Iterasi Pesan        : {}", iterations);
    println!("  Total Durasi         : {:?}", elapsed);
    println!("  Throughput Eksekusi  : {:.2} pesan/detik (Target: > 50,000 ops/sec)", throughput);
    assert!(throughput > 10_000.0, "Throughput aktor harus melampaui 10,000 ops/sec");
}

#[test]
fn test_benchmark_wasi_snapshot_microsecond_cold_start() {
    println!("\n--- [BENCHMARK 2] WASI 0.2 Microsecond Cold-Start Snapshot ---");
    let snapshot_data = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x02, 0x03, 0x04];
    let snapshot = EdgeSnapshotEngine::create_snapshot(1, &snapshot_data, 0x1000);

    let iterations = 1_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let (restored, _) = EdgeSnapshotEngine::restore_snapshot(&snapshot);
        assert!(!restored.is_empty());
    }
    let elapsed = start.elapsed();
    let avg_latency_us = (elapsed.as_micros() as f64) / (iterations as f64);


    println!("  Iterasi Restorasi    : {}", iterations);
    println!("  Total Durasi         : {:?}", elapsed);
    println!("  Rerata Cold-Start    : {:.3} µs/operasi (Target: < 500 µs)", avg_latency_us);
    assert!(avg_latency_us < 50.0, "Cold-start snapshot harus di bawah 50 mikrodetik");
}

#[test]
fn test_benchmark_hybrid_pqc_key_exchange_throughput() {
    println!("\n--- [BENCHMARK 3] Hybrid Post-Quantum TLS 1.3 Key Exchange ---");
    let classical_secret = [0x55u8; 32];
    let client_keypair = PqcEngine::generate_mlkem768_keypair("bench_seed_hybrid");

    let iterations = 200;
    let start = Instant::now();
    for _ in 0..iterations {
        let enc_res = HybridPqcEngine::hybrid_encapsulate(&classical_secret, &client_keypair.public_key_hex)
            .expect("Enkapsulasi gagal");
        let dec_secret = HybridPqcEngine::hybrid_decapsulate(&classical_secret, &client_keypair.private_key_hex, &enc_res.ciphertext_pqc)
            .expect("Dekapsulasi gagal");
        assert_eq!(enc_res.combined_shared_secret_hex, dec_secret);
    }
    let elapsed = start.elapsed();
    let ops_per_sec = (iterations as f64) / elapsed.as_secs_f64();
    let avg_ms = elapsed.as_millis() as f64 / (iterations as f64);

    println!("  Iterasi Pertukaran   : {}", iterations);
    println!("  Total Durasi         : {:?}", elapsed);
    println!("  Rerata Latensi KEM   : {:.3} ms/siklus", avg_ms);
    println!("  Throughput KEM       : {:.2} siklus lengkap/detik", ops_per_sec);
    assert!(ops_per_sec > 10.0, "Throughput KEM pasca-kuantum harus memadai untuk TLS handshakes");
}

#[test]
fn test_benchmark_mcp_protocol_dispatch_latency() {
    println!("\n--- [BENCHMARK 4] Model Context Protocol (MCP) Tool Call Dispatch ---");
    let server = McpServer::new("bench-mcp", "0.1.0");
    server.register_tool(
        "bench_add",
        "Menjumlahkan dua bilangan integer",
        json!({"type": "object"}),
        |args: serde_json::Value| {
            let a = args.get("a").and_then(|v| v.as_i64()).unwrap_or(0);
            let b = args.get("b").and_then(|v| v.as_i64()).unwrap_or(0);
            Ok((a + b).to_string())
        },
    );

    let raw_request = r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"bench_add","arguments":{"a":40,"b":2}}}"#;

    let iterations = 10_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let resp_str = server.handle_raw_message(raw_request).expect("Gagal dispatch MCP");
        assert!(resp_str.contains("42"));
    }
    let elapsed = start.elapsed();
    let avg_us = (elapsed.as_micros() as f64) / (iterations as f64);
    let throughput = (iterations as f64) / elapsed.as_secs_f64();

    println!("  Iterasi Dispatch     : {}", iterations);
    println!("  Total Durasi         : {:?}", elapsed);
    println!("  Rerata Latensi Tool  : {:.2} µs/call", avg_us);
    println!("  Throughput Dispatch  : {:.2} call/detik", throughput);
    assert!(avg_us < 100.0, "Latensi dispatch perkakas MCP harus di bawah 100 mikrodetik");
}

#[test]
fn test_benchmark_database_connection_pool_contention() {
    println!("\n--- [BENCHMARK 5] Enterprise DB Connection Pool Contention ---");
    let config = PoolConfig {
        max_connections: 10,
        min_connections: 5,
        connection_timeout: Duration::from_millis(1000),
        idle_timeout: Duration::from_secs(60),
        max_lifetime: Duration::from_secs(300),
        health_check_on_checkout: false,
        health_check_query: "PILIH 1;".to_string(),
    };

    let is_healthy = Arc::new(Mutex::new(true));
    let factory = Arc::new(MockConnectionFactory::new(is_healthy));
    let pool = EnterpriseConnectionPool::new(config, factory).expect("Gagal inisialisasi pool");

    let iterations = 20_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let mut conn = pool.acquire().expect("Gagal acquire");
        let _ = conn.execute("SELECT 1;");
    }
    let elapsed = start.elapsed();
    let avg_us = (elapsed.as_micros() as f64) / (iterations as f64);
    let throughput = (iterations as f64) / elapsed.as_secs_f64();

    println!("  Iterasi Checkout     : {}", iterations);
    println!("  Total Durasi         : {:?}", elapsed);
    println!("  Rerata Waktu Acquire : {:.2} µs/transaksi", avg_us);
    println!("  Throughput Pool      : {:.2} checkout-release/detik", throughput);
    assert!(throughput > 50_000.0, "Throughput connection pool harus melampaui 50,000 ops/detik");
}
