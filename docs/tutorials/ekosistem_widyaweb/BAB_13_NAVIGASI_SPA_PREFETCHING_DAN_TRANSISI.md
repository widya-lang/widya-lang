# BAB 13: NAVIGASI SISI KLIEN SPA, PREFETCHING & VIEW TRANSITIONS

Bab ini mendalami teknik navigasi sisi klien (*Single Page Application / SPA navigation*) tanpa kedipan halaman putih (*zero white-flash*), mekanisme cerdas *speculative prefetching* tautan berita berdasarkan pergerakan kursor pembaca, sinkronisasi History API, dan implementasi transisi visual sinematik menggunakan standar Web View Transitions API.

---

## 13.1 Arsitektur Navigasi Spekulatif & View Transitions

Ketika pembaca mengklik tautan artikel di portal berita, pemuatan halaman penuh tradisional (*hard reload*) akan membuang status memori, melakukan parsing ulang berkas CSS, dan memicu kedipan layar. WidyaWeb mengatasi hal ini dengan mengintersepsi klik tautan internal, mengambil muatan HTML/JSON secara asinkron di latar belakang, dan menukar simpul konten utama menggunakan *View Transitions API*.

```mermaid
sequenceDiagram
    autonumber
    actor Pembaca as Kursor Pembaca
    participant Link as Tautan Berita (HTML Link)
    participant Prefetcher as WidyaWeb Prefetch Engine
    participant Router as SPA Client Router
    participant ViewTx as View Transitions Engine

    Pembaca->>Link: Hover kursor (65ms idle)
    Link->>Prefetcher: Trigger Spekulasi Prefetch
    Prefetcher->>Prefetcher: Ambil HTML Parsial & Simpan di Cache Memori Klien
    Pembaca->>Link: Klik Tautan Artikel
    Link->>Router: Intersepsi Navigasi Internal
    Router->>ViewTx: document.startViewTransition()
    ViewTx->>ViewTx: Tangkap Snapshot Tampilan Lama
    Router->>Router: Pasang Fragmen DOM Halaman Baru
    ViewTx->>ViewTx: Animasikan Morfologi Hero Image & Judul
    ViewTx-->>Pembaca: Tampilan Artikel Baru Selesai Berpindah Halus
```

Dengan *speculative prefetching*, konten artikel sudah tersedia di memori sebelum jari pembaca selesai menekan mouse, menghasilkan persepsi navigasi seketika (*zero perceived latency*).

---

## 13.2 Implementasi SPA Router & Handler Transisi

Berikut adalah implementasi logika navigasi sisi klien dan konfigurasi View Transitions pada antarmuka pembaca artikel:

```widya
// Modul Navigasi Klien NusantaraNews
gunakan widya::web::klien::{NavigasiKlien, TransisiTampilan, CacheMemori};
gunakan widya::web::dom::{tangkap_elemen, dokumen};

rekod KonfigurasiNavigasi {
    toleransi_hover_ms: Bulat,
    batas_maksimal_cache: Bulat,
    aktifkan_view_transitions: BenarSalah
}

fungsi InisialisasiNavigasiPortal() -> NavigasiKlien {
    tetapan cache = CacheMemori::baru(50); // Simpan maksimal 50 rute artikel
    tetapan cfg = KonfigurasiNavigasi {
        toleransi_hover_ms: 65,
        batas_maksimal_cache: 50,
        aktifkan_view_transitions: benar
    };

    // 1. Intersepsi seluruh tautan internal portal
    dokumen().pada_event("mouseover", fungsi(event: EventKursor) {
        tetapan tautan = event.target.cari_leluhur_terdekat("a[data-widya-link]");
        jika tautan != tak_ada {
            tetapan url = tautan.ambil_atribut("href");
            // Prefetch hanya jika belum tersimpan di cache klien
            jika !cache.memuat(url) {
                PrefetchSpekulatif(url, cache);
            }
        }
    });

    // 2. Tangani klik dengan transisi halus
    dokumen().pada_event("click", fungsi(event: EventKlik) {
        tetapan tautan = event.target.cari_leluhur_terdekat("a[data-widya-link]");
        jika tautan != tak_ada && !event.tombol_modifikasi_ditekan() {
            event.cegah_bawaan();
            tetapan url_tujuan = tautan.ambil_atribut("href");
            BeralihHalaman(url_tujuan, cache, cfg);
        }
    });
}

fungsi BeralihHalaman(url: Teks, cache: CacheMemori, cfg: KonfigurasiNavigasi) {
    tetapan aksi_tukar = fungsi() {
        tetapan html_baru = cache.ambil(url);
        tangkap_elemen("#konten-utama").atur_html_bagian_dalam(html_baru);
        dokumen().atur_judul(cache.ambil_metadata_judul(url));
        dokumen().riwayat_peramban().dorong_state(url);
        dokumen().gulir_ke_atas();
    };

    jika cfg.aktifkan_view_transitions && TransisiTampilan::didukung() {
        TransisiTampilan::mulai(aksi_tukar);
    } jika_tidak {
        aksi_tukar();
    }
}
```

Pada lapisan CSS, kita memetakan nama transisi untuk gambar hero agar tercipta animasi perpindahan halus (*hero element morphing*):

```css
/* Aturan Transisi Tampilan Hero Berita */
.kartu-berita .thumbnail-wrapper img {
  view-transition-name: hero-image;
}

.halaman-detail-artikel .gambar-header-hero {
  view-transition-name: hero-image;
}

::view-transition-old(hero-image),
::view-transition-new(hero-image) {
  animation-duration: 0.3s;
  animation-timing-function: cubic-bezier(0.2, 0, 0, 1);
}
```

