//! Test Integrasi 100% Berdaulat: WidyaWeb Next.js & SvelteKit Parity
//! Zero External Dependencies (Tanpa Node.js, Next.js, Vercel, dll).

use widya::web::pure_router::AppRouterMurni;
use widya::web::pure_ssr::{MesinSSRMurni, IsrCacheHub, KomponenIsland};
use widya::web::pure_actions::ServerActionDispatcher;
use widya::web::pure_middleware::{VerifikatorJWTMurni, RateLimiterBucket};
use widya::web::pure_assets::{CssScopedEngine, MinifierWebMurni};
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_app_router_nested_layouts_and_dynamic_routes() {
    let mut router = AppRouterMurni::baru_memori();

    // Daftarkan rute statis & dinamis dengan layout
    router.daftarkan_rute(
        "/dashboard/user/[id]",
        PathBuf::from("/memori/app/dashboard/user/[id]/page.wya"),
        Some(PathBuf::from("/memori/app/dashboard/layout.wya")),
    );

    // Cocokkan rute /dashboard/user/42
    let hasil = router.cocokkan_url("/dashboard/user/42");
    assert!(hasil.is_some(), "Rute /dashboard/user/42 harus cocok");

    let match_res = hasil.unwrap();
    assert_eq!(match_res.parameter.get("id"), Some(&"42".to_string()));
    assert_eq!(match_res.url_pola, "/dashboard/user/[id]");

    // Render template layout hierarkis
    let root_layout = "<!DOCTYPE html><html><body>{{slot}}</body></html>";
    let dash_layout = "<nav>SideNav</nav><main>{{slot}}</main>";
    let user_page = format!("<h1>Profil Pengguna ID: {}</h1>", match_res.parameter.get("id").unwrap());

    let rendered = router.render_halaman_konten(&user_page, &[root_layout, dash_layout]);
    assert!(rendered.contains("<!DOCTYPE html>"), "Harus berisi root layout");
    assert!(rendered.contains("<nav>SideNav</nav>"), "Harus berisi dashboard layout");
    assert!(rendered.contains("<h1>Profil Pengguna ID: 42</h1>"), "Harus menginterpolasi params.id");
}

#[test]
fn test_streaming_ssr_and_islands_architecture() {
    let chunks = MesinSSRMurni::render_streaming_chunks("Widya Dashboard", "<div id='konten'>Data Realtime</div>");
    assert!(chunks.len() >= 3, "Streaming SSR harus terbagi menjadi setidaknya 3 chunk");
    assert!(chunks[0].contains("<!DOCTYPE html>"));
    assert!(chunks[1].contains("<div id='konten'>Data Realtime</div>"));
    assert!(chunks[2].contains("</html>"));

    let island = KomponenIsland::baru(
        "counter-widget",
        "CounterIsland",
        r#"{"count": 10}"#,
        "<button>10</button>",
    );
    let island_html = MesinSSRMurni::render_island(&island);
    assert!(island_html.contains("data-widya-island=\"CounterIsland\""));
    assert!(island_html.contains("data-widya-props="));
    assert!(island_html.contains("<button>10</button>"));
}

#[test]
fn test_isr_cache_and_tag_revalidation() {
    let mut isr = IsrCacheHub::baru();
    let tags = vec!["produk".to_string(), "kategori-elektronik".to_string()];
    
    isr.simpan_cache("/produk/laptop", "<h1>Laptop Widya</h1>", 60, tags);

    // Cache masih valid
    let hit = isr.ambil_cache("/produk/laptop");
    assert_eq!(hit, Some("<h1>Laptop Widya</h1>".to_string()));

    // Revalidasi berdasarkan tag "produk"
    isr.revalidasi_tag("produk");
    let after_invalidation = isr.ambil_cache("/produk/laptop");
    assert_eq!(after_invalidation, None, "Cache harus gugur setelah revalidasi tag");
}

#[test]
fn test_server_action_zero_api_rpc_and_csrf() {
    let dispatcher = ServerActionDispatcher::baru();

    // Generate token CSRF
    let csrf_token = dispatcher.buat_csrf_token("session_user_abc");
    assert!(dispatcher.validasi_csrf_token("session_user_abc", &csrf_token));
    assert!(!dispatcher.validasi_csrf_token("session_user_hacker", &csrf_token));

    // Daftarkan server action
    dispatcher.daftarkan_action("simpan_biodata", |args| {
        let nama = args.get("nama").cloned().unwrap_or_default();
        Ok(format!("Biodata {} berhasil disimpan di database berdaulat!", nama))
    });

    let mut payload = HashMap::new();
    payload.insert("nama".to_string(), "Budi Santoso".to_string());

    let eksekusi = dispatcher.eksekusi_action("simpan_biodata", &payload);
    assert!(eksekusi.is_ok());
    assert_eq!(eksekusi.unwrap(), "Biodata Budi Santoso berhasil disimpan di database berdaulat!");
}

#[test]
fn test_edge_jwt_and_rate_limiter() {
    let secret = "widya_sovereign_secret_key_2026";
    let token = VerifikatorJWTMurni::buat_token(r#"{"user_id":"admin","role":"sovereign"}"#, secret, 3600);

    let verify_res = VerifikatorJWTMurni::verifikasi_token(&token, secret);
    assert!(verify_res.is_ok(), "JWT valid harus terverifikasi sukses");
    assert!(verify_res.unwrap().contains("admin"));

    let tampered_token = format!("{}tampered", token);
    let verify_tampered = VerifikatorJWTMurni::verifikasi_token(&tampered_token, secret);
    assert!(verify_tampered.is_err(), "JWT palsu harus ditolak");

    // Test Token Bucket Rate Limiter
    let limiter = RateLimiterBucket::baru(2, 1);
    let ip = "192.168.1.100";
    assert!(limiter.izinkan_request(ip), "Request 1 harus lolos");
    assert!(limiter.izinkan_request(ip), "Request 2 harus lolos");
    assert!(!limiter.izinkan_request(ip), "Request 3 harus diblokir (429)");
}

#[test]
fn test_scoped_css_and_minifier() {
    let raw_css = r#"
        .tombol {
            background-color: #2563eb;
            color: white;
            padding: 8px 16px;
        }
        p {
            line-height: 1.5;
        }
    "#;

    let (scoped_css, scope_id) = CssScopedEngine::transformasi_scoped(raw_css, "KartuProduk");
    assert!(scope_id.starts_with("w-scope-"));
    assert!(scoped_css.contains(&format!(".tombol[{}]", scope_id)));
    assert!(scoped_css.contains(&format!("p[{}]", scope_id)));

    // Minifier
    let raw_html = "  <div> \n  <h1>  Judul Berdaulat  </h1> \n  </div>  ";
    let minified = MinifierWebMurni::minify_html(raw_html);
    assert_eq!(minified, "<div> <h1> Judul Berdaulat </h1> </div>");
}
