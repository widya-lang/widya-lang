# Dukungan Resmi Widya-Lang untuk Helix Editor

Integrasi bahasa, grammar Tree-sitter, dan Language Server Protocol (LSP) untuk [Helix Editor](https://helix-editor.com/).

---

## 🚀 Cara Pemasangan

### 1. Daftarkan Bahasa & Grammar di Helix

Tambahkan isi berkas `languages.toml` ini ke konfigurasi Helix Anda:

```bash
mkdir -p ~/.config/helix
cat editors/helix/languages.toml >> ~/.config/helix/languages.toml
```

### 2. Salin Query Tree-sitter untuk Syntax Highlighting & Indentasi

```bash
mkdir -p ~/.config/helix/runtime/queries/widya
cp -r editors/helix/runtime/queries/widya/* ~/.config/helix/runtime/queries/widya/
```

### 3. Kompilasi Grammar di Helix

Jalankan perintah berikut untuk mengunduh dan membangun grammar Tree-sitter:

```bash
hx --grammar fetch
hx --grammar build
```

Pastikan compiler `widya` sudah tersedia di dalam `$PATH` sistem Anda. Helix akan otomatis meluncurkan `widya lsp` setiap kali Anda membuka berkas `.wya`, `.widya`, atau `.wd`.
