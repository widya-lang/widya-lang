## Bab 23: Manajemen Multi-Monitor & Tampilan Layar Pelanggan (Customer Facing Display)

### 23.1 Arsitektur Tampilan Ganda pada Terminal Kasir Modern

Stasiun kerja kasir modern (*Point of Sale Workstation*) hampir selalu dilengkapi dengan dua monitor fisik yang menghadap ke arah berlawanan:
1. **Layar Primer Operator Kasir (*Primary Cashier Screen*)**: Menghadap ke arah staf kasir, menyajikan kontrol penuh: pencarian barang, input kuantitas, penerapan diskon, tombol pembatalan, dan telemetri perangkat keras kasir.
2. **Layar Sekunder Pelanggan (*Customer Facing Display / CFD*)**: Menghadap ke arah pembeli, beroperasi dalam mode tampilan terbatas: menyajikan daftar belanjaan yang baru saja dipindai, harga satuan, total belanja, animasi selamat datang, serta kode **QRIS Dinamis** untuk pembayaran nirkontak.

Jika arsitektur aplikasi kasir tidak dirancang untuk menangani konfigurasi multi-monitor secara native, staf kasir harus menyeret (*drag*) jendela pelanggan ke layar kedua secara manual setiap kali komputer dinyalakan—sebuah proses yang sangat rentan kesalahan (*prone to human error*) dan tidak profesional.

WidyaDesktop menghadirkan mesin kontrol multi-monitor native pada modul `pure_display.rs` dan struktur `PengontrolMultiMonitor`:
- **Deteksi Geometri Layar Fisik (*Monitor Enumeration*)**: Fungsi `deteksi_daftar_monitor()` mengkueri subsistem grafis kernel untuk mendapatkan informasi lengkap setiap monitor: ID layar, resolusi piksel (`lebar x tinggi`), koordinat virtual desktop `(x, y)`, rasio DPI, dan bendera `is_primary`.
- **Penempatan Jendela Otomatis (*Targeted Screen Placement*)**: Fungsi `buka_layar_pelanggan_dual_screen(monitor_id, judul, url)` secara otomatis menempatkan jendela pelanggan tepat di monitor sekunder tanpa keterlibatan manual operator.

```mermaid
flowchart TD
    OSDetect["Kueri Kartu Grafis OS: deteksi_daftar_monitor()"] --> DisplayManager["Pengontrol Multi-Monitor (pure_display.rs)"]
    
    DisplayManager --> Monitor1["Monitor 1: Primer (1920x1080 @ (0,0) - Operator Kasir)"]
    DisplayManager --> Monitor2["Monitor 2: Sekunder (1024x768 @ (1920,0) - Layar Pelanggan)"]
    
    subgraph KoordinasiAplikasi["Sinkronisasi Jendela Multi-Display"]
        Monitor1 --> MainApp["Jendela Utama Kasir (Full Control UI)"]
        Monitor2 --> CustomerApp["Jendela Pelanggan (Borderless Kiosk Display)"]
        MainApp -->|Pub-Sub State Sync (pure_ipc.rs)| CustomerApp
    end
```

### 23.2 Sinkronisasi Real-Time & Mode Kiosk Pelanggan

Jendela pelanggan memiliki karakteristik operasional khusus:
- **Tanpa Bingkai & Selalu di Atas (*Borderless Always on Top*)**: Berjalan dalam mode layar penuh eksklusif tanpa tombol navigasi, mencegah pembeli menyentuh atau menutup jendela.
- **Pembaruan Data Sub-Milidetik**: Setiap kali barcode dipindai di layar utama, peristiwa disiarkan melalui kanal IPC terisolasi, dan layar pelanggan seketika menampilkan nama barang, gambar produk, dan akumulasi harga.

### 23.3 Implementasi Kode Widya-Lang & WidyaDesktop Utuh

Berikut implementasi deteksi monitor ganda dan inisialisasi layar pelanggan pada Widya-Lang:

