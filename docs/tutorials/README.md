# Tutorial & Panduan PDF Widya-Lang

Direktori ini berisi berkas panduan, lembar contekan (*cheatsheet*), dan materi tutorial resmi **Widya-Lang** yang telah dikompilasi secara *native* ke format dokumen PDF Standar ISO 32000-1 (%PDF-1.7) menggunakan mesin **WidyaOffice**.

Repositori ini difokuskan secara minimal pada **inti bahasa pemrograman Widya-Lang** (Level Dasar & Level Tingkat Lanjut).

---

## Berkas Panduan Berdasarkan Level

### 1. Level Dasar (Pemula) — Edisi Komprehensif Cetak (~229 Halaman)
- **PDF Dokumen**: [`01_TUTORIAL_LEVEL_DASAR.pdf`](01_TUTORIAL_LEVEL_DASAR.pdf) (229 Halaman, Tata Letak Halaman Ganjil / *Recto Pages*, Vektor Murni ISO 32000-1)
- **Sumber Markdown Induk**: [`01_TUTORIAL_LEVEL_DASAR.md`](01_TUTORIAL_LEVEL_DASAR.md)
- **Berkas Markdown per Bab**: Tersedia 45 berkas mandiri di direktori [`level_dasar/`](level_dasar/):
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
- **PDF Dokumen**: [`02_TUTORIAL_LEVEL_LANJUT.pdf`](02_TUTORIAL_LEVEL_LANJUT.pdf)
- **Sumber Markdown**: [`02_TUTORIAL_LEVEL_LANJUT.md`](02_TUTORIAL_LEVEL_LANJUT.md)
- **Berkas Markdown per Bab**: Tersedia berkas mandiri di direktori [`level_lanjut/`](level_lanjut/):
  - `00_PENGANTAR.md`
  - `BAB_01_SISTEM_TIPE_STATIS_HINDLEY_MILNER.md` s/d `BAB_40_PROYEK_CAPSTONE_MICROKERNEL_DAN_RUNTIME.md` (40 Bab Pembelajaran Mendalam)
  - Lampiran & Glosarium
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

---

## Modul Ekosistem Lanjutan (Lokal / Ekstensi)

Untuk menjaga repositori inti Widya-Lang tetap ramping, cepat di-*clone*, dan fokus murni pada bahasa pemrograman, buku panduan dan modul tutorial ekosistem lanjutan (WidyaBase, WidyaWeb, WidyaDesktop, WidyaMobile) disimpan secara lokal pada pengembang atau didistribusikan melalui repositori/paket terpisah:
- **WidyaBase**: Rekayasa Mesin Basis Data Berkinerja Tinggi & Mandiri
- **WidyaWeb**: Rekayasa Aplikasi Web Modern Skala Industri (SSR, Islands, QUIC)
- **WidyaDesktop**: Rekayasa Aplikasi Desktop Native & Sistem Tertanam Berdaulat
- **WidyaMobile**: Rekayasa Aplikasi Seluler Native & Sistem Tertanam Berdaulat

---

## Cara Kompilasi Ulang PDF

Apabila Anda melakukan pembaruan isi materi pada berkas Markdown, jalankan perintah baris perintah *WidyaOffice* berikut:

```bash
# Kompilasi Level Dasar
widya office convert docs/tutorials/01_TUTORIAL_LEVEL_DASAR.md -o docs/tutorials/01_TUTORIAL_LEVEL_DASAR.pdf --toc -t "Tutorial Widya-Lang: Level Dasar" -a "Tim Riset & Pengembang Widya-Lang"

# Kompilasi Level Tingkat Lanjut
widya office convert docs/tutorials/02_TUTORIAL_LEVEL_LANJUT.md -o docs/tutorials/02_TUTORIAL_LEVEL_LANJUT.pdf --toc -t "Tutorial Widya-Lang: Level Tingkat Lanjut" -a "Tim Riset & Pengembang Widya-Lang"
```

---

## Modul Slide Presentasi Kuliah & Workshop (PPTX 16:9)

Tersedia modul presentasi slide PowerPoint (.pptx) rasio 16:9 (*widescreen*) untuk kurikulum inti bahasa:

- 📂 **Katalog Lengkap & Panduan Slide**: [`pptx/README.md`](pptx/README.md)
- 📗 **Level Dasar (40 Bab / 530 Slide)**: [`pptx/level_dasar/`](pptx/level_dasar/)
- 📘 **Level Tingkat Lanjut (40 Bab / 480 Slide)**: [`pptx/level_lanjut/`](pptx/level_lanjut/)
