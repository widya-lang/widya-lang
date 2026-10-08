# PRODUCT REQUIREMENT DOCUMENT (PRD)
## PROYEK: SISTEM MANAJEMEN INVENTARIS UMKM NUSANTARA (PRJ-WIDYA-INV-02)
**Kode Proyek**: `PRJ-WIDYA-INV-02`  
**Disusun Oleh**: `agen_analis` (SOP-02: Dekomposisi Kebutuhan & Spesifikasi Produk)  
**Disetujui Oleh**: `agen_orkestrator` (Mandor PM) & `agen_arsitek` (Tech Lead)  
**Status**: RESMI • SIAP EKSEKUSI PRODUKSI  

---

### 1. Ringkasan Eksekutif & Problem Statement
Sektor logistik dan pergudangan UMKM di Indonesia kerap menghadapi tantangan inefisiensi pencatatan stok fisik, selisih inventaris (*phantom inventory*), dan lambatnya proses audit siklus (*cycle count*). Berbagai sistem pergudangan konvensional masih mengandalkan pencatatan manual atau pemindai generik yang tidak terintegrasi secara standar dengan penomoran internasional GS1 dan teknologi frekuensi radio (UHF RFID).

**Tujuan Produk**:
Membangun platform **Manajemen Inventaris UMKM Nusantara** berdaulat berbasis runtime Widya-Lang native dengan kapabilitas:
1. **Standarisasi Barcode Global**: Dukungan penuh kalkulasi checksum EAN-13 dan parsing GS1-128 Application Identifiers (AI `(01)` GTIN, `(10)` Batch, `(17)` Expire, `(21)` Serial).
2. **Otomasi Gerbang RFID**: Protokol EPC Gen2 SGTIN-96 (96-bit UHF) dengan pemantauan RSSI signal level dan deteksi arah perpindahan gerbang (IN/OUT).
3. **Manajemen Bin/Rak Cerdas**: Pemetaan topologi rak 3 dimensi (Zona, Lorong, Rak, Tingkat) dengan kalkulasi kapasitas dan utilisasi.
4. **Audit Stok Opname Real-Time**: Rekonsiliasi perbedaan stok tercatat vs stok fisik otomatis dengan pelaporan deviasi instan.

---

### 2. User Persona & User Journey Map

#### Persona Pengguna:
1. **Kepala Gudang (Warehouse Supervisor)**:
   - *Kebutuhan*: Memantau pergerakan stok real-time, utilisasi kapasitas rak, dan approval rekonsiliasi opname.
   - *Pain Point*: Selisih stok yang baru diketahui saat audit tahunan.
2. **Operator Penerimaan & Pemindaian (Barcode Operator)**:
   - *Kebutuhan*: Pemindaian cepat EAN-13/GS1-128 dengan validasi instan, pencetakan label barcode SKU.
   - *Pain Point*: Barcode cacat atau tidak sesuai standar checksum GS1 menyebabkan penolakan barang.
3. **Petugas Gerbang Logistik (RFID Gate Checker)**:
   - *Kebutuhan*: Deteksi palet/karton otomatis saat melintasi portal RFID UHF tanpa membuka kemasan.
   - *Pain Point*: Pemindaian manual per barang memakan waktu berjam-jam saat truk bongkar muat.
4. **Auditor Keuangan & Kepatuhan (Internal Auditor)**:
   - *Kebutuhan*: Jejak audit mutasi (movement history) yang tamper-proof dan bebas manipulasi.
   - *Pain Point*: Catatan kertas mutasi rentan hilang atau diubah sepihak.

#### User Journey:
```
[Barang Datang] -> [Scan Barcode GS1-128 / Labeling EPC] -> [Alokasi ke Bin/Rak] 
                -> [Pintu Keluar: Deteksi Gate RFID UHF] -> [Rekonsiliasi Stok Otomatis]
```

---

### 3. Matriks Fitur

