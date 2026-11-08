# Lampiran D: Kunci Jawaban Lengkap 120 Soal Esai & 400 Soal Pilihan Ganda

Lampiran ini menyajikan pembahasan analitis komprehensif untuk seluruh 120 soal latihan esai (3 soal per bab) serta kunci jawaban dan pembahasan ilmiah untuk 400 butir soal pilihan ganda (10 butir per bab) dari Bab 1 hingga Bab 40 buku **WidyaMobile: Rekayasa Aplikasi Seluler Native, Perangkat Keras Pintar & Sistem Tertanam Berdaulat**.

---

## Pembahasan Soal Evaluasi Bab 1

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
WidyaMobile mengeliminasi ketergantungan pada Android Studio, Gradle, dan Xcode dengan mengimplementasikan generator biner in-house langsung di dalam kompiler Rust (pure_apk.rs dan pure_ipa.rs). Sistem ini secara mandiri menghasilkan berkas Android Binary XML (AXML), bytecode Dalvik classes.dex v035, struktur biner Mach-O 64-bit ARM64, Info.plist biner, tanda tangan v1/v2, serta kontainer ZIP PKWARE standar RFC 1951 tanpa memanggil utilitas eksternal apa pun.

**Soal 2:**
*Pembahasan Analitis:*
Jembatan serialisasi JSON asinkron pada arsitektur hybrid konvensional memicu latensi tinggi karena setiap event sentuh, frame grafis, dan pemanggilan sensor harus diserialkan ke teks string JSON, melintasi bus perantara, dan diurai ulang di sisi native. Pendekatan memori langsung WidyaMobile meniadakan proses serialisasi ini: kode Widya-Lang beroperasi langsung di atas struktur data memori native dan memanggil subsistem grafis Vulkan/Metal secara zero-copy.

**Soal 3:**
*Pembahasan Analitis:*
Ukuran biner lean binary di bawah 12 MB sangat krusial di wilayah dengan keterbatasan bandwidth internet karena mempercepat waktu unduh dari hitungan menit menjadi hitungan detik, menekan konsumsi kuota data seluler pengguna secara signifikan, dan meminimalkan angka pembatalan instalasi (install abandonment) pada perangkat berkapasitas memori penyimpanan internal rendah.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **B** | WidyaMobile menargetkan ukuran biner rilis lean binary di kisaran 8.5 hingga 12 MB. |
| **2** | **B** | Subsistem grafis Android WidyaMobile merender langsung ke SurfaceView dengan Vulkan Native. |
| **3** | **A** | Pada iOS, WidyaMobile memanfaatkan CAMetalLayer berbasis Metal API untuk rendering tanpa jeda. |
| **4** | **B** | Ketiadaan JVM memangkas konsumsi RAM idle menjadi < 25 MB dan meniadakan jeda Garbage Collector. |
| **5** | **B** | AuditKesiapanAplikasiMobile() adalah fungsi resmi untuk mengaudit kesiapan 10 pilar seluler. |
| **6** | **C** | Waktu cold boot tipikal WidyaMobile berkisar 16 hingga 28 milidetik berkat biner ARM64 murni. |
| **7** | **D** | WidyaMobile tidak membutuhkan Node.js, npm, maupun rantai JavaScript. |
| **8** | **B** | WidyaMobile mengendalikan perangkat keras langsung melalui API standar tanpa telemetri pihak ketiga. |
| **9** | **A** | Kompiler Pure APK membangkitkan bytecode classes.dex berstandar v035 secara mandiri. |
| **10** | **B** | Konsumsi memori kerja idle RAM tipikal aplikasi WidyaMobile berkisar 18 hingga 24 Megabyte. |

---

## Pembahasan Soal Evaluasi Bab 2

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Status Paused terjadi ketika jendela aplikasi masih terlihat sebagian (misalnya terhalang dialog sistem atau dalam mode split-screen), tetapi kehilangan fokus interaksi; utas UI masih dapat dialokasikan secara terbatas. Status Stopped terjadi ketika aplikasi sama sekali tidak terlihat di layar karena pengguna beralih ke aplikasi lain; utas UI dihentikan total dan proses menjadi target utama Low Memory Killer (LMK) bila terjadi tekanan memori.

**Soal 2:**
*Pembahasan Analitis:*
Pengembang tidak boleh mengandalkan event onDestroy() untuk menyimpan data transaksi karena ketika sistem operasi mengalami krisis alokasi memori mendadak, kernel Linux Android berhak langsung mengirimkan sinyal SIGKILL untuk mematikan proses seketika tanpa mengeksekusi siklus onDestroy(). Penyimpanan snapshot data wajib dilakukan pada event onPause() atau onStop().

**Soal 3:**
*Pembahasan Analitis:*
OOM Score Adjustment adalah mekanisme kernel Linux yang memberikan bobot numerik prioritas pembunuhan proses (0 untuk aplikasi latar depan aktif hingga 1000 untuk proses latar belakang tersembunyi). WidyaMobile melindungi integritas data pengguna dengan mendeteksi sinyal peringatan memori (onTrimMemory) dan secara otomatis mengenkripsi snapshot formulir transaksi ke brankas lokal sebelum OOM Score aplikasi diturunkan ke status rentan pembunuhan.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **B** | Subsistem kernel penghenti proses di Android adalah Low Memory Killer (LMK). |
| **2** | **C** | Pada status Resumed (Active Foreground), UI terlihat penuh dan menerima sentuhan 120/240Hz. |
| **3** | **B** | Batas waktu maksimal siklus transisi onPause() sebelum memicu ANR adalah 5.000 ms (5 detik). |
| **4** | **A** | Fungsi mobile_preservasi_state_pulihkan() memuat kembali snapshot state pasca-terminasi LMK. |
| **5** | **B** | Subsistem terminasi memori di Apple iOS dinamakan Jetsam Memory Pressure Termination. |
| **6** | **B** | Saat pengguna membuka aplikasi lain, status aplikasi berubah menjadi Stopped / Background. |
| **7** | **B** | onDestroy() tidak dijamin dipanggil karena kernel dapat langsung mengeksekusi sinyal SIGKILL. |
| **8** | **C** | mobile_preservasi_state_simpan menggunakan enkripsi standar industri AES-256. |
| **9** | **C** | Nilai OOM Score Adjustment untuk proses yang sedang di latar depan (Foreground) bernilai 0. |
| **10** | **B** | Restorasi state yang baik memulihkan kembali seluruh isian formulir dan posisi kursor pengguna. |

---

## Pembahasan Soal Evaluasi Bab 3

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Menempatkan tombol aksi krusial di luar batas Safe Area Insets bagian bawah dapat mengakibatkan tombol terpotong oleh sudut fisik layar membulat atau terhalang oleh garis indikator navigasi gestur sistem (Home Indicator Bar). Hal ini berisiko memicu salah sentuh di mana pengguna yang bermaksud menekan tombol transaksi justru memicu gestur sistem keluar aplikasi (swipe-to-home).

**Soal 2:**
*Pembahasan Analitis:*
Ketika perangkat diputar ke orientasi lanskap, kalkulasi Safe Area Insets menyesuaikan offset geometris secara dinamis: inset atas (top_notch_inset_px) yang tadinya menampung takik kamera depan kini berpindah menjadi kompensasi inset samping (left_px atau right_px) pada sisi layar tempat takik kamera fisik berada, sementara inset atas dan bawah menyusut ke nilai minimal.

**Soal 3:**
*Pembahasan Analitis:*
Pengembang WidyaMobile memanfaatkan parameter hinge_angle_degrees untuk mendeteksi status fisik ponsel lipat: jika sudut berada di antara 80° hingga 135° (Flex Mode), aplikasi secara otomatis membelah tata letak menjadi dua zona (zona atas untuk pratinjau kamera/grafik dan zona bawah untuk kontrol formulir/tombol transaksi), menyajikan pengalaman ergonomis tanpa penyangga ponsel.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **B** | Wilayah layar aman dari potongan takik dan bilah navigasi disebut Safe Area Inset. |
| **2** | **B** | Nilai tipikal bottom inset untuk bilah gestur iPhone adalah 34 piksel. |
| **3** | **A** | mobile_safe_area_insets() mengalkulasi inset piksel dan sudut engsel perangkat lipat. |
| **4** | **B** | Flex Mode pada perangkat layar lipat aktif pada sudut engsel antara 90° hingga 120°. |
| **5** | **B** | Mengabaikan top_notch_inset_px menyebabkan elemen tertutup fisik kamera atau Dynamic Island. |
| **6** | **A** | Pada orientasi lanskap, kompensasi inset samping (left_px dan right_px) otomatis bertambah. |
| **7** | **C** | CRT Electron Gun Shadow adalah teknologi tabung kuno, bukan gangguan layar modern. |
| **8** | **B** | Pada sudut engsel 180°, tata letak beradaptasi menjadi multi-kolom luas setara tablet. |
| **9** | **B** | Satuan pengukuran yang digunakan dalam kalkulasi inset adalah Density-independent Pixels (dp/pt). |
| **10** | **A** | WidyaMobile terikat langsung pada metrik native OS tanpa perlu parsing CSS media query. |

---

## Pembahasan Soal Evaluasi Bab 4

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Pendekatan memaksakan satu gaya visual ke seluruh platform menurunkan kepuasan pengguna karena melanggar konvensi mental navigasi pengguna masing-masing sistem operasi. Pengguna Android terbiasa dengan tombol aksi mengambang (FAB) dan riak sentuh Material You, sedangkan pengguna iOS mengharapkan navigasi berbasis frosted glass blur, tombol kapsul bawah, dan gestur geser tepi layar Cupertino.

**Soal 2:**
*Pembahasan Analitis:*
Mesin adaptif mobile_widget_adaptif memisahkan logika deklaratif murni (struktur hirarki dan penangan aksi) dari pipeline perenderan visual. Saat dijalankan pada Android, runtime memetakan widget ke shader Material 3 dengan kurva sudut 28px dan warna tonal dinamis; saat dijalankan pada iOS, runtime memetakan widget yang sama ke shader Gaussian blur 20pt dengan kurva pegas Cupertino.

**Soal 3:**
*Pembahasan Analitis:*
Tonal Elevation pada Material 3 menciptakan kedalaman visual melalui pemisahan tingkat kecerahan tonalitas warna permukaan (Surface Container Levels) tanpa mengandalkan bayangan hitam pekat, sedangkan Frosted Glass Blur pada Cupertino menciptakan kedalaman melalui peredaman transparansi latar belakang dinamis (Gaussian blur) yang menyerap warna konten di belakangnya secara tembus pandang.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **B** | Bahasa desain standar sistem operasi Android modern adalah Material You (Material 3). |
| **2** | **A** | Ciri visual utama komponen Cupertino iOS adalah efek kaca buram (Frosted Glass / Vibrant Blur). |
| **3** | **A** | mobile_widget_adaptif() mengaktifkan pemetaan widget otomatis lintas platform. |
| **4** | **A** | Material 3 mengekstraksi palet warna dinamis langsung dari gambar wallpaper pengguna. |
| **5** | **B** | Bentuk baku Floating Action Button (FAB) pada Material 3 adalah kotak bersudut tumpul lembut. |
| **6** | **B** | Umpan balik sentuh iOS Cupertino mengombinasikan penurunan opasitas dan fisika pegas. |
| **7** | **B** | Tipografi standar bawaan platform Apple iOS adalah San Francisco (SF Pro). |
| **8** | **A** | Komponen daftar pilihan tab tersegmentasi pada platform iOS dinamakan Segmented Control. |
| **9** | **B** | Radius sudut (corner radius) kartu Material 3 berkisar antara 24 hingga 28 piksel. |
| **10** | **A** | Pendekatan adaptif memungkinkan pengembang menulis satu deklarasi UI yang dipetakan ke gaya OS. |

---

## Pembahasan Soal Evaluasi Bab 5

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Gesture Arena menyelesaikan konflik gestur diagonal dengan menganalisis vektor kecepatan awal dan sudut pergerakan jari: jika pergeseran awal dominan pada sumbu vertikal (> 45° terhadap horizontal), arena memenangkan pengguliran daftar vertikal dan mengunci pergerakan horizontal; sebaliknya, jika pergeseran dominan horizontal, arena memenangkan sapuan kartu.

**Soal 2:**
*Pembahasan Analitis:*
Touch sampling rate 240Hz membaca posisi jari setiap 4.16 milidetik, dua kali lebih cepat dibandingkan laju penyegaran layar 120Hz (8.33 ms). Hal ini memungkinkan mesin gestur memprediksi kurva pergerakan jari secara sub-piksel, mengeliminasi efek keterlambatan visual (input lag), dan menjamin luncuran inersia terasa menempel persis di bawah ujung jari.

**Soal 3:**
*Pembahasan Analitis:*
Inersia fisika pegas memodelkan deselerasi luncuran menggunakan persamaan osilator teredam di mana percepatan balik berbanding lurus dengan jarak simpangan dan laju pelambatan diatur oleh koefisien redaman (0.85). Ketika jari dilepaskan, kecepatan akhir dikalikan peluruhan eksponensial halus hingga kecepatan mencapai nol tanpa berhenti mendadak.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **C** | Mesin gestur WidyaMobile mendukung laju pengambilan sampel sentuh hingga 240 Hertz. |
| **2** | **B** | Penekanan layar kontinu > 500 ms tanpa pergeseran koordinat diklasifikasikan sebagai Long Press. |
| **3** | **A** | Mekanisme penengah konflik gestur di layar disebut Gesture Arena (Touch Arbitrator). |
| **4** | **B** | Nilai koefisien inersia fisika pegas baku pada konfigurasi WidyaMobile adalah 0.85. |
| **5** | **B** | Jarak toleransi pergeseran maksimal sebelum ketukan dibatalkan (slop distance) adalah 8 piksel. |
| **6** | **A** | Gestur multisentuh dua jari untuk mengubah skala tampilan dokumen adalah Pinch to Zoom. |
| **7** | **A** | Nol alokasi heap saat event sentuh mencegah jeda akibat Garbage Collection (GC stutter). |
| **8** | **A** | mobile_gesture_engine_konfigurasi() adalah fungsi resmi pengatur parameter gestur sentuh. |
| **9** | **B** | Ambang batas waktu maksimal antar-ketukan untuk Double Tap adalah di bawah 300 milidetik. |
| **10** | **B** | Deselerasi fisika pegas saat sapuan jari menghasilkan luncuran eksponensial yang alami. |

