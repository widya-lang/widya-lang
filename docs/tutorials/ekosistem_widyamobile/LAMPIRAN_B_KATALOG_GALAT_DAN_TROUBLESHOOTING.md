# Lampiran B: Katalog Galat & Panduan Pemecahan Masalah (Troubleshooting)

Lampiran ini mendokumentasikan 40 kode galat resmi (`MOB-1001` s/d `MOB-5004`) pada runtime WidyaMobile, akar penyebab teknis, serta panduan mitigasi kode solusinya.

---

## 1. Galat Siklus Hidup & Arsitektur Sistem (MOB-1001 s/d MOB-1008)

| Kode Galat | Deskripsi Pesan Galat | Akar Penyebab Teknis | Solusi Perbaikan & Mitigasi |
| :--- | :--- | :--- | :--- |
| **MOB-1001** | `ERR_LIFECYCLE_STATE_LOSS` | Aplikasi dimatikan oleh Low Memory Killer (LMK) saat berada di background tanpa sempat menyimpan snapshot state. | Panggil `mobile_preservasi_state_simpan()` secara deterministik di dalam event callback `onPause()` atau `onStop()`. |
| **MOB-1002** | `ERR_SAFE_AREA_OUT_OF_BOUNDS` | Elemen tombol sentuh krusial ditempatkan di luar batas safe area sehingga tertutup takik atau Dynamic Island. | Gunakan fungsi `mobile_safe_area_insets()` dan terapkan padding atas minimum sebesar `top_notch_inset_px`. |
| **MOB-1003** | `ERR_ANR_MAIN_THREAD_BLOCK` | Utas utama (UI Thread) membeku melebihi 5 detik akibat kueri basis data atau komputasi berat. | Alihkan seluruh kueri basis data dan pembacaan berkas masif ke pekerja latar belakang (*Worker Thread*). |
| **MOB-1004** | `ERR_FOLDABLE_POSTURE_UNDEFINED` | Sudut engsel layar lipat bernilai anomali di luar batas 0° s/d 180° akibat kegagalan sensor HAL. | Terapkan nilai batas pengaman (*clamping*) dan gunakan mode fallback layar tunggal standar jika sensor bernilai nil. |
| **MOB-1005** | `ERR_ADAPTIVE_THEME_NOT_FOUND` | Mesin tema gagal mengekstraksi palet warna dari wallpaper sistem operasi. | Sediakan palet warna cadangan (*fallback color palette*) di dalam deklarasi awal fungsi `BuatAplikasiMobile()`. |
| **MOB-1006** | `ERR_NAVIGATION_STACK_EMPTY` | Pemanggilan operasi `pop()` pada tumpukan navigasi yang sedang berada di halaman akar (*root page*). | Selalu verifikasi kedalaman tumpukan rute sebelum memanggil `pop()`, atau arahkan tombol kembali ke beranda. |
| **MOB-1007** | `ERR_VIRTUAL_LIST_INDEX_OOB` | Pengguliran daftar virtual meminta indeks data di luar batas ukuran array sumber. | Terapkan validasi `indeks >= 0 && indeks < panjang(katalog)` pada callback daur ulang sel `ItemDaftarWidget`. |
| **MOB-1008** | `ERR_SPRING_PHYSICS_DIVERGENCE` | Parameter kekakuan (*stiffness*) atau massa bernilai nol/negatif sehingga simulasi pegas tak hingga. | Pastikan parameter massa > 0.1, kekakuan > 10.0, dan redaman > 1.0 pada konfigurasi model pegas harmonik. |

---

## 2. Galat Sensor & Integrasi Perangkat Keras (MOB-2001 s/d MOB-2008)

