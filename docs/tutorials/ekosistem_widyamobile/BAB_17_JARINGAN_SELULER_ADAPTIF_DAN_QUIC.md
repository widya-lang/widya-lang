# Bab 17: Jaringan Seluler Adaptif: HTTP/3 QUIC, Resilient WebSocket & Throttling

Lingkungan jaringan seluler (*cellular mobile network*) dicirikan oleh ketidakpastian yang tinggi. Ponsel pintar berpindah secara dinamis di antara pemancar BTS 5G berkecepatan gigabit, jaringan Wi-Fi publik yang terputus-putus, koneksi 3G/EDGE di pedesaan terpencil, hingga kondisi *dead zone* total tanpa sinyal di terowongan atau basement. Aplikasi seluler yang rapuh (*fragile apps*) akan langsung mengalami kegagalan transmisi (*timeout exceptions*), menyajikan layar putih kosong (*blank white screens*), atau mengulang pengiriman formulir transaksi yang berujung pada tagihan ganda (*duplicate charges*).

WidyaMobile menghadirkan subsistem **Adaptive Cellular Networking Engine** yang memantau status antarmuka jaringan secara deterministik melalui fungsi bawaan `PeriksaKoneksiJaringan` pada `modul/mobile.wya`. Subsistem ini memadukan protokol transport generasi terbaru **HTTP/3 berbasis UDP/QUIC** untuk mengeliminasi fenomena *Head-of-Line Blocking*, kanal komunikasi dua arah persisten **Resilient WebSocket**, serta arsitektur sakelar pembatas beban (*Circuit Breaker Pattern*).

```
┌────────────────────────────────────────────────────────────────────────┐
│             SPEKTRUM ADAPTASI KONEKTIVITAS JARINGAN SELULER            │
├────────────────────┬─────────────────┬────────────────┬────────────────┤
│ Status Jaringan    │ Tipe Transport  │ Kebijakan Data │ Strategi Cache │
├────────────────────┼─────────────────┼────────────────┼────────────────┤
│ Wi-Fi High Speed   │ HTTP/3 + WS Penuh│ Prefetch Aktif │ Sinkron Cepat  │
│ Seluler 5G / 4G LTE│ HTTP/3 Standar  │ Kompresi Zstd  │ Cache Read-Only│
│ Seluler 3G / EDGE  │ HTTP/2 Minimal  │ Teks Saja      │ Cache Dominan  │
│ Terputus (Offline) │ Tidak Ada       │ Antrean Outbox │ 100% Cache WAL │
└────────────────────┴─────────────────┴────────────────┴────────────────┘
```

Prinsip dasar jaringan adaptif WidyaMobile mencakup:
1. **Zero Head-of-Line Blocking via QUIC**: Berbeda dengan TCP di mana kehilangan satu paket menghentikan seluruh aliran data, HTTP/3 QUIC mengisolasi setiap aliran *request/response* secara independen sehingga aplikasi tetap responsif meskipun paket data terlempar (*packet loss*).
2. **Resilient WebSocket with Exponential Backoff**: Saluran WebSocket yang secara otomatis melakukan rekoneksi dengan pengganda waktu tunggu bertahap (1s, 2s, 4s, 8s, maks 30s) ketika ponsel berpindah menara pemancar atau beralih dari Wi-Fi ke kuota seluler.
3. **Metered Connection Consciousness**: Mendeteksi atribut `isMetered` dari sistem operasi untuk secara otomatis menonaktifkan pengunduhan aset berat (video promosi atau gambar resolusi tinggi) demi menghemat kuota internet berbayar milik pengguna.
4. **Idempotency Token Guarantee**: Menyertakan token UUID v4 kriptografis unik pada setiap *header* permintaan transaksi finansial guna mencegah pemrosesan transaksi berulang di server saat terjadi koneksi terputus sesaat (*network blip*).

## Diagram Alir Siklus Jaringan & Circuit Breaker

Diagram berikut menggambarkan bagaimana WidyaMobile mengevaluasi status jaringan dan mencegah kebanjiran beban saat koneksi tidak stabil:

```mermaid
flowchart TD
    Req["Permintaan Transaksi Baru dari Aplikasi Kasir"] --> NetCheck["Fungsi PeriksaKoneksiJaringan()"]
    
    NetCheck --> Status{"Apakah Jaringan Aktif & Tersambung?"}
    Status -->|Tidak (Offline)| Outbox["Simpan ke Antrean Transaksi Offline (Outbox)"]
    Status -->|Ya| CB{"Status Circuit Breaker?"}
    
    CB -->|Open (Sirkuit Terputus)| Fallback["Tolak Langsung / Ambil Cache Lokal"]
    CB -->|Half-Open| Probe["Kirim Uji Coba Paket Ringan (Probe Request)"]
    CB -->|Closed (Normal)| Send["Kirim via HTTP/3 QUIC + Idempotency Key"]
    
    Send --> Resp{"Apakah Respon Berhasil 200 OK?"}
    Resp -->|Ya| Success["Perbarui State UI & Reset Counter Galat"]
    Resp -->|Timeout / 5xx| Fail["Catat Kegagalan & Pemicu Trip Circuit Breaker"]
    
    Fail --> Retry["Jadwalkan Retry dengan Exponential Backoff"]
```

