use crate::game3d::math3d::Vec3;
use serde::{Deserialize, Serialize};

/// Material Berbasis Fisika Nyata (PBR - Physically Based Rendering)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PbrMaterial {
    pub name: String,
    pub albedo: [f32; 4], // RGBA (0.0 .. 1.0)
    pub metallic: f32,    // 0.0 = Dielectric (Kayu/Plastik), 1.0 = Logam Murni
    pub roughness: f32,   // 0.0 = Cermin Sempurna, 1.0 = Sangat Kasar/Doff
    pub ao: f32,          // Ambient Occlusion (0.0 .. 1.0)
    pub emissive_color: [f32; 3],
    pub emissive_intensity: f32,
    pub albedo_texture_path: Option<String>,
    pub normal_texture_path: Option<String>,
    pub roughness_metallic_texture_path: Option<String>,
}

impl Default for PbrMaterial {
    fn default() -> Self {
        Self {
            name: "DefaultPBR".to_string(),
            albedo: [0.8, 0.8, 0.8, 1.0],
            metallic: 0.1,
            roughness: 0.5,
            ao: 1.0,
            emissive_color: [0.0, 0.0, 0.0],
            emissive_intensity: 0.0,
            albedo_texture_path: None,
            normal_texture_path: None,
            roughness_metallic_texture_path: None,
        }
    }
}

impl PbrMaterial {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn gold() -> Self {
        Self {
            name: "Emas".to_string(),
            albedo: [1.0, 0.78, 0.34, 1.0],
            metallic: 1.0,
            roughness: 0.2,
            ..Default::default()
        }
    }

    pub fn copper() -> Self {
        Self {
            name: "Tembaga".to_string(),
            albedo: [0.95, 0.64, 0.54, 1.0],
            metallic: 0.95,
            roughness: 0.3,
            ..Default::default()
        }
    }

    pub fn plastic(color: [f32; 4]) -> Self {
        Self {
            name: "Plastik".to_string(),
            albedo: color,
            metallic: 0.0,
            roughness: 0.4,
            ..Default::default()
        }
    }
}

/// Sumber Cahaya Matahari / Directional
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: [f32; 3],
    pub intensity: f32,
    pub cast_shadows: bool,
}

impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            direction: Vec3::new(-0.5, -1.0, -0.3).normalize(),
            color: [1.0, 0.98, 0.9],
            intensity: 1.5,
            cast_shadows: true,
        }
    }
}

/// Sumber Cahaya Titik / Point Light
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointLight {
    pub position: Vec3,
    pub color: [f32; 3],
    pub intensity: f32,
    pub range: f32,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 3.0, 0.0),
            color: [1.0, 1.0, 1.0],
            intensity: 10.0,
            range: 15.0,
        }
    }
}

/// Cook-Torrance BRDF Shading Model Evaluator
pub fn hitung_pbr_shading(
    material: &PbrMaterial,
    normal: Vec3,
    view_dir: Vec3,
    light_dir: Vec3,
    light_color: [f32; 3],
    light_intensity: f32,
) -> [f32; 3] {
    let n = normal.normalize();
    let v = view_dir.normalize();
    let l = light_dir.normalize();
    let h = (v + l).normalize();

    let n_dot_v = n.dot(v).max(1e-4);
    let n_dot_l = n.dot(l).max(0.0);
    let n_dot_h = n.dot(h).max(0.0);
    let v_dot_h = v.dot(h).max(0.0);

    if n_dot_l <= 0.0 {
        return [0.0, 0.0, 0.0];
    }

    // 1. Fresnel-Schlick Approximation (F0 = 0.04 untuk dielectric, albedo untuk logam)
    let f0 = if material.metallic > 0.5 {
        [material.albedo[0], material.albedo[1], material.albedo[2]]
    } else {
        [0.04, 0.04, 0.04]
    };

    let f_term = [
        f0[0] + (1.0 - f0[0]) * (1.0 - v_dot_h).powi(5),
        f0[1] + (1.0 - f0[1]) * (1.0 - v_dot_h).powi(5),
        f0[2] + (1.0 - f0[2]) * (1.0 - v_dot_h).powi(5),
    ];

    // 2. Normal Distribution Function (Trowbridge-Reitz GGX)
    let a = material.roughness * material.roughness;
    let a2 = (a * a).max(1e-4);
    let denom_d = (n_dot_h * n_dot_h) * (a2 - 1.0) + 1.0;
    let d_term = a2 / (std::f32::consts::PI * denom_d * denom_d);

    // 3. Geometry Shadowing Function (Smith's Schlick-GGX)
    let k = ((material.roughness + 1.0) * (material.roughness + 1.0)) / 8.0;
    let g_v = n_dot_v / (n_dot_v * (1.0 - k) + k);
    let g_l = n_dot_l / (n_dot_l * (1.0 - k) + k);
    let g_term = g_v * g_l;

    // Spekular Cook-Torrance
    let denom_spec = 4.0 * n_dot_v * n_dot_l;
    let spec_factor = (d_term * g_term) / denom_spec.max(1e-4);

    // Difus Lambertian (dengan konservasi energi kD = 1 - kS)
    let kd = (1.0 - material.metallic) * (1.0 - f_term[0]);
    let diffuse = [
        kd * (material.albedo[0] / std::f32::consts::PI),
        kd * (material.albedo[1] / std::f32::consts::PI),
        kd * (material.albedo[2] / std::f32::consts::PI),
    ];

    let radiance = [
        light_color[0] * light_intensity,
        light_color[1] * light_intensity,
        light_color[2] * light_intensity,
    ];

    [
        (diffuse[0] + f_term[0] * spec_factor) * radiance[0] * n_dot_l,
        (diffuse[1] + f_term[1] * spec_factor) * radiance[1] * n_dot_l,
        (diffuse[2] + f_term[2] * spec_factor) * radiance[2] * n_dot_l,
    ]
}

/// Tone Mapping ACES Filmic untuk Tampilan HDR Sinematik
pub fn tone_mapping_aces_filmic(hdr: [f32; 3]) -> [f32; 3] {
    fn aces(x: f32) -> f32 {
        let a = 2.51;
        let b = 0.03;
        let c = 2.43;
        let d = 0.59;
        let e = 0.14;
        ((x * (a * x + b)) / (x * (c * x + d) + e)).clamp(0.0, 1.0)
    }

    [aces(hdr[0]), aces(hdr[1]), aces(hdr[2])]
}
