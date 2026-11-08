## Bab 5: Pipeline Middleware: Rantai Interseptor & Konteks Sesi

### 5.1 Arsitektur Rantai Tanggung Jawab: Interseptor Masuk & Keluar

Dalam pengembangan aplikasi web skala enterprise, logika lintas-sektoral (*cross-cutting concerns*)—seperti pencatatan log akses (*Access Logging*), pemeriksaan token autentikasi (JWT), pembatasan laju kueri (*Rate Limiting*), penanganan header keamanan CORS, dan kompresi konten—tidak boleh dicampuradukkan ke dalam fungsi kontroler logika bisnis artikel berita.

WidyaWeb menerapkan pola arsitektur **Rantai Tanggung Jawab (*Chain of Responsibility / Middleware Pipeline*)** native (`pure_middleware.rs`):
- **Model Interseptor Bawang (*Onion Middleware Model*)**: Setiap lapisan middleware membungkus kontroler di dalamnya. Middleware dapat mengeksekusi kode sebelum kontroler dipanggil (*Pre-Processing*), memutuskan apakah akan meneruskan permintaan ke lapisan berikutnya via metode `berikutnya()` (*next*), atau mencegat dan menolak akses seketika (*Early Rejection*), serta memodifikasi header respons saat aliran balik (*Post-Processing*).
- **Konteks Sesi Tipe-Aman (*Type-Safe Session Context*)**: Wadah metadata yang melekat pada setiap siklus hidup permintaan HTTP, memungkinkan middleware autentikasi menyuntikkan identitas pengguna (`user_id`, `peran`, `tenant_id`) untuk dibaca secara aman oleh handler di bawahnya.

```mermaid
flowchart TD
    Req["Permintaan HTTP Klien Masuk"] --> M1["1. Logger Middleware (Catat IP, Waktu, User-Agent)"]
    M1 --> M2["2. RateLimiter Middleware (Token Bucket Check)"]
    M2 -->|Token Habis (Limit Melebihi Batas)| E429["Tolak: 429 Too Many Requests"]
    M2 -->|Token Tersedia| M3["3. Cors & SecurityHeaders Middleware"]
    M3 --> M4["4. Auth & JWT Verification Middleware"]
    M4 -->|Token Kadaluwarsa / Palsu| E401["Tolak: 401 Unauthorized"]
    M4 -->|Token Sah: Suntikkan Sesi Pengguna| Handler["Kontroler Logika Bisnis: Baca & Terbitkan Berita"]
    Handler --> M4_out["Post-Auth"]
    M4_out --> M3_out["Suntik Header: CSP, HSTS, X-Frame-Options"]
    M3_out --> M2_out["Post-RateLimit"]
    M2_out --> M1_out["Hitung Durasi Latensi Eksekusi (ms) & Tulis Log"]
    M1_out --> Res["Respons HTTP Mengalir ke Klien"]
```

Jenis tindakan tanggapan middleware pada WidyaWeb:
1. `ResponMiddleware::Lanjutkan`: Melanjutkan eksekusi ke lapisan berikutnya dengan menyertakan header tambahan atau konteks yang diperkaya.
2. `ResponMiddleware::AlihkanRute`: Mencegat permintaan dan mengalihkan klien ke URL lain (misal: redirect `302 Found` ke halaman login jika sesi habis).
3. `ResponMiddleware::TolakAkses`: Menolak permintaan seketika dengan kode status tertentu (misal: `401 Unauthorized` atau `429 Too Many Requests`) tanpa menyentuh kontroler bisnis.

### 5.2 Implementasi Kode Program Utuh

Berikut implementasi pipeline middleware terpadu (Logger, Token Bucket Rate Limiter, dan JWT Verifier) pada WidyaWeb menggunakan Widya-Lang:

