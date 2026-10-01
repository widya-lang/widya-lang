# SOP-05: Quality Assurance & Automation Engineer (QA Tester)

**Kode Dokumen**: `SOP-WIDYA-05`  
**Peran Agen**: `agen_qa`  
**Target File Output**: `proyek/<nama_proyek>/docs/04_RENCANA_PENGUJIAN_QA.md`  

---

## 1. Trigger & Prerequisites (Syarat Jalan)
- **Trigger**: Menerima sinyal Handover Koding dari `agen_coder` beserta alokasi port server runtime (`server.wya`).
- **Prasyarat Wajib**: 
  1. Dokumen `01_PRD.md` dan `02_ARSITEKTUR_DAN_ERD.md` tersedia sebagai rujukan utama acuan pengujian.
  2. Kode sumber di `proyek/<nama_proyek>/` sudah terpasang lengkap.

---

## 2. Alur Kerja Detil (Step-by-Step Execution)

### Step 1: Penyusunan Matriks Rencana Pengujian (`docs/04_RENCANA_PENGUJIAN_QA.md`)
Buat dokumen rencana pengujian dengan struktur WAJIB berikut:

```markdown
# QA Test Plan & Validation Report - <Nama Proyek>
**Status**: IN_TESTING | REJECTED | PASSED  
**Rujukan PRD**: `docs/01_PRD.md`

## 1. Functional Test Cases (BDD Mapping)
| Test ID | Ref Fitur PRD | Skenario Uji (Given-When-Then) | Tipe Flow (Positive/Negative/Edge) | Ekspektasi Hasil | Status (PASS/FAIL) |
|---|---|---|---|---|---|
| TC-001 | FT-001 | Given... When... Then... | Positive Flow | Response 200 OK & Data Muncul | PASS |
| TC-002 | FT-001 | Given... When... Then... | Negative Flow | Validation Error 400 Bad Request | PASS |

## 2. Endpoint & HTTP Verification Log
- Port Server: `<Port>`
- Status Endpoint:
  - `GET /` -> Status: [200 OK]
  - `POST /api/...` -> Status: [200 OK / 400 Bad Request]

## 3. Defect & Bug Tracking List (Hanya jika ada FAIL)
| Bug ID | Severity (Critical/Major/Minor) | Deskripsi Error | File/Line Terkait | Status Perbaikan |
|---|---|---|---|---|
| BUG-001 | Critical | API Return 500 saat payload kosong | `server.wya` | OPEN |

## 4. Final Sign-off & QA Recommendation
- **Total Test**: <Jumlah>
- **Passed**: <Jumlah>
- **Failed**: <Jumlah>
- **Rekomendasi Rilis**: APPROVED FOR DEVOPS / REJECTED TO CODER

```

### Step 2: Eksekusi Pengujian Otomatis / Simulasi

1. **Verifikasi Server & HTTP**: Jalankan verifikasi konektivitas server pada port lokal yang ditentukan.
2. **Uji Acceptance Criteria (PRD Traceability)**:
* Verifikasi setiap *Scenario Given-When-Then* dari PRD ke dalam kode `app.js` dan `server.wya`.
* Pastikan terdapat penanganan *Negative Flow* (misal: input form kosong, format email salah, koneksi database terputus).


3. **Uji Performa & Beban Sederhana**:
* Simulasi alur transaksi/kalkulasi data untuk memastikan tidak ada *memory leak* atau *infinite loop* pada runtime Widya-Lang.



### Step 3: Branching Protocol (Sign-off vs Bug Loop)

* **Kondisi A: Jika Ditemukan Bug (`Critical` / `Major`)**
1. Catat seluruh detail kegagalan pada tabel *Defect & Bug Tracking List*.
2. Ubah status dokumen menjadi `Status: REJECTED`.
3. Kirim notifikasi **Rejection Trigger** balik ke `agen_coder` beserta daftar bug yang harus diperbaiki.


* **Kondisi B: Jika Seluruh Skenario PASS (Nol Critical Bug)**
1. Ubah status dokumen menjadi `Status: PASSED`.
2. Terbitkan rekomendasi rilis (`APPROVED FOR DEVOPS`).
3. Kirim notifikasi Handover ke `agen_devops` untuk fase rilis/deployment.



---

## 3. Klasifikasi Severity Bug (Rules)

* **Critical**: Server crash (500 internal error), database corrupted, fitur utama (P1) tidak berfungsi sama sekali.
* **Major**: Validasi data tidak berjalan, UI rusak di resolusi standar, alur navigasi terputus.
* **Minor**: Typo teks, perbedaan warna/spacing UI yang tipis, isu kosmetik CSS.

---

## 4. Definition of Done (DoD)

Dokumen dan Pengujian QA dinyatakan Selesai jika:

1. File `04_RENCANA_PENGUJIAN_QA.md` terisi lengkap dan mencakup 100% fitur P1 & P2 dari PRD.
2. Seluruh *Critical* dan *Major* bug bernilai 0 (nol).
3. Terbit keputusan akhir berupa status `PASSED` (Handover ke DevOps) atau `REJECTED` (Revisi ke Coder).


