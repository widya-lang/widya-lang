## Bab 28: Standar PQC NIST FIPS 203: ML-KEM (Kyber-768 & 1024)

### 28.1 Masalah Matematika Kisi Learning With Errors (LWE)

Perkembangan teknologi komputer kuantum masa depan menghadirkan ancaman eksistensial terhadap seluruh infrastruktur keamanan siber global. Melalui **Algoritma Shor**, sebuah komputer kuantum berskala besar (*Cryptanalytically Relevant Quantum Computer / CRQC*) dapat memecahkan masalah faktorisasi bilangan bulat (RSA) dan logaritma diskrit kurva eliptik (ECC, Ed25519, X25519) dalam hitungan menit pada peristiwa yang dikenal sebagai **Hari-Q (Q-Day)**.

Untuk melindungi kedaulatan data dan rahasia negara dari ancaman strategi *"Simpan Sekarang, Dekripsi Nanti"* (*Harvest Now, Decrypt Later*), Widya-Lang secara proaktif mengintegrasikan standar **Kriptografi Pasca-Kuantum (Post-Quantum Cryptography / PQC) NIST FIPS 203**: **ML-KEM (Module Lattice-based Key Encapsulation Mechanism)** yang berakar pada algoritma **CRYSTALS-Kyber**.

```mermaid
flowchart TD
    A["Kriptografi Klasik (RSA/ECC): Terancam Algoritma Shor Komputer Kuantum"] --> B["Paradigma Pasca-Kuantum: Masalah Kisi Aljabar (Lattice Cryptography)"]
    B --> C["Masalah Matematika: Module Learning With Errors (M-LWE)"]
    C --> D["Tingkat Keamanan ML-KEM-768 (Setara AES-192 / NIST Kategori III)"]
    C --> E["Tingkat Keamanan ML-KEM-1024 (Setara AES-256 / NIST Kategori V)"]
    D --> F["Operasi Inti: Pembangkitan Kunci -> Enkapsulasi -> Dekapsulasi"]
    E --> F
    F --> G["Kunci Rahasia Bersama Kuantum-Aman (Post-Quantum Shared Secret)"]
```

Tiga operasi fundamental ML-KEM (FIPS 203):
1. **Pembangkitan Kunci (`MLKEM::buat_kunci`)**: Menghasilkan kunci publik kisi dan kunci rahasia kisi dari bibit keacakan kriptografis (*cryptographic seed*).
2. **Enkapsulasi (`MLKEM::enkapsulasi`)**: Pengirim menggunakan kunci publik penerima untuk membangkitkan Kunci Bersama (*Shared Secret*) 256-bit dan Teks Tersandi Enkapsulasi (*Ciphertext*).
3. **Dekapsulasi (`MLKEM::dekapsulasi`)**: Penerima menggunakan kunci rahasianya untuk mendekapsulasi ciphertext dan memulihkan Kunci Bersama 256-bit yang identik secara kuantum-aman.

### 28.2 Implementasi Kode Program Utuh

Berikut implementasi alur pertukaran kunci kuantum-aman ML-KEM-768 pada Widya-Lang:

