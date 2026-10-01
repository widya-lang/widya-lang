# PRODUCT REQUIREMENT DOCUMENT (PRD)
## PROYEK: SISTEM KOPERASI DESA MERAH PUTIH (KDMP 7 PILAR USAHA)
**Disusun Oleh:** `agen_analis` (SOP-02: Dekomposisi Kebutuhan & Spesifikasi Produk)  
**Disetujui Oleh:** `agen_orkestrator` (Mandor PM) & `agen_arsitek` (Tech Lead)  
**Status:** RESMI • SIAP EKSEKUSI PRODUKSI  

---

### 1. Ringkasan Eksekutif & Problem Statement
Sebagian besar dari 75.000+ Koperasi Desa di Indonesia masih mengandalkan pembukuan manual atau aplikasi berbasis cloud asing yang rentan terhadap putus koneksi internet di wilayah 3T dan kebocoran data nasabah. 

**Tujuan Produk**: Membangun sistem ERP Koperasi Desa Berdaulat terintegrasi yang mencakup **7 Pilar Usaha Terpadu** (Perkantoran, Simpan Pinjam, Warung Sembako POS, Apotik Obat Desa, Klinik SatuSehat, Resi Gudang Komoditas, dan Logistik Armada) dengan kapabilitas **Offline-First CRDT** dan mesin database lokal bergaransi **ACID WidyaBase**.

---

### 2. User Persona & Stakeholder
1. **Pengurus & Pengawas Koperasi**: Membutuhkan laporan neraca saldo, omzet harian, dan estimasi bagi hasil SHU per anggota secara real-time.
2. **Kasir Warung Desa**: Membutuhkan sistem kasir POS cepat, pencarian barcode instan, diskon anggota 5%, dan cetak struk thermal.
3. **Petani & Anggota Koperasi**: Mengajukan pinjaman modal usaha dengan bunga rendah 0.8% flat, menitipkan hasil panen di resi gudang digital, dan melihat saldo simpanan.
4. **Tenaga Medis Klinik / Apoteker Desa**: Mencatat rekam medis elektronik terstandar Kemenkes SatuSehat FHIR dan resep obat generik.

---

### 3. Ruang Lingkup Fitur (7 Pilar Usaha)

| No | Pilar Usaha | Modul & Kebutuhan Fungsional | Kriteria Keberhasilan (Acceptance Criteria) |
| :---: | :--- | :--- | :--- |
| **P1** | **Warung POS** | Katalog produk sembako, pencarian barcode, keranjang dinamis, kalkulator diskon 5%, pembayaran QRIS / Tunai / Potong Simpanan. | Latensi transaksi < 50ms, struk thermal tercetak, kas terupdate otomatis. |
| **P2** | **Simpan Pinjam** | Simulasi pinjaman modal kerja flat 0.8% (tenor 6-36 bulan), registrasi permohonan, jadwal angsuran, buku besar ledger simpanan. | Perhitungan pokok + bunga akurat 100%, persetujuan tercatat di database ACID. |
| **P3** | **Apotik Desa** | Manajemen inventaris obat generik, peringatan kedaluwarsa, resep obat dokter, harga subsidi warga. | Stok berkurang otomatis pasca-penjualan, nol kesalahan penyerahan obat. |
| **P4** | **Klinik Desa** | Rekam Medis Elektronik (RME), input tanda vital, diagnosa ICD-10, tindakan ICD-9-CM, interoperabilitas SatuSehat. | Payload HL7/FHIR JSON tervalidasi 100%, bridging BPJS V-Claim sukses. |
| **P5** | **Resi Gudang** | Pencatatan komoditas masuk (gabah, jagung, kopi), uji kadar air SNI, penerbitan sertifikat digital penjaminan. | Sertifikat digital terbit dengan ID unik, taksiran nilai wajar otomatis. |
| **P6** | **Logistik Armada** | Pelacakan status armada truk/pikap desa, penjadwalan rute pengantaran panen ke pasar induk. | Status perjalanan armada terupdate real-time, rute efisien. |
| **P7** | **Dashboard & RAT** | Konsolidasi omzet 7 pilar, neraca laba-rugi, pembagian dividen SHU tahunan, laporan pajak PP 45. | Laporan keuangan siap cetak untuk Rapat Anggota Tahunan (RAT). |

---

### 4. Batasan Non-Fungsional (Non-Functional Requirements)
- **Ketersediaan (SLA)**: 99.9% uptime, data tersimpan di `koperasi.wdb`.
- **Keamanan**: Kepatuhan penuh UU Perlindungan Data Pribadi No. 27/2022 (masking nomor rekening dan NIK).
- **Performa**: Mendukung 500 koneksi bersamaan dengan konsumsi RAM < 150MB.
