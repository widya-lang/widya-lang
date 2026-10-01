# 📚 Buku Panduan Komprehensif Bahasa Pemrograman Widya-Lang
**Versi: 1.0.0-beta.1 (Public Beta Preview & Full Self-Hosting)**  
*Bahasa Pemrograman Modern Berbasis Bahasa Indonesia untuk Sistem, Web, Mobile, AI, dan Skala Enterprise.*

---

## 📑 Daftar Isi
1. [Pengantar & Keunggulan Widya-Lang](#1-pengantar--keunggulan-widya-lang)
2. [Instalasi & Antarmuka CLI](#2-instalasi--antarmuka-cli)
3. [Sintaks Dasar & Tipe Data](#3-sintaks-dasar--tipe-data)
4. [Sistem Tipe Statis & Generics](#4-sistem-tipe-statis--generics)
5. [Struktur Data, Sifat (Traits), & Pemrograman Berorientasi Objek](#5-struktur-data-sifat-traits--pemrograman-berorientasi-objek)
6. [Pencocokan Pola (Pattern Matching)](#6-pencocokan-pola-pattern-matching)
7. [Pengembangan Web & REST API](#7-pengembangan-web--rest-api)
8. [Pengembangan Aplikasi Mobile (Android & iOS)](#8-pengembangan-aplikasi-mobile-android--ios)
9. [C/Native Foreign Function Interface (FFI)](#9-cnative-foreign-function-interface-ffi)
10. [Pustaka Standar C FFI (SQLite3 & Kriptografi)](#10-pustaka-standar-c-ffi-sqlite3--kriptografi)
11. [Audit Keamanan & Pencegahan Kerentanan](#11-audit-keamanan--pencegahan-kerentanan)
12. [Manajemen Memori & Profiler](#12-manajemen-memori--profiler)
13. [Widya Package Manager (WPM) & Lockfile](#13-widya-package-manager-wpm--lockfile)
14. [Perkakas Developer (VS Code, LSP, DAP, REPL, Studio IDE, Web Playground)](#14-perkakas-developer-vs-code-lsp-dap-repl-studio-ide-web-playground)
15. [Multi-Target Compilation (Native, WASM, GPU, eBPF)](#15-multi-target-compilation-native-wasm-gpu-ebpf)
16. [Full Self-Hosting Compiler Pipeline](#16-full-self-hosting-compiler-pipeline)
17. [Sub-Sistem Operasi & Kernel Primitives](#17-sub-sistem-operasi--kernel-primitives-srcos)
18. [Distributed Database Engine](#18-distributed-database-engine-srcdb)

---

## 1. Pengantar & Keunggulan Widya-Lang

Widya-Lang dirancang dengan kata kunci ekspresif berbahasa Indonesia dengan performa tinggi berstandar industri:
- 🚀 **Performa Dekat Mesin**: Menggunakan backend LLVM, C native emitter, dan WebAssembly.
- 🛡️ **Sistem Tipe Kuat**: Hindley-Milner Type Inference dengan dukungan Generic & Type-classes.
- 📱 **Cross-Platform**: Satu basis kode untuk Server, Desktop, Web (WASM/HTML5), dan Mobile (Android & iOS).
- 🔒 **Keamanan Terpadu**: Detektor rahasia, audit SQLi, dan proteksi XSS bawaan.

---

## 2. Instalasi & Antarmuka CLI

### Langkah Instalasi Berdasarkan Sistem Operasi

#### 🐧 Di Linux (Ubuntu, Debian, Fedora, Arch, CentOS, dll.)
```bash
# 1. Kloning repositori
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang

# 2. Berikan izin eksekusi dan jalankan instalasi
chmod +x install.sh
./install.sh

# 3. Verifikasi ketersediaan biner di terminal
widya --version
```

#### 🍎 Di macOS (Intel & Apple Silicon M-Series)
```bash
# 1. Kloning repositori
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang

# 2. Jalankan instalasi otomatis
chmod +x install.sh
./install.sh

# 3. Verifikasi instalasi
widya --version
```

#### 🪟 Di Windows (PowerShell)
```powershell
# 1. Kloning repositori
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang

# 2. Jalankan skrip PowerShell
.\install.ps1

# 3. Buka jendela PowerShell baru dan periksa versi
widya --version
```

---

### Perintah Utama Antarmuka CLI (`widya`)

```bash
# Eksekusi berkas
widya jalankan skrip.wya
widya skrip.wya

# Buka sesi interaktif REPL
widya repl

# Kompilasi native executable (.exe / .out)
widya kompilasi program.wya --output app.exe

# Audit keamanan kode
widya security scan .
widya security secrets .

# Profiling memori
widya profile program.wya

# Membuka IDE Mandiri Widya Studio
widya studio --port 8080

# Menjalankan server LSP & DAP
widya lsp
widya dap
```

---

## 3. Sintaks Dasar & Tipe Data

### Variabel & Konstanta
```widya
// Variabel yang dapat diubah (mutable)
misal nama = "Widya";
misal umur = 25;
nama = "Widya-Lang";

// Konstanta yang tidak dapat diubah (immutable)
tetap PI = 3.14159;
tetap MAKSIMAL_KONEKSI = 1000;
```

### Struktur Kontrol & Perulangan
```widya
// Percabangan Jika-Kalau-Lainnya
jika umur >= 18 {
    cetak("Dewasa");
} kalau umur >= 13 {
    cetak("Remaja");
} lainnya {
    cetak("Anak-anak");
}

// Perulangan untuk (for in)
misal angka = [1, 2, 3, 4, 5];
untuk n dalam angka {
    cetak("Nilai: " + ke_teks(n));
}

// Perulangan selama (while)
misal hitung = 3;
selama hitung > 0 {
    cetak(hitung);
    hitung = hitung - 1;
}
```

---

## 4. Sistem Tipe Statis & Generics

Widya mendukung inferensi tipe otomatis maupun anotasi eksplisit serta generics dengan type constraints:

```widya
// Fungsi Generik
fungsi tukar<T>(a: T, b: T) -> [T] {
    kembalikan [b, a];
}

// Generik dengan Batasan Sifat (Trait Bounds)
fungsi cetak_identitas<T: Tampil>(item: T) {
    cetak(item.ke_teks());
}
```

---

## 5. Struktur Data, Sifat (Traits), & Pemrograman Berorientasi Objek

```widya
// Definisi Sifat (Interface)
sifat DapatDihitung {
    fungsi hitung_luas() -> Angka;
}

// Definisi Struktur
struktur Persegi {
    panjang: Angka,
    lebar: Angka
}

// Implementasi Sifat untuk Struktur
terapkan DapatDihitung untuk Persegi {
    fungsi hitung_luas() -> Angka {
        kembalikan diri.panjang * diri.lebar;
    }
}

// Instansiasi
misal p = Persegi { panjang: 10, lebar: 5 };
cetak("Luas: " + ke_teks(p.hitung_luas()));
```

---

## 6. Pencocokan Pola (Pattern Matching)

Pencocokan pola yang kuat dan aman:

```widya
misal status_http = 200;

misal pesan = cocokkan status_http {
    200 => "Berhasil OK",
    404 => "Halaman Tidak Ditemukan",
    500 => "Kesalahan Server Internal",
    _ => "Kode Status Lain"
};

cetak(pesan);
```

---

## 7. Pengembangan Web & REST API

Widya memiliki server web HTTP/1.1, HTTP/2, dan WebSocket terintegrasi:

```widya
// Layanan Microservice REST API
misal server = ServerHttp::buat(8080);

server.tangani_get("/api/pengguna", fungsi(req) {
    kembalikan Tanggapan::json({
        "status": 200,
        "data": ["Budi", "Siti", "Widya"]
    });
});

server.tangani_post("/api/pesan", fungsi(req) {
    misal data = req.baca_json();
    kembalikan Tanggapan::json({
        "status": "Diterima",
        "pesan": data["pesan"]
    });
});

server.mulai();
```

---

## 8. Pengembangan Aplikasi Mobile (Android, iOS & PWA Offline-First)

WidyaMobile Framework menyatukan performa kompilasi native dengan arsitektur **Tri-Engine** (Android Kotlin, iOS Swift, dan PWA Offline-First) berbasis **10 Pilar Tingkat Industri**:

1. **Primitif Perangkat Keras**: GPS Haversine, Geofencing, Akselerometer 3-Axis, Baterai, Haptik.
2. **UI Seluler Industri**: Dynamic Notch Simulator, Live Clock, Safe-Area insets (`--sat`, `--sab`), ItemDaftar, Lencana, Sakelar, IndikatorProgres, AvatarWidget, BilahNavigasiBawah, TombolAksiMengambang, LembarBawah.
3. **Keamanan & Biometrik**: FaceID, Sidik Jari, Brankas Kunci Terisolasi Hardware KeyStore AES-256.
4. **PWA Offline-First & Audit Engine**: Web Manifest v2, Service Worker Cache-First, CLI `widya mobile periksa` (Predikat Enterprise Grade A+).
5. **Kamera & Scanner QRIS**: Decoder TLV QRIS Bank Indonesia (EMVCo/ASPI Tag 59, 60, 54, NMID, Checksum CRC16).
6. **Notifikasi Lokal & Push APNs/FCM**: Notifikasi instan, terjadwal, badge counter aplikasi.
7. **Antrean Offline Outbox & Network Sync**: Deteksi jaringan seluler & sinkronisasi idempoten saat online kembali.
8. **Mobile Bluetooth & Printer POS**: Pemindaian BLE/Classic, pencetakan struk kasir mini 58mm/80mm ESC/POS.
9. **Native Share Sheet, Deep Links & Papan Klip**: Dialog berbagi OS intent, skema `widya://`, salin papan klip aman.
10. **Basis Data Mobile Terenkripsi**: SQLite / SQLCipher Cache lokal terenkripsi AES-256 zero-trust.

### Contoh Kode Singkat:
```widya
impor "modul/mobile.wya";

fungsi utama() {
    misal app = BuatAplikasiMobile("KasirPOS", "emerald");

    // 1. Memindai & Mengurai QRIS Bank Indonesia
    misal qr = app.pindai_qr();
    misal qris = app.urai_qris(qr.konten);
    cetak("Merchant: " + qris.nama_merchant + " - Rp " + ke_teks(qris.nominal));

    // 2. Cetak Struk via Bluetooth Mini POS
    app.hubungkan_bluetooth("66:32:B1:88:AC:21");
    app.cetak_struk({
        "nama_toko": "WARUNG KOPI WIDYA",
        "total": qris.nominal,
        "metode_bayar": "QRIS Dinamis"
    });

    // 3. UI Deklaratif dengan Dynamic Notch & Safe-Area
    kembalikan Aplikasi({
        "judul": app.nama,
        "badan": Halaman({
            "badan": Kolom([
                TeksWidget("Terminal POS Aktif"),
                Tombol("Bayar & Cetak", fungsi() {
                    misal auth = app.otentikasi_biometrik("Otorisasi Pembayaran");
                    jika (auth.sukses) { app.getar("sukses"); }
                })
            ])
        })
    });
}
```

### Perintah Kompilasi & Audit Mobile:
```bash
# Inisialisasi proyek seluler baru
widya mobile inisialisasi NamaProyek

# Audit kesiapan kualitas & manifest (Enterprise Grade A+)
widya mobile periksa .

# Bangun paket PWA Offline-First
widya mobile bangun src/main.wya --target webview --output dist_pwa/

# Bangun paket APK (Android)
widya mobile bangun src/main.wya --target android --output build_android/

# Bangun paket IPA (iOS)
widya mobile bangun src/main.wya --target ios --output build_ios/
```

*Dokumentasi Lengkap:* Lihat [DOKUMENTASI_MOBILE.md](file:///c:/widya-lang/scratch/widya_mobile_framework/DOKUMENTASI_MOBILE.md) dan [PANDUAN_TUTORIAL_MOBILE.md](file:///c:/widya-lang/scratch/widya_mobile_framework/PANDUAN_TUTORIAL_MOBILE.md).

---

## 9. C/Native Foreign Function Interface (FFI)

Memanggil library C ABI dan shared library OS (`.dll`, `.so`, `.dylib`):

```widya
// Memanggil fungsi matematika libc C
misal hasil_akar = Ffi::panggil("sqrt", [144.0]);
cetak("Akar 144: " + ke_teks(hasil_akar)); // 12.0

// Memanggil C puts
Ffi::panggil("puts", ["Halo dari Native C ABI!"]);
```

---

## 10. Audit Keamanan & Pencegahan Kerentanan

Widya dilengkapi scanner statis untuk keamanan enterprise:

```bash
# Pindai seluruh direktori proyek
widya security scan .

# Pindai kebocoran kunci API & token rahasia
widya security secrets .

# Pasang otomatis proteksi ke Git pre-commit
widya security install-hook
```

---

## 11. Manajemen Memori & Profiler

Untuk aplikasi backend berskala besar, jalankan memory profiler untuk melacak alokasi heap dan mendeteksi siklus referensi (*circular memory leaks*):

```bash
widya profile server.wya
```

Contoh Laporan:
```
🧠 Widya-Lang Memory & Allocation Profiler Report
══════════════════════════════════════════════════
  Memori Aktif           : 42.50 KB
  Penggunaan Puncak (Peak): 128.00 KB
  Jumlah Objek Aktif     : 140
  Deteksi Siklus (Cycles): 0

Distribusi Penggunaan Tipe Data:
  • Teks            : 18400 bytes
  • Daftar          : 12500 bytes
  • Peta            : 11600 bytes

✅ Bebas kebocoran memori siklik (Zero cyclic leaks detected).
```

---

## 10. Pustaka Standar C FFI (SQLite3 & Kriptografi)

Widya menyediakan binding pustaka C tingkat tinggi bawaan yang siap pakai:

```widya
// Integrasi Basis Data SQLite3
misal db = Sqlite::buka("data.db");
db.eksekusi("CREATE TABLE pengguna (id INTEGER, nama TEXT);");
db.eksekusi("INSERT INTO pengguna VALUES (1, 'Budi');");
misal hasil = db.kueri("SELECT * FROM pengguna;");

// Kriptografi & Token Keamanan
misal hash = Kripto::sha256("rahasia");
misal token = Kripto::buat_token("auth_session");
```

---

## 11. Audit Keamanan & Pencegahan Kerentanan

Widya dilengkapi linter keamanan AST statis bawaan:

```bash
# Menjalankan pemindaian kerentanan pada proyek
widya audit .
widya security skrip.wya
```
- **Secret Scanner**: Mendeteksi kebocoran API Key (AWS, Stripe, GitHub, Private Keys).
- **SQLi Detector**: Mencegah konkatenasi string tidak aman pada kueri basis data.
- **XSS Detector**: Mendeteksi penyuntikan raw HTML tanpa sanitasi.

---

## 12. Manajemen Memori & Profiler

Widya menggabungkan **Safe Reference Counting** dan **Tarjan's Strongly Connected Components Cycle Detection** untuk mencegah kebocoran memori siklis:

```bash
# Memeriksa jejak alokasi heap dan pendeteksi siklus memori
widya profile skrip.wya
```

---

## 13. Widya Package Manager (WPM) & Lockfile

Untuk menjamin keamanan rantai pasok (*supply chain integrity*), WPM menyediakan lockfile otomatis:

```bash
# Membuat berkas widya.lock dengan checksum SHA-256
widya wpm lock

# Memverifikasi integritas checksum pustaka
widya wpm verify
```

---

## 14. Perkakas Developer (VS Code, LSP, DAP, REPL, Studio IDE, Web Playground)

Widya menyediakan ekosistem perkakas lengkap:
- **VS Code Extension**: Dukungan resmi di direktori `editors/vscode/` (TextMate grammar, LSP, dan DAP).
- **Language Server Protocol (LSP)**: `widya lsp` untuk auto-complete, diagnostics, hover, dan go-to definition.
- **Debug Adapter Protocol (DAP)**: `widya dap` untuk breakpoint debugging visual dan stack trace inspect.
- **Interactive WebAssembly Playground**: `www/index.html` dan `src/wasm_playground.rs` untuk mencoba Widya langsung di browser via WASM.
- **Interactive REPL**: `widya repl` dilengkapi `.waktu` (stopwatch) dan `.tipe` (type inspect).
- **Widya Studio**: IDE grafis lokal via `widya studio`.

---

## 15. Multi-Target Compilation

Widya dapat dikompilasi ke berbagai target komputasi khusus:

```bash
# WebAssembly untuk browser dan edge runtime
widya wasm modul.wya --output modul.wasm

# GPU Compute Shaders (WGSL / WebGPU)
widya gpu komputasi.wya --output shader.wgsl

# eBPF Linux Kernel Tracing & Probing
widya ebpf monitor.wya --output probe.c

# LLVM Intermediate Representation (IR)
widya llvm core.wya --output core.ll
```

---

## 16. Full Self-Hosting Compiler Pipeline

Widya-Lang kini **100% Full Self-Hosted** di mana seluruh modul kompiler ditulis dalam bahasa Widya (`compiler_self_hosted/`):
- `token.widya` & `lexer.widya`: Scanner leksikal mandiri.
- `ast.widya` & `parser.widya`: Parser gramatika rekursif.
- `typesystem.widya`: Inferensi tipe Hindley-Milner murni.
- `codegen.widya` & `main.widya`: Generator C-Emitter dan driver CLI.

Menjalankan verifikasi bootstrap deterministik multi-tahap (Stage-0 -> Stage-1 -> Stage-2):
```bash
widya bootstrap
```

---

## 17. Sub-Sistem Operasi & Kernel Primitives (`src/os`)

Widya memiliki modul kernel POSIX & microkernel primitives bawaan untuk sistem embedded, bare-metal, dan OS runtime:

- **Virtual Memory & Paging (`src/os/memory.rs`)**: Paging 4KB, isolasi Virtual Address Space, dan alokator halaman memori.
- **Kernel Threading & Scheduler (`src/os/threading.rs`)**: Preemptive Multi-Level Priority Scheduler dan context switching.
- **Interrupts & Traps (`src/os/interrupts.rs`)**: Global Interrupt Controller (GIC) dan Hardware Exception Handler.
- **Virtual Filesystem (`src/os/vfs/`)**: Inode-based VFS dengan mount points (`/`, `/dev`, `/proc`).
- **Syscall Dispatch Table (`src/os/syscall/`)**: POSIX interface untuk `read`, `write`, `open`, `close`, `fork`, `exec`.

---

## 18. Mesin Basis Data Mandiri WidyaBase (`src/widyabase`)

Widya menyediakan mesin basis data mandiri berkinerja native, berdaulat, dan multi-modal (*Industrial Autonomous Engine*):

- **Slotted-Page Storage 4KB & Buffer Pool (`src/widyabase/storage.rs`)**: Halaman data 4096-byte terstandar dengan slot directory dan free-space tracking, didukung in-memory LRU buffer pool.
- **Write-Ahead Logging / WAL (`src/widyabase/wal.rs`)**: Pencatatan siklik LSN, Group Commit, Checkpoint berkala, dan pemulihan otomatis pasca-crash (ARIES Recovery).
- **MVCC Snapshot Isolation (`src/widyabase/mvcc.rs`)**: Penanda transaksi `xmin` dan `xmax` (*readers never block writers, writers never block readers*).
- **Indeks B+Tree Disk-Based (`src/widyabase/btree.rs`)**: Pencarian kunci utama dan range scan O(log N) berlatensi sub-milidetik.
- **SQL Dwibahasa Simetris (`src/widyabase/sql_parser.rs`)**: Mendukung sintaks **Bahasa Indonesia** (`PILIH`, `DARI`, `DIMANA`, `SISIP_KE`, `NILAI`, `PERBARUI`, `ATUR`, `HAPUS_DARI`, `BUAT_TABEL`) dan **Standar Global** (`SELECT`, `FROM`, `WHERE`, dll) dengan AST kanonikal 100% identik.
- **Daemon Jaringan `pgwire` v3.0 (`src/widyabase/server.rs`)**: Server TCP mandiri di port 5432 yang kompatibel penuh dengan alat standar industri (**DBeaver, psql, Prisma ORM, Grafana**).
- **Multi-Modal AI & JSONB (`src/widyabase/multimodal.rs`)**: Kalkulasi kemiripan kosinus vektor HNSW untuk embedding AI/RAG dan ekstraksi dokumen semi-terstruktur JSONB.
- **Antarmuka CLI Terpadu**:
  - `widya base inisialisasi [path]` (Inisialisasi database `.wdb` baru)
  - `widya base mulai [--port 5432] [--data path]` (Menjalankan server daemon pgwire)
  - `widya base sql [--data path]` (Terminal interaktif REPL SQL dwibahasa)
  - `widya base kueri "<sql>" [--data path]` (Eksekusi kueri satu baris langsung)
  - `widya base periksa [path]` (Audit kesehatan halaman, tabel, dan log WAL)
- **Modul Standar**: [modul/widyabase.wya](file:///c:/widya-lang/modul/widyabase.wya) & Dokumentasi Lengkap di [DOKUMENTASI_WIDYABASE.md](file:///c:/widya-lang/DOKUMENTASI_WIDYABASE.md).

---

### 🇮🇩 Widya-Lang: Kedaulatan Teknologi & Standar Rekayasa Perangkat Lunak Modern.
