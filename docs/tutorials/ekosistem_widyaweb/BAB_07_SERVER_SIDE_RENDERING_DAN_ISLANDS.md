## Bab 7: Mesin Rendering Server (SSR) & Arsitektur Islands

### 7.1 Paradigma Islands Architecture: 0 KB JavaScript Default

Dalam perancangan portal berita skala nasional yang melayani puluhan juta rakyat Indonesia dengan beragam spesifikasi perangkat—mulai dari komputer desktop hingga ponsel cerdas terjangkau berdaya komputasi rendah di daerah pelosok—arsitektur *Single Page Application* (SPA) murni berbasis JavaScript sering kali menjadi bumerang:
- SPA mengharuskan peramban mengunduh berkas bundel JavaScript raksasa (sering kali 2–5 MB) sebelum teks berita pertama kali dapat dibaca.
- Pada jaringan seluler yang lambat, pembaca disuguhi layar putih kosong (*blank white screen*) selama berminggu-minggu interaksi akumulatif.
- Perayap mesin pencari (*Search Engine Crawlers*) dan bot media sosial kerap gagal mengindeks konten berita secara akurat.

WidyaWeb memelopori standar rendering modern melalui **Mesin Hybrid Rendering & Islands Architecture** native (`pure_ssr.rs`):
- **Dokumen HTML Statis 100% Cepat (0 KB Client JS Default)**: Seluruh tata letak halaman, artikel berita, navigasi, dan footer dirender menjadi HTML murni di sisi peladen (*Server-Side Rendering*) tanpa menyertakan kode JavaScript klien sama sekali.
- **Pulau Interaktif Terisolasi (*Islands of Interactivity*)**: Kode interaktivitas berbasis JavaScript atau WebAssembly **hanya dikirimkan untuk komponen-komponen kecil yang benar-benar membutuhkan interaksi pengguna langsung** (seperti: pemilih ukuran font, kolom pencarian cerdas autocomplete, atau tombol jempol suka artikel).
- **Streaming SSR via HTTP Chunked Transfer**: Peladen mengalirkan potongan header `<head>` dan tata letak utama ke peramban secara seketika, sementara bagian konten yang membutuhkan kueri basis data dialirkan berikutnya (*streaming response*) menggunakan penanda *Suspense*.

```mermaid
flowchart TD
    subgraph HalamanBeritaNusantara["Halaman Web Portal Berita NusantaraNews (Islands Architecture)"]
        H["Header Berita (HTML Statis Murni - 0 KB JS)"]
        Nav["Navigasi Kanal (HTML Statis Murni - 0 KB JS)"]
        Art["Isi Teks Artikel Berita & Foto (HTML Statis Murni - 0 KB JS)"]
        
        Island1["Pulau 1: Komponen Font Resizer<br>[Hidrasi Mandiri ~2 KB JS]"]
        Island2["Pulau 2: Live Comment & Poll<br>[Hidrasi WebSockets ~4 KB JS]"]
        
        Foot["Footer & Hak Cipta (HTML Statis Murni - 0 KB JS)"]
    end
    Klien["Peramban Klien"] -->|Kueri Permintaan GET /berita/101| Server["Peladen WidyaWeb SSR (pure_ssr.rs)"]
    Server -->|1. Alirkan HTML Bersih Seketika (FCP < 300 ms)| Klien
    Server -.->|2. Hidrasi Terisolasi Hanya untuk Pulau 1 & 2| Island1 & Island2
```

Empat mode rendering terpadu pada WidyaWeb:
1. `ModeRender::StreamingSSR`: Merender halaman secara asinkron di peladen dan mengalirkannya secara bertahap via HTTP chunked encoding.
2. `ModeRender::StaticSiteGen`: Mengompilasi halaman statis sekali saat build time untuk konten yang jarang berubah.
3. `ModeRender::IncrementalStatic` (ISR): Menyimpan halaman di tembolok dan menyegarkannya di latar belakang berbasis kebijakan *Stale-While-Revalidate* (SWR).
4. `ModeRender::IslandsOnly`: Memadukan konten statis bebas JS dengan pulau-pulau interaktif mandiri.

### 7.2 Implementasi Kode Program Utuh

Berikut implementasi Server-Side Rendering dan pendaftaran komponen pulau interaktif pada WidyaWeb menggunakan Widya-Lang:

