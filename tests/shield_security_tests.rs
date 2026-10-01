use std::collections::HashMap;
use widya::shield::{
    WafShield, WafConfig, ThreatType, PostQuantumEnclave, ZeroTrustMeshEngine, SbomGenerator, SbomPackage
};

#[test]
fn test_waf_owasp_top10_and_rate_limiting() {
    let waf = WafShield::new(WafConfig {
        block_mode: true,
        max_requests_per_minute: 5,
        max_payload_size: 1024 * 1024,
    });

    let headers = HashMap::new();

    // 1. Valid request
    assert!(waf.inspect_request("192.168.1.10", "GET", "/api/v1/products", "", &headers).is_ok());

    // 2. SQL Injection Attack
    let sqli_res = waf.inspect_request("192.168.1.10", "POST", "/login", "username=admin' or '1'='1&password=xyz", &headers);
    assert!(sqli_res.is_err());
    assert_eq!(sqli_res.unwrap_err().threat_type, ThreatType::SqlInjection);

    // 3. XSS Attack
    let xss_res = waf.inspect_request("192.168.1.11", "POST", "/comment", "<script>document.cookie</script>", &headers);
    assert!(xss_res.is_err());
    assert_eq!(xss_res.unwrap_err().threat_type, ThreatType::CrossSiteScripting);

    // 4. Path Traversal Attack
    let path_res = waf.inspect_request("192.168.1.12", "GET", "/download?file=../../../../etc/passwd", "", &headers);
    assert!(path_res.is_err());
    assert_eq!(path_res.unwrap_err().threat_type, ThreatType::PathTraversal);

    // 5. SSRF Attack
    let ssrf_res = waf.inspect_request("192.168.1.13", "POST", "/fetch-avatar", "http://169.254.169.254/latest/meta-data/", &headers);
    assert!(ssrf_res.is_err());
    assert_eq!(ssrf_res.unwrap_err().threat_type, ThreatType::ServerSideRequestForgery);

    // 6. Rate Limiting Check
    let attacker_ip = "10.0.0.99";
    for _ in 0..5 {
        assert!(waf.inspect_request(attacker_ip, "GET", "/api/ping", "", &headers).is_ok());
    }
    // 6th request must exceed limit
    let rate_res = waf.inspect_request(attacker_ip, "GET", "/api/ping", "", &headers);
    assert!(rate_res.is_err());
    assert_eq!(rate_res.unwrap_err().threat_type, ThreatType::RateLimitExceeded);
}

#[test]
fn test_post_quantum_cryptography_enclave() {
    let keypair = PostQuantumEnclave::generate_kem_keypair();
    assert_eq!(keypair.algorithm, "ML-KEM-768 (Kyber)");
    assert_eq!(keypair.public_key.len(), 1184);
    assert_eq!(keypair.secret_key.len(), 2400);

    let encapsulation = PostQuantumEnclave::encapsulate(&keypair.public_key);
    assert_eq!(encapsulation.ciphertext.len(), 1088);
    assert_eq!(encapsulation.shared_secret.len(), 32); // 256-bit symmetric shared secret
}

#[test]
fn test_zero_trust_spiffe_workload_identity_and_mtls() {
    let zt_mesh = ZeroTrustMeshEngine::new();

    // Server Workload: Payment Service (only allows orders-svc)
    let payment_svid = zt_mesh.issue_workload_svid(
        "finance",
        "payment-svc",
        vec!["spiffe://widya.cluster/ns/ecommerce/sa/orders-svc".to_string()],
    );
    assert!(payment_svid.x509_svid_cert.contains("finance"));

    // Client Workload A: Authorized Orders Service
    let client_authorized = "spiffe://widya.cluster/ns/ecommerce/sa/orders-svc";
    assert!(zt_mesh.authorize_mTLS_handshake(client_authorized, &payment_svid.spiffe_id));

    // Client Workload B: Unauthorized Analytics Service
    let client_unauthorized = "spiffe://widya.cluster/ns/marketing/sa/analytics-svc";
    assert!(!zt_mesh.authorize_mTLS_handshake(client_unauthorized, &payment_svid.spiffe_id));
}

#[test]
fn test_sbom_spdx_generation() {
    let packages = vec![
        SbomPackage {
            name: "widya-runtime".to_string(),
            version: "1.0.0-beta.1".to_string(),
            supplier: "Sovereign Nusantara Foundation".to_string(),
            license: "Apache-2.0".to_string(),
            sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
        },
        SbomPackage {
            name: "widyabase-engine".to_string(),
            version: "1.0.0-beta.1".to_string(),
            supplier: "Sovereign Nusantara Foundation".to_string(),
            license: "Apache-2.0".to_string(),
            sha256: "ca978112ca1bbdcaf06427e55656462949320d536cf77b19e0b6098311fb4793".to_string(),
        }
    ];

    let spdx_json = SbomGenerator::generate_spdx_json("widya-sovereign-core", packages);
    assert!(spdx_json.contains("SPDX-2.3"));
    assert!(spdx_json.contains("widya-runtime"));
    assert!(spdx_json.contains("widyabase-engine"));
}
