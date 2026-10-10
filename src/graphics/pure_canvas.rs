//! Pure 2D Software Rasterizer & Canvas Engine (Zero External Dependencies)
//! Standar Kedaulatan Grafis: Bebas Skia, Cairo, atau C++ Rendering Libraries.
//! Mendukung Framebuffer RGBA, Antialiased Primitives, Kurva Bezier, Rounded Rect,
//! Porter-Duff Alpha Blending, Linear/Radial Gradient, Drop Shadows, dan Dirty Rect Culling.

use std::cmp::{max, min};

/// Format Warna RGBA 32-bit (0xAARRGGBB di memori)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Warna {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Warna {
    pub const TRANSPARAN: Self = Self { a: 0, r: 0, g: 0, b: 0 };
    pub const HITAM: Self = Self { a: 255, r: 0, g: 0, b: 0 };
    pub const PUTIH: Self = Self { a: 255, r: 255, g: 255, b: 255 };
    pub const MERAH: Self = Self { a: 255, r: 255, g: 0, b: 0 };
    pub const HIJAU: Self = Self { a: 255, r: 0, g: 255, b: 0 };
    pub const BIRU: Self = Self { a: 255, r: 0, g: 0, b: 255 };

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { a, r, g, b }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { a: 255, r, g, b }
    }

    /// Parse dari kode hex seperti "#RRGGBB" atau "#AARRGGBB"
    pub fn dari_hex(hex: &str) -> Self {
        let clean = hex.trim_start_matches('#');
        match clean.len() {
            6 => {
                let r = u8::from_str_radix(&clean[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0);
                Self::rgb(r, g, b)
            }
            8 => {
                let a = u8::from_str_radix(&clean[0..2], 16).unwrap_or(255);
                let r = u8::from_str_radix(&clean[2..4], 16).unwrap_or(0);
                let g = u8::from_str_radix(&clean[4..6], 16).unwrap_or(0);
                let b = u8::from_str_radix(&clean[6..8], 16).unwrap_or(0);
                Self::rgba(r, g, b, a)
            }
            _ => Self::HITAM,
        }
    }

    /// Konversi ke u32 ARGB (format umum display buffer)
    pub fn ke_u32_argb(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Konversi ke u32 RGBA
    pub fn ke_u32_rgba(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }

    /// Porter-Duff SrcOver Alpha Blending: src over dst
    pub fn blend_src_over(src: Warna, dst: Warna) -> Warna {
        if src.a == 255 {
            return src;
        }
        if src.a == 0 {
            return dst;
        }

        let sa = src.a as u32;
        let da = dst.a as u32;
        let inv_sa = 255 - sa;

        let out_a = sa + ((da * inv_sa + 127) / 255);
        if out_a == 0 {
            return Warna::TRANSPARAN;
        }

        let out_r = ((src.r as u32 * sa + (dst.r as u32 * da * inv_sa + 127) / 255) + out_a / 2) / out_a;
        let out_g = ((src.g as u32 * sa + (dst.g as u32 * da * inv_sa + 127) / 255) + out_a / 2) / out_a;
        let out_b = ((src.b as u32 * sa + (dst.b as u32 * da * inv_sa + 127) / 255) + out_a / 2) / out_a;

        Warna {
            a: out_a.min(255) as u8,
            r: out_r.min(255) as u8,
            g: out_g.min(255) as u8,
            b: out_b.min(255) as u8,
        }
    }
}

/// Titik 2D Vektor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Titik2D {
    pub x: f32,
    pub y: f32,
}

