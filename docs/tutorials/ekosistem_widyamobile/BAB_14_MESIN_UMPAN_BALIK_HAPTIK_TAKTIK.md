# Bab 14: Mesin Umpan Balik Haptik Taktil: LRA, Taptic Engine & Gelombang Khusus

Sebuah aplikasi seluler yang luar biasa tidak hanya memanjakan mata melalui tampilan visual dan telinga melalui efek suara, melainkan juga berkomunikasi dengan indra peraba pengguna melalui getaran haptik taktil (*tactile haptic feedback*). Di era tombol layar sentuh yang serba datar, haptik adalah satu-satunya jembatan fisik yang meyakinkan pengguna bahwa sebuah ketukan tombol telah berhasil terdaftar, pemindaian QRIS berhasil diproses, atau terjadi kesalahan input formulir.

Perangkat seluler modern telah meninggalkan motor getar mekanis eksentrik kuno (*Eccentric Rotating Mass - ERM*) yang bergetar kasar dan lambat, beralih ke aktuator getar resonan linier canggih: **Linear Resonant Actuators (LRA)** pada perangkat Android unggulan dan **Taptic Engine** pada Apple iPhone. Aktuator ini mampu merespons dalam hitungan 5 milidetik dan menghasilkan spektrum gelombang getar (*custom haptic waveforms*) dari ketukan mikro lembut (*light click*) hingga denyutan konfirmasi tegas (*success rumble*).

```
┌────────────────────────────────────────────────────────────────────────┐
│             SPEKTRUM AKTUATOR HAPTIK PADA PERANGKAT SELULER            │
├────────────────────┬─────────────────┬────────────────┬────────────────┤
│ Teknologi Aktuator │ Waktu Respon    │ Ketajaman Rasa │ Dukungan Khusus│
├────────────────────┼─────────────────┼────────────────┼────────────────┤
│ Motor ERM Kuno     │ 40 - 80 ms      │ Kasar & Dengung│ Getar Sederhana│
│ LRA Standar Android│ 10 - 20 ms      │ Cukup Tajam    │ Pola Durasi    │
│ Flagship LRA (X-Ax)│ 4 - 8 ms        │ Sangat Tajam   │ Waveform Amplit│
│ Apple Taptic Engine│ 3 - 5 ms        │ Presisi Mikro  │ Core Haptics   │
└────────────────────┴─────────────────┴────────────────┴────────────────┘
```

WidyaMobile menyediakan kendali haptik tingkat tinggi melalui fungsi bawaan `GetarHaptik` dan integrasi antarmuka pada `modul/mobile.wya`. Pengembang dapat memicu haptik dengan mengirimkan larik pola milidetik (misalnya `[100, 50, 100]`) yang merepresentasikan urutan durasi getar dan jeda hening.

Tiga pola sensasi haptik standar industri yang didukung mencakup:
1. **Light Impact / Tick**: Denyut mikro instan (durasi 15–30 ms) saat pengguna menggulir pemilih angka (*picker wheel*) atau menekan tombol papan ketik virtual.
2. **Success Confirmation**: Dua denyutan harmonis berturutan (misalnya 40 ms getar, 60 ms jeda, 80 ms getar) yang memberikan kepuasan psikologis saat transaksi pembayaran berhasil diselesaikan.
3. **Error / Rejection Alert**: Tiga getaran tajam bervolume tinggi berturutan untuk menandakan penolakan otentikasi PIN, saldo tidak mencukupi, atau kegagalan koneksi.

## Diagram Alir Pipeline Pemanggilan Haptik Taktil

Diagram berikut menggambarkan bagaimana perintah haptik dari kode Widya-Lang diterjemahkan menjadi sinyal elektrik aktuator fisik:

```mermaid
flowchart TD
    Event["Aksi Pengguna (Pembayaran Sukses / Geser Slider)"] --> API["Fungsi GetarHaptik([40, 60, 80]) WidyaMobile"]
    
    API --> Platform{"Evaluasi Platform & Hardware"}
    Platform -->|iOS (Taptic Engine)| CoreHaptics["Picu UIImpactFeedbackGenerator / Core Haptics"]
    Platform -->|Android (LRA)| Vibrator["Picu VibratorService & VibrationEffect"]
    
    subgraph "Hardware Actuator Pulse"
        CoreHaptics --> Pulse1["Denyutan Mikro 40ms"]
        Vibrator --> Pulse1
        Pulse1 --> Pause["Jeda Hening 60ms"]
        Pause --> Pulse2["Denyutan Tegas 80ms"]
    end
    
    Pulse2 --> User["Pengguna Merasakan Konfirmasi Transaksi Nyata"]
```

