//! Declarative Page Router, Navigation Stack & Deep Linking Engine (Zero External Dependencies)
//! Standar Navigasi Standar Industri: Navigator.push, Navigator.pop,
//! Dynamic URL Parsing (/produk/:id), dan State Route Stack.

use std::collections::HashMap;
use crate::graphics::pure_engine::WidgetMurni;

/// Representasi Halaman / Rute Layar
#[derive(Debug, Clone)]
pub struct RuteHalaman {
    pub nama_rute: String,
    pub widget_halaman: WidgetMurni,
    pub parameter: HashMap<String, String>,
}

/// Pengelola Navigasi Tumpukan Layar (WidyaNavigator)
#[derive(Debug, Clone)]
pub struct WidyaNavigator {
    pub tumpukan_rute: Vec<RuteHalaman>,
}

impl Default for WidyaNavigator {
    fn default() -> Self {
        Self {
            tumpukan_rute: Vec::new(),
        }
    }
}

impl WidyaNavigator {
    pub fn baru(halaman_awal: RuteHalaman) -> Self {
        Self {
            tumpukan_rute: vec![halaman_awal],
        }
    }

    /// Pindah ke halaman baru (Push Page)
    pub fn push(&mut self, halaman: RuteHalaman) {
        self.tumpukan_rute.push(halaman);
    }

    /// Kembali ke halaman sebelumnya (Pop Page)
    pub fn pop(&mut self) -> Option<RuteHalaman> {
        if self.tumpukan_rute.len() > 1 {
            self.tumpukan_rute.pop()
        } else {
            None // Jangan tutup halaman akar
        }
    }

    /// Ambil halaman aktif di puncak layar
    pub fn halaman_aktif(&self) -> Option<&RuteHalaman> {
        self.tumpukan_rute.last()
    }

    /// Parsing URL Deep Linking (contoh: "/produk/102" dicocokkan dengan pola "/produk/:id")
    pub fn cocokan_pola_url(pola: &str, url_aktual: &str) -> Option<HashMap<String, String>> {
        let pola_segments: Vec<&str> = pola.trim_matches('/').split('/').collect();
        let aktual_segments: Vec<&str> = url_aktual.trim_matches('/').split('/').collect();

        if pola_segments.len() != aktual_segments.len() {
            return None;
        }

        let mut params = HashMap::new();
        for (p, a) in pola_segments.iter().zip(aktual_segments.iter()) {
            if p.starts_with(':') {
                let key = &p[1..];
                params.insert(key.to_string(), a.to_string());
            } else if p != a {
                return None;
            }
        }

        Some(params)
    }
}
