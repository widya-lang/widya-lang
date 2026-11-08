## Bab 24: Mesin HTAP Terpadu (Hybrid Transactional & Analytical)

### 24.1 Runtuhnya Tembok Pemisah OLTP & OLAP

Selama empat dekade sejarah industri perangkat lunak basis data, dunia enterprise terpaksa memisahkan sistem penyimpanan datanya menjadi dua kubu yang saling terisolasi:
1. **Basis Data Baris Transaksional (OLTP)**: Dioptimalkan untuk membaca dan menulis catatan tunggal berlatensi rendah menggunakan *Row-Oriented Slotted-Page* (seperti PostgreSQL atau MySQL).
2. **Gudang Data Analitis (OLAP)**: Dioptimalkan untuk memindai miliaran data kolom dan agregasi masif menggunakan *Columnar Storage* (seperti ClickHouse atau Snowflake).

Pemisahan dua sistem ini mengharuskan arsitek perangkat lunak membangun pipa data **ETL** (*Extract, Transform, Load*) yang rapuh, mahal perawatannya, dan menimbulkan jeda waktu (*latency lag*) berjam-jam hingga harian sebelum data operasional dapat dianalisis oleh para pengambil keputusan.

WidyaBase menghapus pemisahan kuno ini dengan menghadirkan **Mesin HTAP Terpadu (*Hybrid Transactional & Analytical Processing*)** mandiri tanpa ETL (`htap.rs`):
- **Arsitektur Dual-Engine Simultan**: Menjalankan mesin baris (*Row-Store*) ACID untuk kueri transaksi OLTP dan mesin kolom (*Columnar-Store*) dalam satu wadah runtime yang sama.
- **Sinkronisasi Replikasi In-Memory Zero-ETL**: Setiap mutasi komit pada mesin OLTP secara instan dipropagasi ke dalam format kolom terkompresi di memori kerja RAM dengan jeda waktu mendekati nol (< 5 milidetik).
- **Perutean Kueri Otonom (*Autonomous Query Router*)**: Pengoptimal kueri CBO secara cerdas dan otomatis mendeteksi karakter kueri:
  - Kueri OLTP transaksi (seperti `PILIH * DARI rekening WHERE id = 101`) otomatis dirutekan ke *Row-Store / B+Tree Index*.
  - Kueri OLAP analitis berat (seperti `PILIH divisi, JUMLAH(anggaran) DARI anggaran_nasional GROUP BY divisi`) otomatis dialihkan ke *Columnar-Store SIMD*.

```mermaid
flowchart TD
    A["Kueri SQL Masuk dari Klien"] --> B["Autonomous Query Router (htap.rs & planner.rs)"]
    subgraph MesinDualHTAP["Inti Mesin Dual-Engine WidyaBase (100% Terpadu Zero-ETL)"]
        B -->|Kueri Transaksional Berbobot Halus| C["Row-Store Engine (OLTP)"]
        B -->|Kueri Agregasi & Analitis Masif| D["Columnar Engine (OLAP)"]
        C -->|1. Mutasi Tulis ACID & WAL| E["Buffer Pool Halaman Slotted 4KB"]
        E -->|2. Asynchronous In-Memory Streamer (< 5 ms)| F["In-Memory Arrow / Parquet Column Vectors"]
        F --> D
    end
    C -->|Respons Instan OLTP| G["Klien Pengguna Aplikasi"]
    D -->|Kalkulasi SIMD Kilat OLAP| G
```

Keuntungan berdaulat arsitektur HTAP WidyaBase:
1. **Data Segar Real-Time**: Laporan keuangan, deteksi penipuan transaksi, dan dasbor pimpinan selalu mencerminkan data detik terkini tanpa menunggu proses batch tengah malam.
2. **Pengurangan Kompleksitas Infrastruktur**: Mengeliminasi server ETL tambahan, konektor CDC pihak ketiga, dan lisensi gudang data ganda yang memboroskan anggaran negara.
3. **Penyimpanan Terpadu**: Transaksi ACID tetap terjamin penuh tanpa kompromi konsistensi.

### 24.2 Implementasi Kode Program Utuh

