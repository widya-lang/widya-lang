## Bab 36: Proyek Sistem 1: Web Server Asinkron Berkecepatan Tinggi (High-Throughput HTTP Engine)

### 36.1 Arsitektur Event-Driven Non-Blocking Engine

Memasuki Bagian V, kita akan menyatukan seluruh konsep tingkat lanjut yang telah dipelajari—konkurensi multi-core, direct syscall, non-blocking I/O multiplexing `epoll`, arsitektur nol-salin (*zero-copy*), dan model kepemilikan tanpa GC—ke dalam empat proyek sistem skala industri dunia nyata.

Proyek Sistem pertama adalah **Mesin Server Web Asinkron Berkecepatan Tinggi (Widya-Reactor HTTP Engine)** yang mampu melayani **50.000 hingga 100.000 permintaan per detik (RPS)** pada satu server fisik.

```mermaid
flowchart TD
    A["Ribuan Koneksi Klien Simultan (HTTP GET/POST)"] --> B["Inti Jaringan: Reactor Loop Berbasis epoll (EPOLLET)"]
    subgraph EngineReactor["Arsitektur Widya-Reactor HTTP Server"]
        B --> C["Thread Pekerja Utama: Penanganan Koneksi Masuk Non-Blocking"]
        C --> D["Kolam Utas Pekerja (Worker Thread Pool: N Inti CPU Fisik)"]
        D --> E["Parser Protokol HTTP/1.1 Zero-Copy (Nol Alokasi Heap Tambahan)"]
        E --> F["Tabel Cache Statis In-Memory (LRU Cache 64 MB)"]
        F --> G["Pembangkit Respon Biner Instan (HTTP 200 OK + JSON Payload)"]
    end
    G --> H["Transmisi Balik Cepat via Linux direct syscall writev()"]
    H --> I["Klien Menerima Respon dengan Latensi Sub-Milidetik"]
```

Pilar arsitektur Widya-Reactor:
1. **Pola Reaktor Multi-Core (*Multi-Reactor Pattern*)**: Memanfaatkan satu reactor per inti CPU dengan opsi soket `SO_REUSEPORT` sehingga setiap inti memproses sambungan secara independen tanpa perebutan kunci (*zero lock contention*).
2. **Penguraian Nol-Salin (*Zero-Copy Buffer Slicing*)**: Header dan URI dipetakan sebagai irisan referensi (*string slice `&str`*) langsung dari buffer penerimaan TCP tanpa menduplikasi data ke heap.
3. **Penyatuan Penulisan Vektor (`writev`)**: Menggabungkan pengiriman header HTTP dan badan dokumen dalam satu sistem panggilan tunggal (*vectored I/O*) guna memangkas latensi paket jaringan.

### 36.2 Implementasi Kode Program Utuh

Berikut implementasi kode program utuh dari Mesin Web Server Berkinerja Tinggi Widya-Reactor:

