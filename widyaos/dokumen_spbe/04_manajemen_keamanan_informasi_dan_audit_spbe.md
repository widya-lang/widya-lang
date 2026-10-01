# MANAJEMEN KEAMANAN INFORMASI DAN AUDIT SPBE
## PEDOMAN TATA KELOLA KEAMANAN SIBER, KELAIKAN BSSN, DAN KEPATUHAN PERLINDUNGAN DATA PRIBADI (UU NO. 27/2022)
### Standar Keamanan Berlapis (*Defense-in-Depth*), Penanganan Insiden CSIRT, dan Kesiapsiagaan Bencana Ekosistem WidyaOS & Sewidya (`sewidya.id`)

---

**Penyusun:** Tim Keamanan Siber dan Audit Kelaikan Sistem SPBE  
**Dasar Regulasi:** Perpres No. 95/2018 (Pasal 41-45 tentang Keamanan SPBE), UU No. 27/2022 (UU PDP), Standar BSSN No. 4/2021 (KCSN), ISO/IEC 27001:2022  
**Klasifikasi Dokumen:** Pedoman Keamanan Informasi & Audit Kelaikan  
**Tahun:** 2026  

---

## DAFTAR ISI

1. [BAGIAN I: PRINSIP DAN KERANGKA KERJA KEAMANAN SPBE](#bagian-i-prinsip-dan-kerangka-kerja-keamanan-spbe)
2. [BAGIAN II: ARSITEKTUR PERTAHANAN BERLAPIS (*DEFENSE-IN-DEPTH*)](#bagian-ii-arsitektur-pertahanan-berlapis)
   - 2.1. Lapis 1: Perangkat Keras, Silikon, dan Root-of-Trust (TPM/HSM)
   - 2.2. Lapis 2: Sistem Operasi, Kernel, dan Isolasi Proses (Sandbox)
   - 2.3. Lapis 3: Basis Data dan Perlindungan Data Pribadi (UU PDP)
   - 2.4. Lapis 4: Jaringan, Peramban, dan Jalan Tol Data (WidyaWAN)
   - 2.5. Lapis 5: Kriptografi Pascakuantum (*Post-Quantum Cryptography*)
3. [BAGIAN III: STRUKTUR DAN TATA KELOLA CSIRT-WIDYAOS](#bagian-iii-struktur-dan-tata-kelola-csirt-widyaos)
4. [BAGIAN IV: RENCANA KEBERLANGSUNGAN BISNIS & PEMULIHAN BENCANA (BCP/DRP)](#bagian-iv-rencana-keberlangsungan-bisnis--pemulihan-bencana)
5. [BAGIAN V: MATRIKS KEPATUHAN UU PERLINDUNGAN DATA PRIBADI (UU NO. 27/2022)](#bagian-v-matriks-kepatuhan-uu-perlindungan-data-pribadi)

---

## BAGIAN I: PRINSIP DAN KERANGKA KERJA KEAMANAN SPBE

Keamanan Sistem Pemerintahan Berbasis Elektronik (SPBE) pada ekosistem WidyaOS dan Sewidya dibangun berdasarkan prinsip **Kerahasiaan (*Confidentiality*)**, **Keutuhan (*Integrity*)**, **Ketersediaan (*Availability*)**, **Keaslian (*Authenticity*)**, dan **Kenirsangkalan (*Non-Repudiation*)**.

Kerangka kerja mengadopsi integrasi **NIST Cybersecurity Framework (CSF 2.0)** dan **Kategori Kematangan Keamanan Siber Nasional (KCSN Level 4 - BSSN)**:
1. **Identify (Identifikasi)**: Inventarisasi aset TIK sekolah, pemetaan klasifikasi data (Publik, Terbatas, Rahasia, Sangat Rahasia), dan penilaian risiko.
2. **Protect (Proteksi)**: Penerapan *Zero-Trust Architecture*, enkripsi end-to-end, isolasi hak akses (RBAC), dan *Multi-Factor Authentication* (MFA).
3. **Detect (Deteksi)**: Pemantauan anomali jaringan, *File Integrity Monitoring (FIM)* BKU ARKAS, dan deteksi malware heuristik kernel (WidyaSec EDR).
4. **Respond (Respons)**: Prosedur isolasi otomatis terhadap simpul (*node*) yang terinfeksi dalam waktu < 10 detik.
5. **Recover (Pemulihan)**: Pemulihan citra emas (*Golden Recovery*) dan replikasi WAL multi-region dalam hitungan menit.

---

## BAGIAN II: ARSITEKTUR PERTAHANAN BERLAPIS (*DEFENSE-IN-DEPTH*)

```
+-----------------------------------------------------------------------------------------------+
|                                    DEFENSE-IN-DEPTH WIDYAOS                                   |
+-----------------------------------------------------------------------------------------------+
|  LAPIS 5: KRIPTOGRAFI PASCAKUANTUM (ML-KEM Kyber-1024 | Shamir's Secret Sharing 3-of-5)       |
+-----------------------------------------------------------------------------------------------+
|  LAPIS 4: JARINGAN BERDAULAT (WidyaWAN 10.38.0.0/16 | Anti-DDoS 50k RPS | DNS TrustPositif)   |
+-----------------------------------------------------------------------------------------------+
|  LAPIS 3: BASIS DATA & PRIVASI (Dynamic Data Masking UU PDP | ARKAS BKU Immutable Shield)     |
+-----------------------------------------------------------------------------------------------+
|  LAPIS 2: KERNEL & OS (Sandboxing | Zero-VBA Macro Shield | WidyaSec EDR Heuristik)          |
+-----------------------------------------------------------------------------------------------+
|  LAPIS 1: PERANGKAT KERAS (TPM 2.0 PCR Secure Boot | RISC-V Open ISA | TRNG Entropi Fisik)    |
+-----------------------------------------------------------------------------------------------+
```

### 2.1. Lapis 1: Perangkat Keras, Silikon, dan Root-of-Trust (TPM/HSM)
- **Validasi Secure Boot PCR**: Mengukur integritas firmware (PCR 0), bootloader (PCR 4), dan kernel WidyaOS (PCR 7). Booting diblokir seketika bila terjadi injeksi modifikasi biner.
- **Entropi TRNG Kuantum**: Pembangkitan bilangan acak murni dari derau termal perangkat keras untuk pengacakan kunci sesi kriptografi.
- **Modul Penggerak**: [kedaulatan_perangkat_keras_fisik.wya](file:///c:/widya-lang/widyaos/modul/kedaulatan_perangkat_keras_fisik.wya).

### 2.2. Lapis 2: Sistem Operasi, Kernel, dan Isolasi Proses
- **Isolasi Proses Sandboxing**: Menjalankan setiap aplikasi pihak ketiga pada ruang memori terisolasi dengan pembatasan hak akses berbasis kapabilitas (*capability-based security*).
- **Zero-VBA Macro Shield**: Mencegah eksekusi makro Visual Basic (vektor ransomware nomor 1 dunia) dan menggantikannya dengan otomasi Widya-Lang ber-segel digital W3C DID.
- **WidyaSec EDR**: Deteksi heuristik di level kernel yang memblokir serangan Ransomware LockBit dan ancaman zero-day secara otomatis.
- **Modul Penggerak**: [widyamacro_automation_engine.wya](file:///c:/widya-lang/widyaos/modul/widyamacro_automation_engine.wya), [widyasec_guard.wya](file:///c:/widya-lang/widyaos/modul/widyasec_guard.wya).

### 2.3. Lapis 3: Basis Data dan Perlindungan Data Pribadi
- **ARKAS BKU Immutable Shield**: Larangan mutlak perintah SQL `DELETE/DROP/TRUNCATE` pada tabel kas umum sekolah. Setiap pembatalan transaksi wajib menggunakan pencatatan jurnal pembalik (*storno entry*) demi transparansi audit BPK RI.
- **Dynamic Data Masking UU PDP**: Penyembunyian otomatis Nomor Induk Kependudukan (NIK), nomor telepon, dan data finansial bagi pengguna non-otoritas (`3201************`).
- **AST SQL Injection Guard**: Validasi pohon sintaks kueri sebelum dieksekusi oleh mesin basis data.
- **Modul Penggerak**: [arkas_bku_immutable_shield.wya](file:///c:/widya-lang/widyaos/modul/arkas_bku_immutable_shield.wya), [widyabase_engine.wya](file:///c:/widya-lang/widyaos/modul/widyabase_engine.wya).

### 2.4. Lapis 4: Jaringan, Peramban, dan Jalan Tol Data
- **WidyaWAN Zero-Trust Intranet**: Jalur komunikasi antar-sekolah dan kementerian menggunakan subnet privat 10.38.0.0/16 dengan enkripsi mTLS dan port publik tertutup (*stealth mode*).
- **Mitigasi Serangan DDoS Lonjakan PPDB**: Pembatasan laju dinamis (*Dynamic Rate Limiting*) berkapasitas 50.000 permintaan/detik dengan circuit breaker otomatis.
- **WidyaBrowser Enterprise Shield**: Penegakan Content Security Policy (CSP Level 3), pemblokiran pelacak komersial, dan penapisan situs judi online/pornografi melalui WidyaNet DNS.
- **Modul Penggerak**: [widyawan_secure_highway.wya](file:///c:/widya-lang/widyaos/modul/widyawan_secure_highway.wya), [widyanet_enterprise_shield.wya](file:///c:/widya-lang/widyaos/modul/widyanet_enterprise_shield.wya), [widyabrowser_security_enterprise.wya](file:///c:/widya-lang/widyaos/modul/widyabrowser_security_enterprise.wya).

### 2.5. Lapis 5: Kriptografi Pascakuantum (*Post-Quantum Cryptography*)
- **ML-KEM / Crystals-Kyber-1024**: Implementasi enkripsi kunci berbasis kisi (*lattice-based cryptography*) standar NIST Level 5 untuk menjamin kerahasiaan data rahasia negara dari ancaman pemecahan sandi komputer kuantum di masa mendatang.
- **Shamir's Secret Sharing (3-of-5)**: Pembagian kunci pemulihan darurat sistem kementerian kepada 5 pejabat kunci (Mendikdasmen, Kepala BSSN, Kepala Pusdatin, Dirjen PAUD Dikdasmen, Inspektur Jenderal), di mana dekripsi hanya dapat dilakukan apabila minimal 3 pejabat hadir secara sah.
- **Modul Penggerak**: [widyaquantum_pqc.wya](file:///c:/widya-lang/widyaos/modul/widyaquantum_pqc.wya), [widyarescue_self_healing.wya](file:///c:/widya-lang/widyaos/modul/widyarescue_self_healing.wya).

---

## BAGIAN III: STRUKTUR DAN TATA KELOLA CSIRT-WIDYAOS

Tim Tanggap Insiden Keamanan Siber (**CSIRT-WidyaOS**) dibentuk sebagai garda terdepan koordinasi keamanan siber pendidikan nasional, terhubung langsung dengan Pusat Operasi Keamanan Siber Nasional (Pusopskamsinas) BSSN:

```
+-----------------------------------------------------------------------------------------------+
|                                      STRUKTUR CSIRT-WIDYAOS                                   |
+-----------------------------------------------------------------------------------------------+
|                                  PENGARAH: KEPALA BSSN & MENDIKDASMEN                         |
|                                                    |                                          |
|                                    KETUA CSIRT: KEPALA PUSDATIN                               |
|                                                    |                                          |
|              +-------------------------------------+----------------------------------+       |
|              |                                     |                                  |       |
|     SUB-TIM TRIAGE & DETEKSI              SUB-TIM FORENSIK DIGITAL             SUB-TIM MITIGASI|
|  - Monitoring Sensor SOC 24/7             - Reverse Engineering Malware        - Patching & OTA|
|  - Analisis Log WidyaWAN                  - Analisis Bukti Digital             - Pemulihan RPO |
+-----------------------------------------------------------------------------------------------+
```

### Prosedur Penanganan Insiden Keamanan Siber (SOP CSIRT):
1. **Notifikasi Awal (< 5 Menit)**: Sistem telemetri otomatis membunyikan alarm pada Command Center jika terdeteksi anomali integritas data atau lonjakan serangan.
2. **Isolasi Simpul (< 15 Menit)**: Sub-Tim Mitigasi memutus koneksi simpul terdampak dari jaringan WidyaWAN untuk mencegah penyebaran lateral.
3. **Pelaporan ke BSSN (< 1x24 Jam)**: Pengiriman laporan insiden resmi kepada BSSN sesuai amanat Perpres SPBE dan regulasi keamanan informasi nasional.
4. **Investigasi Forensik (< 72 Jam)**: Penerbitan laporan analisis akar masalah (*Root Cause Analysis - RCA*) dan rekomendasi pencegahan permanen.

---

## BAGIAN IV: RENCANA KEBERLANGSUNGAN BISNIS & PEMULIHAN BENCANA (BCP/DRP)

Untuk menjamin layanan publik pendidikan tidak pernah berhenti beroperasi (*zero downtime*), diterapkan strategi redundansi multi-region **Segitiga Emas Nusantara**:

```
                              [PUSAT DATA NASIONAL 1: JAKARTA]
                                        /          \
                                       /            \
                                      /              \
           [PDN 2: BATAM] ----------------------------- [PDN 3: IKN NUSANTARA]
```

- **Metode Sinkronisasi**: Replikasi transaksional real-time antar tiga wilayah geografis terpisah.
- **RPO (*Recovery Point Objective*)**: **0 Detik** (Tidak ada data transaksi keuangan atau Dapodik yang hilang).
- **RTO (*Recovery Time Objective*)**: **< 3 Menit** (Pengalihan beban trafik otomatis ke pusat data cadangan jika salah satu region mengalami gempa bumi, banjir, atau kegagalan daya).
- **Penyimpanan Abadi WORM**: Penyimpanan arsip ijazah dan laporan keuangan menggunakan format *Write Once, Read Many* yang kebal terhadap modifikasi dan ransomware.

---

## BAGIAN V: MATRIKS KEPATUHAN UU PERLINDUNGAN DATA PRIBADI (UU NO. 27/2022)

| Pasal UU PDP | Ketentuan Regulasi | Implementasi Teknis pada WidyaOS & Sewidya | Status Pemenuhan |
|---|---|---|---|
| **Pasal 20 - 24** | Pemrosesan Data Pribadi secara sah, terbatas, dan transparan | Persetujuan eksplisit orang tua melalui Portal Wali Murid saat registrasi siswa baru. | **✅ MEMENUHI LENGKAP** |
| **Pasal 35** | Kewajiban Pengamanan Data Pribadi Tingkat Tinggi | Enkripsi end-to-end ChaCha20-Poly1305 dan isolasi penyimpanan berbasis W3C DID. | **✅ MEMENUHI LENGKAP** |
| **Pasal 40** | Pelindungan Data Pribadi Anak di Bawah Umur | Anonimitas telemetri IFP (Differential Privacy Laplace noise) & isolasi data anak tanpa pelacak iklan. | **✅ MEMENUHI LENGKAP** |
| **Pasal 46** | Pemberitahuan Kegagalan Perlindungan Data (< 72 Jam) | Modul audit otomatis yang memicu notifikasi seketika kepada subjek data dan Lembaga PDP. | **✅ MEMENUHI LENGKAP** |
| **Pasal 53** | Penunjukan Pejabat Pelindung Data Pribadi (*DPO*) | Pembentukan unit DPO internal di Pusdatin dan Dinas Pendidikan terhubung dengan BSSN. | **✅ MEMENUHI LENGKAP** |

---
*Ditetapkan sebagai Dokumen Standar Manajemen Keamanan Informasi dan Audit Kelaikan SPBE Nasional.*
