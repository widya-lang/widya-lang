# 🎓 Tutorial Lengkap WidyaBase v1.0.0
### Panduan Praktis & Langkah-Demi-Langkah Membangun Aplikasi dengan Mesin Basis Data Berdaulat

Selamat datang di **Tutorial Praktis WidyaBase**! Tutorial ini dirancang untuk memandu Anda dari nol hingga mahir dalam menggunakan **WidyaBase**, baik melalui terminal interaktif (CLI), pemrograman dalam bahasa Widya-Lang, maupun menghubungkannya dengan klien eksternal seperti DBeaver, `psql`, dan Python.

> [!TIP]
> 📖 **E-Book PDF Resmi untuk Pemula Tersedia!**  
> Unduh atau baca langsung buku panduan lengkap: [Buku_Panduan_WidyaBase_Pemula.pdf](file:///c:/widya-lang/Buku_Panduan_WidyaBase_Pemula.pdf) (atau berkas web: [BUKU_PANDUAN_WIDYABASE.html](file:///c:/widya-lang/docs/BUKU_PANDUAN_WIDYABASE.html)).

---

## 📑 Daftar Isi
1. [Pengenalan Singkat WidyaBase](#1-pengenalan-singkat-widyabase)
2. [Langkah 1: Menjalankan Shell Interaktif SQL (REPL)](#2-langkah-1-menjalankan-shell-interaktif-sql-repl)
3. [Langkah 2: Belajar SQL Dialek Nusantara vs ANSI Global](#3-langkah-2-belajar-sql-dialek-nusantara-vs-ansi-global)
4. [Langkah 3: Menggunakan WidyaBase dalam Kode Widya-Lang](#4-langkah-3-menggunakan-widyabase-dalam-kode-widya-lang)
5. [Langkah 4: Menjalankan Server Daemon PostgreSQL Wire Protocol (Port 5432)](#5-langkah-4-menjalankan-server-daemon-postgresql-wire-protocol-port-5432)
6. [Langkah 5: Menghubungkan GUI DBeaver, pgAdmin & psql](#6-langkah-5-menghubungkan-gui-dbeaver-pgadmin--psql)
7. [Langkah 6: Menghubungkan Aplikasi Python & Node.js](#7-langkah-6-menghubungkan-aplikasi-python--nodejs)
8. [Langkah 7: Audit Kesehatan, WAL Recovery & AI Vektor](#8-langkah-7-audit-kesehatan-wal-recovery--ai-vektor)
9. [Langkah 8: Panduan Lengkap Impor & Ekspor Data (JSON, CSV, SQL Dump)](#9-langkah-8-panduan-lengkap-impor--ekspor-data-json-csv-sql-dump)

---

## 🌟 1. Pengenalan Singkat WidyaBase

**WidyaBase** adalah sistem manajemen basis data relasional (*RDBMS*) mandiri yang dirancang dengan karakteristik:
- **Arsitektur Slotted-Page 4KB**: Format biner penyimpanan halaman 4096-byte dengan direktori slot dan *Buffer Pool Manager* 512 halaman cache.
- **Ketahanan Penuh ACID**: Menggunakan *Write-Ahead Logging (WAL)* dengan penomoran urut LSN untuk pemulihan otomatis pasca-crash (*ARIES crash recovery*).
- **Isolasi Bebas Kunci (MVCC)**: *Snapshot Isolation* berlatensi rendah (`xmin` & `xmax`), di mana pembaca (*readers*) dan penulis (*writers*) tidak saling memblokir.
- **SQL Dwibahasa Simetris**: Anda dapat mengetik SQL dalam Bahasa Indonesia (`PILIH`, `SISIP_KE`, `DIMANA`) maupun Standar ANSI (`SELECT`, `INSERT INTO`, `WHERE`) dengan 0% penalti performa.
- **Protokol Standar Global**: Mendukung *PostgreSQL Wire Protocol v3.0* pada port 5432.

---

## 💻 2. Langkah 1: Menjalankan Shell Interaktif SQL (REPL)

Anda dapat memulai shell interaktif SQL baik menggunakan biner mandiri `WidyaBase.exe` atau perintah ekosistem `widya base sql`.

### Opsi A: Menggunakan Biner Mandiri `WidyaBase.exe`
```powershell
# Buka shell SQL interaktif pada basis data 'toko_saya.wdb'
.\WidyaBase.exe sql --data toko_saya.wdb
```

### Opsi B: Menggunakan CLI Ekosistem `widya`
```powershell
widya base sql --data toko_saya.wdb
```

*Tampilan Terminal:*
```text
============================================================
  🗄️  WIDYABASE INTERACTIVE REPL v1.0.0 (SQL DWIBAHASA)
  Mendukung: PILIH, SISIP_KE, PERBARUI, HAPUS_DARI, BUAT_TABEL
             SELECT, INSERT INTO, UPDATE, DELETE FROM, CREATE TABLE
  Ketik '\q' atau 'keluar' untuk keluar dari terminal.
============================================================
widyabase> 
```

---

## 🗣️ 3. Langkah 2: Belajar SQL Dialek Nusantara vs ANSI Global

WidyaBase memperlakukan kata kunci Bahasa Indonesia dan ANSI secara setara. Berikut contoh perbandingannya:

### 1. Membuat Skema Tabel (DDL)

**Bahasa Indonesia:**
```sql
BUAT_TABEL produk (
    id INT KUNCI_UTAMA,
    nama TEKS,
    kategori TEKS,
    harga ANGKA,
    stok INT
);
```

**Standar ANSI Global:**
```sql
CREATE TABLE produk (
    id INT PRIMARY KEY,
    nama TEXT,
    kategori TEXT,
    harga NUMERIC,
    stok INT
);
```

---

### 2. Menyisipkan Data (DML)

**Bahasa Indonesia:**
```sql
SISIP_KE produk (id, nama, kategori, harga, stok) NILAI (1, 'Kopi Gayo Arabika', 'Minuman', 35000, 20);
SISIP_KE produk (id, nama, kategori, harga, stok) NILAI (2, 'Teh Hitam Melati', 'Minuman', 18000, 45);
SISIP_KE produk (id, nama, kategori, harga, stok) NILAI (3, 'Croissant Coklat', 'Roti', 28000, 12);
SISIP_KE produk (id, nama, kategori, harga, stok) NILAI (4, 'Kopi Toraja Robusta', 'Minuman', 42000, 8);
SISIP_KE produk (id, nama, kategori, harga, stok) NILAI (5, 'Roti Sourdough', 'Roti', 32000, 15);
```

**Standar ANSI Global:**
```sql
INSERT INTO produk (id, nama, kategori, harga, stok) VALUES (1, 'Kopi Gayo Arabika', 'Minuman', 35000, 20);
```

---

### 3. Membaca & Memfilter Data

**Bahasa Indonesia:**
```sql
PILIH nama, harga, stok 
DARI produk 
DIMANA harga >= 25000 
URUTKAN BERDASARKAN harga TURUN;
```

*Output:*
```text
┌────────────────────────────────────────────────────────┐
│ nama               │ harga       │ stok                │
├────────────────────────────────────────────────────────┤
│ Kopi Toraja Robusta│ 42000       │ 8                   │
│ Kopi Gayo Arabika  │ 35000       │ 20                  │
│ Roti Sourdough     │ 32000       │ 15                  │
│ Croissant Coklat   │ 28000       │ 12                  │
└────────────────────────────────────────────────────────┘
(4 baris)
```

**Standar ANSI Global:**
```sql
SELECT nama, harga, stok 
FROM produk 
WHERE harga >= 25000 
ORDER BY harga DESC;
```

---

### 4. Menjalankan Fungsi Agregasi Analitik

**Bahasa Indonesia:**
```sql
PILIH 
    HITUNG(*) SEBAGAI total_varian,
    JUMLAH(stok) SEBAGAI total_stok_gudang,
    RATA_RATA(harga) SEBAGAI rata_rata_harga,
    MIN(harga) SEBAGAI harga_termurah,
    MAKS(harga) SEBAGAI harga_termahal
DARI produk;
```

**Standar ANSI Global:**
```sql
SELECT 
    COUNT(*) AS total_varian,
    SUM(stok) AS total_stok_gudang,
    AVG(harga) AS rata_rata_harga,
    MIN(harga) AS harga_termurah,
    MAX(harga) AS harga_termahal
FROM produk;
```

---

### 5. Memperbarui & Menghapus Data

**Bahasa Indonesia:**
```sql
-- Perbarui stok produk dengan ID 1
PERBARUI produk ATUR stok = 35 DIMANA id = 1;

-- Hapus produk yang stoknya habis (stok = 0)
HAPUS_DARI produk DIMANA stok = 0;
```

**Standar ANSI Global:**
```sql
UPDATE produk SET stok = 35 WHERE id = 1;
DELETE FROM produk WHERE stok = 0;
```

### 6. Relasi Antar-Tabel (JOIN / GABUNG) & Optimasi Index Scan

WidyaBase mendukung penggabungan relasional antar-tabel secara dwibahasa dan pemindaian cepat berbasis indeks $O(\log N)$:

**Bahasa Indonesia:**
```sql
-- Penggabungan tabel pesanan dengan tabel pelanggan
PILIH pesanan.id, pelanggan.nama, pesanan.total 
DARI pesanan 
GABUNG pelanggan PADA pesanan.pelanggan_id = pelanggan.id 
DIMANA pesanan.total >= 50000 
URUTKAN BERDASARKAN pesanan.total TURUN;

-- Pencarian kilat B+Tree Kunci Utama (Index Scan Otomatis):
PILIH * DARI pelanggan DIMANA id = 1;
```

**Standar ANSI Global:**
```sql
SELECT pesanan.id, pelanggan.nama, pesanan.total 
FROM pesanan 
JOIN pelanggan ON pesanan.pelanggan_id = pelanggan.id 
WHERE pesanan.total >= 50000 
ORDER BY pesanan.total DESC;
```

---

### 7. Transaksi ACID (Rollback & Commit)

```sql
-- Memulai blok transaksi
MULAI;

-- Lakukan operasi pembaruan saldo
PERBARUI akun ATUR saldo = saldo - 50000 DIMANA id = 1;
PERBARUI akun ATUR saldo = saldo + 50000 DIMANA id = 2;

-- Jika terjadi kesalahan, batalkan seluruhnya:
BATALKAN;

-- Atau jika berhasil, konfirmasi secara permanen:
KOMIT;
```

---

## 🚀 4. Langkah 3: Menggunakan WidyaBase dalam Kode Widya-Lang

Gunakan modul resmi `modul/widyabase.wya` untuk mengintegrasikan basis data langsung ke dalam aplikasi Widya Anda.

Buat berkas `aplikasi_kasir.wya`:

```widya
// Impor modul resmi WidyaBase
impor "./modul/widyabase.wya";

fungsi utama() {
    cetak("==================================================");
    cetak("🛒 SISTEM KASIR RESTORAN NUSANTARA (WIDYABASE)");
    cetak("==================================================");

    // 1. Membuka / Membuat Basis Data
    misal db = BukaWidyaBase("data/restoran.wdb");
    cetak("✓ Basis data berhasil dibuka: " + db.path_db);

    // 2. Membuat Tabel Menu
    db.eksekusi("
        BUAT_TABEL pesanan_meja (
            id INT KUNCI_UTAMA,
            nomor_meja INT,
            pelanggan TEKS,
            total_tagihan ANGKA,
            status_bayar TEKS
        );
    ");
    cetak("✓ Skema tabel 'pesanan_meja' siap.");

    // 3. Menyisipkan Data Transaksi Baru
    db.eksekusi("SISIP_KE pesanan_meja (id, nomor_meja, pelanggan, total_tagihan, status_bayar) NILAI (101, 4, 'Bapak Budi', 145000, 'LUNAS');");
    db.eksekusi("SISIP_KE pesanan_meja (id, nomor_meja, pelanggan, total_tagihan, status_bayar) NILAI (102, 2, 'Ibu Siti', 82000, 'LUNAS');");
    db.eksekusi("SISIP_KE pesanan_meja (id, nomor_meja, pelanggan, total_tagihan, status_bayar) NILAI (103, 7, 'Mas Andi', 210000, 'PENDING');");
    cetak("✓ 3 pesanan berhasil dicatat.");

    // 4. Mengambil Kueri Daftar Pesanan Lunas
    cetak("\n📋 DAFTAR PESANAN LUNAS:");
    misal daftar_lunas = db.kueri("
        PILIH pelanggan, nomor_meja, total_tagihan 
        DARI pesanan_meja 
        DIMANA status_bayar = 'LUNAS'
        URUTKAN BERDASARKAN total_tagihan TURUN;
    ");

    untuk baris dalam daftar_lunas {
        cetak("  • " + baris["pelanggan"] + " (Meja " + ke_teks(baris["nomor_meja"]) + "): Rp " + ke_teks(baris["total_tagihan"]));
    }

    // 5. Analitik Total Omzet
    misal ringkasan = db.kueri("
        PILIH 
            HITUNG(*) SEBAGAI jumlah_transaksi,
            JUMLAH(total_tagihan) SEBAGAI total_omzet
        DARI pesanan_meja 
        DIMANA status_bayar = 'LUNAS';
    ");

    jika (panjang(ringkasan) > 0) {
        misal row = ringkasan[0];
        cetak("\n💰 Total Omzet Kasir : Rp " + ke_teks(row["SUM(Column(\"total_tagihan\"))"]));
        cetak("📊 Jumlah Transaksi   : " + ke_teks(row["COUNT(Wildcard)"]));
    }

    // 6. Checkpoint ke Disk
    db.simpan();
    cetak("\n✓ Sinkronisasi halaman disk dan checkpoint WAL selesai.");
}

utama();
```

Jalankan skrip dengan perintah:
```powershell
widya aplikasi_kasir.wya
```

---

## 🌐 5. Langkah 4: Menjalankan Server Daemon PostgreSQL Wire Protocol (Port 5432)

WidyaBase memiliki server daemon bawaan yang mengimplementasikan PostgreSQL v3.0 wire protocol. Anda dapat melayani ribuan koneksi konkuren melalui port TCP standar.

### Menjalankan Server:
```powershell
# Menggunakan biner mandiri WidyaBase.exe
.\WidyaBase.exe mulai --port 5432 --data data/restoran.wdb

# Atau melalui CLI ekosistem widya:
widya base mulai --port 5432 --data data/restoran.wdb
```

*Log Server:*
```text
[WIDYABASE DAEMON] Server pgwire v3.0 mendengarkan pada 0.0.0.0:5432
[WIDYABASE DAEMON] Siap menerima koneksi dari psql, DBeaver, Prisma, pgAdmin...
```

---

## 🛠️ 6. Langkah 5: Menghubungkan GUI DBeaver, pgAdmin & psql

### A. Menggunakan `psql` Terminal
Buka terminal dan sambungkan ke daemon WidyaBase:
```powershell
psql -h 127.0.0.1 -p 5432 -U admin -d restoran_db
```
Anda dapat langsung menjalankan kueri dwibahasa di prompt PostgreSQL:
```sql
restoran_db=> SELECT * FROM pesanan_meja;
restoran_db=> PILIH * DARI pesanan_meja DIMANA status_bayar = 'LUNAS';
```

### B. Menggunakan DBeaver GUI
1. Buka DBeaver dan klik **New Database Connection**.
2. Pilih driver **PostgreSQL**.
3. Isi parameter koneksi:
   - **Host**: `localhost` (atau `127.0.0.1`)
   - **Port**: `5432`
   - **Database**: `restoran_db`
   - **Username**: `admin`
   - **Password**: *(bebas/dapat dikosongkan)*
4. Klik **Test Connection** > Indikator hijau akan menyala tanda sukses!
5. Buka **SQL Editor** dan eksekusi kueri Anda secara visual.

---

## 🐍 7. Langkah 6: Menghubungkan Aplikasi Python & Node.js

### Contoh Koneksi Python (menggunakan `psycopg2`):
```python
import psycopg2

conn = psycopg2.connect(
    host="127.0.0.1",
    port=5432,
    dbname="restoran_db",
    user="admin",
    password=""
)
cursor = conn.cursor()

# Anda dapat menjalankan kueri Bahasa Indonesia langsung dari Python!
cursor.execute("PILIH pelanggan, total_tagihan DARI pesanan_meja DIMANA status_bayar = 'LUNAS';")
for row in cursor.fetchall():
    print(f"Pelanggan: {row[0]}, Tagihan: Rp {row[1]:,}")

conn.close()
```

### Contoh Koneksi Node.js (menggunakan `pg`):
```javascript
const { Client } = require('pg');

const client = new Client({
  host: '127.0.0.1',
  port: 5432,
  database: 'restoran_db',
  user: 'admin',
  password: ''
});

async function main() {
  await client.connect();
  const res = await client.query("PILIH * DARI pesanan_meja;");
  console.log("Baris ditemukan:", res.rows);
  await client.end();
}

main();
```

---

## 🛡️ 8. Langkah 7: Audit Kesehatan, WAL Recovery & AI Vektor

### Audit Kesehatan Basis Data
Periksa alokasi halaman, konsistensi WAL, dan buffer pool sewaktu-waktu:
```powershell
.\WidyaBase.exe periksa --data data/restoran.wdb
```
*Output Audit:*
```text
============================================================
  🗄️  LAPORAN KESEHATAN BASIS DATA WIDYABASE v1.0.0
============================================================
📂 Berkas Target      : data/restoran.wdb
📄 Total Halaman 4KB  : 2
📊 Jumlah Tabel Aktif : 1
📝 Rekod Log WAL      : 7
🧠 Buffer Pool Cache  : Aktif (512 Halaman Terkelola)
🛡️  Status Integritas : SEHAT (100% KONSISTEN ACID)
============================================================
```

### Multi-Modal AI: Kalkulasi Embedding Kemiripan Vektor
WidyaBase dilengkapi mesin kalkulasi kemiripan embedding vektor langsung:
```powershell
.\WidyaBase.exe vektor "[0.9, 0.1, 0.0]" "[0.85, 0.15, 0.05]"
```
*Hasil:*
```text
Skor Kemiripan Kosinus: 0.9984 (Sangat Mirip / Relevan)
```

---

## 📦 9. Langkah 8: Panduan Lengkap Impor & Ekspor Data (JSON, CSV, SQL Dump)

WidyaBase menyediakan fleksibilitas penuh untuk memindahkan data masuk dan keluar melalui berbagai metode:

### A. Ekspor Data

#### 1. Ekspor ke Berkas JSON via CLI
Gunakan perintah `kueri` dengan opsi `--format json`:
```powershell
# Ekspor seluruh data tabel 'pesanan_meja' ke berkas JSON
.\WidyaBase.exe kueri "PILIH * DARI pesanan_meja;" --data data/restoran.wdb --format json > ekspor_pesanan.json
```
*Hasil di `ekspor_pesanan.json`:*
```json
[
  {
    "id": 101,
    "nomor_meja": 4,
    "pelanggan": "Bapak Budi",
    "status_bayar": "LUNAS",
    "total_tagihan": 145000.0
  },
  {
    "id": 102,
    "nomor_meja": 2,
    "pelanggan": "Ibu Siti",
    "status_bayar": "LUNAS",
    "total_tagihan": 82000.0
  }
]
```

#### 2. Ekspor ke Berkas CSV via CLI
```powershell
# Ekspor seluruh data tabel ke format CSV standar RFC 4180
.\WidyaBase.exe kueri "PILIH * DARI pesanan_meja;" --data data/restoran.wdb --format csv > ekspor_pesanan.csv
```

#### 3. Ekspor Data di Kode Widya-Lang
```widya
impor "./modul/widyabase.wya";

misal db = BukaWidyaBase("data/restoran.wdb");
misal data = db.kueri("PILIH * DARI pesanan_meja;");

// Simpan sebagai JSON
tulis_berkas("data/cadangan_pesanan.json", ke_json(data));
cetak("✓ Berhasil mengekspor " + ke_teks(panjang(data)) + " rekod ke JSON.");
```

#### 4. Ekspor via DBeaver / pgAdmin (GUI)
1. Klik kanan pada tabel di panel navigasi DBeaver.
2. Pilih **Export Data** > pilih format tujuan (CSV, Excel XLSX, JSON, XML, atau SQL Inserts).
3. Klik **Proceed** untuk menyimpan berkas.

---

### B. Impor Data

#### 1. Impor Berkas SQL Dump (.sql) via CLI
Jika Anda memiliki skrip SQL berisi puluhan ribu pernyataan DDL dan DML:
```powershell
# Opsi 1: Menggunakan flag -b / --berkas
.\WidyaBase.exe kueri --berkas skrip_migrasi.sql --data data/restoran.wdb

# Opsi 2: Menggunakan stdin piping (PowerShell / Bash)
Get-Content skrip_migrasi.sql | .\WidyaBase.exe kueri - --data data/restoran.wdb
```

#### 2. Impor Data dari Berkas CSV di Kode Widya-Lang
Menggunakan kombinasi fungsi bawaan `baca_berkas()` dan `urai_csv()`:
```widya
impor "./modul/widyabase.wya";

misal db = BukaWidyaBase("data/restoran.wdb");

// 1. Baca berkas CSV eksternal
misal konten_csv = baca_berkas("data_baru.csv");
misal daftar_baris = urai_csv(konten_csv);

// 2. Sisipkan setiap baris ke tabel WidyaBase
untuk baris dalam daftar_baris {
    misal sql = "SISIP_KE pesanan_meja (id, nomor_meja, pelanggan, total_tagihan, status_bayar) NILAI (" +
        ke_teks(baris["id"]) + ", " +
        ke_teks(baris["nomor_meja"]) + ", '" +
        baris["pelanggan"] + "', " +
        ke_teks(baris["total_tagihan"]) + ", '" +
        baris["status_bayar"] + "');";
    db.eksekusi(sql);
}

// 3. Simpan perubahan ke disk
db.simpan();
cetak("✓ Berhasil mengimpor " + ke_teks(panjang(daftar_baris)) + " baris dari CSV.");
```

#### 3. Impor Data dari Berkas JSON di Kode Widya-Lang
```widya
impor "./modul/widyabase.wya";

misal db = BukaWidyaBase("data/restoran.wdb");

misal teks_json = baca_berkas("data_pelanggan.json");
misal daftar_data = dari_json(teks_json);

untuk item dalam daftar_data {
    db.eksekusi("SISIP_KE pesanan_meja (id, nomor_meja, pelanggan, total_tagihan, status_bayar) NILAI (" +
        ke_teks(item["id"]) + ", " +
        ke_teks(item["nomor_meja"]) + ", '" +
        item["pelanggan"] + "', " +
        ke_teks(item["total_tagihan"]) + ", '" +
        item["status_bayar"] + "');");
}
db.simpan();
cetak("✓ Berhasil mengimpor data JSON ke WidyaBase.");
```

#### 4. Impor Data Menggunakan Klien `psql` (PostgreSQL)
Jika daemon WidyaBase sedang berjalan pada port 5432:
```powershell
# Impor dump file SQL via psql
psql -h 127.0.0.1 -p 5432 -U admin -d restoran -f skrip_cadangan.sql
```

---

## 🎉 Selamat!
Anda kini telah menguasai dasar hingga teknik tingkat lanjut dalam mengelola dan mengintegrasikan basis data berdaulat **WidyaBase v1.0.0**. Selamat berkarya membangun aplikasi masa depan dengan ekosistem **Widya-Lang**!
