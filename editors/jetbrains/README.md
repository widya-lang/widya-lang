# Dukungan Resmi Widya-Lang untuk JetBrains IDEs

Plugin resmi bahasa pemrograman **Widya-Lang** untuk rangkaian IDE JetBrains:
- **IntelliJ IDEA** (Ultimate & Community)
- **WebStorm**
- **PyCharm** (Professional & Community)
- **CLion**
- **RustRover**
- **GoLand**

---

## 📦 Pemasangan

### 1. Pemasangan Berkas Plugin Langsung (`.zip` / `.jar`)
1. Buka IDE JetBrains Anda.
2. Masuk ke **Settings** / **Preferences** (`Ctrl + Alt + S` di Linux/Windows atau `Cmd + ,` di macOS).
3. Pilih **Plugins** > ikon roda gigi ⚙️ > **Install Plugin from Disk...**.
4. Pilih arsip plugin `widya-jetbrains.zip` yang telah dikompilasi atau pilih direktori `editors/jetbrains/`.
5. Restart IDE.

### 2. Menggunakan TextMate Bundles Bawaan JetBrains
Jika Anda ingin penyorotan instan tanpa mengompilasi plugin:
1. Masuk ke **Settings** > **Editor** > **TextMate Bundles**.
2. Klik tanda tambah **+** dan arahkan ke direktori `editors/jetbrains/src/main/resources/syntaxes/`.
3. Buka tab **File Types**, pastikan ekstensi `*.wya`, `*.widya`, dan `*.wd` dipetakan ke bundle `Widya`.

---

## ✨ Fitur
- **Penyorotan Dwibahasa**: Mendukung kata kunci Nusantara (`fungsi`, `misal`, `tetap`, `jika`, `kembalikan`) dan Internasional (`fn`, `let`, `const`, `if`, `return`).
- **Komentar Otomatis**: Shortcut `Ctrl + /` (Line Comment) dan `Ctrl + Shift + /` (Block Comment).
- **Deteksi Otomatis**: Ikon berkas dan asosiasi tipe berkas `.wya`.
- **Integrasi LSP**: Kompatibel dengan fitur JetBrains LSP support (IntelliJ IDEA Ultimate, CLion, RustRover) untuk menghubungkan `widya lsp`.
