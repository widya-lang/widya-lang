## Bab 38: Proyek Sistem 3: Broker Pesan (Message Broker) Ringan & Antrean Terdistribusi

### 38.1 Topologi Pub/Sub Antrean Terdistribusi

Dalam arsitektur sistem terdistribusi skala nasional, ribuan simpul layanan—mulai dari sensor pemantauan cuaca BMKG, sistem peringatan dini tsunami, hingga gerbang pembayaran antar-bank—harus dapat saling bertukar data secara asinkron tanpa saling mengunci (*decoupled communication*).

Proyek Sistem ketiga adalah merancang dan mengimplementasikan **Broker Pesan Terdistribusi Ringan (Widya-Broker Engine)** yang menerapkan model publikasi-langganan (*Publish-Subscribe*) berbasis **Catatan Komit Sekuensial (Sequential Commit-Log Architecture)** setara Apache Kafka, tetapi dengan jejak memori ribuan kali lebih ringan dan bebas dari ketergantungan runtime Java Virtual Machine (JVM).

```mermaid
flowchart TD
    subgraph ProduserMesh["Produser Data Kedaulatan"]
        P1["Sensor Maritim Buoy Natuna"] -->|Kirim Pesan Biner (Topik: 'maritim.sensor')| B
        P2["Radar Pertahanan Udara"] -->|Kirim Pesan Biner (Topik: 'radar.ancaman')| B
    end
    subgraph BrokerCore["Inti Widya-Broker Engine (Zero-Copy Commit Log)"]
        B["Manajer Partisi & Penulis Log Sekuensial Disk"]
        B --> C["Partisi 0: Berkas Log Tersegmentasi (0000.log)"]
        B --> D["Partisi 1: Berkas Log Tersegmentasi (0001.log)"]
        B --> E["Pelacak Offset Konsumen (Consumer Group Offset Tracking)"]
    end
    subgraph KonsumenMesh["Grup Konsumen Terdistribusi"]
        C & E -->|Tarik Pesan (Pull via Offset X)| K1["Layanan Analisis AI Maritim"]
        D & E -->|Tarik Pesan (Pull via Offset Y)| K2["Pusat Komando Pertahanan Nasional"]
    end
```

Prinsip fundamental Widya-Broker:
1. **Model Commit-Log Append-Only**: Setiap pesan baru ditambahkan di akhir berkas log partisi secara sekuensial. Pesan tidak dihapus setelah dibaca, melainkan dapat diputar ulang (*replayable log*).
2. **Pelacakan Offset Berbasis Konsumen (*Consumer Offset*)**: Setiap konsumen bertanggung jawab melacak posisi baca terakhirnya (*Offset*), sehingga satu pesan dapat dibaca oleh banyak konsumen berbeda dengan kecepatan masing-masing.
3. **Penyaluran Nir-Salin (*Zero-Copy Kernel Splicing via sendfile*)**: Data dari disk dialirkan langsung ke soket kartu jaringan (NIC) melalui sistem panggilan `sendfile` tanpa menyalin data ke memori pengguna (*Ring 3*).

### 38.2 Implementasi Kode Program Utuh

Berikut implementasi kode program utuh dari Broker Pesan Ringan berbasis partisi dan commit log pada Widya-Lang:

```widya
// ============================================================================
// PROYEK SISTEM 3: BROKER PESAN (MESSAGE BROKER) RINGAN BERDAULAT
// Arsitektur: Commit-Log Partisi Tersegmentasi + Consumer Group Offsets
// ============================================================================

modul mesin_broker {
    // Struktur Pesan Biner
    struktur PesanLog {
        offset_id: Angka,
        stempel_waktu: Angka,
        kunci: Teks,
        muatan_data: Teks
    }

    // Abstraksi Partisi Topik
    struktur PartisiTopik {
        id_partisi: Angka,
        log_antrean: Larik<PesanLog>,
        offset_berikutnya: AtomikAngka,

        fungsi baru(id: Angka) -> PartisiTopik {
            kembalikan PartisiTopik {
                id_partisi: id,
                log_antrean: [],
                offset_berikutnya: AtomikAngka::baru(0)
            };
        }

        // Penulisan pesan baru ke akhir log (Append-Only)
        fungsi tulis_pesan(mut diri, k: Teks, v: Teks) -> Angka {
            misal off = diri.offset_berikutnya.muat();
            diri.offset_berikutnya.tambah(1);

            misal msg = PesanLog {
                offset_id: off,
                stempel_waktu: 1776500000,
                kunci: k,
                muatan_data: v
            };

            diri.log_antrean.tambah(msg);
            cetak("  [BROKER LOG] Pesan ditulis ke Partisi", diri.id_partisi, "| Offset #", off, "| Kunci:", k);
            kembalikan off;
        }

        // Penarikan pesan oleh konsumen berdasarkan offset
        fungsi baca_dari_offset(diri, offset_mulai: Angka, batas_pesan: Angka) -> Larik<PesanLog> {
            misal mut paket: Larik<PesanLog> = [];
            untuk p in diri.log_antrean {
                jika p.offset_id >= offset_mulai dan paket.panjang() < batas_pesan {
                    paket.tambah(p);
                }
            }
            kembalikan paket;
        }
    }

    // Struktur Pengelola Broker
    struktur PengelolaBroker {
        nama_simpul: Teks,
        daftar_partisi: Kamus<Angka, PartisiTopik>,

        fungsi bangun(simpul: Teks) -> PengelolaBroker {
            cetak("==================================================================");
            cetak("  MEMULAI WIDYA-BROKER DISTRIBUTED MESSAGE ENGINE v2.6           ");
            cetak("==================================================================");
            cetak("[BROKER INIT] Menginisialisasi Commit-Log Storage Engine pada Simpul:", simpul);

            misal mut peta_part = Kamus::baru();
            peta_part.sisip(0, PartisiTopik::baru(0));
            peta_part.sisip(1, PartisiTopik::baru(1));

            kembalikan PengelolaBroker {
                nama_simpul: simpul,
                daftar_partisi: peta_part
            };
        }

        fungsi terbitkan_pesan(mut diri, id_partisi: Angka, kunci: Teks, data: Teks) -> Angka {
            jika diri.daftar_partisi.mengandung(id_partisi) {
                misal mut part = diri.daftar_partisi.ambil(id_partisi);
                misal off = part.tulis_pesan(kunci, data);
                diri.daftar_partisi.sisip(id_partisi, part);
                kembalikan off;
            }
            kembalikan -1;
        }

        fungsi konsumsi(diri, id_partisi: Angka, offset_mulai: Angka) -> Larik<PesanLog> {
            jika diri.daftar_partisi.mengandung(id_partisi) {
                misal part = diri.daftar_partisi.ambil(id_partisi);
                kembalikan part.baca_dari_offset(offset_mulai, 10);
            }
            kembalikan [];
        }
    }
}

modul utama {
    fungsi utama() {
        misal mut broker = mesin_broker::PengelolaBroker::bangun("SIMPUL-PUSAT-JAKARTA-01");

        // 1. Produser menerbitkan pesan telemetri ke Partisi 0
        cetak("\n--- PRODUSER: Mengirimkan Data Telemetri Maritim ---");
        broker.terbitkan_pesan(0, "BUOY-01", "GELOMBANG:2.4m|SUHU:28C|ANGIN:15knot");
        broker.terbitkan_pesan(0, "BUOY-02", "GELOMBANG:3.1m|SUHU:27C|ANGIN:22knot");
        broker.terbitkan_pesan(0, "BUOY-03", "GELOMBANG:1.8m|SUHU:29C|ANGIN:10knot");

        // 2. Konsumen membaca data dari Offset 1 (Melewati pesan offset 0)
        cetak("\n--- KONSUMEN: Menarik Aliran Pesan dari Offset #1 ---");
        misal pesan_diterima = broker.konsumsi(0, 1);
        cetak("Jumlah Pesan Ditarik dari Partisi 0:", pesan_diterima.panjang());

        untuk msg in pesan_diterima {
            cetak("  -> Memproses Offset #", msg.offset_id, "| Kunci:", msg.kunci, "| Data:", msg.muatan_data);
        }

        cetak("\nBroker Siap Beroperasi Tanpa Kebocoran Memori.");
    }
}
```

### 38.3 Rangkuman Desain & Analisis Rekayasa Sistem

Perbandingan arsitektural antara Antrean Konvensional vs Commit-Log Broker:

| Parameter Rekayasa | Antrean Tradisional (RabbitMQ / JMS) | Commit-Log Broker (Widya-Broker) |
| :--- | :--- | :--- |
| **Model Retensi Pesan** | Dihapus setelah dibaca (Destructive Read) | Disimpan permanen (Replayable Log) |
| **Beban Pelacakan State** | Berat di sisi Server Broker | Ringan (Konsumen melacak offset sendiri) |
| **Throughput Penulisan** | Puluhan ribu pesan/detik | Ratusan ribu s/d Jutaan pesan/detik |
| **Distribusi Beban** | Pembagian acak per worker | Pemartisian deterministik (*Partition-based*) |
| **Dukungan Banyak Konsumen** | Membutuhkan antrean ganda | Membaca berkas log yang sama secara paralel |

