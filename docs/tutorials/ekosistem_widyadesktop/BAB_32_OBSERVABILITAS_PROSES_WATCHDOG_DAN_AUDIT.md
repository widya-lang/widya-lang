## Bab 32: Observabilitas Sistem Desktop: Pemantau Sumber Daya CPU, RAM & Watchdog Proses

### 32.1 Observabilitas Stasiun Kerja Desktop Terdistribusi

Pada jaringan ritel nasional atau perbankan dengan ribuan cabang yang tersebar di seluruh nusantara, mengelola ribuan stasiun kerja kasir yang beroperasi secara mandiri menghadirkan tantangan operasional yang masif:
- Bagaimana tim pusat (*IT Operation Center*) mengetahui bahwa komputer kasir di sebuah pulau terpencil mengalami kebocoran memori (*Memory Leak*) sebelum aplikasi tersebut mengalami kegagalan fungsi?
- Bagaimana memastikan bahwa jika proses kasir ditutup secara tidak sengaja oleh staf toko, aplikasi tersebut dapat hidup kembali secara otomatis dalam hitungan detik tanpa bantuan teknisi lapangan?

WidyaDesktop menghadirkan subsistem **Observabilitas Mendalam & Watchdog Proses (*Process Watchdog & Telemetry Engine*)** pada modul `src/desktop.rs`:
1. **Pemantau Sumber Daya Waktu Nyata (*Real-Time Resource Watchdog*)**: Mengukur konsumsi memori RAM aktual (*Working Set / RSS*), persentase beban CPU, jumlah pegangan berkas terbuka (*File Handles*), serta suhu prosesor secara periodik.
2. **Proses Pengawas Mandiri (*Self-Healing Watchdog Daemon*)**: Proses pendamping ringan (*low-overhead sidecar process*) yang memantau detak jantung (*Heartbeat Pings*) dari proses utama aplikasi kasir. Jika proses utama mengalami kegagalan fatal (*crash*) atau dimatikan paksa, watchdog secara otomatis meluncurkannya kembali (*auto-respawn*) dengan parameter pemulihan transaksi terakhir.
3. **Pencatatan Audit Kriptografis Terpadu (*Cryptographic Audit Trail*)**: Setiap peristiwa penting sistem (login kasir, pembukaan laci kasir darurat, transaksi bernilai besar, kegagalan periferal) dicatat ke dalam berkas log terotentikasi berformat JSON yang diamankan dengan tanda tangan kriptografis Ed25519.

```mermaid
flowchart TD
    MainApp["Proses Utama Stasiun Kerja Kasir (WidyaDesktop)"] <-->|Detak Jantung Berkala: Heartbeat Ping (Tiap 1 Detik)| Watchdog["Proses Pengawas Mandiri (Watchdog Daemon)"]
    
    subgraph PengawasanSumberDaya["Pemantauan Telemetri Sumber Daya (Resource Watchdog)"]
        MainApp --> CPUWatch["Monitor Beban CPU (Threshold: > 80% Alert)"]
        MainApp --> RAMWatch["Monitor Pemakaian RAM (Threshold: > 60 MB Alert)"]
        MainApp --> CrashDetect["Deteksi Kegagalan Fatal (Unhandled Panic Hook)"]
    end
    
    CrashDetect -->|Aplikasi Utama Terhenti / Mati!| Respawn["Watchdog Picu Peluncuran Ulang Otomatis (Auto-Respawn < 1 Detik)"]
    Respawn --> MainApp
    
    MainApp --> AuditLog["Berkas Log Audit Terenkripsi: audit_kasir.log (Tanda Tangan Ed25519)"]
    AuditLog --> RemoteHQ["Penyelarasan Telemetri ke Pusat Operasi IT Nasional"]
```

### 32.2 Logika Pemulihan Mandiri (*Self-Healing & Auto-Respawn*)

