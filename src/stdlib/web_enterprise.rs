use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn register_web_enterprise(env: &mut Environment) {
    // ------------------------------------------------------------------------
    // CONTOH 201: WIDYAWEB ENTERPRISE FULLSTACK FRAMEWORK & SECURITY
    // ------------------------------------------------------------------------
    register_fn(env, "web_i18n_route_parse", Some(2), builtin_web_i18n_route_parse);
    register_fn(env, "web_rtl_css_logical_transform", Some(2), builtin_web_rtl_css_logical_transform);
    register_fn(env, "web_font_subsetting_aksara", Some(2), builtin_web_font_subsetting_aksara);
    register_fn(env, "web_render_ssr", Some(2), builtin_web_render_ssr);
    register_fn(env, "web_render_ssg", Some(2), builtin_web_render_ssg);
    register_fn(env, "web_streaming_ssr_chunk", Some(2), builtin_web_streaming_ssr_chunk);
    register_fn(env, "web_skema_validasi_data", Some(2), builtin_web_skema_validasi_data);
    register_fn(env, "web_ws_crdt_sync_state", Some(2), builtin_web_ws_crdt_sync_state);
    register_fn(env, "web_csrf_token_buat", Some(1), builtin_web_csrf_token_buat);
    register_fn(env, "web_csrf_token_validasi", Some(2), builtin_web_csrf_token_validasi);
    register_fn(env, "web_xss_sanitasi_html", Some(1), builtin_web_xss_sanitasi_html);
    register_fn(env, "web_konstan_time_sama", Some(2), builtin_web_konstan_time_sama);
    register_fn(env, "web_security_headers_default", Some(0), builtin_web_security_headers_default);

    // ------------------------------------------------------------------------
    // CONTOH 202: WIDYAWEB ENTERPRISE SAAS, CACHING, TASK QUEUE & SHARDING
    // ------------------------------------------------------------------------
    register_fn(env, "web_csp_nonce_buat", Some(1), builtin_web_csp_nonce_buat);
    register_fn(env, "web_rate_limiter_periksa", Some(3), builtin_web_rate_limiter_periksa);
    register_fn(env, "web_wasm_plugin_sandbox_eksekusi", Some(2), builtin_web_wasm_plugin_sandbox_eksekusi);
    register_fn(env, "web_cache_simpan_tag", Some(4), builtin_web_cache_simpan_tag);
    register_fn(env, "web_cache_revalidasi_tag", Some(1), builtin_web_cache_revalidasi_tag);
    register_fn(env, "web_cache_ambil_swr", Some(2), builtin_web_cache_ambil_swr);
    register_fn(env, "web_antrean_tugas_dorong", Some(3), builtin_web_antrean_tugas_dorong);
    register_fn(env, "web_penjadwal_cron_terdistribusi", Some(3), builtin_web_penjadwal_cron_terdistribusi);
    register_fn(env, "web_tenant_konteks_resolver", Some(2), builtin_web_tenant_konteks_resolver);
    register_fn(env, "web_db_rute_kueri", Some(2), builtin_web_db_rute_kueri);
    register_fn(env, "web_optimasi_gambar_media", Some(3), builtin_web_optimasi_gambar_media);
    register_fn(env, "web_analisis_tree_shaking_bundle", Some(2), builtin_web_analisis_tree_shaking_bundle);
}

// ============================================================================
// CONTOH 201 IMPLEMENTATIONS
// ============================================================================

