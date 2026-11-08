## Bab 25: Format Penyimpanan Kolom Analitis OLAP

### 25.1 Anatomi Penyimpanan Kolom: Mengapa OLAP Menuntut Columnar Layout

Pada sistem pemrosesan analitis skala besar (*Big Data Analytics* / OLAP), karakteristik kueri berbeda 180 derajat dibandingkan kueri transaksional OLTP. Kueri analitis tipikal (seperti kueri laporan keuangan tahunan atau agregasi sensus penduduk) biasanya memindai puluhan juta hingga miliaran baris data, namun **hanya membutuhkan 2 atau 3 kolom saja** (misalnya kolom `provinsi` dan `nominal_belanja`).

Jika kueri tersebut dijalankan di atas basis data berorientasi baris (*Row-Oriented*) seperti slotted-page biasa:
- Sistem terpaksa membaca seluruh lebar baris dari piringan disk ke RAM (termasuk puluhan kolom lain yang tidak relevan seperti alamat, nomor telepon, catatan medis).
- Menghabiskan 95% bandwidth I/O secara sia-sia untuk membuang data yang tidak diminta.

WidyaBase menyediakan **Mesin Penyimpanan Kolom Analitis (*Columnar OLAP Engine*)** berkinerja tinggi (`columnar.rs`):
- **Tata Letak Berorientasi Kolom (*Columnar Layout*)**: Data disimpan berurutan per kolom. Nilai-nilai dari kolom yang sama dikelompokkan ke dalam blok memori (*Column Chunks*).
- **Rasio Kompresi Homogen Ekstrem**: Karena seluruh nilai di dalam satu blok kolom memiliki tipe data dan distribusi yang seragam, mesin dapat menerapkan algoritma kompresi khusus:
  - *Run-Length Encoding* (RLE): Sangat optimal untuk kolom dengan nilai berulang berurutan (misal: kode negara, status aktif).
  - *Dictionary Encoding*: Mengganti string berulang dengan bilangan bulat ringkas (1–2 byte).
  - *Delta Encoding*: Menyimpan selisih selang antar bilangan terurut.
- **MergeTree Engine**: Mengadopsi arsitektur penyisipan tumpukan (*batched append-only*) yang secara berkala menggabungkan dan mengurutkan potongan blok (*background block merging*) mirip dengan arsitektur ClickHouse.

```mermaid
flowchart TD
    subgraph TataLetakBaris["1. Row-Oriented Layout (OLTP Slotted-Page)"]
        R1["Baris 1: [ID=1, Nama='Budi', Kota='Jakarta', Belanja=150rb]"]
        R2["Baris 2: [ID=2, Nama='Siti', Kota='Bandung', Belanja=200rb]"]
        R3["Baris 3: [ID=3, Nama='Agus', Kota='Surabaya', Belanja=350rb]"]
        R1 --- R2 --- R3
    end
    subgraph TataLetakKolom["2. Columnar Layout WidyaBase (OLAP Column Chunks)"]
        C1["Kolom ID: [1, 2, 3] (Bit-Packed)"]
        C2["Kolom Nama: ['Budi', 'Siti', 'Agus'] (Dictionary)"]
        C3["Kolom Kota: ['Jakarta', 'Bandung', 'Surabaya'] (RLE)"]
        C4["Kolom Belanja: [150000, 200000, 350000] (SIMD Vector)"]
    end
    Q["Kueri Analitis: JUMLAH(Belanja)"] -->|Hanya membaca 1 kolom tunggal (C4)! Menghemat 80% I/O Disk| C4
```

### 25.2 Implementasi Kode Program Utuh

