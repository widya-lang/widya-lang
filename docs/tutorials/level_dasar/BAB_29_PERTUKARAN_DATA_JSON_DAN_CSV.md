## Bab 29: Format Pertukaran Data JSON dan CSV

### 29.1 Urgensi Standar Pertukaran Data

```mermaid
graph TD
    A[Mulai Bab 29: Pertukaran Data JSON dan CSV] --> B[Pahami Konsep parsing data CSV]
    B --> C[Terapkan Sintaksis ke_json() dan dari_json()]
    C --> D[Uji Validitas Data & integrasi API web]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Ketika aplikasi Widya-Lang perlu berkomunikasi dengan basis data eksternal, web service REST API, atau perangkat lunak perkantoran (seperti spreadsheet Excel dan Calc), data dalam memori harus diubah menjadi format standar yang dapat dipahami oleh beragam sistem operasi dan bahasa pemrograman.

### 29.2 Pengolahan Berkas CSV (Comma-Separated Values)
CSV adalah format tabular paling populer di lingkungan perkantoran dan sains data karena kesederhanaan strukturnya.

```widya
// Membangun dokumen CSV dari larik data
variabel baris_csv = [];
tambah(baris_csv, "SKU,NamaProduk,Kategori,HargaSatuan,Stok");
tambah(baris_csv, "PRD-001,Kopi Luwak Organik 250g,Minuman,125000,45");
tambah(baris_csv, "PRD-002,Gula Aren Asli 500g,Bumbu,32000,80");
tambah(baris_csv, "PRD-003,Teh Hitam Melati 100g,Minuman,18500,120");

tetap konten_lengkap = gabung(baris_csv, "\n");
tulis_berkas("katalog_produk.csv", konten_lengkap);
cetak("Katalog CSV berhasil diekspor.");
```

Untuk membaca kembali dan memproses data CSV:

```widya
tetap mentah = baca_berkas("katalog_produk.csv");
tetap baris = pecah(mentah, "\n");

// Mengabaikan baris header pertama
variabel idx = 1;
variabel total_nilai_aset = 0;

selama (idx < panjang(baris)) {
    tetap baris_aktif = pangkas(baris[idx]);
    jika (panjang(baris_aktif) > 0) {
        tetap kolom = pecah(baris_aktif, ",");
        tetap sku = kolom[0];
        tetap nama = kolom[1];
        tetap harga = angka(kolom[3]);
        tetap stok = angka(kolom[4]);
        
        tetap subtotal = harga * stok;
        total_nilai_aset = total_nilai_aset + subtotal;
        
        cetak("Produk [" + sku + "] " + nama + " -> Nilai Aset: Rp " + teks(subtotal));
    }
    idx = idx + 1;
}

cetak("Total Seluruh Valuasi Aset Gudang: Rp " + teks(total_nilai_aset));
```

### 29.3 Serialisasi dan Deserialisasi JSON
JSON (*JavaScript Object Notation*) adalah standar baku pertukaran data API di internet. Widya-Lang mendukung konversi dua arah antara struktur kamus/larik dengan teks JSON:

```widya
// Menyiapkan struktur data kompleks dalam kamus
variabel profil = {};
profil["nama_sekolah"] = "SMK Negeri 1 Rekayasa Perangkat Lunak";
profil["tahun_berdiri"] = 1998;
profil["akreditasi"] = "A";
profil["jurusan_unggulan"] = ["Teknik Komputer", "Rekayasa Perangkat Lunak", "Kecerdasan Buatan"];

// Serialisasi Kamus ke String JSON
tetap json_teks = ke_json(profil);
cetak("Hasil Serialisasi JSON:");
cetak(json_teks);

// Menyimpan ke berkas JSON
tulis_berkas("profil_sekolah.json", json_teks);

// Deserialisasi dari String JSON kembali ke Objek/Kamus
tetap json_dari_disk = baca_berkas("profil_sekolah.json");
tetap objek_kembali = dari_json(json_dari_disk);

cetak("Data Berhasil Dimuat:");
cetak("Nama Lembaga: " + objek_kembali["nama_sekolah"]);
cetak("Akreditasi:   " + objek_kembali["akreditasi"]);
```

### 29.4 Rangkuman Konsep Kunci Bab 29
- CSV digunakan untuk pertukaran data tabular spreadsheet.
- JSON digunakan untuk struktur data bertingkat dan komunikasi API web.
- `ke_json` dan `dari_json` menangani serialisasi dua arah secara transparan.

### 29.5 Latihan Mandiri & Evaluasi Pemahaman
1. Mengapa JSON lebih disukai dibandingkan format XML dalam pengembangan web API modern?
2. Bagaimana cara menangani nilai kolom CSV yang mengandung tanda koma di dalam teksnya?
3. **Tantangan Praktik**: Buatlah program yang mengonversi berkas data nilai siswa berekstensi `.csv` menjadi berkas terstruktur berekstensi `.json`!

---

### 29.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 29 (Pertukaran Data JSON dan CSV)?**
- A. Membahas secara mendalam Format data standar
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `ke_json() dan dari_json()`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep parsing data CSV sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana integrasi API web memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 29?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah parsing data CSV?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan ke_json() dan dari_json()?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 29 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 29, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Format data standar secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
