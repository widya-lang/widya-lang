# Lampiran C: Parameter Konfigurasi, Security Manifest & Panduan Rilis

Lampiran ini menyajikan spesifikasi berkas manifest Android (`AndroidManifest.xml`), manifes properti Apple iOS (`Info.plist`), kamus izin privasi sistem operasi, parameter optimasi runtime seluler, serta panduan langkah demi langkah rilis produksi.

---

## 1. Templat Manifes Android Mandiri (`AndroidManifest.xml`)

Kompiler mandiri `pure_apk.rs` menghasilkan berkas biner AXML berdasarkan templat konfigurasi standar berikut:

```xml
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="id.co.widya.kasirnusantara"
    android:versionCode="100"
    android:versionName="1.0.0-berdaulat">

    <!-- 1. Hak Akses Jaringan & Komunikasi -->
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />

    <!-- 2. Hak Akses Geospasial & Lokasi -->
    <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
    <uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />

    <!-- 3. Hak Akses Perangkat Keras Optik & Biometrik -->
    <uses-permission android:name="android.permission.CAMERA" />
    <uses-permission android:name="android.permission.USE_BIOMETRIC" />
    <uses-permission android:name="android.permission.VIBRATE" />

    <!-- 4. Hak Akses Nirkabel Jarak Dekat (NFC & BLE) -->
    <uses-permission android:name="android.permission.NFC" />
    <uses-permission android:name="android.permission.BLUETOOTH" />
    <uses-permission android:name="android.permission.BLUETOOTH_ADMIN" />
    <uses-permission android:name="android.permission.BLUETOOTH_SCAN" />
    <uses-permission android:name="android.permission.BLUETOOTH_CONNECT" />

    <!-- 5. Hak Akses Notifikasi & Latar Belakang -->
    <uses-permission android:name="android.permission.POST_NOTIFICATIONS" />
    <uses-permission android:name="android.permission.SCHEDULE_EXACT_ALARM" />
    <uses-permission android:name="android.permission.WAKE_LOCK" />

    <!-- 6. Fitur Perangkat Keras Opsional (Hardware Features) -->
    <uses-feature android:name="android.hardware.camera" android:required="false" />
    <uses-feature android:name="android.hardware.nfc" android:required="false" />
    <uses-feature android:name="android.hardware.bluetooth_le" android:required="false" />

    <application
        android:label="Nusantara Kasir POS"
        android:icon="@drawable/ic_launcher"
        android:hasCode="true"
        android:extractNativeLibs="true"
        android:hardwareAccelerated="true"
        android:networkSecurityConfig="@xml/network_security_config">

        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:configChanges="orientation|screenSize|screenLayout|keyboardHidden"
            android:windowSoftInputMode="adjustResize">
            
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>

            <!-- Integrasi Universal Links / Deep Link Scheme -->
            <intent-filter android:autoVerify="true">
                <action android:name="android.intent.action.VIEW" />
                <category android:name="android.intent.category.DEFAULT" />
                <category android:name="android.intent.category.BROWSABLE" />
                <data android:scheme="nusantara" android:host="kasir" />
            </intent-filter>
        </activity>

        <!-- Penyedia Berkas Aman Antar-Aplikasi -->
        <provider
            android:name="androidx.core.content.FileProvider"
            android:authorities="id.co.widya.kasirnusantara.fileprovider"
            android:exported="false"
            android:grantUriPermissions="true">
            <meta-data
                android:name="android.support.FILE_PROVIDER_PATHS"
                android:resource="@xml/file_paths" />
        </provider>

    </application>
</manifest>
```

---

## 2. Templat Manifes Apple iOS (`Info.plist`)

