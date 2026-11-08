## Bab 6: Sistem Tipe Data Primitif Bagian 1 - Angka Numerik

### 6.1 Karakteristik Tipe Numerik dalam Arsitektur Komputer

```mermaid
graph TD
    A[Mulai Bab 6: Tipe Primitif Angka] --> B[Pahami Konsep bilangan bulat]
    B --> C[Terapkan Sintaksis angka()]
    C --> D[Uji Validitas Data & pecahan desimal]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Seluruh kalkulasi ilmiah, pemrosesan transaksi perbankan, grafik rendering video game, hingga kecerdasan buatan bermuara pada manipulasi angka numerik. Komputer mengolah dua kategori angka numerik yang memiliki karakteristik representasi bit berbeda secara mendasar:
1. **Bilangan Bulat (*Integer*)**:
   Bilangan bulat tanpa komponen desimal, seperti `-450`, `0`, `42`, `1000000`. Di dalam Widya-Lang, bilangan bulat secara bawaan disimpan sebagai **Integer 64-bit Bertanda (*Signed 64-bit Integer* / `i64`)**.
2. **Bilangan Titik Mengambang (*Floating-Point*)**:
   Bilangan dengan komponen pecahan desimal, seperti `3.14159`, `-0.0075`, `98.6`. Disimpan menggunakan standar internasional **IEEE 754 Presisi Ganda 64-bit (*Double Precision Float* / `f64`)**.

```
+---------------------------------------------------------------+
|             REPRESENTASI BIT BILANGAN BULAT 64-BIT            |
+---------------------------------------------------------------+
| [Bit 63: Tanda (+/-)] | [Bit 62 s.d. Bit 0: Besaran Nilai]    |
| Rentang Minimum: -9.223.372.036.854.775.808                   |
| Rentang Maksimum: +9.223.372.036.854.775.807                  |
+---------------------------------------------------------------+
```

### 6.2 Bahaya Peluapan Batas (*Integer Overflow*)
Dalam bahasa pemrograman lawas seperti C atau C++, jika sebuah angka melebihi batas maksimum representasi bitnya, nilai akan meluap kembali ke angka negatif terkecil tanpa peringatan (*Integer Overflow*). Fenomena ini pernah memicu kegagalan peluncuran roket luar angkasa Ariane 5 pada tahun 1996 yang meledak 37 detik setelah lepas landas.

Widya-Lang mengadopsi sistem perlindungan aktif: kompilator dan mesin runtime secara konstan memantau batas operasi aritmetika dan memberikan peringatan keamanan diagnostik jika terdeteksi peluapan kapasitas numerik.

### 6.3 Pemisah Digit Visual Garis Bawah (`_`)
Ketika mengolah data keuangan berskala nasional bernilai ratusan triliun rupiah, mata pemrogram sangat rentan salah menghitung jumlah nol. Widya-Lang mendukung penyisipan garis bawah sebagai pembatas ribuan visual yang diabaikan oleh mesin kompilator:

```widya
fungsi utama() {
    tetap pagu_anggaran_pendidikan = 660_800_000_000_000; // Rp 660,8 Triliun
    tetap jumlah_siswa_nasional = 53_400_000; // 53,4 Juta Siswa
    
    tetap alokasi_per_siswa = pagu_anggaran_pendidikan / jumlah_siswa_nasional;
    cetak("Alokasi Rata-rata per Siswa: Rp " + teks(alokasi_per_siswa));
}
```

### 6.4 Format Basis Alternatif: Heksadesimal dan Biner
Untuk pemrograman tingkat rendah, manipulasi register mikrokontroler, atau pengaturan kode warna RGB, Widya-Lang menyediakan notasi basis biner dan heksadesimal:
- **Literal Biner (`0b...`)**: Basis 2 (simbol `0` dan `1`). Contoh: `0b1100_1010`.
- **Literal Heksadesimal (`0x...`)**: Basis 16 (simbol `0-9` dan `A-F`). Contoh: `0xFF_57_33`.

### 6.5 Konversi Eksplisit String ke Angka
Data yang diterima dari formulir pengguna terminal atau pembacaan berkas teks disk selalu bertipe teks string. Untuk melakukan operasi penjumlahan matematika, string tersebut harus dikonversi secara eksplisit menjadi angka menggunakan fungsi bawaan `angka(...)`:

```widya
fungsi utama() {
    tetap masukan_str = "125000";
    tetap kuantitas = 4;
    
    // Konversi eksplisit teks ke angka
    tetap nominal = angka(masukan_str);
    tetap total = nominal * kuantitas;
    
    cetak("Total Pembayaran: Rp " + teks(total));
}
```

### 6.6 Contoh Kode Praktik 1: Perhitungan Amortisasi Pinjaman Bank
Program berikut mengilustrasikan perhitungan angsuran pokok dan bunga pinjaman:

```widya
// Berkas: bab06_angsuran_bank.widya

