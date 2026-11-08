// ============================================================================
// WIDYACODER CLI: ASISTEN AI PEMROGRAMAN BERDAULAT 100% LOKAL (src/bin/widyacoder.rs)
// ============================================================================
// CLI Cerdas Berbasis WidyaBase HNSW Local RAG & LLM Engine Mandiri:
// - indeks : Memindai dokumentasi (.md) & contoh kode (.wya), membangun vektor HNSW lokal
// - tanya  : Menjawab pertanyaan pemrograman dengan RAG hibrida + LLM lokal (Ollama / llama.cpp)
// - chat   : Konsol REPL interaktif pemrograman berdaulat
// - status : Memeriksa kesehatan basis pengetahuan & ketersediaan model LLM lokal
// - serve  : Menjalankan API Server HTTP Lokal (Port 8088) untuk ekstensi IDE & alat luar
// ============================================================================

use clap::{Parser, Subcommand};
use colored::*;
use rustyline::DefaultEditor;
use serde_json::json;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::time::Instant;

use widya::ai::widyacoder::{KnowledgeBase, KnowledgeMatch, WidyaCoderEngine};

const BANNER: &str = r#"
╔══════════════════════════════════════════════════════════════════════╗
║  ╦ ╦┬╔╦╗╦ ╦╔═╗╔═╗╔═╗╔╦╗╔═╗╦═╗   WidyaCoder v2.6.0 GA Stable         ║
║  ║║║│ ║║╚╦╝╠═╣║  ║ ║ ║║║╣ ╠╦╝   Asisten AI Pemrograman Berdaulat     ║
║  ╚╩╝┴═╩╝ ╩ ╩ ╩╚═╝╚═╝═╩╝╚═╝╩╚═   100% Lokal • HNSW RAG • CPU Native   ║
╚══════════════════════════════════════════════════════════════════════╝
"#;

#[derive(Parser)]
#[command(name = "widyacoder")]
#[command(author = "Tim Pengembang Widya-Lang <dev@widya-lang.org>")]
#[command(version = "0.1.0")]
#[command(
    about = "🤖 WidyaCoder: Asisten AI Pemrograman Berdaulat Berbasis WidyaBase Local RAG",
    long_about = "WidyaCoder adalah asisten AI lokal mandiri yang memahami seluruh ekosistem Widya-Lang.\n\
                  Menggabungkan pencarian leksikal BM25 dan vektor semantik HNSW 384-dimensi WidyaBase\n\
                  dengan model open-source lokal (Ollama / llama.cpp / Qwen / Llama) tanpa ketergantungan cloud."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Lokasi berkas cache basis pengetahuan JSON
    #[arg(long, default_value = "widyacoder_kb.json", global = true)]
    kb: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Bangun atau perbarui indeks vektor basis pengetahuan dari ruang kerja
    #[command(name = "indeks", alias = "index")]
    Indeks {
        /// Direktori akar ruang kerja untuk dipindai
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
    },

    /// Ajukan pertanyaan teknis atau minta pembuatan kode Widya-Lang
    #[command(name = "tanya", alias = "ask")]
    Tanya {
        /// Kalimat pertanyaan atau instruksi tugas
        query: String,

        /// Jumlah rujukan potongan dokumen paling relevan yang disuntikkan
        #[arg(short = 'k', long, default_value = "4")]
        top_k: usize,

        /// Nama model LLM lokal (Ollama / llama.cpp)
        #[arg(short, long)]
        model: Option<String>,

        /// URL endpoint LLM lokal (misal: http://localhost:11434 atau http://localhost:8080)
        #[arg(short, long)]
        endpoint: Option<String>,

        /// Hanya cetak prompt RAG & rujukan referensi tanpa memanggil LLM
        #[arg(long)]
        prompt_only: bool,
    },

    /// Masuk ke sesi percakapan interaktif (REPL) di terminal
    #[command(name = "chat", alias = "repl")]
    Chat {
        /// Jumlah rujukan potongan dokumen paling relevan
        #[arg(short = 'k', long, default_value = "4")]
        top_k: usize,

        /// Nama model LLM lokal
        #[arg(short, long)]
        model: Option<String>,

        /// URL endpoint LLM lokal
        #[arg(short, long)]
        endpoint: Option<String>,
    },

    /// Periksa status basis pengetahuan lokal dan server LLM
    #[command(name = "status", alias = "info")]
    Status,

    /// Jalankan HTTP API Server lokal untuk integrasi editor/IDE
    #[command(name = "serve", alias = "server")]
    Serve {
        /// Port pendengar HTTP lokal
        #[arg(short, long, default_value = "8088")]
        port: u16,

        /// Nama model LLM lokal
        #[arg(short, long)]
        model: Option<String>,

        /// URL endpoint LLM lokal
        #[arg(short, long)]
        endpoint: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Indeks { dir } => {
            cmd_indeks(&dir, &cli.kb);
        }
        Commands::Tanya {
            query,
            top_k,
            model,
            endpoint,
            prompt_only,
        } => {
            cmd_tanya(&query, top_k, model.as_deref(), endpoint.as_deref(), prompt_only, &cli.kb);
        }
        Commands::Chat { top_k, model, endpoint } => {
            cmd_chat(top_k, model.as_deref(), endpoint.as_deref(), &cli.kb);
        }
        Commands::Status => {
            cmd_status(&cli.kb);
        }
        Commands::Serve { port, model, endpoint } => {
            cmd_serve(port, model.as_deref(), endpoint.as_deref(), &cli.kb);
        }
    }
}

