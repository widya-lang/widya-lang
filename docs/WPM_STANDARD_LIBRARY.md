# 📚 Dokumentasi Pustaka Standar Resmi Widya-Lang (WPM Standard Library)

Dokumen ini menyediakan referensi API resmi untuk 6 pustaka standar utama ekosistem Widya-Lang (`v0.1.0`), yang dirancang untuk memberikan paritas fungsional penuh terhadap pustaka terkemuka dalam ekosistem Python dan Node.js.

---

## 1. `@widya/tensor` (Setara NumPy)
Pustaka komputasi larik N-dimensi dan aljabar linier performa tinggi.

### Cara Impor:
```widya
impor "@widya/tensor" sebagai np;
```

### Fungsi & API Utama:
- `np.tensor_buat(data, dimensi)`: Membuat instance tensor baru dengan data elemen bertipe float dan dimensi tertentu.
- `np.matriks_buat(baris, kolom, elemen)`: Membuat matriks 2D dengan jumlah baris dan kolom tertentu.
- `np.identitas(n)`: Membuat matriks identitas $n \times n$ di mana elemen diagonal utama bernilai 1.0.
- `np.matmul(a, b)`: Menghitung perkalian dua buah matriks $A \times B$ berdimensi $(m \times k)$ dan $(k \times n)$.
- `np.dot(v1, v2)`: Menghitung hasil perkalian titik (*dot product*) dari dua buah vektor atau tensor 1D.
- `np.transpose(matriks)`: Menghasilkan transposisi dari matriks (menukar baris menjadi kolom).
- `np.jumlah(t)`: Menghitung nilai akumulasi total dari seluruh elemen tensor.
- `np.rata_rata(t)`: Menghitung nilai rata-rata (*mean*) dari seluruh elemen tensor.

---

## 2. `@widya/dataframe` (Setara Pandas)
Pustaka manipulasi, transformasi, dan analisis data tabular kolumnar.

### Cara Impor:
```widya
impor "@widya/dataframe" sebagai pd;
```

### Fungsi & API Utama:
- `pd.df_buat(kolom, baris_data)`: Menginisialisasi DataFrame tabular baru dengan daftar nama kolom dan baris rekaman (kamus/objek).
- `pd.baca_csv(jalur_berkas)`: Membaca berkas CSV dan mengonversinya menjadi DataFrame secara otomatis.
- `pd.saring(df, nama_kolom, predikat_fungsi)`: Menyaring baris data berdasarkan fungsi evaluasi kondisi tertentu.
- `pd.pilih_kolom(df, kolom_terpilih)`: Memproyeksikan DataFrame hanya dengan kolom-kolom tertentu yang dipilih.
- `pd.ringkasan(df, nama_kolom)`: Menghitung ringkasan statistik deskriptif (min, max, rata-rata, jumlah rekaman).
- `pd.kelompokkan_dan_hitung(df, kolom_grup)`: Melakukan pengelompokan (*group by*) berdasarkan nilai kolom dan menghitung frekuensi kejadiannya.

---

## 3. `@widya/web` (Setara Express / FastAPI)
Microframework RESTful web router berbasis radix-tree dengan dukungan middleware pipelining.

### Cara Impor:
```widya
impor "@widya/web" sebagai web;
```

### Fungsi & API Utama:
- `web.buat_router()`: Menginstansiasi router baru.
- `web.gunakan_middleware(router, fungsi_middleware)`: Mendaftarkan fungsi middleware ke dalam rantai pemrosesan permintaan (*request pipeline*).
- `web.rute_get(router, jalur, penangan)`: Mendaftarkan rute HTTP `GET`.
- `web.rute_post(router, jalur, penangan)`: Mendaftarkan rute HTTP `POST`.
- `web.rute_put(router, jalur, penangan)`: Mendaftarkan rute HTTP `PUT`.
- `web.rute_delete(router, jalur, penangan)`: Mendaftarkan rute HTTP `DELETE`.
- `web.respons_json(status_kode, data)`: Menghasilkan objek respons HTTP dengan payload berformat JSON dan header `Content-Type: application/json`.
- `web.respons_teks(status_kode, teks)`: Menghasilkan objek respons teks polos.
- `web.tangani_permintaan(router, req)`: Menjalankan pencocokan rute dan memproses permintaan melalui seluruh rantai middleware.

