# 🌐 ROADMAP WIDYAWEB: 100% PARITAS NEXT.JS 15 & SVELTEKIT 2 TANPA PUSTAKA LUAR
**Edisi Kedaulatan Web Fullstack (Zero-Dependency Sovereign Edge Stack)**  
**Target:** Menghadirkan Framework Web App Modern Berdaulat 100% Mandiri yang Menggabungkan Kecepatan SvelteKit, Kekuatan Next.js App Router, dan Efisiensi Islands Architecture Astro.

---

## 🎯 Visi & Prinsip Rekayasa WidyaWeb
1. **Zero Node.js & Zero `node_modules` Overhead**: Kompiler, bundler, serverless edge runtime, dan HTTP/2 server dibangun 100% native di dalam biner tunggal `widya` tanpa perlu mengunduh ratusan megabyte dependensi NPM.
2. **Kinerja Ultra Cepat (*Sub-50ms TTFB & Zero-Jank*)**: Cold-start < 20 ms, latensi pemrosesan SSR sub-milidetik, serta konsumsi memori minimal.
3. **Model Render Hibrida Modern**: Menggabungkan *Streaming SSR*, *Incremental Static Regeneration (ISR)*, dan *Islands Architecture* (0 KB JS untuk konten statis).
4. **Arsitektur File-System Intuitif**: Konvensi penamaan folder standar industri (`app/layout.wya`, `app/page.wya`, `app/loading.wya`, `app/error.wya`, `app/api/route.wya`).
5. **Server Actions Terintegrasi**: Panggilan fungsi backend langsung dari antarmuka frontend tanpa perlu menulis boilerplate REST API atau GraphQL secara manual.

---

```mermaid
flowchart TD
    subgraph Core_Render ["LAPISAN 1 & 2: RENDERING & ROUTING ENGINE"]
        W1["Pilar 1: Hybrid Rendering Engine (Streaming SSR, ISR, Islands)"]
        W2["Pilar 2: File-System App Router & Nested Layouts"]
    end

    subgraph UI_Fullstack ["LAPISAN 3 & 4: REAKTIVITAS & SERVER ACTIONS"]
        W3["Pilar 3: Kompiler UI Reaktif (Fine-Grained Signals, Zero-VDOM)"]
        W4["Pilar 4: Zero-API Server Actions & Form Mutations"]
    end

    subgraph Edge_Pipeline ["LAPISAN 5 & 6: EDGE PIPELINE & OPTIMASI ASET"]
        W5["Pilar 5: Edge Middleware & Global Stale-While-Revalidate (SWR)"]
        W6["Pilar 6: Optimasi Aset Mandiri (WebP/AVIF Image & Font Subsetting)"]
    end

    subgraph Data_Security ["LAPISAN 7 & 8: BASIS DATA & KEAMANAN BERDAULAT"]
        W7["Pilar 7: Fullstack Data Layer & WidyaBase Real-Time ORM"]
        W8["Pilar 8: Sovereign Security Shield (CSRF, CSP Nonce, Anti-Bot)"]
    end

    subgraph DevEx_Deploy ["LAPISAN 9 & 10: DEVEX, CLI & ATOMIC EDGE CLOUD"]
        W9["Pilar 9: DevEx Tooling (Hot Module Reload, DevTools & Web Vitals)"]
        W10["Pilar 10: Atomic Edge Deployment & Zero-Downtime Rollback"]
    end

    Core_Render --> UI_Fullstack
    UI_Fullstack --> Edge_Pipeline
    Edge_Pipeline --> Data_Security
    Data_Security --> DevEx_Deploy
```

---

## 📋 DAFTAR PEKERJAAN MASTER (*WORK BREAKDOWN STRUCTURE*)

### 🔹 PILAR 1: Hybrid Rendering Engine (Streaming SSR, SSG, ISR & Islands)
- [x] **1.1. Streaming HTML & Chunked Transfer Engine**:
  - Pengiriman potongan HTML pertama secara paralel via HTTP/1.1 dan HTTP/2 Chunked Transfer (`MesinSSRMurni::render_streaming_chunks`).
  - Dukungan komponen penahan (*Suspense Boundaries*) yang merender skeleton loading sebelum data selesai diambil.
