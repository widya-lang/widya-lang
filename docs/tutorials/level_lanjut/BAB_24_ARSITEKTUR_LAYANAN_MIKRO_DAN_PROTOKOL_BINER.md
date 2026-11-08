## Bab 24: Arsitektur Layanan Mikro Berdaulat & Protokol Biner

### 24.1 Topologi Komunikasi Layanan Mikro

Dalam arsitektur perangkat lunak skala enterprise dan sistem komputasi terdistribusi nasional, aplikasi monolitik raksasa dipecah menjadi kumpulan **Layanan Mikro (Microservices)** yang independen, terisolasi, dan berkomunikasi melalui jaringan. Namun, penggunaan format teks seperti JSON atau XML untuk komunikasi internal antar-layanan (*East-West traffic*) menimbulkan pemborosan bandwidth yang masif dan beban CPU tinggi akibat parsing teks berulang-ulang.

Widya-Lang mengadopsi **Protokol Serialisasi Biner Ringan Berdaulat (Sovereign Binary Serialization)** yang kompak, cepat, dan terikat skema statis tanpa ketergantungan pada runtime engine asing yang membengkak.

```mermaid
flowchart TD
    A["Klien Eksternal / Web Gateway"] -->|HTTP/JSON Publik| B["Pintu Gerbang Layanan (API Gateway Nusantara)"]
    subgraph MeshLayanan["Jaringan Internal Berdaulat: Komunikasi Protokol Biner"]
        B -->|Paket Biner Cepat (ID: 0x01)| C["Layanan Otentikasi & Identitas Digital"]
        B -->|Paket Biner Cepat (ID: 0x02)| D["Layanan Transaksi & Perbendaharaan Negara"]
        D -->|Paket Biner Cepat (ID: 0x03)| E["Mesin Basis Data Terdistribusi"]
    end
    C --> F["Pola Circuit Breaker & Health Check Otomatis"]
    D --> F
```

Prinsip fundamental komunikasi layanan mikro Widya-Lang:
1. **Serialisasi Biner Kompak**: Data ditransformasikan menjadi urutan byte terstruktur yang langsung memetakan ke tata letak memori tanpa overhead tag teks JSON.
2. **Pola Ketahanan Sistem (Fault-Tolerance Patterns)**:
   - **Circuit Breaker (Pemutus Sirkuit)**: Mencegah kegagalan satu layanan merambat ke seluruh ekosistem (*cascading failure*).
   - **Retry dengan Exponential Backoff**: Mengulang pemanggilan yang gagal sementara dengan jeda waktu eksponensial.
3. **Penemuan Layanan Mandiri (*Self-Healing Service Discovery*)**: Layanan secara otomatis mendaftarkan status kesehatannya (*heartbeat*) ke pendaftar simpul nasional.

### 24.2 Implementasi Kode Program Utuh

Berikut implementasi enkoder-dekoder protokol biner kustom dan implementasi pola Circuit Breaker pada Widya-Lang:

