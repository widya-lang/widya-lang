// tests/multi_camera_bandwidth_test.rs
// ==============================================================================
// TEST SUITE: MULTI-CAMERA & BANDWIDTH OPTIMIZATION (ARENA SILAT SAAS)
// 100% In-House Sovereign Code powered by Widya-Lang Ecosystem
// ==============================================================================

#[path = "../Projects/arenasilat/src/multi_camera_and_bandwidth.rs"]
mod multi_camera_and_bandwidth;

use multi_camera_and_bandwidth::{
    MultiCameraCourtManager, CameraAngle, CameraSourceType, BandwidthStrategy,
};

#[test]
fn test_multi_camera_registration_and_qr_tokens() {
    let mut manager = MultiCameraCourtManager::baru("TENANT_IPSI_JATENG", "KEJURNAS_2026", "192.168.1.100");

    // Daftarkan 4 Sudut Kamera untuk Gelanggang 1 (Matras A)
    let cam1 = manager.daftarkan_kamera(
        1,
        CameraAngle::Sudut1DepanKiri,
        CameraSourceType::WebRtcSmartphone { device_model: "Samsung S23 Ultra".to_string(), battery_percent: 95 },
        60,
        3500,
    );

    let cam2 = manager.daftarkan_kamera(
        1,
        CameraAngle::Sudut2DepanKanan,
        CameraSourceType::WebRtcSmartphone { device_model: "iPhone 15 Pro".to_string(), battery_percent: 88 },
        60,
        3500,
    );

    let cam3 = manager.daftarkan_kamera(
        1,
        CameraAngle::Sudut3BelakangKiri,
        CameraSourceType::NdiHxBroadcast { ip_address: "192.168.1.50".to_string(), stream_channel: "CAM3_BL".to_string() },
        60,
        3500,
    );

    let cam4 = manager.daftarkan_kamera(
        1,
        CameraAngle::Sudut4BelakangKanan,
        CameraSourceType::UsbDirectCapture { device_id: "USB_CAM_CAPTURE_01".to_string() },
        60,
        3500,
    );

    assert_eq!(cam1, "CAM-G1-CAM_1_FL");
    assert_eq!(cam2, "CAM-G1-CAM_2_FR");
    assert_eq!(cam3, "CAM-G1-CAM_3_BL");
    assert_eq!(cam4, "CAM-G1-CAM_4_BR");

    let cameras_g1 = manager.ambil_kamera_gelanggang(1);
    assert_eq!(cameras_g1.len(), 4);
    assert!(cameras_g1.iter().any(|c| c.webrtc_ingest_url.contains("WIDYA_CAM-G1-CAM_1_FL")));
}

#[test]
fn test_bandwidth_optimization_strategies() {
    let mut manager = MultiCameraCourtManager::baru("TENANT_IPSI_JATENG", "KEJURNAS_2026", "192.168.1.100");

    // Daftarkan 4 Kamera per Gelanggang untuk 6 Gelanggang (Total 24 Kamera)
    for court in 1..=6 {
        manager.daftarkan_kamera(court, CameraAngle::Sudut1DepanKiri, CameraSourceType::WebRtcSmartphone { device_model: "HP1".to_string(), battery_percent: 90 }, 60, 3500);
        manager.daftarkan_kamera(court, CameraAngle::Sudut2DepanKanan, CameraSourceType::WebRtcSmartphone { device_model: "HP2".to_string(), battery_percent: 90 }, 60, 3500);
        manager.daftarkan_kamera(court, CameraAngle::Sudut3BelakangKiri, CameraSourceType::WebRtcSmartphone { device_model: "HP3".to_string(), battery_percent: 90 }, 60, 3500);
        manager.daftarkan_kamera(court, CameraAngle::Sudut4BelakangKanan, CameraSourceType::WebRtcSmartphone { device_model: "HP4".to_string(), battery_percent: 90 }, 60, 3500);
    }

    assert_eq!(manager.cameras.len(), 24);

    // 1. Direct Multi-Channel (Tanpa Optimasi) -> 24 * 3500 = 84,000 Kbps (84 Mbps)
    manager.active_strategy = BandwidthStrategy::DirectMultiChannel;
    let unoptimized = manager.hitung_total_bandwidth_upload_kbps();
    assert_eq!(unoptimized, 84000);

    // 2. Local Edge Ring Buffer (VAR Lokal) -> 6 Gelanggang * 2500 Kbps = 15,000 Kbps (15 Mbps)
    manager.active_strategy = BandwidthStrategy::LocalEdgeRingBufferOnly;
    let local_edge_kbps = manager.hitung_total_bandwidth_upload_kbps();
    assert_eq!(local_edge_kbps, 15000);
    let saving_edge = manager.hitung_persentase_penghematan();
    assert!(saving_edge > 80.0, "Penghematan mode Local Edge harus > 80% (aktual: {:.2}%)", saving_edge);

    // 3. Mosaic Quad-View (6 Gelanggang -> 2 Stream Quad) -> 2 * 6000 Kbps = 12,000 Kbps (12 Mbps)
    manager.active_strategy = BandwidthStrategy::MosaicQuadView;
    let mosaic_kbps = manager.hitung_total_bandwidth_upload_kbps();
    assert_eq!(mosaic_kbps, 12000);
    let saving_mosaic = manager.hitung_persentase_penghematan();
    assert!(saving_mosaic > 85.0, "Penghematan mode Mosaic harus > 85% (aktual: {:.2}%)", saving_mosaic);
}

#[test]
fn test_var_replay_clip_extraction_and_mosaic_config() {
    let mut manager = MultiCameraCourtManager::baru("TENANT_IPSI_JATENG", "KEJURNAS_2026", "192.168.1.100");

    manager.daftarkan_kamera(1, CameraAngle::Sudut1DepanKiri, CameraSourceType::WebRtcSmartphone { device_model: "HP1".to_string(), battery_percent: 90 }, 60, 3500);
    manager.daftarkan_kamera(1, CameraAngle::Sudut2DepanKanan, CameraSourceType::WebRtcSmartphone { device_model: "HP2".to_string(), battery_percent: 90 }, 60, 3500);

    let clip = manager.buat_klip_var_replay(1, 30);
    assert_eq!(clip.court_id, 1);
    assert_eq!(clip.duration_seconds, 30);
    assert_eq!(clip.playback_speeds, vec![1.0, 0.5, 0.25]);
    assert!(clip.stream_urls.contains_key("CAM_1_FL"));

    let mosaic_json = manager.buat_konfigurasi_mosaic_compositor(&[1, 2, 3, 4]);
    assert!(mosaic_json.contains(r#""layout":"2x2_grid""#));
    assert!(mosaic_json.contains(r#""fps":60"#));
}
