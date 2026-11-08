## Bab 21: Mesin Graf Relasional & Jalur Terpendek

### 21.1 Model Property Graph di Atas Mesin Relasional

Banyak domain data modern—seperti jejaring interaksi sosial, pemodelan jalur rantai pasok logistik maritim antar pulau, pelacakan transaksi mencurigakan pencucian uang (*anti-money laundering*), dan graf pengetahuan ontologi—memiliki entitas yang saling terhubung dalam relasi banyak-ke-banyak (*many-to-many*) yang sangat padat.

Mengeksekusi penelusuran hubungan multi-derajat pada basis data relasional biasa memerlukan operasi `JOIN` rekursif berulang-kali yang akan menyebabkan ledakan komputasi dan memori. Untuk mengatasi kebutuhan ini, WidyaBase menyematkan **Mesin Graf Terpadu (*Property Graph Engine*)** native (`graph.rs`):
- **Model Property Graph Berarah (*Directed Property Graph*)**: Terdiri atas Simpul (*Nodes / Vertices*) dan Sisi (*Edges / Relationships*) yang memiliki label klasifikasi serta atribut kunci-nilai dinamis (*properties*).
- **Penelusuran Tanpa Kunci Indeks Menengah (*Index-Free Adjacency*)**: Setiap simpul memegang pointer langsung ke daftar sisi tetangga yang terhubung, memungkinkan traversal graf berjalan dengan kompleksitas waktu proporsional terhadap ukuran subgraf yang dijelajahi, bukan ukuran total basis data.
- **Algoritma Analisis Graf Built-In**: Dilengkapi algoritma pencarian jalur terpendek berbobot (*Dijkstra Shortest Path*), penelusuran tingkat (*Breadth-First Search* / BFS), penelusuran kedalaman (*Depth-First Search* / DFS), pemeringkatan pengaruh simpul (*PageRank*), dan pendeteksian komunitas (*Weakly Connected Components*).

```mermaid
flowchart LR
    subgraph JejaringGrafPerbankan["Graf Analisis Transaksi Finansial (graph.rs)"]
        N1["Rekening A<br>[Nasabah: Budi]"] -->|TRANSFER<br>{nominal: 50jt, tgl: '2026-03-01'}| N2["Rekening B<br>[Nasabah: CV Maju]"]
        N2 -->|INVESTASI<br>{nominal: 45jt}| N3["Rekening C<br>[Rekening Cangkang]"]
        N3 -->|TARIK_TUNAI<br>{lokasi: 'ATM Jayapura'}| N4["Pelaku Lapangan D"]
        N1 -.->|Pencarian Jalur Terpendek / Siklus Pencucian Uang| N4
    end
```

Karakteristik implementasi di WidyaBase:
1. **Dukungan Kueri Pencocokan Pola Hubungan**: Memungkinkan penulisan kueri alur relasi seperti `(A)-[:TRANSFER]->(B)-[:TRANSFER]->(C)` secara intuitif.
2. **Kepatuhan ACID Bersama**: Pembuatan node dan edge graf berada dalam satu transaksi ACID yang sama dengan mutasi tabel relasional dan dokumen JSONB.
3. **Optimasi Memori Berbobot Ringan**: ID simpul graf dikelola via penomoran atomik 64-bit (`NodeId`, `EdgeId`) yang diindeks dalam peta hash berkinerja tinggi.

### 21.2 Implementasi Kode Program Utuh

