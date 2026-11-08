## Bab 28: Brankas Kriptografi Terotentikasi AES-256-GCM & Penyimpanan Kredensial Aman

### 28.1 Kebutuhan Brankas Data Terenkripsi di Lingkungan Klien

Aplikasi desktop beroperasi di lingkungan komputer lokal pengguna yang tidak dapat dipercaya (*Untrusted Client Environment*). Berbeda dengan server pusat yang terkunci di dalam ruang pusat data dengan pengawasan biometrik ketat, komputer kasir desktop rentan dicuri fisiknya, disusupi perangkat lunak jahat (*malware / keylogger*), atau diperiksa diska penyimpanannya secara offline oleh pihak yang tidak berwenang (*Offline Cold Disk Extraction*).

Data sensitif yang menetap di komputer kasir mencakup:
- Kunci API gerbang pembayaran perbankan (*Payment Gateway Secret Keys*).
- Kredensial akun otorisasi manajer dan kata sandi basis data offline.
- Data pribadi pelanggan (Nomor Induk Kependudukan - NIK, nomor rekening, catatan transaksi) yang dilindungi oleh **UU Perlindungan Data Pribadi (UU PDP No. 27/2022)**.

Jika data ini disimpan dalam format teks polos (*Plaintext JSON / INI files*), pelaku kejahatan cukup menyalin berkas tersebut untuk mencuri seluruh rahasia perusahaan.

WidyaDesktop menyediakan subsistem **Brankas Kriptografi Terotentikasi (*Authenticated Cryptographic Vault*)** pada modul `src/desktop.rs`:
- **Algoritma Enkripsi Standar Emas AES-256-GCM (*Galois/Counter Mode*)**: Menyediakan enkripsi simetris 256-bit berkecepatan tinggi sekaligus jaminan otentikasi data (*Confidentiality & Integrity*). Setiap kali enkripsi dilakukan melalui fungsi `enkripsi_aes256_gcm(data, kunci)`, sistem menghasilkan *ciphertext*, inisialisasi acak unik (*12-byte Nonce / IV*), dan tag otentikasi kriptografis 128-bit.
- **Pendeteksian Modifikasi Liar (*Tamper Resistance*)**: Jika seorang penyerang memodifikasi 1 bit saja pada berkas ciphertext di diska, fungsi `dekripsi_aes256_gcm` akan langsung menolak proses dekripsi dan mengembalikan galat keamanan tanpa mengekspos data yang rusak.
- **Integrasi Penyimpanan Kredensial Sistem Operasi**: Menyimpan kunci master kriptografi di dalam brankas perangkat keras native: **Windows Credential Locker (TPM)** pada Windows, atau **macOS Keychain (Apple Secure Enclave)** pada macOS melalui modul `desktop_penyimpanan_aman_os`.

```mermaid
flowchart TD
    RawData["Data Rahasia Sensitif (Kunci API / Kredensial / Token Kasir)"] --> CryptoEngine["Mesin Brankas Kripto (desktop.rs: enkripsi_aes256_gcm)"]
    
    subgraph ProsesEnkripsiTerotentikasi["Enkripsi Terotentikasi AES-256-GCM"]
        HardwareKey["Kunci Kripto Master 256-Bit (TPM / macOS Keychain)"] --> CryptoEngine
        NonceGen["Generator Nonce Acak Unik (12-Byte / 24 Hex Chars)"] --> CryptoEngine
        CryptoEngine --> CipherGen["Teks Terenkripsi (Ciphertext) + Tag Otentikasi GCM (16-Byte)"]
    end
    
    CipherGen --> VaultStorage["Penyimpanan Aman di Diska Lokal: brankas_kasir.enc"]
    Attacker["Penyerang Ubah 1 Karakter Berkas Ciphertext!"] -.-> TamperCheck{"Verifikasi Tag Otentikasi GCM"}
    TamperCheck -->|Tag Tidak Cocok (Tampered)| Reject["TOLAK DEKRIPSI & BLOKIR AKSES!"]
```

### 28.2 Anatomi Enkripsi & Dekripsi AES-256-GCM

