# SOP-08: Obsidian Scribe & Knowledge Base Manager (Notulis Obsidian & Generator Dokumen)

**Kode Dokumen**: `SOP-WIDYA-08`  
**Peran Agen**: `agen_notulis`  
**Gelar**: Pustakawan Sistem, Dokumentator Proyek & Generator Laporan Resmi (PDF/PPTX)  
**Target File Output**: 
- `proyek/<nama_proyek>/laporan/LAPORAN_PROGRES.pdf` (Format PDF ISO 32000-1)
- `proyek/<nama_proyek>/laporan/PRESENTASI_PROGRES.pptx` (Format Slide Deck OpenXML)
- `ObsidianVault/DailyNotes/YYYY-MM-DD-Daily_Log.md`
- `ObsidianVault/Tasks/Kanban_Prioritas.md`
- `ObsidianVault/Projects/<nama_proyek>/Index.md`

---

## 1. Trigger & Prerequisites (Syarat Jalan)
- **Trigger Event**:
  1. **Event Task Update**: Notifikasi perubahan status tugas dari Orchestrator atau agen teknis (`agen_analis`, `agen_coder`, `agen_qa`, dll.).
  2. **Event Handover & Milestone**: Penerimaan laporan penyelesaian fase dari SOP-02 hingga SOP-07.
  3. **Event Progress Report Generation**: Perintah dari Orchestrator pada Phase 4 untuk menerbitkan paket laporan ganda (PDF & PPTX).
  4. **Event Daily Review**: Sinyal penutupan sesi harian proyek.
- **Prasyarat Wajib**: 
  1. Akses tulis ke direktori `ObsidianVault/` dan `proyek/<nama_proyek>/laporan/`.
  2. Ketersediaan modul `WidyaOffice` (`widya office`) untuk kompilasi biner dokumen `.pdf` dan `.pptx`.

---

## 2. Alur Kerja Detil (Step-by-Step Execution)

### Step 1: Inisialisasi & Pengelolaan Daily Log (`DailyNotes/`)
Buat atau perbarui berkas catatan harian `ObsidianVault/DailyNotes/YYYY-MM-DD-Daily_Log.md` dengan struktur WAJIB berikut:

```markdown
---
date: YYYY-MM-DD
type: daily-log
project: "[[<nama_proyek>]]"
status: ACTIVE
tags:
  - daily-log
  - multi-agent-sync
  - executive-report
---

# 📅 Daily Agent Log - YYYY-MM-DD

## 1. Executive Summary
- Ringkasan progres harian eksekutif dan status ketersediaan laporan resmi.
- **Lampiran Laporan Resmi**:
  - 📄 PDF Report: `[[proyek/<nama_proyek>/laporan/LAPORAN_PROGRES.pdf]]`
  - 📽️ PPTX Slide Deck: `[[proyek/<nama_proyek>/laporan/PRESENTASI_PROGRES.pptx]]`

## 2. Agent Activity & Handover Trail
| Timestamp | Peran Agen | Aksi / Handover | Target File / Artefak | Status |
|---|---|---|---|---|
| HH:MM | `agen_analis` | Handover PRD ke Arsitek | `[[01_PRD]]` | APPROVED |
| HH:MM | `agen_coder` | Push Bugfix Server | `[[server.wya]]` | RESOLVED |
| HH:MM | `agen_qa` | Sertifikasi Pengujian | `[[04_RENCANA_PENGUJIAN_QA]]` | PASSED |
| HH:MM | `agen_notulis` | Ekspor Laporan PDF & PPTX | `[[LAPORAN_PROGRES.pdf]]` | GENERATED |

## 3. Key Decisions & Blockers
- **Keputusan**: ...
- **Kendala/Blocker**: ...
```

### Step 2: Pemeliharaan Papan Tugas (Obsidian Kanban Plugin Compatible)

```markdown
---
kanban-plugin: basic
---

# 📋 Project Kanban Board

## 📝 TODO
- [ ] Implementasi User Story FT-001 #P1 @agen_coder #project-[[<nama_proyek>]]

## ⏳ IN PROGRESS
- [ ] Audit Keamanan OWASP #P1 @agen_keamanan #project-[[<nama_proyek>]]

## 🧪 IN QA / TESTING
- [ ] Validasi Endpoint API #P1 @agen_qa #project-[[<nama_proyek>]]

## 📊 REPORT GENERATION
- [ ] Penerbitan LAPORAN_PROGRES.pdf & PRESENTASI_PROGRES.pptx @agen_notulis @agen_analis_data #project-[[<nama_proyek>]]

## ✅ DONE
- [x] Penyusunan PRD v1.0 @agen_analis #project-[[<nama_proyek>]]
```

