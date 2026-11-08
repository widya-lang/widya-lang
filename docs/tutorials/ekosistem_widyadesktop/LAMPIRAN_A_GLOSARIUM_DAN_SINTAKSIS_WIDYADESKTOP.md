# Lampiran A: Glosarium & Sintaksis Lengkap API WidyaDesktop

Lampiran ini menyajikan referensi komprehensif istilah teknis, modul bawaan, dan tanda tangan fungsi pustaka standar WidyaDesktop Framework Enterprise Edition.

---

## A.1 Glosarium Istilah Teknis

| Istilah Teknis | Definisi & Penjelasan Arsitektural |
| :--- | :--- |
| **AccessKit** | Pustaka abstraksi aksesibilitas lintas platform yang menghubungkan antarmuka dengan pembaca layar sistem operasi (NVDA, JAWS, VoiceOver, Orca). |
| **AES-256-GCM** | Standar enkripsi simetris terotentikasi (*Authenticated Encryption with Associated Data - AEAD*) dengan kunci 256-bit dan tag integritas 128-bit. |
| **App Nap** | Fitur hemat daya pada macOS yang menurunkan prioritas CPU dan frekuensi timer aplikasi saat jendelanya tidak terlihat. |
| **Circular Ring Buffer** | Struktur memori melingkar berukuran tetap di mana data baru menimpa data terlama tanpa realokasi memori berulang (*zero allocation churn*). |
| **Content Security Policy (CSP)** | Mekanisme keamanan antarmuka yang membatasi asal sumber daya skrip, gaya, dan koneksi jaringan guna menangkal serangan XSS. |
| **Customer Facing Display (CFD)** | Layar monitor sekunder yang menghadap ke arah pembeli untuk menyajikan rincian belanjaan dan kode QRIS dinamis. |
| **Dirty Region Tracking** | Algoritma perenderan grafis selektif yang hanya menggambar ulang kotak koordinat piksel yang mengalami perubahan data. |
| **EcoQoS** | Mekanisme penjadwalan energi pada Windows 11 yang mengarahkan eksekusi utas ke inti prosesor hemat daya (*Efficiency Cores*). |
| **Ed25519** | Skema tanda tangan digital berbasis kurva eliptis Edwards-25519 yang sangat cepat, aman, dan tahan serangan saluran samping. |
| **ESC/POS** | Protokol perintah biner standar industri internasional untuk mengendalikan printer kasir thermal dan laci kasir otomatis. |
| **Fuel Metering** | Mekanisme pengukuran kuota instruksi komputasi pada kotak pasir WebAssembly guna mencegah perulangan tak terbatas (*infinite loops*). |
| **IPC (Inter-Process Communication)** | Saluran komunikasi berkecepatan tinggi antar-proses menggunakan pipa anonim (*anonymous pipes*) dan memori bersama. |
| **Kiosk Mode** | Mode tampilan layar penuh eksklusif tanpa bilah judul dan taskbar disertai pencegatan tombol sistem seperti `Alt+Tab` dan `WinKey`. |
| **Modbus RTU** | Protokol komunikasi serial master-slave industri yang berjalan di atas kabel RS-485 berpelindung siklik CRC-16. |
| **Noise Protocol Framework** | Kerangka kerja kriptografi jabat tangan modern berbasis X25519 dan ChaCha20-Poly1305 untuk sesi P2P terenkripsi ujung-ke-ujung (E2EE). |
| **PC/SC (Personal Computer/Smart Card)** | Standar antarmuka pemrograman sistem operasi untuk berkomunikasi dengan pembaca kartu pintar dan chip KTP-el (ISO 7816). |
| **Signals** | Primitif reaktivitas berbutir halus (*fine-grained*) yang melacak fungsi dependensi secara otomatis dengan kompleksitas $O(1)$. |
| **Strict Origin** | Kebijakan pemetaan aset lokal ke skema kustom aman (`widya-app://`) guna mencegah eksploitasi pembacaan berkas lintas direktori. |
| **WIDYADELTA10** | Format biner tambalan diferensial resmi WidyaDesktop yang menghemat konsumsi kuota pembaruan otomatis hingga lebih dari 99%. |
| **Write-Ahead Logging (WAL)** | Mode transaksi basis data di mana mutasi dicatat terlebih dahulu ke berkas log terpisah sebelum disinkronkan ke berkas utama. |

---

## A.2 Katalog Fungsi Bawaan Modul Deklaratif `modul/desktop.wya`

### 1. Inisialisasi & Pengendalian Jendela
- `BuatAplikasiDesktop(judul, lebar, tinggi) -> ObjekAplikasiDesktop`: Menginstansiasi aplikasi desktop baru lengkap dengan jendela native, bilah menu, baki sistem, dan state reaktif.
- `app.set_judul(teks_judul)`: Mengubah teks bilah judul jendela secara dinamis.
- `app.set_ukuran(lebar, tinggi)`: Menetapkan dimensi lebar dan tinggi jendela dalam satuan piksel.
- `app.pusatkan()`: Memosisikan jendela tepat di tengah-tengah layar monitor primer.
- `app.set_layar_penuh(aktif)`: Mengaktifkan atau menonaktifkan mode layar penuh eksklusif.
- `app.tutup()`: Menutup jendela dan melepaskan seluruh sumber daya grafis secara anggun.

### 2. Antarmuka Menu, Baki Sistem & Notifikasi
- `app.tambah_menu(label, pintasan)`: Mendaftarkan item baru pada bilah menu native beserta kombinasi tombol pintasannya.
- `app.kirim_notifikasi(judul, pesan)`: Mengirimkan spanduk notifikasi toast sistem operasi melalui baki sistem.
- `tray_set_tooltip(baki, teks)`: Mengubah teks tooltip yang muncul saat kursor diarahkan ke ikon baki sistem.

