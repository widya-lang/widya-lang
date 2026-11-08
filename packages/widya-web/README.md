# 🌐 @widya/web (Widya-Express / FastAPI)

> **Kerangka Kerja RESTful Routing, Middleware Pipeline, & Server Asinkron untuk Widya-Lang.**  
> *Setara fungsional dan berkinerja setara Express / Fastify (Node.js) & FastAPI (Python).*

---

## 🚀 Fitur Utama
- **Routing RESTful:** `buat_router()`, `get(app, path, fn)`, `post(app, path, fn)`, `put(app, path, fn)`, `hapus(app, path, fn)`.
- **Pipeline Middleware:** `gunakan_middleware(app, fn)` dengan dukungan interupsi rantai eksekusi dan penanganan otentikasi.
- **Pembangun Respons HTTP Tipe Aman:** `respons_json(data, status)`, `respons_teks(teks, status)`, `respons_galat(pesan, status)`.
- **Penanganan Permintaan:** `tangani_permintaan(app, req)` dengan otomatis menangani respon 404 jika rute tidak terdaftar.

## 📦 Pemasangan via WPM
```bash
widya wpm add @widya/web
```

## 💻 Contoh Penggunaan
```widya
impor "@widya/web" sebagai web;

misal app = web.buat_router();

// 1. Rute Status Layanan
app = web.get(app, "/status", fungsi(req) {
    kembalikan web.respons_json({
        "status": "ONLINE",
        "layanan": "Widya API Sovereign Gateway"
    }, 200);
});

// 2. Rute Pembuatan Pengguna
app = web.post(app, "/pengguna", fungsi(req) {
    kembalikan web.respons_json({
        "sukses": true,
        "pesan": "Akun berhasil dibuat"
    }, 201);
});
```

