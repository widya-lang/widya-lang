# Spesifikasi Interoperabilitas Format Berkas & Round-Trip Lossless 📦

> **Spesifikasi Teknis Dekomposisi & Rekomposisi Format Terbuka Internasional**  
> *ISO/IEC 29500 (OpenXML: DOCX, XLSX, PPTX) & ISO/IEC 26300 (OpenDocument: ODT, ODS, ODP)*

---

## 1. Pendahuluan

Tantangan terbesar yang dihadapi seluruh pengolah kata alternatif di dunia adalah **"Kerusakan Format Setelah Disimpan" (Format Degradation)**. Pengguna korporat sering kali mengeluhkan bahwa setelah mengedit dokumen di aplikasi alternatif dan membukanya kembali di Microsoft Office:
- Tata letak halaman bergeser.
- Diagram SmartArt atau bentuk vektor khusus Microsoft hilang.
- Gaya penomoran bertingkat (*multi-level list*) rusak.
- Formula matematika berubah menjadi teks mentah atau gambar buram.

**WidyaOffice Desktop** menerapkan arsitektur **Round-Trip Lossless Preservation** murni dalam Rust/Widya-Lang untuk menjamin integritas berkas 100% tetap utuh saat diedit bolak-balik antara WidyaOffice dan Microsoft Office.

---

## 2. Struktur Anatomi Kontainer Berkas (OpenXML OPC)

Dokumen `.docx`, `.xlsx`, dan `.pptx` pada dasarnya adalah arsip ZIP yang mematuhi standar *Open Packaging Conventions (OPC)*.

```
Dokumen.docx (Arsip ZIP Berdaulat)
├── [Content_Types].xml               # Kamus tipe konten MIME seluruh bagian
├── _rels/
│   └── .rels                         # Relasi utama paket ke bagian induk
├── docProps/
│   ├── app.xml                       # Metadata aplikasi penyunting (WidyaOffice 1.0)
│   └── core.xml                      # Judul, penulis, tanggal pembuatan & modifikasi
└── word/
    ├── document.xml                  # Batang tubuh dokumen, paragraf, tabel, rumus OMML
    ├── styles.xml                    # Definisi gaya paragraf, karakter, dan tabel
    ├── numbering.xml                 # Definisi butir penomoran (bullets & numbering)
    ├── settings.xml                  # Pengaturan zoom, tata letak, dan kompatibilitas
    ├── fontTable.xml                 # Daftar font dan metrik Panose
    ├── _rels/
    │   └── document.xml.rels         # Relasi hyperlink, gambar internal, dan header/footer
    ├── media/                        # Berkas gambar mentah (.png, .jpeg, .svg)
    └── _xmlsignatures/               # Tanda tangan digital PQC ML-DSA-87 (ISO/IEC 29500-2)
        └── sig1.xml
```

---

## 3. Mekanisme Kunci: *Round-Trip Lossless Preservation*

Ketika pengguna membuka dokumen kompleks buatan Microsoft Office 365 di WidyaOffice Desktop:

```
[ Berkas .docx Asli ]
        │
        ▼
[ OPC Zip Unpacker (src/office/zip.rs) ]
        │
        ├───────────────────────────────┬───────────────────────────────┐
        ▼                               ▼                               ▼
[ Elemen Standar yang Didukung ] [ Elemen Tertutup Microsoft ]   [ Bagian XML Tak Dikenal ]
(Teks, OMML Math, Tabel, Gaya)   (DrawingML SmartArt, ActiveX)   (customXml/, VBA macro)
        │                               │                               │
        ▼                               ▼                               ▼
[ AST Model Dokumen Aktif ]      [ Passthrough Node Vault ]      [ Raw Package Part Cache ]
(Dapat diedit di Ribbon UI)      (Dipertahankan struktur aslinya)(Disimpan byte-per-byte)
        │                               │                               │
        └───────────────────────────────┼───────────────────────────────┘
                                        │
                                        ▼ (Saat Pengguna Menekan Simpan)
                        [ OPC Recomposer & Zip Repackager ]
                                        │
                                        ▼
                        [ Berkas .docx Baru Tanpa Kehilangan Data ]
```

### Aturan Preservasi Semantik:
1. **Node Passthrough (`UnparsedXmlNode`):** Tag XML kepemilikan Microsoft (seperti elemen visual `<w:drawing>` dengan efek 3D kompleks) yang belum didukung untuk disunting di UI tidak dibuang! Elemen tersebut dibungkus sebagai node passthrough dan disisipkan kembali ke posisi XML aslinya saat disimpan.
2. **Raw Package Preservation:** Folder khusus seperti `customXml/`, `word/vbaProject.bin`, atau metadata SharePoint disimpan secara biner mentah (*byte-per-byte*) tanpa modifikasi.
3. **Hasil:** Pengguna dapat mengedit teks dan rumus di WidyaOffice tanpa merusak makro, grafik interaktif, atau tata letak lanjutan yang sudah dibuat oleh rekan kerjanya di Microsoft Word!

---

## 4. Pemetaan Metrik Font & Fallback Hirarki

Untuk mencegah pergeseran baris (*line wrapping shift*) akibat perbedaan sistem operasi:

| Font Dokumen (Microsoft Word) | Font Pengganti Layar (WidyaOffice Desktop) | Karakteristik Metrik |
| :--- | :--- | :--- |
| **Aptos / Calibri** | **Inter / Liberation Sans** | Lebar glif dan tinggi x-height identik |
| **Cambria / Times New Roman** | **DejaVu Serif / Tinos** | Proporsi serif setara dengan perataan seimbang |
| **Consolas / Courier New** | **JetBrains Mono / Liberation Mono**| Monospace dengan spasi karakter terkalibrasi |
| **Cambria Math** | **Widya Math OpenType / KaTeX Vector** | Simbol matematika OMML ter-render sempurna |

> **PENTING:** Nama font pada `word/fontTable.xml` dan `<w:rFonts>` tetap ditulis sebagai font aslinya (misalnya `Calibri` atau `Aptos`). Penggantian font di atas hanya terjadi pada *viewport rendering* jika sistem operasi pengguna tidak memiliki lisensi font tersebut. Ketika berkas dibuka kembali di komputer Windows berlisensi Microsoft Office, Word akan langsung menggunakan font aslinya tanpa modifikasi apa pun.

---

## 5. Matriks Validasi Standar

Setiap dokumen keluaran WidyaOffice Desktop diuji secara otomatis melalui skrip validasi skema:
- **Validasi Skema XSD:** Lolos validasi skema resmi Microsoft OpenXML ECMA-376 / ISO 29500.
- **Deteksi Namespace Matematika:** Menjamin tag `<m:oMath>` selalu berada di bawah namespace `http://schemas.openxmlformats.org/officeDocument/2006/math`.
- **Integritas Penomoran:** Menjamin `numId` dan `abstractNumId` pada `numbering.xml` selalu tersinkronisasi tanpa *dangling reference*.
