//! Pure Sovereign Desktop Window Management & Native Surface Engine
//! Zero External C Dependencies (No Qt, No Electron, No GTK, No SDL).
//! Manages native window lifecycles, geometry constraints, dirty region rendering,
//! acrylic/mica visual vibrancy effects, and multi-monitor placement.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Gaya Efek Visual Latar Belakang Jendela
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EfekLatarJendela {
    Solid,
    AcrylicBlur,
    MicaDark,
    MicaLight,
    Transparan,
}

/// Status Siklus Hidup Jendela
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusJendela {
    Dibuat,
    Ditampilkan,
    Diminimalkan,
    Dimaksimalkan,
    Kiosk,
    Ditutup,
}

/// Batasan Geometri Jendela
#[derive(Debug, Clone)]
pub struct BatasanGeometri {
    pub min_lebar: u32,
    pub min_tinggi: u32,
    pub max_lebar: Option<u32>,
    pub max_tinggi: Option<u32>,
    pub rasio_aspek: Option<f32>,
}

/// Konfigurasi & State Jendela Desktop Murni
#[derive(Debug, Clone)]
pub struct JendelaMurni {
    pub id: String,
    pub judul: String,
    pub x: i32,
    pub y: i32,
    pub lebar: u32,
    pub tinggi: u32,
    pub tanpa_bingkai: bool,
    pub selalu_di_atas: bool,
    pub efek_latar: EfekLatarJendela,
    pub status: StatusJendela,
    pub batasan: BatasanGeometri,
    pub monitor_id: u32,
    pub custom_titlebar: bool,
    pub dirty_regions: Vec<(u32, u32, u32, u32)>, // (x, y, w, h)
}

impl JendelaMurni {
    pub fn baru(id: &str, judul: &str, lebar: u32, tinggi: u32) -> Self {
        Self {
            id: id.to_string(),
            judul: judul.to_string(),
            x: 100,
            y: 100,
            lebar,
            tinggi,
            tanpa_bingkai: false,
            selalu_di_atas: false,
            efek_latar: EfekLatarJendela::Solid,
            status: StatusJendela::Dibuat,
            batasan: BatasanGeometri {
                min_lebar: 320,
                min_tinggi: 240,
                max_lebar: None,
                max_tinggi: None,
                rasio_aspek: None,
            },
            monitor_id: 0,
            custom_titlebar: false,
            dirty_regions: Vec::new(),
        }
    }

    /// Ubah ukuran jendela dengan mematuhi batasan geometri
    pub fn ubah_ukuran(&mut self, target_w: u32, target_h: u32) {
        let mut w = target_w.max(self.batasan.min_lebar);
        let mut h = target_h.max(self.batasan.min_tinggi);

        if let Some(max_w) = self.batasan.max_lebar {
            w = w.min(max_w);
        }
        if let Some(max_h) = self.batasan.max_tinggi {
            h = h.min(max_h);
        }

        if let Some(rasio) = self.batasan.rasio_aspek {
            if rasio > 0.0 {
                h = (w as f32 / rasio) as u32;
            }
        }

        self.lebar = w;
        self.tinggi = h;
        self.tandai_kotor(0, 0, w, h);
    }

    /// Tandai area tampilan yang perlu di-render ulang (Dirty Region Tracking)
    pub fn tandai_kotor(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.dirty_regions.push((x, y, w, h));
    }

    /// Bersihkan dirty regions setelah frame selesai dipaint ke framebuffer
    pub fn bersihkan_dirty_regions(&mut self) -> Vec<(u32, u32, u32, u32)> {
        std::mem::take(&mut self.dirty_regions)
    }

    pub fn tampilkan(&mut self) {
        self.status = StatusJendela::Ditampilkan;
    }

    pub fn minimalkan(&mut self) {
        self.status = StatusJendela::Diminimalkan;
    }

    pub fn maksimalkan(&mut self) {
        self.status = StatusJendela::Dimaksimalkan;
    }

    pub fn masuk_mode_kiosk(&mut self) {
        self.tanpa_bingkai = true;
        self.selalu_di_atas = true;
        self.status = StatusJendela::Kiosk;
    }

    pub fn tutup(&mut self) {
        self.status = StatusJendela::Ditutup;
    }
}

/// Pengelola Semua Jendela Desktop (Window Manager Hub)
pub struct PengelolaJendelaMurni {
    jendela_map: Arc<Mutex<HashMap<String, JendelaMurni>>>,
}

impl Default for PengelolaJendelaMurni {
    fn default() -> Self {
        Self {
            jendela_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

static GLOBAL_WINDOW_MANAGER: OnceLock<PengelolaJendelaMurni> = OnceLock::new();

pub fn get_global_window_manager() -> &'static PengelolaJendelaMurni {
    GLOBAL_WINDOW_MANAGER.get_or_init(PengelolaJendelaMurni::default)
}

impl PengelolaJendelaMurni {
    pub fn baru() -> Self {
        Self::default()
    }

    pub fn daftarkan_jendela(&self, jendela: JendelaMurni) {
        self.jendela_map.lock().unwrap().insert(jendela.id.clone(), jendela);
    }

    pub fn ambil_jendela<F, R>(&self, id: &str, f: F) -> Option<R>
    where
        F: FnOnce(&JendelaMurni) -> R,
    {
        self.jendela_map.lock().unwrap().get(id).map(f)
    }

    pub fn mutasi_jendela<F, R>(&self, id: &str, f: F) -> Option<R>
    where
        F: FnOnce(&mut JendelaMurni) -> R,
    {
        self.jendela_map.lock().unwrap().get_mut(id).map(f)
    }

    pub fn total_jendela(&self) -> usize {
        self.jendela_map.lock().unwrap().len()
    }

    pub fn tutup_semua(&self) {
        let mut map = self.jendela_map.lock().unwrap();
        for j in map.values_mut() {
            j.tutup();
        }
    }
}
