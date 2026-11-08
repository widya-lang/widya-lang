# Spesifikasi Utilitas Bawaan WidyaOS (Built-in Utilities Suite) 🧰

> **Katalog Perangkat Bantu Sistem Harian (Daily Essential Utilities)**  
> *Arsip/Kompresi · Kalkulator Saintifik · Media Player 4K · Jam & Timer · Snipping Tool · Task Manager*

---

## 1. Pendahuluan

Salah satu pengalaman pengguna paling mengecewakan saat baru menginstal sebuah sistem operasi adalah ketika hal-hal mendasar tidak bisa dilakukan secara langsung:
- Tidak bisa mengekstrak file `.rar` atau membuat file `.zip`.
- Tidak bisa memutar video `.mp4`/`.mkv` karena ketiadaan codec.
- Tidak ada kalkulator, jam alarm, atau pemotong tangkapan layar (*screenshot*).

**WidyaOS** membundel **8 Utilitas Bawaan Esensial** yang dirancang ringan, berakselerasi hardware, bebas iklan, dan langsung siap digunakan sejak detik pertama instalasi.

---

## 2. Katalog 8 Utilitas Esensial WidyaOS

```
┌────────────────────────────────────────────────────────────────────────┐
│                   SUITE UTILITAS BAWAAN WIDYAOS                        │
├────────────────────┬────────────────────┬──────────────────────────────┤
│ 1. WidyaArchive    │ 2. WidyaCalc Pro   │ 3. WidyaMedia Player         │
│ (Kompresi Zip/Rar) │ (Kalkulator 4-in-1)│ (Audio & Video 4K 120 FPS)   │
├────────────────────┼────────────────────┼──────────────────────────────┤
│ 4. WidyaClock      │ 5. WidyaCapture    │ 6. WidyaNotepad              │
│ (Jam Dunia & Timer)│ (Screenshot & Rec) │ (Editor Teks Cepat < 5ms)    │
├────────────────────┼────────────────────┼──────────────────────────────┤
│ 7. WidyaTaskMgr    │ 8. WidyaScan & OCR │                              │
│ (Monitor Performa) │ (Pemindai Dokumen) │                              │
└────────────────────┴────────────────────┴──────────────────────────────┘
```

---

### 1. WidyaArchive (Pengarsip & Kompresi Berkas)
*Pengganti WinRAR, 7-Zip, dan Keka.*

- **Format yang Didukung:**
  - **Ekstraksi (Buka & Unpack):** `.zip`, `.rar` (semua versi RAR 4 & 5), `.7z`, `.tar`, `.tar.gz`, `.tar.zst` (Zstandard), `.tar.xz`, `.iso`, `.cab`.
  - **Kompresi (Buat Arsip Baru):** `.zip` (kompatibel universal), `.7z` (rasio kompresi tinggi), `.tar.zst` (kecepatan kilat performa industri).
- **Fitur Unggulan:**
  - **Enkripsi Kriptografi Kuat:** Mengunci arsip dengan kata sandi menggunakan enkripsi **AES-256-GCM** atau kunci pasca-kuantum.
  - **Arsip Multi-Bagian (*Split Volume*):** Membagi arsip menjadi potongan 100MB, 500MB, atau 1GB untuk memudahkan pengiriman.
  - **Integrasi Menu Klik Kanan:** Klik kanan berkas di **WidyaExplorer** langsung memunculkan opsi: *"Ekstrak Di Sini"*, *"Ekstrak ke Folder..."*, atau *"Kompres ke .ZIP"*.

---

### 2. WidyaCalc Pro (Kalkulator Saintifik & Programmer)
*Pengganti Windows Calculator dan macOS Calculator.*

- **4 Mode Komputasi:**
  1. **Mode Standar:** Operasi matematika dasar harian, tombol persen, dan riwayat pita hitungan (*calculation tape history*).
  2. **Mode Saintifik:** Fungsi trigonometri ($\sin, \cos, \tan$), logaritma ($\ln, \log_{10}$), eksponensial, faktorial, dan konstanta ilmiah ($\pi, e$).
  3. **Mode Programmer:** Konversi langsung 4 basis bilangan (**HEX**, **DEC**, **OCT**, **BIN**) secara simultan, operasi bitwise (AND, OR, XOR, NOT, Bit-shift).
  4. **Konverter Satuan & Valuta:** Konversi instan panjang (m, cm, inci), berat (kg, pound), suhu (°C, °F, K), volume, serta kurs mata uang (IDR, USD, EUR, SGD).

---

### 3. WidyaMedia Player (Pemutar Musik & Video Universal)
*Pengganti Windows Media Player, VLC, dan QuickTime.*

- **Pemutar Video (4K/8K 60–120 FPS):**
  - Format: `.mp4`, `.mkv`, `.webm`, `.avi`, `.mov`, `.flv`.
  - **Akselerasi Perangkat Keras GPU Penuh:** Memanfaatkan hardware decoding kartu grafis (Intel QuickSync, AMD VCN, NVIDIA NVDEC) sehingga CPU tetap dingin dan baterai laptop hemat saat menonton video 4K.
  - **Dukungan Subtitle Cerdas:** Otomatis mendeteksi subtitle eksternal (`.srt`, `.vtt`, `.ass`) dengan pengaturan ukuran, font, dan penyesuaian sinkronisasi audio.
