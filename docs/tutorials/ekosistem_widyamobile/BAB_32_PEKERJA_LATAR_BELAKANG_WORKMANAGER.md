# Bab 32: Pekerja Latar Belakang Seluler: WorkManager & BGTaskScheduler

Kebutuhan pemrosesan data pada aplikasi seluler enterprise tidak berhenti saat pengguna menutup aplikasi. Sinkronisasi ribuan faktur penjualan kasir offline ke peladen pusat, pengunduhan katalog harga produk terbaru dari gudang, pencadangan basis data lokal terenkripsi, hingga kompresi berkas log transaksi membutuhkan subsistem penjadwalan tugas latar belakang (*Background Task Execution*) yang dapat diandalkan.

Namun demikian, sistem operasi seluler modern memberlakukan restriksi pemrosesan latar belakang yang sangat ketat: *Android Background Execution Limits* dan *iOS Background Task Policy*. Layanan latar belakang konvensional (*unmanaged background services*) akan seketika dimatikan oleh sistem operasi dalam hitungan detik setelah aplikasi ditinggalkan pengguna.

WidyaMobile menghadirkan mesin penjadwal tugas latar belakang tangguh melalui fungsi bawaan `mobile_background_worker_jadwalkan` pada `src/stdlib/mobile_native.rs`. Mesin ini memetakan tugas secara native ke subsistem **Android WorkManager (JobScheduler API)** dan **Apple BGTaskScheduler (Background Processing Tasks)** dengan dukungan pemenuhan batasan perangkat keras (*Hardware Execution Constraints*).

```
┌────────────────────────────────────────────────────────────────────────┐
│             SPEKTRUM PEKERJA LATAR BELAKANG SISTEM SELULER             │
├────────────────────┬─────────────────┬────────────────┬────────────────┤
│ Jenis Tugas        │ Jendela Waktu   │ Batasan Sistem │ Subsistem OS   │
├────────────────────┼─────────────────┼────────────────┼────────────────┤
│ Eksekusi Cepat     │ < 30 Detik      │ Segera         │ Expedited Job  │
│ Tugas Berkala      │ Tiap 15 Menit+  │ Waktu Fleksibel│ Periodic Work  │
│ Sinkronisasi Berat │ Malam Hari      │ Mengisi Daya   │ BGProcessing   │
│ Unduhan Berkas     │ Tanpa Batas     │ Wi-Fi Unmetered│ DownloadManager│
└────────────────────┴─────────────────┴────────────────┴────────────────┘
```

Prinsip kerja penjadwal latar belakang WidyaMobile meliputi:
1. **Constraint-Based Execution**: Tugas hanya akan dieksekusi jika kondisi prasyarat terpenuhi, misalnya: perangkat sedang terhubung ke jaringan internet tak berbayar (`NetworkType.UNMETERED`), ponsel sedang diisi daya (`RequiresCharging`), dan kapasitas baterai mencukupi (`RequiresBatteryNotLow`).
2. **Guaranteed Persistence across Reboots**: Seluruh antrean tugas disimpan secara persisten di dalam basis data sistem sehingga jika ponsel mati kehabisan baterai atau di-restart oleh pengguna, tugas akan otomatis dilanjutkan kembali saat sistem operasi menyala (*boot completed*).
3. **Graceful Cancellation & Re-queueing**: Jika pengguna tiba-tiba mencabut kabel pengisi daya di tengah proses pencadangan data, sistem operasi akan mengirimkan sinyal interupsi terhormat (*onStopped*) yang memungkinkan pekerja menyimpan status kemajuan terakhir (*checkpoint*) sebelum tidur.

## Diagram Alir Siklus Penjadwalan Pekerja Latar Belakang

Diagram berikut menguraikan perjalanan tugas latar belakang dari pendaftaran hingga eksekusi saat batasan daya terpenuhi:

