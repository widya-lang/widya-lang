# Analisis Kelayakan Finansial & Kalkulasi Titik Impas (BEP) 📊📈

> **Studi Kelayakan Bisnis Berlangganan WidyaWorkspace Rp 50.000 / Bulan (Kapasitas 1 TB Storage + WidyaMail)**  
> *Break-Even Point (BEP) · Rasio Utilisasi Riil Industri · Analisis Biaya Tetap & Variabel · Margin Laba Bersih*

---

## 1. Pertanyaan Kunci: Apakah Tarif Rp 50.000 / Bulan Feasible?

**JAWABAN TEGAS: SANGAT FEASIBLE DAN SANGAT MENGUNTUNGKAN (HIGHLY PROFITABLE).**

Banyak orang mengira jika memberi kuota 1 TB kepada 100 pengguna, kita harus membeli 100 TB harddisk fisik sejak hari pertama seharga ratusan juta rupiah. **Ini adalah kesalahpahaman umum.**

Dalam industri penyimpanan awan global (*seperti Google Drive, Dropbox, Microsoft OneDrive, dan iCloud*), model bisnis cloud storage beroperasi di atas hukum statistik:
1. **Rasio Utilisasi Riil (Industry Average Real Utilization):**  
   Ketika pengguna diberi kuota 1.000 GB (1 TB), rata-rata pengguna perkantoran biasa **hanya menggunakan 20 GB hingga 50 GB** di tahun pertama (sebagian besar berupa dokumen teks, spreadsheet, dan slide presentasi yang berukuran kilobyte).
2. **Deduplikasi Tingkat Blok (*Block-Level Deduplication*):**  
   Jika 50 staf menyimpan salinan dokumen regulasi atau lampiran PDF pengumuman yang sama berukuran 100 MB, sistem hanya menyimpan file tersebut **1 kali di harddisk fisik**, menghemat 98% ruang disk.
3. **Kompresi Zstandard Real-Time:**  
   Seluruh data teks dan tabel dikompresi otomatis, memangkas ukuran fisik sebesar 30–50%.

> 💡 **Kesimpulan Teknis:** Untuk 1 pengguna berkuota 1 TB, konsumsi kapasitas disk fisik riil di server rata-rata hanya memakan **~30 GB hingga 40 GB storage fisik**.

---

## 2. Struktur Biaya Operasional (Cost Breakdown)

Mari kita hitung secara ketat menggunakan harga riil pasar cloud saat ini (menggunakan arsitektur Hetzner / Cloudflare):

### A. Biaya Tetap (Fixed Cost - $FC$) per Bulan
Biaya yang harus dibayar tidak peduli ada 1 pengguna atau 200 pengguna:

| Komponen Biaya Tetap | Rincian Layanan | Biaya per Bulan |
| :--- | :--- | :--- |
| **Server Inti Dedicated** | AMD Ryzen 7 12-Core, 64 GB RAM, 2x 1.92 TB NVMe (Hetzner) | ~Rp 800.000 |
| **Domain & WAF Security** | Domain `widya.email` + Cloudflare DNS/WAF | ~Rp 150.000 |
| **TOTAL BIAYA TETAP ($FC$)**| Melayani seluruh traffic sistem & email | **Rp 950.000 / bulan** |

---

### B. Biaya Variabel (Variable Cost - $VC$) per Pengguna per Bulan
Biaya yang timbul seiring bertambahnya setiap 1 pengguna baru:

| Komponen Biaya Variabel | Dasar Perhitungan | Biaya per User / Bulan |
| :--- | :--- | :--- |
| **Sewa Storage Fisik Riil** | 10 TB Storage Box = Rp 410.000/bln sanggup menampung 200 user aktif (~50 GB/user) | Rp 2.050 / user |
| **Biaya Gateway Pembayaran** | Biaya resmi MDR QRIS Bank Indonesia (0,7% per transaksi Rp 50.000) | Rp 350 / user |
| **Cadangan I/O Bandwidth** | Cadangan traffic unduhan | Rp 600 / user |
| **TOTAL BIAYA VARIABEL ($VC$)**| Biaya langsung per pelanggan aktif | **Rp 3.000 / user / bulan** |

---

## 3. Kalkulasi Titik Impas (Break-Even Point - BEP)

Rumus standar manajemen keuangan untuk menghitung titik impas:

$$\text{BEP (Jumlah Pengguna)} = \frac{\text{Biaya Tetap (Fixed Cost)}}{\text{Harga Jual per Pengguna} - \text{Biaya Variabel per Pengguna}}$$

