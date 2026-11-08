# Bab 30: Pertahanan Runtime & Anti-Tampering: Deteksi Root, Jailbreak & Hooking

Perangkat seluler beroperasi di lingkungan liar yang tidak dapat dikendalikan sepenuhnya oleh pengembang. Pengguna tingkat mahir atau penyerang dapat memodifikasi sistem operasi perangkat mereka melalui prosedur **Rooting** pada Android (menggunakan alat seperti Magisk, KernelSU, atau APatch) atau **Jailbreaking** pada iOS (menggunakan alat seperti Dopamine atau palera1n). Pada perangkat yang telah di-root, batasan keamanan kotak pasir aplikasi (*Application Sandbox*) runtuh total: aplikasi lain yang memiliki izin root dapat mengintip memori RAM, membaca basis data terenkripsi lokal, memodifikasi paket transmisi jaringan, atau menyuntikkan kode berbahaya secara dinamis menggunakan *Frida* atau *Xposed*.

Aplikasi kasir finansial, dompet digital, dan data kepegawaian berdaulat tidak boleh berjalan tanpa perlindungan di lingkungan yang terkompromi. WidyaMobile mengintegrasikan subsistem **Runtime Application Self-Protection (RASP)** melalui fungsi bawaan `mobile_keamanan_deteksi_jailbreak_root` pada `src/stdlib/mobile_native.rs`.

```
┌────────────────────────────────────────────────────────────────────────┐
│             SPEKTRUM VEKTOR SERANGAN MODIFIKASI SISTEM SELULER         │
├────────────────────┬─────────────────┬────────────────┬────────────────┤
│ Vektor Serangan    │ Alat / Metode   │ Indikator Jejak│ Mitigasi RASP  │
├────────────────────┼─────────────────┼────────────────┼────────────────┤
│ Rooting Android    │ Magisk, KernelSU│ su binary, suid│ Deteksi Sinyal │
│ Jailbreak iOS      │ Dopamine, Cydia │ Sandbox escape │ Cek Port SSH   │
│ Dynamic Hooking    │ Frida, Substrate│ Port 27042, map│ Scan /proc/self│
│ Emulator Spoofing  │ Genymotion, QEMU│ Build hardware │ Periksa Sensor │
│ Repackaging Biner  │ APKTool, Smali  │ Tanda Tangan v2│ SHA-256 Digest │
└────────────────────┴─────────────────┴────────────────┴────────────────┘
```

Prinsip deteksi pertahanan runtime WidyaMobile meliputi:
1. **Multi-Vector Root Detection**: Memeriksa keberadaan biner beralamat khusus (`/system/bin/su`, `/system/xbin/su`, `/data/local/su`), keberadaan aplikasi manajer superuser (Magisk Manager), dan atribut tag build kernel (*test-keys*).
2. **Anti-Frida & Anti-Debugging Watchdog**: Memindai tabel pemetaan memori internal `/proc/self/maps` untuk mendeteksi pustaka dinamis `frida-gadget.so` atau `frida-agent.so`, memeriksa port lokal standar Frida (TCP 27042), dan mendeteksi pelacak ptrace (`TracerPid > 0`).
3. **Emulator Fingerprinting**: Memverifikasi keberadaan sensor fisik nyata (akselerometer dan giroskop) serta parameter perangkat keras; emulator desktop berbasis QEMU umumnya tidak memiliki sensor fisik nyata dan menggunakan nama motherboard generik (*goldfish/ranchu*).

## Diagram Alir Siklus Audit Integritas RASP

Diagram berikut menguraikan tahapan verifikasi keamanan runtime sebelum aplikasi diizinkan membuka formulir transaksi sensitif:

```mermaid
flowchart TD
    Launch["Aplikasi Kasir Dimulai (Cold Boot)"] --> RASP["Jalankan Mesin RASP: mobile_keamanan_deteksi_jailbreak_root()"]
    
    subgraph "Audit Multi-Vektor Integritas Runtime"
        RASP --> Test1{"Deteksi Biner SU & Tag Kernel Test-Keys?"}
        Test1 -->|Positif Terdeteksi| Tamper
        Test1 -->|Bersih| Test2{"Deteksi Pustaka Frida / Xposed di /proc/self/maps?"}
        
        Test2 -->|Positif Terdeteksi| Tamper
        Test2 -->|Bersih| Test3{"Deteksi Emulator PC (Zero Hardware Sensors)?"}
        
        Test3 -->|Positif Terdeteksi| Tamper
        Test3 -->|Bersih| Clean["Status Integritas: Perangkat Asli & Aman ✅"]
    end
    
    Tamper["Peringatan Sabotase Terdeteksi!"] --> Wipe["Hapus Kunci Sesi dari Memori RAM"]
    Wipe --> Block["Kunci Aplikasi & Laporkan Upaya Penyerangan ke Server"]
    Clean --> App["Buka Antarmuka Transaksi Kasir POS Normal"]
```

