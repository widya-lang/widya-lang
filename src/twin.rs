//! WidyaTwin — Sovereign Spatial Digital Twin & 3D GIS Engine
//! Provides 3D spatial node hierarchies, real-time IoT telemetry binding,
//! GeoPackage GIS coordinate projection, and WebGPU/Canvas scene generation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialCoordinate {
    pub lat: f64,
    pub lon: f64,
    pub altitude_m: f64,
    pub local_x: f32,
    pub local_y: f32,
    pub local_z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinEntity {
    pub id: String,
    pub name: String,
    pub entity_type: String, // "Gedung", "Sensor_Suhu", "PLTS_Solar", "Pintu_Air", "Drone"
    pub coordinate: SpatialCoordinate,
    pub telemetry_metrics: HashMap<String, f64>,
    pub status: String, // "NORMAL", "WARNING", "CRITICAL"
}

pub struct DigitalTwinScene {
    pub scene_name: String,
    entities: Arc<RwLock<HashMap<String, TwinEntity>>>,
}

impl DigitalTwinScene {
    pub fn new(scene_name: &str) -> Self {
        Self {
            scene_name: scene_name.to_string(),
            entities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_entity(&self, entity: TwinEntity) {
        let mut map = self.entities.write().unwrap();
        map.insert(entity.id.clone(), entity);
    }

    pub fn update_telemetry(&self, entity_id: &str, metric: &str, value: f64) -> Result<(), String> {
        let mut map = self.entities.write().unwrap();
        if let Some(entity) = map.get_mut(entity_id) {
            entity.telemetry_metrics.insert(metric.to_string(), value);
            // Auto status evaluation
            if metric.contains("suhu") && value > 80.0 {
                entity.status = "CRITICAL".to_string();
            }
            Ok(())
        } else {
            Err(format!("Entitas '{}' tidak ditemukan", entity_id))
        }
    }

    pub fn get_entities(&self) -> Vec<TwinEntity> {
        self.entities.read().unwrap().values().cloned().collect()
    }

    pub fn generate_webgpu_scene_html(&self) -> String {
        let entities = self.get_entities();
        let entity_count = entities.len();
        format!(
            r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>WidyaTwin 3D Scene — {name}</title>
    <style>
        body {{ margin: 0; background: #070b14; color: #38bdf8; font-family: sans-serif; overflow: hidden; }}
        #hud {{ position: absolute; top: 16px; left: 16px; background: rgba(15,23,42,0.8); backdrop-filter: blur(10px); padding: 16px; border-radius: 8px; border: 1px solid rgba(56,189,248,0.2); }}
    </style>
</head>
<body>
    <div id="hud">
        <h2>🏙️ Digital Twin: {name}</h2>
        <p>Total Entitas Terpantau: <strong>{count}</strong></p>
    </div>
    <canvas id="renderCanvas" style="width: 100vw; height: 100vh;"></canvas>
</body>
</html>"#,
            name = self.scene_name,
            count = entity_count
        )
    }
}
