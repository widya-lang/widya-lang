## Bab 33: Enkripsi Data Diam (TDE) & Keamanan Kriptografi

### 33.1 Perlindungan Lapisan Fisik: Kriptografi Berdaulat Tanpa Kebocoran

Keamanan basis data tidak cukup hanya mengandalkan kata sandi login pengguna atau firewall jaringan. Ancaman pencurian data fisik—seperti pencurian media SSD dari rak data center, kebocoran berkas cadangan (*backup snapshots*), atau akses ilegal langsung ke tingkat sistem berkas piringan disk—menuntut perlindungan kriptografis pada lapisan penyimpanan terdalam.

WidyaBase menghadirkan subsistem **Enkripsi Data Diam (*Transparent Data Encryption* / TDE)** dan integritas kriptografi berdaulat:
- **Enkripsi Halaman Transparan 4KB**: Setiap blok halaman slotted-page dan segmen berkas WAL dienkripsi sebelum dituliskan ke disk fisik (`fsync`) menggunakan algoritma enkripsi simetris standar militer **AES-256-GCM** atau **ChaCha20-Poly1305**.
- **Integritas Terotentikasi (*Authenticated Encryption with Associated Data* / AEAD)**: Menggunakan Tag Otentikasi 128-bit yang menjamin bahwa modifikasi atau korupsi bit sekecil apa pun pada media disk fisik akan langsung terdeteksi seketika sebelum data dibaca ke dalam RAM Buffer Pool.
- **Hierarki Kunci Dua Tingkat (*Two-Tier Key Hierarchy*)**:
  - *Master Key* (Kunci Master): Disimpan di modul keamanan perangkat keras (*Hardware Security Module* / HSM) eksternal atau KMS berdaulat.
  - *Table Encryption Keys* (Kunci Tabel / DEK): Kunci unik per tabel yang dienkripsi (*wrapped*) menggunakan Kunci Master.
- **Kriptografi Kolom Tertarget (*Column-Level Cryptography*)**: Menyediakan fungsi bawaan untuk mengenkripsi kolom data yang sangat sensitif (seperti NIK, nomor kartu kredit, nomor rekam medis) menggunakan kunci acak per baris.

```mermaid
flowchart TD
    subgraph MemoriRAM["Buffer Pool RAM (Data Terbuka / Dekripsi)"]
        P1["Halaman 4KB Bersih (Plaintext Slotted-Page)"]
    end
    subgraph MesinTDE["Mesin TDE Kriptografi WidyaBase"]
        KMS["Kunci Enkripsi Data (DEK AES-256)"]
        IV["Inisialisasi Vektor Unik (HalamanID + LSN)"]
        AEAD["Enkripsi Terotentikasi AES-256-GCM / Poly1305"]
    end
    subgraph MediaDiskFisik["Media Penyimpanan Fisik (Terenkripsi 100%)"]
        CIPHER["Chipertext Halaman 4KB + Tag Otentikasi 16-Byte"]
    end
    P1 -->|Sebelum Ditulis ke Disk (Flushing)| AEAD
    KMS & IV --> AEAD
    AEAD -->|Tulis Aman Tanpa Kebocoran| CIPHER
    CIPHER -->|Saat Dibaca ke Buffer Pool| AEAD
    AEAD -->|Verifikasi Tag Otentikasi Sukses| P1
```

Keunggulan arsitektur TDE WidyaBase:
1. **Transparan bagi Aplikasi**: Aplikasi pengguna, kueri SQL, dan indeks B+Tree berjalan secara normal tanpa perlu memodifikasi kode program klien.
2. **Pencegahan Serangan Replay**: Nilai Initialization Vector (IV) diturunkan dari gabungan `PageID` dan `PageLSN`, memastikan dua halaman yang memiliki isi data yang sama akan menghasilkan ciphertext yang berbeda total di media disk fisik.
3. **Pemusnahan Kriptografis Instan (*Crypto-Shredding*)**: Membuang atau menghapus kunci DEK tabel tertentu akan memusnahkan seluruh data tabel tersebut secara instan dan tidak dapat dipulihkan oleh pihak manapun di dunia.

### 33.2 Implementasi Kode Program Utuh