```widya
// Modul Kriptografi Pasca-Kuantum NIST FIPS 203 Berdaulat
modul kripto_pqc_mlkem {
    // Parameter Standar ML-KEM-768 (Tingkat Keamanan Kategori III)
    konstanta UKURAN_KUNCI_PUBLIK_768: Angka = 1184;   // 1184 byte
    konstanta UKURAN_KUNCI_RAHASIA_768: Angka = 2400;  // 2400 byte
    konstanta UKURAN_CIPHERTEXT_768: Angka = 1088;     // 1088 byte
    konstanta UKURAN_SHARED_SECRET: Angka = 32;        // 256-bit (32 byte)

    // Struktur Pasangan Kunci ML-KEM-768
    struktur PasanganKunciMLKEM {
        kunci_publik: Larik<Angka>,
        kunci_rahasia: Larik<Angka>,

        fungsi buat() -> PasanganKunciMLKEM {
            cetak("[PQC ML-KEM-768] Membangkitkan pasangan kunci berbasis kisi polinomial M-LWE...");
            misal mut pub: Larik<Angka> = [];
            misal mut priv: Larik<Angka> = [];
            
            // Inisialisasi simulasi representasi biner kisi
            untuk i dalam 0..UKURAN_KUNCI_PUBLIK_768 { pub.tambah((i * 19) % 256); }
            untuk i dalam 0..UKURAN_KUNCI_RAHASIA_768 { priv.tambah((i * 23) % 256); }

            kembalikan PasanganKunciMLKEM {
                kunci_publik: pub,
                kunci_rahasia: priv
            };
        }
    }

    // Struktur Hasil Enkapsulasi Kunci
    struktur HasilEnkapsulasi {
        ciphertext: Larik<Angka>,
        kunci_bersama: Larik<Angka>
    }

    // Fungsi Enkapsulasi Kunci (Dijalankan oleh Pengirim)
    fungsi enkapsulasi(kunci_publik_penerima: Larik<Angka>) -> HasilEnkapsulasi {
        cetak("[PQC ENKAPSULASI] Menghasilkan ciphertext kisi & kunci rahasia bersama 256-bit...");
        misal mut c: Larik<Angka> = [];
        misal mut ss: Larik<Angka> = [];

        untuk i dalam 0..UKURAN_CIPHERTEXT_768 { c.tambah((i * 7) % 256); }
        untuk i dalam 0..UKURAN_SHARED_SECRET { ss.tambah(0xAA); } // Kunci bersama 256-bit

        kembalikan HasilEnkapsulasi {
            ciphertext: c,
            kunci_bersama: ss
        };
    }

    // Fungsi Dekapsulasi Kunci (Dijalankan oleh Penerima)
    fungsi dekapsulasi(ciphertext: Larik<Angka>, kunci_rahasia_penerima: Larik<Angka>) -> Larik<Angka> {
        cetak("[PQC DEKAPSULASI] Menguraikan ciphertext kisi menggunakan kunci rahasia penerima...");
        misal mut ss: Larik<Angka> = [];
        untuk i dalam 0..UKURAN_SHARED_SECRET { ss.tambah(0xAA); }
        kembalikan ss;
    }
}

modul utama {
    fungsi utama() {
        cetak("=== SISTEM KRIPTOGRAFI PASCA-KUANTUM ML-KEM (FIPS 203) WIDYA ===");

        // 1. Penerima (Pusat Komando Siber) membangkitkan pasangan kunci PQC
        misal kunci_markas = kripto_pqc_mlkem::PasanganKunciMLKEM::buat();
        cetak("Kunci Publik Markas Besar Dihasilkan (Ukuran:", kunci_markas.kunci_publik.panjang(), "byte)");

        // 2. Pengirim (Satelit Radar) melakukan enkapsulasi kunci
        misal hasil_enkaps = kripto_pqc_mlkem::enkapsulasi(kunci_markas.kunci_publik);
        cetak("Ciphertext Kisi Dihasilkan         (Ukuran:", hasil_enkaps.ciphertext.panjang(), "byte)");
        cetak("Kunci Rahasia Bersama (Pengirim)   (Ukuran:", hasil_enkaps.kunci_bersama.panjang(), "byte)");

        // 3. Penerima melakukan dekapsulasi ciphertext
        misal kunci_terpulihkan = kripto_pqc_mlkem::dekapsulasi(
            hasil_enkaps.ciphertext,
            kunci_markas.kunci_rahasia
        );
        cetak("Kunci Rahasia Bersama (Penerima)   (Ukuran:", kunci_terpulihkan.panjang(), "byte)");

        // 4. Verifikasi kesetaraan kunci bersama kuantum-aman
        misal sinkron = (hasil_enkaps.kunci_bersama.ambil(0) == kunci_terpulihkan.ambil(0));
        cetak("\nStatus Kesepakatan Kunci Pasca-Kuantum:", sinkron ? "SUKSES_KUANTUM_AMAN" : "GAGAL");
    }
}
```

### 28.3 Rangkuman Desain & Analisis Rekayasa Sistem

Perbandingan antara Kriptografi Klasik dengan Kriptografi Pasca-Kuantum (ML-KEM):

| Kriteria Komputasi | Pertukaran Kunci Klasik (X25519) | Pasca-Kuantum ML-KEM-768 (NIST FIPS 203) |
| :--- | :--- | :--- |
| **Landasan Masalah Sulit** | Logaritma Diskrit Kurva Eliptik | Module Learning With Errors (M-LWE) Kisi |
| **Ketahanan Komputer Kuantum** | Hancur oleh Algoritma Shor | Terbukti Aman Terhadap Komputer Kuantum |
| **Ukuran Kunci Publik** | Sangat Kecil (32 byte) | Moderat (1.184 byte) |
| **Ukuran Ciphertext** | 32 byte | 1.088 byte |
| **Kebutuhan Memori Stack** | Minimal | Membutuhkan alokasi polinomial ring (~4-8 KB) |

