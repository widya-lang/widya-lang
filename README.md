<div align="center">

# ⚡ Widya-Lang
### *Bahasa Pemrograman Modern, Tipe Aman, Performa Tinggi, dan Full Self-Hosting.*

[![Version](https://img.shields.io/badge/version-1.0.0--stable-blue.svg)](https://github.com/Bakulcoding/widya-lang/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Security Policy](https://img.shields.io/badge/security-RFC%209116-red.svg)](SECURITY.md)
[![Web Framework](https://img.shields.io/badge/WidyaWeb-54%2F54%20PASS-brightgreen.svg)](#-widyaweb-framework)
[![Desktop Framework](https://img.shields.io/badge/WidyaDesktop-Grade%20A%2B-blueviolet.svg)](#-widyadesktop-framework)

[Portal Resmi](https://widya-lang.org) • [Dokumentasi Lengkap](DOKUMENTASI_KOMPREHENSIF.md) • [Tutorial WidyaBase](TUTORIAL_WIDYABASE.md) • [Ekstensi VS Code](editors/vscode/) • [Panduan Kontribusi](CONTRIBUTING.md) • [Kebijakan Keamanan](SECURITY.md)

</div>

---

## 🌟 Tentang Widya-Lang

**Widya-Lang** adalah bahasa pemrograman tingkat sistem dan aplikasi yang menggabungkan ekspresivitas sintaks dwibahasa (Nusantara & Internasional), inferensi tipe statis matematis **Hindley-Milner**, manajemen memori aman tanpa jeda Garbage Collector besar (*Safe Reference Counting + Tarjan Cycle Detection*), dan ekosistem terpadu untuk **Web** dan **Desktop Native** tingkat industri.

---

## 🚀 Fitur & Keunggulan Utama

- 🧠 **Sistem Tipe Hindley-Milner**: Inferensi tipe otomatis tanpa boilerplate, Aljabar Tipe Data (ADT), Pattern Matching, dan Traits / Type Classes.
- 🌐 **WidyaWeb Framework Bawaan**: Kerangka kerja web berkinerja tinggi kelas industri (Routing, Middleware, HTTP Security Headers, WebSocket, SSE Live Reload, Session Hijacking Protection) dengan 54/54 uji lulus.
- 🖥️ **WidyaDesktop Framework (Grade A+)**: Kerangka kerja desktop native modern berbasis Hybrid IPC Bridge, manajemen Multi-Window, Baki Sistem (System Tray), Brankas Kripto AES-256-GCM, Generator Struk Thermal POS ESC/POS, dan Pipeline Pembaruan Kriptografis Ed25519.
- 🏗️ **100% Full Self-Hosting**: Kompiler Widya ditulis seluruhnya dalam bahasa Widya (`compiler_self_hosted/`) dengan 3-Stage Bootstrap deterministik (`widya bootstrap`).
- ⚡ **Multi-Target Native Compiler**: Menghasilkan biner Native OS executable, LLVM IR (`.ll`), WebAssembly (`.wasm`), WebGPU Compute Shader (`.wgsl`), dan Linux Kernel eBPF (`.bpf.c`).
- 🔒 **Audit Keamanan Statis & Pertahanan Berlapis**: Detektor kebocoran Secret/API Key, pencegahan SQL Injection, Token IPC Anti-CSRF, dan berkas integritas SHA-256 (`widya_security.lock`).
- 🛠️ **All-in-One Toolchain**: Language Server Protocol (LSP), Debug Adapter Protocol (DAP), Memory Profiler, dan WPM Package Manager dengan integritas kriptografis.
- 🗄️ **Pustaka Standar C FFI Lengkap**: Binding resmi tingkat tinggi untuk **SQLite3**, **DuckDB**, **PostgreSQL**, **MySQL**, **Redb**, **SurrealDB**, dan **Kriptografi** (AES-256-GCM, Ed25519, SHA-256).

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

# 2. Jalankan skrip instalasi otomatis (memerlukan Rust/Cargo)
chmod +x install.sh
./install.sh

# 3. Verifikasi instalasi
widya --version
```
> *Alternatif manual:* Jalankan `cargo install --path .` untuk memasang biner `widya` langsung ke `~/.cargo/bin/widya`.

---

### 🍎 Di macOS (Apple Silicon M1/M2/M3 & Intel)
```bash
# 1. Kloning repositori
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang

# 2. Jalankan skrip instalasi otomatis
chmod +x install.sh
./install.sh

# 3. Verifikasi instalasi
widya --version
```

---

### 🪟 Di Windows (PowerShell / Command Prompt)
```powershell
# 1. Buka PowerShell dan kloning repositori
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang

# 2. Jalankan skrip instalasi otomatis Windows
.\install.ps1

# 3. Buka jendela terminal baru lalu verifikasi instalasi
widya --version
```
> *Alternatif manual:* Jalankan `cargo build --release`, lalu tambahkan folder `target\release` ke variabel `PATH` lingkungan Windows Anda.

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

# Mengompilasi aplikasi mobile Android / iOS
widya mobile bangun app.wya --target android
```

---

## 📁 Struktur Direktori Repositori

```text
widya-lang/
├── src/                    # Source code kompiler utama & runtime
│   ├── compiler/           # Codegen, monomorphization, type-aware optimizations
│   ├── typesystem.rs       # Hindley-Milner Type Inferrer & Unification
│   ├── stdlib_ffi/         # Pustaka C FFI (SQLite3, Cryptography)
│   ├── dap.rs              # Debug Adapter Protocol Engine
│   ├── lsp.rs              # Language Server Protocol Engine
│   ├── profiler.rs         # Memory Profiler & Tarjan Cycle Detector
│   ├── security_tools.rs   # Static Security AST Linter (SQLi, XSS, Secrets)
│   └── wasm_playground.rs  # WASM Compiler Engine
├── compiler_self_hosted/   # Kompiler mandiri ditulis 100% dalam Widya (.widya)
├── editors/                # Ekstensi resmi editor VS Code
│   └── vscode/             # TextMate Grammar, LSP client, DAP contribution
├── www/                    # Website resmi & Interactive WebAssembly Playground
├── tests/                  # 111 Unit & Integration Test Suites
├── contoh/                 # Kumpulan contoh aplikasi resmi
└── DOKUMENTASI_KOMPREHENSIF.md # Buku panduan 18 bab lengkap
```

---

## 🧪 Pengujian (Test Suite)

Seluruh komponen compiler dan ekosistem diuji secara otomatis:

```bash
cargo test
```
**Status Saat Ini:** ✅ **111 / 111 Tests Passed (100%)** — 0 Errors, 0 Warnings.

---

## 📖 Dokumentasi Lengkap

Untuk panduan mendalam tentang arsitektur, sintaks, sistem operasi, basis data terdistribusi, dan tutorial bertahap, silakan baca:
👉 **[DOKUMENTASI_KOMPREHENSIF.md](DOKUMENTASI_KOMPREHENSIF.md)**

---

## 📄 Lisensi

Widya-Lang dirilis di bawah lisensi open source [MIT](LICENSE) / Apache-2.0.

<div align="center">
  <sub>Dibangun dengan dedikasi untuk kedaulatan teknologi dan standar rekayasa perangkat lunak modern.</sub>
</div>
