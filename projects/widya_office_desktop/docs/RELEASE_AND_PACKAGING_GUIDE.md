# Panduan Kompilasi Biner & Distribusi Multi-Platform 🚀

> **Spesifikasi Packaging Executable Mandiri Tingkat Perusahaan**  
> *Windows (.exe/.msi) · Linux (.bin/.deb/.rpm) · macOS (.app/.dmg) · Air-Gapped Deployment*

---

## 1. Filosofi Distribusi Berdaulat

Sebagian besar aplikasi perkantoran modern didistribusikan dalam paket raksasa berukuran 200–800 MB karena membawa seluruh runtime Chromium, Node.js, atau pustaka dinamis raksasa.

**WidyaOffice Desktop** memanfaatkan engine [src/desktop/pure_packager.rs](file:///home/devops/widya-lang/src/desktop/pure_packager.rs) untuk menghasilkan:
- **Ukuran Biner Ringkas:** Biner mandiri berukuran **< 25 MB** (sudah mencakup antarmuka Ribbon, seluruh engine dokumen DOCX/XLSX/PPTX, font dasar, dan modul kriptografi PQC).
- **Zero-Dependency Portable:** Berjalan langsung tanpa perlu menginstal runtime tambahan (tidak perlu Java, .NET Framework, Node.js, maupun GTK runtime).
- **100% Air-Gapped Friendly:** Bebas dari telemetri tersembunyi (*zero phone-home*), siap digunakan di instalasi militer, kementerian, perbankan, dan jaringan intranet tertutup.

---

## 2. Matriks Target Distribusi

```
┌────────────────────────────────────────────────────────────────────────┐
│                    WIDYAOFFICE DESKTOP BUILD MATRIX                    │
├──────────────────┬─────────────────┬───────────────────────────────────┤
│ Target Platform  │ Format Paket    │ Keterangan & Karakteristik        │
├──────────────────┼─────────────────┼───────────────────────────────────┤
│ **Windows**      │ `WidyaOffice.exe`│ Portabel PE32+, ASLR & DEP aktif  │
│ (10 / 11 / Server)│ `WidyaOffice.msi`│ Installer enterprise (GPO Silent) │
├──────────────────┼─────────────────┼───────────────────────────────────┤
│ **Linux**        │ `widya-office.bin`│ Statis ELF 64-bit tunggal        │
│ (Debian/Ubuntu)  │ `.deb`          │ Desktop shortcut & asosiasi MIME  │
│ (RHEL/Fedora)    │ `.rpm`          │ Paket korporat terverifikasi GPG  │
├──────────────────┼─────────────────┼───────────────────────────────────┤
│ **macOS**        │ `WidyaOffice.app`│ Universal Mach-O 64 (M1-M4 & x86) │
│ (12+ s/d 15+)    │ `WidyaOffice.dmg`│ Gambar disk drag-to-Applications  │
└──────────────────┴─────────────────┴───────────────────────────────────┘
```

---

## 3. Struktur Bundling Mandiri (Single Executable)

Saat proses kompilasi rilis dijalankan, aset antarmuka pengguna (HTML, CSS Ribbon, ikon SVG, palet rumus) dikompresi dan ditanamkan langsung ke dalam segmen data biner (*embedded binary assets*):

```
WidyaOffice Executable Biner (Contoh: WidyaOffice.exe)
├── Segmen Kode Mesin (Rust / Widya Core)
│   ├── JendelaMurni & Pengelola Window Native
│   ├── JembatanIPCMurni
│   ├── Engine DOCX, OMML Math, XLSX, PPTX, PDF
│   └── Engine Kriptografi Pasca-Kuantum ML-DSA-87
├── Segmen Sumber Daya Tertanam (Embedded Resources)
│   ├── UI Ribbon HTML5/CSS/JS (Terkompresi Zstd)
│   ├── Ikon Aplikasi & Tray Menu
│   └── Vektor Glif Font Matematika Standar
└── Tanda Tangan Digital Biner (Ed25519 & Authenticode)
```

Ketika biner dijalankan, aset dimuat langsung dari memori tanpa mengekstrak ratusan berkas sementara ke direktori `/tmp`, sehingga tahan terhadap serangan injeksi *DLL Hijacking* atau *Tmp Race Condition*.

---

## 4. Mekanisme Pembaruan In-Place (*Delta Patching*)

Untuk memperbarui aplikasi di jaringan korporat tanpa harus mengunduh ulang installer puluhan megabyte:
1. Engine [src/desktop/pure_packager.rs](file:///home/devops/widya-lang/src/desktop/pure_packager.rs) menyediakan fungsi bawaan:
   - `buat_delta_patch(biner_lama, biner_baru)`: Menghasilkan berkas `.patch` berukuran rata-rata **< 1,8 MB**.
   - `terapkan_delta_patch(biner_lama, patch)`: Memperbarui biner secara lokal secara atomik.
2. Setiap delta patch ditandatangani dengan kunci privat **Ed25519**. Biner tidak akan dieksekusi jika verifikasi kriptografi gagal (`verifikasi_tanda_tangan_ed25519`).

---

## 5. Langkah-Langkah Kompilasi Rilis

### A. Kompilasi Target Linux Lokal:
```bash
# Kompilasi biner release dengan optimasi LTO (Link-Time Optimization)
cargo build --release --bin widya

# Memaketkan distribusi mandiri WidyaOffice
widya desktop paket --platform linux --format deb --target-dir ./dist
```

### B. Kompilasi Silang Windows (.exe / .msi):
```bash
widya desktop paket --platform windows --format exe --keluaran WidyaOffice.exe
```

### C. Kompilasi Silang macOS Universal (.app / .dmg):
```bash
widya desktop paket --platform macos --format app --keluaran WidyaOffice.app
```

---

## 6. Prosedur Instalasi Lingkungan Terisolasi (*Air-Gapped*)

1. Salin berkas instalasi biner (`WidyaOffice.exe` atau `.deb`) ke flashdisk terenkripsi atau server repositori lokal.
2. Pada mesin tujuan, jalankan biner langsung:
   - Pada Windows: Cukup klik ganda `WidyaOffice.exe`. Aplikasi langsung aktif tanpa instalasi.
   - Pada Linux: `sudo dpkg -i widya-office_1.0.0_amd64.deb`
3. Seluruh fitur (termasuk OMML equation editor, Mail Merge, dan penandatanganan dokumen PQC/HSM) berfungsi **100% secara offline** tanpa membutuhkan akses internet sama sekali.
