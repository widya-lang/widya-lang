# 🛡️ Kebijakan Keamanan (Security Policy) — Widya-Lang

Tim Pengembang Widya-Lang berkomitmen untuk menjaga keamanan, keandalan, dan privasi ekosistem **Widya-Lang**, termasuk *Compiler/Runtime*, **WidyaWeb Framework**, dan **WidyaDesktop Framework**.

Dokumen ini menjelaskan versi yang didukung, cara melaporkan kerentanan secara etis dan bertanggung jawab (*Responsible Disclosure*), serta proses mitigasi keamanan kami.

---

## 📌 Versi yang Didukung (Supported Versions)

Kami menyediakan pembaruan keamanan, perbaikan bug kritis, dan rilis patch untuk versi-versi berikut:

| Versi | Didukung? | Status |
| :--- | :---: | :--- |
| **1.0.x** | ✅ **Ya** | Versi Rilis Stabil Utama (Current Stable) |
| **< 1.0.0** (Beta/Alpha) | ❌ Tidak | Telah usang; pengguna diharapkan memperbarui ke versi 1.0.x |

---

## 🚨 Pelaporan Kerentanan (Reporting a Vulnerability)

**JANGAN PERNAH** melaporkan potensi kerentanan keamanan melalui *GitHub Public Issues*, media sosial, atau saluran publik lainnya.

Jika Anda menemukan atau mencurigai adanya celah keamanan pada ekosistem Widya-Lang, silakan laporkan secara privat melalui salah satu jalur berikut:

1. **Email Resmi Tim Keamanan**:
   Kirimkan rincian temuan Anda ke:
   ```text
   security@widya-lang.org
   ```
2. **GitHub Private Vulnerability Reporting**:
   Gunakan tab **Security** -> **Advisories** -> **Report a vulnerability** pada repositori resmi GitHub kami (`github.com/widya-lang/widya`).

---

### Informasi yang Perlu Disertakan dalam Laporan:
Agar kami dapat mereproduksi dan menanggapi laporan Anda secepat mungkin, mohon sertakan:
- **Deskripsi Kerentanan**: Jenis celah keamanan (misal: Memory Safety, IPC Token Bypass, Buffer Overflow, RCE, Path Traversal, dll.).
- **Komponen Terdampak**: *Compiler*, *Interpreter*, *WidyaWeb*, *WidyaDesktop*, atau *Standard Library*.
- **Langkah Reproduksi (*Proof of Concept / PoC*)**: Cuplikan kode `.wya` atau perintah pengujian yang dapat memicu masalah.
- **Dampak Potensial**: Skenario eksploitasi dan risiko terhadap pengguna atau sistem host.
- **Saran Perbaikan** *(opsional)*: Jika Anda memiliki patch atau rekomendasi mitigasi.

---

## ⏱️ Standar Waktu Respons (SLA Response Timeline)

Tim Keamanan kami beroperasi dengan komitmen respons berikut:
- **Konfirmasi Awal**: Maksimal **48 jam** kerja sejak email atau laporan diterima.
- **Penilaian Awal & Validasi Celah**: Maksimal **5 hari kerja**.
- **Penyusunan Patch & Perbaikan**: Bergantung pada tingkat keparahan, perbaikan darurat (*hotfix*) diprioritaskan dalam **7 – 14 hari**.
- **Masa Embargo Publikasi**: Kami meminta masa embargo selama **30 hingga 90 hari** sebelum rincian kerentanan dipublikasikan ke publik, guna memberikan waktu yang cukup bagi pengguna untuk memperbarui sistem mereka.

---

## 🔒 Ruang Lingkup Keamanan Khusus

Widya-Lang mengadopsi prinsip pertahanan berlapis (*Defense-in-Depth*):
- **WidyaWeb Framework**: Perlindungan CSRF, sanitasi header, HTTP Security Headers, proteksi Session Hijacking, dan isolasi worker.
- **WidyaDesktop Framework**: Validasi token IPC kriptografis (`X-Widya-IPC-Token`), penegakan Content Security Policy (Strict CSP), Brankas Terotentikasi AES-256-GCM, dan integritas aset SHA-256 (`widya_security.lock`).
- **Integritas Distribusi**: Setiap rilis biner resmi ditandatangani dengan tanda tangan digital Ed25519 dan ringkasan SHA-256 (`SHA256SUMS.txt`).

---

## 🏆 Penghargaan & Pengakuan (*Hall of Fame*)

Kami sangat menghargai kontribusi para peneliti keamanan (*whitehat hackers*) dan komunitas. Setiap kontributor yang menemukan celah keamanan yang terverifikasi akan dicantumkan dalam catatan rilis (*Release Notes*) dan daftar **Widya Security Hall of Fame** (dengan persetujuan Anda).

Terima kasih telah membantu menjaga ekosistem Widya-Lang tetap aman dan andal!
