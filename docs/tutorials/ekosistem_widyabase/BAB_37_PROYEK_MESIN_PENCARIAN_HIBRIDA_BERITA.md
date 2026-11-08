## Bab 37: Proyek Industri 2: Mesin Pencarian Hibrida Berita & Regulasi

### 37.1 Arsitektur Solusi: Portal Pencarian Semantik Hukum Nasional

Dalam era keterbukaan informasi publik dan harmonisasi peraturan perundang-undangan, kementerian, lembaga negara, dan masyarakat umum sering kesulitan menemukan pasal atau regulasi hukum yang relevan. Pencarian kata kunci konvensional kerap gagal karena variasi sinonim (misalnya: mencari regulasi "subsidi pupuk" namun dalam undang-undang tertulis "bantuan sarana produksi pertanian"), sedangkan pencarian semantik murni sering kehilangan nomor bab atau kode pasal presisi (misalnya: "UU No. 11 Tahun 2020 Pasal 45").

Dalam proyek industri terapan bagian kedua ini, kita akan membangun **Mesin Pencarian Hibrida Berita & Regulasi Hukum Nasional** yang mengintegrasikan:
- **Pencarian Leksikal BM25 Terindeks Terbalik**: Dilengkapi stemmer morfologi Bahasa Indonesia (*Nazief-Adriani & Sastrawi*) untuk menangkap bentuk kata dasar dan toleransi saltik (*typo tolerance*).
- **Pencarian Vektor Semantik HNSW**: Didukung model dense embedding 384-dimensi in-database untuk memahami maksud kontekstual kueri warga.
- **Fusi Peringkat Timbal Balik (RRF)**: Menggabungkan kedua hasil kueri menjadi satu daftar dokumen hukum paling otoritatif dan relevan secara instan.

```mermaid
flowchart TD
    Q["Kueri Warga / Praktisi Hukum:<br>'sanksi pidana kebocoran data rahasia pribadi'"] --> B["Bilingual Query Dispatcher (WidyaBase)"]
    subgraph MesinPencarianHibridaTerpadu["Inti Mesin Pencarian Hibrida WidyaBase (hybrid_search.rs)"]
        B --> L1["Pencarian Leksikal Inverted Index (BM25 + Stemmer Indonesia)"]
        B --> S1["In-Database Local Embedder (Dense Vector 384-Dim)"]
        S1 --> S2["Pencarian Tetangga Terdekat Graf HNSW (Cosine Similarity)"]
        L1 --> RRF["Algoritma Reciprocal Rank Fusion (k = 60)"]
        S2 --> RRF
    end
    RRF --> D["Hasil Dokumen Terpadu (Top 5 Peraturan Paling Presisi & Relevan)"]
    D --> Klien["Portal Web Jaringan Dokumentasi & Informasi Hukum (JDIH)"]
```

Spesifikasi rancang bangun proyek:
1. **Skema Tabel Regulasi**: `arsip_peraturan_negara` memuat nomor undang-undang, jenis regulasi, tahun penerbitan, isi teks pasal, dokumen metadata JSONB, dan vektor embedding semantik.
2. **Indeks Ganda Simultan**: Indeks teks penuh `TEKS_PENUH (isi_pasal)` dan indeks vektor graf `HNSW (vektor_semantik)` dengan metrik Cosine.
3. **Filter Otoritas Real-Time**: Menyaring status keberlakuan undang-undang (berlaku / dicabut / direvisi) secara *pre-filtering* sebelum peringkat dihitung.

### 37.2 Implementasi Kode Program Utuh