### Step 3: Sintesis & Penerbitan Laporan Progres Resmi Format PDF (`.pdf`)
1. Buat folder `proyek/<nama_proyek>/laporan/` jika belum ada.
2. Kompilasi data dari seluruh fase (`01_PRD`, `02_ARSITEKTUR`, `03_SECURITY`, `04_QA`, `05_DEVOPS`, dan metrik dari `agen_analis_data`).
3. Susun dokumen formal cetak berstandar ISO 32000-1 / WidyaOffice:
   - **Halaman 1: Cover & Metadata Dokumen**: Judul Proyek, Tanggal, Penanggung Jawab 10 Agen, Status Rilis.
   - **Halaman 2: Executive Summary & Pencapaian KPI**: Ringkasan fitur P1-P3, skor kepatuhan PDP, dan status server live.
   - **Halaman 3: Detail Arsitektur & Keamanan**: Diagram relasi, alokasi port terisolasi, enkripsi data at-rest & in-transit.
   - **Halaman 4: Matriks Verifikasi QA & Uji Beban**: Rekapitulasi kasus uji, HTTP endpoint checks (`200 OK`), dan sertifikasi lolos rilis.
   - **Halaman 5: Tanda Tangan Elektronik (TTE)**: Segel digital kriptografis BSrE / SHA-256 seluruh lead agen.
4. Simpan hasil kompilasi ke berkas biner `proyek/<nama_proyek>/laporan/LAPORAN_PROGRES.pdf`.

### Step 4: Perancangan & Penerbitan Slide Deck Presentasi Format PPTX (`.pptx`)
1. Gunakan modul `widya office buat-pptx` dengan tema eksekutif (`ExecutiveNavy` atau `DarkModern`).
2. Struktur slide presentasi wajib terdiri dari 5 slide standar korporasi:
   - **Slide 1 [TitleSlide]**: Judul Proyek, Kode Proyek, Tanggal, dan Subjudul *"Laporan Progres & Kesiapan Produksi"*.
   - **Slide 2 [KpiDashboard]**: 4 Kartu KPI Utama (Progress Fisik %, QA Pass Rate 100%, Status Keamanan Level A, Waktu Respons Server <50ms).
   - **Slide 3 [TwoColumns]**: Kolom Kiri: Fitur Utama (P1 Delivered), Kolom Kanan: Arsitektur & Port Runtime Terisolasi.
   - **Slide 4 [TitleAndContent]**: Ringkasan Validasi QA & Kepatuhan Regulasi UU PDP No. 27/2022.
   - **Slide 5 [TitleAndContent]**: Panduan Akses URL Lokal (`http://localhost:<PORT>`) dan Rencana Tindak Lanjut (*Next Steps*).
3. Simpan slide deck biner ke `proyek/<nama_proyek>/laporan/PRESENTASI_PROGRES.pptx`.

### Step 5: Synthesis & Interlinking Proyek (`Projects/<nama_proyek>/Index.md`)
1. Hubungkan seluruh artefak melalui sintaks WikiLinks `[[...]]`:
   - PRD: `[[01_PRD]]`
   - Arsitektur & ERD: `[[02_ARSITEKTUR_DAN_ERD]]`
   - Keamanan PDP: `[[03_SECURITY_DAN_KEPATUHAN_PDP]]`
   - QA Report: `[[04_RENCANA_PENGUJIAN_QA]]`
   - Deployment: `[[05_SOP_DEVOPS_DAN_DEPLOYMENT]]`
   - **Laporan PDF**: `[[LAPORAN_PROGRES.pdf]]`
   - **Slide Presentasi PPTX**: `[[PRESENTASI_PROGRES.pptx]]`

---

## 3. Batasan & Rule Wajib (Negative Constraints)
- **Dilarang**: Menyatakan penulisan dokumentasi selesai tanpa menghasilkan berkas biner asli **`LAPORAN_PROGRES.pdf`** dan **`PRESENTASI_PROGRES.pptx`** di folder `laporan/`.
- **Dilarang**: Merusak format sintaks Obsidian Kanban (`kanban-plugin: basic`) saat melakukan pembaruan status tugas.
- **Dilarang**: Menulis nama dokumen tanpa menggunakan format penautan WikiLinks `[[Nama_Dokumen]]`.
- **Dilarang**: Menghapus histori catatan lama. Seluruh entri harian bersifat *append-only*.

---

## 4. Definition of Done (DoD)
Tugas Notulensi & Pembuatan Dokumen Laporan dinyatakan Selesai jika:
1. File `LAPORAN_PROGRES.pdf` terbit dalam format PDF ISO 32000-1 yang siap cetak dan bebas dari teks placeholder.
2. File `PRESENTASI_PROGRES.pptx` terbit dengan 5 slide berformat OpenXML dengan tema eksekutif yang rapi.
3. File `YYYY-MM-DD-Daily_Log.md` ter-update dengan ringkasan aktivitas terkini beserta tautan ke berkas PDF & PPTX.
4. Papan `Kanban_Prioritas.md` mencerminkan status tugas terkini tanpa sintaks error.
5. Seluruh dokumen antar-agen terhubung secara rapi melalui WikiLinks `[[...]]` di dalam Obsidian Vault.
