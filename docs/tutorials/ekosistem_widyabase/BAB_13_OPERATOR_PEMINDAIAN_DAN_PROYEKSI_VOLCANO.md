## Bab 13: Operator Pemindaian, Pemfilteran & Proyeksi Fisik

### 13.1 Model Eksekusi Volcano: Iterator Buka-Berikutnya-Tutup

Dalam rekayasa mesin basis data modern, salah satu pola arsitektur paling legendaris dan elegan untuk mengeksekusi rencana fisik kueri adalah **Model Iterator Volcano** (diperkenalkan oleh Goetz Graefe). Model ini memungkinkan pemrosesan kueri data berukuran bergiga-giga byte dengan jejak memori RAM yang sangat kecil dan konstan ($O(1)$).

Setiap operator fisik dalam pohon eksekusi WidyaBase mengimplementasikan antarmuka iterator baku dengan tiga metode utama:
1. `buka()` (*open*): Menginisialisasi status internal, menyiapkan sumber daya, atau memposisikan kursor awal.
2. `berikutnya()` (*next*): Mengembalikan satu baris data (*tuple*) berikutnya atau `TidakAda` jika aliran data telah habis.
3. `tutup()` (*close*): Membersihkan status internal, menutup berkas, dan membebaskan memori kerja.

```mermaid
flowchart TD
    A["Klien / Konsumen Kueri"] -->|1. panggil berikutnya()| B["Operator Fisik Proyeksi (PhysicalProject)"]
    B -->|2. panggil berikutnya()| C["Operator Fisik Filter (PhysicalFilter)"]
    C -->|3. panggil berikutnya()| D["Operator Fisik Pemindaian (PhysicalSeqScan / IndexScan)"]
    D -->|4. kembalikan Baris Tupel| C
    C -->|5. loloskan jika Predikat Benar| B
    B -->|6. transformasi kolom luaran| A
```

Prinsip kerja aliran data berbasis dorongan-permintaan (*Demand-Driven / Pull-Based Pipeline*):
- **Evaluasi Malas (*Lazy Streaming*)**: Data diproses baris demi baris hanya saat operator induk di atasnya meminta `berikutnya()`. Tidak ada tabel sementara raksasa yang harus ditampung di RAM untuk operasi filter dan proyeksi dasar.
- **Waktu ke Baris Pertama Cepat (*Low Time-to-First-Row*)**: Klien antarmuka pengguna dapat langsung menerima dan menampilkan 10 baris pertama di layar dalam hitungan mikrodetik, tanpa menunggu sistem menyelesaikan pembacaan jutaan baris berikutnya.
- **Penyatuan Loop SIMD & Vektorisasi**: Untuk kueri analitis berat, WidyaBase juga mendukung varian *Batch Volcano*, di mana `berikutnya()` mengembalikan satu bundel berisi 1.024 baris data sekaligus (*Vector of Tuples*) untuk memanfaatkan instruksi vektor CPU modern (AVX-512 / ARM NEON).

### 13.2 Implementasi Kode Program Utuh

Berikut implementasi antarmuka Iterator Volcano dan rantai operator fisik pada WidyaBase menggunakan Widya-Lang:

```widya
// Modul Model Eksekusi Volcano Iterator WidyaBase
modul eksekutor_volcano {
    impor ekosistem::widyabase::{
        WidyaBase, BarisData, NilaiKolom, HalamanID,
        OperatorFisik, StatusEksekusi
    };

    antarmuka IteratorVolcano {
        fungsi buka(&mut diri) -> Hasil<(), Teks>;
        fungsi berikutnya(&mut diri) -> Hasil<Opsi<BarisData>, Teks>;
        fungsi tutup(&mut diri) -> Hasil<(), Teks>;
    }

    struktur PindaianTabelFisik {
        nama_tabel: Teks,
        halaman_kini: Bulat,
        slot_kini: Bulat,
        total_halaman: Bulat,
        mesin: &WidyaBase,
    }

    implementasi IteratorVolcano untuk PindaianTabelFisik {
        fungsi buka(&mut diri) -> Hasil<(), Teks> {
            diri.halaman_kini = 0;
            diri.slot_kini = 0;
            cetak(">>> [VOLCANO-SCAN] Membuka tabel: " + diri.nama_tabel);
            kembalikan Ok(());
        }

        fungsi berikutnya(&mut diri) -> Hasil<Opsi<BarisData>, Teks> {
            selama diri.halaman_kini < diri.total_halaman {
                biar baris_opsi = diri.mesin.baca_tupel(diri.nama_tabel, diri.halaman_kini, diri.slot_kini)?;
                diri.slot_kini = diri.slot_kini + 1;

                jika biar Ada(baris) = baris_opsi {
                    kembalikan Ok(Ada(baris));
                } selain_itu {
                    // Pindah ke halaman berikutnya
                    diri.halaman_kini = diri.halaman_kini + 1;
                    diri.slot_kini = 0;
                }
            }
            kembalikan Ok(TidakAda); // Aliran data habis
        }

        fungsi tutup(&mut diri) -> Hasil<(), Teks> {
            cetak(">>> [VOLCANO-SCAN] Menutup tabel: " + diri.nama_tabel);
            kembalikan Ok(());
        }
    }

    struktur FilterFisik<T: IteratorVolcano> {
        anak: T,
        nama_kolom: Teks,
        nilai_ambang: Desimal,
    }

    implementasi<T: IteratorVolcano> IteratorVolcano untuk FilterFisik<T> {
        fungsi buka(&mut diri) -> Hasil<(), Teks> {
            kembalikan diri.anak.buka();
        }

        fungsi berikutnya(&mut diri) -> Hasil<Opsi<BarisData>, Teks> {
            ulang {
                biar hasil_anak = diri.anak.berikutnya()?;
                cocokkan hasil_anak {
                    Ada(baris) => {
                        biar nilai = baris.ambil_desimal(&diri.nama_kolom);
                        jika nilai >= diri.nilai_ambang {
                            kembalikan Ok(Ada(baris)); // Lolos filter
                        }
                        // Jika tidak lolos, lanjutkan loop untuk mengambil baris berikutnya
                    },
                    TidakAda => kembalikan Ok(TidakAda), // Selesai
                }
            }
        }

        fungsi tutup(&mut diri) -> Hasil<(), Teks> {
            kembalikan diri.anak.tutup();
        }
    }
}
```

