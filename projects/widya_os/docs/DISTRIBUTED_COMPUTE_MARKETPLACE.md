# Spesifikasi Pasar Komputasi Terdistribusi & Langganan Cloud (WidyaGrid) ⚡☁️

> **Arsitektur Komputasi Sesuai Permintaan (On-Demand Compute-as-a-Service & DePIN)**  
> *Sewa Daya Komputasi Transparan · Enkripsi Confidential Computing TEE · Langganan Hybrid Cloud & Tokenized Ledger*

---

## 1. Pendahuluan & Peluang Pasar

Banyak pengguna menggunakan perangkat laptop portabel hemat daya (spesifikasi 4-core, RAM 4–8 GB) untuk mobilitas harian. Namun, pada momen-momen tertentu, mereka membutuhkan daya komputasi yang sangat masif, misalnya:
- **Pelatihan atau Inferensi AI Model Raksasa (LLM 70B+ Parameter)** untuk asisten pintar WidyaNeural.
- **Pemrosesan Dokumen & Mail Merge Skala Besar (Ratusan Ribu Berkas)** di WidyaWriter.
- **Kompilasi Proyek Software Raksasa & Simulasi Finansial Kompleks** di WidyaCalc.
- **Rendering Animasi Video 4K/8K** di WidyaMedia.

Jika harus membeli laptop workstation bertenaga tinggi seharga Rp 40–80 juta, biayanya sangat mahal dan boros listrik.

**WidyaOS menghadirkan WidyaGrid**: sebuah subsistem bawaan di mana pengguna dapat **menyewa jasa komputasi awan (*Cloud Compute*) secara fleksibel (berlangganan bulanan atau bayar per-penggunaan / *pay-as-you-go*) langsung dari antarmuka desktop saat terhubung ke internet.**

---

## 2. Arsitektur Dua-Arah (Two-Way Compute Marketplace)

WidyaGrid beroperasi sebagai ekosistem dua-arah (*Decentralized Physical Infrastructure Network - DePIN*):

```
┌────────────────────────────────────────────────────────────────────────┐
│                        WIDYAGRID COMPUTE MARKETPLACE                   │
├───────────────────────────────────┬────────────────────────────────────┤
│   PENGGUNA KONSUMEN (PENYEWA)     │    PENYEDIA KOMPUTASI (HOST)       │
│  - Mahasiswa / Karyawan Laptop    │  - Server Instansi Pemerintah      │
│  - Membutuhkan Akselerasi GPU/CPU │  - Laboratorium Komputer Kampus    │
│  - Berlangganan Kuota Komputasi   │  - PC Desktop Ber-GPU Menganggur   │
└─────────────────┬─────────────────┴──────────────────┬─────────────────┘
                  │                                    │
                  │ Permintaan Tugas                   │ Menawarkan Daya
                  │ Terenkripsi                        │ Komputasi
                  ▼                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│               WIDYAGRID ROUTER & SMART LEDGER (WIDYALEDGER)            │
│  - Penjadwalan Tugas Cerdas (Beban Terdistribusi via Raft / Actor)     │
│  - Validasi Pembayaran / Pengurangan Saldo Berlangganan Otomatis       │
│  - Verifikasi Bukti Kerja Komputasi (Proof-of-Computation Verification)│
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                  Eksekusi Terisolasi di Hardware Aman
                  (Confidential Computing TEE: AMD SEV / Intel TDX)
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                   HASIL KOMPUTASI TERENKRIPSI PQC                      │
│            Dikirim kembali ke laptop pengguna dalam mikrodetik         │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Pengalaman Pengguna (UX) di Desktop WidyaOS

Integrasi layanan komputasi ini dibuat sangat mulus di antarmuka sistem:

```
┌────────────────────────────────────────────────────────────────────────┐
│  WidyaWriter - Pemrosesan Surat Massal (100.000 Rekaman Data)          │
├────────────────────────────────────────────────────────────────────────┤
│ Estimasi Komputasi Lokal (Laptop Core i3): 48 Menit.                   │
│                                                                        │
│ [ ⚡ SELESAIKAN LEBIH CEPAT DENGAN WIDYAGRID CLOUD ]                   │
│ - Node Tersedia : 64-Core AMD EPYC + Akselerasi AI                     │
│ - Estimasi Waktu: 4,2 Detik                                            │
│ - Biaya Tugas   : Rp 250 (Dipotong dari Saldo Langganan Bulanan)       │
│                                                                        │
│ [x] Gunakan Enkripsi Rahasia PQC (Pemilik server tidak bisa baca data) │
│                                                                        │
│ [ Batal ]                                   [ Eksekusi di Awan (Enter) ]
└────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Jaminan Privasi Mutlak: *Confidential Computing (TEE)*

