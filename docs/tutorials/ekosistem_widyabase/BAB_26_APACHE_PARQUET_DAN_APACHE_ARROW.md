## Bab 26: Integrasi Apache Parquet & Format Memori Apache Arrow

### 26.1 Standar Terbuka Ekosistem Big Data: Parquet Disk & Arrow RAM

Dalam era kedaulatan digital nasional, sebuah sistem basis data modern tidak boleh terkungkung dalam format biner tertutup (*proprietary format*) yang mengunci pengguna (*vendor lock-in*). WidyaBase dibangun dengan prinsip keterbukaan penuh terhadap standar industri Big Data dunia melalui implementasi 100% mandiri native dari dua standar paling berpengaruh:
1. **Apache Parquet (Standar Penyimpanan Disk Fisik)**: Format berkas biner kolom berorientasi partisi kelompok baris (*RowGroup*) dengan metadata footer hierarkis dan penanda tanda ajaib (*Magic Bytes*) `PAR1` (`parquet_arrow.rs`).
2. **Apache Arrow (Standar Memori Kerja RAM & Streaming IPC)**: Format representasi data memori kolom berkecepatan tinggi dengan penyelarasan memori SIMD 64-byte (*64-byte memory alignment*) dan penanda `ARROW1`.

```mermaid
flowchart LR
    subgraph DiskParquet["Penyimpanan Fisik Disk: Apache Parquet (PAR1)"]
        P1["Magic: PAR1"] --> RG1["RowGroup 1 (1M Baris)"]
        RG1 --> RG2["RowGroup 2 (1M Baris)"]
        RG2 --> P2["FileMetaData Footer & Schema Dictionary"]
        P2 --> P3["Magic: PAR1"]
    end
    subgraph MemoriArrow["Memori Kerja RAM: Apache Arrow (ARROW1 Zero-Copy)"]
        A1["RecordBatch IPC Stream (Continuation 0xFFFFFFFF)"]
        A2["SIMD 64-Byte Aligned Data Buffers (Float64 / Int64)"]
        A3["Validity Bitmaps (Masker Nilai Null)"]
    end
    DiskParquet -->|Membaca Tanpa Dekompresi Menengah (Zero-Copy Transfer)| MemoriArrow
    MemoriArrow -->|Interoperabilitas Kilat Tanpa Konversi Serialisasi| BI["Ekosistem Analitik: Python Polars / DuckDB / Rust DataFusion"]
```

Pilar interoperabilitas berdaulat WidyaBase:
- **Zero-Copy Data Transfer**: Pembacaan berkas Parquet ke memori Arrow tidak memerlukan penyalinan buffer atau alokasi ulang memori (*zero-copy IPC streaming*).
- **Pemotongan Kelompok Baris Berdasarkan Statistik (*RowGroup Pruning*)**: Metadata footer Parquet mencatat nilai minimum dan maksimum (`min_value`, `max_value`) untuk setiap blok kolom. Kueri dengan filter nilai dapat langsung melewati puluhan gigabyte data tanpa membacanya sama sekali.
- **Dukungan Tipe Data Kanonikal**: Mendukung secara native tipe `Int64`, `Float64`, `Utf8`, dan `Boolean` dengan penanganan nilai kosong berbasis *Validity Bitmaps*.

### 26.2 Implementasi Kode Program Utuh

