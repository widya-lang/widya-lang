# Lampiran C: Parameter Konfigurasi `widya_desktop.json` & Manifest Keamanan

Lampiran ini menyajikan spesifikasi skema konfigurasi terpusat berkas `widya_desktop.json`, parameter lingkungan produksi, serta pengaturan penegakan keamanan *Strict Content Security Policy*.

---

## C.1 Skema Lengkap Berkas Konfigurasi `widya_desktop.json`

Setiap proyek aplikasi WidyaDesktop dikendalikan oleh berkas konfigurasi utama `widya_desktop.json` yang menetapkan perilaku jendela, jembatan IPC, periferal perangkat keras, dan kebijakan keamanan:

```json
{
  "name": "NusantaraPOS",
  "app_id": "id.widya.aplikasi.kasir",
  "version": "1.0.0",
  "author": "Tim Riset & Pengembang Widya-Lang",
  "description": "Stasiun Kerja Kasir POS & Industri Berdaulat Enterprise",
  "window": {
    "title": "Nusantara POS Enterprise Workstation",
    "width": 1280,
    "height": 800,
    "min_width": 1024,
    "min_height": 720,
    "resizable": true,
    "centered": true,
    "fullscreen": false,
    "frameless": false,
    "dark_mode": true,
    "icon": "assets/icon.png",
    "splash_screen": true
  },
  "menu": {
    "enabled": true,
    "items": [
      { "label": "Transaksi Baru", "shortcut": "F2", "action": "transaksi_baru" },
      { "label": "Buka Laci Kasir", "shortcut": "F8", "action": "buka_laci" },
      { "label": "Audit Keamanan", "shortcut": "Ctrl+Shift+A", "action": "audit_keamanan" },
      { "label": "Keluar", "shortcut": "Alt+F4", "action": "keluar_aplikasi" }
    ]
  },
  "tray": {
    "enabled": true,
    "icon": "assets/tray_icon.png",
    "tooltip": "Nusantara POS - Layanan Latar Belakang Aktif",
    "minimize_to_tray": true
  },
  "bridge": {
    "port": 9095,
    "transport": "anonymous_pipe",
    "zero_copy": true,
    "timeout_ms": 5000
  },
  "security": {
    "require_ipc_token": true,
    "csp_enabled": true,
    "asset_integrity_check": true,
    "strict_origin": true,
    "encrypted_storage": true
  },
  "industrial": {
    "security_grade": "Enterprise Grade (A+)",
    "pos_hardware": {
      "printer_type": "esc_pos_thermal",
      "paper_width_mm": 58,
      "serial_baud_rate": 9600,
      "scale_protocol": "toledo_continuous_ascii"
    },
    "scada_modbus": {
      "enabled": true,
      "transport": "rs485_rtu",
      "default_baud": 115200,
      "parity": "none"
    }
  },
  "targets": [
    "windows",
    "linux",
    "macos"
  ]
}
```

---

## C.2 Tabel Kamus Properti Konfigurasi

| Blok Properti | Kunci Konfigurasi | Tipe Data | Nilai Bawaan (Default) | Penjelasan Teknis & Rekomendasi Industri |
| :--- | :--- | :--- | :--- | :--- |
| **window** | `title` | String | `"Aplikasi Desktop Widya"` | Judul teks resmi pada bilah judul jendela dan taskbar. |
| | `width` / `height` | Bilangan Bulat | `1100` / `720` | Dimensi resolusi awal jendela saat pertama kali dibuka. |
| | `min_width` / `min_height` | Bilangan Bulat | `800` / `600` | Batas dimensi fisik minimum yang ditegakkan mesin geometri. |
| | `frameless` | Boolean | `false` | Menghilangkan bilah judul standar sistem operasi untuk kustomisasi penuh. |
| | `dark_mode` | Boolean | `true` | Mengaktifkan tema palet gelap hemat energi secara default. |
| | `splash_screen` | Boolean | `true` | Menampilkan layar percik dengan transisi memudar halus 300 ms. |
| **menu** | `enabled` | Boolean | `true` | Mengaktifkan bilah menu native sistem operasi. |
| | `items` | Array Objek | 4 Item Standar | Daftar menu cepat beserta label, pintasan keyboard, dan handler. |
| **tray** | `enabled` | Boolean | `true` | Mengaktifkan ikon layanan di baki sistem (Notification Area / Status Bar).|
| | `minimize_to_tray` | Boolean | `true` | Menyembunyikan jendela ke baki sistem saat tombol tutup diklik kasir. |
| **bridge** | `transport` | String | `"anonymous_pipe"` | Mekanisme saluran IPC: pipa anonim memori bebas port jaringan. |
| | `timeout_ms` | Bilangan Bulat | `5000` | Ambang batas pembatalan RPC jika periferal fisik tidak merespons (ms). |
| **security**| `require_ipc_token` | Boolean | `true` | Mewajibkan validasi token kriptografis `X-Widya-IPC-Token` per jendela. |
| | `csp_enabled` | Boolean | `true` | Menegakkan Content Security Policy ketat anti-injeksi skrip liar. |
| | `asset_integrity_check` | Boolean | `true` | Memverifikasi hash SHA-256 seluruh berkas terhadap `widya_security.lock`.|
| | `encrypted_storage` | Boolean | `true` | Mengenkripsi seluruh berkas kredensial offline menggunakan AES-256-GCM. |
| **industrial**| `printer_type` | String | `"esc_pos_thermal"`| Format driver biner pencetak struk kasir thermal POS. |
| | `paper_width_mm` | Bilangan Bulat | `58` atau `80` | Lebar fisik kertas struk thermal (58 mm = 32 kolom karakter). |
| | `serial_baud_rate` | Bilangan Bulat | `9600` | Laju transmisi serial standar untuk komunikasi timbangan kasir. |

---

## C.3 Tajuk Keamanan & Aturan Strict Content Security Policy (CSP)

Berikut adalah tajuk kebijakan keamanan HTTP/Meta yang disematkan secara otomatis oleh mesin perender WidyaDesktop pada berkas `ui/index.html`:

```html
<meta http-equiv="Content-Security-Policy" content="
    default-src 'self';
    script-src 'self' 'nonce-WIDYAD3SKT0P2026';
    style-src 'self' 'unsafe-inline';
    img-src 'self' data: blob:;
    font-src 'self';
    connect-src 'self';
    media-src 'self';
    object-src 'none';
    child-src 'none';
    frame-src 'none';
    worker-src 'self';
    form-action 'self';
    base-uri 'none';
">
```

### Penjelasan Direktif Keamanan:
1. `default-src 'self';`: Sumber daya apa pun yang tidak didefinisikan secara eksplisit hanya diizinkan dimuat dari asal aplikasi lokal internal yang sah.
2. `script-src 'self' 'nonce-...';`: Memblokir eksekusi seluruh skrip sebaris tanpa nonce dan melarang injeksi skrip dari jaringan luar, serta memblokir pemanggilan fungsi dinamis `eval()`.
3. `object-src 'none';` dan `frame-src 'none';`: Melarang pemuatan elemen `<object>`, `<embed>`, atau `<iframe>` yang rentan disusupi eksploitasi pembajakan tampilan (*Clickjacking*).
4. `base-uri 'none';`: Mencegah manipulasi tag `<base>` yang dapat mengarahkan resolusi URL relatif ke domain berbahaya penyerang.
