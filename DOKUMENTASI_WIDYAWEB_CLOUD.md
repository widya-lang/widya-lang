# 🌐 WidyaWeb Cloud: Sovereign Edge & Serverless Platform (Vercel-Equivalent)

**WidyaWeb Cloud** adalah platform komputasi edge dan hosting serverless berdaulat (*Sovereign Frontend-as-a-Service & Edge Cloud*) yang dirancang khusus untuk ekosistem **Widya-Lang** dan **WidyaWeb Framework**. 

Platform ini menghadirkan pengalaman pengembangan kelas dunia yang serupa dengan **Vercel**, namun berjalan dengan **100% Kedaulatan Digital Penuh (*Zero External Dependency*)**, performa eksekusi native sub-milidetik, serta jaminan penyimpanan dan komputasi di jaringan edge nasional.

---

## 🚀 Perbandingan Fitur: Vercel vs WidyaWeb Cloud

| Fitur Utama | Vercel | WidyaWeb Cloud |
| :--- | :--- | :--- |
| **Model Deployment** | Atomic Content-Addressed Git CI/CD | **Atomic SHA-256 Merkle Snapshot (Instant)** |
| **Lingkungan Pratinjau** | Instant Preview URLs per PR | **Preview URL Instan (`dpl_xxx`) & Subdomain** |
| **Serverless Runtime** | Node.js, Python, Go, Wasm | **Widya-Lang Edge Native (.wya) & WebAssembly** |
| **Arsitektur Jaringan** | Global Anycast Edge CDN | **Sovereign Regional Edge (JKT1 Primary PoP)** |
| **Waktu Respon Caching** | Stale-While-Revalidate (SWR) | **SWR Bawaan + Smart ETag + Zero-Copy Caching** |
| **Kecepatan Rollback** | Instant One-Click Rollback | **Zero-Downtime Pointer Shift (< 1 milidetik)** |
| **Brankas Rahasia (Env)** | Project Environment Variables | **Encrypted Target-Scoped Vault (AES-256-GCM)** |
| **Analitik Web Vitals** | Real User Monitoring (LCP/FID/CLS) | **Native Edge RUM & Latensi P95/P99 Built-in** |
| **Control Plane UI** | Dashboard Web Vercel | **Glassmorphic Dark Mode Dashboard Bawaan** |
| **Kedaulatan Data** | Cloud Provider Pihak Ketiga (AWS/GCP) | **100% Mandiri, On-Premise & Sovereign Ready** |

---

## 🛠️ Panduan Perintah CLI (`widya web`)

Widya-Lang menyematkan perintah `widya web` (alias `widya cloud`) langsung ke dalam binary compiler utama:

### 1. Inisialisasi Proyek Baru
Membuat struktur proyek web modern siap deploy lengkap dengan `widyaweb.json`, direktori `public/`, dan serverless endpoint `api/`:
```bash
widya web init AplikasiTokoSaya
cd AplikasiTokoSaya
```

### 2. Menjalankan Emulator Edge Lokal (Live Dev)
Menjalankan server simulasi gateway edge dengan hot-reload dan akses dashboard lokal:
```bash
widya web dev --port 3000
```
- **Aplikasi Web**: `http://localhost:3000`
- **Control Plane**: `http://localhost:3000/_widya/dashboard`

### 3. Deploy Atomik (Preview & Production)
- **Deploy Pratinjau (Preview)**:
  ```bash
  widya web deploy
  # atau
  widya deploy
  ```
  Menghasilkan URL pratinjau instan (contoh: `http://localhost:8088/?_widya_deploy=dpl_a8f9c12b`).

- **Deploy Produksi Utama**:
  ```bash
  widya web deploy --prod
  ```
  Mengalihkan domain produksi utama langsung ke deployment terbaru tanpa *downtime*.

### 4. Melihat Riwayat Deployment
```bash
widya web ls --proyek AplikasiTokoSaya
```

### 5. Rollback Instan (Zero Downtime)
Kembalikan lalu lintas produksi ke deployment ID sebelumnya secara instan:
```bash
widya web rollback AplikasiTokoSaya dpl_a8f9c12b
```

### 6. Manajemen Variabel Lingkungan & Secrets
```bash
# Simpan secret untuk Production
widya web env set API_KEY "kunci_rahasia_bank_2026" --proyek AplikasiTokoSaya --prod

# Simpan secret untuk Preview
widya web env set API_KEY "kunci_testing_sandbox" --proyek AplikasiTokoSaya

# Lihat daftar secret
widya web env ls --proyek AplikasiTokoSaya

# Hapus secret
widya web env rm API_KEY --proyek AplikasiTokoSaya --prod
```

