## Lampiran C: Glosarium Standar Formal Algoritma Kriptografi Pasca-Kuantum (PQC) & Roadmap Sistem

Lampiran ini memuat parameter spesifikasi matematis baku algoritma **Kriptografi Pasca-Kuantum (PQC)** yang diadopsi oleh Widya-Lang sesuai dokumen resmi **NIST FIPS 203 & FIPS 204**, serta Peta Jalan (*Roadmap*) Kedaulatan Sistem.

---

### C.1 Spesifikasi Formal ML-KEM (NIST FIPS 203 / CRYSTALS-Kyber)

| Parameter Kriptografis | ML-KEM-512 (Kategori I) | ML-KEM-768 (Kategori III) | ML-KEM-1024 (Kategori V) |
| :--- | :---: | :---: | :---: |
| **Setara Keamanan Klasik** | AES-128 | AES-192 | AES-256 |
| **Dimensi Matriks Kisi ($k$)** | $k = 2$ | $k = 3$ | $k = 4$ |
| **Modulus Polinomial ($q$)** | 3.329 | 3.329 | 3.329 |
| **Derajat Polinomial ($n$)** | 256 | 256 | 256 |
| **Ukuran Kunci Publik ($pk$)** | 800 byte | **1.184 byte** | 1.568 byte |
| **Ukuran Kunci Privat ($sk$)** | 1.632 byte | **2.400 byte** | 3.168 byte |
| **Ukuran Ciphertext ($ct$)** | 768 byte | **1.088 byte** | 1.568 byte |
| **Ukuran Kunci Bersama ($ss$)** | 32 byte (256-bit) | **32 byte (256-bit)** | 32 byte (256-bit) |

---

### C.2 Spesifikasi Formal ML-DSA (NIST FIPS 204 / CRYSTALS-Dilithium)

| Parameter Kriptografis | ML-DSA-44 (Kategori II) | ML-DSA-65 (Kategori III) | ML-DSA-87 (Kategori V) |
| :--- | :---: | :---: | :---: |
| **Dimensi Matriks ($k \times l$)** | $4 \times 4$ | **$6 \times 5$** | $8 \times 7$ |
| **Modulus Polinomial ($q$)** | 8.380.417 ($2^{23} - 2^{13} + 1$) | **8.380.417** | 8.380.417 |
| **Ukuran Kunci Publik ($pk$)** | 1.312 byte | **1.952 byte** | 2.592 byte |
| **Ukuran Kunci Privat ($sk$)** | 2.560 byte | **4.032 byte** | 4.896 byte |
| **Ukuran Tanda Tangan ($sig$)** | 2.420 byte | **3.309 byte** | 4.627 byte |

---

### C.3 Glosarium Istilah Rekayasa Sistem Berdaulat

- **Direct Syscall**: Mekanisme pengeksekusian instruksi CPU assembly murni (`syscall` atau `svc #0`) untuk meminta layanan kernel tanpa melalui pustaka perantara glibc/musl.
- **Zero-Copy Architecture**: Teknik perancangan perangkat lunak di mana data biner mengalir antar subsistem atau soket jaringan tanpa melibatkan operasi duplikasi memori di heap (*no memory copying*).
- **Borrow Checker**: Komponen penganalisis statis pada kompilator yang menegakkan aturan kepemilikan dan peminjaman memori secara matematis tanpa bantuan garbage collector.
- **Monomorphization**: Strategi kompilasi fungsi generik yang membangkitkan salinan kode mesin konkret khusus per tipe data, menghasilkan eksekusi setara kode biasa tanpa vtable overhead.
- **Fast IPC**: Komunikasi antar-proses berkecepatan tinggi pada arsitektur microkernel yang memindahkan pesan melalui register CPU dan pemetaan memori virtual bersama.
