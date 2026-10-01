# 🏛️ Widya-Lang Enterprise Sovereign Stack

Dokumentasi resmi untuk 3 pilar lanjutan kelas enterprise di ekosistem **Widya-Lang**:
1. **WidyaStream**: Distributed Event Streaming & Partitioned Commit Log Engine (*Apache Kafka Equivalent*).
2. **WidyaPass**: Sovereign IAM & WebAuthn / Passkeys Engine (*Keycloak / Supabase Auth Equivalent*).
3. **WidyaTrace**: Sovereign APM & Distributed Tracing Engine (*OpenTelemetry / Datadog Equivalent*).

---

## 1. 🌊 WidyaStream (Distributed Event Streaming Engine)

Engine streaming terdistribusi mandiri dengan partisi berbasis hash, append-only log segments, dan consumer group offset tracking tanpa perlu Zookeeper atau KRaft eksternal.

### Karakteristik & Fitur
- **Partitioned Topics**: Pembagian topik ke dalam N partisi untuk throughput jutaan event/detik.
- **Key-Based Routing**: Deterministic hashing memastikan urutan event dari entitas yang sama (misal `user_id` atau `order_id`) selalu berada di partisi yang sama.
- **Consumer Group Offset Management**: Auto-balancing dan commit offset persisten per grup pembaca.

### Contoh Penggunaan dalam Widya-Lang
```rust
use widya::streaming::{TopicConfig, WidyaStreamBroker};

let broker = WidyaStreamBroker::new();
broker.create_topic(TopicConfig {
    name: "iot-telemetri".to_string(),
    partitions_count: 8,
    retention_ms: 7 * 86400000,
    max_message_size: 5 * 1024 * 1024,
});

// Produce event
let (partition, offset) = broker.produce("iot-telemetri", Some("sensor-42".into()), payload_bytes, headers)?;

// Consume dengan Consumer Group
let events = broker.consume_group("analitik-ai-group", "iot-telemetri", 100)?;
```

---

## 2. 🔐 WidyaPass (Sovereign IAM & WebAuthn / Passkeys)

Sistem Identity & Access Management (IAM) kedaulatan tinggi dengan autentikasi biometrik tanpa kata sandi (*passwordless*) standar FIDO2 / WebAuthn, OAuth 2.0 / OIDC Provider, dan Portal Single Sign-On (SSO).

### Karakteristik & Fitur
- **Passwordless Passkeys (FIDO2/WebAuthn)**: Autentikasi biometrik langsung memanfaatkan Apple Touch ID / Face ID, Windows Hello, atau Android Biometric hardware enclave.
- **OIDC Discovery (`.well-known/openid-configuration`)**: Kompatibel dengan semua client OAuth 2.0 dan OIDC standar industri.
- **Role-Based Access Control (RBAC)**: Manajemen hak akses bertingkat dengan token session terenkripsi.

---

## 3. ⚡ WidyaTrace (Distributed APM & Observability)

Sistem pemantauan kinerja aplikasi (*Application Performance Monitoring*) terdistribusi sesuai standar W3C TraceContext dengan visualisasi *Flamegraph Waterfall* interaktif.

### Karakteristik & Fitur
- **Distributed Context Propagation**: Pelacakan span melintasi gateway, microservice, database, dan serverless edge via `trace_id` dan `span_id`.
- **Latency Percentiles**: Perhitungan otomatis P50, P95, P99, Throughput, dan Error Rate.
- **Interactive Visual Flamegraph**: Halaman dashboard HTML visual untuk inspeksi latensi secara real-time.

---

## 🧪 Validasi Pengujian
Semua pilar telah teruji 100% lulus pada suite pengujian `tests/enterprise_stack_tests.rs`:
- `test_widyastream_partitioned_streaming_and_consumer_groups`
- `test_widyapass_webauthn_passkeys_and_oidc`
- `test_widyatrace_distributed_apm_and_flamegraph`
