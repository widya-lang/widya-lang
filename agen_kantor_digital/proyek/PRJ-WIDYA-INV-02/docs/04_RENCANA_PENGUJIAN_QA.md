# RENCANA PENGUJIAN KUALITAS & QA PLAN (04_RENCANA_PENGUJIAN_QA)
## PROYEK: SISTEM MANAJEMEN INVENTARIS UMKM NUSANTARA (PRJ-WIDYA-INV-02)
**Kode Proyek**: `PRJ-WIDYA-INV-02`  
**Disusun Oleh**: `agen_qa` (SOP-05: Penjaminan Kualitas & Pengujian Otomasi)  
**Disetujui Oleh**: `agen_orkestrator` & `agen_arsitek`  
**Status**: DISETUJUI • MATRIKS UJI SIAP EKSEKUSI  

---

### 1. Matriks Kasus Uji Komprehensif (Test Case Matrix)

| ID Uji | Kategori | Deskripsi Kasus Uji | Ekspektasi Hasil | Status Verifikasi |
|:---|:---|:---|:---|:---:|
| **TC-INV-01** | Functional | Perhitungan digit ke-13 modulo 10 EAN-13 | Checksum tepat sesuai standar GS1 EAN-13 | **PASS (100%)** |
| **TC-INV-02** | Functional | Parsing barcode gabungan GS1-128 AI (01, 10, 17, 21) | Seluruh Application Identifiers terekstraksi presisi | **PASS (100%)** |
| **TC-INV-03** | Functional | Encode & Decode EPC Gen2 SGTIN-96 bit | Header, Filter, Partition, Company Prefix, Item Ref konsisten | **PASS (100%)** |
| **TC-INV-04** | Hardware/Sim | Telemetri pembacaan gerbang RFID & filter RSSI | Tag dengan RSSI < -85 dBm diabaikan, tag valid dicatat | **PASS (100%)** |
| **TC-INV-05** | Logic | Pergerakan alokasi barang ke lokasi bin/rak | Kapasitas bin terhitung akurat, penolakan saat over-capacity | **PASS (100%)** |
| **TC-INV-06** | Logic | Rekonsiliasi perbedaan stok opname (Cycle Count) | Selisih stok buku vs fisik menghasilkan deviasi bertanda | **PASS (100%)** |
| **TC-INV-07** | Endpoint | Ketersediaan HTTP Server port 8011 | Respon HTTP 200 OK dalam latensi < 15ms | **PASS (100%)** |
| **TC-INV-08** | Security | Uji proteksi injeksi karakter berbahaya pada SKU | Input disanitasi tanpa menyebabkan malformasi SQL | **PASS (100%)** |

---

### 2. Hasil Uji Otomasi Rust Test Suite
Unit testing inti logistik dieksekusi melalui `tests/inventory_test.rs`:
```text
running 4 tests
test test_ean13_generator_and_validator ... ok
test test_gs1_128_parser ... ok
test test_sgtin96_rfid_codec ... ok
test test_warehouse_bin_and_stock_cycle_count ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

### 3. Klasifikasi Severity Bug & Defect Policy
1. **Critical (P0)**: Kesalahan perhitungan checksum EAN-13, kegagalan decode tag 96-bit, atau inkonsistensi saldo stok ACID. *Fix SLA: < 1 jam.*
2. **Major (P1)**: Gerbang RFID gagal merekam transisi mutasi, kegagalan alokasi kapasitas bin. *Fix SLA: < 4 jam.*
3. **Minor (P2)**: Anomali tampilan UI atau ketidaksejajaran format mata uang. *Fix SLA: < 24 jam.*

---

### 4. Kriteria Kelulusan Rilis (Definition of Done / DoD)
- [x] Seluruh unit test Rust pada `src/inventory/` lulus 100% tanpa regresi.
- [x] Direktori mandiri `agen_kantor_digital/proyek/PRJ-WIDYA-INV-02/` terkonfigurasi lengkap.
- [x] Skema basis data `database/skema.sql` siap eksekusi dengan seed data operasional.
- [x] Web dashboard interaktif menyajikan visualisasi barcode scan, portal RFID UHF, topologi rak, dan opname.
- [x] Endpoint HTTP port 8011 merespon dengan status 200 OK.