// ============================================================================
// PERINTAH: INDEKS (INDEX)
// ============================================================================
fn cmd_indeks(workspace_root: &Path, kb_path: &Path) {
    println!("{}", BANNER.cyan());
    println!("{}", "🔍 MEMULAI PENGINDEKSAN BASIS PENGETAHUAN WIDYA-LANG...".bold().green());
    println!("Direktori Target : {}", workspace_root.display().to_string().yellow());
    println!("Berkas Keluaran  : {}", kb_path.display().to_string().yellow());

    let start = Instant::now();
    let mut engine = WidyaCoderEngine::new();

    match engine.build_from_workspace(workspace_root) {
        Ok(_) => {
            let total = engine.kb.total_chunks;
            let duration = start.elapsed();

            if let Err(e) = engine.kb.save_to_file(kb_path) {
                eprintln!("{} Gagal menyimpan berkas basis data: {}", "❌ ERROR:".bold().red(), e);
                std::process::exit(1);
            }

            println!("\n{}", "✅ PENGINDEKSAN SELESAI DENGAN SUKSES!".bold().green());
            println!("  • Total Potongan Pengetahuan (Chunks) : {}", total.to_string().bold().yellow());
            println!("  • Dimensi Vektor HNSW                  : {}", "384 (L2 Normalized)".cyan());
            println!("  • Waktu Pemrosesan & Embedding        : {:.2?}", duration);
            println!("  • Basis Data Tersimpan di             : {}", kb_path.display().to_string().bold());
            println!("\n{}", "💡 Coba jalankan: widyacoder tanya \"bagaimana cara membuat server web di widya-lang?\"".italic().bright_black());
        }
        Err(e) => {
            eprintln!("{} Gagal memindai ruang kerja: {}", "❌ ERROR:".bold().red(), e);
            std::process::exit(1);
        }
    }
}

