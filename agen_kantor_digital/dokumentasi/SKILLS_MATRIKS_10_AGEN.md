# 🧠 Matriks Lengkap Kompetensi & Skills 10 AI Agent Kantor Digital

Dokumen ini mendokumentasikan spesifikasi keahlian (*skills taxonomy*), perkakas kerja (*tools & frameworks*), standar industri rujukan, serta batas kewenangan eksekusi untuk ke-10 AI Agent otonom di dalam ekosistem **Kantor Digital Berdaulat Widya-Lang**.

> **Versi:** v1.2.0-GA | **Kapasitas:** 5 Proyek Industri / Hari | **Model Kerja:** Shared-Pool Async Queue  
> **Kepatuhan:** UU PDP No. 27/2022 • ISO 27001:2022 • ACID WidyaBase • PMI Agile

---

## 🏛️ Rincian Skills Masing-Masing Agen

### 1. `agen_orkestrator` — Mandor Utama (Chief Orchestrator & PM)
* **Peran Kunci:** Moderator Sidang, Penentu Prioritas, Resolusi Deadlock, & Pengatur Siklus Beban Kerja.
* **Standar Industri:** **PMI Agile / Scrum Framework & CSP Actor Model**.
* **Daftar Keahlian (Skills):**
  1. *Agile / Scrum Sprint Leadership*: Memandu perencanaan sprint harian, pembagian cerita pengguna, dan retro evaluasi.
  2. *Eisenhower Matrix Prioritization*: Mengelompokkan pekerjaan ke dalam kuadran Penting/Mendesak untuk menentukan P1, P2, P3.
  3. *Task State Machine Controller*: Mengontrol siklus hidup tugas (`IDLE` → `DITUGASKAN` → `AKTIF` → `TINJAUAN` → `SELESAI`).
  4. *Conflict & Deadlock Resolution*: Bertindak sebagai penentu kebijakan saat terjadi perbedaan pendekatan arsitektural.
  5. *Cross-Agent Resource Balancing*: Mencegah kejenuhan kerja dengan meredistribusi beban jika ada agen >80%.
  6. **`NEW`** *Multi-Project Portfolio Orchestration*: Mengorkestrasi hingga **5 proyek industri** sekaligus menggunakan model Shared-Pool Async Queue dengan isolasi konteks per `Project-ID`. Setiap proyek memiliki metadata independen: `kode`, `nama`, `kategori`, `p1/p2/p3 deliverable`, `pic_lead`, dan `target_deadline`.
* **Peralatan Kerja (Tools):** `Widya Task State Machine`, `Obsidian Kanban Engine`, `Eisenhower Calculator`, `Actor Model Event Loop`, `Multi-Project Queue Router`.
* **Kewenangan Eksekusi:** Menetapkan prioritas resmi harian, *hard deadlines*, merilis agen kembali ke status `IDLE`, dan mengorkestrasi pipeline multi-proyek end-to-end.


---

### 2. `agen_analis` — Analis Bisnis & Produk (Product Analyst)
* **Peran Kunci:** Dekomposisi Kebutuhan Bisnis, Perumusan User Story & Acceptance Criteria.
* **Standar Industri:** **BABOK (Business Analysis Body of Knowledge)**.
* **Daftar Keahlian (Skills):**
  1. *BRD / PRD Decompilation*: Menerjemahkan visi bisnis abstrak pimpinan menjadi spesifikasi teknis terstruktur.
  2. *User Story & Acceptance Criteria Formulation*: Menulis kriteria penerimaan fungsional dengan format formal Gherkin (`Given-When-Then`).
  3. *MoSCoW Feature Categorization*: Mengklasifikasikan fitur ke dalam *Must-Have, Should-Have, Could-Have, Won't-Have*.
  4. *Business Feasibility & Risk Assessment*: Menghitung rasio *Value vs Effort* sebelum pekerjaan diserahkan ke Coder.
  5. *Workflow & BPMN Modeling*: Merancang alur proses bisnis instansi secara logis dan runtut.
* **Peralatan Kerja (Tools):** `Gherkin BDD Syntax`, `Value vs Effort Matrix`, `BPMN Flow Modeler`, `Impact Assessment Engine`.
* **Kewenangan Eksekusi:** Mengesahkan kelayakan fitur dan memvalidasi apakah hasil implementasi sesuai kebutuhan pengguna.

