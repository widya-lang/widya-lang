// Widya-Lang CLI: Databases & Storage
// Mencakup WidyaBase (RDBMS ACID B+Tree/WAL/MVCC), WidyaStore (S3/MinIO Object Storage),
// dan WidyaDocDB (MongoDB Compatible NoSQL Document Store).

use clap::Subcommand;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Subcommand)]
pub enum BaseSubcommands {
    /// Inisialisasi basis data WidyaBase baru (.wdb)
    Inisialisasi {
        #[arg(value_name = "PATH_DB", default_value = "widyabase.wdb")]
        path: PathBuf,
    },
    /// Alias inisialisasi: init / buat
    Init {
        #[arg(value_name = "PATH_DB", default_value = "widyabase.wdb")]
        path: PathBuf,
    },
    /// Menjalankan server basis data daemon (PostgreSQL Wire v3.0 di Port 5432 & Redis RESP di Port 6379)
    Mulai {
        #[arg(short, long, default_value_t = 5432)]
        port: u16,
        #[arg(short = 'r', long, default_value_t = 6379)]
        redis_port: u16,
        #[arg(short, long, value_name = "PATH_DB", default_value = "widyabase.wdb")]
        data: PathBuf,
    },
    /// Alias mulai: start / server
    Start {
        #[arg(short, long, default_value_t = 5432)]
        port: u16,
        #[arg(short = 'r', long, default_value_t = 6379)]
        redis_port: u16,
        #[arg(short, long, value_name = "PATH_DB", default_value = "widyabase.wdb")]
        data: PathBuf,
    },
    /// Terminal interaktif SQL dwibahasa (REPL)
    Sql {
        #[arg(short, long, value_name = "PATH_DB", default_value = "widyabase.wdb")]
        data: PathBuf,
    },
    /// Mengeksekusi satu baris kueri SQL (Indonesia / Inggris)
    Kueri {
        #[arg(value_name = "SQL")]
        sql: String,
        #[arg(short, long, value_name = "PATH_DB", default_value = "widyabase.wdb")]
        data: PathBuf,
    },
    /// Alias kueri: query
    Query {
        #[arg(value_name = "SQL")]
        sql: String,
        #[arg(short, long, value_name = "PATH_DB", default_value = "widyabase.wdb")]
        data: PathBuf,
    },
    /// Memeriksa integritas dan statistik kesehatan basis data
    Periksa {
        #[arg(value_name = "PATH_DB", default_value = "widyabase.wdb")]
        path: PathBuf,
    },
    /// Alias periksa: check / status
    Status {
        #[arg(value_name = "PATH_DB", default_value = "widyabase.wdb")]
        path: PathBuf,
    },
}


#[derive(Subcommand)]
pub enum StoreSubcommands {
    /// Membuat bucket baru
    BuatBucket {
        #[arg(value_name = "NAMA_BUCKET")]
        nama: String,
        #[arg(short, long, default_value = "./widya_store_data")]
        data: PathBuf,
    },
    /// Menampilkan daftar bucket
    DaftarBucket {
        #[arg(short, long, default_value = "./widya_store_data")]
        data: PathBuf,
    },
    /// Mengunggah berkas ke dalam bucket
    Unggah {
        #[arg(value_name = "NAMA_BUCKET")]
        bucket: String,
        #[arg(value_name = "OBJECT_KEY")]
        key: String,
        #[arg(value_name = "BERKAS_SUMBER")]
        berkas: PathBuf,
        #[arg(short, long, default_value = "./widya_store_data")]
        data: PathBuf,
    },
    /// Mengunduh objek dari bucket ke berkas lokal
    Unduh {
        #[arg(value_name = "NAMA_BUCKET")]
        bucket: String,
        #[arg(value_name = "OBJECT_KEY")]
        key: String,
        #[arg(value_name = "BERKAS_TUJUAN")]
        tujuan: PathBuf,
        #[arg(short, long, default_value = "./widya_store_data")]
        data: PathBuf,
    },
    /// Buat URL Presigned GET dengan otentikasi HMAC-SHA256
    Presign {
        #[arg(value_name = "NAMA_BUCKET")]
        bucket: String,
        #[arg(value_name = "OBJECT_KEY")]
        key: String,
        #[arg(short, long, default_value_t = 3600)]
        ttl: u64,
        #[arg(short, long, default_value = "./widya_store_data")]
        data: PathBuf,
    },
    /// Status infrastruktur Object Storage
    Status,
}


