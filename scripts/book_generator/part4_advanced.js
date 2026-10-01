module.exports = {
  content: `
<!-- BAB 12 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">12</span> Pemrograman Konkuren, Paralelisme & Jaringan Mikroservis</h1>

<h2>12.1 Green Threads & Task Worker: Model Konkurensi Ringan</h2>
<p>
Dalam arsitektur komputasi modern, pembuatan thread sistem operasi (<em>OS native thread</em>) memerlukan biaya alokasi memori yang cukup besar (umumnya 1–2 MB per tumpukan thread). Widya-Lang menerapkan model konkurensi <strong>Green Threads (M:N Task Scheduler)</strong> di mana ribuan tugas komputasi independen dijadwalkan secara kooperatif di atas sejumlah kecil thread fisik prosesor. Dengan jejak tumpukan awal hanya 4 KB, sistem dapat menjalankan 100.000 tugas konkuren secara simultan tanpa menghabiskan kapasitas RAM server.
</p>

<div class="diagram-box">
  ┌────────────────────────────────────────────────────────┐
  │  Tugas Konkuren Widya (Green Threads: T1, T2, ... Tn)  │
  └───────────────────────────┬────────────────────────────┘
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │       Penjadwal Tugas Tingkat Pengguna (Work-Stealing) │
  └───────────────────────────┬────────────────────────────┘
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │ Thread Prosesor Fisik OS (Core 1, Core 2, Core 3...)   │
  └────────────────────────────────────────────────────────┘
</div>

<h2>12.2 Saluran Komunikasi (Channels) Antar-Tugas</h2>
<p>
Mengikuti filosofi komputasi modern: <em>"Jangan berkomunikasi dengan berbagi memori; melainkan bagilah memori dengan berkomunikasi,"</em> Widya-Lang menyediakan saluran (<em>channels</em>) yang aman untuk bertukar pesan antar-tugas konkuren tanpa risiko korupsi data:
</p>

<pre><code><span class="kw">impor</span> <span class="str">"./modul/konkurensi.wya"</span>;

<span class="fn">fungsi</span> <span class="fn">simulasi_antrean_pesanan</span>() {
    <span class="com">// Buat saluran pesan dengan buffer kapasitas 10 elemen</span>
    <span class="kw">misal</span> kanal_pesanan = <span class="fn">buat_saluran</span>(<span class="num">10</span>);

    <span class="com">// Tugas Produsen (Penghasil Pesanan Belanja)</span>
    <span class="fn">jalankan_tugas</span>(<span class="fn">fungsi</span>() {
        <span class="kw">untuk</span> i <span class="kw">dalam</span> <span class="num">1</span>..<span class="num">5</span> {
            <span class="kw">misal</span> pesanan = { <span class="str">"id"</span>: <span class="str">"ORD-2026-"</span> + <span class="fn">ke_teks</span>(i), <span class="str">"item"</span>: <span class="str">"Kopi Gayo Premium"</span> };
            <span class="fn">kirim_ke_saluran</span>(kanal_pesanan, pesanan);
            <span class="fn">cetak</span>(<span class="str">"[PRODUSEN] Menerbitkan Pesanan:"</span>, pesanan[<span class="str">"id"</span>]);
        }
        <span class="fn">tutup_saluran</span>(kanal_pesanan);
    });

    <span class="com">// Tugas Konsumen (Pemroses Pembayaran & Pengemasan)</span>
    <span class="fn">jalankan_tugas</span>(<span class="fn">fungsi</span>() {
        <span class="kw">selama</span> <span class="kw">benar</span> {
            <span class="kw">misal</span> pesan = <span class="fn">terima_dari_saluran</span>(kanal_pesanan);
            <span class="kw">jika</span> pesan == <span class="kw">nihil</span> {
                <span class="fn">cetak</span>(<span class="str">"[KONSUMEN] Seluruh pesanan telah selesai diproses!"</span>);
                <span class="kw">henti</span>;
            }
            <span class="fn">cetak</span>(<span class="str">"[KONSUMEN] Mengemas & Memproses:"</span>, pesan[<span class="str">"id"</span>]);
        }
    });
}
</code></pre>

<h2>12.3 Membangun Server HTTP RESTful Mikroservis</h2>
<p>
Widya-Lang menyertakan server HTTP berlatensi rendah di dalam modul <code>modul/http_server.wya</code> yang mampu melayani ribuan permintaan per detik (<em>Requests Per Second / RPS</em>) dengan pipeline middleware terstruktur:
</p>

<pre><code><span class="kw">impor</span> <span class="str">"./modul/http_server.wya"</span>;
<span class="kw">impor</span> <span class="str">"./modul/widyabase.wya"</span>;

<span class="fn">fungsi</span> <span class="fn">mulai_layanan_api</span>() {
    <span class="kw">misal</span> server = <span class="fn">BuatServerHTTP</span>(<span class="str">"0.0.0.0"</span>, <span class="num">8080</span>);
    <span class="kw">misal</span> db = <span class="fn">BukaWidyaBase</span>(<span class="str">"produk_store.wdb"</span>);

    <span class="com">// Rute 1: GET /api/v1/produk - Mengambil Katalog Barang</span>
    server.<span class="fn">rute_get</span>(<span class="str">"/api/v1/produk"</span>, <span class="fn">fungsi</span>(permintaan, respon) {
        <span class="kw">misal</span> data_produk = db.<span class="fn">kueri</span>(<span class="str">"PILIH * DARI produk BATAS 50;"</span>);
        respon.<span class="fn">kirim_json</span>(<span class="num">200</span>, {
            <span class="str">"status"</span>: <span class="str">"sukses"</span>,
            <span class="str">"data"</span>: data_produk
        });
    });

    <span class="com">// Rute 2: POST /api/v1/transaksi - Memproses Pesanan Baru</span>
    server.<span class="fn">rute_post</span>(<span class="str">"/api/v1/transaksi"</span>, <span class="fn">fungsi</span>(permintaan, respon) {
        <span class="kw">misal</span> body = <span class="fn">urai_json</span>(permintaan.body_teks);
        <span class="kw">misal</span> id_baru = <span class="fn">buat_uuid</span>();

        db.<span class="fn">eksekusi</span>(<span class="str">"SISIP_KE pesanan (id, user, total) NILAI ('"</span> + id_baru + <span class="str">"', '"</span> + body[<span class="str">"user"</span>] + <span class="str">"', "</span> + <span class="fn">ke_teks</span>(body[<span class="str">"total"</span>]) + <span class="str">");"</span>);

        respon.<span class="fn">kirim_json</span>(<span class="num">201</span>, {
            <span class="str">"status"</span>: <span class="str">"terbuat"</span>,
            <span class="str">"id_transaksi"</span>: id_baru
        });
    });

    <span class="fn">cetak</span>(<span class="str">"🚀 Mikroservis API Widya-Lang Aktif di Port 8080!"</span>);
    server.<span class="fn">dengarkan</span>();
}
</code></pre>

<!-- BAB 13 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">13</span> Komputasi Sains, Vektor AI & Data Engineering</h1>

<h2>13.1 Analisis Data Kolumnar dengan Dataframe</h2>
<p>
Untuk pengolahan data terstruktur skala besar, modul <code>modul/dataframe_analitika.wya</code> menyediakan struktur data DataFrame kolumnar yang teroptimasi secara SIMD (<em>Single Instruction Multiple Data</em>) untuk komputasi agregasi statistik cepat:
</p>

<pre><code><span class="kw">impor</span> <span class="str">"./modul/dataframe_analitika.wya"</span>;

<span class="fn">fungsi</span> <span class="fn">analisis_penjualan_nasional</span>() {
    <span class="com">// Membaca berkas CSV transaksi skala ratusan ribu baris</span>
    <span class="kw">misal</span> df = <span class="fn">MuatDataframeCSV</span>(<span class="str">"data_transaksi_2026.csv"</span>);

    <span class="com">// 1. Saring transaksi bernilai di atas Rp 1.000.000</span>
    <span class="kw">misal</span> df_skala_besar = df.<span class="fn">saring_kolom</span>(<span class="str">"omzet"</span>, <span class="str">">="</span>, <span class="num">1000000</span>);

    <span class="com">// 2. Kelompokkan berdasarkan provinsi dan hitung agregasi omzet</span>
    <span class="kw">misal</span> ringkasan_provinsi = df_skala_besar
        .<span class="fn">kelompokkan</span>(<span class="str">"provinsi"</span>)
        .<span class="fn">agregasi</span>({
            <span class="str">"omzet"</span>: [<span class="str">"JUMLAH"</span>, <span class="str">"RERATA"</span>, <span class="str">"MAKSIMUM"</span>],
            <span class="str">"id_transaksi"</span>: [<span class="str">"HITUNG"</span>]
        });

    ringkasan_provinsi.<span class="fn">tampilkan_tabel</span>();
}
</code></pre>

<h2>13.2 Ruang Vektor & Indeks HNSW untuk AI RAG (Retrieval-Augmented Generation)</h2>
<p>
Model kecerdasan buatan modern merepresentasikan makna semantik teks dalam bentuk vektor berdimensi tinggi (<em>embedding vectors</em>). Untuk mencari dokumen yang paling relevan dengan pertanyaan pengguna dalam hitungan milidetik, Widya-Lang menyertakan pustaka grafis <strong>HNSW (Hierarchical Navigable Small World)</strong> yang mengukur kemiripan sudut kosinus (<em>Cosine Similarity</em>):
</p>

<pre><code><span class="kw">impor</span> <span class="str">"./modul/vektor_hnsw.wya"</span>;

<span class="fn">fungsi</span> <span class="fn">pencarian_semantik_dokumen</span>() {
    <span class="com">// Inisialisasi indeks vektor 4-dimensi</span>
    <span class="kw">misal</span> indeks = <span class="fn">IndeksVektorHNSW</span>(<span class="num">4</span>);

    <span class="com">// Mendaftarkan vektor representasi pengetahuan hukum dan regulasi</span>
    indeks.<span class="fn">tambah_vektor</span>(<span class="str">"PASAL-01"</span>, [<span class="num">0.88</span>, <span class="num">0.12</span>, <span class="num">0.02</span>, <span class="num">0.05</span>]); <span class="com">// Topik: Perlindungan Data Pribadi</span>
    indeks.<span class="fn">tambah_vektor</span>(<span class="str">"PASAL-02"</span>, [<span class="num">0.85</span>, <span class="num">0.18</span>, <span class="num">0.04</span>, <span class="num">0.08</span>]); <span class="com">// Topik: Keamanan Siber Nasional</span>
    indeks.<span class="fn">tambah_vektor</span>(<span class="str">"PASAL-03"</span>, [<span class="num">0.05</span>, <span class="num">0.02</span>, <span class="num">0.91</span>, <span class="num">0.82</span>]); <span class="com">// Topik: Pajak Perdagangan Ekspor</span>

    <span class="com">// Vektor pertanyaan pengguna mengenai privasi data</span>
    <span class="kw">misal</span> vektor_pertanyaan = [<span class="num">0.90</span>, <span class="num">0.10</span>, <span class="num">0.01</span>, <span class="num">0.03</span>];

    <span class="com">// Cari 2 dokumen terdekat berdasarkan Cosine Similarity</span>
    <span class="kw">misal</span> hasil_relevan = indeks.<span class="fn">cari_terdekat</span>(vektor_pertanyaan, <span class="num">2</span>);

    <span class="kw">untuk</span> doc <span class="kw">dalam</span> hasil_relevan {
        <span class="fn">cetak</span>(<span class="str">"Dokumen Terpilih :"</span>, doc.id);
        <span class="fn">cetak</span>(<span class="str">"Skor Kemiripan   :"</span>, doc.skor_kemiripan);
    }
}
</code></pre>

<!-- BAB 14 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">14</span> Kolaborasi Cerdas dengan AI Lokal (WidyaCoder)</h1>

<h2>14.1 Mengapa AI Lokal Penting untuk Kedaulatan Data?</h2>
<p>
Mengirimkan kode sumber sistem perbankan, data rekam medis pasien, atau rahasia manufaktur perusahaan ke server AI awan di luar negeri menyimpan risiko kebocoran data strategis yang melanggar Undang-Undang Perlindungan Data Pribadi (UU PDP No. 27/2022). Ekosistem Widya-Lang menghadirkan model <strong>WidyaCoder (4.7 GB GGUF fine-tuned)</strong> yang dapat dijalankan secara 100% lokal, offline, dan tanpa koneksi internet di atas perangkat keras GPU atau CPU Anda via <strong>Ollama</strong>.
</p>

<h2>14.2 Mengoperasikan WidyaCoder di Terminal Lokal</h2>
<pre><code><span class="com"># Menjalankan model AI lokal yang telah dilatih korpus Widya-Lang</span>
$ ollama run widya-coder:latest
&gt;&gt;&gt; Bagaimana cara membuat tabel di WidyaBase dengan kunci utama otomatis?

Kompiler WidyaBase menyediakan sintaksis DDL Dialek Nusantara:
BUAT_TABEL pengguna (
    id INT KUNCI_UTAMA,
    nama_lengkap TEKS,
    saldo INT
);
</code></pre>

<h2>14.3 Merancang Agen AI Otonom dengan Pola ReAct & Tool Calling</h2>
<p>
Dengan menghubungkan Widya-Lang ke Ollama API lokal pada <code>http://127.0.0.1:11434</code>, pengembang dapat membangun agen AI otonom yang mampu melakukan pemikiran logis (<em>Reasoning</em>) dan eksekusi tindakan nyata (<em>Acting / Tool Calling</em>) ke dalam basis data WidyaBase:
</p>

<pre><code><span class="kw">impor</span> <span class="str">"./modul/http_klien.wya"</span>;
<span class="kw">impor</span> <span class="str">"./modul/widyabase.wya"</span>;

<span class="fn">fungsi</span> <span class="fn">agen_layanan_pelanggan_ai</span>(pertanyaan_pembeli) {
    <span class="kw">misal</span> db = <span class="fn">BukaWidyaBase</span>(<span class="str">"toko_retail.wdb"</span>);

    <span class="com">// Definisi Fungsi/Tool yang Dapat Dipanggil oleh Agen AI</span>
    <span class="kw">misal</span> alat_cek_stok = <span class="fn">fungsi</span>(nama_barang) {
        <span class="kw">misal</span> baris = db.<span class="fn">kueri_satu</span>(<span class="str">"PILIH stok DARI produk DIMANA nama SEPERTI '%"</span> + nama_barang + <span class="str">"%';"</span>);
        <span class="kw">jika</span> baris == <span class="kw">nihil</span> { <span class="kw">kembalikan</span> <span class="str">"Barang tidak terdaftar di sistem inventaris."</span>; }
        <span class="kw">kembalikan</span> <span class="str">"Sisa stok gudang: "</span> + <span class="fn">ke_teks</span>(baris[<span class="str">"stok"</span>]) + <span class="str">" unit."</span>;
    };

    <span class="com">// Rekayasa Prompt Terstruktur ReAct</span>
    <span class="kw">misal</span> prompt_sistem = <span class="str">"Anda adalah asisten cerdas Widya-Lang. Jika pengguna bertanya tentang ketersediaan barang, panggil tool [CEK_STOK: nama_barang]."</span>;

    <span class="kw">misal</span> permintaan_ollama = {
        <span class="str">"model"</span>: <span class="str">"widya-coder:latest"</span>,
        <span class="str">"prompt"</span>: prompt_sistem + <span class="str">"\nPertanyaan: "</span> + pertanyaan_pembeli,
        <span class="str">"stream"</span>: <span class="kw">salah</span>
    };

    <span class="kw">misal</span> respon = <span class="fn">KirimPOST</span>(<span class="str">"http://127.0.0.1:11434/api/generate"</span>, permintaan_ollama);
    <span class="kw">misal</span> teks_ai = respon[<span class="str">"response"</span>];

    <span class="fn">cetak</span>(<span class="str">"Pikiran Agen AI:"</span>, teks_ai);
    <span class="kw">kembalikan</span> teks_ai;
}
</code></pre>
`
};