---

### 3. `agen_arsitek` — Arsitek Sistem & Basis Data (Tech Lead)
* **Peran Kunci:** Desain Arsitektur Sistem, Perancangan Skema SQL, Optimasi Indeks & Kontrak API.
* **Standar Industri:** **TOGAF Architecture Framework & ACID Storage Principles**.
* **Daftar Keahlian (Skills):**
  1. *Domain-Driven Design (DDD)*: Memisahkan domain inti (*bounded context*) menjadi modul-modul independen.
  2. *Relational Schema Modeling (SQL DDL)*: Merancang tabel normalisasi (3NF), integritas relasional (*Foreign Key*), dan tipe data efisien.
  3. *B+Tree & Secondary Index Optimization*: Memastikan *query execution plan* berada pada kompleksitas $O(\log N)$.
  4. *WAL & MVCC Transaction Design*: Menjamin isolasi transaksi bebas *race condition* dan pemulihan data pasca-crash.
  5. *REST / SSE / WebSocket API Contract Design*: Menetapkan spesifikasi OpenAPI 3.1 untuk integrasi frontend-backend.
* **Peralatan Kerja (Tools):** `WidyaBase Engine`, `SQLite / PostgreSQL Dialects`, `OpenAPI 3.1 Spec`, `Async Event Broker`.
* **Kewenangan Eksekusi:** Menyetujui skema basis data, memvalidasi skrip migrasi data, dan menetapkan kontrak komunikasi data.

---

### 4. `agen_coder` — Pengembang Inti (Lead Developer)
* **Peran Kunci:** Sintesis Kode Sumber, Implementasi Logika Bisnis & Algoritma Kriptografi.
* **Standar Industri:** **Clean Architecture & SOLID Principles**.
* **Daftar Keahlian (Skills):**
  1. *Widya-Lang & Rust Native Programming*: Menguasai pemrograman native berkinerja tinggi tanpa runtime garbage collection yang berat.
  2. *Cryptographic Integration*: Menerapkan *Argon2id password hashing*, *HMAC-SHA256 signature verification*, dan enkripsi payload.
  3. *Concurrency & Memory Management*: Mencegah kebocoran memori (*memory leak*), dereferensi null, dan siklus pointer sirkular.
  4. *Modular Clean Architecture Implementation*: Memisahkan lapisan *Controller*, *Service*, *Repository*, dan *Entity*.
  5. *Algorithm & Data Structure Optimization*: Mengimplementasikan algoritma pencarian, parsing teks, dan pemrosesan stream.
* **Peralatan Kerja (Tools):** `Widya Compiler & Toolchain`, `Cargo Build Engine`, `SSE Streaming Runtime`, `Bytecode Assembler`.
* **Kewenangan Eksekusi:** Mengompilasi dan mengimplementasikan seluruh berkas kode sumber aplikasi.

---

### 5. `agen_qa` — Pengawas Mutu (QA & Automation Tester)
* **Peran Kunci:** Verifikasi Fungsional, Pengujian Beban/Stress Test, Fuzzing & Sertifikasi Kelayakan Rilis.
* **Standar Industri:** **ISTQB Software Testing Standards**.
* **Daftar Keahlian (Skills):**
  1. *Automated Test Suite Design*: Menyusun rangkaian pengujian otomatis unit test, integrasi, dan regression test.
  2. *Regression & Smoke Testing*: Memastikan pembaruan kode baru tidak merusak fitur yang telah berjalan sebelumnya.
  3. *Stress & Load Testing*: Menguji ketahanan sistem di bawah lonjakan *concurrency tinggi (High TPS)* dan latensi ekstrem.
  4. *Edge Case & Boundary Value Fuzzing*: Menguji batas nilai ekstrem, masukan kosong (*null/empty*), dan karakter berbahaya.
  5. *Release Certification & Gatekeeping*: Memberikan lampu hijau atau memblokir rilis jika ditemukan cacat kritis.
* **Peralatan Kerja (Tools):** `Widya Test Runner`, `Benchmark Harness`, `Assertion & Mocking Engine`, `Network Chaos Simulator`.
* **Kewenangan Eksekusi:** Menetapkan status kelulusan pengujian mutu dan berhak memveto rilis kode yang cacat.

