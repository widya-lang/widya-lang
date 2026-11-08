# Spesifikasi Display Server & GPU Compositor (WidyaCompositor) 🖥️

> **Perenderan Grafis Tingkat Rendah Bebas X11 & Wayland Warisan**  
> *Linux DRM/KMS Direct Framebuffer · 120 FPS V-Sync · Efek Kaca Mica/Acrylic GPU Shader*

---

## 1. Pendahuluan & Masalah Display Server Tradisional

Dalam ekosistem grafis desktop Linux konvensional, tumpukan server grafis sering kali menjadi sumber masalah performa dan keamanan:
- **X11 (X Window System):** Arsitektur kuno sejak tahun 1984 yang memiliki cacat keamanan mendasar (*setiap aplikasi dapat menyadap penekanan tombol/keylogger atau menangkap layar aplikasi lain tanpa izin*), serta sering mengalami patahan gambar (*screen tearing*).
- **Kompositor Wayland Berat (Mutter / KWin):** Menuntut ratusan dependensi pustaka C++ pihak ketiga yang rumit dan berat.

**WidyaOS** mengimplementasikan **WidyaCompositor**: sebuah engine kompositor grafis berdaulat murni dalam Rust/Widya-Lang yang langsung mengontrol hardware grafis melalui subsistem **Linux DRM (Direct Rendering Manager)** dan **KMS (Kernel Mode Setting)**.

---

## 2. Diagram Alur Rendering Grafis (Zero-Overhead Graphics Pipeline)

```
┌────────────────────────────────────────────────────────┐
│           APLIKASI DESKTOP (WidyaWriter / Calc)        │
│    (Menghasilkan Perubahan Teks / Formula / Scroll)    │
└───────────────────────────┬────────────────────────────┘
                            │
               Pelacakan Area Kotor (Dirty Regions)
               (src/desktop/pure_window.rs)
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│                    WIDYACOMPOSITOR                     │
│  - Pengurutan Z-Order Jendela & Deteksi Oklusi         │
│  - Shader Efek Visual Mica Dark / Acrylic Blur (GPU)   │
│  - Komposisi Triple-Buffering Bebas Patahan (Tear-Free)│
└───────────────────────────┬────────────────────────────┘
                            │ Direct EGL / GBM Handle
┌───────────────────────────▼────────────────────────────┐
│                  LINUX DRM / KMS KERNEL                │
│  - Mode Setting Resolusi Layar & Laju Penyegaran 120Hz │
│  - Page Flipping Tersinkronisasi V-Sync Otomatis       │
└───────────────────────────┬────────────────────────────┘
                            │ Hardware Scanout
┌───────────────────────────▼────────────────────────────┐
│                    MONITOR FISIK                       │
│      (HDMI 2.1, DisplayPort 2.0, eDP Laptop Internal)  │
└────────────────────────────────────────────────────────┘
```

---

## 3. Fitur Utama Perenderan Grafis

### A. Pelacakan Area Kotor (*Dirty-Region Redraw Engine*)
Ketika pengguna mengetik satu karakter di WidyaWriter atau kursor berkedip:
- Kompositor konvensional sering kali menggambar ulang (*redraw*) seluruh layar 4K (8,3 juta piksel).
- **WidyaCompositor** hanya menggambar ulang kotak pembatas kursor (*bounding box* 16x24 piksel).
- **Hasil:** Penggunaan daya baterai laptop sangat hemat, dan latensi ketik turun drastis ke **< 8 milidetik**.

### B. Shading Visual Tingkat Lanjut (Mica & Acrylic Vibrancy)
Efek transparansi buram yang elegan pada Ribbon dan titlebar dijalankan langsung pada GPU fragment shader:
- Menggunakan algoritma *Dual Kawase Blur* yang dioptimasi untuk berjalan 120 FPS tanpa membebani CPU.
- Mengadaptasi warna dominan dari wallpaper desktop secara dinamis untuk memberikan nuansa visual yang menyatu (*harmonious aesthetic*).

### C. Manajemen Multi-Monitor Cerdas ([src/desktop/pure_display.rs](file:///home/devops/widya-lang/src/desktop/pure_display.rs))
- **Hotplug Detection:** Deteksi instan saat kabel HDMI/DisplayPort dicolokkan tanpa kedipan layar (*zero flicker*).
- **Per-Monitor Independent DPI Scaling:** Layar laptop 4K internal dapat diatur pada skala 150%, sementara monitor eksternal Full HD 1080p tetap berjalan pada skala 100% tanpa distorsi font atau efek kabur.
- **Variable Refresh Rate (VRR / FreeSync):** Menyesuaikan refresh rate layar secara dinamis (48 Hz hingga 240 Hz) untuk menghemat daya saat membaca dan meningkatkan kelancaran saat animasi.
