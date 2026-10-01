# STANDAR TINGKAT LAYANAN (SLA) DAN PERJANJIAN LISENSI ENTERPRISE
**PT Widya Ekosistem Digital Indonesia (Widya-Lang Corporation)**
*Service Level Agreement (SLA), Definisi Keparahan Insiden, Garansi Uptime, & Klausul Escrow*

---

## 1. TINGKATAN SLA & TARGET KETERSEDIAAN SISTEM (UPTIME GUARANTEE)

| Parameter Layanan | Tier Standard Enterprise | Tier Mission-Critical Banking / Telco |
| :--- | :--- | :--- |
| **Garansi Ketersediaan (Uptime SLA)** | **99.9%** (Maks. downtime 43.8 menit/bulan) | **99.999% ("Five Nines")** (Maks. downtime 26 detik/bulan) |
| **Jam Operasional Dukungan Teknis** | Jam Kerja (Senin - Jumat, 08:00 - 17:00 WIB) | **24 Jam x 7 Hari x 365 Hari Non-Stop** |
| **Saluran Komunikasi Dukungan** | Portal Tiket Helpdesk & Email Resmi | Hotline Darurat Khusus, Dedicated Slack/Telegram VIP, On-Site Standby |
| **Dedicated Technical Account Manager** | Tidak Termasuk | **Termasuk (1 Orang Senior Principal Architect)** |
| **Hot-Patch Security Release** | Maksimal 48 Jam | **Maksimal 4 Jam sejak Kerentanan Teridentifikasi** |

---

## 2. MATRIKS RESPON DAN RESOLUSI INSIDEN (INCIDENT SEVERITY MATRIX)

```mermaid
graph TD
    Inc[Insiden Dilaporkan Klien] --> P1{P1: Sistem Utama Lumpuh / Transaksi Finansial Berhenti?}
    P1 -- Ya --> ResP1[Respon < 15 Menit | Resolusi < 2 Jam | Tim Krisis Dibentuk]
    P1 -- Tidak --> P2{P2: Fungsi Mayor Terganggu, Kinerja Turun Signifikan?}
    P2 -- Ya --> ResP2[Respon < 30 Menit | Resolusi < 6 Jam]
    P2 -- Tidak --> P3{P3: Fitur Minor Terkendala, Sistem Tetap Operasional?}
    P3 -- Ya --> ResP3[Respon < 2 Jam | Resolusi < 24 Jam]
    P3 -- Tidak --> ResP4[P4: Pertanyaan Teknis / Konsultasi Fitur | Respon < 8 Jam]
```

### Rincian Waktu Respon & Eskalasi:

| Level Keparahan | Deskripsi Insiden | Waktu Respon Awal | Target Resolusi / Workaround |
| :--- | :--- | :--- | :--- |
| **P1 - Critical (Darurat)** | Cluster basis data mati total, kegagalan transaksi QRIS/BI-FAST perbankan, sistem switching offline. | **< 15 Menit** | **< 2 Jam** |
| **P2 - Major (Kritis)** | Sebagian node cluster mati, latensi pemrosesan naik drastis, fungsi laporan vital tidak merespon. | **< 30 Menit** | **< 6 Jam** |
| **P3 - Minor (Sedang)** | Galat minor pada fitur non-esensial, ada solusi alternatif (*workaround*) yang dapat digunakan sementara. | **< 2 Jam** | **< 24 Jam** |
| **P4 - Informational (Rendah)**| Pertanyaan panduan konfigurasi, permintaan fitur baru, pertanyaan umum sintaks bahasa. | **< 8 Jam** | Sesuai jadwal rilis berikutnya |

---

## 3. SKEMA KOMPENSASI SERVICE CREDIT JIKA SLA TIDAK TERCAPAI

Jika ketersediaan bulanan berada di bawah batas komitmen kontrak:

```
+------------------------------------+---------------------------------------+
| Uptime Bulanan Aktual              | Kompensasi Service Credit Klien       |
+------------------------------------+---------------------------------------+
| 99.5% - < 99.9% (Standard)         | 10% Pengurangan Tagihan Bulan Tersebut|
| 99.0% - < 99.5%                    | 25% Pengurangan Tagihan Bulan Tersebut|
| < 99.0%                            | 50% Pengurangan Tagihan Bulan Tersebut|
| < 99.999% (Mission-Critical Tier)  | 30% Pengurangan Tagihan Bulan Tersebut|
+------------------------------------+---------------------------------------+
```

---

## 4. KLAUSUL JAMINAN KEDAULATAN & SOURCE CODE ESCROW (KHUSUS PERBANKAN & BUMN)

Untuk memenuhi standar kepatuhan Otoritas Jasa Keuangan (OJK) dan regulasi ketahanan infrastruktur informasi vital nasional:

1. **Penyimpanan Kode Sumber di Lembaga Escrow Netral**:
   - Seluruh kode sumber kompiler, runtime, dan engine basis data Widya-Lang disimpan di lembaga notaris escrow terakreditasi nasional di Indonesia.
2. **Kondisi Pelepasan Kode Sumber (*Trigger Events*)**:
   - Klien enterprise berhak atas akses penuh terhadap salinan kode sumber jika PT Widya Ekosistem Digital Indonesia mengalami kepailitan likuidasi atau menghentikan dukungan produk secara sepihak (*End-of-Life Unilateral*).
3. **Kepatuhan Data Processing Agreement (DPA) & UU PDP**:
   - Widya-Lang menjamin bahwa tidak ada telemetri data sensitif transaksi nasabah atau rekam medis yang dikirim ke luar server on-premise milik klien.

---