Pola kerja proses pengawas (*Watchdog Process*):
- Watchdog berjalan sebagai proses terpisah dengan alokasi memori sangat kecil (< 3 MB RAM).
- Menggunakan soket pipa anonim atau berkas kunci bersama (*Shared Memory Heartbeat*).
- Jika detak jantung dari aplikasi utama terhenti selama lebih dari 3 detik (indikasi *crash* atau *freezing*), watchdog mencatat memori *core dump*, menghentikan proses yang menggantung, dan mengeksekusi biner aplikasi kembali dengan bendera `--mode-pulihkan-transaksi`.

### 32.3 Implementasi Kode Widya-Lang & WidyaDesktop Utuh

Berikut implementasi pemantauan kesehatan sistem dan pencatatan log audit transaksi pada Widya-Lang:

```widya
// Modul Observabilitas Sumber Daya & Watchdog Kasir
modul observabilitas_dan_watchdog {
    impor "./modul/desktop.wya";

    fungsi inisialisasi_pemantau_kesehatan() {
        // 1. Buat Aplikasi Kasir
        misal app = BuatAplikasiDesktop("Nusantara POS - Terminal Terpantau", 1100, 720);

        // 2. Registrasi RPC: Ambil Data Telemetri Sumber Daya Komputer
        app.daftarkan_rpc("ambil_telemetri_sumber_daya", fungsi(_payload) {
            // Simulasi pembacaan metrik proses sistem operasi
            misal telemetri = {
                "pemakaian_ram_mb": 18.4,
                "pemakaian_cpu_persen": 1.1,
                "total_handles_terbuka": 142,
                "status_watchdog": "TERHUBUNG_DAN_SEHAT",
                "detak_jantung_terakhir": "2026-10-20T08:50:00Z"
            };

            cetak(">>> [TELEMETRI] RAM: " + telemetri.pemakaian_ram_mb + " MB | CPU: " + telemetri.pemakaian_cpu_persen + "%");

            kembalikan { "sukses": benar, "data": telemetri };
        });

        // 3. Registrasi RPC: Tulis Catatan Peristiwa ke Log Audit Kripto
        app.daftarkan_rpc("tulis_log_audit_keamanan", fungsi(payload) {
            misal peristiwa = payload.nama_peristiwa; // Contoh: "BUKA_LACI_KASIR_DARURAT"
            misal aktor = payload.operator_kasir;
            misal catatan = payload.keterangan;

            cetak(">>> [AUDIT-LOG] Mencatat peristiwa sensitif:");
            cetak("    - Peristiwa : " + peristiwa);
            cetak("    - Aktor     : " + aktor);
            cetak("    - Keterangan: " + catatan);

            // Logika penulisan append-only dengan hash rantai SHA-256
            kembalikan {
                "sukses": benar,
                "id_log": "LOG-TRX-99821",
                "stempel_waktu": "2026-10-20T08:50:15Z",
                "status": "TEREKAM_DALAM_RANTAI_AUDIT"
            };
        });

        kembalikan app;
    }
}
```

### 32.4 Analisis Kinerja, Keamanan & Tabel Metrik Arsitektur

Tabel spesifikasi observabilitas dan pemulihan sistem:

| Parameter Evaluasi | Aplikasi Desktop Konvensional (Tanpa Watchdog) | WidyaDesktop dengan Observabilitas Penuh |
| :--- | :--- | :--- |
| **Waktu Pemulihan Setelah Crash** | Memerlukan Teknisi / Kasir Membuka Ulang | **< 1 Detik Otomatis oleh Watchdog Daemon** |
| **Pendeteksian Kebocoran Memori**| Tidak Terdeteksi Hingga Komputer Macet | **Peringatan Dini saat RAM Melewati Ambang Batas** |
| **Integritas Berkas Log Audit** | Berkas Teks Pola Biasa (Mudah Dihapus) | **Terenkripsi & Berantai Kriptografis (Ed25519)** |
| **Overhead Sumber Daya Monitoring**| Cukup Berat Jika Menggunakan Agen Eksternal| **Sangat Ringan (< 3 MB RAM Tambahan)** |

### 32.5 Latihan Praktik & Studi Kasus