```widya
// Modul Protokol Biner Antar-Layanan
modul protokol_biner {
    // Pola Status Pemutus Sirkuit (Circuit Breaker)
    pilihan StatusSirkuit {
        Tertutup,     // Normal: seluruh lalu lintas dialirkan
        Terbuka,      // Gagal: lalu lintas diblokir instan untuk perlindungan
        SetengahTerbuka // Pengujian pemulihan layanan
    }

    // Struktur Paket Biner Kompak
    struktur PaketBinerLayanan {
        id_pesan: Angka,        // 16-bit
        kode_layanan: Angka,    // 8-bit
        muatan_data: Larik<Angka>
    }

    // Serialisasi struktur menjadi aliran byte mentah
    fungsi serialisasi(paket: PaketBinerLayanan) -> Larik<Angka> {
        misal mut bytes: Larik<Angka> = [];
        // Header biner: Magic Byte 0xAA 0x55
        bytes.tambah(0xAA);
        bytes.tambah(0x55);
        bytes.tambah((paket.id_pesan >> 8) & 0xFF);
        bytes.tambah(paket.id_pesan & 0xFF);
        bytes.tambah(paket.kode_layanan);
        bytes.tambah(paket.muatan_data.panjang());
        
        untuk b in paket.muatan_data {
            bytes.tambah(b);
        }
        kembalikan bytes;
    }

    // Deserialisasi aliran byte kembali ke objek biner
    fungsi deserialisasi(bytes: Larik<Angka>) -> Hasil<PaketBinerLayanan, Teks> {
        jika bytes.panjang() < 6 {
            kembalikan Hasil::Gagal("Ukuran byte paket terlalu pendek!");
        }
        jika bytes.ambil(0) != 0xAA atau bytes.ambil(1) != 0x55 {
            kembalikan Hasil::Gagal("Magic Byte protokol tidak valid!");
        }

        misal id = (bytes.ambil(2) << 8) | bytes.ambil(3);
        misal kode = bytes.ambil(4);
        misal pjg = bytes.ambil(5);

        misal mut payload: Larik<Angka> = [];
        untuk i dalam 6..(6 + pjg) {
            payload.tambah(bytes.ambil(i));
        }

        kembalikan Hasil::Sukses(PaketBinerLayanan {
            id_pesan: id,
            kode_layanan: kode,
            muatan_data: payload
        });
    }
}

// Modul Pengelola Sirkuit Perlindungan
struktur PemutusSirkuit {
    kegagalan_beruntun: Angka,
    ambang_batas: Angka,
    status: protokol_biner::StatusSirkuit,

    fungsi baru(batas: Angka) -> PemutusSirkuit {
        kembalikan PemutusSirkuit {
            kegagalan_beruntun: 0,
            ambang_batas: batas,
            status: protokol_biner::StatusSirkuit::Tertutup
        };
    }

    fungsi catat_kegagalan(mut diri) {
        diri.kegagalan_beruntun += 1;
        jika diri.kegagalan_beruntun >= diri.ambang_batas {
            diri.status = protokol_biner::StatusSirkuit::Terbuka;
            cetak("[PERINGATAN SIRKUIT] Ambang batas terlampaui! Status berubah: TERBUKA (Isolasi Layanan)!");
        }
    }

    fungsi catat_sukses(mut diri) {
        diri.kegagalan_beruntun = 0;
        diri.status = protokol_biner::StatusSirkuit::Tertutup;
    }
}

modul utama {
    fungsi utama() {
        cetak("=== SISTEM PROTOKOL BINER & LAYANAN MIKRO WIDYA ===");

        // 1. Membangun paket data biner
        misal pkt = protokol_biner::PaketBinerLayanan {
            id_pesan: 1042,
            kode_layanan: 0x05, // Layanan Keuangan
            muatan_data: [10, 20, 30, 40]
        };

        misal aliran_byte = protokol_biner::serialisasi(pkt);
        cetak("Hasil Serialisasi Biner (Panjang:", aliran_byte.panjang(), "byte)");

        // 2. Deserialisasi kembali
        misal hasil_decode = protokol_biner::deserialisasi(aliran_byte);
        cocokkan hasil_decode {
            Hasil::Sukses(p) => {
                cetak("[DECODE OK] ID Pesan Diterima:", p.id_pesan);
                cetak("[DECODE OK] Kode Layanan    :", p.kode_layanan);
                cetak("[DECODE OK] Ukuran Payload  :", p.muatan_data.panjang(), "byte");
            },
            Hasil::Gagal(e) => cetak("[GALAT PROTOKOL]", e)
        }

        // 3. Uji Coba Circuit Breaker
        misal mut cb = PemutusSirkuit::baru(3);
        cb.catat_kegagalan();
        cb.catat_kegagalan();
        cb.catat_kegagalan(); // Memicu pembukaan sirkuit
    }
}
```

### 24.3 Rangkuman Desain & Analisis Rekayasa Sistem

Perbandingan format komunikasi antar-layanan terdistribusi:

| Karakteristik | Format Teks JSON | Format XML | Protokol Biner Widya-Lang |
| :--- | :--- | :--- | :--- |
| **Kepadatan Ukuran Data** | Sedang (Banyak kunci teks berulang) | Rendah (Sangat boros tag) | Sangat Tinggi (Bit/byte padat) |
| **Kecepatan Serialisasi** | Lambat (Perlu konversi string ke angka) | Sangat Lambat (Parsing DOM pohon) | Ultra Cepat (Nol konversi teks) |
| **Beban Penggunaan CPU** | Signifikan pada throughput tinggi | Sangat Berat | Minimal (Zero-Cost Alignment) |
| **Keterbacaan Manusia** | Mudah dibaca di terminal | Mudah dibaca | Membutuhkan alat dekoder biner |

