# 🥋 ArenaSilat.id - Platform Digital Pencak Silat Berdaulat Indonesia

[![Widya-Lang](https://img.shields.io/badge/Language-Widya--Lang%202026-blue.svg)](file:///home/devops/widya-lang)
[![Regulasi](https://img.shields.io/badge/Regulasi-PERSILAT%20%26%20IPSI%202026-red.svg)](file:///home/devops/widya-lang/projects/arenasilat.id/docs/pembaruan_peraturan_ipsi_persilat_2026.md)
[![Status](https://img.shields.io/badge/Status-Production%20Ready-brightgreen.svg)]()

**ArenaSilat.id** adalah platform *SaaS Digital Sports Management* dan penyiaran langsung (*Live Broadcast MCR*) resmi untuk cabang olahraga Pencak Silat. Dibangun 100% di atas ekosistem bahasa pemrograman dan basis data berdaulat **Widya-Lang**, sistem ini dirancang untuk kejuaraan tingkat daerah (Kejurcab/Kejurda), nasional (Kejurnas/PON), hingga turnamen internasional standar Olimpiade.

---

## 🌟 Fitur Unggulan Platform

### 1. Digital Scoreboard & Konsensus 5 Juri (PERSILAT 2026)
- **Aturan Skor Tanding**: Pukulan (+1), Tendangan (+2), Bantingan/Jatuhan Sah (+3).
- **Hukuman Wasit Matras**: Teguran 1 (-1), Teguran 2 (-2), Peringatan 1 (-5), Peringatan 2 (-10), Binaan, dan Diskualifikasi.
- **Konsensus Juri Real-time**: Poin disahkan otomatis jika minimal 3 dari 5 juri (atau 2 dari 3 juri) menekan tombol dalam jendela toleransi waktu 1.5 detik.
- **Sintesis Audio Web Audio API Bawaan**: Suara Gong tradisional Jawa/Silat saat babak dimulai/selesai dan peluit wasit frekuensi tinggi tanpa dependensi file eksternal (*100% offline & air-gapped ready*).

### 2. Sistem Video Assistant Referee (VAR) Dewan Hakim
- **Kuota 2 Kartu Protes**: Sudut Merah dan Sudut Biru masing-masing dibekali 2 kartu protes resmi.
- **Aturan Protes**: Jika protes diterima oleh Dewan Juri, kartu **tetap utuh**; jika ditolak, kartu **hangus**.
- **Player Replay Slow-Motion Multi-Angle**: Canvas video interaktif gerak lambat (0.25x dan 0.5x) dengan penanda waktu (*timestamp*) dan pilihan sudut kamera.

### 3. Monitoring 4 Gelanggang Paralel (Multi-Court Grid)
- Pemantauan 4 matras gelanggang sekaligus secara simultan dengan sinkronisasi skor dan status partai secara *real-time*.

### 4. Bagan Turnamen Otomatis (Interactive Bracket Tree)
- Bagan eliminasi $2^n$ (Perempat Final, Semifinal, Grand Final).
- Algoritma *Seeding Separation* (pemisahan kontingen se-daerah di babak penyisihan awal).
- Klik interaktif pada nama pesilat untuk memajukan pemenang secara otomatis.

### 5. Timbangan Digital Bluetooth BLE & Marshalling Antrean
- Integrasi timbangan badan via Web Bluetooth API (W3C Standard) bebas ketik manual.
- Verifikasi batas kelas tanding (contoh: Kelas C 55–60 Kg) dalam 5 detik per atlet.
- Antrean ruang tunggu (Marshalling H-3 partai) dan checklist pelindung tubuh.

### 6. Sport Science & Peringkat Nasional (Weighted-Elo Rating)
- Database pesilat nasional terverifikasi NIK & BPJS Ketenagakerjaan.
- Profiling 6 indikator biomotorik (Speed, Power, VO2Max, Agility, Flexibility, Reaction Time).

### 7. E-Sertifikat Juara & Anti-Fraud Puspresnas
- Penerbitan piagam vektor digital berbingkai emas dengan stempel tanda tangan digital berdaulat.
- Kode QR dengan segel kriptografi **HMAC SHA-256** lolos verifikasi SIMT Puspresnas Kemendikdasmen (siap untuk jalur prestasi PPDB).

### 8. Mode Siaran Layar Kaca & OBS Overlay Transparan
- 1-Klik **"Mode OBS"** mengubah seluruh latar belakang menjadi transparan (*Chroma/Alpha Key*), menyisakan grafis *scorebug* dan *lower-third* untuk dimasukkan ke OBS Studio / vMix / YouTube Live.

### 9. Konsol Dokter Medis (3 Menit)
- Hitung mundur kumulatif 3 menit di atas matras sesuai standar medis PERSILAT.
- Wewenang penghentian darurat (*Doctor Stoppage Authority*) dan penerbitan berkas rujukan PLKK BPJS Ketenagakerjaan.

### 10. Konsol Penilaian Seni TGR (Tunggal, Ganda, Regu) PERSILAT 2026
- **Hitung Mundur Seni 3 Menit (180 Detik)**: Visualisasi *Safe Zone Bar* (02:55 - 03:05 bebas penalti), deteksi otomatis deviasi waktu kurang/lebih.
- **Kebenaran 100 Gerakan Baku IPSI**: Nilai dasar 9.90, pengurangan instan -0.01 per kesalahan gerakan, dan input kemantapan/penjiwaan.
- **Penalti Pelanggaran Teknis**: Tombol cepat senjata jatuh/patah (-0.5), keluar arena (-0.5), pakaian/atribut rusak (-0.5), dan vokal ilegal (-0.5).
- **Tabulasi 5 Juri Median PERSILAT**: Membuang nilai tertinggi dan terendah, menghasilkan skor resmi rata-rata 3 juri tengah.

### 11. Konsol Dewan Hakim & Forensic Black-Box Audit Logger
- **Waveform Timeline Milidetik**: Grafik forensik penekanan tombol Juri 1..5 untuk membuktikan kuorum konsensus < 1.5 detik saat insiden bantingan.
- **Sidang Banding Resmi**: Formulir banding ofisial kontingen, verifikasi uang jaminan protes resmi (Rp 3.000.000), disposisi majelis hakim (Diterima / Ditolak).
- **Cetak BAP Resmi Ber-QR Code**: Penerbitan Berita Acara Pemeriksaan (BAP) sidang banding ber-QR code dan hash HMAC-SHA256 untuk antisipasi sengketa ke PB IPSI / BAORI.

### 12. Bagan Pool Pemasalan (Pool 4 Pesilat Ramah Anak) & Ekspor Puspresnas
- **Mode Switcher Bagan**: Beralih instan antara **Bagan Prestasi Eliminasi ($2^n$)** dan **Pool Pemasalan (4 Anak)**.
- **Pembinaan Karakter Ramah Anak**: Seluruh 4 pesilat di setiap pool dipastikan meraih medali (🥇 Emas, 🥈 Perak, dan 🥉 Dua Perunggu Bersama).
- **Ekspor SIMT Puspresnas 1-Klik**: Unduhan CSV/Excel resmi berformat SIMT Kemendikdasmen (NISN, NPSN sekolah, nomor SK kejuaraan, hash verifikasi) untuk jalur prestasi PPDB.

### 13. Portal Kontingen Mandiri & Simulasi Billing Xendit
- **Pendaftaran Roster Mandiri**: Entri data pesilat, NIK terproteksi UU PDP, NISN, kategori tanding/seni, dan upload berkas kesehatan.
- **Faktur Tagihan Otomatis**: Integrasi payment gateway simulasi Dynamic QRIS Bank Indonesia (NMID resmi) dan Virtual Account multi-bank (BCA, Mandiri, BRI, BNI, BSI).
- **Automated Split Disbursement (xenPlatform)**: 95% dana pendaftaran langsung cair ke rekening Panitia Pelaksana EO, 5% dialokasikan untuk SaaS fee platform.

---

## 📂 Struktur Direktori Proyek

```text
projects/arenasilat.id/
├── widya.toml             # Konfigurasi paket & metadata proyek Widya
├── server.py              # Server web HTTP port 8080
├── jalankan.sh            # Script eksekusi satu-klik (Backend Widya + Web Suite)
├── sumber/                # Kode sumber logika murni Widya-Lang (*.wya)
│   ├── utama.wya          # Titik masuk utama & pengujian seluruh modul
│   ├── skor_persilat.wya  # Mesin aturan penilaian Tanding PERSILAT, VAR & Konsensus
│   ├── seni_persilat.wya  # Mesin aturan penilaian Seni TGR & Formula Penalti Waktu
│   ├── bracket_generator.wya # Generator bagan eliminasi & pool pemasalan 4 pesilat
│   ├── dewan_hakim.wya    # Sidang banding dewan hakim, audit black-box & uang jaminan
│   ├── kejuaraan.wya      # Struktur data kontingen, atlet, dan turnamen
│   └── widyabase_skema.wya# Skema data entitas relasional WidyaBase
├── web/                   # Antarmuka Web Suite Terpadu (HTML5, Vanilla CSS, JS)
│   ├── index.html         # Portal publik & landing page
│   ├── arena.html         # Konsol gelanggang terpadu 11-in-1
│   ├── style.css          # Desain cyberpunk-traditional dark mode glassmorphism
│   └── app.js             # Engine state, Web Audio API, VAR Replay, TGR, Black-Box
└── docs/                  # 53 Dokumen arsitektur, regulasi, ERD, dan analisis
```


---

## 🚀 Panduan Menjalankan

### Cara 1: Menjalankan Sekaligus (Rekomendasi)
```bash
cd /home/devops/widya-lang/projects/arenasilat.id
./jalankan.sh
```

### Cara 2: Menjalankan Backend Engine Widya-Lang Saja
```bash
cd /home/devops/widya-lang/projects/arenasilat.id
widya run sumber/utama.wya
```

### Cara 3: Menjalankan Server Web Portal Saja
```bash
cd /home/devops/widya-lang/projects/arenasilat.id
python3 server.py
```
Akses web browser di: **`http://localhost:8080`**

---

## 📜 Kepatuhan Regulasi & Standar
- **Aturan Olahraga**: Regulasi Pertandingan PB IPSI / PERSILAT 2026 (*Road to Olympic*).
- **Hukum & Keamanan**: UU Perlindungan Data Pribadi (UU No. 27/2022) dan UU Keolahragaan (UU No. 11/2022).
- **Kurasi Nasional**: Standar SIMT Pusat Prestasi Nasional (Puspresnas) Kemendikdasmen.