---

## Pembahasan Soal Evaluasi Bab 6

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Stack Navigation mengelola halaman secara hierarkis dalam tumpukan LIFO di mana halaman lama tetap tertahan di bawah memori halaman baru untuk alur bertahap (seperti checkout), sedangkan Bottom Tab Navigation memelihara beberapa halaman utama sejajar secara persisten dalam memori tanpa menghancurkan posisi kursor atau status pengguliran saat pengguna berpindah tab.

**Soal 2:**
*Pembahasan Analitis:*
Universal Links (iOS) dan App Links (Android) memverifikasi kepemilikan domain melalui berkas manifestasi digital bertanda tangan kriptografis yang diunggah di peladen web resmi (`.well-known/assetlinks.json` di Android dan `apple-app-site-association` di iOS). Sistem operasi memverifikasi kesesuaian tanda tangan biner aplikasi dengan domain tersebut sebelum membuka rute.

**Soal 3:**
*Pembahasan Analitis:*
Predictive Back Gesture memerlukan koordinasi erat antara runtime dan OS karena saat pengguna mulai menggeser tepi layar, runtime harus merender pratinjau halaman sebelumnya di bawah halaman aktif secara real-time selaras dengan persentase pergeseran jari, serta membatalkan atau menyelesaikan transisi tergantung apakah pengguna melepaskan atau mengembalikan jarinya.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **B** | Struktur data navigasi hierarkis adalah tumpukan LIFO (Last In, First Out). |
| **2** | **B** | Operasi mendorong halaman baru ke atas tumpukan rute dinamakan Push. |
| **3** | **B** | Operasi menutup halaman teratas dan kembali ke layar sebelumnya dinamakan Pop. |
| **4** | **A** | mobile_universal_links_tangani() mengurai dan mengamankan rute tautan dalam (Deep Link). |
| **5** | **B** | Skema URL kustom aplikasi seluler berformat protokol khusus seperti nusantara://. |
| **6** | **A** | Bilah navigasi bawah (Bottom Navigation Bar) menampung menu utama di batas bawah layar. |
| **7** | **A** | Halaman di latar belakang tumpukan dilarang menjalankan animasi berat guna mencegah LMK. |
| **8** | **A** | Fitur pratinjau halaman saat menggeser tepi layar Android 14+ disebut Predictive Back Gesture. |
| **9** | **B** | Status kursor formulir disimpan pada state tree lokal tanpa membebani GPU saat pindah tab. |
| **10** | **A** | Latensi penguraian tautan Deep Link pada runtime native berkisar 0.4 milidetik. |

---

## Pembahasan Soal Evaluasi Bab 7

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Merender ribuan elemen daftar secara naif mengalokasikan ribuan objek widget grafis lengkap dengan simpul tekstur GPU ke dalam memori heap. Hal ini seketika memicu lonjakan penggunaan RAM hingga ratusan megabyte, menyebabkan Garbage Collector bekerja berlebihan, dan akhirnya memicu penghentian paksa aplikasi oleh Low Memory Killer (LMK).

**Soal 2:**
*Pembahasan Analitis:*
Cell Recycling memelihara jumlah sel konstan di memori (misalnya 12 sel). Ketika sel nomor 1 bergerak keluar melewati batas atas layar akibat pengguliran ke bawah, wadah objek grafis sel tersebut tidak dihancurkan, melainkan dipindahkan koordinat Y-nya ke batas bawah layar dan diisi ulang (rebind) dengan data baris nomor 13 secara zero-allocation.

**Soal 3:**
*Pembahasan Analitis:*
Strategi Prefetching pada Infinite Scroll memantau batas ambang pengguliran (misalnya saat pengguna mencapai 75% dari data yang ada di memori). Sistem secara proaktif meluncurkan kueri pengambilan batch data berikutnya di latar belakang sehingga saat pengguna tiba di baris terakhir, data baru telah siap tanpa memunculkan loading spinner.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Teknik optimasi yang hanya menggambar sel yang sedang tampak disebut Viewport Virtualization. |
| **2** | **B** | Sel yang keluar dari batas layar dimasukkan ke kolam daur ulang (recycle pool) untuk dipakai lagi. |
| **3** | **B** | Daftar virtual hanya memelihara 12 hingga 16 sel aktif di memori meskipun data berjumlah 100K. |
| **4** | **A** | ItemDaftarWidget() adalah komponen pembantu untuk menyusun baris daftar secara deklaratif. |
| **5** | **B** | Prefetching batch data berikutnya idealnya dimulai saat pengguliran mencapai 70% s/d 80%. |
| **6** | **B** | Rendering daftar naif pada 50.000 data memicu kehabisan memori RAM (Out of Memory Crash). |
| **7** | **A** | Judul kategori yang menempel di batas atas selama kategori aktif disebut Sticky Headers. |
| **8** | **A** | Gestur menarik layar dari atas ke bawah untuk memuat data baru disebut Pull-to-Refresh. |
| **9** | **B** | Waktu pengikatan ulang data (rebind) sel daur ulang WidyaMobile hanya sekitar 0.05 milidetik. |
| **10** | **A** | Daftar virtual menjamin pengguliran tetap lancar 120 FPS di ponsel kelas pemula (entry-level). |

---

## Pembahasan Soal Evaluasi Bab 8

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Batas waktu render pada layar 120Hz adalah 8.33 milidetik per frame (1 detik / 120). Jika komputasi tata letak dan penggambaran GPU melebihi 8.33 ms, subsistem V-Sync terpaksa melewatkan siklus tersebut dan menahan frame lama, menghasilkan fenomena frame drop atau jank yang terlihat patah-patah oleh mata manusia.

**Soal 2:**
*Pembahasan Analitis:*
Animasi fisika pegas terasa lebih alami karena mematuhi hukum dinamika gerak fisik nyata di mana percepatan dan deselerasi dipengaruhi oleh gaya dorong, elastisitas kekakuan, dan gesekan redaman, menyerupai interaksi benda di dunia nyata. Animasi linier statis terasa kaku dan artifisial karena kecepatan geraknya konstan dan berhenti mendadak tanpa inersia.

**Soal 3:**
*Pembahasan Analitis:*
Shared Element Transition bekerja dengan mencatat koordinat layar, dimensi piksel, dan radius sudut awal elemen (misalnya foto thumbnail kecil di katalog), lalu menginterpolasi matriks transformasi geometri tersebut secara mulus menuju koordinat dan ukuran target di halaman detail selama transisi rute berlangsung di bawah sinkronisasi V-Sync GPU.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **B** | Batas anggaran waktu render satu frame pada layar 120Hz adalah 8.33 milidetik. |
| **2** | **B** | Tiga parameter model pegas adalah Massa (Mass), Kekakuan (Stiffness), dan Redaman (Damping). |
| **3** | **A** | Fenomena hilangnya frame animasi yang menyebabkan gerakan tersendat dinamakan Jank. |
| **4** | **A** | Sinyal pulsa perangkat keras untuk sinkronisasi render frame baru adalah V-Sync. |
| **5** | **A** | Shared Element Transition memungkinkan elemen visual bertransformasi mulus antar-halaman. |
| **6** | **B** | Metal API adalah subsistem grafis native performa tinggi pada sistem operasi Apple iOS. |
| **7** | **A** | Vulkan API adalah subsistem grafis native dengan latensi terendah pada Android modern. |
| **8** | **A** | Perhitungan animasi diisolasi dari thread logika agar tidak tersendat saat komputasi data berat. |
| **9** | **B** | Sistem pegas redaman kritis mencapai target secepat mungkin tanpa memantul berlebihan. |
| **10** | **A** | Rata-rata frame rate animasi transisi lembar bawah WidyaMobile mencapai 119.98 FPS stabil. |

---

## Pembahasan Soal Evaluasi Bab 9

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Akselerometer membedakan kedua kondisi tersebut melalui analisis gaya gravitasi konstan bumi: saat ponsel diam di atas meja, vektor akselerometer mencatat percepatan tepat 1.0 G (9.81 m/s²) ke arah sumbu Z vertikal dengan variasi dinamis nol. Saat ponsel bergerak di dalam lift, terjadi lonjakan percepatan linear tambahan (positif saat lift mulai naik dan negatif saat mulai berhenti) yang terukur di atas nilai konstan 1.0 G.

**Soal 2:**
*Pembahasan Analitis:*
Sensor giroskop mengonsumsi arus baterai jauh lebih besar (3.5 mA) dibandingkan akselerometer (0.02 mA) karena giroskop MEMS memanfaatkan elemen massa mikro yang digetarkan secara terus-menerus menggunakan osilator frekuensi tinggi elektrostatik guna mendeteksi gaya Coriolis saat terjadi rotasi sudut, sedangkan akselerometer bekerja secara kapasitif pasif.

**Soal 3:**
*Pembahasan Analitis:*
Algoritma Man-Down Detection mendeteksi insiden pekerja jatuh melalui dua fase kinematika berturutan: fase pertama adalah deteksi jatuh bebas (Free Fall) di mana total magnitudo vektor percepatan anjlok mendekati 0 G (< 0.2 G) selama minimal 250 ms, diikuti fase kedua berupa lonjakan benturan keras (Impact Shock) di mana vektor percepatan melonjak drastis melampaui 3.5 G.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Satuan pengukuran baku sensor akselerometer adalah meter per detik kuadrat (m/s²). |
| **2** | **B** | Nilai percepatan gravitasi bumi saat diam mendatar adalah 9.81 m/s² (1 G) ke pusat bumi. |
| **3** | **A** | Giroskop berfungsi mengukur laju kecepatan sudut rotasi tiga sumbu ruang (Roll, Pitch, Yaw). |
| **4** | **B** | Satuan intensitas penerangan yang dihasilkan sensor cahaya ambien adalah Lux (Lumen/m²). |
| **5** | **A** | mobile_sensor_lengkap_baca() membaca seluruh larik sensor seluler secara terpadu. |
| **6** | **B** | Nilai vektor magnitudo mendekati 0 G menandakan perangkat sedang mengalami jatuh bebas (Free Fall). |
| **7** | **A** | Barometer mengukur tekanan atmosfer untuk mendeteksi perubahan elevasi vertikal lantai gedung. |
| **8** | **A** | Filter Kalman menggabungkan data akselerometer dan giroskop untuk menghasilkan orientasi stabil. |
| **9** | **A** | Sampling Rate Throttling menurunkan frekuensi pembacaan sensor untuk menghemat daya baterai. |
| **10** | **A** | Mode kontras tinggi luar ruangan direkomendasikan saat intensitas cahaya > 8.000 Lux. |

---

## Pembahasan Soal Evaluasi Bab 10

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Formulasi Haversine mengompensasi kelengkungan bumi dengan memodelkan bumi sebagai bola beradius 6.371 km. Rumus ini menghitung sudut pusat antara dua koordinat bola menggunakan selisih garis lintang dan bujur melalui fungsi sinus kuadrat separuh sudut, mengeliminasi distorsi jarak linier Pythagoras datar yang meleset pada jarak geografis jauh.

**Soal 2:**
*Pembahasan Analitis:*
Aplikasi presensi enterprise wajib memadukan sinyal GNSS satelit dengan pemindaian sinyal Wi-Fi BSSID sekitar dan menara BTS untuk memitigasi manipulasi lokasi (Mock Location / Fake GPS). Aplikasi palsu penyedia koordinat virtual hanya memalsukan API lokasi perangkat lunak, tetapi tidak dapat memalsukan keberadaan fisik alamat MAC pemancar Wi-Fi kantor.

**Soal 3:**
*Pembahasan Analitis:*
Adaptive Accuracy Switching menghemat baterai dengan memantau status aktivitas pengguna: jika sensor akselerometer mendeteksi pengguna sedang duduk diam di meja kantor, frekuensi penarikan lokasi GPS diturunkan ke mode pasif jaringan seluler (tiap 5–10 menit); saat terdeteksi pergerakan kendaraan, frekuensi dinaikkan secara otomatis ke satelit GNSS berdaya penuh.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Rumus Haversine menghitung jarak lingkaran besar terpendek pada permukaan bola bumi. |
| **2** | **B** | Nilai rata-rata radius bola bumi yang digunakan dalam perhitungan adalah 6.371 kilometer. |
| **3** | **A** | Assisted GPS (AGPS) mempercepat waktu penguncian satelit menggunakan jaringan seluler. |
| **4** | **A** | mobile_geofencing_atur() mendaftarkan perimeter batas virtual ke sistem operasi. |
| **5** | **B** | Transisi GEOFENCE_ENTER terpicu saat perangkat melangkah masuk ke dalam radius wilayah. |
| **6** | **A** | Pantulan sinyal satelit akibat dinding kaca gedung pencakar langit disebut Urban Multipath. |
| **7** | **A** | Modul GPS satelit terus-menerus dapat menguras daya baterai dalam 2 hingga 4 jam. |
| **8** | **B** | OS memantau geofence di perangkat keras berdaya rendah dan hanya membangunkan app saat batas lewat. |
| **9** | **A** | Izin ACCESS_FINE_LOCATION wajib didaftarkan untuk memperoleh koordinat satelit presisi. |
| **10** | **A** | Akurasi horizontal penerima GNSS ponsel di ruang terbuka berkisar 1.5 hingga 5 meter. |

---

## Pembahasan Soal Evaluasi Bab 11

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Pemrosesan aliran frame kamera pada bidang Y-plane (luminansi) format YUV jauh lebih efisien karena informasi intensitas cahaya dan kontras hitam-putih yang dibutuhkan algoritma pemindai barcode dan OCR telah tersedia lengkap di bidang Y tanpa memerlukan kalkulasi matriks konversi warna RGB yang memboroskan siklus CPU dan bandwidth memori RAM.

**Soal 2:**
*Pembahasan Analitis:*
Zero-Shutter Lag (ZSL) bekerja dengan memelihara buffer sirkular berisi frame-frame gambar resolusi penuh di memori perantara secara terus-menerus saat aplikasi kamera dibuka. Ketika pengguna menekan tombol rana, sistem tidak baru mulai mengambil foto, melainkan langsung mengambil frame yang telah terekam tepat pada saat tombol sentuh tertekan.