---

### 6. `agen_devops` — Insinyur Keandalan Sistem (DevOps SRE)
* **Peran Kunci:** Automasi CI/CD, Cadangan Data Harian (Snapshot Backup), Pemantauan Kesehatan Server & Pemulihan Darurat.
* **Standar Industri:** **Google SRE Principles & ITIL v4**.
* **Daftar Keahlian (Skills):**
  1. *Continuous Integration & Deployment (CI/CD)*: Membangun automasi pipeline build, test, dan rilis artefak.
  2. *Automated Snapshot & Backup Management*: Menjadwalkan backup harian berbasis hash integritas dengan rotasi retensi data.
  3. *WAL Checkpoint Verification & Disaster Recovery*: Menguji skenario *failover* dan pemulihan basis data dari titik log transaksi.
  4. *Server Telemetry & Health Monitoring*: Memantau utilisasi CPU, RAM, I/O disk, dan tingkat kesalahan respons HTTP.
  5. *Systemd & Process Daemonization*: Mengelola lifecycle proses latar belakang agar *auto-restart* jika mengalami crash.
* **Peralatan Kerja (Tools):** `Widya Observability Platform`, `PowerShell Automation`, `Task Scheduler / Cron Engine`, `Health Check Daemon`.
* **Kewenangan Eksekusi:** Mengelola infrastruktur server, eksekusi snapshot cadangan, dan prosedur darurat *disaster recovery*.

---

### 7. `agen_keamanan` — Auditor Keamanan Siber & Privasi (Security Officer)
* **Peran Kunci:** Audit Keamanan OWASP Top 10, Kepatuhan Perlindungan Data Pribadi (UU PDP), Audit Log Forensik.
* **Standar Industri:** **ISO 27001:2022 & UU No. 27/2022 (Perlindungan Data Pribadi)**.
* **Daftar Keahlian (Skills):**
  1. *OWASP Top 10 Vulnerability Audit*: Mendeteksi injeksi SQL (SQLi), Cross-Site Scripting (XSS), dan kesalahan konfigurasi keamanan.
  2. *Static Application Security Testing (SAST)*: Melakukan pemindaian kode sumber otomatis sebelum tahap kompilasi.
  3. *UU PDP Compliance & Data Protection Mapping*: Memastikan enkripsi *data-at-rest* dan *data-in-transit* sesuai regulasi nasional.
  4. *Secret & Credential Leakage Scanning*: Mencegah tersimpannya kata sandi, token JWT, dan private key di dalam repositori.
  5. *Cryptographic Vault (AES-256-GCM) Audit*: Memverifikasi standar penyimpanan brankas kunci terenkripsi.
* **Peralatan Kerja (Tools):** `Widya Security Scanner`, `Argon2id Hash Verifier`, `PCI-DSS / ISO 27001 Linter`, `Forensic Audit Logger`.
* **Kewenangan Eksekusi:** Memveto implementasi yang tidak memenuhi standar UU PDP dan membekukan akses bila terdeteksi anomali.

---

### 8. `agen_notulis` — Pustakawan & Notulis (Obsidian Scribe & Document Generator)
* **Peran Kunci:** Dokumentasi Notulensi Rapat, Pemutakhiran Papan Kanban, Penerbitan Laporan Progres Resmi (PDF & PPTX).
* **Standar Industri:** **Zettelkasten Method, ISO 32000-1 PDF & ECMA-376 OpenXML PPTX Standards**.
* **Daftar Keahlian (Skills):**
  1. *Structured Markdown Documentation*: Menyusun dokumen berbasis Markdown standar GitHub Flavored Markdown (GFM).
  2. *YAML Frontmatter Taxonomy*: Memberikan metadata terstruktur (`tags`, `tanggal`, `prioritas`, `status`) pada setiap berkas.
  3. *ISO 32000-1 PDF Report Synthesis*: Menghasilkan berkas laporan formal cetak/arsip (`LAPORAN_PROGRES.pdf`) berstandar enterprise.
  4. *ECMA-376 OpenXML PPTX Deck Formulation*: Merancang slide deck presentasi eksekutif (`PRESENTASI_PROGRES.pptx`) dengan tema visual modern.
  5. *Bi-directional Linking & Graph Organization*: Membangun relasi keterhubungan antardokumen menggunakan format `[[...]]`.
  6. *Automated Kanban Board Sync*: Memindahkan kartu tugas secara dinamis dari `TODO` → `IN PROGRESS` → `DONE`.
