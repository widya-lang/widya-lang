# 🩺 Proyek Mandiri 04: WidyaMedika SIMRS & SatuSehat HL7 FHIR

Sistem Informasi Manajemen Rumah Sakit (SIMRS) dan Rekam Medis Elektronik terstandar **HL7 FHIR v4.0.1** Kemenkes RI dan bridging **BPJS V-Claim**.

---

## 📂 Struktur Direktori Mandiri

```
04_widyamedika_simrs/
├── README.md                           # Dokumentasi ringkas proyek
├── server.wya                          # Skrip peluncur server mandiri Widya (Port 8004)
├── docs/                               # 📑 Dokumen Persiapan Pelaksanaan Lengkap
│   ├── 01_PRD.md                       # Product Requirement Document (SOP-02)
│   ├── 02_ARSITEKTUR_DAN_ERD.md        # Arsitektur SIMRS & FHIR ERD (SOP-03)
│   ├── 03_SECURITY_DAN_KEPATUHAN_PDP.md# Proteksi Data Medis & UU PDP (SOP-07)
│   ├── 04_RENCANA_PENGUJIAN_QA.md      # Test Plan Bridging SatuSehat (SOP-05)
│   └── 05_SOP_DEVOPS_DAN_DEPLOYMENT.md # Panduan Deployment Server (SOP-06)
├── web/                                # 🌐 Aplikasi Web RME Pasien
│   ├── index.html
│   ├── style.css
│   └── app.js
└── database/                           # 🗄️ Basis Data WidyaBase Lokal
    ├── skema.sql
    └── widyamedika.wdb
```

---

## 🚀 Cara Menjalankan Layanan Mandiri

```bash
cd /home/devops/widya-lang/proyek/04_widyamedika_simrs
../../target/release/widya server.wya
```
Buka di peramban: **`http://localhost:8004`**
