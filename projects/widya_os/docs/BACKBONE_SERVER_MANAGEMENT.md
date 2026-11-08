# Spesifikasi Pengelolaan Server Backbone Sistem WidyaOS & WidyaWorkspace 🖥️🌐

> **Panduan Arsitektur Pusat Kendali, Orkestrasi Cluster, Penyimpanan Skala Petabyte & Pemulihan Mandiri**  
> *WidyaControl Plane · Multi-Node Raft Cluster · Erasure Coding 8+4 · Auto-Scaling MicroVM · Zero Data Loss*

---

## 1. Pendahuluan

Mengoperasikan infrastruktur backbone nasional yang melayani ribuan pengguna **WidyaOS**, jutaan berkas di **WidyaWorkspace (1 TB per user)**, dan beban kerja AI di **WidyaGrid** menuntut arsitektur server yang:
- **Tahan Banting (*High Availability 99.95%+*):** Tetap berjalan normal meski beberapa server listriknya mati atau kabel jaringannya putus.
- **Mudah Dikelola (*Low Operational Overhead*):** Dilengkapi dashboard visual dan perintah CLI otomatis tanpa butuh puluhan teknisi manual.
- **Efisien Biaya (*Cost-Optimized Hardware*):** Memanfaatkan kompresi Zstandard dan deduplikasi blok, sehingga 1.000 pengguna (total kuota 1 Petabyte) dapat ditampung secara riil dengan ~350–400 Terabyte penyimpanan fisik.
- **Berdaulat & Mandiri (*Zero Foreign Cloud Lock-In*):** Dapat dipasang di data center lokal mana pun (Telkom, Indosat, Pusat Data Nasional/PDN, atau server mandiri instansi).

---

## 2. Topologi Tiga Tingkat Infrastruktur Backbone (3-Tier Cluster Architecture)

```
┌────────────────────────────────────────────────────────────────────────┐
│                        LAPISAN 1: EDGE & INGRESS                       │
│  - Load Balancer HTTP/3 QUIC & TLS 1.3 Pasca-Kuantum (ML-KEM-768)      │
│  - DDoS Protection & Anycast BGP Routing (Jakarta, Surabaya, IKN)      │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│                    LAPISAN 2: CLUSTER KONTROL & KOMPUTASI              │
│  ┌──────────────────────────────┐    ┌──────────────────────────────┐  │
│  │ WidyaControl (Admin Plane)   │    │ WidyaGrid Compute Nodes      │  │
│  │ - Pengelola Akun & Kuota 1 TB│    │ - MicroVM Firecracker / WASI │  │
│  │ - Integrasi Pembayaran QRIS  │    │ - Eksekutor Mail Merge & AI  │  │
│  └──────────────┬───────────────┘    └──────────────┬───────────────┘  │
│                 │                                   │                  │
│                 └─────────────────┬─────────────────┘                  │
└───────────────────────────────────┼────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│                  LAPISAN 3: STORAGE & KONSENSUS DATA                   │
│  ┌──────────────────────────────┐    ┌──────────────────────────────┐  │
│  │ WidyaDrive Distributed Store │    │ WidyaBase Metadata & Ledger  │  │
│  │ - NVMe Hot Cache Tier        │    │ - Raft Multi-Node Consensus  │  │
│  │ - HDD Object Tier (8+4 EC)   │    │ - Transaksi Saldo & Hak Akses│  │
│  └──────────────────────────────┘    └──────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Pusat Kendali Pengelolaan: *WidyaControl Plane*

Administrator sistem mengelola seluruh armada server melalui dua antarmuka resmi:

### A. Dashboard Web Pengelola (Single Pane of Glass Web Console)
Tersedia di portal aman intranet instansi (misal: `https://admin.widya.internal`):
```
┌────────────────────────────────────────────────────────────────────────┐
│  🏢 PUSAT KENDALI BACKBONE WIDYACONTROL        [Administrator: BSSN]   │
├────────────────────────────────────────────────────────────────────────┤
│ TOTAL SERVER     PENGGUNA AKTIF     STORAGE TERPAKAI    BEBAN AI GRID  │
│ 32 Node (Sehat)  4.820 Pengguna     382 TB / 1.2 PB     18% (Normal)   │
├────────────────────────────────────────────────────────────────────────┤
│ [ MONITORING KESEHATAN SERVER ]                                        │
│ - Node-JKT-01 s/d 16 (Jakarta Tier 4) : 🟢 100% Normal (CPU 24%, RAM 31%)│
│ - Node-SBY-01 s/d 12 (Surabaya DR)    : 🟢 100% Normal (Replica Pasif)  │
│ - Node-IKN-01 s/d 04 (Nusantara Hub)  : 🟢 100% Normal (Ledger Validator)│
├────────────────────────────────────────────────────────────────────────┤
│ [ OPERASI CEPAT ]                                                      │
│ [ + Tambah Node Baru ]  [ 🔄 Rebalance Storage ]  [ 🛡️ Rotasi Kunci HSM]│
└────────────────────────────────────────────────────────────────────────┘
```