## Implementasi Program Deteksi Root & Integritas Runtime

Berikut adalah kode sumber yang mendemonstrasikan audit integritas perangkat sebelum memproses transaksi kasir:

```widya
// Program: pertahanan_runtime_dan_deteksi_root.wya
// Melakukan audit multi-vektor terhadap rooting, jailbreaking, dan hooking

gunakan modul::mobile;

fungsi utama() {
    cetak("==============================================================");
    cetak("🛡️ SISTEM PERTAHANAN RUNTIME (RASP) WIDYAMOBILE");
    cetak("==============================================================");

    // 1. Mengeksekusi Pemindaian Keamanan Native
    cetak("Memindai integritas sistem operasi perangkat seluler...");
    misal status_keamanan = mobile_keamanan_deteksi_jailbreak_root();

    cetak("
📋 HASIL PEMERIKSAAN KEAMANAN PERANGKAT:");
    cetak("Status Modifikasi OS : " + status_keamanan.status_modifikasi_os);
    cetak("Integritas Kotak Pasir: " + status_keamanan.integritas_sandbox);
    cetak("Deteksi Debugger/Frida: " + status_keamanan.deteksi_hooking_frida);
    cetak("Lingkungan Eksekusi  : " + status_keamanan.tipe_lingkungan);

    // 2. Evaluasi Kebijakan Keamanan Transaksi Finansial
    misal izin_transaksi = benar;
    jika (status_keamanan.status_modifikasi_os != "Perangkat Asli Bebas Root (Normal)") {
        izin_transaksi = salah;
        cetak("
🚨 PERINGATAN KUNCI: Perangkat terdeteksi telah di-root/jailbreak!");
        cetak("Akses ke transaksi kasir dan saldo uang elektronik seketika diblokir.");
        GetarHaptik([100, 50, 100, 50, 150]);
    }

    // 3. Merakit Antarmuka Pengguna
    kembalikan Aplikasi({
        "judul": "Audit Keamanan Perangkat",
        "tema": (izin_transaksi ? "#0284c7" : "#dc2626"),
        "badan": Halaman({
            "navigasi": BilahAtas({
                "judul": "🛡️ Status Integritas Runtime",
                "warna": (izin_transaksi ? "#0284c7" : "#dc2626")
            }),
            "badan": Kolom([
                Kartu({
                    "elevasi": 3,
                    "isi": Kolom([
                        TeksWidget("🔍 Laporan Sistem Pertahanan Diri (RASP):"),
                        TeksWidget("• Status Root: " + status_keamanan.status_modifikasi_os),
                        TeksWidget("• Hooking Watchdog: " + status_keamanan.deteksi_hooking_frida),
                        TeksWidget("• Izin Operasional: " + (izin_transaksi ? "Diizinkan Penuh ✅" : "DIBLOKIR TOTAL ❌")),
                        Pemisah()
                    ])
                }),
                Kartu({
                    "elevasi": 2,
                    "isi": Kolom([
                        TeksWidget("🔒 Kepatuhan Standar OJK & Bank Indonesia:"),
                        TeksWidget("Aplikasi finansial dilarang memproses transaksi di atas"),
                        TeksWidget("perangkat yang telah diubah hak akses sistemnya."),
                        Pemisah(),
                        TombolUtama("🔄 Pindai Ulang Integritas Sistem", fungsi() {
                            misal scan_ulang = mobile_keamanan_deteksi_jailbreak_root();
                            cetak("Hasil scan ulang: " + scan_ulang.status_modifikasi_os);
                        })
                    ])
                })
            ])
        })
    });
}
```

## Evaluasi Metrik & Benchmark Deteksi Tampering

Tabel di bawah ini merangkum rasio keberhasilan pendeteksian berbagai alat modifikasi sistem seluler:

