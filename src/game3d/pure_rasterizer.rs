//! Pure Sovereign Zero-Dependency Software 3D Rasterizer Engine (pure_rasterizer.rs)
//!
//! 100% In-House CPU Software Rendering Pipeline:
//! - 3D Perspective Camera (LookAt & Perspective Matrix)
//! - Sub-Pixel Barycentric Coordinate Triangle Rasterization
//! - Z-Buffer (Depth Buffer) Hidden Surface Removal
//! - Configurable Backface Culling & Directional Lighting Flat Shading
//! - Direct Output to Pure `ImageBuffer` (RgbaPixel) tanpa Driver GPU / Vulkan / OpenGL

use crate::game3d::math3d::{Mat4, Vec3};
use crate::multimedia::image::{ImageBuffer, RgbaPixel};

/// Vertex 3D untuk Software Rasterizer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoftwareVertex {
    pub position: Vec3,
    pub color: RgbaPixel,
}

impl SoftwareVertex {
    pub fn new(position: Vec3, color: RgbaPixel) -> Self {
        Self { position, color }
    }
}

/// Segitiga 3D
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SoftwareTriangle {
    pub v0: SoftwareVertex,
    pub v1: SoftwareVertex,
    pub v2: SoftwareVertex,
}

impl SoftwareTriangle {
    pub fn new(v0: SoftwareVertex, v1: SoftwareVertex, v2: SoftwareVertex) -> Self {
        Self { v0, v1, v2 }
    }

    /// Hitung vektor normal permukaan segitiga di ruang objek/dunia
    pub fn normal(&self) -> Vec3 {
        let edge1 = self.v1.position - self.v0.position;
        let edge2 = self.v2.position - self.v0.position;
        let n = edge1.cross(edge2);
        let len = n.length();
        if len > 1e-6 {
            n / len
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        }
    }
}

/// Model Geometris 3D
#[derive(Debug, Clone)]
pub struct SoftwareMesh {
    pub triangles: Vec<SoftwareTriangle>,
}

impl SoftwareMesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    /// Bangkitkan Geometri Kubus 3D dengan Normal Menghadap ke Luar
    pub fn create_cube(size: f32, color: RgbaPixel) -> Self {
        let h = size * 0.5;
        let mut mesh = Self::new();

        // 8 Titik sudut kubus
        let p000 = Vec3::new(-h, -h, -h);
        let p001 = Vec3::new(-h, -h,  h);
        let p010 = Vec3::new(-h,  h, -h);
        let p011 = Vec3::new(-h,  h,  h);
        let p100 = Vec3::new( h, -h, -h);
        let p101 = Vec3::new( h, -h,  h);
        let p110 = Vec3::new( h,  h, -h);
        let p111 = Vec3::new( h,  h,  h);

        let mut add_quad = |a, b, c, d| {
            let va = SoftwareVertex::new(a, color);
            let vb = SoftwareVertex::new(b, color);
            let vc = SoftwareVertex::new(c, color);
            let vd = SoftwareVertex::new(d, color);
            // a -> b -> c dan a -> c -> d
            mesh.triangles.push(SoftwareTriangle::new(va, vb, vc));
            mesh.triangles.push(SoftwareTriangle::new(va, vc, vd));
        };

        // 6 Sisi kubus dengan urutan outward CCW normal
        add_quad(p001, p101, p111, p011); // Depan (+Z)
        add_quad(p100, p000, p010, p110); // Belakang (-Z)
        add_quad(p000, p001, p011, p010); // Kiri (-X)
        add_quad(p101, p100, p110, p111); // Kanan (+X)
        add_quad(p011, p111, p110, p010); // Atas (+Y)
        add_quad(p000, p100, p101, p001); // Bawah (-Y)

        mesh
    }
}

/// Software 3D Rasterizer Engine
pub struct SoftwareRasterizer {
    pub width: u32,
    pub height: u32,
    pub z_buffer: Vec<f32>,
    pub light_dir: Vec3,
    pub cull_backface: bool,
}

impl SoftwareRasterizer {
    pub fn new(width: u32, height: u32) -> Self {
        let total = (width as usize) * (height as usize);
        Self {
            width,
            height,
            z_buffer: vec![f32::INFINITY; total],
            light_dir: Vec3::new(0.5, 0.8, 0.6).normalize(),
            cull_backface: false,
        }
    }

