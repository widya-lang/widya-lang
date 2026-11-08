// ============================================================================
// WIDYACODER: LOCAL RAG & AI PROGRAMMING ASSISTANT (src/ai/widyacoder.rs)
// ============================================================================
// Asisten Cerdas AI Berdaulat 100% Lokal untuk Ekosistem Widya-Lang:
// - Ekstraksi & Chunking Otomatis Dokumentasi Markdown (.md) dan Kode Contoh (.wya)
// - Indeks Vektor In-Database Berbasis WidyaBase HNSW & BM25 Lexical Matching
// - Penggabungan Peringkat Cerdas Reciprocal Rank Fusion (RRF)
// - Penyuntikan Konteks RAG (Retrieval-Augmented Generation) Tanpa Halusinasi
// - Integrasi Terpadu LLM Lokal (Ollama / llama.cpp / GGUF Server) Tanpa Cloud
// ============================================================================

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use serde::{Deserialize, Serialize};

use crate::widyabase::hybrid_search::{HybridDocument, HybridSearchEngine, LocalEmbeddingEngine};

/// Potongan Pengetahuan (Knowledge Chunk) untuk Basis Pengetahuan RAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeChunk {
    pub id: String,
    pub source_path: String,
    pub category: String, // "dokumentasi" | "contoh_kode" | "arsitektur"
    pub title: String,
    pub content: String,
    pub token_count: usize,
    pub embedding: Vec<f32>,
}

/// Basis Pengetahuan Serializabel (Cache JSON Berkas Disk)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub version: String,
    pub created_at: String,
    pub total_chunks: usize,
    pub chunks: Vec<KnowledgeChunk>,
}

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self {
            version: "2.6.0".to_string(),
            created_at: chrono::Local::now().to_rfc3339(),
            total_chunks: 0,
            chunks: Vec::new(),
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let kb: Self = serde_json::from_reader(reader)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        Ok(kb)
    }
}

/// Hasil Pencarian Relevansi RAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeMatch {
    pub chunk: KnowledgeChunk,
    pub lexical_score: f32,
    pub vector_score: f32,
    pub rrf_score: f32,
}

/// Mesin Utama WidyaCoder Local RAG
pub struct WidyaCoderEngine {
    pub kb: KnowledgeBase,
    pub search_engine: HybridSearchEngine,
    pub embedding_engine: LocalEmbeddingEngine,
}

impl Default for WidyaCoderEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl WidyaCoderEngine {
    pub fn new() -> Self {
        Self {
            kb: KnowledgeBase::new(),
            search_engine: HybridSearchEngine::new(),
            embedding_engine: LocalEmbeddingEngine::new(),
        }
    }

    /// Bangun mesin dari basis pengetahuan yang sudah ada
    pub fn from_kb(kb: KnowledgeBase) -> Self {
        let mut engine = HybridSearchEngine::new();
        let embedding_engine = LocalEmbeddingEngine::new();

        for chunk in &kb.chunks {
            let mut metadata = HashMap::new();
            metadata.insert("source".to_string(), chunk.source_path.clone());
            metadata.insert("category".to_string(), chunk.category.clone());
            metadata.insert("title".to_string(), chunk.title.clone());

            let doc = HybridDocument {
                id: chunk.id.clone(),
                text_content: chunk.content.clone(),
                embedding: chunk.embedding.clone(),
                metadata,
            };
            engine.index_document(doc);
        }

        Self {
            kb,
            search_engine: engine,
            embedding_engine,
        }
    }

    /// Muat berkas basis data dari path, atau bangun ulang jika belum ada
    pub fn load_or_build<P: AsRef<Path>>(kb_path: P, workspace_root: &Path) -> Result<Self, io::Error> {
        let path = kb_path.as_ref();
        if path.exists() {
            let kb = KnowledgeBase::load_from_file(path)?;
            Ok(Self::from_kb(kb))
        } else {
            let mut engine = Self::new();
            engine.build_from_workspace(workspace_root)?;
            let _ = engine.kb.save_to_file(path);
            Ok(engine)
        }
    }

