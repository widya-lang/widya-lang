# 🚀 Catatan Rilis Resmi: Widya-Lang v1.0.0 (General Availability)

Selamat datang di rilis stabil perdana **Widya-Lang v1.0.0**! 

Widya-Lang adalah bahasa pemrograman modern berkinerja tinggi yang menggabungkan inferensi tipe statis matematis **Hindley-Milner**, *Full Self-Hosting*, ekspresivitas sintaks dwibahasa (Nusantara & Internasional), serta ekosistem bawaan terpadu untuk **Web** dan **Desktop Native** dengan standar keamanan industri bertingkat (*Defense-in-Depth*).

---

## 🌟 Sorotan Utama Rilis v1.0.0

### 1. 🧠 Kompilator & Runtime Inti (Core Engine)
- **Sistem Tipe Hindley-Milner**: Inferensi tipe otomatis tanpa boilerplate anotasi manual, monomorphization generics, ADT (`pilihan`), dan pattern matching (`cocokkan`).
- **100% Full Self-Hosting**: Kompiler mandiri ditulis seluruhnya dalam bahasa Widya (`compiler_self_hosted/`) dengan 3-stage bootstrap deterministik.
- **Manajemen Memori Bebas Jeda GC**: Mengadopsi *Safe Reference Counting* dengan deteksi siklus Tarjan (*Tarjan Cycle Detector*).
- **Pustaka Standar FFI Lengkap**: Binding resmi tingkat tinggi untuk **SQLite3**, **DuckDB**, **PostgreSQL**, **MySQL**, **Redb**, **SurrealDB**, dan mesin Kriptografi.

### 2. 🌐 WidyaWeb Framework (Kelas Industri)
- **Arsitektur 6-Tier Enterprise Hardening**: Lulus 100% pada 54 uji master test suite tanpa regresi.
- **Protokol Real-time**: Mendukung Server-Sent Events (SSE) dengan auto-reconnect, WebSockets persisten, dan HTTP/2 streaming.
- **Pertahanan Keamanan Berlapis**: Token anti-CSRF dinamis, isolasi origin ketat, sanitasi header, proteksi pembajakan sesi (*Session Hijacking Guard*), dan isolasi worker pool.

### 3. 🖥️ WidyaDesktop Framework (Enterprise Grade A+)
- **Hybrid IPC Native Bridge**: Komunikasi JSON-RPC lokal berlatensi super rendah (`< 1 ms`) antara backend native Rust dan antarmuka web modern mandiri.
- **Manajemen Multi-Window & Modal**: Pembuatan jendela sekunder independen dan dialog konfirmasi modal (`ChildWindowConfig`).
- **Native Splash Screen**: Animasi *fade-out* halus saat inisialisasi state aplikasi.
- **Pencetakan Kasir Thermal POS ESC/POS**: Generator biner langsung untuk printer kasir thermal standar industri (58mm/80mm) lengkap dengan opcode pemotong kertas (`GS V 66 0`).
- **Pipeline Pembaruan Kriptografis Ed25519**: Verifikasi tanda tangan digital biner otomatis untuk proteksi serangan *Man-in-the-Middle* (MitM).
- **Brankas Kripto AES-256-GCM**: Enkripsi dan dekripsi data sensitif pengguna terotentikasi langsung di sisi desktop.

### 4. 📱 WidyaMobile Framework (Enterprise 10 Pilar)
- **Arsitektur Tri-Engine**: Kompilasi satu basis kode untuk Android APK, iOS IPA, dan PWA Offline-First.
- **Pilar 1-4 Fondasi**: Primitif Hardware (GPS Haversine, Geofencing, Akselerometer 3-Axis, Baterai, Haptik), Industrial UI Phone Notch Simulator dengan jam live, Biometrik FaceID & KeyStore AES-256 terisolasi, serta PWA Cache-First dengan mesin audit `widya mobile periksa` (Predikat Enterprise Grade A+ 100/100).
- **Pilar 5-10 Lanjutan**: Kamera & Pemindai QRIS Standar Bank Indonesia (EMVCo/ASPI TLV Decoder), Notifikasi Lokal & Push APNs/FCM, Antrean Offline Outbox dengan sinkronisasi idempoten, Printer POS Thermal Bluetooth ESC/POS mini 58mm/80mm, Native Share Sheet & Deep Links (`widya://`), dan Basis Data Terenkripsi Lokal SQLite/SQLCipher AES-256.
- **Test Suite 100% Lulus**: 15 / 15 pengujian integrasi seluler terverifikasi sempurna.

