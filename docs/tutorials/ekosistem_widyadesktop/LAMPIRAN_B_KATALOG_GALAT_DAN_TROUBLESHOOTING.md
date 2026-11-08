# Lampiran B: Katalog Galat & Panduan Troubleshooting

Lampiran ini menyajikan katalog pesan galat (*Error Codes*), penyebab kegagalan teknis, serta solusi pemecahan masalah (*Troubleshooting*) pada pengembangan dan pengoperasian WidyaDesktop Framework.

---

## B.1 Katalog Kode Galat Sistem (*System Error Catalog*)

| Kode Galat | Nama Galat & Deskripsi Masalah | Solusi Rekayasa & Tindakan Perbaikan |
| :--- | :--- | :--- |
| **ERR_IPC_001** | `Invalid IPCToken: Token tidak cocok atau tidak terdaftar` | Pastikan jendela menyertakan header `X-Widya-IPC-Token` resmi yang diberikan saat inisialisasi awal. |
| **ERR_IPC_002** | `RPC Method Not Found: Metode handler belum didaftarkan` | Daftarkan fungsi penangan melalui `app.daftarkan_rpc(nama_metode, handler)` sebelum dipanggil oleh antarmuka. |
| **ERR_IPC_003** | `RPC Execution Timeout: Batas waktu operasi 5.0s terlampaui` | Periksa perangkat keras periferal (printer/scanner) apakah sedang offline atau mengalami hambatan fisik. |
| **ERR_SEC_001** | `Security Lock Mismatch: Hash SHA-256 berkas aset tidak cocok` | Jalankan kompilasi ulang paket melalui `hasilkan_manifest_keamanan_lock` untuk memperbarui berkas `widya_security.lock`. |
| **ERR_SEC_002** | `Strict CSP Violation: Skrip tanpa nonce kriptografis diblokir` | Sertakan atribut `nonce="..."` resmi pada setiap tag `<script>` dan hindari pemanggilan `eval()`. |
| **ERR_SEC_003** | `Crypto Vault Tampered: Tag otentikasi AES-256-GCM tidak cocok` | Ciphertext di diska mengalami kerusakan atau modifikasi ilegal. Pulihkan berkas brankas dari cadangan resmi. |
| **ERR_HW_001** | `Serial Port Unavailable: Port serial COM / tty tidak ditemukan` | Periksa kabel USB serial fisik, pastikan driver UART (CH340/FTDI) terpasang dan port tidak sedang dipakai aplikasi lain. |
| **ERR_HW_002** | `Printer Thermal Offline: Printer ESC/POS tidak merespons` | Periksa kabel daya printer kasir, pastikan penutup kertas tertutup rapat dan gulungan kertas struk tidak habis. |
| **ERR_HW_003** | `Scale Stream Unstable: Angka timbangan digital masih berguncang` | Tunggu hingga piringan timbangan tenang dan bendera stabilitas bernilai `ST` sebelum memproses harga. |
| **ERR_HW_004** | `Modbus CRC Checksum Failed: Paket serial RS-485 terdistorsi` | Periksa pelindung kabel (*shielding*) RS-485 dari gangguan induksi motor listrik dan pastikan baud rate identik. |
| **ERR_BIO_001** | `Smart Card Read Failed: Chip KTP-el tidak terbaca` | Pastikan kartu KTP-el dimasukkan ke slot pembaca kartu dengan posisi chip kuningan menghadap ke arah sensor yang benar. |
| **ERR_BIO_002** | `Fingerprint Mismatch: Skor minutiae sidik jari di bawah 70.0` | Bersihkan permukaan kaca pemindai optik dan tempelkan jari pemohon dengan tekanan wajar tanpa miring. |
| **ERR_WASM_001** | `Fuel Limit Exceeded: Plugin menghabiskan kuota bahan bakar` | Plugin pihak ketiga mengalami perulangan tak terbatas (*infinite loop*) atau komputasi berlebihan. Optimalkan kode plugin. |
| **ERR_WASM_002** | `Memory Out of Bounds: Akses di luar batas memori linier WASM` | Plugin mencoba mengakses memori di luar alokasi 256 KB yang ditentukan. Periksa indeks array pada kode WASM. |
| **ERR_UPD_001** | `Patch Signature Invalid: Tanda tangan Ed25519 tambalan palsu` | Unduhan berkas tambalan biner mengalami korupsi atau diubah oleh pihak ketiga. Unduh ulang dari repositori resmi. |

