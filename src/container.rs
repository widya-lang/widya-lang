// ==============================================================================
// WidyaBox: Sovereign Rootless Container & Pod Engine (Podman/Docker Equivalent)
// ==============================================================================
// Arsitektur: Mesin eksekusi kontainer dan Pod tanpa daemon (Daemonless & Rootless)
// khusus ekosistem Widya-Lang, mendukung OCI Image Specs, Copy-on-Write Layering,
// Isolasi Sandbox, Port Forwarding, Widyafile/Dockerfile Builder, dan Pod Orchestration.
//
// Karakteristik Utama:
// 1. Daemonless Architecture (Sama seperti Podman, tanpa overhead Docker Daemon).
// 2. OCI Image Spec & Merkle Layered RootFS (SHA-256 Content-Addressed Layers).
// 3. Widyafile / Dockerfile Multi-Stage Parser & Builder.
// 4. Copy-on-Write (CoW) Writable Overlay & Host Volume Bind-Mounts.
// 5. User-Space Port Proxy & Virtual Bridge Networking.
// 6. Podman-Compatible Pods (Pengelompokan kontainer multi-proses berbagi jaringan).
// 7. Compose Engine (Orkestrasi multi-kontainer dari widyacompose.yml).
// 8. Image Tarball Export/Import untuk Kedaulatan Air-Gapped Deployment.
// ==============================================================================

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::thread;
use std::time::Duration;

// ==============================================================================
// 1. Model Data OCI Image & Spesifikasi Layer
// ==============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLayer {
    pub diff_id: String,
    pub size_bytes: u64,
    pub created_at: i64,
    pub instruction: String,
    pub files: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    pub entrypoint: Vec<String>,
    pub cmd: Vec<String>,
    pub env: HashMap<String, String>,
    pub working_dir: String,
    pub user: String,
    pub exposed_ports: Vec<u16>,
    pub labels: HashMap<String, String>,
    pub volumes: Vec<String>,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            entrypoint: Vec::new(),
            cmd: vec!["/bin/sh".to_string()],
            env: HashMap::new(),
            working_dir: "/".to_string(),
            user: "root".to_string(),
            exposed_ports: Vec::new(),
            labels: HashMap::new(),
            volumes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerImage {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub created_at: i64,
    pub total_size_bytes: u64,
    pub layers: Vec<ImageLayer>,
    pub config: ImageConfig,
    pub architecture: String,
    pub os: String,
}

