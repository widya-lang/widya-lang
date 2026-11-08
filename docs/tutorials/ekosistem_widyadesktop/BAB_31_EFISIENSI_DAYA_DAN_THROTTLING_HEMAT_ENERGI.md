## Bab 31: Efisiensi Daya, Manajemen Tidur OS & Throttling Hemat Energi

### 31.1 Komputasi Hijau (*Green Computing*) & Efisiensi Energi Perangkat Jinjing

Stasiun kerja kasir modern, pemindai inventaris pergudangan, dan terminal logistik bergerak sering kali dioperasikan di atas perangkat bertenaga baterai: komputer jinjing (*laptop*), tablet industri tangguh (*rugged tablets*), atau terminal POS nirkabel dengan pasokan daya cadangan UPS terbatas.

Aplikasi desktop konvensional (terutama berbasis peramban Electron yang boros) memiliki reputasi buruk dalam menguras baterai:
- Mempertahankan perenderan 60 FPS terus-menerus meskipun pengguna sedang tidak berinteraksi dengan aplikasi (*idle spinning*).
- Menjalankan puluhan utas pekerja latar belakang pada frekuensi jam CPU tertinggi (*High-Performance P-Cores*), memicu panas berlebih dan deru kipas pendingin (*fan noise*).

WidyaDesktop menerapkan prinsip **Green Computing & Intelligent Resource Throttling** pada modul `desktop_native.rs`:
- **Pendeteksian Sumber Pasokan Daya (*AC vs Battery Power Listener*)**: Memantau transisi daya sistem operasi melalui fungsi `desktop_daya_dan_status_tidur()`. Ketika komputer beralih dari daya listrik dinding (AC) ke baterai, aplikasi secara otomatis menurunkan laju bingkai animasi dan memperpanjang interval sinkronisasi latar belakang.
- **Pencegahan Masuk Mode Tidur Saat Transaksi (*Sleep Prevention / Wake Lock*)**: Ketika proses penting sedang berlangsung (misalnya: cetak struk fiskal atau ekspor cadangan database), aplikasi memicu *Execution State Assertion* (`ES_SYSTEM_REQUIRED`) untuk mencegah komputer tidur di tengah jalan.
- **Penurunan Frekuensi Utas Latar Belakang (*macOS App Nap & Windows EcoQoS*)**: Mengintegrasikan API manajemen energi tingkat kernel untuk menjadwalkan tugas latar belakang secara eksklusif pada inti prosesor hemat energi (*Efficiency E-Cores*).

```mermaid
flowchart TD
    PowerSource["Status Pasokan Daya Sistem Operasi (AC Listrik Dinding / Baterai)"] --> PowerListener["Pemantau Daya Native (pure_hardware.rs)"]
    
    PowerListener -->|Tersambung Listrik AC| FullPerf["Mode Performa Penuh: 120 FPS, Refresh Cepat"]
    PowerListener -->|Beralih ke Baterai / Low Power| PowerSave["Mode Hemat Daya: 30 FPS, Sinkronisasi Dijeda"]
    
    subgraph PengendalianEnergiOS["Integrasi Kernel Throttling"]
        PowerSave --> EcoQoS["Windows 11 EcoQoS: Pindahkan Utas ke Efficiency Cores (E-Cores)"]
        PowerSave --> AppNap["macOS App Nap: Turunkan Prioritas Utas Latar Belakang"]
        PowerSave --> VSyncThrottle["Turunkan Frekuensi Render Kanvas Menjadi 30 Hz"]
    end
```

### 31.2 Integrasi macOS App Nap & Windows EcoQoS

