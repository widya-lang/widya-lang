# STANDAR OPERASIONAL PROSEDUR (SOP) DEVOPS & DEPLOYMENT
## APLIKASI RENCANA KEGIATAN DAN ANGGARAN SEKOLAH (ARKAS) WEB TERINTEGRASI
**Kode Proyek:** `PRJ-ARKAS-WEB-01` | **Status:** Siap Produksi (Port 8012)

---

### 1. Spesifikasi Runtime & Jaringan
- **Port Layanan**: `http://localhost:8012`
- **Direktori Root Web**: `agen_kantor_digital/proyek/PRJ-ARKAS-WEB-01/web/`
- **Engine Binary**: `/home/devops/widya-lang/target/release/widya`
- **Penyimpanan Data**: `agen_kantor_digital/proyek/PRJ-ARKAS-WEB-01/database/data.wdb`
- **Log Server**: `agen_kantor_digital/proyek/PRJ-ARKAS-WEB-01/server.log`
- **PID Server**: `agen_kantor_digital/proyek/PRJ-ARKAS-WEB-01/server.pid`

---

### 2. Prosedur Menjalankan Server Mandiri
#### Opsi A: Menggunakan Script Launcher Widya-Lang (`server.wya`)
```bash
cd /home/devops/widya-lang/agen_kantor_digital/proyek/PRJ-ARKAS-WEB-01
../../target/release/widya server.wya
```

#### Opsi B: Daemon Mode (Background Orchestrator)
```bash
nohup /home/devops/widya-lang/target/release/widya web dev ./web -p 8012 > server.log 2>&1 &
echo $! > server.pid
```

---

### 3. Prosedur Backup & Pemeliharaan Basis Data
1. **Checkpointing WAL**:
   WidyaBase secara berkala menyinkronkan berkas Write-Ahead Log (`data.wdb.wal`) ke penyimpanan utama (`data.wdb`).
2. **Snapshot Harian**:
   Snapshot basis data otomatis dibuat setiap pukul 00:00 WIB ke subdirektori `database/backups/`.
3. **Pemeriksaan Integritas Berkas**:
   ```bash
   /home/devops/widya-lang/target/release/widya base periksa database/data.wdb
   ```
