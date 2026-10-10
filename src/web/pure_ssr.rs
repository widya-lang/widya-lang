//! Hybrid Rendering Engine (Streaming SSR, ISR, SWR & Islands Architecture)
//! Standar Rendering Modern: 0 KB Client JS Default, Selective Hydration (Islands),
//! HTTP Chunked Transfer Streaming, Stale-While-Revalidate Caching, dan Resumability.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Metadata Konfigurasi Rendering Halaman
#[derive(Debug, Clone)]
pub struct KonfigurasiRenderHalaman {
    pub mode_render: ModeRender,
    pub revalidasi_detik: Option<u64>,
    pub tags_revalidasi: Vec<String>,
    pub suspense_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeRender {
    StreamingSSR,      // Server-Side Rendering via HTTP Chunked Transfer
    StaticSiteGen,     // Static Site Generation (SSG)
    IncrementalStatic, // ISR dengan Stale-While-Revalidate
    IslandsOnly,       // 0 KB JS default dengan pulau interaktif terisolasi
}

/// Struktur Pulau Interaktif (Interactive Island Component)
#[derive(Debug, Clone)]
pub struct PulauInteraktif {
    pub id_pulau: String,
    pub nama_komponen: String,
    pub state_props_json: String,
    pub html_fallback: String,
    pub client_bundle_url: String,
}

impl PulauInteraktif {
    pub fn render_ke_html(&self) -> String {
        format!(
            r#"<div id="{}" data-widya-island="{}" data-widya-props="{}" class="widya-interactive-island">{}<script type="module" src="{}"></script></div>"#,
            self.id_pulau,
            self.nama_komponen,
            html_escape(&self.state_props_json),
            self.html_fallback,
            self.client_bundle_url
        )
    }
}

/// Mesin Penyimpanan Cache ISR (Incremental Static Regeneration Store)
#[derive(Debug, Clone)]
pub struct EntryCacheISR {
    pub url_path: String,
    pub html_konten: String,
    pub dibuat_pada: u64,
    pub revalidasi_detik: u64,
    pub tags: Vec<String>,
}

pub struct HubCacheISR {
    pub entries: Arc<Mutex<HashMap<String, EntryCacheISR>>>,
}

impl Default for HubCacheISR {
    fn default() -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

static GLOBAL_ISR_CACHE: OnceLock<HubCacheISR> = OnceLock::new();

pub fn get_global_isr_cache() -> &'static HubCacheISR {
    GLOBAL_ISR_CACHE.get_or_init(HubCacheISR::default)
}

impl HubCacheISR {
    pub fn simpan(&self, url: &str, html: &str, revalidasi_detik: u64, tags: Vec<String>) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let entry = EntryCacheISR {
            url_path: url.to_string(),
            html_konten: html.to_string(),
            dibuat_pada: now,
            revalidasi_detik,
            tags,
        };
        self.entries.lock().unwrap().insert(url.to_string(), entry);
    }

    pub fn ambil(&self, url: &str) -> Option<(String, bool)> {
        let map = self.entries.lock().unwrap();
        if let Some(entry) = map.get(url) {
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
            let is_stale = now > entry.dibuat_pada + entry.revalidasi_detik;
            Some((entry.html_konten.clone(), is_stale))
        } else {
            None
        }
    }

    pub fn revalidasi_tag(&self, tag: &str) -> usize {
        let mut map = self.entries.lock().unwrap();
        let mut count = 0;
        map.retain(|_, entry| {
            if entry.tags.iter().any(|t| t == tag) {
                count += 1;
                false // Hapus dari cache untuk diregenerasi
            } else {
                true
            }
        });
        count
    }

    pub fn baru() -> Self {
        Self::default()
    }

    pub fn simpan_cache(&mut self, url: &str, html: &str, revalidasi_detik: u64, tags: Vec<String>) {
        self.simpan(url, html, revalidasi_detik, tags);
    }

    pub fn ambil_cache(&self, url: &str) -> Option<String> {
        self.ambil(url).map(|(html, _)| html)
    }
}

pub type IsrCacheHub = HubCacheISR;
pub type KomponenIsland = PulauInteraktif;

impl PulauInteraktif {
    pub fn baru(id: &str, nama: &str, props_json: &str, fallback: &str) -> Self {
        Self {
            id_pulau: id.to_string(),
            nama_komponen: nama.to_string(),
            state_props_json: props_json.to_string(),
            html_fallback: fallback.to_string(),
            client_bundle_url: format!("/_widya/islands/{}.js", nama),
        }
    }
}

/// Mesin Streaming SSR & HTML Chunk Generator
pub struct StreamingSSREngine;
pub type MesinSSRMurni = StreamingSSREngine;

impl StreamingSSREngine {
    /// Hasilkan potongan pertama HTML (*First Shell Chunk*) dengan Suspense Skeleton
    pub fn buat_shell_chunk(judul: &str, meta_deskripsi: &str) -> String {
        format!(
            r#"<!DOCTYPE html><html lang="id"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>{}</title><meta name="description" content="{}"><link rel="stylesheet" href="/_widya/style.css"></head><body class="widya-body"><div id="__widya_app">"#,
            html_escape(judul),
            html_escape(meta_deskripsi)
        )
    }

    /// Hasilkan potongan penutup HTML (*Closing Chunk*) dengan Script Hydration minimalis
    pub fn buat_penutup_chunk() -> String {
        r#"</div><script type="module" src="/_widya/client-bootstrap.js"></script></body></html>"#.to_string()
    }

    /// Streaming chunks generator
    pub fn render_streaming_chunks(judul: &str, konten: &str) -> Vec<String> {
        vec![
            Self::buat_shell_chunk(judul, "Widya Sovereign Web"),
            konten.to_string(),
            Self::buat_penutup_chunk(),
        ]
    }

    /// Render island component
    pub fn render_island(island: &KomponenIsland) -> String {
        island.render_ke_html()
    }
}

fn html_escape(input: &str) -> String {
    input.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