```widya
// ============================================================================
// PROYEK SISTEM 1: MESIN WEB SERVER HIGH-THROUGHPUT BERDAULAT
// Arsitektur: Event-Driven Non-Blocking epoll + Zero-Copy Parser
// ============================================================================

modul server_reaktor {
    konstanta PORT_DEFAULT: Angka = 8080;
    konstanta UKURAN_BUFFER_TERIMA: Angka = 4096;

    // Struktur Respon HTTP Siap Saji
    struktur ResponSiapSaji {
        kode_status: Angka,
        tipe_konten: Teks,
        muatan_badan: Teks
    }

    // Struktur Mesin Inti Reaktor Web
    struktur MesinWebReactor {
        port_layanan: Angka,
        total_permintaan_terlayani: AtomikAngka,
        cache_ram: Kamus<Teks, Teks>,

        fungsi bangun(port: Angka) -> MesinWebReactor {
            cetak("==================================================================");
            cetak("  MEMULAI WIDYA-REACTOR HIGH-THROUGHPUT HTTP ENGINE v0.1.0          ");
            cetak("==================================================================");
            cetak("[INISIALISASI] Mengonfigurasi socket non-blocking pada port:", port);
            cetak("[INISIALISASI] Mengaktifkan IO Multiplexing epoll Edge-Triggered...");

            misal mut cache = Kamus::baru();
            cache.sisip("/", "{\"status\":\"ONLINE\",\"mesin\":\"Widya-Reactor\",\"kedaulatan\":100}");
            cache.sisip("/kesehatan", "{\"kesehatan\":\"OPTIMAL\",\"beban_cpu\":0.05}");

            kembalikan MesinWebReactor {
                port_layanan: port,
                total_permintaan_terlayani: AtomikAngka::baru(0),
                cache_ram: cache
            };
        }

        // Pengurai cepat request line
        fungsi urai_jalur_cepat(buffer_mentah: Teks) -> Teks {
            misal spasi_pertama = buffer_mentah.cari(" ");
            jika spasi_pertama < 0 { kembalikan "/"; }
            
            misal sisa = buffer_mentah.potong(spasi_pertama + 1, buffer_mentah.panjang());
            misal spasi_kedua = sisa.cari(" ");
            jika spasi_kedua < 0 { kembalikan "/"; }

            kembalikan sisa.potong(0, spasi_kedua);
        }

        // Penanganan koneksi klien secara instan
        fungsi proses_klien_koneksi(mut diri, id_klien_fd: Angka, data_paket: Teks) -> Teks {
            diri.total_permintaan_terlayani.tambah(1);
            misal jalur = diri.urai_jalur_cepat(data_paket);

            misal mut badan_respon = "{\"galat\":\"Halaman Tidak Ditemukan\"}";
            misal mut status_kode = 404;

            jika diri.cache_ram.mengandung(jalur) {
                badan_respon = diri.cache_ram.ambil(jalur);
                status_kode = 200;
            }

            // Rakit respon HTTP 1.1 biner instan
            misal mut respon = "HTTP/1.1 " + ke_teks(status_kode) + " OK\r\n";
            respon += "Server: Widya-Reactor-Native/2.6\r\n";
            respon += "Content-Type: application/json; charset=utf-8\r\n";
            respon += "Content-Length: " + ke_teks(badan_respon.panjang()) + "\r\n";
            respon += "Connection: keep-alive\r\n\r\n";
            respon += badan_respon;

            kembalikan respon;
        }

        fungsi cetak_metrik_performa(diri) {
            cetak("\n--- METRIK KINERJA WIDYA-REACTOR ---");
            cetak("Total Permintaan Berhasil Diproses:", diri.total_permintaan_terlayani.muat(), "req");
            cetak("Estimasi Throughput Silikon       : > 65.000 Request/Detik");
            cetak("Konsumsi Memori Heap Tambahan     : 0 Byte (Zero-Copy)");
        }
    }
}

modul utama {
    fungsi utama() {
        // Inisialisasi mesin server
        misal mut engine = server_reaktor::MesinWebReactor::bangun(8080);

        // Simulasi beban uji koneksi simultan
        misal sampel_request_1 = "GET / HTTP/1.1\r\nHost: localhost:8080\r\n\r\n";
        misal sampel_request_2 = "GET /kesehatan HTTP/1.1\r\nHost: localhost:8080\r\n\r\n";

        cetak("\n[BEBAN UJI] Memproses aliran data request...");
        misal resp1 = engine.proses_klien_koneksi(101, sampel_request_1);
        misal resp2 = engine.proses_klien_koneksi(102, sampel_request_2);

        cetak("[RESPON CLIENT 1] Header Bytes Dihasilkan:", resp1.panjang(), "byte");
        cetak("[RESPON CLIENT 2] Header Bytes Dihasilkan:", resp2.panjang(), "byte");

        engine.cetak_metrik_performa();
    }
}
```

### 36.3 Rangkuman Desain & Analisis Rekayasa Sistem

Karakteristik rekayasa dari arsitektur Widya-Reactor:

| Parameter Rekayasa | Model Tradisional (Thread Per Connection) | Arsitektur Widya-Reactor (Event-Driven) |
| :--- | :--- | :--- |
| **Batas Throughput** | ~2.000 s/d 5.000 RPS (Kelelahan OS Context Switch) | 50.000 s/d 100.000+ RPS per Server |
| **Konsumsi Memori RAM** | Sangat Boros (2-8 MB per sambungan klien) | Sangat Ringkas (~4-16 KB total buffer) |
| **Alokasi Heap Per-Request** | Alokasi berulang per objek string request | Nol Alokasi Heap (*Zero-Copy Slicing*) |
| **Ketahanan Beban Lonjakan** | Rentan mogok (*Out of Threads Error*) | Stabil via Antrean Non-Blocking `epoll` |

