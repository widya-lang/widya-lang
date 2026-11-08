## Bab 32: Algoritma Rekursi dan Strategi Pembagian Masalah

### 32.1 Konsep Dasar Pemanggilan Diri Sendiri

```mermaid
graph TD
    A[Mulai Bab 32: Algoritma Rekursi] --> B[Pahami Konsep kasus rekursif]
    B --> C[Terapkan Sintaksis kasus dasar (base case)]
    C --> D[Uji Validitas Data & mencegah stack overflow]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Rekursi terjadi ketika sebuah fungsi memanggil dirinya sendiri secara langsung atau tidak langsung untuk menyelesaikan sub-masalah yang lebih kecil dari permasalahan utama. 

Setiap algoritma rekursif wajib memiliki dua komponen esensial:
1. **Kasus Dasar (*Base Case*)**: Kondisi terminasi di mana fungsi berhenti memanggil dirinya sendiri dan langsung mengembalikan nilai konstan. Tanpa kasus dasar, program akan terperangkap dalam perulangan tanpa henti hingga memori tumpukan meluap (*Stack Overflow*).
2. **Kasus Rekursif (*Recursive Step*)**: Langkah pemanggilan fungsi terhadap data yang telah disederhanakan/dikecilkan skalanya.

```
  faktorial(4)
    |---> 4 * faktorial(3)
                |---> 3 * faktorial(2)
                            |---> 2 * faktorial(1)
                                        |---> 1 (Kasus Dasar tercapai!)
```

### 32.2 Faktorial dan Deret Fibonacci
Perhitungan faktorial matematis $n! = n 	imes (n-1)!$:

```widya
fungsi faktorial(n) {
    // 1. Kasus Dasar
    jika (n <= 1) {
        kembalikan 1;
    }
    // 2. Kasus Rekursif
    kembalikan n * faktorial(n - 1);
}

cetak("5! = " + teks(faktorial(5))); // 120
cetak("7! = " + teks(faktorial(7))); // 5040
```

Deret angka Fibonacci di mana setiap angka adalah jumlah dari dua angka sebelumnya:

```widya
fungsi fibonacci(n) {
    jika (n <= 0) {
        kembalikan 0;
    }
    jika (n == 1) {
        kembalikan 1;
    }
    kembalikan fibonacci(n - 1) + fibonacci(n - 2);
}

variabel i = 0;
variabel deret = [];
selama (i <= 10) {
    tambah(deret, teks(fibonacci(i)));
    i = i + 1;
}
cetak("Deret 10 Fibonacci Pertama: " + gabung(deret, ", "));
```

### 32.3 Permainan Menara Hanoi
Masalah klasik Menara Hanoi mengilustrasikan kekuatan rekursi dalam menyelesaikan puzzle rumit hanya dengan beberapa baris kode:

```widya
fungsi pindah_piringan(n, tiang_asal, tiang_tujuan, tiang_bantu) {
    jika (n == 1) {
        cetak("Pindahkan piringan 1 dari " + tiang_asal + " langsung ke " + tiang_tujuan);
        kembalikan;
    }
    
    // Pindahkan n-1 piringan dari asal ke bantu
    pindah_piringan(n - 1, tiang_asal, tiang_bantu, tiang_tujuan);
    
    // Pindahkan piringan terbesar ke tujuan
    cetak("Pindahkan piringan " + teks(n) + " dari " + tiang_asal + " ke " + tiang_tujuan);
    
    // Pindahkan n-1 piringan dari bantu ke tujuan
    pindah_piringan(n - 1, tiang_bantu, tiang_tujuan, tiang_asal);
}

fungsi utama() {
    cetak("--- SOLUSI MENARA HANOI (3 Piringan) ---");
    pindah_piringan(3, "Pasak A", "Pasak C", "Pasak B");
}
```

### 32.4 Rangkuman Konsep Kunci Bab 32
- Rekursi memecah masalah menjadi salinan lebih kecil dari dirinya sendiri.
- Wajib memiliki kasus dasar untuk mencegah luapan stack (*Stack Overflow*).

### 32.5 Latihan Mandiri & Evaluasi Pemahaman
1. Mengapa fungsi rekursif tanpa kasus dasar menyebabkan program terhenti dengan pesan *Stack Overflow*?
2. Bagaimana cara menghitung deret pangkat $x^n$ menggunakan rekursi?
3. **Tantangan Praktik**: Rancanglah fungsi rekursif `hitung_jumlah_digit(angka)` yang menjumlahkan setiap digit angka (contoh: 1234 -> 1+2+3+4 = 10)!

---

### 32.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 32 (Algoritma Rekursi)?**
- A. Membahas secara mendalam Pemanggilan diri sendiri
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `kasus dasar (base case)`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep kasus rekursif sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana mencegah stack overflow memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 32?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah kasus rekursif?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan kasus dasar (base case)?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 32 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 32, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Pemanggilan diri sendiri secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
