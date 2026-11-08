# Spesifikasi Keamanan Dokumen & Tanda Tangan Pasca-Kuantum (PQC) 🛡️

> **Spesifikasi Teknis Integritas Dokumen & Tanda Tangan Elektronik Tersertifikasi**  
> *Standar Industri: NIST FIPS 204 (ML-DSA-87) · HSM PKCS#11 v2.40 · OpenXML Digital Signatures*

---

## 1. Urgensi Kriptografi Pasca-Kuantum (PQC) pada Dokumen Kantor

Dokumen perkantoran formal seperti akta notaris, keputusan pemerintah, laporan keuangan korporat, dan kontrak dagang sering kali memiliki masa retensi legal antara 10 hingga 50 tahun. Kriptografi asimetris konvensional (seperti RSA-2048/4096 atau ECDSA P-256) rentan didekripsi atau dipalsukan di masa depan menggunakan komputer kuantum melalui Algoritma Shor (*Harvest Now, Decrypt Later*).

**WidyaOffice Desktop** mengintegrasikan sistem keamanan dokumen berdaulat langsung dari [src/kripto/hsm_pkcs11.rs](file:///home/devops/widya-lang/src/kripto/hsm_pkcs11.rs) dan [src/office/security.rs](file:///home/devops/widya-lang/src/office/security.rs), memungkinkan penandatanganan berkas DOCX, XLSX, dan PDF dengan algoritma kuantum modern.

---

## 2. Arsitektur Penandatanganan Dokumen

```
┌────────────────────────────────────────────────────────────────────────┐
│                        DOKUMEN ASLI (DOCX / PDF)                       │
│    (Teks Paragraf, Rumus OMML, Tabel, Gambar, Metadata Properti)       │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                        Kompilasi Hash Semantik
                        (SHA-256 / SHA3-512)
                                    │
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        RINGKASAN HASH DOKUMEN                          │
│               [ e3b0c44298fc1c149afbf4c8996fb92427ae... ]             │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ├───────────────────────────────────┐
                                    │                                   │
                                    ▼                                   ▼
┌───────────────────────────────────────────────┐ ┌───────────────────────────────────────┐
│     Tanda Tangan Konvensional (Ed25519)       │ │   Tanda Tangan Pasca-Kuantum (ML-DSA) │
│       (Kompatibilitas Sistem Warisan)         │ │         (NIST FIPS 204 ML-DSA-87)     │
└───────────────────────┬───────────────────────┘ └───────────────────┬───────────────────┘
                        │                                             │
                        └──────────────────────┬──────────────────────┘
                                               │
                                               ▼
┌────────────────────────────────────────────────────────────────────────┐
│                    HYBRID SIGNATURE CONTAINER PKCS#7                   │
│      Disematkan ke dalam berkas: word/_xmlsignatures/sig1.xml          │
│      Kunci Privat terlindungi dalam HSM PKCS#11 (Non-Extractable)      │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Integrasi Hardware Security Module (HSM PKCS#11)

WidyaOffice Desktop mendukung penandatanganan dokumen legal menggunakan token fisik atau smartcard berbasis standar internasional Cryptoki **PKCS#11 v2.40**:
- Kunci privat **tidak pernah meninggalkan chip perangkat keras** (*non-extractable key*).
- Pengguna memasukkan PIN otentikasi melalui dialog aman desktop.
- Operasi kriptografi dilakukan langsung di dalam modul perangkat keras kripto yang terpasang pada port USB atau modul HSM jaringan.

---

## 4. Antarmuka Verifikasi & Inspektur Tanda Tangan

Pada antarmuka desktop, dokumen yang ditandatangani akan menampilkan indikator visual yang jelas pada Status Bar dan Panel Sisi (*Sidebar Inspector*):

```
┌────────────────────────────────────────────────────────┐
│              INSPEKTUR TANDA TANGAN DOKUMEN            │
├────────────────────────────────────────────────────────┤
│ [✓] STATUS: DOKUMEN ASLI & TERVERIFIKASI               │
│                                                        │
│ Penandatangan: Dr. Ir. Widya Daniswara, M.T.           │
│ Lembaga      : Badan Keamanan Siber & Sandi Nasional   │
│ Waktu Tanda  : 28 Oktober 2026, 11:00:24 WIB           │
│ Sertifikat   : Root CA Berdaulat Nusantara             │
│ Algoritma    : Post-Quantum ML-DSA-87 (FIPS 204)       │
│ Perangkat    : HSM Token YubiKey / Sovereign PKCS#11   │
│                                                        │
│ Uji Integritas:                                        │
│ - Teks Dokumen   : Cocok (SHA3-512 Valid)              │
│ - Rumus OMML Math: Cocok (Tidak ada modifikasi)        │
│ - Lampiran Berkas: Cocok                               │
│                                                        │
│ [ Unduh Sertifikat X.509 ]      [ Periksa Pencabutan ] │
└────────────────────────────────────────────────────────┘
```

Jika ada pihak ketiga yang memodifikasi bahkan hanya satu karakter spasi atau rumus pada dokumen setelah ditandatangani, WidyaOffice Desktop akan seketika menampilkan pita merah:
> ⚠️ **PERINGATAN: Tanda tangan digital rusak! Dokumen telah mengalami modifikasi setelah penandatanganan.**