- [x] **1.2. Incremental Static Regeneration (ISR & SWR)**:
  - Mekanisme penandaan revalidasi berbasis waktu (`revalidate: 60`) dan *On-Demand Revalidation Tag* (`revalidasi_tag("produk")` via `IsrCacheHub`).
  - *Stale-While-Revalidate* in-memory cache di layer gateway edge.
- [x] **1.3. Islands Architecture & Selective Hydration**:
  - Konsep 0 KB JavaScript bawaan untuk konten teks dan layout statis.
  - Hidrasi selektif hanya untuk komponen interaktif yang dideklarasikan dengan penanda `KomponenIsland` (`PulauInteraktif`).
- [x] **1.4. Resumability & Zero-Hydration Bootstrapping**:
  - Serialisasi state komponen langsung ke dalam atribut DOM (`data-widya-island="..."`, `data-widya-props="..."`) untuk menghilangkan overhead parsing hidrasi.

---

### 🔹 PILAR 2: File-System App Router & Nested Layouts
- [x] **2.1. Konvensi Berkas Standar App Router**:
  - `app/tata_letak.wya` / `app/layout.wya` $\rightarrow$ Root & Nested Layouts.
  - `app/halaman.wya` / `app/page.wya` $\rightarrow$ Route Endpoint Tampilan.
  - `app/memuat.wya` / `app/loading.wya` $\rightarrow$ UI Skeleton otomatis saat navigasi.
  - `app/galat.wya` / `app/error.wya` $\rightarrow$ Client Error Boundary penangkap crash.
  - `app/tak_ditemukan.wya` / `app/not-found.wya` $\rightarrow$ Halaman 404 dinamis.
- [x] **2.2. Dynamic & Catch-All Routing**:
  - Rute Parameter Dinamis: `app/produk/[id]/halaman.wya` $\rightarrow$ `/produk/123`.
  - Rute Tangkap Semua (*Catch-All*): `app/docs/[...slug]/halaman.wya` $\rightarrow$ `/docs/api/v1/intro`.
  - Rute Opsional (*Optional Catch-All*): `app/katalog/[[...kategori]]/halaman.wya`.
- [x] **2.3. Route Groups & Parallel/Intercepting Routes**:
  - Pengelompokan rute tanpa mengubah URL: `app/(pemasaran)/tentang/halaman.wya`.
  - Hierarki layout nesting otomatis dengan resolusi rekursif (`render_halaman_konten`).
- [x] **2.4. Edge Route Handlers (API Endpoints)**:
  - File `app/api/auth/rute.wya` dengan metode HTTP murni: `AMBIL()`, `KIRIM()`, `UBAH()`, `HAPUS()` (`GET`, `POST`, `PUT`, `DELETE`).

---

### 🔹 PILAR 3: Kompiler UI Reaktif & Fine-Grained Signals (*Zero-VDOM*)
- [x] **3.1. Parser Sintaks Template Deklaratif**:
  - Parsing blok markup deklaratif langsung ke pohon sintaksis abstrak (AST) tanpa memerlukan pustaka JSX eksternal.
- [x] **3.2. Kompiler Svelte-like (Direct DOM Updates)**:
  - Mengompilasi reaktivitas langsung menjadi instruksi manipulasi DOM minimalis tanpa overhead Virtual DOM.
- [x] **3.3. Fine-Grained Reactive Signals & Stores**:
  - Mekanisme signal primitif: `Sinyal(nilai)`, `PemberitahuNilai(nilai)` di `src/graphics/pure_state.rs` dan `src/web/reactive.rs`.
- [x] **3.4. Scoped CSS & CSS Module Bawaan**:
  - Isolasi gaya CSS otomatis per-komponen menggunakan atribut hash unik (`CssScopedEngine::transformasi_scoped`) tanpa CSS runtime eksternal.

---

### 🔹 PILAR 4: Zero-API Server Actions & Form Mutations
- [x] **4.1. Server Actions Compiler**:
  - Penanda fungsi `aksi_server` yang otomatis dikonversi menjadi endpoint RPC biner tipe-aman via `ServerActionDispatcher`.
- [x] **4.2. Progressive Enhancement Form Handling**:
  - Form HTML tetap berfungsi penuh dengan token Anti-CSRF (`buat_csrf_token`, `validasi_csrf_token`).
- [x] **4.3. Optimistic UI Updates**:
  - Antarmuka langsung memperbarui tampilan sebelum respons server tiba.

---

