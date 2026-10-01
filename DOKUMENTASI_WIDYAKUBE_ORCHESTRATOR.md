# ☸️ WidyaKube — Sovereign Kubernetes-Grade Multi-Node Cluster Orchestrator

**WidyaKube** adalah sistem orkestrasi klaster terdistribusi mandiri (*sovereign container & pod orchestrator*) bawaan **Widya-Lang**. WidyaKube dirancang untuk mengelola ribuan Pod dan Container melintasi node bare-metal maupun cloud tanpa dependensi pada daemon luar (seperti `kubelet` atau `etcd` berat), menggunakan arsitektur Rust yang sangat ringan, aman, dan berkecepatan tinggi.

---

## 🌟 Fitur Utama WidyaKube

1. **Declarative Manifests (YAML / JSON / Bilingual .wya)**:
   - Mendukung spesifikasi standar Kubernetes: `Deployment`, `Service`, `Ingress`, `HorizontalPodAutoscaler (HPA)`, `ConfigMap`, dan `Secret`.
   - Bilingual support (Bahasa Indonesia & English).
2. **Intelligent Scheduler**:
   - Algoritma *Least-Allocated / Bin-Packing* berbasis real-time CPU (milliCPU) & Memory (MB) allocation.
   - Node status checking (*Ready*, *NotReady*, *Draining*).
3. **Controller Manager & Self-Healing**:
   - Reconcile loop otomatis yang memonitor *desired replicas* vs *actual running pods*.
   - Deteksi otomatis kegagalan node/pod dan penjadwalan ulang (*reschedule & restart*) instan tanpa downtime.
4. **Zero-Downtime Rolling Update & Rollback**:
   - Strategi `RollingUpdate` cerdas dengan `max_surge` dan `max_unavailable`.
   - Riwayat revisi rilis dan rollback satu perintah (`widya k8s rollout undo`).
5. **Built-in Service Mesh & ClusterIP DNS**:
   - Alokasi IP internal (`10.96.x.x`), DNS resolution internal klaster (`<service-name>.<namespace>.svc.widya`).
   - Round-robin load balancing ke Pod Endpoints sehat.
6. **Horizontal Pod Autoscaler (HPA)**:
   - Metrik CPU/RAM real-time memicu scale up/scale down otomatis berdasarkan target utilisasi persentase.

---

## 🏗️ Struktur Arsitektur WidyaKube

```
                     +----------------------------------------+
                     |           widya k8s CLI / API          |
                     +----------------------------------------+
                                         |
                                         v
                     +----------------------------------------+
                     |        WidyaKube Control Plane         |
                     |  - API Engine / Manifest Parser       |
                     |  - Least-Allocated Scheduler           |
                     |  - Controller Manager (Self-Healing)   |
                     |  - Service Mesh / ClusterIP DNS        |
                     |  - HPA Auto-scaler Controller          |
                     +----------------------------------------+
                                  /      |      \
                                 /       |       \
               +----------------+ +---------------+ +----------------+
               |  Worker Node 1 | | Worker Node 2 | |  Worker Node 3 |
               | (WidyaBox Pods)| | (WidyaBox Pods)| | (WidyaBox Pods)|
               +----------------+ +---------------+ +----------------+
```

---

## 📜 Contoh Manifest WidyaKube

### `deployment.yaml`
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: widyaweb-portal
  namespace: produksi
  labels:
    app: portal
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    maxSurge: 1
    maxUnavailable: 0
  selector:
    matchLabels:
      app: portal
  template:
    metadata:
      labels:
        app: portal
    spec:
      containers:
        - name: app
          image: widya/portal:v1.2.0
          ports:
            - containerPort: 3000
          resources:
            requests:
              cpu: "250m"
              memory: "512Mi"
            limits:
              cpu: "500m"
              memory: "1024Mi"
```

### `service.yaml`
```yaml
apiVersion: v1
kind: Service
metadata:
  name: portal-service
  namespace: produksi
spec:
  type: ClusterIP
  selector:
    app: portal
  ports:
    - port: 80
      targetPort: 3000
```

---

## 💻 CLI Commands Cheat Sheet

| Perintah | Deskripsi |
|---|---|
| `widya k8s init` | Inisialisasi Control Plane dan cluster lokal |
| `widya k8s apply -f <manifest.yaml>` | Menerapkan manifest (Deployment, Service, dll) |
| `widya k8s get nodes` | Menampilkan daftar worker nodes dan kapasitasnya |
| `widya k8s get pods [-n <namespace>]` | Menampilkan seluruh Pod, status, IP, dan node |
| `widya k8s get deployments` | Menampilkan status deployment dan replika |
| `widya k8s get services` | Menampilkan daftar Service, ClusterIP, dan Endpoints |
| `widya k8s scale deploy <nama> --replicas=N` | Melakukan manual scaling replika deployment |
| `widya k8s rollout restart deploy <nama>` | Melakukan zero-downtime rolling restart |
| `widya k8s logs <pod-id>` | Membaca log output dari pod |

---

## 🧪 Validasi & Pengujian

WidyaKube telah diuji secara komprehensif melalui suite pengujian otomatis di `tests/orchestrator_tests.rs`:
1. `test_node_registration_and_scheduling`: Pendaftaran 2 node dan verifikasi Least-Allocated Scheduler.
2. `test_deployment_controller_and_replicas`: Reconcile 3 replika pod secara otomatis.
3. `test_service_mesh_and_endpoints`: Registrasi ClusterIP dan round-robin load balancing.
4. `test_self_healing_controller`: Simulasi pod down dan auto-recovery penggantian pod otomatis.
5. `test_rolling_update`: Zero-downtime rolling update dari versi v1 ke v2.
6. `test_hpa_autoscaling`: Auto-scale up dari 2 ke 5 replika saat utilisasi CPU mencapai 85%.
