## Bab 27: Tingkat Penyimpanan Objek Dingin S3 (Cold Storage Tiering)

### 27.1 Hierarki Penyimpanan Biaya Efisien: Panas, Hangat, Dingin

Dalam pengelolaan data berskala petabyte—seperti riwayat rekaman CCTV jalan raya nasional, arsip transaksi perbankan 10 tahun terakhir untuk kepatuhan regulator OJK/BI, atau log audit sistem kependudukan—menyimpan seluruh data di media SSD NVMe berkecepatan tinggi merupakan pemborosan anggaran infrastruktur yang sangat masif. Data historis yang berusia lebih dari 90 hari jarang sekali diakses (*cold data*), namun wajib disimpan secara legal dan dapat di-kueri sewaktu-waktu.

WidyaBase menyediakan subsistem **Tingkat Penyimpanan Bertingkat (*Automated Storage Tiering Engine*)** yang menghubungkan kluster basis data dengan repositori penyimpanan objek kompatibel S3 (*Simple Storage Service*):
- **Tingkat Panas (*Hot Tier*)**: Media SSD NVMe lokal berlatensi sub-milidetik untuk transaksi aktif OLTP hari ini, buffer pool RAM, dan WAL.
- **Tingkat Hangat (*Warm Tier*)**: Media SSD SATA atau harddisk terpasang jaringan (NAS/SAN) untuk data berusia 7 hingga 30 hari dalam format kolom terkompresi.
- **Tingkat Dingin Objek (*Cold S3 Tier*)**: Kluster penyimpanan objek awan (seperti MinIO on-premise, AWS S3, atau Ceph Object Store) dengan biaya penyimpanan hingga 80–90% lebih murah.

```mermaid
flowchart TD
    A["Aliran Transaksi Baru (Hari Ini)"] --> B["Hot Tier: SSD NVMe Lokal (Slotted-Page & WAL)"]
    B -->|Kebijakan Siklus Hidup: Usia > 7 Hari| C["Warm Tier: SSD SATA / Format Kolom Lokal"]
    C -->|Kebijakan Siklus Hidup: Usia > 30 Hari| D["Mesin Tiering WidyaBase (S3 Cloud Connector)"]
    subgraph ColdObjectStorage["Cold Tier: S3 Object Storage (MinIO / Ceph / Cloud)"]
        D --> E["Berkas Parquet Terkompresi ZSTD / Parquet Chunks"]
        E --> F["Metadata Index Cache Lokal (Mempertahankan Skema Kueri)"]
    end
    Q["Kueri Transparan: PILIH * DARI arsip_audit WHERE thn = 2024;"] --> F
    F -->|Hanya Mengunduh Rentang Byte yang Diminta (HTTP Range Request)| E
```

Prinsip kerja penyimpanan dingin transparan WidyaBase:
1. **Kueri Transparan Tanpa Restorasi Manual (*Transparent In-Place Querying*)**: Analis data dapat menjalankan kueri SQL biasa terhadap tabel arsip tanpa perlu meminta tim IT melakukan *restore backup* secara manual.
2. **Pengambilan Parsial Berbasis Rentang (*HTTP Range Request GET*)**: Mesin kueri tidak mengunduh seluruh berkas multi-gigabyte dari S3, melainkan hanya membaca potongan byte spesifik (*byte ranges*) dari kolom dan baris yang dibutuhkan memanfaatkan metadata footer Parquet.
3. **Tembolok Cerdas Berbasis LRU (*Local SSD Cache*)**: Data yang baru saja ditarik dari S3 disimpan di tembolok lokal sementara untuk mempercepat kueri berulang berikutnya.

### 27.2 Implementasi Kode Program Utuh

