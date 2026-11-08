## Lampiran B: Panduan Pesan Galat dan Pemecahan Masalah (*Troubleshooting*)

Ketika kompilator atau runtime Widya-Lang menemukan ketidaksesuaian instruksi, sistem akan mencetak pesan diagnostik berdaulat dalam Bahasa Indonesia. Rujukan di bawah ini membantu Anda memahami dan memperbaikinya secara cepat:

| Pesan Galat Kompilator | Kemungkinan Akar Masalah | Solusi & Rekomendasi Tindakan |
| :--- | :--- | :--- |
| `Galat Sintaksis: Kurung kurawal buka '{' tidak ditemukan penutupnya` | Ada blok kode fungsi atau percabangan yang lupa ditutup dengan tanda `}`. | Periksa indentasi dan pastikan jumlah pasang `{` dan `}` tepat seimbang. |
| `Galat Nama: Variabel 'xyz' belum dideklarasikan` | Menggunakan nama pengenal variabel yang belum dibuat atau salah eja (*typo*). | Deklarasikan terlebih dahulu dengan `variabel xyz = ...;` sebelum digunakan. |
| `Galat Mutasi: Tidak dapat mengubah nilai konstanta 'tetap'` | Mencoba mengubah nilai variabel yang dideklarasikan dengan kata kunci `tetap`. | Ganti deklarasi menjadi `variabel` jika nilai memang dimaksudkan untuk berubah. |
| `Galat Tipe: Operator '+' tidak dapat menggabungkan tipe Angka dan Larik` | Mencoba melakukan operasi aritmetika pada dua tipe data yang tidak sepadan. | Gunakan fungsi konversi tipe eksplisit seperti `teks(...)` atau `angka(...)`. |
| `Galat Indeks: Indeks 5 berada di luar batas larik (panjang 3)` | Mengakses indeks larik yang melebihi jumlah elemen yang tersedia. | Pastikan pengujian indeks tidak melebihi `panjang(larik) - 1`. |
| `Galat I/O: Berkas tidak ditemukan pada jalur yang ditentukan` | Fungsi `baca_berkas` memanggil berkas yang tidak ada di direktori kerja. | Pastikan jalur berkas tepat atau gunakan pengujian `berkas_ada(jalur)` terlebih dahulu. |
