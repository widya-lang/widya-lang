# SPESIFIKASI KEAMANAN DATA MEDIS & UU PDP
## PROYEK: WIDYAMEDIKA SIMRS & SATUSEHAT FHIR
**Disusun Oleh:** `agen_keamanan` (SOP-07)  

---

### 1. Perlindungan Data Medis Sensitif (UU PDP No. 27/2022)
1. **Data Kategori Khusus**: Data kesehatan diklasifikasikan sebagai Data Pribadi Spesifik yang memerlukan enkripsi tingkat tinggi (**AES-256-GCM**).
2. **Role-Based Access Control (RBAC)**: Hanya dokter yang menangani yang memiliki hak akses penuh membaca riwayat anamnesis lengkap pasien.
3. **Audit Trail**: Setiap aktivitas pembacaan dan pengubahan rekam medis dicatat dengan timestamp mikrodetik dan ID dokter pemeriksa.
