# SPESIFIKASI KEAMANAN & AUDIT KEPATUHAN UU PDP
## PROYEK: SISTEM KOPERASI DESA MERAH PUTIH (KDMP)
**Disusun Oleh:** `agen_keamanan` (SOP-07: Audit OWASP Top 10, UU PDP No. 27/2022, Enkripsi Kripto)  
**Status Audit:** LEVEL A — ZERO RAW DATA LEAKAGE  

---

### 1. Kepatuhan Undang-Undang Perlindungan Data Pribadi (UU PDP No. 27/2022)
1. **Masking NIK & Rekening Anggota**: Pada seluruh log publik, notulensi rapat, dan tampilan struk kasir, NIK disamarkan menjadi `330215******0002` dan nomor rekening `0001****01948`.
2. **Prinsip Minimalisasi Data**: Data yang disimpan di tabel basis data hanya mencakup parameter yang relevan dengan operasional koperasi.
3. **Persetujuan Pemrosesan Data (Consent)**: Anggota koperasi menandatangani klausul persetujuan digital saat mendaftar menjadi anggota.

---

### 2. Standar Kriptografi & Proteksi Kredensial
- **Otentikasi Pengurus**: Menggunakan algoritma hash **Argon2id** ($m=65536, t=3, p=4$) dengan garam acak 128-bit untuk mencegah serangan GPU/ASIC rainbow table.
- **Enkripsi Data At-Rest**: Berkas basis data `koperasi.wdb` dan snapshot WAL dienkripsi dengan standar **AES-256-GCM**.
- **Integritas Audit**: Setiap perubahan saldo simpan pinjam dan mutasi kasir diverifikasi dengan jejak tanda tangan **HMAC-SHA256**.

---

### 3. Matriks Pencegahan OWASP Top 10

| Vektor Serangan | Mekanisme Pencegahan di KDMP |
| :--- | :--- |
| **SQL Injection** | Parameterized queries native pada mesin WidyaBase B+Tree engine. |
| **Cross-Site Scripting (XSS)** | Sanitasi input otomatis & escaping konten HTML pada seluruh render DOM. |
| **Broken Authentication** | Cookie session berbendera `HttpOnly`, `SameSite=Strict`, dan `Secure`. |
| **Mass Assignment** | Validasi skema payload JSON sebelum dieksekusi oleh service layer. |