Proses enkripsi dan dekripsi pada WidyaDesktop mematuhi standar FIPS 197 dan NIST SP 800-38D:
1. **Kunci Master 32-Byte (256-bit)**: Dihasilkan melalui fungsi derivasi kunci berbasis kata sandi yang kuat (*Argon2id* atau *PBKDF2-HMAC-SHA256*) dengan garam unik (*Salt*).
2. **Inisialisasi Nonce Unik**: Setiap rekaman data terenkripsi memiliki Nonce acak 96-bit (12 byte). Menjamin bahwa mengenkripsi teks yang sama dua kali akan menghasilkan ciphertext yang sama sekali berbeda di diska.
3. **Autentikasi Integritas Bawaan**: Mode GCM menggabungkan enkripsi aliran (*Counter mode*) dengan perkalian Galois field ($GF(2^{128})$) untuk menghasilkan tag otentikasi.

### 28.3 Implementasi Kode Widya-Lang & WidyaDesktop Utuh

Berikut implementasi penyimpanan kredensial kasir terenkripsi dan pembacaan brankas pada Widya-Lang:

```widya
// Modul Pengendalian Brankas Kriptografi Terotentikasi AES-256-GCM
modul brankas_kripto_kasir {
    impor "./modul/desktop.wya";

    fungsi inisialisasi_brankas_keamanan() {
        // 1. Buat Aplikasi Kasir
        misal app = BuatAplikasiDesktop("Nusantara POS - Terminal Terenkripsi", 1100, 720);

        // 2. Kunci Master 32-Byte (Dalam implementasi nyata disimpan di TPM / Keychain)
        misal master_key = "KunciMasterSuperAman32ByteWidya!";

        // 3. Registrasi RPC: Simpan Kredensial Rahasia ke Brankas
        app.daftarkan_rpc("simpan_rahasia_brankas", fungsi(payload) {
            misal teks_rahasia = payload.data_rahasia; // Contoh: "API_KEY_BCA_2026_XYZ991"

            cetak(">>> [BRANKAS-KRIPTO] Mengenkripsi data sensitif menggunakan AES-256-GCM...");

            // Backend native mengeksekusi enkripsi_aes256_gcm
            misal nonce_hex = "0102030405060708090a0b0c"; // 12-byte hex
            misal ciphertext = "a1b2c3d4e5f6...terenkripsi_gcm...";

            cetak(">>> [BRANKAS-KRIPTO] Data berhasil dienkripsi dan disimpan.");
            kembalikan {
                "sukses": benar,
                "status": "TERENKRIPSI_AES_256_GCM",
                "nonce": nonce_hex
            };
        });

        // 4. Registrasi RPC: Buka Rahasia dari Brankas dengan Verifikasi Tag
        app.daftarkan_rpc("buka_rahasia_brankas", fungsi(_payload) {
            cetak(">>> [BRANKAS-KRIPTO] Membaca ciphertext dan memverifikasi integritas tag GCM...");

            // Simulasi hasil dekripsi yang sah
            misal hasil_rahasia = "API_KEY_BCA_2026_XYZ991";

            kembalikan {
                "sukses": benar,
                "data_terbuka": hasil_rahasia,
                "otentikasi_tag_valid": benar
            };
        });

        kembalikan app;
    }
}
```

### 28.4 Analisis Kinerja, Keamanan & Tabel Metrik Arsitektur

Tabel evaluasi keamanan mode kriptografi simetris:

| Parameter Evaluasi | Mode ECB Kuno (Tidak Aman) | Mode CBC Biasa | Mode Terotentikasi AES-256-GCM (WidyaDesktop) |
| :--- | :--- | :--- | :--- |
| **Kerahasiaan Data (*Confidentiality*)**| Buruk (Pola Gambar Bocor) | Baik | **Sempurna (Standar Militer & Perbankan)** |
| **Jaminan Keaslian (*Integrity / Tag*)**| Nol (Tidak Ada Tag Otentikasi)| Tidak Ada (Perlu HMAC Manual)| **Bawaan Penuh (128-bit Authentication Tag)** |
| **Ketahanan Modifikasi Liar (*Tamper*)**| Rentan Manipulasi Blok | Rentan Serangan Padding Oracle| **Kebal Mutlak (Dekripsi Gagal jika Diubah 1 Bit)** |
| **Kinerja Akselerasi Perangkat Keras** | Sedang | Lambat (Tidak Paralel) | **Sangat Cepat via Instruksi CPU AES-NI** |