Berikut implementasi inisialisasi TDE, enkripsi halaman berorientasi AEAD, dan kueri data terenkripsi pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Enkripsi Data Diam (TDE) dan Kriptografi Berdaulat WidyaBase
modul enkripsi_tde {
    impor ekosistem::widyabase::{
        WidyaBase, MesinTDE, AlgoritmaEnkripsi, KunciDEK,
        HalamanID, BufferHalaman, HasilKripto
    };

    fungsi inisialisasi_tde_berdaulat(mesin: &mut WidyaBase) -> Hasil<Boolean, Teks> {
        cetak("==================================================================");
        cetak(">>> [TDE] Mengonfigurasi Enkripsi Data Diam AES-256-GCM...");

        // 1. Daftarkan Kunci Master dari HSM / KMS Berdaulat
        biar mut tde = mesin.ambil_mesin_tde();
        tde.tetapkan_kunci_master("WIDYA-KMS-MASTER-KEY-INDONESIA-2026")?;
        cetak(">>> [TDE] Kunci Master berhasil diverifikasi dan dimuat di RAM aman.");

        // 2. Buat Tabel Data Kependudukan Terenkripsi Penuh (TDE Aktif)
        biar sql_tabel = "
            BUAT TABEL data_kependudukan_rahasia (
                nik TEKS KUNCI_UTAMA,
                nama_lengkap TEKS TIDAK_KOSONG,
                sidik_jari_vektor BLOB,
                catatan_intelijen TEKS
            ) DENGAN (
                ENKRIPSI_TDE = 'AKTIF',
                ALGORITMA = 'AES_256_GCM'
            );
        ";
        mesin.eksekusi_sql(sql_tabel)?;
        cetak(">>> [TDE] Tabel 'data_kependudukan_rahasia' terikat dengan Kunci DEK unik.");

        // 3. Sisipkan Data Sensitif (Dienkripsi Transparan di Balik Layar)
        biar sql_sisip = "
            SISIP_KE data_kependudukan_rahasia (nik, nama_lengkap, catatan_intelijen)
            NILAI ('3201998877660001', 'Raden Wijaya', 'Aset strategis kedaulatan informasi nasional.');
        ";
        mesin.eksekusi_sql(sql_sisip)?;
        cetak(">>> [TDE] Data berhasil disisipkan dan dienkripsi sebelum menyentuh SSD.");

        // 4. Kueri Transparan: Data Didekripsi Otomatis untuk Pengguna Sah
        biar sql_baca = "PILIH nik, nama_lengkap, catatan_intelijen DARI data_kependudukan_rahasia;";
        biar hasil = mesin.kueri_sql(sql_baca)?;
        untuk baris di hasil.baris_data() {
            cetak("  ★ Data Terdekripsi Sah: " + baris.ambil_teks("nama_lengkap") + 
                  " [NIK: " + baris.ambil_teks("nik") + "]");
        }

        // 5. Buktikan Keamanan: Periksa Isi Berkas Fisik Disk Secara Mentah
        biar byte_mentah_disk = mesin.baca_blok_fisik_mentah("data_kependudukan_rahasia", 0)?;
        cetak(">>> [VERIFIKASI FISIK DISK]:");
        cetak("  - Apakah teks 'Raden Wijaya' tampak di berkas disk? " + 
              teks(byte_mentah_disk.berisi_string("Raden Wijaya"))); // Pasti SALAH
        cetak("  - Status Fisik: 100% Acak / Chipertext AEAD Tanpa Kebocoran.");

        cetak("==================================================================");
        kembalikan Ok(benar);
    }
}
```

### 33.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan algoritma kriptografi pada modul TDE WidyaBase:

| Algoritma Kriptografi | Standar Panjang Kunci | Kategori AEAD | Akselerasi Hardware CPU | Keamanan Pascakuantum |
| :--- | :--- | :--- | :--- | :--- |
| **AES-256-GCM (Baku)** | 256 Bit | **Ya (Otentikasi 128-bit)** | Sangat Cepat (Instruksi Intel AES-NI / ARM Crypto)| Sangat Kuat (Symmetric 256-bit) |
| **ChaCha20-Poly1305** | 256 Bit | **Ya (Otentikasi 128-bit)** | Super Cepat di CPU Tanpa AES-NI (Perangkat IoT)| Sangat Kuat |
| **AES-128-CBC (Lama)**| 128 Bit | Tidak (Rawan Padding Oracle)| Sedang | Kurang Direkomendasikan |
| **SM4 (Standar Regional)**| 128 Bit | Ya (Mode GCM) | Tergantung Prosesor Khusus | Standar Kepatuhan Khusus |

### 33.4 Latihan Soal Esai

1. Terangkan perbedaan mendasar antara enkripsi pada tingkat jaringan (*In-Transit Encryption* seperti TLS/SSL) dan enkripsi pada tingkat piringan disk (*At-Rest Encryption* / TDE)! Mengapa sistem perbankan mewajibkan keduanya?
2. Bagaimana konsep *Authenticated Encryption with Associated Data* (AEAD) melindungi sistem basis data dari serangan perusakan fisik berkas (*bit-flipping attack*)?
3. Uraikan teknik pemusnahan data *Crypto-Shredding*! Mengapa teknik ini jauh lebih aman, instan, dan ramah lingkungan dibandingkan memformat fisik piringan harddisk berkali-kali?

### 33.5 Soal Pilihan Ganda

1. Singkatan dari TDE dalam sistem keamanan basis data modern adalah...
   A. Transparent Data Encryption
   B. Total Disk Eraser
   C. Transmission Data Engine
   D. Terminal Desktop Emulator

2. Algoritma enkripsi simetris standar yang paling umum digunakan pada TDE WidyaBase karena akselerasi perangkat keras CPU adalah...
   A. AES-256-GCM
   B. DES 56-bit
   C. ROT13
   D. MD5

3. Apa keuntungan enkripsi data bersifat "transparan" (Transparent Data Encryption)?
   A. Aplikasi klien dapat menjalankan kueri SQL biasa tanpa perlu memodifikasi kode program atau melakukan enkripsi manual
   B. Data dapat dibaca tanpa kata sandi oleh siapa saja
   C. File basis data otomatis tidak terlihat di Windows Explorer
   D. Layar monitor menjadi tembus pandang

4. Sifat kriptografi AEAD yang memastikan data tidak dapat diubah atau dirusak oleh pihak luar dinamakan...
   A. Integritas Terotentikasi (*Authenticated Integrity*)
   B. Anomali
   C. Rekursi
   D. Sharding

5. Dalam hierarki kunci TDE, kunci yang bertugas mengenkripsi berkas data tabel secara langsung disebut...
   A. Data Encryption Key (DEK)
   B. Master Key (KEK)
   C. Password Windows
   D. Token Wifi

6. Nilai Initialization Vector (IV) yang unik pada setiap penulisan halaman diturunkan dari gabungan...
   A. Nomor Halaman (PageID) dan Log Sequence Number (LSN)
   B. Jam istirahat peladen
   C. Jumlah karyawan bank
   D. Angka nol mutlak

7. Teknik pemusnahan seluruh data tabel secara instan dan permanen dengan cara membuang kunci enkripsinya dinamakan...
   A. Crypto-Shredding
   B. Defragmentasi
   C. Format C:
   D. Overclocking

8. Instruksi prosesor modern yang mengizinkan proses enkripsi dan dekripsi AES berjalan dengan kecepatan gigabyte per detik tanpa membebani CPU adalah...
   A. Intel AES-NI / ARMv8 Crypto Extension
   B. SIMD MMX
   C. Direct3D
   D. Ray Tracing

9. Apa yang terjadi jika sebuah halaman terenkripsi dimodifikasi secara ilegal oleh peretas di luar sistem basis data saat berkas dibaca kembali ke RAM?
   A. Verifikasi Tag Otentikasi GCM gagal seketika dan mesin menolak membaca halaman yang terkontaminasi (*Tamper Detection*)
   B. Data diterima apa adanya
   C. Peladen otomatis meledak
   D. Sistem operasi diinstal ulang

10. Mengapa pemanfaatan TDE berdaulat sangat krusial bagi pusat data nasional yang menampung data kependudukan dan perbankan?
    A. Mencegah kebocoran rahasia negara apabila perangkat keras fisik SSD dicuri atau disita oleh pihak luar
    B. Supaya tidak perlu membayar teknisi data center
    C. Menghilangkan kebutuhan kabel internet
    D. Menjadikan server tidak membutuhkan pendingin udara
