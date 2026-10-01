//! Engine Pengolahan & Editing Foto / Citra Digital Widya-Lang (image.rs)
//! Mendukung manipulasi raw RGBA buffer, Color Grading, 2D Convolution Kernels, Blending Modes, dan Geometric Transforms.

use std::f64::consts::PI;

/// Representasi Piksel RGBA 8-bit per kanal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbaPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RgbaPixel {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
    pub const TRANSPARENT: Self = Self { r: 0, g: 0, b: 0, a: 0 };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

/// Mode Pencampuran Layer (Blending Modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    Additive,
    Difference,
}

/// Buffer Citra RGBA Contiguous
#[derive(Debug, Clone, PartialEq)]
pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<RgbaPixel>,
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32, fill: RgbaPixel) -> Self {
        let total = (width as usize) * (height as usize);
        Self {
            width,
            height,
            pixels: vec![fill; total],
        }
    }

    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RgbaPixel> {
        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = (y as usize * self.width as usize) + x as usize;
            Some(self.pixels[idx])
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: RgbaPixel) -> bool {
        if x >= self.width || y >= self.height {
            false
        } else {
            let idx = (y as usize * self.width as usize) + x as usize;
            self.pixels[idx] = pixel;
            true
        }
    }

    // ==========================================
    // 1. PENYESUAIAN WARNA & COLOR GRADING
    // ==========================================

    /// Penyesuaian Kecerahan (Brightness): faktor -1.0 s/d 1.0
    pub fn adjust_brightness(&mut self, factor: f32) {
        let offset = (factor * 255.0) as i32;
        for p in &mut self.pixels {
            p.r = (p.r as i32 + offset).clamp(0, 255) as u8;
            p.g = (p.g as i32 + offset).clamp(0, 255) as u8;
            p.b = (p.b as i32 + offset).clamp(0, 255) as u8;
        }
    }

    /// Penyesuaian Kontras (Contrast): faktor > 0.0 (1.0 = normal)
    pub fn adjust_contrast(&mut self, contrast: f32) {
        for p in &mut self.pixels {
            let r = (((p.r as f32 / 255.0 - 0.5) * contrast + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
            let g = (((p.g as f32 / 255.0 - 0.5) * contrast + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
            let b = (((p.b as f32 / 255.0 - 0.5) * contrast + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
            p.r = r;
            p.g = g;
            p.b = b;
        }
    }

    /// Penyesuaian Saturasi (Saturation): 0.0 = Grayscale, 1.0 = Normal, > 1.0 = Lebih Pekat
    pub fn adjust_saturation(&mut self, saturation: f32) {
        for p in &mut self.pixels {
            // Formula Luminans Standar ITU-R BT.601
            let gray = 0.299 * (p.r as f32) + 0.587 * (p.g as f32) + 0.114 * (p.b as f32);
            p.r = (gray + (p.r as f32 - gray) * saturation).clamp(0.0, 255.0) as u8;
            p.g = (gray + (p.g as f32 - gray) * saturation).clamp(0.0, 255.0) as u8;
            p.b = (gray + (p.b as f32 - gray) * saturation).clamp(0.0, 255.0) as u8;
        }
    }

    /// Konversi ke Grayscale murni
    pub fn to_grayscale(&mut self) {
        self.adjust_saturation(0.0);
    }

    /// Invert / Negatif Foto
    pub fn invert(&mut self) {
        for p in &mut self.pixels {
            p.r = 255 - p.r;
            p.g = 255 - p.g;
            p.b = 255 - p.b;
        }
    }

    /// Efek Tone Sepia Klasik
    pub fn apply_sepia(&mut self) {
        for p in &mut self.pixels {
            let r = p.r as f32;
            let g = p.g as f32;
            let b = p.b as f32;

            let tr = (0.393 * r + 0.769 * g + 0.189 * b).clamp(0.0, 255.0) as u8;
            let tg = (0.349 * r + 0.686 * g + 0.168 * b).clamp(0.0, 255.0) as u8;
            let tb = (0.272 * r + 0.534 * g + 0.131 * b).clamp(0.0, 255.0) as u8;

            p.r = tr;
            p.g = tg;
            p.b = tb;
        }
    }

    // ==========================================
    // 2. 2D CONVOLUTION KERNEL FILTERS
    // ==========================================

    /// Menerapkan matriks konvolusi 3x3 ke citra
    pub fn apply_kernel_3x3(&self, kernel: [[f32; 3]; 3], factor: f32, bias: f32) -> ImageBuffer {
        let mut output = ImageBuffer::new(self.width, self.height, RgbaPixel::TRANSPARENT);

        for y in 0..self.height {
            for x in 0..self.width {
                let mut sum_r = 0.0;
                let mut sum_g = 0.0;
                let mut sum_b = 0.0;

                for ky in 0..3 {
                    for kx in 0..3 {
                        let px = (x as i32 + kx - 1).clamp(0, self.width as i32 - 1) as u32;
                        let py = (y as i32 + ky - 1).clamp(0, self.height as i32 - 1) as u32;

                        if let Some(pixel) = self.get_pixel(px, py) {
                            let weight = kernel[ky as usize][kx as usize];
                            sum_r += (pixel.r as f32) * weight;
                            sum_g += (pixel.g as f32) * weight;
                            sum_b += (pixel.b as f32) * weight;
                        }
                    }
                }

                let orig_alpha = self.get_pixel(x, y).map(|p| p.a).unwrap_or(255);
                let r = ((sum_r * factor) + bias).clamp(0.0, 255.0) as u8;
                let g = ((sum_g * factor) + bias).clamp(0.0, 255.0) as u8;
                let b = ((sum_b * factor) + bias).clamp(0.0, 255.0) as u8;

                output.set_pixel(x, y, RgbaPixel::new(r, g, b, orig_alpha));
            }
        }
        output
    }

    /// Gaussian Blur 3x3
    pub fn gaussian_blur(&self) -> ImageBuffer {
        let kernel = [
            [1.0, 2.0, 1.0],
            [2.0, 4.0, 2.0],
            [1.0, 2.0, 1.0],
        ];
        self.apply_kernel_3x3(kernel, 1.0 / 16.0, 0.0)
    }

    /// Sharpening (Penajaman Detail)
    pub fn sharpen(&self) -> ImageBuffer {
        let kernel = [
            [ 0.0, -1.0,  0.0],
            [-1.0,  5.0, -1.0],
            [ 0.0, -1.0,  0.0],
        ];
        self.apply_kernel_3x3(kernel, 1.0, 0.0)
    }

    /// Sobel Edge Detection (Deteksi Garis Batas)
    pub fn edge_detect_sobel(&self) -> ImageBuffer {
        let kernel_x = [
            [-1.0, 0.0, 1.0],
            [-2.0, 0.0, 2.0],
            [-1.0, 0.0, 1.0],
        ];
        let kernel_y = [
            [-1.0, -2.0, -1.0],
            [ 0.0,  0.0,  0.0],
            [ 1.0,  2.0,  1.0],
        ];

        let mut output = ImageBuffer::new(self.width, self.height, RgbaPixel::BLACK);

        for y in 0..self.height {
            for x in 0..self.width {
                let mut gx = 0.0;
                let mut gy = 0.0;

                for ky in 0..3 {
                    for kx in 0..3 {
                        let px = (x as i32 + kx - 1).clamp(0, self.width as i32 - 1) as u32;
                        let py = (y as i32 + ky - 1).clamp(0, self.height as i32 - 1) as u32;

                        if let Some(pixel) = self.get_pixel(px, py) {
                            let gray = 0.299 * (pixel.r as f32) + 0.587 * (pixel.g as f32) + 0.114 * (pixel.b as f32);
                            gx += gray * kernel_x[ky as usize][kx as usize];
                            gy += gray * kernel_y[ky as usize][kx as usize];
                        }
                    }
                }

                let magnitude = ((gx * gx + gy * gy).sqrt()).clamp(0.0, 255.0) as u8;
                output.set_pixel(x, y, RgbaPixel::rgb(magnitude, magnitude, magnitude));
            }
        }
        output
    }

    // ==========================================
    // 3. TRANSFORMASI GEOMETRIS
    // ==========================================

    /// Crop / Pemotongan area gambar
    pub fn crop(&self, start_x: u32, start_y: u32, crop_width: u32, crop_height: u32) -> ImageBuffer {
        let mut output = ImageBuffer::new(crop_width, crop_height, RgbaPixel::TRANSPARENT);
        for y in 0..crop_height {
            for x in 0..crop_width {
                let src_x = start_x + x;
                let src_y = start_y + y;
                if let Some(pixel) = self.get_pixel(src_x, src_y) {
                    output.set_pixel(x, y, pixel);
                }
            }
        }
        output
    }

    /// Flip Horizontal
    pub fn flip_horizontal(&self) -> ImageBuffer {
        let mut output = ImageBuffer::new(self.width, self.height, RgbaPixel::TRANSPARENT);
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(pixel) = self.get_pixel(x, y) {
                    output.set_pixel(self.width - 1 - x, y, pixel);
                }
            }
        }
        output
    }

    /// Flip Vertical
    pub fn flip_vertical(&self) -> ImageBuffer {
        let mut output = ImageBuffer::new(self.width, self.height, RgbaPixel::TRANSPARENT);
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(pixel) = self.get_pixel(x, y) {
                    output.set_pixel(x, self.height - 1 - y, pixel);
                }
            }
        }
        output
    }

    /// Resize dengan interpolasi Bilinear
    pub fn resize_bilinear(&self, new_width: u32, new_height: u32) -> ImageBuffer {
        let mut output = ImageBuffer::new(new_width, new_height, RgbaPixel::TRANSPARENT);
        let x_ratio = if new_width > 1 { (self.width - 1) as f32 / (new_width - 1) as f32 } else { 0.0 };
        let y_ratio = if new_height > 1 { (self.height - 1) as f32 / (new_height - 1) as f32 } else { 0.0 };

        for y in 0..new_height {
            for x in 0..new_width {
                let gx = x as f32 * x_ratio;
                let gy = y as f32 * y_ratio;
                let gxi = gx.floor() as u32;
                let gyi = gy.floor() as u32;
                let x_diff = gx - (gxi as f32);
                let y_diff = gy - (gyi as f32);

                let p00 = self.get_pixel(gxi, gyi).unwrap_or(RgbaPixel::BLACK);
                let p10 = self.get_pixel((gxi + 1).min(self.width - 1), gyi).unwrap_or(RgbaPixel::BLACK);
                let p01 = self.get_pixel(gxi, (gyi + 1).min(self.height - 1)).unwrap_or(RgbaPixel::BLACK);
                let p11 = self.get_pixel((gxi + 1).min(self.width - 1), (gyi + 1).min(self.height - 1)).unwrap_or(RgbaPixel::BLACK);

                let interp = |c00: u8, c10: u8, c01: u8, c11: u8| -> u8 {
                    let top = (c00 as f32) * (1.0 - x_diff) + (c10 as f32) * x_diff;
                    let bot = (c01 as f32) * (1.0 - x_diff) + (c11 as f32) * x_diff;
                    (top * (1.0 - y_diff) + bot * y_diff).round().clamp(0.0, 255.0) as u8
                };

                let r = interp(p00.r, p10.r, p01.r, p11.r);
                let g = interp(p00.g, p10.g, p01.g, p11.g);
                let b = interp(p00.b, p10.b, p01.b, p11.b);
                let a = interp(p00.a, p10.a, p01.a, p11.a);

                output.set_pixel(x, y, RgbaPixel::new(r, g, b, a));
            }
        }
        output
    }

    // ==========================================
    // 4. LAYER COMPOSITING & BLENDING MODES
    // ==========================================

    /// Menumpuk layer gambar lain di atas kanvas dengan Blend Mode dan Opasitas
    pub fn blend_layer(&mut self, overlay: &ImageBuffer, dest_x: i32, dest_y: i32, mode: BlendMode, opacity: f32) {
        let op = opacity.clamp(0.0, 1.0);

        for oy in 0..overlay.height {
            for ox in 0..overlay.width {
                let dx = dest_x + ox as i32;
                let dy = dest_y + oy as i32;

                if dx < 0 || dx >= self.width as i32 || dy < 0 || dy >= self.height as i32 {
                    continue;
                }

                let top = overlay.get_pixel(ox, oy).unwrap_or(RgbaPixel::TRANSPARENT);
                if top.a == 0 {
                    continue;
                }

                let base = self.get_pixel(dx as u32, dy as u32).unwrap_or(RgbaPixel::BLACK);

                let blended_rgb = match mode {
                    BlendMode::Normal => (top.r, top.g, top.b),
                    BlendMode::Multiply => (
                        ((base.r as u32 * top.r as u32) / 255) as u8,
                        ((base.g as u32 * top.g as u32) / 255) as u8,
                        ((base.b as u32 * top.b as u32) / 255) as u8,
                    ),
                    BlendMode::Screen => (
                        (255 - ((255 - base.r as u32) * (255 - top.r as u32) / 255)) as u8,
                        (255 - ((255 - base.g as u32) * (255 - top.g as u32) / 255)) as u8,
                        (255 - ((255 - base.b as u32) * (255 - top.b as u32) / 255)) as u8,
                    ),
                    BlendMode::Overlay => {
                        let blend_ch = |b: u8, t: u8| -> u8 {
                            if b < 128 {
                                ((2 * b as u32 * t as u32) / 255) as u8
                            } else {
                                (255 - (2 * (255 - b as u32) * (255 - t as u32) / 255)) as u8
                            }
                        };
                        (blend_ch(base.r, top.r), blend_ch(base.g, top.g), blend_ch(base.b, top.b))
                    }
                    BlendMode::Darken => (base.r.min(top.r), base.g.min(top.g), base.b.min(top.b)),
                    BlendMode::Lighten => (base.r.max(top.r), base.g.max(top.g), base.b.max(top.b)),
                    BlendMode::Additive => (
                        (base.r as u32 + top.r as u32).min(255) as u8,
                        (base.g as u32 + top.g as u32).min(255) as u8,
                        (base.b as u32 + top.b as u32).min(255) as u8,
                    ),
                    BlendMode::Difference => (
                        (base.r as i32 - top.r as i32).abs() as u8,
                        (base.g as i32 - top.g as i32).abs() as u8,
                        (base.b as i32 - top.b as i32).abs() as u8,
                    ),
                };

                // Alpha blending dengan opasitas
                let top_alpha_norm = (top.a as f32 / 255.0) * op;
                let out_r = ((base.r as f32) * (1.0 - top_alpha_norm) + (blended_rgb.0 as f32) * top_alpha_norm).round() as u8;
                let out_g = ((base.g as f32) * (1.0 - top_alpha_norm) + (blended_rgb.1 as f32) * top_alpha_norm).round() as u8;
                let out_b = ((base.b as f32) * (1.0 - top_alpha_norm) + (blended_rgb.2 as f32) * top_alpha_norm).round() as u8;
                let out_a = ((base.a as f32).max(top.a as f32 * op)) as u8;

                self.set_pixel(dx as u32, dy as u32, RgbaPixel::new(out_r, out_g, out_b, out_a));
            }
        }
    }
}
