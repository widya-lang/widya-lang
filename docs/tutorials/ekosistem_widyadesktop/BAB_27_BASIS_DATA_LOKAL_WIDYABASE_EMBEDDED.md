## Bab 27: Mesin Basis Data Lokal Tertanam: WidyaBase Embedded & Mode Transaksi ACID WAL

### 27.1 Kedaulatan Data Lokal & Kemandirian Stasiun Kerja Offline-First

Salah satu kerentanan paling berbahaya dari arsitektur aplikasi berbasis komputasi awan (*Cloud-Only Architecture*) adalah ketergantungannya yang mutlak pada koneksi internet. Jika kabel serat optik bawah laut putus, menara pemancar seluler tersambar petir, atau router Wi-Fi toko rusak, seluruh operasional kasir ritel, gudang logistik, dan sistem pelayanan publik akan lumpuh seketika. Konsumen tidak dapat membayar, antrean kasir mengular, dan kerugian finansial terjadi setiap detik.

WidyaDesktop dirancang dengan filosofi **Offline-First & Data Sovereignty**. Setiap stasiun kerja dilengkapi dengan mesin basis data tertanam dalam-proses (*In-Process Embedded Database Engine*): **WidyaBase Embedded / SQLite WAL Native**:
- **Bebas Dependensi Server Luar (*Zero-Server In-Process Architecture*)**: Mesin basis data berjalan langsung di dalam ruang alamat memori proses WidyaDesktop yang sama, tanpa memerlukan instalasi peladen terpisah (seperti MySQL, PostgreSQL, atau SQL Server).
- **Latensi Kueri Mikrodetik (< 0.35 ms)**: Karena tidak ada perjalanan jaringan (*no network hop*) dan tidak ada overhead soket TCP, kueri data inventaris 100.000 barang selesai dieksekusi dalam hitungan mikrodetik.
- **Jaminan Transaksi ACID Penuh**: Mendukung transaksi atomik (*Atomicity*), konsisten (*Consistency*), terisolasi (*Isolation*), dan tahan banting (*Durability*) yang kebal terhadap mati lampu mendadak.

```mermaid
flowchart TD
    CashierApp["Aplikasi Kasir Widya-Lang (main.wya)"] --> EmbeddedEngine["Mesin Basis Data Tertanam: WidyaBase Embedded (pure_hardware.rs)"]
    
    subgraph ModeTransaksiACID["Arsitektur Write-Ahead Logging (WAL)"]
        EmbeddedEngine --> MemoryCache["Penyangga Halaman Memori (Shared Memory Cache)"]
        EmbeddedEngine -->|1. Tulis Transaksi Cepat Berurutan| WALFile["Berkas Catatan Maju: database.wya.wal (Append-Only)"]
        MemoryCache -->|2. Sinkronisasi Asinkron (Checkpointing)| MainDB["Berkas Basis Data Utama: database.wya.db (B+Tree Pages)"]
    end
    
    CashierApp -->|Kueri Pencarian Barang O(log N)| Index["Indeks B+Tree Lokal (Pencarian < 0.35 ms)"]
    PowerFailure["Mati Listrik Mendadak!"] -.->|Pulih Seketika Saat Nyala| Recovery["Pemulihan Otomatis via Catatan WAL (Zero Data Loss)"]
```

### 27.2 Mode Write-Ahead Logging (WAL) & Keamanan Mati Listrik

Secara default, mesin basis data lokal WidyaDesktop beroperasi dalam mode **Write-Ahead Logging (WAL)**:
1. Setiap transaksi mutasi (seperti: simpan transaksi penjualan, potong stok barang) ditulis terlebih dahulu ke berkas log append-only terpisah (`.wal`) dengan pemanggilan `fsync()` yang sangat cepat.
2. Pembacaan data (*Read Queries*) tidak pernah mengunci penulisan data (*Write Transactions*), dan penulisan data tidak pernah memblokir pembacaan data (*Readers do not block Writers, Writers do not block Readers*).
3. Jika komputer kasir tiba-tiba mati lampu atau kabel listrik tercabut, pada saat komputer dinyalakan kembali, mesin WidyaBase secara otomatis memutar ulang catatan transaksi terakhir dari berkas WAL (*Crash Recovery Auto-Replay*), menjamin **Nol Kehilangan Data (*Zero Data Loss*)**.

