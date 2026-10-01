# 🌾 Proyek Mandiri 01: Sistem Koperasi Desa Merah Putih (KDMP 7 Pilar)

Sistem ERP Koperasi Desa Berdaulat Terpadu berbasis **Widya-Lang** dan **WidyaBase**, siap diterapkan di 75.000+ Desa di seluruh Indonesia.

---

## 📂 Struktur Direktori Mandiri

```
01_koperasi_kdmp/
├── README.md                           # Dokumentasi ringkas proyek
├── server.wya                          # Skrip peluncur server mandiri Widya (Port 8001)
├── docs/                               # 📑 Dokumen Persiapan Pelaksanaan Lengkap
│   ├── 01_PRD.md                       # Product Requirement Document (SOP-02)
│   ├── 02_ARSITEKTUR_DAN_ERD.md        # Arsitektur Komponen & Skema ERD (SOP-03)
│   ├── 03_SECURITY_DAN_KEPATUHAN_PDP.md# Spesifikasi Keamanan & UU PDP (SOP-07)
│   ├── 04_RENCANA_PENGUJIAN_QA.md      # Test Plan & 124 Test Cases (SOP-05)
│   └── 05_SOP_DEVOPS_DAN_DEPLOYMENT.md # Panduan Deployment & Backup WAL (SOP-06)
├── web/                                # 🌐 Aplikasi Web Mandiri
│   ├── index.html
│   ├── style.css
│   └── app.js
└── database/                           # 🗄️ Basis Data WidyaBase Lokal
    ├── skema.sql
    └── koperasi.wdb
```

---

## 🚀 Cara Menjalankan Layanan Mandiri

```bash
cd /home/devops/widya-lang/proyek/01_koperasi_kdmp
../../target/release/widya server.wya
```
Buka di peramban: **`http://localhost:8001`**