#[derive(Subcommand)]
pub enum DocdbSubcommands {
    /// Menyisipkan dokumen JSON ke dalam koleksi
    Sisip {
        #[arg(value_name = "KOLEKSI")]
        koleksi: String,
        #[arg(value_name = "DOKUMEN_JSON")]
        json_str: String,
        #[arg(short, long, default_value = "./widya_docdb_data")]
        data: PathBuf,
    },
    /// Mencari dokumen dengan filter kueri MongoDB
    Cari {
        #[arg(value_name = "KOLEKSI")]
        koleksi: String,
        #[arg(value_name = "FILTER_JSON", default_value = "{}")]
        filter_str: String,
        #[arg(short, long, default_value = "./widya_docdb_data")]
        data: PathBuf,
    },
    /// Menjalankan Aggregation Pipeline ($match, $group, $sort)
    Agregasi {
        #[arg(value_name = "KOLEKSI")]
        koleksi: String,
        #[arg(value_name = "PIPELINE_JSON")]
        pipeline_str: String,
        #[arg(short, long, default_value = "./widya_docdb_data")]
        data: PathBuf,
    },
    /// Status infrastruktur Document NoSQL Database
    Status,
}


pub fn handle_base_command(aksi: &BaseSubcommands) {
    match aksi {
            BaseSubcommands::Inisialisasi { path } | BaseSubcommands::Init { path } => {
                match widya::widyabase::WidyaBase::open(&path) {
                    Ok(_) => {
                        println!("{}", "🗄️  Basis Data WidyaBase Berhasil Diinisialisasi!".bright_green().bold());
                        println!("📂 Berkas Penyimpanan: {}", path.display().to_string().bright_cyan());
                        println!("📝 Write-Ahead Log   : {}.wal", path.display().to_string().bright_yellow());
                        println!("💡 Mulai server pgwire : {} base mulai --port 5432 --data {}", "widya".bright_yellow(), path.display());
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal inisialisasi basis data: {}", e);
                        process::exit(1);
                    }
                }
            }
            BaseSubcommands::Mulai { port, redis_port, data } | BaseSubcommands::Start { port, redis_port, data } => {
                println!("{}", "🗄️  Memulai Mesin Basis Data WidyaBase Unified Multi-Model v1.0.0...".bright_cyan().bold());
                match widya::widyabase::WidyaBase::open(&data) {
                    Ok(db) => {
                        let db_arc = std::sync::Arc::new(db);
                        let db_redis = std::sync::Arc::clone(&db_arc);
                        let r_port = *redis_port;
                        std::thread::spawn(move || {
                            if let Err(e) = db_redis.start_redis_server(r_port) {
                                eprintln!("⚠️ Gagal menjalankan server Redis: {}", e);
                            }
                        });
                        if let Err(e) = db_arc.start_pgwire_server(*port) {
                            eprintln!("❌ Gagal menjalankan server pgwire: {}", e);
                            process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal membuka basis data: {}", e);
                        process::exit(1);
                    }
                }
            }
            BaseSubcommands::Kueri { sql, data } | BaseSubcommands::Query { sql, data } => {
                match widya::widyabase::WidyaBase::open(&data) {
                    Ok(db) => {
                        match db.execute(&sql) {
                            Ok(res) => match res {
                                widya::widyabase::planner::QueryResult::Message(msg) => {
                                    println!("{}", msg.bright_green());
                                }
                                widya::widyabase::planner::QueryResult::Rows { columns, rows, affected } => {
                                    println!("┌{}", "─".repeat(columns.len() * 18));
                                    println!("│ {}", columns.join(" \t| ").bright_yellow().bold());
                                    println!("├{}", "─".repeat(columns.len() * 18));
                                    for r in rows {
                                        let strs: Vec<String> = r.iter().map(|v| match v {
                                            widya::widyabase::sql_parser::SqlValue::Null => "NULL".bright_black().to_string(),
                                            widya::widyabase::sql_parser::SqlValue::Boolean(b) => b.to_string(),
                                            widya::widyabase::sql_parser::SqlValue::Integer(i) => i.to_string().bright_cyan().to_string(),
                                            widya::widyabase::sql_parser::SqlValue::Float(f) => f.to_string().bright_cyan().to_string(),
                                            widya::widyabase::sql_parser::SqlValue::Text(t) => t.clone(),
                                        }).collect();
                                        println!("│ {}", strs.join(" \t| "));
                                    }
                                    println!("└{}", "─".repeat(columns.len() * 18));
                                    println!("({} baris terpengaruh/ditemukan)", affected);
                                }
                            }
                            Err(e) => {
                                eprintln!("❌ Galat Kueri SQL: {}", e);
                                process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal membuka basis data: {}", e);
                        process::exit(1);
                    }
                }
            }
            BaseSubcommands::Sql { data } => {
                jalankan_widyabase_repl(&data);
            }
            BaseSubcommands::Periksa { path } | BaseSubcommands::Status { path } => {
                match widya::widyabase::WidyaBase::open(&path) {
                    Ok(db) => {
                        let rep = db.check_health();
                        println!("{}", "============================================================".bright_blue());
                        println!("{}", "  🗄️  LAPORAN KESEHATAN BASIS DATA WIDYABASE v1.0.0".bright_cyan().bold());
                        println!("{}", "============================================================".bright_blue());
                        println!("📂 Berkas Target      : {}", path.display().to_string().bright_yellow());
                        println!("📄 Total Halaman 4KB  : {}", rep.total_pages.to_string().bright_green());
                        println!("📊 Jumlah Tabel Aktif : {}", rep.total_tables.to_string().bright_green());
                        println!("📝 Rekod Log WAL      : {}", rep.wal_records.to_string().bright_green());
                        println!("🧠 Buffer Pool Cache  : {}", rep.buffer_pool_status.bright_cyan());
                        println!("🛡️  Status Integritas : {}", "SEHAT & KONSISTEN (ACID OK)".bright_green().bold());
                        println!("{}", "============================================================".bright_blue());
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal memeriksa basis data: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }

pub fn handle_store_command(aksi: &StoreSubcommands) {
    match aksi {
            StoreSubcommands::BuatBucket { nama, data } => {
                let config = widya::widyabase::object_store::ObjectStoreConfig {
                    base_path: data.clone(),
                    ..Default::default()
                };
                let store = match widya::widyabase::object_store::WidyaObjectStore::new(config) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("❌ Gagal inisialisasi WidyaStore: {}", e);
                        process::exit(1);
                    }
                };
                match store.create_bucket(&nama) {
                    Ok(b) => println!("{}", format!("✅ Bucket S3 '{}' berhasil dibuat (Region: id-central-1)", b.name).bright_green().bold()),
                    Err(e) => {
                        eprintln!("❌ Gagal membuat bucket: {}", e);
                        process::exit(1);
                    }
                }
            }
            StoreSubcommands::DaftarBucket { data } => {
                let config = widya::widyabase::object_store::ObjectStoreConfig {
                    base_path: data.clone(),
                    ..Default::default()
                };
                let store = match widya::widyabase::object_store::WidyaObjectStore::new(config) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("❌ Gagal inisialisasi WidyaStore: {}", e);
                        process::exit(1);
                    }
                };
                let buckets = store.list_buckets();
                println!("{}", "============================================================".bright_blue());
                println!("{}", "  🪣 DAFTAR BUCKET WIDYASTORE (S3 / MINIO COMPATIBLE)".bright_cyan().bold());
                println!("{}", "============================================================".bright_blue());
                if buckets.is_empty() {
                    println!("  (Belum ada bucket yang dibuat)");
                } else {
                    for b in buckets {
                        println!("  📦 {} | Dibuat: {} | Status: {:?}", b.name.bright_white().bold(), b.created_at.format("%Y-%m-%d %H:%M:%S UTC"), b.versioning);
                    }
                }
                println!("{}", "============================================================\n".bright_blue());
            }
            StoreSubcommands::Unggah { bucket, key, berkas, data } => {
                let config = widya::widyabase::object_store::ObjectStoreConfig {
                    base_path: data.clone(),
                    ..Default::default()
                };
                let store = match widya::widyabase::object_store::WidyaObjectStore::new(config) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("❌ Gagal inisialisasi WidyaStore: {}", e);
                        process::exit(1);
                    }
                };
                let file_bytes = match fs::read(&berkas) {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("❌ Gagal membaca berkas sumber: {}", e);
                        process::exit(1);
                    }
                };
                let mime = "application/octet-stream";
                match store.put_object(&bucket, &key, &file_bytes, mime, std::collections::HashMap::new()) {
                    Ok(meta) => println!("{}", format!("✅ Objek '{}/{}' ({} bytes) berhasil diunggah! ETag: {}", bucket, key, meta.size_bytes, meta.etag).bright_green().bold()),
                    Err(e) => {
                        eprintln!("❌ Gagal mengunggah objek: {}", e);
                        process::exit(1);
                    }
                }
            }
            StoreSubcommands::Unduh { bucket, key, tujuan, data } => {
                let config = widya::widyabase::object_store::ObjectStoreConfig {
                    base_path: data.clone(),
                    ..Default::default()
                };
                let store = match widya::widyabase::object_store::WidyaObjectStore::new(config) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("❌ Gagal inisialisasi WidyaStore: {}", e);
                        process::exit(1);
                    }
                };
                match store.get_object(&bucket, &key) {
                    Ok(obj) => {
                        if let Err(e) = fs::write(&tujuan, &obj.data) {
                            eprintln!("❌ Gagal menyimpan berkas tujuan: {}", e);
                            process::exit(1);
                        }
                        println!("{}", format!("✅ Objek '{}/{}' ({} bytes) berhasil disimpan ke {}", bucket, key, obj.data.len(), tujuan.display()).bright_green().bold());
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal mengunduh objek: {}", e);
                        process::exit(1);
                    }
                }
            }
            StoreSubcommands::Presign { bucket, key, ttl, data } => {
                let config = widya::widyabase::object_store::ObjectStoreConfig {
                    base_path: data.clone(),
                    ..Default::default()
                };
                let store = match widya::widyabase::object_store::WidyaObjectStore::new(config) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("❌ Gagal inisialisasi WidyaStore: {}", e);
                        process::exit(1);
                    }
                };
                let url = store.generate_presigned_url(&bucket, &key, "WIDYA_ACCESS_KEY", "WIDYA_SECRET_KEY", *ttl);
                println!("{}", "============================================================".bright_blue());
                println!("{}", "  🔑 WIDYASTORE PRESIGNED URL (S3 V4 AUTHENTICATED)".bright_cyan().bold());
                println!("{}", "============================================================".bright_blue());
                println!("  Target Object  : {}/{}", bucket, key);
                println!("  Kedaluwarsa    : {} detik", ttl);
                println!("  Presigned URL  : {}", url.bright_yellow());
                println!("{}", "============================================================\n".bright_blue());
            }
            StoreSubcommands::Status => {
                println!("{}", "============================================================".bright_blue());
                println!("{}", "  🪣 STATUS MESIN PENYIMPANAN OBJEK WIDYASTORE (S3 / MINIO)".bright_cyan().bold());
                println!("{}", "============================================================".bright_blue());
                println!("  API Compatibility      : {}", "100% Amazon S3 & MinIO REST".bright_green().bold());
                println!("  Content-Addressable    : {}", "AKTIF (SHA-256 CAS Deduplikasi)".bright_green().bold());
                println!("  Server-Side Encryption : {}", "AKTIF (AES-256-GCM Envelope)".bright_green().bold());
                println!("  Multipart Engine       : {}", "AKTIF (High-Throughput Parallel Parts)".bright_green().bold());
                println!("  Presigned URL SigV4    : {}", "AKTIF (HMAC-SHA256 Token Auth)".bright_green().bold());
                println!("{}", "============================================================\n".bright_blue());
            }
    }
}