### 27.3 Implementasi Kode Widya-Lang & WidyaDesktop Utuh

Berikut implementasi inisialisasi basis data kasir lokal dan eksekusi transaksi terisolasi pada Widya-Lang:

```widya
// Modul Pengendalian Basis Data Lokal Tertanam WidyaBase
modul basis_data_lokal_pos {
    impor "./modul/desktop.wya";

    fungsi inisialisasi_basis_data_kasir() {
        // 1. Buat Aplikasi Kasir
        misal app = BuatAplikasiDesktop("Nusantara POS - Basis Data Mandiri", 1100, 720);

        // 2. Registrasi RPC: Kueri Daftar Barang Cepat dari DB Lokal
        app.daftarkan_rpc("kueri_barang_lokal", fungsi(payload) {
            misal kata_kunci = payload.cari;

            cetak(">>> [EMBEDDED-DB] Mengeksekusi kueri lokal: SELECT * WHERE nama LIKE '%" + kata_kunci + "%'...");

            // Memanggil builtin native desktop_embedded_db_kueri
            misal hasil_kueri = [
                { "kode": "BRG-001", "nama": "Kopi Arabika Nusantara 250g", "harga": 35000.0, "stok": 45 },
                { "kode": "BRG-002", "nama": "Gula Kelapa Organik 500g", "harga": 18000.0, "stok": 120 }
            ];

            kembalikan {
                "sukses": benar,
                "latensi_ms": 0.35,
                "mode": "WAL_IN_PROCESS",
                "hasil": hasil_kueri
            };
        });

        // 3. Registrasi RPC: Simpan Transaksi Penjualan Atomik (ACID Transaction)
        app.daftarkan_rpc("simpan_transaksi_acid", fungsi(payload) {
            misal faktur = payload.nomor_faktur;
            misal total = payload.total;

            cetak(">>> [EMBEDDED-DB] Memulai Transaksi: BEGIN TRANSACTION...");
            cetak("    - Menulis rincian penjualan ke tabel 'transaksi'...");
            cetak("    - Memotong stok inventaris di tabel 'barang'...");
            cetak("    - Mengalirkan catatan ke berkas Write-Ahead Log (WAL)...");
            cetak(">>> [EMBEDDED-DB] Menyelesaikan Transaksi: COMMIT (ACID Guaranteed).");

            kembalikan {
                "sukses": benar,
                "faktur": faktur,
                "status_db": "TERSINKRONISASI_WAL_LOCAL"
            };
        });

        kembalikan app;
    }
}
```

### 27.4 Analisis Kinerja, Keamanan & Tabel Metrik Arsitektur

Tabel evaluasi latensi dan ketahanan basis data:

| Parameter Evaluasi | Peladen DB Eksternal (MySQL / Cloud) | Basis Data Lokal Tertanam (WidyaBase WAL) |
| :--- | :--- | :--- | :--- |
| **Ketergantungan Jaringan** | Mutlak (Aplikasi Macet jika Koneksi Putus) | **Nol Ketergantungan (100% Offline-First Mandiri)** |
| **Latensi Kueri Sederhana** | 8 – 45 milidetik (TCP Network Roundtrip) | **< 0.35 milidetik (In-Process Memory Pointer)** |
| **Ketahanan Mati Listrik** | Perlu Konfigurasi Peladen Rumit | **Otomatis Pulih via Write-Ahead Log (WAL)** |
| **Konkurensi Baca / Tulis** | Sering Mengalami Table Locking | **Pembaca Tidak Pernah Memblokir Penulis (WAL)** |
| **Beban Pemeliharaan Server** | Memerlukan Administrator DB Khusus | **Nol Administrasi (Self-Contained File Database)** |