**Soal 3:**
*Pembahasan Analitis:*
Enkoder video akselerasi perangkat keras (Hardware MediaCodec/VideoToolbox) menggunakan sirkuit silikon khusus di dalam SoC yang didedikasikan murni untuk kompresi bitstream H.264/H.265. Penggunaan enkoder perangkat keras menghemat konsumsi daya hingga 80% dan mencegah lonjakan suhu prosesor dibandingkan enkoder perangkat lunak berbasis CPU.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Format warna native subsistem kamera seluler sebelum konversi adalah YUV (YUV_420_888). |
| **2** | **A** | Bidang Y dalam format YUV merepresentasikan intensitas kecerahan (luminance) citra. |
| **3** | **A** | Camera2 / CameraX adalah API kontrol kamera tingkat rendah native pada Android modern. |
| **4** | **A** | AmbilFotoKamera() mengambil satu frame foto resolusi tinggi dan mengembalikan URI berkas. |
| **5** | **A** | H.265 / HEVC adalah standar kompresi video efisiensi tinggi terakselerasi perangkat keras. |
| **6** | **A** | Fitur Tap-to-Focus menyesuaikan fokus lensa mengikuti koordinat sentuhan jari di layar. |
| **7** | **A** | Alokasi Bitmap Java di setiap frame memicu lonjakan memori dan jeda berat Garbage Collection. |
| **8** | **A** | Izin CAMERA wajib disetujui pengguna sebelum aplikasi dapat membuka sensor kamera. |
| **9** | **A** | Wadah berkas standar perekaman video seluler adalah format MP4 (MPEG-4 Part 14). |
| **10** | **A** | Latensi rana (shutter lag) kamera native WidyaMobile hanya sekitar 18 milidetik. |

---

## Pembahasan Soal Evaluasi Bab 12

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Format Type-Length-Value (TLV) menyusun data ke dalam tiga komponen berurutan: Tag (2 digit pengenal data), Length (2 digit panjang karakter isi), dan Value (isi data sebenarnya). Sebagai contoh, Tag 54 dengan nominal Rp 75.000 direpresentasikan sebagai `540575000` di mana `54` adalah tag nominal, `05` adalah panjang 5 karakter, dan `75000` adalah nilai nominal transaksi.

**Soal 2:**
*Pembahasan Analitis:*
Verifikasi CRC-16 CCITT mutlak wajib dieksekusi sebelum menampilkan nominal ke pengguna guna mendeteksi kerusakan data akibat distorsi optik kamera atau manipulasi sengaja oleh penyerang. Jika nilai checksum Tag 63 tidak cocok persis dengan hasil perhitungan hash string payload, data tidak boleh diproses karena nominal transaksi berisiko telah diubah.

**Soal 3:**
*Pembahasan Analitis:*
QRIS Statis (Tipe 11) memuat kode identitas toko tanpa nominal transaksi, sehingga aplikasi kasir atau pelanggan wajib memasukkan nominal belanjaan secara manual di formulir input. QRIS Dinamis (Tipe 12) diterbitkan per-transaksi oleh mesin kasir dan memuat nominal spesifik pada Tag 54, sehingga aplikasi langsung mengunci angka pembayaran tanpa risiko salah ketik nominal.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | ASPI bersama Bank Indonesia merumuskan dan mengatur standarisasi pembayaran QRIS nasional. |
| **2** | **B** | Payload QRIS EMVCo menggunakan struktur data berformat Type-Length-Value (TLV). |
| **3** | **B** | Tag 59 pada QRIS EMVCo memuat identitas nama resmi pedagang (Merchant Name). |
| **4** | **B** | Kode numerik ISO 4217 untuk mata uang Rupiah Indonesia (Tag 53) adalah 360. |
| **5** | **A** | UraiKodeQRIS() membedah string mentah QRIS dan memvalidasi keaslian checksum. |
| **6** | **A** | Tag 63 memuat 4 karakter heksadesimal kode verifikasi siklik integritas CRC-16 CCITT. |
| **7** | **A** | Koreksi kesalahan Reed-Solomon memungkinkan kode QR tetap terbaca meski tergores 20%. |
| **8** | **A** | Pada QRIS dinamis (Tag 01 bernilai 12), nominal transaksi tertera langsung di Tag 54. |
| **9** | **A** | Jika checksum CRC-16 tidak cocok, transaksi wajib seketika ditolak karena indikasi sabotase. |
| **10** | **A** | Pemindai native WidyaMobile mendeteksi dan mengurai kode QRIS dalam waktu 6.4 milidetik. |

---

## Pembahasan Soal Evaluasi Bab 13

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Doze Mode menonaktifkan akses jaringan, menunda eksekusi pekerjaan latar belakang (JobScheduler), mengabaikan sinkronisasi sinkron, dan menghentikan penahanan CPU (wakelocks) saat perangkat tidak diisi daya, berada dalam kondisi diam tidak bergerak, dan layarnya mati. Tujuannya adalah memperpanjang masa siaga baterai ponsel dari hitungan jam menjadi berhari-hari.

**Soal 2:**
*Pembahasan Analitis:*
Suhu baterai di atas 42°C berbahaya karena mempercepat degradasi elektroda kimia sel Lithium-Ion dan meningkatkan risiko fenomena thermal runaway. Thermal Throttling melindungi integritas fisik ponsel dengan menurunkan laju frekuensi kerja clock CPU/GPU, meredupkan kecerahan layar, dan mematikan pengisian daya cepat hingga suhu turun ke batas aman.

**Soal 3:**
*Pembahasan Analitis:*
App Standby Buckets mengelompokkan aplikasi ke dalam 5 tingkatan: Active (tanpa batasan job), Working Set (job tiap 2 jam), Frequent (job tiap 8 jam), Rare (job tiap 24 jam), dan Restricted (job diblokir total). Sistem membatasi kuota eksekusi dan frekuensi pesan push FCM sebanding dengan seberapa sering pengguna berinteraksi dengan aplikasi tersebut.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Doze Mode membatasi akses CPU dan jaringan saat ponsel diam tidak diisi daya. |
| **2** | **B** | Thermal Throttling dipicu sistem operasi saat suhu sel baterai melampaui 42°C hingga 45°C. |
| **3** | **A** | PeriksaStatusBaterai() memeriksa persentase, status pengisian, dan temperatur baterai. |
| **4** | **A** | App Standby Buckets mengelompokkan aplikasi berdasarkan frekuensi pemakaian pengguna. |
| **5** | **A** | Aplikasi yang sedang aktif dibuka di latar depan ditempatkan pada Active Bucket. |
| **6** | **A** | Partial Wakelock mencegah CPU tidur lelap sehingga menguras daya baterai di latar belakang. |
| **7** | **A** | Saat baterai < 20%, aplikasi idealnya menurunkan refresh rate dan menunda sinkronisasi berat. |
| **8** | **A** | Satuan suhu telemetri baterai yang dihasilkan WidyaMobile adalah derajat Celsius (°C). |
| **9** | **A** | Konsumsi daya siaga aplikasi WidyaMobile selama 8 jam berada di bawah 0.5% baterai. |
| **10** | **A** | Arsitektur Zero-JVM meniadakan siklus Garbage Collection yang membangunkan CPU dari tidur. |

---

## Pembahasan Soal Evaluasi Bab 14

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Motor getar kuno ERM menghasilkan getaran melalui putaran mekanis poros bermassa eksentrik tidak seimbang menggunakan motor DC, membutuhkan waktu akselerasi dan pengereman yang lambat (40–80 ms) sehingga getaran terasa mendengung kasar. Aktuator LRA menggunakan kumparan elektromagnetik yang menggerakkan massa bolak-balik secara linier pada pegas resonansi, menghasilkan respons instan (< 5 ms) yang tajam.

**Soal 2:**
*Pembahasan Analitis:*
Apple Taptic Engine menghasilkan sensasi klik mekanis yang sangat meyakinkan karena aktuator mampu menghasilkan gaya akselerasi tinggi dalam jendela waktu ultra-singkat (durasi 3–5 milidetik) yang dihentikan secara aktif dengan pulsa pengereman berlawanan arah, memicu reseptor taktil kulit Pacinian corpuscles di jari tanpa menimbulkan dengungan sekunder.

**Soal 3:**
*Pembahasan Analitis:*
Pedoman desain UX membatasi pemanggilan haptik hanya pada peristiwa interaksi yang bermakna (seperti konfirmasi tombol penting, penyelesaian transaksi, atau peringatan galat) dan menghindari pemicuan haptik pada setiap siklus pengguliran piksel biasa. Pemanggilan haptik yang terlalu sering memicu kelelahan sensorik (haptic fatigue), kejengkelan pengguna, dan pemborosan daya baterai.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Linear Resonant Actuator (LRA) adalah aktuator getar resonan linier presisi tinggi modern. |
| **2** | **A** | Taptic Engine adalah nama dagang aktuator haptik presisi tinggi pada iPhone Apple. |
| **3** | **A** | GetarHaptik() adalah fungsi resmi untuk memicu pola getaran taktil aktuator. |
| **4** | **A** | Parameter fungsi GetarHaptik berupa larik durasi milidetik getar dan jeda hening. |
| **5** | **A** | Taptic Engine membutuhkan waktu akselerasi puncak kurang dari 5 milidetik. |
| **6** | **A** | Pola konfirmasi sukses ideal berupa dua hingga tiga denyutan harmonis yang tegas. |
| **7** | **A** | Motor getar kuno ERM berdengung kasar dan lamban berhenti akibat inersia mekanis putar. |
| **8** | **A** | Izin android.permission.VIBRATE wajib disertakan untuk mengendalikan motor getar. |
| **9** | **A** | Pemanggilan haptik berlebihan memicu kelelahan sensorik (haptic fatigue) dan boros baterai. |
| **10** | **A** | Durasi denyutan mikro (light tick) pada kontrol virtual berkisar antara 15 hingga 30 ms. |

---

## Pembahasan Soal Evaluasi Bab 15

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Opus (RFC 6716) mengungguli MP3 karena dirancang khusus untuk transmisi suara real-time dengan latensi algoritmik ultra-rendah (5–26.5 ms berbanding 100+ ms pada MP3) dan efisiensi kompresi superior yang menghasilkan kejernihan suara vokal tinggi pada bitrate 16–24 kbps, menghemat ukuran berkas hingga 80% lebih kecil dibandingkan format MP3 128 kbps.

**Soal 2:**
*Pembahasan Analitis:*
Saat ada panggilan telepon masuk di tengah pemutaran audio kasir, sistem operasi memicu event hilangnya fokus audio (OnAudioFocusLoss). Aplikasi WidyaMobile secara otomatis menangkap sinyal ini untuk menghentikan sementara (pause) pemutaran audio kasir atau melakukan ducking (menurunkan volume) agar percakapan telepon darurat pengguna tidak terganggu oleh suara aplikasi.

**Soal 3:**
*Pembahasan Analitis:*
Acoustic Echo Cancellation (AEC) menggunakan filter adaptif untuk memodelkan jalur suara dari speaker ponsel kembali ke mikrofon dan secara matematis mengurangkan sinyal speaker tersebut dari sinyal mikrofon, mencegah lawan bicara mendengar suara gema mereka sendiri. Noise Suppression memfilter spektrum bising konstan (seperti deru AC atau keramaian restoran) menggunakan analisis frekuensi spectral subtraction.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Opus (RFC 6716) adalah codec audio modern efisiensi tinggi berstandar terbuka IETF. |
| **2** | **A** | Ukuran rekaman suara 1 menit berformat Opus pada bitrate 24 kbps hanya sekitar 180 KB. |
| **3** | **A** | Audio Focus Management mengatur hak pemutaran suara antar-aplikasi di sistem operasi. |
| **4** | **A** | Audio Ducking adalah penurunan volume otomatis audio latar saat notifikasi bersuara diputar. |
| **5** | **A** | Frekuensi pengambilan sampel standar studio untuk merekam suara vokal adalah 48.000 Hz. |
| **6** | **A** | Acoustic Echo Cancellation (AEC) menghilangkan pantulan gema speaker dari mikrofon. |
| **7** | **A** | Izin android.permission.RECORD_AUDIO wajib disetujui pengguna untuk merekam audio. |
| **8** | **A** | AAudio / Oboe adalah subsistem audio native latensi rendah pada Android modern. |
| **9** | **A** | Format WAV Linear PCM mentah tidak disarankan karena ukurannya memboroskan kuota (>10MB/menit). |
| **10** | **A** | Latensi algoritmik minimum yang mampu dicapai oleh enkoder Opus adalah sekitar 5 milidetik. |

---

## Pembahasan Soal Evaluasi Bab 16

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Skema `file://` dilarang sejak Android 7.0 karena mengekspos jalur absolut sistem berkas privat perangkat dan memaksa aplikasi target meminta izin penyimpanan global yang berbahaya (`FileUriExposedException`). Skema `content://` via FileProvider memecahkan masalah ini dengan menerbitkan URI virtual terenkripsi yang memberikan izin akses baca sementara (`FLAG_GRANT_READ_URI_PERMISSION`) hanya untuk berkas target tanpa membuka direktori lain.

**Soal 2:**
*Pembahasan Analitis:*
Penghapusan otomatis papan klip (auto-clearing clipboard) dalam batas waktu 60 detik sangat penting karena papan klip sistem operasi dapat dibaca oleh aplikasi lain yang berjalan di latar belakang. Jika pengguna menyalin nomor PIN, kode OTP, atau nomor rekening bank dan lupa menimpanya, data sensitif tersebut dapat dicuri oleh aplikasi pencatat klip jahat (clipboard snooper).

