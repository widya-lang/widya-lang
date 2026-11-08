# Panduan Desain Antarmuka Pengguna & UX (Ribbon System) 🎨

> **Standar Desain Antarmuka WidyaOffice Desktop**  
> *Mica Visual Aesthetics · Dynamic Ribbon Toolbar · Ergonomi Produktivitas Perkantoran*

---

## 1. Filosofi Estetika & Desain Sistem

WidyaOffice Desktop mengadopsi bahasa desain **Sovereign Fluent & Mica Dynamic**:
- **Efek Transparansi Berkelas (Mica & Acrylic Blur):** Panel atas Ribbon dan bilah judul (*custom titlebar*) membaur secara halus dengan wallpaper desktop sistem operasi pengguna.
- **Konsistensi Visual Paritas Industri:** Tata letak tombol, ikon, dan hierarki menu mengikuti konvensi internasional suite pengolah kata terdepan sehingga pengguna dapat langsung produktif tanpa kurva belajar yang curam.
- **Dukungan Monitor Beresolusi Tinggi (DPI Scaling Murni):** Render ikon vektor dan tipografi bebas buram pada layar 1080p, 2K, 4K, hingga 8K.

---

## 2. Struktur Tab & Komponen Ribbon Toolbar

Ribbon diatur dalam 6 tab fungsional utama:

### A. Tab "Beranda" (Home)
- **Papan Klip:** Tempel (*Paste*), Potong (*Cut*), Salin (*Copy*), Format Painter.
- **Font & Tipografi:** Pemilihan Jenis Huruf (Inter, Roboto, DejaVu Serif), Ukuran Font, Bold (`Ctrl+B`), Italic (`Ctrl+I`), Underline (`Ctrl+U`), Coretan, Subskrip/Superskrip, Warna Teks, Warna Stabilo.
- **Paragraf:** Rata Kiri (`Ctrl+L`), Rata Tengah (`Ctrl+E`), Rata Kanan (`Ctrl+R`), Rata Kanan-Kiri/Justify (`Ctrl+J`), Bullet List, Numbered List, Indentasi, Spasi Baris (1.0, 1.15, 1.5, 2.0).
- **Gaya (Styles):** Judul 1 (H1), Judul 2 (H2), Judul 3 (H3), Teks Normal, Kutipan, Kode.

### B. Tab "Sisipkan" (Insert)
- **Halaman:** Halaman Sampul, Halaman Kosong, Pemisah Halaman (*Page Break*).
- **Tabel:** Grid visual pemilih baris & kolom (hingga 10x8), sisipkan tabel kustom.
- **Ilustrasi:** Gambar lokal (.png, .jpg, .svg), Diagram Alur Arsitektur (WidyaDiagram).
- **Simbol & Persamaan:**
  - 🧮 **Persamaan Matematika (`Alt + =`):** Membuka dialog persamaan OMML LaTeX/Visual.
  - **Simbol Khusus:** Mata uang (Rp, $, €), simbol teknis (©, ®, ™, §).

### C. Tab "Tata Letak" (Layout)
- **Batas Margin:** Normal (2.54 cm), Sempit (1.27 cm), Lebar (5.08 cm), Kustom.
- **Orientasi:** Potret / Lanskap.
- **Ukuran Kertas:** A4, Letter, Legal, Folio/F4.
- **Kolom:** Satu Kolom, Dua Kolom, Tiga Kolom.

### D. Tab "Surat" (Mailings)
- **Mulai Gabung Surat:** Surat Personalisasi, Label, Amplop, Direktori.
- **Pilih Penerima:** Muat Berkas CSV/JSON, Hubungkan Database **WidyaBase**.
- **Sisipkan Bidang Gabungan:** Dropdown daftar kolom data (`{{nama}}`, `{{alamat}}`, dll).
- **Pratinjau Hasil:** Navigasi maju-mundur rekaman data (*live record viewer*).
- **Selesaikan & Gabungkan:** Ekspor berkas mandiri, gabung jadi 1 berkas, atau cetak massal.

### E. Tab "Tinjau" (Review)
- **Lacak Perubahan (Track Changes):** Aktifkan/Nonaktifkan pencatatan riwayat revisi berbasis CRDT.
- **Komentar:** Tambah komentar berutas (*threaded comments*) pada seleksi teks.
- **Keamanan & Sandi:** Tanda tangani dokumen dengan HSM / Post-Quantum ML-DSA-87.

### F. Tab "Tampilan" (View)
- **Mode Tampilan:** Tata Letak Cetak (*Print Layout*), Layar Penuh (*Distraction-Free Kiosk*), Tata Letak Web.
- **Alat Bantu:** Penggaris/Mistar (*Ruler*), Garis Kisi (*Gridlines*), Panel Navigasi Dokumen.
- **Zooming:** 50%, 75%, 100%, 150%, 200%, Pas Lebar Halaman (*Page Width*).

---

## 3. Pintasan Papan Ketik Standar (*Keyboard Shortcuts*)

| Pintasan Tombol | Aksi yang Dijalankan |
| :--- | :--- |
| `Ctrl + N` | Buat dokumen baru |
| `Ctrl + O` | Buka dokumen dari disk |
| `Ctrl + S` | Simpan dokumen saat ini |
| `Ctrl + Shift + S` | Simpan sebagai berkas baru (*Save As*) |
| `Ctrl + P` | Cetak dokumen atau ekspor ke PDF |
| `Ctrl + Z` | Batalkan aksi terakhir (*Undo* via CRDT) |
| `Ctrl + Y` | Ulangi aksi terakhir (*Redo*) |
| `Alt + =` | Buka dialog penyisipan rumus matematika OMML |
| `F7` | Pemeriksaan ejaan & tata bahasa Indonesia/Inggris |
| `F11` | Beralih ke Mode Layar Penuh Tanpa Gangguan (*Focus Kiosk*) |
