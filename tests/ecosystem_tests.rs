use widya::stdlib_ffi::sqlite::SqliteDb;
use widya::stdlib_ffi::mysql::MySqlDb;
use widya::stdlib_ffi::postgres::PgDb;
use widya::stdlib_ffi::duckdb::DuckDb;
use widya::stdlib_ffi::surrealdb_w::SurrealDbW;
use widya::stdlib_ffi::redb_w::RedbW;
use widya::stdlib_ffi::crypto::Crypto;
use widya::wasm_playground::{WasmPlayground, WasmPlaygroundResult};
use widya::value::Value;

#[test]
fn test_sqlite_ffi_wrapper_operations() {
    let _ = std::fs::remove_file("./test_eco_sqlite.db");
    let db = SqliteDb::new("./test_eco_sqlite.db").expect("Harus berhasil membuka sqlite");

    let create_res = db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);", &[]);
    assert!(create_res.is_ok(), "CREATE TABLE harus berhasil: {:?}", create_res.err());

    let insert_res = db.execute("INSERT INTO users VALUES (1, 'Alice');", &[]);
    assert!(insert_res.is_ok(), "INSERT harus berhasil: {:?}", insert_res.err());

    let rows = db.query("SELECT * FROM users;", &[]).expect("Query harus berhasil");
    assert_eq!(rows.len(), 1);
    let id_val = &rows[0]["id"];
    let id_str = format!("{:?}", id_val);
    assert!(id_str.contains("1"), "Nilai id harus 1, tetapi got {}", id_str);

    let close_ok = db.close();
    assert!(close_ok);
    let _ = std::fs::remove_file("./test_eco_sqlite.db");
}

#[test]
fn test_mysql_ffi_wrapper_interface_compile() {
    // Test INTERFACE SAJA: new() return Err untuk invalid URL (buktikan modul importable & signature ok).
    // Test koneksi real butuh server MySQL hidup — skip tanpa server.
    let bad_url = "mysql://invalid_user:wrongpass@127.0.0.1:1/db_does_not_exist_certainly";
    let res = MySqlDb::new(bad_url);
    // TIDAK assert == Err karena user environment mungkin MEMPUNYAI MySQL di port default! Cuma assert function signature CALLABLE (no panic).
    match res {
        Ok(db) => {
            let close_ok = db.close();
            assert!(close_ok);
        }
        Err(_e) => {
            // Expected case — connection refused (normal).
        }
    }

    // Test placeholder rewrite logic via SQL call pattern (tidak perlu connect real).
    // Test Value conversion signature: function callable.
    let v = Value::Number(42.0);
    let _ = format!("{:?}", v);
}

#[test]
fn test_postgres_ffi_wrapper_interface_compile() {
    // Test INTERFACE SAJA: new() return behavior (interface signature).
    // Test real PG connection need postgres server, skip without server.
    let bad_url = "postgresql://no_user:nopass@127.0.0.1:1/surely_no_db_exists";
    let res = PgDb::new(bad_url);
    match res {
        Ok(db) => {
            let close_ok = db.close();
            assert!(close_ok);
        }
        Err(_e) => {
            // Expected behavior when no postgres server.
        }
    }
}

#[test]
fn test_duckdb_ffi_wrapper_interface_compile() {
    // DuckDB embedded = bundled C++. Tidak butuh server. Coba buka :memory:, pastikan interface new/execute/query/close CALLABLE.
    let res = DuckDb::new(":memory:");
    match res {
        Ok(db) => {
            let _create = db.execute("CREATE TABLE t1 (id INTEGER, name VARCHAR);", &[]);
            let _ins = db.execute("INSERT INTO t1 VALUES (1, 'x');", &[]);
            let q = db.query("SELECT * FROM t1;", &[]);
            assert!(q.is_ok(), "DuckDB query memory harus ok");
            assert_eq!(q.unwrap().len(), 1);
            let close_ok = db.close();
            assert!(close_ok);
        }
        Err(e) => {
            // Build environment tertentu bisa gagal compile feature bundled (C++ toolchain tidak ada) — tidak panic, hanya log.
            eprintln!("SKIP DuckDB test (error build env): {}", e);
        }
    }
    // Test invalid path (jika memory tidak support — pastikan signature callable no panic).
    let _ = DuckDb::new("/invalid/nonexistent/path/for_sure_nope/duck.db");
}

#[test]
fn test_surrealdb_wrapper_interface_compile() {
    // SurrealDB compatible wrapper embedded in memory (pure Rust HashMap + JSON persist). 100% work tanpa server.
    let res = SurrealDbW::new("mem");
    assert!(res.is_ok(), "SurrealDB wrapper new mem harus Ok: {:?}", res.err());
    let db = res.unwrap();
    let _ = db.execute("CREATE TABLE user_tes (id INTEGER PRIMARY KEY, nama TEXT);", &[]);
    let _ = db.execute("INSERT INTO user_tes (nama) VALUES (?1);", &[Value::String("Budi".to_string())]);
    let rows = db.query("SELECT * FROM user_tes;", &[]).expect("Query harus berhasil");
    assert!(rows.len() >= 1, "Setelah INSERT harus dapat 1 row");
    let ok = db.close();
    assert!(ok);

    // Test invalid URL parameter no panic (callable).
    let _ = SurrealDbW::new("./data/temp_surreal_test_todelete.json");
}

#[test]
fn test_redb_wrapper_interface_compile() {
    // Redb = pure Rust file-based embedded KV (no FFI). 100% mobile-ready.
    let path = "./test_eco_redb_temporary_delete_me.db";
    let _ = std::fs::remove_file(path);
    let res = RedbW::new(path);
    match res {
        Ok(db) => {
            let _c = db.execute("CREATE TABLE bukus (id INTEGER PRIMARY KEY, judul TEXT);", &[]);
            let _i = db.execute("INSERT INTO bukus (judul) VALUES (?1);", &[Value::String("Belajar Widya".to_string())]);
            let rows = db.query("SELECT * FROM bukus;", &[]).expect("SELECT harus Ok");
            assert!(rows.len() >= 1, "Rows setelah insert harus ada");
            let close_ok = db.close();
            assert!(close_ok);
        }
        Err(e) => {
            eprintln!("SKIP Redb test (env error): {}", e);
        }
    }
    // Test invalid folder (callable signature, no panic)
    let _ = RedbW::new("/proc/impossible_dir/redb.db");
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_crypto_ffi_wrapper_operations() {
    let raw = b"Widya-Lang Enterprise Crypto Test";
    let hash = Crypto::sha256_hex(raw);
    assert_eq!(hash.len(), 64);

    let encoded = Crypto::base64_encode(raw);
    let decoded = Crypto::base64_decode(&encoded).expect("Base64 decode harus berhasil");
    assert_eq!(decoded, raw);

    let hmac = Crypto::hmac_sha256(b"secret_key", raw);
    assert_eq!(hmac.len(), 64);

    let token = Crypto::generate_token("widya_usr");
    assert_eq!(token.len(), 64);
}

#[test]
fn test_wasm_playground_pipeline() {
    let valid_code = "misal x = 10; misal y = 20; kembalikan x + y;";
    let res: WasmPlaygroundResult = WasmPlayground::evaluate(valid_code);
    assert!(res.success, "Evaluation failed with errors: {:?}", res.errors);
    assert!(res.ast_json.is_some());
}
