# SOP-02: Business & Product Analyst (Analis Produk & Kebutuhan)

**Kode Dokumen**: `SOP-WIDYA-02`  
**Peran Agen**: `agen_analis`  
**Target File Output**: `proyek/<nama_proyek>/docs/01_PRD.md`  

---

## 1. Trigger & Prerequisites (Syarat Jalan)
- **Input Mandat**: Pesan dari Orchestrator/Pengguna berupa deskripsi proyek atau file rujukan di `bisnis/`.
- **Prasyarat**: Agen wajib membaca seluruh isi direktori `bisnis/` (jika ada) sebelum menyusun PRD.

---

## 2. Alur Kerja Detil (Step-by-Step Execution)

### Step 1: Gap Analysis & Clarification Loop
1. Evaluasi input proyek dari pengguna/bisnis.
2. Jika deskripsi kebutuhan memiliki tingkat ambiguitas tinggi, `agen_analis` **WAJIB** mengajukan maksimal 3 pertanyaan klarifikasi singkat secara terstruktur sebelum membuat draf.
3. Jika informasi sudah mencukupi ($\ge 80\%$), langsung masuk ke Step 2.

### Step 2: Penyusunan PRD (Strict Markdown Structure)
Buat atau perbarui file `proyek/<nama_proyek>/docs/01_PRD.md` dengan struktur WAJIB berikut:

```markdown
# Product Requirements Document (PRD) - <Nama Proyek>
**Status**: DRAFT | APPROVED
**Versi**: 1.0

## 1. Executive Summary & Problem Statement
- Problem Statement: (Masalah yang diselesaikan)
- Target Audience: (Persona Pengguna)

## 2. Feature Matrix & Prioritization
| ID Fitur | Nama Fitur | Prioritas (P1-MVP / P2-Penting / P3-Nice to have) | Rationale |
|---|---|---|---|
| FT-001 | ... | P1 | ... |

## 3. Detailed User Stories & Acceptance Criteria
### [FT-001] Nama Fitur
- **User Story**: As a <role>, I want to <action>, so that <benefit>.
- **Acceptance Criteria** (BDD Format):
  - **Scenario 1**: <Judul>
    - **Given**: <Kondisi Awal>
    - **When**: <Aksi Pengguna>
    - **Then**: <Hasil Diharapkan yang>

## 4. Technical & Non-Functional Constraints
- Constraints: (Misal: Skalabilitas, Keamanan, Latensi)
- Out of Scope: (Cakupan yang TIDAK dikerjakan pada fase ini)

```

### Step 3: Handover & Validation Protocol

1. **Validasi Internal (`agen_qa`)**:
* Kirim notifikasi ke `agen_qa` untuk memeriksa kejelasan *Acceptance Criteria* (harus bisa diuji secara logis).


2. **Handover ke Arsitek (`agen_arsitek`)**:
* Setelah `agen_qa` memberikan konfirmasi `PASS`, ubah status di PRD menjadi `Status: APPROVED`.
* Lakukan trigger panggil `agen_arsitek` untuk membuat `02_ERD.md` dan `03_Architecture.md`.



---

## 3. Batasan & Rule Wajib (Negative Constraints)

* **Dilarang**: Menulis kriteria penerimaan yang ambigu seperti "Aplikasi harus cepat" atau "Tampilan harus bagus". Wajib menggunakan kuantitas/kondisi yang jelas (misal: "Response time < 200ms").
* **Dilarang**: Memasukkan detail teknis seperti nama tabel database atau sintaks koding (itu wewenang `agen_arsitek` dan `agen_coder`).
* **Dilarang**: Menyelesaikan tugas tanpa menyertakan minimal 1 skenario *Negative Flow* (edge case/error handling) pada Acceptance Criteria.

---

## 4. Definition of Done (DoD)

Dokumen `01_PRD.md` dinyatakan Selesai (Done) jika:

1. Seluruh bagian template PRD terisi tanpa ada *placeholder* (`TBD` / `N/A`).
2. Setiap fitur P1 memiliki minimal 1 User Story + Acceptance Criteria berformat Given-When-Then.
3. Status dokumen telah disetujui (`APPROVED`).