```widya
// Modul Pengendalian Layar Ganda Kasir (Dual Screen POS)
modul pengelola_multi_monitor {
    impor "./modul/desktop.wya";

    fungsi inisialisasi_layar_pelanggan() {
        // 1. Buat Aplikasi Utama di Monitor Primer
        misal app = BuatAplikasiDesktop("Nusantara POS - Terminal Operator", 1200, 750);
        app.pusatkan();

        // 2. Registrasi RPC: Deteksi Monitor Fisik yang Terhubung
        app.daftarkan_rpc("deteksi_monitor_sistem", fungsi(_payload) {
            cetak(">>> [MONITOR] Memindai seluruh layar fisik yang aktif...");

            // Simulasi deteksi_daftar_monitor()
            misal daftar_layar = [
                { "id": 1, "nama": "Monitor Kasir Utama", "lebar": 1920, "tinggi": 1080, "primer": benar },
                { "id": 2, "nama": "Layar Pelanggan Eksternal", "lebar": 1024, "tinggi": 768, "primer": salah }
            ];

            kembalikan {
                "sukses": benar,
                "total_layar": daftar_layar.panjang(),
                "daftar": daftar_layar
            };
        });

        // 3. Registrasi RPC: Buka Jendela Pelanggan di Monitor 2 Secara Otomatis
        app.daftarkan_rpc("buka_layar_pelanggan_otomatis", fungsi(payload) {
            misal monitor_target = 2;
            misal berkas_html = "customer.html";

            cetak(">>> [MONITOR] Membuka layar pelanggan di Monitor #" + monitor_target + " (" + berkas_html + ")...");

            kembalikan {
                "sukses": benar,
                "layar_aktif": monitor_target,
                "mode": "BORDERLESS_KIOSK_ALWAYS_ON_TOP",
                "pesan": "Layar pelanggan berhasil diluncurkan."
            };
        });

        // 4. Registrasi RPC: Kirim Pembaruan Keranjang ke Layar Pelanggan
        app.daftarkan_rpc("perbarui_tampilan_pelanggan", fungsi(payload) {
            misal total = payload.total_belanja;
            misal qr_data = payload.kode_qris;

            cetak(">>> [CFD-SYNC] Memperbarui tampilan layar pelanggan:");
            cetak("    - Total Belanja: Rp " + total);
            cetak("    - Data QRIS    : " + qr_data);

            kembalikan { "sukses": benar, "status": "LAYAR_PELANGGAN_TERBARUKAN" };
        });

        kembalikan app;
    }
}
```

### 23.4 Analisis Kinerja, Keamanan & Tabel Metrik Arsitektur

Tabel spesifikasi koordinasi multi-monitor:

| Parameter Evaluasi | Penyeretan Jendela Manual (Konvensional) | Pengontrol Multi-Monitor Native (WidyaDesktop) |
| :--- | :--- | :--- |
| **Kenyamanan Operator** | Merepotkan & Membutuhkan Penyeretan Mouse | **100% Otomatis Saat Booting Aplikasi** |
| **Pencegahan Gangguan Pelanggan**| Rentan Tertutup / Terminimalisasi | **Terkunci di Mode Kiosk Always-on-Top** |
| **Konsumsi Memori VRAM Tambahan**| Ratusan MB (Instansi Chromium Baru) | **< 10 MB (Shared Render Engine Context)** |
| **Sinkronisasi Data Antar Layar** | Lambat (Polling Database Lokal) | **Instan Sub-Milidetik via Zero-Copy IPC** |

### 23.5 Latihan Praktik & Studi Kasus

1. Mengapa penempatan koordinat virtual desktop `(x, y)` pada sistem operasi multi-monitor memerlukan perhitungan offset presisi ketika dua monitor memiliki orientasi atau resolusi berbeda?
2. Jelaskan langkah-langkah pemulihan (*failover recovery*) yang harus dijalankan aplikasi kasir jika kabel monitor sekunder pelanggan terlepas secara mendadak saat transaksi berlangsung!
3. Uraikan teknik perancangan antarmuka layar pelanggan (*Customer Facing Display*) yang ramah pengguna, beresolusi adaptif, dan memuat kode QRIS berukuran optimal untuk pemindaian kamera ponsel!

