# PRJ-ARKAS-WEB-01: Aplikasi ARKAS Web Terintegrasi

Sistem Perencanaan & Tata Kelola Anggaran Sekolah Berdaulat berbasis **Widya-Lang Native Runtime** dan **WidyaBase v1.0.0**.

---

## 📌 Ringkasan Eksekutif
Aplikasi ARKAS Web Terintegrasi mengimplementasikan standar pengelolaan dana Bantuan Operasional Satuan Pendidikan (BOSP) sesuai **Permendikbudristek No. 63 Tahun 2023** dan sistem pengadaan barang/jasa daring **Permendikdasmen No. 15 Tahun 2026 (SIPLah)**. Sistem ini dilengkapi rantai audit kriptografis berstandar tinggi (*tamper-evident BKU ledger*) guna menjamin kepatuhan audit BPK, BPKP, dan Inspektorat Daerah.

- **Port Layanan**: `http://localhost:8012`
- **Database**: `database/data.wdb` (Slotted-Page 4KB, B+Tree, WAL ACID, 5 Tabel Aktif)
- **Status Operasional**: AKTIF & TERVERIFIKASI QA (100% Pass)
- **Tugas Terkait**: `[TSK-ARKAS-01]` Perancangan Modul Anggaran & Belanja Sekolah (BOS)

---

## 📂 Struktur Direktori Proyek (SOP-00)

```text
PRJ-ARKAS-WEB-01/
├── docs/                                   # 📄 Dokumen Persiapan Resmi (ISO 32000-1)
│   ├── 00_BUKU_DOKUMEN_LENGKAP.pdf         # Master Book PDF
│   ├── 01_PRD.md                           # Product Requirement Document
│   ├── 01_PRD.pdf
│   ├── 02_ARSITEKTUR_DAN_ERD.md            # Arsitektur C4 & ERD WidyaBase
│   ├── 02_ARSITEKTUR_DAN_ERD.pdf
│   ├── 03_SECURITY_DAN_KEPATUHAN_PDP.md    # Standar Keamanan & UU PDP No. 27/2022
│   ├── 03_SECURITY_DAN_KEPATUHAN_PDP.pdf
│   ├── 04_RENCANA_PENGUJIAN_QA.md          # Matriks Pengujian & Uji Fuzzing
│   ├── 04_RENCANA_PENGUJIAN_QA.pdf
│   ├── 05_SOP_DEVOPS_DAN_DEPLOYMENT.md     # Prosedur Deployment & Healthcheck
│   └── 05_SOP_DEVOPS_DAN_DEPLOYMENT.pdf
├── database/                               # 🗄️ Basis Data Terisolasi WidyaBase
│   ├── skema.sql                           # DDL Diterapkan & Seed Data
│   └── data.wdb                            # Slotted-Page B+Tree Storage
├── web/                                    # 🌐 Dashboard Web Interaktif
│   ├── index.html                          # User Interface Dashboard
│   ├── style.css                           # Visual Design System
│   └── app.js                              # Frontend State & Kalkulator Pajak
├── laporan/                                # 📊 Laporan Progres Resmi
│   └── LAPORAN_PROGRES_ARKAS.md
├── server.wya                              # 🚀 Server Launcher Mandiri
└── README.md                               # 📖 Dokumentasi Proyek
```

---

## 🚀 Cara Menjalankan

### 1. Menjalankan Server Web (Port 8012)
```bash
cd /home/devops/widya-lang/agen_kantor_digital/proyek/PRJ-ARKAS-WEB-01
../../target/release/widya server.wya
```

Atau menggunakan mode daemon background:
```bash
nohup ../../target/release/widya web dev ./web -p 8012 > server.log 2>&1 &
echo $! > server.pid
```

### 2. Memeriksa Status Basis Data
```bash
../../target/release/widya base periksa database/data.wdb
```

### 3. Eksekusi Unit Test Rust
```bash
cargo test --test arkas_bos_budget_test
cargo test --test arkas_multi_sumber_dana_test
```
