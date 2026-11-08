## Bab 5: Manajemen Multi-Jendela Hierarkis, Jendela Anak Modal & Splash Screen Fade-Out

### 5.1 Arsitektur Multi-Jendela Hierarkis & Modal Dialog Terisolasi

Aplikasi desktop skala enterprise sering kali menuntut lebih dari satu jendela visual yang beroperasi secara serentak namun terkoordinasi erat: jendela utama kasir, jendela dialog konfigurasi periferal kasir (modal), jendela pemantau inventaris gudang yang berjalan di latar belakang, serta jendela layar kedua (*secondary customer display*) untuk pembeli.

WidyaDesktop mengelola relasi antar-jendela melalui struktur hierarkis **Parent-Child Windows** pada modul `pure_window.rs` dan konfigurasi `ChildWindowConfig`:
1. **Hubungan Induk-Anak (*Parent-Child Relationship*)**: Jendela anak secara topologis terikat pada jendela induk. Jika jendela induk diminimalkan atau ditutup, seluruh jendela anak yang berada di bawahnya akan secara otomatis diminimalkan atau dihancurkan secara anggun, mencegah terjadinya jendela yatim (*orphan/dangling window leaks*).
2. **Isolasi Modalitas (*Modal Window Isolation*)**: Ketika jendela anak bertipe modal (`modal: true`) ditampilkan (misalnya kotak dialog pembayaran otorisasi manajer), jendela induk secara otomatis dinonaktifkan dari menerima masukan kursor dan penekanan tombol keyboard (`is_enabled = false`). Fokus sistem dipaksa berada pada jendela modal hingga jendela tersebut disetujui atau dibatalkan.
3. **Atribut Selalu di Atas (*Always on Top*)**: Digunakan pada jendela monitor darurat atau kalkulator kasir cepat agar posisinya tidak tertutup oleh aplikasi pihak ketiga lainnya.

```mermaid
flowchart TD
    Splash["Layar Percik (Splash Screen)"] -->|Transisi Fade-Out Halus (300ms)| MainWin["Jendela Induk Utama (Main POS Window)"]
    MainWin -->|Buka Dialog Pengaturan| ChildModal["Jendela Anak Modal (Settings / Auth Manager)"]
    MainWin -->|Buka Layar Sekunder| ChildDual["Jendela Sekunder Pelanggan (Customer Display)"]
    
    subgraph KontrolSistem["Koordinasi Pengelola Jendela (PengelolaJendelaMurni)"]
        ChildModal -.->|Nonaktifkan Masukan Induk| MainWin
        MainWin -.->|Tutup Induk Menghancurkan Seluruh Anak| ChildModal
        MainWin -.->|Tutup Induk Menghancurkan Seluruh Anak| ChildDual
    end
```

### 5.2 Layar Percik (*Splash Screen*) & Transisi Halus (*Fade-Out*)

Waktu pemuatan awal (*cold startup*) adalah kesan pertama yang dialami pengguna. Meskipun aplikasi WidyaDesktop dapat menyala dalam hitungan milidetik, pemuatan berkas aset besar, inisialisasi basis data lokal WidyaBase, atau penemuan port serial periferal kasir dapat memerlukan waktu beberapa ratus milidetik.

Fitur **Splash Screen** terintegrasi secara otomatis menyajikan layar percik ringan tanpa bingkai sejak milidetik pertama eksekusi biner. Setelah seluruh dependensi backend siap dan jendela utama selesai dirender (*First Contentful Paint*), sistem mengeksekusi transisi memudar halus (*smooth fade-out opacity transition*) selama 300 ms, menciptakan transisi visual kelas profesional tanpa kedipan layar hitam (*black screen flicker*).

### 5.3 Implementasi Kode Widya-Lang & WidyaDesktop Utuh

Berikut implementasi pengelolaan jendela anak modal dan koordinasi multi-jendela pada Widya-Lang:

```widya
// Modul Pengendalian Multi-Jendela & Dialog Modal
modul pengelola_multi_jendela {
    impor "./modul/desktop.wya";

    fungsi inisialisasi_sistem_multi_jendela() {
        // 1. Jendela Utama Kasir
        misal app_utama = BuatAplikasiDesktop("Nusantara POS - Terminal Utama", 1200, 750);
        app_utama.pusatkan();

        // 2. Registrasi RPC untuk Membuka Jendela Modal Otorisasi Manajer
        app_utama.daftarkan_rpc("buka_modal_otorisasi", fungsi(payload) {
            misal nominal_diskon = payload.diskon;
            cetak(">>> [MODAL] Membuka jendela dialog otorisasi diskon: " + nominal_diskon + "%");

            // Menggunakan helper dialog native untuk konfirmasi instan
            misal pin = BukaDialogPilihBerkas("*.pin"); // Simulasi seleksi berkas token
            misal pesan_hasil = "Otorisasi Diskon Berhasil Disetujui";

            kembalikan {
                "sukses": benar,
                "status_otorisasi": "DISETUJUI_MANAJER",
                "pesan": pesan_hasil
            };
        });

        // 3. Registrasi RPC untuk Membuka Layar Pelanggan pada Monitor Sekunder
        app_utama.daftarkan_rpc("buka_layar_pelanggan", fungsi(_payload) {
            cetak(">>> [MULTI-DISPLAY] Membuka jendela sekunder pada monitor 2...");
            // Konfigurasi ChildWindowConfig dijalankan oleh backend native
            kembalikan {
                "sukses": benar,
                "jendela_id": "customer_display_01",
                "target_monitor": 2,
                "resolusi": "1024x768"
            };
        });

        kembalikan app_utama;
    }
}
```

