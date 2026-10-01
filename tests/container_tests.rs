// ==============================================================================
// WidyaBox Container & Pod Engine Test Suite (Podman / Docker Equivalent)
// ==============================================================================

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use widya::container::{
    ComposeEngine, ContainerSpec, ContainerStatus, PodStatus, PortMapping,
    VolumeMount, WidyaBoxEngine, WidyafileInstruction, WidyafileParser,
};

#[test]
fn test_widyafile_parsing() {
    let widyafile_src = r#"
        # Berkas Konfigurasi Widyafile
        DARI widya-base:latest
        LINGKUNGAN PORT=8080
        LINGKUNGAN NODE_ENV=production
        KERJA /app
        SALIN src /app/src
        SALIN widya.toml /app/widya.toml
        JALANKAN widya pasang
        PORT 8080
        TITIK_MASUK ["/bin/widya"]
        PERINTAH ["jalankan", "/app/src/main.wya"]
        LABEL vendor=WidyaNusantara
    "#;

    let instructions = WidyafileParser::parse(widyafile_src).expect("Parsing Widyafile harus sukses");
    assert_eq!(instructions.len(), 11);

    match &instructions[0] {
        WidyafileInstruction::From { image, tag } => {
            assert_eq!(image, "widya-base");
            assert_eq!(tag, "latest");
        }
        _ => panic!("Instruksi pertama harus DARI/FROM"),
    }

    match &instructions[1] {
        WidyafileInstruction::Env { key, value } => {
            assert_eq!(key, "PORT");
            assert_eq!(value, "8080");
        }
        _ => panic!("Instruksi kedua harus LINGKUNGAN/ENV"),
    }

    match &instructions[3] {
        WidyafileInstruction::Workdir { path } => {
            assert_eq!(path, "/app");
        }
        _ => panic!("Instruksi keempat harus KERJA/WORKDIR"),
    }
}

