# KAMUS DIGITAL MULTIBAHASA DAERAH NUSANTARA
## *Widya-Lang Native Lexicographical & Script Engine*

---

## 📖 Deskripsi
**Kamus Digital Bahasa Daerah Nusantara** adalah mesin leksikografi dan penerjemah komparatif yang mengintegrasikan leksikon 12+ bahasa daerah di Indonesia dengan sistem tingkatan tutur (*undak-usuk, unggah-ungguh, anggah-ungguhing basa*) serta transliterasi otomatis ke aksara tradisional Nusantara.

---

## 🌟 Fitur Utama

1. **Pencarian Dua Arah (Bidirectional Lookup)**:
   - Bahasa Indonesia ➔ 12 Bahasa Daerah (Jawa, Sunda, Bali, Minang, Bugis, Makassar, Batak Toba, Aceh, Banjar, Lampung, Melayu Papua).
   - Bahasa Daerah ➔ Bahasa Indonesia.

2. **Tingkatan Kesantunan Tutur Terintegrasi**:
   - Bahasa Jawa: *Ngoko, Krama Madya, Krama Inggil*.
   - Bahasa Sunda: *Loma, Lemes keur Sorangan, Lemes keur Batur*.
   - Bahasa Bali: *Andap, Alus Sor, Alus Singgih*.

3. **Transliterasi Aksara Tradisional Otomatis**:
   - ꦲ Aksara Hanacaraka Jawa
   - ᮃ Aksara Sunda Baku
   - ᬅ Aksara Bali
   - ᯀ Surat Batak
   - ᨕ Aksara Lontara Bugis-Makassar
   - ᰀ Had Lampung KaGaNga

4. **Keluaran Terstruktur & Integrasi JSON**:
   - Berkas basis data leksikon format JSON siap pakai: [database_kamus_nusantara.json](file:///home/devops/widya-lang/kurikulum_bahasa_daerah/kamus_bahasa_daerah/database_kamus_nusantara.json) (Versi 6.0 dengan **85 Entri Lengkap**).
   - Kategori leksikon mencakup:
     - 🔢 **Numeralia (Bilangan)**: 1 hingga 1.000 (*satu, dua, tiga, empat, lima, enam, tujuh, delapan, sembilan, sepuluh, seratus, seribu*).
     - 🎨 **Adjektiva Warna**: *putih, hitam, merah, kuning, hijau, biru*.
     - 🏃 **Verba Aktivitas & Pergerakan**: *makan, minum, tidur, mandi, bangun, berjalan, berlari, duduk, berdiri, melihat, mendengar, berbicara, membaca, menulis, bekerja, belajar, membeli, menjual, memasak, mencuci, tertawa, menangis, bernyanyi, menari, mencintai, berdoa, membantu, memberi, menerima*.
     - 🫀 **Nomina Anatomi & Tubuh**: *kepala, mata, telinga, hidung, mulut, gigi, tangan, kaki, hati*.
     - 🌄 **Nomina Alam, Waktu & Benda**: *air, api, tanah, matahari, bulan, laut, gunung, rumah, pagi, malam*.
     - 👨‍👩‍👧‍👦 **Nomina Keluarga & Sosial**: *ayah, ibu, anak, kakek, nenek, sahabat*.
     - 💬 **Adjektiva Sifat & Kualitas**: *besar, kecil, baik, lezat, manis, cepat, pintar, indah*.
     - 🤝 **Ungkapan & Kesantunan**: *terima kasih, maaf, apa kabar, selamat jalan, selamat datang*.

## 🗄️ Persistensi Mesin Basis Data WidyaBase (.wdb)

Seluruh 85 entri leksikon telah dipersistensikan secara permanen ke mesin basis data **WidyaBase** (`leksikon_nusantara.wdb`) dengan kepatuhan transaksi ACID, Slotted-Page Storage 4KB, dan indeks B+Tree.

### Skema Tabel WidyaBase:
1. `leksikon_master` (`id_leksikon`, `kata_indonesia`, `kelas_kata`, `kategori`)
2. `leksikon_ringkas` (`id_leksikon`, `kata_indonesia`, `kelas_kata`, `kategori`, `jawa_ngoko`, `jawa_krama`, `sunda_loma`, `sunda_lemes`, `bali_andap`, `bali_singgih`, `minang`, `bugis`, `batak`, `aceh`, `banjar`, `lampung`, `papua`)
3. `leksikon_detail_daerah` (`id_entri`, `id_leksikon`, `bahasa`, `bentuk_netral`, `bentuk_halus`, `aksara`, `contoh_kalimat`)

---

## 🚀 Cara Menjalankan

### 1. Menjalankan Mesin Kamus Komparatif Interaktif:
```bash
cargo run -- kurikulum_bahasa_daerah/kamus_bahasa_daerah/kamus_nusantara_engine.wya
```

### 2. Menjalankan Sinkronisasi / Migrasi ke WidyaBase:
```bash
cargo run -- kurikulum_bahasa_daerah/kamus_bahasa_daerah/simpan_widyabase_leksikon.wya
```

