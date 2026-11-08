# Lampiran A: Glosarium Istilah & Sintaksis Lengkap API WidyaMobile

Lampiran ini menyajikan referensi komprehensif mengenai istilah teknis rekayasa seluler dan dokumentasi sintaksis seluruh pustaka standar modul `modul::mobile` serta fungsi bawaan runtime `src/stdlib/mobile_native.rs`.

---

## 1. Glosarium Istilah Rekayasa Seluler Berdaulat

| Istilah Teknis | Definisi & Konteks Rekayasa |
| :--- | :--- |
| **AXML (Android Binary XML)** | Format XML biner terkompresi berstandar AOSP yang digunakan oleh sistem operasi Android untuk membaca berkas `AndroidManifest.xml` secara efisien di memori. |
| **App Clip / Instant App** | Potongan biner aplikasi mikro berukuran di bawah 10–15 MB yang dapat dijalankan secara instan via pemindaian NFC/QR tanpa instalasi penuh. |
| **APDU (Protocol Data Unit)** | Struktur format paket perintah dan respon biner berstandar ISO/IEC 7816-4 yang digunakan untuk berkomunikasi dengan kartu pintar chip perbankan. |
| **BLE (Bluetooth Low Energy)** | Protokol komunikasi nirkabel jarak dekat 2.4 GHz berdaya ultra-rendah yang dirancang untuk transmisi data hemat energi ke perangkat periferal. |
| **CRDT (Conflict-Free Replicated Data Types)** | Struktur data terdistribusi yang menjamin konvergensi konsistensi data multi-perangkat secara otomatis tanpa benturan dan tanpa penguncian terpusat. |
| **DEX (Dalvik Executable)** | Format berkas biner eksekusi mesin virtual Android (berstandar v035) yang memuat bytecode teroptimasi untuk arsitektur memori rendah. |
| **Dynamic Island** | Komponen antarmuka pulau interaktif kamera pada iOS yang dapat mengembang dan menyusut untuk menampilkan informasi status aktif secara real-time. |
| **EMV Contactless Level 2** | Spesifikasi protokol pembayaran kartu perbankan nirsentuh yang mengatur otentikasi kriptografis kartu dan pembangkitan token transaksi ARQC. |
| **Geofencing** | Pemantauan batas perimeter geografis virtual di permukaan bumi menggunakan kombinasi satelit GNSS, Cell-ID, dan sinyal Wi-Fi. |
| **Haversine Formula** | Formulasi trigonometri bola bumi untuk menghitung jarak lingkaran besar terpendek antara dua koordinat garis lintang dan bujur. |
| **LRA (Linear Resonant Actuator)** | Aktuator getar elektromagnetik presisi tinggi yang mampu merespons dalam 5 milidetik untuk menghasilkan umpan balik haptik taktil yang tajam. |
| **Mach-O 64-bit ARM64** | Format biner eksekusi native pada sistem operasi Apple iOS dan macOS untuk prosesor arsitektur 64-bit berafiliasi magic number `0xFEEDFACF`. |
| **NPU (Neural Processing Unit)** | Akselerator perangkat keras pada SoC ponsel yang dioptimalkan untuk mengeksekusi operasi tensor model kecerdasan buatan secara instan dan efisien. |
| **Outbox Pattern** | Pola arsitektur ketahanan di mana seluruh aksi transaksi disimpan secara lokal di basis data sebelum dikirimkan ke server saat jaringan pulih. |
| **PBKDF2 HMAC-SHA512** | Algoritma peregangan kunci kriptografis yang mengulang proses hashing ribuan kali guna mencegah serangan tebak kata sandi secara brute-force. |
| **Public Key Pinning (HPKP)** | Pembatasan validasi sertifikat SSL/TLS hanya pada sidik jari hash SHA-256 kunci publik resmi server guna mencegah serangan Man-in-the-Middle. |
| **QRIS (Quick Response Code Indonesian Standard)** | Standar pembayaran kode QR nasional Republik Indonesia berlandaskan spesifikasi global EMVCo yang diatur oleh Bank Indonesia dan ASPI. |
| **RASP (Runtime Application Self-Protection)** | Sistem perlindungan diri aplikasi yang aktif mendeteksi ada tidaknya rooting, jailbreaking, debugger, emulator, dan manipulasi memori. |
| **Safe Area Inset** | Wilayah penyangga batas layar aman dari takik kamera depan, Dynamic Island, dan bar navigasi sistem operasi. |
| **SoftPOS (Software Point of Sale)** | Transformasi ponsel cerdas komersial menjadi terminal EDC penerima kartu debit/kredit nirsentuh berstandar keamanan PCI-CPoC. |

---

## 2. Sintaksis Lengkap API Pustaka `modul::mobile`

### Konstruktor Aplikasi & Audit
- `BuatAplikasiMobile(nama, tema)` -> Mengembalikan objek `AplikasiMobile` yang membungkus seluruh modul perangkat keras.
- `AuditKesiapanAplikasiMobile()` -> Mengevaluasi kepatuhan 10 pilar seluler dan mengembalikan skor kesiapan enterprise (0–100).

### Pilar 1: Sensor & Perangkat Keras Seluler
- `InisialisasiGPS(akurasi_meter, timeout_ms)` -> Mengaktifkan modul GPS satelit dengan batas toleransi meter.
- `HitungJarakGeografis(lat1, lon1, lat2, lon2)` -> Menghitung jarak riil antar-koordinat menggunakan rumus Haversine (meter).
- `PeriksaGeofence(lat, lon, center_lat, center_lon, radius_m)` -> Mengembalikan boolean apakah posisi berada di dalam radius.
- `InisialisasiSensor(tipe_sensor)` -> Menginisialisasi sensor MEMS (akselerometer, giroskop, barometer).
- `GetarHaptik(pola_larik_ms)` -> Memicu aktuator haptik taktil berdasarkan durasi milidetik getar dan jeda.
- `PeriksaStatusBaterai()` -> Membaca persentase daya (0–100%), status cas, mode hemat energi, dan suhu baterai (°C).

