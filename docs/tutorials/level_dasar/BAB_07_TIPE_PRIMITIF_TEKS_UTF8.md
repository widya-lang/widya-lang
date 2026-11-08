## Bab 7: Sistem Tipe Data Primitif Bagian 2 - Teks dan Karakter

### 7.1 Standar Pengkodean UTF-8 Dinamis

```mermaid
graph TD
    A[Mulai Bab 7: Tipe Primitif Teks UTF-8] --> B[Pahami Konsep indeks berbasis 0]
    B --> C[Terapkan Sintaksis teks()]
    C --> D[Uji Validitas Data & panjang()]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Teks adalah media komunikasi fundamental antara manusia dan aplikasi perangkat lunak. Pada dekade 1960-an, standar ASCII (*American Standard Code for Information Interchange*) dirancang dengan hanya mengalokasikan 7 bit memori (128 karakter), yang hanya mencukupi untuk alfabet bahasa Inggris tanpa aksen.

Di era globalisasi dan kedaulatan informasi modern, pembatasan ASCII telah usang. Widya-Lang menerapkan standar pengkodean **UTF-8 (*8-bit Unicode Transformation Format*)** secara murni pada tingkat pustaka inti. UTF-8 adalah sistem pengkodean variabel (1 hingga 4 bita per karakter) yang mampu menampung lebih dari 1,1 juta karakter dari seluruh sistem peradaban manusia:
- **1 Bita**: Karakter Latin standar (kompatibel penuh dengan ASCII).
- **2 Bita**: Aksara Arab, Cyrillic, Yunani, dan Ibrani.
- **3 Bita**: Aksara Nusantara (Aksara Jawa Hanacaraka, Bali, Sunda, Batak) serta karakter Tionghoa/Jepang/Korea (CJK).
- **4 Bita**: Simbol piktografik emoji modern, simbol matematika rumit, dan naskah sejarah kuno.

```
+---------------------------------------------------------------+
|             ALOKASI PANJANG BITA KARAKTER UTF-8               |
+---------------------------------------------------------------+
|  Karakter Contoh       | Aksara / Kategori     | Panjang Bita |
+------------------------+-----------------------+--------------+
|  "A"                   | Alfabet Latin Standar | 1 Bita       |
|  "ꦱ" (Sa)              | Aksara Jawa Nusantara | 3 Bita       |
|  "🚀"                  | Piktografik Emoji     | 4 Bita       |
+------------------------+-----------------------+--------------+
```

### 7.2 Operasi Penggabungan String (*Concatenation*)
Dua atau lebih string dapat disatukan menggunakan operator tambah `+`. Jika salah satu operan bertipe numerik atau boolean, ubahlah terlebih dahulu menjadi string menggunakan fungsi `teks(...)`:

```widya
fungsi utama() {
    tetap sapaan = "Selamat Pagi";
    tetap nama = "Ibu Kartini";
    tetap kalimat = sapaan + ", " + nama + "! Selamat berkarya.";
    cetak(kalimat);
}
```

### 7.3 Pengukuran Panjang Teks dengan `panjang(...)`
Fungsi bawaan `panjang(...)` mengembalikan jumlah elemen karakter dalam teks string:

```widya
fungsi utama() {
    tetap judul_buku = "Kamus Besar Bahasa Indonesia";
    tetap jumlah_huruf = panjang(judul_buku);
    cetak("Judul : " + judul_buku);
    cetak("Panjang Karakter: " + teks(jumlah_huruf));
}
```

### 7.4 Akses Karakter Berbasis Indeks Nol
Karakter di dalam string dapat diakses menggunakan kurung siku `[indeks]`. Widya-Lang menggunakan konvensi indeks berbasis nol (*zero-based indexing*):

```widya
fungsi utama() {
    tetap negara = "INDONESIA";
    // Karakter pertama berada pada indeks 0
    cetak("Huruf Pertama: " + negara[0]); // 'I'
    cetak("Huruf Kedua  : " + negara[1]); // 'N'
    cetak("Huruf Akhir  : " + negara[panjang(negara) - 1]); // 'A'
}
```

### 7.5 Contoh Kode Praktik 1: Generator Format Surat Dinas Resmi
Program ini menyusun nomor surat dinas kementerian secara otomatis dari variabel-variabel penanda:

```widya
// Berkas: bab07_generator_surat.widya