### 3. Komunikasi Jembatan IPC & Dispatcher RPC
- `app.daftarkan_rpc(nama_metode, fn_handler)`: Mendaftarkan fungsi logika bisnis backend yang dapat dipanggil dari antarmuka pengguna.
- `app.jalankan_rpc(nama_metode, payload)`: Mengeksekusi penangan RPC terdaftar secara mandiri.

### 4. Kotak Dialog Sistem Operasi Native
- `BukaDialogPilihBerkas(filter) -> Teks`: Membuka kotak dialog pemilihan berkas native dengan penyaring ekstensi (misal: `"*.csv;*.xlsx"`).
- `BukaDialogSimpanBerkas(nama_default) -> Teks`: Membuka kotak dialog penentuan lokasi penyimpanan berkas dengan saran nama awal.
- `TampilkanKotakPesan(tipe, judul, pesan) -> Teks`: Memunculkan kotak dialog modal pesan sistem operasi native.

### 5. Manajemen State Reaktif Berbasis Signals
- `BuatStateReaktif(nilai_awal) -> ObjekSinyal`: Menciptakan sinyal reaktif baru dengan nilai awal terstruktur.
- `AmbilNilaiState(sinyal) -> Nilai`: Membaca nilai sinyal sekaligus mendaftarkan pemanggil ke graf dependensi otomatis.
- `PerbaruiNilaiState(sinyal, nilai_baru)`: Memperbarui nilai sinyal dan memicu seluruh efek pengamat terdaftar seketika $O(1)$.
- `LanggananState(sinyal, callback)`: Menetapkan fungsi efek pengamat otomatis terhadap mutasi sinyal.

### 6. Papan Klip & Audit Keamanan Bawaan
- `papan_klip_tulis(teks) -> Boolean`: Menulis konten teks ke papan klip sistem operasi tanpa jeda.
- `papan_klip_baca() -> Teks`: Membaca konten teks yang tersimpan di papan klip sistem operasi saat ini.
- `AuditKeamananAplikasi() -> Map`: Mengembalikan laporan audit kepatuhan keamanan resmi (skor 100/100, status token IPC, Strict CSP, dan brankas AES-256-GCM).

---

## A.3 Katalog Fungsi Native Tingkat Rendah `src/desktop.rs`

- `desktop_kanvas_gpu_buat(backend, lebar, tinggi, fps)`: Menginisialisasi kanvas GPU WGPU 120 FPS.
- `desktop_kanvas_gambar_bingkai(canvas_id, frame_num)`: Merender pergantian bingkai pada kanvas GPU.
- `desktop_kanvas_sensor_stream(canvas_id, sensor_id, data_point)`: Mengalirkan data osiloskop ke penyangga lingkar memori.
- `desktop_kanvas_audio_waveform_render(canvas_id, buffer, channels)`: Merender visualisasi gelombang audio stereo/mono.
- `desktop_embedded_db_kueri(db_name, query, params, wal_mode)`: Mengeksekusi kueri lokal WidyaBase Embedded mode ACID WAL.
- `desktop_penyimpanan_aman_os(key_id, payload, hardware_vault)`: Menyimpan kredensial ke modul TPM atau macOS Keychain.
- `desktop_a11y_pohon_perbarui(tree_id, focused_node)`: Memperbarui pohon semantik AccessKit untuk pembaca layar.
- `desktop_a11y_umumkan(pesan, kesopanan)`: Menyiarkan pengumuman suara langsung ke pembaca layar.
- `desktop_a11y_audit_wcag(tree_id)`: Mengevaluasi kepatuhan aksesibilitas terhadap standar WCAG 2.2 AAA.
- `desktop_remote_sesi_buat(peer_id)`: Membuka sesi kendali jarak jauh terenkripsi Noise Protocol E2EE.
- `desktop_remote_dorong_bingkai(session_id, w, h, format)`: Mengompresi dan mengalirkan tangkapan layar VRAM ke teknisi.
- `desktop_remote_injeksi_input(session_id, event)`: Menginjeksikan pergerakan kursor mouse dan ketukan keyboard jarak jauh.
- `desktop_ktpel_baca_chip(slot)`: Mengekstrak NIK dan data demografi resmi langsung dari chip pintar KTP-el ISO 7816.
- `desktop_biometrik_cocokkan_sidik_jari(probe, gallery)`: Mencocokkan template minutiae sidik jari ANSI/NIST-ITL 1-2011.
- `desktop_ai_lokal_ocr_faktur(panjang_citra)`: Mengekstrak teks faktur belanja secara offline via NPU.
- `desktop_ai_lokal_transkripsi_suara(panjang_sampel)`: Mentranskripsi rekaman suara operator kasir via Whisper NPU lokal.
- `desktop_ai_lokal_inferensi_teks(prompt, max_tokens)`: Menjalankan inferensi model bahasa kecil (SLM) on-device.
- `desktop_plugin_muat(nama_plugin)`: Memuat modul ekstensi WebAssembly ke dalam kotak pasir memori linier terisolasi.
- `desktop_plugin_eksekusi(id_plugin, fungsi)`: Mengeksekusi fungsi plugin WASM dengan pembatasan kuota bahan bakar (*Fuel Metering*).
- `desktop_paket_bangun_distribusi(target, format, out_dir)`: Mengemas biner mandiri distribusi (MSI, AppImage, DMG).
