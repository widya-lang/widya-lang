# 🖥️ DOKUMENTASI WIDYADESKTOP FRAMEWORK (ENTERPRISE & INDUSTRIAL GRADE)

**WidyaDesktop** adalah framework pembuatan aplikasi desktop native modern, berkinerja tinggi, dan berstandar keamanan militer (*Enterprise Grade A+*) yang terintegrasi secara bawaan di dalam **Widya-Lang**.

---

## 🌟 1. Keunggulan Arsitektur WidyaDesktop

* **Zero Electron Bloat**: Ukuran biner rilis sangat ramping (~10–25 MB) dengan konsumsi RAM sangat rendah (35–60 MB) dan startup instan (< 200 ms).
* **Industrial & Retail Hardware Bridge**:
  * 🖨️ **ESC/POS Thermal Receipt Generator**: Bytecode native untuk printer kasir thermal USB, Serial COM, dan Network LAN.
  * ⚖️ **Serial Port (RS-232 / COM) & Timbangan Digital**: Parser otomatis pembacaan bobot timbangan industri (Toledo / CAS / Avery).
  * 🏭 **Modbus RTU Protocol**: Pembuatan & validasi frame Modbus RTU dengan kalkulasi CRC16 otomatis (Polinomial 0xA001).
* **Multi-Monitor Display & Customer-Facing Dual Screen**:
  * Manajemen multi-layar terpisah (Layar 1 untuk Kasir/Operator, Layar 2 untuk Pelanggan / Display Iklan & Ringkasan Belanja).
* **Delta Auto-Updater (Ed25519 Cryptographic Signatures)**:
  * Pembaruan otomatis berbasis *diff patching* (hanya mengunduh berkas patch beberapa KB) terverifikasi dengan tanda tangan digital Ed25519.
* **Global System Hotkeys**:
  * Tombol pintas OS global (`Ctrl+Shift+Space`, `F12`, dll.) untuk memanggil fungsi atau overlay aplikasi.
* **Defense-in-Depth Security (Grade A+)**:
  * Token Anti-CSRF isolasi sesi IPC, Content Security Policy (Strict CSP), Anti-Path Traversal sanitization, dan Brankas Kriptografi AES-256-GCM hardware-accelerated.

---

## 🛠️ 2. Perintah CLI WidyaDesktop

```bash
# Inisialisasi proyek baru dengan template pilihan (standard, pos, dashboard)
widya desktop inisialisasi TokoKasir --template pos

# Menjalankan aplikasi secara langsung dengan Live HMR (Auto-Reload saat kode diedit)
widya desktop jalankan TokoKasir --dev

# Kompilasi aplikasi ke paket distribusi mandiri
widya desktop bangun TokoKasir --target windows # Menghasilkan .exe mandiri
widya desktop bangun TokoKasir --target linux   # Menghasilkan .bin Linux
widya desktop bangun TokoKasir --target macos   # Menghasilkan .app macOS

# Audit keamanan & kesiapan rilis (Grade A+)
widya desktop periksa TokoKasir
```

---

## 🔌 3. Antarmuka API JavaScript / IPC Native

Aplikasi desktop frontend dapat langsung memanggil kapabilitas native melalui `window.widya.panggil(metode, payload)`:

### A. Komunikasi Serial Port & Timbangan Industri
```javascript
// Membaca timbangan digital
const hasil = await window.widya.panggil('serial_baca_timbangan', {
    raw_stream: "ST,GS,+0024.75kg\r\n"
});
console.log(`Bobot: ${hasil.data.weight_kg} ${hasil.data.unit} (Status: ${hasil.data.status})`);

// Membaca register Modbus RTU PLC Industri
const modbus = await window.widya.panggil('modbus_baca_register', {
    slave_id: 1,
    addr: 100,
    count: 5
});
console.log(`Frame Hex Modbus: ${modbus.data.frame_hex}`);
```

### B. Layar Ganda Pelanggan (Dual-Screen Customer Display)
```javascript
// Mendeteksi monitor terhubung
const monitors = await window.widya.panggil('monitor_daftar');

// Membuka layar pelanggan di monitor kedua
await window.widya.panggil('monitor_buka_layar_pelanggan', {
    monitor_id: 2,
    judul: "Layar Pelanggan WidyaPOS",
    url: "customer_display.html"
});
```

### C. Papan Klip & Pencetakan Struk Thermal POS
```javascript
// Menulis ke Clipboard OS
await window.widya.panggil('papan_klip_tulis', { teks: "TRX-2026-998811" });

// Mencetak struk kasir
await window.widya.panggil('cetak_thermal_struk', {
    toko: "Widya Coffee Shop",
    alamat: "Jl. Jenderal Sudirman No. 1",
    telepon: "0812-3456-7890",
    kasir: "Budi Santoso",
    items: [
        { nama: "Kopi Susu Gula Aren", harga: 18000, jumlah: 2 },
        { nama: "Butter Croissant", harga: 22000, jumlah: 1 }
    ],
    total: 58000,
    metode_bayar: "QRIS",
    pesan_bawah: "Terima Kasih atas Kunjungan Anda!"
});
```

### D. Brankas Kriptografi AES-256-GCM & Delta Update
```javascript
// Enkripsi data sensitif ke brankas lokal
const enc = await window.widya.panggil('brankas_enkripsi', {
    plaintext: "Kredensial_Sangat_Rahasia_Widya",
    kunci: "Kunci32KarakterRahasiaPabrik2026"
});

// Dekripsi data
const dec = await window.widya.panggil('brankas_dekripsi', {
    ciphertext: enc.data.ciphertext,
    nonce: enc.data.nonce,
    kunci: "Kunci32KarakterRahasiaPabrik2026"
});
```
