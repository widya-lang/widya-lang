## Bab 21: Operasi Mutasi Larik: Menambah dan Mengubah Data

### 21.1 Kapasitas Dinamis dan Realokasi Memori

```mermaid
graph TD
    A[Mulai Bab 21: Operasi Mutasi Larik] --> B[Pahami Konsep mutasi per indeks]
    B --> C[Terapkan Sintaksis tambah(larik, item)]
    C --> D[Uji Validitas Data & struktur FIFO antrian]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Di dalam memori, larik dinamis (*dynamic array*) bekerja dengan mengalokasikan kapasitas penyangga (*buffer capacity*). Ketika kapasitas penuh dan fungsi `tambah(larik, item)` dipanggil, mesin runtime secara otomatis mengalokasikan blok memori baru yang berukuran dua kali lipat, menyalin data lama, lalu menambahkan elemen baru (*amortized $O(1)$*).

```widya
fungsi utama() {
    variabel antrian_bank = []; // Larik kosong mula-mula
    
    tambah(antrian_bank, "Nasabah 001 - Budi");
    tambah(antrian_bank, "Nasabah 002 - Siti");
    tambah(antrian_bank, "Nasabah 003 - Joko");
    
    cetak("Jumlah nasabah dalam antrian: " + teks(panjang(antrian_bank)));
    
    // Menampilkan seluruh antrian
    untuk orang dalam antrian_bank {
        cetak("- " + orang);
    }
}
```

### 21.2 Mengubah Elemen pada Indeks Tertentu
Elemen larik dapat diganti nilainya dengan menunjuk indeks yang diinginkan:

```widya
fungsi utama() {
    variabel daftar_nilai = [70, 80, 65, 90];
    
    // Siswa ketiga (indeks 2) mengikuti ujian perbaikan (remedial)
    daftar_nilai[2] = 85;
    
    cetak("Nilai setelah remedial: " + teks(daftar_nilai[2]));
}
```

### 21.3 Contoh Kode Praktik: Simulasi Struktur Data Antrian (FIFO)
Program ini mensimulasikan pemanggilan nomor antrian loket layanan masyarakat:

```widya
// Berkas: bab21_antrian_loket.widya

fungsi utama() {
    variabel loket = [];
    tambah(loket, "Antrian A-01: Perpanjangan KTP");
    tambah(loket, "Antrian A-02: Akta Kelahiran");
    tambah(loket, "Antrian A-03: Kartu Keluarga");
    
    cetak("=== SISTEM PELAYANAN DISDUKCAPIL ===");
    cetak("Total antrian aktif: " + teks(panjang(loket)));
    
    variabel i = 0;
    selama (i < panjang(loket)) {
        cetak(">> Memanggil sekarang: " + loket[i]);
        i += 1;
    }
    cetak("Seluruh antrian telah selesai dilayani.");
}
```

### 21.4 Rangkuman Konsep Kunci Bab 21
- `tambah(larik, item)` menambahkan elemen di akhir koleksi secara efisien.
- Mutasi nilai per elemen dilakukan melalui sintaksis `larik[indeks] = nilai_baru`.

### 21.5 Latihan Mandiri & Evaluasi Pemahaman
1. Bagaimana cara memperbarui elemen terakhir dalam larik dinamis?
2. Apa perbedaan perilaku antara penugasan ulang seluruh larik dan pembaruan elemen perorangan?
3. **Tantangan Praktik**: Rancanglah simulator keranjang belanja `keranjang.widya` di mana pengguna dapat menambahkan nama barang baru secara dinamis hingga mencapai kapasitas 5 barang!

---

### 21.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 21 (Operasi Mutasi Larik)?**
- A. Membahas secara mendalam Kapasitas dinamis
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `tambah(larik, item)`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep mutasi per indeks sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana struktur FIFO antrian memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 21?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah mutasi per indeks?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan tambah(larik, item)?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 21 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 21, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Kapasitas dinamis secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
