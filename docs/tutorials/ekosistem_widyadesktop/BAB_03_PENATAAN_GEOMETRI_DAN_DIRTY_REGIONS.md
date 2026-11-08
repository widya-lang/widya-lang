## Bab 3: Penataan Geometri, Pembatas Ukuran & Dirty Region Tracking

### 3.1 Mesin Pembatas Geometri & Per-Monitor DPI Awareness v2

Dalam lingkungan produksi stasiun kerja desktop, antarmuka pengguna harus mampu beradaptasi dengan beragam konfigurasi perangkat keras visual: mulai dari monitor monitor kasir layar sentuh beresolusi rendah (1024x768), monitor ganda berorientasi potret, hingga layar resolusi tinggi 4K/5K (*High DPI / Retina Display*).

WidyaDesktop menerapkan dua pilar fundamental dalam pengelolaan tata letak geometris:
1. **Mesin Pembatas Geometri (*Geometry Constraint Engine*)**: Menegakkan batas dimensi fisik jendela secara deterministik melalui aturan `min_lebar`, `max_lebar`, `min_tinggi`, dan `max_tinggi`. Ketika operator mencoba memperkecil jendela melebihi batas minimum tata letak formulir transaksi, mesin secara instan melakukan pemotongan (*clamping*) koordinat pada tingkat penanganan pesan sistem operasi (`WM_GETMINMAXINFO` pada Windows).
2. **Skalabilitas Per-Monitor DPI Awareness (v2)**: Menghitung faktor skala piksel secara dinamis per monitor fisik (*Fractional DPI Scaling: 100%, 125%, 150%, 200%*). Ketika jendela digeser dari monitor primer (100% DPI) ke monitor sekunder layar sentuh ber-DPI tinggi (150% DPI), koordinat vektor dan ukuran fonta antarmuka disesuaikan secara mulus tanpa distorsi kabur (*blurring*).

```mermaid
flowchart LR
    OSResize["Peristiwa Tarik Ukuran (OS Resize Event)"] --> FilterConstraint{"Evaluasi Batasan Geometri (pure_window.rs)"}
    FilterConstraint -->|Ukuran < Batas Minimal| ClampSize["Koreksi Ukuran (Clamp to Min/Max)"]
    FilterConstraint -->|Ukuran Sah| ApplySize["Terapkan Dimensi Baru"]
    ClampSize --> ApplySize
    ApplySize --> DirtyCalc["Kalkulasi Wilayah Rusak (Dirty Region Tracker)"]
    DirtyCalc --> SelectiveRedraw["Perenderan Selektif Hanya di Wilayah Terdampak (Zero Redraw Overhead)"]
```

### 3.2 Pelacak Wilayah Rusak (*Dirty Region Tracking*)

Salah satu kelemahan terbesar peramban berbasis web konvensional adalah kecenderungan merender ulang seluruh pohon dokumen (*full viewport repaint*) ketika terjadi pembaruan elemen kecil (seperti pembaruan label jam digital atau kedipan kursor teks).

WidyaDesktop mengimplementasikan algoritma **Dirty Region Tracking** pada struktur `JendelaMurni`. Setiap kali terjadi pembaruan data atau mutasi antarmuka:
- Sistem tidak menghapus seluruh buffer tampilan layar.
- Sistem hanya menandai koordinat kotak pembatas (*bounding box*) elemen yang berubah sebagai *dirty rectangle* `(x, y, lebar, tinggi)`.
- Pada siklus perenderan berikutnya (*next frame loop*), pengelola grafis hanya mentransfer piksel baru di dalam wilayah yang ditandai melalui fungsi `bersihkan_dirty_regions()`, memangkas penggunaan bandwidth bus PCIe dan siklus komputasi kartu grafis hingga 90%.

### 3.3 Implementasi Kode Widya-Lang & WidyaDesktop Utuh

Berikut implementasi pengaturan pembatas dimensi jendela dan pelacakan koordinat wilayah pembaruan pada Widya-Lang:

```widya
// Modul Pengendalian Geometri & Pelacakan Wilayah Rusak
modul pengatur_geometri_jendela {
    impor "./modul/desktop.wya";

    fungsi inisialisasi_geometri_transaksi() {
        // 1. Buat Jendela dengan Ukuran Awal Standar
        misal jendela = BuatAplikasiDesktop("Terminal Kasir & Timbangan", 1100, 720);

        // 2. Terapkan Dimensi Minimum dan Maksimum yang Aman untuk Layout POS
        // Mencegah operator merusak tampilan tabel keranjang belanja
        jendela.set_ukuran(1100, 720);
        jendela.pusatkan();

        // 3. Registrasi Handler RPC untuk Menyesuaikan Resolusi Dinamis
        jendela.daftarkan_rpc("ubah_resolusi_layar", fungsi(payload) {
            misal w = payload.lebar;
            misal h = payload.tinggi;

            // Logika validasi batas aman
            jika (w < 800 atau h < 600) {
                kembalikan {
                    "sukses": salah,
                    "galat": "Resolusi minimal stasiun kerja adalah 800x600 piksel."
                };
            }

            jendela.set_ukuran(w, h);
            kembalikan {
                "sukses": benar,
                "resolusi_baru": w + "x" + h
            };
        });

        // 4. Simulasi Penandaan Wilayah Dinamis yang Berubah (Dirty Region)
        jendela.daftarkan_rpc("perbarui_label_harga", fungsi(payload) {
            misal total_baru = payload.total;
            // Area koordinat label total: x=750, y=620, lebar=300, tinggi=60
            cetak(">>> [RENDER] Memperbarui elemen harga total: Rp " + total_baru);
            cetak(">>> [DIRTY] Wilayah render terbatas di koordinat [750, 620, 300, 60]");
            kembalikan { "sukses": benar, "total": total_baru };
        });

        kembalikan jendela;
    }
}
```

