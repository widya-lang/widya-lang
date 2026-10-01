// =========================================================================
// Widya-Lang Documentation Engine (docs/js/docs.js)
// Search Indexing, Interactive Deep Linking, & Code Copying
// =========================================================================

const searchDatabase = [
  { id: "intro", title: "1.1 Pengantar Widya-Lang", desc: "Arsitektur, filosofi kedaulatan teknologi, dan sistem tipe Hindley-Milner", group: "Memulai" },
  { id: "install", title: "1.2 Instalasi Multi-Platform", desc: "Panduan pasang Windows PowerShell, Linux, macOS, dan verifikasi biner", group: "Memulai" },
  { id: "cli", title: "1.3 Panduan Perintah CLI", desc: "widya run, compile, audit, profile, lsp, dap, dan bootstrap", group: "Memulai" },
  { id: "syntax", title: "2.1 Sintaks Dasar & Dwibahasa", desc: "Kata kunci Nusantara (fungsi, misal, jika) & standar Internasional", group: "Widya Core" },
  { id: "types", title: "2.2 Sistem Tipe Hindley-Milner", desc: "Inferensi otomatis, Aljabar Tipe Data (ADT), dan penanganan nil/null", group: "Widya Core" },
  { id: "generics", title: "2.3 Generics & Monomorphization", desc: "Fungsi parametrik tanpa overhead performa dan kompilasi langsung", group: "Widya Core" },
  { id: "pattern-matching", title: "2.4 Pencocokan Pola (cocokkan)", desc: "Destructuring enum pilihan dan jaminan kehandalan exhaustiveness", group: "Widya Core" },
  { id: "memory", title: "2.5 Manajemen Memori Aman", desc: "Safe Reference Counting tanpa jeda Garbage Collector besar dan siklus Tarjan", group: "Widya Core" },
  { id: "web-intro", title: "3.1 Arsitektur WidyaWeb", desc: "Pondasi 6-Tier Enterprise, HTTP/2, dan performa tinggi", group: "WidyaWeb" },
  { id: "web-routing", title: "3.2 Routing & Restful API", desc: "Pemetaan rute GET, POST, PUT, DELETE, dan parser JSON payload", group: "WidyaWeb" },
  { id: "web-sse", title: "3.3 Server-Sent Events (SSE) & WebSocket", desc: "Koneksi stream real-time dan saluran broadcast dua arah", group: "WidyaWeb" },
  { id: "web-security", title: "3.4 Middleware & Pertahanan Keamanan", desc: "Token anti-CSRF, Strict Origin, dan Session Hijacking Protection", group: "WidyaWeb" },
  { id: "desktop-intro", title: "4.1 Arsitektur WidyaDesktop", desc: "Hybrid IPC Native Bridge berlatensi <1ms dan antarmuka web mandiri", group: "WidyaDesktop" },
  { id: "desktop-window", title: "4.2 Jendela & Baki Sistem", desc: "Manajemen siklus hidup jendela, menu bilah, dan notifikasi tray OS", group: "WidyaDesktop" },
  { id: "desktop-multiwin", title: "4.3 Multi-Window & Modal Dialogs", desc: "Konfigurasi ChildWindowConfig dan dialog sekunder terisolasi", group: "WidyaDesktop" },
  { id: "desktop-clipboard", title: "4.4 Papan Klip & Drag-and-Drop", desc: "Akses papan klip OS dan penanganan berkas dropzone native", group: "WidyaDesktop" },
  { id: "desktop-esc-pos", title: "4.5 Cetak Struk Kasir Thermal POS", desc: "Generator biner ESC/POS standar industri dengan pemotong kertas", group: "WidyaDesktop" },
  { id: "desktop-crypto", title: "4.6 Brankas Kripto AES-256-GCM", desc: "Enkripsi terotentikasi dan pipeline update digital signature Ed25519", group: "WidyaDesktop" },
  { id: "mobile-intro", title: "5.1 Arsitektur WidyaMobile & Tri-Engine", desc: "Kompilasi multi-target Android APK, iOS IPA, dan PWA Offline-First", group: "WidyaMobile" },
  { id: "mobile-hardware", title: "5.2 GPS, Geofence & Sensor Gerak", desc: "Haversine distance, geofencing presisi, akselerometer 3-axis, dan haptik", group: "WidyaMobile" },
  { id: "mobile-qris", title: "5.3 Kamera & Decoder QRIS ASPI", desc: "Snapshot foto, perekaman video, dan parser standar QRIS Bank Indonesia", group: "WidyaMobile" },
  { id: "mobile-pos", title: "5.4 Bluetooth Thermal POS & Struk ESC/POS", desc: "Discovery BLE/Classic, pairing otomatis, dan cetak struk kasir mini UMKM", group: "WidyaMobile" },
  { id: "mobile-offline", title: "5.5 Outbox Offline & SQLite Terenkripsi", desc: "Antrean transaksi saat offline dan basis data lokal terenkripsi SQLCipher", group: "WidyaMobile" },
  { id: "security-lock", title: "6.1 Integritas widya_security.lock", desc: "Penguncian SHA-256 seluruh aset frontend dan backend rilis", group: "Keamanan" },
  { id: "security-audit", title: "6.2 Audit Keamanan & Kesiapan Grade A+", desc: "Pemeriksaan otomatis skor Grade A+ melalui perintah widya mobile periksa", group: "Keamanan" },
  { id: "base-storage", title: "7.1 Slotted-Page & WAL 4KB", desc: "Arsitektur storage engine slotted-page, buffer pool, dan crash recovery", group: "WidyaBase" },
  { id: "base-sql", title: "7.2 SQL Versi Bahasa Indonesia", desc: "Kueri dwibahasa PILIH DARI DIMANA dan padanan ANSI SQL", group: "WidyaBase" },
  { id: "base-pgwire", title: "7.3 pgwire Daemon & Multi-Modal", desc: "Protokol PostgreSQL v3.0, koneksi DBeaver/psql, dan vektor HNSW AI", group: "WidyaBase" }
];