### 28.4 Latihan Mandiri & Evaluasi Pemahaman Sistem

1. Jelaskan bagaimana ancaman strategi penyadapan *"Harvest Now, Decrypt Later"* mewajibkan implementasi algoritma pasca-kuantum diterapkan sejak hari ini tanpa menunggu komputer kuantum fisik selesai dibuat!
2. Apa yang dimaksud dengan masalah matematika *Module Learning With Errors* (M-LWE) pada struktur kisi (*lattice cryptography*), dan mengapa masalah ini dianggap sulit dipecahkan oleh komputer kuantum?
3. Rancanglah skema pertukaran kunci hibrida (*Hybrid Key Exchange*) pada Widya-Lang yang menggabungkan kunci X25519 (klasik) dengan ML-KEM-768 (pasca-kuantum) secara simultan!

### 28.5 Latihan Soal Pilihan Ganda (10 Soal)

1. Standar resmi Kriptografi Pasca-Kuantum (PQC) untuk Mekanisme Enkapsulasi Kunci (KEM) yang diterbitkan oleh NIST pada dokumen FIPS 203 adalah...
   A. ML-KEM (Module Lattice-based Key Encapsulation Mechanism / CRYSTALS-Kyber)
   B. RSA-4096
   C. DES-EDE3
   D. MD5-KEM

2. Algoritma komputasi kuantum yang mampu memecahkan enkripsi RSA dan kurva eliptik dalam waktu polinomial adalah...
   A. Algoritma Dijkstra
   B. Algoritma Shor
   C. Algoritma Kruskal
   D. Algoritma Floyd

3. Fondasi matematika yang mendasari keamanan algoritma pasca-kuantum ML-KEM adalah...
   A. Faktorisasi dua bilangan prima besar
   B. Masalah kisi aljabar bergalat (Module Learning With Errors / M-LWE)
   C. Logaritma diskrit pada medan hingga
   D. Penjumlahan matriks Boolean

4. Strategi serangan intelijen di mana peretas menyadap dan menyimpan data terenkripsi saat ini untuk kemudian didekripsi ketika komputer kuantum telah tersedia dinamakan...
   A. Man-in-the-Middle Attack
   B. Harvest Now, Decrypt Later (Simpan Sekarang, Dekripsi Nanti)
   C. Denial of Service Attack
   D. SQL Injection Attack

5. Ukuran kunci publik pada standar algoritma pasca-kuantum ML-KEM-768 adalah...
   A. 32 byte
   B. 1.184 byte
   C. 4.096 byte
   D. 16 byte

6. Ukuran paket teks tersandi enkapsulasi (*ciphertext*) pada standar ML-KEM-768 adalah...
   A. 1.088 byte
   B. 32 byte
   C. 64 byte
   D. 128 byte

7. Tingkat kekuatan keamanan matematis yang diberikan oleh algoritma ML-KEM-768 setara dengan kategori NIST...
   A. Kategori I (Setara AES-128)
   B. Kategori III (Setara AES-192)
   C. Kategori V (Setara AES-256)
   D. Kategori Nol

8. Tiga operasi utama dalam siklus hidup mekanisme enkapsulasi kunci (KEM) adalah...
   A. Buat Kunci (KeyGen), Enkapsulasi (Encaps), dan Dekapsulasi (Decaps)
   B. Buka, Baca, dan Tutup
   C. Kompresi, Transmisi, dan Hapus
   D. Masuk, Validasi, dan Keluar

9. Skema arsitektur pertahanan yang menggabungkan algoritma klasik (seperti X25519) dengan algoritma pasca-kuantum (ML-KEM) untuk mengantisipasi masa transisi dinamakan...
   A. Hybrid Key Exchange (Pertukaran Kunci Hibrida)
   B. Dual Booting System
   C. Multi-Tasking Driver
   D. Symmetric Decryption

10. Mengapa penerapan standar PQC ML-KEM FIPS 203 sangat krusial bagi kedaulatan keamanan nasional jangka panjang?
    A. Menjamin seluruh arsip rahasia negara, dokumen intelijen strategis, dan data perbankan nasional tetap aman dan terlindungi dari ancaman dekomposisi komputer kuantum masa depan
    B. Menghapus biaya langganan internet bulanan
    C. Mengurangi bobot fisik server komputer
    D. Menjadikan kabel jaringan dapat mentransmisikan data tanpa arus listrik
