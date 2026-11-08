## Lampiran A: Glosarium Lengkap SQL Dwibahasa

### A.1 Pendahuluan Kamus Sintaksis Kanonikal

WidyaBase memperlakukan kosa kata Dialek Nusantara dan standar internasional ANSI SQL secara setara (*isomorphic mapping*). Seluruh kata kunci dipetakan ke dalam token kanonikal internal yang sama persis di dalam modul `sql_parser.rs`.

Berikut katalog lengkap kata kunci, klausa, operator logika, dan fungsi bawaan dwibahasa pada WidyaBase:

### A.2 Kata Kunci Kueri Data (DQL) & Manipulasi Data (DML)

| Dialek Nusantara (Bahasa Indonesia) | Standar ANSI SQL Internasional | Kategori Tata Bahasa | Penjelasan Fungsi Operasional |
| :--- | :--- | :--- | :--- |
| `PILIH` | `SELECT` | DQL Utama | Mengambil kolom atau ekspresi dari tabel |
| `DARI` | `FROM` | DQL Sumber | Menentukan tabel atau relasi masukan |
| `DIMANA` | `WHERE` | DQL Filter | Menyaring baris berdasarkan predikat |
| `SISIP_KE` / `SISIP` | `INSERT INTO` | DML Penyisipan | Menyisipkan baris baru ke dalam tabel |
| `NILAI` | `VALUES` | DML Data | Mendefinisikan tupel nilai yang disisipkan |
| `PERBARUI` | `UPDATE` | DML Modifikasi | Mengubah nilai kolom baris yang ada |
| `ATUR` | `SET` | DML Penugasan | Menetapkan nilai baru pada kolom |
| `HAPUS_DARI` | `DELETE FROM` | DML Penghapusan | Menghapus baris tertentu dari tabel |
| `KOSONGKAN` | `TRUNCATE` | DDL/DML Cepat | Menghapus seluruh isi tabel secara instan |
| `SEBAGAI` | `AS` | Alias | Memberi nama alias pada kolom atau tabel |
| `UNIK` / `BERBEDA` | `DISTINCT` | Eliminasi | Menghilangkan duplikasi baris luaran |
| `KELOMPOKKAN_BERDASARKAN` | `GROUP BY` | Agregasi Grup | Mengelompokkan baris berdasarkan atribut |
| `MEMILIKI` | `HAVING` | Filter Grup | Menyaring hasil agregasi kelompok |
| `URUTKAN_BERDASARKAN` | `ORDER BY` | Pengurutan | Mengurutkan baris luaran |
| `MENAIK` | `ASC` | Arah Urut | Pengurutan dari kecil ke besar (A-Z, 0-9) |
| `MENURUN` | `DESC` | Arah Urut | Pengurutan dari besar ke kecil (Z-A, 9-0) |
| `BATAS` | `LIMIT` | Paginasi | Membatasi jumlah maksimum baris luaran |
| `LEWATI` | `OFFSET` | Paginasi | Melewati sejumlah $N$ baris pertama |

### A.3 Kata Kunci Definisi Skema (DDL) & Integritas

| Dialek Nusantara | Standar ANSI SQL | Kategori | Penjelasan Fungsi Operasional |
| :--- | :--- | :--- | :--- |
| `BUAT TABEL` | `CREATE TABLE` | DDL Skema | Mendefinisikan tabel relasional baru |
| `HAPUS TABEL` | `DROP TABLE` | DDL Skema | Menghapus tabel beserta seluruh datanya |
| `UBAH TABEL` | `ALTER TABLE` | DDL Skema | Menambah, mengubah, atau menghapus kolom |
| `TAMBAH KOLOM` | `ADD COLUMN` | DDL Kolom | Menyisipkan kolom baru pada tabel yang ada |
| `KUNCI_UTAMA` | `PRIMARY KEY` | Batasan Unik | Menetapkan pengenal unik baris tabel |
| `KUNCI_ASING` | `FOREIGN KEY` | Integritas Ref | Menghubungkan kolom ke tabel induk |
| `MERUJUK_KE` | `REFERENCES` | Integritas Ref | Menentukan tabel dan kolom yang dirujuk |
| `TIDAK_KOSONG` | `NOT NULL` | Batasan Nilai | Melarang kolom bernilai kosong |
| `BAKU` / `DEFAULT` | `DEFAULT` | Nilai Awal | Menetapkan nilai baku saat penyisipan |
| `PERIKSA` | `CHECK` | Batasan Logis | Memvalidasi ekspresi logika nilai kolom |
| `BUAT INDEKS` | `CREATE INDEX` | Akselerasi | Membangun struktur indeks (B+Tree, GIN, HNSW) |
| `BUAT TAMPILAN` | `CREATE VIEW` | Virtualisasi | Menyimpan definisi kueri sebagai view |
| `BUAT PEMICU` | `CREATE TRIGGER` | Otomasi Event | Mendaftarkan prosedur pemicu transaksi |

