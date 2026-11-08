# 📖 The Widya Book: Panduan Definitif Bahasa Pemrograman Berdaulat

> **Widya-Lang v2.6.0 (Enterprise & Sovereign Edition)**  
> *Bahasa Pemrograman Generasi Baru Berkeamanan Memori Formal, Berkemandirian Penuh Tanpa Ketergantungan Eksternal, dan Dirancang untuk Komputasi Misi-Kritis.*

---

## Daftar Isi
1. [Filosofi & Visi Kedaulatan Digital](#1-filosofi--visi-kedaulatan-digital)
2. [Model Keamanan Memori: MIR Borrow Checker Tanpa GC](#2-model-keamanan-memori-mir-borrow-checker-tanpa-gc)
3. [Arsitektur Konkurensi: Actor Supervision Tree & Chaos Simulator](#3-arsitektur-konkurensi-actor-supervision-tree--chaos-simulator)
4. [Cloud-Native & Edge: WASI 0.2 Preview 2 & Microsecond Cold-Start](#4-cloud-native--edge-wasi-02-preview-2--microsecond-cold-start)
5. [Kriptografi Pertahanan & Hardware Enclave: PKCS#11 & Post-Quantum TLS 1.3](#5-kriptografi-pertahanan--hardware-enclave-pkcs11--post-quantum-tls-13)
6. [Native AI Sovereign Engine & Model Context Protocol (MCP)](#6-native-ai-sovereign-engine--model-context-protocol-mcp)
7. [Enterprise Observability & WidyaBase Connection Pooling](#7-enterprise-observability--widyabase-connection-pooling)
8. [Perintah Antarmuka Terminal (CLI Cheatsheet)](#8-perintah-antarmuka-terminal-cli-cheatsheet)

---

## 1. Filosofi & Visi Kedaulatan Digital
Widya-Lang lahir untuk menjawab tantangan keamanan nasional dan ketahanan infrastruktur komputasi modern. Sebagian besar bahasa kompilasi bergantung pada rantai pasok compiler raksasa (GCC, LLVM, Clang) dan ribuan *crates/packages* pihak ketiga yang rentan terhadap serangan *supply-chain injection*.

Widya-Lang mengusung prinsip **Sovereign Stack (Zero External Dependencies)**:
- **Kompiler Mandiri:** Mengemisikan biner Linux ELF-64 langsung ke tingkat instruksi mesin tanpa memerlukan `gcc` atau `ld`.
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

## 8. Perintah Antarmuka Terminal (CLI Cheatsheet)

```bash
# Menjalankan atau mengompilasi berkas Widya
widya jalankan skrip.wya
widya build skrip.wya -o biner_mandiri

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