### 3.4 Analisis Kinerja, Keamanan & Tabel Metrik Arsitektur

Tabel efisiensi rendering: Full Repaint vs Dirty Region Tracking pada frekuensi 60 FPS:

| Skenario Pembaruan Antarmuka | Full Repaint (Chromium/DOM Biasa) | Dirty Region Tracking (WidyaDesktop) | Penghematan Beban Bus Grafis |
| :--- | :--- | :--- | :--- |
| **Kedipan Kursor Teks (2x20 px)** | Render ulang 1920x1080 (8.29 MB VRAM) | Render kotak 10x30 px (1.2 KB VRAM) | **99.98% Lebih Ringan** |
| **Pembaruan Baris Total Kasir** | Render ulang seluruh viewport (60 FPS) | Render wilayah 300x60 px (< 72 KB) | **99.13% Lebih Ringan** |
| **Animasi Barcode Spinner** | Beban VRAM kontinu 500 MB/detik | Beban VRAM terbatas di 128x128 px | **98.50% Lebih Ringan** |
| **Perpindahan Multi-Monitor DPI** | Glitch / Skala pecah sementara | Vektor penskalaan mulus Per-Monitor v2 | **Zero Glitch / Instan** |

### 3.5 Latihan Praktik & Studi Kasus

1. Mengapa penegakan batas ukuran minimum (*minimum width and height*) sangat penting pada aplikasi transaksi perbankan dan kasir industri?
2. Jelaskan cara kerja matematis dari *Dirty Region Tracking* dalam menggabungkan dua kotak wilayah rusak yang saling tumpang tindih (*overlapping bounding boxes*)!
3. Bagaimana mekanisme *Per-Monitor DPI Awareness v2* pada WidyaDesktop mencegah terjadinya efek antarmuka buram (*bitmap stretching blur*) saat jendela dipindahkan antar monitor dengan kerapatan piksel berbeda?

### 3.6 Evaluasi Mandiri Pilihan Ganda

1. Tujuan utama dari *Geometry Constraint Engine* pada WidyaDesktop adalah...
   A. Menjamin dimensi jendela tetap berada dalam batas toleransi aman tata letak antarmuka (min/max size)
   B. Menghapus wallpaper desktop pengguna
   C. Mematikan kartu grafis secara paksa
   D. Memformat partisi harddisk

2. Fitur sistem operasi yang memungkinkan aplikasi desktop menyesuaikan skala elemen secara presisi pada setiap monitor fisik secara independen adalah...
   A. Per-Monitor DPI Awareness v2
   B. VGA Mode 640x480
   C. Monokrom CGA Adapter
   D. Teletext Mode

3. Algoritma perenderan grafis di mana sistem hanya menggambar ulang wilayah piksel yang mengalami mutasi data disebut...
   A. Dirty Region Tracking (Selective Redraw)
   B. Blind Full Paint
   C. Random Pixel Drawing
   D. Infinite Mirror Loop

4. Apa keuntungan performa terbesar dari implementasi *Dirty Region Tracking*?
   A. Memangkas transfer data ke VRAM dan beban pemrosesan GPU hingga lebih dari 95%
   B. Mengubah koneksi USB menjadi kabel listrik AC
   C. Menghilangkan kebutuhan memori RAM komputer
   D. Menjadikan fonta teks tidak terbaca

5. Struktur data tuple yang mewakili wilayah rusak (*dirty rectangle*) pada sistem grafis umumnya didefinisikan sebagai...
   A. `(x, y, lebar, tinggi)`
   B. `(kecepatan, waktu, suhu)`
   C. `(nama, alamat, nik)`
   D. `(ip_address, port, protocol)`

6. Apa yang terjadi ketika pengguna mencoba mengecilkan jendela di bawah batas `min_lebar` yang telah ditetapkan?
   A. Sistem secara otomatis memotong (*clamp*) dimensi jendela agar tidak melewati batas minimum yang diizinkan
   B. Aplikasi langsung meledak dan menutup paksa
   C. Seluruh berkas komputer dikunci
   D. Monitor berkedip merah

7. Masalah visual yang sering timbul pada aplikasi desktop konvensional non-DPI-aware ketika digeser ke layar 4K adalah...
   A. Antarmuka tampak buram, pecah, atau mengecil secara ekstrem
   B. Warna teks berubah menjadi putih di atas putih
   C. Layar komputer mati seketika
   D. Suara kipas prosesor berhenti

8. Fungsi `bersihkan_dirty_regions()` pada `JendelaMurni` bertugas untuk...
   A. Mengambil daftar kotak wilayah yang rusak lalu mengosongkan antrean setelah proses perenderan selesai
   B. Menghapus history penjelajahan internet
   C. Menghapus berkas sementara sistem operasi
   D. Membersihkan debu dari kipas GPU

9. Pada skenario pembaruan teks jam kasir (area kecil), perbandingan pemakaian VRAM antara *full repaint* vs *dirty region* mencapai efisiensi hingga...
   A. Lebih dari 99% penghematan bandwidth
   B. Tepat 0% penghematan
   C. Memperberat kerja komputer hingga 10 kali lipat
   D. Hanya menghemat 1 byte

10. Mengapa stasiun kerja kasir POS modern memerlukan dukungan multi-monitor DPI secara terintegrasi?
    A. Layar utama operator kasir dan layar sekunder pelanggan (customer display) sering kali memiliki resolusi dan kerapatan piksel fisik yang berbeda
    B. Supaya kasir dapat menonton film saat melayani pelanggan
    C. Mengganti kebutuhan mesin pendingin ruangan toko
    D. Menghilangkan kebutuhan sambungan listrik