**Soal 3:**
*Pembahasan Analitis:*
Native Share Sheet beroperasi di tingkat sistem operasi di luar kotak pasir aplikasi pemanggil. Sistem operasi mengelola sendiri daftar aplikasi terpasang dan kontak sering dihubungi pengguna berdasarkan kebiasaan pribadi mereka, sehingga aplikasi pengirim cukup menyerahkan payload berkas ke sistem operasi tanpa perlu meminta izin membaca seluruh daftar paket aplikasi di ponsel (`QUERY_ALL_PACKAGES`).

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Native Share Sheet menyajikan dialog berbagi sistem ke aplikasi lain yang terpasang. |
| **2** | **A** | Skema content:// via FileProvider adalah mekanisme berbagi berkas aman berstandar modern. |
| **3** | **A** | BagikanKonten() memicu lembar dialog berbagi bawaan sistem operasi seluler. |
| **4** | **A** | Auto-Expiring Clipboard menghapus data sensitif secara otomatis setelah batas waktu berlalu. |
| **5** | **A** | Batas waktu kedaluwarsa baku papan klip sensitif WidyaMobile adalah 60 detik. |
| **6** | **A** | UIActivityViewController adalah komponen antarmuka berbagi native pada Apple iOS. |
| **7** | **A** | SalinKePapanKlip() menyalin teks string ke papan klip sistem operasi secara terproteksi. |
| **8** | **A** | FileUriExposedException terjadi di Android jika aplikasi memaksakan penggunaan skema file://. |
| **9** | **A** | Izin FLAG_GRANT_READ_URI_PERMISSION memberikan hak baca berkas sementara ke aplikasi target. |
| **10** | **A** | Native Share Sheet mempersonalisasi daftar kontak dan aplikasi sering dihubungi pengguna. |

---

## Pembahasan Soal Evaluasi Bab 17

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
HTTP/3 berbasis UDP/QUIC mengeliminasi Head-of-Line Blocking karena setiap permintaan data dikelola sebagai aliran (stream) biner independen di dalam koneksi tunggal. Jika sebuah paket data hilang di jaringan seluler, hanya aliran yang kehilangan paket tersebut yang menunggu transmisi ulang, sementara aliran data transaksi lainnya tetap meluncur tanpa terhenti, berbeda dengan TCP yang membekukan seluruh koneksi.

**Soal 2:**
*Pembahasan Analitis:*
Connection ID (CID) pada protokol QUIC adalah pengenal koneksi acak 64-bit yang independen dari alamat IP atau port perangkat. Ketika ponsel pengguna berpindah dari Wi-Fi rumah ke jaringan seluler 5G di jalan raya, alamat IP perangkat berubah drastis, namun koneksi QUIC tetap bertahan tanpa perlu melakukan jabat tangan TLS ulang karena kedua belah pihak tetap mengenali Connection ID yang sama.

**Soal 3:**
*Pembahasan Analitis:*
Pola Circuit Breaker memantau kegagalan koneksi: pada status Closed, permintaan mengalir normal. Jika rasio kegagalan melampaui ambang batas, sirkuit berpindah ke status Open dan seketika menolak permintaan baru tanpa membebani server yang sedang tumbang. Setelah waktu jeda, sirkuit beralih ke Half-Open untuk mengirimkan satu paket uji coba; jika berhasil, sirkuit menutup kembali ke kondisi normal.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | QUIC yang berjalan di atas protokol UDP adalah fondasi transport modern HTTP/3. |
| **2** | **A** | Head-of-Line Blocking pada TCP menghentikan seluruh aliran data saat ada paket yang hilang. |
| **3** | **A** | PeriksaKoneksiJaringan() memeriksa ketersediaan internet dan tipe antarmuka jaringan. |
| **4** | **A** | Fitur 0-RTT Connection Resumption pada QUIC memungkinkan pengiriman data pada paket pertama. |
| **5** | **A** | Koneksi internet seluler berbayar kuota terukur diklasifikasikan sebagai Metered Network. |
| **6** | **A** | Circuit Breaker memutus sementara aliran permintaan ke server saat terdeteksi rangkaian galat. |
| **7** | **A** | Idempotency Key menjamin sebuah aksi transaksi finansial tidak diproses ganda di backend. |
| **8** | **A** | Exponential Backoff melipatgandakan jeda waktu rekoneksi setiap kali upaya sambungan gagal. |
| **9** | **A** | QPACK adalah format kompresi header teroptimasi yang digunakan pada standar HTTP/3. |
| **10** | **A** | Resilient WebSocket menerima notifikasi pembayaran kasir seketika dalam hitungan milidetik. |

---

## Pembahasan Soal Evaluasi Bab 18

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Peran BLE Central (ponsel pintar) bertindak sebagai pemindai aktif yang memprakarsai pembukaan koneksi data dan mengendalikan permintaan transfer, sedangkan BLE Peripheral (printer thermal POS atau sensor) bertindak sebagai pemancar paket iklan (advertising) dan penyedia layanan data (GATT Server) yang menerima instruksi cetak dari Central.

**Soal 2:**
*Pembahasan Analitis:*
Negosiasi MTU dari 23 byte menjadi 512 byte sangat penting karena batas standar 23 byte hanya menyisakan 20 byte payload efektif per paket transmisi setelah dikurangi header protokol. Mengirimkan berkas grafis struk belanja sebesar 10 Kilobyte pada MTU standar membutuhkan lebih dari 500 paket transfer, sedangkan pada MTU 512 byte hanya membutuhkan 20 paket transfer, memangkas waktu cetak hingga 90%.

**Soal 3:**
*Pembahasan Analitis:*
Penyaringan berbasis RSSI (Received Signal Strength Indicator) mengukur daya sinyal radio yang diterima ponsel: sinyal printer yang berada di atas meja kasir (< 1 meter) memiliki nilai RSSI kuat (di atas -65 dBm), sedangkan printer di toko tetangga yang berjarak 10 meter memiliki nilai RSSI lemah (di bawah -85 dBm). Aplikasi menetapkan batas ambang minimal -65 dBm agar pemasangan periferal otomatis hanya terjadi pada printer meja kerja kasir itu sendiri.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | GATT (Generic Attribute Profile) mengatur struktur hierarki data Bluetooth Low Energy. |
| **2** | **A** | RSSI (Received Signal Strength Indicator) mengukur kekuatan sinyal radio dalam -dBm. |
| **3** | **A** | PindaiPerangkatBluetooth() memindai perangkat BLE sekitar dan mengembalikan data periferal. |
| **4** | **A** | UUID 128-bit adalah pengidentifikasi unik universal untuk layanan dan karakteristik GATT. |
| **5** | **A** | Ukuran paket data bawaan baku (default MTU) sebelum negosiasi pada BLE adalah 23 byte. |
| **6** | **A** | Alamat MAC adalah pengenal perangkat keras fisik kartu jaringan sepanjang 6 oktet heksadesimal. |
| **7** | **A** | Karakteristik Notify/Indicate memungkinkan periferal mengirim data asinkron ke ponsel. |
| **8** | **A** | Izin android.permission.BLUETOOTH_SCAN wajib disertakan untuk memindai perangkat BLE. |
| **9** | **A** | Nilai RSSI lebih kuat dari -65 dBm menandakan periferal berada sangat dekat (< 1 meter). |
| **10** | **A** | BLE 5.3 beroperasi pada arus mikroampere sehingga periferal dapat menyala berbulan-bulan. |

---

## Pembahasan Soal Evaluasi Bab 19

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Kepala pemanas (thermal print head) memiliki larik elemen pemanas mikro linier (dot matrix) yang dialiri pulsa listrik singkat secara terprogram. Ketika kertas thermal yang dilapisi bahan kimia leuco dye dan zat pengembang asam melintasi kepala pemanas, panas lokal memicu reaksi kimia pencampuran kedua zat tersebut sehingga menghasilkan warna hitam pekat seketika tanpa membutuhkan tetesan tinta cair maupun pita karbon.

**Soal 2:**
*Pembahasan Analitis:*
Standarisasi lebar karakter (32 karakter pada kertas 58mm dan 48 karakter pada kertas 80mm) sangat krusial untuk memastikan keterbacaan struk belanja. Layout struk menuntut nama produk rata kiri dan harga belanjaan rata kanan dalam satu baris horizontal; jika jumlah karakter tidak distandarisasi, perhitungan spasi pemisah akan meleset dan menyebabkan angka harga terpotong atau jatuh ke baris berikutnya secara berantakan.

**Soal 3:**
*Pembahasan Analitis:*
Teknik Chunked Streaming memecah paket biner cetak menjadi potongan-potongan berukuran 200 hingga 500 byte dan menyisipkan jeda waktu mikro di antara pengiriman setiap potongan. Hal ini mencegah terjadinya luapan buffer (buffer overflow) pada mikrokontroler printer Bluetooth portabel yang umumnya hanya memiliki memori penyangga internal sebesar 2 hingga 4 Kilobyte.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | ESC/POS adalah bahasa instruksi universal untuk mengendalikan printer kasir thermal. |
| **2** | **A** | Lebar fisik kertas printer kasir portabel yang paling umum digunakan UMKM adalah 58 milimeter. |
| **3** | **A** | Jumlah karakter per baris baku pada kertas thermal 58mm menggunakan Font A adalah 32 karakter. |
| **4** | **A** | Kode heksadesimal 1B 40 (ESC @) menginisialisasi dan mereset printer ke kondisi awal. |
| **5** | **A** | CetakStrukThermalPOS() menyusun dan mencetak struk transaksi kasir secara instan. |
| **6** | **A** | Floyd-Steinberg Dithering mengonversi logo warna menjadi bitmap monokrom hitam-putih tajam. |
| **7** | **A** | Perintah GS V (1D 56) memotong kertas struk secara otomatis pada printer berpisau cutter. |
| **8** | **A** | Chunking transmisi 200-500 byte mencegah buffer overflow pada memori RAM printer perifer. |
| **9** | **A** | Kertas thermal memiliki lapisan zat kimia peka panas yang berubah warna gelap saat dipanaskan. |
| **10** | **A** | Waktu transmisi struk belanja 35 baris ke printer thermal BLE modern adalah < 0.5 detik. |

---

## Pembahasan Soal Evaluasi Bab 20

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Kartu pintar nirsentuh pasif tidak membutuhkan baterai karena memanfaatkan prinsip kopling induktif medan elektromagnetik frekuensi 13.56 MHz yang dipancarkan kumparan antena NFC ponsel. Medan frekuensi radio bolak-balik ini menginduksi arus listrik kecil pada antena kumparan di dalam kartu pintar, yang kemudian disearahkan oleh sirkuit power harvesting internal kartu untuk memberi daya pada mikrokontroler dan chip kriptografi kartu.

**Soal 2:**
*Pembahasan Analitis:*
Struktur APDU berstandar ISO/IEC 7816-4 terdiri dari dua jenis: Command APDU berformat `CLA INS P1 P2 Lc Data Le` (di mana CLA adalah kelas instruksi, INS adalah kode perintah operasi, P1/P2 adalah parameter referensi, Lc adalah panjang data kirim, Data adalah muatan biner, dan Le adalah panjang respon yang diharapkan) serta Response APDU berformat `Data + SW1 SW2` (di mana SW1/SW2 adalah status kata respon heksadesimal).

**Soal 3:**
*Pembahasan Analitis:*
Jarak fisik operasional yang sangat dekat (< 4 cm) memberikan keunggulan keamanan intrinsik karena secara fisik menuntut tindakan penempelan yang disengaja oleh pengguna (user intent), meminimalkan risiko serangan penyadapan gelombang radio jarak jauh (eavesdropping) dari pihak luar, serta mencegah pembajakan transaksi yang tidak disadari saat ponsel berada di dalam tas atau saku pakaian.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Frekuensi radio internasional yang digunakan oleh seluruh teknologi NFC adalah 13.56 MHz. |
| **2** | **A** | NDEF (NFC Data Exchange Format) adalah standar pertukaran data ringan pada tag nirkabel. |
| **3** | **A** | ISO/IEC 14443 adalah standar komunikasi internasional kartu pintar tanpa kontak perbankan. |
| **4** | **A** | APDU (Application Protocol Data Unit) adalah struktur format paket perintah biner kartu pintar. |
| **5** | **A** | Jarak operasional efektif pertukaran data NFC seluler adalah kurang dari 4 sentimeter. |
| **6** | **A** | Izin android.permission.NFC wajib disertakan pada berkas manifes untuk mengaktifkan NFC. |
| **7** | **A** | UID (Unique Identifier) adalah nomor seri unik perangkat keras chip kartu tanpa kontak. |
| **8** | **A** | Mode operasi di mana ponsel membaca data dari kartu pasif disebut Reader / Writer Mode. |
| **9** | **A** | Kartu e-Money ditenagai secara nirkabel oleh induksi medan elektromagnetik antena ponsel. |
| **10** | **A** | Waktu penguncian sinyal RF saat kartu didekatkan ke antena ponsel adalah sekitar 12 ms. |

---

## Pembahasan Soal Evaluasi Bab 21

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Teknologi SoftPOS (Tap on Mobile) memanfaatkan antena NFC bawaan ponsel komersial dan modul keamanan perangkat lunak terakreditasi untuk memproses transaksi kartu debit dan kredit nirsentuh tanpa memerlukan perangkat pembaca kartu fisik eksternal. Hal ini mendemokratisasi penerimaan pembayaran non-tunai bagi UMKM karena pedagang tidak perlu mengeluarkan biaya investasi atau sewa bulanan mesin EDC fisik.

**Soal 2:**
*Pembahasan Analitis:*
Standar PCI-CPoC mewajibkan pembersihan memori seketika (zeroization) data nomor kartu (PAN) dan CVV untuk memitigasi risiko pembajakan data melalui serangan dump memori RAM oleh malware tingkat lanjut. Data kartu hanya boleh berada di memori kerja selama beberapa milidetik saat pembentukan kriptogram ARQC, setelah itu blok memori wajib ditimpa dengan nilai biner nol agar tidak meninggalkan jejak forensik.

