# Bab 33: Pilar 1: Edge AI On-Device Vision, 3D Liveness & OCR KTP-el

Dalam lanskap komputasi seluler modern, ketergantungan pada model kecerdasan buatan berbasis awan (*Cloud AI Inference*) memunculkan tiga masalah besar: latensi transmisi jaringan (200–800 ms), biaya tagihan bandwidth peladen API yang membengkak, serta ancaman pelanggaran undang-undang pelindungan data pribadi (UU PDP) jika foto identitas nasabah dikirim ke server luar negeri. Oleh karena itu, kemampuan eksekusi inferensi model AI langsung di perangkat lokal (*Edge AI On-Device*) menjadi pilar keunggulan mutlak.

WidyaMobile mengintegrasikan subsistem akselerasi perangkat keras NPU (*Neural Processing Unit*), Apple Neural Engine (ANE), dan Qualcomm Hexagon DSP melalui fungsi bawaan `mobile_ai_liveness_verifikasi` dan `mobile_ai_ktp_ocr_instan` pada `src/stdlib/mobile_native.rs`.

Model jaringan saraf tiruan dikuantisasi secara presisi ke format terkompresi **INT8 / FP16**, memungkinkan pemrosesan visi komputer berkecepatan tinggi tanpa membebani inti prosesor utama (CPU) dan tanpa menguras daya baterai.

```
┌────────────────────────────────────────────────────────────────────────┐
│             SPEKTRUM INFERENSI MODEL EDGE AI WIDYAMOBILE               │
├────────────────────┬─────────────────┬────────────────┬────────────────┤
│ Model Visi On-Device│ Format Bobot    │ Latensi NPU    │ Ketepatan F1   │
├────────────────────┼─────────────────┼────────────────┼────────────────┤
│ 3D Face Liveness   │ INT8 Quantized  │ 12 milidetik   │ 99.85% (AntiSp)│
│ KTP-el OCR Scanner │ Tensor FP16     │ 34 milidetik   │ 99.40% Karakter│
│ Landmark Mesh 468pt│ INT8 Quantized  │ 8 milidetik    │ Sub-Piksel     │
│ Voice Denoise Filter│ MobileNet WYA  │ 4 milidetik    │ Realtime Audio │
└────────────────────┴─────────────────┴────────────────┴────────────────┘
```

Kapabilitas utama dari Edge AI On-Device WidyaMobile meliputi:
1. **Passive & Active 3D Liveness Detection**: Mendeteksi keaslian subjek manusia hidup menggunakan analisis kedalaman tekstur kulit, pantulan mikro kornea mata, dan kedipan alami guna menggagalkan serangan pemalsuan foto cetak 2D, pemutaran video di layar tablet (*screen replay attack*), maupun topeng silikon 3D.
2. **Deterministic KTP-el Field Extraction**: Mengurai nomor induk kependudukan (NIK 16 digit), nama lengkap, tempat/tanggal lahir, alamat, dan status pernikahan secara seketika dalam waktu kurang dari 40 milidetik langsung dari aliran kamera ponsel tanpa membutuhkan sebutir pun paket internet (*100% Offline OCR*).
3. **Zero Data Leakage Guarantee**: Citra foto mentah KTP dan wajah hanya diproses di dalam memori kerja terisolasi (*in-memory tensor buffer*) dan tidak pernah disimpan ke berkas penyimpanan permanen kecuali hasil ekstraksi teks yang telah disetujui pengguna.

## Diagram Alir Pipeline Inferensi Edge AI NPU

Diagram berikut menguraikan alur kerja dari lensa kamera melalui akselerator perangkat keras NPU hingga ekstraksi data teks:

```mermaid
flowchart TD
    Frame["Aliran Frame Kamera (YUV_420_888 Native Buffer)"] --> Pre["Prapemrosesan Tensor (Penskalaan 256x256 & Normalisasi INT8)"]
    
    Pre --> NPU["Akselerator NPU Native (Apple Neural Engine / Hexagon DSP)"]
    
    subgraph "Pipeline Visi On-Device Terakselerasi"
        NPU --> Face["Model Liveness 3D (Uji Kedipan, Tekstur Kulit & Pantulan)"]
        NPU --> OCR["Model Deteksi Teks KTP-el (Text Detection & Recognition)"]
    end
    
    Face --> LiveCheck{"Skor Keaslian Manusia (Liveness Score) > 0.95?"}
    LiveCheck -->|Gagal (Foto Palsu / Layar Replay)| Spoof["Tolak: Terdeteksi Upaya Manipulasi Identitas ❌"]
    LiveCheck -->|Lolos (Subjek Hidup Nyata)| ParseOCR["Ekstraksi Bidang Data KTP (NIK, Nama, Tanggal Lahir)"]
    
    OCR --> ParseOCR
    ParseOCR --> Clean["Format Nilai Teks & Validasi Checksum NIK"]
    Clean --> Output["Kirim Data Terstruktur ke Formulir Aplikasi Kasir/Bank ✅"]
```

