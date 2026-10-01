// ============================================================================
// WIDYABASE STANDALONE INDUSTRIAL DATABASE SERVER & CLI (widyabase.exe)
// ============================================================================
// Biner mandiri untuk WidyaBase - Mesin Basis Data Skala Industri
// - Mode Server: PostgreSQL Wire Protocol v3.0 (Port 5432)
// - Mode REPL Interaktif: Konsol SQL Dwibahasa (Nusantara & ANSI Global)
// - Mode CLI Batch: Eksekusi kueri langsung via flag atau pipe stdin
// - Utilitas: Inisialisasi, Pemeriksaan Kesehatan, Checkpoint, dan Vektor AI
// ============================================================================

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use serde_json::{json, Value as JsonValue};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::Instant;

use widya::widyabase::planner::QueryResult;
use widya::widyabase::sql_parser::SqlValue;
use widya::widyabase::WidyaBase;

#[derive(Parser)]
#[command(name = "WidyaBase")]
#[command(author = "Widya-Lang Team <info@widya-lang.org>")]
#[command(version = "1.0.0")]
#[command(
    about = "🗄️ WidyaBase: Mesin Basis Data Mandiri Skala Industri (Industrial Autonomous Database)",
    long_about = "WidyaBase adalah mesin basis data ACID mandiri berkinerja tinggi yang mendukung:\n\
                  • Slotted-Page Storage 4096-byte dengan Buffer Pool Manager LRU\n\
                  • Write-Ahead Logging (WAL) dengan ARIES Crash Recovery\n\
                  • Multi-Version Concurrency Control (MVCC) Snapshot Isolation\n\
                  • B+Tree Indexing O(log N) & Vektor HNSW Cosine Similarity untuk AI\n\
                  • SQL Dwibahasa Simetris: Bahasa Indonesia Nusantara & Standar ANSI Global\n\
                  • PostgreSQL Wire Protocol v3.0 Network Daemon (kompatibel DBeaver, psql, pg driver)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path ke berkas basis data (.wdb)
    #[arg(short, long, value_name = "BERKAS_DB", default_value = "widyabase.wdb", global = true)]
    data: PathBuf,

    /// Format keluaran hasil kueri (tabel, json, csv)
    #[arg(short, long, value_enum, default_value = "tabel", global = true)]
    format: OutputFormat,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq)]
enum OutputFormat {
    Tabel,
    Json,
    Csv,
}

#[derive(Subcommand)]
enum Commands {
    /// Menjalankan server basis data daemon (PostgreSQL Wire Protocol v3.0)
    #[command(alias = "start", alias = "serve")]
    Mulai {
        /// Port TCP untuk mendengarkan koneksi pgwire (default: 5432)
        #[arg(short, long, default_value_t = 5432)]
        port: u16,

        /// Alamat antarmuka jaringan yang diikat (default: 0.0.0.0)
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
    },

    /// Membuka terminal interaktif SQL dwibahasa (REPL)
    #[command(alias = "repl", alias = "console")]
    Sql,

    /// Menginisialisasi basis data baru (.wdb) beserta Write-Ahead Log (.wal)
    #[command(alias = "init", alias = "create")]
    Inisialisasi,

    /// Mengeksekusi satu baris perintah kueri SQL langsung dari argumen CLI
    #[command(alias = "query", alias = "exec")]
    Kueri {
        /// String kueri SQL atau '-' untuk membaca dari stdin
        #[arg(value_name = "SQL")]
        sql: Option<String>,

        /// Path ke berkas skrip SQL untuk dieksekusi
        #[arg(short, long, value_name = "BERKAS_SQL")]
        berkas: Option<PathBuf>,
    },

    /// Memeriksa integritas berkas, halaman buffer pool, dan WAL
    #[command(alias = "status", alias = "health")]
    Periksa,

    /// Melakukan sinkronisasi halaman kotor ke disk dan menulis checkpoint WAL
    #[command(alias = "checkpoint", alias = "sync")]
    Sinkronisasi,