**Soal 3:**
*Pembahasan Analitis:*
Cardholder Verification Method (CVM) adalah mekanisme verifikasi pemegang kartu sah untuk mencegah penyalahgunaan kartu yang hilang. Pada transaksi bernilai kecil di bawah batas limit (No-CVM Limit Rp 1.000.000), transaksi disetujui instan tanpa verifikasi; pada transaksi bernilai besar di atas limit, sistem wajib meminta verifikasi tanda tangan digital atau memasukkan nomor PIN di layar ponsel pedagang (PIN on Glass).

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | PCI-CPoC mengatur standar keamanan penerimaan pembayaran nirsentuh pada ponsel komersial. |
| **2** | **A** | Kernel EMV Contactless Level 2 mengeksekusi logika verifikasi transaksi kartu perbankan. |
| **3** | **A** | ARQC (Application Cryptogram) adalah token kriptografis bukti keaslian transaksi finansial. |
| **4** | **A** | Scrambled PIN Keypad mengacak posisi tombol angka virtual saat meminta masukan PIN kasir. |
| **5** | **A** | mobile_nfc_softpos_mulai_transaksi() menginisialisasi penerimaan pembayaran kartu nirsentuh. |
| **6** | **A** | TEE / Secure Enclave adalah lingkungan perangkat keras terisolasi untuk data perbankan rahasia. |
| **7** | **A** | Ambang batas transaksi kartu nirsentuh tanpa PIN di Indonesia adalah Rp 1.000.000,00. |
| **8** | **A** | CDA membuktikan keaslian kartu menggunakan tanda tangan digital dinamis unik di tiap transaksi. |
| **9** | **A** | Deteksi rooting atau Frida seketika membatalkan transaksi SoftPOS dan menghapus memori. |
| **10** | **A** | SoftPOS meniadakan biaya sewa mesin EDC, biaya kertas thermal, dan perangkat keras ekstra. |

---

## Pembahasan Soal Evaluasi Bab 22

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
BLE Mesh memungkinkan komunikasi data tetap berjalan tanpa BTS karena membentuk topologi jaringan jala nirkabel terdesentralisasi di mana setiap ponsel pintar bertindak sebagai simpul pemancar dan simpul relai. Pesan darurat dipancarkan melalui gelombang radio Bluetooth antar-ponsel dan diteruskan melompat dari satu relawan ke relawan lainnya secara berantai (multi-hop) hingga tiba di posko komando.

**Soal 2:**
*Pembahasan Analitis:*
Managed Flooding with Sequence Cache mencegah badai siaran dengan cara mencatat nomor urut unik (Sequence Number) dan alamat pengirim setiap paket yang diterima ke dalam tabel cache sementara di memori ponsel. Jika ponsel menerima paket dengan nomor urut yang sudah pernah tercatat di cache, paket tersebut langsung dibuang dan tidak diteruskan kembali ke udara.

**Soal 3:**
*Pembahasan Analitis:*
BLE Mesh memisahkan keamanan menjadi dua lapisan kunci enkripsi: Kunci Jaringan (NetKey) digunakan pada Lapisan Jaringan untuk memverifikasi keanggotaan simpul relai di dalam kluster mesh dan mengenkripsi alamat broadcast, sedangkan Kunci Aplikasi (AppKey) digunakan pada Lapisan Aplikasi untuk mengenkripsi isi payload pesan sehingga simpul relai perantara hanya dapat meneruskan paket tanpa dapat membaca isi percakapan rahasia.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Bluetooth Mesh Profile Specification adalah standar jaringan nirkabel terdesentralisasi BLE. |
| **2** | **A** | Simpul Relai (Relay Node) bertugas meneruskan paket data dari simpul lain menuju target. |
| **3** | **A** | TTL (Time-To-Live) menyatakan sisa batas jumlah lompatan paket sebelum dihanguskan. |
| **4** | **A** | AES-CCM 128-bit adalah algoritma enkripsi simetris standar pada jaringan BLE Mesh. |
| **5** | **A** | mobile_ble_mesh_siarkan_pesan() menyiarkan pesan teks ke jaringan radio ad-hoc BLE Mesh. |
| **6** | **A** | Paket dengan nilai TTL = 0 segera dihentikan dari udara guna mencegah looping tak berujung. |
| **7** | **A** | Advertising Bearer (saluran 37, 38, 39) menyiarkan paket mesh tanpa proses pairing koneksi. |
| **8** | **A** | Jangkauan fisik transmisi langsung 1 hop antar-ponsel di ruang terbuka adalah 30 s/d 50 meter. |
| **9** | **A** | Application Key (AppKey) menjamin hanya aplikasi yang dituju yang dapat membaca isi pesan. |
| **10** | **A** | Waktu tempuh rata-rata pesan darurat melompati 3 simpul perantara (3 hops) adalah ~92 ms. |

---

## Pembahasan Soal Evaluasi Bab 23

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Android mewajibkan Notification Channels sejak Android 8.0 agar pengguna memiliki kendali privasi granular atas jenis notifikasi yang ingin mereka terima. Pengguna dapat memilih untuk menonaktifkan suara notifikasi promosi pemasaran tanpa kehilangan notifikasi transaksi finansial penting, dan sistem operasi dapat memberlakukan aturan prioritas serta perilaku visual yang berbeda untuk masing-masing saluran.

**Soal 2:**
*Pembahasan Analitis:*
Alarm biasa (inexact alarm) dikelompokkan oleh sistem operasi ke dalam batch window bersama alarm aplikasi lain guna meminimalkan frekuensi perangkat bangun dari Doze Mode, menyebabkan keterlambatan hingga 15 menit. Exact Alarm mengatasi hal ini dengan meminta izin khusus pada kernel untuk menjadwalkan pewaktu presisi RTC yang membangunkan CPU tepat pada detik yang ditentukan terlepas dari status tidur perangkat.

**Soal 3:**
*Pembahasan Analitis:*
Local Notification dihasilkan dan dikendalikan sepenuhnya oleh kode aplikasi di dalam ponsel menggunakan pewaktu sistem operasi lokal tanpa memerlukan sambungan internet atau peladen backend. Push Notification dikirimkan dari peladen cloud jarak jauh melintasi jaringan internet publik melalui infrastruktur gerbang pabrikan (APNs atau FCM) untuk memberitahukan peristiwa eksternal secara instan.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | AlarmManager adalah subsistem native Android untuk pemicuan tugas tepat waktu di level kernel. |
| **2** | **A** | UNUserNotificationCenter adalah subsistem native iOS untuk mengelola notifikasi lokal. |
| **3** | **A** | KirimNotifikasiLokal() menampilkan notifikasi instan pada saluran tertentu di baki sistem. |
| **4** | **A** | Heads-Up Notification Banner adalah tampilan notifikasi melayang di bagian atas layar. |
| **5** | **A** | Izin SCHEDULE_EXACT_ALARM wajib didaftarkan untuk menjadwalkan alarm presisi tepat waktu. |
| **6** | **A** | BatalkanNotifikasiLokal() membatalkan jadwal notifikasi lokal yang belum sempat terpicu. |
| **7** | **A** | Saluran dengan prioritas High/Urgent memicu banner melayang, dering suara, dan getaran. |
| **8** | **A** | Notifikasi lokal beroperasi 100% mandiri tanpa membutuhkan sambungan internet aktif. |
| **9** | **A** | Satuan waktu penundaan pada fungsi JadwalkanNotifikasiLokal adalah detik. |
| **10** | **A** | Deviasi waktu (jitter) alarm presisi native WidyaMobile berada di bawah 15 milidetik. |

---

## Pembahasan Soal Evaluasi Bab 24

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Alur penerbitan token push dimulai saat aplikasi memanggil API pendaftaran native; sistem operasi kemudian membuka koneksi aman ke gerbang pabrikan (APNs/FCM) untuk memvalidasi identitas biner aplikasi. Gerbang pabrikan menerbitkan string token pendaftaran acak 64-byte unik yang kemudian dikembalikan ke aplikasi seluler. Aplikasi selanjutnya mengirimkan token tersebut ke peladen backend WidyaBase untuk disimpan sebagai alamat pengiriman notifikasi.

**Soal 2:**
*Pembahasan Analitis:*
Enkripsi RFC 8291 (Message Encryption for Web Push) mutlak diperlukan karena paket push notification konvensional melintasi infrastruktur awan perantara pihak ketiga (Google FCM dan Apple APNs). Dengan enkripsi RFC 8291 berbasis pertukaran kunci kurva eliptik ECDH P-256 dan AES-128-GCM, isi pesan transaksi perbankan dienkripsi sejak dari peladen bank dan hanya dapat didekripsi oleh kunci privat di dalam ponsel nasabah, mencegah penyadapan oleh operator gateway.

**Soal 3:**
*Pembahasan Analitis:*
Silent Background Push adalah pesan dorong berprioritas data tanpa muatan teks banner visual yang dikirim dengan atribut khusus (`content-available: 1`). Saat pesan diterima, sistem operasi secara hening membangunkan aplikasi di latar belakang selama maksimal 30 detik untuk mengunduh pembaruan data terbaru (misalnya penyesuaian harga atau jadwal shift kerja), sehingga saat pengguna membuka aplikasi, seluruh data telah segar seketika.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | APNs (Apple Push Notification service) adalah gerbang push notification resmi untuk iOS. |
| **2** | **A** | FCM (Firebase Cloud Messaging) adalah gerbang push notification resmi untuk platform Android. |
| **3** | **A** | RFC 8291 adalah standar enkripsi ujung-ke-ujung internasional untuk muatan pesan push. |
| **4** | **A** | DapatkanTokenPushFCM() meminta dan mengembalikan token registrasi perangkat dari gateway OS. |
| **5** | **A** | Silent Background Push membangunkan aplikasi di latar belakang secara hening tanpa banner. |
| **6** | **A** | HTTP/2 over TLS adalah protokol transport streaming biner modern yang digunakan server APNs. |
| **7** | **A** | Notification Service Extension mendekripsi payload pesan sebelum digambar ke layar oleh iOS. |
| **8** | **A** | Izin POST_NOTIFICATIONS wajib disetujui pengguna Android 13+ untuk memunculkan notifikasi. |
| **9** | **A** | Parameter priority: high pada FCM memastikan pesan langsung membangunkan perangkat dari Doze. |
| **10** | **A** | Latensi pengiriman pesan push dari backend hingga tiba di layar via APNs adalah ~85 ms. |

---

## Pembahasan Soal Evaluasi Bab 25

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Enkripsi tingkat halaman (Page-Level Encryption) bekerja dengan memotong berkas basis data menjadi blok-blok sektor berukuran tepat 4.096 byte (4KB). Setiap kali halaman dibaca dari disk, sektor tersebut didekripsi di dalam memori penyangga (buffer pool) menggunakan kunci AES-256 dan vektor inisialisasi acak; setiap kali halaman dimodifikasi, sektor tersebut dienkripsi ulang sebelum ditulis ke disk fisik, menjamin data di disk selalu berupa ciphertext biner.

**Soal 2:**
*Pembahasan Analitis:*
Write-Ahead Logging (WAL) sangat disarankan karena memisahkan operasi pembacaan dan penulisan: kueri penulisan transaksi baru dicatat secara berurutan ke berkas log WAL terpisah tanpa menimpa berkas basis data utama. Hal ini memungkinkan kueri pembacaan inventaris berjalan secara bersamaan tanpa pernah terblokir oleh transaksi penulisan kasir, melipatgandakan konkurensi throughput I/O pada media flash seluler.

**Soal 3:**
*Pembahasan Analitis:*
Serangan ekstraksi forensik fisik dilakukan penyerang dengan menyalin seluruh partisi memori flash ponsel yang terkunci melalui antarmuka debug perangkat keras (JTAG atau bootloader exploit). Jika basis data disimpan dalam teks biasa (plaintext SQLite), seluruh data nomor kartu pelanggan, saldo kasir, dan riwayat transaksi dapat dibaca seketika menggunakan editor SQLite biasa di komputer penyerang.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | AES-256 (CBC/XTS) mengenkripsi setiap blok sektor halaman basis data pada WidyaBase Mobile. |
| **2** | **A** | Ukuran satu sektor halaman enkripsi basis data relasional seluler adalah 4.096 Byte (4 KB). |
| **3** | **A** | BukaBasisDataMobile() membuka berkas basis data terenkripsi lokal dengan kunci master. |
| **4** | **A** | Write-Ahead Logging (WAL) memungkinkan operasi baca dan tulis berjalan secara konkuren. |
| **5** | **A** | PBKDF2 meregangkan kata sandi teks menjadi kunci biner 256-bit berulang puluhan ribu kali. |
| **6** | **A** | HMAC-SHA512 memverifikasi integritas setiap halaman untuk mendeteksi sabotase berkas. |
| **7** | **A** | Kunci master tidak boleh ditulis statis di kode karena rentan dekompilasi reverse engineering. |
| **8** | **A** | EksekusiKueriMobile() mengeksekusi perintah modifikasi data (INSERT, UPDATE, DELETE). |
| **9** | **A** | Overhead latensi enkripsi AES-256 per halaman 4KB pada ARM64 adalah kurang dari 0.08 ms. |
| **10** | **A** | Basis data lokal terenkripsi memungkinkan kasir terus bekerja aman di daerah tanpa sinyal. |

---

## Pembahasan Soal Evaluasi Bab 26

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Pendekatan Online-First berasumsi koneksi internet selalu aktif; setiap aksi kasir langsung mengirim permintaan HTTP ke server dan antarmuka dibekukan menunggu respon, sehingga aplikasi gagal total saat sinyal hilang. Pendekatan Offline-First memperlakukan basis data lokal sebagai sumber kebenaran utama di mana transaksi kasir langsung diproses dan dicatat lokal seketika, sementara pengiriman ke server dilakukan di latar belakang saat sinyal tersedia.

**Soal 2:**
*Pembahasan Analitis:*
Transactional Outbox menjamin keandalan pengiriman dengan membungkus penyimpanan data faktur penjualan dan pencatatan tugas sinkronisasi antrean ke dalam satu transaksi basis data atomik lokal (`BEGIN TRANSACTION ... COMMIT`). Jika salah satu proses gagal, seluruh transaksi dibatalkan, menjamin tidak akan pernah terjadi kondisi di mana faktur tercatat di toko namun tugas pengiriman ke server hilang.

