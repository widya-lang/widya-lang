## Bab 24: Paradigma Pemrosesan Larik Modern: Petakan dan Saring

### 24.1 Transformasi Elemen Koleksi

```mermaid
graph TD
    A[Mulai Bab 24: Pemrosesan Larik Petakan Saring] --> B[Pahami Konsep petakan (map)]
    B --> C[Terapkan Sintaksis saring (filter)]
    C --> D[Uji Validitas Data & transformasi deklaratif]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Alih-alih menulis perulangan `selama` atau `untuk` yang rawan kesalahan indeks, Widya-Lang mendukung pola transformasi deklaratif:
- **Penyaringan (*Filter*)**: Memilih hanya data yang memenuhi kriteria tertentu.
- **Pemetaan (*Map*)**: Mengubah setiap elemen menjadi format nilai baru.

```widya
fungsi saring_bilangan_genap(larik_angka) {
    variabel hasil = [];
    untuk x dalam larik_angka {
        jika (x % 2 == 0) {
            tambah(hasil, x);
        }
    }
    kembalikan hasil;
}

fungsi gandakan_nilai(larik_angka) {
    variabel hasil = [];
    untuk x dalam larik_angka {
        tambah(hasil, x * 2);
    }
    kembalikan hasil;
}

fungsi utama() {
    tetap angka_mentah = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    tetap genap = saring_bilangan_genap(angka_mentah);
    tetap lipat_dua = gandakan_nilai(genap);
    
    cetak("Hasil Seleksi & Transformasi: " + gabung(lipat_dua, ", "));
}
```

### 24.2 Rangkuman Konsep Kunci Bab 24
- Penyaringan memilih subset data berdasarkan predikat kebenaran logika.
- Pemetaan mentransformasikan setiap nilai menjadi bentuk baru tanpa mengubah panjang larik.

### 24.3 Latihan Mandiri & Evaluasi Pemahaman
1. Sebutkan kelebihan gaya fungsional (*map/filter*) dibandingkan perulangan imperatif biasa!
2. Bagaimana cara menyaring siswa yang memiliki nilai kelulusan di atas rata-rata kelas?
3. **Tantangan Praktik**: Rancanglah fungsi yang menerima larik harga barang dalam Dollar (USD), lalu memetakannya menjadi larik harga dalam Rupiah (IDR) dengan kurs Rp 15.500 per USD!

---

### 24.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 24 (Pemrosesan Larik Petakan Saring)?**
- A. Membahas secara mendalam Paradigma fungsional
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `saring (filter)`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep petakan (map) sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana transformasi deklaratif memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 24?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah petakan (map)?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan saring (filter)?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 24 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 24, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Paradigma fungsional secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
