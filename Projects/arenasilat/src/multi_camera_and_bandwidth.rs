// Projects/arenasilat/src/multi_camera_and_bandwidth.rs
// ==============================================================================
// ARENA SILAT SAAS — MULTI-CAMERA VAR & BANDWIDTH OPTIMIZATION ENGINE
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sudut Penempatan Kamera VAR di Gelanggang (Standar Matras 10m x 10m)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CameraAngle {
    Sudut1DepanKiri,
    Sudut2DepanKanan,
    Sudut3BelakangKiri,
    Sudut4BelakangKanan,
    KameraUtamaOverview,
}

impl CameraAngle {
    pub fn label(&self) -> &'static str {
        match self {
            CameraAngle::Sudut1DepanKiri => "CAM-1: Sudut Depan Kiri (Merah Area)",
            CameraAngle::Sudut2DepanKanan => "CAM-2: Sudut Depan Kanan (Biru Area)",
            CameraAngle::Sudut3BelakangKiri => "CAM-3: Sudut Belakang Kiri (Kuncian/Bantingan)",
            CameraAngle::Sudut4BelakangKanan => "CAM-4: Sudut Belakang Kanan (Garis Matras)",
            CameraAngle::KameraUtamaOverview => "CAM-0: Kamera Utama (Overview Matras)",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            CameraAngle::Sudut1DepanKiri => "CAM_1_FL",
            CameraAngle::Sudut2DepanKanan => "CAM_2_FR",
            CameraAngle::Sudut3BelakangKiri => "CAM_3_BL",
            CameraAngle::Sudut4BelakangKanan => "CAM_4_BR",
            CameraAngle::KameraUtamaOverview => "CAM_0_MAIN",
        }
    }
}

/// Tipe Sumber Perangkat Kamera
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CameraSourceType {
    WebRtcSmartphone { device_model: String, battery_percent: u8 },
    NdiHxBroadcast { ip_address: String, stream_channel: String },
    UsbDirectCapture { device_id: String },
    RtspIpCamera { rtsp_url: String },
}

/// Strategi Optimasi Bandwidth Multi-Gelanggang
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BandwidthStrategy {
    /// 1 Stream Terpisah per Gelanggang (Full HD 1080p - Butuh Bandwidth Tinggi)
    DirectMultiChannel,
    /// 4 Gelanggang Digabung Menjadi 1 Frame Matrix 4-in-1 (Hemat Bandwidth 75%)
    MosaicQuadView,
    /// Rekaman VAR Disimpan Lokal di Ring Buffer RAM Gelanggang (0 Mbps Internet Upload)
    LocalEdgeRingBufferOnly,
    /// Bitrate Otomatis Turun Saat Internet Venue Menurun (1080p -> 720p)
    AdaptiveBitrate,
}

/// Metadata Feed Kamera Aktif
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraStreamFeed {
    pub camera_id: String,
    pub court_id: usize,
    pub angle: CameraAngle,
    pub source_type: CameraSourceType,
    pub resolution: String,
    pub fps: u32,
    pub bitrate_kbps: u32,
    pub is_active: bool,
    pub ring_buffer_seconds: u32,
    pub webrtc_ingest_url: String,
    pub last_frame_timestamp_ms: u64,
}

/// Clip Video Replay Slow Motion untuk Meja KP & Dewan Hakim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarReplayClip {
    pub clip_id: String,
    pub court_id: usize,
    pub triggered_at_ms: u64,
    pub duration_seconds: u32,
    pub camera_angles: Vec<CameraAngle>,
    pub stream_urls: HashMap<String, String>,
    pub playback_speeds: Vec<f32>, // [1.0, 0.5, 0.25]
}

/// Manager Multi-Kamera & Bandwidth Gelanggang
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiCameraCourtManager {
    pub tenant_id: String,
    pub event_id: String,
    pub cameras: HashMap<String, CameraStreamFeed>,
    pub active_strategy: BandwidthStrategy,
    pub total_allocated_bandwidth_kbps: u32,
    pub local_edge_server_ip: String,
}

