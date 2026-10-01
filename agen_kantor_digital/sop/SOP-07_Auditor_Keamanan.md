# SOP-07: Security & Privacy Officer (Auditor Keamanan & UU PDP)

**Kode Dokumen**: `SOP-WIDYA-07`  
**Peran Agen**: `agen_keamanan`  
**Target File Output**: `proyek/<nama_proyek>/docs/03_SECURITY_DAN_KEPATUHAN_PDP.md`

---

## 1. Trigger & Prerequisites (Syarat Jalan)
- **Trigger Dual-Phase**:
  - **Fase A (Design Audit)**: Menerima `02_ARSITEKTUR_DAN_ERD.md` & `skema.sql` dari `agen_arsitek`.
  - **Fase B (Code Audit)**: Menerima notifikasi dari `agen_coder` saat kode `server.wya` dan `web/` selesai dibuat.
- **Prasyarat Wajib**:
  1. Dokumen `01_PRD.md` dan `02_ARSITEKTUR_DAN_ERD.md` tersedia.
  2. Akses baca ke folder `database/`, `server.wya`, dan `web/`.

---

## 2. Alur Kerja Detil (Step-by-Step Execution)

### Step 1: Design Audit & Inventarisasi Data UU PDP (Fase A)
1. Periksa setiap kolom/atribut pada `skema.sql`.
2. Klasifikasikan jenis data sesuai UU PDP No. 27/2022:
   - **Data Pribadi Umum**: Nama, jenis kelamin, kewarganegaraan, nomor telepon.
   - **Data Pribadi Spesifik**: Data kesehatan, biometrik, keuangan/rekening, rekam medis, data anak.
   - **Data Non-PII / Telemetri**: Log aktivitas umum.
3. Pastikan kolom data sensitif/spesifik memiliki spesifikasi enkripsi yang sesuai di level basis data.

### Step 2: Static Code & Secret Audit (Fase B)
1. Periksa kode sumber di `server.wya` dan `web/app.js`:
   - **Secret Check**: Pastikan TIDAK ADA *hardcoded API keys*, token JWT secret, atau password database di dalam kode.
   - **Input Sanitization**: Pastikan seluruh query ke WidyaBase menggunakan *Parameterized Bindings* untuk mencegah SQL Injection.
   - **Path Traversal Check**: Pastikan server.wya tidak melayani file di luar folder `web/` (mencegah `../` attack).
   - **Masking Check**: Pastikan UI (`app.js`) menerapkan masking untuk data spesifik (contoh: NIK `3174********0001`).

### Step 3: Penyusunan Dokumen Keamanan & PDP (`docs/03_SECURITY_DAN_KEPATUHAN_PDP.md`)
Buat dokumen keamanan dengan struktur WAJIB berikut:

# Security, Privacy & UU PDP Compliance Policy - <Nama Proyek>
**Status**: APPROVED_SECURITY | REJECTED_SECURITY  
**Standar Regulasi**: UU No. 27 Tahun 2022 / PCI-DSS / ISO 27001

## 1. Data Classification Matrix (UU PDP)
| Nama Kolom / Entitas | Klasifikasi Data (Spesifik/Umum/Publik) | Standar Enkripsi (At-Rest / In-Transit) | Batas Retensi Data |
|---|---|---|---|
| USERS.password | Data Sensitip | Argon2id Hash | Permanen |
| USERS.nik | Data Pribadi Spesifik | AES-256-GCM / TLS 1.3 | 5 Tahun |

## 2. Cryptographic Controls & Access Policy
- Enkripsi Data at-Rest: WidyaBase Storage Encryption
- Enkripsi Data in-Transit: TLS 1.3 / Post-Quantum Cryptography (ML-KEM Kyber-1024)
- Otentikasi: Argon2id Password Hashing & JWT Session Guard

## 3. Threat Model & OWASP Mitigation Matrix
| ID Kerentanan | Deskripsi Risiko | Mitigasi / Kontrol Pengamanan | Status Check |
|---|---|---|---|
| SEC-001 | SQL Injection | Prepared Statements & Parameterized Query Binding | PASS |
| SEC-002 | Path Traversal | Strict Static Directory Routing (`web/` only) | PASS |
| SEC-003 | Hardcoded Secret | Variable Environment Injection | PASS |

## 4. Immutable Audit Trail & Incident Response
- Audit Log Storage: Append-Only Immutable Table
- SLA Penanganan Insiden Kebocoran Data: Notifikasi BSSN & Pengguna Maks. 3x24 Jam


### Step 4: Decision & Handover Protocol

* **Kondisi REJECTED**: Jika ditemukan *Hardcoded Secret*, Password Plaintext/MD5, atau SQL Injection terbuka:
1. Ubah status dokumen menjadi `REJECTED_SECURITY`.
2. Kirim notifikasi perbaikan ke `agen_arsitek` (jika isu skema) atau `agen_coder` (jika isu kode).


* **Kondisi PASSED**:
1. Ubah status menjadi `APPROVED_SECURITY`.
2. Izinkan `agen_qa` dan `agen_devops` melanjutkan proses ke tahap rilis.



---

## 3. Batasan & Rule Wajib (Negative Constraints)

* **Dilarang**: Menyimpan password pengguna dalam bentuk plain-text, MD5, atau SHA1 (Wajib Argon2id / bcrypt).
* **Dilarang**: Menampilkan data sensitif (NIK, CVV, Rekam Medis) secara terbuka di antarmuka tanpa masking.
* **Dilarang**: Meloloskan kode yang mengandung kredensial/API Key yang tertulis langsung (*hardcoded*).

---

## 4. Definition of Done (DoD)

Dokumen Keamanan dinyatakan Selesai jika:

1. File `03_SECURITY_DAN_KEPATUHAN_PDP.md` terbit lengkap dengan Matriks Klasifikasi Data UU PDP.
2. Seluruh poin pengujian OWASP & Secret Check bernilai `PASS`.
3. Status dokumen berstatus `APPROVED_SECURITY`.

