# 🏛️ Proyek Mandiri 06: WidyaOS Sovereign Cloud System

Proyek ini berdiri sendiri dan merupakan implementasi sistem operasi web kedaulatan digital (*Digital Sovereignty Web OS*) dengan microkernel architecture dan Post-Quantum Cryptography.

## 📁 Struktur Proyek Mandiri
```
06_widyaos_sovereign/
├── docs/
│   ├── 01_PRD.md                     <- Product Requirements Document
│   ├── 02_ARSITEKTUR_DAN_ERD.md      <- Desain Arsitektur & ERD
│   ├── 03_SECURITY_DAN_KEPATUHAN_PDP.md <- Keamanan PQC & ISO 27001
│   ├── 04_RENCANA_PENGUJIAN_QA.md    <- Skenario Pengujian Sandbox QA
│   └── 05_SOP_DEVOPS_DAN_DEPLOYMENT.md  <- Prosedur Operasional & Deployment
├── web/
│   ├── index.html                    <- Desktop & Window Manager
│   ├── style.css                     <- Glassmorphic OS UI Styling
│   └── app.js                        <- Kernel Simulator, Terminal REPL, Task Manager
├── database/
│   ├── skema.sql                     <- Skema SQL Audit & Modul
│   └── sovereign.wdb                 <- WidyaBase Engine Data
├── server.wya                        <- Skrip Server Widya-Lang
└── README.md                         <- Petunjuk Proyek Mandiri
```

## 🚀 Cara Menjalankan Aplikasi Web Mandiri
Gunakan runtime resmi **Widya-Lang**:
```bash
# Jalankan langsung server web pada port 8006
widya web dev web -p 8006
```
Buka peramban di: `http://localhost:8006`
