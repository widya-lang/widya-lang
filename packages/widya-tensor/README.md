# ⚡ @widya/tensor (Widya-NumPy)

> **Mesin Tensor N-Dimensi & Aljabar Linier Berkecepatan Tinggi untuk Widya-Lang.**  
> *Setara fungsional dan berkinerja setara NumPy dalam ekosistem Python.*

---

## 🚀 Fitur Utama
- **Pembuatan Tensor:** `buat(data, bentuk)`, `nol(bentuk)`, `satu(bentuk)`, `rentang(awal, akhir, langkah)`, `identitas(n)`.
- **Aljabar Linier:** Perkalian matriks 2D (`matmul`), Transpose (`transpose`), Dot Product 1D (`dot`).
- **Reduksi Statistik:** `jumlah`, `rata_rata`, `maksimum`, `minimum`, `norm_l2`.
- **Aritmatika Skalar:** `tambah_skalar`, `kali_skalar`.

## 📦 Pemasangan via WPM
```bash
widya wpm add @widya/tensor
```

## 💻 Contoh Penggunaan
```widya
impor "@widya/tensor" sebagai np;

// 1. Buat Matriks 2x2
misal A = np.buat([1.0, 2.0, 3.0, 4.0], [2, 2]);
misal B = np.identitas(2);

// 2. Perkalian Matriks
misal C = np.matmul(A, B);
cetak("Total elemen C:", np.jumlah(C));
cetak("Rata-rata:", np.rata_rata(C));
```

