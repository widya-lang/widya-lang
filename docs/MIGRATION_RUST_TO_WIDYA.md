# 🦀 Widya for Rust Developers (Panduan Migrasi dari Rust ke Widya-Lang)

Panduan ini ditujukan bagi pengembang Rust yang ingin memahami kesamaan arsitektur, perbedaan sintaksis, serta keunggulan unik dari Widya-Lang.

---

## 1. Perbandingan Konseptual Cepat

| Fitur | Rust | Widya-Lang |
| :--- | :--- | :--- |
| **Model Memori** | Ownership, Borrowing & Lifetimes | Affine Type System, MIR Borrow Checker |
| **Garbage Collector** | Tidak Ada (Deterministic Drop) | Tidak Ada (Zero-Cost Destruction) |
| **Tingkat Kompiler** | rustc -> LLVM Backend | Self-Hosted Compiler (Direct ELF-64 Emitter & LLVM Option) |
| **Model Konkurensi** | Threads, Async/Await (Tokio) | Native Actor Supervision (Erlang OTP) + Async Channels |
| **Serverless & Edge** | WASI Preview 1 via wasm32-wasi | WASI 0.2 Preview 2 (Component Model) + Microsecond Snapshot Engine |
| **Kriptografi** | RustCrypto / Ring Crates | Native HSM PKCS#11 v2.40 + Post-Quantum TLS 1.3 (FIPS 203 & 204) |
| **AI Protocol** | Pustaka Eksternal | Native Model Context Protocol (MCP v2024-11-05) Server & Client |

---

## 2. Perbandingan Sintaksis & Kepemilikan Memori

### Rust:
```rust
struct User {
    id: u64,
    name: String,
}

fn process_user(mut user: User) -> User {
    user.name.push_str(" [Verified]");
    user
}

fn main() {
    let u = User { id: 1, name: String::from("Budi") };
    let updated = process_user(u);
    // println!("{}", u.name); // Error E0382: value borrowed here after move!
    println!("{}: {}", updated.id, updated.name);
}
```

### Widya-Lang:
```widya
struktur Pengguna {
    id: u64,
    nama: String,
}

fungsi proses_pengguna(mut user: Pengguna) -> Pengguna {
    user.nama.tambah(" [Verified]");
    kembalikan user;
}

fungsi utama() {
    biar u = Pengguna { id: 1, nama: "Budi" };
    biar diperbarui = proses_pengguna(u);
    // cetak_baris(u.nama); // Error: nilai telah dipindahkan (moved value)!
    cetak_baris("{}: {}", diperbarui.id, diperbarui.nama);
}
```

---

## 3. Peminjaman & Non-Lexical Lifetimes (NLL)
Mirip dengan Rust 2018/2021 NLL:
```widya
biar mut data = [1, 2, 3];
biar ref_pinjam = &data[0];
cetak_baris("Nilai elemen: {}", ref_pinjam);

// Valid di Widya MIR BorrowCK: ref_pinjam tidak lagi aktif setelah baris di atas
data.tambah(4); 
```

---

## 4. Keunggulan Unik Widya Dibandingkan Rust
1. **Zero External Crates Requirement (Sovereign Stack):**
   Di Rust, membuat web server dengan TLS, database, dan format JSON umumnya membutuhkan `tokio`, `hyper`, `rustls`, `sqlx`, dan `serde`. Di Widya-Lang, seluruh modul ini dibangun langsung di dalam pustaka standar tanpa risiko kerentanan dependensi.
2. **Actor Supervision vs Manual Thread Handling:**
   Rust memerlukan arsitektur thread/task rumit untuk menangani crash task (`tokio::spawn` tidak menyediakan supervision tree bawaan). Widya menyediakan pohon pengawasan `OneForOne`, `OneForAll`, dan `RestForOne` out-of-the-box dengan isolasi batas *panic*.
3. **Microsecond Cold-Start vs Standar WASM:**
   WASI Component Model di Widya-Lang dilengkapi dengan `EdgeSnapshotEngine` yang menyimpan dan merestorasi status eksekusi VM dalam waktu kurang dari 2 mikrodetik.
