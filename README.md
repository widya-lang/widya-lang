<div align="center">

# ⚡ Widya-Lang
### *Bahasa Pemrograman Berdaulat, Tipe Aman, Kinerja Tinggi, dan Zero-External-Dependency Stack.*

[![Version](https://img.shields.io/badge/version-2.6.0--GA%20Stable-blue.svg)](https://github.com/Bakulcoding/widya-lang/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Security Policy](https://img.shields.io/badge/security-RFC%209116-red.svg)](SECURITY.md)
[![Penta-Core Matrix](https://img.shields.io/badge/Penta--Core-100%25%20PASS-brightgreen.svg)](#-penta-core-sovereign-matrix)
[![Tests Status](https://img.shields.io/badge/Tests-250%2B%20PASS-brightgreen.svg)](#-pengujian-test-suite)

[Portal Resmi](https://widya-lang.org) • [Dokumentasi Lengkap](DOKUMENTASI_KOMPREHENSIF.md) • [Pilar Kemandirian](DOKUMENTASI_PILAR_KEMANDIRIAN.md) • [Ekstensi VS Code](editors/vscode/) • [Panduan Kontribusi](CONTRIBUTING.md) • [Kebijakan Keamanan](SECURITY.md)

</div>

---

## 🌟 Tentang Widya-Lang (v2.6.0 General Availability / GA)

**Widya-Lang** adalah bahasa pemrograman tingkat sistem dan aplikasi yang menggabungkan ekspresivitas sintaks dwibahasa (Nusantara & Internasional), inferensi tipe statis matematis **Hindley-Milner**, alokator memori aman berkinerja tinggi, serta arsitektur **Zero-Foreign-Dependency** terpadu untuk ekosistem **OS, Web, Desktop, Mobile, dan 18 Pilar Industri Strategis**.

Mulai rilis **v2.6.0 GA**, Widya-Lang mandiri 100% dari pustaka pihak ketiga: menghapus ketergantungan pada GCC, Clang, GNU ld, LLVM, Node.js, Chromium, Python, dan C-runtime eksternal.

---

## 🏛️ Matriks Kedaulatan Penta-Core (100% Pure In-House)

```
+---------------------------------------------------------------------------------------------------+
|                                  WIDYA LANGUAGE & RUNTIME                                         |
+-------------------+--------------------+--------------------+--------------------+----------------+
|     WidyaOS       |     WidyaBase      |      WidyaWeb      |    WidyaDesktop    |  WidyaMobile   |
|   (OS & Kernel)   | (Multi-Model DB)   |  (Fullstack SSR)   | (Workstation POS)  | (Canvas 2D UI) |
+-------------------+--------------------+--------------------+--------------------+----------------+
| - PML4/5 Paging   | - Slotted 4KB Page | - App Router       | - Window Lifecycle | - Pure Canvas  |
| - CFS Scheduler   | - MVCC & WAL ACID  | - Streaming SSR    | - Mica/Acrylic     | - Font Raster  |
| - WidyaFS (CoW)   | - Vector HNSW      | - Islands Hydration| - Zero-Copy IPC    | - Reactive UI  |
| - IDT/PIC Handler | - Document JSONB   | - Server Actions   | - ESC/POS Bytecode | - Touch Gest   |
| - Live ISO UEFI   | - Parquet & Arrow  | - HTTP/3 QUIC / SRT| - Pure Packagers   | - Pure APK/IPA |
+-------------------+--------------------+--------------------+--------------------+----------------+
| 100% PURE RUST & WIDYA CODE — ZERO C/C++ DEPENDENCY — ZERO CHROMIUM/NODE/FLUTTER/QT/SKIA/ELECTRON |
+---------------------------------------------------------------------------------------------------+
```

---

## 🚀 Fitur & Keunggulan Utama

- 🧠 **Sistem Tipe Hindley-Milner**: Inferensi tipe otomatis tanpa boilerplate, Aljabar Tipe Data (ADT), Pattern Matching, dan Traits / Type Classes.
- ⚡ **Multi-Target Native Emitter Mandiri**: Pembangkit biner ELF-64 murni (`x86_64`, `aarch64`, `riscv64`), Windows PE32+ (`.exe`), WebAssembly v1 (`.wasm`), WebGPU (`.wgsl`), dan Linux Kernel eBPF (`.bpf.c`).
- 🗄️ **WidyaBase Multi-Model Autonomous Database**: Slotted-page 4KB, MVCC snapshot isolation, WAL crash-recovery ARIES, B+Tree, HNSW Vector search untuk AI RAG lokal, serta Parquet & Arrow columnar engine. Kompatibel penuh dengan `pgwire` v3.0 dan Redis protocol.
- 🌐 **WidyaWeb Framework Bawaan**: Nested App Router, Streaming SSR, Islands Architecture, Zero-API Server Actions, HTTP/3 QUIC transport, dan protokol live streaming RTMP/SRT murni.
- 🖥️ **WidyaDesktop & Standalone Packagers**: Window lifecycle state machine, zero-copy IPC bridge, Mica/Acrylic visual styling, POS ESC/POS thermal bytecode generator, serta builder installer biner murni `.deb`, `.rpm`, `.tar.gz`, `.run`, dan `.wapp`.
- 📱 **WidyaMobile & Mobile Bundler**: Pure 2D software canvas rasterizer, TrueType font parser, Three-Tree Reactive UI, serta generator paket fisik mandiri `.apk` (AXML/DEX/ARSC/v1 signature) dan `.ipa` (Mach-O ARM64).
- 📦 **WPM (Widya Package Manager)**: Solver dependensi PubGrub, tanda tangan digital Ed25519, verifikasi checksum SHA-256, dan mode air-gapped `widya pasok` (vendor).
- 🏭 **18 Pilar Kedaulatan Industri Strategis**:
  1. *Kripto Pasca-Kuantum & ZK*: FIPS 203 ML-KEM, FIPS 204 ML-DSA, R1CS synthesizer, Fiat-Shamir NIZK proof.
  2. *Energi & Smart Grid*: IEC 61850 GOOSE (`0x88B8`) & Sampled Values (`0x88BA`), True RMS, sub-3ms relay trip.
  3. *Kesehatan & Radiologi*: DICOM v3.0 Explicit VR, Hounsfield Unit scaling, windowing presets, ImageBuffer.
  4. *Pertanian Presisi*: Indeks multispektral NDVI/NDRE/SAVI/EVI, ISOBUS TLG.BIN grid & ISO-XML.
  5. *Material Canggih*: 7 Sistem Kristal Bravais, tensor metrik resiprokal, difraksi sinar-X serbuk (XRD) Bragg Cu-Kα1.
  6. *Otomotif & Kendaraan Listrik*: ISO 14229 UDS server, ISO-TP DoCAN multi-frame transport, Coulomb counting EV BMS.
  7. *Eksplorasi Pertambangan*: SEG-Y Rev 1/2 binary engine, konverter IBM 360/370 float, transkoder EBCDIC, seismik Zoeppritz.
  8. *Smart City & Urban*: Hidrologi banjir rasional ($Q = 0.278 \cdot C \cdot I \cdot A$), Webster traffic cycle, Rooftop PV.
  9. *Telekomunikasi 5G/6G*: 5G Core UPF line-rate packet router, GTP-U tunnel encap/decap, protokol PFCP.
  10. *Antariksa & Satelit*: Paket telemetri CCSDS, sinkronisasi ASM `0x1ACFFC1D`, kalkulator orbit TLE SGP4.
  11. *Maritim & Navigasi*: Dekoder ITU-R M.1371 NMEA AIS Type 1/2/3/5, perhitungan CPA/TCPA anti-tabrakan kapal.
  12. *Sistem Tertanam RTOS*: Penjadwal preemptif berbasis prioritas, Priority-Inheritance Mutex (PI-Mutex).
  13. *Game & Spasial 3D*: Software 3D rasterizer CPU, interpolasi Barycentric, Z-buffer, backface culling.
  14. *Neuroteknologi & BCI*: Spiking Neural Network (SNN), model Leaky Integrate-and-Fire (LIF), STDP.
  15. *Komputasi Kuantum*: Simulator state-vector kompleks 64-bit hingga 24 qubit, parser/eksekutor OpenQASM 3.0.
  16. *AI & Bobot Model*: Parser SafeTensors dan GGUF v3 lokal untuk inferensi model bahasa LLM/SLM lokal.
  17. *Kantor & Dokumen ODF*: ISO/IEC 26300 ODF (.odt, .ods, .odp) dan ISO/IEC 29500 OOXML (.docx, .xlsx, .pptx) murni.
  18. *Multimedia & Codec*: Enkoder/dekoder gambar QOI, BMP bit-depth penuh, dan audio PCM WAV lossless.

---

## 💻 Sintaks Dasar

Widya-Lang mendukung kata kunci berbahasa Indonesia maupun sintaks internasional standar:

```widya
// Contoh 1: Fungsi Generics & Inferensi Tipe Hindley-Milner
fungsi tambah<T>(a: T, b: T) -> T {
    kembalikan a + b;
}

misal hasil = tambah(10, 25);
cetak("Hasil:", hasil);

// Contoh 2: Pencocokan Pola (Pattern Matching)
pilihan StatusRespon {
    Sukses(String),
    Galat(Int, String)
}

misal status = StatusRespon::Sukses("Data berhasil diproses");

cocokkan status {
    StatusRespon::Sukses(pesan) => cetak("OK:", pesan),
    StatusRespon::Galat(kode, err) => cetak("Error [", kode, "]:", err),
    _ => cetak("Status tidak dikenal")
}
```

---

## ⚡ Cara Instalasi & Menjalankan

### 🐧 Di Linux (Ubuntu, Debian, Fedora, Arch, dll.)
```bash
# 1. Kloning repositori
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang

# 2. Pasang langsung biner rilis
cargo install --path .

# 3. Verifikasi instalasi
widya --version
# Output: widya 2.6.0
```

### 🍎 Di macOS (Apple Silicon M1/M2/M3 & Intel)
```bash
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang
cargo install --path .
widya --version
```

### 🪟 Di Windows (PowerShell / Command Prompt)
```powershell
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang
cargo install --path .
widya --version
```

---

## 🛠️ Perintah CLI Utama

```bash
# Menjalankan skrip langsung
widya run contoh/01_halo_dunia.wya

# Kompilasi ke biner mandiri (.exe / ELF)
widya compile app.wya --output app.exe

# Memverifikasi Full Self-Hosting Compiler
widya bootstrap

# Menjalankan audit keamanan AST linter
widya audit .

# Memeriksa profil memori & deteksi siklus referensi
widya profile app.wya

# Menjalankan Language Server Protocol (LSP) untuk IDE
widya lsp

# Menjalankan server Debug Adapter Protocol (DAP)
widya dap

# Mengompilasi WebAssembly untuk browser
widya wasm app.wya --output app.wasm

# Manajemen Paket WPM & Keamanan Rantai Pasok
widya wpm add widya-office --versi 2.6.0
widya wpm audit          # Audit rantai pasok (CVE/WSA, SHA256 & Ed25519)
widya wpm outdated       # Pengecekan versi usang (SemVer)
widya wpm serve-registry # Jalankan registry lokal di port 8090
```

---

## 🧪 Pengujian & Jaminan Mutu

Seluruh komponen compiler, runtime, database, operating system, dan 18 pilar industri diuji secara otomatis:

```bash
cargo test
```
**Status GA Saat Ini:** ✅ **250+ / 250+ Tests Passed (100%)** — 0 Errors, 0 Warnings, 0 Flaky Tests.

---

## 📖 Dokumentasi Lengkap

Untuk panduan mendalam tentang arsitektur, sintaks, sistem operasi, basis data terdistribusi, dan tutorial bertahap:
* 👉 **[DOKUMENTASI_PILAR_KEMANDIRIAN.md](DOKUMENTASI_PILAR_KEMANDIRIAN.md)** — Spesifikasi Arsitektur Kedaulatan Nol-Dependensi & 18 Pilar Industri.
* 👉 **[ROADMAP_PENTA_CORE_SOVEREIGN_MATRIX.md](ROADMAP_PENTA_CORE_SOVEREIGN_MATRIX.md)** — Spesifikasi Matriks 5 Pilar Komputasi Mandiri.
* 👉 **[DOKUMENTASI_KOMPREHENSIF.md](DOKUMENTASI_KOMPREHENSIF.md)** — Buku Panduan Lengkap Pemrograman Widya.

---

## 📄 Lisensi

Widya-Lang dirilis di bawah lisensi open source [MIT](LICENSE) / Apache-2.0.

<div align="center">
  <sub>Dibangun dengan dedikasi untuk kedaulatan teknologi dan standar rekayasa perangkat lunak modern tingkat global.</sub>
</div>