### Variabel yang Digunakan:
- **Harga Jual ($P$):** Rp 50.000 / bulan
- **Biaya Tetap ($FC$):** Rp 950.000 / bulan
- **Biaya Variabel ($VC$):** Rp 3.000 / user / bulan
- **Margin Kontribusi ($P - VC$):** $\text{Rp } 50.000 - \text{Rp } 3.000 = \mathbf{\text{Rp } 47.000} \text{ per pengguna}$.

### Perhitungan BEP:
$$\text{BEP} = \frac{\text{Rp } 950.000}{\text{Rp } 47.000} = \mathbf{20,2 \approx 21 \text{ Pengguna!}}$$

> 🎯 **KESIMPULAN BEP:**  
> **Hanya dengan 21 PENGGUNA BERLANGGANAN, seluruh biaya sewa server, domain, dan infrastruktur backend SUDAH BALIK MODAL (LUNAS)!**  
> Pengguna ke-22 dan seterusnya adalah **MURNI KEUNTUNGAN BERSIH (NET PROFIT)**.

---

## 4. Proyeksi Keuntungan Bersih (Profitability Projection)

Berikut adalah proyeksi finansial berdasarkan skala pertumbuhan pengguna:

```
┌────────────────────────────────────────────────────────────────────────┐
│               PROYEKSI LABA BERSIH BERLANGGANAN WIDYAWORKSPACE         │
├─────────┬────────────────┬────────────────┬───────────────┬────────────┤
│ Jumlah  │ Pendapatan     │ Total Biaya    │ Laba Bersih   │ Margin     │
│ User    │ Kotor / Bulan  │ Server / Bulan │ (Net Profit)  │ Laba (%)   │
├─────────┼────────────────┼────────────────┼───────────────┼────────────┤
│ **21**  │ Rp 1.050.000   │ Rp 1.013.000   │ **Rp 37.000** │ **Titik BEP│
│ **50**  │ Rp 2.500.000   │ Rp 1.100.000   │ **Rp 1.400.000│ **56,0%**  │
│ **100** │ Rp 5.000.000   │ Rp 1.250.000   │ **Rp 3.750.000│ **75,0%**  │
│ **250** │ Rp 12.500.000  │ Rp 1.700.000   │ **Rp 10.800.000*| **86,4%**│
│ **500** │ Rp 25.000.000  │ Rp 2.800.000   │ **Rp 22.200.000*| **88,8%**│
│ **1.000**│ Rp 50.000.000 │ Rp 5.500.000*  │ **Rp 44.500.000*| **89,0%**│
└─────────┴────────────────┴────────────────┴───────────────┴────────────┘
```
*\*Catatan: Pada 1.000 pengguna, biaya server sudah ditingkatkan mencakup cluster dedicated server 64-core dengan kapasitas storage fisik 40 TB.*

---

## 5. Manajemen Risiko: Bagaimana Jika Ada "Power User" yang Menghabiskan Full 1 TB?

Bagaimana jika ada pengguna yang benar-benar mengisi penuh 1.000 GB dengan video 4K?
1. **Subsidi Silang Alami:**  
   Dari 100 pengguna, rata-rata 90 pengguna hanya memakai < 30 GB (menghasilkan laba Rp 45.000+ per user). Keuntungan dari 90 pengguna ini melimpah ruah untuk menutupi biaya 5–10 power user.
2. **Kebijakan Pemakaian Wajar (*Fair Usage Policy - FUP*):**  
   - Kuota 1 TB tetap diberikan penuh tanpa batas.
   - Jika pengguna mengunggah lebih dari 100 GB dalam waktu 1 jam, kecepatan bandwidth diatur menjadi 20 Mbps (*traffic shaping*) agar tidak mengganggu kestabilan pengguna lain.
3. **Penyimpanan Berkas Dingin (*Cold Storage Tiering*):**  
   Berkas yang tidak pernah dibuka selama lebih dari 90 hari otomatis dipindahkan ke media penyimpanan dingin (*Wasabi / Deep Archive*) yang biayanya hanya ~$1 / TB / bulan.

---

## 6. Ringkasan Eksekutif

Skema tarif **Rp 50.000 per bulan** untuk **WidyaWorkspace (1 TB Storage + WidyaMail `@widya.email` + WidyaOffice Suite)**:
- **Sangat Murah bagi Konsumen:** 50% lebih murah daripada Microsoft 365 (Rp 96.000) dan 70% lebih murah daripada Google Workspace (Rp 180.000).
- **Titik Impas (BEP) Sangat Cepat:** Tercapai hanya dengan **21 pengguna aktif**.
- **Margin Keuntungan Sangat Tinggi:** Menghasilkan margin laba bersih di atas **75% – 88%** seiring bertambahnya pelanggan.
