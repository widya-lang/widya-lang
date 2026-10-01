// ==============================================================================
// WidyaWeb Cloud Sovereign Edge Platform Test Suite (Vercel-like Ecosystem)
// ==============================================================================

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use widya::web::cloud::{
    get_global_cloud_engine, DashboardControlPlane, DeploymentState, DeploymentTarget,
    EdgeTelemetryEngine, EnvironmentVault, RequestMetric, ScaffoldCloudEngine,
    ServerlessEdgeRunner, ServerlessFunctionEntry, WebVitalsMetric, WidyaCloudEngine,
    WidyaWebConfig,
};

#[test]
fn test_widyaweb_config_parsing() {
    let json_config = r#"{
        "name": "toko-nusantara",
        "version": "2.1.0",
        "framework": "widyaweb",
        "public_dir": "dist",
        "api_dir": "api",
        "redirects": [
            { "source": "/lama", "destination": "/baru", "status": 301 }
        ],
        "rewrites": [
            { "source": "/produk/:id", "destination": "/api/produk" }
        ],
        "crons": [
            { "path": "/api/sync-stok", "schedule": "0 */4 * * *" }
        ],
        "functions": {
            "/api/bayar": {
                "memory_mb": 256,
                "max_duration_sec": 15,
                "runtime": "widya-edge"
            }
        },
        "env": {
            "API_GATEWAY": "https://api.widya.id"
        }
    }"#;

    let config: WidyaWebConfig = serde_json::from_str(json_config).expect("Gagal parse widyaweb.json");
    assert_eq!(config.name, "toko-nusantara");
    assert_eq!(config.version, "2.1.0");
    assert_eq!(config.public_dir, "dist");
    assert_eq!(config.redirects.len(), 1);
    assert_eq!(config.redirects[0].status, 301);
    assert_eq!(config.rewrites.len(), 1);
    assert_eq!(config.crons.len(), 1);
    assert_eq!(config.functions.get("/api/bayar").unwrap().memory_mb, 256);
    assert_eq!(config.env.get("API_GATEWAY").unwrap(), "https://api.widya.id");
}

#[test]
fn test_atomic_project_scaffolding() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widyaweb_test_scaffold_{}", pid));
    let proj_name = "aplikasi-demo-edge";

    let created_path = ScaffoldCloudEngine::init_project(&temp_dir, proj_name)
        .expect("Inisialisasi scaffold proyek harus sukses");

    assert!(created_path.exists());
    assert!(created_path.join("widyaweb.json").exists());
    assert!(created_path.join("public/index.html").exists());
    assert!(created_path.join("api/halo.wya").exists());

    let cfg_content = fs::read_to_string(created_path.join("widyaweb.json")).unwrap();
    assert!(cfg_content.contains(proj_name));

    // Bersihkan
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_atomic_deployment_and_preview_urls() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widyaweb_test_deploy_{}", pid));
    let proj_name = "web-portofolio";

    let proj_dir = ScaffoldCloudEngine::init_project(&temp_dir, proj_name).unwrap();

    // Tambah aset statis ekstra
    fs::write(
        proj_dir.join("public/style.css"),
        "body { background: #000; color: #fff; }",
    )
    .unwrap();

    let engine = WidyaCloudEngine::new(8088);

    // 1. Deploy Preview
    let dep_preview = engine
        .deploy_project(&proj_dir, DeploymentTarget::Preview, None)
        .expect("Deploy preview harus sukses");

    assert_eq!(dep_preview.project_name, proj_name);
    assert_eq!(dep_preview.target, DeploymentTarget::Preview);
    assert_eq!(dep_preview.state, DeploymentState::Ready);
    assert!(dep_preview.id.starts_with("dpl_"));
    assert!(dep_preview.preview_url.contains(&dep_preview.id));
    assert!(dep_preview.files.contains_key("/index.html"));
    assert!(dep_preview.files.contains_key("/style.css"));
    assert!(dep_preview.functions.contains_key("/api/halo"));

    // 2. Deploy Production
    let dep_prod = engine
        .deploy_project(&proj_dir, DeploymentTarget::Production, None)
        .expect("Deploy production harus sukses");

    assert_eq!(dep_prod.target, DeploymentTarget::Production);
    assert!(dep_prod.production_alias.is_some());
    assert_eq!(
        engine.get_active_production_id(proj_name),
        Some(dep_prod.id.clone())
    );

    // Bersihkan
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_zero_downtime_rollback() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widyaweb_test_rollback_{}", pid));
    let proj_name = "sistem-perbankan-nasional";

    let proj_dir = ScaffoldCloudEngine::init_project(&temp_dir, proj_name).unwrap();
    let engine = WidyaCloudEngine::new(8088);

    // Deploy v1 (Production)
    let dep_v1 = engine
        .deploy_project(&proj_dir, DeploymentTarget::Production, None)
        .unwrap();

    // Ubah berkas untuk v2
    fs::write(
        proj_dir.join("public/index.html"),
        "<h1>Versi 2.0.0 Update</h1>",
    )
    .unwrap();

    // Deploy v2 (Production)
    let dep_v2 = engine
        .deploy_project(&proj_dir, DeploymentTarget::Production, None)
        .unwrap();

    assert_ne!(dep_v1.id, dep_v2.id);
    assert_eq!(
        engine.get_active_production_id(proj_name),
        Some(dep_v2.id.clone())
    );

    // Lakukan Rollback Instan ke v1
    let rb_res = engine.rollback_production(proj_name, &dep_v1.id);
    assert!(rb_res.is_ok());
    assert_eq!(
        engine.get_active_production_id(proj_name),
        Some(dep_v1.id.clone())
    );

    // Bersihkan
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_serverless_edge_function_execution() {
    let fn_entry = ServerlessFunctionEntry {
        route_path: "/api/kurs".to_string(),
        file_name: "kurs.wya".to_string(),
        source_code: r#"
            // Simulasi Serverless Endpoint Kurs Mata Uang
            fungsi hitung_kurs() {
                kembalikan {
                    "usd_idr": 16250,
                    "sgd_idr": 12100,
                    "status": "aktif"
                };
            }
            hitung_kurs();
        "#
        .to_string(),
        memory_mb: 128,
        max_duration_sec: 5,
    };

    let headers = HashMap::new();
    let mut query = HashMap::new();
    query.insert("simbol".to_string(), "USD".to_string());
    let mut env_vars = HashMap::new();
    env_vars.insert("KUNCI_API".to_string(), "kunci_rahasia_123".to_string());

    let (status, resp_headers, body) = ServerlessEdgeRunner::execute_function(
        &fn_entry,
        "/api/kurs",
        "GET",
        &headers,
        &query,
        b"",
        &env_vars,
    )
    .expect("Eksekusi serverless harus sukses");

    assert_eq!(status, 200);
    assert!(resp_headers.contains_key("x-widya-edge-fn"));
    let body_str = String::from_utf8_lossy(&body);
    assert!(body_str.contains("sukses"));
    assert!(body_str.contains("/api/kurs"));
}

