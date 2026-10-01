use crate::game3d::math3d::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub tangent: [f32; 4],
    pub color: [f32; 4],
    pub bone_weights: [f32; 4],
    pub bone_indices: [u32; 4],
}

impl Default for Vertex3D {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            uv: [0.0, 0.0],
            tangent: [1.0, 0.0, 0.0, 1.0],
            color: [1.0, 1.0, 1.0, 1.0],
            bone_weights: [1.0, 0.0, 0.0, 0.0],
            bone_indices: [0, 0, 0, 0],
        }
    }
}

impl Vertex3D {
    pub fn new(pos: [f32; 3], normal: [f32; 3], uv: [f32; 2]) -> Self {
        Self {
            position: pos,
            normal,
            uv,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh3D {
    pub name: String,
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<u32>,
    pub min_bounds: Vec3,
    pub max_bounds: Vec3,
}

impl Mesh3D {
    pub fn new(name: &str, vertices: Vec<Vertex3D>, indices: Vec<u32>) -> Self {
        let mut min_bounds = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max_bounds = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        for v in &vertices {
            min_bounds.x = min_bounds.x.min(v.position[0]);
            min_bounds.y = min_bounds.y.min(v.position[1]);
            min_bounds.z = min_bounds.z.min(v.position[2]);

            max_bounds.x = max_bounds.x.max(v.position[0]);
            max_bounds.y = max_bounds.y.max(v.position[1]);
            max_bounds.z = max_bounds.z.max(v.position[2]);
        }

        Self {
            name: name.to_string(),
            vertices,
            indices,
            min_bounds,
            max_bounds,
        }
    }

    /// Primitif: Kubus 3D (Cube)
    pub fn cube(size: f32) -> Self {
        let h = size * 0.5;
        let positions = [
            // Depan (+Z)
            [-h, -h,  h], [ h, -h,  h], [ h,  h,  h], [-h,  h,  h],
            // Belakang (-Z)
            [ h, -h, -h], [-h, -h, -h], [-h,  h, -h], [ h,  h, -h],
            // Atas (+Y)
            [-h,  h,  h], [ h,  h,  h], [ h,  h, -h], [-h,  h, -h],
            // Bawah (-Y)
            [-h, -h, -h], [ h, -h, -h], [ h, -h,  h], [-h, -h,  h],
            // Kanan (+X)
            [ h, -h,  h], [ h, -h, -h], [ h,  h, -h], [ h,  h,  h],
            // Kiri (-X)
            [-h, -h, -h], [-h, -h,  h], [-h,  h,  h], [-h,  h, -h],
        ];

        let normals = [
            [0.0, 0.0, 1.0], [0.0, 0.0, -1.0],
            [0.0, 1.0, 0.0], [0.0, -1.0, 0.0],
            [1.0, 0.0, 0.0], [-1.0, 0.0, 0.0],
        ];

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..6 {
            let base_idx = (i * 4) as u32;
            let norm = normals[i];
            let uvs = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

            for j in 0..4 {
                vertices.push(Vertex3D::new(positions[i * 4 + j], norm, uvs[j]));
            }

            indices.extend_from_slice(&[
                base_idx, base_idx + 1, base_idx + 2,
                base_idx, base_idx + 2, base_idx + 3,
            ]);
        }

        Self::new("Cube", vertices, indices)
    }

    /// Primitif: Bola 3D (Sphere / UV Sphere)
    pub fn sphere(radius: f32, rings: u32, sectors: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let r_step = std::f32::consts::PI / (rings as f32);
        let s_step = (std::f32::consts::PI * 2.0) / (sectors as f32);

        for r in 0..=rings {
            let phi = (r as f32) * r_step;
            for s in 0..=sectors {
                let theta = (s as f32) * s_step;

                let x = radius * phi.sin() * theta.cos();
                let y = radius * phi.cos();
                let z = radius * phi.sin() * theta.sin();

                let norm = [x / radius, y / radius, z / radius];
                let u = (s as f32) / (sectors as f32);
                let v = (r as f32) / (rings as f32);

                vertices.push(Vertex3D::new([x, y, z], norm, [u, v]));
            }
        }

        for r in 0..rings {
            for s in 0..sectors {
                let first = r * (sectors + 1) + s;
                let second = first + sectors + 1;

                indices.push(first);
                indices.push(second);
                indices.push(first + 1);

                indices.push(second);
                indices.push(second + 1);
                indices.push(first + 1);
            }
        }

        Self::new("Sphere", vertices, indices)
    }

    /// Primitif: Bidang Datar 3D (Plane / Ground)
    pub fn plane(width: f32, depth: f32) -> Self {
        let hw = width * 0.5;
        let hd = depth * 0.5;

        let vertices = vec![
            Vertex3D::new([-hw, 0.0, -hd], [0.0, 1.0, 0.0], [0.0, 0.0]),
            Vertex3D::new([ hw, 0.0, -hd], [0.0, 1.0, 0.0], [1.0, 0.0]),
            Vertex3D::new([ hw, 0.0,  hd], [0.0, 1.0, 0.0], [1.0, 1.0]),
            Vertex3D::new([-hw, 0.0,  hd], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ];

        let indices = vec![0, 2, 1, 0, 3, 2];
        Self::new("Plane", vertices, indices)
    }

    /// Primitif: Kapsul 3D (Capsule untuk Karakter Pemain)
    pub fn capsule(radius: f32, height: f32, rings: u32, segments: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let cylinder_height = (height - 2.0 * radius).max(0.0);
        let half_cyl = cylinder_height * 0.5;

        // Generator vertex silinder dan hemisphere
        for r in 0..=rings {
            let v_ratio = (r as f32) / (rings as f32);
            let phi = v_ratio * std::f32::consts::PI;
            let is_top = r <= rings / 2;
            let y_offset = if is_top { half_cyl } else { -half_cyl };

            for s in 0..=segments {
                let theta = (s as f32) * std::f32::consts::PI * 2.0 / (segments as f32);
                let x = radius * phi.sin() * theta.cos();
                let y = radius * phi.cos() + y_offset;
                let z = radius * phi.sin() * theta.sin();

                let norm = [x / radius, phi.cos(), z / radius];
                vertices.push(Vertex3D::new([x, y, z], norm, [(s as f32)/(segments as f32), v_ratio]));
            }
        }

        for r in 0..rings {
            for s in 0..segments {
                let first = r * (segments + 1) + s;
                let second = first + segments + 1;
                indices.push(first);
                indices.push(second);
                indices.push(first + 1);

                indices.push(second);
                indices.push(second + 1);
                indices.push(first + 1);
            }
        }

        Self::new("Capsule", vertices, indices)
    }

    /// Parser Wavefront .OBJ 3D Model
    pub fn parse_obj(obj_content: &str) -> Result<Self, String> {
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut vertices: Vec<Vertex3D> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for line in obj_content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            match parts[0] {
                "v" if parts.len() >= 4 => {
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    positions.push([x, y, z]);
                }
                "vn" if parts.len() >= 4 => {
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    normals.push([x, y, z]);
                }
                "vt" if parts.len() >= 3 => {
                    let u: f32 = parts[1].parse().unwrap_or(0.0);
                    let v: f32 = parts[2].parse().unwrap_or(0.0);
                    uvs.push([u, v]);
                }
                "f" if parts.len() >= 4 => {
                    let mut face_indices = Vec::new();
                    for part in &parts[1..] {
                        let sub: Vec<&str> = part.split('/').collect();
                        let p_idx: usize = sub[0].parse::<usize>().unwrap_or(1) - 1;
                        let uv_idx: Option<usize> = sub.get(1).and_then(|s| s.parse::<usize>().ok().map(|i| i - 1));
                        let n_idx: Option<usize> = sub.get(2).and_then(|s| s.parse::<usize>().ok().map(|i| i - 1));

                        let pos = positions.get(p_idx).copied().unwrap_or([0.0, 0.0, 0.0]);
                        let uv = uv_idx.and_then(|i| uvs.get(i).copied()).unwrap_or([0.0, 0.0]);
                        let norm = n_idx.and_then(|i| normals.get(i).copied()).unwrap_or([0.0, 1.0, 0.0]);

                        let v_idx = vertices.len() as u32;
                        vertices.push(Vertex3D::new(pos, norm, uv));
                        face_indices.push(v_idx);
                    }

                    // Triangulate face polygon jika quad atau N-gon
                    for i in 1..face_indices.len() - 1 {
                        indices.push(face_indices[0]);
                        indices.push(face_indices[i]);
                        indices.push(face_indices[i + 1]);
                    }
                }
                _ => {}
            }
        }

        Ok(Self::new("ImportedObj", vertices, indices))
    }

    /// Ekspor Mesh ke Format Wavefront .OBJ
    pub fn export_obj(&self) -> String {
        let mut obj = String::new();
        obj.push_str(&format!("# Widya-Lang 3D Engine Export: {}\n", self.name));
        obj.push_str(&format!("o {}\n", self.name));

        for v in &self.vertices {
            obj.push_str(&format!("v {:.4} {:.4} {:.4}\n", v.position[0], v.position[1], v.position[2]));
        }
        for v in &self.vertices {
            obj.push_str(&format!("vt {:.4} {:.4}\n", v.uv[0], v.uv[1]));
        }
        for v in &self.vertices {
            obj.push_str(&format!("vn {:.4} {:.4} {:.4}\n", v.normal[0], v.normal[1], v.normal[2]));
        }

        for chunk in self.indices.chunks(3) {
            if chunk.len() == 3 {
                let i1 = chunk[0] + 1;
                let i2 = chunk[1] + 1;
                let i3 = chunk[2] + 1;
                obj.push_str(&format!("f {i1}/{i1}/{i1} {i2}/{i2}/{i2} {i3}/{i3}/{i3}\n"));
            }
        }

        obj
    }
}
