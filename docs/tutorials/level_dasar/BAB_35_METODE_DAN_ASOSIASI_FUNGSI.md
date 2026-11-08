## Bab 35: Pemrograman Berorientasi Objek Bagian 2 - Metode dan Asosiasi Fungsi

### 35.1 Menyatukan Data dan Perilaku

```mermaid
graph TD
    A[Mulai Bab 35: Metode dan Asosiasi Fungsi] --> B[Pahami Konsep kata kunci diri (self)]
    B --> C[Terapkan Sintaksis implementasi tipe]
    C --> D[Uji Validitas Data & metode baru()]
    D --> E[Eksekusi Sistem Handal & Mandiri]
```

Struktur data murni (*data-only*) hanya menyimpan keadaan pasif. Untuk membuat model objek yang mandiri dan berdaya guna tinggi, kita mengasosiasikan fungsi (disebut **metode**) langsung ke dalam tipe struktur menggunakan blok `implementasi`.

### 35.2 Penggunaan Kata Kunci `diri` (*Self*)
Di dalam badan metode, kata kunci `diri` merepresentasikan instansi objek aktif yang sedang menjalankan metode tersebut:

```widya
struktur RekeningBank {
    pemilik,
    saldo,
    maksimum_tarik_harian
}

implementasi RekeningBank {
    // Fungsi konstruktor pembangun objek baru
    fungsi baru(nama_pemilik, saldo_awal) {
        kembalikan RekeningBank {
            pemilik: nama_pemilik,
            saldo: saldo_awal,
            maksimum_tarik_harian: 5000000
        };
    }
    
    // Metode informasi
    fungsi info(diri) {
        cetak("Akun [" + diri.pemilik + "] - Saldo Saat Ini: Rp " + teks(diri.saldo));
    }
    
    // Metode penyetoran uang
    fungsi setor(diri, jumlah) {
        jika (jumlah <= 0) {
            cetak("Nilai setoran harus lebih besar dari 0!");
            kembalikan salah;
        }
        diri.saldo = diri.saldo + jumlah;
        cetak("Setoran Rp " + teks(jumlah) + " diterima. Saldo sekarang: Rp " + teks(diri.saldo));
        kembalikan benar;
    }
    
    // Metode penarikan uang dengan validasi keamanan
    fungsi tarik(diri, jumlah) {
        jika (jumlah <= 0) {
            cetak("Nilai penarikan tidak sah!");
            kembalikan salah;
        }
        jika (jumlah > diri.maksimum_tarik_harian) {
            cetak("Penarikan melebihi batas limit harian!");
            kembalikan salah;
        }
        jika (jumlah > diri.saldo) {
            cetak("Saldo tidak mencukupi untuk penarikan sebesar Rp " + teks(jumlah));
            kembalikan salah;
        }
        
        diri.saldo = diri.saldo - jumlah;
        cetak("Penarikan Rp " + teks(jumlah) + " berhasil. Sisa saldo: Rp " + teks(diri.saldo));
        kembalikan benar;
    }
}
```

### 35.3 Menjalankan Metode Objek
Penggunaan metode dilakukan dengan sintaks tanda titik `objek.metode(...)`:

```widya
// Membuka akun tabungan baru
variabel tabungan_agus = RekeningBank::baru("Agus Setiawan", 1000000);

tabungan_agus.info();
tabungan_agus.setor(500000);
tabungan_agus.tarik(200000);
tabungan_agus.tarik(2000000); // Gagal: saldo tidak cukup
tabungan_agus.info();
```

### 35.4 Rangkuman Konsep Kunci Bab 35
- Blok `implementasi` mengasosiasikan fungsi ke dalam cetak biru struktur.
- Kata kunci `diri` memberikan akses mutasi ke properti internal objek.

### 35.5 Latihan Mandiri & Evaluasi Pemahaman
1. Mengapa enkapsulasi data di dalam metode lebih aman daripada memodifikasi bidang struktur secara langsung dari luar?
2. Bagaimana cara merancang fungsi konstruktor `baru(...)` pada tipe struktur?
3. **Tantangan Praktik**: Tambahkan metode `transfer(diri, rekening_tujuan, jumlah)` pada struktur `RekeningBank` yang memvalidasi kecukupan saldo sebelum memindahkan dana!

---

### 35.9 Latihan Soal Pilihan Ganda (10 Soal)

Pilihlah satu jawaban yang paling tepat (A, B, C, atau D) untuk menguji penguasaan materi bab ini:

**1. Apa fokus utama pembahasan dalam Bab 35 (Metode dan Asosiasi Fungsi)?**
- A. Membahas secara mendalam Penyatuan data dan perilaku
- B. Mengonfigurasi kabel jaringan lokal
- C. Memperbaiki motherboard komputer
- D. Menghapus database peladen

**2. Manakah kata kunci atau sintaksis yang menjadi inti materi bab ini?**
- A. Kata kunci `implementasi tipe`
- B. perintah format disk
- C. bahasa assembly x86
- D. tag HTML

**3. Mengapa konsep kata kunci diri (self) sangat penting dalam rekayasa perangkat lunak modern?**
- A. Menjamin efisiensi komputasi dan keterbacaan kode secara modular
- B. Hanya formalitas akademis
- C. Agar program berjalan lambat
- D. Mengurangi resolusi layar

**4. Dalam pengolahan data, bagaimana metode baru() memengaruhi performa memori?**
- A. Mengoptimalkan struktur alokasi data di RAM secara efisien
- B. Membuat memori penuh seketika
- C. Menghapus seluruh file disk
- D. Menghentikan kipas prosesor

**5. Manakah best practice yang dianjurkan dalam menerapkan materi Bab 35?**
- A. Menerapkan validasi data dan isolasi ruang lingkup yang bersih
- B. Menulis seluruh kode dalam satu baris panjang
- C. Mengabaikan penanganan kesalahan
- D. Menggunakan variabel global sembarangan

**6. Apa risiko yang timbul jika pemrogram mengabaikan kaidah kata kunci diri (self)?**
- A. Munculnya kutu logika atau kerentanan runtime saat data ekstrem masuk
- B. Komputer langsung terbakar
- C. Kecepatan internet naik dua kali lipat
- D. Keyboard tidak berfungsi

**7. Bagaimana kompilator Widya-Lang memproses instruksi yang berkaitan dengan implementasi tipe?**
- A. Divalidasi pada tahap analisis semantik dan diterjemahkan ke instruksi biner native
- B. Dikirimkan ke cloud pihak ketiga
- C. Diterjemahkan baris per baris secara lambat
- D. Diabaikan seluruhnya

**8. Di lingkungan industri, penerapan modul Bab 35 lazim ditemukan pada...**
- A. Sistem perbankan, backend peladen, aplikasi sains data, dan otomatisasi bisnis
- B. Hanya pada kalkulator saku sederhana
- C. Hanya pada game lawas 8-bit
- D. Tidak pernah digunakan di industri

**9. Fungsi pendukung apa yang paling sering dikombinasikan dengan materi bab ini?**
- A. Fungsi bawaan `cetak()`, `teks()`, atau asersi pengujian `pastikan()`
- B. Perintah mematikan komputer
- C. Fungsi memutar musik
- D. Fungsi mencetak kertas printer

**10. Setelah menuntaskan Bab 35, kompetensi apa yang diharapkan dikuasai oleh pembaca?**
- A. Mampu merancang, mengimplementasikan, dan menguji solusi berbasis Penyatuan data dan perilaku secara mandiri
- B. Mampu merakit kabel listrik tegangan tinggi
- C. Mampu mendesain grafis vektor 3D
- D. Mampu memasang antena radio
