module.exports = {
  content: `
<!-- BAB 15 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">15</span> Keamanan Siber, Kriptografi & Kepatuhan UU PDP</h1>

<h2>15.1 Kriptografi Simetris Skala Perbankan: AES-256-GCM</h2>
<p>
Data sensitif seperti Nomor Induk Kependudukan (NIK), rincian kartu pembayaran, dan saldo perbankan dilarang keras disimpan dalam bentuk teks polos (<em>plaintext</em>). Modul resmi <code>modul/kripto_aman.wya</code> menyediakan enkripsi simetris <strong>AES-256-GCM (Galois/Counter Mode)</strong> yang memadukan kerahasiaan data (<em>confidentiality</em>) dengan verifikasi integritas anti-manipulasi (<em>authenticated encryption</em>):
</p>

<pre><code><span class="kw">impor</span> <span class="str">"./modul/kripto_aman.wya"</span>;

<span class="fn">fungsi</span> <span class="fn">amankan_data_nasabah</span>() {
    <span class="com">// Kunci rahasia 256-bit (32 bita) disimpan di Vault HSM</span>
    <span class="kw">tetap</span> KUNCI_MASTER_HEX = <span class="str">"603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4"</span>;
    <span class="kw">misal</span> data_rahasia = <span class="str">"NIK: 3171010901920002 | Saldo: Rp 1.500.000.000"</span>;

    <span class="com">// 1. Enkripsi Terotentikasi (Menghasilkan Ciphertext + Tag Integritas)</span>
    <span class="kw">misal</span> hasil_enkripsi = <span class="fn">EnkripsiAES256GCM</span>(data_rahasia, KUNCI_MASTER_HEX);
    <span class="fn">cetak</span>(<span class="str">"Teks Terenkripsi (Ciphertext):"</span>, hasil_enkripsi.ciphertext);
    <span class="fn">cetak</span>(<span class="str">"Nonce / IV Unik             :"</span>, hasil_enkripsi.nonce);
    <span class="fn">cetak</span>(<span class="str">"Tag Autentikasi Integritas  :"</span>, hasil_enkripsi.tag);

    <span class="com">// 2. Dekripsi & Verifikasi Integritas</span>
    <span class="kw">misal</span> data_asli = <span class="fn">DekripsiAES256GCM</span>(hasil_enkripsi.ciphertext, KUNCI_MASTER_HEX, hasil_enkripsi.nonce, hasil_enkripsi.tag);
    <span class="fn">cetak</span>(<span class="str">"Hasil Dekripsi Sah          :"</span>, data_asli);
}
</code></pre>

<h2>15.2 Kriptografi Asimetris: Pasangan Kunci Ed25519 & Digital Signature</h2>
<p>
Untuk pengesahan kontrak digital, verifikasi paket rilis biner, dan otorisasi pembayaran lintas institusi, Widya-Lang menggunakan kurva eliptis modern <strong>Ed25519 (Edwards-curve Digital Signature Algorithm)</strong> yang memiliki ketahanan luar biasa terhadap serangan komputasi kuantum awal:
</p>

<pre><code><span class="fn">fungsi</span> <span class="fn">tanda_tangan_dokumen_digital</span>() {
    <span class="com">// Pembuatan Pasangan Kunci Publik & Privat Ed25519</span>
    <span class="kw">misal</span> pasangan_kunci = <span class="fn">BuatPasanganKunciEd25519</span>();
    <span class="kw">misal</span> dokumen_transaksi = <span class="str">"Persetujuan Pembiayaan Investasi Rp 25.000.000.000 - Bank Mandiri"</span>;

    <span class="com">// Penandatanganan menggunakan Kunci Privat Penjamin</span>
    <span class="kw">misal</span> tanda_tangan = <span class="fn">TandatanganiEd25519</span>(dokumen_transaksi, pasangan_kunci.kunci_privat);
    <span class="fn">cetak</span>(<span class="str">"Tanda Tangan Digital Sah:"</span>, tanda_tangan);

    <span class="com">// Verifikasi Publik oleh Siapapun Menggunakan Kunci Publik</span>
    <span class="kw">misal</span> verifikasi_sah = <span class="fn">VerifikasiEd25519</span>(dokumen_transaksi, tanda_tangan, pasangan_kunci.kunci_publik);
    <span class="fn">cetak</span>(<span class="str">"Keabsahan Dokumen Diakui:"</span>, verifikasi_sah); <span class="com">// benar</span>
}
</code></pre>

<h2>15.3 Kepatuhan Terhadap Regulasi UU Perlindungan Data Pribadi (UU PDP No. 27/2022)</h2>
<p>
Undang-Undang Nomor 27 Tahun 2022 tentang Pelindungan Data Pribadi mewajibkan seluruh pengelola sistem elektronik di Indonesia untuk menerapkan standar keamanan tingkat tinggi. Arsitektur sistem Widya-Lang secara baku mematuhi prinsip-prinsip tersebut:
</p>
<ul>
  <li><strong>Prinsip Minimisai Data (<em>Data Minimization</em>)</strong>: Hanya data yang benar-benar esensial yang disimpan di dalam kolom WidyaBase.</li>
  <li><strong>Hashing Kata Sandi dengan Argon2id</strong>: Menggunakan algoritma pemenang password-hashing competition (Argon2id) dengan parameter memori tinggi untuk mencegah serangan brute-force via ASIC/GPU.</li>
  <li><strong>Hak untuk Dihapus (<em>Right to Erasure / Shreddable Data</em>)</strong>: Penghapusan baris data pelanggan di WidyaBase secara fisik menimpa slot halaman memori dengan angka nol (<em>cryptographic wiping</em>).</li>
</ul>

<!-- BAB 16 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">16</span> Praktik Pengujian Perangkat Lunak, Benchmarking & Standar Koding</h1>

<h2>16.1 Kerangka Pengujian Unit Bawaan (Built-in Test Framework)</h2>
<p>
Keandalan perangkat lunak tidak dapat diserahkan pada keberuntungan. Widya-Lang menyertakan eksekutor pengujian unit langsung di dalam perkakas CLI tanpa memerlukan pustaka testing eksternal. Anda cukup mendeklarasikan blok pengujian menggunakan kata kunci <code>tes</code>:
</p>

<pre><code><span class="com">// Berkas: tes_kalkulator_pajak.wya</span>
<span class="kw">impor</span> <span class="str">"./modul/pajak.wya"</span>;

<span class="fn">tes</span> <span class="str">"Perhitungan Tarif PPN 11% Wajib Presisi"</span> {
    <span class="kw">misal</span> omzet = <span class="num">10000000</span>;
    <span class="kw">misal</span> ppn = <span class="fn">hitung_ppn_standar</span>(omzet);
    
    <span class="com">// Rangkaian Asersi Penjamin Kualitas</span>
    <span class="fn">pastikan_sama</span>(ppn, <span class="num">1100000</span>);
    <span class="fn">pastikan_benar</span>(ppn > <span class="num">0</span>);
}

<span class="fn">tes</span> <span class="str">"Transaksi Bernilai Negatif Wajib Melempar Galat"</span> {
    <span class="fn">pastikan_gagal</span>(<span class="fn">fungsi</span>() {
        <span class="fn">hitung_ppn_standar</span>(-<span class="num">50000</span>);
    });
}
</code></pre>

<p>Jalankan seluruh pengujian unit melalui CLI terminal:</p>
<pre><code>$ widya tes
Menjalankan Rangkaian Pengujian Otomatis Widya-Lang...
✓ [SUKSES] Perhitungan Tarif PPN 11% Wajib Presisi (1.2 ms)
✓ [SUKSES] Transaksi Bernilai Negatif Wajib Melempar Galat (0.8 ms)

Hasil Akhir: 2 Lulus | 0 Gagal | Total Waktu: 2.0 ms (100% Sempurna)
</code></pre>

<h2>16.2 Pengukuran Kinerja Komputasi (Benchmarking Engine)</h2>
<pre><code><span class="fn">tolok_ukur</span> <span class="str">"Throughput Pencarian Biner WidyaBase"</span> {
    <span class="kw">misal</span> koleksi = <span class="fn">buat_deret_acak</span>(<span class="num">1000000</span>);
    <span class="fn">urutkan</span>(koleksi);

    <span class="com">// Mesin benchmark mengeksekusi blok ini sebanyak 100.000 iterasi</span>
    <span class="fn">pencarian_biner</span>(koleksi, <span class="num">754210</span>);
}
</code></pre>

<h2>16.3 Panduan Gaya Penulisan Resmi (Official Style Guide)</h2>
<table>
  <thead>
    <tr>
      <th>Unsur Sintaks</th>
      <th>Konvensi Penulisan</th>
      <th>Contoh Baku yang Direkomendasikan</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>Variabel Dinamis</td><td><code>snake_case</code> huruf kecil</td><td><code>misal saldo_akhir = 0;</code></td></tr>
    <tr><td>Konstanta Abadi</td><td><code>SCREAMING_SNAKE_CASE</code></td><td><code>tetap PERSENTASE_BUNGA = 0.05;</code></td></tr>
    <tr><td>Nama Fungsi</td><td><code>snake_case</code> kata kerja</td><td><code>fungsi hitung_anuitas() { }</code></td></tr>
    <tr><td>Entitas Struktur</td><td><code>PascalCase</code> kata benda</td><td><code>struktur RekeningGiro { }</code></td></tr>
    <tr><td>Nama Sifat (Trait)</td><td><code>PascalCase</code> kata sifat/kemampuan</td><td><code>sifat DapatDicetak { }</code></td></tr>
    <tr><td>Indentasi Blok</td><td>4 Karakter Spasi (Bukan Tab)</td><td>Konsisten di seluruh fungsi & blok cabang</td></tr>
  </tbody>
</table>

<!-- BAB 17 -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">17</span> Panduan Distribusi, Kompilasi Biner & DevOps</h1>

<h2>17.1 Kompilasi Biner Mandiri Lintas Platform</h2>
<p>
Untuk mendistribusikan aplikasi kepada pengguna akhir tanpa meminta mereka memasang perkakas compiler Widya-Lang, lakukan kompilasi biner mandiri (<em>standalone binary compilation</em>):
</p>

<pre><code># Kompilasi untuk Sistem Operasi Windows 64-bit (.exe)
$ widya bangun aplikasi_kasir.wya --target windows-x64 --keluaran dist/KasirNusantara.exe

# Kompilasi untuk Server Linux Cloud (.elf)
$ widya bangun server_api.wya --target linux-x64 --keluaran dist/server_api
</code></pre>

<h2>17.2 Pengemasan Rilis Resmi & Verifikasi Hash SHA-256 + Ed25519</h2>
<p>
Setiap berkas artefak rilis wajib dihitung checksum SHA-256 dan ditandatangani secara kriptografis menggunakan kunci privat Ed25519 pengembang resmi untuk menjamin biner bebas dari sisipan malware atau pemalsuan rantai pasok (<em>supply-chain tampering</em>):
</p>

<pre><code># 1. Menghitung Hash Kriptografis SHA-256
$ sha256sum dist/releases/v1.0.0/widya-v1.0.0-windows-x64.zip &gt; dist/releases/v1.0.0/SHA256SUMS.txt

# 2. Menandatangani berkas checksum dengan Kunci Ed25519 Resmi
$ widya tanda-tangani dist/releases/v1.0.0/SHA256SUMS.txt --kunci-privat kunci_rilis.pem --keluaran dist/releases/v1.0.0/SHA256SUMS.sig
</code></pre>

<h2>17.3 Kontainerisasi Dockerfile Multi-Stage</h2>
<div class="diagram-box">
# ========================================================
# Dockerfile Multi-Stage: Aplikasi Widya-Lang Enterprise
# ========================================================

# Tahap 1: Pembangunan (Builder)
FROM rust:1.80-slim AS pembangun
WORKDIR /app
COPY . .
RUN cargo build --release

# Tahap 2: Citra Produksi Minimalis (Hanya ~15 MB)
FROM alpine:3.20
RUN apk add --no-cache ca-certificates libgcc
WORKDIR /aplikasi
COPY --from=pembangun /app/target/release/widya /usr/local/bin/widya
COPY modul/ /aplikasi/modul/
COPY aplikasi_server.wya /aplikasi/

EXPOSE 8080
ENTRYPOINT ["widya", "jalan", "aplikasi_server.wya"]
</div>

<!-- LAMPIRAN A -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">A</span> Lampiran: Kamus Lengkap Sintaks Dialek Nusantara</h1>

<table>
  <thead>
    <tr>
      <th>Kata Kunci Nusantara</th>
      <th>Padanan Bahasa Inggris</th>
      <th>Kategori Sintaksis</th>
      <th>Deskripsi Semantik & Contoh Penggunaan</th>
    </tr>
  </thead>
  <tbody>
    <tr><td><code>misal</code></td><td><code>let / var / mut</code></td><td>Variabel</td><td>Mendeklarasikan variabel mutable dinamis: <code>misal x = 10;</code></td></tr>
    <tr><td><code>tetap</code> / <code>tetapan</code></td><td><code>const / immutable</code></td><td>Konstanta</td><td>Mendeklarasikan konstanta terkunci: <code>tetap PI = 3.14;</code></td></tr>
    <tr><td><code>fungsi</code></td><td><code>function / def / fn</code></td><td>Subrutin</td><td>Mendefinisikan blok fungsi komputasi: <code>fungsi hitung() { }</code></td></tr>
    <tr><td><code>kembalikan</code></td><td><code>return</code></td><td>Alur Fungsi</td><td>Mengembalikan nilai dari eksekusi fungsi: <code>kembalikan total;</code></td></tr>
    <tr><td><code>jika</code></td><td><code>if</code></td><td>Percabangan</td><td>Mengevaluasi kondisi logika pertama: <code>jika nilai >= 75 { }</code></td></tr>
    <tr><td><code>kalau</code></td><td><code>else if / elif</code></td><td>Percabangan</td><td>Mengevaluasi cabang kondisi berikutnya: <code>kalau nilai >= 60 { }</code></td></tr>
    <tr><td><code>lainnya</code></td><td><code>else</code></td><td>Percabangan</td><td>Cabang alternatif jika seluruh kondisi salah: <code>lainnya { }</code></td></tr>
    <tr><td><code>untuk</code></td><td><code>for</code></td><td>Perulangan</td><td>Iterasi rentang angka atau koleksi data: <code>untuk i dalam 1..10 { }</code></td></tr>
    <tr><td><code>selama</code></td><td><code>while</code></td><td>Perulangan</td><td>Perulangan berbasis syarat kondisi benar: <code>selama aktif { }</code></td></tr>
    <tr><td><code>henti</code></td><td><code>break</code></td><td>Kendali Loop</td><td>Menghentikan perulangan seketika: <code>henti;</code></td></tr>
    <tr><td><code>lanjut</code></td><td><code>continue</code></td><td>Kendali Loop</td><td>Melompati iterasi saat ini menuju iterasi berikutnya: <code>lanjut;</code></td></tr>
    <tr><td><code>struktur</code></td><td><code>struct / class</code></td><td>Tipe Bentukan</td><td>Mendefinisikan entitas data terstruktur: <code>struktur Nasabah { }</code></td></tr>
    <tr><td><code>sifat</code></td><td><code>trait / interface</code></td><td>Kontrak Perilaku</td><td>Mendefinisikan antarmuka polimorfisme: <code>sifat Transaksi { }</code></td></tr>
    <tr><td><code>coba</code></td><td><code>try</code></td><td>Penanganan Galat</td><td>Membuka blok eksekusi rawan kegagalan: <code>coba { ... }</code></td></tr>
    <tr><td><code>tangkap</code></td><td><code>catch / except</code></td><td>Penanganan Galat</td><td>Menangkap objek galat yang terlempar: <code>tangkap (galat) { ... }</code></td></tr>
    <tr><td><code>akhirnya</code></td><td><code>finally</code></td><td>Penanganan Galat</td><td>Blok yang dijamin selalu dieksekusi: <code>akhirnya { tutup(); }</code></td></tr>
    <tr><td><code>lempar</code></td><td><code>throw / raise</code></td><td>Pemicu Galat</td><td>Memicu galat terstruktur secara eksplisit: <code>lempar "Saldo minus";</code></td></tr>
    <tr><td><code>impor</code></td><td><code>import / use</code></td><td>Modularitas</td><td>Memuat berkas atau pustaka lain: <code>impor "./modul/desktop.wya";</code></td></tr>
    <tr><td><code>cetak</code></td><td><code>print / println</code></td><td>I/O Standar</td><td>Mencetak baris data ke terminal konsol: <code>cetak("Halo Indonesia");</code></td></tr>
    <tr><td><code>dan</code></td><td><code>&& / and</code></td><td>Operator Logika</td><td>Evaluasi konjungsi logika biner: <code>jika a dan b { }</code></td></tr>
    <tr><td><code>atau</code></td><td><code>|| / or</code></td><td>Operator Logika</td><td>Evaluasi disjungsi logika biner: <code>jika a atau b { }</code></td></tr>
    <tr><td><code>bukan</code></td><td><code>! / not</code></td><td>Operator Logika</td><td>Negasi nilai kebenaran logika: <code>jika bukan aktif { }</code></td></tr>
    <tr><td><code>benar</code> / <code>salah</code></td><td><code>true / false</code></td><td>Literal Boolean</td><td>Konstanta nilai kebenaran logika: <code>misal valid = benar;</code></td></tr>
    <tr><td><code>nihil</code></td><td><code>null / nil / none</code></td><td>Literal Ketiadaan</td><td>Menandakan ketiadaan nilai: <code>misal data = nihil;</code></td></tr>
    <tr><td><code>panjang</code></td><td><code>len / length / size</code></td><td>Fungsi Bawaan</td><td>Menghitung panjang deret atau jumlah karakter string: <code>panjang(deret)</code></td></tr>
    <tr><td><code>tambah</code></td><td><code>push / append</code></td><td>Fungsi Bawaan</td><td>Menyisipkan elemen ke bagian ekor deret: <code>tambah(daftar, item)</code></td></tr>
    <tr><td><code>iris</code></td><td><code>slice / substring</code></td><td>Fungsi Bawaan</td><td>Mengambil potongan deret atau string: <code>iris(teks, 0, 5)</code></td></tr>
    <tr><td><code>ke_angka</code></td><td><code>parseInt / toInt</code></td><td>Konversi Tipe</td><td>Mengonversi representasi string menjadi integer: <code>ke_angka("150")</code></td></tr>
    <tr><td><code>ke_teks</code></td><td><code>toString / str</code></td><td>Konversi Tipe</td><td>Mengonversi nilai tipe apapun menjadi string teks: <code>ke_teks(42)</code></td></tr>
  </tbody>
</table>

<!-- LAMPIRAN B -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">B</span> Lampiran: Tabel Komparasi Bahasa Lintas Platform</h1>

<table>
  <thead>
    <tr>
      <th>Fitur / Karakteristik</th>
      <th>Widya-Lang v1.0</th>
      <th>Rust 1.80</th>
      <th>Python 3.12</th>
      <th>Go 1.22</th>
      <th>TypeScript / Node</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>Dialek Bahasa Asli</strong></td>
      <td>Bahasa Indonesia (Nusantara)</td>
      <td>Bahasa Inggris</td>
      <td>Bahasa Inggris</td>
      <td>Bahasa Inggris</td>
      <td>Bahasa Inggris</td>
    </tr>
    <tr>
      <td><strong>Mesin Komputasi</strong></td>
      <td>Native Rust VM & Compiler</td>
      <td>LLVM Native Binary</td>
      <td>CPython Bytecode VM</td>
      <td>Go SSA Native Compiler</td>
      <td>V8 JIT Engine</td>
    </tr>
    <tr>
      <td><strong>Jejak Startup Biner</strong></td>
      <td>&lt; 5 ms (Instan)</td>
      <td>&lt; 2 ms (Instan)</td>
      <td>~ 50-100 ms</td>
      <td>&lt; 5 ms (Instan)</td>
      <td>~ 150-250 ms</td>
    </tr>
    <tr>
      <td><strong>Basis Data Tersemat</strong></td>
      <td>WidyaBase 16 Pilar Bawaan</td>
      <td>Membutuhkan Crate Luar</td>
      <td>SQLite Bawaan</td>
      <td>Membutuhkan Driver Cgo</td>
      <td>Membutuhkan Paket NPM</td>
    </tr>
    <tr>
      <td><strong>Dukungan GUI Native</strong></td>
      <td>Tersedia (modul/desktop.wya)</td>
      <td>Pustaka Eksternal Rumit</td>
      <td>Tkinter Lambat</td>
      <td>Eksternal Cgo</td>
      <td>Electron Berat (&gt;150MB)</td>
    </tr>
    <tr>
      <td><strong>Dukungan Model AI Lokal</strong></td>
      <td>WidyaCoder (Ollama Native)</td>
      <td>Candle / Burn bindings</td>
      <td>PyTorch / Transformers</td>
      <td>Ollama REST client</td>
      <td>LangChain / Ollama JS</td>
    </tr>
    <tr>
      <td><strong>Standar Finansial Nasional</strong></td>
      <td>QRIS ASPI EMVCo Terintegrasi</td>
      <td>Tidak Ada Standar</td>
      <td>Tidak Ada Standar</td>
      <td>Tidak Ada Standar</td>
      <td>Tidak Ada Standar</td>
    </tr>
  </tbody>
</table>

<!-- LAMPIRAN C -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">C</span> Lampiran: Direktori Modul Ekosistem Resmi</h1>

<table>
  <thead>
    <tr>
      <th>Path Berkas Modul Resmi</th>
      <th>Fungsi Utama & Tanggung Jawab Komputasi</th>
      <th>Contoh Penggunaan API Kunci</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>modul/widyabase.wya</code></td>
      <td>Mesin basis data relasional slotted-page 4KB, transaksi ACID, WAL, CTE rekursif, dan window functions.</td>
      <td><code>misal db = BukaWidyaBase("data.wdb");</code></td>
    </tr>
    <tr>
      <td><code>modul/desktop.wya</code></td>
      <td>Framework antarmuka grafis desktop native ringan (Windows/Linux) untuk aplikasi kasir & dashboard.</td>
      <td><code>misal app = AplikasiDesktop("Kasir", 800, 600);</code></td>
    </tr>
    <tr>
      <td><code>modul/mobile.wya</code></td>
      <td>Framework mobile sentuh dengan pembacaan kamera QRIS ASPI, printer bluetooth, dan outbox offline.</td>
      <td><code>misal mob = AplikasiMobile("Warung Mobile");</code></td>
    </tr>
    <tr>
      <td><code>modul/dataframe_analitika.wya</code></td>
      <td>Struktur data tabel kolumnar bervolume tinggi, operasi pembersihan missing values, agregasi GroupBy.</td>
      <td><code>misal df = MuatDataframeCSV("omzet.csv");</code></td>
    </tr>
    <tr>
      <td><code>modul/vektor_hnsw.wya</code></td>
      <td>Indeks pencarian semantik vektor kemiripan kosinus untuk arsitektur Retrieval-Augmented Generation (RAG).</td>
      <td><code>misal idx = IndeksVektorHNSW(128);</code></td>
    </tr>
    <tr>
      <td><code>modul/kripto_aman.wya</code></td>
      <td>Kriptografi kelas militer: AES-256-GCM, kurva Ed25519, hashing Argon2id, dan Post-Quantum Kyber.</td>
      <td><code>misal enc = EnkripsiAES256GCM(teks, kunci);</code></td>
    </tr>
    <tr>
      <td><code>modul/http_server.wya</code></td>
      <td>Server HTTP mikroservis berkecepatan tinggi dengan router RESTful dan penangan JSON otomatis.</td>
      <td><code>misal srv = BuatServerHTTP("0.0.0.0", 8080);</code></td>
    </tr>
  </tbody>
</table>

<!-- LAMPIRAN D -->
<div class="page-break"></div>
<h1 class="chapter-title"><span class="chapter-number">D</span> Lampiran: Diagnostik Galat Compiler & Solusi Perbaikan</h1>

<table>
  <thead>
    <tr>
      <th>Kode Galat</th>
      <th>Pesan Diagnostik Kompiler</th>
      <th>Akar Penyebab Permasalahan</th>
      <th>Tindakan Solusi Pemulihan Kode</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>W001</strong></td>
      <td>Tanda titik koma (<code>;</code>) tidak ditemukan di akhir baris</td>
      <td>Lupa menutup pernyataan perintah komputasi</td>
      <td>Tambahkan karakter <code>;</code> pada akhir baris instruksi.</td>
    </tr>
    <tr>
      <td><strong>W002</strong></td>
      <td>Simbol pengenal variabel belum dideklarasikan</td>
      <td>Mengakses variabel sebelum dideklarasikan dengan <code>misal</code></td>
      <td>Pastikan mendeklarasikan variabel dengan <code>misal nama = nilai;</code> sebelum diakses.</td>
    </tr>
    <tr>
      <td><strong>W004</strong></td>
      <td>Upaya mutasi pada tetapan abadi terlarang</td>
      <td>Mencoba mengubah nilai variabel yang dideklarasikan dengan <code>tetap</code></td>
      <td>Ganti kata kunci deklarasi menjadi <code>misal</code> jika nilai memang perlu berubah.</td>
    </tr>
    <tr>
      <td><strong>W008</strong></td>
      <td>Indeks deret berada di luar batas alokasi memori</td>
      <td>Mengakses elemen deret dengan indeks &gt;= panjang deret</td>
      <td>Periksa panjang koleksi dengan fungsi <code>panjang(deret)</code> sebelum mengakses indeks.</td>
    </tr>
    <tr>
      <td><strong>W015</strong></td>
      <td>Kueri WidyaBase melanggar kendala kunci asing</td>
      <td>Menyisipkan ID relasi yang tidak terdaftar di tabel induk</td>
      <td>Pastikan entitas pada tabel rujukan telah terdaftar sebelum menyisipkan baris anak.</td>
    </tr>
    <tr>
      <td><strong>W021</strong></td>
      <td>Checksum CRC-16 QRIS tidak sesuai dengan payload</td>
      <td>String QRIS mengalami korupsi atau manipulasi data saat pemindaian</td>
      <td>Minta pembeli melakukan scan ulang kode QRIS dari aplikasi perbankan resmi.</td>
    </tr>
  </tbody>
</table>

<div class="callout callout-tip" style="margin-top: 30px;">
  <div class="callout-title">🇮🇩 Penutup: Menyongsong Masa Depan Teknologi Berdaulat</div>
  Dengan selesainya buku panduan resmi jilid 2 ini, Anda telah menguasai seluruh spektrum rekayasa perangkat lunak modern: dari algoritma dasar Nusantara, rekayasa desktop kasir UMKM, transaksi pembayaran QRIS nasional, basis data mandiri WidyaBase, hingga orkestrasi agen AI lokal tanpa ketergantungan asing. Selamat berkarya, berinovasi, dan mengibarkan kedaulatan teknologi Indonesia Emas 2045!
</div>

</body>
</html>
`
};
