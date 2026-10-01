# SOP-06: DevOps & Site Reliability Engineer (DevOps & SRE)

**Kode Dokumen**: `SOP-WIDYA-06`  
**Peran Agen**: `agen_devops`  
**Target File Output**: 
- `proyek/<nama_proyek>/docs/05_SOP_DEVOPS_DAN_DEPLOYMENT.md`
- `proyek/<nama_proyek>/database/data.wdb`
- `proyek/<nama_proyek>/database/data.wdb.wal`
- `proyek/<nama_proyek>/server.pid`
- `proyek/<nama_proyek>/server.log`

---

## 1. Trigger & Prerequisites (Syarat Jalan)
- **Trigger**: Menerima rekomendasi rilis berstatus `PASSED / APPROVED FOR DEVOPS` dari `agen_qa`.
- **Prasyarat Wajib**:
  1. Dokumen `04_RENCANA_PENGUJIAN_QA.md` berstatus `PASSED`.
  2. File `server.wya`, `web/`, dan `database/skema.sql` terpasang lengkap.
  3. Biner resmi runtime Widya-Lang (`target/release/widya`) tersedia dan executable.

---

## 2. Alur Kerja Detil (Step-by-Step Execution)

### Step 1: Pre-Flight Check & Dynamic Port Resolution
1. BACA alokasi port dari `02_ARSITEKTUR_DAN_ERD.md`.
2. PERIKSA apakah port tersebut sedang digunakan di OS (Gunakan mekanisme cek port aktif).
3. Jika port terpakai, CARI port bebas berikutnya (misal: `<PORT> + 1`) dan CATAT perubahan port tersebut ke dalam dokumen deployment.

### Step 2: Inisialisasi Database & Eksekusi Skema SQL
1. Eksekusi inisialisasi penyimpanan WidyaBase:
   ```bash
   widya base inisialisasi "proyek/<nama_proyek>/database/data.wdb"

```

2. Migrasikan DDL dari file `skema.sql` ke database yang baru diinisialisasi:
```bash
widya base import --db "proyek/<nama_proyek>/database/data.wdb" --file "proyek/<nama_proyek>/database/skema.sql"

```


3. Pastikan file `.wdb` dan `.wdb.wal` terbentuk serta tidak *corrupt*.

### Step 3: Deployment & Process Tracking (Daemon Launch)

1. Jalankan server web mandiri dan simpan Process ID (PID):
```bash
nohup widya web dev "proyek/<nama_proyek>/web" -p <PORT> > "proyek/<nama_proyek>/server.log" 2>&1 & echo $! > "proyek/<nama_proyek>/server.pid"

```


2. **Healthcheck Loop**:
* Tunggu 2 detik untuk memastikan server tidak langsung crash.
* Lakukan pengujian respons HTTP hingga 3x percobaan:
```bash
curl -s -o /dev/null -w "%{http_code}" http://localhost:<PORT>

```




3. **Kondisi Fail-Safe / Rollback**:
* Jika HTTP response bukan `200 OK` setelah 3x retry:
a. Matikan proses menggunakan PID di `server.pid`.
b. Baca 20 baris terakhir `server.log`.
c. Kirim sinyal **DEPLOY_FAILED** kembali ke `agen_coder` beserta log error-nya.



### Step 4: Penyusunan Dokumen SOP Deployment (`docs/05_SOP_DEVOPS_DAN_DEPLOYMENT.md`)

Buat dokumen playbook deployment dengan struktur WAJIB berikut:

```markdown
# DevOps, Deployment & SRE Playbook - <Nama Proyek>
**Status**: ACTIVE_DEPLOYED  
**Target Port**: `http://localhost:<PORT>`  
**Process PID**: `<PID_NUM>`

## 1. Spesifikasi Runtime & Perintah Peluncur
- Runtime: Widya-Lang Web Engine
- Command: `widya web dev "proyek/<nama_proyek>/web" -p <PORT>`
- Environment: Production Standalone (Isolated Process)

## 2. Healthcheck & Observability Protocol
- Endpoint Healthcheck: `GET /` -> HTTP 200 OK
- Process PID File: `proyek/<nama_proyek>/server.pid`
- Process Log File: `proyek/<nama_proyek>/server.log`

## 3. Disaster Recovery & Backup Routine
- Strategi Backup: WAL Continuous Archiving (`data.wdb.wal`)
- Recovery Command: `widya base restore --db "data.wdb" --wal "data.wdb.wal"`

## 4. Final Handover Metrics
- Deployment Status: SUCCESS
- Response Time: <50ms

```

### Step 5: Final Handover & Broadcast

1. Kirim laporan aktif layanan ke `agen_notulis` untuk pencatatan di Obsidian Vault.
2. Kirim URL akses (`http://localhost:<PORT>`) dan ringkasan metrik ke `agen_humas` untuk siaran publikasi.

---

## 3. Batasan & Rule Wajib (Negative Constraints)

* **Dilarang**: Menjalankan proses tanpa mencatat `PID` ke file `server.pid`.
* **Dilarang**: Menggunakan server runtime luar (Node/Python/Apache/Nginx) untuk proses serving inti.
* **Dilarang**: Menghentikan atau mengganggu server proyek lain yang sedang berjalan di port berbeda.
* **Dilarang**: Menyatakan deployment sukses sebelum pengujian `curl` memberikan kode status `200 OK`.

---

## 4. Definition of Done (DoD)

Tahap DevOps dinyatakan Selesai jika:

1. File `05_SOP_DEVOPS_DAN_DEPLOYMENT.md` terbit dengan data PID dan Port yang akurat.
2. File `data.wdb` dan `data.wdb.wal` terinisialisasi dan skema SQL berhasil di-import.
3. File `server.pid` dan `server.log` tersedia di direktori proyek.
4. Server terkonfirmasi merespons `HTTP 200 OK` pada pengujian live `curl`.