Berikut implementasi konfigurasi kebijakan migrasi penyimpanan bertingkat dan eksekusi kueri transparan terhadap penyimpanan objek S3 pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Penyimpanan Objek Dingin S3 dan Tiering Otomatis WidyaBase
modul penyimpanan_s3_tiering {
    impor ekosistem::widyabase::{
        WidyaBase, KonfigurasiS3, KebijakanTiering,
        TingkatPenyimpanan, HasilMigrasiTier
    };

    fungsi inisialisasi_penyimpanan_bertingkat(mesin: &mut WidyaBase) -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [TIERING-S3] Mengonfigurasi Integrasi Penyimpanan Objek Dingin...");

        // 1. Daftarkan Kredensial Repositori Penyimpanan Objek S3 Berdaulat
        biar sql_konfigurasi_s3 = "
            BUAT REPOSITORI_OBJEK s3_arsip_nasional
            TIPE = 'S3_KOMPATIBEL'
            DENGAN (
                titik_akhir = 'https://s3-internal.data.go.id',
                ember = 'arsip-audit-keuangan',
                kunci_akses = 'WIDYA_ACCESS_KEY_2026',
                kunci_rahasia = 'WIDYA_SECRET_KEY_BERDAULAT',
                wilayah = 'id-nusantara-1'
            );
        ";
        mesin.eksekusi_sql(sql_konfigurasi_s3)?;
        cetak(">>> [S3] Repositori objek 's3_arsip_nasional' berhasil terhubung.");

        // 2. Pasang Kebijakan Siklus Hidup Otomatis pada Tabel Audit
        biar sql_kebijakan = "
            ATUR KEBIJAKAN_TIERING PADA log_audit_rekening
            PINDAHKAN_KE 'WARM_TIER' SETELAH '14 HARI',
            PINDAHKAN_KE s3_arsip_nasional SETELAH '60 HARI'
            DENGAN (FORMAT_EKSPOR = 'PARQUET', KOMPRESI = 'ZSTD');
        ";
        mesin.eksekusi_sql(sql_kebijakan)?;
        cetak(">>> [TIERING] Aturan siklus hidup 14 hari (Warm) dan 60 hari (Cold S3) aktif.");

        // 3. Simulasikan Pemindahan Data Historis Lampau ke Objek Dingin
        cetak(">>> [MIGRASI] Menjalankan daemon migrasi blok data lampau...");
        biar hasil_migrasi = mesin.jalankan_migrasi_tiering_seketika("log_audit_rekening")?;
        cetak(">>> [MIGRASI] Berhasil memindahkan " + teks(hasil_migrasi.jumlah_berkas) + " berkas chunk ke S3.");
        cetak(">>> [PENGHEMATAN] Ruang disk NVMe lokal dibebaskan: " + teks(hasil_migrasi.mb_dibebaskan) + " MB.");

        // 4. Kueri Transparan: Mengambil Data Tahun 2024 dari S3 via HTTP Range Request
        biar sql_kueri_s3 = "
            PILIH no_rek, saldo_lama, saldo_baru, waktu_kejadian
            DARI log_audit_rekening
            DIMANA waktu_kejadian ANTARA '2024-01-01' DAN '2024-01-31'
            BATAS 3;
        ";
        cetak(">>> [SQL] Menjalankan kueri data dingin secara transparan...");
        biar hasil = mesin.kueri_sql(sql_kueri_s3)?;
        cetak(">>> [HASIL DARI S3]: Data terambil langsung via HTTP Range Request:");
        untuk baris di hasil.baris_data() {
            cetak("  - Rekening: " + baris.ambil_teks("no_rek") + 
                  " | Saldo: Rp " + teks(baris.ambil_desimal("saldo_baru")) +
                  " | Waktu: " + teks(baris.ambil_cap_waktu("waktu_kejadian")));
        }

        cetak("==================================================================");
        kembalikan Ok(benar);
    }
}
```

### 27.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan karakteristik tiga tingkat penyimpanan (*Storage Tiers*) pada WidyaBase:

| Karakteristik Tingkat | Hot Tier (SSD NVMe) | Warm Tier (SSD SATA / Kolom) | Cold Tier (S3 Object Storage) |
| :--- | :--- | :--- | :--- |
| **Media Fisik** | SSD PCIe 4.0/5.0 NVMe | SSD SATA / SAS HDD | Awan Kompatibel S3 (MinIO/Ceph) |
| **Latensi Akses I/O** | 10–50 Mikrodetik | 200–800 Mikrodetik | 20–80 Milidetik |
| **Throughput Transfer**| 5.000–7.000 MB/s | 500–1.200 MB/s | 100–1.000 MB/s Jaringan LAN |
| **Biaya Penyimpanan per TB**| Sangat Tinggi (Tier 1) | Sedang (Tier 2) | **Sangat Murah (Hemat s/d 90%)** |
| **Dukungan Kueri SQL** | Mutasi Baca/Tulis Penuh ACID | Baca/Tulis Batch Analitis | **Hanya-Baca Transparan (Parquet)** |

### 27.4 Latihan Soal Esai

1. Mengapa pemindahan data historis ke tingkat penyimpanan objek dingin (*Cold S3 Tier*) harus dilakukan secara transparan tanpa mengubah skema tabel yang dilihat oleh aplikasi klien?
2. Bagaimana mekanisme fitur *HTTP Range Request* (Header `Range: bytes=start-end`) bekerja sama dengan metadata berkas Parquet untuk meminimalkan volume data yang ditransfer dari S3 ke memori RAM peladen?
3. Uraikan strategi pengelolaan *Local Cache Replacement* (misalnya menggunakan algoritma 2Q atau LRU) untuk menjaga agar kueri berulang pada data dingin tetap responsif tanpa memenuhi kapasitas disk SSD lokal peladen!

### 27.5 Soal Pilihan Ganda

1. Tiga tingkatan hierarki penyimpanan data bertingkat pada sistem enterprise modern dinamakan...
   A. Hot Tier, Warm Tier, dan Cold Tier
   B. Red Tier, Green Tier, dan Blue Tier
   C. Top Tier, Mid Tier, dan Low Tier
   D. Root Tier, Node Tier, dan Leaf Tier

2. Media penyimpanan fisik yang paling tepat dialokasikan untuk Hot Tier transaksi aktif adalah...
   A. SSD NVMe berlatensi sub-milidetik
   B. Pita Magnetik Kaset
   C. CD-ROM
   D. Floppy Disk

3. Standar protokol jaringan penyimpanan objek awan yang didukung WidyaBase untuk tingkat dingin adalah...
   A. S3-Compatible API (Simple Storage Service)
   B. FTP Biasa
   C. Telnet
   D. POP3

4. Fitur protokol HTTP yang memungkinkan pengunduhan potongan byte spesifik dari suatu berkas besar di S3 tanpa mengunduh seluruh isi berkas dinamakan...
   A. HTTP Range Request (`Range: bytes=X-Y`)
   B. HTTP Post Redirect
   C. HTTP 404 Not Found
   D. HTTP Cookie Injection

5. Format berkas yang paling optimal digunakan untuk menyimpan data terarsip di tingkat dingin S3 adalah...
   A. Apache Parquet terkompresi ZSTD / Snappy
   B. Berkas teks mentah TXT tanpa kompresi
   C. Gambar berformat BMP
   D. Dokumen Microsoft Word DOC

6. Apa yang terjadi pada aplikasi pengguna ketika mengeksekusi kueri terhadap data yang sudah dipindahkan ke Cold Tier S3?
   A. Kueri tetap berjalan normal secara transparan melalui SQL biasa tanpa pengguna menyadari data berada di S3
   B. Aplikasi otomatis memunculkan galat server
   C. Komputer pengguna meminta restart
   D. Pengguna diminta mengunduh file secara manual

7. Komponen lokal yang bertugas menampung potongan blok data S3 yang baru saja dibaca agar kueri berikutnya berjalan lebih cepat dinamakan...
   A. Local SSD Cache
   B. WAL Ring Buffer
   C. Dirty Page Table
   D. Wait-For Graph

8. Penghematan biaya infrastruktur penyimpanan yang dapat dicapai dengan memindahkan data historis ke Cold Tier S3 mencapai...
   A. 70% hingga 90% dibandingkan mempertahankan seluruh data di SSD NVMe
   B. Tepat 0%
   C. Biaya justru berlipat ganda
   D. 1% saja

9. Tindakan yang dilakukan oleh daemon *Tiering Policy* ketika sebuah blok data mencapai batas usia yang ditentukan (misal > 60 hari) adalah...
   A. Mengonversi data ke format Parquet, mengunggahnya ke S3, dan membebaskan ruang disk SSD lokal
   B. Menghapus data selamanya tanpa salinan
   C. Mengirimkan email ke administrator
   D. Mematikan listrik ruang data center

10. Mengapa pemanfaatan MinIO atau Ceph lokal (*on-premise S3-compatible*) sangat penting bagi instansi pertahanan dan keamanan nasional?
    A. Memberikan efisiensi biaya penyimpanan objek dingin sekaligus menjamin kedaulatan data fisik tetap berada di dalam teritorial Indonesia tanpa bocor ke awan asing
    B. Supaya tidak perlu membayar teknisi komputer
    C. Menghilangkan kebutuhan kabel listrik
    D. Menjadikan server dapat bekerja tanpa koneksi daya