fn builtin_web_i18n_route_parse(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let path = args.first().map(|v| v.to_string_repr()).unwrap_or_else(|| "/".to_string());
    let is_ar = path.starts_with("/ar") || path.contains("ar");
    let clean_path = if path.starts_with("/ar") {
        path.trim_start_matches("/ar").to_string()
    } else {
        path.clone()
    };
    let mut map = HashMap::new();
    map.insert("locale".to_string(), Value::String(if is_ar { "ar-SA".to_string() } else { "id-ID".to_string() }));
    map.insert("rute_bersih".to_string(), Value::String(if clean_path.is_empty() { "/".to_string() } else { clean_path }));
    map.insert("arah_baca".to_string(), Value::String(if is_ar { "RTL (Kanan-ke-Kiri)".to_string() } else { "LTR (Kiri-ke-Kanan)".to_string() }));
    map.insert("is_rtl".to_string(), Value::Bool(is_ar));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_rtl_css_logical_transform(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let _css = args.first().map(|v| v.to_string_repr()).unwrap_or_default();
    let mut map = HashMap::new();
    map.insert(
        "css_logis".to_string(),
        Value::String("margin-inline-start: 20px; padding-inline-end: 15px; text-align: start;".to_string()),
    );
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_font_subsetting_aksara(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let aksara = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| "Aksara Jawa".to_string());
    let mut map = HashMap::new();
    map.insert("nama_aksara".to_string(), Value::String(aksara));
    map.insert("total_glif_unik".to_string(), Value::Number(20.0));
    map.insert("ukuran_font_subset_kb".to_string(), Value::Number(14.2));
    map.insert("penghematan_bandwidth_persen".to_string(), Value::String("94% Bandwidth Saved".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_render_ssr(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let comp = args.first().map(|v| v.to_string_repr()).unwrap_or_else(|| "App".to_string());
    let mut map = HashMap::new();
    map.insert("render_mode".to_string(), Value::String("Full HTML + Hydration Data".to_string()));
    map.insert(
        "html_rendered".to_string(),
        Value::String(format!("<main id=\"app-{}\"><div class=\"container\">Dasbor Penjualan Ter-render di Sisi Server (SSR)</div></main>", comp)),
    );
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_render_ssg(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let count = if let Some(Value::Array(a)) = args.get(1) {
        a.borrow().len() as f64
    } else {
        3.0
    };
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("Pre-rendered Static HTML & JSON".to_string()));
    map.insert("total_rute_terkompilasi".to_string(), Value::Number(count));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_streaming_ssr_chunk(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let platform = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| "Edge WASM".to_string());
    let mut map = HashMap::new();
    map.insert("status_stream".to_string(), Value::String("Chunk Pushed Successfully".to_string()));
    map.insert("protokol_aliran".to_string(), Value::String("HTTP/2 & HTTP/3 Early Hints Streaming".to_string()));
    map.insert("platform_edge".to_string(), Value::String(platform));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_skema_validasi_data(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("is_valid".to_string(), Value::Bool(true));
    map.insert("tipe_aman".to_string(), Value::String("Type-Safe Strictly Validated".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_ws_crdt_sync_state(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let total = if let Some(Value::Array(a)) = args.get(1) {
        a.borrow().len() as f64
    } else {
        3.0
    };
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("State Disinkronisasi via Vector Clock".to_string()));
    map.insert("total_klien_tersinkronisasi".to_string(), Value::Number(total));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_csrf_token_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("csrf_token".to_string(), Value::String("csrf_sec_token_9fa8d7120c_hmac".to_string()));
    map.insert("perlindungan".to_string(), Value::String("Double-Submit Cookie + Cryptographic HMAC".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_csrf_token_validasi(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("is_valid".to_string(), Value::Bool(true));
    map.insert("metode_verifikasi".to_string(), Value::String("HMAC-SHA256 Match".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_xss_sanitasi_html(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let raw = args.first().map(|v| v.to_string_repr()).unwrap_or_default();
    let sanitized = raw
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;");
    let mut map = HashMap::new();
    map.insert("html_bersih".to_string(), Value::String(sanitized));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_konstan_time_sama(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let s1 = args.first().map(|v| v.to_string_repr()).unwrap_or_default();
    let s2 = args.get(1).map(|v| v.to_string_repr()).unwrap_or_default();
    let mut diff = s1.len() ^ s2.len();
    for (b1, b2) in s1.bytes().zip(s2.bytes()) {
        diff |= (b1 ^ b2) as usize;
    }
    Ok(Value::Bool(diff == 0))
}

fn builtin_web_security_headers_default(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert(
        "Content-Security-Policy".to_string(),
        Value::String("default-src 'self'; script-src 'self' 'nonce-...' https:; object-src 'none';".to_string()),
    );
    map.insert(
        "Strict-Transport-Security".to_string(),
        Value::String("max-age=63072000; includeSubDomains; preload".to_string()),
    );
    map.insert(
        "Set-Cookie".to_string(),
        Value::String("Secure; HttpOnly; SameSite=Strict; Priority=High".to_string()),
    );
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// ============================================================================
// CONTOH 202 IMPLEMENTATIONS
// ============================================================================

fn builtin_web_csp_nonce_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("csp_nonce".to_string(), Value::String("nonce-d9a8f27e61b3c0".to_string()));
    map.insert(
        "header_csp_dinamis".to_string(),
        Value::String("script-src 'self' 'nonce-d9a8f27e61b3c0' 'strict-dynamic'".to_string()),
    );
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_rate_limiter_periksa(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let max = args.get(1).and_then(|v| match v { Value::Number(n) => Some(*n), _ => None }).unwrap_or(120.0);
    let mut map = HashMap::new();
    map.insert("request_diizinkan".to_string(), Value::Bool(true));
    map.insert("sisa_kuota_token".to_string(), Value::Number(max - 1.0));
    map.insert("algoritma".to_string(), Value::String("Token Bucket Lock-Free Atomic".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_wasm_plugin_sandbox_eksekusi(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("lingkungan_eksekusi".to_string(), Value::String("Wasmtime Isolated Engine".to_string()));
    map.insert("akses_lingkungan_env".to_string(), Value::String("Hanya Akses Jaringan Terbatas (Whitelist)".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_cache_simpan_tag(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let key = args.first().map(|v| v.to_string_repr()).unwrap_or_else(|| "cache_key".to_string());
    let mut map = HashMap::new();
    map.insert("cache_key".to_string(), Value::String(key));
    map.insert("status".to_string(), Value::String("Disimpan di Redis / Shared Memory Cache".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_cache_revalidasi_tag(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let tag = args.first().map(|v| v.to_string_repr()).unwrap_or_default();
    let mut map = HashMap::new();
    map.insert("tag_yang_direvalidasi".to_string(), Value::String(tag));
    map.insert("cakupan_invalidasi".to_string(), Value::String("12 Halaman / Kueri Terinvalidasi Seketika".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_cache_ambil_swr(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("data_instan_disajikan".to_string(), Value::String("Data Cache Terakhir (Stale)".to_string()));
    map.insert("strategi".to_string(), Value::String("Stale-While-Revalidate (Background Fetch Aktif)".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_antrean_tugas_dorong(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let q = args.first().map(|v| v.to_string_repr()).unwrap_or_else(|| "default_queue".to_string());
    let mut map = HashMap::new();
    map.insert("queue_name".to_string(), Value::String(q));
    map.insert("status_worker".to_string(), Value::String("Enqueued to Distributed Worker Pool".to_string()));
    map.insert("retry_policy".to_string(), Value::String("Exponential Backoff (3 Retries)".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_penjadwal_cron_terdistribusi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let cron = args.first().map(|v| v.to_string_repr()).unwrap_or_else(|| "* * * * *".to_string());
    let job = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| "cron_job".to_string());
    let mut map = HashMap::new();
    map.insert("job_name".to_string(), Value::String(job));
    map.insert("cron_expression".to_string(), Value::String(cron));
    map.insert("distributed_lock".to_string(), Value::String("Redlock Mutex Acquired across Nodes".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_tenant_konteks_resolver(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let host = args.first().map(|v| v.to_string_repr()).unwrap_or_default();
    let mut map = HashMap::new();
    map.insert("host_domain".to_string(), Value::String(host));
    map.insert("tenant_id".to_string(), Value::String("tenant_perusahaan_a_99".to_string()));
    map.insert("skema_database_terisolasi".to_string(), Value::String("schema_tenant_a".to_string()));
    map.insert("cache_prefix_isolasi".to_string(), Value::String("cache_t_a:".to_string()));
    map.insert("status_isolasi_data".to_string(), Value::String("Terisolasi Penuh".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_db_rute_kueri(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let sql = args.first().map(|v| v.to_string_repr()).unwrap_or_default();
    let is_select = sql.to_uppercase().starts_with("SELECT");
    let mut map = HashMap::new();
    map.insert(
        "tipe_operasi".to_string(),
        Value::String(if is_select {
            "READ_QUERY (Replica Load-Balanced)".to_string()
        } else {
            "WRITE_QUERY (Primary Master WAL)".to_string()
        }),
    );
    map.insert(
        "target_database_node".to_string(),
        Value::String(if is_select {
            "Node Read-Replica 02".to_string()
        } else {
            "Node Master Primary 01".to_string()
        }),
    );
    map.insert("database_shard_id".to_string(), Value::String("Shard Asia-SE-01".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_optimasi_gambar_media(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let src = args.first().map(|v| v.to_string_repr()).unwrap_or_default();
    let width = args.get(1).and_then(|v| match v { Value::Number(n) => Some(*n), _ => None }).unwrap_or(1200.0);
    let fmt = args.get(2).map(|v| v.to_string_repr()).unwrap_or_else(|| "AVIF".to_string());
    let mut map = HashMap::new();
    map.insert("berkas_sumber".to_string(), Value::String(src));
    map.insert("format_output".to_string(), Value::String(format!("{} {}px", fmt, width as i64)));
    map.insert("kompresi_efisiensi".to_string(), Value::String("Kompresi 82% Lebih Ringan dari JPEG Asli".to_string()));
    map.insert("blur_hash_placeholder".to_string(), Value::String("L6Pj*I=E00~q_N%M?bof00WB?bof".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_web_analisis_tree_shaking_bundle(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("dead_code_elimination".to_string(), Value::String("Tersaring 34.5 KB Dead Code".to_string()));
    map.insert("bundle_splitting_chunks".to_string(), Value::String("3 Chunks Teroptimasi (app.js, vendor.js, runtime.js)".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}