| Kode Galat | Deskripsi Pesan Galat | Akar Penyebab Teknis | Solusi Perbaikan & Mitigasi |
| :--- | :--- | :--- | :--- |
| **MOB-2001** | `ERR_SENSOR_HARDWARE_UNAVAILABLE` | Perangkat tidak memiliki sensor fisik tertentu (misalnya sensor giroskop atau barometer). | Lakukan pengecekan ketersediaan sensor sebelum memanggil `mobile_sensor_lengkap_baca()` dan sediakan mode fallback. |
| **MOB-2002** | `ERR_GPS_TIMEOUT_NO_FIX` | Modul GPS satelit gagal mengunci posisi dalam batas batas waktu akibat sinyal terhalang gedung. | Tingkatkan batas timeout ke 15.000 ms dan aktifkan Assisted GPS (AGPS) berbasis triangulasi menara BTS. |
| **MOB-2003** | `ERR_GEOFENCE_RADIUS_TOO_SMALL` | Radius geofence yang didaftarkan kurang dari batas toleransi akurasi hardware (< 10 meter). | Tetapkan radius geofence minimal 30 hingga 50 meter guna mengompensasi distorsi sinyal satelit perkotaan. |
| **MOB-2004** | `ERR_CAMERA_PERMISSION_DENIED` | Pengguna menolak memberikan izin hak akses `CAMERA` pada dialog izin sistem operasi. | Tampilkan dialog edukasi (*Rationale Dialog*) yang menjelaskan mengapa kamera diperlukan sebelum meminta izin ulang. |
| **MOB-2005** | `ERR_CAMERA_HARDWARE_IN_USE` | Kamera sedang dikunci dan digunakan oleh aplikasi lain di sistem operasi. | Tangani pengecualian kamera terkunci dan coba buka kembali kamera setelah jeda waktu 500 milidetik. |
| **MOB-2006** | `ERR_QRIS_INVALID_CRC16` | Checksum Tag 63 pada string QRIS tidak cocok dengan hasil perhitungan hash CRC-16 CCITT. | Tolak kode QR seketika; beri peringatan bahwa kode QR rusak atau ada upaya manipulasi data transaksi. |
| **MOB-2007** | `ERR_QRIS_NON_IDR_CURRENCY` | Tag 53 QRIS memuat kode mata uang selain Rupiah Indonesia (kode selain 360). | Batalkan transaksi; regulasi Bank Indonesia mewajibkan seluruh pembayaran transaksi domestik dalam Rupiah. |
| **MOB-2008** | `ERR_THERMAL_THROTTLING_TRIPPED` | Suhu sensor baterai melampaui 45°C sehingga sistem operasi membatasi performa prosesor. | Turunkan frekuensi layar ke 60Hz, matikan komputasi latar belakang, dan beri peringatan istirahat perangkat. |

---

## 3. Galat Komunikasi Nirkabel & Periferal (MOB-3001 s/d MOB-3008)

| Kode Galat | Deskripsi Pesan Galat | Akar Penyebab Teknis | Solusi Perbaikan & Mitigasi |
| :--- | :--- | :--- | :--- |
| **MOB-3001** | `ERR_BLE_ADAPTER_DISABLED` | Antena Bluetooth perangkat seluler sedang dimatikan oleh pengguna di menu pengaturan. | Minta sistem operasi memunculkan dialog pengaktifan Bluetooth (`ACTION_REQUEST_ENABLE`). |
| **MOB-3002** | `ERR_BLE_DEVICE_OUT_OF_RANGE` | Periferal printer thermal berjarak terlalu jauh sehingga nilai RSSI lebih lemah dari -85 dBm. | Dekatkan ponsel kasir ke printer thermal (jarak optimal < 2 meter) dan pastikan baterai printer terisi. |
| **MOB-3003** | `ERR_BLE_MTU_NEGOTIATION_FAIL` | Periferal menolak negosiasi buffer MTU 512 byte dan memaksa nilai baku 23 byte. | Turunkan ukuran potongan data (*chunk size*) pencetakan struk ke 20 byte saat transmisi data berlangsung. |
| **MOB-3004** | `ERR_ESCPOS_BUFFER_OVERFLOW` | Data struk belanja dikirim terlalu cepat tanpa jeda sehingga buffer RAM printer meluap. | Sisipkan penundaan waktu (*sleep*) 25 milidetik di antara pengiriman paket chunk biner ESC/POS. |
| **MOB-3005** | `ERR_NFC_TAG_LOST_PREMATURE` | Pengguna mengangkat kartu e-Money sebelum pertukaran paket perintah APDU selesai. | Tampilkan animasi panduan agar pengguna menahan kartu secara stabil pada antena belakang selama 1 detik. |
| **MOB-3006** | `ERR_SOFTPOS_EMV_TRANSACTION_DECLINED` | Chip kartu debit/kredit menolak transaksi karena PIN salah atau saldo rekening tidak mencukupi. | Cetak kode penolakan bank dan minta pelanggan menggunakan metode pembayaran lain. |
| **MOB-3007** | `ERR_BLE_MESH_TTL_EXPIRED` | Paket pesan darurat radio mesh kehabisan batas lompatan (TTL = 0) sebelum mencapai simpul tujuan. | Naikkan batas nilai TTL awal ke 7 atau 10 lompatan pada fungsi `mobile_ble_mesh_siarkan_pesan()`. |
| **MOB-3008** | `ERR_HTTP3_QUIC_SOCKET_ERROR` | Pelabuhan UDP 443 diblokir oleh firewall jaringan Wi-Fi publik tempat kasir terhubung. | Aktifkan mekanisme fallback otomatis ke protokol HTTP/2 over TCP pada subsistem jaringan adaptif. |

