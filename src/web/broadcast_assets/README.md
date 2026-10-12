# 🎥 Widya Sovereign Broadcast Studio MCR (Pencak Silat Multi-Camera)

Aplikasi web interaktif **Master Control Room (MCR)** resmi ke-8 dalam ekosistem kedaulatan digital Widya-Lang. Dirancang khusus untuk memproduksi siaran langsung pertandingan olahraga dan kejuaraan pencak silat multi-gelanggang setara standar penyiaran Olimpiade dan Tier-1 internasional (OBS Studio, vMix, EVS LSM-VIA).

---

## 🌟 Fitur Utama

1. **Master Control Room (MCR) Switcher 60 FPS:**
   - Dual Master Monitor: **PREVIEW** (Standby Hijau) & **PROGRAM** (Live Merah).
   - Transisi instan **CUT**, **AUTO CROSS-FADE** (300ms), **WIPE**, dan slider **T-Bar** interaktif.
   - Multiview Matrix 6 Kamera HP (Gelanggang A, B, C) + 1 SkyCam / WireCam 3D Orbit.
   - Sinkronisasi timecode **SMPTE ST 2059-2** (PTP Locked) & meter kenyaringan audio **EBU R128** (-23 LUFS).

2. **Konsol Video Assistant Referee (VAR) PERSILAT:**
   - Pemicu Kartu Tantangan sudut merah atau biru.
   - Scrubbing frame lambat (-30 s/d +30 frames @ 0.25x Ultra Slow-Motion).
   - Pengambilan putusan resmi dewan hakim berstempel digital (`+3 Poin Bantingan Sah`, `0 Poin Batal`).

3. **Papan Skor Digital Resmi:**
   - Kontrol jam tanding babak (120 detik countdown).
   - Tombol penambahan poin resmi PERSILAT (+1 Pukulan, +2 Tendangan, +3 Bantingan).
   - HUD on-air overlay dinamis yang terpasang di atas monitor program siaran.

4. **Kecerdasan Buatan & Biomekanika:**
   - MediaPipe 33-Keypoints Skeleton Overlay untuk anatomi sendi pesilat.
   - Sensor kecepatan serangan (Strike Velocity 64.8 km/jam, Benturan 18.5 G, Reaksi 140 ms).
   - Hawk-Eye Virtual Arena Boundary Sensor.

5. **Telemetri BCI Otak Atlet (EEG):**
   - Pengawasan status mental real-time (Gelombang Alpha/Beta/Theta).
   - Deteksi otomatis *Flow State* (Tenang & Fokus) vs *Panic Under Pressure* (Stres Bertanding).

6. **Audio Dialek Nusantara & Saweria On-Chain:**
   - Sintesis komentator AI dalam dialek daerah (Jawa Krama, Sunda, Minang, Nasional).
   - Widget donasi Saweria On-Chain dengan banner animasi on-air dan validasi hash transaksi berdaulat.

7. **4-Modem Bonded Cellular Failover:**
   - Agregator 4-modem (Telkomsel 5G, Indosat 5G, XL LTE, Starlink LEO).
   - Uji simulasi pemutusan jalur modem dengan failover instan 0 drop frame.

8. **Klien Smartphone Juru Kamera (`kamera_hp.html`):**
   - Halaman khusus kamera HP juru kamera gelanggang.
   - Border Tally dinamis raksasa: Merah berkedip (*Live On Air*), Hijau (*Standby Preview*), Abu-abu (*Inactive*).
   - Pasangkan HP ke MCR hanya dengan memindai kode QR dari layar MCR!

---

## 🚀 Cara Menjalankan

Buka peramban dan akses:
- **MCR Switcher Utama**: [`http://localhost:8080/08_widya_broadcasting_studio/index.html`](file:///home/devops/widya-lang/aplikasi/08_widya_broadcasting_studio/index.html)
- **Kamera HP Juru Kamera**: [`http://localhost:8080/08_widya_broadcasting_studio/kamera_hp.html`](file:///home/devops/widya-lang/aplikasi/08_widya_broadcasting_studio/kamera_hp.html)
- **Terminal Khusus Dewan Hakim VAR**: [`http://localhost:8080/08_widya_broadcasting_studio/var_dewan_hakim.html`](file:///home/devops/widya-lang/aplikasi/08_widya_broadcasting_studio/var_dewan_hakim.html)
