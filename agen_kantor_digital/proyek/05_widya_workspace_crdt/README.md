# 📄 Proyek Mandiri 05: Widya Workspace Collaborative CRDT

Engine pengolah kata dan dokumen ilmiah kolaboratif real-time berbasis **CRDT LWW (Last-Write-Wins)** bebas konflik, live **LaTeX/KaTeX** equation formatter, dan ekspor **ISO 32000-1 PDF**.

---

## 📂 Struktur Direktori Mandiri

```
05_widya_workspace_crdt/
├── README.md                           # Dokumentasi ringkas proyek
├── server.wya                          # Skrip peluncur server mandiri Widya (Port 8005)
├── docs/                               # 📑 Dokumen Persiapan Pelaksanaan Lengkap
│   ├── 01_PRD.md                       # Product Requirement Document (SOP-02)
│   ├── 02_ARSITEKTUR_DAN_ERD.md        # Arsitektur CRDT State Machine (SOP-03)
│   ├── 03_SECURITY_DAN_KEPATUHAN_PDP.md# Spesifikasi Keamanan E2EE (SOP-07)
│   ├── 04_RENCANA_PENGUJIAN_QA.md      # Test Plan Multi-Klien (SOP-05)
│   └── 05_SOP_DEVOPS_DAN_DEPLOYMENT.md # Panduan Deployment (SOP-06)
├── web/                                # 🌐 Aplikasi Web Workspace Editor
│   ├── index.html
│   ├── style.css
│   └── app.js
└── database/                           # 🗄️ Basis Data WidyaBase Lokal
    ├── skema.sql
    └── workspace.wdb
```

---

## 🚀 Cara Menjalankan Layanan Mandiri

```bash
cd /home/devops/widya-lang/proyek/05_widya_workspace_crdt
../../target/release/widya server.wya
```
Buka di peramban: **`http://localhost:8005`**