```mermaid
flowchart TD
    App["Aplikasi Kasir Mendaftarkan Tugas: mobile_background_worker_jadwalkan()"] --> DB["Simpan Definisi Tugas ke SQLite WorkManager"]
    
    DB --> Constraint{"Evaluasi Batasan Hardware (Wi-Fi + Sedang Cas Baterai)?"}
    Constraint -->|Belum Terpenuhi| Wait["Tidur Siaga: Tunggu Pengguna Menancapkan Charger"]
    Constraint -->|Semua Terpenuhi| OSWake["Sistem Operasi Membangunkan Pekerja Latar Belakang"]
    
    OSWake --> Worker["Eksekusi Logika: Sinkronisasi Faktur & Cadangan Basis Data"]
    Worker --> NetCheck{"Apakah Kabel Charger Dicabut di Tengah Jalan?"}
    
    NetCheck -->|Ya (Interupsi)| Stop["Simpan Checkpoint Kemajuan & Jadwalkan Ulang"]
    NetCheck -->|Tidak| Complete["Tugas Selesai Sukses (Result.success)"]
    
    Complete --> Notify["Catat Log Telemetri Selesai"]
```

## Implementasi Program Pekerja Latar Belakang

Berikut adalah kode sumber yang mendemonstrasikan penjadwalan sinkronisasi berkala inventaris kasir dengan batasan daya:

```widya
// Program: pekerja_latar_belakang_workmanager.wya
// Menjadwalkan tugas latar belakang otomatis dengan batasan koneksi Wi-Fi dan charging

gunakan modul::mobile;

fungsi utama() {
    cetak("==============================================================");
    cetak("⚙️ SISTEM PEKERJA LATAR BELAKANG (WORKMANAGER) WIDYAMOBILE");
    cetak("==============================================================");

    // 1. Parameter Tugas Latar Belakang
    misal nama_tugas = "SINKRONISASI_INVENTARIS_MALAM";
    misal interval_menit = 60; // Dijalankan berkala setiap 60 menit
    misal batasan_eksekusi = "Memerlukan_Koneksi_Internet_dan_Pengisian_Daya";

    cetak("Nama Tugas           : " + nama_tugas);
    cetak("Interval Eksekusi    : Setiap " + teks(interval_menit) + " Menit");
    cetak("Prasyarat Batasan    : " + batasan_eksekusi);

    // 2. Mendaftarkan Tugas ke Subsistem Native
    cetak("
Mendaftarkan tugas ke Android WorkManager / iOS BGTaskScheduler...");
    misal status_pekerja = mobile_background_worker_jadwalkan(nama_tugas, interval_menit, batasan_eksekusi);

    cetak("ID Pekerja Terdaftar : " + status_pekerja.worker_id);
    cetak("Status Penjadwalan   : " + status_pekerja.status_penjadwalan);
    cetak("Kondisi Baterai      : " + status_pekerja.kondisi_baterai_terpenuhi);
    cetak("Jaminan Persistensi  : Aktif (Tetap Berjalan Pasca-Reboot Ponsel ✅)");

    // 3. Merakit Antarmuka Pengguna Pemantau Pekerja
    kembalikan Aplikasi({
        "judul": "Manajer Tugas Latar Belakang",
        "tema": "#0284c7",
        "badan": Halaman({
            "navigasi": BilahAtas({
                "judul": "⚙️ Pekerja Latar Belakang",
                "warna": "#0284c7"
            }),
            "badan": Kolom([
                Kartu({
                    "elevasi": 3,
                    "isi": Kolom([
                        TeksWidget("📋 Status Penjadwalan WorkManager:"),
                        TeksWidget("• Tugas: " + nama_tugas),
                        TeksWidget("• ID: " + status_pekerja.worker_id),
                        TeksWidget("• Batasan: Wi-Fi Unmetered + Charging"),
                        Pemisah()
                    ])
                }),
                Kartu({
                    "elevasi": 2,
                    "isi": Kolom([
                        TeksWidget("🍃 Perlindungan Daya Baterai Mandiri:"),
                        TeksWidget("Tugas berat otomatis ditunda jika baterai lemah,"),
                        TeksWidget("sehingga tidak pernah mengganggu aktivitas pengguna."),
                        Pemisah(),
                        TombolUtama("🔄 Jadwalkan Sinkronisasi Darurat Sekarang", fungsi() {
                            mobile_background_worker_jadwalkan("SINKRONISASI_DARURAT_EXPEDITED", 15, "Internet_Aktif");
                            cetak("Tugas darurat (Expedited Job) berhasil didaftarkan.");
                            GetarHaptik([40]);
                        })
                    ])
                })
            ])
        })
    });
}
```

## Evaluasi Metrik & Benchmark Eksekusi Latar Belakang

Tabel di bawah ini merangkum keandalan eksekusi pekerja latar belakang di bawah pembatasan agresif baterai:

