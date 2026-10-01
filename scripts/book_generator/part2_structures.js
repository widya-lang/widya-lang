module.exports = {
  content: `
<!-- BAB 5 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">5</span> Koleksi Data: Deret Dinamis & Kamus Asosiatif</h1>

<h2>5.1 Deret Dinamis (Dynamic Arrays): Alokasi & Operasi CRUD</h2>
<p>
Deret dalam Widya-Lang adalah struktur data berurutan dinamis yang ukurannya dapat membesar atau mengecil secara otomatis pada saat program berjalan. Elemen deret disimpan dalam blok memori kontigu dengan pengindeksan berbasis 0 (<em>zero-based indexing</em>), memberikan akses acak berkecepatan $O(1)$ instan:
</p>

<pre><code><span class="com">// Inisialisasi Deret Dinamis</span>
<span class="kw">misal</span> daftar_antrean = [<span class="str">"Nasabah-A01"</span>, <span class="str">"Nasabah-A02"</span>];

<span class="com">// 1. CREATE: Menambahkan Elemen ke Ekor Deret</span>
<span class="fn">tambah</span>(daftar_antrean, <span class="str">"Nasabah-A03"</span>);
<span class="fn">tambah</span>(daftar_antrean, <span class="str">"Nasabah-A04"</span>);
<span class="fn">cetak</span>(<span class="str">"Jumlah Antrean Aktif:"</span>, <span class="fn">panjang</span>(daftar_antrean)); <span class="com">// 4</span>

<span class="com">// 2. READ: Mengakses Elemen Berdasarkan Posisi Indeks</span>
<span class="fn">cetak</span>(<span class="str">"Panggilan Pertama   :"</span>, daftar_antrean[<span class="num">0</span>]); <span class="com">// Nasabah-A01</span>
<span class="fn">cetak</span>(<span class="str">"Panggilan Terakhir  :"</span>, daftar_antrean[<span class="fn">panjang</span>(daftar_antrean) - <span class="num">1</span>]);

<span class="com">// 3. UPDATE: Memperbarui Nilai Elemen Tertentu</span>
daftar_antrean[<span class="num">1</span>] = <span class="str">"Nasabah-VIP-99"</span>;
<span class="fn">cetak</span>(<span class="str">"Antrean Setelah VIP :"</span>, daftar_antrean);

<span class="com">// 4. SLICE: Memotong Irisan Deret [indeks_mulai, panjang_elemen]</span>
<span class="kw">misal</span> kloter_pagi = <span class="fn">iris</span>(daftar_antrean, <span class="num">0</span>, <span class="num">2</span>);
<span class="fn">cetak</span>(<span class="str">"Kloter Prioritas Pagi:"</span>, kloter_pagi);
</code></pre>

<h2>5.2 Kamus Asosiatif (Hash Maps / Dictionaries)</h2>
<p>
Kamus adalah pasangan kunci-nilai (<em>key-value pairs</em>) yang diimplementasikan menggunakan algoritma tabel cincang (<em>hash table</em>). Operasi pencarian, penyisipan, dan pembaruan data pada Kamus berlangsung dalam kompleksitas waktu rata-rata $O(1)$, menjadikannya pilihan ideal untuk pemodelan data domain dan konfigurasi sistem:
</p>

<pre><code><span class="kw">misal</span> profil_merchant = {
    <span class="str">"id_toko"</span>: <span class="str">"MCH-88210"</span>,
    <span class="str">"nama_usaha"</span>: <span class="str">"Kedai Kopi Nusantara Berdaulat"</span>,
    <span class="str">"kota"</span>: <span class="str">"Bandung"</span>,
    <span class="str">"kategori_mcc"</span>: <span class="num">5812</span>,
    <span class="str">"status_qris"</span>: <span class="kw">benar</span>,
    <span class="str">"total_saldo"</span>: <span class="num">18450000</span>
};

<span class="com">// Mengambil data dengan kunci teks</span>
<span class="fn">cetak</span>(<span class="str">"Nama Badan Usaha :"</span>, profil_merchant[<span class="str">"nama_usaha"</span>]);
<span class="fn">cetak</span>(<span class="str">"Status QRIS Aktif:"</span>, profil_merchant[<span class="str">"status_qris"</span>]);

<span class="com">// Memperbarui saldo setelah transaksi</span>
profil_merchant[<span class="str">"total_saldo"</span>] = profil_merchant[<span class="str">"total_saldo"</span>] + <span class="num">35000</span>;
<span class="fn">cetak</span>(<span class="str">"Saldo Rekening Baru: Rp"</span>, profil_merchant[<span class="str">"total_saldo"</span>]);
</code></pre>

<h2>5.3 Struktur Data Bersarang & Serialisasi JSON</h2>
<p>
Widya-Lang menyediakan dukungan bawaan untuk serialisasi dan deserialisasi format pertukaran data JSON (JavaScript Object Notation) melalui fungsi <code>urai_json(teks)</code> dan <code>ke_json(objek)</code>:
</p>

<pre><code><span class="com">// Teks payload JSON yang diterima dari jaringan API</span>
<span class="kw">misal</span> respons_api_mentah = <span class="str">'{
    "kode_status": 200,
    "pesan": "Transaksi Berhasil",
    "data": {
        "id_ref": "TRX-2026-9901",
        "nominal": 75000,
        "metode": "QRIS_DOMPET_KILAT"
    }
}'</span>;

<span class="com">// Mengurai teks JSON menjadi Kamus Widya-Lang</span>
<span class="kw">misal</span> payload_terurai = <span class="fn">urai_json</span>(respons_api_mentah);
<span class="fn">cetak</span>(<span class="str">"Kode Respons Status:"</span>, payload_terurai[<span class="str">"kode_status"</span>]);

<span class="kw">misal</span> detail = payload_terurai[<span class="str">"data"</span>];
<span class="fn">cetak</span>(<span class="str">"Nomor Referensi     :"</span>, detail[<span class="str">"id_ref"</span>]);
<span class="fn">cetak</span>(<span class="str">"Nominal Terbayar    : Rp"</span>, detail[<span class="str">"nominal"</span>]);

<span class="com">// Mengonversi kembali Kamus menjadi teks JSON padat</span>
<span class="kw">misal</span> teks_json_keluaran = <span class="fn">ke_json</span>(payload_terurai);
<span class="fn">cetak</span>(<span class="str">"Hasil Serialisasi JSON:"</span>, teks_json_keluaran);
</code></pre>

<h2>5.4 Studi Kasus Lengkap: Sistem Manajemen Inventaris Multi-Gudang</h2>
<pre><code><span class="fn">fungsi</span> <span class="fn">kelola_inventaris</span>() {
    <span class="kw">misal</span> gudang = {
        <span class="str">"GD-JKT"</span>: [
            { <span class="str">"sku"</span>: <span class="str">"LAP-01"</span>, <span class="str">"nama"</span>: <span class="str">"Laptop Merah Putih"</span>, <span class="str">"stok"</span>: <span class="num">25</span>, <span class="str">"harga"</span>: <span class="num">8500000</span> },
            { <span class="str">"sku"</span>: <span class="str">"MOU-02"</span>, <span class="str">"nama"</span>: <span class="str">"Mouse Nirkabel"</span>, <span class="str">"stok"</span>: <span class="num">120</span>, <span class="str">"harga"</span>: <span class="num">150000</span> }
        ],
        <span class="str">"GD-SBY"</span>: [
            { <span class="str">"sku"</span>: <span class="str">"LAP-01"</span>, <span class="str">"nama"</span>: <span class="str">"Laptop Merah Putih"</span>, <span class="str">"stok"</span>: <span class="num">10</span>, <span class="str">"harga"</span>: <span class="num">8500000</span> },
            { <span class="str">"sku"</span>: <span class="str">"KEY-03"</span>, <span class="str">"nama"</span>: <span class="str">"Papan Ketik Mekanikal"</span>, <span class="str">"stok"</span>: <span class="num">45</span>, <span class="str">"harga"</span>: <span class="num">450000</span> }
        ]
    };

    <span class="com">// Hitung valuasi total aset di seluruh gudang nasional</span>
    <span class="kw">misal</span> total_valuasi = <span class="num">0</span>;
    <span class="kw">untuk</span> kode_gudang <span class="kw">dalam</span> [<span class="str">"GD-JKT"</span>, <span class="str">"GD-SBY"</span>] {
        <span class="kw">misal</span> daftar_barang = gudang[kode_gudang];
        <span class="kw">untuk</span> barang <span class="kw">dalam</span> daftar_barang {
            <span class="kw">misal</span> nilai_stok = barang[<span class="str">"stok"</span>] * barang[<span class="str">"harga"</span>];
            total_valuasi = total_valuasi + nilai_stok;
            <span class="fn">cetak</span>(<span class="str">"["</span> + kode_gudang + <span class="str">"] "</span> + barang[<span class="str">"nama"</span>] + <span class="str">" | Valuasi: Rp"</span>, nilai_stok);
        }
    }

    <span class="fn">cetak</span>(<span class="str">"=================================================="</span>);
    <span class="fn">cetak</span>(<span class="str">"TOTAL VALUASI ASET INVENTARIS NASIONAL: Rp"</span>, total_valuasi);
}

<span class="fn">kelola_inventaris</span>();
</code></pre>

<!-- BAB 6 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">6</span> Pemrograman Berorientasi Entitas: `struktur` & `sifat`</h1>

<h2>6.1 Filosofi Komposisi Entitas Menggantikan Pewarisan Kelas</h2>
<p>
Bahasa pemrograman generasi terdahulu sering kali terjebak dalam perangkap <em>fragile base class problem</em> akibat pewarisan bertingkat yang terlalu dalam (<em>deep class inheritance</em>). Widya-Lang memutus tradisi tersebut dengan mengadopsi model komposisi entitas modern yang bersih:
</p>
<ol>
  <li><strong><code>struktur</code> (Structs)</strong>: Digunakan untuk memodelkan struktur data dan keadaan (<em>state</em>).</li>
  <li><strong><code>sifat</code> (Traits / Interfaces)</strong>: Digunakan untuk mendeklarasikan kontrak perilaku (<em>behavioral contracts</em>) yang dapat diimplementasikan oleh entitas manapun tanpa relasi hierarki kaku.</li>
</ol>

<h2>6.2 Deklarasi `struktur`, Properti, & Pola Pabrik Objek (Factory Pattern)</h2>
<pre><code><span class="com">// Definisi entitas data Nasabah</span>
<span class="kw">struktur</span> <span class="typ">NasabahBank</span> {
    nomor_rekening,
    nama_lengkap,
    nomor_ktp,
    saldo,
    status_aktif
}

<span class="com">// Pola Pabrik Objek untuk menjamin integritas inisialisasi</span>
<span class="fn">fungsi</span> <span class="fn">buat_nasabah_baru</span>(norek, nama, ktp, setoran_awal) {
    <span class="kw">jika</span> setoran_awal < <span class="num">50000</span> {
        <span class="fn">cetak</span>(<span class="str">"Peringatan: Setoran awal minimal pembukaan akun adalah Rp 50.000!"</span>);
    }

    <span class="kw">kembalikan</span> <span class="typ">NasabahBank</span> {
        nomor_rekening: norek,
        nama_lengkap: nama,
        nomor_ktp: ktp,
        saldo: setoran_awal,
        status_aktif: <span class="kw">benar</span>
    };
}

<span class="kw">misal</span> nasabah1 = <span class="fn">buat_nasabah_baru</span>(<span class="str">"008-2026-1192"</span>, <span class="str">"Siti Rahmawati"</span>, <span class="str">"3273010190001"</span>, <span class="num">500000</span>);
<span class="fn">cetak</span>(<span class="str">"Nasabah Terdaftar:"</span>, nasabah1.nama_lengkap);
<span class="fn">cetak</span>(<span class="str">"Nomor Rekening   :"</span>, nasabah1.nomor_rekening);
<span class="fn">cetak</span>(<span class="str">"Saldo Tersimpan  : Rp"</span>, nasabah1.saldo);
</code></pre>

<h2>6.3 Kontrak Polimorfisme Menggunakan `sifat` (Traits)</h2>
<p>
Kata kunci <code>sifat</code> mendefinisikan himpunan fungsi yang harus dipenuhi oleh suatu entitas. Ini memungkinkan pemrosesan transaksi yang agnostik terhadap metode pembayaran:
</p>

<pre><code><span class="com">// Mendefinisikan kontrak perilaku pembayaran</span>
<span class="kw">sifat</span> <span class="typ">KanalPembayaran</span> {
    <span class="fn">fungsi</span> <span class="fn">dapatkan_nama_kanal</span>();
    <span class="fn">fungsi</span> <span class="fn">hitung_biaya_layanan</span>(nominal);
    <span class="fn">fungsi</span> <span class="fn">proses_potong_dana</span>(nominal);
}

<span class="kw">struktur</span> <span class="typ">MetodeQRIS</span> { id_merchant, biaya_mdr_persen }
<span class="kw">struktur</span> <span class="typ">MetodeVirtualAccount</span> { bank_tujuan, biaya_admin_tetap }

<span class="com">// Fungsi pemroses transaksi polimorfik</span>
<span class="fn">fungsi</span> <span class="fn">eksekusi_pembayaran</span>(metode, nominal_belanja) {
    <span class="kw">misal</span> nama_kanal = metode.<span class="fn">dapatkan_nama_kanal</span>();
    <span class="kw">misal</span> biaya_layanan = metode.<span class="fn">hitung_biaya_layanan</span>(nominal_belanja);
    <span class="kw">misal</span> total_bayar = nominal_belanja + biaya_layanan;

    <span class="fn">cetak</span>(<span class="str">"Memproses Kanal Pembayaran:"</span>, nama_kanal);
    <span class="fn">cetak</span>(<span class="str">"Tagihan Belanja  : Rp"</span>, nominal_belanja);
    <span class="fn">cetak</span>(<span class="str">"Biaya Transaksi  : Rp"</span>, biaya_layanan);
    <span class="fn">cetak</span>(<span class="str">"Total Tagihan    : Rp"</span>, total_bayar);

    <span class="kw">misal</span> berhasil = metode.<span class="fn">proses_potong_dana</span>(total_bayar);
    <span class="kw">kembalikan</span> berhasil;
}
</code></pre>

<!-- BAB 7 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">7</span> Penanganan Galat & Ketahanan Crash-Resilient</h1>

<h2>7.1 Filosofi Zero-Panic & Penanganan Deterministik</h2>
<p>
Dalam arsitektur perangkat lunak skala misi-kritis (<em>mission-critical</em>), penghentian program secara mendadak akibat galat yang tak tertangani (<em>unhandled panic crash</em>) adalah bentuk kegagalan sistem paling fatal. Widya-Lang menerapkan penanganan galat deterministik berbasis blok <code>coba</code> - <code>tangkap</code> - <code>akhirnya</code> yang menjamin sumber daya tertutup aman dan program terus beroperasi:
</p>

<div class="diagram-box">
  [ Eksekusi Perintah Kritis ] ─── Normal ───► [ Selesai Sukses ] ───► [ Blok 'akhirnya' ]
                │                                                              ▲
              Galat Terjadi                                                    │
                ▼                                                              │
  [ Intersepsi Blok 'tangkap' ] ─── Pemulihan & Log ───────────────────────────┘
</div>

<h2>7.2 Penanganan Transaksi Finansial dengan `coba`, `tangkap`, `lempar`</h2>
<pre><code><span class="fn">fungsi</span> <span class="fn">transfer_dana_antar_rekening</span>(rekening_asal, rekening_tujuan, nominal) {
    <span class="kw">coba</span> {
        <span class="kw">jika</span> nominal <= <span class="num">0</span> {
            <span class="kw">lempar</span> <span class="str">"GALAT_NOMINAL_INVALID: Nominal transfer harus bernilai positif!"</span>;
        }
        
        <span class="kw">jika</span> rekening_asal.saldo < nominal {
            <span class="kw">lempar</span> <span class="str">"GALAT_SALDO_INSUFISIEN: Saldo sumber tidak mencukupi untuk transfer!"</span>;
        }

        <span class="kw">jika</span> <span class="kw">bukan</span> rekening_tujuan.status_aktif {
            <span class="kw">lempar</span> <span class="str">"GALAT_REKENING_DIBLOKIR: Rekening tujuan berstatus non-aktif!"</span>;
        }

        <span class="com">// Mutasi saldo atomik</span>
        rekening_asal.saldo = rekening_asal.saldo - nominal;
        rekening_tujuan.saldo = rekening_tujuan.saldo + nominal;

        <span class="fn">cetak</span>(<span class="str">"✅ TRANSFER SUKSES SEBESAR Rp"</span>, nominal);
        <span class="kw">kembalikan</span> <span class="kw">benar</span>;
    } <span class="kw">tangkap</span> (pesan_galat) {
        <span class="com">// Blok tangkap mengisolasi galat sehingga thread program tidak runtuh</span>
        <span class="fn">cetak</span>(<span class="str">"❌ GAGAL MEMPROSES TRANSAKSI:"</span>, pesan_galat);
        <span class="kw">kembalikan</span> <span class="kw">salah</span>;
    } <span class="kw">akhirnya</span> {
        <span class="com">// Blok ini selalu dieksekusi baik operasi berhasil maupun gagal</span>
        <span class="fn">cetak</span>(<span class="str">"[AUDIT] Sesi mutasi rekening ditutup secara aman pada catatan jurnal."</span>);
    }
}
</code></pre>

<h2>7.3 Pola Ketahanan Resilient: Retry dengan Exponential Backoff</h2>
<p>
Ketika berkomunikasi dengan layanan jaringan eksternal (seperti payment gateway atau server awan), kegagalan temporer sering kali terjadi akibat fluktuasi koneksi. Pola pengulangan otomatis (<em>retry with exponential backoff</em>) mencegah kegagalan instan:
</p>

<pre><code><span class="fn">fungsi</span> <span class="fn">hubungi_server_dengan_toleransi</span>(url, batas_maksimal_percobaan) {
    <span class="kw">misal</span> percobaan = <span class="num">1</span>;
    <span class="kw">misal</span> jeda_detik = <span class="num">1</span>;

    <span class="kw">selama</span> percobaan <= batas_maksimal_percobaan {
        <span class="kw">coba</span> {
            <span class="fn">cetak</span>(<span class="str">"Menghubungi endpoint"</span>, url, <span class="str">"(Percobaan ke-"</span>, percobaan, <span class="str">")..."</span>);
            <span class="com">// Simulasi pemanggilan jaringan</span>
            <span class="kw">jika</span> percobaan < <span class="num">3</span> {
                <span class="kw">lempar</span> <span class="str">"Timeout Jaringan (Kode 504)"</span>;
            }
            <span class="fn">cetak</span>(<span class="str">"✨ Koneksi Jaringan Berhasil Terhubung!"</span>);
            <span class="kw">kembalikan</span> <span class="kw">benar</span>;
        } <span class="kw">tangkap</span> (galat) {
            <span class="fn">cetak</span>(<span class="str">"Peringatan: Gagal terhubung ("</span>, galat, <span class="str">"). Menunggu"</span>, jeda_detik, <span class="str">"detik..."</span>);
            percobaan = percobaan + <span class="num">1</span>;
            jeda_detik = jeda_detik * <span class="num">2</span>; <span class="com">// Eksponensial: 1s, 2s, 4s...</span>
        }
    }

    <span class="fn">cetak</span>(<span class="str">"❌ Seluruh upaya percobaan gagal. Mengaktifkan mode fail-safe lokal."</span>);
    <span class="kw">kembalikan</span> <span class="kw">salah</span>;
}
</code></pre>

<!-- BAB 8 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">8</span> Modularitas, Sistem Impor & Arsitektur Proyek</h1>

<h2>8.1 Desain Arsitektur Piramida 4-Lapisan (Clean Architecture)</h2>
<p>
Untuk membangun sistem perangkat lunak yang dapat dirawat (<em>maintainable</em>) selama puluhan tahun oleh ratusan insinyur perangkat lunak, Widya-Lang menerapkan arsitektur piramida modular bersih:
</p>

<div class="diagram-box">
  sistem_perbankan/
  ├── widya.toml                    # Manifes konfigurasi proyek & dependensi
  ├── aplikasi_utama.wya            # Titik masuk eksekusi (Entry point)
  └── modul/
      ├── domain/                   # Lapisan 1: Entitas Bisnis Inti (Struktur & Sifat)
      │   └── rekening.wya
      ├── repositori/               # Lapisan 2: Akses Data (WidyaBase Engine / File Storage)
      │   └── repositori_rekening.wya
      ├── layanan/                  # Lapisan 3: Logika Bisnis & Validasi Aturan
      │   └── layanan_transfer.wya
      └── api/                      # Lapisan 4: Pengendali Antarmuka (HTTP REST / Desktop UI)
          └── pengendali_http.wya
</div>

<h2>8.2 Kata Kunci `impor` & Resolusi Modul Relatif</h2>
<p>
Perintah <code>impor</code> memuat simbol dan fungsi dari berkas lain ke dalam ruang lingkup saat ini secara deterministik tanpa pencemaran variabel global:
</p>

<pre><code><span class="com">// Berkas: modul/layanan/layanan_transfer.wya</span>
<span class="kw">impor</span> <span class="str">"../domain/rekening.wya"</span>;
<span class="kw">impor</span> <span class="str">"../repositori/repositori_rekening.wya"</span>;

<span class="fn">fungsi</span> <span class="fn">proses_layanan_transfer</span>(id_asal, id_tujuan, jumlah) {
    <span class="kw">misal</span> akun_pengirim = <span class="fn">cari_rekening_by_id</span>(id_asal);
    <span class="kw">misal</span> akun_penerima = <span class="fn">cari_rekening_by_id</span>(id_tujuan);

    <span class="kw">jika</span> akun_pengirim.saldo < jumlah {
        <span class="kw">kembalikan</span> { <span class="str">"sukses"</span>: <span class="kw">salah</span>, <span class="str">"alasan"</span>: <span class="str">"Saldo tidak mencukupi"</span> };
    }

    <span class="fn">simpan_mutasi_jurnal</span>(id_asal, id_tujuan, jumlah);
    <span class="kw">kembalikan</span> { <span class="str">"sukses"</span>: <span class="kw">benar</span>, <span class="str">"waktu"</span>: <span class="fn">waktu_saat_ini</span>() };
}
</code></pre>

<h2>8.3 Manifes Proyek: `widya.toml`</h2>
<p>
Setiap proyek berstandar enterprise menyertakan berkas <code>widya.toml</code> pada direktori akar yang mendefinisikan metadata aplikasi, versi biner, dan paket modul resmi yang digunakan:
</p>

<pre><code>[paket]
nama = "widya-core-banking"
versi = "1.0.0"
penulis = "Tim Rekayasa Perbankan Berdaulat &lt;dev@bank.id&gt;"
edisi = "2026"
lisensi = "MIT"

[dependensi]
widyabase = { versi = "1.0.0", fitur = ["wal", "cte", "vektor"] }
kriptografi = { versi = "1.0.0", fitur = ["aes-256", "ed25519"] }
desktop_gui = { versi = "1.0.0" }

[pembangunan]
optimasi = "kecepatan_maksimal"
pemeriksaan_tipe = "ketat"
</code></pre>
`
};
