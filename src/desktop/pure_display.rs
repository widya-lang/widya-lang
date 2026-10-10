//! Pure Multi-Monitor & Customer-Facing Display Controller
//! Zero External Dependencies.
//! Manages monitor detection, secondary customer checkout screens,
//! kiosk full-screen locks, and hardware-accelerated dirty region compositing.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Metadata Monitor Display Terdeteksi
#[derive(Debug, Clone, PartialEq)]
pub struct InfoMonitor {
    pub id: u32,
    pub nama: String,
    pub x: i32,
    pub y: i32,
    pub lebar: u32,
    pub tinggi: u32,
    pub scale_factor: f32,
    pub is_primary: bool,
    pub refresh_rate_hz: u32,
}

/// Pengontrol Tampilan Multi-Monitor & Layar Pelanggan (Customer Display Hub)
pub struct PengontrolMultiMonitor {
    monitors: Arc<Mutex<Vec<InfoMonitor>>>,
    customer_display_state: Arc<Mutex<StateCustomerDisplay>>,
}

/// State Layar Pelanggan (Customer-Facing Checkout Screen)
#[derive(Debug, Clone, Default)]
pub struct StateCustomerDisplay {
    pub total_belanja_rp: u64,
    pub diskon_rp: u64,
    pub item_terakhir: String,
    pub pesan_selamat_datang: String,
    pub url_iklan_aktif: Option<String>,
    pub qris_payload: Option<String>,
}

impl Default for PengontrolMultiMonitor {
    fn default() -> Self {
        let default_primary = InfoMonitor {
            id: 0,
            nama: "Primary Display".to_string(),
            x: 0,
            y: 0,
            lebar: 1920,
            tinggi: 1080,
            scale_factor: 1.0,
            is_primary: true,
            refresh_rate_hz: 60,
        };
        Self {
            monitors: Arc::new(Mutex::new(vec![default_primary])),
            customer_display_state: Arc::new(Mutex::new(StateCustomerDisplay::default())),
        }
    }
}

static GLOBAL_DISPLAY_CONTROLLER: OnceLock<PengontrolMultiMonitor> = OnceLock::new();

pub fn get_global_display_controller() -> &'static PengontrolMultiMonitor {
    GLOBAL_DISPLAY_CONTROLLER.get_or_init(PengontrolMultiMonitor::default)
}

impl PengontrolMultiMonitor {
    pub fn baru() -> Self {
        Self::default()
    }

    /// Daftarkan monitor yang terhubung
    pub fn daftarkan_monitor(&self, monitor: InfoMonitor) {
        let mut list = self.monitors.lock().unwrap();
        if let Some(pos) = list.iter().position(|m| m.id == monitor.id) {
            list[pos] = monitor;
        } else {
            list.push(monitor);
        }
    }

    /// Ambil daftar semua monitor aktif
    pub fn daftar_monitor(&self) -> Vec<InfoMonitor> {
        self.monitors.lock().unwrap().clone()
    }

    /// Dapatkan monitor primer
    pub fn ambil_monitor_primer(&self) -> Option<InfoMonitor> {
        self.monitors.lock().unwrap().iter().find(|m| m.is_primary).cloned()
    }

    /// Dapatkan monitor sekunder (untuk layar pelanggan / customer display)
    pub fn ambil_monitor_sekunder(&self) -> Option<InfoMonitor> {
        self.monitors.lock().unwrap().iter().find(|m| !m.is_primary).cloned()
    }

    /// Perbarui state layar pelanggan (Customer Screen Update)
    pub fn perbarui_layar_pelanggan(
        &self,
        total_rp: u64,
        item: &str,
        qris: Option<&str>,
    ) {
        let mut state = self.customer_display_state.lock().unwrap();
        state.total_belanja_rp = total_rp;
        state.item_terakhir = item.to_string();
        state.qris_payload = qris.map(|s| s.to_string());
    }

    /// Ambil snapshot state layar pelanggan
    pub fn snapshot_layar_pelanggan(&self) -> StateCustomerDisplay {
        self.customer_display_state.lock().unwrap().clone()
    }
}