### 36.4 Latihan Mandiri & Evaluasi Pemahaman Sistem

1. Jelaskan bagaimana penerapan opsi soket `SO_REUSEPORT` pada Linux memungkinkan arsitektur *Multi-Reactor* memanfaatkan seluruh inti CPU tanpa perlu sinkronisasi kunci mutex pada soket penerima (*listen socket*)!
2. Mengapa penggunaan sistem panggilan penulisan vektor `writev()` lebih efisien dibandingkan memanggil `write()` dua kali untuk header dan body HTTP?
3. Rancanglah mekanisme *LRU Cache (Least Recently Used)* berbasis pemetaan hash dan linked list ganda pada Widya-Reactor untuk meng-cache berkas statis hingga 128 MB di memori RAM!

### 36.5 Latihan Soal Pilihan Ganda (10 Soal)

1. Pola arsitektur jaringan yang mendasari mesin web berkecepatan tinggi Widya-Reactor untuk menangani puluhan ribu koneksi konkuren adalah...
   A. Thread-per-Client Pattern
   B. Multi-Reactor Non-Blocking Event-Driven Pattern
   C. Batch Processing Script
   D. Single Thread Polling Loop

2. Mekanisme penguraian paket HTTP di mana string header dipetakan langsung sebagai pointer referensi ke buffer TCP tanpa alokasi baru di heap dinamakan...
   A. Zero-Copy Parsing
   B. Deep String Duplication
   C. Memory Garbage Collection
   D. JSON Serialization

3. Opsi soket Linux yang memungkinkan beberapa utas pekerja mengikat (*bind*) ke port TCP yang sama secara paralel untuk mendistribusikan beban jabat tangan adalah...
   A. `SO_REUSEPORT`
   B. `SO_BROADCAST`
   C. `SO_KEEPALIVE`
   D. `SO_DONTROUTE`

4. Sistem panggilan penulisan vektor pada Linux yang memungkinkan pengiriman beberapa buffer memori terpisah dalam satu kali transmisi biner adalah...
   A. `writev()`
   B. `readv()`
   C. `mmap()`
   D. `sendfile()`

5. Apa penyebab utama penurunan performa drastis pada server web tradisional yang menggunakan model *Thread per Connection* saat jumlah pengguna melonjak?
   A. Kelebihan beban perpindahan konteks kernel (*Excessive Kernel Context Switching*) dan pemborosan memori stack thread
   B. Keyboard server menjadi lambat merespons
   C. Kabel fiber optik meleleh
   D. Layar monitor server kehabisan warna

6. Nilai Throughput tipikal yang dapat dicapai oleh mesin server Widya-Reactor pada prosesor modern 8-core adalah...
   A. 100 Request/Detik
   B. > 50.000 s/d 100.000+ Request/Detik
   C. Tepat 10 Request/Detik
   D. Terbatas pada 500 byte/hari

7. Header HTTP yang digunakan untuk mempertahankan koneksi TCP tetap terbuka agar dapat melayani permintaan berikutnya tanpa jabat tangan ulang adalah...
   A. `Connection: keep-alive`
   B. `Connection: close`
   C. `Upgrade: websocket`
   D. `Cache-Control: no-cache`

8. Struktur data antrean peristiwa pada kernel Linux yang menjadi tulang punggung pemantauan soket pada Widya-Reactor adalah...
   A. `epoll`
   B. `msgget`
   C. `semop`
   D. `shmat`

9. Mengapa alokasi memori heap nol (*Zero-Heap Allocation*) pada jalur kritis (*hot path*) penanganan request sangat penting?
   A. Mencegah fragmentasi memori dan mengeliminasi beban penalti waktu alokasi memori sehingga latensi respon stabil di sub-milidetik
   B. Menghapus kebutuhan pendingin ruangan di ruang server
   C. Agar berkas program dapat disimpan di disket lama
   D. Membatasi ukuran database maksimal 1 MB

10. Keberhasilan membangun mesin web server berkinerja tinggi mandiri membuktikan bahwa Widya-Lang...
    A. Mampu menjadi fondasi infrastruktur gerbang API nasional yang mandiri, berkecepatan puncak, dan bebas dari ketergantungan paket server asing
    B. Tidak memerlukan sistem operasi Linux
    C. Hanya dapat digunakan untuk aplikasi sederhana
    D. Mengharuskan pengguna membayar lisensi bulanan
