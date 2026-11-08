# Changelog

Semua perubahan penting pada ekosistem bahasa pemrograman Widya-Lang didokumentasikan dalam berkas ini.

Format berkas ini didasarkan pada [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), dan versi rilis mengikuti standar [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-11-08

### Ditambahkan (Added)
- **Kompiler Mandiri Deterministik (*Full Self-Hosting*)**: Pipeline kompilasi mandiri Stage-0 → Stage-1 → Stage-2 terverifikasi deterministik bit-for-bit (`widya bootstrap`).
- **Sistem Tipe Hindley-Milner**: Inferensi tipe otomatis, Algebraic Data Types (ADT `pilihan`), dan *exhaustive pattern matching* (`cocokkan`).
- **Mesin Virtual Berbasis Register**: Eksekusi bytecode berkinerja tinggi, alokasi memori slab pool terpadu, dan borrow checker statis tanpa jeda *garbage collector*.
- **Pilar Kemandirian (*Direct Syscalls*)**: Akses langsung kernel OS (Linux x86_64 & AArch64) melalui raw syscalls tanpa perantara glibc/musl.
- **Kriptografi Pasca-Kuantum (PQC)**: Implementasi algoritma standar NIST FIPS 203 (ML-KEM Kyber-768/1024) dan FIPS 204 (ML-DSA Dilithium).
- **Manajer Paket WPM**: Penyelesai pohon dependensi berbasis PubGrub, verifikasi tanda tangan Ed25519, SHA-256 lockfile, serta mode pasok (*offline vendoring*).
- **Paket Standar Resmi WPM**: Pustaka `@widya/tensor`, `@widya/dataframe`, `@widya/ai`, `@widya/sql`, `@widya/web`, dan `@widya/test`.
- **Perkakas Pengembang Terintegrasi**: CLI terpadu (`widya run`, `widya compile`, `widya test`, `widya format`, `widya lsp`).
- **Ekstensi Visual Studio Code**: Penyorotan sintaksis dwibahasa, cuplikan kode (*snippets*), dan semantic tokens (`editors/vscode/widya-lang-0.1.0.vsix`).
- **Buku Panduan & Tutorial ISO 32000-1**: Tutorial Level Dasar (229 halaman) dan Level Tingkat Lanjut berserta 80 modul slide presentasi PPTX.
