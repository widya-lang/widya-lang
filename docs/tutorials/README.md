# Tutorial & Panduan PDF Widya-Lang

Direktori ini berisi berkas panduan, lembar contekan (*cheatsheet*), dan materi tutorial resmi **Widya-Lang** yang telah dikompilasi secara *native* ke format dokumen PDF Standar ISO 32000-1 (%PDF-1.7) menggunakan mesin **WidyaOffice**.

---

## Berkas Panduan Berdasarkan Level

### 1. Level Dasar (Pemula) — Edisi Komprehensif Cetak (~229 Halaman)
- **PDF Dokumen**: [`01_TUTORIAL_LEVEL_DASAR.pdf`](file:///home/devops/widya-lang/docs/tutorials/01_TUTORIAL_LEVEL_DASAR.pdf) (229 Halaman, Tata Letak Halaman Ganjil / *Recto Pages*, Vektor Murni ISO 32000-1)
- **Sumber Markdown Induk**: [`01_TUTORIAL_LEVEL_DASAR.md`](file:///home/devops/widya-lang/docs/tutorials/01_TUTORIAL_LEVEL_DASAR.md)
- **Berkas Markdown per Bab**: Tersedia 45 berkas mandiri di direktori [`docs/tutorials/level_dasar/`](file:///home/devops/widya-lang/docs/tutorials/level_dasar/):
  - `00_PENGANTAR.md`
  - `BAB_01_FILOSOFI_DAN_ARSITEKTUR.md` s/d `BAB_40_BANK_SOAL_TANTANGAN_DAN_SOLUSI.md` (40 Bab Pembelajaran Mendalam)
  - `LAMPIRAN_A_GLOSARIUM_KOSAKATA.md`
  - `LAMPIRAN_B_PANDUAN_PESAN_GALAT.md`
  - `LAMPIRAN_C_PETA_JALAN_PEMBELAJARAN.md`
  - `LAMPIRAN_D_KUNCI_JAWABAN_LATIHAN.md` (Kunci Jawaban Lengkap 400 Butir Soal PG Pilihan Ganda A-D)
- **Fitur Khusus Buku Cetak & Diagram**:
  - **Diagram Vektor Bergaya Mermaid.js Asli**: Blok ````mermaid ```` dirender langsung menjadi elemen vektor PDF dengan sudut membulat (*Bézier rounded corners*), palet warna khas Mermaid (lavender `#ECECFF`, ungu `#9370DB`, mint `#E8F5E9`, dan amber `#FFF8E1`), panah presisi, serta lencana label relasi.
  - **Aturan Tata Letak Halaman Ganjil (*Recto Page Break*)**: Setiap Bab, Bagian, dan Lampiran selalu dimulai pada halaman bernomor ganjil (sisi kanan buku terbuka), memenuhi standar industri penerbitan buku cetak profesional.
  - **Bank Soal Evaluasi Mandiri**: Setiap bab dilengkapi 10 soal pilihan ganda (total 400 soal) dengan kunci jawaban dan pembahasan komprehensif di Lampiran D.

### 2. Level Tingkat Lanjut (Menengah & Mahir)
- **PDF Dokumen**: [`02_TUTORIAL_LEVEL_LANJUT.pdf`](file:///home/devops/widya-lang/docs/tutorials/02_TUTORIAL_LEVEL_LANJUT.pdf)
- **Sumber Markdown**: [`02_TUTORIAL_LEVEL_LANJUT.md`](file:///home/devops/widya-lang/docs/tutorials/02_TUTORIAL_LEVEL_LANJUT.md)
- **Cakupan Materi**:
  - Sistem inferensi tipe Hindley-Milner dan fungsi generik (`<T>`).
  - Algebraic Data Types (ADT `pilihan`) & *Exhaustive Pattern Matching* (`cocokkan`).
  - Pemrograman Berorientasi Objek berbasis `struktur`, metode, dan referensi diri `ini`.
  - Konkurensi tingkat OS murni: `buat_utas`, `gabung_utas`, dan penjadwalan.
  - Saluran komunikasi pesan nir-jeda (*Actor Model Channel*: `saluran`, `saluran_kirim`, `saluran_terima`).
  - Keamanan Memori Statis & *Borrow Checker* tanpa jeda *Garbage Collection* (Aliasing XOR Mutability: `&` vs `&mut`).
  - Pilar Kemandirian: *Direct Syscall* x86_64/AArch64 langsung ke kernel Linux bebas libc (`sys_write`, `sys_read`).
  - Foreign Function Interface (FFI) C ABI untuk integrasi pustaka eksternal.
  - Kriptografi berstandar Pasca-Kuantum (NIST FIPS 203: ML-KEM Kyber-768/1024 dan FIPS 204: ML-DSA Dilithium).

### 3. Level Ekosistem & Aplikasi Berdaulat
- **PDF Dokumen**: [`03_TUTORIAL_LEVEL_EKOSISTEM.pdf`](file:///home/devops/widya-lang/docs/tutorials/03_TUTORIAL_LEVEL_EKOSISTEM.pdf)
- **Sumber Markdown**: [`03_TUTORIAL_LEVEL_EKOSISTEM.md`](file:///home/devops/widya-lang/docs/tutorials/03_TUTORIAL_LEVEL_EKOSISTEM.md)
- **Cakupan Materi**:
  - Lanskap arsitektur komputasi berdaulat *multiplatform*.
  - **WidyaWeb**: Fullstack SSR, arsitektur *Islands Hydration* (0KB static JS), WebSockets real-time, dan pengerasan keamanan 6-tier.
  - **WidyaDesktop**: Sistem *windowing native* ultra-ringan (<15MB RAM), *hybrid IPC bridge* (<1ms), dan driver cetak thermal printer ESC/POS untuk kasir POS koperasi.
  - **WidyaMobile**: *Pure software canvas 2D rasterizer*, parser dan validasi standar QRIS ASPI (Bank Indonesia), serta komunikasi BLE.
  - **WidyaBase**: Basis data mandiri berkinerja tinggi multi-model (Relasional ACID B+Tree/WAL, PGWire v3.0, WidyaDocDB NoSQL, dan WidyaStore S3) dengan sintaks SQL Dwibahasa Nusantara.
  - **WidyaOffice**: Mesin konversi dokumen universal DOCX, XLSX, PPTX, PDF ISO 32000-1, EPUB, dan Tanda Tangan Elektronik (TTE) BSrE.
  - Kepatuhan Keamanan & AST Linter UU PDP No. 27/2022 untuk sensor otomatis data sensitif (NIK, KK, Rekening).
  - Studi kasus implementasi arsitektur sistem kasir dan keuangan koperasi terpadu *end-to-end*.

### 4. Ekosistem WidyaBase — Edisi Komprehensif Cetak (~236 Halaman)
- **PDF Dokumen**: [`03_BUKU_WIDYABASE.pdf`](file:///home/devops/widya-lang/docs/tutorials/03_BUKU_WIDYABASE.pdf)
- **Sumber Markdown Induk**: [`03_BUKU_WIDYABASE.md`](file:///home/devops/widya-lang/docs/tutorials/03_BUKU_WIDYABASE.md)
- **Direktori Bab Lengkap**: [`docs/tutorials/ekosistem_widyabase/`](file:///home/devops/widya-lang/docs/tutorials/ekosistem_widyabase/) (Bab 1–40, Lampiran A–D)

### 5. Ekosistem WidyaWeb — Edisi Komprehensif Cetak (~245 Halaman)
- **PDF Dokumen**: [`04_BUKU_WIDYAWEB.pdf`](file:///home/devops/widya-lang/docs/tutorials/04_BUKU_WIDYAWEB.pdf)
- **Sumber Markdown Induk**: [`04_BUKU_WIDYAWEB.md`](file:///home/devops/widya-lang/docs/tutorials/04_BUKU_WIDYAWEB.md)
- **Direktori Bab Lengkap**: [`docs/tutorials/ekosistem_widyaweb/`](file:///home/devops/widya-lang/docs/tutorials/ekosistem_widyaweb/) (Bab 1–40, Lampiran A–D)

### 6. Ekosistem WidyaDesktop — Edisi Komprehensif Cetak (~250 Halaman)
- **PDF Dokumen**: [`05_BUKU_WIDYADESKTOP.pdf`](file:///home/devops/widya-lang/docs/tutorials/05_BUKU_WIDYADESKTOP.pdf)
- **Sumber Markdown Induk**: [`05_BUKU_WIDYADESKTOP.md`](file:///home/devops/widya-lang/docs/tutorials/05_BUKU_WIDYADESKTOP.md)
- **Direktori Bab Lengkap**: [`docs/tutorials/ekosistem_widyadesktop/`](file:///home/devops/widya-lang/docs/tutorials/ekosistem_widyadesktop/):
  - `00_PENGANTAR.md`
  - `BAB_01_FILOSOFI_DAN_ARSITEKTUR_WIDYADESKTOP.md` s/d `BAB_40_PROYEK_CAPSTONE_STASIUN_KERJA_KASIR_POS.md` (40 Bab Pembelajaran Lengkap)
  - `LAMPIRAN_A_GLOSARIUM_DAN_SINTAKSIS_WIDYADESKTOP.md`
  - `LAMPIRAN_B_KATALOG_GALAT_DAN_TROUBLESHOOTING.md`
  - `LAMPIRAN_C_PARAMETER_KONFIGURASI_DAN_SECURITY_MANIFEST.md`
  - `LAMPIRAN_D_KUNCI_JAWABAN_DAN_PEMBAHASAN_LENGKAP.md` (Kunci Jawaban Lengkap 120 Soal Esai dan 400 Soal PG A–D)
- **Fitur Khusus**:
  - Arsitektur native murni ultra-ringan (< 25 MB RAM, nol Chromium/Electron).
  - Jembatan IPC berlatensi sub-milidetik dengan proteksi token kriptografis `X-Widya-IPC-Token` dan Strict CSP.
  - Driver perangkat keras ritel & industri: Printer thermal POS ESC/POS, laci kasir solenoid, timbangan serial RS-232, dan SCADA Modbus RTU CRC-16.
  - Kanvas grafis GPU native 120 FPS WGPU (DirectX 12, Metal, Vulkan).
  - Basis data tertanam lokal WidyaBase Embedded mode transaksi ACID WAL dan brankas AES-256-GCM.
  - 6 Pilar Mutakhir: AccessKit WCAG 2.2 AAA, Remote Desktop Noise Protocol E2EE, KTP-el ISO 7816 & Biometrik Minutiae, Edge On-Device AI (OCR & Whisper NPU), WASM Sandbox, dan Rantai Distribusi Lintas Platform dengan OTA Delta Patching `WIDYADELTA10`.

### 7. Ekosistem WidyaMobile — Edisi Komprehensif Cetak (~270 Halaman)
- **PDF Dokumen**: [`06_BUKU_WIDYAMOBILE.pdf`](file:///home/devops/widya-lang/docs/tutorials/06_BUKU_WIDYAMOBILE.pdf)
- **Sumber Markdown Induk**: [`06_BUKU_WIDYAMOBILE.md`](file:///home/devops/widya-lang/docs/tutorials/06_BUKU_WIDYAMOBILE.md)
- **Direktori Bab Lengkap**: [`docs/tutorials/ekosistem_widyamobile/`](file:///home/devops/widya-lang/docs/tutorials/ekosistem_widyamobile/):
  - `00_PENGANTAR.md`
  - `BAB_01_FILOSOFI_DAN_PARADIGMA_NATIVE_SOVEREIGN.md` s/d `BAB_40_PROYEK_CAPSTONE_NUSANTARAMOBILE_SUPERAPP.md` (40 Bab Pembelajaran Lengkap)
  - `LAMPIRAN_A_GLOSARIUM_DAN_SINTAKSIS_WIDYAMOBILE.md`
  - `LAMPIRAN_B_KATALOG_GALAT_DAN_TROUBLESHOOTING.md`
  - `LAMPIRAN_C_PARAMETER_KONFIGURASI_DAN_SECURITY_MANIFEST.md`
  - `LAMPIRAN_D_KUNCI_JAWABAN_DAN_PEMBAHASAN_LENGKAP.md` (Kunci Jawaban Lengkap 120 Soal Esai dan 400 Soal PG A–D)
- **Fitur Khusus**:
  - Arsitektur native sovereign murni tanpa runtime berat pihak ketiga (< 10 MB total instalasi APK/IPA).
  - Integrasi perangkat keras seluler mendalam: Kamera YUV, GPS Haversine Geofencing, Thermal Printer ESC/POS Bluetooth, NFC ISO 14443 & SoftPOS EMV Contactless Level 2.
  - Komunikasi seluler tangguh: BLE 5.3 GATT, BLE Mesh Peer-to-Peer Multi-hop ad-hoc tanpa internet, HTTP/3 QUIC 0-RTT, dan Push Notification APNs/FCM RFC 8291.
  - Arsitektur data berdaulat: Basis data terenkripsi AES-256 WAL, Offline-first Outbox Pattern, Kolaborasi Sinkronisasi Bebas Konflik CRDT (LWW-Register & PN-Counter).
  - Keamanan seluler tingkat tinggi: Hardware Keystore/Secure Enclave TEE, Biometrik Class 3 Crypto-Object, Pertahanan RASP Anti-Root/Anti-Frida, dan Strict SSL Public Key Pinning SHA-256.
  - 6 Pilar Seluler Mutakhir: Edge AI NPU 3D Liveness & OCR KTP-el, Lean Binary & App Clips Instant Experience, Ekosistem Wearable Tiles & Kesehatan PPG, Lock Screen Widgets & Live Activities Dynamic Island, Kompiler Mandiri Pure APK (AXML/DEX v035) Zero-Gradle, Kompiler Mandiri Pure IPA (Mach-O ARM64) Zero-Xcode, dan OTA Delta Patching `WIDYAMOBILE_DELTA10`.

---

## Cara Kompilasi Ulang PDF
Apabila Anda melakukan pembaruan isi materi pada berkas Markdown, jalankan perintah baris perintah *WidyaOffice* berikut:

```bash
# Kompilasi Level Dasar
widya office convert docs/tutorials/01_TUTORIAL_LEVEL_DASAR.md -o docs/tutorials/01_TUTORIAL_LEVEL_DASAR.pdf --toc -t "Tutorial Widya-Lang: Level Dasar" -a "Tim Riset & Pengembang Widya-Lang"

# Kompilasi Level Tingkat Lanjut
widya office convert docs/tutorials/02_TUTORIAL_LEVEL_LANJUT.md -o docs/tutorials/02_TUTORIAL_LEVEL_LANJUT.pdf --toc -t "Tutorial Widya-Lang: Level Tingkat Lanjut" -a "Tim Riset & Pengembang Widya-Lang"

# Kompilasi Level Ekosistem
widya office convert docs/tutorials/03_TUTORIAL_LEVEL_EKOSISTEM.md -o docs/tutorials/03_TUTORIAL_LEVEL_EKOSISTEM.pdf --toc -t "Tutorial Widya-Lang: Level Ekosistem & Solusi Berdaulat" -a "Tim Riset & Pengembang Widya-Lang"

# Kompilasi Buku WidyaBase
widya office convert docs/tutorials/03_BUKU_WIDYABASE.md -o docs/tutorials/03_BUKU_WIDYABASE.pdf --toc -t "Buku Panduan Komprehensif WidyaBase: Rekayasa Mesin Basis Data Berkinerja Tinggi & Mandiri" -a "Tim Riset & Pengembang Widya-Lang"

# Kompilasi Buku WidyaWeb
widya office convert docs/tutorials/04_BUKU_WIDYAWEB.md -o docs/tutorials/04_BUKU_WIDYAWEB.pdf --toc -t "Buku Panduan Komprehensif WidyaWeb: Rekayasa Aplikasi Web Modern Skala Industri" -a "Tim Riset & Pengembang Widya-Lang"

# Kompilasi Buku WidyaDesktop
widya office convert docs/tutorials/05_BUKU_WIDYADESKTOP.md -o docs/tutorials/05_BUKU_WIDYADESKTOP.pdf --toc -t "Buku Panduan Komprehensif WidyaDesktop: Rekayasa Aplikasi Desktop Native & Sistem Tertanam Berdaulat" -a "Tim Riset & Pengembang Widya-Lang"

# Kompilasi Buku WidyaMobile
widya office convert docs/tutorials/06_BUKU_WIDYAMOBILE.md -o docs/tutorials/06_BUKU_WIDYAMOBILE.pdf --toc -t "Buku Panduan Komprehensif WidyaMobile: Rekayasa Aplikasi Seluler Native, Perangkat Keras Pintar & Sistem Tertanam Berdaulat" -a "Tim Riset & Pengembang Widya-Lang"
```

---

## Modul Slide Presentasi Kuliah & Workshop (PPTX 16:9)

Tersedia arsip lengkap **240 modul presentasi slide PowerPoint (.pptx)** rasio 16:9 (*widescreen*) dengan total **2.964 slide** yang mencakup seluruh 40 bab dari 6 buku ekosistem:

- 📂 **Katalog Lengkap & Panduan Slide**: [`docs/tutorials/pptx/README.md`](pptx/README.md)
- 📗 **Level Dasar (40 Bab / 530 Slide)**: [`docs/tutorials/pptx/level_dasar/`](pptx/level_dasar/)
- 📘 **Level Tingkat Lanjut (40 Bab / 480 Slide)**: [`docs/tutorials/pptx/level_lanjut/`](pptx/level_lanjut/)
- 📙 **Ekosistem WidyaBase (40 Bab / 443 Slide)**: [`docs/tutorials/pptx/ekosistem_widyabase/`](pptx/ekosistem_widyabase/)
- 🌐 **Ekosistem WidyaWeb (40 Bab / 505 Slide)**: [`docs/tutorials/pptx/ekosistem_widyaweb/`](pptx/ekosistem_widyaweb/)
- 🖥️ **Ekosistem WidyaDesktop (40 Bab / 527 Slide)**: [`docs/tutorials/pptx/ekosistem_widyadesktop/`](pptx/ekosistem_widyadesktop/)
- 📱 **Ekosistem WidyaMobile (40 Bab / 479 Slide)**: [`docs/tutorials/pptx/ekosistem_widyamobile/`](pptx/ekosistem_widyamobile/)