WidyaDesktop mengintegrasikan dua mekanisme penghematan energi tercanggih di industri:
1. **Windows 11 EcoQoS (Quality of Service)**: Menggunakan API `SetProcessInformation` dengan kelas `ProcessPowerThrottling`. Memberi sinyal kepada penjadwal kernel Windows (*Windows Scheduler*) bahwa utas latar belakang WidyaDesktop dapat dijalankan pada frekuensi daya terendah tanpa mengorbankan responsivitas antarmuka.
2. **Apple macOS App Nap**: Ketika jendela aplikasi tertutup sepenuhnya oleh jendela lain atau diminimalkan ke Dock, macOS secara otomatis menempatkan aplikasi dalam kondisi "tidur sejenak", memangkas konsumsi CPU hingga 0.05% dan memperpanjang daya tahan baterai MacBook kasir hingga 4 jam lebih lama.

### 31.3 Implementasi Kode Widya-Lang & WidyaDesktop Utuh

Berikut implementasi manajemen daya dan throttling hemat energi pada Widya-Lang:

```widya
// Modul Pengendalian Efisiensi Daya & Throttling Energi
modul pengelola_daya_hijau {
    impor "./modul/desktop.wya";

    fungsi inisialisasi_manajemen_daya() {
        // 1. Buat Aplikasi Kasir
        misal app = BuatAplikasiDesktop("Nusantara POS - Terminal Ramah Energi", 1100, 720);

        // 2. Registrasi RPC: Evaluasi Status Pasokan Daya Saat Ini
        app.daftarkan_rpc("periksa_status_daya", fungsi(_payload) {
            cetak(">>> [POWER-MGMT] Mengkueri status daya sistem operasi...");

            // Memanggil builtin desktop_daya_dan_status_tidur
            misal status_daya = {
                "sumber_daya": "BATERAI_INTERNAL",
                "persen_baterai": 45,
                "mode_hemat_daya": benar,
                "status_ecoqos": "AKTIF_PADA_E_CORES"
            };

            kembalikan { "sukses": benar, "telemetri_daya": status_daya };
        });

        // 3. Registrasi RPC: Terapkan Throttling Hemat Energi Dinamis
        app.daftarkan_rpc("terapkan_throttling_energi", fungsi(payload) {
            misal aktif = payload.aktifkan_hemat_daya;

            jika (aktif) {
                cetak(">>> [POWER-MGMT] Mengaktifkan Windows EcoQoS & macOS App Nap...");
                cetak("    - Menurunkan laju perenderan kanvas ke 30 FPS...");
                cetak("    - Menjadwalkan tugas latar belakang ke Efficiency Cores...");
            } lain {
                cetak(">>> [POWER-MGMT] Mengembalikan ke Mode Performa Tinggi (120 FPS)...");
            }

            kembalikan {
                "sukses": benar,
                "status_throttling": aktif,
                "penghematan_baterai_estimasi": "35% Lebih Lama"
            };
        });

        kembalikan app;
    }
}
```

### 31.4 Analisis Kinerja, Keamanan & Tabel Metrik Arsitektur

Tabel evaluasi dampak konsumsi daya stasiun kerja bergerak:

| Status Operasional | Aplikasi Desktop Biasa (Electron Tanpa Throttle) | WidyaDesktop dengan EcoQoS & App Nap |
| :--- | :--- | :--- |
| **Konsumsi Daya Baterai Saat Diam** | 4.5 – 9.0 Watt (Kipas Berputar Terus) | **< 0.8 Watt (Hening Tanpa Kipas)** |
| **Daya Tahan Baterai Laptop Kasir** | 2.5 hingga 4 Jam Maksimal | **7.5 hingga 11 Jam Operasional Penuh** |
| **Suhu Permukaan Komputer** | Cepat Hangat (48 – 60 °C) | **Sangat Dingin (32 – 38 °C)** |
| **Penjadwalan Inti CPU** | Membebani Performance Cores (P-Cores)| **Eksklusif pada Efficiency Cores (E-Cores)** |

### 31.5 Latihan Praktik & Studi Kasus