- **Pemutar Musik & Audio:**
  - Format: `.mp3`, `.flac` (Lossless Audio), `.aac`, `.ogg`, `.m4a`, `.wav`.
  - **Equalizer Grafis 10-Band:** Pilihan preset (Bass Boost, Vokal, Akustik, Flat, Pop).
  - **Mode Mini-Player Mengambang:** Jendela kecil ringkas di pojok layar atau widget kontrol musik di bilah status / taskbar tray.

---

### 4. WidyaClock (Jam Dunia, Alarm, Stopwatch & Timer)
*Pengganti Alarms & Clock.*

- **Jam Dunia (World Clock):** Menampilkan perbandingan zona waktu nasional (WIB, WITA, WIT) serta kota dunia (Tokyo, London, New York) dalam peta visual siang/malam.
- **Alarm Cerdas:** Alarm berulang harian/hari kerja dengan nada dering gamelan Nusantara lembut atau melodi kustom.
- **Stopwatch Presisi Milidetik:** Dilengkapi fitur pencatat putaran waktu (*Lap Time*).
- **Timer Fokus Pomodoro:** Mode pengatur waktu 25 menit kerja fokus + 5 menit istirahat untuk meningkatkan produktivitas belajar dan bekerja.

---

### 5. WidyaCapture (Tangkapan & Perekam Layar Bawaan)
*Pengganti Snipping Tool Windows dan Screenshot macOS.*

- **Pintasan Cepat:** Tekan `PrtScn` (Windows persona) atau `Cmd + Shift + 4` (macOS persona) atau `Super + Shift + S`.
- **3 Mode Tangkapan:**
  1. *Seleksi Persegi Panjang (Rectangular Snap)*
  2. *Jendela Aplikasi Tunggal (Window Snap dengan bayangan drop shadow bersih)*
  3. *Seluruh Layar (Fullscreen)*
- **Alat Anotasi Cepat:**
  - Tambah panah penunjuk, kotak penegas, stabilo warna, teks penjelasan.
  - **Kuas Sensor Informasi Rahasia (Pixelate / Redact Blur):** Sensor NIK, nomor kartu kredit, atau kata sandi dalam 1 usapan mouse sebelum dibagikan.
- **Perekam Layar Ringkas (*Screen Recorder*):**
  - Merekam layar bersama suara mikrofon dan audio sistem ke format berkas MP4/WebM ringan tanpa watermark.

---

### 6. WidyaNotepad (Penyunting Teks Kilat)
*Pengganti Notepad Windows dan TextEdit Mac.*

- **Kecepatan Cold-Start < 5 Milidetik:** Terbuka instan tanpa jeda kursor.
- **Mampu Membuka Berkas Berukuran Gigabyte:** Menggunakan alokasi memori bertahap (*memory-mapped I/O*), mampu membuka berkas log atau file teks 2 GB tanpa membuat komputer hang.
- **Deteksi Encoding Otomatis:** UTF-8, UTF-16, ASCII, ISO-8859-1.
- **Penyorotan Sintaks Sederhana (*Syntax Highlighting*):** Pewarnaan kode otomatis saat membuka file `.json`, `.md`, `.wya`, `.rs`, `.py`, `.sql`, `.xml`.

---

### 7. WidyaTaskMgr (Pengelola Tugas & Monitor Kinerja)
*Pengganti Task Manager Windows dan Activity Monitor Mac.*

- **Visualisasi Kinerja Real-Time:** Grafik pemakaian CPU (tampilan per-core), RAM, I/O Harddisk/SSD, GPU, dan Jaringan Wi-Fi/LAN.
- **Daftar Proses & Aplikasi:** Menampilkan penggunaan memori dan prosesor per-aplikasi.
- **Hentikan Proses Aman (*Force Kill Safe*):** Menghentikan aplikasi yang tidak merespons secara bersih tanpa merusak data sistem operasi.
- **Status Keamanan PQC:** Menampilkan indikator apakah suatu proses berjalan di dalam sandbox anti-ransomware atau memiliki tanda tangan digital yang sah.

---

### 8. WidyaScan & OCR (Pusat Pemindai & Pembaca Teks Cerdas)
*Pengganti Windows Fax and Scan.*

- **Dukungan Mesin Pemindai (Scanner):** Kompatibel dengan 99% scanner USB dan scanner jaringan (eSCL, WSD, SANE).
- **OCR Teks Otomatis (Optical Character Recognition):**
  - Kertas hasil scan surat/kontrak otomatis dikenali teksnya (Bahasa Indonesia & Inggris).
  - Hasil scan dapat langsung disimpan sebagai **Searchable PDF** atau diekspor langsung ke dokumen **WidyaWriter** untuk diedit ulang tanpa perlu mengetik ulang secara manual!