### 7. Pantau Telemetri Edge & Core Web Vitals
```bash
widya web metrics --proyek AplikasiTokoSaya
```
Menampilkan laporan real-time:
- Throughput permintaan & pembagian status HTTP (2xx, 3xx, 4xx, 5xx)
- Latensi P95 dan P99 Edge Gateway
- Cache Hit Rate persentase
- Core Web Vitals (LCP, FID/INP, CLS, TTFB)

### 8. Membuka Dashboard Control Plane Interaktif
```bash
widya web dashboard --port 8088
```

---

## 📄 Spesifikasi Konfigurasi `widyaweb.json`

Mirip dengan `vercel.json`, berkas `widyaweb.json` mengontrol seluruh aspek perutean, pengalihan (*redirects*), penulisan ulang rute (*rewrites*), header keamanan kustom, penjadwalan cron, dan pengaturan fungsi serverless:

```json
{
  "name": "portal-layanan-publik",
  "version": "1.0.0",
  "framework": "widyaweb",
  "public_dir": "public",
  "api_dir": "api",
  "redirects": [
    {
      "source": "/layanan-lama",
      "destination": "/layanan",
      "status": 307
    }
  ],
  "rewrites": [
    {
      "source": "/warga/:nik",
      "destination": "/api/profil_warga"
    }
  ],
  "headers": [
    {
      "source": "/(.*)",
      "headers": {
        "X-Content-Type-Options": "nosniff",
        "X-Frame-Options": "DENY",
        "Strict-Transport-Security": "max-age=63072000; includeSubDomains; preload"
      }
    }
  ],
  "crons": [
    {
      "path": "/api/sinkronisasi_data",
      "schedule": "0 2 * * *"
    }
  ],
  "functions": {
    "/api/verifikasi_ktp": {
      "memory_mb": 256,
      "max_duration_sec": 10,
      "runtime": "widya-edge"
    }
  },
  "regions": ["jkt1", "sgp1", "syd1", "fra1", "iad1"],
  "env": {
    "NODE_ENV": "production",
    "EDGE_REGION": "jkt1"
  }
}
```

---

## ⚡ Menulis Serverless Edge Function (`api/halo.wya`)

Fungsi serverless di dalam folder `api/` dieksekusi secara instan saat rute `/api/<nama_berkas>` dipanggil:

```widya
// Berkas: api/kurs.wya
// Mengakses objek 'permintaan' bawaan (metode, jalur, kueri, badan)

fungsi hitung_kurs_fintech() {
    tetap simbol = permintaan["kueri"]["simbol"];
    
    jika (simbol == "USD") {
        kembalikan {
            "status": "sukses",
            "mata_uang": "USD/IDR",
            "nilai": 16250.0,
            "region": "jkt1-edge",
            "waktu": "2026-09-30T18:00:00Z"
        };
    } selain itu {
        kembalikan {
            "status": "sukses",
            "mata_uang": "SGD/IDR",
            "nilai": 12100.0,
            "region": "jkt1-edge",
            "waktu": "2026-09-30T18:00:00Z"
        };
    }
}

hitung_kurs_fintech();
```

---

## 🖥️ Antarmuka Dashboard Control Plane

Dashboard Control Plane interaktif terintegrasi di port gateway (`/_widya/dashboard`) dan menyediakan:
- **Metrik Kinerja Real-Time**: Status 2xx/3xx/4xx/5xx, Bandwidth, Cache Hit Rate.
- **Tampilan Core Web Vitals RUM**:
  - **LCP (Largest Contentful Paint)**: Render visual utama dalam < 500ms.
  - **FID / INP**: Responsivitas antarmuka dalam < 15ms.
  - **CLS**: Stabilitas layout bebas pergeseran visual (< 0.01).
  - **TTFB**: Waktu terima byte pertama super cepat (< 45ms).
- **Riwayat Snapshot Atomik**: Daftar semua deployment, SHA checksum, pratinjau langsung, dan tombol Rollback sekali klik.
- **Topologi Jaringan Edge PoP**: Visualisasi status replikasi PoP Jakarta (JKT1 Core), Singapura (SGP1), Sydney (SYD1), Frankfurt (FRA1), dan Virginia (IAD1).
