<div align="center">

# ⚡ Widya-Lang
### *Bahasa Pemrograman Berdaulat, Tipe Aman, Kinerja Tinggi, dan Zero-External-Dependency Stack.*

[![Version](https://img.shields.io/badge/version-0.1.0--Stable-blue.svg)](https://github.com/widya-lang/widya-lang/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Security Policy](https://img.shields.io/badge/security-RFC%209116-red.svg)](SECURITY.md)
[![Penta-Core Matrix](https://img.shields.io/badge/Penta--Core-100%25%20PASS-brightgreen.svg)](#-penta-core-sovereign-matrix)
[![Tests Status](https://img.shields.io/badge/Tests-250%2B%20PASS-brightgreen.svg)](#-pengujian-test-suite)

[Portal Resmi & Dokumentasi](https://widya-lang.org) • [Contoh Kode](contoh/) • [Ekstensi VS Code](editors/vscode/) • [Panduan Kontribusi](CONTRIBUTING.md) • [Kebijakan Keamanan](SECURITY.md)

</div>

---

## 🌟 Tentang Widya-Lang (v0.1.0 Genesis Release)

**Widya-Lang** adalah bahasa pemrograman tingkat sistem dan aplikasi yang menggabungkan ekspresivitas sintaks dwibahasa (Nusantara & Internasional), inferensi tipe statis matematis **Hindley-Milner**, alokator memori aman berkinerja tinggi, serta arsitektur **Zero-Foreign-Dependency** terpadu untuk ekosistem **OS, Web, Desktop, Mobile, dan 18 Pilar Industri Strategis**.

Mulai rilis **v0.1.0**, Widya-Lang mandiri 100% dari pustaka pihak ketiga: menghapus ketergantungan pada GCC, Clang, GNU ld, LLVM, Node.js, Chromium, Python, dan C-runtime eksternal.

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
- 🔄 **Kompiler Mandiri Deterministik (*Full Self-Hosting*)**: Siklus Stage-0 → Stage-1 → Stage-2 terverifikasi bit-for-bit dengan emisi kode C99 mandiri ber-Ed25519.

---

## 📦 Ekosistem Paket Standar Resmi WPM (Paritas Penuh Python & Node.js)

Widya Package Manager (WPM) menyediakan pustaka standar berlisensi MIT yang ditulis murni dalam sintaksis Widya:

| Paket Resmi | Padanan Python / Node.js | Kemampuan Utama |
| :--- | :--- | :--- |
| **`@widya/tensor`** | **NumPy** | Array N-Dimensi, aljabar linier, matmul 2D, transpose, dot product, reduksi statistik. |
| **`@widya/dataframe`** | **Pandas** | Data tabular berorientasi kolom/baris, parser CSV, filter predikat, describe (`ringkasan`), groupby. |
| **`@widya/ai`** | **PyTorch** | Autograd DAG diferensiasi otomatis reverse-mode topologis, linear layer, activation ReLU, MSE loss, optimizer SGD. |
| **`@widya/sql`** | **SQLite3 / Prisma** | Mesin basis data relasional in-memory, query builder (CRUD), WHERE filters, update, delete, count. |
| **`@widya/web`** | **Express / FastAPI** | Router RESTful modular berkinerja tinggi, pipeline middleware, typed HTTP response builder. |
| **`@widya/test`** | **Jest / Pytest** | Test suite runner, assertions lengkap (`tegaskan_sama`, `tegaskan_benar`, dll.), pelaporan hasil otomatis. |

### Contoh Kode: Layanan Analitika & Inferensi AI Terpadu
```widya
impor "@widya/web" sebagai web;
impor "@widya/sql" sebagai sql;
impor "@widya/dataframe" sebagai pd;
impor "@widya/tensor" sebagai np;
impor "@widya/ai" sebagai torch;

// 1. Inisialisasi basis data relasional in-memory
misal db = sql.db_buka();
sql.db_buat_tabel(db, "sensor");
sql.db_sisip(db, "sensor", {"id": 1, "beban": 14.0, "suhu": 35.0});

// 2. Transformasi tabular DataFrame
misal df = pd.df_buat(["id", "beban", "suhu"], sql.db_pilih(db, "sensor"));
misal stat = pd.ringkasan(df);

// 3. Pelatihan satu langkah Autograd Deep Learning
misal layer = torch.lapisan_linear(1.5, 0.5);
misal out = torch.linear_maju(layer, torch.variabel(stat[0]["mean"]));
misal rugi = torch.mse_loss(out, 40.0);
torch.mundur(rugi);
torch.sgd_langkah([layer.bobot, layer.bias], 0.01);

// 4. Sajikan via REST Microservice
misal app = web.buat_router();
web.get(app, "/api/suhu", fungsi(req) {
    kembalikan web.respons_json({"suhu_prediksi": out.data, "status": "optimal"});
});
```


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
git clone https://github.com/widya-lang/widya-lang.git
cd widya-lang

# 2. Pasang langsung biner rilis
cargo install --path .

# 3. Verifikasi instalasi
widya --version
# Output: widya 0.1.0
```

### 🍎 Di macOS (Apple Silicon M1/M2/M3 & Intel)
```bash
git clone https://github.com/widya-lang/widya-lang.git
cd widya-lang
cargo install --path .
widya --version
```

### 🪟 Di Windows (PowerShell / Command Prompt)
```powershell
git clone https://github.com/widya-lang/widya-lang.git
cd widya-lang
cargo install --path .
widya --version
```

---

## 🛠️ Perintah CLI Utama

```bash
# Menjalankan skrip langsung
widya run contoh/01_halo_dunia.wya

# Menjalankan seluruh pengujian unit test bawaan (Auto-Discovery)
widya test

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
widya wpm add @widya/tensor
widya wpm add @widya/ai
widya wpm audit          # Audit rantai pasok (CVE/WSA, SHA256 & Ed25519)
widya wpm outdated       # Pengecekan versi usang (SemVer)
widya wpm serve-registry # Jalankan registry lokal di port 8090

# Asisten AI Pemrograman Berdaulat 100% Lokal (WidyaCoder - Opsi 3)
widyacoder indeks        # Indeks seluruh dokumentasi & contoh kode via HNSW
widyacoder tanya "bagaimana cara membuat server web?" # Tanya jawab RAG lokal + Ollama
widyacoder chat          # Sesi percakapan REPL interaktif pemrograman
widyacoder serve         # Jalankan REST API server lokal di port 8088
```

---

## 🧪 Pengujian & Jaminan Mutu

Seluruh komponen compiler, runtime, database, operating system, dan 18 pilar industri diuji secara otomatis:

```bash
cargo test
```
**Status GA Saat Ini:** ✅ **250+ / 250+ Tests Passed (100%)** — 0 Errors, 0 Warnings, 0 Flaky Tests.

---

## 📖 Dokumentasi & Komunitas

Dokumentasi resmi lengkap, panduan arsitektur, dan tutorial interaktif tersedia di:
* 🌐 **Portal Resmi & Dokumentasi:** [https://widya-lang.org](https://widya-lang.org)
* 💡 **Contoh Kode Rujukan:** Direktori [contoh/](contoh/) memuat kode rujukan bahasa dan pustaka inti.

---

## 📄 Lisensi

Widya-Lang dirilis di bawah lisensi open source [MIT](LICENSE) / Apache-2.0.

<div align="center">
  <sub>Dibangun dengan dedikasi untuk kedaulatan teknologi dan standar rekayasa perangkat lunak modern tingkat global.</sub>
</div>