**Soal 3:**
*Pembahasan Analitis:*
Thundering Herd Problem adalah fenomena lonjakan beban ekstrem di mana ribuan ponsel kasir yang sebelumnya terisolasi sinyal serentak mencoba mengirimkan seluruh antrean transaksi mereka pada detik yang sama ketika menara BTS pulih. Teknik Exponential Backoff with Jitter memitigasi hal ini dengan melipatgandakan interval penundaan dan menyisipkan variasi waktu acak pada setiap perangkat sehingga beban terdistribusi merata.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Offline-First memperlakukan media penyimpanan lokal sebagai sumber kebenaran data utama. |
| **2** | **A** | Transactional Outbox mencatat tugas pengiriman data ke basis data lokal sebelum dikirim. |
| **3** | **A** | CatatAksiOffline() menyimpan aksi transaksi baru ke dalam antrean offline secara atomik. |
| **4** | **A** | Dead-Letter Queue (DLQ) menampung transaksi yang berulang kali gagal ditolak server. |
| **5** | **A** | ProsesSinkronisasiOffline() mengirimkan seluruh antrean transaksi tertunda ke server cloud. |
| **6** | **A** | At-Least-Once Delivery menjamin sebuah transaksi dipastikan tiba di server minimal satu kali. |
| **7** | **A** | Idempotency Key mencegah pencatatan transaksi ganda di server jika terjadi blip koneksi. |
| **8** | **A** | Jitter menambahkan variasi waktu acak pada rekoneksi untuk mencegah lonjakan serentak. |
| **9** | **A** | Ketiadaan pola Outbox menyebabkan kasir macet dan data penjualan hilang saat internet putus. |
| **10** | **A** | Persentase kehilangan data transaksi pada sistem Resilient Outbox WidyaMobile adalah 0.00%. |

---

## Pembahasan Soal Evaluasi Bab 27

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Resolusi benturan berbasis jam fisik (wall-clock timestamp) sering merusak data karena jam internal perangkat seluler dapat meleset beberapa detik akibat perbedaan sinkronisasi NTP, zona waktu manual, atau latency jaringan. Akibatnya, pembaruan stok yang sebenarnya terjadi lebih dulu di dunia nyata dapat keliru dianggap terjadi belakangan dan menimpa pembaruan sah dari perangkat lain.

**Soal 2:**
*Pembahasan Analitis:*
Tiga sifat semilattice menjamin konsistensi CRDT: sifat Komutatif menjamin urutan penggabungan data tidak mempengaruhi hasil akhir ($A \sqcup B = B \sqcup A$); sifat Asosiatif menjamin pengelompokan operasi penggabungan menghasilkan nilai sama ($(A \sqcup B) \sqcup C = A \sqcup (B \sqcup C)$); dan sifat Idempoten menjamin penggabungan data yang sama berulang kali tidak mengubah status ($A \sqcup A = A$), menjamin konvergensi deterministik tanpa kunci.

**Soal 3:**
*Pembahasan Analitis:*
Delta-State CRDT meminimalkan konsumsi kuota data dengan hanya mengirimkan paket selisih perubahan data mikro (delta mutations) yang terjadi sejak titik sinkronisasi terakhir, bukan mentransmisikan seluruh struktur dokumen basis data yang berukuran puluhan megabyte. Pendekatan ini menghemat konsumsi bandwidth seluler hingga lebih dari 99% dan mempercepat proses penggabungan di edge cloud.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | CRDT (Conflict-Free Replicated Data Types) menjamin penggabungan data otomatis tanpa benturan. |
| **2** | **A** | Sifat komutatif menjamin bahwa urutan penggabungan data (A ⊔ B = B ⊔ A) menghasilkan nilai sama. |
| **3** | **A** | Lamport Timestamps / Vector Clocks menentukan urutan kausalitas peristiwa tanpa jam fisik. |
| **4** | **A** | mobile_crdt_dokumen_buat() menginisialisasi dokumen terdistribusi CRDT baru. |
| **5** | **A** | PN-Counter (Positive-Negative Counter) mengelola stok inventaris yang bertambah dan berkurang. |
| **6** | **A** | Strong Eventual Consistency menjamin seluruh replika mencapai status data final yang sama. |
| **7** | **A** | mobile_crdt_sinkronisasi_edge() menggabungkan paket mutasi delta ke dalam dokumen lokal. |
| **8** | **A** | Sifat idempoten menjamin penggabungan data yang sama berulang kali (A ⊔ A = A) tidak mengubah isi. |
| **9** | **A** | Ukuran payload satu mutasi delta CRDT WidyaMobile hanya berkisar 100 hingga 200 Byte. |
| **10** | **A** | Pendekatan Lock-Free memastikan kasir di lapangan tidak pernah terhalang oleh loading spinner. |

---

## Pembahasan Soal Evaluasi Bab 28

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Kunci kriptografi pada Android StrongBox dan Apple Secure Enclave bersifat Non-Exportable karena materi kunci dibangkitkan secara acak langsung di dalam perangkat keras mikrokontroler keamanan terpisah dan silikon chip tersebut secara fisik tidak memiliki gerbang logika atau instruksi perintah mesin untuk menyalin kunci keluar. Kunci hanya dapat digunakan di dalam chip untuk operasi penandatanganan atau enkripsi.

**Soal 2:**
*Pembahasan Analitis:*
Hardware Key Attestation memungkinkan server backend perbankan memverifikasi keaslian kunci melalui rantai sertifikat digital X.509 yang diterbitkan dan ditandatangani langsung oleh kunci privat pabrikan prosesor (Google atau Apple) di dalam silikon chip. Sertifikat ini memuat atribut verifikasi bahwa kunci dibangkitkan di dalam modul perangkat keras asli dan perangkat berada dalam status integritas aman bebas modifikasi.

**Soal 3:**
*Pembahasan Analitis:*
Pengikatan biometrik kriptografis (Biometric-Bound Crypto Object) memastikan bahwa kunci privat transaksi yang tersimpan di dalam Secure Enclave hanya dapat dibuka kuncinya jika sensor biometrik fisik berhasil mencocokkan ciri biologis pemilik sah dalam jendela waktu otorisasi beberapa detik. Hal ini mencegah malware yang memiliki hak akses sistem mengeksekusi operasi penandatanganan transaksi tanpa kehadiran fisik pengguna.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Android StrongBox Keymaster adalah chip prosesor keamanan terpisah pada ponsel unggulan. |
| **2** | **A** | Secure Enclave Processor (SEP) adalah koprosesor keamanan perangkat keras pada Apple iOS. |
| **3** | **A** | SimpanKunciAman() menyimpan nilai rahasia ke dalam brankas hardware seluler terisolasi. |
| **4** | **A** | FIPS 140-2 Level 3 / CC EAL5+ adalah standar sertifikasi chip keamanan perangkat keras. |
| **5** | **A** | HapusKunciAman() menghapus dan memusnahkan kunci rahasia secara permanen dari hardware. |
| **6** | **A** | ECDSA kurva P-256 dieksekusi langsung di dalam Secure Enclave untuk tanda tangan digital. |
| **7** | **A** | Biometric-Bound Access membatasi penggunaan kunci hanya beberapa detik pasca-sidik jari valid. |
| **8** | **A** | Kunci tetap tidak dapat dibaca pencuri memori fisik karena terisolasi di dalam silikon chip. |
| **9** | **A** | AmbilKunciAman() membaca kembali nilai rahasia dari brankas hardware setelah terotentikasi. |
| **10** | **A** | Waktu rata-rata eksekusi tanda tangan digital ECDSA di dalam Secure Enclave adalah ~12.4 ms. |

---

## Pembahasan Soal Evaluasi Bab 29

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Otentikasi biometrik yang hanya mengandalkan nilai boolean `true/false` dari dialog antarmuka sangat rentan karena penyerang dapat menyuntikkan skrip Frida untuk membajak fungsi pemanggil (function hooking) dan secara paksa mengubah nilai balik fungsi menjadi `true` meskipun pemindaian sidik jari gagal atau dibatalkan. Pengikatan Crypto-Object mencegah hal ini karena kunci kriptografi fisik tidak akan pernah terbuka tanpa verifikasi perangkat keras asli.

**Soal 2:**
*Pembahasan Analitis:*
Kamera 2D biasa hanya menangkap intensitas warna datar pada bidang dua dimensi sehingga sangat mudah dikelabui oleh foto cetak atau rekaman video korban di layar ponsel. Teknologi 3D inframerah (Apple TrueDepth / Structured Light) memproyeksikan puluhan ribu titik inframerah tak terlihat ke wajah pengguna untuk membaca kedalaman kontur tiga dimensi, kehangatan biologis, dan pantulan mikro kornea mata, kebal terhadap serangan tipuan 2D.

**Soal 3:**
*Pembahasan Analitis:*
Sistem perbankan mewajibkan pembatalan otomatis kunci biometrik (auto-invalidation) saat ada pendaftaran sidik jari baru untuk melindungi pengguna dari serangan perampasan fisik: jika orang lain yang mengetahui kode sandi layar membuka pengaturan ponsel dan mendaftarkan sidik jarinya sendiri ke sistem operasi, seluruh kunci transaksi finansial lama seketika hangus dan pengguna wajib mengotentikasi ulang menggunakan kredensial perbankan master.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | BiometricPrompt API adalah antarmuka resmi Android untuk memicu pemindaian biometrik. |
| **2** | **A** | FAR (False Acceptance Rate) adalah rasio di mana orang yang tidak berhak diterima keliru. |
| **3** | **A** | MintaOtentikasiBiometrik() meminta otentikasi biometrik pengguna dengan dialog native. |
| **4** | **A** | Crypto-Object mengikat dialog biometrik secara atomik dengan inisialisasi kunci sandi TEE. |
| **5** | **A** | Rasio penerimaan palsu (FAR) Apple 3D FaceID adalah 1 banding 1.000.000 pengguna. |
| **6** | **A** | Class 3 (Strong) adalah klasifikasi biometrik yang diizinkan mengoperasikan kunci perbankan. |
| **7** | **A** | InvalidatedByBiometricEnrollment membatalkan kunci otomatis bila ada jari baru didaftarkan. |
| **8** | **A** | Frida / Xposed sering digunakan penyerang untuk memanipulasi nilai balik fungsi boolean. |
| **9** | **A** | Izin android.permission.USE_BIOMETRIC wajib disertakan pada berkas manifes Android. |
| **10** | **A** | Kecepatan rata-rata buka kunci sensor sidik jari ultrasonik di bawah layar adalah < 200 ms. |

---

## Pembahasan Soal Evaluasi Bab 30

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Rooting pada Android atau Jailbreaking pada iOS meruntuhkan arsitektur isolasi kotak pasir (Application Sandbox) dengan menonaktifkan kebijakan SELinux dan perlindungan integritas kernel. Akibatnya, batasan hak akses file privat antar-aplikasi runtuh, memungkinkan aplikasi berbahaya yang memiliki hak akses root membaca isi memori kerja aplikasi perbankan, menyalin basis data lokal, atau menyadap kunci sesi transaksi.

**Soal 2:**
*Pembahasan Analitis:*
Dynamic Function Hooking menggunakan Frida bekerja dengan menyuntikkan pustaka dinamis (in-process agent) ke dalam ruang alamat memori proses aplikasi yang sedang berjalan. Frida kemudian memodifikasi tabel instruksi fungsi (PLT/GOT hooking atau inline hooking) untuk mengalihkan eksekusi kode resmi ke kode buatan peretas, memungkinkan penyerang memanipulasi parameter fungsi, mencuri kunci kriptografi, atau memotong alur verifikasi keamanan.

**Soal 3:**
*Pembahasan Analitis:*
RASP WidyaMobile mendeteksi emulator PC dengan memeriksa ketiadaan perangkat keras sensor fisik nyata (seperti sensor akselerometer MEMS, giroskop, dan barometer yang selalu menghasilkan nilai statis 0 di emulator), memindai properti motherboard generik QEMU/Goldfish, mendeteksi ketiadaan sel baterai kimia asli, serta mengukur anomali laju instruksi siklus eksekusi CPU pada arsitektur translasi x86 ke ARM.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | RASP (Runtime Application Self-Protection) adalah kerangka perlindungan diri aplikasi seluler. |
| **2** | **A** | Frida adalah alat pengait memori dinamis yang sering digunakan untuk membajak logika aplikasi. |
| **3** | **A** | Berkas biner /system/bin/su memungkinkan proses menaikkan hak aksesnya menjadi superuser. |
| **4** | **A** | mobile_keamanan_deteksi_jailbreak_root() memverifikasi ada tidaknya root, jailbreak, dan hooking. |
| **5** | **A** | /proc/self/maps memuat daftar seluruh pustaka dinamis (.so) yang dimuat ke dalam memori proses. |
| **6** | **A** | Port TCP 27042 adalah port jaringan lokal standar yang secara default dibuka oleh daemon Frida. |
| **7** | **A** | Emulator PC mudah dideteksi karena pada umumnya tidak memiliki sensor fisik akselerometer asli. |
| **8** | **A** | Tag kompilasi release-keys menandakan build sistem operasi resmi pabrikan berstandar rilis. |
| **9** | **A** | Deteksi Frida seketika menghapus kunci sesi dari memori dan memblokir proses transaksi. |
| **10** | **A** | Rasio keberhasilan deteksi RASP WidyaMobile terhadap Magisk modern berada di atas 99%. |

---

## Pembahasan Soal Evaluasi Bab 31

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Serangan Man-in-the-Middle (MITM) terjadi ketika penyerang berhasil memasang sertifikat Root CA berbahaya ke dalam gudang sertifikat sistem operasi pengguna. Ketika aplikasi membuka koneksi HTTPS, penyerang mencegat sambungan dan menyajikan sertifikat palsu yang ditandatangani oleh CA berbahaya tersebut; karena OS mempercayai CA tersebut, peramban menganggap koneksi sah dan penyerang dapat mendekripsi seluruh teks transaksi.

**Soal 2:**
*Pembahasan Analitis:*
Public Key Pinning (SPKI Hashing) jauh lebih fleksibel dibandingkan Certificate Pinning karena hanya mengunci sidik jari hash SHA-256 dari kunci publik server. Ketika sertifikat SSL tahunan kedaluwarsa dan diperbarui oleh otoritas penerbit, pengelola server dapat memperbarui sertifikat menggunakan pasangan kunci privat/publik yang sama (CSR yang sama) sehingga hash SPKI tidak berubah dan aplikasi seluler tidak terputus koneksinya.

