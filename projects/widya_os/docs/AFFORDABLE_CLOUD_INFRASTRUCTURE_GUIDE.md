# Panduan Infrastruktur Cloud Terjangkau (Affordable Cloud Hosting) ☁️💰

> **Rekomendasi Arsitektur Server Hemat Biaya Sebelum Memiliki On-Premises Mandiri**  
> *Hetzner Dedicated · Cloudflare R2 (Zero Egress) · IDCloudHost Jakarta · Anggaran Mulai Rp 1 Juta/Bulan*

---

## 1. Pendahuluan

Sebelum memiliki ruang server fisik sendiri (*On-Premises Bare-Metal* di kantor), memilih penyedia hosting yang tepat adalah faktor penentu kelangsungan finansial proyek.

Sebagian besar startup gagal karena tergiur menggunakan **AWS, Google Cloud, atau Azure** di awal, yang mengenakan biaya sangat mahal untuk sewa CPU mentah, NVMe disk, dan biaya unduh bandwidth keluar (*egress fees*).

WidyaOS dan WidyaWorkspace dirancang dengan efisiensi biner Rust/Widya yang sangat hemat RAM dan CPU, sehingga dapat berjalan di infrastruktur berbiaya sangat terjangkau namun berkinerja tinggi.

---

## 2. Perbandingan Provider Cloud Ramah Anggaran

| Provider & Lokasi | Spesifikasi & Layanan | Estimasi Biaya Bulanan | Keunggulan Utama |
| :--- | :--- | :--- | :--- |
| **Hetzner Dedicated** *(Singapura / Jerman)* | AMD Ryzen 7/9 12-Core, 64 GB DDR5 RAM, 2x 1.92 TB NVMe Gen4 | **~Rp 750.000 – Rp 850.000 / bln** (€39 – €49) | **Raja Rasio Harga/Performa Global**. CPU kencang untuk WidyaNeural & Mail Merge. |
| **Hetzner Storage Box** *(Arsip/Drive)* | 1 TB = €3.80 (~Rp 65rb), 5 TB = €12.50 (~Rp 215rb), 10 TB = €24 (~Rp 410rb) | **~Rp 215.000 / bln (untuk 5 TB)** | Sangat murah untuk penyimpanan berkas dokumen sekunder. |
| **Cloudflare R2** *(Global CDN Edge)* | Object Storage S3-Compatible, $0.015/GB | **Pay-As-You-Use (Gratis Egress $0)** | **Bebas biaya unduh bandwidth**, integrasi CDN global instan. |
| **IDCloudHost** *(Jakarta, Indonesia)* | Dedicated Server Tier 3 Cyber Building Jakarta, IIX OpenIXP 10 Gbps | **Mulai ~Rp 1.500.000 / bln** | **Data Center di Indonesia**, patuh 100% UU PDP & regulasi perbankan/pemerintah. |
| **Wasabi Storage** *(Object Store)* | $6.99 per 1 TB Flat Rate | **~Rp 115.000 / TB / bln** | Tanpa biaya API call dan tanpa biaya transfer keluar. |

---

## 3. Rekomendasi Arsitektur Tahap Awal (Total Biaya < Rp 1,5 Juta/Bulan)

```
┌────────────────────────────────────────────────────────────────────────┐
│             ARSITEKTUR BACKBONE HEMAT TAHAP AWAL (< RP 1,5 JT/BLN)     │
├─────────────────────┬───────────────────────┬──────────────────────────┤
│ Komponen            │ Provider yang Dipakai │ Biaya / Bulan            │
├─────────────────────┼───────────────────────┼──────────────────────────┤
│ **DNS, DDoS & WAF** │ **Cloudflare Free**   │ **Rp 0 (Gratis)**        │
│ **Server Inti & AI**│ **Hetzner Dedicated** │ **~Rp 800.000 / bulan**  │
│ (WidyaMail & Engine)│ (AMD Ryzen 12-Core)   │                          │
│ **Storage Drive**   │ **Hetzner Storage /** │ **~Rp 410.000 / bulan**  │
│ (WidyaWorkspace 1TB)│ **Cloudflare R2**     │ *(Kapasitas 10.000 GB)*  │
├─────────────────────┼───────────────────────┼──────────────────────────┤
│ **TOTAL BIAYA**     │ Sanggup 200–500 User  │ **~Rp 1.210.000 / bulan**│
└─────────────────────┴───────────────────────┴──────────────────────────┘
```
