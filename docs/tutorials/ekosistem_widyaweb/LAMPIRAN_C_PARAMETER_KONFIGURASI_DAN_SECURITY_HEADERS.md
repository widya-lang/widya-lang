# LAMPIRAN C: PARAMETER KONFIGURASI & SECURITY HEADERS

Lampiran ini mendokumentasikan berkas konfigurasi produksi `widyaweb.toml` serta spesifikasi tajuk keamanan (*Security Headers*) yang wajib diterapkan pada peladen web NusantaraNews.

---

## C.1 Naskah Konfigurasi Produksi `widyaweb.toml`

```toml
# widyaweb.toml - NusantaraNews Production Configuration
[peladen]
lingkungan = "production"
alamat_ikat = "0.0.0.0"
port_http = 8080
port_https = 8443
jumlah_thread_pekerja = 16
timeout_baca_detik = 10
timeout_tulis_detik = 30
maksimal_badan_permintaan_mb = 10

[tls]
aktif = true
domain = "nusantaranews.id"
email_acme = "admin@nusantaranews.id"
protokol_minimal = "TLSv1.3"
alpn = ["h3", "h2", "http/1.1"]

[basis_data]
host = "127.0.0.1"
port = 5432
nama_db = "nusantara_production"
pengguna = "nusantara_app"
ukuran_pool_min = 10
ukuran_pool_maks = 50
timeout_pinjam_ms = 3000
masa_hidup_koneksi_menit = 60

[caching_kv]
host = "127.0.0.1"
port = 6379
ukuran_pool = 20
ttl_bawaan_detik = 300
koefisien_xfetch_beta = 1.0

[rate_limit]
kapasitas_ip_publik = 30
isi_per_detik_ip = 5
kapasitas_api_key = 500
isi_per_detik_key = 50

[observabilitas]
aktifkan_prometheus = true
jalur_metrik = "/metrics"
format_log = "json"
tingkat_keparahan_log = "INFO"
```

---

## C.2 Spesifikasi Tajuk Keamanan Peramban (Security Headers)

Tabel berikut merangkum konfigurasi tajuk respons HTTP yang diterapkan secara konsisten pada setiap halaman dan endpoint portal berita:

| Tajuk Keamanan HTTP | Nilai Konfigurasi Resmi | Tujuan Perlindungan Siber |
| :--- | :--- | :--- |
| **Content-Security-Policy** | `default-src 'self'; script-src 'self' 'nonce-{N}'; style-src 'self'; img-src 'self' https://cdn.nusantaranews.id data:; object-src 'none'; frame-ancestors 'none';` | Memblokir serangan XSS, pembajakan skrip, serta injeksi konten asing berbahaya. |
| **Strict-Transport-Security** | `max-age=63072000; includeSubDomains; preload` | Memaksa peramban menggunakan koneksi terenkripsi HTTPS secara permanen (HSTS). |
| **X-Frame-Options** | `DENY` | Mencegah situs disematkan di dalam tag iframe situs lain (Anti-Clickjacking). |
| **X-Content-Type-Options** | `nosniff` | Mencegah peramban melakukan penafsiran tipe MIME di luar yang dideklarasikan. |
| **Referrer-Policy** | `strict-origin-when-cross-origin` | Membatasi kebocoran parameter URL sensitif saat pembaca mengklik tautan keluar. |
| **Permissions-Policy** | `camera=(), microphone=(), geolocation=()` | Menonaktifkan sensor perangkat keras peramban yang tidak diperlukan oleh portal berita. |
