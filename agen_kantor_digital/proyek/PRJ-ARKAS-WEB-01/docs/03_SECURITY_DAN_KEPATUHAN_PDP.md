# KEBIJAKAN KEAMANAN INFORMASI & KEPATUHAN UU PDP NO. 27/2022
## APLIKASI RENCANA KEGIATAN DAN ANGGARAN SEKOLAH (ARKAS) WEB TERINTEGRASI
**Kode Proyek:** `PRJ-ARKAS-WEB-01` | **Klasifikasi:** Dokumen Kepatuhan Hukum & Keamanan Siber

---

### 1. Kepatuhan Undang-Undang Perlindungan Data Pribadi (UU PDP No. 27/2022)
Sistem ARKAS Web memproses berbagai data spesifik dan sensitif milik pendidik, tenaga kependidikan, serta rekanan sekolah, antara lain NIK Guru, NPWP Rekanan, Nomor Rekening Bank Sekolah, serta nominal honorarium. Untuk memenuhi UU PDP No. 27/2022, diterapkan standar berikut:
1. **Penyamaran Data Otomatis (*Data Masking*)**:
   - NIK Guru pada laporan publik disamarkan: `330215******0002`.
   - Nomor Rekening Bank pada tampilan layar: `140-**-*****-2`.
   - NPWP Rekanan: `00.***.***.*-521.000`.
2. **Kriptografi Penyimpanan Lokal (*Encryption at Rest*)**:
   - Seluruh berkas penyimpanan basis data `database/data.wdb` dan catatan audit dienkripsi menggunakan ChaCha20-Poly1305 / AES-GCM 256-bit standar industri.
3. **Penyimpanan On-Premise Berdaulat (*Zero Telemetry*)**:
   - Data keuangan sekolah tidak dialirkan ke cloud komersial pihak ketiga atau server asing. Semua data tetap berada di bawah kedaulatan server sekolah / dinas pendidikan setempat.

---

### 2. Rantai Audit Kriptografis Anti-Tampering (BPK / Inspektorat)
Setiap transaksi Buku Kas Umum (BKU) dilengkapi dengan *cryptographic signature hash chain*:
- Algoritma: **SHA-256** (Secure Hash Algorithm 256-bit).
- Formula: `Hash(N) = SHA256(Hash(N-1) || NoUrut || Tanggal || NoBukti || Terima || Keluar || Saldo)`.
- Keunggulan: Apabila terjadi perubahan nilai pada transaksi masa lalu (misal Rp 15.000.000 diubah menjadi Rp 5.000.000), seluruh hash transaksi berikutnya akan menjadi tidak valid (*chain broken*), sehingga manipulasi dapat dideteksi seketika oleh auditor BPK/Inspektorat.

---

### 3. Kontrol Akses Berbasis Peran (*Role-Based Access Control / RBAC*)
| Peran (*Role*) | Hak Akses RKAS | Hak Akses BKU | Hak Tutup Buku | Akses Pajak |
|:--------------|:--------------|:--------------|:---------------|:------------|
| **Kepala Sekolah** | Baca, Usul, Sahkan | Baca, Verifikasi | Setujui Tutup Buku | Baca Laporan |
| **Bendahara BOSP** | Baca, Rancang | Baca, Tulis, Hapus Draft | Ajukan Tutup Buku | Pungut & Setor |
| **Operator Komite** | Baca Ringkasan | Baca Transaksi Umum | Tidak Ada | Tidak Ada |
| **Auditor Pengawas** | Baca Lengkap | Verifikasi Rantai Hash | Audit Log | Rekonsiliasi NTPN |

---

### 4. Pencegahan Serangan Siber Aplikasi Web
- **Proteksi XSS (Cross-Site Scripting)**: Sanitasi otomatis entri teks uraian belanja menggunakan encoder HTML entitas WidyaWeb.
- **Proteksi CSRF**: Penggunaan token kriptografi acak satu kali pakai (*one-time nonce*) pada setiap mutasi anggaran.
- **SQL Injection Prevention**: Penggunaan prepared statement dan parser parser internal WidyaBase.