#[test]
fn test_image_building_and_layer_hashing() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_box_test_build_{}", pid));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    // Buat berkas mock
    fs::write(temp_dir.join("main.wya"), "fungsi utama() { cetak(\"Halo Kontainer!\"); }").unwrap();
    let widyafile_path = temp_dir.join("Widyafile");
    fs::write(
        &widyafile_path,
        r#"
        FROM widya-base:latest
        ENV APLIKASI=WidyaMicroservice
        WORKDIR /workspace
        COPY main.wya /workspace/main.wya
        EXPOSE 9090
        ENTRYPOINT ["/bin/widya", "/workspace/main.wya"]
    "#,
    )
    .unwrap();

    let storage_dir = temp_dir.join("storage");
    let engine = WidyaBoxEngine::new(&storage_dir).expect("Inisialisasi WidyaBox gagal");

    let built_image = engine
        .build_image(&temp_dir, &widyafile_path, "widya-app:v1.0")
        .expect("Build image harus sukses");

    assert_eq!(built_image.repository, "widya-app");
    assert_eq!(built_image.tag, "v1.0");
    assert!(built_image.id.starts_with("img_"));
    assert_eq!(built_image.config.exposed_ports, vec![9090]);
    assert_eq!(built_image.config.working_dir, "/workspace");
    assert_eq!(
        built_image.config.env.get("APLIKASI").unwrap(),
        "WidyaMicroservice"
    );

    // Pastikan image terdaftar di repositori lokal
    let images = engine.list_images();
    assert!(images.iter().any(|img| img.repository == "widya-app"));

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_container_create_start_stop_rm() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_box_test_lifecycle_{}", pid));
    let _ = fs::remove_dir_all(&temp_dir);

    let storage_dir = temp_dir.join("storage");
    let engine = WidyaBoxEngine::new(&storage_dir).unwrap();

    // 1. Create Container
    let spec = ContainerSpec {
        name: "layanan-web-utama".to_string(),
        image_tag_or_id: "widya-base:latest".to_string(),
        command: vec!["/bin/widya".to_string(), "serve".to_string()],
        env: {
            let mut m = HashMap::new();
            m.insert("PORT".to_string(), "8080".to_string());
            m
        },
        ports: vec![PortMapping {
            host_port: 8080,
            container_port: 80,
            protocol: "tcp".to_string(),
        }],
        volumes: vec![VolumeMount {
            host_path: "/data/db".to_string(),
            container_path: "/var/lib/data".to_string(),
            read_only: false,
        }],
        working_dir: Some("/app".to_string()),
        memory_limit_mb: Some(512),
        cpu_quota_pct: Some(100),
        detach: true,
        pod_id: None,
    };

    let ctr = engine.create_container(spec).expect("Create container gagal");
    assert_eq!(ctr.name, "layanan-web-utama");
    assert_eq!(ctr.status, ContainerStatus::Created);
    assert!(ctr.id.starts_with("ctr_"));
    assert!(PathBuf::from(&ctr.rootfs_dir).exists());

    // 2. Start Container
    let start_res = engine.start_container(&ctr.id);
    assert!(start_res.is_ok());

    let running_list = engine.list_containers(false);
    assert_eq!(running_list.len(), 1);
    assert_eq!(running_list[0].status, ContainerStatus::Running);

    // 3. Exec Command
    let exec_out = engine
        .exec_command(&ctr.id, &["echo".to_string(), "Tes".to_string()])
        .expect("Exec gagal");
    assert!(exec_out.contains("Tes"));

    // 4. Logs
    let logs = engine.get_container_logs(&ctr.id).expect("Get logs gagal");
    assert!(logs.len() >= 2);

    // 5. Stop Container
    let stop_res = engine.stop_container(&ctr.id);
    assert!(stop_res.is_ok());

    let running_after_stop = engine.list_containers(false);
    assert_eq!(running_after_stop.len(), 0);

    // 6. Remove Container
    let rm_res = engine.remove_container(&ctr.id, false);
    assert!(rm_res.is_ok());
    assert_eq!(engine.list_containers(true).len(), 0);

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_podman_compatible_pods() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_box_test_pod_{}", pid));
    let _ = fs::remove_dir_all(&temp_dir);

    let storage_dir = temp_dir.join("storage");
    let engine = WidyaBoxEngine::new(&storage_dir).unwrap();

    // 1. Buat Pod
    let pod = engine
        .create_pod(
            "pod-microservices-fintech",
            vec![PortMapping {
                host_port: 8080,
                container_port: 8080,
                protocol: "tcp".to_string(),
            }],
        )
        .expect("Create pod gagal");

    assert_eq!(pod.name, "pod-microservices-fintech");
    assert_eq!(pod.status, PodStatus::Created);

    // 2. Masukkan 2 Kontainer ke dalam Pod (Sharing Network Namespace)
    let spec1 = ContainerSpec {
        name: "ctr-gateway".to_string(),
        image_tag_or_id: "widya-base:latest".to_string(),
        command: vec!["/bin/gateway".to_string()],
        env: HashMap::new(),
        ports: Vec::new(),
        volumes: Vec::new(),
        working_dir: None,
        memory_limit_mb: Some(256),
        cpu_quota_pct: Some(50),
        detach: true,
        pod_id: Some(pod.id.clone()),
    };

    let spec2 = ContainerSpec {
        name: "ctr-auth".to_string(),
        image_tag_or_id: "widya-base:latest".to_string(),
        command: vec!["/bin/auth".to_string()],
        env: HashMap::new(),
        ports: Vec::new(),
        volumes: Vec::new(),
        working_dir: None,
        memory_limit_mb: Some(256),
        cpu_quota_pct: Some(50),
        detach: true,
        pod_id: Some(pod.id.clone()),
    };

    engine.create_container(spec1).unwrap();
    engine.create_container(spec2).unwrap();

    // 3. Start Pod (Seluruh kontainer di dalamnya otomatis menyala)
    engine.start_pod(&pod.id).expect("Start pod gagal");

    let running_ctrs = engine.list_containers(false);
    assert_eq!(running_ctrs.len(), 2);

    // 4. Stop Pod
    engine.stop_pod(&pod.id).expect("Stop pod gagal");
    assert_eq!(engine.list_containers(false).len(), 0);

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_compose_orchestrator() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_box_test_compose_{}", pid));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let compose_file = temp_dir.join("widyacompose.json");
    let compose_content = r#"{
        "version": "3.8",
        "services": {
            "api-service": {
                "image": "widya-base:latest",
                "ports": ["3000:3000"],
                "environment": {
                    "DB_HOST": "localhost",
                    "DEBUG": "true"
                },
                "command": "widya run server.wya"
            },
            "cache-service": {
                "image": "widya-base:latest",
                "ports": ["6379:6379"],
                "command": "widyabase cache start"
            }
        }
    }"#;
    fs::write(&compose_file, compose_content).unwrap();

    let storage_dir = temp_dir.join("storage");
    let engine = std::sync::Arc::new(WidyaBoxEngine::new(&storage_dir).unwrap());

    let launched = ComposeEngine::parse_and_up(&engine, &compose_file)
        .expect("Compose parse & up harus sukses");

    assert_eq!(launched.len(), 2);
    assert_eq!(engine.list_containers(false).len(), 2);

    let _ = fs::remove_dir_all(temp_dir);
}
