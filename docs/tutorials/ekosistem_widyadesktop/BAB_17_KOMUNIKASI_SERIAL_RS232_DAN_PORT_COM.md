## Bab 17: Komunikasi Serial RS-232, Port COM & Penemuan Port Otomatis

### 17.1 Komunikasi Perangkat Keras Serial di Lingkungan Ritel & Industri

Meskipun protokol antarmuka modern seperti USB dan Thunderbolt mendominasi perangkat komersial, di dunia **stasiun kerja kasir ritel (POS)**, pergudangan, dan otomasi industri, protokol komunikasi serial asinkron **RS-232 (Port COM)** tetap menjadi standar tak tergantikan. Perangkat keras industri seperti timbangan digital terverifikasi metrologi, modem telemetri maritim, layar tampilan pelanggan fluoresen (VFD Customer Display), dan pengendali mesin kasir terhubung melalui port serial fisik (`COM1`–`COM8` pada Windows atau `/dev/ttyUSB0` / `/dev/ttyS0` pada Linux).

Aplikasi berbasis web murni di peramban mengalami kendala fundamental: Web Serial API memiliki keterbatasan izin yang kaku, tidak mendukung konfigurasi paritas tingkat rendah secara konsisten, serta tidak dapat beroperasi di latar belakang saat jendela diminimalkan.

WidyaDesktop menyediakan driver serial murni pada modul `pure_hardware.rs` dan fungsi `daftar_serial_ports()`:
- **Penemuan Port Otomatis (*Auto-Enumeration of COM Ports*)**: Memindai registri perangkat keras sistem operasi untuk mendeteksi seluruh port serial yang tersedia beserta nomor pengenal VID/PID dan nama manufaktur perangkat.
- **Konfigurasi Parameter Fisik Presisi**: Mengonfigurasi laju transmisi (*Baud Rate: 9600, 19200, 38400, 115200 bps*), panjang bit data (*Data Bits: 7 atau 8 bit*), bit henti (*Stop Bits: 1 atau 2 bit*), serta pemeriksaan paritas (*Parity: None, Even, Odd*).
- **Aliran I/O Asinkron Bebas Pemblokiran (*Non-Blocking Ring Buffer*)**: Mengalirkan data byte serial melalui penyangga cincin (*circular buffer*) di utas pekerja terpisah, menjamin bahwa keterlambatan sinyal perangkat keras tidak akan membekukan antarmuka visual kasir.

```mermaid
flowchart LR
    Hardware["Perangkat Keras Serial (Timbangan / Scanner / Modbus)"] -->|Kabel Fisik RS-232 / USB-to-UART (COM3)| OSKernel["Driver Kernel Serial OS (serialcomm / ttyUSB)"]
    OSKernel --> SerialModule["Mesin Serial Murni WidyaDesktop (pure_hardware.rs)"]
    
    subgraph PengolahanSerial["Pemrosesan Aliran Data Serial"]
        SerialModule --> BaudConfig["Konfigurasi Baud Rate (9600/115200) & Parity"]
        SerialModule --> RingBuffer["Penyangga Lingkar Byte (Ring Buffer)"]
        RingBuffer --> Parser["Parser Aliran Teks & Validasi Frame"]
    end
    
    Parser --> IPCBridge["Jembatan IPC (pure_ipc.rs)"]
    IPCBridge --> POSApp["Aplikasi Kasir Widya-Lang (main.wya)"]
```

### 17.2 Penemuan Port Otomatis (*Auto-Discovery*) & Pemetaan Perangkat

Pada lingkungan produksi di mana kabel USB-Serial sering dipindah-pindah oleh staf toko, menetapkan nama port secara statis (seperti hardcoding `COM3`) akan menimbulkan malapetaka saat komputer dihidupkan ulang dan sistem operasi menetapkan port baru (`COM5`).

WidyaDesktop menerapkan pola **Smart Port Auto-Discovery**:
1. Sistem mengirimkan bingkai jabat tangan uji coba (*probing packet*) ke seluruh port serial yang terdeteksi.
2. Respons balik diidentifikasi berdasarkan pola tanda tangan perangkat (misalnya: timbangan digital mengembalikan prefiks `ST,GS` atau printer mengembalikan kode identifikasi model).
3. Setelah port teridentifikasi secara deterministik, sistem secara otomatis mengikat koneksi tanpa intervensi manual dari operator kasir.

### 17.3 Implementasi Kode Widya-Lang & WidyaDesktop Utuh

Berikut implementasi pemindaian port serial dan konfigurasi komunikasi data timbangan pada Widya-Lang:

```widya
// Modul Pengendalian Komunikasi Serial RS-232
modul pengelola_serial_pos {
    impor "./modul/desktop.wya";

    fungsi inisialisasi_komunikasi_serial() {
        // 1. Buat Aplikasi Kasir
        misal app = BuatAplikasiDesktop("Nusantara POS - Terminal Serial", 1100, 720);

        // 2. Registrasi RPC: Pindai Port Serial yang Tersedia
        app.daftarkan_rpc("pindai_port_serial", fungsi(_payload) {
            cetak(">>> [SERIAL] Memindai seluruh port COM / tty serial yang aktif...");

            // Simulasi hasil deteksi dari daftar_serial_ports()
            misal daftar_port = [
                { "port": "COM1", "tipe": "Standard Hardware Serial Port", "tersedia": benar },
                { "port": "COM3", "tipe": "USB-to-Serial CH340 (Timbangan Kasir)", "tersedia": benar }
            ];

            kembalikan {
                "sukses": benar,
                "total_port": daftar_port.panjang(),
                "daftar": daftar_port
            };
        });

        // 3. Registrasi RPC: Sambungkan ke Port Timbangan Kasir
        app.daftarkan_rpc("buka_koneksi_timbangan", fungsi(payload) {
            misal port_target = payload.port; // misal: "COM3"
            misal baud = payload.baud_rate;   // misal: 9600

            cetak(">>> [SERIAL] Membuka saluran serial pada " + port_target + " (Baud: " + baud + ")");
            // Inisialisasi saluran I/O serial non-blocking
            kembalikan {
                "sukses": benar,
                "status_koneksi": "TERHUBUNG_DAN_AKTIF",
                "port_aktif": port_target,
                "baud": baud
            };
        });

        kembalikan app;
    }
}
```