// ============================================================================
// PERINTAH: TANYA (ASK)
// ============================================================================
fn cmd_tanya(
    query: &str,
    top_k: usize,
    custom_model: Option<&str>,
    custom_endpoint: Option<&str>,
    prompt_only: bool,
    kb_path: &Path,
) {
    let engine = load_engine_or_exit(kb_path);

    println!("{}", "🔎 Mencari potongan rujukan terdekat (Hybrid BM25 + Vektor HNSW)...".italic().dimmed());
    let matches = engine.search(query, top_k);

    if matches.is_empty() {
        println!("{}", "⚠️  Tidak ditemukan rujukan relevan untuk pertanyaan ini.".yellow());
        return;
    }

    println!("\n{}", "📚 Rujukan Pengetahuan Ditemukan:".bold().cyan());
    for (i, m) in matches.iter().enumerate() {
        println!(
            "  [{}] {} (Kategori: {}, RRF: {:.4})",
            i + 1,
            m.chunk.source_path.bold().white(),
            m.chunk.category.yellow(),
            m.rrf_score
        );
        let preview: String = m.chunk.content.lines().take(2).collect::<Vec<_>>().join(" ");
        println!("      \"{}\"...", preview.dimmed());
    }

    let (system_prompt, user_prompt) = engine.format_prompt(query, &matches);

    if prompt_only {
        println!("\n{}", "=== [SYSTEM PROMPT DENGAN RAG CONTEXT] ===".bold().blue());
        println!("{}", system_prompt);
        println!("\n{}", "=== [USER PROMPT] ===".bold().blue());
        println!("{}", user_prompt);
        return;
    }

    // Tentukan endpoint & model
    let (endpoint, model) = match (custom_endpoint, custom_model) {
        (Some(ep), Some(m)) => (ep.to_string(), m.to_string()),
        (Some(ep), None) => (ep.to_string(), "ornith-coder:latest".to_string()),
        (None, Some(m)) => {
            let ep = WidyaCoderEngine::probe_local_llm()
                .map(|(e, _)| e)
                .unwrap_or_else(|| "http://localhost:11434".to_string());
            (ep, m.to_string())
        }
        (None, None) => match WidyaCoderEngine::probe_local_llm() {
            Some((ep, m)) => (ep, m),
            None => {
                println!("\n{}", "ℹ️  Server LLM Lokal Belum Terdeteksi Aktif.".bold().yellow());
                println!("    WidyaCoder telah menyusun prompt berkonteks penuh.");
                println!("    Untuk inferensi lokal otomatis, jalankan salah satu di latar belakang:");
                println!("      • Ollama : {} (atau {})", "ollama run ornith-coder:latest".cyan(), "ollama run llama3.2:latest".cyan());
                println!("      • llama  : {}", "llama-server -m <model>.gguf --port 8080".cyan());
                println!("\n{}", "=== [PROMPT RAG TEROPTIMALISASI SIAP GUNAKAN] ===".bold().green());
                println!("{}\n", system_prompt);
                return;
            }
        },
    };

    println!(
        "\n{} Memanggil model {} di {}...",
        "⚡".bold(),
        model.bold().yellow(),
        endpoint.cyan()
    );

    let start_llm = Instant::now();
    match WidyaCoderEngine::query_llm(&endpoint, &model, &system_prompt, &user_prompt) {
        Ok(jawaban) => {
            let duration = start_llm.elapsed();
            println!("\n{}", "🤖 Jawaban WidyaCoder:".bold().green());
            println!("{}\n", jawaban.trim());
            println!(
                "{}",
                format!("(Waktu inferensi lokal: {:.2?})", duration).dimmed()
            );
        }
        Err(e) => {
            eprintln!("\n{} {}", "❌ Gagal memanggil LLM:".bold().red(), e);
            println!("\n{}", "💡 Menampilkan Prompt RAG yang telah disusun:".yellow());
            println!("{}\n", system_prompt);
        }
    }
}

