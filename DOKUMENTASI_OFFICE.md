# 🏢 DOKUMENTASI RESMI WIDYAOFFICE ENTERPRISE SUITE (v2.5.0)

**WidyaOffice** adalah modul aplikasi pemrosesan dokumen produktivitas kantor berstandar industri internasional (ECMA-376 / OpenXML) yang terintegrasi secara *native* di dalam Widya-Lang. Modul ini memungkinkan pembuatan, manipulasi, kalkulasi rumus, digital signature (TTE), dan penyajian presentasi secara mandiri (*zero external cloud dependency* & 100% kedaulatan data lokal).

---

## 📑 1. Modul Dokumen Microsoft Word (`.docx`) — `src/office/docx.rs`

Menyediakan pembuatan, templating, dan enkapsulasi dokumen OpenXML biner:
- **Heading & Paragraf Berjenjang**: Penataan hierarki dokumen (H1-H6) dan atribut tipografi (*Bold*, *Italic*, *Underline*, *Font Size pt*, *Color Hex*, *Callout Box*).
- **Tabel Enterprise Multi-Kolom**: Pembuatan tabel data bergaris batas resmi dengan *header row*.
- **Template & Mail Merge Engine**: Pengisian otomatis placeholder `{{kunci_variabel}}` untuk penerbitan dokumen massal (Faktur, Slip Gaji, Ijazah, Surat Keputusan).
- **Watermarking & Cap Resmi**: Cap air dinamis (*"RAHASIA"*, *"DRAF"*, *"SALINAN"*) dengan sudut rotasi dan transparansi adaptif.
- **Tanda Tangan Elektronik (TTE)**: Penyematan sertifikat TTE berstandar BSrE / X.509 PKI berbasis ringkasan SHA-256 dan kunci rahasia HMAC.
- **Sensor Redaksi Otomatis UU PDP No. 27/2022**: Penyamaran otomatis nomor NIK, KK, rekening bank, nomor telepon, dan email sebelum dokumen dipublikasikan.
- **Ekspor Biner Asli (`.docx`) & Pratinjau Cetak**: Menghasilkan kontainer biner ZIP OpenXML (`.docx`) murni dan rendering HTML5 siap cetak (*Print-Ready / PDF Export*).

```bash
# Membuat berkas biner .docx asli
widya office buat-docx --nama "Surat Keputusan Direksi 2026" --output sk_direksi.docx --watermark "RAHASIA"

# Menandatangani dokumen secara elektronik (TTE)
widya office tte-docx sk_direksi.docx --penandatangan "Satriyo" --peran "Direktur Utama" --output sk_direksi_signed.html

# Sensor redaksi otomatis data pribadi sesuai UU PDP
widya office sensor-pdp data_pegawai.txt --output data_pegawai_terlindungi.txt
```

---

## 📊 2. Modul Spreadsheet Microsoft Excel (`.xlsx`) — `src/office/xlsx.rs`

Menyediakan lembar kerja multi-dimensi, format sel, dan mesin kalkulasi matematika komprehensif:
- **Format Sel & Tampilan Finansial**:
  - Format Mata Uang Rupiah: `Rp 150.000.000` (IDR)
  - Format Persentase: `15.5%`
  - Pengaturan presisi angka desimal
- **Pustaka Rumus Industri Komprehensif (15+ Rumus)**:
  - **Agregasi**: `=SUM(A1:A10)`, `=AVERAGE(B1:B5)`, `=MIN(A1:A5)`, `=MAX(A1:A5)`, `=COUNT(C1:C20)`
  - **Logika & Kondisional**: `=IF(B2>=100, "TARGET TERCAPAI", "EVALUASI")`
  - **Pencarian Data (Lookup)**: `=VLOOKUP("PRJ-01", A1:D10, 2)`
  - **Manipulasi Teks**: `=CONCAT(A1, " - ", B1)`, `=UPPER(C1)`, `=LOWER(C1)`, `=TRIM(D1)`, `=LEN(E1)`
  - **Matematika & Pembulatan**: `=ROUND(A1, 2)`, `=ABS(B1)`
  - **Finansial & Fintech**: `=PMT(rate, nper, pv)` (Kalkulasi cicilan pinjaman koperasi/kredit)
  - **Tanggal & Waktu**: `=TODAY()`, `=NOW()`
  - **Aritmatika Langsung**: `=A1 + B1 * C1`
- **Ekspor Biner Asli (`.xlsx`) & Impor/Ekspor CSV**: Menghasilkan biner ZIP OpenXML (`.xlsx`) murni yang dapat dibuka langsung di Microsoft Excel, LibreOffice, atau Google Sheets.

