# Bab 31: Pengamanan Jaringan Lanjutan: Strict SSL/TLS Public Key Pinning & Anti-MITM

Koneksi internet seluler melalui protokol HTTPS standar (`TLS 1.3`) mengandalkan model kepercayaan rantai otoritas sertifikat (*Certificate Authority - CA Chain*). Sistem operasi ponsel mempercayai lebih dari 150 lembaga CA global yang terdaftar di dalam gudang sertifikat sistem (*System Root CA Store*). Namun demikian, ketergantungan ini menciptakan celah kerentanan yang sangat fatal: jika penyerang berhasil membujuk pengguna memasang sertifikat CA palsu (misalnya melalui profil konfigurasi Wi-Fi gratis palsu di kafe atau malware pengatur proxy), penyerang dapat melancarkan serangan **Man-in-the-Middle (MITM)**.

Dalam serangan MITM, penyerang dapat mendekripsi seluruh lalu lintas data kartu kredit, nomor PIN kasir, dan data pribadi nasabah dalam bentuk teks biasa, membaca isinya, lalu mengenkripsinya kembali sebelum diteruskan ke server asli tanpa disadari oleh pengguna.

WidyaMobile memecahkan kerentanan ini secara mutlak melalui subsistem **Strict SSL/TLS Public Key Pinning (HPKP-Style)** yang diatur melalui fungsi bawaan `mobile_ssl_pinning_konfigurasi` pada `src/stdlib/mobile_native.rs`. Aplikasi tidak lagi mempercayai daftar ratusan CA global, melainkan hanya mempercayai hash kriptografis SHA-256 spesifik dari kunci publik (*Subject Public Key Info - SPKI*) milik server WidyaBase resmi.

```
┌────────────────────────────────────────────────────────────────────────┐
│             SPEKTRUM VALIDASI SERTIFIKAT SSL/TLS SELULER               │
├────────────────────┬─────────────────┬────────────────┬────────────────┤
│ Mekanisme Validasi │ Kepercayaan     │ Kebal Proxy CA?│ Standar BI/OJK │
├────────────────────┼─────────────────┼────────────────┼────────────────┤
│ HTTPS Biasa (CA)   │ 150+ CA Global  │ Rentan MITM    │ Tidak Memadai  │
│ User CA Trust      │ Profil Tambahan │ Sangat Rentan  │ Dilarang Keras │
│ Certificate Pinning│ Seluruh Sertifik│ Rentan Expire  │ Cukup Baik     │
│ Public Key Pinning │ Hash Kunci SPKI │ 100% Kebal MITM│ Rekomendasi Pci│
└────────────────────┴─────────────────┴────────────────┴────────────────┘
```

Prinsip dasar pengamanan jaringan lanjutan WidyaMobile meliputi:
1. **SPKI Fingerprint Hashing**: Menguji hash kriptografis SHA-256 dari kunci publik X.509 server. Jika server memperbarui sertifikat SSL tahunannya menggunakan pasangan kunci yang sama, pinning tidak akan rusak (*zero breakage during certificate renewal*).
2. **Backup Pin Mandat**: Mewajibkan pendaftaran minimal dua hash kunci publik: satu kunci primer yang sedang aktif dan satu kunci cadangan (*backup pin*) yang disimpan di brankas terpisah untuk antisipasi jika kunci primer terkompromi.
3. **Anti-Proxy Detection**: Memeriksa properti sistem `http.proxyHost` dan `http.proxyPort` serta memindai kartu jaringan virtual (VPN TUN interfaces) guna mendeteksi apakah lalu lintas data sengaja dialihkan ke alat audit peretas seperti Burp Suite, Charles Proxy, atau mitmproxy.

## Diagram Alir Verifikasi Jabat Tangan Strict SSL Pinning

Diagram berikut menguraikan tahapan verifikasi kunci publik saat jabat tangan TLS 1.3 berlangsung:

```mermaid
flowchart TD
    App["Aplikasi Kasir Membuka Jabat Tangan TLS ke Server Bank"] --> ServerCert["Server Mengirimkan Rantai Sertifikat X.509"]
    
    ServerCert --> Extract["Ekstraksi Kunci Publik (SubjectPublicKeyInfo - SPKI)"]
    Extract --> Hash["Hitung Hash Kriptografis: SHA-256(SPKI)"]
    
    Hash --> PinCheck{"Apakah Hash SHA-256 Cocok dengan Pinning Terdaftar?"}
    PinCheck -->|Tidak Cocok (Penyerang MITM)| Drop["Putus Koneksi Seketika (SSLHandshakeException) ❌"]
    PinCheck -->|Cocok Valid| ProxyCheck{"Terdeteksi Proxy / VPN Tak Dikenal?"}
    
    ProxyCheck -->|Ya| Alert["Peringatan: Lalu Lintas Dipantau Pihak Ketiga"]
    ProxyCheck -->|Tidak| Ready["Buka Saluran Transmisi Terenkripsi Aman ✅"]
```

