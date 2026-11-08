# Skema Monetisasi Terjangkau, Inklusif & Berkeadilan (Affordable & Inclusive Pricing Strategy) Arena Silat SaaS

Dokumen ini memuat analisis dan rancangan struktur harga (*pricing structure*) yang **sangat terjangkau, adil (*fair*), tanpa modal awal (*Zero Upfront Cost*), dan ramah anggaran (*budget-friendly*)** bagi seluruh tingkatan **Penyelenggara Event (EO Grassroot s.d. Nasional)** maupun **Perguruan Pencak Silat (Tingkat Ranting Sekolah, Cabang Daerah, hingga Pengurus Pusat DPP)** di seluruh Indonesia.

---

## 1. Filosofi & Prinsip Penetapan Harga Inklusif

```mermaid
graph TD
    PRICING[Filosofi Monetisasi Terjangkau Arena Silat] --> P1[1. Prinsip Zero Upfront Risk / Modal Awal Rp 0]
    PRICING --> P2[2. Prinsip Pay-As-You-Grow / Bayar Sesuai Skala Peserta]
    PRICING --> P3[3. Prinsip Micro-Fee Terjangkau / Setara Harga Secangkir Kopi]
    PRICING --> P4[4. Prinsip Freemium untuk Ranting Pelosok / Sekolah]

    P1 --> P1_A["EO & Ranting bisa pakai software di H-90 tanpa bayar sepeser pun di awal"]
    P2 --> P2_A["Event kecil 100 atlet bayar sedikit, event besar 1.000 atlet bayar proporsional"]
    P3 --> P3_A["Biaya sistem Rp 2.000 - Rp 10.000 / atlet (Sangat terjangkau dibanding biaya pendaftaran Rp 250rb)"]
    P4 --> P4_A["Fitur presensi latihan harian ranting GRATIS SELAMANYA"]
```

---

## 2. Struktur Tarif Terjangkau untuk Penyelenggara Event Turnamen (EO)

Penyelenggara turnamen silat (sekolah, klub, pengcab) sering kali memiliki modal terbatas. Oleh karena itu, disediakan skema yang tidak mengganggu arus kas (*cash flow*):

```mermaid
graph LR
    subgraph 3 Opsi Paket Tarif Penyelenggara Event
        EO_A[Opsi A: Pay-As-You-Earn / Potong Pendaftaran]
        EO_B[Opsi B: Paket Sewa Event Flat Terjangkau]
        EO_C[Opsi C: Paket Komunitas / Sekolah Mandiri]
    end

    EO_A --> OA["Rp 10.000 / atlet terdaftar (Dipotong otomatis saat peserta bayar online, TANPA MODAL AWAL)"]
    EO_B --> OB["Mulai Rp 1.500.000 / event (Aktif 120 hari penuh dari H-90 s.d. H+30, bebas atlet)"]
    EO_C --> OC["GRATIS untuk Kejuaraan Internal Sekolah / Ranting (< 50 atlet, 1 Gelanggang)"]
```

### Rincian Paket Penyelenggara Event (EO):

| Paket Event | Target Pengguna | Kapasitas & Fitur | Biaya Layanan SaaS |
|---|---|---|---|
| **Komunitas / Sekolah (Free Tier)** | Kejuaraan Antar-Kelas / Ranting Internal | • 1 Gelanggang<br>• Hingga 60 Atlet<br>• Scoring Digital Tanding/TGR<br>• Cetak Bagan Manual | **GRATIS (Rp 0)** *(Bentuk kontribusi sosial Arena Silat untuk pembinaan silat usia dini)* |
| **Pay-As-You-Earn (Paling Populer)** | Kejuaraan Terbuka / Open Pelajar / Kejurcab | • Multi-Gelanggang (Bebas)<br>• Pendaftaran Online QRIS/VA<br>• Auto-Bracket & Seeding<br>• Jury Pad PWA & TV Scoreboard<br>• E-Piagam QR Code Anti-Pemalsuan | **Rp 10.000 / atlet** *(Dipotong otomatis dari biaya pendaftaran peserta Rp 250rb - Rp 300rb)*<br>**MODAL AWAL: Rp 0** |
| **Sewa Flat Event Pass (120 Hari)** | Pengcab / Pengprov IPSI / EO Profesional | • Unlimited Atlet & Kategori<br>• Masa Aktif 120 Hari (H-90 s.d. H+30)<br>• Video Review VAR & OBS Stream Overlay<br>• Ekspor Format SIMT Puspresnas | **Mulai Rp 1.500.000 – Rp 3.500.000 / event** *(Bisa DP Rp 500rb di H-90)* |

---

## 3. Struktur Tarif Terjangkau untuk Perguruan Pencak Silat (Guild / Dojo)

Untuk mendukung digitalisasi ribuan perguruan silat tradisional di Indonesia tanpa memberatkan kas:

```mermaid
graph LR
    subgraph 3 Tingkatan Tarif Perguruan Silat
        G_FREE[Tier 1: Ranting / Dojang Sekolah]
        G_CAB[Tier 2: Pengurus Cabang / Pengcab Kota-Kab]
        G_DPP[Tier 3: Pengurus Pusat DPP / PB Nasional]
    end

    G_FREE --> GF["GRATIS SELAMANYA: Presensi latihan harian siswa & jadwal latihan ranting"]
    G_CAB --> GC["Rp 350.000 / TAHUN per Cabang: Database atlet cabang + Radar analisis taktik pelatih"]
    G_DPP --> GD["Rp 5.000.000 - Rp 10.000.000 / TAHUN: Monitoring se-Indonesia & KTA digital nasional"]
```

### Rincian Paket Perguruan Silat:

| Tingkatan Perguruan | Target Pengguna | Fitur yang Diperoleh | Biaya Langganan |
|---|---|---|---|
| **Ranting / Dojang (Free Tier)** | Pelatih Ranting Sekolah / Padepokan Desa | • Presensi Siswa via Scan QR Code HP<br>• Jadwal Latihan Rutin Mingguan<br>• Catatan Materi Jurus Latihan | **GRATIS SELAMANYA (Rp 0)** |
| **Cabang Daerah (Pengcab)** | Pengurus Cabang Kota / Kabupaten | • Database seluruh atlet se-Kota/Kab<br>• Radar Taktik & Analisis Kelemahan Atlet<br>• Pendaftaran Massal Turnamen 1-Klik<br>• Akses Approval Ujian Kenaikan Sabuk | **Rp 350.000 / tahun** *(Hanya ~Rp 29.000 / bulan, sangat terjangkau bagi kas cabang)* |
| **Pengurus Daerah (Pengda)** | Pengurus Tingkat Provinsi | • Multi-Cabang Leaderboard Provinsi<br>• Scouting Atlet Berbakat Pelatda<br>• Monitoring Ranting se-Provinsi | **Rp 1.500.000 / tahun** per Provinsi |
| **Pengurus Pusat (DPP / PB)** | Dewan Pimpinan Pusat Perguruan Nasional | • **Peta Monitoring Latihan se-Indonesia**<br>• Master Database Siswa & Sabuk Nasional<br>• KTA Digital Ber-QR Code Terpusat<br>• Modul Kejurnas Internal Perguruan | **Rp 5.000.000 – Rp 10.000.000 / tahun** *(Sangat murah untuk mengelola puluhan ribu anggota se-Indonesia)* |

---

## 4. Skema Mikro-Fee Ujian Kenaikan Tingkat (UKT / Sabuk Digital): Hanya Rp 2.500 / Siswa

Saat perguruan menggelar Ujian Kenaikan Sabuk (UKT):
- **Biaya Layanan Sistem**: Hanya **Rp 2.500 – Rp 3.500 / siswa peserta UKT**.
- **Nilai yang Didapat Perguruan & Siswa**:
  - Pendaftaran ujian online & cetak nomor ujian.
  - Form input nilai digital untuk dewan penguji sabuk di lapangan.
  - Penerbitan otomatis **E-Sertifikat Ijazah Kenaikan Sabuk ber-QR Code Anti-Pemalsuan** yang tersimpan permanen di akun siswa.
- **Sangat Terjangkau**: Jika iuran UKT siswa adalah Rp 75.000 – Rp 150.000, biaya sistem Rp 2.500 hanya mengambil **$< 3\%$** dari total iuran!

---

## 5. Simulasi Perbandingan Biaya Tradisional vs Menggunakan Arena Silat

### A. Untuk Penyelenggara Turnamen (Event 500 Atlet):
* **Sistem Manual / Cetak Kertas Jadul**:
  - Cetak piagam & ID card: Rp 3.500.000
  - Kertas bagan & ATK timbang: Rp 750.000
  - Jasa pembuatan bagan manual: Rp 1.500.000
  - Sewa hardware scoring manual jadul: Rp 4.000.000
  - **Total Biaya Tradisional: Rp 9.750.000**
* **Menggunakan Arena Silat (Model Pay-As-You-Earn)**:
  - Biaya SaaS (500 atlet x Rp 10.000): **Rp 5.000.000** *(Dipotong otomatis dari uang pendaftaran, tanpa modal awal di H-90)*.
  - **Panitia HEMAT Rp 4.750.000 (Hemat 48%) + Mendapatkan Sistem Digital Real-time, VAR Kamera HP, dan Sertifikat Anti-Fraud!**

---

### B. Untuk Pengurus Cabang Perguruan Silat (1 Tahun):
* **Biaya Langganan Arena Silat**: **Rp 350.000 / tahun**.
* **Keuntungan**:
  - Menghemat jutaan rupiah biaya cetak blangko sertifikat sabuk.
  - Mencegah kebocoran dana iuran ujian sabuk oleh ranting ilegal.
  - Memiliki database digital seluruh pesilat cabang yang rapi dan profesional.
