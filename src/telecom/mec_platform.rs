use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// ETSI Multi-Access Edge Computing (MEC) Platform & Dynamic Service Registry
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MecAppStatus {
    Starting,
    Running,
    Degraded,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MecServiceInfo {
    pub service_id: String,
    pub service_name: String,
    pub version: String,
    pub endpoint_uri: String,
    pub latency_sla_ms: f64,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MecAppInstance {
    pub app_id: String,
    pub app_name: String,
    pub cluster_zone: String,
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub status: MecAppStatus,
    pub exposed_services: Vec<MecServiceInfo>,
}

pub struct MecPlatform {
    pub platform_id: String,
    pub region: String,
    pub apps: HashMap<String, MecAppInstance>,
    pub services: HashMap<String, MecServiceInfo>,
}

impl MecPlatform {
    pub fn new(platform_id: &str, region: &str) -> Self {
        Self {
            platform_id: platform_id.to_string(),
            region: region.to_string(),
            apps: HashMap::new(),
            services: HashMap::new(),
        }
    }

    /// Mendaftarkan aplikasi MEC baru (ETSI GS MEC 011 App Lifecycle)
    pub fn register_app(&mut self, app_id: &str, app_name: &str, zone: &str, cpu: u32, mem_mb: u64) {
        let app = MecAppInstance {
            app_id: app_id.to_string(),
            app_name: app_name.to_string(),
            cluster_zone: zone.to_string(),
            cpu_cores: cpu,
            memory_mb: mem_mb,
            status: MecAppStatus::Running,
            exposed_services: Vec::new(),
        };
        self.apps.insert(app_id.to_string(), app);
    }

    /// Mendaftarkan layanan mikro pada service mesh MEC
    pub fn register_service(&mut self, app_id: &str, service_id: &str, name: &str, uri: &str, latency_ms: f64) -> Result<(), String> {
        if let Some(app) = self.apps.get_mut(app_id) {
            let svc = MecServiceInfo {
                service_id: service_id.to_string(),
                service_name: name.to_string(),
                version: "v1.0".to_string(),
                endpoint_uri: uri.to_string(),
                latency_sla_ms: latency_ms,
                state: "ACTIVE".to_string(),
            };
            app.exposed_services.push(svc.clone());
            self.services.insert(service_id.to_string(), svc);
            Ok(())
        } else {
            Err(format!("Aplikasi MEC {} tidak terdaftar", app_id))
        }
    }

    /// Temukan endpoint layanan edge terdekat
    pub fn lookup_service(&self, service_id: &str) -> Option<&MecServiceInfo> {
        self.services.get(service_id)
    }
}