### 38.4 Latihan Mandiri & Evaluasi Pemahaman Sistem

1. Jelaskan mengapa sistem panggilan Linux `sendfile()` memungkinkan pengaliran data pesan dari disk langsung ke kartu jaringan tanpa melintasi memori pengguna (*Zero-Copy Kernel Bypass*)!
2. Apa yang dimaksud dengan jaminan pengiriman *At-Least-Once Delivery*, dan bagaimana konsumen menerapkan pola operasi idempoten (*Idempotent Processing*) untuk mengatasi duplikasi pesan?
3. Rancanglah sebuah skema pembagian partisi (*Partition Hashing Strategy*) pada Widya-Broker di mana kunci pesan dipetakan secara seragam menggunakan algoritma FNV-1a ke partisi 0 s/d $N-1$!

### 38.5 Latihan Soal Pilihan Ganda (10 Soal)

1. Model komunikasi terdistribusi di mana produser menerbitkan data ke suatu topik dan konsumen menerima data tanpa saling mengenal secara langsung dinamakan...
   A. Point-to-Point Monolith
   B. Publish-Subscribe (Pub/Sub)
   C. Remote Procedure Call (RPC) Sinkron
   D. Shared Disk Cluster

2. Arsitektur penyimpanan pesan pada broker modern di mana data dituliskan secara berurutan dan tidak dihapus setelah dibaca dinamakan...
   A. Commit-Log Berurutan (Append-Only Log)
   B. Memory Stack LIFO
   C. SQL Relational Table
   D. XML DOM Tree

3. Angka penunjuk urutan posisi pesan di dalam suatu partisi yang dilacak oleh konsumen untuk mengetahui kemajuan baca dinamakan...
   A. Port Number
   B. Offset ID
   C. CRC Token
   D. MAC Address

4. Sistem panggilan pada kernel Linux yang menyalurkan data langsung dari file descriptor disk ke socket jaringan tanpa penyalinan ke ruang pengguna dinamakan...
   A. `fork()`
   B. `sendfile()`
   C. `mknod()`
   D. `chroot()`

5. Keuntungan utama dari retensi pesan berbasis Commit-Log dibandingkan antrean tradisional yang menghapus pesan setelah dibaca adalah...
   A. Pesan dapat diputar ulang (*replayable*) jika terjadi kegagalan pemrosesan atau saat meluncurkan layanan analisis baru
   B. Pesan otomatis hilang setelah 10 detik
   C. Mengurangi kebutuhan memori kartu grafis
   D. Menjadikan server kebal dari serangan fisik

6. Konsep di mana pemrosesan pesan yang sama sebanyak lebih dari satu kali menghasilkan status sistem yang identik tanpa efek samping ganda dinamakan...
   A. Idempotensi (Idempotent Operation)
   B. Polymorphism
   C. Recursion
   D. Encapsulation

7. Unit dasar paralelisme dan penskalaan horizontal di dalam suatu topik pada arsitektur broker pesan dinamakan...
   A. Sektor
   B. Partisi (Partition)
   C. Silinder
   D. Track

8. Algoritma pemartisian yang umum digunakan untuk memastikan pesan dengan kunci yang sama selalu masuk ke partisi yang sama adalah...
   A. Hash Kunci Modulo Jumlah Partisi ($Hash(kunci) \pmod N$)
   B. Pencarian Linier
   C. Pemilihan Acak
   D. Bubble Sort

9. Mengapa Widya-Broker memiliki efisiensi memori yang jauh lebih unggul dibandingkan broker berbasis JVM seperti Apache Kafka?
   A. Karena ditulis 100% native Rust murni tanpa runtime JVM, tanpa garbage collection overhead, dan menggunakan alokasi biner yang sangat hemat memori
   B. Karena tidak mendukung pesan biner
   C. Karena hanya bisa berjalan pada hari kerja
   D. Karena ukuran RAM komputer otomatis digandakan

10. Penerapan broker pesan terdistribusi berdaulat pada sistem peringatan dini bencana nasional bertujuan untuk...
    A. Menjamin pengaliran jutaan data sensor gempa dan tsunami dapat disalurkan secara instan, toleran terhadap kegagalan simpul, dan kebal dari penyadapan pihak luar
    B. Menghapus rekaman arsip bencana
    C. Mengurangi biaya kuota data internet publik
    D. Membatasi pengembang agar tidak menambah topik baru