    /// Pindai seluruh dokumentasi Markdown dan berkas contoh Widya-Lang di ruang kerja
    pub fn build_from_workspace(&mut self, workspace_root: &Path) -> Result<(), io::Error> {
        let mut chunks = Vec::new();
        let mut chunk_id = 0;

        // 1. Pindai Dokumentasi Markdown (.md)
        if let Ok(entries) = fs::read_dir(workspace_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if (file_name.starts_with("DOKUMENTASI_") || file_name.starts_with("ROADMAP_") || file_name == "README.md")
                        && file_name.ends_with(".md")
                    {
                        if let Ok(content) = fs::read_to_string(&path) {
                            let doc_chunks = Self::chunk_markdown(&content, file_name, &mut chunk_id, &self.embedding_engine);
                            chunks.extend(doc_chunks);
                        }
                    }
                }
            }
        }

        // 2. Pindai Berkas Contoh Kode Widya (.wya)
        let contoh_dir = workspace_root.join("contoh");
        if contoh_dir.exists() && contoh_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&contoh_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("wya") {
                        let rel_name = format!("contoh/{}", path.file_name().and_then(|n| n.to_str()).unwrap_or(""));
                        if let Ok(content) = fs::read_to_string(&path) {
                            let code_chunks = Self::chunk_wya_code(&content, &rel_name, &mut chunk_id, &self.embedding_engine);
                            chunks.extend(code_chunks);
                        }
                    }
                }
            }
        }

        self.kb.total_chunks = chunks.len();
        self.kb.chunks = chunks;

        // Masukkan ke HybridSearchEngine
        self.search_engine = HybridSearchEngine::new();
        for chunk in &self.kb.chunks {
            let mut metadata = HashMap::new();
            metadata.insert("source".to_string(), chunk.source_path.clone());
            metadata.insert("category".to_string(), chunk.category.clone());
            metadata.insert("title".to_string(), chunk.title.clone());

            let doc = HybridDocument {
                id: chunk.id.clone(),
                text_content: chunk.content.clone(),
                embedding: chunk.embedding.clone(),
                metadata,
            };
            self.search_engine.index_document(doc);
        }

        Ok(())
    }

    /// Algoritma pemecah teks Markdown dengan pelestarian blok kode
    pub fn chunk_markdown(
        content: &str,
        file_name: &str,
        id_counter: &mut usize,
        embedder: &LocalEmbeddingEngine,
    ) -> Vec<KnowledgeChunk> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut current_title = file_name.to_string();
        let mut current_buf = Vec::new();
        let mut in_code_block = false;

        let category = if file_name.starts_with("ROADMAP_") {
            "roadmap".to_string()
        } else {
            "dokumentasi".to_string()
        };

        for line in lines {
            let trimmed = line.trim();

            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                current_buf.push(line);
                continue;
            }

            // Jika menemukan heading tingkat 1, 2, atau 3 di luar blok kode
            if !in_code_block && (trimmed.starts_with("# ") || trimmed.starts_with("## ") || trimmed.starts_with("### ")) {
                if !current_buf.is_empty() {
                    let text = current_buf.join("\n").trim().to_string();
                    if text.len() >= 40 {
                        *id_counter += 1;
                        let token_count = text.split_whitespace().count();
                        let embedding = embedder.embed(&text);
                        chunks.push(KnowledgeChunk {
                            id: format!("chunk_{:05}", id_counter),
                            source_path: file_name.to_string(),
                            category: category.clone(),
                            title: current_title.clone(),
                            content: text,
                            token_count,
                            embedding,
                        });
                    }
                    current_buf.clear();
                }

                current_title = trimmed
                    .trim_start_matches('#')
                    .trim()
                    .to_string();
                current_buf.push(line);
                continue;
            }

            current_buf.push(line);

            // Jika paragraf sudah cukup panjang (>250 kata) dan diakhiri baris kosong
            if !in_code_block && trimmed.is_empty() && current_buf.len() > 25 {
                let text = current_buf.join("\n").trim().to_string();
                let word_count = text.split_whitespace().count();
                if word_count >= 180 {
                    *id_counter += 1;
                    let embedding = embedder.embed(&text);
                    chunks.push(KnowledgeChunk {
                        id: format!("chunk_{:05}", id_counter),
                        source_path: file_name.to_string(),
                        category: category.clone(),
                        title: current_title.clone(),
                        content: text,
                        token_count: word_count,
                        embedding,
                    });
                    current_buf.clear();
                }
            }
        }

        if !current_buf.is_empty() {
            let text = current_buf.join("\n").trim().to_string();
            if text.len() >= 40 {
                *id_counter += 1;
                let token_count = text.split_whitespace().count();
                let embedding = embedder.embed(&text);
                chunks.push(KnowledgeChunk {
                    id: format!("chunk_{:05}", id_counter),
                    source_path: file_name.to_string(),
                    category,
                    title: current_title,
                    content: text,
                    token_count,
                    embedding,
                });
            }
        }

        chunks
    }

    /// Pemecah berkas contoh kode Widya-Lang (.wya)
    pub fn chunk_wya_code(
        content: &str,
        source_path: &str,
        id_counter: &mut usize,
        embedder: &LocalEmbeddingEngine,
    ) -> Vec<KnowledgeChunk> {
        let mut chunks = Vec::new();
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return chunks;
        }

        // Ekstraksi judul dari komentar awal jika ada
        let mut title = source_path.to_string();
        for line in trimmed.lines().take(6) {
            let l = line.trim();
            if l.starts_with("//") {
                let clean = l.trim_start_matches('/').trim();
                if !clean.starts_with('=') && !clean.is_empty() {
                    title = clean.to_string();
                    break;
                }
            }
        }

        let lines: Vec<&str> = trimmed.lines().collect();

        // Jika berkas ringkas (< 120 baris), jadikan satu chunk utuh
        if lines.len() <= 120 {
            *id_counter += 1;
            let token_count = trimmed.split_whitespace().count();
            let embedding = embedder.embed(trimmed);
            chunks.push(KnowledgeChunk {
                id: format!("chunk_{:05}", id_counter),
                source_path: source_path.to_string(),
                category: "contoh_kode".to_string(),
                title,
                content: trimmed.to_string(),
                token_count,
                embedding,
            });
            return chunks;
        }

        // Untuk berkas panjang, pecah berdasarkan blok seksi komentar besar (// --- Seksi ---)
        let mut current_buf = Vec::new();
        let mut current_section = title.clone();

        for line in lines {
            let l = line.trim();
            if (l.starts_with("// ===") || l.starts_with("// ---") || (l.starts_with("// ") && l.chars().nth(3).map_or(false, |c| c.is_ascii_digit())))
                && current_buf.len() > 30
            {
                let text = current_buf.join("\n").trim().to_string();
                if text.len() >= 50 {
                    *id_counter += 1;
                    let token_count = text.split_whitespace().count();
                    let embedding = embedder.embed(&text);
                    chunks.push(KnowledgeChunk {
                        id: format!("chunk_{:05}", id_counter),
                        source_path: source_path.to_string(),
                        category: "contoh_kode".to_string(),
                        title: current_section.clone(),
                        content: text,
                        token_count,
                        embedding,
                    });
                }
                current_buf.clear();
                current_section = format!("{} ({})", title, l.trim_start_matches('/').trim());
            }

            current_buf.push(line);
        }

        if !current_buf.is_empty() {
            let text = current_buf.join("\n").trim().to_string();
            if text.len() >= 50 {
                *id_counter += 1;
                let token_count = text.split_whitespace().count();
                let embedding = embedder.embed(&text);
                chunks.push(KnowledgeChunk {
                    id: format!("chunk_{:05}", id_counter),
                    source_path: source_path.to_string(),
                    category: "contoh_kode".to_string(),
                    title: current_section,
                    content: text,
                    token_count,
                    embedding,
                });
            }
        }

        chunks
    }

    /// Jalankan pencarian hibrida RAG (Weighted Lexical + HNSW Semantic + RRF)
    pub fn search(&self, query: &str, top_k: usize) -> Vec<KnowledgeMatch> {
        let stopwords: &[&str] = &[
            "di", "ke", "dari", "dan", "yang", "ini", "itu", "pada", "untuk", "dengan",
            "adalah", "cara", "bagaimana", "apa", "buat", "bikin", "saya", "mau", "bisa",
            "contoh", "the", "a", "an", "is", "in", "to", "for", "and", "of", "how", "what",
            "kode", "program",
        ];

        let raw_tokens: Vec<String> = query
            .split_whitespace()
            .map(|w| {
                w.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
                    .to_lowercase()
            })
            .filter(|w| w.len() >= 3 && !stopwords.contains(&w.as_str()))
            .collect();

        let query_vec = self.embedding_engine.embed(query);
        let mut lexical_ranks: Vec<(usize, f32)> = Vec::new();
        let mut vector_ranks: Vec<(usize, f32)> = Vec::new();

        for (idx, chunk) in self.kb.chunks.iter().enumerate() {
            let content_lower = chunk.content.to_lowercase();
            let title_lower = chunk.title.to_lowercase();
            let source_lower = chunk.source_path.to_lowercase();

            let mut lex_score = 0.0f32;
            for token in &raw_tokens {
                if title_lower.contains(token) {
                    lex_score += 5.0;
                }
                if source_lower.contains(token) {
                    lex_score += 4.0;
                }
                if content_lower.contains(token) {
                    lex_score += 1.0;
                }
            }

            if lex_score > 0.0 {
                lexical_ranks.push((idx, lex_score));
            }

            // Kemiripan vektor kosinus (embedding sudah ternormalisasi L2)
            if chunk.embedding.len() == query_vec.len() {
                let dot: f32 = query_vec.iter().zip(&chunk.embedding).map(|(a, b)| a * b).sum();
                vector_ranks.push((idx, dot));
            }
        }

        lexical_ranks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        vector_ranks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let rrf_constant = 40.0f32;
        let mut chunk_scores: HashMap<usize, (f32, f32, f32)> = HashMap::new();

        for (rank, &(idx, score)) in lexical_ranks.iter().enumerate() {
            let rrf = 1.0 / (rrf_constant + rank as f32 + 1.0);
            let entry = chunk_scores.entry(idx).or_insert((0.0, score, 0.0));
            entry.0 += rrf * 2.0; // Berikan prioritas tinggi pada kata kunci persis
            entry.1 = score;
        }

        for (rank, &(idx, score)) in vector_ranks.iter().enumerate() {
            let rrf = 1.0 / (rrf_constant + rank as f32 + 1.0);
            let entry = chunk_scores.entry(idx).or_insert((0.0, 0.0, score));
            entry.0 += rrf;
            entry.2 = score;
        }

        let mut ranked: Vec<(usize, f32, f32, f32)> = chunk_scores
            .into_iter()
            .map(|(idx, (rrf, lex, vec))| (idx, rrf, lex, vec))
            .collect();

        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        ranked
            .into_iter()
            .take(top_k)
            .map(|(idx, rrf, lex, vec)| KnowledgeMatch {
                chunk: self.kb.chunks[idx].clone(),
                lexical_score: lex,
                vector_score: vec,
                rrf_score: rrf,
            })
            .collect()
    }

    /// Susun Prompt RAG Lengkap dengan Injeksi Konteks Widya-Lang Ringkas & Akurat
    pub fn format_prompt(&self, query: &str, matches: &[KnowledgeMatch]) -> (String, String) {
        let mut context_builder = String::new();

        for (i, m) in matches.iter().enumerate() {
            // Batasi panjang cuplikan tiap rujukan agar prompt efisien di CPU lokal (maks ~600 karakter per chunk)
            let content_preview = if m.chunk.content.len() > 650 {
                let mut cut = m.chunk.content[..650].to_string();
                if let Some(last_newline) = cut.rfind('\n') {
                    cut.truncate(last_newline);
                }
                format!("{} \n... [dilanjutkan di berkas asli]", cut)
            } else {
                m.chunk.content.clone()
            };

            context_builder.push_str(&format!(
                "--- [RUJUKAN #{}: {} | {} | RRF: {:.4}] ---\n{}\n\n",
                i + 1,
                m.chunk.source_path,
                m.chunk.title,
                m.rrf_score,
                content_preview.trim()
            ));
        }

        let system_prompt = format!(
            "Anda adalah WidyaCoder, asisten AI berdaulat tingkat ahli untuk ekosistem bahasa pemrograman Widya-Lang.\n\
             Tugas Anda: menjawab pertanyaan pemrograman, memberikan contoh kode (.wya), dan menjelaskan pustaka standar Widya-Lang secara akurat.\n\n\
             ATURAN SINTAKS WIDYA-LANG:\n\
             - Deklarasi variabel: 'misal nama = nilai;' atau 'tetap NAMA = nilai;'\n\
             - Fungsi: 'fungsi nama(parameter) {{ kembalikan hasil; }}' atau 'fn nama(p) {{ ... }}'\n\
             - Kontrol alur: 'jika (kondisi) {{ ... }} lainnya {{ ... }}', 'selama (kondisi) {{ ... }}', 'untuk (x dalam daftar) {{ ... }}'\n\
             - Output terminal: 'cetak(\"teks\", variabel);'\n\
             - Modul: widyabase, widyaweb, widyadesktop, widyamobile, kripto, ai, wpm.\n\n\
             === RUJUKAN RESMI WIDYA-LANG (RAG CONTEXT) ===\n\
             {}\n\
             ================================================",
            context_builder.trim()
        );

        let user_prompt = query.to_string();

        (system_prompt, user_prompt)
    }

    /// Deteksi server LLM lokal yang aktif (Ollama atau llama.cpp / vLLM)
    pub fn probe_local_llm() -> Option<(String, String)> {
        // 1. Coba deteksi Ollama di port 11434
        if let Ok(resp) = ureq::get("http://localhost:11434/api/tags").timeout(std::time::Duration::from_millis(600)).call() {
            if let Ok(body) = resp.into_json::<serde_json::Value>() {
                if let Some(models) = body.get("models").and_then(|m| m.as_array()) {
                    let model_names: Vec<String> = models
                        .iter()
                        .filter_map(|m| m.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
                        .collect();

                    // Prioritas model koding terbaik
                    for preferred in &[
                        "ornith-coder:latest",
                        "rafw007/ornith-claude-coder:latest",
                        "qwen2.5-coder:7b",
                        "qwen2.5-coder:latest",
                        "deepseek-coder:latest",
                        "llama3.2:latest",
                        "hermes3:8b",
                    ] {
                        if let Some(found) = model_names.iter().find(|m| m == preferred) {
                            return Some(("http://localhost:11434".to_string(), found.clone()));
                        }
                    }

                    if let Some(first) = model_names.first() {
                        return Some(("http://localhost:11434".to_string(), first.clone()));
                    }
                }
            }
        }

        // 2. Coba deteksi llama.cpp / llama-server di port 8080
        if let Ok(resp) = ureq::get("http://localhost:8080/v1/models").timeout(std::time::Duration::from_millis(600)).call() {
            if let Ok(body) = resp.into_json::<serde_json::Value>() {
                if let Some(data) = body.get("data").and_then(|d| d.as_array()) {
                    if let Some(m) = data.first().and_then(|x| x.get("id").and_then(|i| i.as_str())) {
                        return Some(("http://localhost:8080".to_string(), m.to_string()));
                    }
                }
            }
            return Some(("http://localhost:8080".to_string(), "local-model".to_string()));
        }

        None
    }

    /// Jalankan query ke Ollama atau OpenAI-compatible local server
    pub fn query_llm(
        endpoint: &str,
        model: &str,
        system_prompt: &str,
        user_query: &str,
    ) -> Result<String, String> {
        let ep = endpoint.trim_end_matches('/');

        // Format Ollama Chat API
        if ep.contains(":11434") {
            let url = format!("{}/api/chat", ep);
            let payload = serde_json::json!({
                "model": model,
                "messages": [
                    { "role": "system", "content": system_prompt },
                    { "role": "user", "content": user_query }
                ],
                "stream": false,
                "options": {
                    "temperature": 0.2,
                    "top_p": 0.9,
                    "num_ctx": 4096,
                    "num_predict": 384
                }
            });

            let resp = ureq::post(&url)
                .timeout(std::time::Duration::from_secs(240))
                .send_json(payload)
                .map_err(|e| format!("Gagal memanggil Ollama API: {}", e))?;

            let json: serde_json::Value = resp.into_json()
                .map_err(|e| format!("Gagal membaca respons Ollama: {}", e))?;

            if let Some(content) = json.get("message").and_then(|m| m.get("content")).and_then(|c| c.as_str()) {
                return Ok(content.to_string());
            }

            // Fallback ke /api/generate
            if let Some(response) = json.get("response").and_then(|r| r.as_str()) {
                return Ok(response.to_string());
            }

            return Err(format!("Format respons tidak dikenal dari Ollama: {}", json));
        }

        // Format OpenAI / llama-server API
        let url = format!("{}/v1/chat/completions", ep);
        let payload = serde_json::json!({
            "model": model,
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": user_query }
            ],
            "temperature": 0.2,
            "stream": false,
            "max_tokens": 384
        });

        let resp = ureq::post(&url)
            .timeout(std::time::Duration::from_secs(240))
            .send_json(payload)
            .map_err(|e| format!("Gagal memanggil LLM Server ({}): {}", url, e))?;

        let json: serde_json::Value = resp.into_json()
            .map_err(|e| format!("Gagal membaca respons LLM: {}", e))?;

        if let Some(content) = json.get("choices")
            .and_then(|c| c.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|msg| msg.get("content"))
            .and_then(|c| c.as_str())
        {
            return Ok(content.to_string());
        }

        Err(format!("Format respons tidak dikenal dari server: {}", json))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_markdown_and_search() {
        let md_content = r#"
# Dokumentasi WidyaBase
Ini adalah pendahuluan WidyaBase.

## Bab 1: Slotted Page
Slotted-page adalah mekanisme penyimpanan blok 4096 byte di basis data.

```wya
misal db = widyabase::buka("data.wdb");
db.eksekusi("PILIH * DARI pengguna;");
```

## Bab 2: Indeks HNSW
HNSW digunakan untuk pencarian kemiripan vektor AI tingkat tinggi.
"#;
        let embedder = LocalEmbeddingEngine::new();
        let mut id_counter = 0;
        let chunks = WidyaCoderEngine::chunk_markdown(md_content, "DOKUMENTASI_TEST.md", &mut id_counter, &embedder);

        assert!(!chunks.is_empty());
        assert!(chunks.iter().any(|c| c.title.contains("Slotted Page") || c.content.contains("Slotted-page")));

        let mut kb = KnowledgeBase::new();
        kb.total_chunks = chunks.len();
        kb.chunks = chunks;

        let engine = WidyaCoderEngine::from_kb(kb);
        let matches = engine.search("slotted page basis data", 2);
        assert!(!matches.is_empty());
        assert!(matches[0].chunk.content.to_lowercase().contains("slotted"));
    }

    #[test]
    fn test_chunk_wya_code() {
        let wya_code = r#"
// ==========================================
// Contoh Server Web Sederhana di Widya-Lang
// ==========================================
impor widyaweb;

fungsi jalankan_server() {
    misal app = widyaweb::buat_aplikasi();
    app.rute("/", fungsi(req, res) {
        res.kirim("Halo Dunia!");
    });
    app.dengarkan(8080);
}

jalankan_server();
"#;
        let embedder = LocalEmbeddingEngine::new();
        let mut id_counter = 0;
        let chunks = WidyaCoderEngine::chunk_wya_code(wya_code, "contoh/web_test.wya", &mut id_counter, &embedder);

        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].content.contains("widyaweb::buat_aplikasi"));
        assert_eq!(chunks[0].category, "contoh_kode");
    }

    #[test]
    fn test_prompt_generation() {
        let engine = WidyaCoderEngine::new();
        let embedder = LocalEmbeddingEngine::new();
        let mut id = 0;
        let chunks = WidyaCoderEngine::chunk_markdown(
            "## Enkripsi AES\nModul kripto menyediakan enkripsi AES-256-GCM mandiri.",
            "DOKUMENTASI_KRIPTO.md",
            &mut id,
            &embedder,
        );
        let mut kb = KnowledgeBase::new();
        kb.total_chunks = chunks.len();
        kb.chunks = chunks;
        let engine = WidyaCoderEngine::from_kb(kb);

        let matches = engine.search("enkripsi AES", 1);
        let (sys, user) = engine.format_prompt("bagaimana cara enkripsi AES?", &matches);

        assert!(sys.contains("WidyaCoder"));
        assert!(sys.contains("AES"));
        assert_eq!(user, "bagaimana cara enkripsi AES?");
    }
}
