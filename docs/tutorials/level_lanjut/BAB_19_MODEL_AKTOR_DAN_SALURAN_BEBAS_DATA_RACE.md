## Bab 19: Model Aktor & Saluran Komunikasi Bebas Data-Race (Channels)

### 19.1 Filosofi Komunikasi Antar-Aktor

Meskipun penguncian memori bersama (*shared memory*) via Mutex sangat berguna, pada sistem terdistribusi berskala besar koordinasi berbasis penguncian rentan menimbulkan masalah kebuntuan (*deadlock*), perebutan kunci (*lock contention*), dan arsitektur yang sulit dipahami.

Widya-Lang mengadopsi filosofi arsitektur konkurensi modern:
> *"Jangan berkomunikasi dengan berbagi memori; melainkan bagilah memori dengan saling berkomunikasi."*

```mermaid
flowchart TD
    subgraph ModelAktor["Arsitektur Saluran FIFO Bebas Data-Race"]
        P1["Produser A (Utas 1)"] -->|saluran_kirim(pesan)| S["Saluran Komunikasi FIFO Aman (Pipa Data Terisolasi)"]
        P2["Produser B (Utas 2)"] -->|saluran_kirim(pesan)| S
        S -->|saluran_terima()| K["Konsumen Aktor (Utas 3: Pemilik Tunggal State)"]
        K --> M["State Internal Dimutasi Aman Tanpa Mutex!"]
    end
```

Karakteristik fundamental Saluran Komunikasi (*Channels*) Widya-Lang:
1. **Model Produser-Konsumen FIFO**: Pesan dialirkan melalui antrean berurut yang aman secara multi-thread.
2. **Transfer Kepemilikan Data**: Ketika produser mengirim sebuah objek melalui saluran, kepemilikan objek tersebut **berpindah** ke saluran dan diterima oleh konsumen. Produser tidak lagi memiliki akses ke objek tersebut sehingga *data race* mustahil terjadi.
3. **Dua Varian Kapasitas**:
   - **Saluran Terbatas (*Bounded Channel*)**: Memiliki batas kapasitas antrean tertentu. Jika antrean penuh, pengirim akan menunggu (*backpressure support*).
   - **Saluran Tak Terbatas (*Unbounded Channel*)**: Menampung pesan dinamis sesuai ketersediaan memori sistem.

### 19.2 Implementasi Kode Program Utuh

Berikut implementasi sistem antrean pemrosesan dokumen pertahanan menggunakan Model Aktor berbasis saluran:

```widya
// Modul Pesan Protokol Antar-Utas
pilihan PesanAudit {
    MuatData { id_dokumen: Teks, prioritas: Angka },
    VerifikasiSandi(Teks),
    PerintahSelesai
}

modul aktor_pemroses {
    impor super::PesanAudit;

    // Fungsi pekerja konsumen (Aktor Penerima)
    fungsi jalankan_aktor_konsumen(pipa_masuk: SaluranTerima<PesanAudit>) {
        cetak("[AKTOR AUDIT] Konsumen siap mendengarkan antrean saluran...");
        misal mut berjalan = benar;
        misal mut total_dokumen = 0;

        selama berjalan {
            // Menerima pesan secara sinkron dari saluran
            misal pesan = saluran_terima(pipa_masuk);
            cocokkan pesan {
                PesanAudit::MuatData { id_dokumen, prioritas } => {
                    total_dokumen += 1;
                    cetak("  [PROSES] Dokumen:", id_dokumen, "| Prioritas Tingkat:", prioritas);
                },
                PesanAudit::VerifikasiSandi(token) => {
                    cetak("  [VERIFIKASI] Memvalidasi token sandi:", token);
                },
                PesanAudit::PerintahSelesai => {
                    cetak("[AKTOR AUDIT] Menerima sinyal terminasi. Total Dokumen Diproses:", total_dokumen);
                    berjalan = salah;
                }
            }
        }
    }
}

modul utama {
    fungsi utama() {
        cetak("=== SISTEM MODEL AKTOR & SALURAN FIFO WIDYA ===");

        // 1. Membuat pasangan saluran: Pengirim (Tx) dan Penerima (Rx)
        misal (tx, rx) = saluran_terbatas(10);

        // 2. Meluncurkan aktor konsumen pada utas latar belakang
        misal handle_konsumen = buat_utas(fungsi() {
            aktor_pemroses::jalankan_aktor_konsumen(rx);
        });

        // 3. Utas produser mengirim serangkaian pesan aman
        saluran_kirim(tx, PesanAudit::MuatData { id_dokumen: "DOK-RAHASIA-001", prioritas: 1 });
        saluran_kirim(tx, PesanAudit::VerifikasiSandi("KEY-PQC-KYBER-768"));
        saluran_kirim(tx, PesanAudit::MuatData { id_dokumen: "DOK-MARITIM-002", prioritas: 2 });
        saluran_kirim(tx, PesanAudit::PerintahSelesai);

        // 4. Menunggu aktor konsumen menuntaskan seluruh antrean
        gabung_utas(handle_konsumen);

        cetak("Seluruh saluran komunikasi telah ditutup secara bersih.");
    }
}
```