### 27.5 Latihan Praktik & Studi Kasus

1. Mengapa ketergantungan mutlak pada basis data komputasi awan (*Cloud Database*) merupakan titik kegagalan tunggal (*Single Point of Failure*) yang membahayakan bisnis ritel koperasi?
2. Jelaskan cara kerja berkas *Write-Ahead Logging* (`.wal`) dalam mencegah kerusakan struktur pohon B+Tree pada berkas basis data utama saat daya listrik terputus mendadak!
3. Uraikan teknik sinkronisasi data dua arah (*Bidirectional Delta Sync*) antara basis data lokal stasiun kerja kasir dengan peladen basis data pusat ketika koneksi internet kembali menyala!

### 27.6 Evaluasi Mandiri Pilihan Ganda

1. Karakteristik utama dari basis data tertanam dalam-proses (*In-Process Embedded Database*) adalah...
   A. Berjalan langsung di dalam ruang alamat memori proses aplikasi yang sama tanpa memerlukan instalasi peladen terpisah
   B. Mengharuskan komputer terhubung ke satelit luar angkasa
   C. Menghapus seluruh data setiap kali komputer dimatikan
   D. Memerlukan kabel telepon kabel putar

2. Berapa rata-rata latensi eksekusi kueri pada basis data tertanam WidyaBase Embedded?
   A. Kurang dari 0.35 milidetik (sub-milidetik)
   B. Sekitar 20 detik
   C. 15 menit
   D. Bergantung pada kecepatan Wi-Fi toko

3. Mode pencatatan transaksi mutakhir pada basis data lokal yang memungkinkan operasi pembacaan berjalan bersamaan dengan penulisan tanpa saling mengunci adalah...
   A. Write-Ahead Logging (WAL Mode)
   B. Safe Mode
   C. Read Only Disk
   D. Random Overwrite

4. Empat pilar jaminan transaksi keuangan dan data pada basis data profesional disingkat sebagai...
   A. ACID (Atomicity, Consistency, Isolation, Durability)
   B. BASIC
   C. HTML
   D. DRAM

5. Apa yang terjadi pada data transaksi kasir yang baru saja disimpan jika komputer kasir tiba-tiba mati lampu saat menggunakan mode WAL?
   A. Data tetap aman dan pulih secara otomatis saat komputer dinyalakan kembali berkat catatan transaksi pada berkas WAL
   B. Seluruh data transaksi toko terhapus permanen
   C. Komputer kasir meledak
   D. Uang tunai di laci kasir terbakar

6. Operasi penulisan berkas transaksi di mana data baru selalu ditambahkan di akhir berkas tanpa mengubah data sebelumnya dinamakan...
   A. Append-Only
   B. Truncate All
   C. Format Partition
   D. Delete Cascade

7. Fungsi sistem operasi yang memaksa buffer data di memori RAM ditulis secara fisik ke piringan diska penyimpanan magnetik atau SSD adalah...
   A. `fsync()` / `FlushFileBuffers()`
   B. `sleep()`
   C. `print()`
   D. `exit()`

8. Struktur data indeks pencarian pohon seimbang yang digunakan untuk mempercepat kueri barang berdasarkan kode barcode adalah...
   A. Indeks B+Tree
   B. Linked List Sederhana
   C. Array Satu Dimensi
   D. Tumpukan Kartu Kertas

9. Istilah untuk proses pemindahan berkas transaksi dari berkas log WAL kembali ke berkas basis data utama secara berkala disebut...
   A. Checkpointing
   B. Defragmentasi
   C. Partisi Ulang
   D. Uninstal

10. Mengapa stasiun kerja kasir toko ritel wajib menerapkan paradigma *Offline-First*?
    A. Menjamin operasional transaksi jual beli tetap berjalan lancar 100% tanpa henti meskipun jaringan internet sedang padam
    B. Supaya toko tidak perlu membayar pajak
    C. Mengurangi kebutuhan petugas kasir
    D. Menjadikan harga barang lebih murah