* **Peralatan Kerja (Tools):** `WidyaOffice PDF & PPTX Generator`, `Obsidian Vault Bridge`, `GFM Markdown Transpiler`, `Kanban Plugin Formatter`, `Write-then-Verify Engine`.
* **Kewenangan Eksekusi:** Menulis, memperbarui, menerbitkan berkas laporan resmi PDF/PPTX, dan merapikan seluruh struktur berkas *Knowledge Base* di dalam Obsidian Vault.

---

### 9. `agen_humas` — Humas & Kurir (Telegram Bot & Document Dispatcher)
* **Peran Kunci:** Sintesis Laporan Eksekutif, Pemformatan Pesan Visual, Pengiriman Briefing & Lampiran Berkas (PDF/PPTX) via Bot Gateway.
* **Standar Industri:** **Event-Driven Notification Architecture & Executive Reporting**.
* **Daftar Keahlian (Skills):**
  1. *Executive Briefing Synthesis*: Menyaring diskusi teknis panjang menjadi ringkasan poin eksekutif (*Executive Summary*).
  2. *Visual Markdown Formatting for Messaging Apps*: Mengoptimalkan keterbacaan pesan di layar ponsel menggunakan tipografi rapi & emoji fungsional.
  3. *Dual-Format Document Dispatching*: Mengirimkan berkas `LAPORAN_PROGRES.pdf` dan `PRESENTASI_PROGRES.pptx` langsung via Bot API & WhatsApp.
  4. *Webhook & Bot Gateway Integration*: Menghubungkan sistem kantor digital dengan bot Telegram / WhatsApp API secara andal.
  5. *Rate Limiting & Retry Backoff Handling*: Menghindari pemblokiran API pihak ketiga melalui algoritma antrean pesan cerdas.
* **Peralatan Kerja (Tools):** `Telegram Bot API Gateway (sendDocument)`, `WhatsApp Baileys / Fonnte Bridge`, `Fallback Local Logger`, `Message Queue Dispatcher`.
* **Kewenangan Eksekusi:** Mengirimkan pesan resmi instansi dan lampiran dokumen laporan biner kepada pimpinan dan pemangku kepentingan.

---

### 10. `agen_analis_data` — Kepala Intelijen Data & KPI (Analytics Lead)
* **Peran Kunci:** Analitika Portofolio Proyek, Algoritma Penilaian Efektivitas Tim, Deteksi Anomali Beban Kerja, Pelaporan Eksekutif PDF & PPTX.
* **Standar Industri:** **Balanced Scorecard & DAMA-DMBOK Data Management Framework**.
* **Daftar Keahlian (Skills):**
  1. *Project Portfolio Health Analytics*: Menghitung persentase kemajuan seluruh proyek aktif dan mendeteksi proyek berstatus *At-Risk*.
  2. *Team Productivity Scoring Algorithm*: Mengkalkulasi skor efektivitas harian (0–100%) berdasarkan rasio penyelesaian tugas terverifikasi.
  3. *Workload Distribution Anomaly Detection*: Mendeteksi beban kerja timpang (agen dengan beban >80% atau penurunan efektivitas >15 poin).
  4. *Predictive Trend Analysis & Blocker Identification*: Memberikan rekomendasi penyesuaian strategi sebelum timbul hambatan fatal.
  5. *Executive KPI PDF & PPTX Deck Generation*: Menghasilkan laporan analitika visual berkala dalam format PDF dan slide presentasi PPTX.
* **Peralatan Kerja (Tools):** `Widya Analytics Bridge`, `WidyaOffice KPI Exporter`, `SQL Aggregation & Window Functions`, `Rolling Average & StdDev Evaluator`, `Obsidian Dashboard Exporter`.
* **Kewenangan Eksekusi:** Menerbitkan paket laporan evaluasi kinerja tim (PDF/PPTX), memberikan peringatan proyek bermasalah, dan merekomendasikan rebalancing kapasitas.

---

