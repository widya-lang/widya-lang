# 🛠️ Spesifikasi Arsitektur Self-Hosting Compiler Widya-Lang

Spesifikasi ini menjelaskan implementasi teknis dan landasan matematis dari kompiler *Self-Hosting* penuh Widya-Lang (`v0.1.0`), membuktikan bahwa bahasa ini dapat mengompilasi dirinya sendiri tanpa ketergantungan pada runtime bahasa lain.

---

## 1. Siklus Bootstrap 3-Tahap (Stage-0 → Stage-1 → Stage-2)

Kompiler mandiri Widya-Lang beroperasi melalui siklus bootstrap tiga tahap untuk mencapai konvergensi deterministik bit-for-bit:

```
+-----------------------------------------------------------+
| Stage-0 (Bootstrap Host: Rust Implementation)             |
| Menerjemahkan kode sumber kompiler murni Widya ke C99     |
+-----------------------------+-----------------------------+
                              |
                              v
                +----------------------------+
                | Stage-1 Compiler (C Code)  | ---> SHA-256 Hash_1
                +-------------+--------------+
                              |
                              | Menjalankan dirinya sendiri
                              v
                +----------------------------+
                | Stage-2 Compiler (C Code)  | ---> SHA-256 Hash_2
                +-------------+--------------+
                              |
                              v
        +---------------------------------------------+
        | Validasi Deterministik: Hash_1 == Hash_2    |
        | Penandatanganan Kriptografis Kurva Ed25519   |
        +---------------------------------------------+
```

### Definisi Tahapan:
1. **Stage-0:** Host interpreter/compiler berbasis Rust (`src/bootstrap.rs` dan `crates/widya-compiler/src/selfhost.rs`). Bertindak sebagai peluncur awal (*seed*) yang membaca modul kompiler murni Widya di `compiler_self_hosted/` dan menghasilkan representasi kode C tahap pertama (`Stage-1`).
2. **Stage-1:** Kode sumber C hasil translasi tahap pertama. Kode ini berisi logika penuh kompiler Widya yang ditulis dalam C99 ANSI murni tanpa ketergantungan eksternal selain libc standar.
3. **Stage-2:** Biner yang dihasilkan ketika Stage-1 digunakan untuk mengompilasi kembali seluruh modul `compiler_self_hosted/`.

### Kriteria Kelulusan (*Convergence Guarantee*):
$$\text{SHA-256}(\text{Stage-1}) == \text{SHA-256}(\text{Stage-2})$$

Jika kedua hash kriptografis identik bit-for-bit, maka kompiler terbukti 100% deterministik dan bebas dari anomali translasi (*fixpoint reached*).

---

## 2. Struktur Modul Mandiri (`compiler_self_hosted/`)

Seluruh logika kompiler mandiri ditulis dalam bahasa Widya murni dengan ketersediaan berkas ganda (`.wya` dan `.widya`):

| Modul | Peran Arsitektur |
| :--- | :--- |
| `token.wya` | Definisi struktur enum token leksikal, jenis token (`KataKunci`, `Pengenal`, `Angka`, `Teks`, `Simbol`), dan pelacakan posisi baris/kolom. |
| `lexer.wya` | Analisis leksikal berbasis *lookahead*, penanganan komentar, pemindaian string ber-escape, dan tokenisasi kode sumber berkecepatan tinggi. |
| `ast.wya` | Representasi Pohon Sintaks Abstrak (*Abstract Syntax Tree*): deklarasi fungsi, ekspresi binari, variabel lokal, kontrol alur (`jika`, `untuk`, `selama`), dan pemanggilan modul. |
| `parser.wya` | Parser *Recursive Descent* dengan algoritma *Operator Precedence Climbing* untuk memastikan hierarki prioritas ekspresi matematika dan logika. |
| `typesystem.wya` | Sistem inferensi tipe statis Hindley-Milner, unifikasi tipe primitif, dan pencegahan *type mismatch* sebelum fase emisi. |
| `codegen.wya` | Pembangkit kode emisi C99 yang memetakan konstruksi bahasa Widya ke kode C terstruktur dan deterministik. |
| `main.wya` | Driver CLI mandiri yang menerima argumen baris perintah, memanggil tahapan pemrosesan, dan memproduksi berkas keluaran. |

---

## 3. Penandatanganan Kriptografis Ed25519

Untuk menjamin kedaulatan pasokan (*supply-chain integrity*), setiap hasil verifikasi bootstrap mandiri ditandatangani menggunakan kurva kriptografi Ed25519:
- **Kunci Publik (32 Byte):** Digunakan untuk verifikasi tanda tangan oleh pihak ketiga atau pipeline CI/CD.
- **Tanda Tangan Digital (64 Byte):** Mengunci hash biner Stage-1 dan Stage-2, memastikan bahwa biner kompiler tidak mengalami modifikasi atau injeksi kode berbahaya di tingkat rilis.

Verifikasi ini dapat dijalankan kapan saja melalui perintah:
```bash
widya bootstrap
```
atau melalui Rust integration tests:
```bash
cargo test --test self_hosting_tests
```
