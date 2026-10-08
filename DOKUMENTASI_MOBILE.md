# 📱 DOKUMENTASI WIDYAMOBILE FRAMEWORK (ENTERPRISE EDITION)

**WidyaMobile** adalah framework pengembangan aplikasi seluler lintas platform (Android, iOS, dan PWA Offline-First) yang terintegrasi secara bawaan di dalam compiler dan interpreter **Widya-Lang**.

---

## 🚀 1. Fitur Utama WidyaMobile

* **Declarative WidyaUI**: Pemrograman antarmuka deklaratif yang reaktif dan ringan tanpa ketergantungan JavaScript pihak ketiga.
* **4 Template Enterprise Siap Pakai**:
  1. `standard`: Template serbaguna dengan tombol, kartu info, input teks, dan pembacaan sensor GPS & Baterai.
  2. `pos` (Point of Sale): Kasir pintar dengan scanner Barcode/QRIS, katalog menu, dan bridge printer thermal Bluetooth.
  3. `presensi` (Attendance & Smart Office): Presensi karyawan dengan GPS Geofencing, verifikasi FaceID/Biometrik, dan kamera selfie.
  4. `dashboard` (Executive Analytics): Monitoring performa kluster server & database WidyaBase dengan metrik real-time dan notifikasi.
* **Live Dev Simulator Interaktif (`widya mobile dev`)**:
  * Menjalankan simulator visual smartphone langsung di browser dengan Hot-Module Reloading (HMR).
  * Dilengkapi **Mock Hardware Dock**: Slider baterai, preset koordinat GPS, simulator QR/Barcode, verifikasi biometrik, dan log pencetakan struk thermal POS.
* **Multi-Target Compilation**:
  * **Android**: Menghasilkan proyek Kotlin + Gradle (`.apk` / `.aab`).
  * **iOS**: Menghasilkan proyek Swift + Xcode (`.ipa`).
  * **PWA & Webview**: Aplikasi instan dengan Service Worker Offline-First dan WebManifest.

---

## 🛠️ 2. Perintah Antarmuka Baris Perintah (CLI)

Semua fungsionalitas mobile dapat diakses langsung melalui executable `widya`:

### A. Inisialisasi Proyek Baru
```bash
# Inisialisasi proyek standar
widya mobile init TokoKuApp

# Inisialisasi dengan template POS/Kasir
widya mobile init TokoKopi --template pos

# Inisialisasi dengan template Presensi Karyawan
widya mobile init AbsensiKantor --template presensi

# Inisialisasi dengan template Dashboard Analytics
widya mobile init ExecutiveDashboard --template dashboard
```

### B. Menjalankan Live Simulator / Hot-Reload Server
```bash
# Menjalankan simulator di port 3000 (otomatis membuka browser)
widya mobile dev TokoKuApp

# Atau pada port kustom
widya mobile dev TokoKuApp --port 8080
```

### C. Mengompilasi Aplikasi
```bash
# Kompilasi ke PWA / Webview Standalone (Bisa langsung di-host)
widya mobile bangun TokoKuApp/src/main.wya --target webview

# Kompilasi paket Android APK
widya mobile bangun TokoKuApp/src/main.wya --target android

# Kompilasi paket iOS IPA
widya mobile bangun TokoKuApp/src/main.wya --target ios
```

### D. Audit dan Pemeriksaan Kesiapan Aplikasi
```bash
widya mobile periksa TokoKuApp
```
*Output audit mencakup validasi sintaks, kelengkapan izin privasi hardware, kesiapan manifest, dan predikat mutu (Grade A+).*

---

## 💻 3. Contoh Kode Sumber Lengkap