// ============================================================================
// PERINTAH: CHAT (INTERACTIVE REPL)
// ============================================================================
fn cmd_chat(
    top_k: usize,
    custom_model: Option<&str>,
    custom_endpoint: Option<&str>,
    kb_path: &Path,
) {
    println!("{}", BANNER.cyan());
    let engine = load_engine_or_exit(kb_path);

    let (endpoint, model) = match (custom_endpoint, custom_model) {
        (Some(ep), Some(m)) => (ep.to_string(), m.to_string()),
        (Some(ep), None) => (ep.to_string(), "ornith-coder:latest".to_string()),
        (None, Some(m)) => {
            let ep = WidyaCoderEngine::probe_local_llm()
                .map(|(e, _)| e)
                .unwrap_or_else(|| "http://localhost:11434".to_string());
            (ep, m.to_string())
        }
        (None, None) => match WidyaCoderEngine::probe_local_llm() {
            Some((ep, m)) => (ep, m),
            None => ("http://localhost:11434".to_string(), "ornith-coder:latest".to_string()),
        },
    };

    println!("📡 Server LLM : {}", endpoint.cyan());
    println!("🧠 Model      : {}", model.yellow().bold());
    println!("📚 Basis Data : {} potongan terindeks", engine.kb.total_chunks.to_string().green());
    println!("\nKetik pertanyaan Anda (atau ':keluar' / ':exit' untuk selesai, ':sumber' untuk alihkan info rujukan).");
    println!("--------------------------------------------------------------------------------");

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Gagal membuat konsol interaktif: {}", e);
            return;
        }
    };

    let mut show_sources = true;

    loop {
        let readline = rl.readline("WidyaCoder> ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(trimmed);

                if trimmed == ":keluar" || trimmed == ":exit" || trimmed == "keluar" || trimmed == "exit" {
                    println!("{}", "Sampai jumpa! Selamat berkarya dengan Widya-Lang.".green());
                    break;
                }

                if trimmed == ":sumber" {
                    show_sources = !show_sources;
                    println!("Status tampilan rujukan: {}", if show_sources { "AKTIF".green() } else { "NONAKTIF".yellow() });
                    continue;
                }

                if trimmed == ":info" || trimmed == ":status" {
                    println!("Total Chunks : {}", engine.kb.total_chunks);
                    println!("Endpoint     : {}", endpoint);
                    println!("Model        : {}", model);
                    continue;
                }

                // Cari rujukan RAG
                let matches = engine.search(trimmed, top_k);
                if show_sources && !matches.is_empty() {
                    println!("{}", "  [Rujukan Konteks]:".dimmed());
                    for m in &matches {
                        println!("    • {} ({})", m.chunk.source_path.dimmed(), m.chunk.title.dimmed());
                    }
                }

                let (system_prompt, user_prompt) = engine.format_prompt(trimmed, &matches);
                print!("{}", "🤖 Sedang berpikir...".dimmed());
                io::stdout().flush().unwrap();

                let start = Instant::now();
                match WidyaCoderEngine::query_llm(&endpoint, &model, &system_prompt, &user_prompt) {
                    Ok(resp) => {
                        print!("\r                      \r"); // Hapus indikator berpikir
                        println!("{}\n", resp.trim());
                        println!("{}", format!("(Selesai dalam {:.2?})", start.elapsed()).dimmed());
                    }
                    Err(e) => {
                        print!("\r                      \r");
                        eprintln!("{} {}", "⚠️  Koneksi LLM error:".red(), e);
                    }
                }
            }
            Err(_) => break,
        }
    }
}