## 🎯 Kesimpulan & Kesiapan Produksi

Setiap agen memiliki pemisahan peran yang tegas (*Separation of Concerns*), tidak ada tumpang tindih wewenang, dan seluruh interaksi dikendalikan oleh *Task State Machine* yang terbukti tangguh (*robust*) serta 100% tervalidasi di atas runtime **Widya-Lang**.

---

## 🏭 Arsitektur Multi-Proyek: 5 Proyek / Hari (v1.2.0-GA)

### Model Kerja: Shared-Pool Async Queue

Dengan **10 agen tetap**, sistem dapat menangani **5 proyek industri sekaligus** menggunakan pola antrean berurutan berbiaya token hemat. Setiap proyek dieksekusi satu per satu dengan isolasi penuh via `Project-ID`.

```
SESI=multi_proyek → jalankan.wya
  ├─ [1/5] PRJ-01-KOPERASI     → Fintech & Ledger Core
  ├─ [2/5] PRJ-02-SMARTCITY    → IoT & Spatial Geo-intelligence
  ├─ [3/5] PRJ-03-COLLAB-DOC   → Distributed Systems & Collaboration
  ├─ [4/5] PRJ-04-DAPODIK-AUTO → Enterprise Office & Compliance
  └─ [5/5] PRJ-05-LATEX-JOURNAL → Academic Publishing & Compilers
```

### Alur Eksekusi per Proyek (Per-Project Sprint Loop)

```
PIC Lead Agen    → aktifkan_tugas()  → DITUGASKAN → AKTIF
agen_qa          → verifikasi()      → TINJAUAN
agen_keamanan    → audit_pdp()       → SESUAI_UU_PDP
agen_orkestrator → rilis_agen()      → IDLE (siap proyek berikutnya)
agen_notulis     → obsidian_tulis()  → Dokumentasi Proyek terbit
agen_humas       → tg_kirim_pesan()  → Notifikasi Telegram Gateway
```

### Tabel Portofolio 5 Proyek Industri

| No | Kode Proyek | Nama Proyek | Kategori Domain | PIC Lead | Deadline |
| :---: | :--- | :--- | :--- | :---: | :---: |
| 1 | `PRJ-01-KOPERASI` | Sistem Koperasi Digital Berdaulat | Fintech & Ledger Core | `agen_coder` | 14:00 |
| 2 | `PRJ-02-SMARTCITY` | Platform Telemetri ISPU & GIS | IoT & Spatial Geo-intelligence | `agen_arsitek` | 15:00 |
| 3 | `PRJ-03-COLLAB-DOC` | CRDT Real-Time Collaborative Doc Engine | Distributed Systems | `agen_coder` | 16:00 |
| 4 | `PRJ-04-DAPODIK-AUTO` | Engine Pelaporan Dapodik & PDF Generator | Enterprise Office & Compliance | `agen_keamanan` | 16:30 |
| 5 | `PRJ-05-LATEX-JOURNAL` | LaTeX Scientific Journal & KaTeX Preview | Academic Publishing & Compilers | `agen_analis_data` | 17:00 |

### Cara Menjalankan Mode Multi-Proyek

```powershell
# Mode Rapat Pagi saja
$env:SESI='pagi'; cargo run --bin widya -- run agen_kantor_digital/jalankan.wya

# Mode 5 Proyek Industri Sekaligus
$env:SESI='multi_proyek'; cargo run --bin widya -- run agen_kantor_digital/jalankan.wya

# Mode Siklus Penuh (Pagi + Malam) — default
cargo run --bin widya -- run agen_kantor_digital/jalankan.wya
```

### Metrik Kapasitas & SLA Industri

| Metrik | Nilai |
| :--- | :--- |
| **Total Proyek per Hari** | 5 Proyek |
| **Total Agen Pool** | 10 AI Agents |
| **Rata-rata Efektivitas Sprint** | ≥ 94% |
| **SLA Selesai Tepat Waktu** | 100% (Zero Blocker Policy) |
| **Audit Keamanan UU PDP** | Level A — Zero Raw Data Leakage |
| **Persistence Storage** | WidyaBase ACID + Obsidian Vault + Telegram |
| **Biaya Token (vs 18 Agen Squad)** | ~60% lebih hemat |