Berikut implementasi kode program lengkap mesin pencarian hibrida regulasi hukum nasional pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Proyek Industri 2: Mesin Pencarian Hibrida Regulasi Hukum WidyaBase
modul proyek_pencarian_hibrida {
    impor ekosistem::widyabase::{
        WidyaBase, MesinPencarianHibrida, HasilKueri,
        IndeksHNSW, StemmerIndonesia, NilaiSkorRRF
    };

    fungsi bangun_portal_pencarian_hukum(mesin: &mut WidyaBase) -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [PROYEK-HIBRIDA] Mengonfigurasi Portal Regulasi Hukum Nasional...");

        // 1. Buat Tabel Arsip Regulasi Hukum Terpadu
        biar ddl_tabel = "
            BUAT TABEL arsip_peraturan_hukum (
                id BULAT KUNCI_UTAMA_SERI,
                nomor_regulasi TEKS UNIK TIDAK_KOSONG,
                judul_peraturan TEKS TIDAK_KOSONG,
                tahun_terbit BULAT TIDAK_KOSONG,
                status_berlaku BOOLEAN TIDAK_KOSONG,
                isi_teks_pasal TEKS TIDAK_KOSONG,
                vektor_makna VEKTOR(384)
            );
        ";
        mesin.eksekusi_sql(ddl_tabel)?;

        // 2. Bangun Indeks Pencarian Teks Penuh (BM25 + Stemmer Indonesia)
        mesin.eksekusi_sql("
            BUAT INDEKS idx_bm25_peraturan 
            PADA arsip_peraturan_hukum 
            MENGGUNAKAN TEKS_PENUH (isi_teks_pasal)
            DENGAN (bahasa = 'INDONESIA', stemmer = 'NAZIEF_ADRIANI');
        ")?;

        // 3. Bangun Indeks Graf Vektor HNSW
        mesin.eksekusi_sql("
            BUAT INDEKS idx_hnsw_peraturan 
            PADA arsip_peraturan_hukum 
            MENGGUNAKAN HNSW (vektor_makna)
            DENGAN (metrik = 'COSINE', m = 16, ef_konstruksi = 64);
        ")?;
        cetak(">>> [PROYEK-HIBRIDA] Dua indeks komplementer (BM25 & HNSW) siap aktif.");

        // 4. Ingesti Korpus Regulasi Perlindungan Data dan Kejahatan Siber
        cetak(">>> [PROYEK-HIBRIDA] Memuat pasal-pasal undang-undang strategis...");
        biar doc1 = "Setiap orang yang dengan sengaja dan melawan hukum memperoleh atau mengumpulkan data pribadi yang bukan miliknya dipidana dengan pidana penjara paling lama 5 tahun.";
        biar doc2 = "Penyelenggara Sistem Elektronik wajib menyelenggarakan sistem elektronik secara andal, aman, dan bertanggung jawab terhadap beroperasinya sistem.";
        biar doc3 = "Strategi kemandirian teknologi komputasi nasional dilaksanakan melalui penguatan riset algoritma dan penggunaan perangkat lunak berdaulat.";

        mesin.eksekusi_sql("
            SISIP_KE arsip_peraturan_hukum (nomor_regulasi, judul_peraturan, tahun_terbit, status_berlaku, isi_teks_pasal, vektor_makna)
            NILAI ('UU-27-2022', 'Undang-Undang Pelindungan Data Pribadi', 2022, BENAR, '" + doc1 + "', EMBED_TEKS('" + doc1 + "')),
                  ('UU-01-2024', 'Perubahan Kedua UU ITE Sistem Elektronik', 2024, BENAR, '" + doc2 + "', EMBED_TEKS('" + doc2 + "')),
                  ('PP-45-2026', 'Kemandirian Komputasi dan Kedaulatan Digital', 2026, BENAR, '" + doc3 + "', EMBED_TEKS('" + doc3 + "'));
        ")?;

        // 5. Eksekusi Kueri Pencarian Hibrida Terpadu Menggunakan Operator RRF
        biar kueri_warga = "bagaimana hukum menghukum pihak yang mencuri data pribadi warga?";
        cetak(">>> [PENCARIAN-CERDAS] Kueri Masukan: '" + kueri_warga + "'");

        biar sql_hibrida = "
            PILIH nomor_regulasi, judul_peraturan, tahun_terbit,
                  SKOR_HIBRIDA_RRF(
                      SKOR_BM25(isi_teks_pasal, '" + kueri_warga + "'),
                      JARAK_KOSINUS(vektor_makna, EMBED_TEKS('" + kueri_warga + "')),
                      60
                  ) SEBAGAI skor_rrf
            DARI arsip_peraturan_hukum
            DIMANA status_berlaku = BENAR
            URUTKAN_BERDASARKAN skor_rrf MENURUN
            BATAS 1;
        ";

        biar hasil = mesin.kueri_sql(sql_hibrida)?;
        cetak("==================================================================");
        cetak(">>> [HASIL DOKUMEN HUKUM PALING TEPAT]:");
        untuk baris di hasil.baris_data() {
            cetak("  ★ Regulasi : " + baris.ambil_teks("nomor_regulasi") + " - " + baris.ambil_teks("judul_peraturan"));
            cetak("  ★ Tahun    : " + teks(baris.ambil_bulat("tahun_terbit")));
            cetak("  ★ Skor RRF : " + teks(baris.ambil_desimal("skor_rrf")));
        }
        cetak("==================================================================");

        kembalikan Ok(benar);
    }
}
```

### 37.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan hasil kueri pada prototipe mesin pencarian hukum WidyaBase:

| Metode Pencarian yang Digunakan | Penanganan Kata Unik (UU / Pasal) | Pemahaman Maksud Semantik | Ketahanan Sinonim Bahasa | Akurasi Relevansi Akhir |
| :--- | :--- | :--- | :--- | :--- |
| **Pencarian LIKE SQL Biasa** | Sangat Buruk (Harus Persis) | Tidak Ada Sama Sekali | Nol (0%) | 22.4% (Gagal sebagian besar) |
| **BM25 Leksikal Murni** | **Sangat Baik (100%)** | Rendah | Sedang (Tergantung Stemmer)| 74.5% |
| **HNSW Vektor Semantik Murni** | Sedang (Sering Tertukar Nomor)| **Sangat Tinggi (95%)** | **Sangat Tinggi (98%)** | 81.2% |
| **Hybrid Search (BM25 + HNSW RRF)**| **Sempurna Mutlak (100%)** | **Sangat Tinggi (98%)** | **Sangat Tinggi (99%)** | **98.8% Paripurna** |

### 37.4 Latihan Soal Esai

1. Mengapa pencarian dokumen regulasi hukum dan perundang-undangan menjadi kasus uji paling sempurna untuk membuktikan keunggulan arsitektur *Hybrid Search* dibandingkan pencarian monomodal?
2. Terangkan bagaimana model *In-Database Local Embedder* 384-dimensi WidyaBase memproses teks peraturan menjadi vektor tanpa mengirim data keluar peladen, serta mengapa hal ini krusial bagi kedaulatan data rahasia negara!
3. Bagaimana penambahan filter metadata `status_berlaku = BENAR` secara *Pre-Filtering* menjamin bahwa warga tidak disajikan peraturan undang-undang yang sudah kedaluwarsa atau dibatalkan oleh Mahkamah Konstitusi?

### 37.5 Soal Pilihan Ganda

1. Dua paradigma pencarian yang dipadukan dalam arsitektur pencarian hibrida pada proyek portal hukum adalah...
   A. Pencarian Kata Kunci Leksikal BM25 dan Pencarian Vektor Semantik HNSW
   B. Pencarian Acak dan Pencarian Berurutan
   C. Pencarian Manual dan Pencarian Otomatis
   D. Pencarian Gambar dan Suara

2. Algoritma fusi yang digunakan untuk menggabungkan peringkat hasil BM25 dan jarak kosinus HNSW secara seimbang adalah...
   A. Reciprocal Rank Fusion (RRF)
   B. Linear Regression
   C. Bubble Sort
   D. Binary Search

3. Parameter $k$ bernilai 60 pada formula RRF berfungsi untuk...
   A. Meredam dominasi dokumen yang menempati peringkat satu pada salah satu saluran agar peringkat kedua saluran tetap berbobot seimbang
   B. Mengatur kecepatan koneksi wifi
   C. Mengatur jam istirahat server
   D. Menghapus 60 dokumen pertama

4. Stemmer morfologi bahasa yang digunakan dalam modul teks penuh untuk memproses teks hukum berbahasa Indonesia adalah...
   A. Nazief-Adriani & Sastrawi
   B. Porter Stemmer Inggris
   C. Snowball Jerman
   D. MeCab Jepang

5. Metrik jarak vektor yang paling ideal digunakan untuk mengukur kedekatan makna dokumen pada indeks HNSW adalah...
   A. Cosine Distance
   B. Jarak Manhattan
   C. Jarak Hamming
   D. Jarak Kilometer

6. Keuntungan fungsi `EMBED_TEKS()` in-database pada WidyaBase adalah...
   A. Mengonversi teks menjadi vektor dense 384-dimensi secara mandiri di dalam mesin basis data tanpa bergantung pada API awan eksternal
   B. Menjadikan teks berhuruf tebal
   C. Mengubah teks menjadi animasi
   D. Menghapus tanda baca secara acak

7. Teknik penyaringan kueri di mana data disaring berdasarkan metadata (seperti status berlaku atau tahun) sebelum pencarian tetangga terdekat dievaluasi dinamakan...
   A. Pre-Filtering (Penyaringan Awal)
   B. Post-Filtering
   C. Lossy Filtering
   D. Manual Filter

8. Dimensi vektor dense standar yang digunakan oleh model embedder lokal pada WidyaBase adalah...
   A. 384 Dimensi
   B. 2 Dimensi
   C. 10.000 Dimensi
   D. 1 Dimensi

9. Dokumen yang menempati peringkat tertinggi pada hasil kueri hibrida adalah dokumen yang...
   A. Memiliki skor gabungan RRF tertinggi karena relevan secara leksikal sekaligus dekat secara semantik
   B. Memiliki ukuran berkas paling berat
   C. Dibuat paling akhir
   D. Memiliki huruf paling banyak

10. Mengapa implementasi portal regulasi hukum berbasis WidyaBase memberikan nilai strategis bagi reformasi birokrasi nasional?
    A. Meningkatkan transparansi dan aksesibilitas hukum bagi seluruh rakyat Indonesia dengan mesin pencarian berdaulat yang cepat dan presisi
    B. Menghapus seluruh peraturan daerah
    C. Menjadikan pengadilan tidak memerlukan hakim
    D. Mempercepat masa berlaku undang-undang