```widya
// Modul Server-Side Rendering dan Islands Architecture WidyaWeb
modul mesin_rendering_ssr {
    impor ekosistem::widyaweb::{
        WidyaWeb, MesinRenderSSR, ModeRender, KonfigurasiRenderHalaman,
        PulauInteraktif, ResponHttp, StatusHttp, KontenTipe
    };

    fungsi render_halaman_artikel_portal(id_artikel: Bulat) -> Hasil<ResponHttp, Teks> {
        cetak("==================================================================");
        cetak(">>> [SSR-ISLANDS] Merender Halaman Artikel Berita ID: " + teks(id_artikel));

        biar mut mesin_ssr = MesinRenderSSR::buat();

        // 1. Definisikan Konfigurasi Mode Rendering (Islands + Streaming)
        biar konfigurasi = KonfigurasiRenderHalaman {
            mode_render: ModeRender::IslandsOnly,
            revalidasi_detik: Ada(60), // ISR: Revalidasi setiap 60 detik (SWR)
            tags_revalidasi: ["artikel_berita", "kanal_utama"],
            suspense_enabled: benar,
        };

        // 2. Daftarkan Pulau Interaktif (Interactive Island) untuk Widget Reaksi Pembaca
        biar pulau_reaksi = PulauInteraktif {
            id_pulau: "island-reaksi-101",
            nama_komponen: "WidgetReaksiPembaca",
            state_props_json: "{\"total_suka\": 850, \"id_artikel\": 101}",
        };
        mesin_ssr.daftarkan_pulau(pulau_reaksi);

        // 3. Bangun Struktur HTML Semantik Lengkap (SEO Otoritatif)
        biar html_output = "
            <!DOCTYPE html>
            <html lang='id'>
            <head>
                <meta charset='UTF-8'>
                <meta name='viewport' content='width=device-width, initial-scale=1.0'>
                <title>Kedaulatan Digital Nusantara - NusantaraNews</title>
                <meta name='description' content='Indonesia melangkah mandiri dengan ekosistem perangkat lunak berdaulat penuh.'>
                <!-- 0 KB Bundel JS Monolitik! Hanya script pemantik pulau yang dimuat -->
                <script type='module' src='/_widya/islands-bootstrap.js' defer></script>
            </head>
            <body>
                <header class='site-header'>
                    <h1>NusantaraNews</h1>
                    <nav><a href='/'>Beranda</a> | <a href='/politik'>Politik</a> | <a href='/teknologi'>Teknologi</a></nav>
                </header>

                <main class='content-container'>
                    <article>
                        <h2>Indonesia Mantapkan Kemandirian Infrastruktur Digital</h2>
                        <p class='metadata'>Diterbitkan: 20 Oktober 2026 | Penulis: Redaksi Nasional</p>
                        <div class='body-text'>
                            <p>Langkah bersejarah diambil dalam memperkuat ketahanan komputasi nasional...</p>
                        </div>
                    </article>

                    <!-- PULAU INTERAKTIF 1: Widget Suka (Hanya bagian ini yang terhidrasi) -->
                    <div id='island-reaksi-101' data-widya-island='WidgetReaksiPembaca' class='island-container'>
                        <button class='btn-like'>👍 Suka (<span id='like-count'>850</span>)</button>
                    </div>
                </main>

                <footer>
                    <p>&copy; 2026 NusantaraNews. Mengabdi untuk Kedaulatan Informasi Bangsa.</p>
                </footer>
            </body>
            </html>
        ";

        cetak(">>> [SSR] Dokumen HTML berhasil disintesis. Ukuran: " + teks(html_output.panjang()) + " byte.");
        cetak(">>> [ISLANDS] Hanya 1 pulau terisolasi yang membutuhkan hidrasi klien.");
        cetak("==================================================================");

        kembalikan Ok(ResponHttp::buat(StatusHttp::Ok)
            .tambah_header("Content-Type", "text/html; charset=utf-8")
            .tambah_header("X-Render-Mode", "WidyaWeb-Islands")
            .isi_badan(html_output));
    }
}
```

### 7.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan paradigma rendering antarmuka pada platform web:

| Parameter Metrik Web | Client-Side SPA (React SPA) | Full SSR Monolitik (Next.js) | Islands Architecture (WidyaWeb) |
| :--- | :--- | :--- | :--- |
| **Beban JavaScript Klien** | Sangat Berat (1.5 – 4 MB JS) | Berat (Ratusan KB JS Hidrasi)| **Hampir Nol (0 – 15 KB JS Total)** |
| **First Contentful Paint (FCP)**| Sangat Buruk (1.500–3.500 ms) | Sangat Cepat (300–600 ms) | **Ultra Cepat (< 250 milidetik)** |
| **Time to Interactive (TTI)** | Lambat (Menunggu parse JS) | Lambat (Hidrasi seluruh DOM) | **Seketika (Hanya pulau kecil)** |
| **Keterbacaan Robot SEO** | Rawan Gagal / Terpotong | Sempurna | **Sempurna Mutlak (HTML Murni)** |
| **Ketahanan Jaringan Lemah** | Sangat Buruk (Layar Putih) | Baik untuk teks | **Sangat Unggul & Tetap Terbaca** |