### 19.3 Rangkuman Desain & Analisis Rekayasa Sistem

Perbandingan arsitektural antara Shared Memory Mutex dengan Actor Model Channel:

| Parameter Arsitektur | Shared Memory (Mutex) | Actor Model (Saluran Komunikasi) |
| :--- | :--- | :--- |
| **Model Sinkronisasi** | Penguncian memori fisik bersama | Pertukaran pesan terisolasi (*Message Passing*) |
| **Risiko Kebuntuan (Deadlock)** | Tinggi jika urutan kunci salah | Sangat Rendah (Topologi aliran searah) |
| **Tekanan Balik (Backpressure)** | Sulit diterapkan | Alami melalui kapasitas *Bounded Channel* |
| **Kemudahan Pengujian** | Sulit diuji karena status bersama | Sangat mudah diuji secara modular per-aktor |

### 19.4 Latihan Mandiri & Evaluasi Pemahaman Sistem

1. Jelaskan bagaimana semantik pemindahan kepemilikan (*move semantics*) pada Widya-Lang menjamin bahwa pengirim pesan tidak dapat lagi memutasi data setelah mengirimkannya ke dalam saluran!
2. Apa yang dimaksud dengan fenomena *Backpressure* dalam sistem terdistribusi, dan bagaimana saluran berkapasitas terbatas (*bounded channel*) mencegah kehabisan memori (*Out-Of-Memory*)?
3. Rancang sebuah pola *Worker Pool* di mana sebuah saluran didistribusikan ke 4 utas pekerja independen yang berlomba memproses paket data (*competing consumers*)!

### 19.5 Latihan Soal Pilihan Ganda (10 Soal)

1. Filosofi desain konkurensi modern yang mendasari penggunaan Saluran Komunikasi (Channels) pada Widya-Lang adalah...
   A. Jangan berkomunikasi dengan berbagi memori; bagilah memori dengan saling berkomunikasi
   B. Simpan semua data di memori video kartu grafis
   C. Gunakan kunci mutex di setiap baris fungsi
   D. Hindari pembuatan utas paralel

2. Fungsi bawaan yang digunakan untuk membuat saluran komunikasi baru dengan kapasitas antrean tertentu adalah...
   A. `buat_pipa()`
   B. `saluran_terbatas(kapasitas)`
   C. `inisiasi_jaringan()`
   D. `alokasi_antrean()`

3. Dua komponen yang dihasilkan dari pembuatan sebuah saluran komunikasi adalah...
   A. Monitor dan Keyboard
   B. Pengirim (Sender / Tx) dan Penerima (Receiver / Rx)
   C. Heap dan Stack
   D. Input dan Output berkas

4. Apa yang terjadi pada kepemilikan data saat sebuah variabel dikirimkan melalui saluran dengan `saluran_kirim(tx, data)`?
   A. Kepemilikan berpindah sepenuhnya ke saluran, dan variabel asal dinonaktifkan oleh kompilator
   B. Data diduplikasi menjadi 10 salinan
   C. Variabel asal diubah menjadi nilai null
   D. Data langsung dihapus dari disk

5. Saluran yang menahan proses pengirim (*blocking sender*) ketika kapasitas antreannya telah penuh dinamakan...
   A. Unbounded Channel
   B. Bounded Channel (Saluran Terbatas)
   C. Circular Bus
   D. Dynamic Pipeline

6. Mekanisme di mana produser memperlambat laju pengiriman data karena konsumen belum sanggup memproses antrean dikenal sebagai...
   A. Backpressure (Tekanan Balik)
   B. Memory Swapping
   C. Stack Unwinding
   D. Priority Boosting

7. Operasi penerimaan data `saluran_terima(rx)` secara default bersifat...
   A. Asinkron tanpa menunggu
   B. Menunggu secara sinkron (*blocking*) hingga setidaknya ada satu pesan yang tiba di saluran
   C. Mematikan sistem operasi jika antrean kosong
   D. Mengembalikan nilai acak

8. Di dalam Model Aktor, status internal (*state*) dari suatu entitas aktor dimodifikasi secara...
   A. Bersama-sama oleh ratusan thread asing
   B. Terisolasi hanya oleh aktor itu sendiri secara sekuensial saat memproses pesan yang masuk
   C. Melalui modifikasi langsung di BIOS komputer
   D. Melalui instruksi terminal jarak jauh

9. Mengapa Model Aktor berbasis saluran dianggap lebih aman dibandingkan penguncian Mutex konvensional pada sistem skala besar?
   A. Karena mengeliminasi risiko perebutan kunci dan kebuntuan (*deadlock*) melalui isolasi data yang ketat
   B. Karena tidak membutuhkan pasokan listrik
   C. Karena dapat dijalankan tanpa kompilasi
   D. Karena ukuran kode program selalu 10 baris

10. Penerapan Model Aktor pada sistem komando dan kendali pertahanan negara bertujuan untuk...
    A. Menjamin setiap instruksi militer dan pembaruan radar diproses secara terurut, deterministik, dan bebas dari balapan data (*data-race free*)
    B. Menghapus rekaman arsip setiap 1 jam
    C. Mempercepat koneksi radio AM/FM
    D. Membatasi pengembang agar tidak menambah modul baru