| Prioritas | Modul / Fitur | Deskripsi Fungsional | Acceptance Criteria (BDD) |
|:---:|:---|:---|:---|
| **P1** | **EAN-13 & GS1-128 Generator/Validator** | Validasi checksum modulo 10 EAN-13 dan pemecahan elemen AI `(01)`, `(10)`, `(17)`, `(21)`. | **Given** SKU 12 digit, **When** fungsi generate dipanggil, **Then** digit ke-13 valid sesuai kalkulasi GS1. |
| **P1** | **UHF RFID EPC Gen2 (SGTIN-96)** | Encode dan decode 96-bit EPC tag yang memuat Company Prefix, Item Reference, dan Serial Number. | **Given** EPC hex string valid, **When** didekode, **Then** atribut Company Prefix & Serial terbaca presisi. |
| **P1** | **Simulasi Gerbang RFID (Gate Portal)** | Deteksi transisi barang masuk (Gate Masuk) dan keluar (Gate Keluar) berdasarkan RSSI & antena. | **Given** sinyal tag dengan RSSI > -70 dBm, **When** melewati antena, **Then** event Masuk/Keluar tercatat dalam 10ms. |
| **P2** | **Topologi Rak / Bin Warehouse** | Representasi spasial lokasi penyimpanan (Zona-Lorong-Rak-Tingkat) beserta kuota muatan. | **Given** lokasi `ZONA-A-01-02-03`, **When** barang dialokasikan, **Then** kuota berkurang dan status bin terupdate. |
| **P2** | **Rekonsiliasi Stok Opname (Cycle Count)** | Penghitungan selisih antara stok tercatat pada sistem dan stok fisik riil hasil scan. | **Given** stok sistem 100 dan fisik 98, **When** audit dijalankan, **Then** selisih -2 dicatat beserta status persetujuan. |
| **P3** | **Telemetri & Geofencing Gudang** | Visualisasi RSSI real-time, status gate scanner, dan peringatan tag anomali. | **Given** tag asing tak terdaftar, **When** tertangkap antena, **Then** notifikasi alert dipicu pada dashboard. |

---

### 4. Acceptance Criteria (BDD Specification)

```gherkin
Skenario 1: Validasi Barcode GS1 EAN-13
Given Sebuah produk dengan prefix pabrik dan kode item 12 digit "899275312345"
When Sistem mengeksekusi perhitungan Ean13Generator
Then Checksum digit ke-13 dihitung menjadi "6"
And Barcode lengkap "8992753123456" dinyatakan valid oleh validator GS1.

Skenario 2: Deteksi Masuk Melalui RFID UHF Gate
Given Tag RFID EPC SGTIN-96 "3014257BF7194E4000001A85" terpasang pada kardus barang
When Palet melewati Gate Masuk Gudang A dengan RSSI -42.5 dBm
Then Sistem mencatat event "BarangMasuk" secara instan
And Stok sistem untuk SKU bersangkutan bertambah sejumlah unit pada palet.

Skenario 3: Audit Stok Opname dan Deteksi Deviasi
Given SKU "BRG-KOP-001" memiliki stok buku sistem sebanyak 50 unit
When Tim gudang melakukan stock opname fisik dan memindai 48 unit
Then Sistem menghasilkan selisih -2 unit (Deviasi: -4.0%)
And Kartu stok opname diterbitkan dengan status "MenungguApproval".
```

---

### 5. Metrik Keberhasilan Bisnis (KPI)
1. **Kecepatan Penerimaan Barang**: Reduksi waktu check-in logistik dari 15 menit per palet menjadi < 30 detik menggunakan RFID Gate.
2. **Akurasi Inventaris**: Peningkatan akurasi pencatatan stok fisik vs buku hingga **99.8%**.
3. **Zero Phantom Stock**: Eliminasi stok fiktif melalui jejak audit pergerakan mutasi real-time.
4. **Kepatuhan Standar Industri**: 100% interoperabilitas format GS1-128 dan EPC Global Gen2 UHF.