    /// Utilitas Multi-Modal AI: kalkulasi kemiripan kosinus embedding vektor
    #[command(alias = "vector")]
    Vektor {
        /// Vektor A dalam format JSON array float (contoh: "[0.1, 0.9, 0.5]")
        #[arg(value_name = "VEKTOR_A")]
        vektor_a: String,

        /// Vektor B dalam format JSON array float (contoh: "[0.2, 0.8, 0.4]")
        #[arg(value_name = "VEKTOR_B")]
        vektor_b: String,
    },

    /// Menampilkan panduan sintaksis SQL dwibahasa (Nusantara vs ANSI Global)
    #[command(alias = "bantuan")]
    Panduan,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // Mode default jika dipanggil tanpa sub-perintah: langsung buka REPL interaktif
            jalankan_repl(&cli.data, cli.format);
        }
        Some(Commands::Mulai { port, host }) => {
            jalankan_server(&cli.data, &host, port);
        }
        Some(Commands::Sql) => {
            jalankan_repl(&cli.data, cli.format);
        }
        Some(Commands::Inisialisasi) => {
            jalankan_inisialisasi(&cli.data);
        }
        Some(Commands::Kueri { sql, berkas }) => {
            jalankan_kueri(&cli.data, sql, berkas, cli.format);
        }
        Some(Commands::Periksa) => {
            jalankan_periksa(&cli.data);
        }
        Some(Commands::Sinkronisasi) => {
            jalankan_sinkronisasi(&cli.data);
        }
        Some(Commands::Vektor { vektor_a, vektor_b }) => {
            jalankan_vektor(&vektor_a, &vektor_b);
        }
        Some(Commands::Panduan) => {
            tampilkan_panduan();
        }
    }
}

// ============================================================================
// SERVER DAEMON (PGWIRE v3.0)
// ============================================================================

fn jalankan_server(data_path: &Path, host: &str, port: u16) {
    println!("{}", "======================================================================".bright_blue());
    println!("{}", "  🗄️  WIDYABASE DATABASE SERVER v1.0.0 (SKALA INDUSTRI)".bright_cyan().bold());
    println!("{}", "======================================================================".bright_blue());
    println!("  📂 Berkas Basis Data  : {}", data_path.display().to_string().bright_yellow());
    println!("  📝 Write-Ahead Log    : {}.wal", data_path.display().to_string().bright_yellow());
    println!("  🌐 Protokol Jaringan  : {}", "PostgreSQL Wire Protocol v3.0 (pgwire)".bright_green());
    println!("  🔌 Bind Interface     : {}:{}", host.bright_white(), port.to_string().bright_cyan().bold());
    println!("  ⚡ Buffer Pool        : 512 Halaman (2 MB Cache, LRU Eviction)");
    println!("  🛡️  Isolasi Transaksi : MVCC Snapshot Isolation (xmin / xmax)");
    println!("  🔎 Indeks & AI        : B+Tree O(log N) + Vektor HNSW Cosine Distance");
    println!("  🗣️  Dialek SQL        : Dwi-Bahasa (Indonesia Nusantara & ANSI Global)");
    println!("{}", "──────────────────────────────────────────────────────────────────────".bright_black());
    println!("  Kompatibel dengan:");
    println!("    • CLI Client     : psql -h {} -p {} -U widya widyabase", host, port);
    println!("    • GUI IDE        : DBeaver, TablePlus, pgAdmin, DataGrip");
    println!("    • Web / Backend  : Node pg, Python psycopg2, Rust postgres, Go pgx");
    println!("{}", "======================================================================".bright_blue());

    let db = match WidyaBase::open(data_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ [WidyaBase] Gagal menginisialisasi basis data: {}", e.bright_red());
            std::process::exit(1);
        }
    };

    println!("🚀 [WidyaBase] Daemon siap menerima koneksi client...");
    if let Err(e) = db.start_pgwire_server(port) {
        eprintln!("❌ [WidyaBase] Kesalahan server socket: {}", e.to_string().bright_red());
        std::process::exit(1);
    }
}

// ============================================================================
// INTERACTIVE REPL
// ============================================================================