// Document Section Contents Dictionary
const docSections = {
  "intro": {
    group: "Memulai",
    title: "1.1 Pengantar Widya-Lang",
    lead: "Widya-Lang adalah bahasa pemrograman modern berkinerja tinggi yang menggabungkan inferensi tipe Hindley-Milner, Full Self-Hosting, dan ekosistem terintegrasi untuk Web dan Desktop Native.",
    html: `
      <h2>Visi & Kedaulatan Teknologi</h2>
      <p>Widya-Lang dirancang untuk memecahkan dilema umum dalam rekayasa perangkat lunak: memilih antara kecepatan eksekusi tingkat rendah (seperti C/Rust) atau kenyamanan produktivitas tingkat tinggi (seperti Python/TypeScript). Dengan Widya-Lang, Anda mendapatkan keduanya tanpa kompromi.</p>
      
      <div class="callout callout-info">
        <div class="callout-title">💡 Karakteristik Utama</div>
        Widya-Lang mengadopsi sintaks dwibahasa (Nusantara & Internasional). Anda dapat menulis program menggunakan kata kunci berbahasa Indonesia (seperti <code>fungsi</code>, <code>misal</code>, <code>jika</code>) maupun kata kunci standar internasional (<code>fn</code>, <code>let</code>, <code>if</code>).
      </div>

      <h2>Arsitektur Tiga Lapis</h2>
      <ul>
        <li><strong>Widya Core Engine</strong>: Kompilator tipe statis berbasis Hindley-Milner, monomorphization generics, multi-target codegen (Native ELF/EXE, WebAssembly, eBPF, WGSL).</li>
        <li><strong>WidyaWeb Framework</strong>: Kerangka kerja server web kelas industri berkecepatan tinggi dengan 6-Tier Enterprise Hardening (54/54 uji lulus).</li>
        <li><strong>WidyaDesktop Framework</strong>: Kerangka kerja aplikasi desktop native berbasis Hybrid IPC Bridge (&lt;1ms), multi-window, baki sistem, dan generator cetak thermal POS ESC/POS.</li>
      </ul>
    `
  },
  "install": {
    group: "Memulai",
    title: "1.2 Instalasi Multi-Platform",
    lead: "Pasang toolchain kompilator Widya-Lang di sistem operasi Windows, Linux, atau macOS dengan satu perintah.",
    html: `
      <h2>Instalasi Cepat Satu Baris</h2>
      <p>Gunakan skrip instalasi resmi untuk mengunduh biner v1.0.0 dan mengonfigurasi variabel lingkungan sistem secara otomatis:</p>

      <h3>Di Windows (PowerShell):</h3>
      <div class="code-wrapper">
        <div class="code-header">PowerShell <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>iwr https://get.widya-lang.org/ps1 -useb | iex</code></pre>
      </div>

      <h3>Di Linux & macOS (Terminal Bash):</h3>
      <div class="code-wrapper">
        <div class="code-header">Bash <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>curl -fsSL https://get.widya-lang.org | sh</code></pre>
      </div>

      <h2>Verifikasi Instalasi</h2>
      <p>Buka terminal baru dan jalankan perintah verifikasi:</p>
      <div class="code-wrapper">
        <div class="code-header">Terminal <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>widya --version</code></pre>
      </div>
      <p>Keluaran sukses akan menampilkan: <code>widya 1.0.0</code>.</p>
    `
  },
  "cli": {
    group: "Memulai",
    title: "1.3 Panduan Perintah CLI",
    lead: "Widya-Lang menyediakan toolchain baris perintah (CLI) lengkap untuk kompilasi, audit keamanan, inspeksi profil memori, dan scaffold.",
    html: `
      <h2>Daftar Perintah Baris Perintah Utama</h2>
      <table class="doc-table">
        <thead>
          <tr>
            <th>Perintah</th>
            <th>Fungsi & Deskripsi</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td><code>widya run berkas.wya</code></td>
            <td>Mengeksekusi berkas kode Widya secara langsung melalui interpreter native berkecepatan tinggi.</td>
          </tr>
          <tr>
            <td><code>widya desktop inisialisasi &lt;Nama&gt;</code></td>
            <td>Membuat scaffolding proyek aplikasi desktop enterprise baru lengkap dengan struktur UI, backend, dan berkas lock keamanan.</td>
          </tr>
          <tr>
            <td><code>widya desktop bangun &lt;Dir&gt;</code></td>
            <td>Mengompilasi dan mengemas aplikasi desktop menjadi paket distribusi biner mandiri (Windows / Linux).</td>
          </tr>
          <tr>
            <td><code>widya desktop periksa &lt;Dir&gt;</code></td>
            <td>Menjalankan audit keamanan bertingkat dan kalkulasi skor kepatuhan (Grade A+).</td>
          </tr>
          <tr>
            <td><code>widya audit .</code></td>
            <td>Menjalankan static AST security linter untuk mendeteksi celah SQLi, XSS, dan kebocoran API Key.</td>
          </tr>
          <tr>
            <td><code>widya lsp</code></td>
            <td>Menjalankan Language Server Protocol bawaan untuk editor (VS Code, Neovim, Zed).</td>
          </tr>
        </tbody>
      </table>
    `
  },
  "syntax": {
    group: "Widya Core",
    title: "2.1 Sintaks Dasar & Dwibahasa",
    lead: "Pelajari gaya penulisan kode Widya-Lang yang bersih, ekspresif, dan mendukung pilihan bahasa Nusantara maupun internasional.",
    html: `
      <h2>Struktur Program Utama</h2>
      <p>Setiap program dapat ditulis dengan kata kunci lokal maupun internasional:</p>

      <div class="code-wrapper">
        <div class="code-header">Widya-Lang (.wya) <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code><span class="comment">// Deklarasi Variabel & Konstanta</span>
<span class="kw">misal</span> nama = <span class="str">"Widya Developer"</span>;
<span class="kw">tetap</span> VERSI_SISTEM = <span class="num">1.0</span>;

<span class="comment">// Pembuatan Fungsi Sederhana</span>
<span class="kw">fungsi</span> <span class="fn">sapa</span>(pengguna: <span class="type">Teks</span>) -&gt; <span class="type">Teks</span> {
    <span class="kw">kembalikan</span> <span class="str">"Selamat Datang, "</span> + pengguna + <span class="str">"!"</span>;
}

<span class="comment">// Pemanggilan Fungsi & Pencetakan</span>
<span class="kw">misal</span> ucapan = <span class="fn">sapa</span>(nama);
<span class="fn">cetak</span>(ucapan);</code></pre>
      </div>

      <div class="callout callout-tip">
        <div class="callout-title">✨ Padanan Kata Kunci</div>
        <code>misal</code> ≡ <code>let</code> | <code>fungsi</code> ≡ <code>fn</code> | <code>kembalikan</code> ≡ <code>return</code> | <code>jika</code> ≡ <code>if</code> | <code>lainnya</code> ≡ <code>else</code> | <code>cetak</code> ≡ <code>print</code>.
      </div>
    `
  },
  "types": {
    group: "Widya Core",
    title: "2.2 Sistem Tipe Hindley-Milner",
    lead: "Inferensi tipe otomatis tanpa boilerplate anotasi manual, Aljabar Tipe Data (ADT), dan jaminan keamanan tipe kompilasi.",
    html: `
      <h2>Inferensi Tipe Otomatis</h2>
      <p>Kompilator Widya secara otomatis mendeduksi tipe variabel dan nilai kembalian fungsi menggunakan algoritma matematis unifikasi Hindley-Milner:</p>

      <div class="code-wrapper">
        <div class="code-header">Hindley-Milner Inference <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code><span class="comment">// Tipe diputuskan secara otomatis sebagai Float</span>
<span class="kw">fungsi</span> <span class="fn">kali</span>(a, b) {
    <span class="kw">kembalikan</span> a * b;
}

<span class="kw">misal</span> hasil = <span class="fn">kali</span>(<span class="num">12.5</span>, <span class="num">4.0</span>); <span class="comment">// Inferred: Float = 50.0</span></code></pre>
      </div>

      <h2>Aljabar Tipe Data (ADT / Enum)</h2>
      <p>Mendefinisikan varian data yang kaya dengan muatan nilai (*tagged union*):</p>
      <div class="code-wrapper">
        <div class="code-header">Aljabar Tipe Data <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code><span class="kw">pilihan</span> <span class="type">StatusTransaksi</span> {
    <span class="type">Menunggu</span>,
    <span class="type">Sukses</span>(<span class="type">Teks</span>),
    <span class="type">Gagal</span>(<span class="type">Int</span>, <span class="type">Teks</span>)
}</code></pre>
      </div>
    `
  },
  "pattern-matching": {
    group: "Widya Core",
    title: "2.4 Pencocokan Pola (cocokkan)",
    lead: "Pencocokan pola yang kuat, ekspresif, dan diperiksa secara ketat saat kompilasi.",
    html: `
      <h2>Struktur ` + "`" + `cocokkan` + "`" + `</h2>
      <p>Pencocokan pola memeriksa setiap varian tipe data dan mengekstrak nilai di dalamnya secara aman:</p>

      <div class="code-wrapper">
        <div class="code-header">Pattern Matching <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code><span class="kw">misal</span> transaksi = <span class="type">StatusTransaksi</span>::<span class="type">Sukses</span>(<span class="str">"INV-2026-001"</span>);

<span class="kw">cocokkan</span> transaksi {
    <span class="type">StatusTransaksi</span>::<span class="type">Sukses</span>(nomor) =&gt; {
        <span class="fn">cetak</span>(<span class="str">"Transaksi Berhasil: "</span> + nomor);
    },
    <span class="type">StatusTransaksi</span>::<span class="type">Gagal</span>(kode, pesan) =&gt; {
        <span class="fn">cetak</span>(<span class="str">"Transaksi Gagal ["</span> + <span class="fn">ke_teks</span>(kode) + <span class="str">"]: "</span> + pesan);
    },
    <span class="type">StatusTransaksi</span>::<span class="type">Menunggu</span> =&gt; {
        <span class="fn">cetak</span>(<span class="str">"Menunggu pembayaran dari pelanggan..."</span>);
    }
}</code></pre>
      </div>
    `
  },
  "web-intro": {
    group: "WidyaWeb",
    title: "3.1 Arsitektur WidyaWeb Framework",
    lead: "Kerangka kerja web tingkat industri dengan kehandalan 6 Tier, performa tinggi, dan pertahanan keamanan bawaan.",
    html: `
      <h2>Fondasi 6-Tier Enterprise</h2>
      <p>WidyaWeb telah lulus pengujian komprehensif <strong>54 / 54 Master Test Suites (100% PASS)</strong>:</p>
      <ul>
        <li><strong>Tier 1: Core & Hardening</strong> — Sanitasi HTTP headers, router trie presisi tinggi, dan isolasi worker threads.</li>
        <li><strong>Tier 2: Zero Regression</strong> — Validasi stabilitas beban tinggi tanpa penurunan throughput.</li>
        <li><strong>Tier 3: Enterprise Features</strong> — Dukungan basis data FFI (SQLite, DuckDB, Postgres, Redis), rate-limiting dinamis.</li>
        <li><strong>Tier 4: Advanced Pillars</strong> — Server-Sent Events (SSE), WebSockets streaming, dan kompresi gzip.</li>
        <li><strong>Tier 5: Next-Horizon Capabilities</strong> — Session Hijacking Protection, token CSRF kriptografis, dan auto-reload dev stream.</li>
        <li><strong>Tier 6: Apex Pinnacle Security</strong> — WAF header security, IP allowlist, dan audit skor 100/100.</li>
      </ul>
    `
  },
  "web-routing": {
    group: "WidyaWeb",
    title: "3.2 Routing & Restful API",
    lead: "Membangun endpoint API berkecepatan tinggi dengan sintaks ringkas dan parser JSON otomatis.",
    html: `
      <h2>Contoh Aplikasi Web Sederhana</h2>
      <div class="code-wrapper">
        <div class="code-header">server.wya <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code><span class="kw">impor</span> <span class="str">"./modul/web.wya"</span>;

<span class="kw">misal</span> app = <span class="fn">BuatAplikasiWeb</span>(<span class="num">8080</span>);

<span class="comment">// Endpoint GET Status</span>
app.<span class="fn">rute_get</span>(<span class="str">"/api/health"</span>, <span class="kw">fungsi</span>(req) {
    <span class="kw">kembalikan</span> {
        <span class="str">"status"</span>: <span class="str">"sehat"</span>,
        <span class="str">"versi"</span>: <span class="str">"1.0.0"</span>,
        <span class="str">"waktu"</span>: <span class="str">"2026-09-25T14:00:00Z"</span>
    };
});

<span class="comment">// Endpoint POST Pemrosesan Data</span>
app.<span class="fn">rute_post</span>(<span class="str">"/api/transaksi"</span>, <span class="kw">fungsi</span>(req) {
    <span class="kw">misal</span> data = req.json;
    <span class="kw">kembalikan</span> {
        <span class="str">"sukses"</span>: <span class="kw">benar</span>,
        <span class="str">"id_transaksi"</span>: <span class="str">"TRX-"</span> + <span class="fn">ke_teks</span>(data.total)
    };
});

<span class="fn">cetak</span>(<span class="str">"Server aktif pada port 8080..."</span>);
app.<span class="fn">dengarkan</span>();</code></pre>
      </div>
    `
  },
  "desktop-intro": {
    group: "WidyaDesktop",
    title: "4.1 Arsitektur WidyaDesktop Framework",
    lead: "Kerangka kerja aplikasi desktop native modern yang menggabungkan performa native Rust dengan antarmuka web modern mandiri.",
    html: `
      <h2>Model Arsitektur Hybrid IPC Native Bridge</h2>
      <p>WidyaDesktop tidak menggunakan runtime raksasa yang boros sumber daya, melainkan menghubungkan engine native Widya dengan chromeless application window mandiri:</p>
      <ul>
        <li><strong>Latensi IPC Sub-Milidetik (&lt;1ms)</strong>: Komunikasi JSON-RPC lokal berkecepatan tinggi antara antarmuka dan backend.</li>
        <li><strong>Keamanan Grade A+</strong>: Token sesi IPC dinamis (<code>X-Widya-IPC-Token</code>), Content Security Policy (Strict CSP), Brankas Kripto AES-256-GCM, dan verifikasi lockfile SHA-256 (<code>widya_security.lock</code>).</li>
        <li><strong>Ukuran Distribusi Ramping</strong>: Paket aplikasi mandiri berukuran jauh lebih hemat memori dibanding framework konvensional.</li>
      </ul>
    `
  },
  "desktop-esc-pos": {
    group: "WidyaDesktop",
    title: "4.5 Cetak Struk Kasir Thermal POS ESC/POS",
    lead: "Generator biner langsung untuk printer kasir thermal standar industri (58mm/80mm) dengan pemotong kertas otomatis.",
    html: `
      <h2>Pemformatan Struk Kasir Native</h2>
      <p>WidyaDesktop menghadirkan mesin pemformatan biner langsung tanpa perlu driver printer pihak ketiga yang rumit:</p>

      <div class="code-wrapper">
        <div class="code-header">Cetak Struk Thermal <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code><span class="kw">impor</span> <span class="str">"./modul/desktop.wya"</span>;

<span class="kw">misal</span> data_struk = {
    <span class="str">"nama_toko"</span>: <span class="str">"WIDYA RETAIL STORE"</span>,
    <span class="str">"alamat"</span>: <span class="str">"Jl. Jenderal Sudirman No. 45"</span>,
    <span class="str">"nomor_struk"</span>: <span class="str">"TRX-2026-001"</span>,
    <span class="str">"kasir"</span>: <span class="str">"Kasir 01"</span>,
    <span class="str">"waktu"</span>: <span class="str">"2026-09-25 14:00"</span>,
    <span class="str">"items"</span>: [
        { <span class="str">"nama"</span>: <span class="str">"Lisensi Pro"</span>, <span class="str">"jumlah"</span>: <span class="num">1</span>, <span class="str">"harga"</span>: <span class="num">750000</span>, <span class="str">"total"</span>: <span class="num">750000</span> },
        { <span class="str">"nama"</span>: <span class="str">"Dongle USB"</span>, <span class="str">"jumlah"</span>: <span class="num">2</span>, <span class="str">"harga"</span>: <span class="num">125000</span>, <span class="str">"total"</span>: <span class="num">250000</span> }
    ],
    <span class="str">"subtotal"</span>: <span class="num">1000000</span>,
    <span class="str">"pajak"</span>: <span class="num">110000</span>,
    <span class="str">"total"</span>: <span class="num">1110000</span>,
    <span class="str">"metode_bayar"</span>: <span class="str">"QRIS"</span>,
    <span class="str">"catatan_kaki"</span>: <span class="str">"Terima kasih atas kunjungan Anda!"</span>
};

<span class="comment">// Hasilkan biner ESC/POS dan kirim ke printer thermal</span>
<span class="kw">misal</span> biner_buffer = <span class="fn">hasilkan_raw_esc_pos</span>(data_struk);
<span class="fn">cetak</span>(<span class="str">"Ukuran biner siap cetak: "</span> + <span class="fn">ke_teks</span>(biner_buffer.<span class="fn">panjang</span>()) + <span class="str">" bytes"</span>);</code></pre>
      </div>

      <div class="callout callout-tip">
        <div class="callout-title">Opcode Biner yang Dihasilkan</div>
        Mencakup inisialisasi printer (<code>ESC @</code>), perataan teks tengah/kanan (<code>ESC a</code>), huruf tebal (<code>ESC E 1</code>), dan perintah pemotong kertas otomatis (<code>GS V 66 0</code>).
      </div>
    `
  },
  "desktop-crypto": {
    group: "WidyaDesktop",
    title: "4.6 Brankas Kripto AES-256-GCM & Update Ed25519",
    lead: "Perlindungan data lokal tingkat militer dan verifikasi integritas pembaruan biner otomatis.",
    html: `
      <h2>Brankas Kripto Terotentikasi (AES-256-GCM)</h2>
      <p>Melindungi data kredensial atau rahasia perusahaan langsung di sistem berkas pengguna:</p>
      <div class="code-wrapper">
        <div class="code-header">Enkripsi & Dekripsi AES-256-GCM <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code><span class="comment">// Dari frontend via WidyaBridge</span>
<span class="kw">const</span> enc = <span class="kw">await</span> window.widya.<span class="fn">enkripsi</span>(<span class="str">"Data Rahasia 2026"</span>, <span class="str">"KunciMaster32Byte!"</span>);
<span class="comment">// Hasil: { ciphertext: "...", nonce: "...", algoritma: "AES-256-GCM" }</span>

<span class="kw">const</span> dec = <span class="kw">await</span> window.widya.<span class="fn">dekripsi</span>(enc.ciphertext, enc.nonce, <span class="str">"KunciMaster32Byte!"</span>);
<span class="comment">// Hasil: { plaintext: "Data Rahasia 2026", terverifikasi: true }</span></code></pre>
      </div>

      <h2>Tanda Tangan Digital Pembaruan (Ed25519)</h2>
      <p>Setiap paket rilis divalidasi dengan tanda tangan digital Ed25519 untuk menggagalkan upaya pembajakan paket di jaringan publik (*Man-in-the-Middle*).</p>
    `
  },
  "mobile-intro": {
    group: "WidyaMobile",
    title: "5.1 Arsitektur WidyaMobile & Tri-Engine",
    lead: "WidyaMobile menyatukan kompilasi multi-target Android (Kotlin), iOS (Swift), dan Standalone PWA Offline-First dengan standar Grade A+.",
    html: `
      <h2>Arsitektur 10 Pilar Native Mobile</h2>
      <p>WidyaMobile dirancang dari awal untuk aplikasi seluler enterprise di Indonesia:</p>
      <ul>
        <li><strong>Pilar 1: Perangkat Keras Seluler</strong> (GPS, Geofence, Akselerometer, Baterai, Haptik)</li>
        <li><strong>Pilar 2: UI Seluler Industri & Dynamic Notch Simulator</strong> (Safe-Area Insets, Bilah Navigasi, FAB, Lencana, Sakelar)</li>
        <li><strong>Pilar 3: Keamanan & Biometrik</strong> (FaceID, Sidik Jari, Brankas Kunci Hardware KeyStore)</li>
        <li><strong>Pilar 4: PWA Offline-First & Mesin Audit</strong> (Manifest v2, Service Worker Cache-First, CLI <code>widya mobile periksa</code>)</li>
        <li><strong>Pilar 5: Kamera & Scanner QRIS Bank Indonesia</strong> (Decoder EMVCo/ASPI TLV otomatis)</li>
        <li><strong>Pilar 6: Notifikasi Lokal & Push APNs/FCM</strong> (Notifikasi terjadwal, badge counter)</li>
        <li><strong>Pilar 7: Antrean Offline Outbox & Network Sync</strong> (Pencatatan transaksi saat jaringan terputus)</li>
        <li><strong>Pilar 8: Mobile Bluetooth & Printer Thermal POS</strong> (Cetak struk kasir mini 58mm/80mm ESC/POS)</li>
        <li><strong>Pilar 9: Native Share Sheet, Deep Links & Papan Klip</strong> (Berbagi nota/faktur, App Links <code>widya://</code>)</li>
        <li><strong>Pilar 10: Basis Data Terenkripsi Lokal</strong> (Mobile SQLite / SQLCipher Cache AES-256)</li>
      </ul>
      <div class="code-wrapper">
        <div class="code-header">Kompilasi CLI Multi-Platform <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code># Bangun paket PWA Offline-First
widya mobile bangun src/main.wya --target webview --output dist_pwa/

# Bangun paket Android APK
widya mobile bangun src/main.wya --target android --output build_android/

# Bangun paket iOS IPA
widya mobile bangun src/main.wya --target ios --output build_ios/</code></pre>
      </div>
    `
  },
  "mobile-hardware": {
    group: "WidyaMobile",
    title: "5.2 GPS, Geofence & Sensor Gerak",
    lead: "Telemetri posisi akurasi tinggi, geofencing berbasis formula Haversine, sensor gerak 3-axis, dan umpan balik haptik.",
    html: `
      <h2>Contoh Akses Hardware Native</h2>
      <div class="code-wrapper">
        <div class="code-header">Widya-Lang <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>impor "modul/mobile.wya";

misal app = BuatAplikasiMobile("KurirKilat", "indigo");

// GPS & Geofencing
misal pos = app.ambil_posisi_gps();
misal jarak = app.hitung_jarak_ke(-6.1754, 106.8272);
misal di_toko = app.apakah_dalam_geofence(-6.2088, 106.8456, 1500.0);

// Sensor & Haptik
misal vec = app.baca_sensor();
app.getar("sukses");

// Status Baterai
misal bat = app.status_baterai();
cetak("Baterai: " + string(bat.level) + "% | Suhu: " + string(bat.suhu) + "°C");</code></pre>
      </div>
    `
  },
  "mobile-qris": {
    group: "WidyaMobile",
    title: "5.3 Kamera & Decoder QRIS ASPI",
    lead: "Integrasi kamera snapshot & perekaman video, serta pemindai dan parser standar QRIS Bank Indonesia (EMVCo/ASPI).",
    html: `
      <h2>Penguraian Otomatis QRIS Nasional</h2>
      <p>Mesin QRIS bawaan Widya mengurai Tag-Length-Value (TLV) QRIS standar ASPI secara instan tanpa pustaka eksternal:</p>
      <div class="code-wrapper">
        <div class="code-header">Widya-Lang <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>misal qr = app.pindai_qr();
misal qris = app.urai_qris(qr.konten);

cetak("Merchant : " + qris.nama_merchant);
cetak("Kota     : " + qris.kota);
cetak("Nominal  : " + qris.mata_uang + " " + string(qris.nominal));
cetak("Tipe QR  : " + qris.tipe_qr); // Dinamis / Statis
cetak("NMID     : " + qris.nmid);
cetak("CRC Valid: " + string(qris.crc_valid));</code></pre>
      </div>
    `
  },
  "mobile-pos": {
    group: "WidyaMobile",
    title: "5.4 Bluetooth Thermal POS & Struk ESC/POS",
    lead: "Pencarian perangkat Bluetooth Low Energy (BLE) & Classic, pairing printer thermal struk mini UMKM, dan format ESC/POS.",
    html: `
      <h2>Pencetakan Struk Kasir Lapangan</h2>
      <div class="code-wrapper">
        <div class="code-header">Widya-Lang <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>misal dev = app.pindai_bluetooth();
app.hubungkan_bluetooth("66:32:B1:88:AC:21");

misal cetak_struk = app.cetak_struk({
    "nama_toko": "WARUNG KOPI WIDYA",
    "alamat": "Jl. Sudirman Kav. 21, Jakarta",
    "nomor_struk": "TRX-2026-0925-001",
    "total": 45000.0,
    "metode_bayar": "QRIS Dinamis"
});
cetak("Status Cetak: " + string(cetak_struk.sukses));
cetak("Bytes Dikirim: " + string(cetak_struk.bytes_terkirim));</code></pre>
      </div>
    `
  },
  "mobile-offline": {
    group: "WidyaMobile",
    title: "5.5 Outbox Offline & SQLite Terenkripsi",
    lead: "Jaminan kehandalan transaksi saat koneksi terputus dengan Antrean Outbox otomatis dan basis data lokal terenkripsi SQLCipher.",
    html: `
      <h2>Antrean Transaksi Offline & SQLite Enkripsi</h2>
      <div class="code-wrapper">
        <div class="code-header">Widya-Lang <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>// 1. Deteksi Jaringan & Catat ke Outbox
misal net = app.status_jaringan();
jika (!net.aktif) {
    app.tambah_antrean_offline("ORDER_KASIR", { "total": 45000, "item": "Kopi Susu" });
} selain {
    app.sinkronkan_offline();
}

// 2. Basis Data Mobile Terenkripsi Zero-Trust
misal db = app.buka_db("kasir_lokal.db", "KUNCI_AES256_DARI_KEYSTORE");
app.eksekusi_sql(db, "CREATE TABLE pesanan (id INT, nota TEXT, total REAL)");
app.eksekusi_sql(db, "INSERT INTO pesanan VALUES (1, 'TRX-001', 45000)", [1, "TRX-001", 45000]);
misal rows = app.kueri_sql(db, "SELECT * FROM pesanan");</code></pre>
      </div>
    `
  },
  "security-lock": {
    group: "Keamanan",
    title: "6.1 Integritas widya_security.lock",
    lead: "Sistem penguncian hash kriptografis SHA-256 untuk mendeteksi manipulasi berkas secara real-time.",
    html: `
      <h2>Penguncian Aset Otomatis</h2>
      <p>Saat perintah <code>widya desktop bangun</code> atau <code>widya mobile bangun</code> dijalankan, compiler menghitung ringkasan SHA-256 dari seluruh berkas kode sumber dan antarmuka, menyimpannya di dalam <code>widya_security.lock</code>:</p>
      
      <div class="code-wrapper">
        <div class="code-header">widya_security.lock <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>{
  "skema": "widya_security_lock_v1",
  "algoritma": "SHA-256",
  "total_berkas": 8,
  "berkas": {
    "manifest.webmanifest": "...",
    "sw.js": "...",
    "src/main.wya": "..."
  }
}</code></pre>
      </div>
      <p>Jika ada berkas yang diubah oleh malware di disk pengguna, runtime akan menolak menyajikannya dan memicu peringatan <em>Tamper Detected</em>.</p>
    `
  },
  "security-audit": {
    group: "Keamanan",
    title: "6.2 Audit Keamanan & Kesiapan Grade A+",
    lead: "Mesin audit otomatis bawaan untuk mengevaluasi kesiapan arsitektur, izin perangkat, integritas manifest, dan skor kepatuhan.",
    html: `
      <h2>Perintah Audit CLI</h2>
      <div class="code-wrapper">
        <div class="code-header">Terminal <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>widya mobile periksa src/main.wya</code></pre>
      </div>
      <p>Keluaran laporan audit:</p>
      <div class="code-wrapper">
        <div class="code-header">Audit Report <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>==================================================
  LAPORAN AUDIT & KESIAPAN APLIKASI SELULER WIDYA
==================================================
Nama Aplikasi      : KasirPintarMobile
Versi              : 1.0.0
Entri Utama        : src/main.wya (Ditemukan)
PWA Offline Ready  : Aktif (manifest.webmanifest & sw.js)
Target Platform    : android, ios, webview
Izin Hardware      : ACCESS_FINE_LOCATION, CAMERA, BLUETOOTH, USE_BIOMETRIC

--------------------------------------------------
SKOR KELAYAKAN     : 100 / 100
PREDIKAT MUTU      : Enterprise Grade (A+)
STATUS AUDIT       : LULUS PENUH (SIAP SUBMIT TO STORE)
==================================================</code></pre>
      </div>
    `
  },
  "base-storage": {
    group: "WidyaBase Engine",
    title: "7.1 Slotted-Page Storage & WAL 4KB",
    lead: "Arsitektur penyimpanan mandiri berbasis halaman 4096-byte (4KB) dengan slot directory dan logging write-ahead log (WAL) untuk ketahanan ACID penuh.",
    html: `
      <h2>Struktur Halaman 4096 Byte</h2>
      <p>Setiap halaman data WidyaBase memiliki ukuran tetap 4096 byte dengan header 32-byte, direktori slot yang bertumbuh ke bawah, dan area record yang bertumbuh ke atas:</p>
      
      <div class="callout callout-success">
        <div class="callout-title">🛡️ Ketahanan Transaksi ACID</div>
        <p>Setiap perubahan dicatat terlebih dahulu ke berkas WAL siklik (.wal) dengan nomor urut log (LSN) sebelum halaman kotor disiram ke disk oleh Checkpointer.</p>
      </div>

      <h2>Inisialisasi Database via CLI</h2>
      <div class="code-wrapper">
        <div class="code-header">Terminal <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code># Inisialisasi basis data baru
widya base inisialisasi data/kasir.wdb

# Periksa status kesehatan & jumlah halaman
widya base periksa data/kasir.wdb</code></pre>
      </div>
    `
  },
  "base-sql": {
    group: "WidyaBase Engine",
    title: "7.2 SQL Versi Bahasa Indonesia",
    lead: "Tulis kueri basis data menggunakan Bahasa Indonesia yang intuitif atau Standar ANSI Global secara simetris dengan AST kanonikal yang 100% identik.",
    html: `
      <h2>Tabel Padanan Kata Kunci Kueri</h2>
      <table class="doc-table">
        <thead><tr><th>SQL Bahasa Indonesia</th><th>SQL Global</th><th>Contoh Penggunaan</th></tr></thead>
        <tbody>
          <tr><td><code>PILIH</code></td><td><code>SELECT</code></td><td><code>PILIH nama, harga</code></td></tr>
          <tr><td><code>DARI</code></td><td><code>FROM</code></td><td><code>DARI produk</code></td></tr>
          <tr><td><code>DIMANA</code></td><td><code>WHERE</code></td><td><code>DIMANA harga >= 25000</code></td></tr>
          <tr><td><code>SISIP_KE ... NILAI</code></td><td><code>INSERT INTO ... VALUES</code></td><td><code>SISIP_KE kasir (id, total) NILAI (1, 50000)</code></td></tr>
          <tr><td><code>PERBARUI ... ATUR</code></td><td><code>UPDATE ... SET</code></td><td><code>PERBARUI produk ATUR stok = 10</code></td></tr>
          <tr><td><code>HAPUS_DARI</code></td><td><code>DELETE FROM</code></td><td><code>HAPUS_DARI cart DIMANA id = 5</code></td></tr>
          <tr><td><code>BUAT_TABEL</code></td><td><code>CREATE TABLE</code></td><td><code>BUAT_TABEL akun (id INT KUNCI_UTAMA)</code></td></tr>
        </tbody>
      </table>

      <h2>Contoh Kueri Analitik Dwibahasa</h2>
      <div class="code-wrapper">
        <div class="code-header">Kueri Nusantara <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>PILIH 
    kategori,
    HITUNG(*) SEBAGAI total_trx,
    JUMLAH(total) SEBAGAI omzet,
    RATA_RATA(total) SEBAGAI belanja_rata_rata
DARI pesanan
DIMANA tanggal >= '2026-09-01'
KELOMPOKKAN BERDASARKAN kategori
URUTKAN BERDASARKAN omzet TURUN
BATAS 5;</code></pre>
      </div>
    `
  },
  "base-pgwire": {
    group: "WidyaBase Engine",
    title: "7.3 pgwire Daemon & Multi-Modal",
    lead: "Jalankan server database mandiri berbasis protokol PostgreSQL v3.0 dan hubungkan dengan DBeaver, psql, atau Prisma ORM secara instan.",
    html: `
      <h2>Menjalankan Server Daemon pgwire</h2>
      <div class="code-wrapper">
        <div class="code-header">Terminal <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code># Jalankan daemon di port standar PostgreSQL 5432
widya base mulai --port 5432 --data data/kasir.wdb

# Atau buka terminal interaktif REPL
widya base sql --data data/kasir.wdb</code></pre>
      </div>

      <h2>Koneksi dari Alat Standar Industri</h2>
      <p>Karena WidyaBase mengimplementasikan wire protocol PostgreSQL v3.0, Anda dapat terhubung langsung via psql atau GUI DBeaver:</p>
      <div class="code-wrapper">
        <div class="code-header">psql Connection <button class="copy-code-btn" onclick="copySnippet(this)">Salin</button></div>
        <pre><code>psql -h 127.0.0.1 -p 5432 -U admin -d kasir_db</code></pre>
      </div>

      <h2>Multi-Modal AI (Vektor HNSW Cosine Similarity)</h2>
      <p>WidyaBase menyediakan mesin kalkulasi kemiripan vektor embedding untuk aplikasi AI, RAG, dan pencarian semantik berkinerja sub-milidetik.</p>
    `
  }
};