```bash
# Membuat berkas biner .xlsx asli
widya office buat-xlsx --nama "Laporan Keuangan Q3" --output laporan_keuangan.xlsx

# Evaluasi rumus spreadsheet dari CLI
widya office hitung-rumus "=IF(A1>=1000000, 150000, 50000)" --data "1500000"
widya office hitung-rumus "=SUM(A1:A3)" --data "1000000\n2500000\n1500000"
```

---

## 📽️ 3. Modul Presentasi Microsoft PowerPoint (`.pptx`) — `src/office/pptx.rs`

Menyediakan perancangan tayangan slide eksekutif dan deck presentasi kelas korporasi:
- **Tema Visual Premium**:
  - `DarkModern`: Tema gelap futuristik biru neon
  - `ExecutiveNavy`: Tema biru safir formal eksekutif
  - `EmeraldFintech`: Tema hijau zamrud perbankan dan koperasi digital
  - `SovereignGold`: Tema emas mewah kenegaraan
- **Tata Letak Slide (*Layouts*)**:
  - `TitleSlide`: Judul utama dan subjudul presentasi
  - `TitleAndContent`: Judul slide dan poin-poin penjelasan
  - `TwoColumns`: Perbandingan dua kolom konten berdampingan
  - `KpiDashboard`: Grid kartu metrik KPI eksekutif (Nilai, Label, Tren MoM, Indikator Positif/Negatif)
- **Ekspor Biner Asli (`.pptx`) & Web Deck HTML5**: Menghasilkan berkas biner PowerPoint OpenXML (`.pptx`) dan penampil slide HTML5 interaktif lengkap dengan navigasi keyboard (`Spasi` / `Panah Kanan` / `Panah Kiri`).

```bash
# Membuat berkas biner .pptx asli
widya office buat-pptx --nama "Briefing Transformasi Digital 2026" --output presentasi.pptx
```

---

## 🔒 4. Keamanan Siber & Kepatuhan Regulasi (UU PDP No. 27/2022) — `src/office/security.rs`

- **TTE Kriptografis (BSrE Ready)**: Setiap dokumen dapat disegel dengan tanda tangan digital SHA-256 berstempel waktu ISO 8601.
- **Redaksi Data Pribadi Otomatis**:
  - NIK & KK (16 digit) disensor menjadi `327101******8901`
  - Rekening Bank (10-14 digit) disensor menjadi `1234****90`
  - Nomor Telepon (+62 / 08xx) disensor menjadi `0812****7890`
  - Email disensor menjadi `b***o@kemdikbud.go.id`
- **Zero Cloud Leakage**: Seluruh dokumen diproses 100% pada mesin lokal (*air-gapped capable*).

---

## 🧪 5. Status Modul & Perintah CLI

```bash
widya office status
```

Output:
```text
============================================================
  🏢 STATUS WIDYAOFFICE ENTERPRISE SUITE v2.5.0
============================================================
  Microsoft Word (.docx)   : AKTIF (OpenXML ZIP, Template & TTE)
  Microsoft Excel (.xlsx)  : AKTIF (15+ Rumus, Grid & ZIP Package)
  PowerPoint (.pptx)       : AKTIF (KPI Cards, Web Deck & ZIP Package)
  TTE & Kepatuhan UU PDP   : AKTIF (SHA-256 BSrE & Auto Redaction)
  Native ISO 32000-1 PDF   : AKTIF (%PDF-1.7 Vector & Text Stream)
  LaTeX / TeX Math Engine  : AKTIF (Equations, KaTeX Preview)
  GFM Markdown Transpiler  : AKTIF (AST, Checklists, Codeblock)
  Kedaulatan Data Dokumen  : 100% OFFLINE & ZERO CLOUD LEAK
============================================================
```

---

## 💻 6. Contoh Penggunaan dalam Bahasa Widya-Lang

```widya
// 1. Membuat dokumen Word berstandar industri
biar doc = office_buat_docx("Surat Perjanjian Kerja Sama")
biar doc_update = office_docx_tambah_paragraf(doc, "Perjanjian ini tunduk pada UU PDP No. 27/2022.", benar, salah, 12.0)
biar teks = office_docx_ekstrak_teks(doc_update)

// 2. Evaluasi rumus spreadsheet kompleks
biar data_csv = "15000000\n25000000\n10000000"
biar total_anggaran = office_xlsx_evaluasi_rumus(data_csv, "=SUM(A1:A3)")
tulis("Total Anggaran: " + teks(total_anggaran))

// 3. Membuat slide presentasi eksekutif
biar pptx = office_buat_pptx("Laporan Tahunan Korporasi 2026")
biar html_deck = office_pptx_render_slideshow(pptx)
```