fn jalankan_repl(data_path: &Path, mut default_format: OutputFormat) {
    println!("{}", "======================================================================".bright_blue());
    println!("{}", "  🗄️  WIDYABASE INTERACTIVE SHELL v1.0.0 (SQL DWIBAHASA)".bright_cyan().bold());
    println!("{}", "======================================================================".bright_blue());
    println!("  Mendukung sintaksis dwibahasa simetris:");
    println!("  • Nusantara   : PILIH, DARI, DIMANA, SISIP_KE, PERBARUI, HAPUS_DARI, BUAT_TABEL");
    println!("  • ANSI Global : SELECT, FROM, WHERE, INSERT INTO, UPDATE, DELETE FROM, CREATE TABLE");
    println!("  Perintah Meta Shell:");
    println!("    \\dt, .tabel    : Menampilkan daftar seluruh tabel");
    println!("    \\d <tabel>     : Menampilkan struktur skema kolom tabel");
    println!("    \\status        : Laporan kesehatan, buffer pool, & WAL");
    println!("    \\sinkron       : Checkpoint paksa ke disk");
    println!("    \\format <fmt>  : Ganti format output (tabel | json | csv)");
    println!("    \\h, .bantuan   : Panduan sintaksis kueri lengkap");
    println!("    \\q, keluar     : Keluar dari shell interaktif");
    println!("{}", "======================================================================".bright_blue());

    let db = match WidyaBase::open(data_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ Gagal membuka basis data di '{}': {}", data_path.display(), e.bright_red());
            return;
        }
    };

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("⚠️ Gagal menginisialisasi terminal editor: {}", e);
            return;
        }
    };

    // Muat riwayat jika ada
    let history_file = ".widyabase_history";
    let _ = rl.load_history(history_file);

    let mut multiline_buf = String::new();
    let mut show_timing = true;

    loop {
        let prompt = if multiline_buf.is_empty() {
            format!("widyabase [{}]> ", data_path.file_name().and_then(|n| n.to_str()).unwrap_or("db"))
                .bright_yellow()
                .bold()
                .to_string()
        } else {
            "        ...> ".bright_blue().to_string()
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim();

                // Deteksi meta-commands hanya jika buffer multi-baris kosong
                if multiline_buf.is_empty() {
                    if trimmed == "\\q" || trimmed == "keluar" || trimmed == "exit" || trimmed == "quit" {
                        println!("Sampai jumpa di WidyaBase! 👋");
                        break;
                    }
                    if trimmed == "\\h" || trimmed == ".bantuan" || trimmed == "help" {
                        tampilkan_panduan();
                        continue;
                    }
                    if trimmed == "\\c" || trimmed == "clear" || trimmed == ".bersih" {
                        print!("\x1B[2J\x1B[1;1H");
                        continue;
                    }
                    if trimmed == "\\dt" || trimmed == ".tabel" || trimmed == "tables" {
                        tampilkan_daftar_tabel(&db);
                        continue;
                    }
                    if trimmed.starts_with("\\d ") || trimmed.starts_with(".skema ") {
                        let tbl_name = trimmed.split_whitespace().nth(1).unwrap_or("");
                        tampilkan_skema_tabel(&db, tbl_name);
                        continue;
                    }
                    if trimmed == "\\status" || trimmed == ".status" {
                        let rep = db.check_health();
                        println!("📊 Status Kesehatan Basis Data:");
                        println!("   • Total Halaman Slotted-Page : {}", rep.total_pages.to_string().bright_cyan());
                        println!("   • Jumlah Tabel Aktif        : {}", rep.total_tables.to_string().bright_cyan());
                        println!("   • Catatan Log WAL (Total)   : {}", rep.wal_records.to_string().bright_yellow());
                        println!("   • Status Buffer Pool        : {}", rep.buffer_pool_status.bright_green());
                        println!("   • Kondisi Integritas        : {}", if rep.is_healthy { "SEHAT (OK)".bright_green().bold() } else { "PERLU RECOVERY".bright_red().bold() });
                        continue;
                    }
                    if trimmed == "\\sinkron" || trimmed == ".checkpoint" {
                        match db.checkpoint() {
                            Ok(_) => println!("{}", "✅ Checkpoint berhasil: Buffer pool & WAL tersinkronisasi ke disk!".bright_green()),
                            Err(e) => eprintln!("❌ Checkpoint gagal: {}", e.bright_red()),
                        }
                        continue;
                    }
                    if trimmed == "\\waktu" {
                        show_timing = !show_timing;
                        println!("⏱️  Penghitung waktu eksekusi: {}", if show_timing { "AKTIF".bright_green() } else { "NON-AKTIF".bright_red() });
                        continue;
                    }
                    if trimmed.starts_with("\\format ") {
                        let fmt_str = trimmed.trim_start_matches("\\format ").trim();
                        match fmt_str.to_lowercase().as_str() {
                            "tabel" | "table" => { default_format = OutputFormat::Tabel; println!("Format diubah ke: Tabel"); }
                            "json" => { default_format = OutputFormat::Json; println!("Format diubah ke: JSON"); }
                            "csv" => { default_format = OutputFormat::Csv; println!("Format diubah ke: CSV"); }
                            _ => println!("Format tidak dikenal! Gunakan: tabel, json, atau csv"),
                        }
                        continue;
                    }
                }

                if trimmed.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(line.as_str());

                // Akumulasi baris kueri
                if !multiline_buf.is_empty() {
                    multiline_buf.push(' ');
                }
                multiline_buf.push_str(trimmed);

                // Periksa apakah query diakhiri titik koma ';'
                if multiline_buf.ends_with(';') {
                    let kueri_lengkap = multiline_buf.clone();
                    multiline_buf.clear();

                    let t0 = Instant::now();
                    match db.execute(&kueri_lengkap) {
                        Ok(res) => {
                            let durasi = t0.elapsed();
                            cetak_hasil(&res, default_format);
                            if show_timing {
                                println!("{}", format!("(Waktu: {:.2?})", durasi).bright_black());
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ {}", e.bright_red());
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C membersihkan buffer
                if !multiline_buf.is_empty() {
                    multiline_buf.clear();
                    println!("^C (Kueri dibatalkan)");
                } else {
                    println!("Tekan Ctrl+D atau ketik '\\q' untuk keluar.");
                }
            }
            Err(ReadlineError::Eof) => {
                println!("Sampai jumpa di WidyaBase! 👋");
                break;
            }
            Err(e) => {
                eprintln!("Galat pembacaan terminal: {}", e);
                break;
            }
        }
    }

    let _ = rl.save_history(history_file);
}

// ============================================================================
// BATCH QUERY EXECUTION
// ============================================================================

fn jalankan_kueri(data_path: &Path, sql: Option<String>, berkas: Option<PathBuf>, format: OutputFormat) {
    let kueri = if let Some(b) = berkas {
        match fs::read_to_string(&b) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("❌ Gagal membaca berkas SQL '{}': {}", b.display(), e);
                std::process::exit(1);
            }
        }
    } else if let Some(s) = sql {
        if s == "-" {
            let mut buf = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut buf) {
                eprintln!("❌ Gagal membaca kueri dari stdin: {}", e);
                std::process::exit(1);
            }
            buf
        } else {
            s
        }
    } else {
        eprintln!("❌ Harap tentukan string kueri SQL atau tentukan berkas kueri (--berkas <PATH>)");
        std::process::exit(1);
    };

    let db = match WidyaBase::open(data_path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("❌ Gagal membuka basis data: {}", e);
            std::process::exit(1);
        }
    };

    let t0 = Instant::now();
    match db.execute(&kueri) {
        Ok(res) => {
            let durasi = t0.elapsed();
            cetak_hasil(&res, format);
            if format == OutputFormat::Tabel {
                eprintln!("{}", format!("(Waktu: {:.2?})", durasi).bright_black());
            }
        }
        Err(e) => {
            eprintln!("❌ Galat Eksekusi Kueri: {}", e.bright_red());
            std::process::exit(1);
        }
    }
}

