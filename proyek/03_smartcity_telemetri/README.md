# 🏙️ Proyek Mandiri 03: SmartCity ISPU Telemetri & GIS Geospasial

Platform telemetri pemantauan kualitas udara standar **Permen LHK No. P.14/2020**, optimasi lampu lalu lintas **ATCS**, dan **Geofencing Haversine**.

---

## 📂 Struktur Direktori Mandiri

```
03_smartcity_telemetri/
├── README.md                           # Dokumentasi ringkas proyek
├── server.wya                          # Skrip peluncur server mandiri Widya (Port 8003)
├── docs/                               # 📑 Dokumen Persiapan Pelaksanaan Lengkap
│   ├── 01_PRD.md                       # Product Requirement Document (SOP-02)
│   ├── 02_ARSITEKTUR_DAN_ERD.md        # Arsitektur Telemetri & ERD (SOP-03)
│   ├── 03_SECURITY_DAN_KEPATUHAN_PDP.md# Spesifikasi Keamanan IoT (SOP-07)
│   ├── 04_RENCANA_PENGUJIAN_QA.md      # Test Plan Validasi ISPU (SOP-05)
│   └── 05_SOP_DEVOPS_DAN_DEPLOYMENT.md # Panduan Deployment & Monitoring (SOP-06)
├── web/                                # 🌐 Aplikasi Web Peta GIS & ISPU
│   ├── index.html
│   ├── style.css
│   └── app.js
└── database/                           # 🗄️ Basis Data WidyaBase Lokal
    ├── skema.sql
    └── smartcity.wdb
```

---

## 🚀 Cara Menjalankan Layanan Mandiri

```bash
cd /home/devops/widya-lang/proyek/03_smartcity_telemetri
../../target/release/widya server.wya
```
Buka di peramban: **`http://localhost:8003`**