// ============================================================================
// PERINTAH: STATUS
// ============================================================================
fn cmd_status(kb_path: &Path) {
    println!("{}", BANNER.cyan());
    println!("{}", "📊 STATUS SISTEM WIDYACODER LOCAL RAG".bold().white());
    println!("------------------------------------------------------------");

    // 1. Status Basis Pengetahuan
    if kb_path.exists() {
        match KnowledgeBase::load_from_file(kb_path) {
            Ok(kb) => {
                let meta = fs::metadata(kb_path).ok();
                let size_mb = meta.map(|m| m.len() as f64 / (1024.0 * 1024.0)).unwrap_or(0.0);
                println!("📂 Basis Pengetahuan (KB) : {}", "DITEMUKAN".bold().green());
                println!("   • Berkas               : {}", kb_path.display());
                println!("   • Ukuran Berkas        : {:.2} MB", size_mb);
                println!("   • Total Chunks         : {}", kb.total_chunks.to_string().bold().yellow());
                println!("   • Terakhir Dibuat      : {}", kb.created_at);
            }
            Err(e) => {
                println!("📂 Basis Pengetahuan (KB) : {} ({})", "RUSAK / TIDAK VALID".bold().red(), e);
            }
        }
    } else {
        println!("📂 Basis Pengetahuan (KB) : {}", "BELUM DIBANGUN".bold().red());
        println!("   Jalankan: {}", "widyacoder indeks".cyan());
    }

    println!("\n🔌 Status Server LLM Lokal:");
    match WidyaCoderEngine::probe_local_llm() {
        Some((ep, model)) => {
            println!("   • Status Server        : {}", "TERHUBUNG (ONLINE)".bold().green());
            println!("   • Endpoint Aktif       : {}", ep.cyan());
            println!("   • Model Direkomendasikan: {}", model.bold().yellow());
        }
        None => {
            println!("   • Status Server        : {}", "TIDAK TERDETEKSI (OFFLINE)".bold().yellow());
            println!("   • Catatan              : Jalankan Ollama di port 11434 atau llama.cpp di port 8080");
        }
    }

    println!("\n🖥️  Kapasitas Mesin:");
    println!("   • Arsitektur           : {}", std::env::consts::ARCH.cyan());
    println!("   • Sistem Operasi       : {}", std::env::consts::OS.cyan());
    println!("------------------------------------------------------------");
}

// ============================================================================
// PERINTAH: SERVE (HTTP REST API SERVER)
// ============================================================================
fn cmd_serve(
    port: u16,
    custom_model: Option<&str>,
    custom_endpoint: Option<&str>,
    kb_path: &Path,
) {
    println!("{}", BANNER.cyan());
    let engine = std::sync::Arc::new(load_engine_or_exit(kb_path));

    let (endpoint, model) = match (custom_endpoint, custom_model) {
        (Some(ep), Some(m)) => (ep.to_string(), m.to_string()),
        (Some(ep), None) => (ep.to_string(), "ornith-coder:latest".to_string()),
        (None, Some(m)) => {
            let ep = WidyaCoderEngine::probe_local_llm()
                .map(|(e, _)| e)
                .unwrap_or_else(|| "http://localhost:11434".to_string());
            (ep, m.to_string())
        }
        (None, None) => match WidyaCoderEngine::probe_local_llm() {
            Some((ep, m)) => (ep, m),
            None => ("http://localhost:11434".to_string(), "ornith-coder:latest".to_string()),
        },
    };

    let bind_addr = format!("0.0.0.0:{}", port);
    let listener = match TcpListener::bind(&bind_addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Gagal mengikat port {}: {}", port, e);
            std::process::exit(1);
        }
    };

    println!("{}", format!("🚀 WidyaCoder HTTP RAG Server mendengarkan di http://{}", bind_addr).bold().green());
    println!("  • Rute Tersedia: POST /api/chat, POST /api/search, GET /api/status, GET /health");
    println!("  • Model LLM    : {} ({})", model.yellow(), endpoint.cyan());

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let engine = engine.clone();
            let endpoint = endpoint.clone();
            let model = model.clone();

            std::thread::spawn(move || {
                handle_http_client(&mut stream, engine, &endpoint, &model);
            });
        }
    }
}

