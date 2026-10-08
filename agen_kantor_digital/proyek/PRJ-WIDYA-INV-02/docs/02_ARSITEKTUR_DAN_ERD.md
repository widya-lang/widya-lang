# DOKUMEN ARSITEKTUR SISTEM & ERD (02_ARSITEKTUR_DAN_ERD)
## PROYEK: SISTEM MANAJEMEN INVENTARIS UMKM NUSANTARA (PRJ-WIDYA-INV-02)
**Kode Proyek**: `PRJ-WIDYA-INV-02`  
**Disusun Oleh**: `agen_arsitek` (SOP-03: Perancangan Arsitektur Basis Data & Sistem)  
**Disetujui Oleh**: `agen_orkestrator` & `agen_keamanan`  
**Status**: RESMI • STANDARISASI MULTI-AGENT WIDYA  

---

### 1. Diagram Komponen Sistem (C4 Container Model)

```mermaid
flowchart TD
    subgraph ClientLayer["Frontend & Perangkat Pemindai"]
        BC[Pemindai Barcode Laser / Kamera EAN-13]
        RFID[Reader Gerbang RFID UHF EPC Gen2]
        WEB[Web Dashboard Manajemen Inventaris Port 8011]
    end

    subgraph ServiceLayer["Widya-Lang Sovereign Runtime Engine"]
        SRV[Server Mandiri WidyaWeb - server.wya]
        BARCODE_ENG[Engine Barcode GS1 Modulo-10 Modul Rust]
        RFID_ENG[Engine UHF SGTIN-96 Decoder Modul Rust]
        LOCATOR[Bin/Rack Space Allocation Engine]
        AUDIT[Cycle Count & Opname Reconciler]
    end

    subgraph StorageLayer["Basis Data WidyaBase ACID (Slotted-Page 4KB)"]
        WDB[(data.wdb - Slotted Page B+Tree)]
        WAL[(data.wal - Write-Ahead Logging)]
        SKEMA[skema.sql - DDL Mandiri]
    end

    BC -->|Payload Raw Barcode| WEB
    RFID -->|Hex Tag 96-bit & RSSI| WEB
    WEB -->|HTTP REST / SSE Stream :8011| SRV
    SRV --> BARCODE_ENG
    SRV --> RFID_ENG
    SRV --> LOCATOR
    SRV --> AUDIT
    LOCATOR --> WDB
    AUDIT --> WDB
    WDB --- WAL
    SKEMA -.->|Inisialisasi DDL| WDB
```

---

### 2. Entity Relationship Diagram (ERD WidyaBase)

```mermaid
erDiagram
    TABEL_BARANG ||--o{ RFID_TAG_EPC : "memiliki tag RFID"
    TABEL_BARANG ||--o{ PERGERAKAN_STOK : "dicatat mutasi"
    TABEL_BARANG ||--o{ KARTU_STOK_OPNAME : "diverifikasi fisik"
    LOKASI_RAK ||--o{ PERGERAKAN_STOK : "tujuan/asal bin"

    TABEL_BARANG {
        TEKS kode_sku PK "Format: BRG-XXX-NNN"
        TEKS barcode_ean13 "13 digit EAN valid GS1"
        TEKS nama_barang "Nama deskriptif komoditas"
        TEKS kategori "Kategori logistik"
        TEKS satuan "Pcs / Box / Sak / Dus"
        INT stok_sistem "Stok buku terkini"
        INT harga_beli "Harga perolehan (IDR)"
        INT harga_jual "Harga jual (IDR)"
    }

    LOKASI_RAK {
        TEKS kode_bin PK "Format: BIN-ZONA-LORONG-RAK-TINGKAT"
        TEKS zona "Zona A, B, C, atau D"
        INT lorong "Nomor lorong gudang"
        INT rak "Nomor rak penyimpanan"
        INT tingkat "Tingkat/level vertikal"
        INT kapasitas_unit "Maksimum kapasitas muat"
    }

    RFID_TAG_EPC {
        TEKS epc_hex PK "24-char hex string (96-bit)"
        TEKS kode_sku FK "Referensi barang terkait"
        TEKS nomor_seri "Nomor seri unik individual item"
        TEKS status_tag "TERDAFTAR, AKTIF, TERKIRIM, RUSAK"
        REAL rssi_dbm "Tingkat kekuatan sinyal terakhir"
        TEKS tgl_terakhir_scan "Timestamp ISO-8601"
    }

    PERGERAKAN_STOK {
        TEKS id_mutasi PK "Format: MUT-YYYYMMDD-NNNN"
        TEKS kode_sku FK "Kode SKU barang"
        TEKS jenis_mutasi "MASUK, KELUAR, RELOKASI, PENYESUAIAN"
        INT jumlah "Kuantitas unit yang berpindah"
        TEKS dari_lokasi "Lokasi awal / Supplier"
        TEKS ke_lokasi "Lokasi tujuan / Pelanggan"
        TEKS waktu_mutasi "Timestamp transaksi"
        TEKS operator "Identitas petugas operasional"
    }

    KARTU_STOK_OPNAME {
        TEKS id_opname PK "Format: OPN-YYYYMMDD-NNN"
        TEKS kode_sku FK "Kode barang yang diaudit"
        INT stok_tercatat "Jumlah stok sistem saat audit"
        INT stok_fisik "Hasil pemindaian fisik riil"
        INT selisih "stok_fisik - stok_tercatat"
        TEKS status_rekonsiliasi "SESUAI, TERTUNDA_APPROVAL, DIREKONSILIASI"
        TEKS waktu_audit "Timestamp pelaksanaan audit"
    }
```

---

### 3. Definisi Tipe Data & Kompatibilitas WidyaBase
Sesuai arsitektur penyimpanan WidyaBase (B+Tree Slotted-Page 4KB):
- `TEKS`: String UTF-8 variable length (disimpan dalam slotted record offset).
- `INT`: Integer 64-bit bertanda (2's complement).
- `REAL`: Floating point 64-bit IEEE-754 (digunakan untuk RSSI dBm dan desimal).
- `KUNCI_UTAMA`: Indeks primer otomatis menggunakan pohon B+Tree terurut untuk pencarian O(log N).

---

### 4. Alokasi Port Jaringan & Skema Aliran Data
- **Port Layanan Web & Dashboard**: `http://localhost:8011`
- **Port Cadangan / Failover**: `http://localhost:8012`
- **Direktori Berkas Web**: `agen_kantor_digital/proyek/PRJ-WIDYA-INV-02/web/`
- **Berkas Basis Data**: `agen_kantor_digital/proyek/PRJ-WIDYA-INV-02/database/data.wdb`
- **Write-Ahead Log**: `agen_kantor_digital/proyek/PRJ-WIDYA-INV-02/database/data.wal`
- **Berkas DDL Skema**: `agen_kantor_digital/proyek/PRJ-WIDYA-INV-02/database/skema.sql`

---

### 5. DDL Fisik Terdaftar
Skema fisik database disimpan secara mandiri dan terisolasi pada `database/skema.sql`, siap dieksekusi oleh mesin runtime tanpa dependensi eksternal.