## Implementasi Program Haptik Multi-Sensasi

Berikut adalah implementasi lengkap berbagai sensasi haptik terstandarisasi untuk alur kasir ritel:

```widya
// Program: mesin_haptik_dan_umpan_balik_taktil.wya
// Mengimplementasikan berbagai pola getaran taktil untuk interaksi kasir

gunakan modul::mobile;

fungsi utama() {
    cetak("==============================================================");
    cetak("📳 PENGUJIAN SISTEM UMPAN BALIK HAPTIK WIDYAMOBILE");
    cetak("==============================================================");

    // 1. Definisi Pola Sensasi Haptik Standar
    misal pola_ketuk_ringan = [25];                  // 25ms getar mikro
    misal pola_sukses       = [40, 60, 80];          // Harmonis konfirmasi
    misal pola_peringatan   = [100, 80, 100];        // Peringatan sedang
    misal pola_galat_kritis = [80, 50, 80, 50, 120]; // 3 getaran tajam

    cetak("Daftar Pola Terdaftar:");
    cetak("• Ketukan Mikro (Light Click) : [25 ms]");
    cetak("• Konfirmasi Sukses (Success) : [40, 60, 80 ms]");
    cetak("• Galat Kritis (Error Reject) : [80, 50, 80, 50, 120 ms]");

    // 2. Simulasi Pemicuan Haptik Transaksi Kasir
    cetak("
[Simulasi] Kasir menekan tombol 'Pindai Barcode'...");
    GetarHaptik(pola_ketuk_ringan);

    cetak("[Simulasi] Barcode terdeteksi dan pembayaran QRIS berhasil...");
    GetarHaptik(pola_sukses);

    // 3. Merakit Antarmuka Pengujian Haptik
    kembalikan Aplikasi({
        "judul": "Konsol Uji Haptik",
        "tema": "#8b5cf6",
        "badan": Halaman({
            "navigasi": BilahAtas({
                "judul": "📳 Sensasi Haptik Taktil",
                "warna": "#8b5cf6"
            }),
            "badan": Kolom([
                Kartu({
                    "elevasi": 3,
                    "isi": Kolom([
                        TeksWidget("🎯 Pengujian Aktuator Getar (LRA / Taptic):"),
                        TeksWidget("Tekan tombol di bawah untuk merasakan respons fisik."),
                        Pemisah(),
                        TombolUtama("👆 Denyut Ringan (Light Click)", fungsi() {
                            GetarHaptik(pola_ketuk_ringan);
                            cetak("Haptik ringan terpicu (25ms).");
                        }),
                        Tombol("✅ Konfirmasi Berhasil (Success)", fungsi() {
                            GetarHaptik(pola_sukses);
                            cetak("Haptik sukses terpicu [40, 60, 80].");
                        }),
                        Tombol("❌ Indikator Ditolak (Error Alert)", fungsi() {
                            GetarHaptik(pola_galat_kritis);
                            cetak("Haptik galat terpicu [80, 50, 80, 50, 120].");
                        })
                    ])
                })
            ])
        })
    });
}
```

## Evaluasi Metrik & Benchmark Aktuator Haptik

Tabel di bawah ini merangkum perbandingan performa mekanis antara teknologi aktuator haptik:

| Parameter Performa | Apple Taptic Engine | Flagship X-Axis LRA | Motor ERM Tradisional |
| :--- | :---: | :---: | :---: |
| **Waktu Akselerasi ke Puncak**| **< 4 milidetik** | 6 - 9 milidetik | 55 - 80 milidetik |
| **Waktu Deselerasi (Pemberhentian)**| **< 3 milidetik** | 5 - 8 milidetik | 40 - 70 milidetik |
| **Sensasi Suara Terdengar** | **Sunyi (Nir-Bising)** | Sunyi | Mendengung Kasar |
| **Konsumsi Daya per Getaran** | **0.015 mAh** | 0.020 mAh | 0.085 mAh |
| **Kemampuan Kontrol Frekuensi**| **Penuh (Hz Dinamis)**| Penuh (Resonansi) | Terbatas (Tegangan) |