---

## B.2 Panduan Pemecahan Masalah Perangkat Keras (*Hardware Troubleshooting*)

### 1. Masalah: Printer Thermal Kasir Tidak Mau Mencetak atau Kertas Macet
- **Gejala**: Perintah pencetakan struk dikirimkan, namun printer thermal diam atau mengeluarkan bunyi *beep* berulang-ulang.
- **Langkah Diagnostik**:
  1. Periksa lampu indikator LED pada printer: jika lampu *Error / Paper Out* menyala merah, buka penutup printer dan periksa apakah kertas thermal terpasang terbalik (sisi berpemanas harus menghadap ke kepala cetak).
  2. Buka terminal baris perintah dan jalankan pengujian port serial:
     ```bash
     # Pada Windows PowerShell
     [System.IO.Ports.SerialPort]::getportnames()
     # Pada Linux
     ls -l /dev/usb/lp* /dev/ttyUSB*
     ```
  3. Pastikan izin akses port perangkat keras pada Linux telah diberikan ke grup pengguna:
     ```bash
     sudo usermod -aG lp,dialout $USER
     ```

### 2. Masalah: Laci Kasir Baja (*Cash Drawer*) Tidak Mau Terbuka Otomatis
- **Gejala**: Nota struk kasir berhasil dicetak dan dipotong, namun laci kasir tetap terkunci rapat.
- **Langkah Diagnostik**:
  1. Periksa lubang kunci manual di bagian depan laci kasir: jika posisi kunci diputar ke posisi *Locked*, laci terkunci secara mekanis dan tidak dapat ditarik oleh solenoid elektrik. Putar kunci ke posisi *Vertical / Auto-Kick*.
  2. Periksa kabel penghubung RJ-11 antara laci kasir dengan bagian belakang printer kasir: pastikan kabel tertancap rapat pada port berlabel *DK* (*Drawer Kick*), bukan port Ethernet jaringan.
  3. Pastikan printer thermal mendukung voltase laci kasir yang sesuai (standar 24V untuk printer kasir industri).

### 3. Masalah: Timbangan Digital Kasir Mengembalikan Angka Acak atau Galat Pembacaan
- **Gejala**: Layar antarmuka kasir menampilkan teks acak seperti `???` atau angka berat melompat-lompat liar.
- **Langkah Diagnostik**:
  1. Periksa kesesuaian parameter transmisi serial pada `widya_desktop.json`: sebagian besar timbangan kasir menuntut parameter `9600-8-N-1` atau `9600-7-E-1` (7 data bits, Even parity).
  2. Bersihkan debu atau kotoran yang terselip di bawah piringan timbang fisik (*load cell assembly*).
  3. Lakukan kalibrasi nol (*Zero Tare Calibration*) menggunakan tombol fisik di panel timbangan sebelum menimbang barang komoditas.

---

## B.3 Panduan Pemecahan Masalah Keamanan & Distribusi (*Security Troubleshooting*)

### 1. Masalah: Aplikasi Gagal Terbuka dengan Pesan "Security Lock Mismatch"
- **Penyebab**: Pengembang baru saja mengubah berkas antarmuka (misalnya mengedit `ui/index.html` atau berkas gaya `style.css`), namun lupa memperbarui berkas manifes integritas `widya_security.lock`.
- **Solusi**:
  Jalankan perintah pembuatan ulang manifes keamanan melalui perkakas baris perintah Widya:
  ```bash
  widya build --update-lock
  ```
  Atau panggil fungsi `hasilkan_manifest_keamanan_lock(&proj_path)` dari skrip build proyek.

### 2. Masalah: Penolakan Notarisasi macOS Gatekeeper
- **Penyebab**: Paket aplikasi macOS `.app` atau `.dmg` belum disematkan tiket notarisasi Apple (*Notarization Ticket*) atau tidak ditandatangani dengan sertifikat *Developer ID Application*.
- **Solusi**:
  1. Terapkan penandatanganan digital resmi dan sertakan berkas *entitlements* yang tepat.
  2. Untuk pengujian internal di lingkungan pengembangan, cabut atribut isolasi karantina sistem operasi melalui terminal:
     ```bash
     xattr -cr /Applications/NusantaraPOS.app
     ```
