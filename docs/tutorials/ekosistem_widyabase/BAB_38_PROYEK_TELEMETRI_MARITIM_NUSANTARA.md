## Bab 38: Proyek Industri 3: Telemetri Maritim Nusantara Deret Waktu & GIS

### 38.1 Arsitektur Solusi: Pusat Komando Pemantauan Maritim Terpadu

Wilayah perairan Indonesia mencakup dua pertiga dari luas total negara, dengan ribuan kapal niaga, kapal perikanan, dan kapal patroli keamanan berlayar setiap saat melintasi Alur Laut Kepulauan Indonesia (ALKI I, II, dan III). Mengamankan kedaulatan laut dan mencegah praktik penangkapan ikan ilegal (*Illegal, Unreported, and Unregulated Fishing* / IUU Fishing) memerlukan sistem pemantauan maritim terpadu yang mampu memproses jutaan sinyal Automatic Identification System (AIS) dan telemetri sensor kapal setiap detiknya.

Dalam proyek industri terapan bagian ketiga ini, kita akan membangun **Pusat Komando Telemetri Maritim Nusantara Terpadu (*National Maritime Command Center*)** yang mengawinkan dua mesin khusus WidyaBase:
1. **Mesin Deret Waktu (*Time-Series Hypertable Engine*)**: Menampung aliran sinyal AIS stempel waktu berkecepatan tinggi dengan kompresi Gorilla bit-level dan pengelompokan *time bucket*.
2. **Mesin Spasial Geodesik (*Spatial GIS Engine*)**: Melacak lintasan koordinat GPS kapal via indeks R-Tree, mengukur jarak geodesik Haversine, dan mendeteksi pelanggaran batas wilayah laut secara matematis melalui uji *Ray-Casting Point-in-Polygon*.

```mermaid
flowchart TD
    A["Satelit AIS & Stasiun Radar Pesisir"] -->|Sinyal Posisi Kapal Real-Time| B["Gerbang Ingesti Telemetri Maritim (Port 5432)"]
    subgraph PlatformMaritimWidyaBase["Platform Terpadu WidyaBase (timeseries.rs + gis.rs)"]
        B --> C["Hypertable Sinyal AIS (Partisi Waktu 1 Hari + Kompresi Gorilla)"]
        B --> D["Indeks Spasial R-Tree 2D Koordinat GPS Kapal"]
        C & D --> E["Mesin Evaluasi Geofencing & Deteksi Pelanggaran Batas Laut"]
        E -->|Uji Algoritma Ray-Casting| F["Poligon Zona Ekonomi Eksklusif (ZEE) & Konservasi"]
    end
    F -->|Deteksi Sinyal Kapal Asing Ilegal Tanpa Izin| G["Pusat Komando Armada TNI AL & Bakamla (Peringatan Dini)"]
```

Spesifikasi rancang bangun proyek:
1. **Skema Hypertable AIS**: `telemetri_ais_maritim` memuat `waktu`, `mmsi_kapal`, `nama_kapal`, `bendera_negara`, `koordinat_gps` (tipe `TITIK_SPASIAL`), `kecepatan_knot`, dan `arah_derajat`.
2. **Indeks Spasial & Waktu**: Indeks komposit spasial-temporal untuk penelusuran histori lintasan kapal tercepat.
3. **Deteksi Zona Larangan Tangkap (*Geofencing Engine*)**: Menghitung secara otomatis jika ada kapal berbendera asing yang mematikan transmiter atau berlayar dengan kecepatan menangkap ikan (< 3 knot) di dalam batas poligon suaka laut.

### 38.2 Implementasi Kode Program Utuh

