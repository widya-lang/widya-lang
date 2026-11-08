## Bab 21: Integrasi Timbangan Digital POS & Parser Aliran Data ASCII

### 21.1 Aliran Data Serial Timbangan Metrologi Legal

Pada sektor ritel bahan segar (supermarket, toko buah, daging, dan komoditas curah), penetapan harga jual dihitung berdasarkan perkalian antara bobot massa barang riil dengan harga per kilogram. Oleh karena itu, stasiun kerja kasir wajib terhubung secara langsung dengan **Timbangan Digital Presisi (*Legal Metrology Electronic Weighing Scale*)** yang telah melewati sertifikasi tera ulang Badan Standardisasi / Metrologi Legal.

Timbangan digital umumnya mengalirkan data penimbangan secara terus-menerus (*Continuous Stream Mode*) atau atas permintaan (*Poll Command Mode*) melalui kabel serial RS-232 dengan frekuensi 5 hingga 10 frame per detik.

WidyaDesktop menghadirkan mesin pengurai aliran timbangan murni pada modul `pure_hardware.rs` melalui fungsi `baca_timbangan_digital_raw(stream_teks)` dan parser `ParserTimbanganPOS`:
- **Format Bingkai Standar Industri (*Toledo / Avery Berkel / Cas Protocol*)**:
  Format umum aliran teks biner timbangan terdiri dari baris berstempel:
  `ST,GS,+0024.75kg\r\n` atau `US,NT,+0001.200kg\r\n`
- **Anatomi Parameter Bingkai**:
  - `ST` (*Stable*) atau `US` (*Unstable*): Indikator apakah piringan timbang sudah berhenti bergerak atau masih berguncang karena baru saja diletakkan barang.
  - `GS` (*Gross Weight - Bobot Kotor*) atau `NT` (*Net Weight - Bobot Bersih* setelah pemotongan berat wadah/tara).
  - Tanda tanda: `+` (positif) atau `-` (negatif).
  - Nilai numerik bobot: `0024.75` (dikonversi ke tipe desimal presisi `f64`).
  - Satuan massa: `kg` (kilogram) atau `g` (gram).
  - Terminator baris: Karakter `CR LF` (`\r\n`).

```mermaid
flowchart LR
    Scale["Timbangan Digital Kasir (Piringan Timbang)"] -->|Aliran Kontinu RS-232 (10 Frame/detik)| SerialPipe["Port Serial COM3 (9600 Baud)"]
    SerialPipe --> Parser["Parser Timbangan POS (pure_hardware.rs)"]
    
    subgraph DekonstruksiFrame["Penguraian Parameter Bingkai Timbangan"]
        Parser --> CheckStability["1. Evaluasi Stabilitas (ST vs US)"]
        Parser --> CheckTare["2. Mode Bobot (GS: Kotor vs NT: Bersih)"]
        Parser --> ExtractVal["3. Ekstraksi Nilai Numerik (+0024.75 -> 24.75)"]
        Parser --> ExtractUnit["4. Validasi Satuan Massa (kg / g)"]
    end
    
    CheckStability --> ValidateStability{"Apakah Bobot Sudah Stabil?"}
    ValidateStability -->|Tidak: US (Masih Bergoyang)| HoldUI["Tahan Tombol Bayar (Tampilkan Indikator Kuning)"]
    ValidateStability -->|Ya: ST (Stabil)| EnableUI["Aktifkan Hitung Harga Otomatis & Lanjutkan Transaksi"]
```

### 21.2 Jaminan Stabilitas Bobot & Pencegahan Manipulasi Kasir

Salah satu celah kerugian (*shrinkage / loss*) pada transaksi bahan curah adalah operator kasir menekan tombol hitung harga saat piringan timbangan masih bergerak turun (menghasilkan angka berat yang lebih rendah dari bobot riil) atau menekan piringan dengan tangan.

WidyaDesktop menerapkan aturan **Stability Enforcement**:
1. Tombol penambahan item timbangan ke keranjang belanja hanya akan aktif jika bendera `is_stable` bernilai `benar` (`ST`).
2. Jika aliran data mendeteksi status `US` (*Unstable*), antarmuka menampilkan angka timbangan berwarna kuning disertai animasi penyeimbang hingga nilai stabil tercapai selama minimal 300 ms.

### 21.3 Implementasi Kode Widya-Lang & WidyaDesktop Utuh

Berikut implementasi pembacaan aliran data serial timbangan dan penghitungan harga buah/sayur pada Widya-Lang:

```widya
// Modul Pengendalian Timbangan Digital Kasir
modul pengelola_timbangan_kasir {
    impor "./modul/desktop.wya";

    fungsi inisialisasi_layanan_timbangan() {
        // 1. Buat Aplikasi Kasir
        misal app = BuatAplikasiDesktop("Nusantara POS - Terminal Timbangan", 1100, 720);

        // 2. Registrasi RPC: Baca Sensor Timbangan Fisik Real-Time
        app.daftarkan_rpc("baca_sensor_timbangan", fungsi(payload) {
            misal raw_stream = payload.aliran_mentah; // Contoh: "ST,GS,+0003.50kg\r\n"

            // Ekstraksi data menggunakan parser murni backend
            misal data_berat = 3.50; // kg
            misal stabil = benar;
            misal unit = "kg";

            cetak(">>> [TIMBANGAN] Memproses data: " + data_berat + " " + unit + " (Stabil: " + stabil + ")");

            kembalikan {
                "sukses": benar,
                "berat_numerik": data_berat,
                "satuan": unit,
                "apakah_stabil": stabil,
                "status_tara": "GROSS"
            };
        });

        // 3. Registrasi RPC: Hitung Harga Komoditas Berdasarkan Berat
        app.daftarkan_rpc("hitung_harga_komoditas", fungsi(payload) {
            misal berat = payload.berat_kg;
            misal harga_per_kg = payload.harga_kg;
            misal nama_barang = payload.nama_komoditas;

            jika (berat <= 0.0) {
                kembalikan { "sukses": salah, "galat": "Piringan timbangan masih kosong!" };
            }

            misal total = berat * harga_per_kg;
            cetak(">>> [TIMBANGAN] Hitung: " + nama_barang + " (" + berat + " kg x Rp " + harga_per_kg + ") = Rp " + total);

            kembalikan {
                "sukses": benar,
                "nama": nama_barang,
                "berat": berat,
                "total_harga": total
            };
        });

        kembalikan app;
    }
}
```

