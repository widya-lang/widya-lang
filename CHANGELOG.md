# Changelog

Semua perubahan penting pada ekosistem bahasa pemrograman Widya-Lang didokumentasikan dalam berkas ini.

Format berkas ini didasarkan pada [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), dan versi rilis mengikuti standar [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [2.6.0] - 2026-10-08

### 🛡️ General Availability (GA) & Arsitektur Zero-Foreign-Dependency

#### ✨ Kedaulatan Sistem Penuh (Pure In-House Sovereign Stack)
- **Kompiler & Linker Mandiri (`src/compiler/pure_emitter.rs`)**:
  - Pembangkit berkas biner eksekusi Linux ELF-64 langsung dan generator instruksi mesin x86_64 murni tanpa membutuhkan GCC, Clang, GNU `ld`, maupun LLVM eksternal.
  - Multi-target native emitter mandiri untuk ELF-64 Linux, Windows PE32+ (`.exe`), WebAssembly v1 (`.wasm`), WebGPU (`.wgsl`), dan Linux Kernel eBPF (`.bpf.c`).
- **Pustaka Standar Murni (In-House Stdlib)**:
  - **Kriptografi Murni (`src/kripto/pure_crypto.rs`)**: Implementasi standar FIPS 180-4 (SHA-256 & SHA-512), RFC 2104 (HMAC-SHA256), RFC 4648 (Hex & Base64), RFC 8439 (ChaCha20-Poly1305 AEAD), dan RFC 8032 (Ed25519) tanpa dependensi pihak ketiga.
  - **Format Serialisasi Murni (`src/stdlib/pure_format.rs`)**: Parser dan serializer RFC 8259 JSON, RFC 4180 CSV, serta konfigurasi INI.
  - **Engine Argumen CLI Mandiri (`src/cli/pure_args.rs`)**: Parser argumen baris perintah mandiri dengan dukungan subcommand, opsi nilai, flags, dan pesan bantuan dwibahasa (Nusantara & Internasional).
  - **HTTP/1.1 Engine Murni (`src/web/pure_http.rs`)**: Klien dan peladen HTTP/1.1 berbasis raw socket `std::net::TcpStream` & `TcpListener`.
- **Direct Kernel Syscalls & Sovereign Allocator (`src/sys/`)**:
  - Panggilan langsung instruksi syscall assembly x86_64 (`sys_write`, `sys_read`, `sys_mmap`, `sys_munmap`, `sys_getpid`) tanpa melalui C runtime glibc/musl.
  - Alokator memori heap mandiri berbasis Slab Block Pools (32B–2048B) di atas halaman memori virtual anonim mmap.
- **Basis Data Berdaulat Murni WidyaBase (`src/widyabase/wire.rs`)**:
  - Implementasi protokol wire biner PGWire v3.0 mandiri untuk klien TCP WidyaBase tanpa ketergantungan driver FFI eksternal.
- **Target Bare-Metal & Unikernel OS (`src/compiler/unikernel.rs`)**:
  - Pembangkit citra unikernel mandiri dengan protokol Linux 64-bit Direct Kernel Boot, Multiboot2, dan PVH untuk boot sub-5ms di atas hypervisor virtual.

#### 📦 Penguatan Ekosistem Widya Package Manager (WPM)
- **Katalog Resmi WPM**:
  - Distribusi paket resmi berdaulat ditandatangani secara kriptografis menggunakan kurva **Ed25519** dan hash SHA-256.
- **Audit Keamanan Rantai Pasok (`WpmSecurityAuditor`)**:
  - Perintah `widya wpm audit` untuk memeriksa integritas paket dan basis data penasihat keamanan (*Widya Security Advisories*).
- **Pemeriksa Versi Usang (`WpmOutdatedChecker`)**:
  - Perintah `widya wpm outdated` untuk deteksi otomatis pembaruan SemVer (*Major*, *Minor*, *Patch*).
- **Inspeksi Metadata Paket**:
  - Perintah `widya wpm info <paket>` untuk menampilkan ringkasan author, lisensi, tag, unduhan, riwayat versi, dan verifikasi tanda tangan digital.

---

## [2.5.0] - 2026-10-02

### ⚡ Production Release — Bare-Metal Firmware, WDoc, AI LSP & Package Management

#### ✨ Fitur Baru & Peningkatan
- **Generator Firmware Bare-Metal & Mikrokontroler (`src/embedded.rs`)**:
  - Dukungan target perangkat keras ARM Cortex-M0/M4F, RISC-V RV32IMAC (`no_std`), dan ESP32.
  - Pembangkitan otomatis Linker Scripts (`.ld`), startup code C, dan Interrupt Vector Table (`widya embedded` / `widya firmware`).
- **Dokumentasi Terintegrasi & Live Doc Server (`src/wdoc.rs`)**:
  - Antarmuka dokumentasi modern dengan filter pencarian instan dan visualisasi signature fungsi.
  - Server dokumentasi lokal terintegrasi via `widya doc --serve -p 8090`.
- **Akademi Pembelajaran Interaktif CLI (`src/learn.rs`)**:
  - Kurikulum interaktif terminal berisi modul pembelajaran bertahap, demonstrasi kode sumber, dan latihan pemrograman (`widya learn` / `widya tutor`).
- **AI-Powered Code Intelligence & LSP Snippets (`src/lsp_enhanced.rs`)**:
  - Snippet cerdas untuk struktur data, pattern matching, actor model, WidyaBase CRUD, dan tanda tangan pasca-kuantum.
- **Hardware Performance Benchmarking Suite (`src/benchmark.rs`)**:
  - Pengukuran throughput transaksi, latensi P50/P95/P99, alokasi memori zero-GC, dan komparasi waktu eksekusi (`widya tolakukur` / `widya bench`).
- **Widya Package Manager CLI (`widya pkg` / `widya wpm`)**:
  - Manajemen inisialisasi proyek, resolusi dependensi, verifikasi hash paket, dan publikasi ke registri lokal.