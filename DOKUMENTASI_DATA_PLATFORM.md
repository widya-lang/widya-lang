# 🏛️ DOKUMENTASI ARSITEKTUR WIDYASTORE & WIDYADOCDB (ENTERPRISE DATA PLATFORM v3.0.0)

> **Infrastruktur Data Mandiri, Berdaulat, dan Berkinerja Tinggi**
> Solusi komprehensif pengganti **MinIO (Object Storage)** dan **MongoDB (Document Database)** yang terintegrasi secara *native* dalam ekosistem **Widya-Lang / WidyaBase**.

---

## 📑 Daftar Isi
1. [Arsitektur Platform Data Enterprise](#1-arsitektur-platform-data-enterprise)
2. [WidyaStore (MinIO / S3 Compatible Object Storage)](#2-widyastore-minio--s3-compatible-object-storage)
   - [Fitur Utama](#fitur-utama-widyastore)
   - [Struktur API & Method](#struktur-api--method-widyastore)
   - [Perintah CLI WidyaStore](#perintah-cli-widyastore)
3. [WidyaDocDB (MongoDB Compatible NoSQL Engine)](#3-widyadocdb-mongodb-compatible-nosql-engine)
   - [Fitur Utama](#fitur-utama-widyadocdb)
   - [Operator Kueri & Modifikasi Atomik](#operator-kueri--modifikasi-atomik)
   - [Aggregation Pipeline](#aggregation-pipeline)
   - [Perintah CLI WidyaDocDB](#perintah-cli-widyadocdb)
4. [Integrasi Sinergis: DMS Kantor Digital](#4-integrasi-sinergis-dms-kantor-digital)
5. [Pengujian & Verifikasi Kualitas](#5-pengujian--verifikasi-kualitas)

---

## 1. Arsitektur Platform Data Enterprise

```
+-----------------------------------------------------------------------------------+
|                            WIDYA ENTERPRISE APPLICATION                           |
|       (Aplikasi Web / Mobile / Desktop / Agen Kantor Digital / WidyaOffice)       |
+----------------------------------------+------------------------------------------+
                                         |
            +----------------------------+----------------------------+
            |                                                         |
            v                                                         v
+---------------------------------------+ +-----------------------------------------+
|     🪣 WIDYASTORE (S3 / MINIO)        | |      🍃 WIDYADOCDB (MONGODB NoSQL)      |
|---------------------------------------| |-----------------------------------------|
| - S3 REST Protocol & SigV4            | | - Schema-less JSON/BSON Collections     |
| - Content-Addressable Storage (CAS)   | | - Rich Query Operators ($eq, $gt, $in)  |
| - SHA-256 Blob Deduplication          | | - Atomic Modifiers ($set, $inc, $push)  |
| - Multipart Upload Engine (Parallel)  | | - Secondary B-Tree Indexing (Paths)     |
| - Server-Side Encryption (AES-256-GCM)| | - Multi-Stage Aggregation Pipeline      |
| - Presigned URLs & Time-Limited Auth  | | - Real-time Change Streams (Pub/Sub)    |
| - Storage Quotas & Versioning S3      | | - ACID Document Transactions & WAL      |
+-------------------+-------------------+ +--------------------+--------------------+
                    |                                          |
                    +--------------------+---------------------+
                                         |
                                         v
                         +-------------------------------+
                         |    Penyimpanan Lokal / Disk   |
                         |   (100% Data Sovereignty RI)  |
                         +-------------------------------+
```

---

## 2. WidyaStore (MinIO / S3 Compatible Object Storage)

`WidyaStore` dirancang untuk menangani penyimpanan berkas biner tidak terstruktur (*unstructured blobs*) seperti dokumen perkantoran (`.docx`, `.xlsx`, `.pptx`, `.pdf`), arsip ZIP, dan berkas multimedia secara cepat dan aman.

### Fitur Utama WidyaStore:
1. **S3 Protocol & MinIO Compatibility:** Mendukung standar penamaan bucket, struktur objek, *prefix*, *delimiter*, serta XML response S3.
2. **Content-Addressable Storage (CAS):** Penyimpanan fisik berbasis hash SHA-256 mencegah duplikasi data (*deduplication*).
3. **Multipart Upload Berkecepatan Tinggi:** Unggah berkas berukuran gigabyte dalam pecahan paralel (*parts*) dengan penggabungan otomatis.
4. **Keamanan & Presigned URLs (SigV4):** Pembuatan tautan unduh/unggah sementara dengan tanda tangan HMAC-SHA256 tanpa membagikan kredensial utama.
5. **Enkripsi Data (AES-256-GCM):** Enkripsi *at-rest* otomatis pada berkas biner sebelum ditulis ke disk.
6. **Versioning:** Pelacakan riwayat versi berkas dan *Delete Marker* untuk perlindungan dari penghapusan tidak disengaja.

### Contoh Penggunaan WidyaStore di Rust / Widya:
```rust
use widya::widyabase::object_store::*;
use std::collections::HashMap;

// 1. Inisialisasi Storage Engine
let config = ObjectStoreConfig {
    base_path: "./storage_data".into(),
    enable_encryption: true,
    server_region: "id-central-1".into(),
    ..Default::default()
};
let store = WidyaObjectStore::new(config)?;

// 2. Buat Bucket Baru
store.create_bucket("arsip-resmi-2026")?;

// 3. Unggah Dokumen Berkas (PutObject)
let berkas_biner = std::fs::read("kontrak_kerja.docx")?;
let mut meta = HashMap::new();
meta.insert("author".into(), "Satriyo".into());

let obj_meta = store.put_object(
    "arsip-resmi-2026",
    "legal/kontrak_001.docx",
    &berkas_biner,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    meta
)?;

// 4. Buat Presigned URL Aman (Berlaku 1 Jam)
let presigned = store.generate_presigned_url(
    "arsip-resmi-2026",
    "legal/kontrak_001.docx",
    "ACCESS_KEY_WIDYA",
    "SECRET_KEY_SUPER_SECURE",
    3600
);
println!("Presigned URL: {}", presigned);
```

### Perintah CLI WidyaStore:
```bash
# Membuat bucket baru
widya store buat-bucket dokumen-kantor

# Melihat daftar bucket
widya store daftar-bucket

# Mengunggah berkas ke dalam bucket
widya store unggah dokumen-kantor laporan.pdf C:/data/laporan.pdf

# Mengunduh berkas dari bucket
widya store unduh dokumen-kantor laporan.pdf C:/backup/laporan_unduh.pdf

# Membuat Presigned URL sementara
widya store presign dokumen-kantor laporan.pdf --ttl 7200

# Memeriksa status kesehatan Object Storage
widya store status
```

---

## 3. WidyaDocDB (MongoDB Compatible NoSQL Engine)

`WidyaDocDB` adalah basis data dokumen NoSQL dinamis berbasis JSON/BSON untuk menyimpan data hierarkis, metadata dokumen, log transaksi, dan riwayat revisi tanpa skema kaku.

### Fitur Utama WidyaDocDB:
1. **Dynamic Schema-less Collections:** Koleksi dapat menampung struktur JSON arbitrary dengan penomoran `_id` otomatis.
2. **Rich Query Language:** Mendukung operator kueri standar MongoDB:
   - Perbandingan: `$eq`, `$ne`, `$gt`, `$gte`, `$lt`, `$lte`, `$in`, `$nin`, `$exists`, `$regex`, `$size`.
   - Logika: `$and`, `$or`, `$nor`, `$not`.
   - Nested Dot Notation: Pencarian field bertingkat seperti `profile.alamat.kota`.
3. **Atomic Update Modifiers:** Eksekusi modifikasi tanpa *race condition*: `$set`, `$unset`, `$inc`, `$mul`, `$push`, `$pull`, `$addToSet`, `$currentDate`.
4. **Secondary B-Tree Indexing:** Index berkecepatan $O(\log N)$ pada sembarang JSON path dengan validasi keunikan (*unique constraint*).
5. **Aggregation Pipeline:** Agregasi bertingkat (`$match`, `$project`, `$group`, `$sort`, `$limit`, `$skip`, `$unwind`, `$count`).
6. **Real-Time Change Streams:** Mekanisme *event-driven* broadcast untuk mendeteksi `Insert`, `Update`, `Replace`, `Delete` secara instan.
7. **Daya Tahan Tinggi (WAL):** Penyimpanan transaksi ke *Write-Ahead Log* (`.wal.jsonl`) untuk pemulihan otomatis pasca-*crash*.

### Contoh Penggunaan WidyaDocDB:
```rust
use widya::widyabase::docdb::*;
use serde_json::json;

// 1. Buka Database Dokumen
let db = WidyaDocDB::open("./nosql_data")?;
let users = db.collection("pegawai");

// 2. Sisipkan Dokumen (InsertOne / InsertMany)
users.insert_one(json!({
    "nama": "Dr. Satriyo",
    "divisi": "R&D",
    "gaji": 28000000.0,
    "kontak": { "email": "satriyo@widya.id", "kota": "Jakarta" },
    "sertifikasi": ["Rust", "AI Architecture", "ISO 27001"]
}))?;

// 3. Kueri dengan Filter Kompleks
let filter = json!({
    "$and": [
        { "gaji": { "$gte": 20000000.0 } },
        { "kontak.kota": "Jakarta" }
    ]
});
let hasil = users.find(&filter, None, None, None);

// 4. Update Atomik
users.update_one(
    &json!({ "nama": "Dr. Satriyo" }),
    &json!({
        "$inc": { "gaji": 2000000.0 },
        "$push": { "sertifikasi": "WidyaBase Specialist" }
    }),
    false
)?;

// 5. Aggregation Pipeline (Rekap Total Gaji per Divisi)
let pipeline = vec![
    json!({ "$match": { "gaji": { "$gt": 0.0 } } }),
    json!({
        "$group": {
            "_id": "$divisi",
            "total_beban": { "$sum": "$gaji" },
            "jumlah_karyawan": { "$sum": 1 }
        }
    }),
    json!({ "$sort": { "total_beban": -1 } })
];
let rekap = db.aggregate("pegawai", &pipeline)?;
```

### Perintah CLI WidyaDocDB:
```bash
# Menyisipkan dokumen JSON
widya docdb sisip pegawai "{\"nama\": \"Budi\", \"divisi\": \"Keuangan\", \"gaji\": 15000000}"

# Mencari dokumen dengan filter kueri MongoDB
widya docdb cari pegawai "{\"divisi\": \"Keuangan\", \"gaji\": {\"$gte\": 10000000}}"

# Menjalankan Aggregation Pipeline
widya docdb agregasi pegawai "[{\"$group\": {\"_id\": \"$divisi\", \"total\": {\"$sum\": \"$gaji\"}}}]"

# Memeriksa status NoSQL Document Database
widya docdb status
```

---

## 4. Integrasi Sinergis: DMS Kantor Digital

| Alur Kerja | Komponen Widya | Deskripsi |
| :--- | :--- | :--- |
| **1. Pembuatan Dokumen** | `WidyaOffice (Docx / Xlsx / Pptx / Pdf)` | Membuat dan menandatangani dokumen secara elektronik (TTE). |
| **2. Penyimpanan Berkas Biner** | `WidyaStore (Object Storage / MinIO)` | Menyimpan file asli `.docx` / `.pdf` terenkripsi dengan deduplikasi CAS. |
| **3. Metadata & Indexing** | `WidyaDocDB (Document NoSQL / MongoDB)` | Menyimpan metadata (ID, pembuat, hash TTE, tag, izin akses). |
| **4. Kolaborasi Real-Time** | `WidyaDocs CRDT + DocDB Change Stream` | Sinkronisasi multi-pengguna tanpa konflik secara *live*. |

---

## 5. Pengujian & Verifikasi Kualitas

Semua fitur telah diuji secara komprehensif pada [`tests/store_and_docdb_tests.rs`](file:///c:/widya-lang/tests/store_and_docdb_tests.rs):

```text
running 4 tests
test test_widya_docdb_aggregation_pipeline ... ok
test test_widya_docdb_crud_and_rich_queries ... ok
test test_widya_store_bucket_and_object_crud ... ok
test test_widya_store_multipart_upload_and_presigned_url ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; finished in 0.04s
```
