<div align="center">

# ⚡ Widya-Lang
### *Bahasa Pemrograman Modern Berkinerja Tinggi, Tipe Aman, dan Mandiri.*

[![Version](https://img.shields.io/badge/version-0.1.0--Stable-blue.svg)](https://github.com/widya-lang/widya-lang/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Security Policy](https://img.shields.io/badge/security-RFC%209116-red.svg)](SECURITY.md)
[![Tests Status](https://img.shields.io/badge/Tests-255%20PASS-brightgreen.svg)](#-pengujian--jaminan-mutu)
[![Self-Hosting](https://img.shields.io/badge/Self--Hosting-Deterministic-brightgreen.svg)](#-kompiler-mandiri-full-self-hosting)

[Dokumentasi Tutorial](docs/tutorials/) • [Contoh Kode](contoh/) • [Ekstensi VS Code](editors/vscode/) • [Panduan Kontribusi](CONTRIBUTING.md) • [Kebijakan Keamanan](SECURITY.md)

</div>

---

## 🌟 Tentang Widya-Lang (v0.1.0)

**Widya-Lang** adalah bahasa pemrograman tingkat sistem dan aplikasi yang menggabungkan ekspresivitas sintaks dwibahasa (Nusantara & Internasional), inferensi tipe statis matematis **Hindley-Milner**, alokator memori aman berkinerja tinggi, serta arsitektur mandiri bebas ketergantungan asing (*zero-libc direct syscall*).

Repositori ini difokuskan secara minimal dan presisi pada **inti bahasa pemrograman Widya-Lang**: kompilator, mesin virtual (VM), pustaka standar, manajer paket (WPM), perkakas pengembang, serta panduan tutorial kurikulum inti.

---

## 🚀 Fitur & Keunggulan Inti

- 🧠 **Sistem Tipe Hindley-Milner**: Inferensi tipe otomatis tanpa boilerplate, Aljabar Tipe Data (ADT), *Exhaustive Pattern Matching*, dan Traits / Interface terstruktur.
- ⚙️ **Kompiler Mandiri Deterministik (*Full Self-Hosting*)**: Kompiler Widya ditulis dalam bahasa Widya itu sendiri (`compiler_self_hosted/`). Siklus *Stage-0 → Stage-1 → Stage-2* terverifikasi bit-for-bit secara deterministik (`widya bootstrap`).
- 🛡️ **Pilar Kemandirian (*Direct Syscalls* Bebas Libc)**: Berkomunikasi langsung dengan kernel OS (Linux x86_64/ARM64) melalui raw syscalls tanpa perantara C-runtime (glibc/musl).
- ⚡ **Mesin Virtual Berbasis Register**: Eksekusi bytecode berlatensi rendah dengan optimasi alokasi memori slab pool dan borrow checker statis.
- 🔒 **Kriptografi Pasca-Kuantum (PQC)**: Algoritma standar NIST FIPS 203 (ML-KEM Kyber-768) dan FIPS 204 (ML-DSA Dilithium) yang terpasang secara native.
- 📦 **WPM (Widya Package Manager)**: Solver dependensi PubGrub, verifikasi integritas kriptografis Ed25519 & SHA-256, serta mode *offline vendor*.
- 🧪 **Test Runner Bawaan**: Pelari pengujian otomatis terintegrasi (`widya test`) yang mendeteksi dan mengeksekusi fungsi pengujian proyek.

---

## 📁 Struktur Repositori

```
widya-lang/
├── crates/                  # Crate modular Rust inti
│   ├── widya-core/          # Sistem tipe, AST, dan evaluasi logika dasar
│   ├── widya-compiler/      # Pembuat bytecode dan emisi target
│   ├── widya-vm/            # Mesin virtual berbasis register
│   ├── widya-wpm/           # Manajer paket berdaulat (WPM)
│   ├── widya-lsp/           # Language Server Protocol untuk editor
│   └── widya-office/        # Mesin kompilasi dokumen ISO 32000-1 PDF
├── src/                     # Kode sumber aplikasi CLI widya
├── compiler_self_hosted/    # Kompiler mandiri dalam bahasa Widya (.wya)
├── contoh/                  # Berkas contoh kode sumber resmi (.wya)
├── docs/tutorials/          # Tutorial resmi Level Dasar & Lanjut (Markdown & PDF)
├── editors/vscode/          # Ekstensi resmi Visual Studio Code (v0.1.0 VSIX)
├── packages/                # Paket resmi standar WPM (@widya/tensor, ai, web, dll.)
└── tests/                   # Rangkaian pengujian integrasi otomatis
```

---

## 💻 Sintaks Bahasa

Widya-Lang mendukung kata kunci berbahasa Indonesia maupun sintaks internasional standar:

### 1. Fungsi & Inferensi Tipe
```widya
fungsi tambah<T>(a: T, b: T) -> T {
    kembalikan a + b;
}

fungsi utama() {
    variabel hasil = tambah(10, 25);
    cetak("Hasil: " + ke_teks(hasil));
}
```

### 2. Algebraic Data Types (ADT) & Pencocokan Pola
```widya
pilihan StatusRespon {
    Sukses(Teks),
    Galat(Bulat, Teks)
}

fungsi evaluasi(status: StatusRespon) {
    cocokkan status {
        StatusRespon::Sukses(pesan) => cetak("OK: " + pesan),
        StatusRespon::Galat(kode, err) => cetak("Error [" + ke_teks(kode) + "]: " + err),
    }
}
```

### 3. Konkurensi Utas Paralel Kernel & Saluran Aktor
```widya
fungsi utama() {
    variabel (tx, rx) = buat_saluran();
    
    buat_utas(fungsi() {
        untuk i dalam 1..4 {
            saluran_kirim(tx, "Paket Data Sensor #" + ke_teks(i));
        }
    });
    
    untuk _ dalam 1..4 {
        variabel pesan = saluran_terima(rx);
        cetak("Diterima: " + pesan);
    }
}
```

---

## 📦 Paket Standar Resmi WPM

Widya Package Manager (WPM) menyediakan paket-paket resmi di direktori `packages/`:

| Paket Resmi | Kemampuan Utama |
| :--- | :--- |
| **`@widya/tensor`** | Array N-Dimensi, aljabar linier, matmul 2D, transpose, dot product. |
| **`@widya/dataframe`** | Data tabular, parser CSV, filter predikat, dan agregasi data. |
| **`@widya/ai`** | Autograd diferensiasi otomatis reverse-mode DAG, linear layer, dan loss optimizer. |
| **`@widya/sql`** | Mesin basis data relasional in-memory dan query builder CRUD. |
| **`@widya/web`** | Router RESTful modular berkinerja tinggi dan pipeline middleware. |
| **`@widya/test`** | Test framework dengan assertions lengkap dan pelaporan hasil uji. |

---

## ⚡ Instalasi & Kompilasi dari Sumber

### Prasyarat
- Rust Toolchain versi stabil (1.80+ direkomendasikan)
- Cargo

### Langkah Pemasangan:
```bash
# 1. Klon repositori
git clone https://github.com/widya-lang/widya-lang.git
cd widya-lang

# 2. Bangun biner rilis
cargo build --release

# 3. (Opsional) Pasang ke PATH lokal sistem
cargo install --path .

# 4. Verifikasi versi
widya --version
# Output: widya 0.1.0
```

---

## 🛠️ Perintah CLI Utama

```bash
# Menjalankan skrip kode sumber (.wya)
widya run contoh/01_halo_dunia.wya

# Menjalankan seluruh pengujian unit bawaan
widya test

# Kompilasi ke executable biner mandiri
widya compile contoh/01_halo_dunia.wya -o halo.exe

# Memvalidasi siklus Full Self-Hosting deterministik
widya bootstrap

# Menjalankan Language Server Protocol (LSP) untuk editor
widya lsp

# Memformat kode program sesuai standar resmi
widya format contoh/

# Audit keamanan rantai pasok WPM
widya wpm audit
```

---

## 📚 Buku Panduan & Tutorial Resmi

Materi pembelajaran resmi bahasa Widya-Lang tersedia lengkap di direktori [`docs/tutorials/`](docs/tutorials/):

- 📗 **[Level Dasar (Pemula)](docs/tutorials/01_TUTORIAL_LEVEL_DASAR.pdf)**: Buku panduan komprehensif cetak ~229 halaman standar ISO 32000-1 (40 bab pembelajaran dari pengenalan hingga OOP dan penanganan galat). Sumber bab mandiri di [`docs/tutorials/level_dasar/`](docs/tutorials/level_dasar/).
- 📘 **[Level Tingkat Lanjut](docs/tutorials/02_TUTORIAL_LEVEL_LANJUT.pdf)**: Buku panduan sistem tingkat lanjut (Hindley-Milner, Borrow Checker, Direct Syscalls, dan Kriptografi PQC). Sumber bab mandiri di [`docs/tutorials/level_lanjut/`](docs/tutorials/level_lanjut/).
- 📊 **[Slide Modul Presentasi (PPTX)](docs/tutorials/pptx/)**: 80 modul slide PowerPoint 16:9 siap ajar untuk kurikulum Level Dasar dan Lanjut.

---

## 🧪 Pengujian & Jaminan Mutu

Seluruh pustaka dan komponen inti bahasa diverifikasi melalui pengujian otomatis:

```bash
# Menguji seluruh unit test pustaka dan biner CLI
cargo test --workspace --lib --bins

# Menjalankan test runner bawaan Widya
cargo run --bin widya -- test

# Validasi kompilator mandiri (Self-Hosting Deterministic)
cargo run --bin widya -- bootstrap
```

**Status Mutu v0.1.0:** ✅ **255 / 255 Unit Tests Passed (100%)** • Deterministic Self-Hosting 100% Match.

---

## 🧩 Ekstensi Editor

Paket ekstensi resmi Visual Studio Code tersedia di direktori [`editors/vscode/`](editors/vscode/):
- Penyorotan sintaks cerdas dwi-bahasa
- Cuplikan kode (*snippets*)
- Integrasi ikon berkas `.wya`
- Paket pemasangan mandiri: `editors/vscode/widya-lang-0.1.0.vsix`

---

## 📄 Lisensi

Widya-Lang dirilis di bawah lisensi terbuka [MIT](LICENSE).

<div align="center">
  <sub>Dibangun dengan dedikasi untuk kedaulatan teknologi dan standar rekayasa perangkat lunak modern.</sub>
</div>