**Soal 3:**
*Pembahasan Analitis:*
Regulasi Bank Indonesia dan standar PCI-DSS mewajibkan penerapan SSL Pinning karena aplikasi finansial menangani data rahasia bernilai uang tinggi yang menjadi sasaran utama penyerang di jaringan publik. Tanpa SSL Pinning, transaksi perbankan rentan disadap dan dimanipulasi oleh operator Wi-Fi publik nakal atau malware proxy, mengancam integritas sistem pembayaran nasional.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | SSL / TLS Public Key Pinning membatasi validasi hanya pada kunci publik resmi milik server. |
| **2** | **A** | Subject Public Key Info (SPKI) adalah struktur kunci publik X.509 yang diambil hash SHA-256-nya. |
| **3** | **A** | mobile_ssl_pinning_konfigurasi() mengonfigurasi Strict Public Key Pinning pada saluran jaringan. |
| **4** | **A** | SHA-256 adalah algoritma fungsi hash standar yang digunakan untuk membuat sidik jari SPKI. |
| **5** | **A** | SSLHandshakeException terpicu ketika kunci publik server tidak cocok dengan pin terdaftar. |
| **6** | **A** | Backup Pin wajib disertakan agar aplikasi tidak bricked jika sertifikat server diganti mendadak. |
| **7** | **A** | Man-in-the-Middle (MITM) adalah serangan siber di mana penyerang mencegat lalu lintas jaringan. |
| **8** | **A** | Burp Suite / Charles Proxy adalah perangkat lunak proxy audit untuk mencegat paket HTTPS. |
| **9** | **A** | Overhead waktu komputasi verifikasi hash SHA-256 SPKI saat jabat tangan TLS adalah < 0.5 ms. |
| **10** | **A** | Strict SSL Pinning menolak secara mutlak sertifikat buatan pengguna demi kedaulatan data. |

---

## Pembahasan Soal Evaluasi Bab 32

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Layanan latar belakang biasa tanpa batas dimatikan oleh sistem operasi Android modern untuk melindungi daya tahan baterai dan kestabilan memori perangkat. Layanan yang berjalan bebas di latar belakang sering kali mempertahankan CPU wakelock secara tidak bertanggung jawab, menguras kapasitas baterai ponsel dalam hitungan jam dan menyebabkan aplikasi latar depan menjadi lamban.

**Soal 2:**
*Pembahasan Analitis:*
Constraint-Based Scheduling adalah penjadwalan tugas yang hanya mengizinkan eksekusi pekerjaan ketika kondisi perangkat keras yang ditentukan terpenuhi. Tiga contoh batasan umum untuk sinkronisasi berat mencakup: `RequiresCharging` (ponsel wajib terhubung ke charger listrik), `NetworkType.UNMETERED` (wajib terhubung ke koneksi Wi-Fi tanpa kuota berbayar), dan `RequiresDeviceIdle` (pengguna tidak sedang aktif menggunakan ponsel).

**Soal 3:**
*Pembahasan Analitis:*
Expedited Job adalah mekanisme prioritas tinggi pada WorkManager yang memungkinkan tugas krusial (seperti pengiriman bukti pembayaran kasir mendesak) dieksekusi secepat mungkin oleh sistem operasi tanpa terhambat oleh antrean batching Doze Mode biasa, dengan memanfaatkan kuota waktu eksekusi darurat khusus yang dialokasikan oleh sistem operasi.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | WorkManager adalah subsistem standar Android modern untuk tugas latar belakang persisten. |
| **2** | **A** | BGTaskScheduler adalah subsistem native iOS untuk menjadwalkan tugas pemrosesan background. |
| **3** | **A** | mobile_background_worker_jadwalkan() mendaftarkan pekerja latar belakang terkelola. |
| **4** | **A** | Execution Constraints adalah syarat kondisi hardware yang wajib terpenuhi sebelum tugas jalan. |
| **5** | **A** | Interval waktu minimum yang diizinkan untuk tugas berkala (Periodic Work) adalah 15 Menit. |
| **6** | **A** | Expedited Work adalah tugas latar belakang prioritas tinggi yang dieksekusi secepat mungkin. |
| **7** | **A** | Daftar antrean WorkManager disimpan persisten di SQLite internal OS sehingga kebal reboot. |
| **8** | **A** | onStopped() adalah sinyal interupsi saat batasan hardware tidak lagi terpenuhi (misal cas dicabut). |
| **9** | **A** | Batasan RequiresCharging memastikan baterai pengguna tidak terkuras saat kompresi data besar. |
| **10** | **A** | Keberhasilan eksekusi tugas latar belakang terkelola WorkManager WidyaMobile mencapai 99.8%. |

---

## Pembahasan Soal Evaluasi Bab 33

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Inferensi Edge AI On-Device lebih unggul dalam kepatuhan UU PDP karena seluruh pemrosesan citra foto KTP-el dan rekaman biometrik wajah diselesaikan secara lokal di dalam memori kerja chip NPU ponsel pengguna tanpa pernah mentransmisikan data mentah tersebut melintasi jaringan internet publik ke peladen luar negeri, meniadakan risiko kebocoran data di transit atau peretasan basis data terpusat.

**Soal 2:**
*Pembahasan Analitis:*
Teknologi 3D Liveness Detection membedakan wajah asli dari foto cetak atau layar tablet melalui analisis multi-faktor: mengukur kedalaman kontur tiga dimensi wajah menggunakan proyeksi titik inframerah, menganalisis pantulan mikro cahaya pada kornea mata manusia hidup, serta mendeteksi variasi fluktuasi mikrosirkulasi darah di bawah kulit wajah (remote photoplethysmography) yang mustahil ditiru oleh layar digital.

**Soal 3:**
*Pembahasan Analitis:*
Kuantisasi model dari FP32 (titik kambang 32-bit) ke INT8 (bilangan bulat 8-bit) memetakan rentang nilai bobot jaringan saraf tiruan ke representasi bilangan bulat 8-bit diskrit. Teknik ini memangkas ukuran memori berkas bobot model hingga 75% lebih kecil dan melipatgandakan kecepatan komputasi perkalian matriks pada silikon NPU dengan penurunan akurasi pengenalan karakter yang dapat diabaikan (< 0.5%).

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | NPU (Neural Processing Unit) adalah akselerator hardware khusus operasi tensor jaringan saraf. |
| **2** | **A** | Photo Print Spoofing Attack adalah serangan pemalsuan dengan memperlihatkan foto cetak korban. |
| **3** | **A** | mobile_ai_liveness_verifikasi() mengeksekusi uji keaslian wajah manusia hidup di chip NPU. |
| **4** | **A** | Format kuantisasi INT8 (Integer 8-bit) memangkas memori bobot model dan mempercepat inferensi. |
| **5** | **A** | mobile_ai_ktp_ocr_instan() mengekstrak data teks KTP-el secara instan tanpa koneksi internet. |
| **6** | **A** | Nomor Induk Kependudukan (NIK) resmi Republik Indonesia terdiri dari 16 digit angka. |
| **7** | **A** | Edge AI berjalan 100% offline karena seluruh bobot model terkompilasi mandiri di biner lokal. |
| **8** | **A** | Waktu inferensi pengujian liveness 3D pada NPU ponsel modern adalah sekitar 12 milidetik. |
| **9** | **A** | Skor liveness < 0.95 seketika menolak pendaftaran akun dan mencatat indikasi manipulasi. |
| **10** | **A** | Edge AI meniadakan biaya tagihan sewa peladen GPU cloud eksternal untuk pemindaian identitas. |

---

## Pembahasan Soal Evaluasi Bab 34

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Ukuran biner aplikasi seluler sangat berpengaruh di negara berkembang karena biaya kuota internet seluler relatif mahal dan kecepatan unduh jaringan di daerah pelosok sering kali terbatas dan tidak stabil. Ukuran berkas yang besar memperpanjang waktu tunggu unduh, meningkatkan risiko kegagalan unduh di tengah jalan, dan mendorong pengguna membatalkan pemasangan aplikasi (high drop-off rate).

**Soal 2:**
*Pembahasan Analitis:*
Apple App Clips dan Android Instant Apps memungkinkan pengguna mengakses fungsi spesifik aplikasi secara instan tanpa melalui proses instalasi penuh di beranda ponsel. Pengguna cukup menempelkan ponsel ke tag NFC atau memindai kode QRIS di meja restoran; sistem operasi langsung mengunduh biner mikro (< 10 MB) ke memori sementara dan menampilkan antarmuka pesanan dalam 2 detik untuk menyelesaikan transaksi seketika.

**Soal 3:**
*Pembahasan Analitis:*
Aggressive Dead-Code Elimination dan Symbol Stripping pada kompiler WidyaMobile menganalisis seluruh pohon sintaksis program untuk membuang fungsi, modul standar, dan metadata debug yang tidak pernah dipanggil oleh kode aplikasi. Dikombinasikan dengan ketiadaan runtime mesin virtual perantara (Zero-JVM dan Zero-JS Engine), ukuran biner biner ARM64 murni dapat ditekan di bawah 12 Megabyte.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Batas ukuran maksimal berkas biner Apple App Clip adalah 15 Megabyte (rekomendasi < 10 MB). |
| **2** | **A** | Google Play Instant Apps memungkinkan aplikasi Android berjalan seketika tanpa instalasi penuh. |
| **3** | **A** | mobile_appclip_konfigurasi() mengonfigurasi modul peluncuran instan App Clip berbobot ringan. |
| **4** | **A** | Asset Slicing mengirimkan hanya aset visual yang sesuai dengan kerapatan piksel layar ponsel. |
| **5** | **A** | mobile_biner_audit_ukuran() mengaudit alokasi ukuran segmen biner paket aplikasi rilis. |
| **6** | **A** | React Native dan Flutter sulit jadi App Clip < 10MB karena ukuran runtime JS/Dart yang besar. |
| **7** | **A** | Penempelan tag NFC atau pemindaian App Clip Code adalah pemicu fisik pembukaan App Clip. |
| **8** | **A** | Sesi App Clip bersifat sementara (ephemeral) dan dibersihkan jika pengguna tidak instal penuh. |
| **9** | **A** | Ukuran biner dasar aplikasi komprehensif WidyaMobile berkisar 8.5 hingga 11.8 Megabyte. |
| **10** | **A** | Waktu peluncuran awal App Clip WidyaMobile dari pemindaian NFC adalah kurang dari 2.5 detik. |

---

## Pembahasan Soal Evaluasi Bab 35

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Antarmuka ponsel layar sentuh besar dirancang untuk interaksi mendalam multi-kolom dan pengguliran daftar panjang, sedangkan antarmuka jam tangan pintar dirancang untuk interaksi mikro sekilas pandang (Glanceable UI) berdurasi kurang dari 3 detik. Elemen jam tangan menitikberatkan pada angka metrik besar, kontras tinggi, teks singkat, dan tombol sentuh melingkar yang dapat dioperasikan secara cepat dengan satu ketukan jari.

**Soal 2:**
*Pembahasan Analitis:*
Layar Ambient Mode / Always-On Display menghemat baterai hingga < 1% per jam dengan memanfaatkan karakteristik fisik panel OLED di mana piksel hitam murni (true black) mematikan emisi cahaya dioda secara total sehingga tidak mengonsumsi arus listrik sama sekali. Selain itu, laju penyegaran layar diturunkan drastis dari 60 FPS ke 1 FPS dan elemen warna cerah disederhanakan menjadi garis monokrom tipis.

**Soal 3:**
*Pembahasan Analitis:*
Sensor fotopletismografi (PPG) memancarkan cahaya LED hijau ke pembuluh darah kapiler pergelangan tangan untuk mengukur laju denyut jantung (Heart Rate) dan variabilitas detak jantung (HRV) guna memprediksi tingkat kelelahan fisik dan stres pekerja tambang. Sensor inframerah mengukur rasio penyerapan cahaya teroksigenasi untuk menghitung saturasi oksigen darah (SpO2) guna mendeteksi ancaman hipoksia di area industri berbahaya.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Wear OS adalah sistem operasi jam tangan pintar standar milik Google yang mendukung Tiles. |
| **2** | **A** | watchOS adalah sistem operasi jam pintar resmi milik Apple yang mendukung Complications. |
| **3** | **A** | mobile_wearable_sinkronisasi_tile() menyinkronkan data ringkasan ke ubin jam tangan pintar. |
| **4** | **A** | Ambient Mode (Always-On) mematikan piksel OLED hitam dan menurunkan refresh rate ke 1 FPS. |
| **5** | **A** | Sensor optik fotopletismografi (PPG) mengukur aliran denyut darah di pergelangan tangan. |
| **6** | **A** | BPM (Beats Per Minute) adalah satuan standar pengukuran laju denyut jantung manusia. |
| **7** | **A** | mobile_wearable_telemetri_kesehatan_baca() membaca denyut jantung dan saturasi SpO2. |
| **8** | **A** | Saturasi oksigen darah (SpO2) normal pada tubuh manusia sehat berada di atas 95% s/d 100%. |
| **9** | **A** | Bluetooth Low Energy (BLE) adalah protokol nirkabel penghubung ponsel dan jam pendamping. |
| **10** | **A** | Latensi transmisi nirkabel pembaruan ubin jam tangan pintar adalah sekitar 18 milidetik. |

---

## Pembahasan Soal Evaluasi Bab 36

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Live Activities mengubah pengalaman pengguna dengan menyajikan bilah status kemajuan interaktif yang terus diperbarui secara langsung di layar kunci dan pulau kamera tanpa mengharuskan pengguna membuka kunci ponsel atau membuka aplikasi utama. Pengguna dapat terus memantau status pesanan makanan atau kedatangan kurir secara pasif sembari tetap fokus menjalankan aktivitas fisik lainnya.

**Soal 2:**
*Pembahasan Analitis:*
Tiga status tata letak Dynamic Island Apple mencakup: status Compact Presentation (tampilan pil kecil di sekitar kamera depan saat pengguna membuka aplikasi lain), status Minimal Presentation (tampilan lingkaran terpisah saat ada dua aplikasi yang menjalankan Live Activity bersamaan), dan status Expanded Presentation (tampilan kartu interaktif penuh saat pulau dinamis ditekan lama oleh pengguna).