```widya
// Modul Pipeline Middleware dan Autentikasi Sesi WidyaWeb
modul rantai_middleware {
    impor ekosistem::widyaweb::{
        WidyaWeb, RantaiMiddleware, PermintaanHttp, ResponHttp,
        KonteksSesi, ResponMiddleware, TokenBucket, VerifikatorJWT
    };

    fungsi pasang_pipeline_keamanan(app: &mut WidyaWeb) {
        cetak("==================================================================");
        cetak(">>> [MIDDLEWARE] Memasang Pipeline Rantai Keamanan Berlapis...");

        // Middleware 1: Pencatat Waktu & Metrik Permintaan (Logger)
        app.gunakan_middleware(|req: &PermintaanHttp, ctx: &mut KonteksSesi| -> ResponMiddleware {
            ctx.simpan_waktu_mulai();
            cetak(">>> [LOG] " + req.metode() + " " + req.jalur() + " | IP: " + req.ip_klien());
            kembalikan ResponMiddleware::Lanjutkan;
        });

        // Middleware 2: Pembatas Laju Akses Token Bucket (Anti-Spam / Anti-DDoS)
        biar mut pembatas_laju = TokenBucket::buat(100, 20); // Kapasitas 100 token, isi ulang 20 token/detik
        app.gunakan_middleware(pembatas_laju.ke_middleware());

        // Middleware 3: Verifikasi JWT dan Pengikatan Konteks Pengguna
        app.gunakan_middleware(|req: &PermintaanHttp, ctx: &mut KonteksSesi| -> ResponMiddleware {
            // Rute publik beranda dan login dibebaskan dari verifikasi token
            jika req.jalur().dimulai_dengan("/api/v1/publik") || req.jalur() == "/" {
                kembalikan ResponMiddleware::Lanjutkan;
            }

            biar header_auth = req.ambil_header("Authorization");
            cocokkan header_auth {
                Ada(auth_str) jika auth_str.dimulai_dengan("Bearer ") => {
                    biar token = auth_str.potong_awal("Bearer ");
                    biar hasil_verifikasi = VerifikatorJWT::verifikasi_ed25519(&token, "KUNCI_PUBLIK_WIDYA_2026");

                    cocokkan hasil_verifikasi {
                        Ok(klaim) => {
                            // Suntikkan metadata ke dalam konteks sesi permintaan
                            ctx.tetapkan("id_pengguna", klaim.ambil_string("sub"));
                            ctx.tetapkan("peran", klaim.ambil_string("role"));
                            kembalikan ResponMiddleware::Lanjutkan;
                        },
                        Salah(_) => {
                            kembalikan ResponMiddleware::TolakAkses(401, "Token JWT tidak sah atau telah kedaluwarsa.");
                        }
                    }
                },
                _ => {
                    kembalikan ResponMiddleware::TolakAkses(401, "Header Authorization Bearer diperlukan untuk rute ini.");
                }
            }
        });

        cetak(">>> [MIDDLEWARE] 3 lapisan middleware aktif melindungi kontroler.");
        cetak("==================================================================");
    }
}
```

### 5.3 Analisis Arsitektur, Metrik & Tabel Spesifikasi

Tabel perbandingan pola middleware pada platform web modern:

| Parameter Evaluasi | Express / Koa (Node.js) | Gin / Fiber (Go) | WidyaWeb Native Pipeline |
| :--- | :--- | :--- | :--- |
| **Pola Eksekusi** | Callback Asinkron Berantai | Rantai Indeks Array (`Next()`) | **Onion Interceptor Zero-Alloc** |
| **Keamanan Tipe Konteks Sesi** | Tidak Aman (`req.user: any`) | Semi Aman (`c.Get(key).(type)`)| **Type-Safe Session Context** |
| **Alokasi Memori per Lapisan** | Alokasi Objek Promise Heap | Alokasi Struct Konteks | **Pointer Referensi Stack (Nol Alokasi)** |
| **Dukungan Circuit Breaker** | Pustaka Luar Terpisah | Pustaka Pihak Ketiga | **Built-in Native Resiliency** |
| **Overhead Latensi Pipeline** | 0.8 – 2.5 milidetik | 0.1 – 0.3 milidetik | **< 0.05 milidetik (Ultra Cepat)** |

### 5.4 Latihan Soal Esai