impl MultiCameraCourtManager {
    pub fn baru(tenant_id: &str, event_id: &str, local_ip: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_string(),
            event_id: event_id.to_string(),
            cameras: HashMap::new(),
            active_strategy: BandwidthStrategy::LocalEdgeRingBufferOnly,
            total_allocated_bandwidth_kbps: 15000, // 15 Mbps default
            local_edge_server_ip: local_ip.to_string(),
        }
    }

    /// Daftarkan Kamera Baru untuk Suatu Gelanggang
    pub fn daftarkan_kamera(
        &mut self,
        court_id: usize,
        angle: CameraAngle,
        source: CameraSourceType,
        fps: u32,
        bitrate_kbps: u32,
    ) -> String {
        let camera_id = format!("CAM-G{}-{}", court_id, angle.code());
        let ingest_url = format!(
            "https://app.arenasilat.com/ingest/{}/{}/court-{}?angle={}&token=WIDYA_{}",
            self.tenant_id, self.event_id, court_id, angle.code(), camera_id
        );

        let feed = CameraStreamFeed {
            camera_id: camera_id.clone(),
            court_id,
            angle,
            source_type: source,
            resolution: "1920x1080".to_string(),
            fps,
            bitrate_kbps,
            is_active: true,
            ring_buffer_seconds: 60,
            webrtc_ingest_url: ingest_url,
            last_frame_timestamp_ms: 1700000000000,
        };

        self.cameras.insert(camera_id.clone(), feed);
        camera_id
    }

    /// Ambil Seluruh Kamera Aktif Berdasarkan ID Gelanggang
    pub fn ambil_kamera_gelanggang(&self, court_id: usize) -> Vec<CameraStreamFeed> {
        self.cameras
            .values()
            .filter(|c| c.court_id == court_id && c.is_active)
            .cloned()
            .collect()
    }

    /// Hitung Total Konsumsi Bandwidth Upload Aktual
    pub fn hitung_total_bandwidth_upload_kbps(&self) -> u32 {
        match self.active_strategy {
            // Local Edge Replay: 0 kbps internet upload untuk VAR (Semua di LAN)
            BandwidthStrategy::LocalEdgeRingBufferOnly => {
                // Hanya kirim 1 main stream overview per court untuk live streaming publik
                let active_courts: std::collections::HashSet<usize> = self.cameras.values().map(|c| c.court_id).collect();
                (active_courts.len() as u32) * 2500 // 2.5 Mbps per gelanggang
            },
            // Mosaic Quad-View: Gabung 4 gelanggang jadi 1 stream tunggal
            BandwidthStrategy::MosaicQuadView => {
                let active_courts: std::collections::HashSet<usize> = self.cameras.values().map(|c| c.court_id).collect();
                let quad_groups = (active_courts.len() as f32 / 4.0).ceil() as u32;
                quad_groups * 6000 // 6.0 Mbps per quad-stream
            },
            // Direct Multi-Channel: Setiap feed kamera diupload ke cloud
            BandwidthStrategy::DirectMultiChannel => {
                self.cameras.values().filter(|c| c.is_active).map(|c| c.bitrate_kbps).sum()
            },
            // Adaptive Bitrate: Kompresi dinamis 1200 kbps per camera
            BandwidthStrategy::AdaptiveBitrate => {
                let active_count = self.cameras.values().filter(|c| c.is_active).count() as u32;
                active_count * 1200
            }
        }
    }

    /// Hitung Persentase Penghematan Bandwidth Dibandingkan Mode Mentah
    pub fn hitung_persentase_penghematan(&self) -> f64 {
        let unoptimized_kbps: u32 = self.cameras.values().filter(|c| c.is_active).map(|c| c.bitrate_kbps).sum();
        if unoptimized_kbps == 0 {
            return 0.0;
        }

        let actual_kbps = self.hitung_total_bandwidth_upload_kbps();
        if actual_kbps >= unoptimized_kbps {
            return 0.0;
        }

        let saved = unoptimized_kbps - actual_kbps;
        ((saved as f64) / (unoptimized_kbps as f64)) * 100.0
    }

    /// Ekstraksi Klip Instant Replay Slow Motion untuk Partai Sedang Berlangsung
    pub fn buat_klip_var_replay(&self, court_id: usize, durasi_detik: u32) -> VarReplayClip {
        let cameras = self.ambil_kamera_gelanggang(court_id);
        let mut stream_urls = HashMap::new();

        for cam in &cameras {
            stream_urls.insert(
                cam.angle.code().to_string(),
                format!(
                    "http://{}:8088/api/var/stream/{}/clip?duration={}",
                    self.local_edge_server_ip, cam.camera_id, durasi_detik
                ),
            );
        }

        VarReplayClip {
            clip_id: format!("VAR-G{}-{}", court_id, chrono_simulasi_timestamp()),
            court_id,
            triggered_at_ms: 1700000005000,
            duration_seconds: durasi_detik,
            camera_angles: cameras.into_iter().map(|c| c.angle).collect(),
            stream_urls,
            playback_speeds: vec![1.0, 0.5, 0.25],
        }
    }

    /// Buat Konfigurasi JSON Mosaic Quad-View Compositor
    pub fn buat_konfigurasi_mosaic_compositor(&self, courts: &[usize]) -> String {
        format!(
            r#"{{"layout":"2x2_grid","resolution":"1920x1080","fps":60,"courts":{:?},"audio_master_court":{},"bitrate_kbps":6000,"overlay_enabled":true}}"#,
            courts,
            courts.first().unwrap_or(&1)
        )
    }
}

fn chrono_simulasi_timestamp() -> u64 {
    1700000000
}