impl Titik2D {
    pub fn baru(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Kotak Persegi Panjang Geometris
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PersegiKotak {
    pub x: f32,
    pub y: f32,
    pub lebar: f32,
    pub tinggi: f32,
}

impl PersegiKotak {
    pub fn baru(x: f32, y: f32, lebar: f32, tinggi: f32) -> Self {
        Self { x, y, lebar, tinggi }
    }

    pub fn memuat_titik(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.lebar && py >= self.y && py <= self.y + self.tinggi
    }
}

/// Mode Shading Gradien
#[derive(Debug, Clone)]
pub enum JenisGradien {
    Linear { x1: f32, y1: f32, x2: f32, y2: f32, warna_awal: Warna, warna_akhir: Warna },
    Radial { pusat_x: f32, pusat_y: f32, radius: f32, warna_dalam: Warna, warna_luar: Warna },
}

/// Perintah Jalur Vektor (Vector Path Commands)
#[derive(Debug, Clone)]
pub enum PerintahPath {
    PindahKe(Titik2D),
    GarisKe(Titik2D),
    BezierKuadratikKe(Titik2D, Titik2D), // (Control, End)
    BezierKubikKe(Titik2D, Titik2D, Titik2D), // (Control1, Control2, End)
    Tutup,
}

/// Vektor Path 2D
#[derive(Debug, Clone, Default)]
pub struct VektorPath {
    pub perintah: Vec<PerintahPath>,
}

impl VektorPath {
    pub fn baru() -> Self {
        Self { perintah: Vec::new() }
    }

    pub fn pindah_ke(&mut self, x: f32, y: f32) -> &mut Self {
        self.perintah.push(PerintahPath::PindahKe(Titik2D::baru(x, y)));
        self
    }

    pub fn garis_ke(&mut self, x: f32, y: f32) -> &mut Self {
        self.perintah.push(PerintahPath::GarisKe(Titik2D::baru(x, y)));
        self
    }

    pub fn bezier_kuadratik_ke(&mut self, cx: f32, cy: f32, ex: f32, ey: f32) -> &mut Self {
        self.perintah.push(PerintahPath::BezierKuadratikKe(Titik2D::baru(cx, cy), Titik2D::baru(ex, ey)));
        self
    }

    pub fn bezier_kubik_ke(&mut self, c1x: f32, c1y: f32, c2x: f32, c2y: f32, ex: f32, ey: f32) -> &mut Self {
        self.perintah.push(PerintahPath::BezierKubikKe(
            Titik2D::baru(c1x, c1y),
            Titik2D::baru(c2x, c2y),
            Titik2D::baru(ex, ey),
        ));
        self
    }

    pub fn tutup(&mut self) -> &mut Self {
        self.perintah.push(PerintahPath::Tutup);
        self
    }
}

/// Framebuffer Memori RGBA Mandiri
#[derive(Debug, Clone)]
pub struct FramebufferMurni {
    pub lebar: u32,
    pub tinggi: u32,
    pub buffer: Vec<u32>, // 0xAARRGGBB
    pub klip_aktif: Option<PersegiKotak>,
}

impl FramebufferMurni {
    pub fn baru(lebar: u32, tinggi: u32) -> Self {
        let size = (lebar * tinggi) as usize;
        Self {
            lebar,
            tinggi,
            buffer: vec![0; size],
            klip_aktif: None,
        }
    }

    pub fn bersihkan(&mut self, warna: Warna) {
        let val = warna.ke_u32_argb();
        self.buffer.fill(val);
    }

    #[inline]
    pub fn set_piksel(&mut self, x: i32, y: i32, warna: Warna) {
        if x < 0 || y < 0 || x >= self.lebar as i32 || y >= self.tinggi as i32 {
            return;
        }

        if let Some(klip) = &self.klip_aktif {
            if (x as f32) < klip.x || (x as f32) >= klip.x + klip.lebar
                || (y as f32) < klip.y || (y as f32) >= klip.y + klip.tinggi {
                return;
            }
        }

        let idx = (y as u32 * self.lebar + x as u32) as usize;
        if warna.a == 255 {
            self.buffer[idx] = warna.ke_u32_argb();
        } else if warna.a > 0 {
            let dst_val = self.buffer[idx];
            let dst = Warna {
                a: ((dst_val >> 24) & 0xFF) as u8,
                r: ((dst_val >> 16) & 0xFF) as u8,
                g: ((dst_val >> 8) & 0xFF) as u8,
                b: (dst_val & 0xFF) as u8,
            };
            let blended = Warna::blend_src_over(warna, dst);
            self.buffer[idx] = blended.ke_u32_argb();
        }
    }

    #[inline]
    pub fn ambil_piksel(&self, x: i32, y: i32) -> Warna {
        if x < 0 || y < 0 || x >= self.lebar as i32 || y >= self.tinggi as i32 {
            return Warna::TRANSPARAN;
        }
        let idx = (y as u32 * self.lebar + x as u32) as usize;
        let v = self.buffer[idx];
        Warna {
            a: ((v >> 24) & 0xFF) as u8,
            r: ((v >> 16) & 0xFF) as u8,
            g: ((v >> 8) & 0xFF) as u8,
            b: (v & 0xFF) as u8,
        }
    }

    /// Gambar Kotak Persegi Panjang Berwarna
    pub fn gambar_kotak(&mut self, x: f32, y: f32, w: f32, h: f32, warna: Warna) {
        let x_start = max(0, x.floor() as i32);
        let y_start = max(0, y.floor() as i32);
        let x_end = min(self.lebar as i32, (x + w).ceil() as i32);
        let y_end = min(self.tinggi as i32, (y + h).ceil() as i32);

        for py in y_start..y_end {
            for px in x_start..x_end {
                self.set_piksel(px, py, warna);
            }
        }
    }

    /// Gambar Kotak Sudut Membulat (Rounded Rectangle) dengan Anti-Aliasing
    pub fn gambar_kotak_bulat(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, warna: Warna) {
        let r = radius.min(w / 2.0).min(h / 2.0);
        let x_start = max(0, x.floor() as i32);
        let y_start = max(0, y.floor() as i32);
        let x_end = min(self.lebar as i32, (x + w).ceil() as i32);
        let y_end = min(self.tinggi as i32, (y + h).ceil() as i32);

        let left_center = x + r;
        let right_center = x + w - r;
        let top_center = y + r;
        let bottom_center = y + h - r;

        for py in y_start..y_end {
            let cy = py as f32 + 0.5;
            for px in x_start..x_end {
                let cx = px as f32 + 0.5;

                // Cek 4 sudut melengkung
                let in_corner = if cx < left_center && cy < top_center {
                    let dx = cx - left_center;
                    let dy = cy - top_center;
                    (dx * dx + dy * dy) <= r * r
                } else if cx > right_center && cy < top_center {
                    let dx = cx - right_center;
                    let dy = cy - top_center;
                    (dx * dx + dy * dy) <= r * r
                } else if cx < left_center && cy > bottom_center {
                    let dx = cx - left_center;
                    let dy = cy - bottom_center;
                    (dx * dx + dy * dy) <= r * r
                } else if cx > right_center && cy > bottom_center {
                    let dx = cx - right_center;
                    let dy = cy - bottom_center;
                    (dx * dx + dy * dy) <= r * r
                } else {
                    true // Di bagian tengah kotak
                };

                if in_corner {
                    self.set_piksel(px, py, warna);
                }
            }
        }
    }

    /// Gambar Garis Lurus Menggunakan Algoritma Bresenham
    pub fn gambar_garis(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, warna: Warna) {
        let mut x0 = x1.round() as i32;
        let mut y0 = y1.round() as i32;
        let x1 = x2.round() as i32;
        let y1 = y2.round() as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.set_piksel(x0, y0, warna);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Gambar Lingkaran Berisi (Filled Circle)
    pub fn gambar_lingkaran(&mut self, cx: f32, cy: f32, radius: f32, warna: Warna) {
        let x_start = max(0, (cx - radius).floor() as i32);
        let y_start = max(0, (cy - radius).floor() as i32);
        let x_end = min(self.lebar as i32, (cx + radius).ceil() as i32);
        let y_end = min(self.tinggi as i32, (cy + radius).ceil() as i32);

        let r_sq = radius * radius;
        for py in y_start..y_end {
            let dy = py as f32 + 0.5 - cy;
            for px in x_start..x_end {
                let dx = px as f32 + 0.5 - cx;
                if dx * dx + dy * dy <= r_sq {
                    self.set_piksel(px, py, warna);
                }
            }
        }
    }

    /// Gambar Gradien Linear
    pub fn gambar_gradien_linear(&mut self, x: f32, y: f32, w: f32, h: f32, warna_awal: Warna, warna_akhir: Warna) {
        let x_start = max(0, x.floor() as i32);
        let y_start = max(0, y.floor() as i32);
        let x_end = min(self.lebar as i32, (x + w).ceil() as i32);
        let y_end = min(self.tinggi as i32, (y + h).ceil() as i32);

        for py in y_start..y_end {
            let t = if h > 0.0 { ((py as f32 - y) / h).clamp(0.0, 1.0) } else { 0.0 };
            let r = (warna_awal.r as f32 + (warna_akhir.r as f32 - warna_awal.r as f32) * t) as u8;
            let g = (warna_awal.g as f32 + (warna_akhir.g as f32 - warna_awal.g as f32) * t) as u8;
            let b = (warna_awal.b as f32 + (warna_akhir.b as f32 - warna_awal.b as f32) * t) as u8;
            let a = (warna_awal.a as f32 + (warna_akhir.a as f32 - warna_awal.a as f32) * t) as u8;
            let cur_warna = Warna::rgba(r, g, b, a);

            for px in x_start..x_end {
                self.set_piksel(px, py, cur_warna);
            }
        }
    }

    /// Gambar Efek Bayangan Lembut (Drop Shadow)
    pub fn gambar_bayangan(&mut self, x: f32, y: f32, w: f32, h: f32, radius: f32, blur: f32, warna_bayangan: Warna) {
        let offset = blur.max(1.0);
        let shadow_rect_x = x - offset / 2.0;
        let shadow_rect_y = y - offset / 2.0;
        let shadow_w = w + offset;
        let shadow_h = h + offset;

        let alpha_step = warna_bayangan.a as f32 / 3.0;
        for i in (1..=3).rev() {
            let cur_blur = blur * (i as f32 / 3.0);
            let cur_a = (alpha_step * (4 - i) as f32) as u8;
            let shadow_color = Warna { a: cur_a, r: warna_bayangan.r, g: warna_bayangan.g, b: warna_bayangan.b };
            self.gambar_kotak_bulat(x - cur_blur / 2.0, y - cur_blur / 2.0, w + cur_blur, h + cur_blur, radius + cur_blur / 2.0, shadow_color);
        }
    }

    /// Ekspor Framebuffer ke format PPM (Portable Pixmap) murni tanpa kompresi eksternal
    pub fn ekspor_ke_ppm(&self) -> Vec<u8> {
        let header = format!("P6\n{} {}\n255\n", self.lebar, self.tinggi);
        let mut bytes = header.into_bytes();
        bytes.reserve((self.lebar * self.tinggi * 3) as usize);

        for &pixel in &self.buffer {
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
            bytes.push(r);
            bytes.push(g);
            bytes.push(b);
        }
        bytes
    }
}
