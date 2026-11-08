## Bab 20: Pengantar Struktur Data Larik (*Array*)

### 20.1 Teori Alokasi Memori Kontigu

```mermaid
graph TD
    A[Mulai Bab 20: Larik Array Dasar] --> B[Pahami Konsep panjang(larik)]
    B --> C[Terapkan Sintaksis indeks berbasis nol]
    C --> D[Uji Validitas Data & alokasi buffer memori]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Hingga saat ini, setiap variabel yang kita pelajari hanya mampu menampung satu nilai tunggal. Jika Anda diminta mengelola data 1.000 nama siswa di sebuah sekolah, membuat variabel `siswa_1`, `siswa_2` sampai `siswa_1000` tentu sangat tidak efisien.

Untuk memecahkan persoalan data berkelompok, hadir struktur data **Larik (*Array*)**—yaitu kumpulan nilai yang disimpan secara berurutan dalam memori berindeks numerik. Di tingkat perangkat keras, elemen-elemen larik dialokasikan pada blok memori yang bersebelahan (*contiguous memory block*), sehingga prosesor dapat mengakses elemen mana pun dengan waktu konstan $O(1)$ menggunakan rumus:
$$	ext{Alamat}(i) = 	ext{AlamatDasar} + (i 	imes 	ext{UkuranElemen})$$

```
+-------------------------------------------------------+
|              STRUKTUR DATA LARIK DI RAM               |
+-------------------------------------------------------+
|  Indeks  |   0    |    1    |    2    |   3    |  4   |
|  Nilai   |  "Aceh"| "Padang"| "Medan" | "Riau" |"Jambi|
+-------------------------------------------------------+
```

### 20.2 Deklarasi dan Akses Elemen Larik
Larik dideklarasikan menggunakan kurung siku `[...]`, dengan setiap elemen dipisahkan koma:

```widya
fungsi utama() {
    // Membuat larik berisi daftar nama provinsi
    tetap pulau_jawa = ["Banten", "DKI Jakarta", "Jawa Barat", "Jawa Tengah", "DIY", "Jawa Timur"];
    
    // Mengakses elemen berdasarkan indeks (mulai dari 0)
    cetak("Provinsi Pertama: " + pulau_jawa[0]);
    cetak("Ibu Kota Negara : " + pulau_jawa[1]);
    
    // Mengetahui jumlah total elemen
    tetap total_provinsi = panjang(pulau_jawa);
    cetak("Total Wilayah di Pulau Jawa: " + teks(total_provinsi));
    
    // Mengakses elemen terakhir
    tetap provinsi_terakhir = pulau_jawa[total_provinsi - 1];
    cetak("Ujung Timur: " + provinsi_terakhir);
}
```

### 20.3 Contoh Kode Praktik: Statistik Nilai Ulangan Siswa
Program ini menghitung total nilai, rata-rata kelas, dan mencari nilai tertinggi dalam larik:

```widya
// Berkas: bab20_statistik_nilai.widya

fungsi utama() {
    tetap daftar_nilai = [75, 88, 92, 64, 80, 95, 70];
    tetap jumlah_siswa = panjang(daftar_nilai);
    
    variabel total = 0;
    variabel nilai_tertinggi = daftar_nilai[0];
    
    untuk n dalam daftar_nilai {
        total = total + n;
        jika (n > nilai_tertinggi) {
            nilai_tertinggi = n;
        }
    }
    
    tetap rata_rata = total / jumlah_siswa;
    
    cetak("STATISTIK NILAI ULANGAN MATEMATIKA:");
    cetak("Jumlah Siswa Peserta : " + teks(jumlah_siswa));
    cetak("Total Nilai Terkumpul: " + teks(total));
    cetak("Rata-rata Nilai Kelas: " + teks(rata_rata));
    cetak("Nilai Ujian Tertinggi: " + teks(nilai_tertinggi));
}
```

### 20.4 Kesalahan Pemula dan Panduan Solusi
- **Galat Indeks di Luar Batas (*Index Out of Bounds*)**:
  Mengakses `larik[panjang(larik)]` akan memicu kegagalan karena indeks terakhir selalu berada pada `panjang - 1`.

### 20.5 Rangkuman Konsep Kunci Bab 20
- Larik menampung koleksi terurut pada memori kontigu.
- Pengindeksan berbasis nol memungkinkan kalkulasi offset alamat RAM secara instan $O(1)$.
- Fungsi bawaan `panjang(...)` mengembalikan total elemen.

### 20.6 Latihan Mandiri & Evaluasi Pemahaman
1. Mengapa alokasi memori kontigu pada larik membuat CPU cache bekerja sangat efisien?
2. Bagaimana cara mengakses elemen tengah dari sebuah larik ganjil?
3. **Tantangan Praktik**: Rancanglah program `statistik_suhu.widya` yang menyimpan catatan suhu udara harian selama 7 hari dalam larik, lalu hitung total dan suhu rata-ratanya!

---

### 20.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 20 (Larik Array Dasar)?**
- A. Membahas secara mendalam Struktur data kontigu
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `indeks berbasis nol`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep panjang(larik) sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana alokasi buffer memori memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 20?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah panjang(larik)?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan indeks berbasis nol?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 20 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 20, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Struktur data kontigu secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