### B. Antarmuka Terminal Operator CLI & TUI (`widya-admin`)
Untuk sysadmin yang terbiasa bekerja melalui SSH:
```bash
# Memeriksa status kesehatan seluruh armada server
widya-admin cluster status

# Menambah server baru ke dalam cluster secara otomatis
widya-admin node gabung --token <TOKEN_PQC_ENKORPORASI> --role storage

# Melihat pemakaian kuota dan transaksi langganan 1 TB
widya-admin langganan ringkasan --format tabel

# Melakukan simulasi kegagalan server (Chaos Testing)
widya-admin chaos simulasi-mati --node Node-JKT-04
```

---

## 4. Pengelolaan Penyimpanan Skala Petabyte: *Erasure Coding 8+4*

Menyimpan data ribuan pengguna dengan kapasitas 1 TB per orang membutuhkan sistem proteksi hardware yang aman tanpa biaya pemborosan replikasi 3x lipat (*3-way replication* yang memakan storage 300%):

- **Teknologi Erasure Coding (Skema 8+4):**
  - Setiap berkas dokumen yang diunggah pengguna dibagi menjadi **8 pecahan data asli** + **4 pecahan paritas kalkulasi**.
  - Pecahan-pecahan ini disebar ke 12 rak/server penyimpanan yang berbeda.
  - **Ketahanan Luar Biasa:** Sistem tetap dapat membaca dan merekonstruksi dokumen pengguna secara utuh meskipun **4 server mati serentak sekaligus**!
- **Pemulihan Otomatis Mandiri (*Self-Healing Storage*):**
  - Jika ada 1 harddisk/server yang rusak di tengah malam, sistem otomatis mendeteksi ketiadaan pecahan blok tersebut, menghitung ulang dari paritas yang tersisa, dan menulis pecahan baru ke server cadangan tanpa perlu membangunkan teknisi.

---

## 5. Orkestrasi Beban Komputasi AI & Mail Merge (Auto-Scaling)

Beban kerja komputasi WidyaGrid (seperti permintaan bantuan AI atau pemrosesan jutaan surat) memiliki karakteristik fluktuatif (sangat tinggi di jam 09.00–16.00 WIB dan rendah di malam hari):

1. **Auto-Scaling Cerdas (*Horizontal Pod/Node Autoscaler*):**
   - Saat jam kerja, sistem secara otomatis meluncurkan ratusan *microVM Firecracker* ([docs/PRODUCTION_DEPLOYMENT_GUIDE.md](file:///home/devops/widya-lang/docs/PRODUCTION_DEPLOYMENT_GUIDE.md)) dalam waktu **< 5 milidetik per-mesin**.
   - Beban Mail Merge diparalelkan ke puluhan node sehingga 100.000 dokumen selesai dalam hitungan detik.
2. **Scale-to-Zero di Malam Hari:**
   - Di luar jam kerja, server komputasi otomatis memasuki mode siaga (*sleep/idle*) untuk menghemat konsumsi listrik hingga 70%.

---

## 6. Prosedur Pencadangan & Pemulihan Bencana (Disaster Recovery - HADR)

Untuk menjamin kepatuhan tingkat pertahanan dan perbankan nasional:
- **Zero Data Loss (RPO = 0):** Seluruh transaksi penulisan dokumen dan perubahan basis data WidyaBase disinkronkan secara kontinu antar-region (Jakarta $\leftrightarrow$ Surabaya) melalui saluran terenkripsi PQC.
- **Waktu Pemulihan Bencana Cepat (RTO < 30 Detik):** Jika data center utama mengalami pemadaman total (misal gempa bumi atau putusnya jalur kabel laut), router BGP dan DNS berdaulat otomatis mengalihkan seluruh lalu lintas pengguna ke data center sekunder dalam waktu **< 30 detik** secara transparan tanpa pengguna merasakan gangguan.
- **Cadangan Anti-Ransomware (WORM Storage):** Salinan cadangan mingguan disimpan dengan proteksi *Write-Once-Read-Many* yang dikunci secara kriptografis; bahkan administrator root tidak dapat menghapus atau mengubah cadangan tersebut selama masa retensi hukum yang ditetapkan.