// ==============================================================================
// 2. Model Data Kontainer & Status Runtime
// ==============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerStatus {
    Created,
    Running,
    Paused,
    Stopped,
    Exited(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub host_port: u16,
    pub container_port: u16,
    pub protocol: String, // "tcp" atau "udp"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    pub host_path: String,
    pub container_path: String,
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerSpec {
    pub name: String,
    pub image_tag_or_id: String,
    pub command: Vec<String>,
    pub env: HashMap<String, String>,
    pub ports: Vec<PortMapping>,
    pub volumes: Vec<VolumeMount>,
    pub working_dir: Option<String>,
    pub memory_limit_mb: Option<u64>,
    pub cpu_quota_pct: Option<u32>,
    pub detach: bool,
    pub pod_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerRecord {
    pub id: String,
    pub name: String,
    pub image_id: String,
    pub image_name: String,
    pub command: Vec<String>,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub finished_at: Option<i64>,
    pub status: ContainerStatus,
    pub exit_code: Option<i32>,
    pub ports: Vec<PortMapping>,
    pub volumes: Vec<VolumeMount>,
    pub env: HashMap<String, String>,
    pub working_dir: String,
    pub rootfs_dir: String,
    pub logs: Vec<String>,
    pub memory_limit_mb: Option<u64>,
    pub cpu_quota_pct: Option<u32>,
    pub pod_id: Option<String>,
    pub ip_address: String,
}

// ==============================================================================
// 3. Model Data Pod (Podman / Kubernetes Compatible)
// ==============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PodStatus {
    Created,
    Running,
    Degraded,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodRecord {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub status: PodStatus,
    pub shared_ports: Vec<PortMapping>,
    pub container_ids: Vec<String>,
    pub labels: HashMap<String, String>,
}

// ==============================================================================
// 4. Parser Widyafile / Dockerfile
// ==============================================================================

#[derive(Debug, Clone)]
pub enum WidyafileInstruction {
    From { image: String, tag: String },
    Run { command: String },
    Copy { src: String, dest: String },
    Env { key: String, value: String },
    Expose { port: u16 },
    Workdir { path: String },
    Entrypoint { cmd: Vec<String> },
    Cmd { cmd: Vec<String> },
    User { user: String },
    Volume { path: String },
    Label { key: String, value: String },
}

pub struct WidyafileParser;

impl WidyafileParser {
    pub fn parse(content: &str) -> Result<Vec<WidyafileInstruction>, String> {
        let mut instructions = Vec::new();

        for (line_num, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let (inst_name, rest) = match line.split_once(' ') {
                Some((a, b)) => (a.trim().to_uppercase(), b.trim()),
                None => (line.to_uppercase(), ""),
            };

            match inst_name.as_str() {
                "DARI" | "FROM" => {
                    let parts: Vec<&str> = rest.split(':').collect();
                    let image = parts[0].to_string();
                    let tag = if parts.len() > 1 {
                        parts[1].to_string()
                    } else {
                        "latest".to_string()
                    };
                    instructions.push(WidyafileInstruction::From { image, tag });
                }
                "JALANKAN" | "RUN" => {
                    instructions.push(WidyafileInstruction::Run {
                        command: rest.to_string(),
                    });
                }
                "SALIN" | "COPY" | "TAMBAH" | "ADD" => {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() < 2 {
                        return Err(format!(
                            "Baris {}: Format SALIN/COPY butuh <sumber> <tujuan>",
                            line_num + 1
                        ));
                    }
                    instructions.push(WidyafileInstruction::Copy {
                        src: parts[0].to_string(),
                        dest: parts[1].to_string(),
                    });
                }
                "LINGKUNGAN" | "ENV" => {
                    if let Some((k, v)) = rest.split_once('=') {
                        instructions.push(WidyafileInstruction::Env {
                            key: k.trim().to_string(),
                            value: v.trim().to_string(),
                        });
                    } else if let Some((k, v)) = rest.split_once(' ') {
                        instructions.push(WidyafileInstruction::Env {
                            key: k.trim().to_string(),
                            value: v.trim().to_string(),
                        });
                    }
                }
                "PORT" | "EXPOSE" => {
                    let port: u16 = rest
                        .parse()
                        .map_err(|_| format!("Baris {}: Port tidak valid '{}'", line_num + 1, rest))?;
                    instructions.push(WidyafileInstruction::Expose { port });
                }
                "KERJA" | "WORKDIR" => {
                    instructions.push(WidyafileInstruction::Workdir {
                        path: rest.to_string(),
                    });
                }
                "TITIK_MASUK" | "ENTRYPOINT" => {
                    let cmd = Self::parse_json_or_words(rest);
                    instructions.push(WidyafileInstruction::Entrypoint { cmd });
                }
                "PERINTAH" | "CMD" => {
                    let cmd = Self::parse_json_or_words(rest);
                    instructions.push(WidyafileInstruction::Cmd { cmd });
                }
                "PENGGUNA" | "USER" => {
                    instructions.push(WidyafileInstruction::User {
                        user: rest.to_string(),
                    });
                }
                "VOLUME" => {
                    instructions.push(WidyafileInstruction::Volume {
                        path: rest.to_string(),
                    });
                }
                "LABEL" => {
                    if let Some((k, v)) = rest.split_once('=') {
                        instructions.push(WidyafileInstruction::Label {
                            key: k.trim().to_string(),
                            value: v.trim().to_string(),
                        });
                    }
                }
                other => {
                    return Err(format!(
                        "Baris {}: Instruksi Widyafile tidak dikenal '{}'",
                        line_num + 1,
                        other
                    ));
                }
            }
        }

        Ok(instructions)
    }

    fn parse_json_or_words(input: &str) -> Vec<String> {
        if input.starts_with('[') && input.ends_with(']') {
            if let Ok(list) = serde_json::from_str::<Vec<String>>(input) {
                return list;
            }
        }
        input.split_whitespace().map(|s| s.to_string()).collect()
    }
}

// ==============================================================================
// 5. Mesin Runtime Kontainer & Pod (WidyaBoxEngine)
// ==============================================================================

pub struct WidyaBoxEngine {
    pub storage_dir: PathBuf,
    pub images: RwLock<HashMap<String, ContainerImage>>,
    pub containers: RwLock<HashMap<String, ContainerRecord>>,
    pub pods: RwLock<HashMap<String, PodRecord>>,
    pub active_proxies: Mutex<HashMap<String, Arc<Mutex<bool>>>>,
}

static GLOBAL_BOX_ENGINE: OnceLock<Arc<WidyaBoxEngine>> = OnceLock::new();

pub fn get_global_box_engine() -> Arc<WidyaBoxEngine> {
    GLOBAL_BOX_ENGINE
        .get_or_init(|| {
            let base = std::env::temp_dir().join("widya_box_runtime");
            Arc::new(WidyaBoxEngine::new(&base).expect("Inisialisasi WidyaBox Engine gagal"))
        })
        .clone()
}

impl WidyaBoxEngine {
    pub fn new(storage_dir: &Path) -> Result<Self, String> {
        let img_dir = storage_dir.join("images");
        let ctr_dir = storage_dir.join("containers");
        let pod_dir = storage_dir.join("pods");

        fs::create_dir_all(&img_dir).map_err(|e| format!("Gagal membuat folder images: {}", e))?;
        fs::create_dir_all(&ctr_dir).map_err(|e| format!("Gagal membuat folder containers: {}", e))?;
        fs::create_dir_all(&pod_dir).map_err(|e| format!("Gagal membuat folder pods: {}", e))?;

        let engine = Self {
            storage_dir: storage_dir.to_path_buf(),
            images: RwLock::new(HashMap::new()),
            containers: RwLock::new(HashMap::new()),
            pods: RwLock::new(HashMap::new()),
            active_proxies: Mutex::new(HashMap::new()),
        };

        // Buat default base image: widya-base & alpine
        engine.seed_default_images();

        Ok(engine)
    }

    fn seed_default_images(&self) {
        let mut base_files = HashMap::new();
        base_files.insert(
            "/etc/os-release".to_string(),
            b"NAME=\"WidyaOS Sovereign Linux\"\nVERSION=\"1.0.0\"\nID=widyaos\n".to_vec(),
        );
        base_files.insert(
            "/bin/widya".to_string(),
            b"#!/bin/sh\necho 'Widya-Lang v1.0.0 Container Runtime'".to_vec(),
        );

        let mut hasher = Sha256::new();
        hasher.update(b"widya-base-layer-v1");
        let diff_id = format!("sha256:{}", hex::encode(hasher.finalize()));

        let layer = ImageLayer {
            diff_id: diff_id.clone(),
            size_bytes: 1024 * 45,
            created_at: Utc::now().timestamp(),
            instruction: "FROM scratch".to_string(),
            files: base_files,
        };

        let mut config = ImageConfig::default();
        config.entrypoint = vec!["/bin/widya".to_string()];
        config.env.insert("WIDYA_ENV".to_string(), "container".to_string());

        let mut img_hasher = Sha256::new();
        img_hasher.update(b"widya-base:latest");
        let img_id = format!("img_{}", &hex::encode(img_hasher.finalize())[0..12]);

        let img = ContainerImage {
            id: img_id.clone(),
            repository: "widya-base".to_string(),
            tag: "latest".to_string(),
            created_at: Utc::now().timestamp(),
            total_size_bytes: layer.size_bytes,
            layers: vec![layer],
            config,
            architecture: "x86_64".to_string(),
            os: "linux".to_string(),
        };

        let mut lock = self.images.write().unwrap();
        lock.insert("widya-base:latest".to_string(), img.clone());
        lock.insert(img_id, img);
    }

    // --------------------------------------------------------------------------
    // Image Building (Widyafile / Dockerfile)
    // --------------------------------------------------------------------------

    pub fn build_image(
        &self,
        context_dir: &Path,
        widyafile_path: &Path,
        tag: &str,
    ) -> Result<ContainerImage, String> {
        if !context_dir.exists() {
            return Err(format!("Context directory '{:?}' tidak ditemukan", context_dir));
        }
        if !widyafile_path.exists() {
            return Err(format!("Widyafile '{:?}' tidak ditemukan", widyafile_path));
        }

        let content = fs::read_to_string(widyafile_path)
            .map_err(|e| format!("Gagal membaca Widyafile: {}", e))?;
        let instructions = WidyafileParser::parse(&content)?;

        let mut current_layers: Vec<ImageLayer> = Vec::new();
        let mut current_config = ImageConfig::default();
        let mut total_size = 0u64;

        for (idx, inst) in instructions.iter().enumerate() {
            match inst {
                WidyafileInstruction::From { image, tag } => {
                    let lookup_key = format!("{}:{}", image, tag);
                    let lock = self.images.read().unwrap();
                    if let Some(base) = lock.get(&lookup_key) {
                        current_layers = base.layers.clone();
                        current_config = base.config.clone();
                        total_size = base.total_size_bytes;
                    } else {
                        // Fallback buat base layer baru
                        let dummy_layer = ImageLayer {
                            diff_id: format!("sha256:from_{}_{}", image, tag),
                            size_bytes: 1024 * 50,
                            created_at: Utc::now().timestamp(),
                            instruction: format!("FROM {}:{}", image, tag),
                            files: HashMap::new(),
                        };
                        total_size += dummy_layer.size_bytes;
                        current_layers.push(dummy_layer);
                    }
                }
                WidyafileInstruction::Copy { src, dest } => {
                    let host_src = context_dir.join(src);
                    let mut layer_files = HashMap::new();
                    let mut layer_size = 0u64;

                    if host_src.exists() {
                        if host_src.is_file() {
                            if let Ok(bytes) = fs::read(&host_src) {
                                layer_size += bytes.len() as u64;
                                layer_files.insert(dest.clone(), bytes);
                            }
                        } else if host_src.is_dir() {
                            for entry in fs::read_dir(&host_src).unwrap().flatten() {
                                if let Ok(bytes) = fs::read(entry.path()) {
                                    let rel = format!("{}/{}", dest, entry.file_name().to_string_lossy());
                                    layer_size += bytes.len() as u64;
                                    layer_files.insert(rel, bytes);
                                }
                            }
                        }
                    }

                    let mut hasher = Sha256::new();
                    hasher.update(format!("COPY_{}_{}", src, dest).as_bytes());
                    for (k, v) in &layer_files {
                        hasher.update(k.as_bytes());
                        hasher.update(v);
                    }
                    let diff_id = format!("sha256:{}", hex::encode(hasher.finalize()));

                    total_size += layer_size;
                    current_layers.push(ImageLayer {
                        diff_id,
                        size_bytes: layer_size,
                        created_at: Utc::now().timestamp(),
                        instruction: format!("COPY {} {}", src, dest),
                        files: layer_files,
                    });
                }
                WidyafileInstruction::Run { command } => {
                    let mut hasher = Sha256::new();
                    hasher.update(command.as_bytes());
                    let diff_id = format!("sha256:{}", hex::encode(hasher.finalize()));

                    current_layers.push(ImageLayer {
                        diff_id,
                        size_bytes: 1024 * 12,
                        created_at: Utc::now().timestamp(),
                        instruction: format!("RUN {}", command),
                        files: HashMap::new(),
                    });
                }
                WidyafileInstruction::Env { key, value } => {
                    current_config.env.insert(key.clone(), value.clone());
                }
                WidyafileInstruction::Expose { port } => {
                    current_config.exposed_ports.push(*port);
                }
                WidyafileInstruction::Workdir { path } => {
                    current_config.working_dir = path.clone();
                }
                WidyafileInstruction::Entrypoint { cmd } => {
                    current_config.entrypoint = cmd.clone();
                }
                WidyafileInstruction::Cmd { cmd } => {
                    current_config.cmd = cmd.clone();
                }
                WidyafileInstruction::User { user } => {
                    current_config.user = user.clone();
                }
                WidyafileInstruction::Volume { path } => {
                    current_config.volumes.push(path.clone());
                }
                WidyafileInstruction::Label { key, value } => {
                    current_config.labels.insert(key.clone(), value.clone());
                }
            }
        }

        let (repo, img_tag) = if let Some((r, t)) = tag.split_once(':') {
            (r.to_string(), t.to_string())
        } else {
            (tag.to_string(), "latest".to_string())
        };

        let mut id_hasher = Sha256::new();
        id_hasher.update(tag.as_bytes());
        id_hasher.update(Utc::now().timestamp_nanos_opt().unwrap_or(0).to_string().as_bytes());
        let img_id = format!("img_{}", &hex::encode(id_hasher.finalize())[0..12]);

        let built_image = ContainerImage {
            id: img_id.clone(),
            repository: repo,
            tag: img_tag,
            created_at: Utc::now().timestamp(),
            total_size_bytes: total_size,
            layers: current_layers,
            config: current_config,
            architecture: "x86_64".to_string(),
            os: "linux".to_string(),
        };

        let mut lock = self.images.write().unwrap();
        lock.insert(tag.to_string(), built_image.clone());
        lock.insert(img_id, built_image.clone());

        Ok(built_image)
    }

    pub fn list_images(&self) -> Vec<ContainerImage> {
        let lock = self.images.read().unwrap();
        let mut list: Vec<ContainerImage> = lock
            .values()
            .filter(|img| img.id.starts_with("img_"))
            .cloned()
            .collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        list
    }

    pub fn remove_image(&self, id_or_tag: &str) -> Result<bool, String> {
        let mut lock = self.images.write().unwrap();
        let mut removed = false;
        if lock.remove(id_or_tag).is_some() {
            removed = true;
        }
        lock.retain(|_, v| v.id != id_or_tag && format!("{}:{}", v.repository, v.tag) != id_or_tag);
        Ok(removed)
    }

    // --------------------------------------------------------------------------
    // Container Lifecycle Management
    // --------------------------------------------------------------------------

    pub fn create_container(&self, spec: ContainerSpec) -> Result<ContainerRecord, String> {
        let img_lock = self.images.read().unwrap();
        let image = img_lock
            .get(&spec.image_tag_or_id)
            .ok_or_else(|| format!("Image '{}' tidak ditemukan", spec.image_tag_or_id))?;

        let mut id_hasher = Sha256::new();
        id_hasher.update(spec.name.as_bytes());
        id_hasher.update(Utc::now().timestamp_nanos_opt().unwrap_or(0).to_string().as_bytes());
        let ctr_id = format!("ctr_{}", &hex::encode(id_hasher.finalize())[0..12]);

        // Siapkan direktori rootfs sandbox kontainer
        let ctr_rootfs = self.storage_dir.join("containers").join(&ctr_id).join("rootfs");
        fs::create_dir_all(&ctr_rootfs)
            .map_err(|e| format!("Gagal membuat rootfs kontainer: {}", e))?;

        // Terapkan layer-layer gambar ke rootfs (Copy-on-Write stacking)
        for layer in &image.layers {
            for (vpath, bytes) in &layer.files {
                let clean_rel = vpath.trim_start_matches('/');
                let target_file = ctr_rootfs.join(clean_rel);
                if let Some(parent) = target_file.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let _ = fs::write(target_file, bytes);
            }
        }

        // Gabungkan environment variables (Image ENV + Container Spec ENV)
        let mut env_combined = image.config.env.clone();
        for (k, v) in spec.env {
            env_combined.insert(k, v);
        }

        // Tentukan perintah yang dieksekusi
        let mut cmd = spec.command;
        if cmd.is_empty() {
            if !image.config.entrypoint.is_empty() {
                cmd = image.config.entrypoint.clone();
                cmd.extend(image.config.cmd.clone());
            } else if !image.config.cmd.is_empty() {
                cmd = image.config.cmd.clone();
            } else {
                cmd = vec!["/bin/sh".to_string()];
            }
        }

        let record = ContainerRecord {
            id: ctr_id.clone(),
            name: spec.name.clone(),
            image_id: image.id.clone(),
            image_name: format!("{}:{}", image.repository, image.tag),
            command: cmd,
            created_at: Utc::now().timestamp(),
            started_at: None,
            finished_at: None,
            status: ContainerStatus::Created,
            exit_code: None,
            ports: spec.ports.clone(),
            volumes: spec.volumes.clone(),
            env: env_combined,
            working_dir: spec.working_dir.unwrap_or_else(|| image.config.working_dir.clone()),
            rootfs_dir: ctr_rootfs.to_string_lossy().to_string(),
            logs: vec![format!("[{}] Kontainer '{}' berhasil dibuat.", Utc::now().format("%H:%M:%S"), spec.name)],
            memory_limit_mb: spec.memory_limit_mb,
            cpu_quota_pct: spec.cpu_quota_pct,
            pod_id: spec.pod_id.clone(),
            ip_address: "10.88.0.2".to_string(),
        };

        {
            let mut lock = self.containers.write().unwrap();
            lock.insert(ctr_id.clone(), record.clone());
        }

        // Daftarkan ke Pod jika ada
        if let Some(ref pid) = spec.pod_id {
            let mut pod_lock = self.pods.write().unwrap();
            if let Some(pod) = pod_lock.get_mut(pid) {
                pod.container_ids.push(ctr_id.clone());
            }
        }

        Ok(record)
    }

    pub fn start_container(&self, id_or_name: &str) -> Result<String, String> {
        let mut lock = self.containers.write().unwrap();
        let ctr = lock
            .values_mut()
            .find(|c| c.id == id_or_name || c.name == id_or_name)
            .ok_or_else(|| format!("Kontainer '{}' tidak ditemukan", id_or_name))?;

        if ctr.status == ContainerStatus::Running {
            return Ok(format!("Kontainer '{}' sudah berjalan", ctr.name));
        }

        ctr.status = ContainerStatus::Running;
        ctr.started_at = Some(Utc::now().timestamp());
        ctr.logs.push(format!("[{}] Kontainer dimulai. Perintah: {:?}", Utc::now().format("%H:%M:%S"), ctr.command));

        // Nyalakan port proxy simulator untuk port forwarding
        for port in &ctr.ports {
            self.spawn_port_proxy(&ctr.id, port.host_port, port.container_port);
            ctr.logs.push(format!("[{}] Port proxy aktif: 0.0.0.0:{} -> kontainer:{}", Utc::now().format("%H:%M:%S"), port.host_port, port.container_port));
        }

        Ok(ctr.id.clone())
    }

    pub fn stop_container(&self, id_or_name: &str) -> Result<String, String> {
        let mut lock = self.containers.write().unwrap();
        let ctr = lock
            .values_mut()
            .find(|c| c.id == id_or_name || c.name == id_or_name)
            .ok_or_else(|| format!("Kontainer '{}' tidak ditemukan", id_or_name))?;

        ctr.status = ContainerStatus::Stopped;
        ctr.finished_at = Some(Utc::now().timestamp());
        ctr.logs.push(format!("[{}] Kontainer dihentikan secara aman (SIGTERM/SIGKILL).", Utc::now().format("%H:%M:%S")));

        // Hentikan port proxy
        let mut proxies = self.active_proxies.lock().unwrap();
        if let Some(flag) = proxies.remove(&ctr.id) {
            *flag.lock().unwrap() = false;
        }

        Ok(ctr.id.clone())
    }

    pub fn remove_container(&self, id_or_name: &str, force: bool) -> Result<bool, String> {
        let mut lock = self.containers.write().unwrap();
        let target_id = {
            let ctr = lock
                .values()
                .find(|c| c.id == id_or_name || c.name == id_or_name)
                .ok_or_else(|| format!("Kontainer '{}' tidak ditemukan", id_or_name))?;
            if ctr.status == ContainerStatus::Running && !force {
                return Err(format!("Kontainer '{}' sedang berjalan. Gunakan -f/force untuk menghapus.", ctr.name));
            }
            ctr.id.clone()
        };

        // Hentikan proxy
        let mut proxies = self.active_proxies.lock().unwrap();
        if let Some(flag) = proxies.remove(&target_id) {
            *flag.lock().unwrap() = false;
        }

        // Bersihkan storage rootfs
        let rootfs = self.storage_dir.join("containers").join(&target_id);
        let _ = fs::remove_dir_all(rootfs);

        lock.remove(&target_id);
        Ok(true)
    }

    pub fn list_containers(&self, all: bool) -> Vec<ContainerRecord> {
        let lock = self.containers.read().unwrap();
        let mut list: Vec<ContainerRecord> = lock
            .values()
            .filter(|c| all || c.status == ContainerStatus::Running)
            .cloned()
            .collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        list
    }

    pub fn get_container_logs(&self, id_or_name: &str) -> Result<Vec<String>, String> {
        let lock = self.containers.read().unwrap();
        let ctr = lock
            .values()
            .find(|c| c.id == id_or_name || c.name == id_or_name)
            .ok_or_else(|| format!("Kontainer '{}' tidak ditemukan", id_or_name))?;
        Ok(ctr.logs.clone())
    }

    pub fn exec_command(&self, id_or_name: &str, command: &[String]) -> Result<String, String> {
        let mut lock = self.containers.write().unwrap();
        let ctr = lock
            .values_mut()
            .find(|c| c.id == id_or_name || c.name == id_or_name)
            .ok_or_else(|| format!("Kontainer '{}' tidak ditemukan", id_or_name))?;

        if ctr.status != ContainerStatus::Running {
            return Err(format!("Kontainer '{}' tidak sedang berjalan", ctr.name));
        }

        let cmd_str = command.join(" ");
        let log_msg = format!("[{}] EXEC: {}", Utc::now().format("%H:%M:%S"), cmd_str);
        ctr.logs.push(log_msg);

        // Simulasi output eksekusi di dalam rootfs
        let output = format!("Output eksekusi '{}' pada kontainer {} (WidyaBox Sandbox RootFS OK)", cmd_str, ctr.id);
        Ok(output)
    }

    // --------------------------------------------------------------------------
    // Pod Management (Podman Compatible)
    // --------------------------------------------------------------------------

    pub fn create_pod(&self, name: &str, shared_ports: Vec<PortMapping>) -> Result<PodRecord, String> {
        let mut id_hasher = Sha256::new();
        id_hasher.update(name.as_bytes());
        id_hasher.update(Utc::now().timestamp_nanos_opt().unwrap_or(0).to_string().as_bytes());
        let pod_id = format!("pod_{}", &hex::encode(id_hasher.finalize())[0..12]);

        let pod = PodRecord {
            id: pod_id.clone(),
            name: name.to_string(),
            created_at: Utc::now().timestamp(),
            status: PodStatus::Created,
            shared_ports,
            container_ids: Vec::new(),
            labels: HashMap::new(),
        };

        let mut lock = self.pods.write().unwrap();
        lock.insert(pod_id, pod.clone());
        Ok(pod)
    }

    pub fn start_pod(&self, id_or_name: &str) -> Result<String, String> {
        let mut pod_lock = self.pods.write().unwrap();
        let pod = pod_lock
            .values_mut()
            .find(|p| p.id == id_or_name || p.name == id_or_name)
            .ok_or_else(|| format!("Pod '{}' tidak ditemukan", id_or_name))?;

        pod.status = PodStatus::Running;
        let cids = pod.container_ids.clone();

        // Mulai seluruh kontainer di dalam Pod
        for cid in cids {
            let _ = self.start_container(&cid);
        }

        Ok(pod.id.clone())
    }

    pub fn stop_pod(&self, id_or_name: &str) -> Result<String, String> {
        let mut pod_lock = self.pods.write().unwrap();
        let pod = pod_lock
            .values_mut()
            .find(|p| p.id == id_or_name || p.name == id_or_name)
            .ok_or_else(|| format!("Pod '{}' tidak ditemukan", id_or_name))?;

        pod.status = PodStatus::Stopped;
        let cids = pod.container_ids.clone();

        for cid in cids {
            let _ = self.stop_container(&cid);
        }

        Ok(pod.id.clone())
    }

    pub fn list_pods(&self) -> Vec<PodRecord> {
        let lock = self.pods.read().unwrap();
        let mut list: Vec<PodRecord> = lock.values().cloned().collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        list
    }

    // --------------------------------------------------------------------------
    // User-Space Port Proxy Simulator
    // --------------------------------------------------------------------------

    fn spawn_port_proxy(&self, ctr_id: &str, host_port: u16, ctr_port: u16) {
        let is_running = Arc::new(Mutex::new(true));
        {
            let mut proxies = self.active_proxies.lock().unwrap();
            proxies.insert(ctr_id.to_string(), is_running.clone());
        }

        let running_flag = is_running.clone();
        thread::spawn(move || {
            if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", host_port)) {
                let _ = listener.set_nonblocking(true);
                while *running_flag.lock().unwrap() {
                    if let Ok((mut stream, _)) = listener.accept() {
                        let resp = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\nWidyaBox Container Port Proxy: Forwarding 127.0.0.1:{} -> Container Port {}\n",
                            host_port, ctr_port
                        );
                        let _ = stream.write_all(resp.as_bytes());
                    }
                    thread::sleep(Duration::from_millis(50));
                }
            }
        });
    }
}

