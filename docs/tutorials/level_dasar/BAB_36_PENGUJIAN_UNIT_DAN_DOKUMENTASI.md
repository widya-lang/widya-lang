## Bab 36: Pengujian Unit (Unit Testing) dan Dokumentasi Kode

### 36.1 Prinsip Pengujian Otomatis

```mermaid
graph TD
    A[Mulai Bab 36: Pengujian Unit dan Dokumentasi] --> B[Pahami Konsep docstrings ///]
    B --> C[Terapkan Sintaksis pastikan() dan pastikan_sama()]
    C --> D[Uji Validitas Data & automasi CI/CD]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Perangkat lunak berskala industri tidak boleh hanya diuji secara manual dengan mengklik tombol antarmuka atau melihat keluaran layar sementara. Setiap modul harus dilengkapi dengan **Pengujian Unit (*Unit Test*)** mandiri yang dapat dijalankan secara instan oleh sistem integrasi berkelanjutan (*Continuous Integration* / CI).

### 36.2 Asersi Kebenaran dengan `pastikan` dan `pastikan_sama`
Widya-Lang menyediakan makro asersi bawaan:
- `pastikan(kondisi, pesan_galat)`: Memvalidasi bahwa kondisi bernilai `benar`.
- `pastikan_sama(nilai_aktual, nilai_ekspektasi, pesan_galat)`: Memvalidasi bahwa nilai keluaran program sama persis dengan target yang diharapkan.

```widya
// Fungsi yang akan diuji
fungsi hitung_diskon_member(total_belanja, adalah_member) {
    jika (bukan adalah_member) {
        kembalikan 0;
    }
    jika (total_belanja >= 500000) {
        kembalikan total_belanja * 0.15; // Diskon 15%
    }
    jika (total_belanja >= 100000) {
        kembalikan total_belanja * 0.10; // Diskon 10%
    }
    kembalikan total_belanja * 0.05;     // Diskon 5%
}

// Blok Pengujian Unit Otomatis
fungsi uji_modul_diskon() {
    cetak("Menjalankan rangkaian uji hitung_diskon_member...");
    
    // Kasus 1: Bukan member tidak dapat diskon
    pastikan_sama(hitung_diskon_member(200000, salah), 0, "Non-member harus menerima diskon 0");
    
    // Kasus 2: Member belanja 600rb dapat 15% (90rb)
    pastikan_sama(hitung_diskon_member(600000, benar), 90000, "Belanja 600rb member harus dapat diskon 90rb");
    
    // Kasus 3: Member belanja 200rb dapat 10% (20rb)
    pastikan_sama(hitung_diskon_member(200000, benar), 20000, "Belanja 200rb member harus dapat diskon 20rb");
    
    cetak("SELURUH PENGUJIAN UNIT BERHASIL LOLOS 100%! ✔");
}

uji_modul_diskon();
```

### 36.3 Standar Dokumentasi Kode (*Docstrings*)
Widya-Lang menganut budaya kode bersih dan terdokumentasi rapi. Komentar dokumentasi diawali dengan garis miring tiga kali `///`:

```widya
/// Menghitung luas permukaan silinder tertutup.
///
/// # Parameter
/// * `jari_jari`: Radius alas silinder dalam satuan meter (harus bernilai positif).
/// * `tinggi`: Ketinggian tabung silinder dalam satuan meter.
///
/// # Contoh Penggunaan
/// ```widya
/// tetap luas = hitung_luas_silinder(7.0, 10.0);
/// ```
fungsi hitung_luas_silinder(jari_jari, tinggi) {
    tetap pi = 3.1415926535;
    tetap luas_alas = pi * (jari_jari ^ 2);
    tetap keliling = 2 * pi * jari_jari;
    tetap luas_selimut = keliling * tinggi;
    
    kembalikan (2 * luas_alas) + luas_selimut;
}
```

### 36.4 Rangkuman Konsep Kunci Bab 36
- Pengujian unit otomatis memverifikasi kebenaran fungsi terhadap regresi kode.
- `pastikan_sama` menguji kesesuaian nilai aktual dengan ekspektasi.
- Dokumentasi `///` diekstrak secara otomatis menjadi dokumen teknis profesional.

### 36.5 Latihan Mandiri & Evaluasi Pemahaman
1. Mengapa pengujian unit harus ditulis sedemikian rupa agar dapat berjalan mandiri tanpa campur tangan manusia?
2. Bagaimana asersi otomatis mendeteksi kutu (*bug*) sebelum kode dirilis ke lingkungan produksi?
3. **Tantangan Praktik**: Tuliskan paket uji unit mandiri untuk fungsi konversi waktu atau kalkulator fisika yang Anda buat di bab sebelumnya!

---

### 36.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 36 (Pengujian Unit dan Dokumentasi)?**
- A. Membahas secara mendalam Verifikasi kualitas perangkat lunak
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `pastikan() dan pastikan_sama()`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep docstrings /// sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana automasi CI/CD memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 36?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah docstrings ///?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan pastikan() dan pastikan_sama()?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 36 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 36, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Verifikasi kualitas perangkat lunak secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