Berikut implementasi sinkronisasi Zero-ETL dan perutean kueri otonom HTAP pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Mesin HTAP Terpadu (Zero-ETL OLTP + OLAP) WidyaBase
modul mesin_htap_terpadu {
    impor ekosistem::widyabase::{
        WidyaBase, MesinHTAP, PeruteKueriOtonom,
        VektorKolom, JenisBebanKerja, HasilKueri
    };

    fungsi demonstrasi_arsitektur_htap(mesin: &mut WidyaBase) -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [HTAP] Menguji Mesin Terpadu Transaksional & Analitis (Zero-ETL)...");

        // 1. Buat Tabel Keuangan Nasional Berkemampuan HTAP
        biar sql_tabel = "
            BUAT TABEL perbendaharaan_negara (
                id BULAT KUNCI_UTAMA_SERI,
                kementerian TEKS TIDAK_KOSONG,
                mata_anggaran TEKS,
                nominal_realisasi DESIMAL,
                waktu_transaksi CAP_WAKTU
            ) DENGAN (AKTIFKAN_HTAP = BENAR);
        ";
        mesin.eksekusi_sql(sql_tabel)?;
        cetak(">>> [HTAP] Tabel perbendaharaan diaktifkan dalam mode Dual Row-Column.");

        // 2. Eksekusi Beban Transaksional Cepat (OLTP)
        cetak(">>> [OLTP] Menyisipkan transaksi pencairan anggaran kementerian...");
        mesin.eksekusi_sql("
            SISIP_KE perbendaharaan_negara (kementerian, mata_anggaran, nominal_realisasi, waktu_transaksi)
            NILAI ('Kemenkes', 'Vaksinasi Nasional', 12500000000.0, SEKARANG()),
                  ('Kemendikbud', 'Bantuan Operasional Sekolah', 45000000000.0, SEKARANG()),
                  ('PUPR', 'Pembangunan Bendungan', 89000000000.0, SEKARANG());
        ")?;

        // 3. Uji Kueri Point-Lookup Transaksional (Dirutekan ke Row-Store)
        biar sql_oltp = "PILIH * DARI perbendaharaan_negara DIMANA id = 1;";
        biar rute_oltp = mesin.analisis_rute_kueri(sql_oltp);
        cetak(">>> [ROUTER] Kueri ID tunggal diarahkan ke: " + rute_oltp.nama_mesin()); // RowStore
        biar hasil_oltp = mesin.kueri_sql(sql_oltp)?;
        cetak(">>> [HASIL OLTP] Data ditemukan via Row-Store.");

        // 4. Uji Kueri Analitis Agregasi Besar (Dirutekan Otonom ke Mesin Kolom SIMD)
        biar sql_olap = "
            PILIH kementerian,
                  JUMLAH(nominal_realisasi) SEBAGAI total_belanja,
                  RATA_RATA(nominal_realisasi) SEBAGAI rata_belanja
            DARI perbendaharaan_negara
            KELOMPOKKAN_BERDASARKAN kementerian
            URUTKAN_BERDASARKAN total_belanja MENURUN;
        ";
        biar rute_olap = mesin.analisis_rute_kueri(sql_olap);
        cetak(">>> [ROUTER] Kueri agregasi kelompok diarahkan ke: " + rute_olap.nama_mesin()); // ColumnarStore SIMD

        biar hasil_olap = mesin.kueri_sql(sql_olap)?;
        cetak(">>> [HASIL OLAP] Analisis dieksekusi secara instan via Columnar Engine:");
        untuk baris di hasil_olap.baris_data() {
            cetak("  - " + baris.ambil_teks("kementerian") + 
                  " | Total: Rp " + teks(baris.ambil_desimal("total_belanja")));
        }

        cetak(">>> [ZERO-ETL] Data real-time langsung teranalisis tanpa jeda waktu ETL!");
        cetak("==================================================================");
        kembalikan Ok(benar);
    }
}
```

### 24.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan paradigma arsitektur antara Tradisional ETL vs WidyaBase Autonomous HTAP:

| Parameter Evaluasi Arsitektur | Pemisahan Tradisional (OLTP + ETL + OLAP) | WidyaBase Autonomous Zero-ETL HTAP |
| :--- | :--- | :--- |
| **Keterkinian Data (*Data Freshness*)**| Tertunda 1–24 Jam (Batch ETL) | **Waktu-Nyata (< 5 milidetik)** |
| **Jumlah Server & Komponen** | 3 Sistem Terpisah (DB, Pipa Kafka/Airflow, Data Warehouse)| **1 Mesin Tunggal Terpadu** |
| **Konsistensi ACID Data** | Rawan desinkronisasi & inkonsistensi | **Kepatuhan ACID 100% Terpadu** |
| **Kecepatan Kueri Analitis** | Cepat di Data Warehouse lampau | **Super Cepat via Instruksi CPU SIMD** |
| **Beban Perawatan & Biaya Operasional**| Sangat Tinggi (Banyak tim spesialis) | **Sangat Hemat & Mandiri Berdaulat** |

### 24.4 Latihan Soal Esai

1. Mengapa pemisahan tradisional antara sistem OLTP dan OLAP yang dihubungkan melalui pipa ETL sering menjadi sumber kerapuhan teknis (*single point of failure*) dan pemborosan anggaran enterprise?
2. Bagaimana mekanisme kerja *Autonomous Query Router* pada WidyaBase dalam menganalisis pohon rencana kueri untuk memutuskan apakah suatu kueri harus dijalankan pada mesin baris atau mesin kolom?
3. Uraikan bagaimana replikasi *In-Memory Zero-ETL* mentransfer perubahan data dari transaksi OLTP ke dalam vektor kolom tanpa menimbulkan dampak penurunan throughput transaksi utama!

### 24.5 Soal Pilihan Ganda

1. Singkatan dari HTAP dalam terminologi sistem basis data adalah...
   A. Hybrid Transactional and Analytical Processing
   B. High Throughput Application Protocol
   C. Hypertext Transmission Access Page
   D. Hardware Testing Automation Platform

2. Dua jenis mesin penyimpanan fisik yang digabungkan dalam arsitektur HTAP WidyaBase adalah...
   A. Row-Store (Berorientasi Baris) dan Columnar-Store (Berorientasi Kolom)
   B. Harddisk dan Floppy Disk
   C. Printer dan Monitor
   D. Bluetooth dan Wifi

3. Keuntungan paling mendasar dari konsep Zero-ETL pada WidyaBase adalah...
   A. Menghilangkan ketergantungan pada pipa ETL eksternal yang lambat dan menyajikan data analitis terkini dalam waktu-nyata
   B. Menghapus kebutuhan data analitis
   C. Memperkecil kapasitas RAM
   D. Mengubah tipe basis data menjadi spreadsheet

4. Komponen cerdas pada WidyaBase yang secara otomatis mengarahkan kueri transaksi ke mesin baris dan kueri analitis ke mesin kolom dinamakan...
   A. Autonomous Query Router
   B. Firewall
   C. Antivirus
   D. DHCP Server

5. Kueri dengan karakteristik seperti `PILIH * DARI nasabah DIMANA nomor_rekening = '123456'` akan diputuskan oleh router untuk dieksekusi pada...
   A. Mesin Baris OLTP (Row-Store via Indeks B+Tree)
   B. Mesin Kolom Parquet
   C. Tape Backup
   D. Server Cetak

6. Kueri dengan karakteristik seperti `PILIH provinsi, JUMLAH(populasi) DARI sensus KELOMPOKKAN_BERDASARKAN provinsi` akan diarahkan ke...
   A. Mesin Kolom Analitis OLAP (Columnar-Store terakselerasi SIMD)
   B. Sequential Scan Lambat
   C. Floppy Drive
   D. Prompt Terminal

7. Jeda waktu sinkronisasi (*replication lag*) perubahan data komit dari mesin transaksi ke mesin kolom memori pada WidyaBase adalah...
   A. Kurang dari 5 milidetik (nyaris instan)
   B. 24 jam
   C. 1 minggu
   D. 1 bulan

8. Akselerasi pemrosesan vektor kolom pada mesin HTAP modern memanfaatkan instruksi pemrosesan paralel tingkat prosesor yang dinamakan...
   A. SIMD (Single Instruction, Multiple Data)
   B. Single Thread Mono Cycle
   C. Interrupt Request
   D. BIOS Call

9. Mengapa kueri agregasi kolom pada Columnar-Store jauh lebih hemat I/O dibandingkan Row-Store?
   A. Mesin kolom hanya membaca data dari kolom-kolom yang terlibat dalam perhitungan tanpa menyentuh kolom lainnya di dalam tabel
   B. Mesin kolom menghapus kolom yang tidak dipakai
   C. Mesin kolom tidak menggunakan media piringan disk
   D. Karena kueri kolom tidak memiliki hasil

10. Dampak positif diterapkannya arsitektur HTAP terpadu WidyaBase pada kedaulatan data instansi pemerintah adalah...
    A. Pengambilan keputusan strategis nasional berbasis data dapat dilakukan secara seketika (*real-time*) dengan biaya infrastruktur mandiri yang efisien
    B. Menghapus kebutuhan pegawai tata usaha
    C. Mematikan sistem pemilu otomatis
    D. Mengganti semua monitor komputer dengan televisi