---

## 4. Galat Penyimpanan, Kriptografi & Keamanan (MOB-4001 s/d MOB-4008)

| Kode Galat | Deskripsi Pesan Galat | Akar Penyebab Teknis | Solusi Perbaikan & Mitigasi |
| :--- | :--- | :--- | :--- |
| **MOB-4001** | `ERR_DATABASE_PASSPHRASE_WRONG` | Kunci master enkripsi yang dimasukkan salah sehingga sektor halaman basis data tidak dapat didekripsi. | Verifikasi kunci master hardware dari Keystore; jangan gunakan string statis yang dapat berubah antar-versi. |
| **MOB-4002** | `ERR_DATABASE_HMAC_CORRUPTED` | Nilai checksum HMAC-SHA512 pada berkas basis data tidak cocok (indikasi berkas dimanipulasi). | Karantina berkas yang rusak dan lakukan pemulihan otomatis dari salinan cadangan (*WAL restore*). |
| **MOB-4003** | `ERR_CRDT_VECTOR_CLOCK_INVERSION` | Terdeteksi pembalikan waktu kausalitas pada jam logis akibat manipulasi jam sistem perangkat. | Terapkan resolusi kausalitas berbasis Lamport Logical Clock yang tidak terpengaruh oleh jam fisik ponsel. |
| **MOB-4004** | `ERR_KEYSTORE_HARDWARE_UNAVAILABLE` | Modul StrongBox / Secure Enclave gagal merespons akibat gangguan mikrokontroler hardware. | Tangani galat perangkat keras dan beralih sementara ke penyimpanan TEE ARM TrustZone terisolasi. |
| **MOB-4005** | `ERR_BIOMETRIC_USER_CANCELLED` | Pengguna menekan tombol 'Batal' pada dialog pemindaian sidik jari atau FaceID. | Hormati keputusan pengguna; kembalikan antarmuka ke layar sebelumnya tanpa memunculkan pesan galat menakutkan. |
| **MOB-4006** | `ERR_RASP_DEVICE_ROOT_DETECTED` | Terdeteksi biner superuser (`su`), Magisk, atau kernel yang dimodifikasi pada perangkat seluler. | Hentikan pemrosesan transaksi finansial seketika dan bersihkan seluruh kunci sesi dari memori kerja. |
| **MOB-4007** | `ERR_RASP_FRIDA_HOOK_DETECTED` | Terdeteksi pustaka injeksi memori dinamis `frida-gadget.so` atau port terbuka 27042. | Musnahkan proses aplikasi secara terhormat (`std::process::exit(1)`) untuk mencegah pembajakan logika. |
| **MOB-4008** | `ERR_SSL_PINNING_HASH_MISMATCH` | Kunci publik server tidak cocok dengan pin SHA-256 yang terdaftar (indikasi serangan proxy MITM). | Putus sambungan jaringan seketika; laporkan domain dan IP mencurigakan ke log telemetri keamanan. |

---

## 5. Galat Kompilasi, Biner & Distribusi (MOB-5001 s/d MOB-5004)

| Kode Galat | Deskripsi Pesan Galat | Akar Penyebab Teknis | Solusi Perbaikan & Mitigasi |
| :--- | :--- | :--- | :--- |
| **MOB-5001** | `ERR_AXML_STRING_POOL_OVERFLOW` | Jumlah string unik pada manifest melampaui kapasitas offset 16-bit AXML generator. | Kompresi dan bersihkan deklarasi string redundan pada manifes paket `pure_apk.rs`. |
| **MOB-5002** | `ERR_DEX_CHECKSUM_MISMATCH` | Perhitungan Adler-32 atau SHA-1 pada berkas `classes.dex` tidak sesuai dengan isi bytecode. | Pastikan urutan perhitungan hash dilakukan setelah seluruh tabel metode dan string selesai disusun. |
| **MOB-5003** | `ERR_MACHO_ALIGNMENT_INVALID` | Segmen biner `LC_SEGMENT_64` tidak selaras dengan batas halaman memori 16KB Apple Silicon. | Terapkan padding nol byte pada segmen `__TEXT` dan `__DATA` agar memenuhi penyelarasan `0x4000` (16KB). |
| **MOB-5004** | `ERR_OTA_DELTA_SIGNATURE_INVALID` | Tanda tangan kriptografis Ed25519 pada paket tambalan `WIDYAMOBILE_DELTA10` tidak sah. | Tolak paket tambalan; jangan pernah menerapkan pembaruan biner yang tidak ditandatangani kunci privat resmi. |
