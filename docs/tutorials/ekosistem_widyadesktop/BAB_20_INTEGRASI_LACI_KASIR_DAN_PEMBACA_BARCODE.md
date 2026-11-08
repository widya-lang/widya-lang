## Bab 20: Integrasi Laci Kasir (Cash Drawer Kick) & Pembaca Barcode/QR Code

### 20.1 Pengendalian Laci Kasir Fisik (*Cash Drawer Kick Pulse*)

Dalam operasional toko ritel dan koperasi konvensional, uang tunai disimpan di dalam **Laci Kasir Fisik (*Cash Drawer*)** berbahan baja yang terkunci kokoh di bawah meja kasir. Demi keamanan, laci ini tidak boleh dibuka sembarangan oleh operator; laci hanya boleh terbuka secara otomatis tepat pada saat tombol *"Selesaikan Pembayaran Tunai"* ditekan pada aplikasi kasir.

Mekanisme fisik pembukaan laci kasir tidak menggunakan motor listrik rumit, melainkan menggunakan kumparan solenoid magnetik bertegangan 12V atau 24V. Solenoid ini dihubungkan melalui kabel konektor standar telepon **RJ-11 / RJ-12** yang dicolokkan ke port *DK (Drawer Kick)* di bagian belakang printer thermal POS.

WidyaDesktop menyediakan pemicu pulsa solenoid laci kasir native melalui perintah standar ESC/POS:
- **Perintah Pulsa Solenoid (`ESC p m t1 t2` / `0x1B, 0x70, 0x00, 0x19, 0xFA`)**:
  - `0x1B 0x70`: Opcode perintah *Generate Pulse*.
  - `m (0x00)`: Pin konektor laci kasir (Pin 2 pada soket RJ-11).
  - `t1 (0x19)`: Durasi pulsa aktif dinyalakan ($25 \times 2\text{ ms} = 50\text{ ms}$).
  - `t2 (0xFA)`: Durasi jeda istirahat sebelum pulsa berikutnya ($250 \times 2\text{ ms} = 500\text{ ms}$).

Ketika pulsa listrik 50 ms ini dialirkan, medan magnet solenoid menarik pengait mekanis dan pegas baja mendorong laci kasir terbuka secara mulus seketika.

```mermaid
flowchart LR
    App["Aplikasi Kasir Widya-Lang (main.wya)"] -->|Pembayaran Tunai Dikonfirmasi| PureHW["Mesin Perangkat Keras (pure_hardware.rs)"]
    PureHW -->|Kirim Perintah Biner: ESC p 0x00 0x19 0xFA| Printer["Printer Thermal POS (Port USB/Serial)"]
    Printer -->|Kabel RJ-11 Salurkan Pulsa Listrik 24V (50ms)| Drawer["Laci Kasir Baja (Cash Drawer Solenoid)"]
    Drawer -->|Pengait Magnetik Terlepas| DrawerOpen["Laci Kasir Terbuka Otomatis (Pop-Out)"]
    
    BarcodeScanner["Pemindai Barcode / QR (USB HID Keyboard)"] -->|Interupsi Masukan Cepat < 30ms| ScannerIngestion["Pencegat Aliran Barcode (Barcode Ingestion Buffer)"]
    ScannerIngestion -->|Kirim Kode Bersih via IPC| App
```

### 20.2 Ingesti Pemindai Barcode USB HID & Kode QR

Pemindai barcode (*Barcode / QR Code Scanner*) pada umumnya beroperasi dalam dua mode:
1. **Mode Emulasi Keyboard USB HID (*Human Interface Device*)**: Pemindai bertindak seolah-olah sebagai papan ketik keyboard yang mengetikkan karakter barcode dengan kecepatan sangat tinggi (kurang dari 30 milidetik untuk 13 digit angka) dan diakhiri dengan tombol `Enter` (`\r` atau `\n`).
2. **Mode Port Serial Virtual (VCP / USB CDC)**: Mengalirkan data byte mentah langsung ke port COM virtual.