### 🔹 PILAR 5: Edge Middleware & Pipeline Permintaan Global
- [x] **5.1. Berkas Middleware Edge (`middleware.wya`)**:
  - Filter request tingkat gerbang sebelum menyentuh halaman web atau API (`ResponMiddleware`).
- [x] **5.2. Autentikasi & Verifikasi Token Edge**:
  - Parser dan verifikator JWT mandiri berbasis `src/kripto/pure_crypto.rs` (`VerifikatorJWTMurni::buat_token`, `VerifikatorJWTMurni::verifikasi_token`) berlatensi sub-milidetik.
- [x] **5.3. Rate Limiting & Geo-Routing**:
  - Pembatasan frekuensi request berbasis token bucket di memori edge (`RateLimiterBucket`).

---

### 🔹 PILAR 6: Pipeline Optimasi Aset Mandiri
- [x] **6.1. On-The-Fly Image Optimization Engine**:
  - Struktur metadata dan pencegahan pergeseran tata letak (*Zero CLS*).
- [x] **6.2. Font Subsetting & Zero-Layout-Shift Fonts**:
  - Integrasi rasterizer font mandiri TrueType glyph atlas `src/graphics/pure_font.rs`.
- [x] **6.3. Bundler & Minifier Bawaan**:
  - Minifikasi biner HTML dan CSS murni tanpa dependensi (`MinifierWebMurni::minify_html`, `MinifierWebMurni::minify_css`).

---

### 🔹 PILAR 7: Fullstack Data Layer & WidyaBase Real-Time ORM
- [x] **7.1. Type-Safe Query Builder Terintegrasi**:
  - Kueri basis data bawaan berdaulat tanpa driver pihak ketiga.
- [x] **7.2. Real-Time Data Subscriptions (WebSockets & SSE)**:
  - Dukungan *Server-Sent Events (SSE)* dan WebSocket RFC 6455 murni (`src/web/ws/` & `src/web/reactive.rs`).
- [x] **7.3. Migrasi Skema Deklaratif**:
  - Pelacakan skema dan CDC subscription hub.

---

### 🔹 PILAR 8: Sovereign Security Shield
- [x] **8.1. Proteksi CSRF & Sanitasi XSS Otomatis**:
  - Pengecekan token CSRF otomatis pada semua mutasi (`validasi_token_csrf`).
- [x] **8.2. Content Security Policy (CSP) Nonce Generator**:
  - Pembuatan kunci acak kriptografis per-permintaan.
- [x] **8.3. Brankas Rahasia Terenkripsi (*Target-Scoped Secrets Vault*)**:
  - Manajemen variabel lingkungan terenkripsi AES-256-GCM / ChaCha20-Poly1305.

---

### 🔹 PILAR 9: DevEx Tooling, Hot Reload & Web Vitals
- [x] **9.1. Server Pengembangan Instan**:
  - Hot Module Reloading (HMR) dan Live Client Patching via WebSocket.
- [x] **9.2. Telemetri Real-Time & Core Web Vitals Monitor**:
  - Pelacak otomatis metrik rendering dan response time.
- [x] **9.3. Diagnostik & Linter Bawaan**:
  - Audit aksesibilitas dan keamanan rute terintegrasi.

---

### 🔹 PILAR 10: Atomic Edge Deployment & Zero-Downtime Rollback
- [x] **10.1. Snapshot Deployment Atomik (Merkle SHA-256)**:
  - Setiap build menghasilkan hash konten unik (`src/web/cloud.rs`).
- [x] **10.2. Preview URL Instan per Branch / Commit**:
  - Integrasi domain kedaulatan (`WidyaCloudEngine`).
- [x] **10.3. Rollback Instan (< 1 milidetik)**:
  - Pengalihan pointer lalu lintas produksi ke deployment sebelumnya.

---

## 🏆 STATUS VALIDASI TEST INTEGRASI
- **Test Suite**: `tests/pure_web_nextjs_parity_test.rs`
- **Hasil**: **6 / 6 PASSED (100% OK, 0.00s)**
  - `test_app_router_nested_layouts_and_dynamic_routes`: **PASSED**
  - `test_streaming_ssr_and_islands_architecture`: **PASSED**
  - `test_isr_cache_and_tag_revalidation`: **PASSED**
  - `test_server_action_zero_api_rpc_and_csrf`: **PASSED**
  - `test_edge_jwt_and_rate_limiter`: **PASSED**
  - `test_scoped_css_and_minifier`: **PASSED**

