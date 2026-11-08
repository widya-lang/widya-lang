# Panduan Hosting & Deployment Portal widya-lang.org

Direktori ini berisi seluruh konten statis produksi, aset unduhan rilis biner resmi **v0.1.0**, serta template konfigurasi web server untuk menjalankan portal resmi **`widya-lang.org`** pada server mandiri (VPS/Dedicated Server).

---

## 📁 Struktur Berkas

```
website/
├── index.html                  # Beranda utama (Hero, Fitur Inti, Showcase, Unduhan v0.1.0)
├── get-started.html            # Panduan instalasi cepat (Windows, Linux, VS Code)
├── playground.html             # Interactive Web Playground (Evaluasi kode di browser)
├── lms.html                    # Akademi LMS & Ujian CBT Interaktif (Sertifikasi WCA)
├── apps.html                   # Galeri Showcase Aplikasi & Ekosistem
├── resources.html              # Pusat Dokumentasi & Lembar Contekan (Cheatsheet)
├── use-cases.html              # Studi kasus rekayasa sistem industri
├── styles.css                  # Stylesheet master dengan "Tema Kalem" & Dark/Light mode
├── main.js                     # Logika tema, clipboard, dan navigasi
├── CNAME                       # Domain kustom (widya-lang.org)
├── robots.txt                  # Pengindeksan mesin pencari
├── sitemap.xml                 # Peta situs resmi
├── .well-known/security.txt    # Kebijakan pelaporan kerentanan RFC 9116
├── downloads/                  # Aset biner rilis resmi
│   ├── widya-v0.1.0-windows-x64.zip  (18.17 MB)
│   ├── widya-lang-0.1.0.vsix          (Ekstensi resmi VS Code)
│   ├── 01_TUTORIAL_LEVEL_DASAR.pdf    (Buku Cetak ISO 32000-1, 229 Halaman)
│   ├── 02_TUTORIAL_LEVEL_LANJUT.pdf   (Buku Cetak ISO 32000-1)
│   └── SHA256SUMS.txt                 (Hash verifikasi integritas kriptografis)
├── nginx.conf                  # Konfigurasi virtual host Nginx (SSL, HTTP/2, Gzip)
├── Caddyfile                   # Konfigurasi Caddy (Otomatis HTTPS Let's Encrypt)
├── Dockerfile                  # Container Nginx Alpine
├── docker-compose.yml          # Konfigurasi Docker Compose 1-perintah
└── deploy.sh                   # Script deployment otomatis via rsync/ssh
```

---

## 🚀 3 Pilihan Cara Hosting di Server Anda

### Pilihan 1: Menggunakan Nginx (Rekomendasi Tradisional)

1. Salin seluruh isi folder `website/` ke server Anda di `/var/www/widya-lang.org`:
   ```bash
   # Dari komputer lokal:
   rsync -avz website/ user@ip-server-anda:/var/www/widya-lang.org/
   ```

2. Pasang konfigurasi Nginx:
   ```bash
   sudo cp /var/www/widya-lang.org/nginx.conf /etc/nginx/sites-available/widya-lang.org
   sudo ln -s /etc/nginx/sites-available/widya-lang.org /etc/nginx/sites-enabled/
   ```

3. Pasang sertifikat SSL gratis (Let's Encrypt):
   ```bash
   sudo certbot --nginx -d widya-lang.org -d www.widya-lang.org
   ```

4. Uji dan muat ulang Nginx:
   ```bash
   sudo nginx -t
   sudo systemctl reload nginx
   ```

---

### Pilihan 2: Menggunakan Caddy (Paling Mudah, Otomatis HTTPS)

Jika server Anda menggunakan Caddy Web Server:
```bash
# Salin berkas ke /var/www/widya-lang.org
rsync -avz website/ user@ip-server-anda:/var/www/widya-lang.org/

# Salin Caddyfile
sudo cp /var/www/widya-lang.org/Caddyfile /etc/caddy/Caddyfile

# Muat ulang Caddy (SSL otomatis diurus oleh Caddy)
sudo systemctl reload caddy
```

---

### Pilihan 3: Menggunakan Docker Compose (1-Klik Container)

Jika server Anda telah terpasang Docker & Docker Compose:
```bash
cd /var/www/widya-lang.org
docker compose up -d --build
```

---

## 🔄 Pembaruan Cepat (1-Klik Deployment)

Anda dapat menggunakan script `deploy.sh` yang sudah disediakan:
```bash
# Jalankan dari direktori website:
./deploy.sh root@103.xxx.xxx.xxx
```

---

## 🎨 Karakteristik "Tema Kalem"

1. **Warna Dominan Menyejukkan**: Menggunakan palet **Sage Emerald** (`#10b981`), **Deep Slate** (`#0b0f19`), dan **Muted Indigo** (`#6366f1`) yang tenang, tidak menusuk mata, dan bebas efek neon berlebihan.
2. **Dukungan Dark & Light Mode**: Tombol toggle di navbar memungkinkan pengunjung memilih mode gelap lembut atau mode terang netral kertas (*warm paper/sand*).
3. **Tipografi Bersih**: Memakai kombinasi modern *Plus Jakarta Sans* untuk keterbacaan artikel dan *JetBrains Mono* untuk terminal/kode program.
4. **Ringan & Cepat**: 100% Vanilla HTML, CSS murni, dan JS zero-framework, memuat halaman dalam waktu < 50 milidetik.
