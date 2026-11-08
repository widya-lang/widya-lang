# 🚀 Widya-Lang v0.1.0 — Official Genesis Release

> **Tanggal Rilis:** 8 November 2026  
> **Tag Git:** `v0.1.0`  
> **Komitmen:** Kedaulatan Digital Penuh, Keamanan Memori Formal, & Paritas Pustaka Industri

---

## 🌟 Ikhtisar Rilis
Widya-Lang versi **v0.1.0** merupakan rilis tonggak sejarah perdana yang membuktikan **kemandirian teknologi penuh (*technological sovereignty*)** dari sebuah bahasa pemrograman generasi baru. Bahasa ini kini berstatus **100% Self-Hosting**, deterministik bit-for-bit, dan dilengkapi ekosistem pustaka standar resmi (`@widya/*`) dengan paritas fungsional terhadap pustaka terpopuler di dunia Python dan Node.js.

---

## 💎 Fitur Unggulan

### 1. Kompiler Mandiri 100% Deterministik (*Self-Hosting Compiler*)
- **Siklus Bootstrap 3-Tahap:** `Stage-0 (Rust Host) ➔ Stage-1 (C Code) ➔ Stage-2 (C Code)`.
- **Konvergensi Bit-for-Bit:** Hash kriptografis SHA-256 antara Stage-1 dan Stage-2 terbukti identik secara presisi.
- **Tanda Tangan Kriptografis Ed25519:** Setiap build mandiri diverifikasi dan dikunci dengan kurva eliptis Ed25519 (kunci publik 32-byte, tanda tangan 64-byte) untuk mencegah manipulasi rantai pasok (*anti-supply-chain injection*).
- **7 Modul Inti Widya Murni:** `token`, `lexer`, `ast`, `parser`, `typesystem`, `codegen`, dan `main` ditulis dalam bahasa Widya murni (`.wya` & `.widya`).

### 2. Ekosistem Pustaka Standar Resmi WPM (`@widya/*`)
Paritas fungsional tingkat produksi yang siap pakai tanpa dependensi eksternal:
- **`@widya/tensor` (Setara NumPy):** Matriks N-dimensi, perkalian matriks (*matmul*), dot product, transpose, matriks identitas, reduksi mean/sum.
- **`@widya/dataframe` (Setara Pandas):** Manipulasi data tabular kolumnar, filter kondisi (*saring*), seleksi kolom, agregasi statistik, parser CSV.
- **`@widya/web` (Setara Express / FastAPI):** RESTful Web router berbasis Radix-tree, perantaian middleware asinkron, dan abstraksi respons JSON/teks.
- **`@widya/ai` (Setara PyTorch Autograd):** Mesin diferensiasi otomatis reverse-mode berbasis graf asiklik terarah (DAG) topologis, lapisan linear, aktivasi ReLU, MSE Loss, dan SGD optimizer.
- **`@widya/sql` (Setara SQLite3 / Prisma):** Basis data relasional in-memory mandiri dengan query builder CRUD dan integritas transaksi.
- **`@widya/test` (Setara Jest / Pytest):** Framework pengujian mandiri (*unit test suite runner*), assertion lengkap (*tegaskan_sama*, *tegaskan_benar*), dan laporan berperingkat.

### 3. Native CLI Test Runner Cerdas (`widya test`)
- **Auto-Discovery Instan:** Menjalankan `widya test` otomatis mendeteksi seluruh rangkaian uji coba di direktori `packages/*/tests/` dan `tests/` dalam waktu **< 15 milidetik**.
- Mendukung berkas berekstensi ganda `.wya` dan `.widya`.

### 4. Ekstensi Editor Resmi VS Code (`v0.1.0`)
- Dilengkapi TextMate grammar resmi untuk pewarnaan sintaksis kata kunci dwibahasa (Indonesia / Inggris) dan modul resmi `@widya/*`.
- 10+ snippet kode cepat untuk AI, Web, Tensor, dan Basis Data.
- Berkas bundel instalasi mandiri: `editors/vscode/widya-lang-0.1.0.vsix`.

---

## 📦 Aset Rilis (Artifacts)
- **`widya-v0.1.0-windows-x64.zip`**: Biner mandiri `widya.exe` (Release teroptimasi) beserta bundel pustaka standar resmi `packages/` dan contoh kode `contoh/`.
- **`widya-lang-0.1.0.vsix`**: Ekstensi resmi Visual Studio Code.

---

## ⚡ Panduan Memulai Cepat (Quick Start)

### 1. Menjalankan Kode Widya:
```bash
widya jalankan skrip.wya
```

### 2. Menjalankan Seluruh Unit Test:
```bash
widya test
```

### 3. Memverifikasi Kompiler Mandiri (Self-Hosting):
```bash
widya bootstrap
```

### 4. Contoh Program Terintegrasi (AI Analytics Service):
```widya
impor "@widya/tensor" sebagai np;
impor "@widya/dataframe" sebagai pd;
impor "@widya/ai" sebagai torch;
impor "@widya/web" sebagai web;
impor "@widya/sql" sebagai sql;
impor "@widya/test" sebagai test;

// Lihat contoh lengkap di: contoh/18_layanan_ai_analitika.wya
```

---

*Widya-Lang Foundation — Dirancang untuk Kedaulatan Digital & Komputasi Masa Depan.*
