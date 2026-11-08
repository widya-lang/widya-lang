## Bab 25: Pengantar Struktur Data Kamus (*Map* / Key-Value)

### 25.1 Mengapa Memerlukan Kamus?

```mermaid
graph TD
    A[Mulai Bab 25: Kamus Map Kunci-Nilai] --> B[Pahami Konsep pencarian instan O(1)]
    B --> C[Terapkan Sintaksis kamus = {}]
    C --> D[Uji Validitas Data & kunci string unik]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Dalam larik, data dicari menggunakan indeks nomor urut integer (`0`, `1`, `2`). Namun dalam kehidupan nyata, kita sering ingin mencari informasi berdasarkan **Nama Pengenal Teks (*Key*)**—seperti mencari harga beras berdasarkan nama barang `"Beras Rojolele"`, atau mencari nomor telepon berdasarkan nama orang `"Budi"`.

Struktur data ini dinamakan **Kamus (*Map* / Dictionary)**, yaitu wadah yang menyimpan pasangan **Kunci dan Nilai (*Key-Value Pairs*)**.

```
+-------------------------------------------------------------+
|                     STRUKTUR DATA KAMUS                     |
+-------------------------------------------------------------+
|    KUNCI PENCARIAN (Key)    |       NILAI DATA (Value)      |
+-----------------------------+-------------------------------+
|    "Beras 5kg"              |       Rp 75.000               |
|    "Minyak Goreng 2L"       |       Rp 34.000               |
|    "Gula Pasir 1kg"         |       Rp 17.500               |
+-----------------------------+-------------------------------+
```

### 25.2 Deklarasi dan Operasi Kamus
Kamus dideklarasikan menggunakan kurung kurawal `{}`:

```widya
fungsi utama() {
    // 1. Membuat kamus katalog harga
    variabel harga_sembako = {};
    
    // 2. Memasukkan entri kunci dan nilai
    harga_sembako["Beras_5kg"] = 75000;
    harga_sembako["Minyak_2L"] = 34000;
    harga_sembako["Gula_1kg"] = 17500;
    harga_sembako["Telur_1kg"] = 28000;
    
    // 3. Mengambil nilai berdasarkan kunci
    cetak("Harga Beras: Rp " + teks(harga_sembako["Beras_5kg"]));
    cetak("Harga Minyak: Rp " + teks(harga_sembako["Minyak_2L"]));
    
    // 4. Memperbarui nilai jika terjadi inflasi/perubahan harga
    harga_sembako["Telur_1kg"] = 29500;
    cetak("Harga Telur Terkini: Rp " + teks(harga_sembako["Telur_1kg"]));
}
```

### 25.3 Rangkuman Konsep Kunci Bab 25
- Kamus memetakan kunci string unik ke suatu nilai data.
- Pencarian data di dalam kamus berlangsung instan $O(1)$ melalui mekanisme hashing.

### 25.4 Latihan Mandiri & Evaluasi Pemahaman
1. Jelaskan perbedaan mendasar antara cara pengindeksan Larik dan Kamus!
2. Apa yang terjadi jika kita memasukkan nilai baru pada kunci yang sudah ada di dalam Kamus?
3. **Tantangan Praktik**: Buatlah kamus konversi nilai huruf ke angka mutu IPK (A -> 4.0, B -> 3.0, C -> 2.0, D -> 1.0, E -> 0.0), lalu cetak nilai mutu untuk mata kuliah "Algoritma"!

---

### 25.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 25 (Kamus Map Kunci-Nilai)?**
- A. Membahas secara mendalam Asosiasi tabel hash
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `kamus = {}`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep pencarian instan O(1) sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana kunci string unik memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 25?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah pencarian instan O(1)?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan kamus = {}?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 25 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 25, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Asosiasi tabel hash secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