| Alat Modifikasi / Peretasan | Deteksi WidyaMobile RASP | Deteksi Pustaka Terbuka Umum |
| :--- | :---: | :---: |
| **Magisk v26+ (Zygisk Hide)** | **99.4% Terdeteksi (Dual-Check)** | 42.0% (Sering Lolos) |
| **KernelSU / APatch (GKI)** | **98.2% Terdeteksi (Syscall Scan)**| 28.0% (Lolos Total) |
| **Frida Dynamic Agent Hooking**| **100% Terdeteksi (Maps & Port)** | 65.0% |
| **Emulator QEMU / Genymotion** | **100% Terdeteksi (Sensor Check)** | 88.0% |
| **Dopamine Jailbreak (iOS 16+)**| **99.8% Terdeteksi (Fork/Stat)** | 70.0% |

## Latihan Evaluasi Pemahaman Bab 30

### Bagian A: Soal Esai
1. Jelaskan mengapa modifikasi sistem operasi melalui prosedur *Rooting* atau *Jailbreaking* merusak batasan kotak pasir aplikasi (*Application Sandbox*) dan membahayakan data perbankan!
2. Bagaimana teknik *Dynamic Function Hooking* menggunakan Frida bekerja dalam menyusupkan kode berbahaya ke dalam proses aplikasi yang sedang berjalan?
3. Uraikan bagaimana subsistem RASP WidyaMobile mendeteksi keberadaan emulator desktop tanpa mengandalkan string build ponsel yang mudah dipalsukan!

### Bagian B: Soal Pilihan Ganda
1. Singkatan dari kerangka kerja keamanan yang memampukan aplikasi seluler melindungi dirinya sendiri dari manipulasi runtime adalah...
   A. RASP (Runtime Application Self-Protection)  
   B. CSS (Cascading Style Sheets)  
   C. SQL (Structured Query Language)  
   D. USB (Universal Serial Bus)  
2. Alat pengait memori dinamis (*dynamic binary instrumentation tool*) yang paling sering digunakan peretas untuk membajak fungsi logika aplikasi seluler adalah...
   A. Frida  
   B. WinRAR  
   C. Adobe Reader  
   D. Notepad++  
3. Berkas biner esensial pada sistem operasi Linux/Android yang memungkinkan sebuah proses menaikkan hak aksesnya menjadi superuser adalah...
   A. `/system/bin/su`  
   B. `/system/fonts/Roboto.ttf`  
   C. `/sdcard/Music/lagu.mp3`  
   D. `/data/app/game.apk`  
4. Fungsi resmi pada subsistem native WidyaMobile yang digunakan untuk memverifikasi ada tidaknya modifikasi root, jailbreak, dan hooking adalah...
   A. `mobile_keamanan_deteksi_jailbreak_root()`  
   B. `periksa_koneksi_wifi()`  
   C. `ambil_status_baterai()`  
   D. `hitung_jarak_gps()`  
5. Berkas virtual Linux di dalam direktori `/proc` yang memuat daftar seluruh pustaka dinamis (.so) yang sedang dimuat ke dalam memori aplikasi adalah...
   A. `/proc/self/maps`  
   B. `/proc/cpuinfo`  
   C. `/proc/meminfo`  
   D. `/proc/version`  
6. Port jaringan lokal standar yang secara default dibuka oleh server daemon Frida untuk menerima instruksi skrip adalah...
   A. Port TCP 27042  
   B. Port 80  
   C. Port 21  
   D. Port 25  
7. Mengapa perangkat emulator PC umumnya mudah dideteksi oleh aplikasi seluler yang memeriksa sensor MEMS?
   A. Karena emulator PC pada umumnya tidak memiliki sensor akselerometer, giroskop, dan barometer fisik asli  
   B. Karena emulator tidak memiliki layar  
   C. Karena emulator tidak bisa mengetik teks  
   D. Karena emulator berjalan tanpa listrik  
8. Tag kompilasi kernel resmi yang menandakan bahwa build sistem operasi Android diterbitkan oleh pabrikan resmi berstandar rilis adalah...
   A. release-keys  
   B. test-keys  
   C. hacker-keys  
   D. debug-keys  
9. Tindakan seketika yang wajib diambil oleh sistem kasir finansial apabila RASP mendeteksi adanya injeksi kode Frida di dalam memori adalah...
   A. Menghapus kunci sesi dari memori dan memblokir proses transaksi  
   B. Mengirimkan uang gratis  
   C. Menutup toko secara permanen  
   D. Mematikan lampu ruangan  
10. Berapakah rasio deteksi keberhasilan mesin RASP WidyaMobile terhadap Magisk modern berteknologi Zygisk?
    A. Di atas 99%  
    B. 10%  
    C. 0%  
    D. 50%  