### A.4 Operator Logika & Pembanding

| Dialek Nusantara | Standar SQL | Representasi Simbol | Makna Logika Matematika |
| :--- | :--- | :--- | :--- |
| `DAN` | `AND` | `&&` | Konjungsi logika (Keduanya harus benar) |
| `ATAU` | `OR` | `\|\|` | Disjungsi logika (Salah satu benar) |
| `BUKAN` | `NOT` | `!` | Negasi logika (Membalik nilai kebenaran) |
| `SAMA_DENGAN` | `=` | `=` | Kesetaraan nilai |
| `TIDAK_SAMA` | `<>` / `!=` | `<>` | Ketidaksamaan nilai |
| `LEBIH_BESAR` | `>` | `>` | Pembanding lebih besar |
| `LEBIH_KECIL` | `<` | `<` | Pembanding lebih kecil |
| `ANTARA ... DAN` | `BETWEEN ... AND` | `BETWEEN` | Pengujian rentang tertutup inklusif |
| `SEPERTI` | `LIKE` | `LIKE` | Pencocokan pola string (wildcard `%` dan `_`) |
| `DI_DALAM` | `IN` | `IN` | Keanggotaan himpunan nilai |
| `ADALAH KOSONG` | `IS NULL` | `IS NULL` | Pengujian ketiadaan nilai |
| `ADALAH BUKAN KOSONG` | `IS NOT NULL`| `IS NOT NULL` | Pengujian keberadaan nilai |

### A.5 Fungsi Bawaan Multi-Modal Terpadu

| Fungsi Dialek Nusantara | Padanan Standar Global | Domain Subsistem | Deskripsi Kegunaan |
| :--- | :--- | :--- | :--- |
| `JUMLAH(kolom)` | `SUM(column)` | Agregasi Matematika | Menghitung total akumulasi nilai angka |
| `RATA_RATA(kolom)` | `AVG(column)` | Agregasi Matematika | Menghitung rata-rata aritmatika |
| `HITUNG(kolom)` | `COUNT(column)` | Agregasi Hitung | Menghitung jumlah baris |
| `MINIMAL(kolom)` | `MIN(column)` | Agregasi Ekstrem | Mengambil nilai terkecil |
| `MAKSIMAL(kolom)` | `MAX(column)` | Agregasi Ekstrem | Mengambil nilai terbesar |
| `NOMOR_BARIS()` | `ROW_NUMBER()` | Fungsi Jendela | Menghasilkan nomor urut baris di partisi |
| `PERINGKAT()` | `RANK()` | Fungsi Jendela | Memberikan peringkat urutan (dengan lompatan) |
| `PERINGKAT_RAPAT()`| `DENSE_RANK()` | Fungsi Jendela | Memberikan peringkat rapat tanpa lompatan |
| `SEBELUMNYA(k, n)` | `LAG(c, n)` | Fungsi Jendela | Mengambil nilai baris lampau berjarak $n$ |
| `BERIKUTNYA(k, n)` | `LEAD(c, n)` | Fungsi Jendela | Mengintip nilai baris masa depan |
| `EMBED_TEKS(string)`| `AI_EMBED(text)` | Vektor AI Semantik | Mengubah teks jadi vektor 384-D lokal |
| `JARAK_KOSINUS(v1, v2)`| `COSINE_DIST(v1, v2)`| Vektor AI Semantik | Menghitung jarak kemiripan sudut kosinus |
| `SKOR_BM25(teks, q)`| `BM25_SCORE(text, q)`| Pencarian Teks Penuh| Mengukur relevansi leksikal kata kunci |
| `SKOR_HIBRIDA_RRF(...)`| `RRF_SCORE(...)` | Mesin Hibrida | Menggabungkan BM25 dan HNSW via RRF |
| `TITIK_SPASIAL(x, y)`| `ST_Point(x, y)` | Geospasial GIS | Membuat objek geometri titik 2D |
| `JARAK_HAVERSINE(p1, p2)`| `ST_DistanceSphere(..)`| Geospasial GIS | Jarak lengkung bumi bola (kilometer) |
| `TITIK_DALAM_POLIGON(..)`| `ST_Contains(poly, pt)`| Geospasial GIS | Uji ray-casting titik dalam poligon |
| `EMBER_WAKTU(interval, t)`| `time_bucket(interval, t)`| Mesin Deret Waktu | Pengelompokan interval deret waktu |
| `BACA_PARQUET(path)`| `read_parquet(path)` | Gudang Data Kolom | Kueri langsung ke berkas Parquet disk |
