//! Gestures, Event Dispatcher, 120 FPS Animation & Tween Engine (Zero External Dependencies)
//! Mendukung Multi-Touch Dispatcher, Hit-Testing, Physics-based Scroll,
//! Easing Curves (Linear, Ease-In, Ease-Out, Ease-In-Out, Bounce, Elastic), dan Tween Interpolation.

use crate::graphics::pure_canvas::{PersegiKotak, Titik2D, Warna};
use crate::graphics::pure_engine::RenderKotakMurni;

/// Jenis Peristiwa Sentuhan Layar (Touch Event Types)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JenisSentuhan {
    TekanBawah,
    LepasAtas,
    PindahGeser,
    Batal,
}

/// Struktur Data Peristiwa Sentuhan (Touch Gesture Event)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PeristiwaSentuh {
    pub jenis: JenisSentuhan,
    pub x: f32,
    pub y: f32,
    pub timestamp_ms: u64,
    pub pointer_id: u32,
}

impl PeristiwaSentuh {
    pub fn baru(jenis: JenisSentuhan, x: f32, y: f32, timestamp_ms: u64) -> Self {
        Self {
            jenis,
            x,
            y,
            timestamp_ms,
            pointer_id: 0,
        }
    }
}

/// Detektor Gestur Tingkat Tinggi
#[derive(Debug, Clone, PartialEq)]
pub enum GesturTerdeteksi {
    Ketuk { id_target: String, x: f32, y: f32 },
    KetukGanda { id_target: String, x: f32, y: f32 },
    TekanLama { id_target: String, durasi_ms: u64 },
    GeserPan { id_target: String, delta_x: f32, delta_y: f32, kecepatan_px_per_s: f32 },
}

/// Mesin Penguji Sasaran Sentuh (Hit-Testing Dispatcher)
pub struct HitTestDispatcher;

impl HitTestDispatcher {
    /// Menemukan ID widget terdalam yang berada di bawah koordinat titik sentuh (x, y)
    pub fn temukan_target<'a>(render_tree: &'a RenderKotakMurni, x: f32, y: f32) -> Option<&'a str> {
        let kotak = PersegiKotak::baru(render_tree.x, render_tree.y, render_tree.lebar, render_tree.tinggi);
        if !kotak.memuat_titik(x, y) {
            return None;
        }

        // Cari di anak-anaknya terlebih dahulu (lapisan paling atas)
        for child in render_tree.anak.iter().rev() {
            if let Some(target) = Self::temukan_target(child, x, y) {
                return Some(target);
            }
        }

        Some(&render_tree.id)
    }
}

/// Kurva Easing Animasi (Animation Easing Curves)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JenisKurva {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    BounceOut,
    ElasticOut,
}

impl JenisKurva {
    /// Evaluasi nilai t (0.0 s.d. 1.0) dengan kurva matematika murni
    pub fn evaluasi(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => t * (2.0 - t),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Self::BounceOut => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
            Self::ElasticOut => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    (-2.0f32.powf(10.0 * (t - 1.0)) * ((t - 1.0 - s) * (2.0 * std::f32::consts::PI) / p).sin()) + 1.0
                }
            }
        }
    }
}

/// Mesin Interpolasi Tween (Tween Value Interpolator)
pub struct Tween<T> {
    pub awal: T,
    pub akhir: T,
    pub kurva: JenisKurva,
}

impl Tween<f32> {
    pub fn baru(awal: f32, akhir: f32, kurva: JenisKurva) -> Self {
        Self { awal, akhir, kurva }
    }

    pub fn transformasikan(&self, t: f32) -> f32 {
        let t_eased = self.kurva.evaluasi(t);
        self.awal + (self.akhir - self.awal) * t_eased
    }
}

impl Tween<Warna> {
    pub fn warna(awal: Warna, akhir: Warna, kurva: JenisKurva) -> Self {
        Self { awal, akhir, kurva }
    }

    pub fn transformasikan(&self, t: f32) -> Warna {
        let t_eased = self.kurva.evaluasi(t);
        let r = (self.awal.r as f32 + (self.akhir.r as f32 - self.awal.r as f32) * t_eased) as u8;
        let g = (self.awal.g as f32 + (self.akhir.g as f32 - self.awal.g as f32) * t_eased) as u8;
        let b = (self.awal.b as f32 + (self.akhir.b as f32 - self.awal.b as f32) * t_eased) as u8;
        let a = (self.awal.a as f32 + (self.akhir.a as f32 - self.awal.a as f32) * t_eased) as u8;
        Warna::rgba(r, g, b, a)
    }
}

/// Status Pengontrol Animasi
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusAnimasi {
    Berhenti,
    Maju,
    Mundur,
    Selesai,
}

/// Pengontrol Animasi (Animation Controller)
#[derive(Debug, Clone)]
pub struct PengontrolAnimasi {
    pub id: String,
    pub durasi_ms: u64,
    pub waktu_berjalan_ms: u64,
    pub nilai_t: f32, // 0.0 s.d. 1.0
    pub status: StatusAnimasi,
    pub looping: bool,
}

impl PengontrolAnimasi {
    pub fn baru(id: &str, durasi_ms: u64) -> Self {
        Self {
            id: id.to_string(),
            durasi_ms: durasi_ms.max(1),
            waktu_berjalan_ms: 0,
            nilai_t: 0.0,
            status: StatusAnimasi::Berhenti,
            looping: false,
        }
    }

    pub fn maju(&mut self) {
        self.status = StatusAnimasi::Maju;
    }

    pub fn mundur(&mut self) {
        self.status = StatusAnimasi::Mundur;
    }

    pub fn berhenti(&mut self) {
        self.status = StatusAnimasi::Berhenti;
    }

    pub fn reset(&mut self) {
        self.waktu_berjalan_ms = 0;
        self.nilai_t = 0.0;
        self.status = StatusAnimasi::Berhenti;
    }

    /// Majukan simulasi waktu per frame (misal 16.6ms untuk 60 FPS atau 8.3ms untuk 120 FPS)
    pub fn perbarui(&mut self, delta_ms: u64) {
        match self.status {
            StatusAnimasi::Maju => {
                self.waktu_berjalan_ms = (self.waktu_berjalan_ms + delta_ms).min(self.durasi_ms);
                self.nilai_t = self.waktu_berjalan_ms as f32 / self.durasi_ms as f32;
                if self.waktu_berjalan_ms >= self.durasi_ms {
                    if self.looping {
                        self.waktu_berjalan_ms = 0;
                        self.nilai_t = 0.0;
                    } else {
                        self.status = StatusAnimasi::Selesai;
                    }
                }
            }
            StatusAnimasi::Mundur => {
                if self.waktu_berjalan_ms <= delta_ms {
                    self.waktu_berjalan_ms = 0;
                    self.nilai_t = 0.0;
                    self.status = StatusAnimasi::Selesai;
                } else {
                    self.waktu_berjalan_ms -= delta_ms;
                    self.nilai_t = self.waktu_berjalan_ms as f32 / self.durasi_ms as f32;
                }
            }
            _ => {}
        }
    }
}