### 24.4 Latihan Mandiri & Evaluasi Pemahaman Sistem

1. Jelaskan mengapa penggunaan protokol biner pada komunikasi internal antar-layanan (*East-West traffic*) dapat menghemat bandwidth jaringan hingga 70% dibandingkan format teks JSON!
2. Bagaimana pola arsitektur *Circuit Breaker* mencegah terjadinya fenomena kegagalan beruntun (*cascading failure*) yang dapat melumpuhkan seluruh pusat data nasional?
3. Rancang sebuah format protokol biner dengan enkripsi simetris terintegrasi (Magic Byte + Nomor Sesi + IV Nonce + Payload Terenkripsi + Tag MAC) pada Widya-Lang!

### 24.5 Latihan Soal Pilihan Ganda (10 Soal)

1. Gaya arsitektur perangkat lunak yang membagi sistem menjadi kumpulan layanan kecil yang independen dan berkomunikasi melalui jaringan disebut...
   A. Arsitektur Monolitik
   B. Arsitektur Layanan Mikro (Microservices)
   C. Pemrograman Prosedural
   D. Model Batch File

2. Keunggulan utama protokol komunikasi biner dibandingkan format teks seperti JSON pada komunikasi internal antar-layanan adalah...
   A. Ukuran paket jauh lebih padat dan proses serialisasi/deserialisasi tidak membutuhkan pemindaian teks CPU yang mahal
   B. Protokol biner tidak membutuhkan kabel jaringan
   C. Dapat dibaca langsung menggunakan mesin ketik
   D. Menjadikan server kebal terhadap debu

3. Byte penanda khusus di awal paket biner yang digunakan untuk memvalidasi bahwa paket tersebut sah sesuai spesifikasi protokol dinamakan...
   A. Magic Bytes
   B. Checksum Bit
   C. Null Terminator
   D. Parity Flag

4. Pola ketahanan arsitektur sistem (*resilience pattern*) yang secara otomatis memutus aliran permintaan ke layanan yang sedang tumbang untuk mencegah kelebihan beban dinamakan...
   A. Circuit Breaker (Pemutus Sirkuit)
   B. Load Balancer
   C. Reverse Proxy
   D. Garbage Collector

5. Tiga status utama dalam pola Circuit Breaker adalah...
   A. Tertutup (Normal), Terbuka (Isolasi), dan Setengah-Terbuka (Uji Pemulihan)
   B. Merah, Kuning, dan Hijau
   C. Masuk, Proses, dan Keluar
   D. Sinkron, Asinkron, dan Paralel

6. Pola pengulangan permintaan dengan penundaan waktu yang meningkat secara eksponensial saat terjadi kegagalan sementara dinamakan...
   A. Linear Delay
   B. Exponential Backoff with Jitter
   C. Random Waiting
   D. Infinite Loop

7. Komponen dalam arsitektur layanan mikro yang bertugas sebagai pintu gerbang tunggal untuk menerima dan menyalurkan permintaan dari klien luar adalah...
   A. Database Engine
   B. API Gateway
   C. Worker Thread
   D. Compacting Garbage Collector

8. Mengapa penambahan faktor keacakan (*Jitter*) pada algoritma *Exponential Backoff* sangat dianjurkan?
   A. Agar server menghasilkan angka keberuntungan
   B. Untuk mencegah terjadinya fenomena badai permintaan serentak (*Thundering Herd Problem*) saat layanan pulih
   C. Karena memori komputer bekerja secara acak
   D. Untuk mengubah alamat IP secara dinamis

9. Istilah *East-West Traffic* dalam topologi jaringan pusat data modern mengacu pada...
   A. Lalu lintas komunikasi internal antar-server/antar-layanan di dalam pusat data
   B. Lalu lintas dari pengguna internet masuk ke server (North-South)
   C. Sinyal siaran radio navigasi kapal
   D. Transmisi satelit cuaca

10. Mengapa standarisasi protokol biner mandiri sangat penting bagi infrastruktur kedaulatan data nasional?
    A. Menjamin kerahasiaan komunikasi antar-lembaga negara dari pemindaian perangkat sniffer pihak asing yang lazim memanfaatkan kelemahan protokol teks terbuka
    B. Menghapus kebutuhan pengujian perangkat lunak
    C. Mengurangi kebutuhan pendingin ruangan di server room
    D. Mengharuskan penulisan program hanya menggunakan huruf kapital