// State
let activeSectionId = "intro";

function renderSection(sectionId) {
  const data = docSections[sectionId] || docSections["intro"];
  activeSectionId = sectionId;

  // Update Breadcrumbs
  document.getElementById("bcGroup").innerText = data.group;
  document.getElementById("bcCurrent").innerText = data.title;

  // Update Main Article
  document.getElementById("docArticle").innerHTML = `
    <h1>${data.title}</h1>
    <p class="lead-p">${data.lead}</p>
    ${data.html}
  `;

  // Update Active Link in Sidebar
  document.querySelectorAll(".nav-item").forEach(item => {
    item.classList.remove("active");
    if (item.getAttribute("data-id") === sectionId) {
      item.classList.add("active");
    }
  });

  // Update Hash in URL
  window.location.hash = sectionId;
  window.scrollTo({ top: 0, behavior: 'smooth' });

  // Close mobile sidebar if open
  document.getElementById("sidebar").classList.remove("open");
}

// Copy Code Button Helper
function copySnippet(btn) {
  const pre = btn.closest(".code-wrapper").querySelector("pre code");
  navigator.clipboard.writeText(pre.innerText);
  const orig = btn.innerText;
  btn.innerText = "Tersalin! ✓";
  btn.style.color = "var(--accent)";
  setTimeout(() => {
    btn.innerText = orig;
    btn.style.color = "";
  }, 1500);
}

