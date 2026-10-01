# 📦 WidyaBox: Sovereign Rootless Container & Pod Engine (Podman & Docker Equivalent)

**WidyaBox** (`widya::container`) adalah mesin kontainer dan orkestrator Pod berdaulat (*Daemonless & Rootless Container Engine*) yang dirancang khusus untuk ekosistem **Widya-Lang**. 

WidyaBox menghadirkan pengalaman eksekusi kontainer kelas industri yang setara dengan **Podman** dan **Docker**, namun dengan arsitektur **tanpa daemon (*daemonless*)**, spesifikasi image OCI berbasis Merkle Tree SHA-256, isolasi sandbox native, serta dukungan **Kubernetes/Podman-compatible Pods**.

---

## 🚀 Perbandingan Fitur: Podman vs Docker vs WidyaBox

| Fitur Utama | Docker | Podman | WidyaBox (Widya-Lang) |
| :--- | :--- | :--- | :--- |
| **Arsitektur Daemon** | Wajib (`dockerd` root) | **Daemonless** (Direct CLI) | **Daemonless & Rootless Penuh** |
| **Hak Akses (*Privilege*)**| Sering butuh `root`/sudo | Rootless Native | **100% Rootless Sandbox Native** |
| **Konsep Pod** | Tidak Ada (Hanya Container) | **Podman Pods (K8s Spec)**| **Kubernetes/Podman Pods Built-in** |
| **Spesifikasi Image** | OCI / Docker Image v2 | OCI Image Specification | **OCI Layered Merkle Tree (SHA-256)** |
| **Bahasa Definisi Image**| `Dockerfile` | `Containerfile` | **`Widyafile` (Dwibahasa) & `Dockerfile`** |
| **Multi-Container Compose**| `docker-compose` | `podman-compose` | **`widya box compose` (`widyacompose.json`)**|
| **Port Forwarding** | Docker Proxy / iptables | pasta / slirp4netns | **Built-in User-Space TCP Port Proxy** |
| **Kedaulatan & Ukuran** | Binari C/Go (~100MB+) | Runtimes eksternal (crun)| **Zero-Dependency Rust Native Sub-ms** |

---

## 🛠️ Panduan Perintah CLI (`widya box` / `widya podman`)

Widya-Lang menyematkan subperintah `widya box` (alias: `widya podman`, `widya container`, `widya pod`) langsung ke dalam compiler utama:

### 1. Membangun Image OCI dari `Widyafile` / `Dockerfile`
```bash
# Bangun image dengan tag
widya box build -t my-app:v1.0 -f Widyafile .
```

### 2. Menjalankan Kontainer Baru
```bash
# Jalankan di latar belakang (-d), mapping port 8080 ke 80, dan beri nama kontainer
widya box run -d -p 8080:80 --name web-layanan-publik widya-base:latest

# Jalankan kontainer dengan volume bind-mount dan variabel lingkungan
widya box run -d -p 5432:5432 -v /data/db:/var/lib/data -e DB_NAME=widyadb widya-base:latest
```

### 3. Melihat Daftar Kontainer
```bash
# Kontainer yang sedang berjalan
widya box ps

# Seluruh kontainer (termasuk yang dihentikan)
widya box ps -a
```

### 4. Melihat Daftar Image OCI Tersimpan
```bash
widya box images
```

### 5. Menghentikan & Memulai Kontainer
```bash
# Hentikan kontainer
widya box stop web-layanan-publik

# Mulai kembali kontainer
widya box start web-layanan-publik

# Hapus kontainer
widya box rm web-layanan-publik
```

### 6. Melihat Log & Eksekusi Perintah Interaktif
```bash
# Melihat log kontainer
widya box logs web-layanan-publik

# Eksekusi perintah di dalam sandbox kontainer yang berjalan
widya box exec web-layanan-publik ls -la /app
```

---

## 🛡️ Manajemen Pod Kontainer (`widya pod`)

Seperti pada Podman dan Kubernetes, sebuah **Pod** adalah kelompok dari satu atau lebih kontainer yang berbagi *namespace* jaringan (IP yang sama) dan memori:

```bash
# 1. Membuat Pod Baru dengan Port Terbuka
widya pod create pod-fintech-gateway -p 8080:8080

# 2. Menjalankan Kontainer di Dalam Pod
widya box run -d --name service-auth --pod pod-fintech-gateway widya-base:latest
widya box run -d --name service-payment --pod pod-fintech-gateway widya-base:latest

# 3. Menampilkan Daftar Pod
widya pod ps

# 4. Menghentikan & Memulai Seluruh Kontainer dalam Pod Sekaligus
widya pod stop pod-fintech-gateway
widya pod start pod-fintech-gateway
```

---

## 📄 Format Spesifikasi `Widyafile` (Dwibahasa)

`Widyafile` mendukung sintaks Bahasa Indonesia dan Inggris secara bersamaan:

```dockerfile
# Contoh Widyafile untuk Microservice Berdaulat
DARI widya-base:latest

LINGKUNGAN PORT=8080
LINGKUNGAN ZONA_WAKTU=Asia/Jakarta

KERJA /app

SALIN src /app/src
SALIN widya.toml /app/widya.toml

JALANKAN widya pasang

PORT 8080

TITIK_MASUK ["/bin/widya"]
PERINTAH ["jalankan", "/app/src/main.wya"]

LABEL organisasi="Pemerintah Digital Nusantara"
LABEL versi="1.0.0"
```

---

## 🐙 Orkestrasi Multi-Kontainer (`widyacompose.json`)

Setara dengan `docker-compose.yml`, Anda dapat mendefinisikan arsitektur multi-layanan dalam satu berkas:

```json
{
  "version": "3.8",
  "services": {
    "web-api": {
      "image": "widya-app:v1.0",
      "ports": ["8080:8080"],
      "environment": {
        "DB_HOST": "basisdata-layanan",
        "CACHE_HOST": "cache-layanan"
      },
      "command": "widya run server.wya"
    },
    "basisdata-layanan": {
      "image": "widya-base:latest",
      "ports": ["5432:5432"],
      "command": "widyabase server start"
    },
    "cache-layanan": {
      "image": "widya-base:latest",
      "ports": ["6379:6379"],
      "command": "widyabase cache start"
    }
  }
}
```

Jalankan orkestrasi sekali klik:
```bash
widya box compose widyacompose.json
```
