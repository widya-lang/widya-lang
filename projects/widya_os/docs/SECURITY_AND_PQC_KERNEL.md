# Spesifikasi Keamanan Kriptografi Pasca-Kuantum (PQC) & Kernel Berdaulat 🛡️

> **Standar Keamanan Tingkat Pertahanan & Kedaulatan Data Nasional**  
> *NIST FIPS 204 (ML-DSA-87) · HSM PKCS#11 v2.40 · Immutable dm-verity · Anti-Ransomware Sandbox*

---

## 1. Pendahuluan

Sistem operasi komersial saat ini menjadi target utama serangan siber global:
- **Ransomware:** Memindai seluruh disk dan mengenkripsi dokumen penting pengguna.
- **Supply-Chain Backdoors:** Pembaruan sistem yang disusupi malware asing.
- **Ancaman Komputer Kuantum (*Q-Day*):** Enkripsi lama seperti RSA-2048 dan kurva eliptik akan dapat dijebol oleh komputer kuantum skala besar.

**WidyaOS** dirancang dari lapisan kernel terbawah dengan filosofi **"Post-Quantum Native Security"**, memadukan modul kriptografi [src/kripto/hsm_pkcs11.rs](file:///home/devops/widya-lang/src/kripto/hsm_pkcs11.rs) ke dalam arsitektur keamanan sistem operasi.

---

## 2. Empat Pilar Keamanan WidyaOS

```
┌────────────────────────────────────────────────────────┐
│           PILAR 4: ISOLASI APLIKASI ANTI-RANSOMWARE    │
│   (Sandbox Akses Berkas Terbatas, Portal Izin Nyata)   │
├────────────────────────────────────────────────────────┤
│           PILAR 3: SISTEM BERKAS KEBAL INTEGRITAS      │
│   (Read-Only dm-verity RootFS + Hash Pohon Merkle)     │
├────────────────────────────────────────────────────────┤
│           PILAR 2: INTEGRASI HSM PERANGKAT KERAS       │
│   (Kunci Enkripsi Terkunci di Chip Fisik PKCS#11/TPM)  │
├────────────────────────────────────────────────────────┤
│           PILAR 1: TANDA TANGAN ANTI-KUANTUM (PQC)     │
│   (Validasi Biner Kernel & Aplikasi via ML-DSA-87)     │
└────────────────────────────────────────────────────────┘
```

---

## 3. Rincian Implementasi Keamanan

### A. Tanda Tangan Pasca-Kuantum (NIST FIPS 204 ML-DSA-87)
Setiap paket pembaruan sistem operasi dan biner executable wajib memiliki tanda tangan digital ML-DSA-87:
- Bootloader memverifikasi tanda tangan kernel sebelum eksekusi (*Secure Boot Pasca-Kuantum*).
- Kernel memverifikasi tanda tangan modul driver dan biner aplikasi sebelum diizinkan berjalan di memori.
- Biner yang dimodifikasi oleh malware akan ditolak dengan kode proteksi kernel `SEC_PQC_SIG_REJECT`.

### B. Proteksi Sistem Berkas Kebal (*Immutable dm-verity*)
Partisi sistem inti (`/usr`, `/bin`, `/lib`) disimpan dalam citra *read-only* dengan pohon hash Merkle (*dm-verity*):
- Jika virus mencoba menyisipkan file ke direktori sistem atau memodifikasi file konfigurasi dasar, kernel mendeteksi ketidakcocokan hash blok seketika dan memblokir akses I/O.
- Sistem operasi tetap murni seperti saat pertama kali diinstal selamanya.

### C. Integrasi Modul Keamanan Fisik (HSM PKCS#11)
- Kunci dekripsi partisi data `/home` tidak pernah disimpan di harddisk dalam bentuk teks mentah.
- Kunci terlindungi di dalam chip perangkat keras (*Smartcard, YubiKey, atau modul HSM Nusantara*). Untuk membuka sesi desktop, pengguna menyentuh token fisik dan memasukkan PIN otentikasi.

### D. Sandbox Akses Berkas (Anti-Ransomware Portal)
Aplikasi pihak ketiga atau file yang diunduh dari internet berjalan di ruang terbatas (*restricted namespace & seccomp-bpf*):
- Aplikasi tidak dapat memindai folder pribadi pengguna secara liar.
- Akses berkas dokumen hanya diberikan melalui **System File Picker Portal** resmi ketika pengguna secara eksplisit memilih berkas tersebut di antarmuka.
- Ransomware menjadi tidak berdaya karena tidak memiliki izin menulis ke partisi dokumen lain.