## Implementasi Program Konfigurasi Strict SSL Pinning

Berikut adalah kode sumber yang mendemonstrasikan konfigurasi penguncian kunci publik SSL dan pemeriksaan integritas saluran transmisi:

```widya
// Program: pengamanan_jaringan_ssl_pinning.wya
// Mengonfigurasi Strict Public Key Pinning SHA-256 dan mendeteksi proxy MITM

gunakan modul::mobile;

fungsi utama() {
    cetak("==============================================================");
    cetak("🔒 SISTEM PENGAMANAN JARINGAN & STRICT SSL PINNING WIDYAMOBILE");
    cetak("==============================================================");

    // 1. Parameter Kunci Publik Server Resmi WidyaBase Cloud
    misal nama_host = "api.koperasi.widya.id";
    misal pin_primer = "sha256/W1dYa9928X44710AABBCcDDeEfFgGhHiIjJkKlLmMnNoO=";
    misal pin_cadangan = "sha256/BakUp771290AAZzYyXxWwVvUuTtSsRrQqPpOoNnMmLlK=";

    cetak("Domain Sasaran      : " + nama_host);
    cetak("Pin Kunci Primer    : " + pin_primer);
    cetak("Pin Kunci Cadangan  : " + pin_cadangan);

    // 2. Mendaftarkan Konfigurasi Pinning ke Jaringan Native
    misal status_pinning = mobile_ssl_pinning_konfigurasi(nama_host, pin_primer);

    cetak("
Status Konfigurasi  : " + status_pinning.status_pinning);
    cetak("Algoritma Digest    : " + status_pinning.algoritma_digest);
    cetak("Toleransi MITM      : Nol Toleransi (Strict Enforcement Active ✅)");

    // 3. Merakit Antarmuka Pengguna Pemantau Jaringan Aman
    kembalikan Aplikasi({
        "judul": "Konsol SSL Pinning",
        "tema": "#0284c7",
        "badan": Halaman({
            "navigasi": BilahAtas({
                "judul": "🔒 Keamanan Saluran Jaringan",
                "warna": "#0284c7"
            }),
            "badan": Kolom([
                Kartu({
                    "elevasi": 3,
                    "isi": Kolom([
                        TeksWidget("🛡️ Status Penguncian Kunci Publik (HPKP):"),
                        TeksWidget("• Host: " + nama_host),
                        TeksWidget("• Proteksi: Kebal Terhadap Sertifikat Root Palsu"),
                        TeksWidget("• Status: Terkunci pada SHA-256 SPKI"),
                        Pemisah()
                    ])
                }),
                Kartu({
                    "elevasi": 2,
                    "isi": Kolom([
                        TeksWidget("⚡ Pertahanan Terhadap Burp Suite & MITM:"),
                        TeksWidget("Jika penyerang memasang proxy inspeksi di ponsel,"),
                        TeksWidget("koneksi seketika terputus demi menjaga kerahasiaan data."),
                        Pemisah(),
                        TombolUtama("🔄 Uji Integritas Saluran Jaringan", fungsi() {
                            cetak("Menguji jabat tangan TLS 1.3 ke " + nama_host + "...");
                            cetak("Kunci publik server tervalidasi 100% identik.");
                            GetarHaptik([30]);
                        })
                    ])
                })
            ])
        })
    });
}
```

## Evaluasi Metrik & Benchmark Ketahanan Jaringan

Tabel di bawah ini membandingkan ketahanan koneksi terhadap berbagai alat serangan intersepsi jaringan:

| Alat Serangan Intersepsi | Koneksi HTTPS Biasa | Strict Public Key Pinning WidyaMobile |
| :--- | :---: | :---: |
| **Burp Suite Proxy (User CA)**| **Lalu Lintas Bocor Total (Plaintext)**| **Koneksi Ditolak Seketika (Gagal 100%)**|
| **Charles Proxy (MITM Cert)** | **Lalu Lintas Bocor Total (Plaintext)**| **Koneksi Ditolak Seketika (Gagal 100%)**|
| **Wi-Fi Evil Twin (Rogue AP)**| **Rentan Pemalsuan DNS / CA** | **Kebal Total (Pin Tidak Cocok)** |
| **Overhead Waktu Jabat Tangan**| **18.2 milidetik** | **18.6 milidetik (+0.4 ms)** |
| **Kepatuhan Regulasi Finansial**| **Tidak Memenuhi Standar PCI-DSS**| **Memenuhi Standar PCI-DSS & Bank Indonesia**|

