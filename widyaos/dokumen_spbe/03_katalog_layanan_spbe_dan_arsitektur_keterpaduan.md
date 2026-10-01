# KATALOG LAYANAN SPBE DAN ARSITEKTUR KETERPADUAN
## SISTEM PEMERINTAHAN BERBASIS ELEKTRONIK SEKTOR PENDIDIKAN DAN KEBUDAYAAN
### Klasifikasi Layanan Administrasi Pemerintahan (G2G/G2E) dan Layanan Publik (G2C/G2B) Berbasis Ekosistem Berdaulat WidyaOS & Sewidya (`sewidya.id`)

---

**Penyusun:** Direktorat Tata Kelola Layanan Digital SPBE  
**Dasar Regulasi:** Perpres No. 95/2018 (Pasal 33-40 tentang Layanan SPBE), Perpres No. 82/2023 (INA Digital), PermenPANRB No. 59/2020  
**Klasifikasi Dokumen:** Katalog Layanan Resmi SPBE Nasional  
**Tahun:** 2026  

---

## DAFTAR ISI

1. [BAGIAN I: KERANGKA UMUM ARSITEKTUR LAYANAN SPBE](#bagian-i-kerangka-umum-arsitektur-layanan-spbe)
2. [BAGIAN II: KATALOG LAYANAN ADMINISTRASI PEMERINTAHAN](#bagian-ii-katalog-layanan-administrasi-pemerintahan)
   - 2.1. Layanan Antar-Instansi Pemerintah (*Government to Government - G2G*)
   - 2.2. Layanan Aparatur Sipil Negara & Pendidik (*Government to Employee - G2E*)
3. [BAGIAN III: KATALOG LAYANAN PUBLIK PENDIDIKAN](#bagian-iii-katalog-layanan-publik-pendidikan)
   - 3.1. Layanan Warga Negara & Peserta Didik (*Government to Citizen - G2C*)
   - 3.2. Layanan Pelaku Usaha & Mitra Industri (*Government to Business - G2B*)
4. [BAGIAN IV: ARSITEKTUR KETERPADUAN & PROTOKOL INTEROPERABILITAS (WIDYABUS)](#bagian-iv-arsitektur-keterpaduan--protokol-interoperabilitas)
5. [BAGIAN V: STANDAR TINGKAT LAYANAN (*SERVICE LEVEL AGREEMENT - SLA*)](#bagian-v-standar-tingkat-layanan-sla)

---

## BAGIAN I: KERANGKA UMUM ARSITEKTUR LAYANAN SPBE

Arsitektur Layanan SPBE pada ekosistem WidyaOS dan Sewidya dirancang dengan mengedepankan prinsip **keterpaduan menyeluruh (end-to-end integration)**, **zero redundancy**, dan **user-centricity** (berpusat pada kebutuhan warga sekolah).

```
+----------------------------------------------------------------------------------------------------+
|                                  ARSITEKTUR LAYANAN SPBE SEWIDYA                                   |
+----------------------------------------------------------------------------------------------------+
|               LAYANAN PUBLIK (G2C & G2B)                |     LAYANAN PEMERINTAHAN (G2G & G2E)     |
|  - PPDB Online Zonasi Haversine                         |  - Dapodik Edge Synchronizer (G2G)       |
|  - Ijazah Digital Abadi (WidyaChain W3C DID)            |  - ARKAS Multi-Dana & BKU Shield (G2G)   |
|  - Portal Orang Tua & Presensi Realtime                 |  - Logistik Makan Bergizi Gratis (G2G)   |
|  - Inklusi Disabilitas SLB (WidyaAkses BISINDO)         |  - Persuratan SINDE & SRIKANDI (G2G)     |
|  - SIPLah SDS Belanja Terpadu Pajak (G2B)               |  - Kinerja ASN Guru SKP & PAK (G2E)      |
|  - Widya App Store & SDK Developer (G2B)                |  - Validasi Sertifikasi TPG 24 JJM (G2E) |
+----------------------------------------------------------------------------------------------------+
|                                    WIDYABUS: INTEROPERABILITAS G2G                                  |
|        (Dukcapil NIK | Kemenkes SatuSehat | Kemensos DTKS | Kemenkeu SPAN | BGN | BKN)             |
+----------------------------------------------------------------------------------------------------+
|                               INFRASTRUKTUR & BASIS DATA BERDAULAT                                 |
|           (WidyaOS Desktop | WidyaBase RDBMS | WidyaOLAP Columnar | WidyaWAN 10.38.0.0/16)          |
+----------------------------------------------------------------------------------------------------+
```

---

## BAGIAN II: KATALOG LAYANAN ADMINISTRASI PEMERINTAHAN

### 2.1. Layanan Antar-Instansi Pemerintah (*Government to Government - G2G*)

#### 1. SPBE-G2G-01: Interoperabilitas Data Pokok Pendidikan (Dapodik Edge Synchronizer)
- **Deskripsi**: Penyeragaman data kelembagaan, sarana prasarana, guru, dan peserta didik dari satuan pendidikan ke server pusat kementerian.
- **Penyedia Layanan**: Pusdatin Kemendikdasmen RI.
- **Pengguna Layanan**: Satuan Pendidikan (434.000 sekolah), Dinas Pendidikan Kab/Kota/Provinsi, Kemenag, BPS.
- **Fitur Kunci**:
  - Validasi 8 pilar Dapodik offline-first di server sekolah.
  - Deteksi anomali duplikasi NIK siswa melalui integrasi instan dengan API Dukcapil.
  - Audit perubahan riwayat berlaku surut (*backdated records*) untuk anak putus sekolah.
- **Modul Penggerak**: [dapodik_edge.wya](file:///c:/widya-lang/widyaos/modul/dapodik_edge.wya), [dapodik_excel_bridge.wya](file:///c:/widya-lang/widyaos/modul/dapodik_excel_bridge.wya).

#### 2. SPBE-G2G-02: Penatausahaan & Kliring BOSP Multi-Dana (ARKAS Modern)
- **Deskripsi**: Pengelolaan buku kas umum (BKU) seluruh sumber dana sekolah (BOSP Reguler, BOS Kinerja, BOSDA, Banpem Revitalisasi, Komite).
- **Penyedia Layanan**: Ditjen PAUD Dikdasmen & Pusdatin Kemendikdasmen.
- **Pengguna Layanan**: Kepala Sekolah, Bendahara BOS, Dinas Pendidikan, BPKAD Pemda, Kemenkeu.
- **Fitur Kunci**:
  - *Immutable Shield*: Larangan mutlak penghapusan fisik rekaman transaksi (`DELETE/TRUNCATE` diblokir total).
  - Otomatisasi jurnal pembalik (*storno*) bertaut ID induk sesuai standar BPK RI.
  - Guardrail Plafon Standar Satuan Harga (SSH) Regional untuk mencegah mark-up anggaran.
  - Interoperabilitas dwiarah dengan SIPD Kemendagri dan SPAN Kemenkeu.
- **Modul Penggerak**: [arkas_multidana.wya](file:///c:/widya-lang/widyaos/modul/arkas_multidana.wya), [arkas_bku_immutable_shield.wya](file:///c:/widya-lang/widyaos/modul/arkas_bku_immutable_shield.wya).

#### 3. SPBE-G2G-03: Tata Kelola Logistik & Keamanan Pangan Makan Bergizi Gratis (MBG BGN)
- **Deskripsi**: Pengawasan distribusi makanan bergizi gratis dari Sentra Dapur Gizi Badan Gizi Nasional (BGN) ke satuan pendidikan.
- **Penyedia Layanan**: Badan Gizi Nasional (BGN) & Kemendikdasmen.
- **Pengguna Layanan**: Dapur Sentra BGN, Tim Distribusi, Sekolah, Puskesmas Pembina.
- **Fitur Kunci**:
  - Validasi presensi riil siswa di gerbang sekolah untuk mencegah penggelembungan porsi fiktif.
  - Interlock alergen siswa terhubung dengan rekam medis UKS (auto-substitusi menu pemicu alergi).
  - Prosedur penyimpanan sampel saksi makanan 2x24 jam untuk forensik anti-keracunan.
- **Modul Penggerak**: [mbg_gizi_gratis.wya](file:///c:/widya-lang/widyaos/modul/mbg_gizi_gratis.wya), [uks_kesehatan_mbg.wya](file:///c:/widya-lang/widyaos/modul/uks_kesehatan_mbg.wya).

#### 4. SPBE-G2G-04: Persuratan Elektronik Kedinasan Terpadu (SINDE & SRIKANDI)
- **Deskripsi**: Pengelolaan naskah dinas, agenda persuratan masuk/keluar, dan lembar disposisi digital kepala sekolah.
- **Penyedia Layanan**: ANRI & Kemendikdasmen.
- **Pengguna Layanan**: Pimpinan K/L, Kepala Dinas, Kepala Sekolah, Tata Usaha.
- **Fitur Kunci**:
  - Ingesti otomatis format SINDE Kemendikdasmen dan SRIKANDI ANRI.
  - Tanda tangan elektronik tersertifikasi BSrE BSSN dan segel W3C DID.
  - Template Tata Naskah Dinas F4 Folio standar baku.
- **Modul Penggerak**: [persuratan_elektronik_srikandi_sinde.wya](file:///c:/widya-lang/widyaos/modul/persuratan_elektronik_srikandi_sinde.wya), [kop_surat_sekolah.wya](file:///c:/widya-lang/widyaos/modul/kop_surat_sekolah.wya).

---

### 2.2. Layanan Aparatur Sipil Negara & Pendidik (*Government to Employee - G2E*)

#### 1. SPBE-G2E-01: Manajemen Kinerja Guru (SKP BKN & Angka Kredit PAK)
- **Deskripsi**: Penilaian kinerja aparatur sipil negara dan guru berdasar target kinerja tahunan dan bukti dukung riil.
- **Penyedia Layanan**: BKN & Kemendikdasmen.
- **Pengguna Layanan**: Guru ASN (PNS & PPPK), Tenaga Kependidikan.
- **Fitur Kunci**:
  - Sinkronisasi instan Sasaran Kinerja Pegawai (SKP) ke portal e-Kinerja BKN.
  - Perhitungan angka kredit kumulatif (PAK Konvensional vs Integrasi) bebas rekayasa.
- **Modul Penggerak**: [kepegawaian_kinerja_skp_pak.wya](file:///c:/widya-lang/widyaos/modul/kepegawaian_kinerja_skp_pak.wya).

#### 2. SPBE-G2E-02: Validasi Sertifikasi TPG 24 Jam & Presensi UPT Terintegrasi
- **Deskripsi**: Verifikasi pemenuhan beban kerja tatap muka minimal 24 Jam Jam Mengajar (JJM) untuk pencairan Tunjangan Profesi Guru (TPG).
- **Penyedia Layanan**: Ditjen GTK Kemendikdasmen & BKN.
- **Pengguna Layanan**: Guru Bersertifikasi Pendidik.
- **Fitur Kunci**:
  - Pemetaan beban mengajar linier termasuk mata pelajaran Muatan Lokal Daerah (2 JJM).
  - Presensi geofence UPT Sekolah Terintegrasi (SD/SMP/SMA 1 atap) dengan liveness biometrik.
  - Otomasi pemotongan Tukin berbasis PP 94/2021 secara transparan.
- **Modul Penggerak**: [presensi_upt_terintegrasi.wya](file:///c:/widya-lang/widyaos/modul/presensi_upt_terintegrasi.wya), [manajemen_akademik_lengkap.wya](file:///c:/widya-lang/widyaos/modul/manajemen_akademik_lengkap.wya).

#### 3. SPBE-G2E-03: Perlindungan Profesi & Advokasi Hukum Guru
- **Deskripsi**: Perlindungan hukum bagi pendidik dari kriminalisasi atas tindakan pendisiplinan edukatif yang sah.
- **Penyedia Layanan**: Kemendikdasmen & LKBH PGRI/Organisasi Profesi Guru.
- **Pengguna Layanan**: Seluruh Tenaga Pendidik.
- **Fitur Kunci**:
  - SOP Disiplin Positif Permendikbud No. 10/2017 ber-audit trail kriptografi.
  - Bantuan hukum instan dan mode jam tenang (*quiet hours*) pada portal wali murid.
- **Modul Penggerak**: [perlindungan_dan_profesi_guru.wya](file:///c:/widya-lang/widyaos/modul/perlindungan_dan_profesi_guru.wya).

---

## BAGIAN III: KATALOG LAYANAN PUBLIK PENDIDIKAN

### 3.1. Layanan Warga Negara & Peserta Didik (*Government to Citizen - G2C*)

#### 1. SPBE-G2C-01: Penerimaan Peserta Didik Baru (PPDB) Zonasi Berkeadilan
- **Deskripsi**: Sistem seleksi calon murid baru secara transparan berbasis jarak geospasial bola bumi presisi meter.
- **Penyedia Layanan**: Dinas Pendidikan Provinsi/Kab/Kota & Sekolah.
- **Pengguna Layanan**: Calon Siswa, Orang Tua/Wali Murid.
- **Fitur Kunci**:
  - Rumus Haversine bola bumi akurasi tinggi mencegah manipulasi jarak.
  - Dynamic Quota Tuning (Zonasi 50%, Afirmasi DTKS 15%, Perpindahan 5%, Prestasi 30%).
  - Pelimpahan otomatis sisa kuota (*spillover*) ke jalur zonasi tanpa kuota siluman.
- **Modul Penggerak**: [ppdb_zonasi.wya](file:///c:/widya-lang/widyaos/modul/ppdb_zonasi.wya), [widyageo_pemetaan.wya](file:///c:/widya-lang/widyaos/modul/widyageo_pemetaan.wya).

#### 2. SPBE-G2C-02: Penerbitan & Verifikasi Ijazah Digital Abadi (WidyaChain)
- **Deskripsi**: Penerbitan ijazah dan sertifikat kompetensi berbasis W3C Verifiable Credentials pada konsensus blockchain 38 provinsi.
- **Penyedia Layanan**: Kemendikdasmen RI & Dinas Pendidikan.
- **Pengguna Layanan**: Alumni, Perguruan Tinggi, Pemberi Kerja / HRD DUDI.
- **Fitur Kunci**:
  - Verifikasi offline instan dalam < 1 detik menggunakan pemindai QR tanpa bergantung server luar.
  - Kebal pemalsuan nilai, kebal kebakaran fisik sekolah, dan bebas pungutan legalisir basah.
- **Modul Penggerak**: [widyachain_ledger.wya](file:///c:/widya-lang/widyaos/modul/widyachain_ledger.wya), [kedaulatan_lintas_kl_nasional.wya](file:///c:/widya-lang/widyaos/modul/kedaulatan_lintas_kl_nasional.wya).

#### 3. SPBE-G2C-03: Aksesibilitas Pendidikan Ramah Disabilitas (WidyaAkses)
- **Deskripsi**: Fasilitas komputasi asistif bagi penyandang disabilitas di Sekolah Luar Biasa (SLB) dan sekolah inklusi.
- **Penyedia Layanan**: Satuan Pendidikan Khusus / Inklusi.
- **Pengguna Layanan**: Peserta Didik Tuna Netra, Tuna Rungu, Tuna Daksa.
- **Fitur Kunci**:
  - Text-to-Speech (TTS) artikulasi vokal alami Bahasa Indonesia on-device.
  - Penerjemah Bahasa Isyarat BISINDO on-device tanpa kirim video ke cloud asing.
  - Navigasi saklar tunggal (*single-switch scanning*) bagi tuna daksa.
- **Modul Penggerak**: [widyaakses_inklusi.wya](file:///c:/widya-lang/widyaos/modul/widyaakses_inklusi.wya).

#### 4. SPBE-G2C-04: Kanal Pengaduan Kekerasan Sekolah (Safe School TPPK)
- **Deskripsi**: Pelaporan insiden perundungan, kekerasan fisik, dan kekerasan seksual di lingkungan sekolah.
- **Penyedia Layanan**: Tim Pencegahan dan Penanganan Kekerasan (TPPK) Sekolah & Kementerian.
- **Pengguna Layanan**: Korban, Saksi, Siswa, Orang Tua.
- **Fitur Kunci**:
  - Pelaporan anonim berbasis enkripsi *Zero-Knowledge* (identitas pelapor terlindungi 100%).
  - Peringatan instan ke Unit PPA Kepolisian jika melibatkan tindak pidana di bawah umur.
- **Modul Penggerak**: [sekolah_aman_tppk.wya](file:///c:/widya-lang/widyaos/modul/sekolah_aman_tppk.wya).

---

### 3.2. Layanan Pelaku Usaha & Mitra Industri (*Government to Business - G2B*)

#### 1. SPBE-G2B-01: Pengadaan Barang/Jasa Sekolah (SIPLah SDS Terpadu)
- **Deskripsi**: Pasar daring pengadaan barang dan jasa satuan pendidikan dengan kepatuhan perpajakan otomatis.
- **Penyedia Layanan**: Kemendikdasmen & Mitra Pengelola SIPLah.
- **Pengguna Layanan**: Penyedia UMKM/Koperasi, Satuan Pendidikan.
- **Fitur Kunci**:
  - SSO SDS OAuth 2.0 (RBAC Kepala Sekolah vs Bendahara).
  - Pemisahan pajak instan: PPN 11% & PPh 22 (DJP via NTPN) dan PBJT Mamin 10% (Bapenda via NTPD).
  - Guardrail Harga Eceran Tertinggi (HET) Buku Puskurbuk Zona 1 s.d. 5B.
- **Modul Penggerak**: [siplah_sds_wartek.wya](file:///c:/widya-lang/widyaos/modul/siplah_sds_wartek.wya), [siplah_pajak.wya](file:///c:/widya-lang/widyaos/modul/siplah_pajak.wya).

#### 2. SPBE-G2B-02: Ekosistem Pengembang Aplikasi Pendidikan (Widya App Store & SDK)
- **Deskripsi**: Marketplace aplikasi pihak ketiga terverifikasi untuk sekolah dengan mekanisme bagi hasil otomatis.
- **Penyedia Layanan**: Pengelola Ekosistem WidyaOS & Sewidya.
- **Pengguna Layanan**: Pengembang Software Nasional, EdTech Startup, Mitra Yayasan.
- **Fitur Kunci**:
  - Pembagian pendapatan otomatis di muka: 11% PPN (DJP), 10% PNBP (Kemendikdasmen), 79% Pengembang.
  - SDK Developer Framework dengan signing W3C DID dan static linter anti-malware.
  - Tagihan langsung terintegrasi BKU ARKAS rekening belanja langganan `5.1.02.02.01.0063`.
- **Modul Penggerak**: [appstore_langganan_pihak_ketiga.wya](file:///c:/widya-lang/widyaos/modul/appstore_langganan_pihak_ketiga.wya), [widyasdk_developer_framework.wya](file:///c:/widya-lang/widyaos/modul/widyasdk_developer_framework.wya).

#### 3. SPBE-G2B-03: Kemitraan Industri & Penelusuran Tamatan SMK (DUDI Tracer)
- **Deskripsi**: Platform penyelarasan kurikulum vokasi (Link & Match 8 Aspek) dan penempatan Praktik Kerja Lapangan.
- **Penyedia Layanan**: Direktorat Mitras DUDI & SMK.
- **Pengguna Layanan**: Dunia Usaha / Dunia Industri, Siswa SMK, Alumni.
- **Fitur Kunci**:
  - Penilaian sertifikat PKL resmi: 60% Hard Skills + 40% Soft Skills K3/5R.
  - Tracer Study otomatis formula BMW (Bekerja, Melanjutkan, Wirausaha).
- **Modul Penggerak**: [dudi_pkl_tracer.wya](file:///c:/widya-lang/widyaos/modul/dudi_pkl_tracer.wya).

---

## BAGIAN IV: ARSITEKTUR KETERPADUAN & PROTOKOL INTEROPERABILITAS (WIDYABUS)

Seluruh pertukaran data antar-layanan SPBE mengalir melalui bus interoperabilitas **WidyaBus (SPLP Bus)**:

```
+-----------------------------------------------------------------------------------------------+
|                            WIDYABUS: SISTEM PENGHUBUNG LAYANAN PEMERINTAH                     |
+-----------------------------------------------------------------------------------------------+
|  KEMENDIKDASMEN          DUKCAPIL              KEMENKEU            KEMENKES           BGN     |
|  (Dapodik & ARKAS)    (Identitas NIK)       (SPAN & DJP)         (SatuSehat)         (MBG)    |
|         |                    |                    |                   |                |      |
|         v                    v                    v                   v                v      |
|  [Adapter Pusdatin]   [Adapter Dukcapil]   [Adapter SPAN]      [Adapter Kes]    [Adapter BGN] |
+-----------------------------------------------------------------------------------------------+
|                         JALUR PESAN AMAN (SECURE MESSAGE BUS & AUDIT TRAIL)                   |
|  - Standar Protokol: JSON-RPC 2.0 / gRPC over TLS 1.3                                         |
|  - Enkripsi Envelop: ChaCha20-Poly1305 Zero-Knowledge                                         |
|  - Otorisasi: W3C DID Verifiable Credentials & OAuth 2.0 OIDC                                 |
|  - Jaminan Integritas: Hash SHA-256 tercatat di Merkle Audit Log BSSN                         |
+-----------------------------------------------------------------------------------------------+
```

---

## BAGIAN V: STANDAR TINGKAT LAYANAN (*SERVICE LEVEL AGREEMENT - SLA*)

| Parameter Layanan | Target Komitmen Layanan | Mekanisme Penjaminan |
|---|---|---|
| **Ketersediaan Layanan (Uptime)** | **99.99%** (Maksimal downtime 52 menit/tahun) | Multi-Datacenter Cluster (Jakarta - Batam - IKN) |
| **Waktu Tanggap Kueri (Latency)** | **< 100 milidetik** untuk transaksi online | WidyaBase In-Memory Engine & Edge Caching |
| **Waktu Pemulihan Bencana (RTO)** | **< 3 menit** | Failover Otomatis WidyaOrchestrator |
| **Toleransi Kehilangan Data (RPO)**| **0 detik (Zero Data Loss)** | Replikasi Sinkron Transaksi Walidata |
| **Kapasitas Beban Lonjakan** | **50.000 permintaan/detik** | Dynamic Rate Limiter & Circuit Breaker |
| **Dukungan Operasi Offline** | **100% Fungsi Esensial Berjalan Offline** | Arsitektur Offline-First & CRDT Auto-Reconcile |

---
*Ditetapkan sebagai Katalog Layanan Resmi dan Standar Keterpaduan SPBE Nasional.*
