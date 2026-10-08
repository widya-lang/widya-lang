use std::fs;
use std::path::Path;
use widya::mobile::{
    inisialisasi_proyek_mobile_dengan_template,
    periksa_aplikasi_mobile,
    bangun_aplikasi_mobile,
    generate_pwa_manifest,
    generate_service_worker,
    MobileCompiler,
};

#[test]
fn test_mobile_project_scaffolding_all_templates() {
    let tmp_dir = std::env::temp_dir().join("widya_mobile_test_templates");
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir).unwrap();

    let templates = ["standard", "pos", "presensi", "dashboard"];

    for t in &templates {
        let app_name = format!("App_{}", t);
        let project_path = inisialisasi_proyek_mobile_dengan_template(&app_name, Some(&tmp_dir), t).unwrap();

        assert!(project_path.exists(), "Direktori proyek harus ada");
        assert!(project_path.join("src").join("main.wya").exists(), "main.wya harus ada");
        assert!(project_path.join("widya.json").exists(), "widya.json harus ada");
        assert!(project_path.join("android").exists(), "android scaffolding harus ada");
        assert!(project_path.join("ios").exists(), "ios scaffolding harus ada");

        let code = fs::read_to_string(project_path.join("src").join("main.wya")).unwrap();
        assert!(code.contains("fungsi utama()"));

        // Audit check
        let audit = periksa_aplikasi_mobile(&project_path).unwrap();
        assert!(audit.entry_exists, "Audit: entry harus ditemukan");
        assert!(audit.manifest_exists, "Audit: manifest harus ditemukan");
        assert_eq!(audit.app_name, app_name);
    }

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_mobile_pwa_manifest_and_sw() {
    let manifest = generate_pwa_manifest("TokoKu");
    assert!(manifest.contains("TokoKu"));
    assert!(manifest.contains("standalone"));
    assert!(manifest.contains("portrait-primary"));

    let sw = generate_service_worker();
    assert!(sw.contains("widya-mobile-cache-v1"));
    assert!(sw.contains("addEventListener('install'"));
    assert!(sw.contains("addEventListener('fetch'"));
}

#[test]
fn test_mobile_compilation_targets() {
    let tmp_dir = std::env::temp_dir().join("widya_mobile_test_build");
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir).unwrap();

    let project_path = inisialisasi_proyek_mobile_dengan_template("DemoKasir", Some(&tmp_dir), "pos").unwrap();
    let entry_file = project_path.join("src").join("main.wya");
    let out_dir = tmp_dir.join("dist");

    // Test Webview / PWA compilation
    let web_res = bangun_aplikasi_mobile(&entry_file, Some(&out_dir), "webview");
    assert!(web_res.is_ok(), "Kompilasi webview/PWA harus berhasil: {:?}", web_res);
    assert!(out_dir.join("index.html").exists());
    assert!(out_dir.join("manifest.webmanifest").exists());
    assert!(out_dir.join("sw.js").exists());

    // Test Android APK compilation (package generation)
    let android_res = bangun_aplikasi_mobile(&entry_file, Some(&out_dir), "android");
    assert!(android_res.is_ok(), "Kompilasi Android APK harus berhasil: {:?}", android_res);

    // Test iOS IPA compilation
    let ios_res = bangun_aplikasi_mobile(&entry_file, Some(&out_dir), "ios");
    assert!(ios_res.is_ok(), "Kompilasi iOS IPA harus berhasil: {:?}", ios_res);

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_mobile_crdt_offline_sync() {
    let hub = widya::mobile::get_global_mobile_crdt_hub();
    let doc = hub.create_document("doc_trx_pos", "transaksi_kasir");
    assert_eq!(doc.doc_id, "doc_trx_pos");

    let v1 = hub.mutate_local("doc_trx_pos", "INSERT", "rec_001", serde_json::json!({"item": "Kopi Susu", "qty": 2})).unwrap();
    assert_eq!(v1, 2);

    let sync_res = hub.sync_with_edge("doc_trx_pos", 0).unwrap();
    assert_eq!(sync_res["status"], "CRDT_SYNC_BERHASIL");
    assert_eq!(sync_res["state_vector_lokal"], 2);
}

#[test]
fn test_mobile_ai_liveness_and_ktp_ocr() {
    let npu = widya::mobile::get_global_mobile_npu_engine();
    let live = npu.verify_face_liveness(15000, 1.0);
    assert!(live.is_live_human);
    assert!(live.liveness_score >= 85.0);
    assert!(live.latency_ms < 20.0);

    let ocr = npu.offline_ktp_ocr(20000);
    assert!(ocr.text.contains("KTP ELEKTRONIK"));
    assert_eq!(ocr.detected_nik, Some("3204121508930002".to_string()));
    assert!(ocr.confidence >= 0.95);
}

#[test]
fn test_mobile_softpos_and_ble_mesh() {
    let softpos = widya::mobile::get_global_softpos_engine();
    let tx = softpos.process_tap_transaction(75000.0, "IDR");
    assert_eq!(tx.amount, 75000.0);
    assert_eq!(tx.status, "PEMBAYARAN_NFC_BERHASIL");
    assert!(tx.masked_pan.contains("5210"));

    let ble = widya::mobile::get_global_ble_mesh_engine();
    let pkt = ble.broadcast_message("pos_sync", "SYNC_ACK_TRX", 3);
    assert_eq!(pkt.channel, "pos_sync");
    assert_eq!(pkt.ttl, 3);

    let inbox = ble.get_inbox();
    assert!(!inbox.is_empty());
}

#[test]
fn test_mobile_instant_app_lean_binary_pipeline() {
    let audit = widya::mobile::MobileLeanPipeline::audit_binary_size("KasirMobile");
    assert_eq!(audit.app_name, "KasirMobile");
    assert!(audit.final_bundle_mb < 10.0, "Bundle harus < 10 MB untuk App Clips");
    assert!(audit.app_clip_supported);
    assert!(audit.instant_app_ready);
}

#[test]
fn test_mobile_wearable_tile_and_health_telemetry() {
    let hub = widya::mobile::get_global_wearable_hub();
    let tile = hub.sync_tile("tile_omzet", "Omzet Kasir", "Rp 2.500.000", "IDR");
    assert_eq!(tile.complication_id, "tile_omzet");
    assert_eq!(tile.value, "Rp 2.500.000");

    let telem = hub.read_health_telemetry();
    assert!(telem.heart_rate_bpm > 50);
    assert!(telem.spo2_percent > 90.0);
}

#[test]
fn test_mobile_dynamic_island_live_activity() {
    let hub = widya::mobile::get_global_live_activity_hub();
    let sess = hub.start_activity("order_99", "Pesanan Kopi Susu", "Sedang Dibuat");
    assert_eq!(sess.activity_id, "order_99");
    assert!(sess.is_active);

    let updated = hub.update_activity("order_99", "Siap Diambil", 100.0).unwrap();
    assert_eq!(updated.state_text, "Siap Diambil");
    assert_eq!(updated.progress_percent, 100.0);

    let ended = hub.end_activity("order_99").unwrap();
    assert!(ended);
}