Berikut implementasi pembuatan tabel kolom analitis MergeTree, penyisipan tumpukan, dan agregasi tervektorisasi pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Format Penyimpanan Kolom Analitis OLAP WidyaBase
modul mesin_kolom_olap {
    impor ekosistem::widyabase::{
        WidyaBase, MesinKolom, VektorKolom, TipeDataKolom,
        MesinMergeTree, HasilAgregasiVektor
    };

    fungsi jalankan_analitik_gudang_data(mesin: &mut WidyaBase) -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [COLUMNAR-OLAP] Mengonfigurasi Tabel Mesin Kolom MergeTree...");

        // 1. Buat Tabel Kolom Analitis Transaksi E-Commerce
        biar sql_tabel = "
            BUAT TABEL log_transaksi_kolom (
                id_transaksi BULAT,
                kategori_produk TEKS,
                kode_provinsi TEKS,
                nilai_belanja DESIMAL,
                status_pembayaran TEKS
            ) MENGGUNAKAN MESIN_MERGETREE()
            URUTKAN_BERDASARKAN (kode_provinsi, kategori_produk);
        ";
        mesin.eksekusi_sql(sql_tabel)?;
        cetak(">>> [MERGETREE] Tabel kolom berhasil dibuat dengan pengurutan kompresi.");

        // 2. Lakukan Penyisipan Tumpukan (Batched Block Insertion)
        cetak(">>> [BATCH-INSERT] Menyisipkan 100.000 baris data analitis...");
        mesin.eksekusi_penyisipan_blok_kolom("log_transaksi_kolom", 100000)?;

        // 3. Picu Penggabungan Blok di Latar Belakang (Merge Chunks)
        mesin.picu_penggabungan_mergetree("log_transaksi_kolom")?;
        cetak(">>> [MERGE] Blok-blok kolom berhasil dipadukan dan dikompresi RLE/Dictionary.");

        // 4. Jalankan Kueri Agregasi Tervektorisasi SIMD
        biar sql_agregasi = "
            PILIH kode_provinsi,
                  HITUNG(*) SEBAGAI volume_transaksi,
                  JUMLAH(nilai_belanja) SEBAGAI total_omset,
                  RATA_RATA(nilai_belanja) SEBAGAI rata_transaksi
            DARI log_transaksi_kolom
            DIMANA status_pembayaran = 'SUKSES'
            KELOMPOKKAN_BERDASARKAN kode_provinsi
            URUTKAN_BERDASARKAN total_omset MENURUN
            BATAS 5;
        ";

        cetak(">>> [SIMD-OLAP] Mengeksekusi agregasi kolom SIMD berkecepatan tinggi...");
        biar hasil = mesin.kueri_sql(sql_agregasi)?;
        cetak(">>> [HASIL ANALISIS OMSET REGIONAL]:");
        untuk baris di hasil.baris_data() {
            cetak("  - Provinsi: " + baris.ambil_teks("kode_provinsi") + 
                  " | Transaksi: " + teks(baris.ambil_bulat("volume_transaksi")) +
                  " | Omset: Rp " + teks(baris.ambil_desimal("total_omset")));
        }

        cetak("==================================================================");
        kembalikan Ok(benar);
    }
}
```

### 25.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan metode kompresi pada format kolom WidyaBase:

| Metode Kompresi Kolom | Karakteristik Pola Data | Rasio Penghematan | Efisiensi Dekompresi CPU |
| :--- | :--- | :--- | :--- |
| **Run-Length Encoding (RLE)** | Data dengan nilai identik berulang berurutan | 80–95% Penghematan | Instan ($O(1)$ kalkulasi panjang) |
| **Dictionary Encoding** | Kolom string ber-kardinalitas rendah (< 10.000 unik) | 70–85% Penghematan | Mengganti string jadi integer ringkas |
| **Bit-Packing / Frame of Ref.**| Kolom integer dengan rentang sempit | 60–75% Penghematan | Memanfaatkan bit CPU langsung |
| **Delta / Double-Delta** | Kolom angka/waktu meningkat monoton | 75–90% Penghematan | Rekonstruksi penjumlahan linier |
| **Snappy / Zstandard Block** | Kompresi blok generik cadangan | 50–70% Penghematan | Throughput gigabyte per detik |

### 25.4 Latihan Soal Esai

1. Terangkan secara fisik bagaimana perbedaan tata letak penyimpanan antara model baris (*Row-Oriented*) dan model kolom (*Column-Oriented*) mempengaruhi konsumsi bandwidth bus memori dan I/O piringan disk saat menjalankan kueri agregasi!
2. Bagaimana cara kerja algoritma kompresi *Dictionary Encoding* pada kolom data teks berskala jutaan baris? Mengapa evaluasi predikat filter dapat dieksekusi langsung di atas kamus integer tanpa mendekompresi teks asli?
3. Jelaskan arsitektur mesin *MergeTree* dalam menangani operasi penulisan data analitis (*append-only batches*) dan bagaimana proses penggabungan blok latar belakang (*background merging*) mempertahankan performa kueri yang stabil!

### 25.5 Soal Pilihan Ganda

1. Karakteristik utama dari format penyimpanan kolom analitis (Columnar Storage) adalah...
   A. Seluruh nilai dari kolom yang sama disimpan berdekatan secara fisik di dalam blok memori piringan disk
   B. Setiap baris tabel disimpan di file yang berbeda
   C. Basis data tidak mengizinkan kueri SELECT
   D. Memori RAM server diubah menjadi harddisk

2. Pada kueri analitis yang hanya menghitung `JUMLAH(gaji)` dari tabel yang memiliki 80 kolom, keuntungan utama format kolom adalah...
   A. Mesin hanya membaca 1 blok kolom gaji dan mengabaikan 79 kolom lainnya sehingga menghemat I/O secara masif
   B. Gaji karyawan otomatis berlipat ganda
   C. Kueri berjalan tanpa menggunakan listrik
   D. Menghapus kebutuhan indeks B+Tree primer

3. Teknik kompresi kolom yang menggantikan rangkaian nilai berulang yang sama berturut-turut dengan pasangan `(nilai, jumlah_pengulangan)` dinamakan...
   A. Run-Length Encoding (RLE)
   B. Huffman Code
   C. RSA Encryption
   D. Fast Fourier Transform

4. Teknik kompresi kolom yang memetakan nilai string unik ke nomor indeks bilangan bulat ringkas disebut...
   A. Dictionary Encoding (Pengodean Kamus)
   B. Bit Reversal
   C. CRC32 Checksum
   D. Base64 Encoding

5. Arsitektur mesin penyimpanan kolom yang mengelompokkan data ke dalam bagian-bagian terurut dan menggabungkannya secara berkala di latar belakang dinamakan...
   A. MergeTree Engine
   B. BubbleSort Engine
   C. Random Forest
   D. Slotted Manager

6. Mengapa kompresi kolom menghasilkan rasio penghematan disk yang jauh lebih tinggi dibandingkan kompresi baris?
   A. Karena data dalam satu kolom memiliki tipe data dan nilai yang homogen sehingga redundansi bit sangat tinggi
   B. Karena data kolom diacak secara acak
   C. Karena kolom disimpan tanpa nama
   D. Karena ukuran disk bertambah otomatis

7. Istilah pemrosesan kueri analitis di mana operasi agregasi dijalankan di atas larik nilai kolom sekaligus memanfaatkan instruksi SIMD CPU dinamakan...
   A. Vectorized Query Execution
   B. Single Thread Processing
   C. Tuple-At-A-Time
   D. Deadlock Mitigation

8. Kapan pengurutan kunci utama (*Primary Sorting Key*) pada tabel MergeTree dievaluasi?
   A. Data diurutkan di dalam masing-masing blok saat ditulis, lalu dipadukan saat penggabungan blok latar belakang
   B. Hanya saat server dimatikan
   C. Setiap kali pengguna login
   D. Tidak pernah diurutkan

9. Jenis beban kerja komputasi data yang paling tidak cocok menggunakan format kolom murni adalah...
   A. Beban transaksi OLTP dengan jutaan operasi INSERT/UPDATE satu baris (*single-row mutation*) berlatensi sub-milidetik
   B. Laporan keuangan tahunan
   C. Analisis tren data sensor cuaca
   D. Sensus demografi kependudukan

10. Penghematan ruang piringan disk yang umum dicapai oleh format kolom WidyaBase dibandingkan penyimpanan tabel baris standar berkisar antara...
    A. 60% hingga 85% penghematan
    B. Tepat 0%
    C. Kurang dari 5%
    D. 100% (data hilang semua)
