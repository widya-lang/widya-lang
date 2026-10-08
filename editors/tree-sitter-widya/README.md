# Official Tree-sitter Grammar for Widya-Lang (`tree-sitter-widya`)

Grammar Tree-sitter resmi untuk bahasa pemrograman Widya-Lang, mendukung penyorotan sintaksis tingkat lanjut (*semantic highlighting*), pelipatan kode (*code folding*), indentasi cerdas (*smart indent*), dan inspeksi cakupan (*scoping/locals*).

## Fitur Utama

- **Sintaksis Dwibahasa Penuh**: Mendukung kata kunci **Nusantara** (`fungsi`, `misal`, `tetap`, `jika`, `selain_itu`, `kembalikan`, `cocokkan`, dll.) maupun **Internasional** (`fn`, `let`, `const`, `if`, `else`, `return`, `match`, dll.).
- **Tipe Data & Generik**: Penyorotan tipe data bawaan (`Int`, `Float`, `Teks`, `Bolean`), inferensi Hindley-Milner, parameter generik `<T, U>`, dan tuple/array.
- **Dekorator & Atribut**: Dukungan anotasi metadata `#[profil("...")]`, `#[uji]`, `#[inline]`.
- **Integrasi Universal**: Siap digunakan di **Neovim** (via `nvim-treesitter`), **Helix Editor**, **Zed**, **Emacs**, dan GitHub linguist.

## Instalasi & Kompilasi

Pastikan Anda telah memasang `tree-sitter-cli` dan Node.js:

```bash
npm install
npx tree-sitter generate
npx tree-sitter test
```

## Konfigurasi Editor

### 1. Neovim (`nvim-treesitter`)

Tambahkan parser kustom di konfigurasi Lua Anda (`init.lua`):

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

Salin berkas queries ke `~/.config/nvim/after/queries/widya/`:
- `highlights.scm`
- `indents.scm`
- `folds.scm`
- `locals.scm`

### 2. Helix Editor

Simpan konfigurasi berikut di `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "widya"
scope = "source.widya"
injection-regex = "widya"
file-types = ["wya", "widya", "wd"]
roots = ["widya.toml", ".git"]
comment-token = "//"
indent = { tab-width = 4, unit = "    " }
language-servers = [ "widya-lsp" ]

[[grammar]]
name = "widya"
source = { git = "https://github.com/widya-lang/tree-sitter-widya", rev = "main" }
```

Salin query `highlights.scm` ke `~/.config/helix/runtime/queries/widya/highlights.scm`.