fn handle_http_client(
    stream: &mut TcpStream,
    engine: std::sync::Arc<WidyaCoderEngine>,
    endpoint: &str,
    model: &str,
) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut req_line = String::new();
    if reader.read_line(&mut req_line).is_err() || req_line.is_empty() {
        return;
    }

    let parts: Vec<&str> = req_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    let method = parts[0];
    let path = parts[1];

    let mut content_length = 0;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header).is_err() || header.trim().is_empty() {
            break;
        }
        if header.to_lowercase().starts_with("content-length:") {
            if let Some(val) = header.split(':').nth(1) {
                content_length = val.trim().parse::<usize>().unwrap_or(0);
            }
        }
    }

    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        let _ = io::Read::read_exact(&mut reader, &mut body);
    }
    let body_str = String::from_utf8_lossy(&body);

    let (status, resp_json) = match (method, path) {
        ("GET", "/health") | ("GET", "/api/status") => {
            let resp = json!({
                "status": "ok",
                "version": "2.6.0",
                "total_chunks": engine.kb.total_chunks,
                "model": model,
                "endpoint": endpoint
            });
            ("200 OK", resp)
        }
        ("POST", "/api/search") => {
            let json_body: serde_json::Value = serde_json::from_str(&body_str).unwrap_or(json!({}));
            let query = json_body.get("query").and_then(|q| q.as_str()).unwrap_or("");
            let top_k = json_body.get("top_k").and_then(|k| k.as_u64()).unwrap_or(4) as usize;

            let matches = engine.search(query, top_k);
            let results: Vec<serde_json::Value> = matches
                .iter()
                .map(|m| {
                    json!({
                        "source": m.chunk.source_path,
                        "category": m.chunk.category,
                        "title": m.chunk.title,
                        "content": m.chunk.content,
                        "score": m.rrf_score
                    })
                })
                .collect();

            let resp = json!({ "query": query, "results": results });
            ("200 OK", resp)
        }
        ("POST", "/api/chat") => {
            let json_body: serde_json::Value = serde_json::from_str(&body_str).unwrap_or(json!({}));
            let query = json_body.get("query").and_then(|q| q.as_str()).unwrap_or("");
            let top_k = json_body.get("top_k").and_then(|k| k.as_u64()).unwrap_or(4) as usize;

            let matches = engine.search(query, top_k);
            let (system_prompt, user_prompt) = engine.format_prompt(query, &matches);

            let answer = WidyaCoderEngine::query_llm(endpoint, model, &system_prompt, &user_prompt)
                .unwrap_or_else(|e| format!("Error inferensi LLM: {}", e));

            let references: Vec<String> = matches.iter().map(|m| m.chunk.source_path.clone()).collect();

            let resp = json!({
                "answer": answer,
                "references": references,
                "model_used": model
            });
            ("200 OK", resp)
        }
        _ => ("404 NOT FOUND", json!({ "error": "Rute tidak ditemukan" })),
    };

    let body_bytes = serde_json::to_vec_pretty(&resp_json).unwrap_or_default();
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n",
        status,
        body_bytes.len()
    );

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(&body_bytes);
    let _ = stream.flush();
}

fn load_engine_or_exit(kb_path: &Path) -> WidyaCoderEngine {
    if !kb_path.exists() {
        println!("{}", "ℹ️  Basis data belum ditemukan. Membangun indeks awal secara otomatis...".yellow());
        let mut engine = WidyaCoderEngine::new();
        if let Err(e) = engine.build_from_workspace(Path::new(".")) {
            eprintln!("Gagal membangun basis data: {}", e);
            std::process::exit(1);
        }
        let _ = engine.kb.save_to_file(kb_path);
        println!("{} Indeks disimpan ke {}\n", "✅".green(), kb_path.display());
        return engine;
    }

    match KnowledgeBase::load_from_file(kb_path) {
        Ok(kb) => WidyaCoderEngine::from_kb(kb),
        Err(e) => {
            eprintln!("Gagal memuat {}: {}", kb_path.display(), e);
            std::process::exit(1);
        }
    }
}
