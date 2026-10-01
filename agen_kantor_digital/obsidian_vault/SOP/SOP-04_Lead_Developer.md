# SOP-04: Lead Software Developer (Lead Coder)

**Kode Dokumen**: `SOP-WIDYA-04`  
**Peran Agen**: `agen_coder`  
**Gelar**: Kepala Rekayasa Kode, Frontend & Backend Widya-Lang  
**Target File Output**: 
- `proyek/<nama_proyek>/server.wya`
- `proyek/<nama_proyek>/web/index.html`
- `proyek/<nama_proyek>/web/style.css`
- `proyek/<nama_proyek>/web/app.js`

---

## 1. Trigger & Prerequisites (Syarat Jalan)
- **Trigger**: Menerima notifikasi status `APPROVED` pada file `01_PRD.md` dan `02_ARSITEKTUR_DAN_ERD.md` dari `agen_arsitek`.
- **Prasyarat Wajib**: 
  1. `proyek/<nama_proyek>/docs/01_PRD.md` tersedia.
  2. `proyek/<nama_proyek>/docs/02_ARSITEKTUR_DAN_ERD.md` tersedia.
  3. `proyek/<nama_proyek>/database/skema.sql` tersedia.
  *Jika salah satu belum ada, TOLAK tugas dan kembalikan ke orchestrator.*

---

## 2. Alur Kerja Detil (Step-by-Step Execution)

### Step 1: Eksekusi Inisialisasi Database
1. Baca file `proyek/<nama_proyek>/database/skema.sql`.
2. Pastikan DDL siap dieksekusi oleh runtime WidyaBase.

### Step 2: Pengembangan Server Backend (`server.wya`)
1. Buat skrip server `server.wya` berbasis Widya-Lang.
2. Sediakan REST API Endpoint yang memproses seluruh operasi CRUD sesuai kebutuhan PRD.
3. Hubungkan logika API langsung ke tabel-tabel di WidyaBase (menggunakan skema dari `skema.sql`).
4. Set port server sesuai alokasi di `02_ARSITEKTUR_DAN_ERD.md`.

### Step 3: Pengembangan Frontend UI/UX (`web/`)
1. **`index.html`**:
   - Susun struktur HTML5 semantik dan responsif.
   - Sertakan elemen form, tabel data, modal, dan notifikasi yang dibutuhkan oleh kriteria penerimaan PRD.
2. **`style.css`**:
   - Terapkan Vanilla CSS dengan tema modern (Glassmorphism / Vibrant).
   - Gunakan CSS Variables untuk warna, font, dan spacing agar konsisten.
   - Wajib menyertakan penanganan responsif (*mobile & desktop view*).
3. **`app.js`**:
   - Tulis logika interaktif menggunakan Vanilla JavaScript (ES6+).
   - Hubungkan Form Event & Fetch API ke endpoint `server.wya`.
   - Implementasikan *Form Validation* di sisi client dan *State Management* sederhana untuk UI feedback (loading, success, error toast).

### Step 4: Self-Verification & Handover ke `agen_qa`
Lakukan pengecekan mandiri terhadap checklist berikut sebelum menyerahkan hasil:
- [ ] Apakah semua file (`server.wya`, `index.html`, `style.css`, `app.js`) telah terbentuk penuh di folder proyek?
- [ ] Apakah ada variabel/fungsi yang lupa diimpor atau belum terdefinisi?
- [ ] Apakah seluruh fitur P1 pada PRD sudah diimplementasikan logikanya?
- Jika seluruh checklist terpenuhi, kirim notifikasi Handover ke `agen_qa`.

---

## 3. Batasan & Rule Wajib (Negative Constraints)
- **Dilarang**: Menggunakan placeholder, komentar pengganti kode, atau stub/mock data. (Contoh terlarang: `// TODO: Implement API call`, `alert("Success")`, data dummy jika API sudah tersedia).
- **Dilarang**: Menggunakan runtime eksternal selain Widya-Lang (Dilarang menggunakan Node.js/Express, Python/Flask, PHP, dsb.).
- **Dilarang**: Menggunakan framework CSS/JS berat berbasis npm/build tool (seperti React, Vue, Tailwind via Node CLI) kecuali CDN Vanilla murni yang diizinkan.
- **Dilarang**: Mengubah skema database atau atribut tabel tanpa melakukan konfirmasi/update bersama `agen_arsitek`.

---

## 4. Definition of Done (DoD)
Aplikasi dinyatakan Selesai Tahap Koding jika:
1. Seluruh file aplikasi berada lengkap dalam isolasi folder `proyek/<nama_proyek>/`.
2. Aplikasi dapat dijalankan melalui runtime resmi Widya (`widya web dev web -p <port>`) tanpa error saat startup.
3. Seluruh alur *Positive Flow* dan *Negative Flow* pada *Acceptance Criteria* PRD dapat dieksekusi secara fungsional.
4. Tampilan antarmuka (UI) rapi, interaktif, dan konsisten.