// Live Search Handling
function handleSearch(query) {
  const resultsBox = document.getElementById("searchResults");
  const q = query.toLowerCase().trim();
  if (!q) {
    resultsBox.style.display = "none";
    return;
  }

  const matches = searchDatabase.filter(item => 
    item.title.toLowerCase().includes(q) || 
    item.desc.toLowerCase().includes(q) ||
    item.group.toLowerCase().includes(q)
  );

  if (matches.length === 0) {
    resultsBox.innerHTML = '<div style="padding: 1rem; color: var(--text-dim); text-align: center; font-size: 0.85rem;">Tidak ada topik yang cocok.</div>';
    resultsBox.style.display = "block";
    return;
  }

  resultsBox.innerHTML = matches.map(m => `
    <div class="search-item" onclick="selectSearchResult('${m.id}')">
      <div class="search-item-title">${m.title}</div>
      <div class="search-item-desc">${m.group} • ${m.desc}</div>
    </div>
  `).join("");

  resultsBox.style.display = "block";
}

function selectSearchResult(id) {
  document.getElementById("searchInput").value = "";
  document.getElementById("searchResults").style.display = "none";
  renderSection(id);
}

// Mobile Sidebar Toggle
function toggleSidebar() {
  document.getElementById("sidebar").classList.toggle("open");
}

// Keyboard Shortcut: press '/' to focus search
window.addEventListener("keydown", (e) => {
  if (e.key === "/" && document.activeElement.id !== "searchInput") {
    e.preventDefault();
    document.getElementById("searchInput").focus();
  } else if (e.key === "Escape") {
    document.getElementById("searchResults").style.display = "none";
  }
});

// Close search when clicking outside
window.addEventListener("click", (e) => {
  if (!e.target.closest(".search-container")) {
    document.getElementById("searchResults").style.display = "none";
  }
});

// Hash Routing Init
window.addEventListener("DOMContentLoaded", () => {
  const hash = window.location.hash.replace("#", "");
  if (hash && docSections[hash]) {
    renderSection(hash);
  } else {
    renderSection("intro");
  }
});
