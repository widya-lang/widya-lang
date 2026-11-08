## Lampiran B: Panduan Pesan Galat Kompiler, Lifetimes Diagnostics, & Troubleshooting Borrow Checker

Sistem penganalisis pinjaman (*Borrow Checker*) dan verifikator siklus hidup (*Lifetimes Analyzer*) pada Widya-Lang dirancang untuk mencegah seluruh bencana korupsi memori sebelum biner diproduksi. Lampiran ini memuat katalog kode galat kompilasi resmi, penyebab arsitektural, dan resep perbaikan standar industri.

---

### B.1 Katalog Kode Galat Kompilator Widya-Lang

#### 1. Galat E0501: `cannot borrow as mutable more than once at a time`
- **Penyebab Arsitektural**: Terjadi pelanggaran aturan eksklusivitas mutabilitas (*Aliasing XOR Mutability*). Program mencoba membuat dua peminjaman mutabel (`&mut T`) aktif ke lokasi memori yang sama secara bersamaan.
- **Dampak Jika Lolos**: Kerusakan integritas data akibat *data-race* atau modifikasi simultan.
- **Resep Solusi Rekayasa**: Batasi lingkup peminjaman pertama menggunakan kurung kurawal `{ ... }` atau selesaikan penggunaan peminjaman pertama sebelum memulai peminjaman kedua.

```widya
// SALAH (Memicu Galat E0501):
misal mut data = [10, 20];
misal ref1 = &mut data;
misal ref2 = &mut data; // GALAT: data sudah dipinjam secara mutabel oleh ref1!

// BENAR (Solusi Rekayasa):
misal mut data = [10, 20];
{
    misal ref1 = &mut data;
    ref1.tambah(30);
} // Lingkup ref1 berakhir di sini, kunci mutabel dilepaskan
misal ref2 = &mut data; // AMAN
```

---

#### 2. Galat E0502: `cannot borrow as mutable because it is also borrowed as immutable`
- **Penyebab Arsitektural**: Terjadi pelanggaran hukum pembaca-penulis. Sebuah data sedang dipinjam secara hanya-baca (`&T`), namun bagian kode lain mencoba membuat peminjaman mutabel (`&mut T`) yang berpotensi mengubah data yang sedang dibaca.
- **Resep Solusi Rekayasa**: Pastikan seluruh referensi imutabel selesai digunakan sebelum melakukan mutasi data.

---

#### 3. Galat E0382: `use of moved value`
- **Penyebab Arsitektural**: Program mencoba membaca atau menggunakan kembali variabel yang kepemilikannya telah dipindahkan (*moved*) ke variabel lain atau dilewatkan ke fungsi lain tanpa melalui peminjaman referensi.
- **Resep Solusi Rekayasa**: Gunakan peminjaman referensi (`&data`) alih-alih memindahkan kepemilikan penuh jika variabel asal masih dibutuhkan, atau gunakan kloning eksplisit (`data.klon()`).

---

#### 4. Galat E0106: `missing lifetime specifier`
- **Penyebab Arsitektural**: Sebuah fungsi menerima dua atau lebih parameter referensi dan mengembalikan sebuah referensi, namun kompilator tidak dapat menentukan secara pasti referensi mana yang menjadi dasar masa hidup kembalian (*Lifetime Elision Failure*).
- **Resep Solusi Rekayasa**: Tambahkan anotasi siklus hidup generik eksplisit seperti `<'a>` pada tanda tangan fungsi:
  ```widya
  fungsi pilih<'a>(x: &'a Teks, y: &'a Teks) -> &'a Teks { ... }
  ```

---

#### 5. Galat E0597: `value does not live long enough` (Dangling Reference)
- **Penyebab Arsitektural**: Sebuah variabel referensi mencoba menunjuk ke data lokal sementara di dalam fungsi atau blok sempit yang memorinya akan dibebaskan (*dropped*) sebelum referensi tersebut selesai digunakan.
- **Resep Solusi Rekayasa**: Kembalikan data kepemilikan penuh secara langsung (*by value*) alih-alih mengembalikan referensi pointer lokal, atau alokasikan data di heap menggunakan `Kotak<T>`.