### 17.4 Analisis Kinerja, Keamanan & Tabel Metrik Arsitektur

Tabel spesifikasi kecepatan dan keandalan transmisi port serial:

| Parameter Komunikasi | Baud Rate 9600 bps | Baud Rate 19200 bps | Baud Rate 115200 bps |
| :--- | :--- | :--- | :--- |
| **Kapasitas Transfer Teoretis** | ~960 Karakter / detik | ~1.920 Karakter / detik | **~11.520 Karakter / detik** |
| **Penggunaan Tipikal** | Timbangan Digital, VFD Display | Pemindai Barcode Industri | **Otomasi PLC SCADA Modbus RTU** |
| **Toleransi Gangguan Kabel (Noise)**| Sangat Tinggi (Jarak Jauh) | Tinggi | **Sedang (Perlu Kabel Terlindung Shielded)** |
| **Latensi Respons Frame** | 10 – 25 milidetik | 5 – 10 milidetik | **< 1.0 milidetik** |

### 17.5 Latihan Praktik & Studi Kasus

1. Mengapa penetapan konfigurasi port serial secara statis (*hardcoded COM port*) sangat berisiko dalam operasional kasir toko ritel skala nasional?
2. Jelaskan perbedaan mendasar fungsi antara bit data (*data bits*), bit henti (*stop bits*), dan bit paritas (*parity bit*) pada bingkai data serial RS-232!
3. Uraikan bagaimana arsitektur *Ring Buffer* pada WidyaDesktop mencegah terjadinya kehilangan karakter (*buffer overrun / data drop*) ketika perangkat keras mengirim data lebih cepat daripada kecepatan pemrosesan antarmuka!

### 17.6 Evaluasi Mandiri Pilihan Ganda

1. Protokol komunikasi asinkron berbasis perangkat keras tertua dan paling stabil yang digunakan pada timbangan kasir dan mesin kasir adalah...
   A. RS-232 (Port COM)
   B. HDMI 2.1
   C. DisplayPort
   D. Bluetooth 1.0

2. Satuan kecepatan transmisi simbol biner per detik pada komunikasi serial dinamakan...
   A. Baud Rate (bps)
   B. Hertz (Hz)
   C. Watt (W)
   D. Decibel (dB)

3. Laju transmisi (*Baud Rate*) standar yang paling umum digunakan pada timbangan digital ritel adalah...
   A. 9600 bps
   B. 10.000.000 bps
   C. 1 bps
   D. 0 bps

4. Fungsi pada modul `pure_hardware.rs` yang digunakan untuk mendeteksi seluruh port serial fisik maupun virtual yang terpasang pada komputer adalah...
   A. `daftar_serial_ports()`
   B. `format_komputer()`
   C. `hapus_driver_printer()`
   D. `matikan_layar()`

5. Struktur penyangga memori melingkar yang digunakan untuk menampung aliran byte data serial secara asinkron tanpa henti dinamakan...
   A. Ring Buffer (Circular Buffer)
   B. Stack Overflow
   C. Deadlock Memory
   D. Garbage Collector

6. Nama konverter chip USB-to-UART yang sangat populer ditemukan pada periferal kasir murah meriah adalah...
   A. CH340 / FTDI FT232R / CP2102
   B. Intel Core i9
   C. Nvidia RTX 4090
   D. AMD Ryzen 9

7. Konfigurasi bingkai serial standar `9600-8-N-1` memiliki arti...
   A. Baud rate 9600, 8 bit data, Paritas None, 1 bit henti
   B. 9600 monitor, 8 keyboard, 1 mouse
   C. Jam 9 malam, 600 detik, 8 menit
   D. Tahun 9600 Masehi

8. Mengapa penanganan I/O serial pada WidyaDesktop dijalankan pada utas pekerja latar belakang (*worker thread*) mandiri?
   A. Mencegah operasi pembacaan serial yang terhambat (*blocking I/O*) membekukan pergerakan antarmuka visual kasir
   B. Mengubah tipe monitor kasir menjadi layar sentuh
   C. Mempercepat pengiriman email ke kantor pusat
   D. Menghilangkan kebutuhan nota belanja

9. Fitur cerdas yang secara otomatis mendeteksi dan mengikat port serial yang benar meskipun kabel USB berpindah-pindah port disebut...
   A. Smart Port Auto-Discovery
   B. Manual Cable Splicing
   C. Random Port Guessing
   D. Blind Port Scan

10. Nilai bit paritas (*Parity Bit*) pada transmisi serial digunakan untuk...
    A. Mendeteksi kesalahan bit tunggal (*single-bit error*) yang disebabkan oleh gangguan induksi elektromagnetik kabel
    B. Menambah kecepatan kipas pendingin komputer
    C. Mengubah teks biasa menjadi huruf kapital
    D. Memformat ulang diska penyimpanan