Kekhawatiran terbesar pengguna saat menyewa jasa komputasi pihak ketiga adalah:  
*"Apakah dokumen rahasia instansi, data keuangan, atau percakapan AI saya bisa dibaca oleh pemilik server yang saya sewa?"*

**Jawaban WidyaOS: TIDAK BISA SAMA SEKALI!**

WidyaGrid menerapkan proteksi **Confidential Computing tingkat perangkat keras**:
1. **Enkripsi dalam Memori (Encrypted Enclaves):** Tugas komputasi dikemas dalam format modul WebAssembly berdaulat ([src/wasm_component.rs](file:///home/devops/widya-lang/src/wasm_component.rs)).
2. **Kunci Enkripsi Terkunci di HSM Pengguna:** Data dokumen dikirim dalam kondisi terenkripsi penuh. CPU server komputasi mengeksekusi komputasi di dalam ruang terisolasi (*hardware-encrypted enclave* seperti AMD SEV-SNP atau Intel TDX).
3. **Bahkan Administrator Server Tidak Bisa Membaca:** Bahkan administrator yang memiliki akses fisik ke server komputasi tidak dapat membaca isi memori atau melihat dokumen pengguna.
4. **Hasil Diberi Segel Digital PQC:** Hasil keluaran dokumen dienkripsi kembali dengan kunci publik pengguna sebelum dikirim kembali ke laptop pengguna.

---

## 5. Model Bisnis & Skema Berlangganan Fleksibel

WidyaOS menyediakan dua metode bagi pengguna untuk mengakses jasa komputasi awan:

### A. Model Berlangganan Bulanan (Sovereign Cloud Subscription)
Didesain untuk instansi, kantor, kampus, dan pekerja profesional:

| Paket Layanan | Harga / Bulan | Kuota Komputasi | Kapasitas AI & Fitur |
| :--- | :--- | :--- | :--- |
| **Widya Personal** | Rp 49.000 / bln | 20 Jam CPU Compute | Akses AI Asisten 7B, Ekspor Mail Merge Cepat |
| **Widya Pro** | Rp 149.000 / bln | 100 Jam CPU + 15 Jam GPU | Akses AI Raksasa 70B, Rendering 4K, Prioritas Antrean |
| **Widya Enterprise** | Rp 499.000 / instansi | Kuota Komputasi Dedicated | Integrasi Server Lokal Kantor, Audit Keamanan HSM |

### B. Model Pay-As-You-Go (Mikrotransaksi via WidyaLedger)
- Pengguna cukup mengisi saldo dompet digital berdaulat (*Sovereign Wallet*).
- Setiap kali pengguna menjalankan tugas komputasi berat, biaya dipotong secara transparan per-detik komputasi nyata (*per-second billing*).

### C. Menghasilkan Pendapatan (Menyewakan PC Pribadi yang Menganggur)
- Pengguna yang memiliki PC Gaming atau Server di kantor dapat mengaktifkan toggle *"Sumbangkan Daya Komputasi Saat Idle"* di malam hari.
- PC mereka akan menjalankan tugas komputasi terenkripsi dari pengguna lain dan secara otomatis menghasilkan saldo token/rupiah yang bisa digunakan untuk memotong biaya langganan bulanan.
