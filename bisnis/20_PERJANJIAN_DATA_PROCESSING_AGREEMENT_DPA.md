# PERJANJIAN PEMROSESAN DATA PRIBADI (DATA PROCESSING AGREEMENT / DPA)
**Kepatuhan Terhadap Undang-Undang No. 27 Tahun 2022 tentang Pelindungan Data Pribadi (UU PDP)**
**Nomor Dokumen: WIDYA/LEGAL/DPA/2026/____**

Perjanjian Pemrosesan Data Pribadi ini ("**DPA**") merupakan bagian tak terpisahkan dari Perjanjian Induk Layanan (*Master Services Agreement*) antara **PT WIDYA EKOSISTEM DIGITAL INDONESIA** (selanjutnya disebut sebagai "**PROSESOR DATA**") dan **[NAMA KLIEN / INSTITUSI]** (selanjutnya disebut sebagai "**PENGENDALI DATA**").

---

### PASAL 1: PERAN & RUANG LINGKUP PEMROSESAN DATA
1. PENGENDALI DATA bertindak sebagai pihak yang menentukan tujuan dan melakukan kendali atas pemrosesan data pribadi nasabah/pengguna akhir.
2. PROSESOR DATA bertindak hanya untuk dan atas nama PENGENDALI DATA dalam menjalankan pemrosesan data teknis (penyimpanan di WidyaBase, pemrosesan transaksi switching, atau enkripsi HSM) sesuai instruksi tertulis PENGENDALI DATA.

---

### PASAL 2: STANDAR KEAMANAN TEKNIS & KRIPTOGRAFIS
PROSESOR DATA wajib menerapkan langkah-langkah keamanan teknis dan organisasional berstandar industri tinggi:
1. **Enkripsi Data saat Beristirahat (*Encryption at Rest*)**: Seluruh berkas basis data WidyaBase dan objek biner WidyaStore wajib dienkripsi menggunakan algoritma **AES-256-GCM** atau modul **Kriptografi Pasca-Kuantum (PQC)**.
2. **Enkripsi Data saat Transit (*Encryption in Transit*)**: Seluruh komunikasi jaringan node server wajib menggunakan protokol **TLS 1.3** dengan sertifikat mTLS (*mutual TLS*).
3. **Pemisahan Data Multi-Tenant (*Tenant Isolation*)**: Menjamin isolasi penuh antar penyewa data menggunakan virtualisasi ringkas **WidyaBox MicroVM**.

---

### PASAL 3: LARANGAN TRANSFER DATA LINTAS BATAS TANPA IZIN
PROSESOR DATA menjamin bahwa seluruh pemrosesan dan penyimpanan data dilakukan di dalam wilayah hukum Negara Kesatuan Republik Indonesia (kedaulatan data nasional). Tidak ada data pribadi yang boleh ditransfer ke luar yurisdiksi Republik Indonesia tanpa persetujuan tertulis eksplisit dari PENGENDALI DATA dan pemenuhan regulasi UU PDP.

---

### PASAL 4: PROTOKOL PEMBERITAHUAN KEBOCOLAN DATA (DATA BREACH NOTIFICATION)
Jika terjadi insiden keamanan atau dugaan kebocoran data pribadi pada infrastruktur PROSESOR DATA:
1. PROSESOR DATA wajib memberitahukan kepada PENGENDALI DATA secara tertulis dalam waktu paling lambat **1x24 jam** sejak insiden terkonfirmasi.
2. Memberikan bantuan teknis lengkap bagi PENGENDALI DATA untuk melaporkan insiden kepada Lembaga Pengawas Pelindungan Data Pribadi / Kominfo dalam batas waktu maksimal **3x24 jam (72 jam)** sebagaimana disyaratkan oleh Pasal 46 UU PDP.

---

### PASAL 5: PENGHAPUSAN DAN PENGEMBALIAN DATA
Setelah pengakhiran layanan atau atas permintaan resmi PENGENDALI DATA, PROSESOR DATA wajib mengembalikan seluruh data pribadi kepada PENGENDALI DATA atau menghapusnya secara aman dan permanen (*cryptographic shredding*) dari seluruh media penyimpanan dan menerbitkan Sertifikat Pemusnahan Data.

---

```
PENGENDALI DATA,                              PROSESOR DATA,
[Nama Perusahaan Klien]                       PT Widya Ekosistem Digital Indonesia



____________________________________          ____________________________________
Nama    :                                     Nama    : 
Jabatan : Data Protection Officer (DPO)       Jabatan : Chief Technology Officer (CTO)
Tanggal :                                     Tanggal : 
```
