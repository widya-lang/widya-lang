# Spesifikasi Bulk Mail Merge Engine & Wizard 📬

> **Spesifikasi Teknis Penggabungan Surat Massal WidyaOffice Desktop**  
> *Otomatisasi Dokumen: Sumber Data ➔ Pemetaan Tag ➔ Pratinjau Interaktif ➔ Batch Export*

---

## 1. Pendahuluan

Fitur **Mail Merge (Penggabungan Surat)** adalah kapabilitas vital dalam operasional industri, administrasi pemerintahan, universitas, perbankan, dan manufaktur untuk mencetak atau memproduksi dokumen personalisasi dalam skala besar, seperti:
- Surat Keputusan (SK) dan Kontrak Perjanjian Kerja
- Sertifikat Pelatihan & Piagam Penghargaan
- Tagihan, Faktur Pajak, dan Slip Gaji Karyawan
- Undangan Resmi dan Label Pengiriman Logistik

WidyaOffice Desktop menyediakan antarmuka **Mail Merge Wizard** berbasis Ribbon modern yang terhubung langsung ke engine performa tinggi [src/office/docx.rs](file:///home/devops/widya-lang/src/office/docx.rs) (`mail_merge_batch`).

---

## 2. Format Sumber Data yang Didukung

WidyaOffice Desktop dapat membaca sumber data secara langsung tanpa perlu konversi manual:

| Tipe Sumber Data | Format / Ekstensi | Contoh Penggunaan |
| :--- | :--- | :--- |
| **Berkas CSV / TSV** | `.csv`, `.tsv` | Ekspor tabel dari HRIS, Google Sheets, atau Excel |
| **Berkas JSON** | `.json` | Payload data dari REST API atau antrian pesan |
| **Lembar Kerja WidyaCalc** | `.xlsx`, `.ods` | Buku kerja spreadsheet internal yang sedang dibuka |
| **Koneksi Database Langsung** | **WidyaBase**, SQLite, PostgreSQL | Query SQL langsung: `SELECT nama, nik, gaji FROM payroll` |

---

## 3. Sintaks Tag Placeholder Dokumen

Di dalam kanvas dokumen WidyaWriter, pengguna dapat menuliskan tag placeholder menggunakan notasi *mustache* ganda:

```
Kepada Yth.
Bapak/Ibu {{nama_lengkap}}
{{jabatan}} - {{departemen}}
Di {{kota}}

Dengan hormat,

Sehubungan dengan evaluasi kinerja triwulan, kami menyampaikan bahwa Saudara berhak atas insentif sebesar {{format_rupiah nominal_bonus}}.

Nomor Registrasi: {{nomor_induk}}
Tanggal Terbit: {{format_tanggal tanggal_terbit}}
```

### Modifier Pemformatan Otomatis:
- `{{format_rupiah nilai}}`: Mengubah `1500000` menjadi `Rp 1.500.000,00`.
- `{{format_tanggal tgl}}`: Mengubah `2026-10-28` menjadi `28 Oktober 2026`.
- `{{huruf_besar teks}}`: Mengubah teks menjadi HURUF BESAR SEMUA.

---

## 4. Alur Kerja Wizard 5-Langkah di Antarmuka Desktop

Antarmuka Ribbon pada tab **"Surat" (Mailings)** menyediakan alur terpandu yang intuitif:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        WIZARD MAIL MERGE WIDYA                         │
├────────────────────────────────────────────────────────────────────────┤
│  [1. Pilih Template] ➔ [2. Muat Data] ➔ [3. Petakan] ➔ [4. Pratinjau] ➔ [5. Ekspor] │
├────────────────────────────────────────────────────────────────────────┤
│                                                                        │
│  Langkah 4: Pratinjau Rekaman Data                                     │
│  Menampilkan Rekaman: [ |< ] [ < ]  14 dari 1.250  [ > ] [ >| ]         │
│                                                                        │
│  Data Aktif:                                                           │
│  - nama_lengkap : Raden Arya Wijaya                                    │
│  - jabatan      : Kepala Divisi Keamanan Sistem                        │
│  - nominal_bonus: Rp 18.500.000,00                                     │
│                                                                        │
│  [x] Sorot Bidang Gabungan (Warna Kuning Lembut di Kanvas)             │
│                                                                        │
├────────────────────────────────────────────────────────────────────────┤
│ [ < Kembali ]                                         [ Lanjut: Ekspor > ]
└────────────────────────────────────────────────────────────────────────┘
```

### Rincian Tiap Langkah:
1. **Langkah 1 (Template):** Gunakan dokumen yang sedang aktif di layar atau muat berkas template `.docx` yang sudah ada.
2. **Langkah 2 (Sumber Data):** Pilih berkas sumber data (misalnya `karyawan.csv`). Sistem otomatis mendeteksi baris header kolom.
3. **Langkah 3 (Pemetaan Kolom):** Mencocokkan tag di dokumen dengan nama kolom data (misal: tag `{{nama}}` cocok dengan kolom `nama_lengkap`).
4. **Langkah 4 (Pratinjau Interaktif):** Pengguna dapat menelusuri data baris demi baris menggunakan tombol navigasi; kanvas dokumen seketika melakukan substitusi teks secara live.
5. **Langkah 5 (Ekspor & Penyelesaian):** Pengguna memilih mode keluaran yang diinginkan:
   - **Berkas Terpisah per Rekaman:** Menghasilkan `Surat_001_Arya.docx`, `Surat_002_Budi.docx`, dst.
   - **Satu Berkas Gabungan (*Consolidated Document*):** Menggabungkan seluruh surat dalam 1 berkas DOCX panjang yang dipisahkan oleh *Section Page Break*.
   - **Langsung ke PDF Siap Cetak:** Mengompilasi seluruh surat langsung menjadi berkas PDF terdistribusi.

---

## 5. Kinerja & Eksekusi Batch Tanpa UI Freeze

Operasi penggabungan ribuan dokumen ditangani di latar belakang (*background worker thread*) menggunakan fungsi native:

```rust
let hasil_batch: Vec<DocxDocument> = template_doc.mail_merge_batch(&dataset);
```

### Fitur Kinerja Utama:
- **Zero-Allocation Template Cloning:** Struktur AST dokumen di-cache dan hanya node teks placeholder yang diganti.
- **Asynchronous IPC Streaming:** Selama pemrosesan 10.000 data, engine mengirimkan event kemajuan secara berkala melalui saluran IPC (`mailmerge.progres { selesai: 4500, total: 10000, persentase: 45% }`) sehingga progress bar di desktop bergerak mulus dan aplikasi tetap responsif.
- **Throughput:** Mampu menghasilkan lebih dari **2.500 halaman dokumen DOCX per detik** pada prosesor laptop standar.