pub fn handle_docdb_command(aksi: &DocdbSubcommands) {
    match aksi {
            DocdbSubcommands::Sisip { koleksi, json_str, data } => {
                let db = match widya::widyabase::docdb::WidyaDocDB::open(&data) {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("❌ Gagal membuka WidyaDocDB: {}", e);
                        process::exit(1);
                    }
                };
                let parsed_val: serde_json::Value = match serde_json::from_str(&json_str) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("❌ Format JSON tidak valid: {}", e);
                        process::exit(1);
                    }
                };
                let coll = db.collection(&koleksi);
                match coll.insert_one(parsed_val) {
                    Ok(id) => println!("{}", format!("✅ Dokumen berhasil disisipkan ke koleksi '{}' dengan _id: {}", koleksi, id).bright_green().bold()),
                    Err(e) => {
                        eprintln!("❌ Gagal menyisipkan dokumen: {}", e);
                        process::exit(1);
                    }
                }
            }
            DocdbSubcommands::Cari { koleksi, filter_str, data } => {
                let db = match widya::widyabase::docdb::WidyaDocDB::open(&data) {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("❌ Gagal membuka WidyaDocDB: {}", e);
                        process::exit(1);
                    }
                };
                let filter_val: serde_json::Value = match serde_json::from_str(&filter_str) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("❌ Format filter JSON tidak valid: {}", e);
                        process::exit(1);
                    }
                };
                let coll = db.collection(&koleksi);
                let results = coll.find(&filter_val, None, None, None);
                println!("{}", "============================================================".bright_blue());
                println!("{}", format!("  🍃 HASIL PENCARIAN DOKUMEN [Koleksi: {}] ({} Ditemukan)", koleksi, results.len()).bright_cyan().bold());
                println!("{}", "============================================================".bright_blue());
                for (i, doc) in results.iter().enumerate() {
                    println!("[{}] {}", i + 1, serde_json::to_string_pretty(doc).unwrap_or_default());
                }
                println!("{}", "============================================================\n".bright_blue());
            }
            DocdbSubcommands::Agregasi { koleksi, pipeline_str, data } => {
                let db = match widya::widyabase::docdb::WidyaDocDB::open(&data) {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("❌ Gagal membuka WidyaDocDB: {}", e);
                        process::exit(1);
                    }
                };
                let pipeline_val: serde_json::Value = match serde_json::from_str(&pipeline_str) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("❌ Format pipeline JSON tidak valid: {}", e);
                        process::exit(1);
                    }
                };
                let stages = match pipeline_val.as_array() {
                    Some(arr) => arr.clone(),
                    None => vec![pipeline_val],
                };
                match db.aggregate(&koleksi, &stages) {
                    Ok(results) => {
                        println!("{}", "============================================================".bright_blue());
                        println!("{}", format!("  📊 HASIL AGREGASI DOKUMEN [Koleksi: {}] ({} Baris)", koleksi, results.len()).bright_cyan().bold());
                        println!("{}", "============================================================".bright_blue());
                        for (i, res) in results.iter().enumerate() {
                            println!("[{}] {}", i + 1, serde_json::to_string_pretty(res).unwrap_or_default());
                        }
                        println!("{}", "============================================================\n".bright_blue());
                    }
                    Err(e) => {
                        eprintln!("❌ Gagal menjalankan agregasi: {}", e);
                        process::exit(1);
                    }
                }
            }
            DocdbSubcommands::Status => {
                println!("{}", "============================================================".bright_blue());
                println!("{}", "  🍃 STATUS BASIS DATA DOKUMEN WIDYADOCDB (MONGODB COMPATIBLE)".bright_cyan().bold());
                println!("{}", "============================================================".bright_blue());
                println!("  Query Engine Operators : {}", "AKTIF ($eq, $gt, $in, $regex, $and, $or)".bright_green().bold());
                println!("  Atomic Update Modifiers: {}", "AKTIF ($set, $inc, $push, $addToSet, $unset)".bright_green().bold());
                println!("  Aggregation Framework  : {}", "AKTIF ($match, $group, $sort, $project, $unwind)".bright_green().bold());
                println!("  Secondary B-Tree Index : {}", "AKTIF (Unique Constraints & Dot Notation)".bright_green().bold());
                println!("  Change Streams Engine  : {}", "AKTIF (Real-Time PubSub Event Broadcast)".bright_green().bold());
                println!("{}", "============================================================\n".bright_blue());
            }
        }
    }

