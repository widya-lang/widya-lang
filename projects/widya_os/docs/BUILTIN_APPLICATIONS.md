# Spesifikasi Aplikasi Bawaan WidyaOS (Built-in App Suite) 💼

> **Katalog Perangkat Lunak Produktivitas Terintegrasi Siap Pakai**  
> *Zero Additional Installation Required · Integrasi Ekosistem Murni · Zero Foreign Dependency*

---

## 1. Pendahuluan

Salah satu kelemahan sistem operasi Linux umum (seperti Arch Linux atau Debian minimal) adalah pengguna harus menghabiskan berjam-jam untuk menginstal driver, office suite, file manager, dan pengatur tema sebelum komputer dapat dipakai bekerja. Sebaliknya, Windows sering kali membundel bloatware dan iklan komersial.

**WidyaOS** hadir dengan filosofi **"Batteries-Included & Ready to Work"**:
Begitu instalasi selesai, seluruh instrumen kerja produktivitas esensial telah terpasang, terkonfigurasi, dan teroptimasi secara sempurna tanpa satu pun aplikasi sampah (*zero bloatware*).

---

## 2. Katalog Aplikasi Bawaan Unggulan

---

### A. WidyaOffice Desktop Suite
*Suite perkantoran berdaulat generasi baru yang terintegrasi langsung dengan arsitektur [projects/widya_office_desktop/](file:///home/devops/widya-lang/projects/widya_office_desktop/).*

```
┌────────────────────────────────────────────────────────────────────────┐
│                        WIDYAOFFICE DESKTOP SUITE                       │
├───────────────┬────────────────┬───────────────────┬───────────────────┤
│  WidyaWriter  │   WidyaCalc    │   WidyaImpress    │     WidyaPDF      │
│ (Dokumen Teks)│  (Spreadsheet) │   (Presentasi)    │(Viewer & Signer)  │
└───────────────┴────────────────┴───────────────────┴───────────────────┘
```

1. **WidyaWriter (Alternatif Microsoft Word):**
   - Format Berkas: `.docx`, `.odt`, `.md`, `.pdf`.
   - Fitur Inti: Formula matematika **OMML `<m:oMath>`** berbasis LaTeX dan palet visual, wizard **Bulk Mail Merge** massal (> 2.500 surat/detik), pelacakan revisi CRDT tanpa konflik.
2. **WidyaCalc (Alternatif Microsoft Excel):**
   - Format Berkas: `.xlsx`, `.ods`, `.csv`.
   - Fitur Inti: Mesin kalkulasi formula sel berkecepatan tinggi, pemformatan tabel dinamis, sparklines, dan koneksi query langsung ke database **WidyaBase**.
3. **WidyaImpress (Alternatif Microsoft PowerPoint):**
   - Format Berkas: `.pptx`, `.odp`.
   - Fitur Inti: Slide presentasi vektor, kartu metrik KPI bisnis, bentuk diagram alur, dan transisi layar penuh 60 FPS.
4. **WidyaPDF (Alternatif Adobe Acrobat):**
   - Format Berkas: `.pdf`.
   - Fitur Inti: Perenderan vektor instan, pembubuhan cap air, bookmark cerdas, dan **tanda tangan digital anti-kuantum (PQC ML-DSA-87 / HSM PKCS#11)**.

---

### B. WidyaExplorer (Pengelola Berkas & Penyimpanan)
*File Manager berkecepatan tinggi pengganti Windows Explorer dan macOS Finder.*

```
┌────────────────────────────────────────────────────────────────────────┐
│  [<] [>]  /home/devops/Dokumen/Laporan_Keuangan_2026/        [Cari...] │
├──────────────┬───────────────────────────────────────────┬─────────────┤
│ PINTASAN     │ Nama Berkas              Ukuran    Diubah │ PRATINJAU   │
│ 📁 Dokumen   │ 📄 Neraca_Q3.xlsx        42.1 KB   Hari ini│ [Live Quick │
│ 📁 Unduhan   │ 📝 Perjanjian_Kerja.docx 88.4 KB   Kemarin │  Look Space]│
│ 📁 Gambar    │ 📊 Ringkasan_KPI.pptx    1.2 MB    25 Okt  │             │
│ 💾 SSD Utama │ 📕 Audit_Sistem.pdf      450 KB    20 Okt  │ Neraca Q3   │
│ 🔒 Drive PQC │                                           │ Total Aktiva│
└──────────────┴───────────────────────────────────────────┴─────────────┘
```

- **Fitur Unggulan:**
  - **QuickLook Pratinjau Instan (Spasi):** Tekan tombol spasi pada berkas DOCX, XLSX, PDF, gambar, atau audio untuk melihat isi berkas secara instan tanpa perlu membuka aplikasi terkait.
  - **Pencarian Cepat Bawaan (< 5 ms):** Memanfaatkan pengindeksan lokal untuk mencari 1.000.000 berkas di seluruh disk dalam hitungan milidetik.
  - **Tab Navigasi & Split Dual-Pane:** Buka beberapa tab folder atau bagi layar menjadi dua panel (kiri dan kanan) untuk memudahkan pemindahan berkas dalam jumlah banyak.
  - **Integrasi Partisi Terenkripsi:** Pembukaan partisi brankas rahasia berbasis hardware token HSM secara langsung dari bilah sisi (*sidebar*).

---

### C. WidyaTerminal (Konsol Sistem Berakselerasi GPU)
*Terminal modern pengganti Command Prompt / GNOME Terminal.*

- **Fitur Unggulan:**
  - **Rendering Glif Berakselerasi GPU:** Perenderan teks hingga 120 FPS tanpa lag menggunakan pipeline Vulkan / OpenGL.
  - **Ligatur Tipografi Indah:** Dukungan penuh untuk font monospace modern (*JetBrains Mono, Fira Code*), perenderan simbol matematis, dan warna 24-bit TrueColor.
  - **Split Panes & Tab:** Membagi jendela terminal secara horizontal dan vertikal (`Ctrl + Shift + O` / `Ctrl + Shift + E`).
  - **Asisten AI Bawaan (WidyaAI Copilot):** Tekan `Ctrl + Spasi` di baris perintah untuk meminta bantuan AI menerjemahkan perintah bahasa alami menjadi perintah shell yang aman (*misal: "carikan file pdf di folder dokumen yang dibuat minggu lalu"*).

---

### D. WidyaSettings (Pusat Kendali Sistem)
*Pengaturan terpadu untuk mengelola seluruh aspek perangkat keras dan antarmuka.*

```
┌────────────────────────────────────────────────────────────────────────┐
│                          PENGATURAN WIDYAOS                            │
├──────────────┬─────────────────────────────────────────────────────────┤
│ 🎭 Tampilan  │ PERSONA ANTARMUKA DESKTOP                               │
│ 🖥️ Layar     │ [ Windows Style ]   [ macOS Style ]   [ Ubuntu Style ]  │
│ 🔊 Suara     │                                                         │
│ 🌐 Jaringan  │ TEMA WARNA & EFEK                                       │
│ 🔒 Keamanan  │ [x] Efek Kaca Mica Dark    [x] Animasi Transisi 120 FPS │
│ 👥 Akun      │ Aksen Warna: [ Merah Putih ] [ Emas ] [ Emerald ]       │
└──────────────┴─────────────────────────────────────────────────────────┘
```

- **Fitur Unggulan:**
  - **Pemilih Persona 1-Klik:** Beralih antara gaya Windows, macOS, dan Ubuntu secara langsung.
  - **Skala Tampilan Fraksional (Hi-DPI Scaling):** 100%, 125%, 150%, 175%, 200% untuk layar laptop 2K/4K/OLED.
  - **Pengelola Kunci Kriptografi:** Melihat sertifikat digital, status modul keamanan fisik HSM PKCS#11, dan pengaturan enkripsi disk.

---

### E. WidyaStore (Pusat Ekosistem Aplikasi Berdaulat)
*Manajer paket aplikasi modern berbasis WebAssembly Component (WASI 0.2).*

- **Fitur Unggulan:**
  - **Aplikasi Terisolasi (Zero-Privilege Sandbox):** Aplikasi berjalan di lingkungan terisolasi mikro, sehingga tidak bisa membaca berkas pribadi pengguna tanpa izin eksplisit.
  - **Instalasi Portabel 1-Klik:** Mengunduh dan memasang aplikasi berdaulat berukuran kecil (rata-rata 2–15 MB) secara instan.
  - **Repositori Tertutup (Air-Gapped Repository):** Instansi dapat membuat repositori aplikasi internal di server intranet lokal tanpa terhubung ke internet.

---

### F. WidyaMail (Surel Berdaulat @widya.email Terintegrasi)
*Klien surel desktop terintegrasi otomatis dengan akun resmi `@widya.email`.*

- **Fitur Unggulan:**
  - **Penerbitan Akun Seketika:** Setiap pendaftaran akun WidyaOS langsung mendapatkan alamat surel `username@widya.email` dalam waktu < 1,5 detik.
  - **Buka & Edit Lampiran Langsung:** Lampiran dokumen Word/Excel/PowerPoint/PDF dapat diedit langsung di WidyaOffice dan dibalas tanpa perlu unduh/unggah manual.
  - **Enkripsi End-to-End PQC (ML-KEM-768):** Isi email terlindungi secara matematis dari penyadapan pihak ketiga maupun pengelola server.
  - **Kapasitas 1 TB Terpadu:** Kuota surel dan lampiran berkas menyatu dalam 1 TB penyimpanan cloud WidyaWorkspace.