**Soal 3:**
*Pembahasan Analitis:*
Sistem operasi membatasi alokasi memori widget Live Activity secara ketat (maksimal 10 MB) untuk menjamin kelancaran sistem operasi secara keseluruhan, karena widget ini berjalan di proses antarmuka sistem (SpringBoard / SystemUI). Jika sebuah widget mengalami kebocoran memori atau mengonsumsi CPU berlebih, sistem operasi akan seketika membekukan dan menghentikan widget tersebut agar tidak mengganggu performa perangkat.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Live Activities (ActivityKit) menyajikan informasi langsung yang terus hidup di layar kunci iOS. |
| **2** | **A** | Ongoing Notification Chips adalah fitur ekuivalen pada Android 15+ di bilah status atas. |
| **3** | **A** | mobile_live_activity_mulai() meluncurkan sesi Live Activity baru pada sistem operasi. |
| **4** | **A** | Expanded Presentation adalah tampilan penuh Dynamic Island saat ditekan lama oleh pengguna. |
| **5** | **A** | mobile_live_activity_perbarui() memperbarui status teks dan estimasi sisa waktu aktivitas. |
| **6** | **A** | Batas waktu kedaluwarsa maksimal sistem operasi untuk sesi Live Activity adalah 8 Jam. |
| **7** | **A** | mobile_live_activity_akhiri() menutup dan membersihkan sesi Live Activity dari layar kunci. |
| **8** | **A** | Batas alokasi memori RAM maksimal yang diizinkan untuk widget Live Activity adalah 10 MB. |
| **9** | **A** | APNs Live Activity Channel via Push Token mendorong pembaruan data langsung ke pulau dinamis. |
| **10** | **A** | Latensi waktu respons rata-rata pembaruan Dynamic Island WidyaMobile adalah < 45 milidetik. |

---

## Pembahasan Soal Evaluasi Bab 37

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Berkas AndroidManifest.xml teks biasa adalah format dokumen XML berbasis string karakter yang mudah dibaca manusia tetapi membutuhkan penguraian (parsing) parser XML yang lambat dan memboroskan memori di perangkat seluler. Format Android Binary XML (AXML) adalah representasi biner terkompresi di mana seluruh nama tag dan atribut diindeks ke dalam String Pool Chunk dan Resource Map Chunk terkompilasi, memungkinkan sistem operasi memuat manifes ke memori secara instan.

**Soal 2:**
*Pembahasan Analitis:*
Header berkas classes.dex berukuran 112 byte memuat field `checksum` (Adler-32) dan `signature` (SHA-1). Perhitungan dilakukan berurutan: mula-mula seluruh isi berkas dari offset 32 hingga akhir dihitung nilai digest SHA-1-nya dan ditulis ke field signature (offset 12..32); selanjutnya seluruh isi berkas dari offset 12 hingga akhir dihitung nilai Adler-32-nya dan ditulis ke field checksum (offset 8..12), menjamin keutuhan bytecode Dalvik.

**Soal 3:**
*Pembahasan Analitis:*
Kemandirian menghasilkan berkas APK tanpa Android Studio, Gradle, dan JVM memberikan kedaulatan perangkat lunak karena meniadakan ketergantungan pada rantai alat luar negeri yang berukuran puluhan gigabyte dan sering mengunduh ratusan dependensi asing secara otomatis dari internet. Hal ini mengeliminasi risiko serangan rantai pasok perangkat lunak (supply chain poisoning) dan menjamin seluruh siklus hidup aplikasi dapat diaudit secara mandiri.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | src/graphics/pure_apk.rs adalah modul kompiler mandiri in-house Android APK pada Widya-Lang. |
| **2** | **A** | Android Binary XML (AXML) adalah format biner terkompresi dari berkas manifes Android. |
| **3** | **A** | classes.dex adalah berkas bytecode eksekusi Dalvik berstandar v035 di dalam paket APK. |
| **4** | **A** | Adler-32 adalah algoritma checksum 32-bit berkecepatan tinggi pada header berkas classes.dex. |
| **5** | **A** | Direktori META-INF/ memuat berkas MANIFEST.MF, CERT.SF, dan CERT.RSA pada skema JAR signing. |
| **6** | **A** | Paket APK dibungkus dalam kontainer arsip ZIP berstandar PKWARE (RFC 1951). |
| **7** | **A** | Kompiler Pure APK hanya membutuhkan ruang disk < 45 MB berbanding belasan GB Android Studio. |
| **8** | **A** | ARM64-v8a adalah target arsitektur prosesor 64-bit utama pustaka native (.so) Android modern. |
| **9** | **A** | resources.arsc adalah berkas tabel sumber daya terkompilasi yang memetakan ID heksadesimal. |
| **10** | **A** | Waktu kompilasi paket APK lengkap dari kondisi bersih menggunakan Pure APK adalah ~1.4 detik. |

---

## Pembahasan Soal Evaluasi Bab 38

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Berkas biner Mach-O 64-bit ARM64 diawali oleh magic number 4-byte `0xFEEDFACF` (`MH_MAGIC_64`) dan header penanda tipe CPU ARM64. Perintah pemuatan `LC_SEGMENT_64` memetakan segmen memori proses saat dimuat oleh dynamic linker dyld: `__PAGEZERO` (mencegah null pointer dereference), `__TEXT` (kode instruksi mesin hanya-baca), `__DATA` (variabel global baca-tulis), dan `LC_MAIN` yang mendefinisikan alamat offset fungsi titik masuk utama.

**Soal 2:**
*Pembahasan Analitis:*
Berkas `_CodeSignature/CodeResources` adalah dokumen XML/bplist penjamin integritas aplikasi Apple yang memuat kamus hash kriptografis SHA-256 dari setiap berkas biner eksekusi, pustaka dinamis, gambar ikon, dan berkas Info.plist di dalam bundel aplikasi. Jika seorang penyerang memodifikasi satu bit saja berkas biner atau menyisipkan malware ke dalam bundel, verifikasi tanda tangan digital kernel Apple akan seketika menolak peluncuran aplikasi.

**Soal 3:**
*Pembahasan Analitis:*
Kompilasi aplikasi iOS mandiri langsung di atas peladen Linux server tanpa komputer Mac mendemokratisasi dan mengotomatiskan seluruh alur integrasi dan pengiriman berkelanjutan (CI/CD) secara masif. Perusahaan tidak perlu membeli dan memelihara kluster perangkat keras komputer Mac mini di pusat data yang mahal, mengurangi biaya operasional infrastruktur build hingga lebih dari 80% dan mempercepat waktu rilis aplikasi.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | src/graphics/pure_ipa.rs adalah modul kompiler mandiri in-house Apple iOS IPA pada Widya-Lang. |
| **2** | **A** | 0xFEEDFACF (MH_MAGIC_64) adalah magic number penanda berkas biner Mach-O 64-bit pada iOS. |
| **3** | **A** | Payload/ adalah direktori kontainer tingkat teratas wajib di dalam paket berkas .ipa. |
| **4** | **A** | 0x0100000C adalah konstanta CPU_TYPE_ARM64 pada header berkas biner eksekusi Mach-O. |
| **5** | **A** | Info.plist adalah berkas manifes konfigurasi utama yang memuat bundle ID dan nama biner Apple. |
| **6** | **A** | LC_MAIN (0x80000028) adalah Load Command penanda titik masuk fungsi utama biner Mach-O. |
| **7** | **A** | SHA-256 adalah algoritma digest verifikasi integritas berkas di dalam _CodeSignature/CodeResources. |
| **8** | **A** | embedded.mobileprovision adalah profil provisi pengembang yang disematkan ke dalam paket iOS. |
| **9** | **A** | Kompiler Pure IPA membutuhkan < 45 MB ruang disk berbanding 40+ Gigabyte instalasi Xcode. |
| **10** | **A** | Waktu kompilasi paket IPA lengkap di atas server Linux menggunakan Pure IPA adalah ~1.18 detik. |

---

## Pembahasan Soal Evaluasi Bab 39

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Algoritma Binary Delta Patching membandingkan biner versi lama (base binary) dan versi baru (target binary) pada tingkat byte menggunakan teknik pencocokan blok mirip bsdiff/courgette. Algoritma ini menghasilkan serangkaian instruksi biner ringkas: instruksi untuk menyalin blok byte yang tidak berubah dari biner lama dan instruksi untuk menyisipkan blok byte baru, menghasilkan paket tambalan berukuran di bawah 150 Kilobyte dari dua biner berukuran belasan megabyte.

**Soal 2:**
*Pembahasan Analitis:*
Tanda tangan digital asimetris (seperti Ed25519) mutlak wajib diverifikasi sebelum tambalan biner diterapkan guna memastikan bahwa berkas pembaruan tersebut benar-benar diterbitkan oleh pemegang kunci privat resmi pengembang dan belum dimodifikasi oleh penyerang di jaringan (anti-tampering). Menerapkan tambalan yang tidak tervalidasi dapat memungkinkan penyerang menyuntikkan kode berbahaya langsung ke dalam biner aplikasi.

**Soal 3:**
*Pembahasan Analitis:*
Teknik penukaran berkas secara atomik (Atomic File Swap) menulis hasil penambalan biner ke berkas sementara terpisah di media penyimpanan privat dan memvalidasi keutuhan checksum SHA-256 berkas tersebut sebelum melakukan operasi penggantian nama berkas secara atomik (`rename()` syscall tingkat kernel). Jika ponsel mati kehabisan baterai di tengah proses penulisan, berkas lama tetap utuh dan aplikasi tidak akan pernah mengalami kondisi rusak (anti-bricking).

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | WIDYAMOBILE_DELTA10 adalah tanda tangan header biner standar paket tambalan selisih seluler. |
| **2** | **A** | Ukuran unduhan tambalan biner delta WidyaMobile rata-rata berkisar 45 hingga 150 Kilobyte. |
| **3** | **A** | Ed25519 adalah algoritma tanda tangan digital kurva eliptik untuk memvalidasi tambalan OTA. |
| **4** | **A** | mobile_obfuscation_dan_ota_update() menerapkan pembaruan dinamis biner delta secara atomik. |
| **5** | **A** | Jika base hash biner lokal tidak cocok, sistem beralih mengunduh biner penuh (Full Fallback). |
| **6** | **A** | Pembaruan OTA dapat diedarkan seketika dalam hitungan menit tanpa antrean ulasan toko aplikasi. |
| **7** | **A** | Hot-Reload Dynamic Activation mengaktifkan logika baru seketika tanpa menutup aplikasi kasir. |
| **8** | **A** | Atomic File Swap menulis berkas baru ke wadah sementara lalu menukarnya secara instan. |
| **9** | **A** | Penambalan biner delta menghemat lebih dari 98% konsumsi kuota data internet pengguna. |
| **10** | **A** | Code Obfuscation mempersulit penyerang melakukan rekayasa balik (reverse engineering) biner. |

---

## Pembahasan Soal Evaluasi Bab 40

### Bagian A: Pembahasan Latihan Esai

**Soal 1:**
*Pembahasan Analitis:*
Integrasi Offline-First Outbox, basis data terenkripsi AES-256, dan jaringan BLE Mesh menjamin kontinuitas operasional kasir ritel saat bencana alam melanda melalui tiga lapisan ketahanan: kasir dapat terus memproses transaksi secara normal di dalam perangkat menggunakan basis data lokal terenkripsi, seluruh mutasi transaksi dicatat secara atomik ke antrean Outbox untuk disinkronkan saat internet pulih, dan pesan koordinasi darurat logistik disiarkan melompati ponsel relawan via radio BLE Mesh tanpa membutuhkan sinyal BTS.

**Soal 2:**
*Pembahasan Analitis:*
Penggabungan biometrik Class 3 Strong di dalam prosesor keamanan hardware (Secure Enclave / StrongBox) dengan audit RASP anti-root mutlak disyaratkan pada terminal SoftPOS perbankan guna mematuhi standar PCI-CPoC. Biometrik hardware menjamin bahwa kunci privat transaksi tidak pernah dapat dibuka tanpa kehadiran fisik pemilik sah, sementara audit RASP memastikan memori kerja transaksi terlindungi dari pembajakan oleh alat injeksi dinamis seperti Frida di perangkat yang dimodifikasi.

**Soal 3:**
*Pembahasan Analitis:*
Filosofi Kedaulatan Perangkat Lunak (Sovereign Software Paradigm) WidyaMobile menegakkan kemandirian teknologi nasional secara menyeluruh: dimulai dari penggunaan bahasa pemrograman berbahasa Indonesia yang meningkatkan literasi teknologi anak bangsa, ketiadaan ketergantungan pada runtime asing raksasa (Zero-JVM dan Zero-Chromium), hingga kepemilikan kompiler mandiri in-house yang mampu menghasilkan biner rilis Android APK dan iOS IPA secara berdaulat langsung dari server lokal.

### Bagian B: Kunci Jawaban & Pembahasan Pilihan Ganda

| No | Kunci | Pembahasan Ilmiah & Landasan Teori |
| :---: | :---: | :--- |
| **1** | **A** | Proyek capstone Bab 40 menyatukan seluruh 10 Pilar Native dan 6 Pilar Mutakhir Industri. |
| **2** | **A** | Rumus Haversine digunakan untuk menghitung jarak akurat posisi kasir terhadap titik kantor. |
| **3** | **A** | PCI-CPoC dan EMV L2 adalah standar internasional pembayaran kartu debit/kredit nirsentuh. |
| **4** | **A** | ESC/POS adalah protokol standar internasional untuk mencetak struk belanja ke printer thermal. |
| **5** | **A** | WIDYAMOBILE_DELTA10 adalah format biner paket pembaruan instan berbobot ringan (< 150 KB). |
| **6** | **A** | src/graphics/pure_apk.rs menghasilkan paket instalasi Android APK tanpa Android Studio & Gradle. |
| **7** | **A** | src/graphics/pure_ipa.rs menghasilkan paket instalasi iOS IPA langsung di Linux tanpa Xcode. |
| **8** | **A** | RASP (Runtime Application Self-Protection) memantau ada tidaknya Frida, rooting, dan hooking. |
| **9** | **A** | Waktu eksekusi inferensi OCR KTP-el on-device di chip NPU adalah sekitar 34 milidetik. |
| **10** | **A** | Ukuran biner rilis NusantaraMobile SuperApp yang memuat seluruh fitur adalah di bawah 10 MB. |

---
