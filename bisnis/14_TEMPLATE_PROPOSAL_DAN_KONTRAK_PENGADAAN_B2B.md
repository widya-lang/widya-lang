# TEMPLATE PROPOSAL KOMERSIAL, SOW, DAN KONTRAK LISENSI B2B
**PT Widya Ekosistem Digital Indonesia (Widya-Lang Corporation)**
*Formulir Penawaran Resmi, Statement of Work (SOW), & Perjanjian Induk Layanan (MSA)*

---

## 1. TEMPLATE FORMULIR PENAWARAN RESMI (COMMERCIAL ORDER FORM)

```text
================================================================================
                    SURAT PENAWARAN HARGA LISENSI ENTERPRISE
                      Nomor Dokumen: WIDYA/QUO/2026/XXXX
================================================================================

Kepada Yth.
Pimpinan Divisi Teknologi Informasi / Procurement
[Nama Perusahaan / Institusi Klien]
[Alamat Klien]

Dengan hormat,
Bersama surat ini, kami dari PT Widya Ekosistem Digital Indonesia mengajukan
penawaran resmi pengadaan lisensi dan layanan dukungan teknis ekosistem Widya-Lang:

RINCIAN PAKET PENGADAAN:
--------------------------------------------------------------------------------
No | Deskripsi Produk / Layanan          | Qty  | Durasi | Harga Total (IDR)
--------------------------------------------------------------------------------
1. | Widya Enterprise Core License       | 3    | 1 Thn  | Rp   450.000.000
   | (Active-Active Raft Cluster HA)     | Node |        |
2. | Modul Switching FinTech / QRIS     | 1    | 1 Thn  | Rp   150.000.000
   | (Native Built-in Connector)         | Sys  |        |
3. | Paket SLA Mission-Critical 24/7/365 | 1    | 1 Thn  | Rp   120.000.000
   | (Respon P1 < 15 Menit + TAM)        | Pkt  |        |
4. | Voucher Ujian Sertifikasi WCP       | 15   | Sekali | Rp    26.250.000
   | (Ujian Standarisasi Insinyur Klien) | Pax  |        |
--------------------------------------------------------------------------------
SUBTOTAL                                                 : Rp   746.250.000
PPN 11%                                                  : Rp    82.087.500
TOTAL INVESTASI                                          : Rp   828.337.500
--------------------------------------------------------------------------------

SYARAT DAN KETENTUAN:
1. Masa Berlaku Penawaran: 30 Hari Kalender sejak tanggal diterbitkan.
2. Termin Pembayaran: 50% Uang Muka (DP) saat penandatanganan kontrak, 50% setelah
   Berita Acara Serah Terima (BAST) dan deployment selesai.
3. Garansi SLA & Hotfix: Aktif selama masa berlaku lisensi tahunan.

Hormat kami,
PT Widya Ekosistem Digital Indonesia

[Tanda Tangan & Meterai]
Direktur Komersial & Penjualan Enterprise
```

---

## 2. STRUKTUR STATEMENT OF WORK (SOW) IMPLEMENTASI SISTEM

Setiap proyek implementasi atau migrasi modernisasi sistem legacy wajib memuat:
1. **Ruang Lingkup Pekerjaan (*Scope of Work*)**: Daftar modul, arsitektur cluster server, dan target TPS yang disepakati.
2. **Jadwal & Milestone Pembayaran**:
   - Milestone 1: Desain Arsitektur & Persiapan Environment (20%).
   - Milestone 2: Migrasi Skema Basis Data ke WidyaBase (30%).
   - Milestone 3: Integrasi API / Switching QRIS & Uji Beban Stress-Test (30%).
   - Milestone 4: Go-Live & Sertifikasi Tim Klien (20%).
3. **Kriteria Keberhasilan (*Acceptance Criteria*)**: Tidak adanya galat data, latensi transaksi < 1 milidetik pada beban puncak, dan kelulusan seluruh unit test kepatuhan ISO 8583 / FHIR.

---