// ============================================================================
// DATABASE INITIALIZATION
// ============================================================================

fn jalankan_inisialisasi(data_path: &Path) {
    if data_path.exists() {
        println!("⚠️  Berkas basis data '{}' sudah ada.", data_path.display().to_string().bright_yellow());
    }

    match WidyaBase::open(data_path) {
        Ok(_) => {
            println!("{}", "🎉 Basis Data WidyaBase Berhasil Diinisialisasi!".bright_green().bold());
            println!("📂 Berkas Penyimpanan : {}", data_path.display().to_string().bright_cyan());
            println!("📝 Write-Ahead Log    : {}.wal", data_path.display().to_string().bright_yellow());
            println!("💡 Buka terminal SQL  : {} sql --data {}", "widyabase".bright_yellow(), data_path.display());
            println!("🚀 Jalankan server pg : {} mulai --port 5432 --data {}", "widyabase".bright_yellow(), data_path.display());
        }
        Err(e) => {
            eprintln!("❌ Gagal inisialisasi basis data: {}", e);
            std::process::exit(1);
        }
    }
}

// ============================================================================
// HEALTH & INTEGRITY CHECK
// ============================================================================

fn jalankan_periksa(data_path: &Path) {
    println!("{}", format!("🔍 Memeriksa Integritas Basis Data: {}", data_path.display()).bright_cyan().bold());

    if !data_path.exists() {
        eprintln!("❌ Berkas basis data '{}' tidak ditemukan!", data_path.display());
        std::process::exit(1);
    }

    let meta = fs::metadata(data_path).unwrap();
    let wal_path = PathBuf::from(format!("{}.wal", data_path.display()));
    let wal_size = if wal_path.exists() {
        fs::metadata(&wal_path).map(|m| m.len()).unwrap_or(0)
    } else {
        0
    };

    match WidyaBase::open(data_path) {
        Ok(db) => {
            let rep = db.check_health();
            println!("┌────────────────────────────────────────────────────────────┐");
            println!("│ INDIKATOR SISTEM             │ STATUS / NILAI              │");
            println!("├────────────────────────────────────────────────────────────┤");
            println!("│ Ukuran Berkas Storage (.wdb) │ {:<27} │", format!("{} bytes ({:.2} KB)", meta.len(), meta.len() as f64 / 1024.0));
            println!("│ Ukuran Berkas WAL (.wal)     │ {:<27} │", format!("{} bytes ({:.2} KB)", wal_size, wal_size as f64 / 1024.0));
            println!("│ Total Halaman Slotted-Page   │ {:<27} │", rep.total_pages);
            println!("│ Jumlah Tabel Terdaftar       │ {:<27} │", rep.total_tables);
            println!("│ Total Rekaman Operasi WAL    │ {:<27} │", rep.wal_records);
            println!("│ Pengelola Buffer Pool        │ {:<27} │", rep.buffer_pool_status);
            println!("│ Status Integritas Keseluruhan│ {:<27} │", if rep.is_healthy { "SEHAT (BERSIH)".bright_green().bold().to_string() } else { "PERLU PEMULIHAN".bright_red().bold().to_string() });
            println!("└────────────────────────────────────────────────────────────┘");
            println!("✅ Basis data siap digunakan untuk transaksi produksi.");
        }
        Err(e) => {
            eprintln!("❌ Basis data rusak atau gagal dibuka: {}", e.bright_red());
            std::process::exit(1);
        }
    }
}

