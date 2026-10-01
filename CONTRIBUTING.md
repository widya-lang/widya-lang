# 🤝 Panduan Kontribusi (Contributing Guide) — Widya-Lang

Terima kasih atas ketertarikan Anda untuk berkontribusi pada **Widya-Lang**! Ekosistem ini dibangun untuk menjadi bahasa pemrograman modern, aman, dan berkinerja tinggi yang memadukan keindahan sintaks dwibahasa (Nusantara & Internasional) dengan arsitektur web dan desktop terpadu.

Setiap kontribusi—mulai dari perbaikan dokumentasi, pelaporan bug, pengoptimalan performa, hingga penambahan fitur baru—sangat dihargai oleh komunitas.

---

## 🧭 Cara Berkontribusi

Anda dapat berkontribusi dalam berbagai bentuk:
1. **Melaporkan Masalah (*Issues*)**: Menemukan bug atau memiliki ide perbaikan sintaks/fitur.
2. **Memperbaiki Dokumentasi**: Menambah contoh kode, memperbaiki kesalahan ketik, atau memperjelas tutorial.
3. **Mengembangkan Fitur / Perbaikan Bug**: Mengirimkan *Pull Request* (PR) pada repositori inti.
4. **Membangun Pustaka & Ekosistem**: Membuat modul `.wya` baru untuk keperluan sains, web, atau desktop.

---

## 🛠️ Menyiapkan Lingkungan Pengembangan Lokal

### 1. Prasyarat
Pastikan sistem Anda telah terpasang:
- **Rust Toolchain** (versi `1.75.0` atau yang lebih baru):
  ```bash
  rustc --version
  cargo --version
  ```
- **Git**: Untuk manajemen cabang dan repositori.
- **PowerShell** (Windows) atau **Bash** (Linux/macOS).

### 2. Kloning Repositori
```bash
git clone https://github.com/widya-lang/widya.git
cd widya
```

### 3. Kompilasi & Pengujian Awal
Pastikan repositori dapat dikompilasi secara lokal:
```bash
cargo build
cargo test
```

---

## 🧪 Menjalankan Rangkaian Pengujian (Test Suites)

Widya-Lang memiliki beberapa rangkaian pengujian terotomasi untuk menjaga stabilitas *zero-regression*:

1. **Pengujian Unit & Kompilator Dasar**:
   ```bash
   cargo test --lib
   ```
2. **Pengujian Framework Desktop**:
   ```bash
   cargo test --test desktop_tests
   ```
3. **Pengujian Ekosistem & Pustaka Database**:
   ```bash
   cargo test --test ecosystem_tests
   ```
4. **Pengujian Master Web Framework (6 Tier)**:
   ```powershell
   # Windows PowerShell
   powershell -ExecutionPolicy Bypass -File scratch\widya_web_framework\test_all_tiers_master.ps1
   ```

Pastikan **seluruh pengujian lulus 100%** sebelum mengajukan *Pull Request*.

---

## 🌿 Strategi Percabangan (*Git Branching Strategy*)

1. Fork repositori resmi ke akun GitHub Anda.
2. Buat cabang baru dari `main` dengan format penamaan deskriptif:
   - Fitur baru: `feat/nama-fitur` (contoh: `feat/widya-mobile-scaffold`)
   - Perbaikan bug: `fix/deskripsi-masalah` (contoh: `fix/esc-pos-alignment`)
   - Dokumentasi: `docs/nama-dokumen` (contoh: `docs/tutorial-ipc`)
3. Lakukan commit dengan pesan terstruktur (*Conventional Commits*):
   - `feat: menambahkan dukungan hmr pada desktop bridge`
   - `fix: memperbaiki memory leak pada worker thread web`
   - `docs: memperbarui panduan instalasi linux`
   - `test: menambah uji unit tanda tangan ed25519`

---

## 📝 Standar Penulisan Kode (Coding Guidelines)

- **Rustfmt**: Format kode Rust Anda secara konsisten:
  ```bash
  cargo fmt --all
  ```
- **Clippy**: Pastikan tidak ada peringatan (*linter warnings*):
  ```bash
  cargo clippy -- -D warnings
  ```
- **Bahasa Kode**:
  - Kode logika internal (*Rust engine*) menggunakan konvensi standar Rust (Bahasa Inggris untuk struktur internal, idiomatic Rust).
  - API publik untuk pengguna bahasa Widya-Lang mendukung penamaan dwibahasa (Bahasa Indonesia & Bahasa Inggris) secara harmonis.
- **Uji Baru**: Setiap penambahan fungsi atau modul baru wajib disertai pengujian unit di folder `tests/` atau contoh script di `contoh/`.

---

## 🚀 Mengajukan Pull Request (PR)

1. Pastikan cabang Anda telah sinkron dengan cabang `main` terbaru (*rebase*).
2. Dorong (*push*) cabang Anda ke fork GitHub Anda.
3. Buka halaman repositori resmi dan klik tombol **New Pull Request**.
4. Isi deskripsi PR dengan jelas:
   - Ringkasan perubahan yang dilakukan.
   - Motivasi / isu yang diselesaikan (misal: `Fixes #42`).
   - Bukti pengujian yang telah dijalankan secara lokal.
5. Tim inti akan meninjau (*code review*) PR Anda dan memberikan umpan balik konstruktif sebelum penggabungan (*merge*).

---

## 📜 Kode Etik Komunitas

Kami menjunjung tinggi budaya kolaborasi yang saling menghormati, ramah, dan profesional. Dengan berpartisipasi dalam proyek ini, Anda menyetujui untuk mematuhi ketentuan [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

Terima kasih atas dedikasi dan kontribusi Anda memajukan bahasa pemrograman Widya-Lang!