fungsi utama() {
    tetap pokok_pinjaman = 120_000_000.0; // Rp 120 Juta
    tetap suku_bunga_tahunan = 0.085; // 8.5% per tahun
    tetap tenor_bulan = 24.0; // 2 tahun
    
    tetap angsuran_pokok_per_bulan = pokok_pinjaman / tenor_bulan;
    tetap bunga_per_bulan = (pokok_pinjaman * suku_bunga_tahunan) / 12.0;
    tetap total_cicilan_per_bulan = angsuran_pokok_per_bulan + bunga_per_bulan;
    
    cetak("SIMULASI PINJAMAN MODAL USAHA MIKRO:");
    cetak("- Pokok Pinjaman  : Rp " + teks(pokok_pinjaman));
    cetak("- Bunga Efektif   : " + teks(suku_bunga_tahunan * 100.0) + " % per tahun");
    cetak("- Tenor Cicilan   : " + teks(tenor_bulan) + " bulan");
    cetak("--------------------------------------------------");
    cetak("Cicilan Pokok/Bln : Rp " + teks(angsuran_pokok_per_bulan));
    cetak("Porsi Bunga/Bln   : Rp " + teks(bunga_per_bulan));
    cetak("TOTAL CICILAN/BLN : Rp " + teks(total_cicilan_per_bulan));
}
```

### 6.7 Contoh Kode Praktik 2: Kalkulator Fisika Mekanika
Program berikut menghitung lintasan parabola peluru meriam (gerak peluru):

```widya
// Berkas: bab06_gerak_peluru.widya

fungsi utama() {
    tetap kecepatan_awal = 150.0; // m/s
    tetap gravitasi = 9.80665; // m/s²
    tetap waktu_detik = 5.0; // waktu pantau
    
    // Asumsi sudut elevasi 45 derajat (sin = cos = 0.7071)
    tetap vx = kecepatan_awal * 0.70710678;
    tetap vy0 = kecepatan_awal * 0.70710678;
    
    tetap jarak_x = vx * waktu_detik;
    tetap ketinggian_y = (vy0 * waktu_detik) - (0.5 * gravitasi * (waktu_detik ^ 2));
    
    cetak("SIMULASI BALISTIK MERIAM PADA T = " + teks(waktu_detik) + " DETIK:");
    cetak("- Posisi Horizontal (X) : " + teks(jarak_x) + " meter");
    cetak("- Ketinggian Vertikal (Y): " + teks(ketinggian_y) + " meter");
}
```

### 6.8 Kesalahan Pemula dan Solusi
- **Pembagian dengan Angka Nol**:
  - Operasi seperti `100 / 0` akan memicu sinyal interupsi keselamatan komputasi. Selalu lakukan validasi penyebut sebelum melakukan pembagian.
- **Masalah Presisi Pecahan Desimal**:
  - Dalam standar biner IEEE 754, pecahan seperti `0.1 + 0.2` menghasilkan nilai `0.30000000000000004`. Hal ini wajar dalam seluruh arsitektur komputer dunia; untuk akurasi uang perbankan yang mutlak, gunakan representasi satuan sen integer bulat terkecil.

### 6.9 Rangkuman Konsep Kunci Bab 6
- Integer 64-bit menampung angka bulat hingga 9 quintillion.
- Titik mengambang 64-bit memproses pecahan desimal presisi tinggi.
- Garis bawah `_` mempermudah pembacaan nominal jutaan dan triliunan.
- Konversi teks ke angka dilakukan dengan fungsi bawaan `angka(...)`.

### 6.10 Latihan Mandiri & Evaluasi Pemahaman
1. Mengapa nilai desimal `0.1` tidak dapat direpresentasikan secara biner mutlak 100% tepat dalam standar IEEE 754?
2. Bagaimana cara menuliskan angka biner 255 dalam sintaksis Widya-Lang?
3. **Tantangan Praktik**: Buatlah program `kalkulator_bmi.widya` yang menghitung Indeks Massa Tubuh (BMI) dengan rumus $BMI = 
rac{berat\_kg}{(tinggi\_m)^2}$, lalu tampilkan hasil desimalnya di terminal!

---

### 6.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 6 (Tipe Primitif Angka)?**
- A. Membahas secara mendalam Integer 64-bit dan Float IEEE 754
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `angka()`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep bilangan bulat sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana pecahan desimal memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 6?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah bilangan bulat?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan angka()?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 6 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 6, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Integer 64-bit dan Float IEEE 754 secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
