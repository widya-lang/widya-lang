# 📖 The Widya Book: Panduan Definitif Bahasa Pemrograman Berdaulat

> **Widya-Lang v0.1.0 (Enterprise & Sovereign Edition)**  
> *Bahasa Pemrograman Generasi Baru Berkeamanan Memori Formal, Berkemandirian Penuh Tanpa Ketergantungan Eksternal, Self-Hosting 100% Determinisik, dan Pustaka Standar Paritas Industri.*

---

## Daftar Isi
1. [Filosofi & Visi Kedaulatan Digital](#1-filosofi--visi-kedaulatan-digital)
2. [Model Keamanan Memori: MIR Borrow Checker Tanpa GC](#2-model-keamanan-memori-mir-borrow-checker-tanpa-gc)
3. [Arsitektur Konkurensi: Actor Supervision Tree & Chaos Simulator](#3-arsitektur-konkurensi-actor-supervision-tree--chaos-simulator)
4. [Cloud-Native & Edge: WASI 0.2 Preview 2 & Microsecond Cold-Start](#4-cloud-native--edge-wasi-02-preview-2--microsecond-cold-start)
5. [Kriptografi Pertahanan & Hardware Enclave: PKCS#11 & Post-Quantum TLS 1.3](#5-kriptografi-pertahanan--hardware-enclave-pkcs11--post-quantum-tls-13)
6. [Native AI Sovereign Engine & Model Context Protocol (MCP)](#6-native-ai-sovereign-engine--model-context-protocol-mcp)
7. [Enterprise Observability & WidyaBase Connection Pooling](#7-enterprise-observability--widyabase-connection-pooling)
8. [Arsitektur Kompiler Mandiri (Self-Hosting Compiler Stage-0 → Stage-1 → Stage-2)](#8-arsitektur-kompiler-mandiri-self-hosting-compiler)
9. [Ekosistem Pustaka Standar Resmi WPM (@widya/*)](#9-ekosistem-pustaka-standar-resmi-wpm-widya)
10. [Pelari Uji Terintegrasi (Native Test Runner: widya test)](#10-pelari-uji-terintegrasi-native-test-runner-widya-test)
11. [Perintah Antarmuka Terminal (CLI Cheatsheet)](#11-perintah-antarmuka-terminal-cli-cheatsheet)

---

## 1. Filosofi & Visi Kedaulatan Digital
Widya-Lang lahir untuk menjawab tantangan keamanan nasional dan ketahanan infrastruktur komputasi modern. Sebagian besar bahasa kompilasi bergantung pada rantai pasok compiler raksasa (GCC, LLVM, Clang) dan ribuan *crates/packages* pihak ketiga yang rentan terhadap serangan *supply-chain injection*.

Widya-Lang mengusung prinsip **Sovereign Stack (Zero External Dependencies)**:
- **Kompiler Mandiri (Self-Hosting):** Kode sumber ditulis dalam Widya murni (`.wya` / `.widya`) dan mampu mengompilasi dirinya sendiri secara deterministik bit-for-bit dengan penandatanganan kurva eliptis Ed25519.
- **Format Pertukaran Mandiri:** Parser JSON, CSV, SafeTensors, dan GGUF murni tanpa memanggil library C/C++.
- **Bare-Metal & Unikernel Native:** Mendukung boot langsung pada hypervisor modern (AWS Firecracker, QEMU, Cloud-Hypervisor) dengan *boot protocol direct-kernel* dan *multiboot2*.

---

## 2. Model Keamanan Memori: MIR Borrow Checker Tanpa GC
Widya-Lang meniadakan *Garbage Collector* (GC) yang menimbulkan *stop-the-world latency*, sekaligus melarang alokasi memori manual yang rawan *use-after-free* dan *buffer overflow*.

### Arsitektur MIR (Mid-level Intermediate Representation)
1. **Representasi CFG (Control Flow Graph):** Setiap fungsi diterjemahkan ke blok dasar (*Basic Blocks*) yang memiliki instruksi penugasan eksplisit (`Assign`), pembacaan operand (`Use`, `Move`, `Copy`), dan terminasi aliran data (`Goto`, `SwitchInt`, `Return`).
2. **Non-Lexical Lifetimes (NLL):** Validitas peminjaman (*borrow*) dievaluasi berdasarkan titik penggunaan terakhir (liveness analysis), bukan batasan kurung kurawal sintaksis leksikal.
3. **Miri Undefined Behavior Inspector:** Simulator interpretatif tingkat MIR yang memeriksa pelanggaran *pointer alignment*, *out-of-bounds array access*, dan *stacked borrows aliasing*.

---

## 3. Arsitektur Konkurensi: Actor Supervision Tree & Chaos Simulator
Mengadopsi keandalan telekomunikasi Erlang/OTP:
- **Isolasi Kegagalan:** Setiap aktor memiliki antrean pesan (*Mailbox*) dan batas *panic* (`std::panic::catch_unwind`). Kegagalan pada satu aktor tidak akan meruntuhkan proses utama.
- **Strategi Pengawasan (Supervision):**
  - `OneForOne`: Hanya merestart aktor anak yang mengalami crash.
  - `OneForAll`: Merestart seluruh aktor yang berada di bawah supervisor yang sama.
  - `RestForOne`: Merestart aktor yang gagal beserta semua aktor yang didaftarkan setelahnya.
- **Jepsen Chaos Simulator:** Modul simulasi partisi jaringan deterministik virtual (`ChaosNetwork`) yang mampu mensimulasikan *split-brain*, tingkat *packet drop*, dan *latency jitter*.

---

## 4. Cloud-Native & Edge: WASI 0.2 Preview 2 & Microsecond Cold-Start
Untuk ekosistem edge serverless dan microVM:
- **WASI 0.2 Component Model:** Antarmuka standar `wasi:cli/run@0.2.0`, `wasi:http/incoming-handler@0.2.0`, `wasi:filesystem/preopens@0.2.0`, dan `wasi:sockets/tcp@0.2.0`.
- **Canonical ABI Memory Bridge:** Prosedur `lift_string` dan `lower_string` yang mentransfer string UTF-8 dan struktur data komposit melintasi batas memori linear WebAssembly tanpa alokasi ganda.
- **Edge Snapshot Engine:** Mengambil snapshot biner dari status memori dan register aktif VM, memungkinkan restorasi instan (*cold-start*) dalam kurun waktu **< 5 mikrodetik**.

---

## 5. Kriptografi Pertahanan & Hardware Enclave: PKCS#11 & Post-Quantum TLS 1.3
Keamanan data berstandar militer dan intelijen:
- **Cryptoki PKCS#11 v2.40 HSM Engine:** Antarmuka standar modul keamanan perangkat keras (HSM / Secure Element / TPM). Kunci privat dibuat dengan atribut `CKA_EXTRACTABLE = FALSE` sehingga kunci tidak pernah keluar dari enclave perangkat keras.
- **Hybrid Post-Quantum TLS 1.3:** Menggabungkan kurva eliptik klasik (X25519) dengan kisi pasca-kuantum (ML-KEM-768 / FIPS 203 Kyber). Kunci rahasia bersama (*shared secret*) diturunkan melalui HKDF-SHA256, menjamin kerahasiaan komunikasi terhadap ancaman komputer kuantum di masa depan.
- **Tanda Tangan Digital ML-DSA-87:** Implementasi FIPS 204 (Dilithium-5) untuk verifikasi identitas dan integritas transaksi.

---

## 6. Native AI Sovereign Engine & Model Context Protocol (MCP)
Widya-Lang mengintegrasikan agen AI otonom langsung ke dalam runtime:
- **Model Context Protocol (MCP v2024-11-05):** Mendukung penuh standar protokol AI antarmuka JSON-RPC 2.0 (Stdio & HTTP SSE) untuk terhubung secara mulus dengan IDE AI modern seperti Cursor, Claude Desktop, dan Windsurf.
- **Registrasi Perkakas & Schema Otomatis:** Perintah `widya mcp tools` dan `widya mcp serve` menyediakan abstraksi perkakas (*tools*), sumber daya (*resources* `widya://...`), dan template perintah (*prompts*).
- **Runtime Bobot Murni (GGUF & SafeTensors):** Membaca dan menjalankan inferensi bobot model bahasa besar (LLM) berformat GGUF (v1–v3) dan SafeTensors dengan dekuantisasi kuantisasi SIMD (Q4_0, Q8_0) secara native tanpa binding Python atau library C++ eksternal.

---

## 7. Enterprise Observability & WidyaBase Connection Pooling
- **WidyaBase Enterprise Connection Pool:** Pool koneksi thread-safe dengan *circuit breaker*, pengujian liveness berkala (*heartbeat ping* `PILIH 1;`), manajemen masa pakai (*max lifetime* & *idle eviction*), dan penanganan beban tinggi.
- **Prometheus Text Exposition Format:** Endpoint `/metrics` standar OpenMetrics v0.0.4 yang mengekspor metrik *counter*, *gauge*, dan *histogram*.
- **Distributed Tracing:** Pelacakan jejak terdistribusi berbasis OpenTelemetry dengan propagasi header standar W3C `traceparent`.

---

## 8. Arsitektur Kompiler Mandiri (Self-Hosting Compiler)
Widya-Lang membuktikan kemandirian teknologi penuh (*full technological sovereignty*) melalui siklus kompilasi mandiri tiga tahap:

```
[Kode Sumber Murni Widya (.wya/.widya)]
            │
            ▼ (Kompilasi via Stage-0 / Rust Host)
      [Stage-1 C Biner] ──► SHA-256 Hash₁
            │
            ▼ (Kompilasi Ulang via Stage-1)
      [Stage-2 C Biner] ──► SHA-256 Hash₂
            │
            ▼
    [Perbandingan Deterministik: Hash₁ == Hash₂] ──► [Ed25519 Cryptographic Signature]
```

### 7 Modul Inti Kompiler Murni:
1. `token.wya`: Definisi 50+ token leksikal (kata kunci, operator, literal, tanda baca).
2. `lexer.wya`: Mesin pemindai kode sumber berkecepatan tinggi dengan pelacakan baris/kolom.
3. `ast.wya`: Pohon Sintaks Abstrak (*Abstract Syntax Tree*) untuk ekspresi dan pernyataan.
4. `parser.wya`: Parser Recursive Descent dengan penanganan presedensi operator Pratt.
5. `typesystem.wya`: Inferensi tipe Hindley-Milner dan pemeriksa kompatibilitas statis.
6. `codegen.wya`: Generator kode C99 ANSI murni tanpa ketergantungan runtime eksternal.
7. `main.wya`: Driver CLI kompiler mandiri (*self-hosting entrypoint*).

---

## 9. Ekosistem Pustaka Standar Resmi WPM (@widya/*)
Ekosistem resmi WPM (*Widya Package Manager*) menyediakan paritas fungsional penuh terhadap pustaka terpopuler di dunia Python dan JavaScript:

| Paket Resmi | Paritas Ekosistem | Deskripsi Fungsional |
| :--- | :--- | :--- |
| **`@widya/tensor`** | NumPy (Python) | Larik N-dimensi, aljabar linier, perkalian matriks (*matmul*), dot product, transpose, matriks identitas, dan agregasi statistik. |
| **`@widya/dataframe`** | Pandas (Python) | Struktur data tabular kolumnar, filter ekspresi (*saring*), seleksi kolom (*pilih_kolom*), ringkasan statistik (*ringkasan*), dan parser CSV. |
| **`@widya/web`** | Express / FastAPI | RESTful web router berbasis Radix-tree, perantaian middleware asinkron, dan abstraksi respons JSON / teks. |
| **`@widya/ai`** | PyTorch Autograd | Mesin autograd diferensiasi otomatis reverse-mode berbasis DAG topologis, lapisan linear, aktivasi ReLU, MSE Loss, dan SGD optimizer. |
| **`@widya/sql`** | SQLite3 / Prisma | Mesin basis data relasional in-memory dengan query builder CRUD, filter kondisi fleksibel, dan integritas transaksi. |
| **`@widya/test`** | Jest / Pytest | Kerangka kerja pengujian mandiri (*unit test suite runner*), assertion lengkap (*tegaskan_sama*, *tegaskan_benar*), dan laporan berperingkat. |

---

## 10. Pelari Uji Terintegrasi (Native Test Runner: `widya test`)
Perintah bawaan `widya test` menyediakan kapabilitas *auto-discovery* cerdas:
- Secara otomatis memindai seluruh direktori proyek (`packages/*/tests/`, `tests/`, `contoh/`).
- Mendukung berkas berekstensi kanonikal ganda `.wya` dan `.widya`.
- Mendukung eksekusi fungsi beranotasi `#[uji]` / `#[test]` dan berkas uji berbasis `@widya/test`.
- Menampilkan ringkasan visual berkecepatan tinggi (*sub-100ms execution*).

---

## 11. Perintah Antarmuka Terminal (CLI Cheatsheet)

```bash
# Informasi Versi
widya --version

# Menjalankan Skrip Sumber Widya
widya jalankan skrip.wya
widya run skrip.wya

# Menjalankan Seluruh Unit Test Suite (Auto-Discovery)
widya test
widya test packages/widya-ai/tests/ai_test.wya

# Kompilasi Biner Mandiri
widya build skrip.wya -o biner_mandiri

# Verifikasi Kompiler Mandiri (Self-Hosting Bootstrap & Ed25519)
widya bootstrap

# Model Context Protocol (MCP Server untuk Cursor / Claude Desktop)
widya mcp serve --transport stdio
widya mcp tools
widya mcp call widya_eval '{"code": "1 + 1"}'

# Audit Keamanan Pasokan & SBOM (CycloneDX v1.5 / SPDX v2.3)
widya sbom audit . --format cyclonedx --output bom.json
widya sbom lisensi .
widya sbom airgap vendor/

# Simulasi Ketahanan & Chaos Network (Jepsen-style)
widya chaos simulate --nodes 5 --drop-rate 0.2 --steps 50
widya chaos supervision --strategy one-for-one --restarts 5

# Modul Keamanan Hardware (HSM PKCS#11) & Kripto Pasca-Kuantum
widya hsm status --slot 1
widya hsm gen-key --algorithm ML-DSA-87 --label master_key
widya hsm sign "dokumen rahasia negara"
widya hsm hybrid-kem

# Memeriksa 5 Pilar Kemandirian Sovereign Stack
widya pilar status
widya pilar uji
```
