// ==============================================================================
// WidyaKube Cluster Orchestrator Test Suite (Kubernetes / K3s Equivalent)
// ==============================================================================

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use widya::container::WidyaBoxEngine;
use widya::orchestrator::{
    ClusterPodStatus, ContainerTemplate, DeploymentManifest, DeploymentSpec, HpaManifest,
    HpaSpec, IngressManifest, IngressPath, IngressRule, IngressSpec, NodeStatus, ObjectMetadata,
    PodSpec, PodTemplateSpec, ServiceManifest, ServicePort, ServiceSpec, WidyaKubeCluster,
};

#[test]
fn test_k8s_manifest_parsing_and_apply_deployment() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_k8s_test_dep_{}", pid));
    let _ = fs::remove_dir_all(&temp_dir);

    let runtime = std::sync::Arc::new(WidyaBoxEngine::new(&temp_dir).unwrap());
    let cluster = WidyaKubeCluster::new(runtime);

    // 1. Buat Deployment Manifest Deklaratif
    let mut selector = HashMap::new();
    selector.insert("app".to_string(), "layanan-fintech".to_string());

    let dep_manifest = DeploymentManifest {
        api_version: "apps/v1".to_string(),
        kind: "Deployment".to_string(),
        metadata: ObjectMetadata {
            name: "fintech-core".to_string(),
            namespace: "production".to_string(),
            labels: selector.clone(),
            annotations: HashMap::new(),
        },
        spec: DeploymentSpec {
            replicas: 3,
            selector: selector.clone(),
            template: PodTemplateSpec {
                metadata: ObjectMetadata {
                    name: "fintech-core".to_string(),
                    namespace: "production".to_string(),
                    labels: selector.clone(),
                    annotations: HashMap::new(),
                },
                spec: PodSpec {
                    containers: vec![ContainerTemplate {
                        name: "api-server".to_string(),
                        image: "widya-base:latest".to_string(),
                        command: vec!["/bin/widya".to_string()],
                        ports: Vec::new(),
                        env: HashMap::new(),
                        resources: None,
                        liveness_probe: None,
                        readiness_probe: None,
                    }],
                    restart_policy: "Always".to_string(),
                    node_selector: HashMap::new(),
                },
            },
            strategy: Default::default(),
        },
    };

    // 2. Terapkan Deployment ke Kluster
    let active_dep = cluster
        .apply_deployment(dep_manifest)
        .expect("Apply deployment harus sukses");

    assert_eq!(active_dep.name, "fintech-core");
    assert_eq!(active_dep.desired_replicas, 3);
    assert_eq!(active_dep.available_replicas, 3);
    assert_eq!(active_dep.pod_ids.len(), 3);

    // 3. Verifikasi Pods terdaftar dan berjalan di Node
    let pods_lock = cluster.pods.read().unwrap();
    assert_eq!(pods_lock.len(), 3);
    for pod in pods_lock.values() {
        assert_eq!(pod.status, ClusterPodStatus::Running);
        assert!(pod.ip_address.starts_with("10.244.1."));
        assert!(pod.node_name.starts_with("widya-"));
    }

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_k8s_service_and_endpoint_routing() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_k8s_test_svc_{}", pid));
    let _ = fs::remove_dir_all(&temp_dir);

    let runtime = std::sync::Arc::new(WidyaBoxEngine::new(&temp_dir).unwrap());
    let cluster = WidyaKubeCluster::new(runtime);

    let mut selector = HashMap::new();
    selector.insert("app".to_string(), "auth-service".to_string());

    // Deployment 2 replika
    let dep_manifest = DeploymentManifest {
        api_version: "apps/v1".to_string(),
        kind: "Deployment".to_string(),
        metadata: ObjectMetadata {
            name: "auth-service".to_string(),
            namespace: "default".to_string(),
            labels: selector.clone(),
            annotations: HashMap::new(),
        },
        spec: DeploymentSpec {
            replicas: 2,
            selector: selector.clone(),
            template: PodTemplateSpec {
                metadata: ObjectMetadata {
                    name: "auth-service".to_string(),
                    namespace: "default".to_string(),
                    labels: selector.clone(),
                    annotations: HashMap::new(),
                },
                spec: PodSpec {
                    containers: vec![ContainerTemplate {
                        name: "auth-container".to_string(),
                        image: "widya-base:latest".to_string(),
                        command: vec![],
                        ports: Vec::new(),
                        env: HashMap::new(),
                        resources: None,
                        liveness_probe: None,
                        readiness_probe: None,
                    }],
                    restart_policy: "Always".to_string(),
                    node_selector: HashMap::new(),
                },
            },
            strategy: Default::default(),
        },
    };
    cluster.apply_deployment(dep_manifest).unwrap();

    // Buat Service ClusterIP
    let svc_manifest = ServiceManifest {
        api_version: "v1".to_string(),
        kind: "Service".to_string(),
        metadata: ObjectMetadata {
            name: "auth-service-lb".to_string(),
            namespace: "default".to_string(),
            labels: HashMap::new(),
            annotations: HashMap::new(),
        },
        spec: ServiceSpec {
            service_type: "ClusterIP".to_string(),
            selector: selector.clone(),
            ports: vec![ServicePort {
                name: "http".to_string(),
                port: 80,
                target_port: 8080,
                node_port: None,
            }],
        },
    };

    let active_svc = cluster.apply_service(svc_manifest).unwrap();
    assert_eq!(active_svc.name, "auth-service-lb");
    assert!(active_svc.cluster_ip.starts_with("10.96.0."));
    assert_eq!(active_svc.endpoint_ips.len(), 2);

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_k8s_deployment_scaling() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_k8s_test_scale_{}", pid));
    let _ = fs::remove_dir_all(&temp_dir);

    let runtime = std::sync::Arc::new(WidyaBoxEngine::new(&temp_dir).unwrap());
    let cluster = WidyaKubeCluster::new(runtime);

    let mut selector = HashMap::new();
    selector.insert("app".to_string(), "portal-warga".to_string());

    let dep_manifest = DeploymentManifest {
        api_version: "apps/v1".to_string(),
        kind: "Deployment".to_string(),
        metadata: ObjectMetadata {
            name: "portal-warga".to_string(),
            namespace: "default".to_string(),
            labels: selector.clone(),
            annotations: HashMap::new(),
        },
        spec: DeploymentSpec {
            replicas: 2,
            selector: selector.clone(),
            template: PodTemplateSpec {
                metadata: ObjectMetadata {
                    name: "portal-warga".to_string(),
                    namespace: "default".to_string(),
                    labels: selector.clone(),
                    annotations: HashMap::new(),
                },
                spec: PodSpec {
                    containers: vec![ContainerTemplate {
                        name: "portal-web".to_string(),
                        image: "widya-base:latest".to_string(),
                        command: vec![],
                        ports: Vec::new(),
                        env: HashMap::new(),
                        resources: None,
                        liveness_probe: None,
                        readiness_probe: None,
                    }],
                    restart_policy: "Always".to_string(),
                    node_selector: HashMap::new(),
                },
            },
            strategy: Default::default(),
        },
    };
    cluster.apply_deployment(dep_manifest).unwrap();
    assert_eq!(cluster.pods.read().unwrap().len(), 2);

    // Scale Up ke 5 replika
    let scaled_up = cluster.scale_deployment("portal-warga", 5).unwrap();
    assert_eq!(scaled_up, 5);
    assert_eq!(cluster.pods.read().unwrap().len(), 5);

    // Scale Down ke 1 replika
    let scaled_down = cluster.scale_deployment("portal-warga", 1).unwrap();
    assert_eq!(scaled_down, 1);
    assert_eq!(cluster.pods.read().unwrap().len(), 1);

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_k8s_self_healing_auto_recovery() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_k8s_test_heal_{}", pid));
    let _ = fs::remove_dir_all(&temp_dir);

    let runtime = std::sync::Arc::new(WidyaBoxEngine::new(&temp_dir).unwrap());
    let cluster = WidyaKubeCluster::new(runtime);

    let mut selector = HashMap::new();
    selector.insert("app".to_string(), "basisdata-cache".to_string());

    let dep_manifest = DeploymentManifest {
        api_version: "apps/v1".to_string(),
        kind: "Deployment".to_string(),
        metadata: ObjectMetadata {
            name: "basisdata-cache".to_string(),
            namespace: "default".to_string(),
            labels: selector.clone(),
            annotations: HashMap::new(),
        },
        spec: DeploymentSpec {
            replicas: 3,
            selector: selector.clone(),
            template: PodTemplateSpec {
                metadata: ObjectMetadata {
                    name: "basisdata-cache".to_string(),
                    namespace: "default".to_string(),
                    labels: selector.clone(),
                    annotations: HashMap::new(),
                },
                spec: PodSpec {
                    containers: vec![ContainerTemplate {
                        name: "cache".to_string(),
                        image: "widya-base:latest".to_string(),
                        command: vec![],
                        ports: Vec::new(),
                        env: HashMap::new(),
                        resources: None,
                        liveness_probe: None,
                        readiness_probe: None,
                    }],
                    restart_policy: "Always".to_string(),
                    node_selector: HashMap::new(),
                },
            },
            strategy: Default::default(),
        },
    };
    cluster.apply_deployment(dep_manifest).unwrap();
    assert_eq!(cluster.pods.read().unwrap().len(), 3);

    // Simulasi kegagalan: 1 Pod mati/crash
    let dead_pod_id = {
        let mut pods_lock = cluster.pods.write().unwrap();
        let first_id = pods_lock.keys().next().unwrap().clone();
        pods_lock.get_mut(&first_id).unwrap().status = ClusterPodStatus::Failed;
        first_id
    };

    // Jalankan rekonsiliasi Self-Healing loop
    let heal_events = cluster.trigger_self_healing_check();
    assert!(!heal_events.is_empty());
    assert!(heal_events[0].contains("[SELF-HEALING]"));

    // Pods yang Running harus kembali 3
    let running_pods: Vec<_> = cluster
        .pods
        .read()
        .unwrap()
        .values()
        .filter(|p| p.status == ClusterPodStatus::Running)
        .cloned()
        .collect();
    assert_eq!(running_pods.len(), 3);

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_k8s_horizontal_pod_autoscaling() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_k8s_test_hpa_{}", pid));
    let _ = fs::remove_dir_all(&temp_dir);

    let runtime = std::sync::Arc::new(WidyaBoxEngine::new(&temp_dir).unwrap());
    let cluster = WidyaKubeCluster::new(runtime);

    let mut selector = HashMap::new();
    selector.insert("app".to_string(), "layanan-pembayaran".to_string());

    let dep_manifest = DeploymentManifest {
        api_version: "apps/v1".to_string(),
        kind: "Deployment".to_string(),
        metadata: ObjectMetadata {
            name: "layanan-pembayaran".to_string(),
            namespace: "default".to_string(),
            labels: selector.clone(),
            annotations: HashMap::new(),
        },
        spec: DeploymentSpec {
            replicas: 2,
            selector: selector.clone(),
            template: PodTemplateSpec {
                metadata: ObjectMetadata {
                    name: "layanan-pembayaran".to_string(),
                    namespace: "default".to_string(),
                    labels: selector.clone(),
                    annotations: HashMap::new(),
                },
                spec: PodSpec {
                    containers: vec![ContainerTemplate {
                        name: "payment-api".to_string(),
                        image: "widya-base:latest".to_string(),
                        command: vec![],
                        ports: Vec::new(),
                        env: HashMap::new(),
                        resources: None,
                        liveness_probe: None,
                        readiness_probe: None,
                    }],
                    restart_policy: "Always".to_string(),
                    node_selector: HashMap::new(),
                },
            },
            strategy: Default::default(),
        },
    };
    cluster.apply_deployment(dep_manifest).unwrap();

    // Buat HPA Manifest (Target CPU 50%, Min 2, Max 6)
    let hpa_manifest = HpaManifest {
        api_version: "autoscaling/v2".to_string(),
        kind: "HorizontalPodAutoscaler".to_string(),
        metadata: ObjectMetadata {
            name: "payment-hpa".to_string(),
            namespace: "default".to_string(),
            labels: HashMap::new(),
            annotations: HashMap::new(),
        },
        spec: HpaSpec {
            target_deployment: "layanan-pembayaran".to_string(),
            min_replicas: 2,
            max_replicas: 6,
            target_cpu_utilization_percentage: 50,
        },
    };

    let active_hpa = cluster.apply_hpa(hpa_manifest).unwrap();
    assert_eq!(active_hpa.name, "payment-hpa");

    // Simulasi lonjakan beban CPU menjadi 85%
    {
        let mut hpa_lock = cluster.hpas.write().unwrap();
        hpa_lock.get_mut("payment-hpa").unwrap().current_cpu_pct = 85;
    }

    // Trigger HPA Autoscaling Loop
    let scale_events = cluster.trigger_hpa_autoscaling_tick();
    assert!(!scale_events.is_empty());
    assert!(scale_events[0].contains("[AUTOSCALE HPA]"));

    // Replika harus bertambah dari 2 -> 3
    let dep_replicas = cluster
        .deployments
        .read()
        .unwrap()
        .get("layanan-pembayaran")
        .unwrap()
        .desired_replicas;
    assert_eq!(dep_replicas, 3);

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_k8s_ingress_routing() {
    let pid = std::process::id();
    let temp_dir = std::env::temp_dir().join(format!("widya_k8s_test_ing_{}", pid));
    let _ = fs::remove_dir_all(&temp_dir);

    let runtime = std::sync::Arc::new(WidyaBoxEngine::new(&temp_dir).unwrap());
    let cluster = WidyaKubeCluster::new(runtime);

    let ing_manifest = IngressManifest {
        api_version: "networking.k8s.io/v1".to_string(),
        kind: "Ingress".to_string(),
        metadata: ObjectMetadata {
            name: "gateway-utama".to_string(),
            namespace: "default".to_string(),
            labels: HashMap::new(),
            annotations: HashMap::new(),
        },
        spec: IngressSpec {
            rules: vec![
                IngressRule {
                    host: "api.widya.id".to_string(),
                    paths: vec![
                        IngressPath {
                            path: "/v1/auth".to_string(),
                            service_name: "auth-service".to_string(),
                            service_port: 80,
                        },
                        IngressPath {
                            path: "/v1/bayar".to_string(),
                            service_name: "payment-service".to_string(),
                            service_port: 80,
                        },
                    ],
                },
            ],
        },
    };

    let active_ing = cluster.apply_ingress(ing_manifest).unwrap();
    assert_eq!(active_ing.name, "gateway-utama");
    assert_eq!(active_ing.spec.rules[0].paths.len(), 2);

    let _ = fs::remove_dir_all(temp_dir);
}