## Implementasi Program Jaringan Seluler Adaptif

Berikut adalah implementasi sistem jaringan adaptif yang memantau konektivitas dan mengendalikan permintaan data berdasarkan jenis koneksi:

```widya
// Program: jaringan_seluler_adaptif_quic.wya
// Memantau jenis koneksi internet dan menerapkan kebijakan penghematan kuota

gunakan modul::mobile;

fungsi utama() {
    cetak("==============================================================");
    cetak("🌐 SISTEM JARINGAN SELULER ADAPTIF WIDYAMOBILE");
    cetak("==============================================================");

    // 1. Memeriksa Status Antarmuka Jaringan OS
    misal status_net = PeriksaKoneksiJaringan();

    cetak("Status Konektivitas : " + (status_net.aktif ? "Tersambung (Online) 🟢" : "Terputus (Offline) 🔴"));
    cetak("Tipe Antarmuka      : " + status_net.tipe);

    // 2. Menentukan Kebijakan Transmisi Data Berdasarkan Tipe Antarmuka
    misal protokol_transport = "HTTP/3 over QUIC (Port UDP 443)";
    misal kebijakan_kuota = "Kualitas Penuh (Prefetching & Aset Gambar Aktif)";
    misal batas_timeout_ms = 5000;

    jika (!status_net.aktif) {
        protokol_transport = "Lokal Offline Mode (Outbox Queue)";
        kebijakan_kuota = "Hanya Akses Basis Data Terenkripsi Lokal";
        cetak("
⚠️ PERINGATAN: Perangkat sedang berada di luar jangkauan sinyal internet.");
    } atau_jika (status_net.tipe == "Seluler (Metered Data)") {
        kebijakan_kuota = "Hemat Kuota (Gambar Dibatasi, Kompresi Zstandard)";
        batas_timeout_ms = 12000;
        cetak("
💡 KONEKSI METERED: Mengaktifkan mode penghematan kuota data pengguna.");
    }

    cetak("Transport Protokol : " + protokol_transport);
    cetak("Aturan Kuota Data  : " + kebijakan_kuota);
    cetak("Ambang Batas Waktu : " + teks(batas_timeout_ms) + " milidetik");

    // 3. Merakit Antarmuka Pengguna Status Jaringan
    kembalikan Aplikasi({
        "judul": "Monitor Jaringan Cerdas",
        "tema": "#0284c7",
        "badan": Halaman({
            "navigasi": BilahAtas({
                "judul": "🌐 Status Konektivitas Jaringan",
                "warna": "#0284c7"
            }),
            "badan": Kolom([
                Kartu({
                    "elevasi": 3,
                    "isi": Kolom([
                        TeksWidget("📊 Status Sambungan Seluler:"),
                        TeksWidget("• Status: " + (status_net.aktif ? "Online ✅" : "Offline ❌")),
                        TeksWidget("• Antarmuka: " + status_net.tipe),
                        TeksWidget("• Protokol: " + protokol_transport),
                        Pemisah()
                    ])
                }),
                Kartu({
                    "elevasi": 2,
                    "isi": Kolom([
                        TeksWidget("🛡️ Ketahanan Terhadap Gangguan Jaringan:"),
                        TeksWidget("Aplikasi secara otomatis beralih ke antrean offline"),
                        TeksWidget("bila sinyal terputus di tengah proses transaksi."),
                        Pemisah(),
                        TombolUtama("🔄 Uji Ping Jaringan Cepat", fungsi() {
                            misal net_baru = PeriksaKoneksiJaringan();
                            cetak("Pemeriksaan Ulang: Tipe Jaringan = " + net_baru.tipe);
                        })
                    ])
                })
            ])
        })
    });
}
```

## Evaluasi Metrik & Benchmark Protokol Jaringan

Tabel berikut membandingkan waktu pembukaan koneksi dan latensi pemulihan paket antara HTTP/3 QUIC dengan HTTP/1.1 TCP pada jaringan seluler 4G/5G:

