//! Pure Three-Tree Reactive UI Architecture & BoxConstraints Layout Engine (Zero Flutter/Dart Dependency)
//! Mengimplementasikan Widget Tree, Element Tree, RenderObject Tree,
//! BoxConstraints (Tight/Loose/Bounded), serta Paint langsung ke FramebufferMurni.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::graphics::pure_canvas::{FramebufferMurni, PersegiKotak, Warna};
use crate::graphics::pure_font::{FontAtlasMurni, GayaTipografi, PerataanTeks};

/// Batasan Geometri Kotak (BoxConstraints)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatasanKotak {
    pub min_lebar: f32,
    pub maks_lebar: f32,
    pub min_tinggi: f32,
    pub maks_tinggi: f32,
}

impl BatasanKotak {
    pub fn baru(min_w: f32, maks_w: f32, min_h: f32, maks_h: f32) -> Self {
        let min_w = min_w.max(0.0);
        let maks_w = maks_w.max(min_w);
        let min_h = min_h.max(0.0);
        let maks_h = maks_h.max(min_h);
        Self {
            min_lebar: min_w,
            maks_lebar: maks_w,
            min_tinggi: min_h,
            maks_tinggi: maks_h,
        }
    }

    pub fn ketat(lebar: f32, tinggi: f32) -> Self {
        let l = lebar.max(0.0);
        let t = tinggi.max(0.0);
        Self {
            min_lebar: l,
            maks_lebar: l,
            min_tinggi: t,
            maks_tinggi: t,
        }
    }

    pub fn longgar(maks_w: f32, maks_h: f32) -> Self {
        let mw = maks_w.max(0.0);
        let mh = maks_h.max(0.0);
        Self {
            min_lebar: 0.0,
            maks_lebar: mw,
            min_tinggi: 0.0,
            maks_tinggi: mh,
        }
    }

    pub fn batasi_ukuran(&self, lebar: f32, tinggi: f32) -> (f32, f32) {
        let w = if self.min_lebar <= self.maks_lebar {
            lebar.clamp(self.min_lebar, self.maks_lebar)
        } else {
            self.min_lebar
        };
        let h = if self.min_tinggi <= self.maks_tinggi {
            tinggi.clamp(self.min_tinggi, self.maks_tinggi)
        } else {
            self.min_tinggi
        };
        (w, h)
    }
}

/// Tipe Widget Inti
#[derive(Debug, Clone)]
pub enum JenisWidget {
    Kontainer {
        lebar: Option<f32>,
        tinggi: Option<f32>,
        warna_latar: Option<Warna>,
        radius_sudut: f32,
        padding: f32,
        anak: Option<Box<WidgetMurni>>,
    },
    Teks {
        konten: String,
        gaya: GayaTipografi,
    },
    Baris {
        anak: Vec<WidgetMurni>,
        spasi: f32,
    },
    Kolom {
        anak: Vec<WidgetMurni>,
        spasi: f32,
    },
    Expanded {
        flex: u32,
        anak: Box<WidgetMurni>,
    },
    Tombol {
        label: String,
        warna: Warna,
        aksi: String,
    },
    Scaffold {
        app_bar_judul: Option<String>,
        app_bar_warna: Option<Warna>,
        body: Box<WidgetMurni>,
    },
}

/// [1] WIDGET TREE: Konfigurasi Immutabel Deklaratif
#[derive(Debug, Clone)]
pub struct WidgetMurni {
    pub id: String,
    pub jenis: JenisWidget,
}

impl WidgetMurni {
    pub fn kontainer(id: &str, lebar: Option<f32>, tinggi: Option<f32>, warna: Option<Warna>, radius: f32, anak: Option<WidgetMurni>) -> Self {
        Self {
            id: id.to_string(),
            jenis: JenisWidget::Kontainer {
                lebar,
                tinggi,
                warna_latar: warna,
                radius_sudut: radius,
                padding: 8.0,
                anak: anak.map(Box::new),
            },
        }
    }

    pub fn teks(id: &str, konten: &str, ukuran: f32, warna: Warna) -> Self {
        let mut gaya = GayaTipografi::default();
        gaya.ukuran_font = ukuran;
        gaya.warna = warna;
        Self {
            id: id.to_string(),
            jenis: JenisWidget::Teks {
                konten: konten.to_string(),
                gaya,
            },
        }
    }

    pub fn kolom(id: &str, anak: Vec<WidgetMurni>, spasi: f32) -> Self {
        Self {
            id: id.to_string(),
            jenis: JenisWidget::Kolom { anak, spasi },
        }
    }

