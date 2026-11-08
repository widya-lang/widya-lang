# LAMPIRAN A: GLOSARIUM & REFERENSI SINTAKSIS WIDYAWEB

Lampiran ini merangkum istilah-istilah teknis penting dalam rekayasa web modern serta ikhtisar sintaksis deklaratif, direktif templat, dan pustaka inti kerangka kerja WidyaWeb.

---

## A.1 Glosarium Rekayasa Web Enterprise

- **Adaptive Bitrate Streaming (ABR)**: Teknik streaming video di mana kualitas video (resolusi dan bitrate) disesuaikan secara dinamis mengikuti kapasitas bandwidth koneksi internet klien secara seketika.
- **Change Data Capture (CDC)**: Pola integrasi data perangkat lunak di mana setiap mutasi baris data (INSERT, UPDATE, DELETE) ditangkap langsung dari berkas jurnal transaksi (*Write-Ahead Log*) basis data dan dipancarkan sebagai aliran peristiwa (*event stream*).
- **Circuit Breaker**: Pola desain resiliensi yang memutus koneksi sementara ke layanan mikro yang mengalami kegagalan berulang guna mencegah kegagalan sistem menjalar ke seluruh infrastruktur (*cascading failure*).
- **Cumulative Layout Shift (CLS)**: Metrik Google Core Web Vitals yang mengukur tingkat pergeseran tata letak visual elemen antarmuka yang tidak terduga selama halaman dimuat.
- **Ed25519**: Skema tanda tangan digital kunci publik berbasis kurva eliptis Edwards (EdDSA) yang menawarkan kecepatan verifikasi ekstrem dan ukuran kunci ringkas 32-byte.
- **Hierarchical Navigable Small World (HNSW)**: Struktur data graf multi-skala berkecepatan tinggi untuk pencarian tetangga terdekat perkiraan (*Approximate Nearest Neighbor*) pada ruang vektor berdimensi tinggi.
- **Islands Architecture**: Paradigma perenderan web di mana halaman sebagian besar disajikan sebagai HTML statis (*Zero-JS*), dengan komponen-komponen interaktif terisolasi (*islands*) yang dihidrasi atau dimuat secara independen.
- **JSON:API**: Standar spesifikasi formal untuk pertukaran data API berbasis JSON yang mengatur konvensi pengalamatan resurse, compound documents, pemfilteran, dan paginasi.
- **Keyset Pagination**: Teknik paginasi basis data yang menggunakan indeks kolom (seperti stempel waktu atau ID unik) sebagai kursor pencarian, menghasilkan kompleksitas waktu konstan $O(1)$ dibanding klausa `OFFSET` yang lambat.
- **Okupasitas Memori (Heap Footprint)**: Total alokasi memori dinamis yang dikonsumsi oleh runtime aplikasi selama masa eksekusi.
- **Power of Two Choices (P2C)**: Algoritma penyeimbangan beban di mana peladen memilih dua simpul upstream secara acak, kemudian mengalirkan permintaan ke simpul yang memiliki koneksi aktif paling sedikit.
- **Server-Sent Events (SSE)**: Standar transmisi web satu arah berbasis HTTP standar (`text/event-stream`) di mana peladen dapat mendorong pesan teks secara seketika ke peramban tanpa perlu polling berulang.
- **View Transitions API**: API peramban standar untuk menganimasikan perpindahan dan perubahan elemen antarmuka antar dokumen atau rute SPA secara sinematik dan mulus.

---

## A.2 Referensi Direktif Templat Deklaratif WidyaWeb

WidyaWeb menyediakan direktif deklaratif yang bersih untuk menghubungkan markup dokumen dengan mesin reaktivitas sisi klien:

| Direktif WidyaWeb | Fungsi & Perilaku Eksekusi | Contoh Penggunaan |
| :--- | :--- | :--- |
| `w-click` | Mengikat fungsi penangan event klik mouse tanpa evaluasi string mentah | `<button w-click: handle_clap>Suka</button>` |
| `w-model` | Pengikatan data dua arah (*Two-way data binding*) antara input dan sinyal | `<input type="text" w-model="nama_penulis" />` |
| `w-island` | Mendeklarasikan wadah sebagai modul interaktif terisolasi (WASM/JS) | `<div w-island="pemilu.wasm" w-trigger="visible">` |
| `w-action` | Mengikat formulir langsung ke fungsi *Server Action* dengan token CSRF | `<form w-action="simpan_komentar_action">` |
| `w-link` | Mengintersepsi tautan navigasi peramban untuk mode navigasi SPA halus | `<a href="/berita/1" data-widya-link>Baca</a>` |
| `w-show` | Mengatur visibilitas simpul DOM secara reaktif berbasis ekspresi boolean | `<div w-show="apakah_tampil.baca()">Pemberitahuan</div>` |

---

## A.3 Ringkasan Modul Pustaka Standar `widya::web`

- `widya::web::http`: Berisi definisi `Permintaan`, `Respon`, `StatusHTTP`, penangan tajuk, dan enkripsi cookie.
- `widya::web::reactive`: Mengelola siklus hidup komponen, sinyal reaktif (`buat_sinyal`), dan reaksi efek (`efek`).
- `widya::web::gateway`: Modul proksi pembalik, algoritma penyeimbangan beban (P2C, Round Robin), dan terminasi TLS.
- `widya::web::websocket`: Peladen dan broker pub/sub WebSockets RFC 6455 dengan pembingkaian biner berkecepatan tinggi.
- `widya::web::streaming`: Penangan aliran Server-Sent Events (SSE) dan HTTP Chunked Transfer.
- `widya::web::jwt`: Pembangkit dan verifikator klaim token JWT berbasis kurva kriptografi Ed25519.
- `widya::web::s3`: Klien kompatibel AWS S3 untuk pembangkitan presigned upload URL dan manajemen media.
- `widya::web::observabilitas`: Instrumen metrik standar OpenMetrics Prometheus (Counter, Gauge, Histogram).
