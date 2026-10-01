# PRODUCT REQUIREMENT DOCUMENT (PRD)
## PROYEK: SMARTCITY ISPU TELEMETRI & SPATIAL GIS
**Disusun Oleh:** `agen_analis` (SOP-02) & `agen_arsitek` (SOP-03)  
**Standar Regulasi:** Permen LHK No. P.14/2020 tentang Indeks Standar Pencemar Udara (ISPU)  

---

### 1. Problem Statement & Target Solusi
Dinas Lingkungan Hidup (DLH) dan Dinas Perhubungan di berbagai kabupaten/kota membutuhkan platform pemantauan kualitas udara dan manajemen lalu lintas cerdas yang terintegrasi secara mandiri tanpa tergantung server cloud asing.

**Tujuan**: Mengembangkan platform telemetri IoT multi-sensor dengan visualisasi GIS peta geospasial interaktif, kalkulator baku mutu resmi ISPU (PM2.5, PM10, SO2, CO, O3), optimasi durasi lampu lalu lintas adaptif (ATCS) rasio V/C, dan geofencing armada truk sampah/patroli.

---

### 2. Fitur Utama
1. **Peta Geospasial Multi-Layer SVG/GIS**: Menampilkan titik stasiun sensor lingkungan, status lampu lalu lintas, dan pergerakan armada secara real-time.
2. **Kalkulator Interpolasi ISPU Permen LHK**: Konversi otomatis konsentrasi mikrogram/m³ menjadi kategori (Baik, Sedang, Tidak Sehat, Sangat Tidak Sehat, Berbahaya).
3. **Adaptive Traffic Control System (ATCS)**: Perhitungan optimasi fase hijau berbasis formula Webster rasio Volume/Kapasitas (V/C).
4. **Geofencing Haversine**: Deteksi radius masuk/keluar armada truk ke zona kelurahan/desa inti.