### Template POS / Kasir Pintar (`src/main.wya`):
```widya
// Aplikasi Mobile Point of Sale & Kasir Pintar
fungsi utama() {
    misal judul_app = "Widya Coffee POS";
    misal total_penjualan = "Rp 1.450.000";
    misal jumlah_transaksi = 34;

    kembalikan Aplikasi({
        "judul": judul_app,
        "tema": "emerald",
        "badan": Halaman({
            "navigasi": BilahAtas({
                "judul": "☕ " + judul_app,
                "warna": "#059669"
            }),
            "badan": Kolom([
                Kartu({
                    "elevasi": 3,
                    "isi": Kolom([
                        TeksWidget("📊 Ringkasan Penjualan Hari Ini:"),
                        TeksWidget("💰 Total Omzet: " + total_penjualan),
                        TeksWidget("🧾 Transaksi Berhasil: " + teks(jumlah_transaksi) + " Pesanan"),
                        Pemisah()
                    ])
                }),
                Kartu({
                    "elevasi": 2,
                    "isi": Kolom([
                        TeksWidget("🛒 Menu Terlaris:"),
                        TeksWidget("• Kopi Susu Gula Aren (Rp 18.000)"),
                        TeksWidget("• Butter Croissant Bakery (Rp 22.000)"),
                        TeksWidget("• Matcha Latte Oatmilk (Rp 25.000)"),
                        Pemisah(),
                        Tombol("📷 Pindai Barcode / QRIS Produk", fungsi() {
                            Mobile::Kamera::pindai_qr(fungsi(hasil_qr) {
                                cetak("QR Terpindai: " + hasil_qr);
                                Mobile::Perangkat::getar(80);
                            });
                        }),
                        Tombol("🖨️ Cetak Struk ke Thermal Bluetooth", fungsi() {
                            misal res = Mobile::POS::cetak_struk({
                                "toko": judul_app,
                                "total": total_penjualan,
                                "kasir": "Widya Mobile POS"
                            });
                            cetak("Status Cetak: " + res.printer);
                        })
                    ])
                }),
                BidangTeks("Nominal Pembayaran Manual (Rp)..."),
                TombolUtama("💳 Selesaikan Transaksi QRIS", fungsi() {
                    Mobile::Perangkat::getar(150);
                    Mobile::Notifikasi::tampilkan("Pembayaran Berhasil", "Struk telah dikirim ke pelanggan!");
                })
            ])
        })
    });
}
```

---

## 🌐 4. Device Hardware Bridge API

Aplikasi mobile dapat berinteraksi langsung dengan sensor native smartphone melalui antarmuka:

| Modul Bridge | Metode | Deskripsi |
| :--- | :--- | :--- |
| `Mobile::Baterai` | `ambil_level()` | Mengambil persentase baterai perangkat |
| `Mobile::GPS` | `ambil_lokasi()` | Mengambil koordinat geolokasi lintang/bujur & akurasi |
| `Mobile::Biometrik` | `verifikasi(prompt)` | Autentikasi biometrik (Sidik Jari / FaceID) |
| `Mobile::Kamera` | `ambil_foto()` / `pindai_qr(cb)` | Mengambil foto selfie atau memindai barcode/QRIS |
| `Mobile::Perangkat` | `getar(ms)` | Haptic feedback getaran perangkat |
| `Mobile::Notifikasi` | `tampilkan(judul, pesan)` | Menampilkan Push Notification & Toast Alert |
| `Mobile::POS` | `cetak_struk(data)` | Mencetak struk transaksi ke printer Bluetooth thermal 58mm |

---

## 🚀 5. WidyaMobile Next-Gen Frontiers: 6 Pilar Arsitektur Masa Depan

WidyaMobile memperkenalkan 6 lompatan teknologi mutakhir untuk memperkuat kedaulatan perangkat lunak seluler dan keandalan operasional di Indonesia:

### 🌟 6 Pilar Pengembangan Mutakhir