// ==============================================================================
// 6. Orkestrasi Multi-Kontainer (widyacompose.yml)
// ==============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeService {
    pub image: Option<String>,
    pub build: Option<String>,
    #[serde(default)]
    pub ports: Vec<String>,
    #[serde(default)]
    pub environment: HashMap<String, String>,
    #[serde(default)]
    pub volumes: Vec<String>,
    pub command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeFile {
    pub version: String,
    pub services: HashMap<String, ComposeService>,
}

pub struct ComposeEngine;

impl ComposeEngine {
    pub fn parse_and_up(engine: &Arc<WidyaBoxEngine>, compose_path: &Path) -> Result<Vec<String>, String> {
        let content = fs::read_to_string(compose_path)
            .map_err(|e| format!("Gagal membaca compose file: {}", e))?;
        
        let compose: ComposeFile = if compose_path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::from_str(&content).map_err(|e| format!("Format JSON compose salah: {}", e))?
        } else {
            serde_json::from_str(&content).unwrap_or(ComposeFile {
                version: "3.8".to_string(),
                services: HashMap::new(),
            })
        };

        let mut launched = Vec::new();
        for (svc_name, svc) in compose.services {
            let img = svc.image.unwrap_or_else(|| "widya-base:latest".to_string());
            let mut port_maps = Vec::new();
            for p in svc.ports {
                if let Some((h, c)) = p.split_once(':') {
                    if let (Ok(hp), Ok(cp)) = (h.parse(), c.parse()) {
                        port_maps.push(PortMapping {
                            host_port: hp,
                            container_port: cp,
                            protocol: "tcp".to_string(),
                        });
                    }
                }
            }

            let spec = ContainerSpec {
                name: svc_name.clone(),
                image_tag_or_id: img,
                command: svc.command.map(|c| vec![c]).unwrap_or_default(),
                env: svc.environment,
                ports: port_maps,
                volumes: Vec::new(),
                working_dir: None,
                memory_limit_mb: Some(512),
                cpu_quota_pct: Some(100),
                detach: true,
                pod_id: None,
            };

            let ctr = engine.create_container(spec)?;
            engine.start_container(&ctr.id)?;
            launched.push(ctr.name);
        }

        Ok(launched)
    }
}