#[test]
fn test_environment_secrets_vault() {
    let vault = EnvironmentVault::new();
    let proj = "aplikasi-keuangan";

    // Set Secret Preview & Prod
    vault.set_secret(
        proj,
        "DATABASE_URL",
        "postgres://preview_db:5432",
        DeploymentTarget::Preview,
    );
    vault.set_secret(
        proj,
        "DATABASE_URL",
        "postgres://prod_cluster_sovereign:5432",
        DeploymentTarget::Production,
    );
    vault.set_secret(
        proj,
        "JWT_SECRET",
        "rahasia_super_aman_widya_2026",
        DeploymentTarget::Production,
    );

    // Ambil Secret
    let prev_db = vault.get_secret(proj, "DATABASE_URL", DeploymentTarget::Preview);
    let prod_db = vault.get_secret(proj, "DATABASE_URL", DeploymentTarget::Production);

    assert_eq!(prev_db, Some("postgres://preview_db:5432".to_string()));
    assert_eq!(
        prod_db,
        Some("postgres://prod_cluster_sovereign:5432".to_string())
    );

    // List & Remove
    let all_sec = vault.list_secrets(proj);
    assert_eq!(all_sec.len(), 3);

    let removed = vault.remove_secret(proj, "DATABASE_URL", DeploymentTarget::Preview);
    assert!(removed);
    assert_eq!(
        vault.get_secret(proj, "DATABASE_URL", DeploymentTarget::Preview),
        None
    );
}

#[test]
fn test_edge_telemetry_and_web_vitals() {
    let tele = EdgeTelemetryEngine::new();
    let did = "dpl_test123";

    // Catat beberapa request
    for i in 1..=20 {
        tele.record_request(RequestMetric {
            timestamp: 1727700000 + i,
            deployment_id: did.to_string(),
            path: "/".to_string(),
            method: "GET".to_string(),
            status: if i == 20 { 500 } else { 200 },
            duration_ms: (i as f64) * 2.5, // 2.5ms .. 50ms
            bytes_sent: 1024,
            client_ip: "127.0.0.1".to_string(),
            user_agent: "Mozilla/5.0 WidyaAgent".to_string(),
            edge_region: "jkt1".to_string(),
            is_cache_hit: i % 2 == 0,
        });
    }

    // Catat Web Vitals
    tele.record_web_vital(WebVitalsMetric {
        timestamp: 1727700000,
        deployment_id: did.to_string(),
        lcp_ms: 380.0,
        fid_ms: 8.5,
        cls_score: 0.005,
        ttfb_ms: 32.0,
    });

    let agg = tele.aggregate(None, Some(did));
    assert_eq!(agg.total_requests, 20);
    assert_eq!(agg.status_2xx, 19);
    assert_eq!(agg.status_5xx, 1);
    assert_eq!(agg.cache_hit_rate_pct, 50.0);
    assert!(agg.p95_latency_ms > 40.0);
    assert_eq!(agg.avg_lcp_ms, 380.0);
    assert_eq!(agg.avg_fid_ms, 8.5);
}

#[test]
fn test_control_plane_dashboard_generation() {
    let engine = std::sync::Arc::new(WidyaCloudEngine::new(8088));
    let html = DashboardControlPlane::render_html(&engine);

    assert!(html.contains("WidyaWeb Cloud"));
    assert!(html.contains("Sovereign Edge"));
    assert!(html.contains("Real-time Core Web Vitals"));
    assert!(html.contains("Riwayat Deployment Atomik"));
    assert!(html.contains("JKT1 (Jakarta Core Gateway)"));
}
