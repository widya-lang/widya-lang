## Bab 23: Algoritma Pengurutan pada Larik: Bubble dan Selection Sort

### 23.1 Algoritma Pengurutan Gelembung (*Bubble Sort*)

```mermaid
graph TD
    A[Mulai Bab 23: Algoritma Pengurutan Larik] --> B[Pahami Konsep Bubble Sort]
    B --> C[Terapkan Sintaksis Selection Sort]
    C --> D[Uji Validitas Data & kompleksitas O(N^2)]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Bubble Sort bekerja dengan membandingkan pasangan elemen yang bersebelahan dan menukarnya jika urutannya salah. Elemen bernilai besar akan "mengapung" ke posisi akhir seperti gelembung udara di dalam air.

```widya
fungsi urutkan_gelembung(larik) {
    tetap n = panjang(larik);
    variabel i = 0;
    selama (i < n - 1) {
        variabel j = 0;
        selama (j < n - i - 1) {
            jika (larik[j] > larik[j + 1]) {
                // Tukar nilai
                tetap sementara = larik[j];
                larik[j] = larik[j + 1];
                larik[j + 1] = sementara;
            }
            j += 1;
        }
        i += 1;
    }
}
```

### 23.2 Algoritma Pengurutan Seleksi (*Selection Sort*)
Selection Sort bekerja dengan mencari nilai minimum dari sisa larik, lalu menukarkannya langsung ke posisi depan yang tepat:

```widya
fungsi urutkan_seleksi(larik) {
    tetap n = panjang(larik);
    variabel i = 0;
    selama (i < n - 1) {
        variabel min_idx = i;
        variabel j = i + 1;
        selama (j < n) {
            jika (larik[j] < larik[min_idx]) {
                min_idx = j;
            }
            j += 1;
        }
        tetap temp = larik[i];
        larik[i] = larik[min_idx];
        larik[min_idx] = temp;
        i += 1;
    }
}
```

### 23.3 Rangkuman Konsep Kunci Bab 23
- Pengurutan data memudahkan pencarian cepat (*Binary Search*) dan visualisasi laporan.
- Bubble Sort menukar pasangan bersebelahan.
- Selection Sort mencari nilai ekstrem (minimum) secara berulang.

### 23.4 Latihan Mandiri & Evaluasi Pemahaman
1. Jelaskan mengapa Bubble Sort memiliki kompleksitas waktu $O(N^2)$ pada kasus terburuk!
2. Bagaimana cara membalik algoritma pengurutan agar menghasilkan urutan menurun (*descending*)?
3. **Tantangan Praktik**: Buatlah program yang mengurutkan 10 skor ujian siswa secara menurun dari nilai tertinggi hingga terendah!

---

### 23.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 23 (Algoritma Pengurutan Larik)?**
- A. Membahas secara mendalam Pengurutan data
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `Selection Sort`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep Bubble Sort sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana kompleksitas O(N^2) memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 23?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah Bubble Sort?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan Selection Sort?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 23 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 23, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Pengurutan data secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