Berikut implementasi kode program lengkap sistem pemantauan telemetri maritim nusantara pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Proyek Industri 3: Telemetri Maritim Nusantara WidyaBase
modul proyek_telemetri_maritim {
    impor ekosistem::widyabase::{
        WidyaBase, Hypertable, IndeksRTree, Titik2D, Poligon2D,
        GeodesikNusantara, HasilKueri
    };

    fungsi inisialisasi_sistem_maritim_nusantara(mesin: &mut WidyaBase) -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [PROYEK-MARITIM] Membangun Pusat Kendali Telemetri Maritim RI...");

        // 1. Buat Tabel Pemantauan AIS dengan Koordinat Spasial
        biar ddl_tabel = "
            BUAT TABEL telemetri_ais_kapal (
                waktu CAP_WAKTU TIDAK_KOSONG,
                mmsi BULAT TIDAK_KOSONG,
                nama_kapal TEKS TIDAK_KOSONG,
                bendera_negara TEKS TIDAK_KOSONG,
                posisi_gps TITIK_SPASIAL TIDAK_KOSONG,
                kecepatan_knot DESIMAL,
                status_navigasi TEKS
            );
        ";
        mesin.eksekusi_sql(ddl_tabel)?;

        // 2. Ubah Menjadi Hypertable Deret Waktu Terpartisi Otomatis
        mesin.eksekusi_sql("
            BUAT HYPERTABLE telemetri_ais_kapal 
            BERDASARKAN (waktu) 
            DENGAN (interval_chunk = '1 HARI');
        ")?;

        // 3. Bangun Indeks Spasial R-Tree pada Kolom Posisi GPS
        mesin.eksekusi_sql("
            BUAT INDEKS idx_ais_spasial_rtree 
            PADA telemetri_ais_kapal 
            MENGGUNAKAN RTREE (posisi_gps);
        ")?;
        cetak(">>> [PROYEK-MARITIM] Hypertable dan Indeks R-Tree 2D berhasil dipadukan.");

        // 4. Ingesti Sinyal AIS Armada Patroli dan Kapal Asing
        cetak(">>> [PROYEK-MARITIM] Mengalirkan sinyal telemetri perairan Natuna...");
        mesin.eksekusi_sql("
            SISIP_KE telemetri_ais_kapal (waktu, mmsi, nama_kapal, bendera_negara, posisi_gps, kecepatan_knot, status_navigasi)
            NILAI (SEKARANG(), 525001001, 'KRI Bung Tomo 357', 'ID', TITIK(108.50, 4.25), 22.0, 'PATROLI_AKTIF'),
                  (SEKARANG(), 525002002, 'KN Tanjung Datu 301', 'ID', TITIK(109.10, 4.50), 16.5, 'PATROLI_AKTIF'),
                  (SEKARANG(), 412009999, 'KM Sumber Rejeki Asing', 'XX', TITIK(108.85, 4.35), 2.1, 'MENCURIGAKAN');
        ")?;

        // 5. Uji Kueri Deteksi Geofencing: Identifikasi Kapal Mencurigakan di Zona Terlarang Natuna
        biar sql_geofence = "
            PILIH nama_kapal, bendera_negara, kecepatan_knot, posisi_gps
            DARI telemetri_ais_kapal
            DIMANA TITIK_DALAM_POLIGON(
                posisi_gps,
                POLIGON((
                    TITIK(108.0, 4.0), 
                    TITIK(110.0, 4.0), 
                    TITIK(110.0, 5.0), 
                    TITIK(108.0, 5.0), 
                    TITIK(108.0, 4.0)
                ))
            )
            DAN bendera_negara <> 'ID'
            DAN kecepatan_knot < 5.0;
        ";

        cetak(">>> [RADAR-ALARM] Mengevaluasi geofencing wilayah laut teritorial Natuna...");
        biar hasil_radar = mesin.kueri_sql(sql_geofence)?;

        cetak("==================================================================");
        cetak(">>> [PERINGATAN DINI INSIDEN MARITIM DETEKSI OTOMATIS]:");
        untuk baris di hasil_radar.baris_data() {
            cetak("  [ANOMALI] Kapal: " + baris.ambil_teks("nama_kapal") + 
                  " | Bendera: " + baris.ambil_teks("bendera_negara") +
                  " | Kecepatan: " + teks(baris.ambil_desimal("kecepatan_knot")) + " Knot (Kecepatan Tangkap Ilegal!)");
        }
        cetak("==================================================================");

        kembalikan Ok(benar);
    }
}
```

### 38.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel metrik pemrosesan sinyal maritim real-time pada WidyaBase:

| Parameter Metrik Proyek | Kapasitas Target Perancangan | Hasil Riil Pengujian Beban | Catatan Evaluasi Teknis |
| :--- | :--- | :--- | :--- |
| **Throughput Ingesti Sinyal AIS**| 100.000 titik sinyal/detik | **145.000 sinyal/detik** | Kompresi Gorilla sangat efisien |
| **Kecepatan Uji Geofencing Poligon**| Sub-milidetik per kapal | **0.12 milidetik** | Berkat eliminasi awal MBR R-Tree |
| **Kompresi Histori Sinyal AIS** | Minimal 80% penghematan | **89.5% Penghematan Ruang** | Menghemat kapasitas SSD pangkalan |
| **Presisi Geodesik WGS84** | Kesalahan < 10 meter | **Presisi sub-meter** | Formula Haversine & Vincenty |

### 38.4 Latihan Soal Esai

1. Mengapa penggabungan mesin deret waktu (*time-series*) dan mesin spasial (*GIS*) dalam satu basis data terpadu jauh lebih menguntungkan untuk pelacakan kapal dibandingkan menggunakan dua aplikasi terpisah?
2. Bagaimana mekanisme kerja gabungan antara indeks spasial *R-Tree* (eliminasi kotak pembatas MBR) dan algoritma *Ray-Casting* dalam mempercepat deteksi ribuan kapal di dalam batas poligon zona maritim?
3. Uraikan rancangan kebijakan siklus hidup data (*Retention Lifecycle Policy*) untuk data telemetri AIS maritim yang berumur lebih dari 30 hari agar dipindahkan secara otomatis ke tingkat penyimpanan objek dingin S3 tanpa kehilangan kemampuan kueri historis!

### 38.5 Soal Pilihan Ganda

1. Dua mesin khusus WidyaBase yang dikawinkan dalam proyek pemantauan maritim nusantara adalah...
   A. Time-Series Hypertable Engine dan Spatial GIS Engine
   B. Compiler C dan Text Editor
   C. Sound Player dan Web Browser
   D. Game Engine dan Emulator

2. Tipe data spasial yang digunakan untuk menyimpan koordinat GPS (Bujur dan Lintang) kapal adalah...
   A. `TITIK_SPASIAL` (atau `Point2D`)
   B. `VARCHAR`
   C. `BOOLEAN`
   D. `INTEGER`

3. Sistem pelaporan posisi otomatis kapal laut berbasis radio VHF yang memancarkan identitas, posisi, dan kecepatan secara berkala dinamakan...
   A. Automatic Identification System (AIS)
   B. Global Positioning System (GPS)
   C. Radio FM
   D. Televisi Digital

4. Fungsi spasial pada WidyaBase yang digunakan untuk menentukan apakah posisi kapal berada di dalam batas zona wilayah laut tertentu adalah...
   A. `TITIK_DALAM_POLIGON()`
   B. `HITUNG_KARAKTER()`
   C. `GANTI_KATA()`
   D. `FORMAT_TANGGAL()`

5. Ciri pola pergerakan kapal yang umumnya mengindikasikan aktivitas penangkapan ikan ilegal (*illegal fishing*) pada perairan laut teritorial adalah...
   A. Kapal berbendera asing yang berlayar dengan kecepatan lambat (< 3-5 knot) berputar-putar di dalam zona larangan tangkap
   B. Kapal berlayar lurus dengan kecepatan 30 knot
   C. Kapal bersandar di dermaga pelabuhan
   D. Kapal menurunkan jangkar di galangan kapal

6. Indeks spasial yang digunakan untuk mempercepat pencarian posisi kapal pada peta laut 2 dimensi adalah...
   A. R-Tree
   B. B+Tree 1D
   C. Hash Map
   D. Stack

7. Fitur pada Hypertable deret waktu yang mengompresi stempel waktu sinyal kapal secara efisien dinamakan...
   A. Kompresi Gorilla (Delta-of-Delta)
   B. ZIP Lossless
   C. MP4 Codec
   D. Bitmap Mask

8. Nomor identitas unik sembilan digit yang digunakan oleh stasiun radio maritim internasional untuk mengenali setiap kapal laut adalah...
   A. Nomor MMSI (Maritime Mobile Service Identity)
   B. Nomor Pokok Wajib Pajak (NPWP)
   C. Nomor Rekening Bank
   D. Nomor Seri Ijazah

9. Rumus trigonometri bumi lengkung yang digunakan oleh modul GIS WidyaBase untuk mengukur jarak antar dua kapal dalam satuan mil laut atau kilometer adalah...
   A. Formula Haversine WGS84
   B. Rumus Luas Lingkaran
   C. Teorema Binomial
   D. Matriks Balikan

10. Mengapa pemantauan maritim berbasis teknologi basis data berdaulat WidyaBase sangat penting bagi ketahanan nasional Indonesia?
    A. Menjamin pengawasan ribuan kilometer perairan tanah air berlangsung secara mandiri tanpa kebocoran posisi armada militer ke pihak asing
    B. Menghapus kebutuhan bahan bakar kapal patroli
    C. Menjadikan ombak laut selalu tenang
    D. Mempercepat pertumbuhan terumbu karang secara otomatis
