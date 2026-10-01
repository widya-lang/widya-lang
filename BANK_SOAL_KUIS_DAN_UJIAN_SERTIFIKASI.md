# BANK SOAL KUIS, MATERI MODUL, DAN UJIAN SERTIFIKASI RESMI WIDYA-LANG
**Dewan Standarisasi & Sertifikasi Widya-Lang (BOS-WL) | Tahun 2026**
*Dokumen Standar Bank Soal Resmi untuk Seluruh Jenjang Hierarki & Spesialisasi Industri*

---

## DAFTAR ISI BANK SOAL
1. [LEVEL 1: WCA - Widya Certified Associate](#1-level-1-wca---widya-certified-associate)
2. [LEVEL 2: WCP - Widya Certified Professional](#2-level-2-wcp---widya-certified-professional)
3. [LEVEL 3: WCE - Widya Certified Expert](#3-level-3-wce---widya-certified-expert)
4. [SPESIALISASI: WCWD - Widya Certified Web Developer](#4-spesialisasi-wcwd---widya-certified-web-developer)
5. [SPESIALISASI: WCMD - Widya Certified Mobile Developer](#5-spesialisasi-wcmd---widya-certified-mobile-developer)
6. [SPESIALISASI: WCDD - Widya Certified Desktop Developer](#6-spesialisasi-wcdd---widya-certified-desktop-developer)
7. [SPESIALISASI: WCSS - Widya Certified Security Specialist](#7-spesialisasi-wcss---widya-certified-security-specialist)
8. [SPESIALISASI: WCDA - Widya Certified Data & AI Engineer](#8-spesialisasi-wcda---widya-certified-data--ai-engineer)
9. [SPESIALISASI: WCCD - Widya Certified Cloud & DevOps Architect](#9-spesialisasi-wccd---widya-certified-cloud--devops-architect)
10. [SPESIALISASI: WCEI - Widya Certified Enterprise & Industrial Architect](#10-spesialisasi-wcei---widya-certified-enterprise--industrial-architect)

---

## 1. LEVEL 1: WCA - WIDYA CERTIFIED ASSOCIATE
*Standar Kompetensi: Pemahaman sintaks natural Indonesia, deklarasi variabel, kontrol logika, perulangan, fungsi dasar, dan manipulasi data dasar.*
*Target Capaian: Mampu membangun aplikasi CLI interaktif seperti Kalkulator Pajak PPh 21, Sistem Kasir Warung Pintar, dan Konversi Suhu Otomatis.*

### A. Bank Soal Pilihan Ganda (Theory & Mechanics)

#### Soal 1 (ID: 1)
**Pertanyaan**: Keyword apa saja di Widya-Lang yang sah dan valid untuk deklarasi variabel baru yang dapat diubah (*mutable*)?
- A. `var` dan `const`
- B. `misal`, `biar`, `diketahui`, `diket`
- C. `let` dan `mut`
- D. `dim` dan `static`
- **Kunci Jawaban**: **B**
- **Pembahasan Mendalam**: Widya-Lang mendukung kosakata natural bahasa Indonesia yang kaya. Deklarasi variabel mutable dapat menggunakan kata kunci `misal`, `biar`, `diketahui`, maupun singkatannya `diket`. Variabel konstanta imutable dideklarasikan menggunakan `tetap`.

#### Soal 2 (ID: 2)
**Pertanyaan**: Bagaimana cara mencetak teks atau nilai variabel ke konsol output di Widya-Lang?
- A. `print("halo")` atau `echo "halo"`
- B. `tulis("halo")` atau `cetak("halo")`
- C. `console.log("halo")`
- D. `System.out.println("halo")`
- **Kunci Jawaban**: **B**
- **Pembahasan Mendalam**: Fungsi bawaan resmi untuk output standar (stdout) pada Widya-Lang adalah `tulis(...)` atau `cetak(...)`. Keduanya menerima berbagai tipe data (Angka, Teks, Boolean, Larik, Kamus) dan mencetaknya secara instan ke terminal.

#### Soal 3 (ID: 3)
**Pertanyaan**: Keyword percabangan kondisi logika apa saja yang valid di Widya-Lang?
- A. `if - elif - else`
- B. `jika`, `kalau`, `jika_tidak`, `selain_itu`
- C. `when - otherwise`
- D. `check - match - fall`
- **Kunci Jawaban**: **B**
- **Pembahasan Mendalam**: Widya-Lang menyediakan fleksibilitas idiomatis: klausa utama dapat memakai `jika` atau `kalau`, dan klausa alternatif dapat memakai `selain_itu` atau `jika_tidak`.

#### Soal 4 (ID: 4)
**Pertanyaan**: Konstruksi perulangan apa yang digunakan untuk mengiterasi seluruh elemen di dalam sebuah larik (array)?
- A. `untuk_setiap item dalam larik { ... }`
- B. `foreach(larik as item) { ... }`
- C. `loop.each { ... }`
- D. `iterate item of larik`
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Perulangan koleksi di Widya-Lang menggunakan sintaks deklaratif `untuk_setiap <elemen> dalam <koleksi> { ... }`. Untuk perulangan berbasis kondisi nilai logika, digunakan `selama (kondisi) { ... }` atau `ulangi_selama (kondisi) { ... }`.

#### Soal 5 (ID: 5)
**Pertanyaan**: Bagaimana cara mendefinisikan fungsi yang mengembalikan nilai di Widya-Lang?
- A. `fungsi hitung(a, b) { kembalikan a + b }`
- B. `def hitung(a, b) => a + b`
- C. `fn hitung(a, b): return a + b`
- D. `sub hitung(a, b) { result = a + b }`
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Deklarasi fungsi di Widya-Lang diawali dengan kata kunci `fungsi`, diikuti parameter di dalam tanda kurung `(a, b)`, blok kode `{ ... }`, dan kata kunci `kembalikan` untuk mengembalikan nilai ke pemanggil.

#### Soal 6 (ID: 6)
**Pertanyaan**: Keyword apa yang digunakan untuk mendeklarasikan nilai konstan yang tidak dapat diubah (imutable)?
- A. `tetap`
- B. `final`
- C. `freeze`
- D. `lock`
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Keyword `tetap` digunakan untuk mendefinisikan konstanta (misal: `tetap TARIF_PPN = 0.11;`). Interpreter dan compiler Widya-Lang akan memvalidasi agar nilai ini tidak di-assign ulang.

#### Soal 7 (ID: 7)
**Pertanyaan**: Manakah sintaks inisialisasi larik (list/array) dan kamus (map/dictionary) yang benar di Widya-Lang?
- A. Larik: `[1, 2, 3]`, Kamus: `{"kunci": "nilai"}`
- B. Larik: `(1, 2, 3)`, Kamus: `<kunci: nilai>`
- C. Larik: `Array(1, 2, 3)`, Kamus: `Map("kunci" -> "nilai")`
- D. Larik: `[| 1; 2; 3 |]`, Kamus: `table(kunci = nilai)`
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Widya-Lang menggunakan tanda kurung siku `[...]` untuk struktur data Larik dan tanda kurung kurawal `{...}` dengan pasangan `kunci: nilai` untuk struktur Kamus.

#### Soal 8 (ID: 8)
**Pertanyaan**: Operator perbandingan apa yang digunakan untuk menguji kesetaraan nilai di Widya-Lang?
- A. `==` atau `adalah`
- B. `===` saja
- C. `equals()`
- D. `:=`
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Widya-Lang mendukung simbol baku `==` serta kata kunci natural `adalah` untuk memeriksa kesamaan nilai dua operan.

---

### B. Bank Soal Tantangan Kode Praktik (Hands-on Coding Challenges)

#### Tantangan 1 (ID: 101): Penjumlahan Dua Bilangan
- **Tantangan**: Buat fungsi `tambah(a, b)` yang menerima dua bilangan dan mengembalikan hasil penjumlahannya.
- **Starter Code**:
  ```widya
  fungsi tambah(a, b) {
      // Tulis kode Anda di sini
  }
  ```
- **Test Cases**:
  - Test 1: `tambah(5, 7)` -> Output yang diharapkan: `12`
  - Test 2: `tambah(-3, 10)` -> Output yang diharapkan: `7`
- **Kunci Solusi Resmi**:
  ```widya
  fungsi tambah(a, b) {
      diketahui hasil = a + b
      kembalikan hasil
  }
  ```

#### Tantangan 2 (ID: 102): Klasifikasi Genap atau Ganjil
- **Tantangan**: Buat fungsi `apakah_genap(n)` yang mengembalikan `"GENAP"` jika angka genap, dan `"GANJIL"` jika ganjil.
- **Starter Code**:
  ```widya
  fungsi apakah_genap(n) {
      // Tulis kode Anda di sini
  }
  ```
- **Test Cases**:
  - Test 1: `apakah_genap(8)` -> Output yang diharapkan: `"GENAP"`
  - Test 2: `apakah_genap(15)` -> Output yang diharapkan: `"GANJIL"`
- **Kunci Solusi Resmi**:
  ```widya
  fungsi apakah_genap(n) {
      jika (n % 2 == 0) {
          kembalikan "GENAP"
      } selain_itu {
          kembalikan "GANJIL"
      }
  }
  ```

#### Tantangan 3 (ID: 103): Mencari Nilai Maksimum
- **Tantangan**: Buat fungsi `maksimum(a, b)` yang mengembalikan nilai tertinggi di antara dua parameter bilangan.
- **Starter Code**:
  ```widya
  fungsi maksimum(a, b) {
      // Tulis kode Anda di sini
  }
  ```
- **Test Cases**:
  - Test 1: `maksimum(25, 40)` -> Output yang diharapkan: `40`
  - Test 2: `maksimum(100, 50)` -> Output yang diharapkan: `100`
- **Kunci Solusi Resmi**:
  ```widya
  fungsi maksimum(a, b) {
      jika (a > b) {
          kembalikan a
      } selain_itu {
          kembalikan b
      }
  }
  ```

---

## 2. LEVEL 2: WCP - WIDYA CERTIFIED PROFESSIONAL
*Standar Kompetensi: Pemrograman Berorientasi Objek, Actor Model Concurrency, Error Handling, Engine Basis Data WidyaBase SQL Dwibahasa, Office Automation Suite, Sinkronisasi CRDT.*
*Target Capaian: Mampu membangun Sistem Manajemen Inventaris Multi-Cabang & Payroll Generator PDF/Excel lengkap dengan proteksi konkurensi.*

### A. Bank Soal Pilihan Ganda (Theory & Mechanics)

#### Soal 201 (ID: 201)
**Pertanyaan**: Fitur konkurensi performa tinggi di Widya-Lang dibangun di atas konsep dan arsitektur apa?
- A. Single-thread event callback hell
- B. Actor Model, Channel Message Passing, dan Task Concurrency
- C. Manual OS thread locking tanpa garansi memori
- D. Generator coroutine Python 2 style
- **Kunci Jawaban**: **B**
- **Pembahasan Mendalam**: Widya-Lang mengadopsi Actor Model di mana setiap aktor memiliki antrian pesan terisolasi (mailbox) dan berkomunikasi secara asinkron tanpa membagi memori mentah (*share-nothing architecture*), menghilangkan resiko *race condition* dan *deadlock*.

#### Soal 202 (ID: 202)
**Pertanyaan**: Bagaimana mekanisme penanganan galat (*error handling*) yang aman dan elegan di Widya-Lang?
- A. Menggunakan blok `coba { ... } tangkap (e) { ... }` atau tipe Result/Opsi
- B. Menggunakan `goto error_handler`
- C. Proses langsung crash (segfault)
- D. Mengandalkan return integer error code C-style
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Widya-Lang menyediakan blok terstruktur `coba - tangkap` untuk eksepsi runtime, serta mendukung *pattern matching* pada tipe pengembalian varian `Hasil` (`Sukses` / `Galat`).

#### Soal 203 (ID: 203)
**Pertanyaan**: Perintah SQL dwibahasa apa di WidyaBase yang setara dengan `SELECT * FROM tabel`?
- A. `PILIH * DARI tabel`
- B. `AMBIL * TABEL tabel`
- C. `CARI_SEMUA tabel`
- D. `GET ALL tabel`
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Engine WidyaBase mendukung query dwibahasa penuh: `PILIH` (SELECT), `SISIP_KE` (INSERT INTO), `PERBARUI` (UPDATE), `HAPUS_DARI` (DELETE FROM), dan `BUAT_TABEL` (CREATE TABLE).

#### Soal 204 (ID: 204)
**Pertanyaan**: Fitur apa yang digunakan dalam modul Widya Workspace untuk kolaborasi dokumen real-time tanpa konflik data?
- A. Conflict-free Replicated Data Types (CRDT)
- B. Central lock single-user
- C. Manual file overwrite
- D. Git merge manual conflict
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Widya Workspace mengintegrasikan struktur data CRDT (Conflict-free Replicated Data Types) dengan algoritma YATA/LWW-Set yang memungkinkan multi-user mengedit teks bersamaan secara P2P/Serverless dengan konvergensi otomatis 100%.

#### Soal 205 (ID: 205)
**Pertanyaan**: Bagaimana cara mendefinisikan struktur objek dan metode di Widya-Lang?
- A. `struktur Nama { ... }` dan `implementasi Nama { ... }`
- B. `class Nama { ... }`
- C. `typedef struct { ... }`
- D. `object Nama { ... }`
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Widya-Lang memisahkan definisi struktur data dengan `struktur` dan implementasi logika metode dengan blok `implementasi`.

#### Soal 206 (ID: 206)
**Pertanyaan**: Format dokumen bisnis resmi apa saja yang dapat dihasilkan secara native oleh Widya Office Suite?
- A. PDF (ISO 32000-1), XLSX Spreadsheet, DOCX Word, PPTX Presentation, dan LaTeX Math
- B. Hanya berkas TXT
- C. Hanya berkas RTF
- D. Hanya gambar bitmap BMP
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Widya Office Suite memproduksi berkas biner standar ISO PDF, XLSX, DOCX, PPTX, dan LaTeX tanpa dependensi runtime pihak ketiga (Node.js/Python/Java).

---

### B. Bank Soal Tantangan Kode Praktik (Hands-on Coding Challenges)

#### Tantangan 1 (ID: 201): Rekursi Perhitungan Faktorial
- **Tantangan**: Buat fungsi rekursif `faktorial(n)` untuk menghitung nilai faktorial $n!$.
- **Starter Code**:
  ```widya
  fungsi faktorial(n) {
      // Tulis kode rekursif Anda di sini
  }
  ```
- **Test Cases**:
  - Test 1: `faktorial(5)` -> Output yang diharapkan: `120`
  - Test 2: `faktorial(0)` -> Output yang diharapkan: `1`
- **Kunci Solusi Resmi**:
  ```widya
  fungsi faktorial(n) {
      jika (n <= 1) {
          kembalikan 1
      }
      kembalikan n * faktorial(n - 1)
  }
  ```

#### Tantangan 2 (ID: 202): Deret Bilangan Fibonacci
- **Tantangan**: Buat fungsi `fibonacci(n)` yang mengembalikan nilai suku ke-$n$ dari deret Fibonacci ($0, 1, 1, 2, 3, 5, 8, \dots$).
- **Starter Code**:
  ```widya
  fungsi fibonacci(n) {
      // Tulis kode Anda di sini
  }
  ```
- **Test Cases**:
  - Test 1: `fibonacci(6)` -> Output yang diharapkan: `8`
  - Test 2: `fibonacci(0)` -> Output yang diharapkan: `0`
- **Kunci Solusi Resmi**:
  ```widya
  fungsi fibonacci(n) {
      jika (n <= 0) {
          kembalikan 0
      }
      jika (n == 1) {
          kembalikan 1
      }
      kembalikan fibonacci(n - 1) + fibonacci(n - 2)
  }
  ```

---

## 3. LEVEL 3: WCE - WIDYA CERTIFIED EXPERT
*Standar Kompetensi: Multi-Agent Systems, High-Performance FFI, Full Self-Hosting Compiler Stage 0-2, LLVM IR, WebAssembly, WGSL GPU Compute, eBPF Kernel Probes.*
*Target Capaian: Mampu membangun Autonomous AI-Agentic Cloud Orchestrator, Transpiler Bahasa Khusus, & Distributed Event Streaming Engine berlatensi mikrodetik.*

### A. Bank Soal Pilihan Ganda (Theory & Mechanics)

#### Soal 301 (ID: 301)
**Pertanyaan**: Dalam arsitektur Multi-Agent Widya-Lang, apa fungsi dan peranan dari Blackboard State Bus?
- A. Sebagai shared in-memory blackboard untuk sinkronisasi state antar-agen secara reaktif
- B. Sebagai rendering pipeline CSS
- C. Sebagai parser file JSON saja
- D. Sebagai pengganti OS kernel
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Blackboard berfungsi sebagai media koordinasi bersama terpusat di mana setiap agen independen dapat mempublikasikan hipotesis, membaca status global, dan mencapai konsensus orkestrasi tugas kompleks tanpa coupling ketat.

#### Soal 302 (ID: 302)
**Pertanyaan**: Bagaimana siklus verifikasi kompilasi mandiri (*Full Self-Hosting Bootstrap Compiler*) di Widya-Lang?
- A. Stage-0 (Rust Bootstrap) -> Stage-1 (Widya Compiler) -> Stage-2 (Deterministic Hash Match)
- B. Transpilasi sementara ke JavaScript
- C. Menjalankan skrip bash tanpa binary compiler
- D. Menggunakan interpreter Java runtime
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Kemandirian kompilasi Widya-Lang diverifikasi melalui 3 tahap: Stage-0 (kompiler awal via Rust), menghasilkan Stage-1 (kompiler yang ditulis dalam Widya-Lang murni), yang kemudian mengompilasi dirinya sendiri menjadi Stage-2. Hasil binary hash Stage-1 dan Stage-2 harus 100% identik (*reproducible builds*).

#### Soal 303 (ID: 303)
**Pertanyaan**: Target biner apa saja yang didukung secara native oleh backend compiler Widya-Lang?
- A. Native Executable (LLVM IR / C), WebAssembly (WASM), WGSL Compute Shaders (GPU), dan eBPF Kernel Probes
- B. Hanya Flash SWF
- C. Hanya Java Bytecode 1.1
- D. Hanya VBScript Windows 98
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Arsitektur modular Widya Backend memancarkan bytecode/IR langsung ke target arsitektur modern: LLVM IR untuk biner mesin ultra-cepat, WASM untuk web, WGSL untuk GPGPU tensor, dan eBPF untuk observabilitas tingkat kernel Linux.

#### Soal 304 (ID: 304)
**Pertanyaan**: Bagaimana mekanisme Foreign Function Interface (FFI) di Widya-Lang meminimalisir overhead pemanggilan fungsi bahasa C/Rust?
- A. Melalui deklarasi C-ABI zero-copy dengan pemetaan pointer langsung tanpa marshalling memory serialization
- B. Melalui HTTP REST API lokal
- C. Menulis ke file temp di harddisk
- D. Menggunakan emulasi QEMU
- **Kunci Jawaban**: **A**
- **Pembahasan Mendalam**: Modul `widya::ffi` menghubungkan simbol pustaka eksternal (.so, .dll, .dylib) melalui ABI C baku dengan *zero-copy direct pointer binding*, menghasilkan latensi panggilan fungsi setara C native (sub-nanodetik).

---

### B. Bank Soal Tantangan Kode Praktik (Hands-on Coding Challenges)

#### Tantangan 1 (ID: 301): Pipeline Filter & Transformasi Data
- **Tantangan**: Buat fungsi `proses_data(arr)` yang memfilter hanya angka positif (> 0), mengalikan tiap angka dengan 2, lalu mengembalikan jumlah total seluruh hasil transformasi tersebut.
- **Starter Code**:
  ```widya
  fungsi proses_data(arr) {
      // Tulis logika pemrosesan data di sini
  }
  ```
- **Test Cases**:
  - Test 1: `proses_data([1, -2, 3, 4, -5])` -> Output yang diharapkan: `16` (karena (1*2) + (3*2) + (4*2) = 2 + 6 + 8 = 16)
  - Test 2: `proses_data([-10, -20, 5])` -> Output yang diharapkan: `10`
- **Kunci Solusi Resmi**:
  ```widya
  fungsi proses_data(arr) {
      diketahui total = 0
      untuk_setiap x dalam arr {
          jika (x > 0) {
              total = total + (x * 2)
          }
      }
      kembalikan total
  }
  ```

---

## 4. SPESIALISASI: WCWD - WIDYA CERTIFIED WEB DEVELOPER
*Standar Kompetensi: WidyaWeb Framework, Virtual DOM, Native SSR Engine, WebSocket Channels, Serverless Edge Runtime, Atomic Preview Deployments.*
*Target Capaian: Portal E-Commerce Nasional dengan SSR Berlatensi Sub-10ms & Real-time Live Auction.*

### A. Bank Soal Pilihan Ganda
- **Soal 401**: Komponen arsitektur apa di WidyaWeb yang menangani perenderan antarmuka web modern dengan reaktivitas tinggi?
  - *Kunci*: **A** (Virtual DOM & SSR Engine native berkecepatan tinggi).
  - *Pembahasan*: WidyaWeb menggabungkan Server-Side Rendering (SSR) untuk SEO instan dengan Virtual DOM client-side untuk interaksi dinamis.
- **Soal 402**: Fitur apa saja pada edge serverless WidyaWeb Cloud?
  - *Kunci*: **A** (Atomic Preview Deployments, Sub-Millisecond Cold Starts, Zero-Downtime Instant Rollback).
  - *Pembahasan*: Edge Runtime WidyaWeb memanfaatkan isolasi V8/Wasm micro-isolate untuk cold start < 1ms.

### B. Bank Soal Tantangan Kode Praktik
- **Tantangan 401 (Pembuat Tag HTML)**: Buat fungsi `buat_tag(tag, konten)` yang mengembalikan string elemen HTML: `<tag>konten</tag>`.
  - *Kunci*:
    ```widya
    fungsi buat_tag(tag, konten) {
        kembalikan "<" + tag + ">" + konten + "</" + tag + ">"
    }
    ```

---

## 5. SPESIALISASI: WCMD - WIDYA CERTIFIED MOBILE DEVELOPER
*Standar Kompetensi: Kompilasi Native Mobile (Android APK/AAB & iOS IPA), Gesture Handler, Bluetooth Low Energy (BLE), SQLite Mobile Sync, Push Notification.*
*Target Capaian: Aplikasi Dompet Digital FinTech Multi-Platform dengan Autentikasi Biometrik & Sinkronisasi Offline-First.*

### A. Bank Soal Pilihan Ganda
- **Soal 501**: Platform target native apa yang didukung oleh modul kompilasi Widya Mobile?
  - *Kunci*: **A** (Android APK/AAB via NDK dan iOS IPA via LLVM Clang target).
  - *Pembahasan*: Widya Mobile mengompilasi kode sumber langsung menjadi binary ARM64 native tanpa bridge JavaScript yang lambat.
- **Soal 502**: Bagaimana cara menangani sinkronisasi data offline-first pada aplikasi mobile Widya?
  - *Kunci*: **A** (Menggunakan engine SQLite terenkripsi lokal dan sync queue berbasis delta CRDT).

### B. Bank Soal Tantangan Kode Praktik
- **Tantangan 501 (Format Nomor Telepon Indonesia)**: Buat fungsi `format_nomor(no)` yang mengubah awalan `08` menjadi `+628`.
  - *Kunci Solusi*:
    ```widya
    fungsi format_nomor(no) {
        kembalikan "+62" + potong_teks(no, 1, panjang(no))
    }
    ```

---

## 6. SPESIALISASI: WCDD - WIDYA CERTIFIED DESKTOP DEVELOPER
*Standar Kompetensi: GUI Canvas Native Berakselerasi GPU (<15MB RAM), Multi-Window Management, Tray System, OS Keystroke Interceptor, File Association.*
*Target Capaian: Aplikasi Workstation Editor Video/Audio Multitrack Ringan & POS Terminal Tanpa Ketergantungan Chromium.*

### A. Bank Soal Pilihan Ganda
- **Soal 601**: Apa keunggulan mendasar arsitektur GUI Desktop Widya dibandingkan framework berbasis Electron?
  - *Kunci*: **A** (GPU-accelerated canvas native dengan konsumsi memori <15MB RAM tanpa runtime Chromium/Node.js).
- **Soal 602**: Bagaimana Widya Desktop menangani skalabilitas tampilan pada layar beresolusi 4K/HiDPI?
  - *Kunci*: **A** (Melalui auto-scaling vector layout engine terintegrasi hardware DPI scaling).

### B. Bank Soal Tantangan Kode Praktik
- **Tantangan 601 (Kalkulasi Dimensi Jendela)**: Buat fungsi `hitung_luas_jendela(w, h)` yang mengembalikan nilai $w \times h$.
  - *Kunci Solusi*:
    ```widya
    fungsi hitung_luas_jendela(w, h) {
        kembalikan w * h
    }
    ```

---

## 7. SPESIALISASI: WCSS - WIDYA CERTIFIED SECURITY SPECIALIST
*Standar Kompetensi: WidyaShield WAF, Mitigasi Serangan DDoS L7, Zero-Trust Architecture, SPIFFE/SPIRE mTLS, Kriptografi Pasca-Kuantum (Kyber & Dilithium).*
*Target Capaian: Gateway Keamanan Perbankan Standar PCI-DSS v4.0 dengan Perlindungan Quantum-Resistant.*

### A. Bank Soal Pilihan Ganda
- **Soal 701**: Algoritma enkripsi pasca-kuantum apa yang diimplementasikan secara native pada modul `widya::crypto`?
  - *Kunci*: **A** (CRYSTALS-Kyber untuk Key Encapsulation dan CRYSTALS-Dilithium untuk Digital Signatures).
- **Soal 702**: Apa fungsi dari identitas SPIFFE ID dalam modul WidyaShield?
  - *Kunci*: **A** (Menyediakan identitas kriptografis terotentikasi otomatis antar microservice dalam jaringan Zero-Trust).

### B. Bank Soal Tantangan Kode Praktik
- **Tantangan 701 (Sanitasi Input XSS)**: Buat fungsi `sanitasi(teks)` yang mengamankan karakter berbahaya `<` dan `>`.
  - *Kunci Solusi*:
    ```widya
    fungsi sanitasi(teks) {
        diketahui t1 = ganti_teks(teks, "<", "&lt;")
        diketahui t2 = ganti_teks(t1, ">", "&gt;")
        kembalikan t2
    }
    ```

---

## 8. SPESIALISASI: WCDA - WIDYA CERTIFIED DATA & AI ENGINEER
*Standar Kompetensi: Native Tensor Math, SIMD GPU Compute (WGSL), Pipeline ETL Streaming, Standar ISPU Smart City PM2.5 (Permen LHK), Model Intelejen Buatan On-Device.*
*Target Capaian: Platform Prediksi Kualitas Udara Smart City Real-time dengan Inferensi AI On-Edge.*

### A. Bank Soal Pilihan Ganda
- **Soal 801**: Bagaimana modul `widya::gpu` mengeksekusi komputasi perkalian matriks tensor skala besar?
  - *Kunci*: **A** (Melalui WGSL Compute Shaders langsung pada chip GPU dengan akselerasi CPU SIMD AVX-512/NEON).
- **Soal 802**: Regulasi resmi Republik Indonesia apa yang menjadi acuan formula baku perhitungan ISPU PM2.5 di modul Smart City Widya?
  - *Kunci*: **A** (Permen LHK No. P.14/MENLHK/SETJEN/KUM.1/7/2020).

### B. Bank Soal Tantangan Kode Praktik
- **Tantangan 801 (Rata-Rata Sensor)**: Buat fungsi `rata_rata(a, b, c)` yang menghitung rata-rata aritmetika 3 nilai sensor.
  - *Kunci Solusi*:
    ```widya
    fungsi rata_rata(a, b, c) {
        kembalikan (a + b + c) / 3
    }
    ```

---

## 9. SPESIALISASI: WCCD - WIDYA CERTIFIED CLOUD & DEVOPS ARCHITECT
*Standar Kompetensi: Isolasi MicroVM Firecracker (WidyaBox), Kubernetes Pod Controller (WidyaKube), Infrastructure as Code (IaC), GitOps Pipeline, Observabilitas eBPF.*
*Target Capaian: Cluster Orkestrasi Serverless Multi-Region Berkemampuan Auto-Healing dalam 10 Milidetik.*

### A. Bank Soal Pilihan Ganda
- **Soal 901**: Apa perbedaan fundamental antara isolasi container biasa dengan MicroVM WidyaBox?
  - *Kunci*: **A** (WidyaBox menggunakan kernel Linux independen minimalis per-tenant dengan isolasi hardware VT-x/AMD-V dan waktu boot <5ms).
- **Soal 902**: Bagaimana eBPF digunakan dalam modul observabilitas Widya Cloud?
  - *Kunci*: **A** (Menyadap probe kernel Linux untuk tracing paket jaringan dan syscall I/O tanpa modifikasi biner aplikasi).

### B. Bank Soal Tantangan Kode Praktik
- **Tantangan 901 (Penyusun URI Endpoint Layanan)**: Buat fungsi `buat_uri_layanan(host, port)` yang menghasilkan format `"http://host:port"`.
  - *Kunci Solusi*:
    ```widya
    fungsi buat_uri_layanan(host, port) {
        kembalikan "http://" + host + ":" + port
    }
    ```

---

## 10. SPESIALISASI: WCEI - WIDYA CERTIFIED ENTERPRISE & INDUSTRIAL ARCHITECT
*Standar Kompetensi: Protokol Industri SCADA/PLC (OPC UA, Modbus TCP/RTU), Standar Perbankan Nasional (QRIS MPM/CPM, BI-FAST, SNAP/OpenAPI, ISO 8583), Interoperabilitas Medis HL7/FHIR.*
*Target Capaian: Core Banking Transaction Switcher Berlatensi Sub-Milidetik & Sistem Otomasi Pabrik Industri 4.0.*

### A. Bank Soal Pilihan Ganda
- **Soal 1001**: Standar protokol sistem pembayaran dan industri apa saja yang didukung native tanpa pustaka eksternal oleh Widya Enterprise?
  - *Kunci*: **A** (OPC UA, Modbus TCP/RTU, QRIS MPM/CPM ASPI, BI-FAST ISO 20022, SNAP OpenAPI BI, ISO 8583 Perbankan, dan FHIR Healthtech).
- **Soal 1002**: Bagaimana cara memastikan kepatuhan checksum EMVCo pada pembuatan QRIS Dinamis?
  - *Kunci*: **A** (Menghitung CRC-16/CCITT-FALSE pada seluruh string payload tag 00 hingga 63).

### B. Bank Soal Tantangan Kode Praktik
- **Tantangan 1001 (Validasi Bit Status Alarm PLC)**: Buat fungsi `cek_alarm_plc(status_word, bit_index)` yang mengembalikan `"ALARM"` jika bit tertentu bernilai 1, dan `"NORMAL"` jika bernilai 0.
  - *Kunci Solusi*:
    ```widya
    fungsi cek_alarm_plc(status_word, bit_index) {
        jika ((status_word & (1 << bit_index)) != 0) {
            kembalikan "ALARM"
        } selain_itu {
            kembalikan "NORMAL"
        }
    }
    ```

---

## PANDUAN PELAKSANAAN UJIAN & STANDAR KELULUSAN
1. **Durasi Ujian**:
   - Tingkat WCA: 60 Menit
   - Tingkat WCP & Spesialisasi (WCWD - WCEI): 90 Menit
   - Tingkat WCE: 120 Menit
2. **Komposisi Bobot Penilaian**:
   - Pilihan Ganda (Teori Arsitektur & Sintaks): **40%**
   - Tantangan Kode Praktik (Hands-on Coding & Test Suite): **60%**
3. **Standar Skor Kelulusan (*Passing Grade*)**:
   - **WCA**: Minimal **70.0%**
   - **WCP & Spesialisasi**: Minimal **75.0% - 80.0%**
   - **WCE**: Minimal **85.0%**
4. **Penerbitan Sertifikat**:
   - Peserta yang lulus akan langsung memperoleh Sertifikat Digital berformat ISO 32000-1 dengan tanda tangan kriptografis SHA-256 dan tautan verifikasi publik `https://verify.widya.dev/cert/WIDYA-<LEVEL>-<UUID>`.
