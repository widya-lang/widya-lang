# NASKAH AKADEMIK
## KEDAULATAN SISTEM OPERASI, KOMPUTASI AWAN, DAN INTEROPERABILITAS LAYANAN DIGITAL PENDIDIKAN NASIONAL
### Landasan Filosofis, Sosiologis, Yuridis, dan Komparasi Teknologi Menuju Swasembada Digital Sistem Pemerintahan Berbasis Elektronik (SPBE) Republik Indonesia

---

**Penyusun:** Tim Pengembang Arsitektur Kedaulatan Digital WidyaOS & Platform Sewidya  
**Target Regulasi:** Rancangan Peraturan Presiden tentang Standar Sistem Operasi, Infrastruktur Komputasi Berdaulat, dan Keterpaduan Layanan Digital Pendidikan Nasional  
**Klasifikasi:** Dokumen Kebijakan Strategis & Pemenuhan SPBE Nasional  
**Tahun:** 2026  

---

## DAFTAR ISI

1. [BAB I: PENDAHULUAN](#bab-i-pendahuluan)
   - 1.1. Latar Belakang
   - 1.2. Identifikasi Masalah & Urgensi Nasional
   - 1.3. Tujuan dan Sasaran Penyusunan
   - 1.4. Ruang Lingkup Pengaturan
2. [BAB II: KAJIAN TEORETIS, EMPIRIS, DAN KOMPARASI INTERNASIONAL](#bab-ii-kajian-teoretis-empiris-dan-komparasi-internasional)
   - 2.1. Teori Kedaulatan Digital (*Technological & Digital Sovereignty*)
   - 2.2. Bahaya Ketergantungan Asing (*Vendor Lock-in*) & Kebocoran Devisa
   - 2.3. Studi Komparasi Global: Pelajaran dari Uni Eropa, Jerman, Tiongkok, dan Rusia
   - 2.4. Realitas Empiris Infrastruktur Digital Sekolah dan Madrasah di Indonesia
3. [BAB III: LANDASAN FILOSOFIS, SOSIOLOGIS, DAN YURIDIS](#bab-iii-landasan-filosofis-sosiologis-dan-yuridis)
   - 3.1. Landasan Filosofis: Amanat Pembukaan UUD 1945
   - 3.2. Landasan Sosiologis: Keadilan Akses Pendidikan & Wilayah 3T
   - 3.3. Landasan Yuridis: Harmonisasi Regulasi Nasional
4. [BAB IV: JANGKAUAN, ARAH PENGATURAN, DAN MATERI MUATAN KEBIJAKAN](#bab-iv-jangkauan-arah-pengaturan-dan-materi-muatan-kebijakan)
   - 4.1. Pilar 1: Sistem Operasi dan Bahasa Pemrograman Berdaulat (WidyaOS & Widya-Lang)
   - 4.2. Pilar 2: Basis Data dan Analitik Kolumnar Mandiri (WidyaBase & WidyaOLAP)
   - 4.3. Pilar 3: Jalan Tol Komunikasi dan Kriptografi Berdaulat (WidyaWAN & PQC)
   - 4.4. Pilar 4: Keterpaduan Interoperabilitas G2G/G2B/G2C (WidyaBus, Dapodik, ARKAS, SIPLah)
   - 4.5. Pilar 5: Standardisasi TKDN Perangkat Keras dan Manufaktur Lokal
5. [BAB V: PENUTUP DAN REKOMENDASI KEBIJAKAN](#bab-v-penutup-dan-rekomendasi-kebijakan)

---

## BAB I: PENDAHULUAN

### 1.1. Latar Belakang
Transformasi digital pemerintahan di Indonesia telah memasuki fase krusial seiring dengan diterbitkannya **Peraturan Presiden Nomor 95 Tahun 2018 tentang Sistem Pemerintahan Berbasis Elektronik (SPBE)**, **Peraturan Presiden Nomor 39 Tahun 2019 tentang Satu Data Indonesia (SDI)**, serta **Peraturan Presiden Nomor 82 Tahun 2023 tentang Percepatan Transformasi Digital dan Keterpaduan Layanan Digital Nasional (INA Digital)**. Amanat utama dari kerangka regulasi ini adalah mewujudkan tata kelola pemerintahan yang bersih, efektif, transparan, dan akuntabel, serta pelayanan publik yang berkualitas dan terpercaya melalui keterpaduan sistem informasi pemerintah.

Namun demikian, pada sektor pendidikan dasar, menengah, dan kejuruan—yang melibatkan lebih dari **434.000 satuan pendidikan**, **3,3 juta pendidik dan tenaga kependidikan (PTK)**, serta **53 juta peserta didik**—fondasi teknologi digital yang dipergunakan masih mengalami ketergantungan mutlak (*critical dependency*) pada perangkat lunak, sistem operasi, protokol jaringan, dan infrastruktur komputasi awan milik korporasi multinasional asing (*Big Tech*). 

Ketergantungan ini menimbulkan tiga kerentanan fundamental:
1. **Kerentanan Keamanan dan Kedaulatan Data**: Data pokok pendidikan (Dapodik), data keuangan BOSP (ARKAS), data asesmen nasional, hingga rekaman biometrik dan profil psikometrik peserta didik tersimpan atau transit pada infrastruktur penyedia asing yang tunduk pada yurisdiksi ekstrateritorial negara asal (*Clarifying Lawful Overseas Use of Data Act / CLOUD Act*), yang bertentangan dengan prinsip **Undang-Undang Nomor 27 Tahun 2022 tentang Perlindungan Data Pribadi (UU PDP)**.
2. **Beban Finansial Devisa Negara**: Pembayaran lisensi tahunan sistem operasi komersial, biaya langganan paket perkantoran (*office suite*), serta biaya sewa *cloud computing* menguras devisa negara dalam jumlah puluhan triliun rupiah setiap tahun anggaran, yang sesungguhnya dapat dialokasikan untuk pemerataan sarana pendidikan fisik di daerah Tertinggal, Terdepan, dan Terluar (3T).
3. **Ketidaksesuaian Arsitektur Lapangan**: Solusi perangkat lunak proprietary global dirancang dengan asumsi konektivitas internet pita lebar (*broadband*) berkecepatan tinggi yang stabil dan pasokan listrik tanpa putus. Kondisi riil kepulauan Nusantara menunjukkan bahwa ribuan sekolah berada di wilayah *blank spot*, mengalami pemadaman listrik berkala, dan mengandalkan Interactive Flat Panel (IFP) bantuan pemerintah yang sering kali terkunci ekosistem tertutup.

Atas dasar pertimbangan tersebut, diperlukan terobosan strategis berupa perumusan **Sistem Operasi dan Ekosistem Layanan Digital Pendidikan Berdaulat** yang dibangun 100% menggunakan kode sumber mandiri (*Native Sovereign Technology Stack*), yang dinamakan **WidyaOS** dan ekosistem terintegrasi **Sewidya** (`sewidya.id`).

---

### 1.2. Identifikasi Masalah & Urgensi Nasional
Dari telaah situasi lapangan dan audit teknis tata kelola SPBE sektor pendidikan, dirumuskan 5 (lima) masalah utama:
1. **Fragmentasi Aplikasi & Silo Data**: Terjadinya tumpang-tindih ribuan aplikasi pendidikan antar-kementerian dan dinas daerah yang tidak memiliki protokol pertukaran data baku, sehingga menyulitkan konsolidasi data nasional.
2. **Ketiadaan Kemandirian Kompilasi & Kernel**: Ketergantungan pada runtime dan kompiler asing yang menyimpan celah keamanan laten (*zero-day vulnerabilities* dan *hardware backdoor*).
3. **Risiko Pemalsuan Dokumen & Fraud Keuangan**: Kerentanan manipulasi ijazah kelulusan, manipulasi laporan belanja dana Bantuan Operasional Satuan Pendidikan (BOSP), serta distorsi perhitungan pajak belanja daerah.
4. **Kesenjangan Mutu Aksesibilitas**: Kurangnya dukungan sistemik bagi sekolah berkebutuhan khusus (SLB), preservasi aksara daerah, dan mitigasi sekolah tanggap bencana.
5. **Rendahnya Nilai TKDN Perangkat Keras**: Mayoritas pengadaan laptop dan IFP sekolah hanya bertindak sebagai perakit cangkang fisik (*box moving*) tanpa penguasaan perangkat lunak di dalamnya, sehingga nilai Tingkat Komponen Dalam Negeri (TKDN) hakiki belum optimal sesuai **Instruksi Presiden Nomor 2 Tahun 2022**.

---

### 1.3. Tujuan dan Sasaran Penyusunan
Naskah Akademik ini disusun dengan tujuan:
1. Memberikan landasan ilmiah, yuridis, dan komparatif bagi penerbitan instrumen kebijakan kedaulatan digital pendidikan nasional.
2. Menetapkan peta jalan arsitektur sistem operasi, basis data, analitik, dan jaringan berdaulat yang wajib diimplementasikan pada seluruh gawai bantuan TIK pemerintah.
3. Menyelaraskan modul-modul operasional sekolah (Dapodik, ARKAS, SIPLah, LMS Sinau, MBG, Ijazah W3C DID) dengan kerangka arsitektur SPBE nasional dan INA Digital.

---

### 1.4. Ruang Lingkup Pengaturan
Ruang lingkup naskah akademik ini mencakup:
- Standardisasi Sistem Operasi Berdaulat (WidyaOS) pada lingkungan satuan pendidikan dan kantor dinas.
- Spesifikasi Bahasa Pemrograman dan Kompiler Mandiri (Widya-Lang).
- Sistem Manajemen Basis Data Relasional dan Kolumnar (WidyaBase dan WidyaOLAP).
- Jalan Tol Komunikasi Terenkripsi Berdaulat (WidyaWAN).
- Standar Interoperabilitas Layanan Pendidikan SPBE (WidyaBus G2G, G2B, G2C, G2E).
- Integrasi Identitas Kependudukan Digital (IKD) Dukcapil dan INA Digital GovTech.

---

## BAB II: KAJIAN TEORETIS, EMPIRIS, DAN KOMPARASI INTERNASIONAL

### 2.1. Teori Kedaulatan Digital (*Technological & Digital Sovereignty*)
Secara teoretis, kedaulatan digital didefinisikan oleh Pohle & Thiel (2020) sebagai kemampuan suatu negara-bangsa untuk bertindak secara mandiri di ruang siber, mengontrol data warga negaranya, mengamankan infrastruktur kritis informasi, serta menentukan arah inovasi teknologi tanpa tekanan hegemoni eksternal. Kedaulatan digital bukan sekadar proteksionisme sempit, melainkan pemenuhan kapasitas strategis untuk melindungi kepentingan nasional.

Kedaulatan teknologi terbagi ke dalam 4 (empat) lapisan hierarki:
1. **Lapisan Perangkat Keras & Silikon (Layer 0-1)**: Penguasaan rancang bangun prosesor (RISC-V), modul keamanan perangkat keras (TPM/HSM), dan telemetri satelit.
2. **Lapisan Sistem Operasi & Kompiler (Layer 2)**: Kemandirian penuh atas *kernel*, manajemen memori, penjadwalan proses, dan penerjemah kode tanpa ketergantungan pada rantai pasok biner asing.
3. **Lapisan Data & Komputasi (Layer 3)**: Kepemilikan penuh atas mesin basis data, kompresi penyimpanan, dan protokol perutean jaringan.
4. **Lapisan Aplikasi & Layanan Publik (Layer 4)**: Keterpaduan antarmuka layanan publik yang melindungi hak-hak privasi warga negara.

---

### 2.2. Bahaya Ketergantungan Asing (*Vendor Lock-in*) & Kebocoran Devisa
Kajian empiris Bank Indonesia dan Kementerian Keuangan menunjukkan bahwa pembayaran royalti atas kekayaan intelektual perangkat lunak impor merupakan salah satu pos defisit neraca jasa yang signifikan. Selain beban ekonomi, ketergantungan pada satu atau dua korporasi sistem operasi global menghadirkan risiko geopolitik:
- **Ancaman Saklar Pembunuh (*Kill-Switch*) dan Embargo**: Apabila terjadi eskalasi tensi geopolitik, pemutusan akses pembaruan (*patching*) atau pemutusan lisensi dapat melumpuhkan sistem administrasi publik dan kegiatan belajar mengajar nasional dalam hitungan jam.
- **Kerentanan Makro dan Ransomware**: Ketergantungan pada mesin makro konvensional berbasis Visual Basic for Applications (VBA) terbukti menjadi vektor serangan ransomware nomor satu di dunia yang membobol ribuan institusi pendidikan setiap tahunnya.

---

### 2.3. Studi Komparasi Global: Pelajaran Internasional
Banyak negara maju dan berkembang telah mengambil langkah konkret menuju kedaulatan digital:

| Negara / Kawasan | Inisiatif Utama | Deskripsi & Pelajaran untuk Indonesia |
|---|---|---|
| **Uni Eropa & Jerman** | *Gaia-X* & *Sovereign Workplace* | Inisiatif Kementerian Dalam Negeri Jerman menggantikan sistem operasi dan office suite komersial asing di seluruh instansi pemerintah dengan stack open-source berdaulat untuk menegakkan GDPR. |
| **Tiongkok** | *KylinOS* & *OpenEuler* | Kewajiban penggantian seluruh perangkat lunak instansi pemerintah dan pendidikan ke sistem operasi domestik berbasis kernel berdaulat dan arsitektur Loongson/RISC-V. |
| **Rusia** | *Astra Linux* & *Aurora OS* | Penerapan OS bersertifikasi keamanan militer di seluruh fasilitas publik dan pendidikan guna menangkal sanksi embargo teknologi barat. |
| **India** | *India Stack* & *BOSS Linux* | Integrasi identitas biometrik digital nasional (Aadhaar) dengan API publik terbuka dan sistem operasi mandiri untuk digitalisasi sekolah pedesaan. |

Indonesia, dengan populasi lebih dari 280 juta jiwa dan kekuatan ekonomi terbesar di Asia Tenggara, memiliki urgensi dan skala ekonomi yang sangat memadai untuk memiliki sistem operasi berdaulat sendiri, yaitu **WidyaOS**.

---

## BAB III: LANDASAN FILOSOFIS, SOSIOLOGIS, DAN YURIDIS

### 3.1. Landasan Filosofis
Landasan filosofis pembentukan regulasi dan pengembangan WidyaOS berakar pada:
1. **Pembukaan Undang-Undang Dasar Negara Republik Indonesia Tahun 1945**: Khususnya alinea keempat yang memandatkan pemerintah negara Indonesia untuk *"melindungi segenap bangsa Indonesia dan seluruh tumpah darah Indonesia"* serta *"mencerdaskan kehidupan bangsa"*. Perlindungan bangsa di abad ke-21 mencakup perlindungan ruang digital, ketahanan siber, dan pengamanan data masa depan anak-anak Indonesia.
2. **Pancasila**:
   - *Sila Kedua (Kemanusiaan yang Adil dan Beradab)*: Menjamin akses teknologi yang manusiawi, inklusif bagi penyandang disabilitas (WidyaAkses), dan bebas diskriminasi geografis.
   - *Sila Ketiga (Persatuan Indonesia)*: Menghubungkan seluruh kepulauan dari Sabang sampai Merauke melalui jalan tol komunikasi mandiri (WidyaWAN) dan kalender pendidikan nasional yang menghormati kearifan lokal.
   - *Sila Kelima (Keadilan Sosial bagi Seluruh Rakyat Indonesia)*: Memastikan dana BOSP dinikmati secara transparan oleh peserta didik tanpa pemotongan liar dan tanpa pemborosan lisensi proprietary asing.

---

### 3.2. Landasan Sosiologis
Secara sosiologis, ekosistem pendidikan Indonesia memiliki karakteristik unik:
- **Tantangan 3T (Tertinggal, Terdepan, Terluar)**: Mayoritas sekolah di pedalaman tidak memiliki akses internet stabil, sehingga menuntut filosofi *Offline-First* dan *Delay-Tolerant Networking* (WidyaMesh).
- **Beban Administrasi Tenaga Pendidik**: Guru di Indonesia kerap mengeluhkan waktu mengajar yang tergerus oleh pengisian data administratif berulang pada sistem yang tidak terintegrasi. Keterpaduan WidyaOS memungkinkan *Single Sign-On* (SewidyaID) dan pengisian otomatis dokumen kedinasan.
- **Perlindungan Peserta Didik**: Perlindungan anak dari bahaya judi online, pinjaman online ilegal, dan konten negatif melalui penyaring konten terpasang (WidyaNet DNS & TrustPositif).

---

### 3.3. Landasan Yuridis
Pengembangan dan standardisasi WidyaOS memiliki pijakan yuridis yang kuat:
1. **Pasal 31 UUD 1945**: Hak warga negara memperoleh pendidikan dan kewajiban pemerintah membiayai pendidikan dasar.
2. **UU No. 11 Tahun 2008 jo. UU No. 1 Tahun 2024 tentang ITE**: Kewajiban penyelenggara sistem elektronik mengoperasikan sistem elektronik secara andal, aman, dan bertanggung jawab.
3. **UU No. 27 Tahun 2022 tentang Pelindungan Data Pribadi (UU PDP)**: Kewajiban pemrosesan data pribadi secara sah, terbatas, akurat, dan menjamin hak subjek data (Pasal 20-50).
4. **Perpres No. 95 Tahun 2018 tentang SPBE**: Pengaturan tata kelola, manajemen, layanan, infrastruktur, dan keamanan SPBE nasional.
5. **Perpres No. 39 Tahun 2019 tentang Satu Data Indonesia**: Standar data, metadata baku, interoperabilitas data, dan kode referensi nasional.
6. **Perpres No. 82 Tahun 2023 tentang INA Digital**: Percepatan keterpaduan layanan digital nasional dan identitas digital kependudukan.
7. **Inpres No. 2 Tahun 2022 tentang P3DN**: Kewajiban pengalokasian minimal 40% anggaran belanja pengadaan untuk produk dalam negeri ber-TKDN.
8. **PermenPANRB No. 59 Tahun 2020 tentang Pemantauan dan Evaluasi SPBE**: Instrumen pengukuran 47 indikator tingkat kematangan SPBE.

---

## BAB IV: JANGKAUAN, ARAH PENGATURAN, DAN MATERI MUATAN KEBIJAKAN

### 4.1. Pilar 1: Sistem Operasi dan Bahasa Pemrograman Berdaulat
- **Widya-Lang**: Bahasa pemrograman berdaulat yang menggunakan sintaks Bahasa Indonesia dan istilah komputasi baku, dikompilasi langsung menjadi bytecode/biner tanpa ketergantungan pada runtime asing.
- **WidyaOS Desktop & Edge Environment**: Sistem operasi dengan lingkungan desktop teroptimasi untuk perangkat keras sekolah, dilengkapi proteksi memori, isolasi proses (*sandbox*), serta bebas dari telemetri terselubung.
- **WidyaMacro**: Mesin otomasi dokumen yang menggantikan VBA dengan sandbox terisolasi dan segel kriptografi W3C DID, menghilangkan 100% risiko eksekusi kode berbahaya.

### 4.2. Pilar 2: Basis Data dan Analitik Kolumnar Mandiri
- **WidyaBase**: Mesin basis data relasional ACID transaksi penuh dengan dukungan kueri WidyaSQL (Bahasa Indonesia) dan ANSI SQL dwiarah.
- **WidyaOLAP**: Mesin analitik data berorientasi kolom (*columnar storage*) dengan kompresi *Run-Length Encoding* (RLE) dan partisi waktu bulanan untuk pemrosesan analitik Pusdatin dalam hitungan submili-detik.

### 4.3. Pilar 3: Jalan Tol Komunikasi dan Kriptografi Berdaulat
- **WidyaWAN**: Jaringan privat virtual berdaulat (10.38.0.0/16) berbasis Zero-Trust mTLS yang menghubungkan sekolah dengan kementerian tanpa melewati perutean publik komersial yang rentan sadap.
- **WidyaQuantum PQC**: Perlindungan keamanan jangka panjang menggunakan kriptografi pasca-kuantum (*Post-Quantum Cryptography*) berbasis kisi ML-KEM (Kyber-1024) yang kebal terhadap dekripsi komputer kuantum masa depan.

### 4.4. Pilar 4: Keterpaduan Interoperabilitas G2G/G2B/G2C
- **WidyaBus (SPLP Bus)**: Bus interoperabilitas G2G yang menghubungkan data Dapodik dengan Dukcapil (NIK), SatuSehat Kemenkes, DTKS Kemensos, BKN, dan Badan Gizi Nasional (BGN).
- **ARKAS BKU Immutable Shield**: Penjaminan integritas pembukuan kas sekolah dengan larangan mutlak penghapusan fisik (*Zero Physical Deletions*) dan pencatatan jurnal pembalik (*storno*).
- **WidyaChain**: Penerbitan ijazah digital berbasis *Verifiable Credentials* konsensus konsorsium 38 provinsi yang dapat diverifikasi secara instan dalam 1 detik tanpa internet.

### 4.5. Pilar 5: Standardisasi TKDN dan Manufaktur Lokal
- Mewajibkan seluruh gawai TIK pendidikan bantuan APBN/APBD (Laptop, Chromebook, IFP) menggunakan citra emas WidyaOS terverifikasi TPM 2.0.
- Injeksi sertifikat TKDN (Kemenperin) minimal 40% dan penomoran identitas perangkat W3C DID langsung di lini perakitan pabrik domestik.

---

## BAB V: PENUTUP DAN REKOMENDASI KEBIJAKAN

Berdasarkan kajian mendalam filosofis, sosiologis, yuridis, dan teknologis dalam Naskah Akademik ini, direkomendasikan langkah-langkah kebijakan konkret sebagai berikut:

1. **Penerbitan Regulasi Payung**: Mengundangkan Peraturan Presiden tentang Standar Arsitektur Sistem Operasi dan Kedaulatan Komputasi Pendidikan Nasional sebagai mandat legal adopsi WidyaOS.
2. **Penetapan Standar Pengadaan TIK**: Menyusun pedoman teknis pengadaan laptop, PC lab, dan IFP di lingkungan Kemendikdasmen dan Kemenag yang mensyaratkan pemenuhan instalasi WidyaOS dan kelaikan Common Criteria EAL4+ BSSN.
3. **Integrasi ke Portal INA Digital**: Menjadikan Sewidya (`sewidya.id`) sebagai simpul resmi layanan digital pendidikan pada ekosistem INA Digital sesuai amanat Perpres No. 82 Tahun 2023.
4. **Pemberdayaan Ekosistem Riset & DUDI**: Membuka program sertifikasi pengembang perangkat lunak melalui Widya SDK untuk menumbuhkan industri perangkat lunak lokal yang kompetitif di tingkat regional dan global.

---
*Disahkan sebagai Naskah Akademik Resmi Penjaminan Mutu & Kedaulatan SPBE Republik Indonesia.*