### 7.4 Latihan Soal Esai

1. Terangkan perbedaan filosofis mendasar antara arsitektur *Single Page Application* (SPA) konvensional dengan *Islands Architecture* dalam mendistribusikan kode JavaScript ke peramban klien!
2. Mengapa pendekatan *0 KB JavaScript Default* pada WidyaWeb menjadi solusi teknologi yang sangat berkeadilan sosial bagi masyarakat Indonesia yang mengakses portal berita di wilayah pelosok kepulauan?
3. Jelaskan cara kerja teknik *Stale-While-Revalidate* (SWR) pada Incremental Static Regeneration (ISR) dalam menjaga agar pembaca selalu disajikan halaman berita dari tembolok instan sementara penyegaran artikel berlangsung di latar belakang!

### 7.5 Soal Pilihan Ganda

1. Prinsip utama dari Islands Architecture yang dipelopori pada WidyaWeb adalah...
   A. Menghasilkan halaman HTML statis bersih (0 KB JavaScript default) dan hanya menghidrasi komponen pulau interaktif terisolasi
   B. Menghapus seluruh tata letak CSS
   C. Mengunci seluruh browser pengguna
   D. Mematikan sambungan internet

2. Metrik kinerja web yang mengukur waktu sejak pengguna mulai membuka halaman hingga konten teks atau gambar pertama kali tampak di layar adalah...
   A. First Contentful Paint (FCP)
   B. Cumulative Layout Shift (CLS)
   C. Disk Throughput
   D. Ping Latency

3. Metrik yang mengukur kapan halaman web menjadi sepenuhnya responsif dan siap menerima interaksi pengguna (seperti klik tombol) dinamakan...
   A. Time to Interactive (TTI)
   B. First Byte Time
   C. Memory Leak
   D. Idle Clock

4. Mode rendering yang menyajikan salinan halaman instan dari tembolok sembari menyegarkan data artikel terbaru di latar belakang adalah...
   A. Incremental Static Regeneration (ISR) dengan Stale-While-Revalidate
   B. Client-Only SPA
   C. Direct Memory Access
   D. Hard Reset

5. Fitur HTTP yang memungkinkan peladen mengalirkan potongan-potongan dokumen HTML secara bertahap sebelum seluruh halaman selesai diproses di server adalah...
   A. HTTP Chunked Transfer Streaming
   B. Half-Duplex Stream
   C. Serial File Dump
   D. Zero Buffer

6. Komponen pembungkus pada HTML yang menandai bagian konten dinamis yang masih menunggu hasil kueri basis data dinamakan konsep...
   A. Suspense Boundary
   B. Error Page
   C. Breakpoint
   D. Iframe

7. Mengapa perayap mesin pencari (SEO Search Engine Crawlers) sangat menyukai halaman yang dirender menggunakan Server-Side Rendering (SSR)?
   A. Karena seluruh konten artikel berita, judul, dan metadata sudah berupa teks HTML lengkap saat pertama kali diunduh tanpa perlu mengeksekusi JavaScript
   B. Karena mesin pencari tidak menyukai gambar
   C. Karena berkas HTML berukuran nol byte
   D. Karena server tidak memiliki alamat IP

8. Apa yang terjadi pada bagian artikel berita statis di luar pulau interaktif saat halaman dibuka di peramban pembaca?
   A. Ditampilkan sebagai teks HTML semantik biasa tanpa ada beban hidrasi JavaScript sama sekali
   B. Terhapus secara otomatis
   C. Diubah menjadi file PDF
   D. Memerlukan login kata sandi

9. Manfaat utama teknik Selective Hydration pada pulau interaktif adalah...
   A. Komponen pulau yang berada di luar layar (off-screen) hanya dihidrasi ketika pengguna menggulir layar mendekatinya
   B. Mematikan fungsi mouse
   C. Menghapus memori RAM
   D. Mengubah bahasa antarmuka

10. Penerapan Islands Architecture pada portal berita nasional NusantaraNews membuktikan bahwa...
    A. Pengalaman membaca berita dapat berlangsung instan di bawah 300 milidetik bahkan pada perangkat ponsel berharga terjangkau dengan koneksi seluler terbatas
    B. Berita online tidak membutuhkan foto
    C. Semua pengguna harus menggunakan komputer super
    D. Website modern tidak boleh menggunakan warna
