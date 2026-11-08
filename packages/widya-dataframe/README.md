# 📊 @widya/dataframe (Widya-Pandas)

> **Mesin Analisis Data Tabular & Transformasi Kolom untuk Widya-Lang.**  
> *Setara fungsional dan berkinerja setara Pandas / Polars dalam ekosistem Python.*

---

## 🚀 Fitur Utama
- **I/O Fleksibel:** `dari_csv(teks)`, `ke_csv(df)`, `dari_array_objek(arr)`.
- **Manipulasi Kolom & Baris:** `pilih_kolom(df, cols)`, `saring(df, col, op, val)`, `kepala(df, n)`, `ekor(df, n)`.
- **Agregasi & Statistik:** `ringkasan(df)` *(describe)*, `kelompokkan_dan_hitung(df, key)`, `kelompokkan_dan_jumlah(df, key, target)`.

## 📦 Pemasangan via WPM
```bash
widya wpm add @widya/dataframe
```

## 💻 Contoh Penggunaan
```widya
impor "@widya/dataframe" sebagai pd;

misal csv = "nama,departemen,gaji\nBudi,IT,15000000\nSiti,Finance,18000000\nAndi,IT,12000000";
misal df = pd.dari_csv(csv);

// Filter gaji di atas 14 juta
misal df_tinggi = pd.saring(df, "gaji", ">=", 14000000);

// Ringkasan Statistik
misal stats = pd.ringkasan(df);
cetak("Statistik:", stats);
```