1. Terangkan model eksekusi interseptor bawang (*Onion Model*) pada pipeline middleware! Mengapa model ini memungkinkan eksekusi kode *Pre-Processing* dan *Post-Processing* secara simetris dalam satu fungsi middleware?
2. Bagaimana mekanisme kerja *Token Bucket Rate Limiter* dalam mencegah serangan banjir permintaan (*Denial of Service*) pada API publik portal berita tanpa menolak pengguna sah yang melakukan lonjakan akses wajar?
3. Mengapa penyimpanan data identitas pengguna pada objek *Context Sesi* yang bertipe aman (*Type-Safe*) jauh lebih unggul dibandingkan menyematkannya secara bebas pada objek *Request* bertipe dinamis?

### 5.5 Soal Pilihan Ganda

1. Pola arsitektur perangkat lunak yang mendasari perancangan pipeline middleware pada WidyaWeb adalah...
   A. Chain of Responsibility (Rantai Tanggung Jawab)
   B. Model-View-Controller
   C. Singleton Murni
   D. Observer Linier

2. Tiga jenis nilai kembalian `ResponMiddleware` yang dapat diputuskan oleh sebuah middleware pada WidyaWeb adalah...
   A. `Lanjutkan`, `AlihkanRute`, dan `TolakAkses`
   B. `Buka`, `Baca`, dan `Tutup`
   C. `Mulai`, `Tunggu`, dan `Selesai`
   D. `Simpan`, `Hapus`, dan `Format`

3. Kode status HTTP standar yang dikembalikan oleh middleware Rate Limiter ketika seorang klien melampaui batas kuota permintaan yang diizinkan adalah...
   A. `429 Too Many Requests`
   B. `200 OK`
   C. `404 Not Found`
   D. `500 Internal Server Error`

4. Kode status HTTP yang dikembalikan oleh middleware autentikasi ketika permintaan tidak menyertakan token kredensial yang sah adalah...
   A. `401 Unauthorized`
   B. `403 Forbidden`
   C. `204 No Content`
   D. `502 Bad Gateway`

5. Komponen pembawa metadata sesi yang mengalir di sepanjang rantai middleware hingga ke kontroler logika bisnis dinamakan...
   A. Konteks Sesi (Session Context)
   B. Harddisk Buffer
   C. Frame Index
   D. Global Mutex

6. Mekanisme keamanan web yang mengatur domain peramban mana saja yang diizinkan memanggil API portal berita dari luar domain asal dinamakan...
   A. Cross-Origin Resource Sharing (CORS)
   B. Content Security Policy (CSP)
   C. Subresource Integrity (SRI)
   D. DNS Prefetch

7. Header keamanan HTTP yang menginstruksikan peramban agar hanya terhubung melalui sambungan terenkripsi HTTPS secara permanen adalah...
   A. `Strict-Transport-Security` (HSTS)
   B. `Cache-Control`
   C. `User-Agent`
   D. `Accept-Encoding`

8. Dalam model Onion Middleware, kapan kode *Post-Processing* suatu middleware dieksekusi?
   A. Setelah fungsi kontroler bisnis selesai mengeksekusi logika dan respons sedang mengalir kembali ke luar
   B. Sebelum server dinyalakan
   C. Saat koneksi jaringan diputus
   D. Setiap tengah malam

9. Standar format token web kompak dan mandiri (*stateless*) yang diverifikasi oleh middleware autentikasi adalah...
   A. JSON Web Token (JWT)
   B. File XML
   C. Cookie Flash
   D. Gambar Bitmap

10. Mengapa pemisahan logika keamanan ke dalam lapisan middleware sangat krusial bagi arsitektur portal berita skala besar?
    A. Mencegah duplikasi kode pengamanan di setiap handler artikel dan memastikan seluruh rute terlindungi secara terpusat dan konsisten
    B. Supaya berita dapat ditulis otomatis tanpa campur tangan redaksi
    C. Menghilangkan kebutuhan koneksi internet
    D. Mengurangi konsumsi memori pada layar monitor pembaca
