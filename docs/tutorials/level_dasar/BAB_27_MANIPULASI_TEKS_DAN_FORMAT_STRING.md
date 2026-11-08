## Bab 27: Manipulasi Teks dan Pemformatan String Lanjutan

### 27.1 Representasi Karakter dan Karakteristik UTF-8

```mermaid
graph TD
    A[Mulai Bab 27: Manipulasi Teks dan Format String] --> B[Pahami Konsep pangkas spasi]
    B --> C[Terapkan Sintaksis pecah() dan gabung()]
    C --> D[Uji Validitas Data & perataan kolom tabel CLI]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Dalam dunia komputasi modern, penanganan teks tidak lagi terbatas pada 128 karakter ASCII standar. Widya-Lang mengadopsi standar pengkodean UTF-8 secara alami pada tingkat inti mesin. Setiap karakter teks (termasuk huruf Nusantara seperti Aksara Jawa, Bali, Batak, serta karakter emoji dan simbol matematika) direpresentasikan secara tepat tanpa distorsi data (*mojibake*).

```widya
// Contoh keragaman karakter dalam UTF-8 murni
tetap salam_nusantara = "ꦱꦸꦒꦼꦁꦫꦮꦸꦃ (Sugeng Rawuh)";
tetap status_sistem = "Sistem Berjalan Prima 🚀✨";

cetak(salam_nusantara);
cetak(status_sistem);
cetak("Panjang bita: " + teks(panjang(status_sistem)));
```

### 27.2 Operasi String Standar
Teks dalam Widya-Lang dilengkapi dengan berbagai metode bawaan berkinerja tinggi untuk memanipulasi konten:
- `mengandung(teks, target)`: Menghasilkan nilai `benar` jika teks target ditemukan dalam rentang string.
- `indeks_dari(teks, target)`: Menghasilkan indeks posisi awal ditemukannya substring (berbasis indeks 0), atau `-1` jika tidak ditemukan.
- `diawali_dengan(teks, awalan)`: Memeriksa apakah teks berawalan frasa tertentu.
- `diakhiri_dengan(teks, akhiran)`: Memeriksa apakah teks berakhir dengan frasa tertentu.

```widya
tetap judul = "Belajar Bahasa Pemrograman Widya-Lang Versi 1.0";

jika (mengandung(judul, "Widya-Lang")) {
    tetap pos = indeks_dari(judul, "Widya-Lang");
    cetak("Kata kunci Widya-Lang ditemukan pada indeks ke: " + teks(pos));
}

jika (diawali_dengan(judul, "Belajar")) {
    cetak("Ini adalah materi edukatif.");
}
```

### 27.3 Transformasi Karakter
Untuk keperluan normalisasi input formulir pengguna atau pencarian data yang kebal terhadap perbedaan huruf besar/kecil (*case-insensitive*), Widya-Lang menyediakan metode transformasi cepat:

```widya
tetap input_mentah = "   pENGGUNA_adMIN@kANTOR.GO.ID   ";

// Menghilangkan spasi liar di ujung teks
tetap bersih = pangkas(input_mentah);

// Mengubah menjadi huruf kecil seluruhnya
tetap email_normal = ke_huruf_kecil(bersih);

// Mengubah menjadi huruf kapital seluruhnya
tetap label_cetak = ke_huruf_besar(email_normal);

cetak("Hasil Normalisasi: [" + email_normal + "]");
cetak("Label Header:      [" + label_cetak + "]");
```

### 27.4 Pemecahan dan Penyatuan String
Operasi pemecahan (*splitting*) dan penyatuan (*joining*) adalah landasan pemrosesan berkas tabular (CSV) dan pemecahan kalimat menjadi token kata:

```widya
tetap data_antrian = "Budi,Siti,Agus,Dewi,Rahman";

// Memecah string menjadi larik kata
tetap daftar_nama = pecah(data_antrian, ",");
cetak("Jumlah antrian: " + teks(panjang(daftar_nama)));

// Menampilkan tiap elemen antrian
untuk nama dalam daftar_nama {
    cetak("- Pemohon: " + nama);
}

// Menyatukan kembali larik dengan pemisah baru
tetap laporan = gabung(daftar_nama, " -> ");
cetak("Urutan Pelayanan: " + laporan);
```

### 27.5 Format Rata Kanan, Rata Kiri, dan Tabel CLI
Saat membangun aplikasi baris perintah (*Command Line Interface*), perataan kolom teks sangat krusial agar informasi angka dan deskripsi barang tersusun rapi layaknya struk belanja:

```widya
fungsi ratakan_kiri(teks_asal, target_lebar) {
    variabel s = teks(teks_asal);
    selama (panjang(s) < target_lebar) {
        s = s + " ";
    }
    kembalikan s;
}

fungsi ratakan_kanan(teks_asal, target_lebar) {
    variabel s = teks(teks_asal);
    selama (panjang(s) < target_lebar) {
        s = " " + s;
    }
    kembalikan s;
}

// Menampilkan tabel harga barang
cetak("========================================");
cetak(ratakan_kiri("NAMA BARANG", 25) + ratakan_kanan("HARGA (RP)", 15));
cetak("----------------------------------------");
cetak(ratakan_kiri("Beras Organik 5kg", 25) + ratakan_kanan("78500", 15));
cetak(ratakan_kiri("Minyak Goreng 2L", 25) + ratakan_kanan("34000", 15));
cetak(ratakan_kiri("Gula Pasir 1kg", 25) + ratakan_kanan("17500", 15));
cetak("========================================");
```

### 27.6 Rangkuman Konsep Kunci Bab 27
- Manipulasi string mencakup pencarian, pemotongan, transformasi huruf besar/kecil, pemecahan (`pecah`), dan penyatuan (`gabung`).
- Standardisasi teks mencegah duplikasi dan mempermudah validasi formulir input.

### 27.7 Latihan Mandiri & Evaluasi Pemahaman
1. Mengapa pemangkasan spasi (`pangkas`) penting dilakukan sebelum memvalidasi alamat surel atau nama pengguna?
2. Bagaimana cara memecah kalimat panjang menjadi daftar kata individual?
3. **Tantangan Praktik**: Rancanglah fungsi `sensor_kata_kotor(kalimat, kata_terlarang)` yang menggantikan kata terlarang di dalam kalimat dengan tanda bintang `***`!

---

### 27.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 27 (Manipulasi Teks dan Format String)?**
- A. Membahas secara mendalam Operasi teks lanjutan
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `pecah() dan gabung()`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep pangkas spasi sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana perataan kolom tabel CLI memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 27?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah pangkas spasi?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan pecah() dan gabung()?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 27 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 27, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Operasi teks lanjutan secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