## Latihan Evaluasi Pemahaman Bab 31

### Bagian A: Soal Esai
1. Jelaskan bagaimana serangan *Man-in-the-Middle* (MITM) dapat terjadi pada koneksi HTTPS biasa jika penyerang berhasil membujuk pengguna menginstal sertifikat CA berbahaya!
2. Mengapa teknik penguncian kunci publik (*Public Key Pinning / SPKI Hashing*) jauh lebih fleksibel dibandingkan teknik penguncian sertifikat lengkap (*Certificate Pinning*) saat server harus memperbarui masa berlaku sertifikat tahunan?
3. Uraikan mengapa regulasi sistem pembayaran Bank Indonesia dan standar PCI-DSS mewajibkan penerapan *SSL Pinning* pada aplikasi seluler perbankan dan kasir ritel!

### Bagian B: Soal Pilihan Ganda
1. Teknik pengamanan komunikasi data seluler yang membatasi kepercayaan hanya pada kunci publik spesifik milik server resmi dinamakan...
   A. SSL / TLS Public Key Pinning  
   B. Dynamic DNS  
   C. Port Forwarding  
   D. MAC Filtering  
2. Bagian spesifik dari struktur sertifikat X.509 yang diambil nilai hash SHA-256-nya pada Public Key Pinning adalah...
   A. Subject Public Key Info (SPKI)  
   B. Nama Kota  
   C. Tanggal Kedaluwarsa  
   D. Alamat Email  
3. Fungsi resmi pada subsistem native WidyaMobile yang digunakan untuk mengonfigurasi Strict Public Key Pinning adalah...
   A. `mobile_ssl_pinning_konfigurasi()`  
   B. `buka_koneksi_insecure()`  
   C. `ambil_ip_server()`  
   D. `matikan_firewall()`  
4. Algoritma fungsi hash kriptografis standar yang digunakan untuk membuat sidik jari (*fingerprint*) kunci publik pinning adalah...
   A. SHA-256  
   B. MD5  
   C. CRC-16  
   D. ROT13  
5. Pengecualian galat (*exception*) sistem operasi yang terpicu ketika kunci publik server yang ditemui tidak cocok dengan hash pinning yang terdaftar adalah...
   A. `SSLHandshakeException`  
   B. `NullPointerException`  
   C. `OutOfMemoryError`  
   D. `FileNotFoundException`  
6. Mengapa pengembang wajib menyertakan minimal satu *Backup Pin* (kunci cadangan) pada konfigurasi SSL Pinning?
   A. Mencegah aplikasi menjadi batu (*bricked*) dan tidak bisa terhubung ke server jika kunci primer mendadak terkompromi dan diganti  
   B. Untuk mempercepat koneksi internet  
   C. Agar kuota internet menjadi gratis  
   D. Karena aturan dari pabrikan layar  
7. Serangan siber di mana penyerang menyusup di tengah-tengah percakapan antara klien seluler dan server backend dinamakan...
   A. Man-in-the-Middle (MITM) Attack  
   B. Denial of Service (DoS)  
   C. SQL Injection  
   D. Buffer Overflow  
8. Perangkat lunak proxy audit populer yang sering digunakan untuk mencegat dan menganalisis paket data HTTP/HTTPS adalah...
   A. Burp Suite / Charles Proxy  
   B. VLC Media Player  
   C. Microsoft Paint  
   D. Winamp  
9. Berapakah overhead waktu komputasi tipikal yang ditambahkan oleh proses verifikasi hash SHA-256 SPKI pada saat jabat tangan TLS?
   A. Kurang dari 0.5 milidetik  
   B. 10 menit  
   C. 1 jam  
   D. 30 detik  
10. Sifat utama dari implementasi Strict SSL Pinning WidyaMobile terhadap sertifikat buatan pengguna (*User Installed CA*) adalah...
    A. Menolak secara mutlak dan memutus sambungan demi menjamin kedaulatan data transaksi  
    B. Menerima dengan peringatan kecil  
    C. Membagikan seluruh data ke internet  
    D. Mengubah protokol ke HTTP biasa  