Kompiler mandiri `pure_ipa.rs` menghasilkan berkas biner Apple Plist berdasarkan templat berikut:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>id.co.widya.kasirnusantara</string>
    <key>CFBundleName</key>
    <string>NusantaraKasir</string>
    <key>CFBundleDisplayName</key>
    <string>Nusantara Kasir POS</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleExecutable</key>
    <string>NusantaraKasir</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>
    <key>UIRequiredDeviceCapabilities</key>
    <array>
        <string>arm64</string>
    </array>

    <!-- Deklarasi Alasan Privasi Penggunaan Perangkat Keras -->
    <key>NSCameraUsageDescription</key>
    <string>Kamera diperlukan untuk memindai kode QRIS pembayaran dan barcode produk.</string>
    <key>NSLocationWhenInUseUsageDescription</key>
    <string>Lokasi GPS diperlukan untuk memvalidasi batas kehadiran presensi kasir.</string>
    <key>NSFaceIDUsageDescription</key>
    <string>FaceID digunakan untuk otentikasi biometrik transaksi bernilai tinggi.</string>
    <key>NSBluetoothAlwaysUsageDescription</key>
    <string>Bluetooth diperlukan untuk mencetak struk belanja ke printer thermal POS.</string>
    <key>NFCReaderUsageDescription</key>
    <string>Sensor NFC digunakan untuk menerima pembayaran kartu debit tanpa kontak.</string>

    <!-- Dukungan Live Activities & Layar Sentuh -->
    <key>NSSupportsLiveActivities</key>
    <true/>
    <key>CADisableMinimumFrameDurationOnPhone</key>
    <true/>
</dict>
</plist>
```

---

## 3. Kamus Parameter Konfigurasi Runtime Produksi

Tabel berikut merangkum konstanta runtime yang dapat disesuaikan pada berkas konfigurasi proyek `widya_mobile.toml`:

| Parameter Runtime | Nilai Baku (Default) | Pilihan Nilai | Dampak Sistem & Keterangan |
| :--- | :---: | :---: | :--- |
| `touch_sampling_rate` | `240.0` | 60.0, 120.0, 240.0 | Laju pengambilan sampel sentuh sensor layar dalam Hertz. |
| `spring_inertia` | `0.85` | 0.10 s/d 0.98 | Koefisien luncuran inersia fisika pegas saat sapuan jari. |
| `max_slop_distance_px`| `8.0` | 4.0 s/d 16.0 | Jarak toleransi pergeseran sebelum ketukan menjadi seretan. |
| `db_page_size_bytes` | `4096` | 1024, 4096, 8192 | Ukuran satu sektor halaman enkripsi basis data AES-256. |
| `db_kdf_iterations` | `64000` | 10000 s/d 100000 | Jumlah putaran fungsi PBKDF2 peregangan kunci master. |
| `clipboard_timeout_s` | `60` | 15 s/d 300 detik | Waktu kedaluwarsa otomatis data sensitif dari papan klip. |
| `ble_mtu_size` | `512` | 23 s/d 512 byte | Ukuran paket negosiasi MTU Bluetooth Low Energy. |
| `thermal_cutoff_c` | `42.0` | 40.0 s/d 48.0 °C | Ambang batas suhu baterai untuk memicu thermal throttling. |
| `max_outbox_retries` | `5` | 1 s/d 20 kali | Batas maksimal percobaan ulang sebelum aksi dipindah ke DLQ. |
| `ssl_strict_enforce` | `true` | true / false | Menolak koneksi secara mutlak jika SSL Pinning tidak cocok. |

---

## 4. Panduan Langkah Demi Langkah Rilis Produksi

Sebelum mendistribusikan paket biner ke jutaan pengguna, pastikan seluruh tahapan checklist keamanan berikut telah terpenuhi:

1. **Audit Biner Mandiri**:
   ```bash
   widya mobile audit
   ```
   Pastikan skor kepatuhan 10 pilar mencapai 100/100 dengan status *Enterprise Grade A+*.

2. **Kompilasi Biner APK Rilis (Android)**:
   ```bash
   cargo run --release --bin widya -- mobile build --android --release
   ```
   Biner APK mandiri akan terbentuk di `build/` dengan ukuran < 12 MB dan bertanda tangan sah v1.

3. **Kompilasi Biner IPA Rilis (iOS)**:
   ```bash
   cargo run --release --bin widya -- mobile build --ios --release
   ```
   Biner IPA Mach-O ARM64 murni akan dibangkitkan langsung di Linux server tanpa memerlukan komputer Mac.

4. **Verifikasi Hash Kriptografi**:
   Hitung nilai SHA-256 berkas APK dan IPA, lalu publikasikan di portal pengembang resmi guna menjamin kedaulatan perangkat lunak terbebas dari serangan tampering rantai pasok.
