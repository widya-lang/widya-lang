# SOP DEVOPS, DEPLOYMENT, & SITE RELIABILITY ENGINEERING (SRE)
## PROYEK: SISTEM KOPERASI DESA MERAH PUTIH (KDMP)
**Disusun Oleh:** `agen_devops` (SOP-06: Monitoring Server, Backup Harian, & Service Daemon)  
**Status:** SIAP PRODUKSI (PRODUCTION READY)  

---

### 1. Prasyarat Lingkungan (Environment Prerequisites)
- **Runtime Compiler**: `widya` binary v1.0.0+ (Linux x86_64 / ARM64 / Windows).
- **Storage**: Direktori `/home/devops/widya-lang/proyek/01_koperasi_kdmp/`.
- **Port Layanan**: `http://localhost:8001` (Khusus Koperasi KDMP).

---

### 2. Panduan Menjalankan Layanan Mandiri (Stand-alone Run)
Untuk menjalankan server mandiri Koperasi KDMP:
```bash
cd /home/devops/widya-lang/proyek/01_koperasi_kdmp
../../target/release/widya server.wya
```
Atau menggunakan perintah web server bawaan Widya:
```bash
../../target/release/widya web dev ./web -p 8001
```

---

### 3. Kebijakan Backup Harian & WAL Checkpoint
1. **Automasi Cron Job (02:00 UTC / 09:00 WIB)**:
   * Snapshot basis data `database/koperasi.wdb` dan arsip WAL disalin ke folder `database/backups/`.
   * Checkpoint WAL dibersihkan secara aman untuk menjaga ukuran file tetap ramping.
2. **Crash Recovery Drill**:
   * Jika server mati mendadak, pada saat startup berikutnya mesin WidyaBase akan membaca log WAL dan melakukan *redo/undo logging* dalam waktu < 1.5 detik.

---

### 4. Konfigurasi Systemd Service (Linux Server)
Simpan berkas `/etc/systemd/system/widya-kdmp.service`:
```ini
[Unit]
Description=Widya Koperasi Desa Merah Putih Service
After=network.target

[Service]
Type=simple
User=devops
WorkingDirectory=/home/devops/widya-lang/proyek/01_koperasi_kdmp
ExecStart=/home/devops/widya-lang/target/release/widya web dev ./web -p 8001
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```