    pub fn baris(id: &str, anak: Vec<WidgetMurni>, spasi: f32) -> Self {
        Self {
            id: id.to_string(),
            jenis: JenisWidget::Baris { anak, spasi },
        }
    }

    pub fn tombol(id: &str, label: &str, warna: Warna, aksi: &str) -> Self {
        Self {
            id: id.to_string(),
            jenis: JenisWidget::Tombol {
                label: label.to_string(),
                warna,
                aksi: aksi.to_string(),
            },
        }
    }

    pub fn scaffold(id: &str, judul: &str, warna_bar: Warna, body: WidgetMurni) -> Self {
        Self {
            id: id.to_string(),
            jenis: JenisWidget::Scaffold {
                app_bar_judul: Some(judul.to_string()),
                app_bar_warna: Some(warna_bar),
                body: Box::new(body),
            },
        }
    }
}

/// [2] RENDER OBJECT TREE: Objek Geometri Mutabel & Penanggung Jawab Painting
#[derive(Debug, Clone)]
pub struct RenderKotakMurni {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub lebar: f32,
    pub tinggi: f32,
    pub batasan: BatasanKotak,
    pub perlu_cat: bool,
    pub anak: Vec<RenderKotakMurni>,
}

impl RenderKotakMurni {
    pub fn baru(id: &str, batasan: BatasanKotak) -> Self {
        Self {
            id: id.to_string(),
            x: 0.0,
            y: 0.0,
            lebar: 0.0,
            tinggi: 0.0,
            batasan,
            perlu_cat: true,
            anak: Vec::new(),
        }
    }

    /// Protokol One-Pass BoxConstraints Layout (Constraints Down, Sizes Up)
    pub fn tata_letak(&mut self, widget: &WidgetMurni, batasan_induk: BatasanKotak) {
        self.batasan = batasan_induk;

        match &widget.jenis {
            JenisWidget::Kontainer { lebar, tinggi, padding, anak, .. } => {
                let target_w = lebar.unwrap_or(batasan_induk.maks_lebar);
                let target_h = tinggi.unwrap_or(batasan_induk.maks_tinggi);
                let (w, h) = batasan_induk.batasi_ukuran(target_w, target_h);
                self.lebar = w;
                self.tinggi = h;

                if let Some(child_widget) = anak {
                    let child_w = (w - padding * 2.0).max(0.0);
                    let child_h = (h - padding * 2.0).max(0.0);
                    let mut child_render = RenderKotakMurni::baru(&child_widget.id, BatasanKotak::longgar(child_w, child_h));
                    child_render.x = self.x + padding.min(w / 2.0);
                    child_render.y = self.y + padding.min(h / 2.0);
                    child_render.tata_letak(child_widget, child_render.batasan);
                    self.anak = vec![child_render];
                }
            }
            JenisWidget::Teks { konten, gaya } => {
                let metrik = FontAtlasMurni::hitung_metrik(konten, gaya);
                let (w, h) = batasan_induk.batasi_ukuran(metrik.lebar, metrik.tinggi);
                self.lebar = w;
                self.tinggi = h;
            }
            JenisWidget::Kolom { anak, spasi } => {
                let mut akumulasi_h = 0.0f32;
                let mut max_w = 0.0f32;
                let mut render_children = Vec::new();

                for child in anak {
                    let child_batasan = BatasanKotak::longgar(batasan_induk.maks_lebar, (batasan_induk.maks_tinggi - akumulasi_h).max(0.0));
                    let mut child_render = RenderKotakMurni::baru(&child.id, child_batasan);
                    child_render.x = self.x;
                    child_render.y = self.y + akumulasi_h;
                    child_render.tata_letak(child, child_batasan);

                    akumulasi_h += child_render.tinggi + spasi;
                    if child_render.lebar > max_w {
                        max_w = child_render.lebar;
                    }
                    render_children.push(child_render);
                }

                let (w, h) = batasan_induk.batasi_ukuran(max_w.max(batasan_induk.maks_lebar), akumulasi_h);
                self.lebar = w;
                self.tinggi = h;
                self.anak = render_children;
            }
            JenisWidget::Baris { anak, spasi } => {
                let mut akumulasi_w = 0.0f32;
                let mut max_h = 0.0f32;
                let mut render_children = Vec::new();

                for child in anak {
                    let child_batasan = BatasanKotak::longgar((batasan_induk.maks_lebar - akumulasi_w).max(0.0), batasan_induk.maks_tinggi);
                    let mut child_render = RenderKotakMurni::baru(&child.id, child_batasan);
                    child_render.x = self.x + akumulasi_w;
                    child_render.y = self.y;
                    child_render.tata_letak(child, child_batasan);

                    akumulasi_w += child_render.lebar + spasi;
                    if child_render.tinggi > max_h {
                        max_h = child_render.tinggi;
                    }
                    render_children.push(child_render);
                }

                let (w, h) = batasan_induk.batasi_ukuran(akumulasi_w, max_h);
                self.lebar = w;
                self.tinggi = h;
                self.anak = render_children;
            }
            JenisWidget::Tombol { label, .. } => {
                let len = label.chars().count() as f32;
                let (w, h) = batasan_induk.batasi_ukuran((len * 10.0 + 32.0).max(120.0), 44.0);
                self.lebar = w;
                self.tinggi = h;
            }
            JenisWidget::Scaffold { app_bar_judul, app_bar_warna, body } => {
                let app_bar_h = if app_bar_judul.is_some() { 56.0 } else { 0.0 };
                let body_batasan = BatasanKotak::ketat(batasan_induk.maks_lebar, batasan_induk.maks_tinggi - app_bar_h);
                let mut body_render = RenderKotakMurni::baru(&body.id, body_batasan);
                body_render.x = self.x;
                body_render.y = self.y + app_bar_h;
                body_render.tata_letak(body, body_batasan);

                self.lebar = batasan_induk.maks_lebar;
                self.tinggi = batasan_induk.maks_tinggi;
                self.anak = vec![body_render];
            }
            JenisWidget::Expanded { flex, anak } => {
                let mut child_render = RenderKotakMurni::baru(&anak.id, batasan_induk);
                child_render.x = self.x;
                child_render.y = self.y;
                child_render.tata_letak(anak, batasan_induk);
                self.lebar = child_render.lebar;
                self.tinggi = child_render.tinggi;
                self.anak = vec![child_render];
            }
        }
    }