// ============================================================================
// CHECKPOINT & SYNC
// ============================================================================

fn jalankan_sinkronisasi(data_path: &Path) {
    println!("{}", format!("🔄 Melakukan Sinkronisasi & Checkpoint Disk: {}", data_path.display()).bright_cyan());
    match WidyaBase::open(data_path) {
        Ok(db) => {
            match db.checkpoint() {
                Ok(_) => {
                    println!("{}", "✅ Checkpoint berhasil!".bright_green().bold());
                    println!("   • Seluruh halaman kotor di Buffer Pool telah di-flush ke disk.");
                    println!("   • LSN Log WAL diselaraskan sebagai baseline pemulihan.");
                }
                Err(e) => {
                    eprintln!("❌ Galat checkpoint: {}", e.bright_red());
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Gagal membuka basis data: {}", e);
            std::process::exit(1);
        }
    }
}

// ============================================================================
// AI VECTOR UTILITY
// ============================================================================

fn jalankan_vektor(va_str: &str, vb_str: &str) {
    println!("{}", "🧠 Mesin Multi-Modal AI WidyaBase: Perhitungan Kemiripan Vektor".bright_cyan().bold());

    let parse_vec = |s: &str| -> Result<Vec<f32>, String> {
        let val: JsonValue = serde_json::from_str(s).map_err(|e| format!("JSON Vektor tidak valid: {}", e))?;
        if let JsonValue::Array(arr) = val {
            let mut f_vec = Vec::new();
            for item in arr {
                if let Some(f) = item.as_f64() {
                    f_vec.push(f as f32);
                } else {
                    return Err("Elemen vektor harus berupa angka numerik float".to_string());
                }
            }
            Ok(f_vec)
        } else {
            Err("Vektor harus berupa JSON array [f32, ...]".to_string())
        }
    };

    let v1 = match parse_vec(va_str) {
        Ok(v) => v,
        Err(e) => { eprintln!("❌ Galat Vektor A: {}", e); std::process::exit(1); }
    };
    let v2 = match parse_vec(vb_str) {
        Ok(v) => v,
        Err(e) => { eprintln!("❌ Galat Vektor B: {}", e); std::process::exit(1); }
    };

    let cosine = widya::widyabase::multimodal::VectorEngine::cosine_similarity(&v1, &v2);
    let euclidean = widya::widyabase::multimodal::VectorEngine::euclidean_distance(&v1, &v2);

    println!("  • Dimensi Vektor          : {} dimensi", v1.len().to_string().bright_yellow());
    println!("  • Cosine Similarity       : {}", format!("{:.6}", cosine).bright_green().bold());
    println!("  • Euclidean Distance      : {}", format!("{:.6}", euclidean).bright_cyan().bold());
    println!("  • Evaluasi Relevansi AI   : {}", if cosine > 0.85 { "SANGAT TINGGI (Sangat Mirip)".bright_green() } else if cosine > 0.6 { "SEDANG (Cukup Relevan)".bright_yellow() } else { "RENDAH (Kurang Mirip)".bright_red() });
}

// ============================================================================
// FORMATTERS & PRINTERS
// ============================================================================

fn cetak_hasil(res: &QueryResult, format: OutputFormat) {
    match res {
        QueryResult::Message(msg) => {
            println!("{}", msg.bright_green());
        }
        QueryResult::Rows { columns, rows, affected } => {
            match format {
                OutputFormat::Tabel => {
                    if columns.is_empty() {
                        println!("(0 kolom, {} baris terdampak)", affected);
                        return;
                    }

                    // Hitung lebar maksimal per kolom
                    let mut col_widths: Vec<usize> = columns.iter().map(|c| c.len()).collect();
                    for r in rows {
                        for (i, val) in r.iter().enumerate() {
                            if i < col_widths.len() {
                                let len = sql_value_to_string(val).len();
                                if len > col_widths[i] {
                                    col_widths[i] = len;
                                }
                            }
                        }
                    }

                    // Cetak Header
                    print!("┌");
                    for (i, w) in col_widths.iter().enumerate() {
                        print!("{}", "─".repeat(w + 2));
                        if i < col_widths.len() - 1 { print!("┬"); }
                    }
                    println!("┐");

                    print!("│");
                    for (i, col) in columns.iter().enumerate() {
                        let pad = col_widths[i] - col.len();
                        print!(" {} {}{}", col.bright_yellow().bold(), " ".repeat(pad), "│");
                    }
                    println!();

                    print!("├");
                    for (i, w) in col_widths.iter().enumerate() {
                        print!("{}", "─".repeat(w + 2));
                        if i < col_widths.len() - 1 { print!("┼"); }
                    }
                    println!("┤");

                    // Cetak Baris
                    for r in rows {
                        print!("│");
                        for (i, val) in r.iter().enumerate() {
                            let val_str = sql_value_to_string(val);
                            let pad = if col_widths[i] >= val_str.len() { col_widths[i] - val_str.len() } else { 0 };
                            let colored_val = match val {
                                SqlValue::Null => "NULL".bright_black().to_string(),
                                SqlValue::Boolean(b) => b.to_string().bright_magenta().to_string(),
                                SqlValue::Integer(v) => v.to_string().bright_cyan().to_string(),
                                SqlValue::Float(v) => format!("{:.2}", v).bright_cyan().to_string(),
                                SqlValue::Text(s) => s.clone(),
                            };
                            print!(" {}{}{} │", colored_val, " ".repeat(pad), "");
                        }
                        println!();
                    }

                    print!("└");
                    for (i, w) in col_widths.iter().enumerate() {
                        print!("{}", "─".repeat(w + 2));
                        if i < col_widths.len() - 1 { print!("┴"); }
                    }
                    println!("┘");

                    println!("({} baris)", affected);
                }
                OutputFormat::Json => {
                    let mut json_rows = Vec::new();
                    for r in rows {
                        let mut obj = serde_json::Map::new();
                        for (i, col) in columns.iter().enumerate() {
                            let val = r.get(i).unwrap_or(&SqlValue::Null);
                            let json_val = match val {
                                SqlValue::Null => JsonValue::Null,
                                SqlValue::Boolean(b) => JsonValue::Bool(*b),
                                SqlValue::Integer(i) => json!(i),
                                SqlValue::Float(f) => json!(f),
                                SqlValue::Text(t) => JsonValue::String(t.clone()),
                            };
                            obj.insert(col.clone(), json_val);
                        }
                        json_rows.push(JsonValue::Object(obj));
                    }
                    println!("{}", serde_json::to_string_pretty(&json_rows).unwrap_or_else(|_| "[]".to_string()));
                }
                OutputFormat::Csv => {
                    println!("{}", columns.join(","));
                    for r in rows {
                        let row_strs: Vec<String> = r.iter().map(|v| match v {
                            SqlValue::Null => "".to_string(),
                            SqlValue::Boolean(b) => b.to_string(),
                            SqlValue::Integer(i) => i.to_string(),
                            SqlValue::Float(f) => f.to_string(),
                            SqlValue::Text(t) => {
                                if t.contains(',') || t.contains('"') || t.contains('\n') {
                                    format!("\"{}\"", t.replace('"', "\"\""))
                                } else {
                                    t.clone()
                                }
                            }
                        }).collect();
                        println!("{}", row_strs.join(","));
                    }
                }
            }
        }
    }
}

fn sql_value_to_string(val: &SqlValue) -> String {
    match val {
        SqlValue::Null => "NULL".to_string(),
        SqlValue::Boolean(b) => b.to_string(),
        SqlValue::Integer(i) => i.to_string(),
        SqlValue::Float(f) => format!("{:.2}", f),
        SqlValue::Text(t) => t.clone(),
    }
}

fn tampilkan_daftar_tabel(db: &WidyaBase) {
    let cat = db.engine.catalog.read().unwrap();
    if cat.is_empty() {
        println!("ℹ️  Belum ada tabel yang dibuat dalam basis data ini.");
        println!("   Gunakan 'BUAT_TABEL nama (id INTEGER, nama TEKS);' untuk membuat.");
        return;
    }

    println!("┌────────────────────────────┬─────────────┬──────────────────────────┐");
    println!("│ NAMA TABEL                 │ JML KOLOM   │ KUNCI UTAMA (PRIMARY KEY)│");
    println!("├────────────────────────────┼─────────────┼──────────────────────────┤");
    for (name, schema) in cat.iter() {
        let pk = schema.primary_key.as_deref().unwrap_or("-");
        println!("│ {:<26} │ {:<11} │ {:<24} │", name.bright_yellow(), schema.columns.len(), pk.bright_cyan());
    }
    println!("└────────────────────────────┴─────────────┴──────────────────────────┘");
    println!("({} tabel terdaftar)", cat.len());
}

fn tampilkan_skema_tabel(db: &WidyaBase, tbl_name: &str) {
    if tbl_name.is_empty() {
        println!("⚠️  Harap tentukan nama tabel: \\d <NAMA_TABEL>");
        return;
    }

    let cat = db.engine.catalog.read().unwrap();
    let upper = tbl_name.to_uppercase();
    if let Some(schema) = cat.get(&upper) {
        println!("📋 Skema Tabel: {}", schema.name.bright_yellow().bold());
        println!("┌────────────────────────────┬────────────────────────────┐");
        println!("│ NAMA KOLOM                 │ TIPE DATA                  │");
        println!("├────────────────────────────┼────────────────────────────┤");
        for (col, ty) in &schema.columns {
            let is_pk = schema.primary_key.as_deref() == Some(col.as_str());
            let ty_display = if is_pk {
                format!("{} (PRIMARY KEY)", ty).bright_cyan().bold().to_string()
            } else {
                ty.clone()
            };
            println!("│ {:<26} │ {:<26} │", col, ty_display);
        }
        println!("└────────────────────────────┴────────────────────────────┘");
    } else {
        println!("❌ Tabel '{}' tidak ditemukan di dalam katalog!", tbl_name);
    }
}

fn tampilkan_panduan() {
    println!("{}", "======================================================================".bright_blue());
    println!("{}", "  📖 PANDUAN SQL DWIBAHASA WIDYABASE (NUSANTARA vs ANSI GLOBAL)".bright_cyan().bold());
    println!("{}", "======================================================================".bright_blue());
    println!("  WidyaBase menerima dialek Bahasa Indonesia atau ANSI SQL secara transparan:\n");
    println!("  ┌───────────────────────────────────┬───────────────────────────────────┐");
    println!("  │ BAHASA INDONESIA (NUSANTARA)      │ ANSI GLOBAL SQL (ENGLISH)         │");
    println!("  ├───────────────────────────────────┼───────────────────────────────────┤");
    println!("  │ BUAT_TABEL produk (               │ CREATE TABLE produk (             │");
    println!("  │   id INTEGER KUNCI_UTAMA,         │   id INTEGER PRIMARY KEY,         │");
    println!("  │   nama TEKS, harga DESIMAL        │   nama TEXT, harga DECIMAL        │");
    println!("  │ );                                │ );                                │");
    println!("  ├───────────────────────────────────┼───────────────────────────────────┤");
    println!("  │ SISIP_KE produk (id, nama, harga) │ INSERT INTO produk(id,nama,harga) │");
    println!("  │ NILAI (1, 'Kopi Java', 25000);    │ VALUES (1, 'Kopi Java', 25000);   │");
    println!("  ├───────────────────────────────────┼───────────────────────────────────┤");
    println!("  │ PILIH * DARI produk               │ SELECT * FROM produk              │");
    println!("  │ DIMANA harga >= 20000             │ WHERE harga >= 20000              │");
    println!("  │ URUTKAN BERDASARKAN harga TURUN   │ ORDER BY harga DESC               │");
    println!("  │ BATAS 10;                         │ LIMIT 10;                         │");
    println!("  ├───────────────────────────────────┼───────────────────────────────────┤");
    println!("  │ PERBARUI produk                   │ UPDATE produk                     │");
    println!("  │ ATUR harga = 27000                │ SET harga = 27000                 │");
    println!("  │ DIMANA id = 1;                    │ WHERE id = 1;                     │");
    println!("  ├───────────────────────────────────┼───────────────────────────────────┤");
    println!("  │ HAPUS_DARI produk DIMANA id = 1;  │ DELETE FROM produk WHERE id = 1;  │");
    println!("  ├───────────────────────────────────┼───────────────────────────────────┤");
    println!("  │ PILIH HITUNG(*), JUMLAH(harga)    │ SELECT COUNT(*), SUM(harga)       │");
    println!("  │ DARI produk;                      │ FROM produk;                      │");
    println!("  └───────────────────────────────────┴───────────────────────────────────┘");
    println!("\n  Agregasi: HITUNG (COUNT), JUMLAH (SUM), RATA_RATA (AVG), MIN, MAKS (MAX)");
    println!("  Logika  : DAN (AND), ATAU (OR), BUKAN (NOT), NIHIL (NULL)");
    println!("{}", "======================================================================".bright_blue());
}
