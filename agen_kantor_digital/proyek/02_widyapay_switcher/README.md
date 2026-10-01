# 💳 Proyek Mandiri 02: WidyaPay Ultra-Fast Payment Switcher

Engine switching transaksi perbankan dan pembayaran nasional berkecepatan **50.000+ TPS** dengan latensi **0.24 ms**.

---

## 📂 Struktur Direktori Mandiri

```
02_widyapay_switcher/
├── README.md                           # Dokumentasi ringkas proyek
├── server.wya                          # Skrip peluncur server mandiri Widya (Port 8002)
├── docs/                               # 📑 Dokumen Persiapan Pelaksanaan Lengkap
│   ├── 01_PRD.md                       # Product Requirement Document (SOP-02)
│   ├── 02_ARSITEKTUR_DAN_ERD.md        # Arsitektur Switching & ERD (SOP-03)
│   ├── 03_SECURITY_DAN_KEPATUHAN_PDP.md# Spesifikasi PCI-DSS & UU PDP (SOP-07)
│   ├── 04_RENCANA_PENGUJIAN_QA.md      # Stress Test 50k TPS Report (SOP-05)
│   └── 05_SOP_DEVOPS_DAN_DEPLOYMENT.md # Panduan Deployment HA Cluster (SOP-06)
├── web/                                # 🌐 Aplikasi Web Terminal Switcher
│   ├── index.html
│   ├── style.css
│   └── app.js
└── database/                           # 🗄️ Basis Data WidyaBase Lokal
    ├── skema.sql
    └── widyapay.wdb
```

---

## 🚀 Cara Menjalankan Layanan Mandiri

```bash
cd /home/devops/widya-lang/proyek/02_widyapay_switcher
../../target/release/widya server.wya
```
Buka di peramban: **`http://localhost:8002`**
