## Lampiran B: Katalog Error & Penanganan Masalah (Troubleshooting)

### B.1 Format Pesan Kesalahan Terstandarisasi

WidyaBase mengadopsi format pelaporan galat standar internasional berstandar ISO SQLState 5-karakter yang dilengkapi pesan penjelasan kontekstual dwibahasa, petunjuk perbaikan teknis (*hint*), dan penunjuk posisi baris/kolom kesalahan kueri.

Format dasar pesan kesalahan:
```
GALAT [KODE_SQLSTATE]: Deskripsi pesan kesalahan dalam Bahasa Indonesia.
DETAIL : Rincian teknis status internal mesin basis data.
PETUNJUK: Langkah perbaikan yang disarankan untuk operator/pengembang.
LOKASI : Baris X, Kolom Y pada kueri masukan.
```

### B.2 Katalog Kode Galat Standar WidyaBase

| Kode SQLSTATE | Nama Galat Internal | Penyebab Umum Masalah | Tindakan Perbaikan Rekomendasi |
| :--- | :--- | :--- | :--- |
| `23505` | `GalatPelanggaranKunciUnik` | Mencoba menyisipkan nilai yang sudah ada pada kolom berindeks UNIK / KUNCI_UTAMA | Periksa duplikasi data atau gunakan klausa `SISIP_ATAU_ABAIKAN` / `ON CONFLICT DO UPDATE` |
| `23503` | `GalatIntegritasKunciAsing` | Memasukkan nilai kunci asing yang tidak terdaftar pada tabel induk, atau menghapus baris induk | Pastikan catatan induk sudah ada sebelum membuat catatan anak, atau gunakan `ON DELETE CASCADE` |
| `23514` | `GalatPelanggaranBatasanCek`| Nilai kolom melanggar ekspresi batasan `PERIKSA` (misal saldo negatif atau umur < 0) | Sesuaikan nilai data agar memenuhi batasan bisnis skema tabel |
| `40P01` | `GalatKebuntuanDeadlock` | Dua transaksi saling menunggu kunci sumber daya satu sama lain dalam siklus tertutup | Tangkap eksepsi pada kode program klien dan lakukan percobaan ulang (*retry transaction with backoff*) |
| `40001` | `GalatKegagalanSerialisasi` | Terjadi konflik snapshot konkurensi pada tingkat isolasi Repeatable Read / Serializable | Ulangi eksepsi transaksi dari awal karena snapshot telah kedaluwarsa oleh transaksi lain |
| `42P01` | `GalatTabelTidakDitemukan` | Nama tabel yang dipanggil dalam klausa `DARI` tidak terdaftar di katalog metadata basis data | Periksa ejaan nama tabel atau pastikan skema basis data aktif telah tepat |
| `42703` | `GalatKolomTidakDitemukan` | Nama kolom yang dipanggil tidak ada di dalam skema tabel yang bersangkutan | Periksa ejaan kolom melalui kueri `INFORMASI_SKEMA` atau `\d nama_tabel` |
| `42601` | `GalatSintaksisKueri` | Struktur pengetikan SQL melanggar aturan tata bahasa leksikal atau parser | Periksa tanda koma, kurung buka/tutup, dan ejaan kata kunci pada posisi kursor yang ditunjukkan |
| `53100` | `GalatPenyimpananPenuh` | Media penyimpanan fisik SSD/HDD telah kehabisan ruang tampung bebas (*disk full*) | Bersihkan log usang, jalankan `VACUUM`, atau pindahkan partisi lampau ke Cold Tier S3 |
| `53200` | `GalatMemoriKerjaHabis` | Alokasi memori kerja RAM (`work_mem` / Buffer Pool) melampaui batas toleransi fisik peladen | Optimalkan alokasi `work_mem` di `widyabase.conf` atau tambahkan RAM fisik |
| `57P01` | `GalatKoneksiJaringanPutus` | Soket koneksi TCP pgwire terputus mendadak oleh klien atau gangguan firewall | Periksa kestabilan jaringan LAN/VPN dan parameter *keepalive* peladen |
| `58000` | `GalatKorupsiHalamanFisik` | Halaman 4KB di disk mengalami kegagalan verifikasi Checksum CRC32 atau dekripsi AEAD | Jalankan prosedur pemulihan bencana via WAL ARIES atau pulihkan berkas dari replika Raft |
| `58P01` | `GalatKonsensusKuorumHilang`| Kluster Raft kehilangan mayoritas simpul (misal 2 dari 3 simpul mati) | Nyalakan kembali simpul yang mati untuk memulihkan kuorum mayoritas kluster |

### B.3 Prosedur Diagnostik & Pemecahan Masalah Sistem

1. **Diagnosis Kemacetan Kueri Lambat (*Slow Queries*)**:
   - Gunakan perintah `JELASKAN ANALISIS PILIH ...` (`EXPLAIN ANALYZE`) untuk melihat rencana eksekusi fisik, perkiraan biaya CBO vs biaya nyata, dan waktu pemrosesan operator.
   - Periksa apakah kueri melakukan pemindaian tabel penuh (*SeqScan*) pada tabel jutaan baris akibat ketiadaan indeks yang tepat.
2. **Pembersihan Kebuntuan Transaksi (*Deadlock Clearing*)**:
   - Periksa kueri aktif melalui tabel pemantauan sistem `PILIH * DARI pg_stat_activity;`.
   - Batalkan transaksi yang macet menggunakan fungsi administratif `HENTIKAN_KONEKSI(pid)`.
3. **Pemulihan Pasca Kerusakan Fisik Data (*Data Corruption Recovery*)**:
   - WidyaBase secara otomatis mengisolasi halaman rusak dan mencoba menarik salinan halaman bersih dari simpul pengikut kluster Raft (*Page Repair RPC*).