### 5.4 Analisis Kinerja, Keamanan & Tabel Metrik Arsitektur

Tabel evaluasi arsitektur multi-jendela dan overhead sistem:

| Metrik Multi-Jendela | Pola Multi-Proses Mandiri (Electron) | Pola Multi-Webview Terpadu | WidyaDesktop Native Child Windows |
| :--- | :--- | :--- | :--- |
| **Konsumsi RAM Tambahan / Jendela** | 120 – 200 MB per jendela baru | 40 – 60 MB | **6 – 12 MB (Sangat Ringan)** |
| **Waktu Pembuatan Jendela Baru** | 800 – 1500 ms (Spawn Process) | 300 – 500 ms | **35 – 80 ms (Instan Native Win32)** |
| **Isolasi Keamanan Antar-Jendela** | Isolasi Proses Penuh | Isolasi Konteks Webview | **Token IPC Terisolasi Per-Jendela** |
| **Dukungan Modalitas Sistem** | Simulasi Web DOM Overlay | Simulasi JavaScript | **Native OS Modal Disabling** |

### 5.5 Latihan Praktik & Studi Kasus

1. Mengapa simulasi jendela modal menggunakan lapisan CSS HTML (*DOM overlay div*) lebih rentan terhadap eksploitasi pengguna dibandingkan jendela modal tingkat sistem operasi (*native modal window*)?
2. Jelaskan mekanisme pelepasan memori (*cleanup lifecycle*) yang dijalankan `PengelolaJendelaMurni` ketika jendela induk ditutup secara mendadak saat beberapa jendela anak sedang aktif!
3. Uraikan langkah-langkah implementasi transisi *Splash Screen Fade-Out* agar tidak menimbulkan kedipan grafis (*graphical tearing*) pada kartu grafis berspesifikasi rendah!

### 5.6 Evaluasi Mandiri Pilihan Ganda

1. Karakteristik utama dari jendela anak modal (*modal child window*) adalah...
   A. Menonaktifkan interaksi pada jendela induk hingga jendela modal diselesaikan atau ditutup
   B. Menutup seluruh koneksi internet
   C. Mengubah seluruh teks menjadi huruf miring
   D. Menghapus data akun pengguna

2. Manfaat arsitektural dari keterikatan hierarki induk-anak (*parent-child hierarchy*) pada jendela desktop adalah...
   A. Jendela anak secara otomatis ikut tertutup atau diminimalkan ketika jendela induk ditutup, mencegah kebocoran jendela yatim
   B. Mengharuskan pengguna membeli dua monitor
   C. Menghapus tombol enter pada keyboard
   D. Mematikan fungsi mouse

3. Tujuan utama penyajian Layar Percik (*Splash Screen*) saat aplikasi pertama kali dinyalakan adalah...
   A. Memberikan umpan balik visual instan sejak milidetik pertama sementara modul backend, basis data, dan port hardware diinisialisasi
   B. Menampilkan iklan video komersial
   C. Memperpanjang waktu booting komputer
   D. Memformat diska penyimpanan

4. Durasi standar transisi memudar (*fade-out opacity transition*) pada layar percik WidyaDesktop untuk menciptakan kesan mulus adalah...
   A. Sekitar 300 milidetik
   B. Tepat 24 jam
   C. 15 menit
   D. 0.0001 nanodetik

5. Struktur konfigurasi yang digunakan untuk mendefinisikan properti jendela anak seperti modalitas dan posisi selalu di atas adalah...
   A. `ChildWindowConfig`
   B. `GlobalHtmlParser`
   C. `SvgIconDescriptor`
   D. `FontGlyphTable`

6. Properti yang memastikan posisi jendela tetap berada di lapisan teratas layar dan tidak tertutup oleh aplikasi lain adalah...
   A. `always_on_top: true`
   B. `invisible: true`
   C. `background_only: true`
   D. `disable_screen: true`

7. Berapa rata-rata konsumsi memori RAM tambahan untuk setiap pembuatan jendela anak baru pada WidyaDesktop?
   A. Hanya sekitar 6 hingga 12 Megabyte
   B. Minimal 2 Gigabyte
   C. 500 Megabyte
   D. 1 Terabyte

8. Bagaimana WidyaDesktop mengamankan komunikasi data antara jendela utama kasir dan jendela sekunder pelanggan?
   A. Menggunakan jembatan IPC yang divalidasi token kriptografis independen untuk masing-masing jendela
   B. Menghapus seluruh data harga
   C. Mengirim data melalui kabel audio speaker
   D. Mencetak data ke kertas terlebih dahulu

9. Perilaku yang terjadi jika jendela induk diminimalkan pada sistem dengan hierarki induk-anak yang benar adalah...
   A. Seluruh jendela anak yang terikat secara otomatis ikut diminimalkan dari layar desktop
   B. Seluruh jendela anak meledak
   C. Komputer langsung restart
   D. Jendela anak berubah menjadi transparan total tanpa bisa diakses

10. Mengapa stasiun kerja kasir ritel memerlukan jendela sekunder terpisah (*Customer Facing Display*)?
    A. Menampilkan rincian belanjaan dan kode QR pembayaran QRIS kepada pelanggan secara transparan tanpa memperlihatkan kontrol kasir internal
    B. Supaya kasir tidak perlu melihat pelanggan
    C. Untuk mematikan lampu toko
    D. Menghilangkan kebutuhan nota belanja