fungsi utama() {
    tetap KODE_KLASIFIKASI = "HM.01.02";
    tetap NOMOR_URUT = "0458";
    tetap KODE_INSTANSI = "WIDYA-PUSAT";
    tetap BULAN_ROMAWI = "X";
    tetap TAHUN_ANGGARAN = "2026";
    
    // Merangkai format resmi: KODE/NOMOR/INSTANSI/BULAN/TAHUN
    tetap nomor_surat_lengkap = KODE_KLASIFIKASI + "/" 
                              + NOMOR_URUT + "/" 
                              + KODE_INSTANSI + "/" 
                              + BULAN_ROMAWI + "/" 
                              + TAHUN_ANGGARAN;
                              
    cetak("KEMENTERIAN RISET DAN KEDAULATAN DIGITAL");
    cetak("REPUBLIK INDONESIA");
    cetak("--------------------------------------------------");
    cetak("Nomor Surat Resmi : " + nomor_surat_lengkap);
    cetak("Lampiran          : 1 (Satu) Berkas");
    cetak("Perihal           : Undangan Simposium Bahasa Pemrograman");
    cetak("--------------------------------------------------");
    cetak("Surat ini telah diverifikasi dan sah secara digital.");
}
```

### 7.6 Contoh Kode Praktik 2: Pelat Nomor Kendaraan dan Validasi Wilayah
Program ini mengurai pelat nomor kendaraan bermotor dan mengidentifikasi asal kota registrasi:

```widya
// Berkas: bab07_pelat_nomor.widya

fungsi utama() {
    tetap pelat_kendaraan = "AB 1234 XY";
    tetap kode_wilayah = pelat_kendaraan[0] + pelat_kendaraan[1];
    
    cetak("Data Identitas Kendaraan Bermotor:");
    cetak("- Pelat Lengkap : " + pelat_kendaraan);
    cetak("- Kode Huruf Awal: " + kode_wilayah);
    
    jika (kode_wilayah == "AB") {
        cetak("- Wilayah Asal  : Daerah Istimewa Yogyakarta (DIY)");
    } jika_tidak jika (kode_wilayah == "AD") {
        cetak("- Wilayah Asal  : Karesidenan Surakarta (Solo)");
    } jika_tidak jika (kode_wilayah == "B ") {
        cetak("- Wilayah Asal  : DKI Jakarta / Depok / Bekasi");
    } lainnya {
        cetak("- Wilayah Asal  : Terdaftar di Luar Wilayah DIY-Solo");
    }
}
```

### 7.7 Kesalahan Pemula dan Panduan Solusi
- **Galat Mengakses Indeks di Luar Batas (*Index Out of Bounds*)**:
  Jika string memiliki panjang 5 huruf (indeks 0 s.d. 4), mencoba memanggil `teks[5]` akan memicu galat runtime fatal. Pastikan indeks selalu lebih kecil dari `panjang(teks)`.
- **Lupa Mengonversi Angka Saat Penggabungan**:
  Menulis `"Total: " + 100` dapat menimbulkan ambiguitas tipe data. Gunakan selalu `"Total: " + teks(100)`.

### 7.8 Rangkuman Konsep Kunci Bab 7
- Mesin teks Widya-Lang mendukung UTF-8 penuh (aksara nusantara dan emoji).
- Penggabungan teks dilakukan dengan operator `+`.
- Panjang karakter diukur dengan fungsi `panjang(...)`.
- Indeks karakter string dimulai dari angka `0`.

### 7.9 Latihan Mandiri & Evaluasi Pemahaman
1. Mengapa standar ASCII tidak memadai untuk bahasa pemrograman berdaulat di Indonesia?
2. Bagaimana cara mengambil karakter terakhir dari sebuah variabel string apa pun tanpa mengetahui panjangnya terlebih dahulu?
3. **Tantangan Praktik**: Buatlah program `sensor_kata.widya` yang memiliki variabel kalimat rahasia, lalu tampilkan karakter pertama dan terakhirnya saja, sedangkan huruf di tengahnya digantikan oleh tanda bintang `*`!

---

### 7.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 7 (Tipe Primitif Teks UTF-8)?**
- A. Membahas secara mendalam Standar UTF-8 multibita
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `teks()`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep indeks berbasis 0 sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana panjang() memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 7?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah indeks berbasis 0?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan teks()?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 7 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 7, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Standar UTF-8 multibita secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
