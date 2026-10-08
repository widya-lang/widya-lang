# RENCANA PENGUJIAN JAMINAN MUTU (QA TEST PLAN)
## APLIKASI RENCANA KEGIATAN DAN ANGGARAN SEKOLAH (ARKAS) WEB TERINTEGRASI
**Kode Proyek:** `PRJ-ARKAS-WEB-01` | **Status:** 100% Pengujian Otomatis Lulus

---

### 1. Lingkup & Matriks Pengujian
Pengujian mencakup aspek kebenaran matematis akuntansi (presisi keuangan sen tanpa floating-point round-off error), integritas rantai audit, validasi batas juknis belanja honorarium BOSP, dan pengujian integrasi antarmuka web.

| Kode Uji | Skenario Pengujian | Hasil yang Diharapkan | Status |
|:---------|:-------------------|:----------------------|:------:|
| **QA-ARKAS-01** | Validasi Multi-Sumber Dana BOSP, BOS Kinerja, BOSDA | Dana dicairkan per tahap dengan pencatatan rekening bank terpisah | `PASS` |
| **QA-ARKAS-02** | Validasi Batas Honorarium BOSP <= 50% | Sistem menolak pengesahan RKAS jika alokasi honor melebihi 50% | `PASS` |
| **QA-ARKAS-03** | Pengesahan RKAS Murni & Pergeseran Internal | Pergeseran internal tidak mengubah pagu total, sandingan before-after tercatat | `PASS` |
| **QA-ARKAS-04** | Pengadaan SIPLah Permen 15/2026 Lengkap 6 Dokumen | DPH, DNH, SP, BAST, Uji Fisik, dan Bukti Bayar terverifikasi | `PASS` |
| **QA-ARKAS-05** | Integritas Rantai Audit Hash SHA-256 BKU | Deteksi modifikasi liar saldo masa lalu memicu status unverified | `PASS` |
| **QA-ARKAS-06** | Penarikan Tunai & Rekonsiliasi Kas Bendahara | Saldo Bank berkurang dan Saldo Tunai bertambah secara simetris | `PASS` |
| **QA-ARKAS-07** | Kalkulasi Otomatis Pajak PPN 11%, PPh 21, PPh 22, PPh 23 | Tarif pajak sesuai ketentuan NPWP/Non-NPWP dan batas nominal | `PASS` |
| **QA-ARKAS-08** | Ekstraksi Buku Pembantu Kas, Bank, dan Pajak | Rekonsiliasi tiga buku pembantu cocok 100% dengan saldo akhir BKU | `PASS` |

---

### 2. Hasil Eksekusi Uji Integrasi Rust (`cargo test`)
Eksekusi pengujian dilakukan melalui dua suite pengujian utama:
1. `tests/arkas_multi_sumber_dana_test.rs`: 4 pengujian lulus (4 passed, 0 failed).
2. `tests/arkas_bos_budget_test.rs`: 5 pengujian lulus (5 passed, 0 failed).

Total skor kelulusan: **100% (9/9 pengujian lulus sempurna)**.
