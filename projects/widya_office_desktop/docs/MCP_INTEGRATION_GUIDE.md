# Panduan Integrasi AI Agent & Protokol MCP (Model Context Protocol) 🤖

> **Spesifikasi Kolaborasi Agen Cerdas pada WidyaOffice Desktop**  
> *Standar MCP v2024-11-05 (src/ai/mcp.rs) · Zero-Dependency Native JSON-RPC*

---

## 1. Pendahuluan

Jika toolkit pihak ketiga seperti `morluto/rea` menggunakan protokol MCP untuk **menganalisis dan merekayasa balik aplikasi**, maka Widya-Lang membalik kapabilitas ini: **WidyaOffice Desktop secara bawaan menyediakan server MCP (Model Context Protocol) resmi**.

Artinya, asisten AI (seperti Claude Code, Cursor, Codex, ChatGPT, atau model AI berdaulat lokal) dapat mengendalikan dan memanipulasi dokumen perkantoran secara langsung melalui pemanggilan *tool* standar industri tanpa perlu membuka antarmuka grafis atau menulis kode manual.

```
┌───────────────────────────────────────┐
│           AI AGENT WORKSPACE          │
│   (Claude Desktop, Cursor, Agentic AI)│
└──────────────────┬────────────────────┘
                   │ Protokol Standar MCP v2024-11-05 (Stdio / SSE)
┌──────────────────▼────────────────────┐
│      WIDYA MCP NATIVE SERVER          │
│         (src/ai/mcp.rs)               │
├───────────────────────────────────────┤
│ Daftar Tools:                         │
│ - office_create_document              │
│ - office_insert_math_equation         │
│ - office_execute_mail_merge           │
│ - office_sign_document_pqc            │
│ - office_export_pdf                   │
└──────────────────┬────────────────────┘
                   │ Direct Rust In-Memory Call
┌──────────────────▼────────────────────┐
│         CORE WIDYAOFFICE ENGINE       │
│    (src/office/docx.rs, pdf.rs, dll)  │
└───────────────────────────────────────┘
```

---

## 2. Konfigurasi Client MCP (Claude Desktop / Cursor)

Untuk menghubungkan agen AI favorit Anda ke WidyaOffice Desktop, tambahkan konfigurasi berikut pada berkas konfigurasi client (misalnya `claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "widya-office": {
      "command": "/usr/local/bin/widya",
      "args": ["mcp", "serve"],
      "env": {
        "WIDYA_OFFICE_WORKSPACE": "/home/devops/dokumen"
      }
    }
  }
}
```

---

## 3. Katalog MCP Tools yang Disediakan

WidyaOffice mengekspos 7 alat cerdas yang siap dipanggil agen AI:

### 1. `office_create_document`
Membuat dokumen baru dari template atau dokumen kosong.
- **Input Schema:**
  ```json
  {
    "type": "object",
    "properties": {
      "format": { "type": "string", "enum": ["docx", "xlsx", "pptx"] },
      "title": { "type": "string" },
      "template": { "type": "string", "enum": ["kosong", "sk_pemerintah", "kontrak_kerja", "sertifikat"] }
    },
    "required": ["format", "title"]
  }
  ```

### 2. `office_insert_math_equation`
Menyisipkan rumus matematika ke dokumen aktif dan otomatis mengonversinya menjadi OpenXML OMML `<m:oMath>`.
- **Input Schema:**
  ```json
  {
    "type": "object",
    "properties": {
      "doc_id": { "type": "string" },
      "latex": { "type": "string", "description": "Formula LaTeX, contoh: \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}" },
      "is_block": { "type": "boolean", "default": true }
    },
    "required": ["doc_id", "latex"]
  }
  ```

### 3. `office_execute_mail_merge`
Memproses pembuatan dokumen massal berdasarkan dataset.
- **Input Schema:**
  ```json
  {
    "type": "object",
    "properties": {
      "template_path": { "type": "string" },
      "dataset_json": { "type": "string", "description": "Array JSON rekaman data" },
      "output_dir": { "type": "string" },
      "format": { "type": "string", "enum": ["docx", "pdf"] }
    },
    "required": ["template_path", "dataset_json", "output_dir"]
  }
  ```

### 4. `office_sign_document_pqc`
Menyematkan tanda tangan kriptografi pasca-kuantum berstandar NIST FIPS 204 (ML-DSA-87).
- **Input Schema:**
  ```json
  {
    "type": "object",
    "properties": {
      "doc_path": { "type": "string" },
      "signer_name": { "type": "string" },
      "institution": { "type": "string" },
      "hsm_pin": { "type": "string" }
    },
    "required": ["doc_path", "signer_name"]
  }
  ```

### 5. `office_export_pdf`
Mengubah dokumen DOCX/XLSX/PPTX menjadi PDF vektor siap cetak.
- **Input Schema:**
  ```json
  {
    "type": "object",
    "properties": {
      "input_path": { "type": "string" },
      "output_path": { "type": "string" },
      "watermark": { "type": "string" }
    },
    "required": ["input_path", "output_path"]
  }
  ```

---

## 4. MCP Resources & Prompts

Selain *Tools*, WidyaOffice juga menyediakan *Resources* dan *Prompts* bawaan:

### Resources (Sumber Data Live)
- `office://documents/active`: Teks semantik dan pohon elemen dokumen yang sedang terbuka di layar.
- `office://templates/catalog`: Daftar template resmi perusahaan/pemerintahan yang tersedia.

### Prompts (Instruksi Terpandu)
- `draft_legal_contract`: Panduan sistem untuk menyusun pasal-pasal perjanjian hukum Indonesia berstandar tinggi lengkap dengan klausul kerahasiaan dan arbitrase.
- `audit_document_security`: Memeriksa apakah suatu berkas DOCX memiliki tanda tangan digital yang sah atau pernah dimodifikasi tanpa izin.

---

## 5. Contoh Percakapan Pengguna & Agen AI

> **Pengguna:** *"Tolong buatkan 500 sertifikat pelatihan AI untuk peserta di berkas `peserta.csv`. Gunakan template resmi BSSN, sertakan tanda tangan digital PQC saya, dan ekspor langsung dalam bentuk PDF ke folder `output_sertifikat/`."*
>
> **Agen AI:**
> 1. Memanggil `office_create_document(format: "docx", template: "sertifikat")`.
> 2. Memanggil `office_execute_mail_merge(template_path, dataset_json, output_dir, format: "docx")`.
> 3. Memanggil `office_sign_document_pqc` untuk setiap berkas menggunakan kunci ML-DSA-87.
> 4. Memanggil `office_export_pdf`.
>
> **Hasil:** 500 sertifikat PDF selesai diproses dan ditandatangani dalam waktu **< 1,2 detik** secara otonom!