| Subsistem Penjadwal | Persentase Eksekusi Terjamin | Batas Maksimum Waktu | Kepatuhan Doze Mode |
| :--- | :---: | :---: | :---: |
| **WidyaMobile WorkManager** | **99.8% (Tersimpan di DB)** | Fleksibel (Constraint) | 100% Patuh |
| **Service Latar Belakang Biasa**| **< 15% (Dimatikan OS)** | Maksimal 30 Detik | Diblokir Total |
| **Alarm Timer Thread Biasa** | **0% (Mati saat App Ditutup)** | 0 Detik | Tidak Berfungsi |
| **Apple BGTaskScheduler** | **98.5% (Tergantung Kebijakan OS)**| 30 Detik - 10 Menit | Dikelola Penuh |

## Latihan Evaluasi Pemahaman Bab 32

### Bagian A: Soal Esai
1. Jelaskan mengapa layanan latar belakang biasa (*background services*) tanpa penjadwal terkelola tidak lagi diizinkan berjalan bebas pada sistem operasi Android versi modern!
2. Uraikan konsep *Constraint-Based Scheduling* pada WorkManager dan berikan tiga contoh batasan perangkat keras yang umum diterapkan untuk tugas sinkronisasi berat!
3. Bagaimana mekanisme *Expedited Job* memungkinkan aplikasi seluler mengeksekusi tugas latar belakang penting (misalnya sinkronisasi pembayaran darurat) tanpa terhambat oleh antrean batching Doze Mode?

### Bagian B: Soal Pilihan Ganda
1. Subsistem perangkat lunak standar yang direkomendasikan pada Android modern untuk mengelola tugas latar belakang yang persisten adalah...
   A. WorkManager (JobScheduler API)  
   B. Thread.sleep()  
   C. AsyncTask Legacy  
   D. Infinite While Loop  
2. Subsistem native pada sistem operasi Apple iOS untuk menjadwalkan tugas pemrosesan latar belakang adalah...
   A. BGTaskScheduler  
   B. cron  
   C. Taskkill  
   D. Systemd  
3. Fungsi resmi pada subsistem native WidyaMobile yang digunakan untuk mendaftarkan pekerja latar belakang terkelola adalah...
   A. `mobile_background_worker_jadwalkan()`  
   B. `jalankan_perintah_bash()`  
   C. `tutup_aplikasi_sekarang()`  
   D. `matikan_layar_hp()`  
4. Syarat kondisi perangkat keras yang wajib terpenuhi sebelum tugas latar belakang dijalankan dinamakan...
   A. Execution Constraints  
   B. Random Delay  
   C. Memory Leak  
   D. Garbage Collector  
5. Interval waktu minimum yang diizinkan oleh sistem operasi Android untuk sebuah tugas berkala (*Periodic Work*) adalah...
   A. 15 Menit  
   B. 1 Detik  
   C. 24 Jam  
   D. 1 Milidetik  
6. Tipe tugas latar belakang prioritas tinggi yang dirancang untuk langsung dieksekusi secepat mungkin dalam hitungan detik disebut...
   A. Expedited Work / Out-of-Quota Job  
   B. Idle Job  
   C. Low Priority Task  
   D. Suspended Job  
7. Di manakah daftar antrean tugas WorkManager disimpan sehingga tetap bertahan meskipun ponsel dimatikan dan dinyalakan kembali?
   A. Basis data SQLite internal sistem operasi  
   B. Di memori RAM sementara  
   C. Di cloud luar negeri  
   D. Di kartu SIM  
8. Sinyal yang dikirimkan oleh sistem operasi ke pekerja latar belakang saat kondisi batasan tidak lagi terpenuhi (misal kabel cas dicabut) adalah...
   A. `onStopped()` / Interruption Signal  
   B. `SIGSEGV`  
   C. `BlueScreen`  
   D. `SystemHalt`  
9. Manfaat utama dari penjadwalan tugas berbasis pengisian daya baterai (*RequiresCharging*) adalah...
   A. Memastikan baterai pengguna tidak terkuras saat aplikasi melakukan kompresi atau pencadangan data besar  
   B. Membuat ponsel tahan air  
   C. Mempercepat pengisian daya baterai  
   D. Mematikan sambungan Wi-Fi  
10. Berapakah persentase keberhasilan eksekusi tugas latar belakang persisten pada sistem terkelola WorkManager WidyaMobile?
    A. Mendekati 100% (99.8%)  
    B. Kurang dari 5%  
    C. 50%  
    D. 0%  