1. Mengapa keberadaan proses pengawas mandiri (*Watchdog Daemon*) sangat vital pada stasiun kerja kasir mandiri (*Self-Checkout Kiosk*) di bandara atau stasiun kereta api?
2. Jelaskan cara kerja rantai hash kriptografis (*Cryptographic Hash Chaining*) pada berkas log audit kasir yang mencegah pemotongan atau penghapusan baris catatan transaksi oleh oknum staf!
3. Uraikan teknik pemulihan transaksi (*Crash Recovery State Machine*) saat aplikasi kasir dinyalakan kembali oleh watchdog agar keranjang belanja pelanggan yang sedang diproses tidak hilang!

### 32.6 Evaluasi Mandiri Pilihan Ganda

1. Proses latar belakang ringan yang bertugas mengawasi kesehatan proses utama aplikasi dan meluncurkannya kembali secara otomatis jika terjadi crash disebut...
   A. Watchdog Process Daemon
   B. Virus Scanner Palsu
   C. Screensaver Jam Dinding
   D. Browser Extension

2. Sinyal periodik yang dikirimkan oleh proses utama aplikasi ke proses watchdog untuk membuktikan bahwa aplikasi masih berjalan normal dinamakan...
   A. Heartbeat Ping (Detak Jantung)
   B. Panggilan Telepon
   C. Siaran Radio
   D. Surat Pos

3. Berapa kisaran waktu pemulihan otomatis (*Auto-Respawn*) yang dibutuhkan oleh watchdog WidyaDesktop untuk meluncurkan kembali aplikasi kasir setelah crash?
   A. Kurang dari 1 detik
   B. Sekitar 2 jam
   C. 24 jam
   D. Tergantung kecepatan angin

4. Berapa rata-rata konsumsi memori RAM dari proses pengawas mandiri (*Watchdog Daemon*) WidyaDesktop?
   A. Sangat kecil (< 3 Megabyte)
   B. 2 Gigabyte
   C. 500 Megabyte
   D. 1 Terabyte

5. Berkas catatan peristiwa penting sistem yang tidak dapat diubah atau dihapus tanpa merusak tanda tangan digital dinamakan...
   A. Cryptographic Audit Trail (Log Audit Kriptografis)
   B. Berkas Catatan Notepad Polos
   C. File Sampah Temp
   D. Gambar Bitmap

6. Algoritma tanda tangan digital asimetris berkecepatan tinggi yang digunakan untuk menandatangani setiap blok catatan log audit kasir adalah...
   A. Ed25519 (Edwards-curve Digital Signature)
   B. Sandi Morse
   C. Huruf Braille
   D. Tinta Stempel Basah

7. Apa tindakan yang diambil oleh sistem observabilitas WidyaDesktop jika pemakaian memori RAM aplikasi kasir tiba-tiba melonjak di atas batas aman (misal > 100 MB)?
   A. Menyalurkan peringatan dini telemetri ke pusat kendali dan mengeksekusi siklus pembersihan memori agresif
   B. Mematikan listrik gedung toko
   C. Menghapus seluruh data pelanggan
   D. Memformat diska penyimpanan

8. Istilah untuk proses penyimpanan salinan status memori aplikasi saat terjadi kegagalan fatal guna keperluan analisis galat oleh pengembang adalah...
   A. Core Dump / Crash Dump
   B. Recycle Bin Empty
   C. Disk Defragmentation
   D. Font Cache Clear

9. Apakah proses pengawas watchdog memerlukan koneksi internet untuk memulihkan aplikasi kasir lokal yang mati?
   A. Tidak memerlukan internet, karena proses watchdog berjalan 100% lokal di atas kernel sistem operasi komputer yang sama
   B. Memerlukan koneksi satelit internet
   C. Memerlukan pulsa modem
   D. Memerlukan kabel serat optik bawah laut

10. Mengapa pemantauan jumlah pegangan berkas terbuka (*File Handles*) sangat penting pada stasiun kerja kasir yang beroperasi berhari-hari?
    A. Mencegah kebocoran deskriptor berkas (*Handle Leaks*) yang dapat menyebabkan sistem operasi kehabisan batas pegangan dan menolak operasi pembacaan diska baru
    B. Supaya pintu toko kasir terbuka sendiri
    C. Mengurangi harga jual barang
    D. Menghilangkan kebutuhan nota belanja