Tantangan terbesar pada aplikasi kasir desktop adalah: jika kursor pengguna tidak sedang berada di dalam kotak input teks barcode, penekanan tombol dari pemindai barcode akan hilang atau merusak input teks lainnya.

WidyaDesktop mengimplementasikan **Global Barcode Ingestion Interceptor**:
- Memantau laju masukan penekanan tombol (*keystroke timing interval*). Jika rentetan karakter masuk dengan jeda antar-tombol di bawah 20 milidetik dan diakhiri penekanan `Enter`, sistem mengenali masukan tersebut sebagai data dari pemindai barcode fisik, bukan ketikan manual manusia.
- Data barcode tersebut langsung disalurkan ke keranjang belanja kasir secara otomatis di mana pun posisi kursor aktif berada.

### 20.3 Implementasi Kode Widya-Lang & WidyaDesktop Utuh

Berikut implementasi pemicu pembukaan laci kasir dan penanganan masukan barcode otomatis pada Widya-Lang:

```widya
// Modul Pengendalian Laci Kasir & Ingesti Barcode
modul pengelola_laci_dan_barcode {
    impor "./modul/desktop.wya";

    fungsi inisialisasi_periferal_kasir() {
        // 1. Buat Aplikasi Kasir
        misal app = BuatAplikasiDesktop("Nusantara POS - Terminal Kasir", 1100, 720);

        // 2. Registrasi RPC: Buka Laci Kasir Otomatis (Cash Drawer Kick)
        app.daftarkan_rpc("buka_laci_kasir", fungsi(payload) {
            misal alasan = payload.alasan; // "transaksi_tunai" atau "buka_manual_supervisor"
            misal kasir = payload.nama_kasir;

            cetak(">>> [DRAWER-KICK] Mengirim pulsa solenoid laci kasir (ESC p 0 25 250)...");
            cetak(">>> [DRAWER-KICK] Alasan pemicuan: " + alasan + " oleh kasir: " + kasir);

            kembalikan {
                "sukses": benar,
                "status_laci": "TERBUKA_OTOMATIS",
                "stempel_waktu": "2026-10-20T08:40:12"
            };
        });

        // 3. Registrasi RPC: Ingesti Data Barcode Masuk
        app.daftarkan_rpc("proses_kode_barcode_masuk", fungsi(payload) {
            misal kode_batang = payload.kode_barcode; // misal: "8999999123456"

            cetak(">>> [BARCODE-SCAN] Menerima pemindaian barcode fisik: " + kode_batang);

            // Cari data barang di basis data lokal secara instan
            kembalikan {
                "sukses": benar,
                "barcode": kode_batang,
                "nama_barang": "Biskuit Gandum Nusantara 250g",
                "harga_satuan": 12500.0,
                "tambah_ke_keranjang": benar
            };
        });

        kembalikan app;
    }
}
```

### 20.4 Analisis Kinerja, Keamanan & Tabel Metrik Arsitektur

Tabel spesifikasi pengendalian periferal kasir fisik:

| Parameter Evaluasi | Laci Kasir Kunci Manual | Pengendalian Solenoid Native (WidyaDesktop) |
| :--- | :--- | :--- |
| **Keamanan Uang Kasir** | Rentan Lupa Mengunci / Kunci Hilang | **Terkunci Otomatis, Hanya Terbuka Saat Transaksi Tunai Sah** |
| **Audit Jejak Keamanan** | Tidak Ada Pencatatan Waktu | **Setiap Pembukaan Laci Dicatat di Audit Log Kripto** |
| **Kecepatan Layanan Kasir** | Lambat (Kasir Memutar Kunci Manual) | **Seketika (< 50 milidetik Pulsa Listrik Cepat)** |
| **Pendeteksian Barcode** | Operator Harus Mengklik Kotak Input | **Deteksi Global Otomatis Berbasis Analisis Waktu Masukan** |

### 20.5 Latihan Praktik & Studi Kasus