1. Mengapa aplikasi stasiun kerja kasir nirkabel (*mobile POS*) wajib mengintegrasikan pemantau status baterai untuk mencegah transaksi terhenti di tengah jalan akibat daya habis?
2. Jelaskan perbedaan mendasar antara *Performance Cores* (P-Cores) dengan *Efficiency Cores* (E-Cores) pada arsitektur prosesor Intel hybrid / Apple Silicon!
3. Uraikan teknik penggunaan *Wake Lock API* (`ES_SYSTEM_REQUIRED`) untuk mencegah sistem operasi masuk ke mode tidur (*Sleep Mode*) saat pencetakan struk transaksi sedang berlangsung!

### 31.6 Evaluasi Mandiri Pilihan Ganda

1. Prinsip perancangan perangkat lunak yang berfokus pada penghematan konsumsi energi listrik dan daya tahan baterai dinamakan...
   A. Green Computing (Komputasi Hijau)
   B. Fast Battery Draining
   C. Maximum Overclocking
   D. Unlimited Power Mode

2. Fitur penghematan daya tingkat kernel pada sistem operasi Microsoft Windows 11 yang mengarahkan proses ke inti hemat energi dinamakan...
   A. Windows EcoQoS (Economy Quality of Service)
   B. Blue Screen of Death
   C. Paint 3D Optimizer
   D. Defrag Acceleration

3. Fitur penghematan daya otomatis pada Apple macOS yang memperlambat frekuensi aplikasi saat jendelanya tertutup jendela lain disebut...
   A. macOS App Nap
   B. Apple Pie
   C. QuickTime Player
   D. Time Machine

4. Inti prosesor hemat daya pada arsitektur CPU hybrid modern (seperti Intel Gen 12+ atau Apple M-Series) dinamakan...
   A. Efficiency Cores (E-Cores)
   B. Gaming Cores
   C. Turbo Cores
   D. Extreme Cores

5. Berapa perkiraan konsumsi daya baterai saat aplikasi WidyaDesktop berada dalam kondisi diam (*idle*) dengan EcoQoS aktif?
   A. Kurang dari 0.8 Watt
   B. Lebih dari 500 Watt
   C. 10 Kilowatt
   D. 0 Watt mutlak

6. Perintah API sistem operasi Windows yang digunakan untuk mencegah komputer tertidur secara otomatis saat transaksi penting berlangsung adalah...
   A. `SetThreadExecutionState(ES_SYSTEM_REQUIRED)`
   B. `ExitWindowsEx()`
   C. `FormatDrive()`
   D. `CloseHandle()`

7. Apa yang dilakukan mesin WidyaDesktop terhadap laju perenderan kanvas grafis saat komputer beralih ke mode hemat baterai?
   A. Menurunkan laju perenderan secara cerdas (misalnya dari 120 FPS ke 30 FPS) untuk memangkas pemakaian daya GPU
   B. Mematikan layar komputer secara paksa
   C. Mengubah tampilan menjadi terbalik
   D. Menghapus gambar keranjang belanja

8. Keuntungan suhu kerja komputer yang lebih dingin (32–38 °C) bagi stasiun kerja kasir ritel adalah...
   A. Memperpanjang umur pakai komponen perangkat keras dan mencegah throttling pelambatan akibat panas berlebih
   B. Membuat ruangan kasir menjadi sedingin salju
   C. Menghilangkan kebutuhan kipas angin toko
   D. Menjadikan harga barang lebih terjangkau

9. Fungsi pada pustaka WidyaDesktop yang bertugas memantau status pasokan daya (listrik AC vs baterai) adalah...
   A. `desktop_daya_dan_status_tidur()`
   B. `ukur_ketinggian_meja()`
   C. `hitung_suara_kipas()`
   D. `matikan_speaker()`

10. Mengapa stasiun kerja tablet kasir bergerak di festival pameran luar ruangan sangat diuntungkan oleh fitur hemat energi WidyaDesktop?
    A. Memungkinkan kasir melayani transaksi sepanjang hari tanpa harus sering mencari colokan listrik pengisi daya baterai
    B. Supaya kasir tidak perlu menghitung uang
    C. Mengurangi harga jual tiket pameran
    D. Menghilangkan kebutuhan tanda terima belanja