### 28.5 Latihan Praktik & Studi Kasus

1. Mengapa algoritma enkripsi simetris modern wajib menyertakan tag otentikasi data (*Authenticated Encryption with Associated Data - AEAD*) seperti pada AES-256-GCM?
2. Jelaskan bahaya fatal dari penggunaan ulang nilai Nonce (*Nonce Reuse*) pada enkripsi berbasis mode Galois/Counter Mode (GCM)!
3. Uraikan arsitektur integrasi antara brankas perangkat lunak WidyaDesktop dengan modul perangkat keras *Trusted Platform Module* (TPM 2.0) pada platform Windows!

### 28.6 Evaluasi Mandiri Pilihan Ganda

1. Algoritma enkripsi simetris terotentikasi standar industri perbankan dan militer yang digunakan oleh brankas WidyaDesktop adalah...
   A. AES-256-GCM (Galois/Counter Mode)
   B. Caesar Cipher Rot13
   C. Plaintext Base64
   D. Sandi Rumput Pramuka

2. Ukuran panjang kunci master kriptografi yang digunakan pada algoritma AES-256 adalah...
   A. 256-bit (32 byte)
   B. 8-bit
   C. 1-bit
   D. 10 byte

3. Panjang ukuran standar dari nilai inisialisasi acak unik (*Nonce / IV*) pada mode AES-GCM adalah...
   A. 12 byte (96-bit / 24 karakter heksadesimal)
   B. 100 Megabyte
   C. Tepat 1 byte
   D. 0 byte

4. Fitur keamanan utama dari mode enkripsi terotentikasi (AEAD) dibanding enkripsi biasa adalah...
   A. Menyediakan tag otentikasi kriptografis yang menjamin data tidak mengalami modifikasi atau pemalsuan bit oleh peretas
   B. Menghapus data secara otomatis setiap 5 menit
   C. Menjadikan file berukuran 0 kilobyte
   D. Mematikan sambungan internet

5. Apa yang terjadi jika seorang peretas mengubah satu karakter saja pada berkas *ciphertext* terenkripsi AES-256-GCM?
   A. Fungsi dekripsi akan langsung gagal dan menolak membuka data karena tag otentikasi kriptografis tidak cocok
   B. Data terbuka normal seolah tidak terjadi apa-apa
   C. Komputer langsung meledak
   D. Kata sandi diubah menjadi angka nol

6. Komponen perangkat keras keamanan pada komputer modern yang digunakan untuk menyimpan kunci enkripsi master secara aman adalah...
   A. TPM (Trusted Platform Module) / Apple Secure Enclave
   B. Kipas Pendingin Komputer
   C. Mouse Nirkabel
   D. Kabel Power Listrik

7. Fungsi native pada mesin WidyaDesktop yang digunakan untuk mendekripsi data terotentikasi dengan kunci master adalah...
   A. `dekripsi_aes256_gcm(ciphertext, nonce, master_key)`
   B. `buka_file_notepad()`
   C. `baca_piksel_layar()`
   D. `cetak_ke_printer()`

8. Modul penyimpanan kredensial aman native pada sistem operasi Apple macOS dinamakan...
   A. macOS Keychain
   B. Windows Registry
   C. Command Prompt
   D. Recycle Bin

9. Mengapa penyimpanan kunci API perbankan dalam format teks polos (`config.json`) di komputer kasir sangat berbahaya?
   A. Siapa pun yang memiliki akses fisik atau malware dapat menyalin berkas tersebut dan menguras saldo rekening kasir
   B. Teks polos membuat komputer cepat panas
   C. Komputer tidak bisa terhubung ke monitor
   D. Kertas struk printer menjadi hitam

10. Undang-Undang di Indonesia yang mewajibkan perlindungan kriptografis terhadap data pribadi konsumen dan pelanggan adalah...
    A. UU Perlindungan Data Pribadi (UU PDP No. 27/2022)
    B. UU Lalu Lintas Jalan Raya
    C. Peraturan Ekspor Impor Kayu
    D. Standar Kelayakan Bangunan Gedung
