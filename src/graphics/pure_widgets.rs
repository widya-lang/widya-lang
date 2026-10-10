//! Material 3 & Cupertino Core Widgets Library (Zero External Dependencies)
//! Standar Komponen UI Kelas Dunia: Card, ElevatedButton, OutlinedButton,
//! TextField, Switch, Checkbox, ProgressBar, Badge, dan Cupertino iOS Components.

use crate::graphics::pure_canvas::Warna;
use crate::graphics::pure_engine::{JenisWidget, WidgetMurni};
use crate::graphics::pure_font::GayaTipografi;

/// Pustaka Widget Material 3 & Cupertino
pub struct WidyaWidgets;

impl WidyaWidgets {
    /// Komponen Card Material 3 dengan Elevasi & Sudut Membulat
    pub fn card(id: &str, anak: WidgetMurni, warna_latar: Option<Warna>, radius: f32) -> WidgetMurni {
        let bg = warna_latar.unwrap_or_else(|| Warna::rgb(255, 255, 255));
        WidgetMurni::kontainer(
            id,
            None,
            None,
            Some(bg),
            radius,
            Some(anak),
        )
    }

    /// Tombol Utama (Elevated Button) Material 3
    pub fn elevated_button(id: &str, label: &str, warna: Warna, aksi: &str) -> WidgetMurni {
        WidgetMurni::tombol(id, label, warna, aksi)
    }

    /// Tombol Garis Tepi (Outlined Button)
    pub fn outlined_button(id: &str, label: &str, warna_teks: Warna, aksi: &str) -> WidgetMurni {
        WidgetMurni::tombol(id, label, Warna::TRANSPARAN, aksi)
    }

    /// Bidang Input Teks (TextField)
    pub fn text_field(id: &str, placeholder: &str, nilai: &str) -> WidgetMurni {
        let teks_tampil = if nilai.is_empty() { placeholder } else { nilai };
        let warna_teks = if nilai.is_empty() {
            Warna::rgb(156, 163, 175) // Gray-400
        } else {
            Warna::rgb(17, 24, 39) // Gray-900
        };

        WidgetMurni::kontainer(
            id,
            None,
            Some(48.0),
            Some(Warna::rgb(243, 244, 246)), // Light gray input background
            8.0,
            Some(WidgetMurni::teks(&format!("{}_txt", id), teks_tampil, 14.0, warna_teks)),
        )
    }

    /// Saklar Toggle Switch (Material 3 / Cupertino)
    pub fn switch(id: &str, aktif: bool, aksi: &str) -> WidgetMurni {
        let warna_bg = if aktif {
            Warna::rgb(37, 99, 235) // Blue-600
        } else {
            Warna::rgb(209, 213, 219) // Gray-300
        };
        let label = if aktif { " [ON] " } else { " [OFF] " };

        WidgetMurni::kontainer(
            id,
            Some(56.0),
            Some(32.0),
            Some(warna_bg),
            16.0,
            Some(WidgetMurni::teks(&format!("{}_lbl", id), label, 10.0, Warna::PUTIH)),
        )
    }

    /// Kotak Centang (Checkbox)
    pub fn checkbox(id: &str, checked: bool, label: &str, aksi: &str) -> WidgetMurni {
        let check_symbol = if checked { "[v] " } else { "[ ] " };
        let full_text = format!("{}{}", check_symbol, label);
        let color = if checked {
            Warna::rgb(16, 185, 129) // Emerald-500
        } else {
            Warna::rgb(75, 85, 99) // Gray-600
        };

        WidgetMurni::tombol(id, &full_text, Warna::TRANSPARAN, aksi)
    }

    /// Bilah Kemajuan (Linear Progress Bar)
    pub fn progress_bar(id: &str, progress_percent: f32, warna_aktif: Warna) -> WidgetMurni {
        let p = progress_percent.clamp(0.0, 100.0);
        let fill_width = (p / 100.0) * 300.0;

        WidgetMurni::kontainer(
            id,
            Some(300.0),
            Some(8.0),
            Some(Warna::rgb(229, 231, 235)), // Gray-200 track
            4.0,
            Some(WidgetMurni::kontainer(
                &format!("{}_fill", id),
                Some(fill_width),
                Some(8.0),
                Some(warna_aktif),
                4.0,
                None,
            )),
        )
    }

    /// Lencana Notifikasi (Badge Pill)
    pub fn badge(id: &str, label: &str, warna_bg: Warna) -> WidgetMurni {
        WidgetMurni::kontainer(
            id,
            None,
            Some(22.0),
            Some(warna_bg),
            11.0,
            Some(WidgetMurni::teks(&format!("{}_txt", id), label, 11.0, Warna::PUTIH)),
        )
    }

    /// Cupertino (iOS) Navigation Bar Translucent
    pub fn cupertino_nav_bar(id: &str, judul: &str, aksi_kembali: Option<&str>) -> WidgetMurni {
        let mut children = Vec::new();
        if let Some(aksi) = aksi_kembali {
            children.push(WidgetMurni::tombol(&format!("{}_back", id), "< Kembali", Warna::TRANSPARAN, aksi));
        }
        children.push(WidgetMurni::teks(&format!("{}_title", id), judul, 17.0, Warna::rgb(0, 0, 0)));

        WidgetMurni::kontainer(
            id,
            None,
            Some(50.0),
            Some(Warna::rgba(255, 255, 255, 220)), // Translucent Blur Header
            0.0,
            Some(WidgetMurni::baris(&format!("{}_row", id), children, 12.0)),
        )
    }
}