1. Mengapa kabel pemicu laci kasir fisik (*Cash Drawer*) umumnya dihubungkan melalui port RJ-11 di belakang printer thermal kasir, bukan dicolokkan langsung ke motherboard komputer kasir?
2. Jelaskan algoritma pembedaan masukan antara ketikan keyboard manusia dengan pemindaian scanner barcode USB HID berdasarkan analisis jeda waktu antar-karakter (*inter-keystroke interval*)!
3. Uraikan teknik audit keamanan kasir (*Cash Drawer Security Auditing*) untuk mendeteksi pembukaan laci kasir darurat tanpa adanya transaksi penjualan tunai yang tercatat!

### 20.6 Evaluasi Mandiri Pilihan Ganda

1. Perintah biner ESC/POS yang digunakan untuk memicu pulsa listrik pembukaan laci kasir otomatis adalah...
   A. `ESC p m t1 t2` (`0x1B, 0x70, ...`)
   B. `ESC @`
   C. `GS V`
   D. `CTRL ALT DEL`

2. Tipe kabel konektor fisik yang umumnya digunakan untuk menghubungkan laci kasir ke printer thermal POS adalah...
   A. RJ-11 / RJ-12 (Konektor soket telepon modular)
   B. HDMI 2.0
   C. Kabel Listrik AC PLN
   D. Kabel Audio 3.5mm

3. Durasi standar pulsa listrik solenoid laci kasir yang dialirkan agar pengait magnetik terlepas secara sempurna adalah...
   A. Sekitar 50 milidetik
   B. Tepat 24 jam
   C. 10 menit
   D. 0.00001 nanodetik

4. Tegangan listrik DC yang umum digunakan untuk mengaktifkan kumparan solenoid laci kasir ritel adalah...
   A. 12V atau 24V DC
   B. 10.000 Volt
   C. 0.1 Volt
   D. 220V AC Listrik Rumah

5. Mode kerja pemindai barcode USB yang paling umum di mana scanner bertindak seolah-olah papan ketik keyboard berkecepatan tinggi adalah...
   A. USB HID Keyboard Emulation Mode
   B. Web Camera Streaming Mode
   C. Microphone Audio Mode
   D. Bluetooth Audio Profile

6. Bagaimana algoritma cerdas WidyaDesktop membedakan antara masukan barcode fisik dengan ketikan manual staf kasir?
   A. Menganalisis jeda waktu antar-ketukan tombol (pemindai barcode menghasilkan jeda sangat cepat di bawah 20 ms antar-karakter)
   B. Menghitung jumlah huruf vokal
   C. Memeriksa warna kabel keyboard
   D. Mengukur suhu ruangan toko

7. Tombol akhiran (*suffix*) yang secara standar dikirimkan oleh pemindai barcode setelah seluruh digit angka berhasil dibaca adalah...
   A. Karakter Enter (`\r` atau `\n`)
   B. Tombol Escape
   C. Tombol Spasi
   D. Tombol Backspace

8. Mengapa pembukaan laci kasir wajib dicatat ke dalam berkas log audit terenkripsi?
   A. Mencegah kecurangan atau pencurian uang tunai oleh operator toko (*fraud prevention*)
   B. Mengetahui apakah laci kasir berkarat
   C. Mengukur berat laci kasir
   D. Menghitung jumlah koin yang ada

9. Apa keuntungan dari fitur *Global Barcode Ingestion Interceptor* pada stasiun kerja kasir WidyaDesktop?
   A. Operator kasir dapat memindai barang kapan saja tanpa harus memastikan kursor mouse sedang aktif di dalam kotak input teks barcode
   B. Mematikan seluruh scanner palsu
   C. Mengubah kode barcode menjadi uang tunai
   D. Menghilangkan kebutuhan barcode pada barang

10. Mengapa laci kasir tidak boleh dibuka sembarangan saat transaksi pembayaran menggunakan kartu debit atau QRIS?
    A. Mencegah manipulasi pencatatan uang tunai fisik di kasir toko (*reconciliation compliance*)
    B. Supaya laci kasir tidak cepat aus
    C. Mengurangi bunyi bising di dalam toko
    D. Mempercepat koneksi internet toko
