module.exports = {
  content: `
<!-- BAB 9 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">9</span> Rekayasa Aplikasi Desktop GUI Modern</h1>

<h2>9.1 Arsitektur WidyaEngine Desktop: Event Loop & Message Dispatcher</h2>
<p>
Berbeda dengan ekosistem modern yang sering kali bergantung pada kerangka peramban web tebal (seperti Electron) yang menghabiskan memori ratusan megabita, framework <code>modul/desktop.wya</code> dirancang secara native di atas pustaka grafis sistem operasi berkecepatan tinggi. Dengan konsumsi RAM kurang dari 35 MB, aplikasi desktop Widya-Lang memiliki waktu startup instan (<em>sub-second startup</em>) dan responsivitas antarmuka yang sangat mulus.
</p>

<div class="diagram-box">
  ┌────────────────────────────────────────────────────────┐
  │                 Siklus Hidup Jendela GUI               │
  │  Inisialisasi Window ──► Pasang Komponen ──► Render UI │
  └───────────────────────────┬────────────────────────────┘
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │            Event-Driven Loop (Thread Utama)            │
  │  Peristiwa Masuk (Klik/Ketik/Timer) ──► Dispatcher     │
  │  Eksekusi Callback ──► Pembaruan DOM GUI ──► Re-Paint  │
  └────────────────────────────────────────────────────────┘
</div>

<h2>9.2 Komponen Antarmuka Grafis (UI Widgets)</h2>
<table>
  <thead>
    <tr>
      <th>Komponen GUI</th>
      <th>Metode Pembuatan</th>
      <th>Properti & Perilaku Interaktif</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>Jendela Utama (Window)</strong></td>
      <td><code>AplikasiDesktop(judul, lebar, tinggi)</code></td>
      <td>Wadah induk aplikasi dengan bilah judul, ikon, dan tombol kontrol standar OS.</td>
    </tr>
    <tr>
      <td><strong>Panel Kontainer</strong></td>
      <td><code>app.tambah_panel(id, x, y, w, h, bg_color)</code></td>
      <td>Wadah tata letak untuk mengelompokkan elemen formulir, bilah sisi, atau kartu statistik.</td>
    </tr>
    <tr>
      <td><strong>Label Teks</strong></td>
      <td><code>app.tambah_label(id, teks, x, y)</code></td>
      <td>Menampilkan teks statis, judul seksi, atau nilai indikator status.</td>
    </tr>
    <tr>
      <td><strong>Kotak Input Teks</strong></td>
      <td><code>app.tambah_input(id, placeholder, x, y)</code></td>
      <td>Bidang input ketikan keyboard untuk SKU barcode, nomor telepon, atau nominal.</td>
    </tr>
    <tr>
      <td><strong>Tombol Aksi (Button)</strong></td>
      <td><code>app.tambah_tombol(id, label, x, y, fungsi_klik)</code></td>
      <td>Memicu penanganan aksi ketika tombol diklik oleh pengguna.</td>
    </tr>
    <tr>
      <td><strong>Tabel Data (TableView)</strong></td>
      <td><code>app.tambah_tabel(id, kolom, x, y, w, h)</code></td>
      <td>Menampilkan data tabular transaksi berbaris dengan bilah gulir (scroll).</td>
    </tr>
    <tr>
      <td><strong>Kotak Dialog (Modal)</strong></td>
      <td><code>app.tampilkan_dialog(judul, pesan)</code></td>
      <td>Jendela pop-up peringatan konfirmasi atau pemberitahuan struk belanja.</td>
    </tr>
  </tbody>
</table>

<h2>9.3 Studi Kasus Lengkap: WidyaPOS - Aplikasi Kasir Retail UMKM</h2>
<p>
Berikut adalah implementasi penuh kode aplikasi kasir retail desktop Point-of-Sale (POS) yang terhubung langsung ke mesin basis data lokal WidyaBase:
</p>

<pre><code><span class="kw">impor</span> <span class="str">"./modul/desktop.wya"</span>;
<span class="kw">impor</span> <span class="str">"./modul/widyabase.wya"</span>;

<span class="fn">fungsi</span> <span class="fn">inisialisasi_kasir_pos</span>() {
    <span class="com">// 1. Hubungkan ke Database Master Barang Lokal</span>
    <span class="kw">misal</span> db = <span class="fn">BukaWidyaBase</span>(<span class="str">"kasir_toko.wdb"</span>);
    db.<span class="fn">eksekusi</span>(<span class="str">"
        BUAT_TABEL JIKA_TIDAK_ADA produk (
            sku TEKS KUNCI_UTAMA,
            nama TEKS,
            harga INT,
            stok INT
        );
    "</span>);

    <span class="com">// 2. Buat Jendela Aplikasi Desktop</span>
    <span class="kw">misal</span> app = <span class="fn">AplikasiDesktop</span>(<span class="str">"WidyaPOS - Kasir Retail Nusantara v1.0"</span>, <span class="num">920</span>, <span class="num">680</span>);

    <span class="com">// 3. Keadaan Internal Keranjang Belanja</span>
    <span class="kw">misal</span> keranjang_item = [];
    <span class="kw">misal</span> total_akumulasi = <span class="num">0</span>;

    <span class="com">// Header Judul Aplikasi</span>
    app.<span class="fn">tambah_label</span>(<span class="str">"hdr_judul"</span>, <span class="str">"🛒 SISTEM KASIR KEDAULATAN NUSANTARA"</span>, <span class="num">30</span>, <span class="num">20</span>);

    <span class="com">// Komponen Input Barcode & Kuantitas</span>
    app.<span class="fn">tambah_label</span>(<span class="str">"lbl_sku"</span>, <span class="str">"Kode Barcode / SKU:"</span>, <span class="num">30</span>, <span class="num">65</span>);
    app.<span class="fn">tambah_input</span>(<span class="str">"input_sku"</span>, <span class="str">"Ketik SKU (cth: KOP-01)..."</span>, <span class="num">30</span>, <span class="num">90</span>);

    app.<span class="fn">tambah_label</span>(<span class="str">"lbl_qty"</span>, <span class="str">"Jumlah (Qty):"</span>, <span class="num">350</span>, <span class="num">65</span>);
    app.<span class="fn">tambah_input</span>(<span class="str">"input_qty"</span>, <span class="str">"1"</span>, <span class="num">350</span>, <span class="num">90</span>);

    <span class="com">// Label Indikator Total Realtime</span>
    app.<span class="fn">tambah_label</span>(<span class="str">"lbl_total_layar"</span>, <span class="str">"TOTAL TAGIHAN: Rp 0"</span>, <span class="num">550</span>, <span class="num">85</span>);

    <span class="com">// 4. Tombol Tambah Barang ke Keranjang Belanja</span>
    app.<span class="fn">tambah_tombol</span>(<span class="str">"btn_tambah"</span>, <span class="str">"+ Masukkan Keranjang"</span>, <span class="num">30</span>, <span class="num">140</span>, <span class="fn">fungsi</span>() {
        <span class="kw">misal</span> kode_sku = app.<span class="fn">ambil_nilai</span>(<span class="str">"input_sku"</span>);
        <span class="kw">misal</span> kuantitas = <span class="fn">ke_angka</span>(app.<span class="fn">ambil_nilai</span>(<span class="str">"input_qty"</span>));

        <span class="kw">jika</span> kuantitas <= <span class="num">0</span> {
            app.<span class="fn">tampilkan_dialog</span>(<span class="str">"Peringatan"</span>, <span class="str">"Kuantitas barang harus minimal 1 pcs!"</span>);
            <span class="kw">kembalikan</span>;
        }

        <span class="com">// Ambil data barang dari WidyaBase</span>
        <span class="kw">misal</span> baris = db.<span class="fn">kueri_satu</span>(<span class="str">"PILIH * DARI produk DIMANA sku = '"</span> + kode_sku + <span class="str">"';"</span>);
        <span class="kw">jika</span> baris == <span class="kw">nihil</span> {
            <span class="com">// Data sampel otomatis jika barang belum ada</span>
            baris = { <span class="str">"sku"</span>: kode_sku, <span class="str">"nama"</span>: <span class="str">"Produk Unggulan "</span> + kode_sku, <span class="str">"harga"</span>: <span class="num">25000</span> };
        }

        <span class="kw">misal</span> subtotal_item = baris[<span class="str">"harga"</span>] * kuantitas;
        total_akumulasi = total_akumulasi + subtotal_item;

        <span class="fn">tambah</span>(keranjang_item, {
            <span class="str">"sku"</span>: kode_sku,
            <span class="str">"nama"</span>: baris[<span class="str">"nama"</span>],
            <span class="str">"qty"</span>: kuantitas,
            <span class="str">"harga"</span>: baris[<span class="str">"harga"</span>],
            <span class="str">"subtotal"</span>: subtotal_item
        });

        <span class="com">// Perbarui tampilan label total di layar kasir</span>
        app.<span class="fn">set_teks</span>(<span class="str">"lbl_total_layar"</span>, <span class="str">"TOTAL TAGIHAN: Rp "</span> + <span class="fn">ke_teks</span>(total_akumulasi));
        app.<span class="fn">tampilkan_dialog</span>(<span class="str">"Sukses"</span>, <span class="str">"Berhasil menambahkan: "</span> + baris[<span class="str">"nama"</span>]);
    });

    <span class="com">// 5. Tombol Selesaikan Transaksi & Cetak Struk</span>
    app.<span class="fn">tambah_tombol</span>(<span class="str">"btn_selesai"</span>, <span class="str">"💳 SELESAIKAN PEMBAYARAN & CETAK STRUK"</span>, <span class="num">30</span>, <span class="num">560</span>, <span class="fn">fungsi</span>() {
        <span class="kw">jika</span> <span class="fn">panjang</span>(keranjang_item) == <span class="num">0</span> {
            app.<span class="fn">tampilkan_dialog</span>(<span class="str">"Peringatan"</span>, <span class="str">"Keranjang belanja masih kosong!"</span>);
            <span class="kw">kembalikan</span>;
        }

        <span class="kw">misal</span> pajak_ppn = <span class="fn">ke_angka</span>(total_akumulasi * <span class="num">0.11</span>);
        <span class="kw">misal</span> total_akhir = total_akumulasi + pajak_ppn;

        <span class="kw">misal</span> pesan_struk = <span class="str">"====================================\n"</span> +
                           <span class="str">"         STRUK PENJUALAN TOKO        \n"</span> +
                           <span class="str">"====================================\n"</span> +
                           <span class="str">"Subtotal Belanja : Rp "</span> + <span class="fn">ke_teks</span>(total_akumulasi) + <span class="str">"\n"</span> +
                           <span class="str">"Pajak PPN (11%)  : Rp "</span> + <span class="fn">ke_teks</span>(pajak_ppn) + <span class="str">"\n"</span> +
                           <span class="str">"Total Akhir Bayar: Rp "</span> + <span class="fn">ke_teks</span>(total_akhir) + <span class="str">"\n"</span> +
                           <span class="str">"Metode           : KASIR TUNAI MANDIRI\n"</span> +
                           <span class="str">"====================================\n"</span> +
                           <span class="str">"    Terima Kasih Atas Kunjungan Anda! \n"</span>;

        app.<span class="fn">tampilkan_dialog</span>(<span class="str">"Transaksi Berhasil"</span>, pesan_struk);

        <span class="com">// Reset Keranjang untuk Transaksi Berikutnya</span>
        keranjang_item = [];
        total_akumulasi = <span class="num">0</span>;
        app.<span class="fn">set_teks</span>(<span class="str">"lbl_total_layar"</span>, <span class="str">"TOTAL TAGIHAN: Rp 0"</span>);
    });

    <span class="com">// 6. Jalankan Loop Antarmuka Desktop</span>
    app.<span class="fn">jalankan</span>();
}

<span class="fn">inisialisasi_kasir_pos</span>();
</code></pre>

<!-- BAB 10 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">10</span> Rekayasa Aplikasi Mobile & Integrasi QRIS ASPI</h1>

<h2>10.1 Standar Nasional QRIS (Quick Response Code Indonesian Standard)</h2>
<p>
Sesuai dengan regulasi Bank Indonesia dan Asosiasi Sistem Pembayaran Indonesia (ASPI), seluruh transaksi pembayaran nirsentuh di Indonesia menggunakan spesifikasi <strong>EMVCo Tag-Length-Value (TLV)</strong>. Format ini menyusun seluruh data merchant, nomor akun nasional (NMID), kategori usaha, mata uang Rupiah (Kode ISO 360), dan checksum keamanan dalam satu deret teks terstandarisasi.
</p>

<h2>10.2 Struktur Tag-Length-Value (TLV) QRIS</h2>
<table>
  <thead>
    <tr>
      <th>Tag ID</th>
      <th>Nama Field Standar ASPI</th>
      <th>Format & Contoh Nilai</th>
      <th>Keterangan Teknis</th>
    </tr>
  </thead>
  <tbody>
    <tr><td><code>00</code></td><td>Payload Format Indicator</td><td><code>01</code></td><td>Menandakan spesifikasi EMVCo versi 1.0.</td></tr>
    <tr><td><code>01</code></td><td>Point of Initiation Method</td><td><code>11</code> (Statis) / <code>12</code> (Dinamis)</td><td>Statis untuk stiker meja, Dinamis jika nominal tersemat.</td></tr>
    <tr><td><code>26-51</code></td><td>Merchant Account Information</td><td>NMID & ID Acquirer</td><td>Data routing perbankan penerima dana.</td></tr>
    <tr><td><code>52</code></td><td>Merchant Category Code</td><td><code>5812</code> (Restoran)</td><td>Klasifikasi bidang usaha merchant (MCC).</td></tr>
    <tr><td><code>53</code></td><td>Transaction Currency</td><td><code>360</code> (Rupiah Indonesia)</td><td>Kode mata uang ISO 4217 untuk IDR.</td></tr>
    <tr><td><code>54</code></td><td>Transaction Amount</td><td><code>50000.00</code></td><td>Nominal uang yang harus dibayar konsumen.</td></tr>
    <tr><td><code>58</code></td><td>Country Code</td><td><code>ID</code></td><td>Kode negara Indonesia sesuai ISO 3166.</td></tr>
    <tr><td><code>59</code></td><td>Merchant Name</td><td><code>KOPERASI MERAH PUTIH</code></td><td>Nama toko fisik yang tampil pada aplikasi pembeli.</td></tr>
    <tr><td><code>60</code></td><td>Merchant City</td><td><code>JAKARTA PUSAT</code></td><td>Kota lokasi merchant terdaftar.</td></tr>
    <tr><td><code>63</code></td><td>CRC-16 Checksum</td><td><code>4 digit heksadesimal</code></td><td>Verifikasi integritas payload mencegah manipulasi data.</td></tr>
  </tbody>
</table>

<h2>10.3 Implementasi Algoritma Validasi Checksum CRC-16 CCITT</h2>
<p>
Setiap kode QRIS wajib divalidasi integritasnya menggunakan algoritma CRC-16 polinomial <code>0x1021</code> dengan inisialisasi awal <code>0xFFFF</code>. Berikut adalah implementasi murni dalam Widya-Lang:
</p>

<pre><code><span class="fn">fungsi</span> <span class="fn">hitung_crc16_qris</span>(teks_payload_tanpa_crc) {
    <span class="kw">misal</span> crc = <span class="num">0xFFFF</span>;
    <span class="kw">misal</span> p = <span class="num">0x1021</span>;
    <span class="kw">misal</span> n = <span class="fn">panjang</span>(teks_payload_tanpa_crc);

    <span class="kw">untuk</span> i <span class="kw">dalam</span> <span class="num">0</span>..(n - <span class="num">1</span>) {
        <span class="kw">misal</span> bita = <span class="fn">kode_karakter</span>(teks_payload_tanpa_crc, i);
        crc = crc ^ (bita << <span class="num">8</span>);

        <span class="kw">untuk</span> bit <span class="kw">dalam</span> <span class="num">0</span>..<span class="num">7</span> {
            <span class="kw">jika</span> (crc & <span class="num">0x8000</span>) != <span class="num">0</span> {
                crc = ((crc << <span class="num">1</span>) ^ p) & <span class="num">0xFFFF</span>;
            } <span class="kw">lainnya</span> {
                crc = (crc << <span class="num">1</span>) & <span class="num">0xFFFF</span>;
            }
        }
    }

    <span class="kw">kembalikan</span> <span class="fn">ke_heksadesimal</span>(crc);
}
</code></pre>

<h2>10.4 Pola Penyimpanan Outbox Offline-First & Sinkronisasi Background</h2>
<p>
Di wilayah dengan konektivitas internet yang fluktuatif, aplikasi mobile kasir pedagang wajib mampu menerima pembayaran secara lokal, menyimpannya di antrean <strong>Outbox Pattern</strong> WidyaBase tersemat, lalu menyinkronkannya ke server pusat secara otomatis saat sinyal kembali pulih:
</p>

<pre><code><span class="kw">impor</span> <span class="str">"./modul/mobile.wya"</span>;

<span class="fn">fungsi</span> <span class="fn">jalankan_kasir_mobile</span>() {
    <span class="kw">misal</span> app = <span class="fn">AplikasiMobile</span>(<span class="str">"Warung Pintar Mobile QRIS"</span>);

    <span class="com">// Menghubungkan ke Scanner Barcode Kamera HP</span>
    app.<span class="fn">aktifkan_pemindai_qris</span>(<span class="fn">fungsi</span>(raw_qris) {
        <span class="kw">misal</span> data = <span class="fn">urai_tlv_qris</span>(raw_qris);
        <span class="fn">cetak</span>(<span class="str">"Terbaca Toko :"</span>, data[<span class="str">"59"</span>]); <span class="com">// Tag 59 = Merchant Name</span>
        <span class="fn">cetak</span>(<span class="str">"Nominal Bayar: Rp"</span>, data[<span class="str">"54"</span>]); <span class="com">// Tag 54 = Amount</span>

        <span class="com">// Simpan ke Outbox Offline</span>
        app.<span class="fn">simpan_outbox_offline</span>({
            <span class="str">"id_transaksi"</span>: <span class="fn">buat_uuid</span>(),
            <span class="str">"merchant"</span>: data[<span class="str">"59"</span>],
            <span class="str">"nominal"</span>: data[<span class="str">"54"</span>],
            <span class="str">"waktu"</span>: <span class="fn">waktu_saat_ini</span>(),
            <span class="str">"status"</span>: <span class="str">"MENUNGGU_SINKRONISASI"</span>
        });

        <span class="com">// Cetak Bukti Bayar via Printer Bluetooth Thermal 58mm</span>
        app.<span class="fn">cetak_bluetooth_thermal</span>([
            <span class="str">"================================"</span>,
            <span class="str">"       WARUNG PINTAR MOBILE     "</span>,
            <span class="str">"================================"</span>,
            <span class="str">"Status : LUNAS (QRIS ASPI)      "</span>,
            <span class="str">"Toko   : "</span> + data[<span class="str">"59"</span>],
            <span class="str">"Total  : Rp "</span> + data[<span class="str">"54"</span>],
            <span class="str">"================================"</span>,
            <span class="str">" Simpan Struk Sebagai Bukti Sah "</span>
        ]);
    });

    app.<span class="fn">mulai</span>();
}
</code></pre>

<!-- BAB 11 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">11</span> Integrasi Basis Data Mandiri WidyaBase Skala Industri</h1>

<h2>11.1 Arsitektur Slotted-Page & Storage Engine B-Tree</h2>
<p>
<strong>WidyaBase</strong> adalah mesin basis data relasional tertanam (<em>embedded relational database engine</em>) berkinerja tinggi yang dikompilasi secara asli di dalam pustaka inti Widya-Lang. Dirancang khusus untuk ketahanan crash skala enterprise, WidyaBase membagi media penyimpanan menjadi halaman-halaman memori tetap berukuran <strong>4096 bita (4 KB)</strong> menggunakan arsitektur <em>Slotted-Page</em>.
</p>

<div class="diagram-box">
  ┌─────────────────────────────────────────────────────────────────┐
  │                 Halaman Slotted-Page (4096 Bita)                │
  ├───────────────┬────────────────────────────────┬────────────────┤
  │ Header Page   │ Array Slot Pointer Offset      │ Ruang Bebas    │
  │ (Nomor Halaman│ [Slot 0: 4010] [Slot 1: 3880]  │ (Free Space    │
  │  & Flag Status│ [Slot 2: 3750] ...             │  Tumbuh Maju)  │
  ├───────────────┴────────────────────────────────┴────────────────┤
  │                                                                 │
  │ Payload Rekaman Data Baris (Tumbuh Mundur dari Ujung Bawah 4KB) │
  │ ... [Baris 2: Data] ... [Baris 1: Data] ... [Baris 0: Data]    │
  └─────────────────────────────────────────────────────────────────┘
</div>

<h2>11.2 Enam Belas Pilar Enterprise WidyaBase</h2>
<ol>
  <li><strong>Storage Engine Slotted-Page</strong>: Manajemen memori 4KB per halaman dengan pemanfaatan ruang bebas optimal dan defragmentasi otomatis.</li>
  <li><strong>Write-Ahead Logging (WAL) & Crash Recovery</strong>: Setiap mutasi data dicatat ke berkas jurnal WAL sebelum halaman disk dimodifikasi, menjamin pemulihan instan pasca mati listrik.</li>
  <li><strong>Indexing B-Tree Primer & Sekunder</strong>: Pencarian data dengan kompleksitas waktu $O(\log N)$ dan pemindaian rentang sekuensial cepat.</li>
  <li><strong>Parser ANSI SQL Lengkap & Dialek Nusantara</strong>: Mendukung kueri standar SQL global (<code>SELECT</code>, <code>INSERT</code>, <code>UPDATE</code>) maupun istilah Nusantara (<code>PILIH</code>, <code>SISIP_KE</code>, <code>PERBARUI</code>).</li>
  <li><strong>Relational Constraints & Kunci Asing (Foreign Keys)</strong>: Penegakan integritas referensial antar-tabel secara otomatis di tingkat mesin penyimpanan.</li>
  <li><strong>Transaksi ACID Penuh & Savepoint Transaksional</strong>: Jaminan <em>Atomicity, Consistency, Isolation, Durability</em> dengan kemampuan rollback bertingkat via savepoint.</li>
  <li><strong>Kueri Analitik CTE Rekursif (`DENGAN REKURSIF`)</strong>: Pemrosesan data hierarkis pohon organisasi dan rantai pasok multi-level dalam kueri tunggal.</li>
  <li><strong>Window Functions Lengkap</strong>: Fungsi analitik analitik modern: <code>NOMOR_BARIS()</code>, <code>PERINGKAT()</code>, <code>PERINGKAT_DENSE()</code>, dan <code>JUMLAH() LEWAT()</code>.</li>
  <li><strong>Full-Text Search Tokenizer Bahasa Indonesia & BM25 Scoring</strong>: Mesin pencarian teks lengkap dengan penanganan imbuhan morfologi kata Indonesia dan pembobotan BM25.</li>
  <li><strong>Kolom Tipe JSON & Operator `JSON_EXTRACT`</strong>: Penyimpanan dokumen semi-terstruktur berkinerja tinggi langsung di dalam kolom tabel relasional.</li>
  <li><strong>Tipe Data Geospasial R-Tree & Fungsi Geometris</strong>: Pengindeksan koordinat spasial dua dimensi dengan fungsi jarak <code>ST_Jarak</code> dan <code>ST_Titik</code>.</li>
  <li><strong>Cadangan Inkremental (Incremental Hot Backup)</strong>: Pencadangan data secara langsung tanpa menghentikan operasi baca dan tulis transaksi pengguna.</li>
  <li><strong>Audit Logging Transaksi</strong>: Pencatatan jejak audit komprehensif untuk mendeteksi perubahan data oleh pengguna sistem perbankan.</li>
  <li><strong>Time-Travel Query Engine (`PADA CAP WAKTU`)</strong>: Kemampuan membaca keadaan historis data pada titik waktu tertentu di masa lalu.</li>
  <li><strong>Zero-Copy In-Memory Vector Store</strong>: Penyimpanan vektor embedding AI berdimensi tinggi untuk pencarian kemiripan semantik latensi mikro-detik.</li>
  <li><strong>Pool Koneksi Multithread Berkinerja Tinggi</strong>: Manajemen antrean koneksi transaksi konkurensi tinggi tanpa risiko kebuntuan (<em>deadlock-free</em>).</li>
</ol>

<h2>11.3 Contoh Kueri Analitik Lanjutan: CTE Rekursif & Window Functions</h2>
<pre><code><span class="kw">impor</span> <span class="str">"./modul/widyabase.wya"</span>;

<span class="fn">fungsi</span> <span class="fn">analisis_hierarki_perusahaan</span>() {
    <span class="kw">misal</span> db = <span class="fn">BukaWidyaBase</span>(<span class="str">"korporasi_nasional.wdb"</span>);

    <span class="com">// Kueri CTE Rekursif untuk Menelusuri Pohon Struktur Manajemen</span>
    <span class="kw">misal</span> kueri_pohon = <span class="str">"
        DENGAN REKURSIF hierarki_karyawan SEBAGAI (
            -- Anggota Basis: Direktur Utama (Tanpa Atasan)
            PILIH nik, nama, jabatan, id_atasan, 1 SEBAGAI level_hierarki
            DARI tabel_karyawan
            DIMANA id_atasan ADALAH NIHIL

            GABUNG_SEMUA

            -- Anggota Rekursif: Bawahan dari Level Sebelumnya
            PILIH k.nik, k.nama, k.jabatan, k.id_atasan, h.level_hierarki + 1
            DARI tabel_karyawan AS k
            GABUNG hierarki_karyawan AS h PADA k.id_atasan = h.nik
        )
        PILIH * DARI hierarki_karyawan URUTKAN_BERDASARKAN level_hierarki NAIK;
    "</span>;

    <span class="kw">misal</span> hasil_pohon = db.<span class="fn">kueri</span>(kueri_pohon);
    <span class="kw">untuk</span> baris <span class="kw">dalam</span> hasil_pohon {
        <span class="fn">cetak</span>(<span class="str">"Level "</span> + <span class="fn">ke_teks</span>(baris[<span class="str">"level_hierarki"</span>]) + <span class="str">" | "</span> + baris[<span class="str">"jabatan"</span>] + <span class="str">" : "</span> + baris[<span class="str">"nama"</span>]);
    }
}
</code></pre>
`
};