---

## 13.3 Analisis Metrik Latensi Pemuatan Navigasi

Navigasi spekulatif memangkas *Navigation Timing* hingga mendekati batas waktu fisik monitor (0–16 ms).

| Skenario Navigasi | Navigasi Halaman Penuh Klasik | Client-Side SPA Polos | WidyaWeb Prefetch + View Transitions |
| :--- | :--- | :--- | :--- |
| **Latensi Respons Klik** | 450 ms – 1200 ms | 120 ms – 300 ms | < 15 ms (Instan dari Cache) |
| **Konsumsi Jaringan saat Klik** | Mengunduh seluruh HTML, CSS, JS | Meminta data API JSON | 0 B (Data telah terambil di awal) |
| **Visual Flickering (Layar Putih)** | Selalu Terjadi (FOUC) | Terkadang Terlihat | Nol (Transisi Morfologi Lembut) |
| **Beban CPU Server per Klik** | Query penuh & perakitan | Query JSON | Nol (Terlayani dari tepi/klien) |

---

## 13.4 Latihan Praktik & Studi Kasus

1. **Implementasi Pembatalan Prefetch (Debouncing)**: Perbaiki fungsi prefetch agar tidak mengunduh data jika kursor hanya melintas cepat di atas tautan kurang dari 50ms.
2. **Indikator Progres Bar Tipis di Atas Layar**: Buat komponen garis pemuatan tipis (*Top Progress Bar*) dengan warna gradasi merah yang muncul jika proses pengambilan rute memakan waktu lebih dari 150ms.
3. **Restorasi Posisi Gulir Riwayat (Scroll Restoration)**: Implementasikan penyimpanan posisi gulir piksel vertikal saat pembaca menekan tombol *Back* pada peramban agar kembali tepat ke posisi artikel sebelumnya.

---

## 13.5 Evaluasi Mandiri Pilihan Ganda

1. Apa tujuan utama dari fitur *Speculative Prefetching* pada portal berita NusantaraNews?
   - A. Menghapus riwayat penjelajahan pembaca
   - B. Mengunduh data artikel berikutnya di latar belakang saat kursor menyentuh tautan sebelum diklik
   - C. Mengunci antarmuka browser agar tidak dapat diubah
   - D. Menjalankan skrip penambangan data kripto

2. API standar web modern apa yang digunakan untuk menganimasikan peralihan elemen visual antar rute secara mulus?
   - A. Canvas 2D API
   - B. View Transitions API
   - C. Web Audio API
   - D. Web Bluetooth API

3. Properti CSS apa yang disematkan pada elemen gambar agar dapat bertransisi secara mulus dari kartu feed ke halaman detail?
   - A. `transform: rotate(45deg);`
   - B. `view-transition-name: hero-image;`
   - C. `filter: blur(5px);`
   - D. `display: none;`

4. Berapa durasi rata-rata jeda kursor manusia (*hover-to-click delay*) yang dimanfaatkan untuk melakukan prefetching spekulatif?
   - A. Kurang lebih 65 ms hingga 200 ms
   - B. Tepat 5 menit
   - C. 1 mikrodetik
   - D. Tidak ada jeda sama sekali

5. Mengapa tautan yang diklik sambil menekan tombol modifikasi (seperti `Ctrl+Click` atau `Cmd+Click`) tidak boleh diintersepsi oleh router SPA?
   - A. Karena akan merusak memori RAM peramban
   - B. Menghormati perilaku alami peramban untuk membuka tautan pada tab atau jendela baru
   - C. Peramban akan langsung memblokir alamat IP portal
   - D. Bahasa Widya-Lang tidak mendukung penekanan tombol keyboard ganda

6. Metode peramban apa yang digunakan untuk memperbarui URL pada bilah alamat tanpa memicu reload halaman penuh?
   - A. `window.location.reload()`
   - B. `history.pushState()`
   - C. `document.clear()`
   - D. `window.close()`

7. Berapa konsumsi transfer jaringan tambahan saat pengguna mengklik artikel yang telah di-prefetch sebelumnya?
   - A. 10 Megabyte
   - B. 0 Byte karena data disajikan langsung dari cache memori klien
   - C. 500 Kilobyte
   - D. Tergantung kecepatan kipas prosesor komputer

8. Apa fungsi dari parameter `batas_maksimal_cache: 50` pada implementasi `CacheMemori` di atas?
   - A. Mencegah pertumbuhan memori tak terbatas di peramban klien dengan strategi LRU (*Least Recently Used*)
   - B. Membatasi pembaca hanya boleh membaca 50 kata per hari
   - C. Memaksa server mematikan koneksi setelah 50 detik
   - D. Menghapus 50 artikel dari database WidyaBase

9. Kapan fungsi pembaruan DOM dalam `TransisiTampilan::mulai(aksi_tukar)` dieksekusi?
   - A. Setelah browser mengambil cuplikan visual (*snapshot*) dari tampilan lama
   - B. Tepat sebelum browser dibuka oleh pengguna
   - C. Di tengah-tengah proses *compiling* kode Rust
   - D. Saat sistem operasi mengalami mati daya listrik

10. Bagaimana cara memastikan peramban lama yang belum mendukung View Transitions API tetap dapat membuka halaman tanpa galat (*graceful degradation*)?
    - A. Memblokir akses pengunjung yang menggunakan browser lama
    - B. Mengecek ketersediaan fitur dengan kondisi `TransisiTampilan::didukung()` dan menjalankan fungsi pergantian DOM secara langsung jika tidak didukung
    - C. Mengirimkan berkas installer browser terbaru ke komputer pengguna
    - D. Menghapus seluruh berkas CSS dari server
