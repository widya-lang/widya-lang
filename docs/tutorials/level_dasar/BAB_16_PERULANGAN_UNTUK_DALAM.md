## Bab 16: Perulangan Koleksi dan Rentang dengan `untuk` (For Loop)

### 16.1 Filosofi Iterasi Elemen Koleksi yang Aman

```mermaid
graph TD
    A[Mulai Bab 16: Perulangan Untuk Dalam] --> B[Pahami Konsep henti (break)]
    B --> C[Terapkan Sintaksis untuk item dalam koleksi]
    C --> D[Uji Validitas Data & lanjutkan (continue)]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Mengakses elemen koleksi data (seperti larik nama atau daftar transaksi) menggunakan indeks manual pada perulangan `selama` seringkali rentan terhadap kesalahan indeks di luar batas (*Array Index Out of Bounds*).

Widya-Lang menyediakan struktur **`untuk ... dalam ...`** (*Iterator-Based For Loop*). Struktur ini menjamin bahwa setiap elemen dalam koleksi dikunjungi tepat satu kali secara berurutan, aman dari galat luapan indeks, dan memiliki performa optimal karena dioptimasi oleh mesin kompilator.

```widya
fungsi utama() {
    tetap pahlawan_nasional = [
        "Pangeran Diponegoro",
        "Cut Nyak Dien",
        "Tuanku Imam Bonjol",
        "I Gusti Ngurah Rai",
        "Pattimura"
    ];
    
    cetak("DAFTAR PAHLAWAN KEMERDEKAAN NUSANTARA:");
    variabel no = 1;
    untuk nama dalam pahlawan_nasional {
        cetak(teks(no) + ". " + nama);
        no += 1;
    }
}
```

### 16.2 Pengendali Lompatan Eksekusi: `henti` dan `lanjutkan`
Widya-Lang menyediakan dua kata kunci untuk memodifikasi alur normal perulangan:
- **`henti` (*break*)**: Menghentikan paksa seluruh proses perulangan seketika dan langsung melompat ke baris setelah penutup blok kurung kurawal.
- **`lanjutkan` (*continue*)**: Menghentikan eksekusi sisa baris kode pada iterasi saat ini, lalu langsung melompat ke iterasi elemen berikutnya.

```widya
fungsi utama() {
    cetak("PENCARIAN DATA BILANGAN:");
    variabel i = 1;
    selama (i <= 10) {
        jika (i == 4) {
            i += 1;
            lanjutkan; // Lewati angka 4
        }
        jika (i == 8) {
            cetak("Batas toleransi 8 tercapai, hentikan proses!");
            henti; // Hentikan seluruh loop
        }
        cetak("Memproses data ke-" + teks(i));
        i += 1;
    }
}
```

### 16.3 Contoh Kode Praktik: Pembangkit Pola Matriks Geometri Bintang
Program berikut mendemonstrasikan perulangan bersarang (*nested loop*) untuk mencetak struktur matriks koordinat:

```widya
// Berkas: bab16_matriks_koordinat.widya

fungsi utama() {
    tetap JUMLAH_BARIS = 4;
    tetap JUMLAH_KOLOM = 5;
    
    cetak("PETA KOORDINAT GRID RADAR GEOSPASIAL:");
    variabel b = 1;
    selama (b <= JUMLAH_BARIS) {
        variabel baris_teks = "";
        variabel k = 1;
        selama (k <= JUMLAH_KOLOM) {
            baris_teks = baris_teks + "[" + teks(b) + "," + teks(k) + "] ";
            k += 1;
        }
        cetak(baris_teks);
        b += 1;
    }
}
```

### 16.4 Kesalahan Pemula dan Panduan Solusi
- **Memodifikasi Panjang Koleksi Saat Sedang Diiterasi**:
  Menghapus atau menyisipkan elemen ke dalam larik yang sedang dijelajahi oleh perulangan `untuk` dapat menyebabkan inkonsistensi iterator. Gandakan larik atau simpan indeks target terlebih dahulu sebelum melakukan mutasi.

### 16.5 Rangkuman Konsep Kunci Bab 16
- `untuk ... dalam ...` adalah cara paling aman dan bersih untuk melintasi elemen koleksi.
- `henti` menghentikan loop, `lanjutkan` melompat ke putaran berikutnya.
- Perulangan bersarang memproses struktur berdimensi banyak seperti matriks dan tabel.

### 16.6 Latihan Mandiri & Evaluasi Pemahaman
1. Mengapa perulangan berbasis iterator (`untuk`) lebih aman daripada perulangan berindeks manual (`selama`)?
2. Kapan sebaiknya kita menggunakan kata kunci `lanjutkan` di dalam pemrosesan data batch?
3. **Tantangan Praktik**: Buatlah program `tabel_faktorial.widya` yang menggunakan perulangan untuk menghitung dan menampilkan nilai faktorial dari angka 1! hingga 10! secara berurutan!

---

### 16.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 16 (Perulangan Untuk Dalam)?**
- A. Membahas secara mendalam Iterasi koleksi data
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `untuk item dalam koleksi`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep henti (break) sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana lanjutkan (continue) memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 16?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah henti (break)?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan untuk item dalam koleksi?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 16 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 16, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Iterasi koleksi data secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
