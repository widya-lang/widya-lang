# LAMPIRAN B: KATALOG GALAT & TROUBLESHOOTING HTTP GATEWAY

Lampiran ini menyajikan katalog referensi penanganan galat (*Troubleshooting Guide*) komprehensif saat mengoperasikan klaster peladen web WidyaWeb, API Gateway Nusantara, dan konektivitas basis data WidyaBase.

---

## B.1 Katalog Galat HTTP & Strategi Remediasi

### 1. HTTP 429 Too Many Requests
- **Penyebab**: Alamat IP klien atau kunci API melampaui kuota alokasi token per detik pada Token Bucket Rate Limiter.
- **Pemeriksaan**: Periksa tajuk respons `X-RateLimit-Remaining` dan `Retry-After`.
- **Solusi**: Klien harus menghentikan permintaan sesaat dan menerapkan penundaan eksponensial (*Exponential Backoff*). Untuk mitra terverifikasi, naikkan kapasitas kuota di tabel kebijakan gateway.

### 2. HTTP 502 Bad Gateway
- **Penyebab**: API Gateway gagal menghubungi proses peladen upstream backend (misal: klaster REST API port 4000 mati, mengalami crash kehabisan memori OOM, atau soket jaringan tertutup).
- **Pemeriksaan**: Jalankan perintah `curl http://127.0.0.1:4000/healthz` di mesin internal dan periksa log sistem: `journalctl -u nusantara_api -n 50`.
- **Solusi**: Pastikan proses backend berjalan, periksa ketersediaan memori RAM, dan pastikan konfigurasi firewall internal mengizinkan port upstream.

### 3. HTTP 503 Service Unavailable (Circuit Breaker Open)
- **Penyebab**: Modul Circuit Breaker memutus sirkuit ke layanan hilir karena tingkat kegagalan melampaui ambang batas toleransi (misalnya 50% kueri timeout).
- **Pemeriksaan**: Pantau metrik `nusantara_circuit_breaker_state{status="open"}` pada dasbor Grafana.
- **Solusi**: Periksa kesehatan layanan dependensi eksternal (layanan AI atau database). Tunggu hingga masa jeda tidur (*sleep cooldown*) selesai untuk memberi kesempatan sirkuit bertransisi ke status Setengah Terbuka (*Half-Open*).

### 4. HTTP 504 Gateway Timeout
- **Penyebab**: Peladen upstream menerima permintaan tetapi tidak mampu mengembalikan respons dalam batas waktu `timeout_koneksi_ms` (misalnya 5000ms).
- **Pemeriksaan**: Kueri SQL lambat tanpa indeks, thread kolam basis data tersandera kueri besar, atau pemanggilan API pihak ketiga yang menggantung.
- **Solusi**: Optimalkan rencana eksekusi kueri menggunakan `EXPLAIN ANALYZE` di WidyaBase, tambahkan indeks B-Tree yang sesuai, dan terapkan batas timeout pada pemanggilan layanan luar.

---

## B.2 Troubleshooting Konektivitas Basis Data WidyaBase

| Gejala Masalah | Indikator Log Sistem | Tindakan Korektif Direkomendasikan |
| :--- | :--- | :--- |
| **Pool Timeout (Soket Habis)** | `[Pool] Timeout menunggu koneksi kosong (3000ms)` | Naikkan `ukuran_pool_maks` atau periksa kebocoran koneksi (*unclosed connections*) di kode controller. |
| **Disk WAL Penuh (Disk Space Outage)** | `[WAL] Gagal menulis rekod: No space left on device` | Periksa slot replikasi CDC yang tidak aktif (`pg_replication_slots`); hapus slot yang tidak digunakan agar basis data dapat menghapus berkas WAL usang. |
| **Kueri Terkunci (Deadlock Detected)** | `[DB Error] Deadlock detected on table komentar` | Pastikan urutan penguncian baris (`SELECT FOR UPDATE`) selalu konsisten dan simetris di seluruh transaksi konkurensi. |
| **Sertifikat TLS Kedaluwarsa** | `[TLS] Handshake failed: certificate has expired` | Periksa daemon ACME Let's Encrypt atau jalankan pembaruan manual sertifikat melalui `widya cert renew`. |