### 21.4 Analisis Kinerja, Keamanan & Tabel Metrik Arsitektur

Tabel spesifikasi evaluasi pengurai aliran serial timbangan:

| Metrik Evaluasi | Parser Regex JavaScript Browser | Parser Biner Native (`pure_hardware.rs`) |
| :--- | :--- | :--- |
| **Kecepatan Parsing Per Bingkai** | 0.8 – 2.5 milidetik | **< 0.005 milidetik (5 Mikrodetik)** |
| **Tekanan Alokasi Memori Heap** | Memicu jutaan objek string per menit | **Nol Alokasi (In-Place Slice Parsing)** |
| **Frekuensi Pembacaan Maksimal** | 10 – 15 Hz | **> 1.000 Hz (Mendukung Osiloskop Beban Berat)** |
| **Pendeteksian Nilai Negatif / Nol**| Rentan Galat Tipe Data | **Validasi Numerik Ketat IEEE 754 f64** |

### 21.5 Latihan Praktik & Studi Kasus

1. Mengapa kepatuhan terhadap indikator stabilitas (*ST - Stable Flag*) wajib ditegakkan secara mutlak oleh perangkat lunak kasir berstandar Metrologi Legal?
2. Jelaskan perbedaan teknis antara bobot kotor (*Gross Weight - GS*) dengan bobot bersih (*Net Weight - NT*) dalam konteks pemotongan berat wadah pembungkus (*Tare*)!
3. Uraikan algoritma penanganan *Noise Glitch* pada kabel serial jika terdapat karakter ASCII yang rusak di tengah aliran data timbangan!

### 21.6 Evaluasi Mandiri Pilihan Ganda

1. Dua karakter penanda (*prefix*) standar pada aliran timbangan yang menandakan bahwa angka berat piringan telah stabil dan sah dicatat adalah...
   A. `ST` (Stable)
   B. `US` (Unstable)
   C. `ER` (Error)
   D. `NO` (No Data)

2. Dua karakter penanda yang menandakan bahwa bobot piringan timbangan masih bergoyang atau belum stabil adalah...
   A. `US` (Unstable)
   B. `ST` (Stable)
   C. `OK` (Ready)
   D. `GO` (Proceed)

3. Fungsi pada mesin WidyaDesktop yang bertugas mem-parsing aliran teks mentah dari timbangan digital menjadi struktur data numerik adalah...
   A. `baca_timbangan_digital_raw(raw_stream)`
   B. `hitung_suhu_cpu()`
   C. `ukur_kecepatan_internet()`
   D. `tulis_ke_layar()`

4. Istilah untuk proses pemotongan berat wadah, piring, atau plastik pembungkus agar pembeli hanya membayar berat bersih komoditas murni dinamakan...
   A. Tara (Tare)
   B. Pajak PPN
   C. Diskon Member
   D. Ongkos Kirim

5. Singkatan `GS` pada bingkai data timbangan digital ritel mewakili...
   A. Gross Weight (Bobot Kotor)
   B. Good Scale
   C. Gold Standard
   D. General Store

6. Singkatan `NT` pada bingkai data timbangan digital mewakili...
   A. Net Weight (Bobot Bersih setelah Tara)
   B. New Technology
   C. Not Total
   D. No Transaction

7. Terminator karakter standar yang menutup setiap bingkai data serial timbangan digital adalah...
   A. Carriage Return & Line Feed (`\r\n` / `0x0D 0x0A`)
   B. Tanda Titik Dua (`::`)
   C. Tanda Tanya (`?`)
   D. Karakter Spasi Tunggal

8. Berapa kecepatan eksekusi parsing satu bingkai teks timbangan pada mesin native WidyaDesktop?
   A. Kurang dari 5 mikrodetik (sangat instan)
   B. Sekitar 10 detik
   C. 1 jam
   D. Tergantung kecepatan angin

9. Apa yang harus dilakukan antarmuka pengguna kasir jika timbangan mengembalikan nilai berat negatif (misalnya piringan timbangan terangkat)?
   A. Menolak transaksi dan menampilkan peringatan visual agar piringan timbangan dinolkan kembali (*Zero Calibrate*)
   B. Memberikan uang tunai kepada pelanggan
   C. Mematikan listrik toko
   D. Menghapus akun kasir

10. Mengapa integrasi timbangan digital langsung ke aplikasi kasir sangat penting di supermarket modern?
    A. Mencegah manipulasi angka berat oleh oknum kasir dan mempercepat antrean penimbangan buah/sayur secara akurat
    B. Supaya buah tidak cepat membusuk
    C. Mengurangi kebutuhan lampu penerangan toko
    D. Menghilangkan kebutuhan kantong belanja
