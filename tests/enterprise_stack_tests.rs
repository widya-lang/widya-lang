use std::collections::HashMap;
use widya::streaming::{TopicConfig, WidyaStreamBroker};
use widya::iam::SovereignIamEngine;
use widya::apm::{Span, SpanStatus, SovereignApmEngine};

#[test]
fn test_widyastream_partitioned_streaming_and_consumer_groups() {
    let broker = WidyaStreamBroker::new();
    let config = TopicConfig {
        name: "transaksi-pembayaran".to_string(),
        partitions_count: 3,
        retention_ms: 86400000,
        max_message_size: 1024 * 1024,
    };
    
    assert!(broker.create_topic(config));
    assert_eq!(broker.list_topics(), vec!["transaksi-pembayaran"]);

    // Produce messages with keys
    let mut headers = HashMap::new();
    headers.insert("origin".to_string(), "widya-pos".to_string());

    let (p1, off1) = broker.produce("transaksi-pembayaran", Some("user_101".to_string()), b"PAYMENT_SUCCESS_1".to_vec(), headers.clone()).unwrap();
    let (p2, off2) = broker.produce("transaksi-pembayaran", Some("user_102".to_string()), b"PAYMENT_SUCCESS_2".to_vec(), headers.clone()).unwrap();
    let (p3, off3) = broker.produce("transaksi-pembayaran", Some("user_101".to_string()), b"PAYMENT_SUCCESS_3".to_vec(), headers).unwrap();

    // Same key should route to the exact same partition
    assert_eq!(p1, p3);
    assert_eq!(off3, off1 + 1);

    // Test consumer group
    let records = broker.consume_group("order-processor-group", "transaksi-pembayaran", 10).unwrap();
    assert_eq!(records.len(), 3);

    // Polling again should yield 0 because offsets were committed
    let empty_records = broker.consume_group("order-processor-group", "transaksi-pembayaran", 10).unwrap();
    assert_eq!(empty_records.len(), 0);
}

#[test]
fn test_widyapass_webauthn_passkeys_and_oidc() {
    let iam = SovereignIamEngine::new("https://auth.widya.id");
    
    // Register user
    let user = iam.register_user("budi_santoso", "budi@widya.id", vec!["developer", "admin"]).unwrap();
    assert_eq!(user.username, "budi_santoso");

    // Create WebAuthn Challenge
    let challenge = iam.create_webauthn_challenge(Some(user.id.clone()));
    assert_eq!(challenge.challenge_bytes.len(), 32);

    // Register Passkey
    let passkey = iam.register_passkey(
        &user.id,
        "cred_macbook_touchid_9988",
        "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0...\n-----END PUBLIC KEY-----",
        "MacBook Pro TouchID",
    ).unwrap();
    assert_eq!(passkey.device_name, "MacBook Pro TouchID");

    // Authenticate with Passkey
    let signature = vec![0x12, 0x34, 0x56, 0x78]; // simulated hardware biometric enclave sig
    let session = iam.authenticate_passkey(&challenge.challenge_id, "cred_macbook_touchid_9988", &signature).unwrap();
    
    assert_eq!(session.username, "budi_santoso");
    assert!(session.access_token.starts_with("wya_at_"));
    assert!(session.roles.contains(&"admin".to_string()));

    // OpenID Discovery document
    let oidc_doc = iam.get_openid_configuration();
    assert!(oidc_doc.contains("https://auth.widya.id/oauth/authorize"));
    assert!(oidc_doc.contains("passkeys_supported"));
}

#[test]
fn test_widyatrace_distributed_apm_and_flamegraph() {
    let apm = SovereignApmEngine::new();
    let trace_id = SovereignApmEngine::generate_trace_id();

    // Span 1: Gateway
    let span_gw = Span {
        trace_id: trace_id.clone(),
        span_id: SovereignApmEngine::generate_span_id(),
        parent_span_id: None,
        name: "HTTP POST /api/v1/orders".to_string(),
        service_name: "widya-gateway".to_string(),
        start_time_us: 1000,
        duration_us: 45000, // 45ms
        status: SpanStatus::Ok,
        attributes: HashMap::new(),
    };
    apm.record_span(span_gw.clone());

    // Span 2: Order Service (Child)
    let span_order = Span {
        trace_id: trace_id.clone(),
        span_id: SovereignApmEngine::generate_span_id(),
        parent_span_id: Some(span_gw.span_id.clone()),
        name: "CreateOrderCommand".to_string(),
        service_name: "order-service".to_string(),
        start_time_us: 5000,
        duration_us: 35000, // 35ms
        status: SpanStatus::Ok,
        attributes: HashMap::new(),
    };
    apm.record_span(span_order.clone());

    // Span 3: DB Query (Child)
    let span_db = Span {
        trace_id: trace_id.clone(),
        span_id: SovereignApmEngine::generate_span_id(),
        parent_span_id: Some(span_order.span_id),
        name: "INSERT INTO orders VALUES (...)".to_string(),
        service_name: "widyabase-cluster".to_string(),
        start_time_us: 10000,
        duration_us: 12000, // 12ms
        status: SpanStatus::Ok,
        attributes: HashMap::new(),
    };
    apm.record_span(span_db);

    // Validate metrics
    let (count, p50, p95, error_rate) = apm.calculate_metrics("widya-gateway");
    assert_eq!(count, 1);
    assert_eq!(p50, 45000);
    assert_eq!(error_rate, 0.0);

    // Validate Flamegraph HTML output
    let html = apm.generate_flamegraph_html(&trace_id).unwrap();
    assert!(html.contains("WidyaTrace Flamegraph"));
    assert!(html.contains("HTTP POST /api/v1/orders"));
    assert!(html.contains("widyabase-cluster"));
}