    /// Melukis (Painting) Hirarki UI ke Framebuffer
    pub fn cat(&self, widget: &WidgetMurni, fb: &mut FramebufferMurni) {
        match &widget.jenis {
            JenisWidget::Kontainer { warna_latar, radius_sudut, anak, .. } => {
                if let Some(warna) = warna_latar {
                    if *radius_sudut > 0.0 {
                        fb.gambar_kotak_bulat(self.x, self.y, self.lebar, self.tinggi, *radius_sudut, *warna);
                    } else {
                        fb.gambar_kotak(self.x, self.y, self.lebar, self.tinggi, *warna);
                    }
                }
                if let (Some(child_widget), Some(child_render)) = (anak, self.anak.first()) {
                    child_render.cat(child_widget, fb);
                }
            }
            JenisWidget::Teks { konten, gaya } => {
                FontAtlasMurni::gambar_teks(fb, konten, self.x, self.y, gaya);
            }
            JenisWidget::Kolom { anak, .. } => {
                for (i, child_render) in self.anak.iter().enumerate() {
                    if let Some(child_widget) = anak.get(i) {
                        child_render.cat(child_widget, fb);
                    }
                }
            }
            JenisWidget::Baris { anak, .. } => {
                for (i, child_render) in self.anak.iter().enumerate() {
                    if let Some(child_widget) = anak.get(i) {
                        child_render.cat(child_widget, fb);
                    }
                }
            }
            JenisWidget::Tombol { label, warna, .. } => {
                // Kotak Tombol
                fb.gambar_kotak_bulat(self.x, self.y, self.lebar, self.tinggi, 8.0, *warna);
                // Teks Tombol
                let mut gaya = GayaTipografi::default();
                gaya.warna = Warna::PUTIH;
                gaya.ukuran_font = 14.0;
                let text_x = self.x + (self.lebar / 2.0);
                let text_y = self.y + (self.tinggi / 2.0) - 6.0;
                gaya.perataan = PerataanTeks::Tengah;
                FontAtlasMurni::gambar_teks(fb, label, text_x, text_y, &gaya);
            }
            JenisWidget::Scaffold { app_bar_judul, app_bar_warna, body } => {
                // Gambar App Bar jika ada
                if let (Some(judul), Some(warna_bar)) = (app_bar_judul, app_bar_warna) {
                    fb.gambar_kotak(self.x, self.y, self.lebar, 56.0, *warna_bar);
                    let mut gaya = GayaTipografi::default();
                    gaya.warna = Warna::PUTIH;
                    gaya.ukuran_font = 16.0;
                    FontAtlasMurni::gambar_teks(fb, judul, self.x + 16.0, self.y + 18.0, &gaya);
                }
                // Gambar Body
                if let Some(body_render) = self.anak.first() {
                    body_render.cat(body, fb);
                }
            }
            JenisWidget::Expanded { anak, .. } => {
                if let Some(child_render) = self.anak.first() {
                    child_render.cat(anak, fb);
                }
            }
        }
    }
}
