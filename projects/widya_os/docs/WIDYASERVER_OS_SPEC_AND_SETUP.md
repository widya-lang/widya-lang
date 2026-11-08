# Spesifikasi & Panduan Pengaturan WidyaServerOS 🖥️⚡

> **Arsitektur Sistem Operasi Server Backbone Berdaulat & Panduan Konfigurasi Data Center**  
> *Headless Minimalist Appliance · RAM Idle < 60 MB · Booting < 500 ms · io_uring & eBPF · PQC TLS 1.3 Termination*

---

## 1. Pendahuluan & Strategi Dua Fase

Dalam mengoperasikan server backbone data center, ada dua jalur implementasi:

```
┌────────────────────────────────────────────────────────────────────────┐
│                   JALUR IMPLEMENTASI SISTEM OPERASI SERVER             │
├───────────────────────────────────┬────────────────────────────────────┤
│ FASE 1: PRAKTIS & CEPAT           │ FASE 2: KEDAULATAN MUTLAK          │
│ (Hari Ini di Cloud Sewaan)        │ (Masa Depan Data Center Mandiri)   │
├───────────────────────────────────┼────────────────────────────────────┤
│ Menggunakan OS Basis Teruji:      │ Membangun **WidyaServerOS** Murni: │
│ **Debian 12 Minimal / Alpine**    │ Citra ISO Server Berdaulat Mandiri │
│ Di-harden otomatis dengan skrip   │ Tanpa dependensi distro luar       │
│ biner murni Widya-Lang.           │ Langsung di-flash via IPMI / KVM.  │
└───────────────────────────────────┴────────────────────────────────────┘
```

---

## 2. Fase 1: Konfigurasi Server Hari Ini (Debian 12 Minimal / Hetzner / IDCloudHost)

Jika Anda menyewa server di **Hetzner Dedicated** atau **IDCloudHost Jakarta** hari ini:
- **Pilihlah OS Image:** **Debian 12 (Bookworm) Minimal 64-bit** (atau **Alpine Linux Standard** jika ingin biner super kecil).
- **Mengapa Debian 12 Minimal?** Sangat stabil (*rock-solid*), bebas bug aneh, tidak memiliki bloatware GUI, dan kompatibel 100% dengan biner statis Rust/Widya-Lang.

### A. Pengaturan Pengerasan Keamanan (Security Hardening Checklist)
1. **Nonaktifkan Login Kata Sandi SSH:** Hanya izinkan otentikasi kunci kriptografi modern (**Ed25519** atau kunci pasca-kuantum).
2. **Ganti Port Standar SSH:** Pindahkan port SSH dari port 22 ke port khusus (misal port 2222 atau 49152) untuk mengeliminasi 99% serangan bot brute-force otomatis di internet.
3. **Firewall Berdaulat (nftables / UFW):** Hanya buka port yang benar-benar dibutuhkan:
   - Port `80` & `443` (HTTP/3 QUIC Web & API Gateway).
   - Port `25`, `465`, `587`, `993` (Protokol Surel WidyaMail `@widya.email`).
   - Port `9090` (Pusat Kendali Admin WidyaControl — hanya diizinkan dari IP VPN Admin).
4. **Isolasi Proses Non-Root:** Seluruh layanan Widya (WidyaBase, WidyaDrive, WidyaMail) dijalankan di bawah pengguna sistem terisolasi tanpa hak akses root (`widya-daemon`).

---

### B. Optimasi Kinerja Jaringan Tingkat Tinggi (Kernel Sysctl Tuning)
Tambahkan konfigurasi berikut pada `/etc/sysctl.d/99-widya-backbone.conf` untuk menangani **1.000.000 koneksi konkuren secara simultan**:

```ini
# Algoritma Pengendalian Kemacetan Jaringan Modern Google BBR
net.core.default_qdisc = fq
net.ipv4.tcp_congestion_control = bbr

# Peningkatan Antrian Koneksi TCP (Mencegah Paket Drop saat Lonjakan Trafik)
net.core.somaxconn = 65535
net.ipv4.tcp_max_syn_backlog = 65535
net.core.netdev_max_backlog = 65535

# Kapasitas Buffer Memori Jaringan (10 Gbps Peering)
net.ipv4.tcp_rmem = 4096 87380 33554432
net.ipv4.tcp_wmem = 4096 65536 33554432

# Alokasi Maksimal File Descriptor
fs.file-max = 2097152
```

---

## 3. Fase 2: Membangun *WidyaServerOS* Mandiri dari Nol

**Ya, kita SANGAT BISA membangun WidyaServerOS sendiri!**

