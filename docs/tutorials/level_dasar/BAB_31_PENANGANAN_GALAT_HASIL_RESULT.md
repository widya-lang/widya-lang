## Bab 31: Penanganan Galat Tipe `Hasil` (Result) dan Pertahanan Sistem

### 31.1 Membedakan Galat Terduga dan Kegagalan Fatal

```mermaid
graph TD
    A[Mulai Bab 31: Penanganan Galat Hasil Result] --> B[Pahami Konsep Hasil::Gagal(e)]
    B --> C[Terapkan Sintaksis Hasil::Sukses(v)]
    C --> D[Uji Validitas Data & validasi multi langkah]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Galat dalam pemrograman dibagi menjadi dua kategori:
1. **Galat Terduga (*Expected Errors*)**: Berkas tidak ada, sambungan jaringan putus sementara, sandi salah, kartu ATM saldo kurang. Ini BUKAN kerusakan program, melainkan variasi input dunia nyata yang harus ditangani secara terstruktur.
2. **Kondisi Fatal (*Unrecoverable Bugs*)**: Kehabisan memori fisik komputer (*out of memory*) atau pelanggaran asersi logika kode.

Widya-Lang menangani galat terduga menggunakan tipe varian `Hasil::Sukses(data)` dan `Hasil::Gagal(pesan)`.

### 31.2 Implementasi Pembagian Angka Aman
Operasi pembagian dengan angka nol adalah kesalahan matematika klasik yang menghentikan eksekusi jika tidak diantisipasi:

```widya
fungsi bagi_aman(pembilang, penyebut) {
    jika (penyebut == 0) {
        kembalikan Hasil::Gagal("Pembagian dengan nol dilarang dalam kalkulasi matematika.");
    }
    kembalikan Hasil::Sukses(pembilang / penyebut);
}

tetap kalkulasi1 = bagi_aman(100, 4);
tetap kalkulasi2 = bagi_aman(50, 0);

fungsi periksa_hasil(res) {
    cocok (res) {
        kasus Hasil::Sukses(nilai) => {
            cetak("Perhitungan sukses, hasil: " + teks(nilai));
        },
        kasus Hasil::Gagal(alasan) => {
            cetak("KESALAHAN OPERASIONAL: " + alasan);
        }
    }
}

periksa_hasil(kalkulasi1);
periksa_hasil(kalkulasi2);
```

### 31.3 Rangkuman Konsep Kunci Bab 31
- Tipe `Hasil` memodelkan hasil operasi yang berpotensi gagal dengan varian `Sukses` dan `Gagal`.
- Tidak ada eksepsi tersembunyi yang membuat peladen crash mendadak.

### 31.4 Latihan Mandiri & Evaluasi Pemahaman
1. Apa keunggulan penanganan galat berbasis tipe data dibandingkan mekanisme `try-catch` tradisional?
2. Bagaimana cara membuat rantai validasi di mana langkah kedua hanya dijalankan jika langkah pertama sukses?
3. **Tantangan Praktik**: Buatlah fungsi `validasi_umur_ktp(umur)` yang mengembalikan `Hasil::Sukses(umur)` jika usia >= 17 atau `Hasil::Gagal("Usia di bawah umur")` jika kurang dari 17!

---

### 31.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 31 (Penanganan Galat Hasil Result)?**
- A. Membahas secara mendalam Pertahanan kegagalan sistem
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `Hasil::Sukses(v)`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep Hasil::Gagal(e) sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana validasi multi langkah memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 31?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah Hasil::Gagal(e)?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan Hasil::Sukses(v)?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 31 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 31, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Pertahanan kegagalan sistem secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