    /// Bersihkan Z-buffer kedalaman
    pub fn clear_depth(&mut self) {
        self.z_buffer.fill(f32::INFINITY);
    }

    /// Render satu mesh 3D ke dalam ImageBuffer
    pub fn render_mesh(
        &mut self,
        mesh: &SoftwareMesh,
        model_matrix: &Mat4,
        view_proj_matrix: &Mat4,
        framebuffer: &mut ImageBuffer,
    ) {
        let mvp = model_matrix.multiply(view_proj_matrix);
        let w_f32 = self.width as f32;
        let h_f32 = self.height as f32;

        for tri in &mesh.triangles {
            // 1. Shading arah cahaya
            let normal_world = tri.normal();
            let n_dot_l = normal_world.dot(self.light_dir).abs();
            let shade_factor = 0.3 + 0.7 * n_dot_l;

            let c = tri.v0.color;
            let shaded_color = RgbaPixel {
                r: ((c.r as f32) * shade_factor).clamp(0.0, 255.0) as u8,
                g: ((c.g as f32) * shade_factor).clamp(0.0, 255.0) as u8,
                b: ((c.b as f32) * shade_factor).clamp(0.0, 255.0) as u8,
                a: c.a,
            };

            // 2. Transformasi ke Screen Coordinates
            let p0 = mvp.transform_point(tri.v0.position);
            let p1 = mvp.transform_point(tri.v1.position);
            let p2 = mvp.transform_point(tri.v2.position);

            // Screen Mapping
            let sx0 = (p0.x * 0.5 + 0.5) * w_f32;
            let sy0 = (1.0 - (p0.y * 0.5 + 0.5)) * h_f32;
            let sx1 = (p1.x * 0.5 + 0.5) * w_f32;
            let sy1 = (1.0 - (p1.y * 0.5 + 0.5)) * h_f32;
            let sx2 = (p2.x * 0.5 + 0.5) * w_f32;
            let sy2 = (1.0 - (p2.y * 0.5 + 0.5)) * h_f32;

            // 3. Area Luas Segitiga di Layar
            let area = (sx1 - sx0) * (sy2 - sy0) - (sy1 - sy0) * (sx2 - sx0);
            if area.abs() < 1e-5 {
                continue; // Segitiga terdegradasi
            }

            if self.cull_backface && area < 0.0 {
                continue;
            }

            // 4. Bounding Box Segitiga
            let min_x = (sx0.min(sx1).min(sx2).floor() as i32).max(0) as u32;
            let max_x = (sx0.max(sx1).max(sx2).ceil() as i32).min(self.width as i32 - 1).max(0) as u32;
            let min_y = (sy0.min(sy1).min(sy2).floor() as i32).max(0) as u32;
            let max_y = (sy0.max(sy1).max(sy2).ceil() as i32).min(self.height as i32 - 1).max(0) as u32;

            let inv_area = 1.0 / area;

            // 5. Barycentric Rasterization Loop
            for y in min_y..=max_y {
                let py = y as f32 + 0.5;
                for x in min_x..=max_x {
                    let px = x as f32 + 0.5;

                    let w0 = (sx2 - sx1) * (py - sy1) - (sy2 - sy1) * (px - sx1);
                    let w1 = (sx0 - sx2) * (py - sy2) - (sy0 - sy2) * (px - sx2);
                    let w2 = (sx1 - sx0) * (py - sy0) - (sy1 - sy0) * (px - sx0);

                    let inside = if area > 0.0 {
                        w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                    } else {
                        w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                    };

                    if inside {
                        let l0 = w0 * inv_area;
                        let l1 = w1 * inv_area;
                        let l2 = w2 * inv_area;

                        let z = l0 * p0.z + l1 * p1.z + l2 * p2.z;
                        let idx = (y * self.width + x) as usize;

                        // 6. Z-Buffer Depth Test
                        if z < self.z_buffer[idx] {
                            self.z_buffer[idx] = z;
                            framebuffer.pixels[idx] = shaded_color;
                        }
                    }
                }
            }
        }
    }
}
