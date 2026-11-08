## Bab 22: Algoritma Pencarian pada Larik: Linear Search vs Binary Search

### 22.1 Konsep Kompleksitas Algoritma Waktu

```mermaid
graph TD
    A[Mulai Bab 22: Algoritma Pencarian Larik] --> B[Pahami Konsep Binary Search O(log N)]
    B --> C[Terapkan Sintaksis Linear Search O(N)]
    C --> D[Uji Validitas Data & larik wajib terurut]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Saat data dalam larik bertambah dari puluhan menjadi jutaan elemen, kecepatan algoritma pencarian menjadi penentu utama apakah aplikasi Anda responsif atau terhenti (*hang*).

Dua pendekatan utama:
1. **Pencarian Berurutan (*Linear Search*)**: Memeriksa elemen satu per satu dari indeks 0 hingga akhir. Kompleksitas: $O(N)$ (Waktu pencarian sebanding dengan jumlah data).
2. **Pencarian Bagi Dua (*Binary Search*)**: Mencari pada larik yang **sudah terurut** dengan membagi rentang pencarian menjadi dua bagian di setiap langkah. Kompleksitas: $O(\log N)$ (Untuk 1 juta data, hanya butuh sekitar 20 kali perbandingan!).

```widya
// IMPLEMENTASI LINEAR SEARCH DI WIDYA-LANG
fungsi cari_linear(larik, target) {
    variabel i = 0;
    selama (i < panjang(larik)) {
        jika (larik[i] == target) {
            kembalikan i; // Ditemukan pada indeks ke-i
        }
        i += 1;
    }
    kembalikan -1; // Tidak ditemukan
}

// IMPLEMENTASI BINARY SEARCH (Larik wajib sudah terurut!)
fungsi cari_biner(larik_terurut, target) {
    variabel kiri = 0;
    variabel kanan = panjang(larik_terurut) - 1;
    
    selama (kiri <= kanan) {
        tetap tengah = (kiri + kanan) / 2;
        jika (larik_terurut[tengah] == target) {
            kembalikan tengah;
        }
        jika (larik_terurut[tengah] < target) {
            kiri = tengah + 1; // Cari di separuh kanan
        } lainnya {
            kanan = tengah - 1; // Cari di separuh kiri
        }
    }
    kembalikan -1;
}

fungsi utama() {
    tetap data_terurut = [12, 25, 38, 47, 56, 69, 78, 89, 94];
    tetap pos = cari_biner(data_terurut, 69);
    cetak("Angka 69 ditemukan pada indeks: " + teks(pos));
}
```

### 22.2 Rangkuman Konsep Kunci Bab 22
- Linear Search bekerja pada larik acak dengan kompleksitas $O(N)$.
- Binary Search membutuhkan larik yang telah terurut rapi, menghasilkan efisiensi spektakuler $O(\log N)$.

### 22.3 Latihan Mandiri & Evaluasi Pemahaman
1. Berapa kali operasi perbandingan maksimal yang dilakukan oleh Binary Search pada kumpulan data sebanyak 1.024 elemen?
2. Mengapa Binary Search tidak dapat langsung diaplikasikan pada larik yang belum terurut?
3. **Tantangan Praktik**: Modifikasilah fungsi `cari_linear` agar menghitung dan mengembalikan frekuensi kemunculan (jumlah kemunculan) suatu angka di dalam larik!

---

### 22.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 22 (Algoritma Pencarian Larik)?**
- A. Membahas secara mendalam Pencarian efisien
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `Linear Search O(N)`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep Binary Search O(log N) sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana larik wajib terurut memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 22?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah Binary Search O(log N)?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan Linear Search O(N)?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 22 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 22, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Pencarian efisien secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
