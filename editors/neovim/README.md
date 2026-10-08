# Dukungan Resmi Widya-Lang untuk Neovim / Vim

Paket plugin resmi untuk teks editor **Neovim** dan **Vim** dengan penyorotan sintaksis tingkat lanjut berbasis **Tree-sitter**, indentasi otomatis, pelipatan kode (*code folding*), deteksi tipe berkas, dan integrasi Language Server Protocol (LSP).

---

## 📦 Pemasangan & Konfigurasi

### 1. Integrasi Tree-sitter (`nvim-treesitter`)
Tambahkan parser kustom Widya pada `nvim-treesitter` di dalam `init.lua`:

```lua
local parser_config = require("nvim-treesitter.parsers").get_parser_configs()
parser_config.widya = {
  install_info = {
    url = "https://github.com/widya-lang/tree-sitter-widya",
    files = { "src/parser.c" },
    branch = "main",
  },
  filetype = "widya",
}
```

Kemudian salin queries resmi ke Neovim:
```bash
mkdir -p ~/.config/nvim/after/queries/widya
cp -r editors/neovim/queries/widya/* ~/.config/nvim/after/queries/widya/
```

### 2. Menggunakan `lazy.nvim`
Tambahkan ke konfigurasi plugin Anda:
```lua
{
  "Bakulcoding/widya-lang",
  ft = "widya",
  config = function()
    require("widya.lsp").setup({
      cmd = { "widya", "lsp" }
    })
  end
}
```

### 3. Menggunakan `vim-plug` (Vim / Neovim)
```vim
Plug 'Bakulcoding/widya-lang', { 'rtp': 'editors/neovim' }
```

### 4. Pemasangan Manual Berkas Vim Tradisional
```bash
cp -r editors/neovim/ftdetect ~/.config/nvim/
cp -r editors/neovim/syntax   ~/.config/nvim/
cp -r editors/neovim/ftplugin ~/.config/nvim/
cp -r editors/neovim/lua      ~/.config/nvim/
```

---

## ✨ Fitur Lengkap
- **Tree-sitter Semantic Highlighting**: Penyorotan kata kunci dwibahasa (Nusantara & Internasional), tipe data, dan identifier dengan akurasi AST.
- **Deteksi berkas otomatis**: Ekstensi `.wya`, `.widya`, `.wd`.
- **Indentasi Cerdas & Code Folding**: Menyesuaikan blok `{ ... }`, struktur, enum, dan pencocokan pola.
- **Language Injections**: Penyorotan SQL di dalam panggilan `db.exec("SELECT ...")` dan shader WGSL.
- **Integrasi LSP Bawaan (`widya lsp`)**: Auto-completion, hover type info, go-to-definition, dan diagnostics linter statis.
