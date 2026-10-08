# DOKUMEN KEBUTUHAN PRODUK (PRD)
## APLIKASI RENCANA KEGIATAN DAN ANGGARAN SEKOLAH (ARKAS) WEB TERINTEGRASI
**Kode Proyek:** `PRJ-ARKAS-WEB-01` | **Versi:** `1.0.0` | **Klasifikasi:** Dokumen Resmi Sistem Kedaulatan Digital

---

### 1. Latar Belakang & Urgensi
Berdasarkan amanat Permendikbudristek No. 63 Tahun 2023 tentang Petunjuk Teknis Pengelolaan Dana Bantuan Operasional Satuan Pendidikan (BOSP) jo. Permendikdasmen No. 15 Tahun 2026 tentang Standar Pengadaan Barang dan Jasa Satuan Pendidikan (SIPLah), tata kelola keuangan satuan pendidikan memerlukan sistem terintegrasi yang menjamin:
1. **Transparansi & Akuntabilitas**: Perencanaan anggaran berbasis Standar Nasional Pendidikan (8 SNP) dan Kode Rekening Akuntansi Permendagri 90/2019.
2. **Kepatuhan Pajak Nasional**: Otomasi pemungutan dan penyetoran PPN 11%, PPh 21, PPh 22, dan PPh 23 dengan integrasi kode e-Billing & Nomor Transaksi Penerimaan Negara (NTPN).
3. **Audit Trail Anti-Fraud**: Rantai audit kriptografis berkecepatan tinggi berarsitektur blockchain lokal (tamper-evident BKU ledger) guna menjamin keaslian bukti audit bagi BPK, BPKP, dan Inspektorat.
4. **Kedaulatan Data Lokal**: Sistem 100% mandiri on-premise berbasis mesin bahasa Widya-Lang dan basis data WidyaBase tanpa ketergantungan runtime asing.

---

### 2. Sasaran Pengguna
- **Kepala Satuan Pendidikan (KPA)**: Pengesahan RKAS, verifikasi pengajuan belanja, dan monitoring serapan anggaran triwulan.
- **Bendahara BOSP (PPK/Bendahara Pengeluaran)**: Pencatatan Buku Kas Umum (BKU), pembuatan SPJ, penarikan/penyetoran kas, dan pemotongan pajak.
- **Operator Sekolah (Tim Manajemen ARKAS)**: Input perencanaan RKAS, pengelolaan dokumen pesanan SIPLah, dan rekonsiliasi bulanan.
- **Dinas Pendidikan & Markas Kabupaten/Kota**: Sinkronisasi data realisasi, verifikasi pengesahan perubahan RKAS, dan konsolidasi BKU.

---

### 3. Ruang Lingkup Fitur Utama
| ID Fitur | Nama Modul | Deskripsi Kebutuhan Teknis |
|:---------|:-----------|:---------------------------|
| **F-01** | Penganggaran RKAS Multi-Sumber | Alokasi BOSP Reguler, BOS Kinerja, BOS Afirmasi, BOSDA, dan Banper APBN ke dalam 8 SNP dengan validasi batas honorarium max 50%. |
| **F-02** | Buku Kas Umum (BKU) Digital | Pencatatan realtime mutasi kas (Bank & Tunai), integrasi pesanan SIPLah, penarikan bank, dan penerimaan transfer. |
| **F-03** | Buku Pembantu Otomatis | Pembentukan Buku Pembantu Kas Tunai, Buku Pembantu Bank, dan Buku Pembantu Pajak secara otomatis tanpa duplikasi entri. |
| **F-04** | Engine Kalkulasi Pajak | Kalkulasi otomatis tarif PPN 11%, PPh 22 (1.5%/3%), PPh 23 (2%/4%), dan PPh 21 tarif progresif PNS/Non-PNS. |
| **F-05** | Rantai Audit Kriptografis | Penyematan hash SHA-256 rantai berantai (*tamper-evident ledger*) pada setiap entri BKU untuk verifikasi integritas instan. |
| **F-06** | Integrasi SIPLah Permen 15/2026 | Sinkronisasi Surat Pesanan (SP), Berita Acara Serah Terima (BAST), faktur elektronik, dan pembayaran Virtual Account. |
| **F-07** | Dashboard Pemantauan Real-time | Visualisasi serapan per SNP, grafik realisasi vs pagu, status tutup buku bulanan, dan audit trail log. |

---

### 4. Batasan & Syarat Non-Fungsional
- **Kinerja**: Waktu muat dashboard < 200 ms, throughput transaksi BKU > 1.000 transaksi/detik pada runtime WidyaWeb.
- **Ketersediaan**: 99.9% uptime dengan fasilitas failover WAL WidyaBase.
- **Kedaulatan**: Zero telemetry eksternal, penyimpanan lokal terenkripsi di `database/data.wdb`.