### 5. 🗄️ WidyaBase Engine & Biner Mandiri (`WidyaBase.exe`)
- **Mesin Basis Data Mandiri Skala Industri**: Halaman Slotted-Page 4096-byte dengan Buffer Pool Manager LRU (512 halaman cache).
- **Ketahanan ACID Penuh**: Write-Ahead Logging (WAL) dengan ARIES Crash Recovery dan LSN sequencing.
- **Isolasi MVCC**: Snapshot Isolation berlatensi rendah (`xmin` / `xmax`), pembaca dan penulis tidak saling memblokir.
- **Indeks B+Tree O(log N) & Vektor HNSW**: Dukungan kueri relasional cepat dan embedding kemiripan kosinus AI multi-modal.
- **SQL Dwibahasa Simetris**: Dialek Nusantara (`PILIH`, `DARI`, `DIMANA`, `SISIP_KE`, dll.) setara 100% tanpa penurunan performa dibanding ANSI SQL (`SELECT`, `FROM`, `WHERE`, dll.).
- **Server Daemon PostgreSQL Wire Protocol v3.0**: Kompatibel langsung dengan `psql`, DBeaver, Prisma, pgAdmin pada port 5432.
- **Biner Mandiri Dedikasi**: Termasuk executable mandiri `WidyaBase.exe` untuk deployment server database terisolasi.

### 6. 🧩 Ekstensi Resmi Visual Studio Code
- Paket resmi **`widya-lang-1.0.0.vsix`** yang menyediakan penyorotan sintaks TextMate dwibahasa, cuplikan kode (*snippets*), integrasi LSP, dan debugger DAP.

---

## 📦 Berkas Unduhan & Integritas Kriptografis

| Berkas Distribusi | Ukuran | SHA-256 Checksum |
| :--- | :---: | :--- |
| **`widya-v1.0.0-windows-x64.zip`** | ~28.0 MB | `75b1a46746734e28b0b4f3605044095e7ecc4c7d7e0a62b600fc9a0905e7e689` |
| **`widya-lang-1.0.0.vsix`** | ~6.5 KB | `9f9c74fa402e1404459a88f6d89b636d4299327439b0438f70dc5a021170dc36` |
| **`Panduan_Pembuatan_Proyek_WidyaWeb.pdf`** | ~1.92 MB | *Buku Panduan Proyek Baru WidyaWeb (44 Halaman Komprehensif & Ramah Pemula)* |
| **`Buku_Panduan_Pemrograman_Widya_Lang.pdf`** | ~1.47 MB | *Buku Jilid 1: Panduan Pemrograman Widya-Lang Edisi Nusantara* |
| **`Buku_Panduan_WidyaBase_Pemula.pdf`** | ~1.36 MB | *Buku Jilid 2: Panduan Basis Data Mandiri WidyaBase Skala Industri* |

### 🔏 Verifikasi Tanda Tangan Digital (Ed25519)
Setiap rilis resmi ditandatangani menggunakan kunci privat rilis vendor:
- **Kunci Publik Vendor (Ed25519)**:
  ```text
  2c6a2895c4b3fce893ca6626c15c9d08dcf4a9a1b3f7d42d9346e8dd4203afd1
  ```
- **Tanda Tangan Digital (`SHA256SUMS.sig`)**:
  ```text
  5b2eb54e71016f3000f7924d602d034063d5343be2bd61c35c618747a6bde45253f3cca03dc802a10a5ea77773ce271af25b1b403adc2501113ed3ece6b29404
  ```

---

## ⚡ Panduan Instalasi Cepat

### Windows (PowerShell):
```powershell
iwr https://get.widya-lang.org/ps1 -useb | iex
```

### Linux / macOS (Bash):
```bash
curl -fsSL https://get.widya-lang.org | sh
```

---

## 📖 Dokumentasi & Komunitas
- **Portal Resmi**: [https://widya-lang.org](https://widya-lang.org)
- **Repositori GitHub**: [https://github.com/widya-lang/widya-lang](https://github.com/widya-lang/widya-lang)
- **Kebijakan Keamanan (RFC 9116)**: [SECURITY.md](SECURITY.md) atau `security@widya-lang.org`
- **Lisensi**: Open Source di bawah lisensi [MIT License](LICENSE).