Berikut implementasi pembangunan topologi graf dan eksekusi algoritma penelusuran jalur terpendek Dijkstra pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Mesin Graf Relasional dan Penelusuran Jalur Terpendek WidyaBase
modul mesin_graf {
    impor ekosistem::widyabase::{
        WidyaBase, MesinGraf, SimpulGraf, SisiGraf,
        AlgoritmaGraf, HasilJalurTerpendek, NilaiProperti
    };

    fungsi demonstrasi_analisis_graf_logistik(mesin: &mut WidyaBase) -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [GRAPH] Memulai Inisialisasi Graf Jalur Tol Laut Logistik...");

        biar mut graf = mesin.ambil_mesin_graf_terpadu();

        // 1. Buat Simpul-Simpul Pelabuhan Utama Nusantara
        biar id_tanjung_priok = graf.buat_simpul("Pelabuhan", [("nama", "Tanjung Priok"), ("kota", "Jakarta")])?;
        biar id_tanjung_perak = graf.buat_simpul("Pelabuhan", [("nama", "Tanjung Perak"), ("kota", "Surabaya")])?;
        biar id_makassar      = graf.buat_simpul("Pelabuhan", [("nama", "Soekarno-Hatta"), ("kota", "Makassar")])?;
        biar id_sorong        = graf.buat_simpul("Pelabuhan", [("nama", "Pelabuhan Sorong"), ("kota", "Sorong")])?;
        biar id_jayapura      = graf.buat_simpul("Pelabuhan", [("nama", "Pelabuhan Jayapura"), ("kota", "Jayapura")])?;

        cetak(">>> [GRAPH] 5 simpul pelabuhan strategis berhasil dibuat.");

        // 2. Hubungkan Sisi-Sisi Rute Pelayaran dengan Bobot Waktu Tempuh (Jam)
        graf.buat_sisi(id_tanjung_priok, id_tanjung_perak, "RUTE_KAPAL", [("jarak_mil", 420.0), ("durasi_jam", 24.0)])?;
        graf.buat_sisi(id_tanjung_perak, id_makassar,      "RUTE_KAPAL", [("jarak_mil", 510.0), ("durasi_jam", 32.0)])?;
        graf.buat_sisi(id_makassar,      id_sorong,        "RUTE_KAPAL", [("jarak_mil", 750.0), ("durasi_jam", 48.0)])?;
        graf.buat_sisi(id_sorong,        id_jayapura,      "RUTE_KAPAL", [("jarak_mil", 620.0), ("durasi_jam", 38.0)])?;
        // Rute Langsung Ekspres Tanjung Perak ke Sorong
        graf.buat_sisi(id_tanjung_perak, id_sorong,        "RUTE_EKSPRES", [("jarak_mil", 1150.0), ("durasi_jam", 65.0)])?;

        cetak(">>> [GRAPH] Topologi rute pelayaran maritim selesai dipetakan.");

        // 3. Eksekusi Algoritma Dijkstra Shortest Path dari Jakarta ke Jayapura
        cetak(">>> [DIJKSTRA] Menghitung rute tercepat dari Tanjung Priok ke Jayapura...");
        biar hasil_jalur: HasilJalurTerpendek = graf.jalur_terpendek_dijkstra(
            id_tanjung_priok,
            id_jayapura,
            "durasi_jam" // Atribut yang dijadikan bobot optimasi
        )?;

        cetak(">>> [HASIL RUTE TERCEPAT]:");
        cetak("  - Total Durasi Perjalanan : " + teks(hasil_jalur.total_bobot) + " Jam");
        cetak("  - Rangkaian Pelabuhan Singgah:");
        untuk id_simpul di hasil_jalur.rute_simpul {
            biar simpul = graf.ambil_simpul(id_simpul).buka();
            cetak("    * Pelabuhan: " + simpul.ambil_teks("nama") + " (" + simpul.ambil_teks("kota") + ")");
        }

        // 4. Hitung Metrik Sentralitas Pengaruh Pelabuhan Menggunakan PageRank
        cetak(">>> [PAGERANK] Menghitung pelabuhan hub paling krusial di Nusantara...");
        biar skor_pagerank = graf.hitung_pagerank(0.85, 20)?; // damping 0.85, 20 iterasi
        untuk (id_node, skor) di skor_pagerank.ambil_top(3) {
            biar node = graf.ambil_simpul(id_node).buka();
            cetak("  ★ Hub Utama: " + node.ambil_teks("nama") + " | Skor Hub: " + teks(skor));
        }

        cetak("==================================================================");
        kembalikan Ok(benar);
    }
}
```

### 21.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan algoritma penelusuran graf bawaan pada WidyaBase:

| Algoritma Graf | Kompleksitas Waktu | Kompleksitas Ruang | Kasus Penggunaan Optimal |
| :--- | :--- | :--- | :--- |
| **Dijkstra Shortest Path** | $O((|V| + |E|) \log |V|)$ | $O(|V|)$ Antrean Prioritas | Rute navigasi logistik tercepat/termurah |
| **Breadth-First Search (BFS)**| $O(|V| + |E|)$ | $O(|V|)$ Antrean FIFO | Deteksi derajat keterhubungan teman ($N$-hop) |
| **Depth-First Search (DFS)**| $O(|V| + |E|)$ | $O(|V|)$ Call Stack / LIFO | Penelusuran pohon silsilah & deteksi siklus |
| **PageRank Centrality** | $O(K \cdot (|V| + |E|))$ | $O(|V|)$ Vektor Status | Pemeringkatan simpul paling berpengaruh/hub |
| **Weakly Connected Components**| $O(|V| + |E|)$ | $O(|V|)$ Disjoint Set | Deteksi kluster komunitas/sindikat kejahatan |

### 21.4 Latihan Soal Esai

1. Mengapa kueri penelusuran hubungan multi-derajat (*deep graph traversal*) pada model Property Graph memiliki performa berkali-kali lipat lebih cepat daripada pendekatan *Multi-Table Relational JOIN* biasa?
2. Terangkan prinsip kerja algoritma *Dijkstra Shortest Path* yang memanfaatkan struktur data *Binary Heap* dalam menentukan rute logistik optimal pada modul `graph.rs`!
3. Bagaimana algoritma *PageRank* dapat dimanfaatkan dalam sistem perbankan untuk mengidentifikasi rekening penampung utama dalam pola transaksi sirkuler mencurigakan?

### 21.5 Soal Pilihan Ganda

1. Komponen struktural dasar dalam model Property Graph yang merepresentasikan entitas objek dinamakan...
   A. Node (Simpul / Vertex)
   B. Sisi (Edge)
   C. Frame Buffer
   D. Root Page

2. Komponen dalam Property Graph yang merepresentasikan hubungan berarah antar dua entitas objek dinamakan...
   A. Edge (Sisi / Hubungan)
   B. Node
   C. Bucket
   D. Slotted

3. Karakteristik arsitektur di mana setiap simpul menyimpan referensi penunjuk memori langsung ke sisi dan tetangga terdekatnya disebut...
   A. Index-Free Adjacency
   B. Full Table Scan
   C. Hash Collision
   D. External Sorting

4. Algoritma penelusuran graf yang mencari jalur dengan total bobot akumulasi terkecil antara dua simpul dinamakan...
   A. Algoritma Dijkstra
   B. Algoritma K-Means
   C. Algoritma Bubble Sort
   D. Algoritma Merkle Tree

5. Algoritma penelusuran graf yang menjelajahi seluruh tetangga terdekat pada tingkat kedalaman yang sama sebelum melangkah ke tingkat berikutnya dinamakan...
   A. Breadth-First Search (BFS)
   B. Depth-First Search (DFS)
   C. Random Walk
   D. Binary Search

6. Algoritma iteratif yang digunakan untuk mengukur tingkat sentralitas atau kepentingan suatu simpul berdasarkan banyaknya simpul lain yang menunjuk ke dirinya adalah...
   A. PageRank
   B. QuickSort
   C. Stemmer Nazief
   D. Gorilla Codec

7. Nilai faktor redaman (*damping factor*) standar yang umum digunakan dalam kalkulasi PageRank adalah...
   A. 0.85
   B. 0.00
   C. 10.0
   D. -1.0

8. Algoritma yang digunakan untuk mengidentifikasi kelompok-kelompok simpul yang saling terhubung dalam graf berarah tanpa memedulikan arah sisi disebut...
   A. Weakly Connected Components (WCC)
   B. Strongly Directed Trees
   C. Linear Regression
   D. Huffman Coding

9. Keuntungan integrasi mesin graf di dalam WidyaBase dibandingkan menggunakan Graph Database eksternal terpisah adalah...
   A. Konsistensi transaksional ACID terjaga utuh antara simpul graf dan tabel relasional tanpa sinkronisasi ETL eksternal
   B. Graf tidak menggunakan memori komputer
   C. Menghilangkan kabel LAN
   D. Gambar otomatis berubah menjadi grafik 3D

10. Kueri pencarian siklus (*Cycle Detection*) pada graf perbankan sangat vital untuk mendeteksi fenomena...
    A. Pencucian uang dan transaksi sirkuler ilegal yang sengaja memutarbalikkan dana untuk mengaburkan asal-usul uang
    B. Jumlah nasabah yang berulang tahun
    C. Kerusakan mesin cetak buku tabungan
    D. Jam istirahat teller bank
