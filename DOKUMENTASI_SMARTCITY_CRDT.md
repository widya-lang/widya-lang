# 🏙️ Dokumentasi Widya SmartCity & Real-Time CRDT Collaboration Engine

Selamat datang di dokumentasi resmi **Widya SmartCity** dan **CRDT Real-Time Collaboration Engine** (v2.2.0). Modul ini menyediakan solusi komprehensif untuk intelijen perkotaan, pengolahan geospasial, pemantauan kualitas udara, rekayasa lalu lintas adaptif, analisis mikrogrid energi terbarukan, serta kolaborasi dokumen multi-pengguna tanpa konflik (*Conflict-Free Replicated Data Types*).

---

## 📑 Daftar Isi
1. [Arsitektur & Komponen Utama](#arsitektur--komponen-utama)
2. [1. Sistem Pemantauan Kualitas Udara (ISPU Permen LHK)](#1-sistem-pemantauan-kualitas-udara-ispu-permen-lhk)
3. [2. Sistem Informasi Geografis & Geospasial (GIS)](#2-sistem-informasi-geografis--geospasial-gis)
4. [3. Rekayasa Lalu Lintas & Sinyal Adaptif (LoS & V/C Ratio)](#3-rekayasa-lalu-lintas--sinyal-adaptif-los--vc-ratio)
5. [4. Analisis Mikrogrid Energi Baru Terbarukan (EBT)](#4-analisis-mikrogrid-energi-baru-terbarukan-ebt)
6. [5. Mesin Kolaborasi Real-Time Bebas Konflik (CRDT)](#5-mesin-kolaborasi-real-time-bebas-konflik-crdt)
7. [Referensi Fungsi Bawaan (Built-in Functions)](#referensi-fungsi-bawaan-built-in-functions)
8. [Perintah Antarmuka Baris Perintah (CLI)](#perintah-antarmuka-baris-perintah-cli)

---

## 🏗️ Arsitektur & Komponen Utama

Widya-Lang dirancang dengan pendekatan modular native tanpa ketergantungan runtime eksternal yang berat:
- **`src/smartcity/mod.rs`**: Core engine pemrosesan ISPU, rumus jarak geodesik Haversine, algoritma Ray-Casting untuk batas wilayah geofencing, optimasi Level of Service (LoS), dan kalkulasi emisi EBT.
- **`src/office/crdt.rs`**: Core engine CRDT menggunakan *Fractional Indexing* dan *Last-Write-Wins (LWW)* untuk sinkronisasi dokumen teks real-time secara komutatif, asosiatif, dan idempoten (*Strong Eventual Consistency*).

---

## 1. Sistem Pemantauan Kualitas Udara (ISPU Permen LHK)

Perhitungan Indeks Standar Pencemar Udara (ISPU) mengacu pada **Peraturan Menteri Lingkungan Hidup dan Kehutanan No. P.14/MENLHK/SETJEN/KUM.1/7/2020**:

$$I = \frac{I_a - I_b}{X_a - X_b} (X_x - X_b) + I_b$$

### Kategori Indeks ISPU:
| Rentang Indeks | Kategori | Warna Indikator | Rekomendasi Kesehatan |
|---|---|---|---|
| **0 – 50** | **Baik** | 🟢 Hijau | Sangat aman untuk semua aktivitas outdoor. |
| **51 – 100** | **Sedang** | 🔵 Biru | Aman, kelompok rentan batasi aktivitas fisik ekstrem. |
| **101 – 200** | **Tidak Sehat** | 🟡 Kuning | Kelompok sensitif wajib mengenakan masker N95/medis. |
| **201 – 300** | **Sangat Tidak Sehat** | 🔴 Merah | Berbahaya. Hindari aktivitas di luar ruangan. |
| **301+** | **Berbahaya** | ⚫ Hitam | Status Darurat Polusi Udara. Wajib di dalam ruangan. |

```widya
biar ispu = smartcity_hitung_ispu_pm25(42.8)
cetak("Indeks ISPU: " + teks(ispu["nilai_ispu"]))
cetak("Kategori   : " + teks(ispu["kategori"]))
cetak("Saran      : " + ispu["saran_kesehatan"])
```

---

## 2. Sistem Informasi Geografis & Geospasial (GIS)

### 2.1 Jarak Geodesik Lingkaran Besar (Haversine Formula)
Menghitung jarak akurat antara dua titik koordinat GPS di permukaan bumi (satuan kilometer):

```widya
// Balai Kota (-6.2088, 106.8456) ke RSUD (-6.2297, 106.8184)
biar jarak = smartcity_jarak_haversine(-6.2088, 106.8456, -6.2297, 106.8184)
cetak("Jarak Fasilitas: " + teks(jarak) + " km")
```

### 2.2 Geofencing & Penentuan Batas Wilayah Desa (Ray-Casting Algorithm)
Menentukan apakah titik koordinat berada di dalam poligon batas administrasi:

```widya
biar batas_desa = [
    [-6.2000, 106.8000],
    [-6.2000, 106.8500],
    [-6.2500, 106.8500],
    [-6.2500, 106.8000]
]

biar di_dalam = smartcity_titik_dalam_poligon(-6.2200, 106.8200, batas_desa)
cetak("Status Wilayah: " + teks(di_dalam)) // true
```

---

## 3. Rekayasa Lalu Lintas & Sinyal Adaptif (LoS & V/C Ratio)

Menghitung rasio Volume terhadap Kapasitas (*V/C Ratio*) dan menentukan *Level of Service* (LoS A-F) serta durasi lampu hijau adaptif secara otomatis:

```widya
biar lalin = smartcity_optimasi_lalu_lintas("Jl. Gatot Subroto", 1450.0, 1600.0, 120)
cetak("V/C Ratio         : " + teks(lalin["vc_ratio"]))
cetak("Level of Service  : " + teks(lalin["level_of_service"]))
cetak("Durasi Lampu Hijau: " + teks(lalin["durasi_lampu_hijau_detik"]) + " detik")
```

---

## 4. Analisis Mikrogrid Energi Baru Terbarukan (EBT)

Mengagregasi pasokan daya bauran EBT (PLTS Surya, PLTA Hidro, PLTP Panas Bumi) dibandingkan pembangkit fosil serta menghitung reduksi emisi karbon $CO_2$:

```widya
// Parameter: PLTS (MW), PLTA (MW), PLTP (MW), Fosil (MW)
biar energi = smartcity_analisis_bauran_energi(45.0, 60.0, 35.0, 20.0)
cetak("Total Daya Pembangkit : " + teks(energi["total_daya_mw"]) + " MW")
cetak("Bauran EBT Bersih     : " + teks(energi["persentase_ebt"]) + "%")
cetak("Reduksi Emisi Karbon  : " + teks(energi["reduksi_emisi_ton_co2_harian"]) + " ton CO2/hari")
```

---

## 5. Mesin Kolaborasi Real-Time Bebas Konflik (CRDT)

Modul **CRDT WidyaDocs** memungkinkan banyak pengguna menyunting dokumen teks yang sama secara bersamaan melalui jaringan tanpa risiko data corrupt atau *overwrite conflict*:

```widya
// Inisialisasi dokumen klien
biar doc = crdt_teks_buat("klien_a")

// Sisip teks pada indeks
doc = crdt_teks_sisip(doc, 0, "Rancangan Dokumen Kebijakan Digital")

// Hapus karakter
doc = crdt_teks_hapus(doc, 0, 10)

// Baca teks terkonsolidasi
cetak("Teks Dokumen: " + crdt_teks_baca(doc))

// Buat Sesi Web Studio Kolaboratif
biar sesi = crdt_sesi_buat("ruang-101", "Dokumen_Strategis.docx", "admin")
biar html = crdt_sesi_render_html(sesi)
```

---

## 🛠️ Perintah CLI (Antarmuka Baris Perintah)

```bash
# Cek status modul Smart City
widya smartcity status

# Hitung ISPU berdasarkan konsentrasi PM2.5
widya smartcity ispu --pm25 55.4

# Hitung jarak geodesik Haversine
widya smartcity jarak --lat1 -6.2088 --lon1 106.8456 --lat2 -6.9175 --lon2 107.6191

# Optimasi sinyal lampu lalu lintas
widya smartcity lalin --jalan "Jl. Sudirman" --volume 1200 --kapasitas 1500 --siklus 120

# Analisis mikrogrid energi EBT
widya smartcity energi --plts 20 --plta 30 --pltp 50 --fosil 0

# Buat ruang kerja kolaborasi real-time CRDT
widya workspace collab --room "ruang-sidang" --doc "Kebijakan2045.docx" --output collab.html
```
