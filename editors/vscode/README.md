# Widya-Lang Official Support for Visual Studio Code

Ekstensi resmi Visual Studio Code untuk bahasa pemrograman **Widya-Lang**.

Ekstensi ini menyediakan dukungan penyorotan sintaks (*syntax highlighting*), cuplikan kode (*code snippets*), konfigurasi braket cerdas, dan integrasi Language Server Protocol (LSP) untuk berkas `.wya` dan `.widya`.

---

## ✨ Fitur Utama

- 🎨 **Penyorotan Sintaks Penuh**: Mendukung kata kunci dwibahasa (Nusantara & Internasional: `fungsi`, `misal`, `jika`, `kembalikan`, `impor`, `struktur`, `pilihan`, `cocokkan`, dll.).
- 💡 **Cuplikan Kode Cerdas (Snippets)**:
  - `fungsi` -> Template pembuatan fungsi.
  - `jika` / `jikala` -> Template percabangan kondisional.
  - `untuk` -> Perulangan koleksi.
  - `desktopapp` -> Scaffold otomatis aplikasi desktop WidyaDesktop.
- ⚡ **Integrasi Berkas**: Mengenali ekstensi berkas `.wya`, `.widya`, dan `.wd`.
- ⚙️ **Dukungan LSP & DAP**: Terintegrasi dengan binary LSP bawaan compiler (`widya lsp`) dan debugger (`widya dap`).

---

## 🚀 Cara Pemasangan

### Dari Visual Studio Marketplace:
Cari **"Widya-Lang"** di tab Extensions (`Ctrl+Shift+X` / `Cmd+Shift+X`), lalu klik **Install**.

### Pemasangan Manual dari Paket VSIX:
1. Unduh berkas `widya-lang-1.0.0.vsix`.
2. Buka menu Extensions di VS Code -> klik ikon titik tiga `...` di pojok kanan atas -> pilih **Install from VSIX...**.
3. Pilih berkas `widya-lang-1.0.0.vsix`.

---

## 📄 Lisensi
Dirilis di bawah lisensi [MIT License](LICENSE).
Copyright (c) 2026 Tim Pengembang Widya-Lang.
