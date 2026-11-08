# Referensi Protokol & Kontrak API IPC WidyaOffice Desktop 📡

> **Spesifikasi Teknis Jembatan Inter-Process Communication (IPC)**  
> *JSON-RPC 2.0 & Protokol Biner Berdaulat (src/desktop/pure_ipc.rs)*

---

## 1. Pendahuluan & Model Keamanan IPC

Antarmuka pengguna (Ribbon UI / Web Canvas) tidak memiliki akses langsung ke sistem operasi atau sistem berkas. Seluruh komunikasi wajib melalui saluran IPC aman berbasis token yang diimplementasikan pada [src/desktop/pure_ipc.rs](file:///home/devops/widya-lang/src/desktop/pure_ipc.rs) (`JembatanIPCMurni`).

Setiap jendela desktop yang dibuat oleh `PengelolaJendelaMurni` memiliki **Token Sesi 256-bit Acak**. Setiap request IPC yang dikirim wajib menyertakan token ini pada header payload:

```json
{
  "jsonrpc": "2.0",
  "token": "wya_ipc_sec_99a8b1c4e7f201d9",
  "method": "dokumen.buka_berkas",
  "params": { ... },
  "id": 101
}
```

Jika token tidak cocok, panggilan langsung ditolak dengan kode error `ERR_IPC_UNAUTHORIZED (-32001)`.

---

## 2. Katalog Metode IPC (IPC Methods Catalog)

---

### A. Siklus Hidup Dokumen (Document Lifecycle & I/O)

#### 1. `dokumen.buat_baru`
Membuat instance dokumen kosong baru dalam memori.
- **Request Params:**
  ```json
  {
    "tipe": "docx" | "xlsx" | "pptx",
    "judul": "Dokumen Tanpa Judul"
  }
  ```
- **Response Result:**
  ```json
  {
    "doc_id": "doc_wya_773921",
    "tipe": "docx",
    "status": "bersih",
    "halaman": 1,
    "waktu_dibuat": 1793160000
  }
  ```

#### 2. `dokumen.buka_berkas`
Membuka dan mendekompres berkas dari sistem berkas lokal.
- **Request Params:**
  ```json
  {
    "path": "/home/devops/dokumen/Laporan_Keuangan.docx"
  }
  ```
- **Response Result:**
  ```json
  {
    "doc_id": "doc_wya_881920",
    "tipe": "docx",
    "judul": "Laporan Keuangan",
    "total_elemen": 142,
    "ada_rumus_omml": true,
    "ada_tanda_tangan": true,
    "html_pratinjau": "<div class='page'>...</div>"
  }
  ```

#### 3. `dokumen.simpan_berkas`
Menyimpan dokumen aktif ke format OpenXML/ODF murni.
- **Request Params:**
  ```json
  {
    "doc_id": "doc_wya_881920",
    "path": "/home/devops/dokumen/Laporan_Keuangan.docx",
    "format": "docx"
  }
  ```
- **Response Result:**
  ```json
  {
    "status": "sukses",
    "bytes_tertulis": 48291,
    "sha256": "8f3b...99a1",
    "waktu_simpan": 1793160120
  }
  ```

#### 4. `dokumen.ekspor_pdf`
Mengompilasi dokumen aktif menjadi berkas PDF vektor berkualitas tinggi.
- **Request Params:**
  ```json
  {
    "doc_id": "doc_wya_881920",
    "path_tujuan": "/home/devops/dokumen/Laporan_Keuangan.pdf",
    "opsi": {
      "ukuran_kertas": "A4",
      "watermark": "RAHASIA NEGARA",
      "sertakan_bookmark": true
    }
  }
  ```
- **Response Result:**
  ```json
  {
    "status": "sukses",
    "total_halaman": 14,
    "bytes": 102490
  }
  ```

---

### B. Persamaan Matematika OMML (Equation Editor)

#### 1. `equation.pratinjau_katex`
Mendapatkan rendering cepat instan saat pengguna mengetik rumus di modal input.
- **Request Params:**
  ```json
  {
    "latex": "\\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}"
  }
  ```
- **Response Result:**
  ```json
  {
    "valid": true,
    "html_render": "<span class='katex'>...</span>",
    "simbol_ditemukan": ["frac", "pm", "sqrt"]
  }
  ```

#### 2. `equation.sisipkan`
Mengompilasi LaTeX menjadi struktur OpenXML OMML (`<m:oMath>`) dan menyematkannya ke dokumen.
- **Request Params:**
  ```json
  {
    "doc_id": "doc_wya_881920",
    "posisi_indeks": 12,
    "latex": "\\int_{0}^{\\infty} e^{-x} dx = 1",
    "is_block": true
  }
  ```
- **Response Result:**
  ```json
  {
    "element_id": "elem_eq_1902",
    "omml_xml": "<m:oMathPara><m:oMath>...</m:oMath></m:oMathPara>",
    "is_block": true
  }
  ```

---

### C. Otomasi Mail Merge Massal

#### 1. `mailmerge.muat_data`
Memuat dan mem-parsing data CSV/JSON/WidyaBase ke dalam cache sesi.
- **Request Params:**
  ```json
  {
    "sumber": "csv",
    "data_path": "/home/devops/data/peserta_pelatihan.csv"
  }
  ```
- **Response Result:**
  ```json
  {
    "dataset_id": "ds_wya_4910",
    "total_rekaman": 1250,
    "daftar_kolom": ["nama_lengkap", "instansi", "nomor_sertifikat", "nilai_akhir"],
    "contoh_baris": {
      "nama_lengkap": "Arya Daniswara",
      "instansi": "BSSN",
      "nomor_sertifikat": "SRT-2026-001",
      "nilai_akhir": "98.5"
    }
  }
  ```

#### 2. `mailmerge.pratinjau_rekaman`
Menghasilkan pratinjau instan pada kanvas dokumen untuk rekaman ke-N.
- **Request Params:**
  ```json
  {
    "doc_id": "doc_wya_881920",
    "dataset_id": "ds_wya_4910",
    "indeks_rekaman": 0
  }
  ```
- **Response Result:**
  ```json
  {
    "indeks": 0,
    "total": 1250,
    "html_terganti": "<div class='page'>Kepada Yth. Arya Daniswara...</div>"
  }
  ```

#### 3. `mailmerge.ekspor_batch`
Menjalankan pembuatan ribuan dokumen secara asinkron di latar belakang (*worker thread*).
- **Request Params:**
  ```json
  {
    "doc_id": "doc_wya_881920",
    "dataset_id": "ds_wya_4910",
    "output_dir": "/home/devops/output_sertifikat/",
    "format_luaran": "docx",
    "mode": "pisah_berkas",
    "pola_nama": "Sertifikat_{{nomor_sertifikat}}_{{nama_lengkap}}.docx"
  }
  ```
- **Response Result:**
  ```json
  {
    "job_id": "job_mm_8812",
    "status": "berjalan",
    "total_antrean": 1250
  }
  ```

#### 4. Event Streaming Progres: `event:mailmerge.progres`
Engine mengirimkan event berkala ke UI tanpa polling:
```json
{
  "event": "mailmerge.progres",
  "job_id": "job_mm_8812",
  "selesai": 625,
  "total": 1250,
  "persentase": 50.0,
  "kecepatan_dok_per_detik": 2650.0
}
```

---

### D. Keamanan Post-Quantum & Modul HSM PKCS#11

#### 1. `hsm.daftar_slot`
Mendeteksi token perangkat keras HSM / smartcard yang terpasang pada port USB.
- **Request Params:** `{}`
- **Response Result:**
  ```json
  {
    "total_slot": 1,
    "slots": [
      {
        "slot_id": 0,
        "token_label": "Widya Sovereign Cryptoki Token",
        "pabrikan": "Nusantara Sovereign Hardware",
        "model": "HSM-PQC-2026",
        "pin_terinisialisasi": true
      }
    ]
  }
  ```

#### 2. `dokumen.tanda_tangani_pqc`
Menandatangani dokumen aktif dengan algoritma NIST FIPS 204 (ML-DSA-87) non-extractable.
- **Request Params:**
  ```json
  {
    "doc_id": "doc_wya_881920",
    "slot_id": 0,
    "pin": "123456",
    "kunci_alias": "kunci_dokumen_resmi",
    "alasan": "Persetujuan Penerbitan Kontrak Kerja",
    "lokasi": "Jakarta Pusat, Indonesia"
  }
  ```
- **Response Result:**
  ```json
  {
    "status": "terverifikasi",
    "algoritma": "ML-DSA-87 (FIPS 204)",
    "signature_digest": "7a9b...ef10",
    "disematkan_di": "word/_xmlsignatures/sig1.xml"
  }
  ```

---

### E. Kolaborasi & Riwayat CRDT (Undo / Redo)

| Metode | Parameter | Deskripsi |
| :--- | :--- | :--- |
| `crdt.undo` | `{ "doc_id": string }` | Mengembalikan status dokumen 1 langkah ke belakang |
| `crdt.redo` | `{ "doc_id": string }` | Memajukan status dokumen 1 langkah ke depan |
| `crdt.track_changes` | `{ "doc_id": string, "aktif": bool }` | Mengaktifkan/mematikan penandaan revisi teks |
| `crdt.tambah_komentar`| `{ "doc_id": string, "range": [10, 25], "teks": string }` | Menyematkan komentar berutas pada seleksi |

---

## 3. Tabel Kode Error Standar (Standard Error Codes)

| Kode Error | Simbol | Solusi & Penjelasan |
| :--- | :--- | :--- |
| `-32001` | `ERR_IPC_UNAUTHORIZED` | Token sesi jendela tidak valid atau kadaluarsa |
| `-32010` | `ERR_FILE_NOT_FOUND` | Berkas pada path yang dituju tidak ditemukan di disk |
| `-32011` | `ERR_FILE_CORRUPT` | Format ZIP berkas OpenXML rusak atau tidak valid |
| `-32020` | `ERR_OMML_SYNTAX` | Sintaks ekspresi LaTeX tidak dapat diterjemahkan ke OMML |
| `-32030` | `ERR_MAILMERGE_FIELD_MISMATCH` | Ada tag placeholder yang tidak memiliki kolom data |
| `-32040` | `ERR_HSM_PIN_INCORRECT` | PIN otentikasi hardware token HSM salah |
| `-32041` | `ERR_PQC_SIGNATURE_INVALID` | Integritas dokumen terganggu (konten diubah setelah tanda tangan) |