---

## 4. `@widya/ai` (Setara PyTorch Autograd)
Mesin autograd diferensiasi otomatis reverse-mode berbasis graf asiklik terarah (DAG) topologis.

### Cara Impor:
```widya
impor "@widya/ai" sebagai torch;
```

### Fungsi & API Utama:
- `torch.variabel(nilai, butuh_gradien)`: Membungkus nilai skalar/float ke dalam simpul komputasi graf yang dapat melacak turunan.
- `torch.tambah_var(a, b)`: Operasi penambahan diferensiabel ($a + b$).
- `torch.kurang_var(a, b)`: Operasi pengurangan diferensiabel ($a - b$).
- `torch.kali_var(a, b)`: Operasi perkalian diferensiabel ($a \times b$).
- `torch.lapisan_linear(bobot_awal, bias_awal)`: Menginstansiasi layer saraf linier ($y = w \cdot x + b$).
- `torch.linear_maju(layer, x)`: Menghitung propagasi maju (*forward pass*) lapisan linier.
- `torch.relu(v)`: Fungsi aktivasi ReLU non-linear diferensiabel ($\max(0, x)$).
- `torch.mse_loss(prediksi, target)`: Menghitung fungsi rugi Mean Squared Error diferensiabel ($0.5 \times (pred - target)^2$).
- `torch.mundur(akar)`: Melakukan propagasi mundur (*backpropagation*) dalam urutan topologis terbalik untuk menghitung akumulasi gradien pada seluruh daun graf.
- `torch.sgd_langkah(parameter_list, laju_pembelajaran)`: Memperbarui nilai parameter menggunakan algoritma optimasi Stochastic Gradient Descent ($param = param - lr \times grad$) dan mereset akumulasi gradien ke 0.0.

---

## 5. `@widya/sql` (Setara SQLite3 / Prisma)
Mesin basis data relasional in-memory mandiri dengan query builder CRUD yang aman dan cepat.

### Cara Impor:
```widya
impor "@widya/sql" sebagai sql;
```

### Fungsi & API Utama:
- `sql.db_buka(nama_basis_data)`: Menginisialisasi basis data relasional in-memory baru.
- `sql.db_buat_tabel(db, nama_tabel, daftar_kolom)`: Mendefinisikan skema tabel baru.
- `sql.db_sisip(db, nama_tabel, baris_rekaman)`: Menyisipkan baris data baru ke dalam tabel.
- `sql.db_pilih(db, nama_tabel, kriteria_saring)`: Mengambil daftar rekaman yang memenuhi kondisi pencarian.
- `sql.db_perbarui(db, nama_tabel, kondisi_saring, data_baru)`: Memperbarui kolom data rekaman yang sesuai kriteria.
- `sql.db_hapus(db, nama_tabel, kondisi_saring)`: Menghapus rekaman yang sesuai kriteria dari tabel.
- `sql.db_hitung(db, nama_tabel)`: Menghitung total jumlah baris dalam tabel.

---

## 6. `@widya/test` (Setara Jest / Pytest)
Kerangka kerja pengujian mandiri (*unit test & assertion engine*) untuk memastikan keandalan kode.

### Cara Impor:
```widya
impor "@widya/test" sebagai test;
```

### Fungsi & API Utama:
- `test.uji_buat_pelari()`: Menginisialisasi runner pengujian baru.
- `test.uji_tambah(pelari, nama_kasus, fungsi_pengujian)`: Mendaftarkan kasus uji ke dalam runner.
- `test.uji_jalankan(pelari)`: Mengeksekusi seluruh pengujian yang terdaftar dan mencetak laporan ringkasan kelulusan.
- `test.tegaskan_sama(t, aktual, ekspektasi, pesan_galat)`: Menegaskan bahwa nilai aktual identik dengan nilai ekspektasi.
- `test.tegaskan_tidak_sama(t, aktual, tidak_diharapkan, pesan_galat)`: Menegaskan bahwa nilai aktual berbeda dengan nilai yang dilarang.
- `test.tegaskan_benar(t, kondisi, pesan_galat)`: Menegaskan bahwa suatu ekspresi bernilai `benar` (`true`).
- `test.tegaskan_salah(t, kondisi, pesan_galat)`: Menegaskan bahwa suatu ekspresi bernilai `salah` (`false`).

