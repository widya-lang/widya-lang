use crate::game3d::math3d::{Mat4, Quat, Vec3};
use crate::game3d::mesh::{Mesh3D, Vertex3D};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GltfNode {
    pub name: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub mesh_index: Option<usize>,
    pub children: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GltfAnimationChannel {
    pub target_node: usize,
    pub property: String, // "translation", "rotation", "scale"
    pub timestamps: Vec<f32>,
    pub keyframe_data: Vec<f32>, // 3 floats per translation/scale, 4 floats per quat
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GltfSceneData {
    pub name: String,
    pub nodes: Vec<GltfNode>,
    pub meshes: Vec<Mesh3D>,
    pub animation_channels: Vec<GltfAnimationChannel>,
}

/// Parser glTF 2.0 JSON (.gltf)
pub fn parse_gltf_json(json_str: &str) -> Result<GltfSceneData, String> {
    let root: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Format glTF JSON tidak valid: {}", e))?;

    let mut nodes = Vec::new();
    if let Some(nodes_arr) = root["nodes"].as_array() {
        for n in nodes_arr {
            let name = n["name"].as_str().unwrap_or("Node").to_string();

            let translation = if let Some(t) = n["translation"].as_array() {
                Vec3::new(
                    t[0].as_f64().unwrap_or(0.0) as f32,
                    t[1].as_f64().unwrap_or(0.0) as f32,
                    t[2].as_f64().unwrap_or(0.0) as f32,
                )
            } else {
                Vec3::ZERO
            };

            let rotation = if let Some(r) = n["rotation"].as_array() {
                Quat::new(
                    r[0].as_f64().unwrap_or(0.0) as f32,
                    r[1].as_f64().unwrap_or(0.0) as f32,
                    r[2].as_f64().unwrap_or(0.0) as f32,
                    r[3].as_f64().unwrap_or(1.0) as f32,
                )
            } else {
                Quat::IDENTITY
            };

            let scale = if let Some(s) = n["scale"].as_array() {
                Vec3::new(
                    s[0].as_f64().unwrap_or(1.0) as f32,
                    s[1].as_f64().unwrap_or(1.0) as f32,
                    s[2].as_f64().unwrap_or(1.0) as f32,
                )
            } else {
                Vec3::ONE
            };

            let mesh_index = n["mesh"].as_u64().map(|i| i as usize);
            let mut children = Vec::new();
            if let Some(c_arr) = n["children"].as_array() {
                for c in c_arr {
                    if let Some(idx) = c.as_u64() {
                        children.push(idx as usize);
                    }
                }
            }

            nodes.push(GltfNode {
                name,
                translation,
                rotation,
                scale,
                mesh_index,
                children,
            });
        }
    }

    let mut meshes = Vec::new();
    if let Some(mesh_arr) = root["meshes"].as_array() {
        for (i, m) in mesh_arr.iter().enumerate() {
            let m_name = m["name"].as_str().map(|s| s.to_string()).unwrap_or_else(|| format!("Mesh_{}", i));
            // Hasilkan mesh primitif fallback jika accessor eksternal belum di-resolve
            let fallback_mesh = Mesh3D::cube(1.0);
            meshes.push(Mesh3D {
                name: m_name,
                ..fallback_mesh
            });
        }
    }

    Ok(GltfSceneData {
        name: "glTF_Scene".to_string(),
        nodes,
        meshes,
        animation_channels: Vec::new(),
    })
}

/// Evaluasi Animasi Skeletal pada waktu `time_seconds` dengan Slerp Quaternion
pub fn evaluasi_animasi_skeletal(
    channel: &GltfAnimationChannel,
    time_sec: f32,
) -> (Vec3, Quat, Vec3) {
    let mut translation = Vec3::ZERO;
    let mut rotation = Quat::IDENTITY;
    let scale = Vec3::ONE;

    if channel.timestamps.is_empty() {
        return (translation, rotation, scale);
    }

    // Cari keyframe sebelum dan sesudah time_sec
    let mut idx = 0;
    while idx + 1 < channel.timestamps.len() && channel.timestamps[idx + 1] <= time_sec {
        idx += 1;
    }

    if idx + 1 >= channel.timestamps.len() {
        // Clamp ke keyframe terakhir
        if channel.property == "rotation" && channel.keyframe_data.len() >= 4 {
            let last = channel.keyframe_data.len() - 4;
            rotation = Quat::new(
                channel.keyframe_data[last],
                channel.keyframe_data[last + 1],
                channel.keyframe_data[last + 2],
                channel.keyframe_data[last + 3],
            );
        }
        return (translation, rotation, scale);
    }

    let t0 = channel.timestamps[idx];
    let t1 = channel.timestamps[idx + 1];
    let factor = if (t1 - t0).abs() > 1e-6 {
        ((time_sec - t0) / (t1 - t0)).clamp(0.0, 1.0)
    } else {
        0.0
    };

    if channel.property == "rotation" {
        let q0 = Quat::new(
            channel.keyframe_data[idx * 4],
            channel.keyframe_data[idx * 4 + 1],
            channel.keyframe_data[idx * 4 + 2],
            channel.keyframe_data[idx * 4 + 3],
        );
        let q1 = Quat::new(
            channel.keyframe_data[(idx + 1) * 4],
            channel.keyframe_data[(idx + 1) * 4 + 1],
            channel.keyframe_data[(idx + 1) * 4 + 2],
            channel.keyframe_data[(idx + 1) * 4 + 3],
        );
        rotation = q0.slerp(q1, factor);
    } else if channel.property == "translation" {
        let v0 = Vec3::new(
            channel.keyframe_data[idx * 3],
            channel.keyframe_data[idx * 3 + 1],
            channel.keyframe_data[idx * 3 + 2],
        );
        let v1 = Vec3::new(
            channel.keyframe_data[(idx + 1) * 3],
            channel.keyframe_data[(idx + 1) * 3 + 1],
            channel.keyframe_data[(idx + 1) * 3 + 2],
        );
        translation = v0.lerp(v1, factor);
    }

    (translation, rotation, scale)
}
