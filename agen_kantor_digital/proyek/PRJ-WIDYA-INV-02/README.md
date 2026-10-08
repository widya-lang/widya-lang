# PRJ-WIDYA-INV-02: Manajemen Inventaris UMKM Nusantara

Sistem Manajemen Inventaris & Pergudangan Berdaulat berbasis **Widya-Lang Native Runtime** dan **WidyaBase v1.0.0**.

---

## 📌 Ringkasan Eksekutif
Proyek ini mengintegrasikan standar penomoran global **GS1 (EAN-13 & GS1-128)** dan teknologi identifikasi frekuensi radio **EPC Global Gen2 UHF (SGTIN-96)** ke dalam ekosistem logistik mandiri UMKM Nusantara.

- **Port Layanan**: `http://localhost:8011`
- **Database**: `database/data.wdb` (Slotted-Page 4KB, B+Tree, WAL ACID)
- **Status Operasional**: AKTIF & TERVERIFIKASI QA (100% Pass)
- **Tugas Terkait**: `[TSK-006]` Perancangan Skema Barcode & RFID Gudang

---

## 📂 Struktur Direktori Proyek (SOP-00)

```text
PRJ-WIDYA-INV-02/
├── docs/                                   # 📄 Dokumen Persiapan Resmi (ISO 32000-1)
│   ├── 01_PRD.md                           # Product Requirement Document
│   ├── 02_ARSITEKTUR_DAN_ERD.md            # Arsitektur C4 & ERD WidyaBase
│   ├── 03_SECURITY_DAN_KEPATUHAN_PDP.md    # Standar Keamanan & UU PDP No. 27/2022
│   ├── 04_RENCANA_PENGUJIAN_QA.md          # Matriks Pengujian & Uji Fuzzing
│   └── 05_SOP_DEVOPS_DAN_DEPLOYMENT.md     # Prosedur Deployment & Healthcheck
├── database/                               # 🗄️ Basis Data Terisolasi WidyaBase
│   ├── skema.sql                           # DDL Diterapkan & Seed Data
│   ├── data.wdb                            # Slotted-Page B+Tree Storage
│   └── data.wdb.wal                        # Write-Ahead Log
├── web/                                    # 🌐 Dashboard Web Interaktif
│   ├── index.html                          # User Interface Dashboard
│   ├── style.css                           # Visual Design System
│   └── app.js                              # Frontend State & Simulasi RFID/Barcode
├── laporan/                                # 📊 Laporan Progres Resmi (PDF/PPTX)
├── server.wya                              # 🚀 Server Launcher Mandiri
└── README.md                               # 📖 Dokumentasi Proyek
```

---

## 🚀 Cara Menjalankan

### 1. Menjalankan Server Web (Port 8011)
```bash
cd /home/devops/widya-lang/agen_kantor_digital/proyek/PRJ-WIDYA-INV-02
../../target/release/widya server.wya
```

Atau menggunakan mode daemon background:
```bash
nohup ../../target/release/widya web dev ./web -p 8011 > server.log 2>&1 &
echo $! > server.pid
```

### 2. Memeriksa Status Basis Data
```bash
../../target/release/widya base periksa database/data.wdb
```

### 3. Eksekusi Unit Test Rust
```bash
cargo test --test inventory_test
```
