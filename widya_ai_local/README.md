# Widya Local Self-Evolving AI Core (RSI, Internet & Enterprise Engine)

Proyek ini merupakan implementasi mesin **Recursive Self-Improvement (RSI)**, **Akses Internet Berdaulat**, **Ekspansi Skill Mandiri**, dan **Layanan Tingkat Industri (Enterprise Grade)** lokal yang berjalan langsung di atas perangkat keras:
- **CPU**: AMD Ryzen 7 8840HS (8 Cores, 16 Threads Zen 4, SIMD AVX-512)
- **GPU**: AMD Radeon 780M RDNA3
- **NPU**: AMD Ryzen AI XDNA
- **Kompiler**: `widya.exe 1.0.0-beta.1`
- **Data Uji Nyata**: `c:\widya-lang\peta_desa_indonesia_kemendagri.gpkg` (83.762 Desa/Kelurahan Kemendagri RI)
- **Kepatuhan Privasi**: UU PDP No. 27/2022 (Zero Local Leakage Guardrail)
- **Standar Industri**: OpenAI API v1 Compatible (`/v1`), Model Context Protocol (MCP), Prometheus Telemetry

---

## Struktur Direktori

```
c:\widya-lang\widya_ai_local\
├── server_widya_ai.wya              # Enterprise HTTP/REST, SSE & MCP Server (Port 8088)
├── konfigurasi_agen.json            # Profil hardware, batas keselamatan, jaringan & skill
├── data/
│   └── sampel_desa_kemendagri.json   # Ekstrak data geo spasial desa riil untuk beban uji
├── genom/
│   ├── v1_0_mesin_pencari.wya       # Genom baseline v1.0.0 (Linear Scan O(N))
│   ├── v1_1_mesin_teroptimasi.wya   # Genom evolusi v1.1.0 (Hash Table O(1))
│   └── v1_2_mesin_kuantisasi.wya    # Genom evolusi v1.2.0 (INT8 Quantization)
├── mcp/                             # Model Context Protocol (MCP) Integration
│   ├── mcp_tools_manifest.json      # Manifest skema alat standar JSON-RPC 2.0
│   └── mcp_server_widya.wya         # Handler protokol MCP (initialize, tools/call, resources)
├── skills/                          # Direktori skill bawaan & hasil sintesis mandiri
│   ├── katalog_skill.json           # Metadata persistensi seluruh skill aktif
│   ├── skill_pencari_web.wya        # Skill penelusuran web & fakta online
│   ├── skill_kalkulator_haversine.wya # Skill kalkulasi jarak geospasial lintang/bujur
│   ├── skill_kurs_finansial.wya     # Skill konversi valuta asing ke Rupiah
│   └── skill_cuaca_geospasial.wya   # Skill hasil sintesis mandiri pemantau cuaca
├── modul_evolusi/
│   ├── sandbox_tingkat_os.wya       # Hardened OS-Level Sandbox & Resource Governor
│   ├── vektor_rag.wya               # Embedded Vector Knowledge & Dense Retrieval RAG
│   ├── akses_internet.wya           # HTTP client aman, web search & sanitasi HTML
│   ├── manajer_skill.wya            # Registri & orkestrasi eksekusi skill dinamis
│   ├── sintesis_skill_mandiri.wya   # Generator skill otonom (Gap -> Research -> Code -> Box -> DID)
│   ├── penalaran_percakapan.wya     # NLU cerdas, routing intent & penalaran kontekstual
│   ├── refleksi_diagnostik.wya      # Engine analisis performa AST & bottleneck detection
│   ├── sintesis_kode_mandiri.wya    # Engine sintesis kode otomatis dalam Widya-Lang
│   ├── sandbox_keamanan.wya         # Isolated sandbox executor & negative safety gate
│   ├── promosi_atomik.wya           # Hot-patching & atomic genome switcher
│   ├── watchdog_rollback.wya        # Auto-rollback sub-10ms pada anomali runtime
│   └── silsilah_merkle_did.wya      # Merkle DAG & W3C DID audit provenance
├── cli_widya_ai.wya                 # Antarmuka interaktif CLI dengan dukungan internet & skills
├── simulasi_industri_widya_ai.wya   # Suite pengujian skala industri (10 Tahap Enterprise)
├── simulasi_evolusi_mandiri.wya     # Driver utama eksekusi siklus evolusi diri komprehensif (16 Tes)
├── tolak_ukur_kinerja.wya           # Benchmark throughput & latency CPU/GPU lokal
├── www/
│   └── studio_ai_lokal.html         # Studio dashboard visual AI lokal mandiri interaktif
└── README.md                        # Dokumentasi arsitektur dan panduan operasional
```

---

## Cara Menjalankan

### 1. Menjalankan Server Enterprise (Port 8088)
Layanan HTTP/REST dan MCP tingkat industri dapat diaktifkan dengan:
```powershell
.\target\release\widya.exe .\widya_ai_local\server_widya_ai.wya
```

**Endpoint Standar yang Didukung**:
* `POST /v1/chat/completions`: Format OpenAI Chat Completions untuk integrasi pihak ketiga.
* `GET  /v1/models`: Metadata model aktif, versi genom, dan telemetri hardware.
* `POST /v1/mcp`: Handler protokol Model Context Protocol (MCP JSON-RPC 2.0).
* `GET  /v1/skills`: Katalog skill aktif yang terdaftar di memori.
* `POST /v1/skills/learn`: Memicu akuisisi skill baru secara terprogram via REST.
* `GET  /v1/telemetry`: Metrik time-series format standar Prometheus.
* `GET  /v1/health`: Liveness & readiness probes.

### 2. Menjalankan Pengujian Suite Industri (10 Tahap)
```powershell
.\target\release\widya.exe .\widya_ai_local\simulasi_industri_widya_ai.wya
```

### 3. Menjalankan Antarmuka Interaktif CLI
```powershell
.\target\release\widya.exe .\widya_ai_local\cli_widya_ai.wya
```
Atau melalui launcher cepat:
```powershell
.\widya_ai_local\jalankan_cli.ps1
# Atau klik ganda berkas: widya_ai_local\jalankan_cli.bat
```

### 4. Menjalankan Simulasi 16 Siklus Evolusi Mandiri & Skills
```powershell
.\target\release\widya.exe .\widya_ai_local\simulasi_evolusi_mandiri.wya
```

### 5. Membuka Studio Visual Dashboard
Buka berkas [studio_ai_lokal.html](file:///c:/widya-lang/widya_ai_local/www/studio_ai_lokal.html) di peramban web untuk memantau:
* **Perbandingan Kode (Diff)**: Visualisasi evolusi genom.
* **Pohon Silsilah Merkle DAG**: Audit silsilah W3C DID.
* **Katalog Skill & Akses Internet**: Live web search tester & generator skill mandiri.
* **Tingkat Industri & MCP**: Penguji REST API OpenAI, penguji protokol MCP, dan metrik Prometheus.