| Skenario Pengujian | HTTP/3 QUIC (WidyaMobile) | HTTP/2 over TLS | HTTP/1.1 over TCP |
| :--- | :---: | :---: | :---: |
| **Waktu Jabat Tangan (Handshake)**| **0-RTT (Instan)** | 1-RTT / 2-RTT | 2-RTT / 3-RTT |
| **Packet Loss Recovery (2% Loss)**| **Nir-Blokir (Stream Terisolasi)**| Head-of-Line Stall| Head-of-Line Stall |
| **Migrasi Jaringan (Wi-Fi ke 5G)**| **Koneksi Tetap Bertahan (CID)**| Putus & Rekoneksi | Putus & Rekoneksi |
| **Overhead Header Transmisi** | **Kompresi QPACK (< 30B)** | Kompresi HPACK | Header Teks Bebas |
| **Konsumsi Baterai Jaringan** | **Sangat Efisien** | Menengah | Boros |

## Latihan Evaluasi Pemahaman Bab 17

### Bagian A: Soal Esai
1. Jelaskan mengapa protokol HTTP/3 berbasis UDP/QUIC mampu mengeliminasi fenomena *Head-of-Line Blocking* yang selama ini menjadi kelemahan mendasar protokol TCP pada jaringan seluler!
2. Bagaimana mekanisme *Connection ID* (CID) pada protokol QUIC memungkinkan sesi transaksi seluler tetap berlangsung mulus tanpa putus ketika ponsel pengguna berpindah dari Wi-Fi rumah ke jaringan 5G luar ruangan?
3. Uraikan pola kerja *Circuit Breaker* (tiga status: Closed, Open, Half-Open) dalam melindungi aplikasi seluler dari lonjakan antrean permintaan saat server backend mengalami degradasi layanan!

### Bagian B: Soal Pilihan Ganda
1. Protokol transport generasi terbaru yang menjadi fondasi standar HTTP/3 adalah...
   A. QUIC (berjalan di atas UDP)  
   B. TCP Tradisional  
   C. NetBIOS  
   D. ICMP Ping  
2. Kelemahan mendasar dari protokol TCP pada jaringan seluler yang rawan kehilangan paket (*packet loss*) adalah...
   A. Head-of-Line Blocking (seluruh aliran data terhenti menunggu paket yang hilang)  
   B. Tidak mendukung enkripsi  
   C. Kecepatan maksimal hanya 10 bps  
   D. Mengharuskan kabel tembaga  
3. Fungsi pembantu resmi pada modul `modul::mobile` yang digunakan untuk membaca status ketersediaan koneksi internet dan tipe antarmukanya adalah...
   A. `PeriksaKoneksiJaringan()`  
   B. `ping_server_pusat()`  
   C. `ambil_ip_publik()`  
   D. `hitung_kuota_habis()`  
4. Fitur pada protokol QUIC yang memungkinkan klien mengirimkan data pada paket pertama tanpa menunggu jabat tangan selesai saat terhubung kembali dinamakan...
   A. 0-RTT Connection Resumption  
   B. 3-Way Handshake  
   C. Syn-Ack Flood  
   D. Slow Start  
5. Istilah untuk koneksi internet seluler di mana pengguna dikenai biaya berbasis kuota data terukur (*metered connection*) adalah...
   A. Metered Network  
   B. Unmetered Fiber  
   C. Local Loopback  
   D. Dark Fiber  
6. Pola arsitektur ketahanan yang secara otomatis memutus sementara aliran permintaan ke server saat terdeteksi rangkaian kegagalan beruntun disebut...
   A. Circuit Breaker Pattern  
   B. Infinite Loop Pattern  
   C. Singleton Pattern  
   D. Factory Pattern  
7. Kunci kriptografis unik yang disertakan pada header permintaan transaksi untuk memastikan sebuah aksi tidak diproses ganda di backend dinamakan...
   A. Idempotency Key (Token Idempoten)  
   B. Primary Key  
   C. Foreign Key  
   D. Private Key  
8. Algoritma penundaan waktu rekoneksi di mana durasi jeda berlipat ganda setiap kali upaya sambungan gagal disebut...
   A. Exponential Backoff  
   B. Linear Stride  
   C. Random Shuffle  
   D. Quick Sort  
9. Format kompresi header berstandar modern yang digunakan secara native pada HTTP/3 adalah...
   A. QPACK  
   B. GZIP  
   C. Deflate  
   D. TAR  
10. Keuntungan utama dari Resilient WebSocket pada aplikasi kasir POS seluler adalah...
    A. Notifikasi transaksi berhasil dari pembeli dapat diterima oleh kasir secara seketika (*real-time*) dalam hitungan milidetik  
    B. Kasir tidak perlu menghitung kembalian  
    C. Printer otomatis memotong kertas  
    D. Pajak otomatis terhapus  
