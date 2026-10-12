# 📦 Panduan Komprehensif Ekosistem Paket Berdaulat WPM (Widya Package Manager)

Dokumen ini merupakan panduan resmi arsitektur, perintah CLI, keamanan rantai pasok (*supply-chain security*), dan tata kelola repositori paket publik **WPM** (Widya Package Manager) untuk bahasa pemrograman **Widya-Lang**.

---

## 📑 Daftar Isi
1. [Pengenalan & Filosofi Kedaulatan Digital](#1-pengenalan--filosofi-kedaulatan-digital)
2. [Format Berkas Manifest (`widya.toml` & `widya.lock`)](#2-format-berkas-manifest-widyatoml--widyalock)
3. [Daftar Perintah CLI Lengkap (Dwibahasa / Bilingual)](#3-daftar-perintah-cli-lengkap-dwibahasa--bilingual)
4. [Katalog 21 Pustaka Resmi Bawaan (10 Mega Frontiers)](#4-katalog-21-pustaka-resmi-bawaan-10-mega-frontiers)
5. [Audit Keamanan Rantai Pasok (`WpmSecurityAuditor`)](#5-audit-keamanan-rantai-pasok-wpmsecurityauditor)
6. [Pemeriksa Dependensi Usang (`WpmOutdatedChecker`)](#6-pemeriksa-dependensi-usang-wpmoutdatedchecker)
7. [Inspeksi Metadata Paket (`info` / `informasi`)](#7-inspeksi-metadata-paket-info--informasi)
8. [Penyediaan Paket Offline & Air-Gapped (`vendor` / `pasok`)](#8-penyediaan-paket-offline--air-gapped-vendor--pasok)
9. [Self-Hosted Private Registry Server & Web Hub Portal](#9-self-hosted-private-registry-server--web-hub-portal)
10. [Spesifikasi REST API Registry](#10-spesifikasi-rest-api-registry)
11. [Kriptografi Ed25519 & Standar Provenance SLSA Level 3](#11-kriptografi-ed25519--standar-provenance-slsa-level-3)

---

## 1. Pengenalan & Filosofi Kedaulatan Digital

**Widya Package Manager (WPM)** dibangun untuk mengatasi kerentanan ekosistem paket modern (seperti *typosquatting*, *dependency confusion*, *malicious lockfile tampering*, dan ketergantungan sepihak pada infrastruktur asing). 

### Karakteristik Kunci WPM:
- **100% Native Rust/Widya Architecture**: Bebas dari runtime eksternal (Zero Python / Zero Node.js).
- **PubGrub Dependency Resolver**: Algoritma SAT-solver modern yang mampu menyelesaikan grafik dependensi berlian (*diamond dependencies*) dengan penjelasan galat yang deterministik.
- **Cryptographic Provenance**: Setiap rilis paket wajib memiliki checksum SHA-256 dan ditandatangani secara kriptografis menggunakan kunci kurva Edwards **Ed25519**.
- **Kedaulatan Dua Mode**: Mendukung Central Registry publik (`wpm.widya-lang.org`) dan Self-Hosted Private Registry lokal untuk lingkungan pemerintahan (*B2G*), perbankan, dan militer yang terisolasi (*Air-Gapped*).

---

## 2. Format Berkas Manifest (`widya.toml` & `widya.lock`)

### A. Berkas Manifest Proyek (`widya.toml`)
```toml
[package]
name = "aplikasi_keuangan"
version = "1.0.0"
authors = ["PT Solusi Digital Merah Putih"]
edition = "2026"
license = "MIT"

[dependencies]
widya-office = "^2.5.0"
widya-sql = ">= 1.0.0, < 2.0.0"
widya-bifast = "= 1.0.0"
widya-crypto = "~ 1.0.0"
```

### B. Berkas Pengunci Integritas (`widya.lock`)
Dihasilkan secara otomatis oleh perintah `widya wpm lock` atau `widya wpm add`:
```toml
[[package]]
name = "widya-office"
version = "2.5.0"
source = "registry+https://registry.widya-lang.org/api/v1"
checksum = "72b269431623bd8ee05b969c3a3bfa5b4b1a206a4b3d7b93b827e8d08cb5f867"
signature = "3a88f0...64bytes"
public_key = "a1b2c3...32bytes"
dependencies = ["widya-sql"]
```

---

## 3. Daftar Perintah CLI Lengkap (Dwibahasa / Bilingual)

WPM dapat diakses menggunakan perintah `widya wpm`, `widya pkg`, maupun `widya package`:

| Perintah Internasional | Alias Nusantara | Deskripsi Penggunaan |
| :--- | :--- | :--- |
| `widya wpm init <nama>` | `widya wpm new <nama>` | Membuat proyek Widya baru dengan `widya.toml` |
| `widya wpm add <paket>` | `widya wpm add <paket> --versi <v>` | Menambahkan dependensi paket dan merekam di lockfile |
| `widya wpm lock` | `widya wpm lock` | Menghitung SHA-256 dan membuat `widya.lock` |
| `widya wpm verify` | `widya wpm verify` | Memvalidasi seluruh checksum pustaka terhadap tampering |
| `widya wpm list` | `widya wpm ls` | Menampilkan seluruh daftar dependensi yang terkunci |
| `widya wpm search <kueri>` | `widya wpm cari <kueri>` | Mencari pustaka pada katalog registry |
| `widya wpm info <paket>` | `widya wpm informasi <paket>` | Menampilkan metadata rinci dan tanda tangan digital |
| `widya wpm audit` | `widya wpm periksa-keamanan` | **Audit Keamanan Rantai Pasok (CVE/WSA, Checksum, Sig)** |
| `widya wpm outdated` | `widya wpm usang` | **Pemeriksa Versi Usang (SemVer Major/Minor/Patch)** |
| `widya wpm vendor` | `widya wpm pasok` | Memasok modul ke direktori lokal untuk mode *Air-Gapped* |
| `widya wpm publish` | `widya wpm publish` | Publikasi pustaka ke Central Registry |
| `widya wpm login <token>` | `widya wpm masuk <token>` | Menyimpan token otentikasi API rilis |
| `widya wpm serve-registry` | `widya wpm layani-registry` | Menjalankan server registry lokal & Web Portal Hub |

---

## 4. Katalog 21 Pustaka Resmi Bawaan (10 Mega Frontiers)

Central Registry WPM dilengkapi dengan 21 paket resmi yang mencakup seluruh spektrum industri nasional:

| No | Nama Paket | Versi | Tag / Domain | Deskripsi Fungsional |
| :-: | :--- | :-: | :--- | :--- |
| 1 | `widya-math` | `1.0.0` | `math, simd, algebra` | Aljabar linier presisi tinggi terakselerasi AVX-512/NEON |
| 2 | `widya-http` | `1.2.0` | `web, http, rest, async` | Klien/server HTTP/HTTPS asinkron berbasis epoll/kqueue |
| 3 | `widya-crypto` | `1.0.0` | `crypto, ed25519, pqc` | Kriptografi PQC (Kyber/Dilithium), Ed25519 & ChaCha20 |
| 4 | `widya-ui` | `0.2.0` | `ui, reactive, canvas` | Kerangka kerja UI reaktif bergaya Flutter dengan GPU canvas |
| 5 | `widya-ai` | `0.9.5` | `ai, autograd, tensor` | Tensor autograd diferensial otomatis & jembatan PyTorch/ONNX |
| 6 | `widya-sql` | `1.1.0` | `database, sql, widyabase` | Driver & ORM WidyaBase (B+Tree, WAL ACID, Vector HNSW) |
| 7 | `widya-office` | `2.5.0` | `office, pdf, docx` | Kompilator dokumen ISO 32000-1 PDF, DOCX OpenXML, XLSX & TTE |
| 8 | `widya-pqc` | `1.0.0` | `pqc, quantum, fips203` | Kriptografi pasca-kuantum NIST FIPS 203 (ML-KEM) & FIPS 204 |
| 9 | `widya-bifast` | `1.0.0` | `fintech, bifast, qris` | Gateway transaksi perbankan nasional BI-FAST ISO 20022 & QRIS |
| 10 | `widya-arkas` | `1.0.0` | `edutech, arkas, bos` | Tata kelola anggaran pendidikan BOS & BKU anti-fraud BPK |
| 11 | `widya-siplah` | `1.0.0` | `siplah, pbj, school` | Integrasi pengadaan barang/jasa satuan pendidikan daring |
| 12 | `widya-wasi` | `0.2.0` | `wasm, wasi, serverless` | Eksekusi serverless WASI Preview 2 Component Model & WIT ABI |
| 13 | `widya-canbus` | `1.0.0` | `automotive, canbus, obd2`| Telemetri otomotif CAN Bus 2.0B / CAN-FD ISO 26262 ASIL-D |
| 14 | `widya-scada` | `1.0.0` | `iiot, scada, modbus` | Otomasi industri IIoT Modbus TCP/RTU, OPC-UA & TSN |
| 15 | `widya-fhir` | `1.0.0` | `healthtech, fhir, dicom` | Rekam medis SatuSehat HL7 FHIR v4, DICOM v3 & DNA sekuens |
| 16 | `widya-quantum`| `1.0.0` | `quantum, openqasm, vqe` | Simulator kuantum OpenQASM 3.0, QIR Alliance & algoritma VQE |
| 17 | `widya-i18n` | `1.0.0` | `i18n, nusantara, aksara`| Lokalisasi 12 Aksara Nusantara & perender Arab RTL |
| 18 | `widya-test` | `1.0.0` | `test, fuzzing, quickcheck`| Framework pengujian berbasis properti QuickCheck & benchmark |
| 19 | `widya-json` | `1.3.0` | `json, simd, parser` | Parser & serialisasi JSON terakselerasi SIMD dengan validator |
| 20 | `widya-grpc` | `1.0.0` | `grpc, protobuf, http2` | RPC terdistribusi berkinerja tinggi berbasis Protobuf & HTTP/2 |
| 21 | `widya-log` | `1.0.0` | `logging, opentelemetry` | Logging terstruktur asinkron dengan integrasi OpenTelemetry APM |

---

## 5. Audit Keamanan Rantai Pasok (`WpmSecurityAuditor`)

Jalankan perintah audit keamanan untuk memindai pohon dependensi:
```bash
widya wpm audit
```

### Mekanisme Validasi:
1. **Basis Data Widya Security Advisories (WSA)**:
   - `WSA-2026-001` (`widya-http` < 1.1.0): Potensi Denial of Service pada chunked stream header (*Medium*).
   - `WSA-2026-002` (`widya-crypto` < 0.9.0): Timing side-channel pada non-constant-time poly1305 lawas (*High*).
   - `WSA-2026-003` (`widya-sql` < 1.0.1): Galat parsing tanda kurung bersarang pada kueri JOIN (*Low*).
2. **Validasi Checksum SHA-256**: Memastikan biner lokal cocok persis dengan rekaman rilis di registry. Bila ada berkas yang dimodifikasi tanpa izin (*tampered*), audit akan langsung memblokir rilis.
3. **Verifikasi Kriptografis Ed25519**: Memastikan seluruh paket ditandatangani oleh pembuat (*author*) yang sah.

---

## 6. Pemeriksa Dependensi Usang (`WpmOutdatedChecker`)

Jalankan perintah berikut untuk memeriksa apakah ada pustaka lokal yang tertinggal dari versi rilis terbaru:
```bash
widya wpm outdated
```

Output menampilkan tabel komparasi SemVer:
```text
================================================================================
  📦 PEMERIKSAAN VERSI PAKET USANG (WPM OUTDATED CHECKER)
================================================================================
  Ditemukan 2 paket dengan pembaruan versi yang tersedia:

  Paket                     Terpasang       Terbaru         Tipe Update
  ------------------------- --------------- --------------- ----------
  widya-office              1.9.0           2.5.0           Major     
  widya-http                1.0.0           1.2.0           Minor     

  Jalankan 'widya wpm add <paket> --versi <terbaru>' untuk memperbarui.
================================================================================
```

---

## 7. Inspeksi Metadata Paket (`info` / `informasi`)

Untuk memeriksa metadata paket sebelum menginstalnya:
```bash
widya wpm info widya-office
```

Menampilkan:
- Nama paket, versi rilis mutakhir, pembuat, dan lisensi.
- Deskripsi dan tag klasifikasi.
- Total unduhan ekosistem.
- Riwayat seluruh versi yang tersedia beserta status tanda tangan Ed25519 dan hash SHA-256.

---

## 8. Penyediaan Paket Offline & Air-Gapped (`vendor` / `pasok`)

Untuk lingkungan pusat data terisolasi tanpa akses internet (*Air-Gapped / Military / Banking*), gunakan perintah vendor:
```bash
# Memasok dependensi ke direktori ./vendor
widya wpm vendor

# Atau dengan alias Nusantara
widya wpm pasok --output /opt/widya/vendor
```

Perintah ini akan:
1. Menyalin kode sumber seluruh dependensi ke sub-direktori lokal.
2. Membentuk berkas manifest `modules.json` berisikan ringkasan hash SHA-256 setiap paket.
3. Runtime Widya dapat langsung mengeksekusi proyek secara mandiri tanpa memerlukan koneksi jaringan internet.

---

## 9. Self-Hosted Private Registry Server & Web Hub Portal

Organisasi atau institusi pemerintah dapat meluncurkan server registry internal mandiri:
```bash
widya wpm serve-registry --port 8090
```

### Fitur Registry Mandiri:
- **Port Kompatibel**: Dapat diatur sesuai port organisasi (default: 8090).
- **Web Portal Hub (`/hub`)**: Antarmuka web modern dengan pencarian langsung (*live search*), filter tag, kartu metadata paket, tombol salin perintah instalasi, dan lencana validasi Ed25519.
- **Dukungan Bearer Token**: Mengamankan publikasi paket internal menggunakan token API.

---

## 10. Spesifikasi REST API Registry

Server Registry WPM menyediakan endpoint REST API standar:

| Metode | Endpoint | Deskripsi | Respons |
| :--- | :--- | :--- | :--- |
| `GET` | `/health` | Memeriksa kesehatan server registry | `{"status":"healthy","version":"1.0.0"}` |
| `GET` | `/hub` | Antarmuka web Package Hub Portal | HTML Web Interface |
| `GET` | `/api/v1/packages` | Mengambil seluruh katalog paket | JSON Array Paket |
| `GET` | `/api/v1/packages/:name` | Mengambil metadata paket spesifik | JSON Object Paket |
| `GET` | `/api/v1/search?q=:query` | Mencari paket berdasarkan kueri | JSON Array Hasil |
| `GET` | `/api/v1/security/advisories` | Mengambil daftar kerentanan WSA | JSON Array Advisories |
| `GET` | `/api/v1/security/advisories/:pkg` | Mengambil kerentanan paket spesifik | JSON Array Advisories |
| `POST` | `/api/v1/packages/publish` | Mempublikasikan paket baru (Bearer Token) | `201 Created` / `401 Unauthorized` |

---

## 11. Kriptografi Ed25519 & Standar Provenance SLSA Level 3

Setiap paket yang didistribusikan melalui WPM memenuhi standar **SLSA Level 3 (Supply-chain Levels for Software Artifacts)**:
- **Asal Usul Build Tervalidasi**: Metadata build mencakup commit hash Git resmi dan identitas pembuat.
- **Log Transparansi Merkle Tree**: Setiap rilis dicatat dalam log transparansi yang tidak dapat diubah (*append-only*).
- **Kunci Publik Kurva Edwards**: Menggunakan algoritma penandatanganan kurva Ed25519 berkecepatan tinggi yang aman terhadap serangan *side-channel*.

---

*Hak Cipta © 2026 Tim Pengembang Widya-Lang (widya-lang). Dilisensikan di bawah Lisensi MIT. Repositori Berdaulat Indonesia.*