## Implementasi Program Verifikasi Wajah & OCR KTP-el

Berikut adalah implementasi sistem registrasi identitas nasabah bank koperasi (*e-KYC On-Device*) tanpa koneksi internet:

```widya
// Program: edge_ai_liveness_dan_ocr_ktp.wya
// Mengeksekusi verifikasi keaslian wajah 3D dan ekstraksi OCR KTP-el secara lokal di NPU

gunakan modul::mobile;

fungsi utama() {
    cetak("==============================================================");
    cetak("🧠 SISTEM EDGE AI VISION & LIVENESS BERDAULAT WIDYAMOBILE");
    cetak("==============================================================");

    // 1. Eksekusi Verifikasi Wajah Hidup 3D (Liveness Detection)
    cetak("Mengarahkan kamera ke wajah pengguna. Menjalankan inferensi NPU...");
    misal data_frame_kamera = "buffer_yuv_frame_stream_01";
    misal hasil_liveness = mobile_ai_liveness_verifikasi(data_frame_kamera, "Uji_Pasif_Kedipan_dan_Tekstur");

    cetak("
👁️ HASIL VERIFIKASI 3D LIVENESS ON-DEVICE:");
    cetak("Status Keaslian    : " + hasil_liveness.status_liveness);
    cetak("Skor Keyakinan     : " + teks(hasil_liveness.skor_keyakinan));
    cetak("Waktu Inferensi NPU: " + teks(hasil_liveness.latensi_npu_ms) + " milidetik (Instan ⚡)");
    cetak("Anti-Spoofing Check: " + hasil_liveness.deteksi_serangan_palsu);

    // 2. Eksekusi Pemindaian OCR KTP-el Nasional (100% Offline)
    cetak("
Mengarahkan kamera ke dokumen fisik KTP-el. Mengekstraksi bidang teks...");
    misal data_frame_ktp = "buffer_yuv_ktp_rect_02";
    misal hasil_ocr = mobile_ai_ktp_ocr_instan(data_frame_ktp);

    cetak("
📋 HASIL EKSTRAKSI OCR KTP-EL LOKAL (OFFLINE):");
    cetak("NIK (16 Digit)     : " + hasil_ocr.nik);
    cetak("Nama Lengkap       : " + hasil_ocr.nama);
    cetak("Tempat/Tgl Lahir   : " + hasil_ocr.tempat_tanggal_lahir);
    cetak("Alamat Domisili    : " + hasil_ocr.alamat);
    cetak("Akurasi Pengenalan : " + teks(hasil_ocr.skor_akurasi_karakter) + "% Tepat");

    // 3. Memberikan Umpan Balik Taktil Keberhasilan e-KYC
    GetarHaptik([40, 50, 40]);

    // 4. Merakit Antarmuka Pengguna Hasil Pemindaian
    kembalikan Aplikasi({
        "judul": "e-KYC Berdaulat On-Device",
        "tema": "#0284c7",
        "badan": Halaman({
            "navigasi": BilahAtas({
                "judul": "🧠 Verifikasi e-KYC Cerdas",
                "warna": "#0284c7"
            }),
            "badan": Kolom([
                Kartu({
                    "elevasi": 3,
                    "isi": Kolom([
                        TeksWidget("👤 Identitas Tervalidasi NPU:"),
                        TeksWidget("NIK: " + hasil_ocr.nik),
                        TeksWidget("Nama: " + hasil_ocr.nama),
                        TeksWidget("Liveness: " + hasil_liveness.status_liveness + " (" + teks(hasil_liveness.skor_keyakinan) + ")"),
                        Pemisah()
                    ])
                }),
                Kartu({
                    "elevasi": 2,
                    "isi": Kolom([
                        TeksWidget("🔒 Kedaulatan Data UU PDP Terjamin:"),
                        TeksWidget("Foto identitas diproses 100% di dalam chip NPU ponsel"),
                        TeksWidget("tanpa pernah dikirim ke peladen luar negeri."),
                        Pemisah(),
                        TombolUtama("✅ Konfirmasi Pendaftaran Rekening", fungsi() {
                            cetak("Data identitas terdaftar secara aman di basis data lokal.");
                            GetarHaptik([50]);
                        })
                    ])
                })
            ])
        })
    });
}
```

## Evaluasi Metrik & Benchmark Inferensi Edge AI

Tabel di bawah ini membandingkan waktu respons dan kepatuhan privasi antara Edge AI WidyaMobile dengan Cloud AI Vision:

| Parameter Evaluasi | WidyaMobile Edge AI (NPU) | Cloud AI Vision API (Server Eksternal) |
| :--- | :---: | :---: |
| **Waktu Respon Total (Latency)**| **34 milidetik** | 650 - 1.400 milidetik (Tergantung Jaringan) |
| **Ketergantungan Kuota Internet**| **0% (100% Offline Tanpa Sinyal)**| Wajib Kuota Internet Cepat |
| **Biaya Operasional per Panggilan**| **Rp 0 (Memakai NPU Pengguna)**| Rp 50 - Rp 250 per API Request |
| **Kepatuhan Privasi Data UU PDP**| **100% Patuh (Zero Data Egress)**| Rentan Penyadapan & Kebocoran Cloud |
| **Beban Penggunaan CPU Inti** | **< 3.5% (Dikelola NPU Khusus)**| 12.0% (Enkoding Jaringan) |

## Latihan Evaluasi Pemahaman Bab 33

### Bagian A: Soal Esai
1. Jelaskan mengapa inferensi model kecerdasan buatan langsung di perangkat lokal (*On-Device Edge AI*) jauh lebih unggul dalam hal kepatuhan Undang-Undang Pelindungan Data Pribadi (UU PDP) dibandingkan model berbasis Cloud API!
2. Bagaimana teknik *3D Liveness Detection* membedakan antara wajah manusia asli dengan serangan tipuan (*presentation attack*) yang menggunakan foto cetak resolusi tinggi atau pemutaran video di layar tablet?
3. Uraikan teknik kuantisasi model (*Model Quantization*) dari format FP32 menjadi INT8 dan jelaskan bagaimana teknik ini memangkas ukuran bobot model jaringan saraf tanpa mengorbankan akurasi pengenalan teks KTP-el!

### Bagian B: Soal Pilihan Ganda
1. Unit pemroses perangkat keras pada prosesor seluler modern yang didesain khusus untuk mengeksekusi operasi tensor jaringan saraf tiruan secara efisien dinamakan...
   A. NPU (Neural Processing Unit) / Neural Engine  
   B. Sound Card  
   C. Floppy Controller  
   D. Hard Disk Controller  
2. Serangan pemalsuan identitas di mana penyerang memperlihatkan foto cetak korban di depan kamera disebut...
   A. Photo Print Spoofing Attack  
   B. DDoS Attack  
   C. Phishing Email  
   D. SQL Injection  
3. Fungsi resmi pada subsistem native WidyaMobile yang digunakan untuk memverifikasi keaslian wajah manusia hidup di NPU adalah...
   A. `mobile_ai_liveness_verifikasi()`  
   B. `ambil_foto_profil()`  
   C. `edit_gambar_filter()`  
   D. `perbesar_mata_kamera()`  
4. Format kuantisasi bobot model jaringan saraf yang paling efisien dalam hal rasio ukuran memori dan kecepatan inferensi pada prosesor seluler adalah...
   A. INT8 (Integer 8-bit)  
   B. Double Precision Float 64-bit  
   C. String ASCII  
   D. File Dokumen DOCX  
5. Fungsi pembantu pada modul native yang digunakan untuk mengekstrak bidang data teks KTP-el secara instan dan offline adalah...
   A. `mobile_ai_ktp_ocr_instan()`  
   B. `baca_ktp_nfc()`  
   C. `unduh_data_dukcapil()`  
   D. `ketik_nik_manual()`  
6. Berapakah jumlah digit nomor identitas kependudukan (NIK) resmi Republik Indonesia yang wajib divalidasi oleh model OCR?
   A. 16 Digit  
   B. 10 Digit  
   C. 8 Digit  
   D. 32 Digit  
7. Mengapa model Edge AI WidyaMobile dapat terus berfungsi normal di pedalaman meskipun tidak ada koneksi sinyal internet sama sekali?
   A. Karena seluruh bobot model kecerdasan buatan telah terkompilasi dan berjalan secara mandiri di dalam biner aplikasi lokal  
   B. Karena ponsel terhubung ke satelit rahasia  
   C. Karena model AI berjalan di kabel listrik  
   D. Karena KTP memancarkan sinyal Wi-Fi  
8. Berapakah latensi rata-rata pemrosesan inferensi visi liveness 3D pada NPU ponsel modern menggunakan WidyaMobile?
   A. Sekitar 12 milidetik  
   B. 30 menit  
   C. 5 detik  
   D. 1 jam  
9. Tindakan seketika yang wajib dilakukan oleh sistem perbankan apabila skor liveness wajah berada di bawah ambang batas aman (< 0.95) adalah...
   A. Menolak proses registrasi dan mencatat indikasi upaya serangan pemalsuan  
   B. Mengabaikan dan tetap menyetujui akun  
   C. Mengirimkan pinjaman uang otomatis  
   D. Mematikan mikrofon  
10. Keunggulan finansial utama penerapan Edge AI bagi pengembang aplikasi adalah...
    A. Meniadakan biaya tagihan sewa peladen GPU awan pihak ketiga yang mahal untuk setiap transaksi pemindaian dokumen  
    B. Menghilangkan pajak perusahaan  
    C. Kuota internet pengguna menjadi gratis  
    D. Mempercepat pengisian daya baterai  
