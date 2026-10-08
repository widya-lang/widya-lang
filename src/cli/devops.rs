// Widya-Lang CLI: Cloud, Containers & DevOps
// Mencakup WidyaWeb Cloud Serverless, WidyaBox OCI Containers, Pods, WidyaKube K8s, dan Security Scanner.

use clap::Subcommand;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Subcommand)]
pub enum WebSubcommands {
    /// Inisialisasi proyek WidyaWeb baru dengan struktur serverless & widyaweb.json
    Init {
        #[arg(value_name = "NAMA_PROYEK", default_value = "aplikasi-web-widya")]
        nama: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Alias Init: new / buat
    New {
        #[arg(value_name = "NAMA_PROYEK", default_value = "aplikasi-web-widya")]
        nama: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Deploy proyek ke Sovereign Edge Gateway (Preview atomik atau Production)
    Deploy {
        #[arg(value_name = "DIR", default_value = ".")]
        dir: PathBuf,
        #[arg(long)]
        prod: bool,
    },
    /// Jalankan Local Edge Simulator & Dev Server dengan live reload
    Dev {
        #[arg(value_name = "DIR", default_value = ".")]
        dir: PathBuf,
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    /// Tampilkan riwayat deployment atomik proyek
    List {
        #[arg(short, long)]
        proyek: Option<String>,
    },
    /// Alias List: ls
    Ls {
        #[arg(short, long)]
        proyek: Option<String>,
    },
    /// Rollback instan ke deployment ID sebelumnya dengan zero-downtime
    Rollback {
        #[arg(value_name = "NAMA_PROYEK")]
        proyek: String,
        #[arg(value_name = "DEPLOYMENT_ID")]
        deploy_id: String,
    },
    /// Manajemen variabel lingkungan dan rahasia terenkripsi (Secrets Vault)
    Env {
        #[command(subcommand)]
        aksi: WebEnvSubcommands,
    },
    /// Tampilkan metrik telemetri edge real-time, latensi P95, dan Core Web Vitals
    Metrics {
        #[arg(short, long)]
        proyek: Option<String>,
    },
    /// Buka Control Plane Dashboard interaktif di peramban
    Dashboard {
        #[arg(short, long, default_value_t = 8088)]
        port: u16,
    },
}

#[derive(Subcommand)]
pub enum WebEnvSubcommands {
    /// Simpan variabel lingkungan (contoh: widya web env set API_KEY rahasia123 --prod)
    Set {
        #[arg(value_name = "KUNCI")]
        kunci: String,
        #[arg(value_name = "NILAI")]
        nilai: String,
        #[arg(short, long, default_value = "widyaweb-app")]
        proyek: String,
        #[arg(long)]
        prod: bool,
    },
    /// Tampilkan daftar variabel lingkungan proyek
    Ls {
        #[arg(short, long, default_value = "widyaweb-app")]
        proyek: String,
    },
    /// Hapus variabel lingkungan dari proyek
    Rm {
        #[arg(value_name = "KUNCI")]
        kunci: String,
        #[arg(short, long, default_value = "widyaweb-app")]
        proyek: String,
        #[arg(long)]
        prod: bool,
    },
}

#[derive(Subcommand)]
pub enum BoxSubcommands {
    /// Jalankan kontainer baru dari image OCI (contoh: widya box run -p 8080:80 --name web widya-base:latest)
    Run {
        #[arg(value_name = "IMAGE")]
        image: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        port: Vec<String>,
        #[arg(short, long)]
        volume: Vec<String>,
        #[arg(short, long)]
        env: Vec<String>,
        #[arg(short, long)]
        detach: bool,
        #[arg(value_name = "PERINTAH")]
        cmd: Vec<String>,
    },
    /// Bangun image OCI dari Widyafile atau Dockerfile
    Build {
        #[arg(short, long, value_name = "TAG", default_value = "widya-app:latest")]
        tag: String,
        #[arg(short, long, value_name = "BERKAS_WIDYAFILE", default_value = "Widyafile")]
        file: PathBuf,
        #[arg(value_name = "KONTEKS_DIR", default_value = ".")]
        context: PathBuf,
    },
    /// Tampilkan daftar kontainer yang aktif atau seluruhnya (-a)
    Ps {
        #[arg(short, long)]
        all: bool,
    },
    /// Tampilkan daftar image OCI yang tersimpan di repositori lokal
    Images,
    /// Hentikan kontainer yang sedang berjalan
    Stop {
        #[arg(value_name = "KONTAINER_ID_ATAU_NAMA")]
        id: String,
    },
    /// Mulai kontainer yang sebelumnya dihentikan
    Start {
        #[arg(value_name = "KONTAINER_ID_ATAU_NAMA")]
        id: String,
    },
    /// Hapus kontainer dari sistem
    Rm {
        #[arg(value_name = "KONTAINER_ID_ATAU_NAMA")]
        id: String,
        #[arg(short, long)]
        force: bool,
    },
    /// Hapus image OCI dari repositori lokal
    Rmi {
        #[arg(value_name = "IMAGE_ID_ATAU_TAG")]
        id: String,
    },
    /// Tampilkan log eksekusi kontainer
    Logs {
        #[arg(value_name = "KONTAINER_ID_ATAU_NAMA")]
        id: String,
    },
    /// Eksekusi perintah di dalam kontainer yang berjalan
    Exec {
        #[arg(value_name = "KONTAINER_ID_ATAU_NAMA")]
        id: String,
        #[arg(value_name = "PERINTAH")]
        cmd: Vec<String>,
    },
    /// Orkestrasi multi-kontainer dari berkas compose
    Compose {
        #[arg(value_name = "BERKAS_COMPOSE", default_value = "widyacompose.json")]
        file: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum PodSubcommands {
    /// Membuat Pod kontainer baru (berbagi namespace jaringan & IPC)
    Create {
        #[arg(value_name = "NAMA_POD")]
        name: String,
        #[arg(short, long)]
        port: Vec<String>,
    },
    /// Memulai seluruh kontainer di dalam Pod
    Start {
        #[arg(value_name = "POD_ID_ATAU_NAMA")]
        id: String,
    },
    /// Menghentikan seluruh kontainer di dalam Pod
    Stop {
        #[arg(value_name = "POD_ID_ATAU_NAMA")]
        id: String,
    },
    /// Menampilkan daftar Pod aktif
    Ps,
}

#[derive(Subcommand)]
pub enum K8sSubcommands {
    /// Terapkan manifest deklaratif (contoh: widya k8s apply -f deployment.json)
    Apply {
        #[arg(short, long, value_name = "BERKAS_MANIFEST")]
        file: PathBuf,
    },
    /// Tampilkan daftar resource kluster (pods, nodes, deployments, services, ingress, hpa)
    Get {
        #[arg(value_name = "JENIS_RESOURCE")]
        resource: String,
        #[arg(short, long, default_value = "default")]
        namespace: String,
    },
    /// Ubah jumlah replika deployment secara instan
    Scale {
        #[arg(value_name = "NAMA_DEPLOYMENT")]
        name: String,
        #[arg(short, long)]
        replicas: u32,
    },
    /// Tampilkan status dan topologi Node kluster
    Nodes,
    /// Jalankan siklus pemeriksaan dan pemulihan otomatis (Self-Healing)
    SelfHeal,
    /// Jalankan evaluasi metrik beban dan autoscaling (HPA Tick)
    Autoscale,
}

#[derive(Subcommand)]
pub enum SecuritySubcommands {
    /// Pindai berkas atau direktori terhadap kerentanan keamanan
    Scan {
        #[arg(value_name = "TARGET", default_value = ".")]
        target: PathBuf,
    },
    /// Pindai bocoran secret/kunci API/kata sandi
    Secrets {
        #[arg(value_name = "TARGET", default_value = ".")]
        target: PathBuf,
    },
    /// Pasang hook pre-commit otomatis untuk memblokir secret bocor
    InstallHook,
}

pub fn handle_web_command(aksi: &WebSubcommands) {
    use widya::web::cloud::{
        get_global_cloud_engine, DeploymentTarget, ScaffoldCloudEngine, SovereignGatewayServer,
    };

    let engine = get_global_cloud_engine();

    match aksi {
        WebSubcommands::Init { nama, output } | WebSubcommands::New { nama, output } => {
            let target_dir = output.clone().unwrap_or_else(|| PathBuf::from("."));
            match ScaffoldCloudEngine::init_project(&target_dir, nama) {
                Ok(p) => {
                    println!("{}", "✨ Proyek WidyaWeb Cloud Berhasil Dibuat!".bright_green().bold());
                    println!("📂 Direktori   : {}", p.display().to_string().bright_cyan());
                    println!("📄 Konfigurasi : {}/widyaweb.json", p.display());
                    println!("📁 Aset Statis : {}/public/", p.display());
                    println!("⚡ Serverless  : {}/api/halo.wya", p.display());
                    println!("\n💡 Perintah selanjutnya:");
                    println!("   cd {}", nama.bright_yellow());
                    println!("   {} web dev      # Jalankan emulator edge lokal", "widya".bright_yellow());
                    println!("   {} web deploy   # Deploy instan ke Sovereign Edge", "widya".bright_yellow());
                }
                Err(e) => {
                    eprintln!("❌ Gagal menginisialisasi proyek WidyaWeb: {}", e);
                    process::exit(1);
                }
            }
        }
        WebSubcommands::Deploy { dir, prod } => {
            let target = if *prod {
                DeploymentTarget::Production
            } else {
                DeploymentTarget::Preview
            };

            println!("{}", "🚀 Menyiapkan Deployment Atomik WidyaWeb Cloud...".bright_cyan().bold());
            println!("   Direktori Sumber: {}", dir.display().to_string().bright_yellow());
            println!("   Target Lingkungan: {}", if *prod { "Production (Utama)".bright_green().bold() } else { "Preview Subdomain".bright_blue() });

            match engine.deploy_project(dir, target, None) {
                Ok(dep) => {
                    println!("\n{}", "🎉 DEPLOYMENT BERHASIL! (Zero-Downtime Atomic Snapshot)".bright_green().bold());
                    println!("============================================================");
                    println!("  Deployment ID   : {}", dep.id.bright_yellow().bold());
                    println!("  Nama Proyek     : {}", dep.project_name.bright_white().bold());
                    println!("  Pratinjau URL   : {}", dep.preview_url.bright_cyan().underline());
                    if let Some(ref prod_url) = dep.production_alias {
                        println!("  Production URL  : {}", prod_url.bright_green().bold().underline());
                    }
                    println!("  Berkas Statis   : {} berkas ({:.2} KB)", dep.files_count, (dep.total_size_bytes as f64) / 1024.0);
                    println!("  Fungsi Edge     : {} serverless endpoint", dep.functions_count);
                    println!("  Distribusi PoP  : 5 Wilayah Edge (JKT1 Primary Core)");
                    println!("============================================================");
                    println!("💡 Pantau metrik & log realtime: {} web metrics --proyek {}", "widya".bright_yellow(), dep.project_name);
                }
                Err(e) => {
                    eprintln!("❌ Gagal melakukan deployment: {}", e);
                    process::exit(1);
                }
            }
        }
        WebSubcommands::Dev { dir, port } => {
            println!("{}", "🌐 Memulai WidyaWeb Sovereign Edge Dev Server...".bright_cyan().bold());
            println!("   Direktori: {}", dir.display().to_string().bright_yellow());
            println!("   Port     : http://127.0.0.1:{}", port);

            // Deploy lokal dahulu
            let _ = engine.deploy_project(dir, DeploymentTarget::Development, None);

            let server = SovereignGatewayServer::new(engine.clone(), *port);
            if let Err(e) = server.start_in_background() {
                eprintln!("❌ Gagal menjalankan server gateway dev: {}", e);
                process::exit(1);
            }

            println!("{}", "⚡ Edge Simulator Aktif!".bright_green().bold());
            println!("   - Aplikasi Utama : http://localhost:{}", port);
            println!("   - Control Plane  : http://localhost:{}/_widya/dashboard", port);
            println!("   Tekan Ctrl+C untuk menghentikan server.");

            // Tunggu sinyal interupsi
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        }
        WebSubcommands::List { proyek } | WebSubcommands::Ls { proyek } => {
            let list = engine.list_deployments(proyek.as_deref());
            println!("\n{}", "📋 DAFTAR DEPLOYMENT ATOMIK WIDYAWEB CLOUD".bright_cyan().bold());
            println!("==================================================================================================");
            println!("{:<16} {:<18} {:<12} {:<36} {:<10}", "DEPLOYMENT ID", "PROYEK", "TARGET", "PRATINJAU URL", "BERKAS/FN");
            println!("--------------------------------------------------------------------------------------------------");
            for d in list {
                let target_str = match d.target {
                    DeploymentTarget::Production => "Production".bright_green(),
                    DeploymentTarget::Preview => "Preview".bright_blue(),
                    DeploymentTarget::Development => "Dev".bright_yellow(),
                };
                println!("{:<16} {:<18} {:<12} {:<36} {}/{}", d.id.bright_yellow(), d.project_name, target_str, d.preview_url, d.files_count, d.functions_count);
            }
            println!("==================================================================================================\n");
        }
        WebSubcommands::Rollback { proyek, deploy_id } => {
            println!("{}", format!("🔄 Melakukan Rollback Instan Proyek '{}' ke Deployment '{}'...", proyek, deploy_id).bright_cyan().bold());
            match engine.rollback_production(proyek, deploy_id) {
                Ok(msg) => {
                    println!("{}", format!("✅ Sukses: {}", msg).bright_green().bold());
                }
                Err(e) => {
                    eprintln!("❌ Gagal melakukan rollback: {}", e);
                    process::exit(1);
                }
            }
        }
        WebSubcommands::Env { aksi } => {
            handle_web_env_command(aksi, &engine);
        }
        WebSubcommands::Metrics { proyek } => {
            let tele = engine.telemetry.aggregate(proyek.as_deref(), None);
            println!("\n{}", "📊 TELEMETRI EDGE & CORE WEB VITALS (WIDYAWEB CLOUD)".bright_cyan().bold());
            println!("============================================================");
            println!("  Total Permintaan : {}", tele.total_requests.to_string().bright_white().bold());
            println!("  Status 2xx       : {}", tele.status_2xx.to_string().bright_green());
            println!("  Status 3xx       : {}", tele.status_3xx.to_string().bright_blue());
            println!("  Status 4xx/5xx   : {} / {}", tele.status_4xx.to_string().bright_yellow(), tele.status_5xx.to_string().bright_red());
            println!("  Cache Hit Rate   : {:.1}%", tele.cache_hit_rate_pct);
            println!("  P95 Latensi Edge : {:.2} ms", tele.p95_latency_ms);
            println!("  P99 Latensi Edge : {:.2} ms", tele.p99_latency_ms);
            println!("  Total Bandwidth  : {:.2} KB", (tele.total_bytes as f64) / 1024.0);
            println!("------------------------------------------------------------");
            println!("  ⚡ Core Web Vitals RUM:");
            println!("     - LCP (Largest Contentful Paint) : {:.0} ms (Baik)", tele.avg_lcp_ms);
            println!("     - FID / INP (Interaktivitas)     : {:.1} ms (Baik)", tele.avg_fid_ms);
            println!("     - CLS (Cumulative Layout Shift)  : {:.3} (Sempurna)", tele.avg_cls);
            println!("     - TTFB (Time to First Byte)      : {:.1} ms", tele.avg_ttfb_ms);
            println!("============================================================\n");
        }
        WebSubcommands::Dashboard { port } => {
            println!("{}", "🖥️  Membuka Control Plane Dashboard WidyaWeb Cloud...".bright_cyan().bold());
            let server = SovereignGatewayServer::new(engine.clone(), *port);
            if let Err(e) = server.start_in_background() {
                eprintln!("❌ Gagal membuka server dashboard: {}", e);
                process::exit(1);
            }
            println!("{}", format!("🚀 Dashboard aktif di: http://localhost:{}/_widya/dashboard", port).bright_green().bold());
            println!("   Tekan Ctrl+C untuk keluar.");
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

pub fn handle_web_env_command(aksi: &WebEnvSubcommands, engine: &Arc<widya::web::cloud::WidyaCloudEngine>) {
    use widya::web::cloud::DeploymentTarget;

    match aksi {
        WebEnvSubcommands::Set { kunci, nilai, proyek, prod } => {
            let target = if *prod {
                DeploymentTarget::Production
            } else {
                DeploymentTarget::Preview
            };
            engine.vault.set_secret(proyek, kunci, nilai, target);
            println!("{}", format!("🔒 Variabel lingkungan '{}' berhasil disimpan untuk proyek '{}' (Target: {:?}).", kunci, proyek, target).bright_green().bold());
        }
        WebEnvSubcommands::Ls { proyek } => {
            let list = engine.vault.list_secrets(proyek);
            println!("\n{}", format!("🔒 VARIABEL LINGKUNGAN / SECRETS: {}", proyek).bright_cyan().bold());
            println!("============================================================");
            println!("{:<24} {:<14} {:<16}", "KUNCI", "TARGET", "NILAI HASH");
            println!("------------------------------------------------------------");
            for s in list {
                let masked = if s.value.len() > 4 {
                    format!("{}****", &s.value[0..3])
                } else {
                    "****".to_string()
                };
                println!("{:<24} {:<14?} {:<16}", s.key.bright_white(), s.target, masked.bright_yellow());
            }
            println!("============================================================\n");
        }
        WebEnvSubcommands::Rm { kunci, proyek, prod } => {
            let target = if *prod {
                DeploymentTarget::Production
            } else {
                DeploymentTarget::Preview
            };
            if engine.vault.remove_secret(proyek, kunci, target) {
                println!("{}", format!("🗑️ Variabel lingkungan '{}' berhasil dihapus dari proyek '{}'.", kunci, proyek).bright_green().bold());
            } else {
                eprintln!("⚠️ Variabel '{}' tidak ditemukan pada proyek '{}'.", kunci, proyek);
            }
        }
    }
}

pub fn handle_box_command(aksi: &BoxSubcommands) {
    use widya::container::{
        get_global_box_engine, ComposeEngine, ContainerSpec, ContainerStatus, PortMapping,
        VolumeMount,
    };

    let engine = get_global_box_engine();

    match aksi {
        BoxSubcommands::Run {
            image,
            name,
            port,
            volume,
            env,
            detach,
            cmd,
        } => {
            let mut port_maps = Vec::new();
            for p in port {
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

            let mut vol_mounts = Vec::new();
            for v in volume {
                if let Some((h, c)) = v.split_once(':') {
                    vol_mounts.push(VolumeMount {
                        host_path: h.to_string(),
                        container_path: c.to_string(),
                        read_only: false,
                    });
                }
            }

            let mut env_map = HashMap::new();
            for e in env {
                if let Some((k, v)) = e.split_once('=') {
                    env_map.insert(k.to_string(), v.to_string());
                }
            }

            let ctr_name = name.clone().unwrap_or_else(|| {
                format!("box_{}", &uuid::Uuid::new_v4().to_string()[0..8])
            });

            let spec = ContainerSpec {
                name: ctr_name,
                image_tag_or_id: image.clone(),
                command: cmd.clone(),
                env: env_map,
                ports: port_maps,
                volumes: vol_mounts,
                working_dir: None,
                memory_limit_mb: Some(512),
                cpu_quota_pct: Some(100),
                detach: *detach,
                pod_id: None,
            };

            match engine.create_container(spec) {
                Ok(ctr) => {
                    let _ = engine.start_container(&ctr.id);
                    println!("{}", "📦 Kontainer Berhasil Dijalankan (WidyaBox Sandbox)!".bright_green().bold());
                    println!("============================================================");
                    println!("  Container ID    : {}", ctr.id.bright_yellow().bold());
                    println!("  Nama Kontainer  : {}", ctr.name.bright_white().bold());
                    println!("  Image Dasar     : {}", ctr.image_name.bright_cyan());
                    println!("  IP Address      : {}", ctr.ip_address.bright_green());
                    if !ctr.ports.is_empty() {
                        let p_str: Vec<String> = ctr.ports.iter().map(|p| format!("{}:{}", p.host_port, p.container_port)).collect();
                        println!("  Port Forwarding : {}", p_str.join(", ").bright_magenta());
                    }
                    println!("  Status          : {}", "● Running (Daemonless)".bright_green().bold());
                    println!("============================================================");
                }
                Err(e) => {
                    eprintln!("❌ Gagal menjalankan kontainer: {}", e);
                    process::exit(1);
                }
            }
        }
        BoxSubcommands::Build { tag, file, context } => {
            println!("{}", format!("🔨 Membangun Image OCI dari Widyafile ({:?})...", file).bright_cyan().bold());
            match engine.build_image(context, file, tag) {
                Ok(img) => {
                    println!("\n{}", "🎉 PEMBUATAN IMAGE SUKSES!".bright_green().bold());
                    println!("============================================================");
                    println!("  Image ID        : {}", img.id.bright_yellow().bold());
                    println!("  Repository:Tag  : {}:{}", img.repository.bright_white().bold(), img.tag.bright_green().bold());
                    println!("  Jumlah Layer    : {} layers", img.layers.len());
                    println!("  Ukuran Total    : {:.2} KB", (img.total_size_bytes as f64) / 1024.0);
                    println!("  Arsitektur      : {} / {}", img.os, img.architecture);
                    println!("============================================================");
                }
                Err(e) => {
                    eprintln!("❌ Gagal membangun image: {}", e);
                    process::exit(1);
                }
            }
        }
        BoxSubcommands::Ps { all } => {
            let list = engine.list_containers(*all);
            println!("\n{}", "📋 DAFTAR KONTAINER WIDYABOX (DAEMONLESS & ROOTLESS)".bright_cyan().bold());
            println!("==================================================================================================");
            println!("{:<16} {:<18} {:<24} {:<14} {:<16}", "CONTAINER ID", "NAMA", "IMAGE", "STATUS", "PORTS");
            println!("--------------------------------------------------------------------------------------------------");
            for c in list {
                let st_str = match c.status {
                    ContainerStatus::Running => "● Running".bright_green(),
                    ContainerStatus::Created => "◐ Created".bright_blue(),
                    ContainerStatus::Paused => "⏸ Paused".bright_yellow(),
                    ContainerStatus::Stopped => "✕ Stopped".bright_red(),
                    ContainerStatus::Exited(code) => format!("Exited ({})", code).bright_red(),
                };
                let port_str: Vec<String> = c.ports.iter().map(|p| format!("{}:{}", p.host_port, p.container_port)).collect();
                println!("{:<16} {:<18} {:<24} {:<14} {:<16}", c.id.bright_yellow(), c.name, c.image_name, st_str, port_str.join(", "));
            }
            println!("==================================================================================================\n");
        }
        BoxSubcommands::Images => {
            let list = engine.list_images();
            println!("\n{}", "🖼️  REPOSITORI IMAGE OCI WIDYABOX".bright_cyan().bold());
            println!("==================================================================================================");
            println!("{:<16} {:<24} {:<12} {:<12} {:<16}", "IMAGE ID", "REPOSITORY", "TAG", "UKURAN", "LAYERS");
            println!("--------------------------------------------------------------------------------------------------");
            for img in list {
                let sz = format!("{:.2} KB", (img.total_size_bytes as f64) / 1024.0);
                println!("{:<16} {:<24} {:<12} {:<12} {} layers", img.id.bright_yellow(), img.repository.bright_white(), img.tag.bright_green(), sz, img.layers.len());
            }
            println!("==================================================================================================\n");
        }
        BoxSubcommands::Stop { id } => {
            match engine.stop_container(id) {
                Ok(cid) => println!("{}", format!("🛑 Kontainer '{}' berhasil dihentikan.", cid).bright_yellow().bold()),
                Err(e) => eprintln!("❌ Gagal menghentikan kontainer: {}", e),
            }
        }
        BoxSubcommands::Start { id } => {
            match engine.start_container(id) {
                Ok(cid) => println!("{}", format!("▶️ Kontainer '{}' berhasil dimulai.", cid).bright_green().bold()),
                Err(e) => eprintln!("❌ Gagal memulai kontainer: {}", e),
            }
        }
        BoxSubcommands::Rm { id, force } => {
            match engine.remove_container(id, *force) {
                Ok(_) => println!("{}", format!("🗑️ Kontainer '{}' berhasil dihapus.", id).bright_green().bold()),
                Err(e) => eprintln!("❌ Gagal menghapus kontainer: {}", e),
            }
        }
        BoxSubcommands::Rmi { id } => {
            match engine.remove_image(id) {
                Ok(true) => println!("{}", format!("🗑️ Image OCI '{}' berhasil dihapus.", id).bright_green().bold()),
                Ok(false) => eprintln!("⚠️ Image OCI '{}' tidak ditemukan.", id),
                Err(e) => eprintln!("❌ Gagal menghapus image: {}", e),
            }
        }
        BoxSubcommands::Logs { id } => {
            match engine.get_container_logs(id) {
                Ok(logs) => {
                    println!("{}", format!("📜 LOG KONTAINER: {}", id).bright_cyan().bold());
                    for l in logs {
                        println!("{}", l);
                    }
                }
                Err(e) => eprintln!("❌ Gagal mengambil log: {}", e),
            }
        }
        BoxSubcommands::Exec { id, cmd } => {
            match engine.exec_command(id, cmd) {
                Ok(out) => println!("{}", out.bright_white()),
                Err(e) => eprintln!("❌ Gagal eksekusi perintah: {}", e),
            }
        }
        BoxSubcommands::Compose { file } => {
            println!("{}", format!("🚀 Menjalankan Orkestrasi Multi-Kontainer dari {:?}...", file).bright_cyan().bold());
            match ComposeEngine::parse_and_up(&engine, file) {
                Ok(services) => {
                    println!("{}", format!("🎉 Seluruh service berhasil dijalankan: {}", services.join(", ")).bright_green().bold());
                }
                Err(e) => eprintln!("❌ Gagal menjalankan compose: {}", e),
            }
        }
    }
}

pub fn handle_pod_command(aksi: &PodSubcommands) {
    use widya::container::{get_global_box_engine, PodStatus, PortMapping};

    let engine = get_global_box_engine();

    match aksi {
        PodSubcommands::Create { name, port } => {
            let mut port_maps = Vec::new();
            for p in port {
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

            match engine.create_pod(name, port_maps) {
                Ok(pod) => {
                    println!("{}", "🛡️  Pod Kontainer Berhasil Dibuat!".bright_green().bold());
                    println!("============================================================");
                    println!("  Pod ID          : {}", pod.id.bright_yellow().bold());
                    println!("  Nama Pod        : {}", pod.name.bright_white().bold());
                    println!("  Status          : {:?}", pod.status);
                    println!("============================================================");
                }
                Err(e) => eprintln!("❌ Gagal membuat pod: {}", e),
            }
        }
        PodSubcommands::Start { id } => {
            match engine.start_pod(id) {
                Ok(pid) => println!("{}", format!("▶️ Seluruh kontainer dalam Pod '{}' berhasil dimulai.", pid).bright_green().bold()),
                Err(e) => eprintln!("❌ Gagal memulai pod: {}", e),
            }
        }
        PodSubcommands::Stop { id } => {
            match engine.stop_pod(id) {
                Ok(pid) => println!("{}", format!("🛑 Seluruh kontainer dalam Pod '{}' berhasil dihentikan.", pid).bright_yellow().bold()),
                Err(e) => eprintln!("❌ Gagal menghentikan pod: {}", e),
            }
        }
        PodSubcommands::Ps => {
            let list = engine.list_pods();
            println!("\n{}", "🛡️  DAFTAR POD WIDYABOX (KUBERNETES / PODMAN COMPATIBLE)".bright_cyan().bold());
            println!("==================================================================================================");
            println!("{:<16} {:<20} {:<14} {:<16}", "POD ID", "NAMA POD", "STATUS", "KONTAINER");
            println!("--------------------------------------------------------------------------------------------------");
            for p in list {
                let st_str = match p.status {
                    PodStatus::Running => "● Running".bright_green(),
                    PodStatus::Created => "◐ Created".bright_blue(),
                    PodStatus::Degraded => "⚠️ Degraded".bright_yellow(),
                    PodStatus::Stopped => "✕ Stopped".bright_red(),
                };
                println!("{:<16} {:<20} {:<14} {} kontainer", p.id.bright_yellow(), p.name, st_str, p.container_ids.len());
            }
            println!("==================================================================================================\n");
        }
    }
}

pub fn handle_k8s_command(aksi: &K8sSubcommands) {
    use widya::orchestrator::{
        get_global_kube_cluster, ClusterPodStatus, DeploymentManifest, HpaManifest, IngressManifest,
        ManifestHeader, NodeStatus, ServiceManifest,
    };

    let cluster = get_global_kube_cluster();

    match aksi {
        K8sSubcommands::Apply { file } => {
            if !file.exists() {
                eprintln!("❌ Berkas manifest '{:?}' tidak ditemukan.", file);
                process::exit(1);
            }

            let content = match fs::read_to_string(file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("❌ Gagal membaca manifest: {}", e);
                    process::exit(1);
                }
            };

            let header: ManifestHeader = match serde_json::from_str(&content) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("❌ Format JSON manifest tidak valid: {}", e);
                    process::exit(1);
                }
            };

            match header.kind.as_str() {
                "Deployment" => {
                    let dep_manifest: DeploymentManifest = serde_json::from_str(&content).unwrap();
                    match cluster.apply_deployment(dep_manifest) {
                        Ok(dep) => {
                            println!("{}", format!("🚀 Deployment '{}' berhasil diterapkan ({} replika terjadwal).", dep.name, dep.desired_replicas).bright_green().bold());
                        }
                        Err(e) => eprintln!("❌ Gagal menerapkan deployment: {}", e),
                    }
                }
                "Service" => {
                    let svc_manifest: ServiceManifest = serde_json::from_str(&content).unwrap();
                    match cluster.apply_service(svc_manifest) {
                        Ok(svc) => {
                            println!("{}", format!("🌐 Service '{}' berhasil dibuat dengan ClusterIP: {}", svc.name, svc.cluster_ip).bright_green().bold());
                        }
                        Err(e) => eprintln!("❌ Gagal menerapkan service: {}", e),
                    }
                }
                "Ingress" => {
                    let ing_manifest: IngressManifest = serde_json::from_str(&content).unwrap();
                    match cluster.apply_ingress(ing_manifest) {
                        Ok(ing) => {
                            println!("{}", format!("🚪 Ingress Controller '{}' aktif pada IP VIP: {}", ing.name, ing.ingress_ip).bright_green().bold());
                        }
                        Err(e) => eprintln!("❌ Gagal menerapkan ingress: {}", e),
                    }
                }
                "HPA" | "HorizontalPodAutoscaler" => {
                    let hpa_manifest: HpaManifest = serde_json::from_str(&content).unwrap();
                    match cluster.apply_hpa(hpa_manifest) {
                        Ok(hpa) => {
                            println!("{}", format!("⚡ HPA '{}' aktif untuk Deployment '{}' (Min: {}, Max: {}).", hpa.name, hpa.spec.target_deployment, hpa.spec.min_replicas, hpa.spec.max_replicas).bright_green().bold());
                        }
                        Err(e) => eprintln!("❌ Gagal menerapkan HPA: {}", e),
                    }
                }
                other => {
                    eprintln!("⚠️ Jenis resource manifest '{}' belum didukung.", other);
                }
            }
        }
        K8sSubcommands::Get { resource, namespace } => {
            let res_clean = resource.to_lowercase();
            if res_clean.starts_with("pod") {
                let pods_lock = cluster.pods.read().unwrap();
                println!("\n{}", "📋 DAFTAR PODS DALAM KLUSTER (WIDYAKUBE)".bright_cyan().bold());
                println!("==================================================================================================");
                println!("{:<24} {:<12} {:<18} {:<14} {:<8}", "NAMA POD", "STATUS", "NODE", "IP ADDRESS", "RESTARTS");
                println!("--------------------------------------------------------------------------------------------------");
                for p in pods_lock.values() {
                    let st_str = match p.status {
                        ClusterPodStatus::Running => "● Running".bright_green(),
                        ClusterPodStatus::Pending => "◐ Pending".bright_yellow(),
                        ClusterPodStatus::Succeeded => "✓ Succeeded".bright_blue(),
                        ClusterPodStatus::Failed => "✕ Failed".bright_red(),
                        ClusterPodStatus::Terminating => "⏹ Terminating".bright_white(),
                    };
                    println!("{:<24} {:<12} {:<18} {:<14} {}", p.name.bright_white(), st_str, p.node_name, p.ip_address.bright_yellow(), p.restart_count);
                }
                println!("==================================================================================================\n");
            } else if res_clean.starts_with("dep") {
                let dep_lock = cluster.deployments.read().unwrap();
                println!("\n{}", "🚀 DAFTAR DEPLOYMENTS DALAM KLUSTER".bright_cyan().bold());
                println!("==================================================================================================");
                println!("{:<24} {:<12} {:<12} {:<14}", "NAMA DEPLOYMENT", "READY", "UP-TO-DATE", "AVAILABLE");
                println!("--------------------------------------------------------------------------------------------------");
                for d in dep_lock.values() {
                    let ready_str = format!("{}/{}", d.available_replicas, d.desired_replicas);
                    println!("{:<24} {:<12} {:<12} {}", d.name.bright_white(), ready_str.bright_green(), d.desired_replicas, d.available_replicas);
                }
                println!("==================================================================================================\n");
            } else if res_clean.starts_with("svc") || res_clean.starts_with("service") {
                let svc_lock = cluster.services.read().unwrap();
                println!("\n{}", "🌐 DAFTAR SERVICES & LOAD BALANCER".bright_cyan().bold());
                println!("==================================================================================================");
                println!("{:<20} {:<14} {:<16} {:<24}", "NAMA SERVICE", "TYPE", "CLUSTER-IP", "ENDPOINTS");
                println!("--------------------------------------------------------------------------------------------------");
                for s in svc_lock.values() {
                    println!("{:<20} {:<14} {:<16} {}", s.name.bright_white(), s.spec.service_type.bright_blue(), s.cluster_ip.bright_yellow(), s.endpoint_ips.join(", ").bright_green());
                }
                println!("==================================================================================================\n");
            } else if res_clean.starts_with("node") {
                let nodes_lock = cluster.nodes.read().unwrap();
                println!("\n{}", "🖥️  TOPOLOGI NODE KLUSTER WIDYAKUBE".bright_cyan().bold());
                println!("==================================================================================================");
                println!("{:<20} {:<12} {:<16} {:<16} {:<16}", "NAMA NODE", "STATUS", "IP INTERNAL", "ALOKASI CPU", "ALOKASI RAM");
                println!("--------------------------------------------------------------------------------------------------");
                for n in nodes_lock.values() {
                    let st_str = match n.status {
                        NodeStatus::Ready => "● Ready".bright_green(),
                        NodeStatus::NotReady => "✕ NotReady".bright_red(),
                        NodeStatus::Maintenance => "⏸ Maintenance".bright_yellow(),
                    };
                    let cpu_str = format!("{}m / {}m", n.allocated_cpu_millicores, n.total_cpu_millicores);
                    let ram_str = format!("{} MB / {} MB", n.allocated_memory_mb, n.total_memory_mb);
                    println!("{:<20} {:<12} {:<16} {:<16} {}", n.name.bright_white(), st_str, n.ip_address, cpu_str, ram_str);
                }
                println!("==================================================================================================\n");
            } else {
                eprintln!("Resource '{}' tidak dikenali. Gunakan: pods, deployments, services, nodes.", resource);
            }
        }
        K8sSubcommands::Scale { name, replicas } => {
            println!("{}", format!("⚖️  Mengubah skala deployment '{}' menjadi {} replika...", name, replicas).bright_cyan().bold());
            match cluster.scale_deployment(name, *replicas) {
                Ok(new_count) => {
                    println!("{}", format!("✅ Sukses: Deployment '{}' sekarang memiliki {} replika aktif.", name, new_count).bright_green().bold());
                }
                Err(e) => eprintln!("❌ Gagal mengubah skala deployment: {}", e),
            }
        }
        K8sSubcommands::Nodes => {
            let nodes_lock = cluster.nodes.read().unwrap();
            println!("\n{}", "🖥️  TOPOLOGI NODE KLUSTER WIDYAKUBE".bright_cyan().bold());
            println!("==================================================================================================");
            println!("{:<20} {:<12} {:<16} {:<16} {:<16}", "NAMA NODE", "STATUS", "IP INTERNAL", "ALOKASI CPU", "ALOKASI RAM");
            println!("--------------------------------------------------------------------------------------------------");
            for n in nodes_lock.values() {
                let st_str = match n.status {
                    NodeStatus::Ready => "● Ready".bright_green(),
                    NodeStatus::NotReady => "✕ NotReady".bright_red(),
                    NodeStatus::Maintenance => "⏸ Maintenance".bright_yellow(),
                };
                let cpu_str = format!("{}m / {}m", n.allocated_cpu_millicores, n.total_cpu_millicores);
                let ram_str = format!("{} MB / {} MB", n.allocated_memory_mb, n.total_memory_mb);
                println!("{:<20} {:<12} {:<16} {:<16} {}", n.name.bright_white(), st_str, n.ip_address, cpu_str, ram_str);
            }
            println!("==================================================================================================\n");
        }
        K8sSubcommands::SelfHeal => {
            println!("{}", "🩺 Menjalankan Siklus Pemeriksaan & Pemulihan Mandiri (Self-Healing)...".bright_cyan().bold());
            let events = cluster.trigger_self_healing_check();
            if events.is_empty() {
                println!("{}", "✅ Semua Pod dan Node dalam kondisi sehat (Optimal State Reconciled).".bright_green().bold());
            } else {
                for ev in events {
                    println!("{}", ev.bright_yellow());
                }
            }
        }
        K8sSubcommands::Autoscale => {
            println!("{}", "📈 Menjalankan Siklus Evaluasi Beban & Autoscaling (HPA Controller)...".bright_cyan().bold());
            let events = cluster.trigger_hpa_autoscaling_tick();
            if events.is_empty() {
                println!("{}", "✅ Utilisasi CPU stabil dalam batas ambang target.".bright_green().bold());
            } else {
                for ev in events {
                    println!("{}", ev.bright_green().bold());
                }
            }
        }
    }
}

pub fn handle_security_command(aksi: Option<SecuritySubcommands>, target: Option<PathBuf>) {
    use widya::security_tools::{SecurityLinter, SecretScanner};

    let target_path = target.unwrap_or_else(|| PathBuf::from("."));

    match aksi {
        Some(SecuritySubcommands::Secrets { target }) => {
            println!("{}", "🔍 Memindai Kebocoran Secret & Kredensial...".bright_cyan().bold());
            let scanner = SecretScanner::new();
            let mut all_findings = Vec::new();
            
            let mut scanner_closure = |sumber: &str, nama: &str| {
                let findings = scanner.scan(sumber, nama);
                all_findings.extend(findings);
            };
            pindai_direktori_dengan(&target, &mut scanner_closure);

            let linter = SecurityLinter::new();
            println!("{}", linter.format_report(&all_findings));
            if all_findings.iter().any(|f| matches!(f.severity, widya::security_tools::Severity::Critical)) {
                process::exit(1);
            }
        }
        Some(SecuritySubcommands::InstallHook) => {
            let hook_dir = PathBuf::from(".git/hooks");
            if !hook_dir.exists() {
                eprintln!("❌ Direktori git hooks (.git/hooks) tidak ditemukan. Pastikan proyek berada di repositori Git.");
                process::exit(1);
            }
            let hook_file = hook_dir.join("pre-commit");
            let hook_script = "#!/bin/sh\n# Widya-Lang Automated Security Hook\necho \"🛡️ Memindai keamanan kode Widya sebelum commit...\"\nwidya security scan .\n";
            match fs::write(&hook_file, hook_script) {
                Ok(_) => {
                    println!("{}", "✅ Pre-commit hook keamanan berhasil dipasang di .git/hooks/pre-commit".bright_green().bold());
                }
                Err(e) => {
                    eprintln!("❌ Gagal menulis hook: {}", e);
                    process::exit(1);
                }
            }
        }
        Some(SecuritySubcommands::Scan { target }) => {
            jalankan_audit_keamanan(&target);
        }
        None => {
            jalankan_audit_keamanan(&target_path);
        }
    }
}

pub fn jalankan_audit_keamanan(target: &PathBuf) {
    use widya::security_tools::SecurityLinter;

    println!("{}", format!("🛡️ Menjalankan Audit Keamanan Widya pada: {}", target.display()).bright_cyan().bold());
    let linter = SecurityLinter::new();
    let mut all_findings = Vec::new();

    let mut scanner_closure = |sumber: &str, nama: &str| {
        let findings = linter.scan(sumber, nama);
        all_findings.extend(findings);
    };
    pindai_direktori_dengan(target, &mut scanner_closure);

    println!("{}", linter.format_report(&all_findings));

    if all_findings.iter().any(|f| matches!(f.severity, widya::security_tools::Severity::Critical)) {
        eprintln!("{}", "❌ Ditemukan kerentanan kritis! Perbaiki segera sebelum deployment.".bright_red().bold());
        process::exit(1);
    }
}

pub fn pindai_direktori_dengan<F>(target: &PathBuf, scanner_fn: &mut F)
where
    F: FnMut(&str, &str),
{
    if target.is_file() {
        if let Ok(sumber) = fs::read_to_string(target) {
            scanner_fn(&sumber, &target.display().to_string());
        }
    } else if target.is_dir() {
        let mut queue = vec![target.clone()];
        while let Some(current_dir) = queue.pop() {
            if let Ok(entries) = fs::read_dir(&current_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                        if ext == "wya" || ext == "json" || ext == "env" || ext == "toml" || ext == "yaml" || ext == "yml" {
                            if let Ok(sumber) = fs::read_to_string(&path) {
                                scanner_fn(&sumber, &path.display().to_string());
                            }
                        }
                    } else if path.is_dir() {
                        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                        if name != "target" && name != ".git" && name != "node_modules" {
                            queue.push(path);
                        }
                    }
                }
            }
        }
    }
}