| No | Pilar Arsitektur | Teknologi / Standar | Manfaat Utama |
|---|---|---|---|
| **1** | **Offline-First CRDT & WidyaBase Edge Sync** | Conflict-Free Replicated Data Types (LWW-MVR) + State Vector | Aplikasi tetap beroperasi lancar di daerah 3T tanpa sinyal. Mutasi lokal otomatis digabung (*merged*) tanpa konflik saat sinyal pulih. |
| **2** | **On-Device Mobile AI & Anti-Spoofing Biometrics** | Qualcomm Hexagon QNN / Apple Neural Engine CoreML | Verifikasi liveness wajah 3D depth anti-titip absen (< 7 ms) dan ekstraksi OCR KTP-el on-device tanpa kirim foto ke cloud. |
| **3** | **SoftPOS Tap-on-Phone (NFC EMV) & BLE Mesh** | ISO/IEC 14443 Type A/B, EMV Contactless L1/L2, BLE 5.3 Mesh | Ubah smartphone jadi mesin EDC kasir contactless (Tap to Pay / QRIS Tap). Jaringan darurat P2P antar-ponsel tanpa BTS/internet. |
| **4** | **Instant App, App Clips, & Lean Binary Pipeline** | Google Play Instant & Apple App Clips (< 10 MB) | Akses cepat tanpa install dari App Store (cukup scan QR). Kompresi Zstd dan tree-shaking agresif menghasilkan biner ramping (< 8 MB). |
| **5** | **Smartwatch & Wearable OS Expansion** | Apple watchOS & Android Wear OS Companion App | Tile & Complication widget di jam tangan pintar. Telemetri sensor denyut nadi PPG, SpO2, dan Fall Detection darurat pekerja lapangan. |
| **6** | **Dynamic Island, Live Activities, & Lock Screen Widgets** | Apple ActivityKit Dynamic Island & Android Glance AppWidget | Pelacakan status transaksi, pesanan kafe, atau kurir secara real-time langsung di pulau dinamis iPhone dan widget lock screen Android. |

---

### 📚 Fungsi Bawaan (Built-in APIs) Widya-Lang

```widya
// PILAR 1: Offline CRDT & Edge Sync
misal doc = mobile_crdt_dokumen_buat("faktur_01", "transaksi");
misal mutasi = mobile_crdt_mutasi_lokal("faktur_01", "INSERT", "rec_01", "Kopi Susu");
misal sync = mobile_crdt_sinkronisasi_edge("faktur_01", 0);

// PILAR 2: On-Device Mobile AI & Liveness
misal live = mobile_ai_liveness_verifikasi("frame_face_data", 1.0);
misal ocr = mobile_ai_ktp_ocr_instan("frame_ktp_data");

// PILAR 3: SoftPOS Tap-on-Phone & BLE Mesh
misal tap = mobile_nfc_softpos_mulai_transaksi(75000, "IDR");
misal mesh_tx = mobile_ble_mesh_siarkan_pesan("kasir", "SYNC_ACK", 4);
misal inbox = mobile_ble_mesh_baca_inbox();

// PILAR 4: App Clips & Lean Binary
misal clip = mobile_appclip_konfigurasi("id.widya.pos.Clip", "https://app.widya.id/clip");
misal audit = mobile_biner_audit_ukuran("WidyaApp");

// PILAR 5: Smartwatch Tile & Telemetri Kesehatan
misal tile = mobile_wearable_sinkronisasi_tile("tile_omzet", "Omzet", "Rp 1.5M", "IDR");
misal health = mobile_wearable_telemetri_kesehatan_baca();

// PILAR 6: Dynamic Island & Live Activity
misal act = mobile_live_activity_mulai("ord_99", "Pesanan Kopi", "Sedang Dibuat");
misal upd = mobile_live_activity_perbarui("ord_99", "Siap Diambil", 100);
misal end = mobile_live_activity_akhiri("ord_99");
```

---

### 💻 Contoh Pengujian Lengkap
Skrip demonstrasi keenam pilar dapat dijalankan langsung dengan:
```bash
./target/debug/widya contoh/102_widyamobile_nextgen_frontiers.wya
```