### 13.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan paradigma eksekusi pemrosesan kueri pada WidyaBase:

| Karakteristik Paradigma | Volcano Tuple-at-a-Time | Vectorized Execution (Batching) | Push-Based Compilation (Hyper) |
| :--- | :--- | :--- | :--- |
| **Model Aliran Data** | Pull (Tarik satu per satu) | Pull (Tarik 1.024 baris/batch) | Push (Dorong melalui JIT LLVM) |
| **Panggilan Fungsi Virtual** | Tinggi (1 panggilan per baris) | Sangat Rendah (1 per blok 1.024) | Hampir Nol (Inlined Code) |
| **Pemanfaatan Cache CPU L1/L2**| Rendah (sering ganti konteks) | Sangat Optimal (Data fit di L1) | Sangat Optimal |
| **Konsumsi Memori RAM** | Sangat Hemat ($O(1)$) | Terukur ($1.024 \times \text{lebar}$) | Terukur |
| **Kasus Penggunaan Terbaik** | Beban OLTP & Kueri Cepat | Analitis OLAP & Agregasi Masif | Batch Analytics Terkompilasi |

### 13.4 Latihan Soal Esai

1. Terangkan cara kerja pemanggilan berantai tiga metode baku (`buka`, `berikutnya`, `tutup`) pada pohon eksekusi operator model Volcano!
2. Mengapa model Volcano murni (*Tuple-at-a-Time*) mengalami fenomena *Instruction Cache Miss* yang tinggi pada pemrosesan kueri analitis berjuta baris?
3. Bagaimana pendekatan eksekusi tervektorisasi (*Vectorized Volcano*) mengatasi kelemahan performa CPU cache tanpa mengorbankan kesederhanaan arsitektur iterator?

### 13.5 Soal Pilihan Ganda

1. Tiga metode inti yang wajib dimiliki oleh setiap operator fisik pada Model Iterator Volcano adalah...
   A. `buka()`, `berikutnya()`, dan `tutup()`
   B. `baca()`, `tulis()`, dan `hapus()`
   C. `komit()`, `batal()`, dan `kunci()`
   D. `mulai()`, `tunggu()`, dan `selesai()`

2. Sifat pemrosesan data di mana baris berikutnya hanya ditarik ketika diminta oleh operator di atasnya dikenal sebagai model...
   A. Demand-Driven (Pull-Based)
   B. Event-Driven (Push-Based)
   C. Batch Processing Luar Jaringan
   D. Random Memory Access

3. Nilai yang dikembalikan oleh metode `berikutnya()` ketika seluruh baris data dari tabel telah selesai dibaca adalah...
   A. Nilai `TidakAda` (atau `None` / `EOF`)
   B. Pesan Galat Sistem
   C. Angka Nol
   D. Memori RAM hang

4. Apa keuntungan utama dari model iterator baris-per-baris terhadap konsumsi memori RAM?
   A. Memori yang dibutuhkan sangat kecil dan konstan ($O(1)$) karena tidak perlu menampung seluruh tabel di RAM
   B. Menghapus kebutuhan harddisk
   C. Menjadikan kabel LAN lebih dingin
   D. Memperbesar kapasitas penyimpanan awan

5. Pada kueri antarmuka pengguna interaktif (misalnya paginasi web), model Volcano memungkinkan...
   A. Tampilan halaman pertama muncul instan dalam hitungan mikrodetik tanpa menunggu jutaan baris selesai dibaca
   B. Otomatisasi restart peladen
   C. Pengunduhan file musik
   D. Penutupan akun pengguna secara sepihak

6. Pendekatan eksekusi yang memproses data dalam potongan bundel berisi ratusan hingga ribuan baris sekaligus dinamakan...
   A. Single Cycle Execution
   B. Vectorized Execution (Pemrosesan Vektorisasi)
   C. Serial One-By-One
   D. Deadlock Mitigation

7. Operator fisik yang bertugas menyaring baris berdasarkan kebenaran predikat logika matematika dinamakan...
   A. PhysicalProject
   B. PhysicalFilter
   C. PhysicalSort
   D. PhysicalJoin

8. Operator fisik yang bertugas mengambil subset kolom tertentu atau menghitung fungsi aritmatika pada kolom luaran adalah...
   A. PhysicalProject
   B. PhysicalLimit
   C. PhysicalVacuum
   D. PhysicalCheckpoint

9. Fenomena di mana instruksi CPU sering berganti-ganti antar modul operator dalam pemrosesan Volcano klasik disebut...
   A. High Context Switching & Instruction Cache Miss
   B. Stack Overflow
   C. Null Pointer Exception
   D. Segment Fault

10. Mengapa operator seperti `URUTKAN_BERDASARKAN` (*ORDER BY*) disebut sebagai operator pemblokir penuh (*Pipeline Breaker*)?
    A. Karena operator tersebut harus membaca seluruh baris data hingga tuntas sebelum dapat mengembalikan baris pertama yang terurut
    B. Karena operator tersebut merusak kabel jaringan
    C. Karena operator tersebut menolak seluruh transaksi pengguna
    D. Karena operator tersebut menghapus tabel dari harddisk