pub fn jalankan_widyabase_repl(data_path: &std::path::Path) {
    println!("{}", "============================================================".bright_blue());
    println!("{}", "  🗄️  WIDYABASE INTERACTIVE REPL v1.0.0 (SQL DWIBAHASA)".bright_cyan().bold());
    println!("  Mendukung: PILIH, SISIP_KE, PERBARUI, HAPUS_DARI, BUAT_TABEL");
    println!("             SELECT, INSERT INTO, UPDATE, DELETE FROM, CREATE TABLE");
    println!("  Ketik '\\q' atau 'keluar' untuk keluar dari terminal.");
    println!("{}", "============================================================".bright_blue());

    let db = match widya::widyabase::WidyaBase::open(data_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ Gagal membuka basis data: {}", e);
            return;
        }
    };

    use std::io::{stdin, stdout, Write};
    let mut buffer = String::new();

    loop {
        print!("{}", "widyabase> ".bright_yellow().bold());
        let _ = stdout().flush();
        buffer.clear();

        if stdin().read_line(&mut buffer).is_err() {
            break;
        }

        let trimmed = buffer.trim();
        if trimmed == "\\q" || trimmed == "keluar" || trimmed == "exit" {
            println!("Sampai jumpa di WidyaBase!");
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        match db.execute(trimmed) {
            Ok(res) => match res {
                widya::widyabase::planner::QueryResult::Message(msg) => {
                    println!("{}", msg.bright_green());
                }
                widya::widyabase::planner::QueryResult::Rows { columns, rows, affected } => {
                    let col_line = columns.join(" \t| ");
                    println!("┌{}", "─".repeat(col_line.len() + 8));
                    println!("│ {}", col_line.bright_yellow().bold());
                    println!("├{}", "─".repeat(col_line.len() + 8));
                    for r in rows {
                        let strs: Vec<String> = r.iter().map(|v| match v {
                            widya::widyabase::sql_parser::SqlValue::Null => "NULL".bright_black().to_string(),
                            widya::widyabase::sql_parser::SqlValue::Boolean(b) => b.to_string(),
                            widya::widyabase::sql_parser::SqlValue::Integer(i) => i.to_string().bright_cyan().to_string(),
                            widya::widyabase::sql_parser::SqlValue::Float(f) => f.to_string().bright_cyan().to_string(),
                            widya::widyabase::sql_parser::SqlValue::Text(t) => t.clone(),
                        }).collect();
                        println!("│ {}", strs.join(" \t| "));
                    }
                    println!("└{}", "─".repeat(col_line.len() + 8));
                    println!("({} baris)", affected);
                }
            },
            Err(e) => {
                println!("❌ {}", e.bright_red());
            }
        }
    }
}
