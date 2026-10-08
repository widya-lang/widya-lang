# DOKUMEN ARSITEKTUR SISTEM & ENTITY RELATIONSHIP DIAGRAM (ERD)
## APLIKASI RENCANA KEGIATAN DAN ANGGARAN SEKOLAH (ARKAS) WEB TERINTEGRASI
**Kode Proyek:** `PRJ-ARKAS-WEB-01` | **Status:** Terverifikasi Arsitektur Sistem

---

### 1. Diagram Arsitektur C4 Level 2 (Container Diagram)

```text
+-----------------------------------------------------------------------------------+
|                           PENGGUNA (BROWSER KLIEN)                                |
|   Kepala Sekolah (KPA)  |  Bendahara BOSP (PPK)  |  Tim Manajemen Mutu Sekolah     |
+-----------------------------------------------------------------------------------+
                                        | (HTTPS / WebSocket WSS)
                                        v
+-----------------------------------------------------------------------------------+
|               WIDYAWEB SOVEREIGN APPLICATION SERVER (Port 8012)                   |
|  +-----------------------------------------------------------------------------+  |
|  | Web Application Gateway & Static Asset Dispatcher                           |  |
|  | - index.html (Responsive Dashboard) | style.css | app.js (State Engine)    |  |
|  +-----------------------------------------------------------------------------+  |
|  | Rust Arkas Core Engine:                                                     |  |
|  | - rkas_anggaran.rs      (Pagu 8 SNP, Plafon Belanja, Sandingan Revisi)      |  |
|  | - bku_transaksi.rs      (Buku Kas Umum, Rantai SHA-256 Tamper-Evident)      |  |
|  | - siplah_integrasi.rs   (SP-SIPLah, BAST Digital Permen 15/2026, Negosiasi)|  |
|  | - spj_markas.rs         (Laporan Rekonsiliasi & Integrasi Dinas Markas)     |  |
|  +-----------------------------------------------------------------------------+  |
+-----------------------------------------------------------------------------------+
                                        | (Koneksi Native Slotted-Page Engine)
                                        v
+-----------------------------------------------------------------------------------+
|                     WIDYABASE STORAGE ENGINE (database/data.wdb)                  |
|  - Slotted-Page 4KB Storage Block                                                 |
|  - B+Tree Index (Kunci Utama NPSN & Nomor BKU)                                    |
|  - Write-Ahead Logging (WAL ACID)                                                 |
|  - Tabel Aktif: tabel_sekolah, tabel_sumber_dana, tabel_rkas_kegiatan,             |
|                 tabel_transaksi_bku, tabel_buku_pajak, tabel_pesanan_siplah       |
+-----------------------------------------------------------------------------------+
```

---

### 2. Entity Relationship Diagram (ERD) WidyaBase

```text
+------------------------+        1:N        +-----------------------------+
|     tabel_sekolah      | ----------------- |      tabel_sumber_dana      |
+------------------------+                   +-----------------------------+
| npsn (PK)              |                   | id_alokasi (PK)             |
| nama_sekolah           |                   | npsn (FK)                   |
| alamat_sekolah         |                   | jenis_dana                  |
| nama_kepsek            |                   | tahun_anggaran              |
| nip_kepsek             |                   | pagu_ditetapkan_rp          |
| nama_bendahara         |                   | dana_cair_rp                |
| rekening_bank          |                   | sisa_saldo_rp               |
+------------------------+                   +-----------------------------+
            |                                               |
            | 1:N                                           | 1:N
            v                                               v
+------------------------+        1:N        +-----------------------------+
|   tabel_rkas_kegiatan  | ----------------- |     tabel_transaksi_bku     |
+------------------------+                   +-----------------------------+
| id_kegiatan (PK)       |                   | nomor_urut (PK)             |
| kode_snp               |                   | tanggal_transaksi           |
| uraian_kegiatan        |                   | nomor_bukti                 |
| kode_rekening          |                   | uraian                      |
| kelompok_belanja       |                   | jenis_transaksi             |
| total_pagu_rp          |                   | id_kegiatan_rkas (FK)       |
| total_realisasi_rp     |                   | metode_bayar                |
| sisa_pagu_rp           |                   | penerimaan_rp               |
+------------------------+                   | pengeluaran_rp              |
                                             | saldo_kumulatif_rp          |
                                             | hash_transaksi (SHA256)     |
                                             | hash_sebelumnya (SHA256)    |
                                             +-----------------------------+
                                                            |
                                                            | 1:1 / 1:N
                                                            v
                                             +-----------------------------+
                                             |      tabel_buku_pajak       |
                                             +-----------------------------+
                                             | id_pajak (PK)               |
                                             | nomor_bukti_bku (FK)        |
                                             | pph_21_rp                   |
                                             | pph_22_rp                   |
                                             | pph_23_rp                   |
                                             | ppn_11_rp                   |
                                             | kode_billing                |
                                             | ntpn_setor                  |
                                             +-----------------------------+
```

---

### 3. Keterangan Kunci Struktur Data
1. **Rantai Hash Audit**: Kolom `hash_transaksi` dan `hash_sebelumnya` membentuk *tamper-evident cryptographic ledger*. Setiap transaksi mengunci riwayat saldo sebelumnya, sehingga mutasi masa lalu tidak dapat direkayasa.
2. **Plafon Belanja RKAS**: Kolom `total_realisasi_rp` pada `tabel_rkas_kegiatan` diperbarui setiap kali transaksi pengeluaran disetujui, dan dijaga agar tidak melampaui `total_pagu_rp`.
3. **Pemisahan Kas**: BKU membedakan mutasi Kas Bank (transfer giro / SIPLah) dan Kas Tunai Bendahara melalui penarikan tunai formal.