### Pilar 3: Keamanan & Brankas Kunci Kriptografi
- `MintaOtentikasiBiometrik(alasan, metode)` -> Memunculkan dialog FaceID/Sidik Jari Class 3 Strong terikat Crypto-Object.
- `SimpanKunciAman(kunci, nilai)` -> Menyimpan rahasia ke dalam Android Keystore StrongBox / iOS Secure Enclave.
- `AmbilKunciAman(kunci)` -> Membaca kembali nilai rahasia dari brankas chip perangkat keras terisolasi.
- `HapusKunciAman(kunci)` -> Menghapus dan memusnahkan kunci rahasia secara permanen dari silikon hardware.

### Pilar 5: Kamera & Scanner QRIS Nasional
- `AmbilFotoKamera(id_kamera)` -> Mengambil foto satu frame resolusi penuh dan mengembalikan URI penyimpanan.
- `RekamVideoSingkat(durasi_detik)` -> Merekam video terkompresi H.265 hardware terakselerasi berdurasi N detik.
- `PindaiKodeQR()` -> Mengambil frame kamera dan menguraikan string payload kode QR.
- `UraiKodeQRIS(string_raw_qris)` -> Mengurai TLV EMVCo, memvalidasi CRC-16, dan mengembalikan nominal, merchant, dan kota.

### Pilar 6: Notifikasi Lokal & Push Notification
- `KirimNotifikasiLokal(judul, pesan, saluran)` -> Menampilkan notifikasi instan pada saluran tertentu.
- `JadwalkanNotifikasiLokal(judul, pesan, tunda_detik, saluran)` -> Menjadwalkan alarm notifikasi tepat waktu via AlarmManager.
- `BatalkanNotifikasiLokal(id_notif)` -> Membatalkan jadwal notifikasi yang belum terpicu.
- `DapatkanTokenPushFCM()` -> Mendaftarkan dan mengembalikan token unik pendaftaran push gateway OS.

### Pilar 7: Antrean Offline Outbox & Network Monitor
- `PeriksaKoneksiJaringan()` -> Memeriksa apakah internet aktif dan mengembalikan tipe antarmuka (Wi-Fi/Seluler/Offline).
- `CatatAksiOffline(tipe_aksi, data_aksi)` -> Menyimpan transaksi ke antrean outbox terenkripsi secara atomik.
- `DaftarAksiOffline()` -> Mengambil daftar seluruh aksi transaksi yang sedang menunggu sinkronisasi.
- `HapusAksiOffline(id_aksi)` -> Menghapus aksi dari antrean setelah server mengonfirmasi respon 200 OK.
- `ProsesSinkronisasiOffline()` -> Memproses seluruh antrean outbox ke server pusat secara berurutan.

### Pilar 8: Bluetooth Low Energy & Printer Thermal POS
- `PindaiPerangkatBluetooth()` -> Memindai perangkat BLE sekitar dan mengembalikan larik nama, MAC, dan RSSI.
- `HubungkanPerangkatBluetooth(mac_address)` -> Membuka saluran GATT dan menegosiasikan kapasitas buffer MTU 512 byte.
- `CetakStrukThermalPOS(data_struk)` -> Mengompilasi format ESC/POS dan mencetak struk belanja ke printer thermal.

### Pilar 9: Share Sheet, Deep Links & Papan Klip
- `BagikanKonten(teks, url, judul_dialog)` -> Membuka Native Share Sheet sistem operasi (FileProvider content://).
- `AmbilTautanDalam()` -> Mengambil URL Deep Link / Universal Link pembuka aplikasi.
- `SalinKePapanKlip(teks)` -> Menyalin teks ke papan klip kripto dengan batas kedaluwarsa otomatis 60 detik.
- `AmbilDariPapanKlip()` -> Membaca isi teks yang sedang tersimpan di papan klip sistem.

### Pilar 10: Basis Data Terenkripsi Lokal
- `BukaBasisDataMobile(nama_db, kunci)` -> Membuka berkas basis data terenkripsi AES-256 sektor 4KB (SQLCipher WAL).
- `EksekusiKueriMobile(db, sql, params)` -> Mengeksekusi kueri manipulasi data (INSERT, UPDATE, DELETE).
- `AmbilDataMobile(db, sql, params)` -> Mengeksekusi kueri penarikan data (SELECT) dan mengembalikan larik baris objek.

---

## 3. Sintaksis Komponen Antarmuka Deklaratif (Widgets)

```widya
Aplikasi({ "judul": String, "tema": String, "badan": Halaman })
Halaman({ "navigasi": BilahAtas, "badan": Kolom / Baris })
BilahAtas({ "judul": String, "warna": HexString })
Kolom([ Widget1, Widget2, ... ])
Baris([ Widget1, Widget2, ... ])
Kartu({ "elevasi": Number, "isi": Widget })
TombolUtama(label: String, aksi: Fungsi)
TombolAksiMengambangWidget(ikon: String)
ItemDaftarWidget(judul: String, subjudul: String, ikon_awal: String, aksi_akhir: String)
LencanaWidget(teks: String, warna: String)
SakelarWidget(label: String, aktif: Boolean)
IndikatorProgresWidget(nilai: Number)
AvatarPengguna(nama: String, online: Boolean)
BilahNavigasiBawahWidget(items: Array)
LembarBawahWidget(judul: String, isi: Widget)
```