## Latihan Evaluasi Pemahaman Bab 14

### Bagian A: Soal Esai
1. Jelaskan perbedaan prinsip kerja mekanis antara motor getar *Eccentric Rotating Mass* (ERM) dengan *Linear Resonant Actuator* (LRA) pada ponsel pintar modern!
2. Mengapa aktuator haptik modern seperti Apple Taptic Engine dapat menghasilkan sensasi klik mekanis yang sangat meyakinkan meskipun layar kaca ponsel tidak bergerak?
3. Uraikan panduan desain pengalaman pengguna (*UX Guidelines*) dalam membatasi frekuensi pemanggilan getaran haptik agar tidak menimbulkan kejenuhan atau kelelahan sensorik pada tangan pengguna!

### Bagian B: Soal Pilihan Ganda
1. Jenis aktuator getar resonan linier canggih yang umum digunakan pada perangkat seluler Android kelas atas adalah...
   A. Linear Resonant Actuator (LRA)  
   B. Motor Kipas DC  
   C. Relay Elektromagnetik  
   D. Solenoid Pompa  
2. Nama dagang dari subsistem aktuator haptik presisi tinggi pada perangkat Apple iPhone adalah...
   A. Taptic Engine  
   B. DualShock  
   C. Force Touchpad  
   D. Rumble Pak  
3. Fungsi resmi pada modul `modul::mobile` yang digunakan untuk mengirimkan instruksi pola getaran haptik adalah...
   A. `GetarHaptik()`  
   B. `goyang_layar_hp()`  
   C. `bunyikan_alarm()`  
   D. `putar_motor_vga()`  
4. Struktur parameter yang dikirimkan ke fungsi `GetarHaptik` untuk merepresentasikan urutan durasi getar dan jeda dalam milidetik adalah...
   A. Larik bilangan bulat (Array angka)  
   B. Teks string Base64  
   C. Boolean benar/salah  
   D. File gambar PNG  
5. Waktu respons tipikal yang dibutuhkan oleh Taptic Engine modern untuk mencapai puncak gaya getaran adalah...
   A. Kurang dari 5 milidetik  
   B. 30 detik  
   C. 500 milidetik  
   D. 1 menit  
6. Pola haptik yang ideal untuk mengonfirmasi bahwa pembayaran transaksi kasir telah berhasil diselesaikan adalah...
   A. Dua atau tiga denyutan harmonis berturutan yang tegas dan menyenangkan  
   B. Getaran konstan tanpa henti selama 10 menit  
   C. Tidak ada getaran sama sekali  
   D. Getaran acak tidak terkendali  
7. Kelemahan mendasar dari motor getar kuno tipe ERM (Eccentric Rotating Mass) adalah...
   A. Waktu deselerasi lambat sehingga getaran terasa berdengung kasar dan lamban berhenti  
   B. Tidak membutuhkan listrik  
   C. Ukurannya terlalu kecil  
   D. Terbuat dari kaca  
8. Izin sistem operasi Android yang wajib didaftarkan pada manifest untuk mengaktifkan aktuator getar adalah...
   A. `android.permission.VIBRATE`  
   B. `android.permission.INTERNET`  
   C. `android.permission.CAMERA`  
   D. `android.permission.BLUETOOTH`  
9. Dampak negatif dari pemanggilan haptik yang terlalu berlebihan (misalnya pada setiap perubahan piksel saat menggulir cepat) adalah...
   A. Menimbulkan kelelahan sensorik (*haptic fatigue*) pada jari pengguna dan menguras daya baterai  
   B. Layar ponsel akan retak  
   C. Sinyal seluler akan terputus  
   D. Memori internal terhapus  
10. Berapakah durasi getar tipikal untuk sensasi ketukan mikro (*light tick*) pada tombol navigasi virtual?
    A. Sekitar 15 hingga 30 milidetik  
    B. 2.000 milidetik  
    C. 10 detik  
    D. 500 milidetik  
