# Strategi Pengujian Kualitas & Validasi Industri (QA & Testing) 🧪

> **Spesifikasi Jaminan Kualitas Standar Industri (Zero-Crash & Zero-Warning)**  
> *Golden Master Corpus · Round-Trip XML Compliance · Latency & Memory Budgets*

---

## 1. Filosofi & Standar Pengujian WidyaOffice

Aplikasi perkantoran adalah instrumen bisnis kritis di mana kegagalan sekecil apa pun (*seperti aplikasi crash di tengah pengetikan skripsi/kontrak atau korupsi berkas saat disimpan*) tidak dapat ditoleransi.

Oleh karena itu, WidyaOffice Desktop menganut filosofi:
- **Zero Panic Guarantee:** Parser format berkas dan penanganan IPC tidak boleh memicu `panic!` runtime di bawah kondisi apa pun (termasuk berkas rusak atau serangan biner).
- **Strict Zero Compiler Warnings:** Seluruh kode wajib lulus kompilasi dengan `0 warnings` pada compiler rustc/widya standar industri.
- **Strict Performance Budgets:** Batasan latensi dan konsumsi memori dipantau otomatis di setiap pipeline pengujian.

---

## 2. Hirarki Pengujian 4-Tingkat (4-Tier Testing Pipeline)

```
┌────────────────────────────────────────────────────────┐
│           TINGKAT 4: PENGUJIAN AKSEPTASI UI            │
│   (Simulasi Jendela, Responsivitas Ketik < 8ms, Kiosk) │
├────────────────────────────────────────────────────────┤
│         TINGKAT 3: GOLDEN MASTER & ROUND-TRIP          │
│   (1.000 Dokumen Nyata Korporat, Uji Validasi Skema)   │
├────────────────────────────────────────────────────────┤
│           TINGKAT 2: FUZZING & KEAMANAN BINER          │
│   (Zip Slip, XML Bomb, Berkas Rusak, Buffer Overflow)  │
├────────────────────────────────────────────────────────┤
│             TINGKAT 1: PENGUJIAN UNIT MURNI            │
│   (OMML Translasi, Mail Merge Batch, CRDT Undo/Redo)   │
└────────────────────────────────────────────────────────┘
```

---

## 3. Ambang Batas Kinerja (Performance Budgets)

Setiap build rilis WidyaOffice Desktop diuji terhadap metrik ambang batas (*threshold metrics*) berikut:

| Parameter Pengujian | Target Maksimal / Minimal | Prosedur Verifikasi |
| :--- | :--- | :--- |
| **Cold-Start Jendela** | **< 15 milidetik** | Mengukur waktu dari eksekusi binary hingga UI siap menerima input kursor |
| **Konsumsi Memori Idle**| **< 35 MB RAM** | Diukur pada dokumen kosong 1 halaman setelah 60 detik idle |
| **Latensi Mengetik** | **< 8 milidetik** | Waktu dari penekanan tombol (*keydown*) hingga piksel ter-render di kanvas |
| **Throughput Mail Merge**| **> 2.500 dok/detik** | Pembuatan berkas DOCX terpisah pada mesin uji prosesor 8-core standar |
| **Dokumen 1.000 Halaman**| **Stabil di 60 FPS** | Scrolling cepat dari halaman pertama hingga halaman terakhir |

---

## 4. Pengujian Golden Master & Interoperabilitas Round-Trip

Untuk menjamin kepatuhan 100% terhadap ekosistem Microsoft Word dan LibreOffice:
1. **Corpus Uji 1.000 Berkas:** Kumpulan dokumen korporat, skripsi, lembar kalkulasi keuangan, dan presentasi nyata yang mencakup:
   - Rumus matematika bertingkat (integrasi, matriks, pecahan bersarang).
   - Tabel dengan penggabungan sel (*merged cells* vertikal dan horizontal).
   - Penomoran bertingkat (*multi-level bulleting*) dan catatan kaki (*footnotes*).
   - Cap air (*watermark*) dan header/footer berbeda pada halaman pertama.
2. **Uji Siklus Penuh (Round-Trip Test Cycle):**
   ```
   [ Berkas Word Asli (.docx) ]
               │
               ▼
   [ Buka di WidyaOffice Desktop ]  ───► Pastikan 0 Error Parsing
               │
               ▼
   [ Sisipkan Paragraf & Rumus OMML ]
               │
               ▼
   [ Simpan Ulang ke .docx ]
               │
               ▼
   [ Buka di Microsoft Word 365 ]   ───► Pastikan Tidak Ada Pesan "File Corrupt"
                                          & Rumus Dapat Diedit di Word
   ```

---

## 5. Pengujian Fuzzing & Keamanan Ekstrem

Modul [src/office/zip.rs](file:///home/devops/widya-lang/src/office/zip.rs) dan dekomposer XML diuji menggunakan teknik mutasi biner (*fuzz testing*):
- **Serangan Zip Slip:** Memastikan entri berkas ZIP dengan path berbahaya (seperti `../../../../etc/passwd`) otomatis diblokir dan tidak menembus direktori luar.
- **Serangan XML Bomb (Billion Laughs):** Pencegahan ekspansi rekursif entitas XML tak terbatas yang berpotensi menghabiskan memori sistem.
- **Ekspresi Rumus Rekursif:** Sintaks LaTeX tak terhingga (misal pecahan bersarang 5.000 tingkat) dipotong secara aman dengan pesan kesalahan semantik yang jelas tanpa menyebabkan *stack overflow*.

---

## 6. Perintah Menjalankan Pengujian

```bash
# Menjalankan seluruh pengujian unit office (DOCX, OMML, Mail Merge, CRDT, PDF)
cargo test --test office_tests

# Menjalankan pengujian desktop engine (Windowing, IPC Token, Hardware)
cargo test --test desktop_tests
cargo test --test pure_desktop_parity_test

# Menjalankan verifikasi benchmark kinerja industri
cargo test --test industrial_benchmarks_test

# Memastikan kepatuhan zero-warning di seluruh target
cargo check --lib --tests && cargo check --bin widya
```