Berikut implementasi ekspor-impor berkas Apache Parquet dan komputasi agregasi tervektorisasi memori Apache Arrow pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Integrasi Apache Parquet dan Apache Arrow WidyaBase
modul mesin_parquet_arrow {
    impor ekosistem::widyabase::{
        WidyaBase, BerkasParquet, KumpulanArrow, SkemaKolom,
        TipeKolom, AgregasiSIMD, HasilAgregasi
    };

    fungsi demonstrasi_parquet_dan_arrow(mesin: &mut WidyaBase) -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [PARQUET-ARROW] Memulai Pengujian Interoperabilitas Parquet & Arrow...");

        // 1. Ekspor Data Tabel Relasional ke Format Berkas Apache Parquet (PAR1)
        biar jalur_berkas = "/home/devops/widya-lang/docs/tutorials/ekspor_keuangan.parquet";
        cetak(">>> [PARQUET] Mengekspor tabel keuangan ke format Parquet...");

        biar kueri_ekspor = "
            SALIN (
                PILIH id, kementerian, nominal_realisasi 
                DARI perbendaharaan_negara
            ) KE '" + jalur_berkas + "' 
            DENGAN (FORMAT = 'PARQUET', KOMPRESI = 'SNAPPY');
        ";
        mesin.eksekusi_sql(kueri_ekspor)?;
        cetak(">>> [PARQUET] Berkas Parquet berhasil dibuat dengan header/footer PAR1.");

        // 2. Baca Berkas Parquet Langsung ke Buffer Memori Apache Arrow (Zero-Copy)
        cetak(">>> [ARROW] Membaca RecordBatch Apache Arrow dengan alignment 64-byte...");
        biar kumpulan_batch: KumpulanArrow = BerkasParquet::baca_ke_arrow(jalur_berkas)?;
        cetak(">>> [ARROW] Total baris dimuat di memori: " + teks(kumpulan_batch.jumlah_baris()));

        // 3. Eksekusi Komputasi Analitik SIMD Langsung di Atas Buffer Memori Arrow
        biar vektor_nominal = kumpulan_batch.ambil_vektor_float64("nominal_realisasi")?;
        biar total_simd = AgregasiSIMD::jumlah_float64(&vektor_nominal);
        biar rata_simd  = AgregasiSIMD::rata_rata_float64(&vektor_nominal);
        biar min_simd   = AgregasiSIMD::nilai_minimum_float64(&vektor_nominal);
        biar max_simd   = AgregasiSIMD::nilai_maksimum_float64(&vektor_nominal);

        cetak(">>> [HASIL AGREGASI SIMD ARROW]:");
        cetak("  - Total Belanja (SUM)  : Rp " + teks(total_simd));
        cetak("  - Rata-rata (MEAN)     : Rp " + teks(rata_simd));
        cetak("  - Nilai Terendah (MIN) : Rp " + teks(min_simd));
        cetak("  - Nilai Tertinggi (MAX): Rp " + teks(max_simd));

        // 4. Kueri SQL Virtual Terhadap Berkas Parquet Eksternal Tanpa Impor
        biar sql_eksternal = "
            PILIH kementerian, nominal_realisasi 
            DARI BACA_PARQUET('" + jalur_berkas + "')
            DIMANA nominal_realisasi > 50000000000.0;
        ";
        cetak(">>> [SQL] Menjalankan kueri virtual BACA_PARQUET()...");
        biar hasil_virtual = mesin.kueri_sql(sql_eksternal)?;
        untuk baris di hasil_virtual.baris_data() {
            cetak("  ★ Realisasi Besar: " + baris.ambil_teks("kementerian") + 
                  " (Rp " + teks(baris.ambil_desimal("nominal_realisasi")) + ")");
        }

        cetak("==================================================================");
        kembalikan Ok(benar);
    }
}
```

### 26.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel spesifikasi arsitektur biner Apache Parquet dan Apache Arrow pada WidyaBase:

| Karakteristik Spesifikasi | Apache Parquet (Penyimpanan Disk) | Apache Arrow (Memori RAM & IPC) |
| :--- | :--- | :--- |
| **Penanda Biner (Magic Bytes)** | `PAR1` (4 byte di awal & akhir berkas) | `ARROW1` (6 byte penanda format) |
| **Lokasi Metadata Utama** | Footer Berkas (*FileMetaData* di ujung akhir) | Header IPC Stream (*Message Framing Flatbuffers*) |
| **Penyelarasan Memori (Alignment)**| Bebas (Berorientasi efisiensi kompresi disk)| **Wajib 64-Byte CPU Cache Alignment** |
| **Representasi Nilai Kosong (Null)**| Definition Levels (Bit-Packed RLE) | **Validity Bitmaps (1 bit per nilai baris)** |
| **Target Penggunaan Utama** | Arsip jangka panjang, Data Lake, S3 Storage | Pemrosesan SIMD in-memory, transfer antar-proses |

### 26.4 Latihan Soal Esai

1. Mengapa metadata footer pada berkas Apache Parquet diletakkan di bagian paling ujung akhir berkas (*file footer*) dan bukan di bagian awal (*header*)?
2. Terangkan pentingnya penyelarasan memori 64-byte (*64-byte memory alignment*) pada format Apache Arrow dalam kaitannya dengan eksekusi instruksi CPU SIMD (seperti AVX-512)!
3. Jelaskan mekanisme kerja teknik *RowGroup Pruning* pada pembaca berkas Parquet saat mengeksekusi kueri dengan klausa filter predikat nilai!

### 26.5 Soal Pilihan Ganda

1. Penanda biner ajaib (*Magic Bytes*) 4-byte yang menandai awal dan akhir berkas Apache Parquet standar adalah...
   A. `PAR1`
   B. `SQL1`
   C. `DOCX`
   D. `ZIP0`

2. Penanda biner ajaib yang menandai awal berkas format IPC Apache Arrow adalah...
   A. `ARROW1`
   B. `PARQUET`
   C. `RUSTDB`
   D. `LINUX0`

3. Penyelarasan batas memori RAM (*memory alignment*) yang diwajibkan oleh standar Apache Arrow untuk memaksimalkan instruksi SIMD CPU adalah...
   A. 64-byte alignment
   B. 1-byte alignment
   C. 10-byte alignment
   D. 1024-byte alignment

4. Struktur pada Apache Arrow yang bertugas mencatat apakah suatu baris data bernilai valid atau bernilai kosong (*NULL*) adalah...
   A. Validity Bitmap (1 bit per baris)
   B. B+Tree Node
   C. Wal Segment
   D. Hash Bucket

5. Keuntungan utama dari pendekatan *Zero-Copy Transfer* antara format memori Apache Arrow dan mesin analitis adalah...
   A. Data dapat langsung diproses tanpa perlu alokasi ulang memori atau konversi serialisasi yang membebani CPU
   B. Basis data tidak memerlukan CPU sama sekali
   C. Menghapus kebutuhan harddisk fisik
   D. Mengurangi penggunaan bandwidth internet hingga nol

6. Komponen pada berkas Parquet yang membagi tabel data besar menjadi potongan-potongan tumpukan vertikal berukuran jutaan baris dinamakan...
   A. RowGroup
   B. Slotted Frame
   C. Partition Ring
   D. Cluster Ring

7. Teknik optimasi pembacaan Parquet yang melewati pembacaan suatu RowGroup jika nilai predikat filter kueri berada di luar rentang statistik `min_value` dan `max_value` dinamakan...
   A. RowGroup Pruning (Pemangkasan Kelompok Baris)
   B. Table Escalation
   C. Latch Crabbing
   D. Vacuuming

8. Fungsi SQL virtual pada WidyaBase yang memungkinkan pembacaan kueri langsung terhadap berkas Parquet eksternal tanpa impor tabel adalah...
   A. `BACA_PARQUET('nama_file.parquet')` (atau `read_parquet`)
   B. `IMPORT_CSV`
   C. `UNZIP_FILE`
   D. `EXEC_SHELL`

9. Mengapa format Parquet sangat dominan digunakan dalam arsitektur Modern Data Lakehouse dan Big Data Warehouse?
   A. Memiliki kompresi kolom yang sangat hemat ruang disk dan didukung oleh hampir seluruh mesin analitik dunia
   B. Karena berkas Parquet bisa dibuka di Microsoft Word
   C. Karena format Parquet tidak memerlukan izin sistem operasi
   D. Karena ukuran berkas selalu kelipatan 10

10. Apa manfaat strategis penguasaan implementasi 100% native Apache Parquet dan Arrow di dalam WidyaBase bagi kemandirian teknologi nasional?
    A. Menjamin kompatibilitas penuh dengan ekosistem AI dan Big Data global tanpa ketergantungan pada pustaka proprietary asing
    B. Menghapus kebutuhan belajar bahasa pemrograman lain
    C. Mengurangi konsumsi memori monitor komputer
    D. Mempercepat koneksi kabel HDMI
