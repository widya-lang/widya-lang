# Pembaruan Peraturan Pertandingan Pencak Silat Nasional (PPPSN) Resmi 2026

Dokumen ini merangkum dan membedah seluruh pembaruan regulasi resmi **PB IPSI / PERSILAT Edisi 2026** menuju standardisasi internasional (*Road to Olympic*), perbandingannya dengan regulasi lama (2012/2022), serta penerapannya pada logic engine platform **Arena Silat**.

---

## 1. Matriks Perbandingan Regulasi (Aturan Lama vs Regulasi Resmi 2026)

| Dimensi Regulasi | Aturan Lama (2012 / 2022 Awal) | Regulasi Resmi 2026 (Terkini) | Implementasi di Engine Arena Silat |
| :--- | :--- | :--- | :--- |
| **Counter Attack Pasca Jatuhan** | Diperbolehkan menyerang balik selama 2 detik pasca bantingan. | **DILARANG KERAS (Strict Prohibition)**. Pesilat wajib langsung kembali ke sikap pasang. | Input pukulan/tendangan dalam 2 detik setelah jatuhan otomatis dianulir (diabaikan konsensus). |
| **Pegang Body Protector & Baju Atas** | Sering terjadi tarik-menarik tanpa sanksi tegas. | **Pelanggaran Sedang (Teguran)**. Menarik rompi pelindung/baju tanpa serangan langsung dihukum. | Tombol Sanksi KP: Teguran 1 (-1) dan Teguran 2 (-2) untuk tarikan rompi fiktif. |
| **Sistem Kartu VAR Pelatih** | 1 Kartu; hangus terlepas dari diterima atau ditolaknya protes. | **2 Kartu Protes Resmi**. Jika protes **DITERIMA**, kartu dikembalikan (tidak berkurang). Jika ditolak, kartu hangus. | Kuota default `vrCardsRemaining: 2`. Reset otomatis jika wasit/dewan menerima protes. |
| **Kategori Seni Prestasi** | Sempat ada pemenggalan materi (hanya tangan kosong pada babak tertentu). | **Wajib Full 3 Menit** (Tangan Kosong + Senjata Golok & Toya penuh). | Timer Seni: Target presisi 03:00 dengan toleransi penalti ±5 detik. |
| **Skor Poin Dasar** | Pukulan 1, Tendangan 2, Jatuhan 3. | **Tetap: Pukulan (+1), Tendangan (+2), Jatuhan (+3)**. | Peta skor `POINT_MAP` di `packages/scoring-engine` terbukti 100% compliant. |

---

## 2. Poin Perubahan Krusial Regulasi 2026

### A. Kategori Tanding (Fighter)
1. **Peniadaan Serangan Lanjutan (Zero Ground-and-Pound / Counter-Drop)**:
   - Begitu lawan menyentuh matras dan wasit menyatakan jatuhan sah, kedua pesilat dilarang melancarkan serangan apa pun dan wajib mengambil jarak kembali ke sikap pasang.
2. **Definisi Pelanggaran Tarik Rompi & Pakaian**:
   - Menarik *body protector*, kerah baju, atau celana lawan tanpa kombinasi teknik serangan langsung dikenakan status **Pelanggaran Sedang (Teguran)** untuk menjaga estetika bela diri pencak silat.

### B. Kategori Seni (TGR: Tunggal, Ganda, Regu)
1. **Integritas Penampilan Penuh 3 Menit**:
   - Seluruh rangkaian jurus tangan kosong, senjata pendek (golok/parang), dan senjata panjang (toya) ditampilkan secara utuh selama 180 detik.

### C. Transparansi VAR & Kartu Protes Pelatih (Official Video Review)
1. **Skema 2 Kartu Protes**:
   - Setiap sudut sudut (Merah dan Biru) dibekali **2 Kartu Protes**.
   - **Protes Diterima**: Wasit menganulir/mengesahkan poin, kartu VAR tetap utuh (tidak berkurang).
   - **Protes Ditolak**: Kartu VAR hangus (menjadi 1, lalu 0 / Kartu Merah).

---

## 3. Penyesuaian Kode pada Engine & UI Arena Silat

1. **`packages/types`**: Inisialisasi kartu VAR diatur default menjadi 2 kartu (`redVrCardsRemaining: 2, blueVrCardsRemaining: 2`).
2. **`apps/web-client`**: Modul VAR Review dan Scoreboard TV menampilkan slot 2 kartu protes interaktif.