Berbeda dengan WidyaOS versi Desktop yang memiliki *Compositor DRM/KMS, Window Manager, dan Wallpaper*, **WidyaServerOS adalah sistem operasi server tanpa antarmuka grafis (*Headless Appliance*)**:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        TUMPUKAN WIDYASERVEROS                          │
├────────────────────────────────────────────────────────────────────────┤
│ 1. LAYANAN BACKBONE TERPADU (BUILT-IN SYSTEM DAEMONS)                  │
│    ├── widya-mail-engine    : SMTP / JMAP server (@widya.email)        │
│    ├── widya-storage-node   : Erasure Coding 8+4 & deduplikasi blok    │
│    ├── widya-base-raft      : Raft consensus metadata & billing ledger │
│    └── widya-compute-worker : Firecracker MicroVM executor             │
├────────────────────────────────────────────────────────────────────────┤
│ 2. WIDYASERVERINIT (INIT SYSTEM RING 3)                                │
│    - Peluncur daemon paralel berkecepatan tinggi (Booting < 300 ms)    │
│    - Supervisor pemantau kesehatan proses (Auto-restart jika crash)    │
├────────────────────────────────────────────────────────────────────────┤
│ 3. SUBSISTEM JARINGAN MURNI (IO_URING & EBPF)                          │
│    - Komunikasi I/O asinkron tanpa overhead interrupt context-switch   │
│    - Terminasi enkripsi TLS 1.3 Pasca-Kuantum di soket kernel          │
├────────────────────────────────────────────────────────────────────────┤
│ 4. KERNEL MINIMALIS BARE-METAL                                         │
│    - Tanpa driver grafis, tanpa audio ALSA, tanpa Bluetooth            │
│    - Hanya driver CPU, NVMe SSD, dan Ethernet 10G/40G/100G NIC         │
└────────────────────────────────────────────────────────────────────────┘
```

### Karakteristik Ekstrem WidyaServerOS:
1. **Ukuran Citra Biner Sangat Kecil:** Ukuran ISO installer **< 80 MB** (dibandingkan Windows Server 2022 yang berukuran 5 GB atau Ubuntu Server 2 GB).
2. **Konsumsi RAM Idle Sangat Rendah:** Hanya memakan **< 60 MB RAM** saat idle! Artinya, pada server dengan RAM 64 GB, **99% memori RAM murni digunakan untuk melayani pengguna**, bukan terbuang untuk sistem operasi.
3. **Waktu Booting Server:** Boot dari BIOS/UEFI hingga seluruh port jaringan siap menerima request dalam waktu **< 500 milidetik (0,5 detik)**.
4. **Sistem Berkas Kebal (Read-Only RootFS):** Tidak bisa disusupi virus/malware server karena seluruh sistem file inti terkunci *read-only* dengan pohon hash Merkle (*dm-verity*).

---

## 4. Antarmuka Manajemen Administrator di WidyaServerOS

Meskipun tanpa GUI desktop berat, administrator tetap mendapatkan kemudahan pemantauan modern melalui dua cara:

### A. Terminal TUI Dashboard (`widya-server top`)
Saat sysadmin terhubung via SSH, layar menampilkan dashboard interaktif berbasis teks berwarna (*ncurses-like terminal UI*):
```
┌────────────────────────────────────────────────────────────────────────┐
│ WIDYASERVEROS BACKBONE v1.0.0                      UPTIME: 142 HARI    │
├────────────────────────────────────────────────────────────────────────┤
│ CPU: [||||||||                      ] 24.2%  (AMD Ryzen 12-Core @4.5GHz│
│ RAM: [||||                          ] 14.8 GB / 64.0 GB (49.2 GB Cache)│
│ NVMe:[||||||||||                    ] 342 GB / 3.84 TB  (IO: 4.8 MB/s) │
│ NET: RX: 142.5 Mbps  |  TX: 388.2 Mbps (1 Gbps Full-Duplex)            │
├────────────────────────────────────────────────────────────────────────┤
│ DAEMON STATUS:                                                         │
│ [🟢 AKTIF] widya-mail-engine      (wmail @widya.email: 42 surat/detik) │
│ [🟢 AKTIF] widya-storage-cluster  (Erasure Coding 8+4: Sehat)          │
│ [🟢 AKTIF] widya-base-consensus   (Raft Leader Node 01: Sinkron)       │
│ [🟢 AKTIF] widya-compute-worker   (0 MicroVM aktif - Mode Siaga)       │
├────────────────────────────────────────────────────────────────────────┤
│ [Q] Keluar  [R] Restart Layanan  [L] Lihat Log  [K] Rotasi Kunci HSM   │
└────────────────────────────────────────────────────────────────────────┘
```

### B. Pusat Kendali Web Admin Terenkripsi (Port 9090)
Dapat dibuka melalui browser laptop sysadmin di alamat `https://ip-server:9090` dengan otentikasi dua faktor (*2FA hardware token*) untuk mengelola pengguna, kuota 1 TB, dan melihat grafik performa server.

---

## 5. Perintah Membangun Citra ISO WidyaServerOS

Di masa depan, ketika siap memasang WidyaServerOS langsung ke server fisik bare-metal di data center:

```bash
# Membangun berkas ISO headless server dari repositori Widya
widya server buat-iso \
  --nama "widya-server-enterprise" \
  --arsitektur x86_64 \
  --edisi "Headless Cloud Appliance" \
  --output ./dist/widya-server-enterprise-1.0.0.iso

# Memasang ke server fisik via IPMI / KVM virtual media
# Server otomatis terpasang dalam 45 detik tanpa intervensi manual (Unattended Cloud-Init)
```
