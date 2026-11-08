# 🐹 Widya for Go Developers (Panduan Migrasi dari Go ke Widya-Lang)

Panduan komparatif bagi pengembang Go (Golang) yang ingin beralih ke Widya-Lang untuk mendapatkan performa tanpa jeda *Garbage Collector* serta keamanan memori tingkat kompilasi.

---

## 1. Perbandingan Konseptual Cepat

| Fitur | Go (Golang) | Widya-Lang |
| :--- | :--- | :--- |
| **Model Memori** | Garbage Collector (Tricolor Mark-Sweep) | MIR Borrow Checker (Affine Types, Zero GC) |
| **Model Konkurensi** | Goroutines (`go func()`) + Channels | Actor Supervision Tree (OTP) + Isolated Panic Boundaries |
| **Error Handling** | `if err != nil` | `Result<T, E>` / `Option<T>` dengan Pattern Matching |
| **Kompilasi** | Kompiler Go Cepat -> Biner ELF/Mach-O | Direct ELF-64 Emitter & LLVM JIT/AOT |
| **Latensi P99** | Terdistorsi oleh jeda GC (Stop-The-World) | Konsisten & Deterministik (Sub-milidetik P99) |
| **Keamanan Kripto** | Standar TLS 1.2/1.3 Klasik | Native Hybrid Post-Quantum TLS 1.3 + HSM PKCS#11 |

---

## 2. Goroutines vs Actor Supervision

### Masalah Goroutine di Go:
Di Go, jika sebuah goroutine mengalami `panic`, seluruh proses server akan mati (*crash*) kecuali setiap goroutine membungkus fungsinya secara manual dengan `defer func() { recover() }()`.

```go
// Go: Goroutine yang panik dapat mematikan seluruh aplikasi
go func() {
    panic("Database socket disconnected!") // CRASH seluruh proses!
}()
```

### Solusi Aktor di Widya-Lang:
Aktor di bawah pengawasan supervisor otomatis memiliki batas kepanikan terisolasi. Jika aktor panik, supervisor memulihkannya secara instan tanpa mengganggu koneksi klien lain:

```widya
biar pengawas = PohonPengawas::baru(Strategi::OneForOne, 5);
biar pekerja = pengawas.buat_aktor("pekerja-antrean", || {
    kembalikan |pesan| {
        jika pesan == "KORUP" {
            panik("Data korup terdeteksi!"); // Supervisor menangkap & merestart aktor
        }
        kembalikan Ok(TidakAda);
    };
});

pekerja.kirim("KORUP"); // Aplikasi TETAP BERJALAN normal!
```

---

## 3. Penanganan Galat: `if err != nil` vs Pattern Matching

### Go:
```go
data, err := readFile("config.json")
if err != nil {
    log.Fatalf("Gagal membaca: %v", err)
}
```

### Widya-Lang:
```widya
pilih baca_berkas("config.json") {
    Ok(data) => cetak_baris("Berhasil membaca: {}", data),
    Galat(e) => cetak_baris("Gagal membaca konfigurasi: {}", e),
}
```

---

## 4. Kapan Harus Menggunakan Widya Dibandingkan Go?
- **Sistem Misi-Kritis & Freestanding:** Bila aplikasi harus berjalan di lingkungan embedded atau boot unikernel microVM (AWS Firecracker) tanpa runtime GC.
- **Kepatuhan Kriptografi Pertahanan:** Bila regulasi mewajibkan tanda tangan digital berbasis enklaf perangkat keras (PKCS#11 HSM) dan algoritma pasca-kuantum (FIPS 203/204).
- **Integritas Ekosistem AI:** Bila Anda membangun agen AI yang membutuhkan integrasi standar Model Context Protocol (MCP) untuk Cursor atau Claude Desktop.