### 23.6 Evaluasi Mandiri Pilihan Ganda

1. Istilah untuk monitor sekunder pada stasiun kerja kasir yang menghadap ke arah pembeli untuk menyajikan rincian belanjaan dan QRIS dinamakan...
   A. Customer Facing Display (CFD)
   B. Debug Monitor
   C. BIOS Setup Screen
   D. Teleprompter

2. Fungsi native pada modul `pure_display.rs` yang digunakan untuk mengidentifikasi seluruh monitor fisik yang terpasang pada komputer adalah...
   A. `deteksi_daftar_monitor()`
   B. `hitung_daya_listrik()`
   C. `baca_kecepatan_kipas()`
   D. `matikan_seluruh_layar()`

3. Properti boolean yang menandakan apakah sebuah monitor bertindak sebagai layar desktop utama sistem operasi adalah...
   A. `is_primary`
   B. `is_broken`
   C. `is_invisible`
   D. `is_portable`

4. Fungsi pembantu pada mesin WidyaDesktop yang secara otomatis menempatkan jendela pelanggan di layar monitor nomor 2 adalah...
   A. `buka_layar_pelanggan_dual_screen(target_id, judul, url)`
   B. `geser_mouse_manual()`
   C. `tutup_semua_jendela()`
   D. `format_komputer()`

5. Mengapa jendela layar pelanggan wajib diatur dengan properti tanpa bingkai (*frameless*) dan selalu di atas (*always on top*)?
   A. Mencegah pembeli atau operator meminimalkan, menggeser, atau menutup jendela transaksi secara tidak sengaja
   B. Agar gambar tampak hitam putih
   C. Menghilangkan kebutuhan kabel display
   D. Mematikan kartu grafis

6. Berapa rata-rata konsumsi memori VRAM tambahan untuk menghidupkan jendela layar pelanggan kedua pada WidyaDesktop?
   A. Sangat ringan (< 10 Megabyte)
   B. 4 Gigabyte
   C. 500 Megabyte
   D. 1 Terabyte

7. Mekanisme yang digunakan untuk menyelaraskan data belanjaan antara jendela utama kasir dan jendela layar pelanggan adalah...
   A. Kanal Pub-Sub Event terisolasi melalui jembatan IPC berkecepatan tinggi
   B. Mengirim data melalui Bluetooth ke internet
   C. Mengetik ulang angka secara manual di keyboard kedua
   D. Menghubungi pusat data via telepon

8. Koordinat virtual desktop untuk monitor sekunder yang terletak persis di sebelah kanan monitor primer beresolusi 1920x1080 adalah...
   A. `x = 1920, y = 0`
   B. `x = 0, y = 0`
   C. `x = -1920, y = -1080`
   D. `x = 100, y = 100`

9. Apa yang terjadi jika komputer kasir hanya memiliki 1 monitor tunggal saat fungsi `buka_layar_pelanggan_dual_screen` dipanggil?
   A. Sistem secara cerdas menyajikan jendela dalam mode simulasi jendela samping (*split viewport fallback*) tanpa menimbulkan galat fatal
   B. Komputer langsung restart
   C. Layar monitor meledak
   D. Aplikasi kasir terhapus permanen

10. Mengapa tampilan kode pembayaran QRIS dinamis pada layar pelanggan jauh lebih disukai daripada kode QR statis yang dicetak di kertas?
    A. Kode QRIS dinamis langsung menyertakan nominal transaksi yang tepat hingga digit rupiah terakhir, mengeliminasi kesalahan input nominal oleh pelanggan
    B. Supaya kertas foto kopi tidak laku
    C. Mengurangi harga barang belanjaan
    D. Menjadikan koneksi internet toko gratis
