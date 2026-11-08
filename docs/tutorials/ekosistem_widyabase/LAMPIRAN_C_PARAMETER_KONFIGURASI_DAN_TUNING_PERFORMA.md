## Lampiran C: Parameter Konfigurasi & Tuning Performa

### C.1 Berkas Konfigurasi Utama: `widyabase.conf`

Seluruh perilaku operasional mesin basis data WidyaBase dikendalikan melalui berkas konfigurasi terpusat `widyabase.conf`. Konfigurasi ini mendukung pembaruan dinamis tanpa penghentian layanan (*Dynamic Reload*) untuk parameter bertanda `(Reloadable)`.

Berikut katalog lengkap parameter konfigurasi utama WidyaBase:

### C.2 Parameter Memori RAM & Buffer Pool

| Nama Parameter | Nilai Baku | Rentang Rekomendasi | Penjelasan Performa |
| :--- | :--- | :--- | :--- |
| `shared_buffers` | `512MB` | 25% – 40% Total RAM Fisik | Ukuran total kolam penyangga (*Buffer Pool*) untuk menampung halaman 4KB di RAM |
| `work_mem` | `64MB` | 32MB – 256MB per Utas | Batas alokasi memori RAM untuk operasi Sort, Hash Join, dan agregasi sebelum meluap ke disk |
| `maintenance_work_mem` | `256MB` | 512MB – 2GB | Alokasi memori untuk pemeliharaan administratif (pembuatan indeks B+Tree, VACUUM) |
| `effective_cache_size`| `2GB` | 50% – 75% Total RAM Fisik | Asumsi CBO mengenai total memori tembolok sistem operasi yang tersedia untuk caching |
| `max_connections` | `200` | 100 – 1.000 | Jumlah koneksi klien konkuren maksimum yang diizinkan terhubung ke peladen |

### C.3 Parameter Transaksi, WAL & Checkpointing

| Nama Parameter | Nilai Baku | Rentang Rekomendasi | Penjelasan Performa |
| :--- | :--- | :--- | :--- |
| `wal_level` | `replica` | `minimal`, `replica`, `logical` | Kedalaman pencatatan informasi WAL (wajib `replica` untuk Raft & CDC) |
| `fsync` | `on` | **Wajib `on` untuk Keamanan ACID**| Menjamin penulisan fisik ke SSD via sistem panggilan kernel Linux `fsync()` |
| `synchronous_commit` | `on` | `on`, `off`, `local`, `remote_apply`| Mengontrol apakah transaksi menunggu WAL tersiram ke disk sebelum respons klien |
| `wal_buffers` | `16MB` | 16MB – 64MB | Ukuran penyangga memori sebelum catatan log WAL dialirkan ke berkas disk |
| `max_wal_size` | `4GB` | 2GB – 32GB | Batas akumulasi volume WAL maksimum sebelum memicu checkpoint otomatis |
| `checkpoint_timeout` | `15m` | 5m – 30m | Batas interval waktu maksimum antar siklus fuzzy checkpointing |
| `checkpoint_completion_target`| `0.9`| 0.7 – 0.9 | Menghaluskan I/O penulisan halaman kotor sepanjang interval checkpoint (*I/O Smoothing*) |

### C.4 Parameter Mesin Khusus (Vektor AI, HNSW & Deret Waktu)

| Nama Parameter | Nilai Baku | Rekomendasi Kasus | Penjelasan Performa |
| :--- | :--- | :--- | :--- |
| `hnsw.default_m` | `16` | 16 – 64 | Jumlah tautan tetangga per node pada indeks graf vektor AI |
| `hnsw.default_ef_construction`| `64` | 64 – 200 | Kedalaman antrean pencarian kandidat saat membangun indeks HNSW |
| `hnsw.default_ef_search` | `40` | 20 – 100 | Kedalaman penelusuran graf saat kueri k-NN ANN berlangsung |
| `timeseries.default_chunk_interval`| `1d` | 1h – 7d | Ukuran rentang partisi waktu otomatis pada pembuatan Hypertable |
| `timeseries.gorilla_compression` | `on` | `on` | Mengaktifkan kompresi bit Delta-of-Delta dan XOR Float64 |

### C.5 Rekomendasi Profil Tuning Sesuai Beban Kerja

1. **Profil OLTP Transaksi Keuangan Perbankan (Throughput & Latensi Rendah)**:
   ```ini
   # widyabase.conf - Profil Perbankan Misi Kritis
   shared_buffers = 16GB              # 25% dari server 64GB RAM
   work_mem = 64MB
   maintenance_work_mem = 2GB
   fsync = on                         # Kepatuhan ACID mutlak
   synchronous_commit = on
   checkpoint_timeout = 15m
   checkpoint_completion_target = 0.9
   max_wal_size = 16GB
   wal_buffers = 32MB
   max_connections = 500
   ```
2. **Profil Data Warehouse Analitis OLAP (Pemrosesan Kolom Agregasi Masif)**:
   ```ini
   # widyabase.conf - Profil Gudang Data Analitis
   shared_buffers = 8GB
   work_mem = 512MB                   # Alokasi besar untuk Hash Join raksasa
   maintenance_work_mem = 4GB
   synchronous_commit = off           # Dioptimalkan untuk kecepatan batch ingestion
   max_wal_size = 32GB
   checkpoint_timeout = 30m
   effective_cache_size = 48GB        # Memanfaatkan tembolok kernel Linux
   ```
