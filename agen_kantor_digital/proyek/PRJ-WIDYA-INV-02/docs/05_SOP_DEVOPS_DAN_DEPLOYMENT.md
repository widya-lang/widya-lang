# SOP DEVOPS, DEPLOYMENT & PEMELIHARAAN (05_SOP_DEVOPS_DAN_DEPLOYMENT)
## PROYEK: SISTEM MANAJEMEN INVENTARIS UMKM NUSANTARA (PRJ-WIDYA-INV-02)
**Kode Proyek**: `PRJ-WIDYA-INV-02`  
**Disusun Oleh**: `agen_devops` (SOP-06: Manajemen Infrastruktur, CI/CD & Reliabilitas SRE)  
**Disetujui Oleh**: `agen_orkestrator` & `agen_qa`  
**Status**: DISETUJUI • OPERASIONAL STANDAR GUDANG  

---

### 1. Pre-flight Check & Alokasi Port Dinamis
Sistem Manajemen Inventaris dialokasikan pada port `8011` guna menjaga isolasi penuh terhadap proyek lainnya (Port 8001 s/d 8010).

```bash
# Verifikasi ketersediaan port 8011
netstat -tuln | grep 8011 || ss -tulpn | grep 8011
# Pastikan tidak ada benturan port sebelum menjalankan server
```

---

### 2. Inisialisasi Basis Data WidyaBase Mandiri
Sebelum server web dijalankan, pastikan direktori `database/` siap dan basis data `data.wdb` diinisialisasi:

```bash
cd /home/devops/widya-lang/agen_kantor_digital/proyek/PRJ-WIDYA-INV-02
../../target/release/widya base inisialisasi database/data.wdb
```

---

### 3. Eksekusi Server Mandiri (Runtime Mode)

#### Opsi A: Eksekusi Langsung via `server.wya`
```bash
cd /home/devops/widya-lang/agen_kantor_digital/proyek/PRJ-WIDYA-INV-02
../../target/release/widya server.wya
```

#### Opsi B: Eksekusi Background Daemon (Production)
```bash
nohup ../../target/release/widya web dev ./web -p 8011 > server.log 2>&1 &
echo $! > server.pid
```

---

### 4. Prosedur Verifikasi Healthcheck & Pemantauan
Setelah peluncuran daemon, verifikasi integritas endpoint dilakukan secara otomatis:

```bash
curl -I -s http://localhost:8011 | head -n 5
```
Respon yang diharapkan:
`HTTP/1.1 200 OK`

---

### 5. Prosedur Pemberhentian Graceful & Recovery
Untuk mematikan daemon secara aman tanpa merusak berkas WAL:
```bash
if [ -f server.pid ]; then
  kill -15 $(cat server.pid)
  rm -f server.pid
fi
```
Jika terjadi crash tak terduga, runtime WidyaBase akan memulihkan data konsisten secara otomatis melalui mekanisme ARIES crash-recovery pada `data.wal`.
