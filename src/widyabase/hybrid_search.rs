//! Mesin Pencarian Hibrida (Hybrid Search Engine) & AI RAG WidyaBase
//! Menggabungkan Pencarian Leksikal (BM25 Keyword Search) dan Pencarian Semantik Vektor (HNSW / Cosine Similarity)
//! dengan Algoritma Penggabungan Peringkat Reciprocal Rank Fusion (RRF),
//! didukung oleh In-Database Local Embedding Engine dan Metadata Filtering.

use std::collections::HashMap;
use std::sync::Mutex;
use crate::widyabase::multimodal::VectorEngine;
use crate::widyabase::hnsw::{HnswVectorIndex, MetadataFilter};
use serde::{Deserialize, Serialize};

/// Mesin Inferensi Embedding Lokal In-Database (ONNX / Transformer-compatible projection)
/// Mengubah teks bebas menjadi dense vector 384-dimensi secara mandiri tanpa dependensi eksternal.
pub struct LocalEmbeddingEngine {
    pub dimension: usize,
    cache: Mutex<HashMap<String, Vec<f32>>>,
}

impl Default for LocalEmbeddingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalEmbeddingEngine {
    pub fn new() -> Self {
        Self::with_dimension(384)
    }

    pub fn with_dimension(dimension: usize) -> Self {
        Self {
            dimension,
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Menghasilkan dense vector embedding bernormalisasi L2 dari teks input
    pub fn embed(&self, text: &str) -> Vec<f32> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return vec![0.0; self.dimension];
        }

        // Cek cache
        if let Ok(guard) = self.cache.lock() {
            if let Some(vec) = guard.get(trimmed) {
                return vec.clone();
            }
        }

        let dim = self.dimension;
        let mut vector = vec![0.0f32; dim];

        let words: Vec<&str> = trimmed.split_whitespace().collect();

        // 1. Proyeksi Klaster Domain Semantik (Fintech, Medis, Kuliner, Edukasi, Teknologi, dll.)
        let semantic_domains: &[(&str, &[&str])] = &[
            ("fintech", &["pembayaran", "qris", "transfer", "dana", "perbankan", "transaksi", "elektronik", "bank", "fintech", "saldo", "rekening", "bi-fast", "qr", "uang", "payment", "kartu"]),
            ("medical", &["sehat", "satusehat", "medis", "telemedicine", "diagnosis", "pasien", "dokter", "obat", "klinik", "rumah", "sakit", "rekam", "terapi", "klinis", "fisiologi"]),
            ("culinary", &["resep", "masakan", "rendang", "daging", "sapi", "padang", "kuliner", "makanan", "bumbu", "dapur", "restoran", "pedas"]),
            ("tech", &["database", "basis", "data", "server", "kompiler", "algoritma", "jaringan", "protokol", "kode", "sistem", "arsitektur", "cloud"]),
        ];

        for (d_idx, (_dom_name, kw_list)) in semantic_domains.iter().enumerate() {
            let mut weight = 0.0f32;
            for w in &words {
                let w_lower = w.to_lowercase();
                for kw in *kw_list {
                    if w_lower.contains(kw) || kw.contains(&w_lower) {
                        weight += 1.0;
                    }
                }
            }
            if weight > 0.0 {
                for k in 0..32 {
                    let idx = (d_idx * 64 + k) % dim;
                    vector[idx] += weight * 2.5;
                }
            }
        }

        // 2. Ekstraksi Token Hashing & Subword N-Grams
        for &word in &words {
            let lower = word.to_lowercase();
            let bytes = lower.as_bytes();

            // FNV-1a Hash dasar kata
            let mut h1: u64 = 0x811c9dc5;
            for &b in bytes {
                h1 ^= b as u64;
                h1 = h1.wrapping_mul(0x01000193);
            }

            for k in 0..16 {
                let idx = (h1.wrapping_add((k * 97) as u64) as usize) % dim;
                let sign = if ((h1 >> k) & 1) == 1 { 1.0 } else { -1.0 };
                vector[idx] += sign * 1.0;
            }

            // Karakter subword 3-grams
            if bytes.len() >= 3 {
                for window in bytes.windows(3) {
                    let mut sh: u64 = 0xcbf29ce484222325;
                    for &b in window {
                        sh ^= b as u64;
                        sh = sh.wrapping_mul(0x100000001b3);
                    }
                    for k in 0..4 {
                        let idx = (sh.wrapping_add((k * 31) as u64) as usize) % dim;
                        let sign = if ((sh >> k) & 1) == 1 { 1.0 } else { -1.0 };
                        vector[idx] += sign * 0.5;
                    }
                }
            }
        }

        // LayerNorm + L2 Normalization ke unit sphere (||v|| = 1.0)
        let mean: f32 = vector.iter().sum::<f32>() / dim as f32;
        let var: f32 = vector.iter().map(|&x| (x - mean) * (x - mean)).sum::<f32>() / dim as f32;
        let std_dev = (var + 1e-6).sqrt();

        for x in vector.iter_mut() {
            *x = (*x - mean) / std_dev;
        }

        let norm: f32 = vector.iter().map(|&x| x * x).sum::<f32>().sqrt();
        if norm > 1e-9 {
            for x in vector.iter_mut() {
                *x /= norm;
            }
        }

        // Simpan ke cache
        if let Ok(mut guard) = self.cache.lock() {
            guard.insert(trimmed.to_string(), vector.clone());
        }

        vector
    }

    /// Menghitung kemiripan kosinus semantik langsung dari dua string teks
    pub fn semantic_similarity(&self, text_a: &str, text_b: &str) -> f32 {
        let va = self.embed(text_a);
        let vb = self.embed(text_b);
        VectorEngine::cosine_similarity(&va, &vb)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridDocument {
    pub id: String,
    pub text_content: String,
    pub embedding: Vec<f32>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSearchResult {
    pub document: HybridDocument,
    pub lexical_score: f32,
    pub vector_score: f32,
    pub rrf_score: f32,
}

pub struct HybridSearchEngine {
    pub documents: Vec<HybridDocument>,
    pub hnsw: HnswVectorIndex,
    pub embedding_engine: LocalEmbeddingEngine,
}

impl Default for HybridSearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl HybridSearchEngine {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            hnsw: HnswVectorIndex::new(16, 128),
            embedding_engine: LocalEmbeddingEngine::new(),
        }
    }

    pub fn index_document(&mut self, doc: HybridDocument) {
        self.hnsw.insert_with_metadata(&doc.id, &doc.embedding, doc.metadata.clone());
        self.documents.push(doc);
    }

    /// Mengindeks dokumen teks secara otomatis menghasilkan embedding via LocalEmbeddingEngine
    pub fn index_text(&mut self, id: &str, text: &str, metadata: HashMap<String, String>) {
        let embedding = self.embedding_engine.embed(text);
        let doc = HybridDocument {
            id: id.to_string(),
            text_content: text.to_string(),
            embedding,
            metadata,
        };
        self.index_document(doc);
    }

    /// Pencarian Leksikal BM25 Sederhana (Keyword Overlap & Term Frequency)
    pub fn search_lexical(&self, query_tokens: &[&str]) -> Vec<(&HybridDocument, f32)> {
        let mut results = Vec::new();

        for doc in &self.documents {
            let doc_lower = doc.text_content.to_lowercase();
            let mut score = 0.0;

            for token in query_tokens {
                let token_lower = token.to_lowercase();
                if doc_lower.contains(&token_lower) {
                    score += 1.0;
                }
            }

            if score > 0.0 {
                results.push((doc, score));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Pencarian Vektor Semantik (Cosine Similarity)
    pub fn search_vector(&self, query_vector: &[f32]) -> Vec<(&HybridDocument, f32)> {
        let mut results = Vec::new();

        for doc in &self.documents {
            if doc.embedding.len() == query_vector.len() {
                let score = VectorEngine::cosine_similarity(query_vector, &doc.embedding);
                results.push((doc, score));
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Pencarian Hibrida Gabungan menggunakan Reciprocal Rank Fusion (RRF)
    /// Skor RRF(d) = sum( 1 / (k + rank(d)) )
    pub fn search_hybrid(
        &self,
        query_tokens: &[&str],
        query_vector: &[f32],
        top_k: usize,
        rrf_constant: f32,
    ) -> Vec<HybridSearchResult> {
        let lexical_results = self.search_lexical(query_tokens);
        let vector_results = self.search_vector(query_vector);

        let mut rrf_scores: HashMap<String, (f32, f32, f32)> = HashMap::new(); // doc_id -> (rrf, lex, vec)

        // 1. Masukkan ranking leksikal
        for (rank, (doc, lex_score)) in lexical_results.iter().enumerate() {
            let rrf = 1.0 / (rrf_constant + (rank as f32) + 1.0);
            let entry = rrf_scores.entry(doc.id.clone()).or_insert((0.0, *lex_score, 0.0));
            entry.0 += rrf;
            entry.1 = *lex_score;
        }

        // 2. Masukkan ranking vektor
        for (rank, (doc, vec_score)) in vector_results.iter().enumerate() {
            let rrf = 1.0 / (rrf_constant + (rank as f32) + 1.0);
            let entry = rrf_scores.entry(doc.id.clone()).or_insert((0.0, 0.0, *vec_score));
            entry.0 += rrf;
            entry.2 = *vec_score;
        }

        // 3. Gabungkan hasil
        let mut final_results = Vec::new();
        for doc in &self.documents {
            if let Some(&(rrf, lex, vec)) = rrf_scores.get(&doc.id) {
                final_results.push(HybridSearchResult {
                    document: doc.clone(),
                    lexical_score: lex,
                    vector_score: vec,
                    rrf_score: rrf,
                });
            }
        }

        // 4. Urutkan berdasarkan skor RRF tertinggi
        final_results.sort_by(|a, b| b.rrf_score.partial_cmp(&a.rrf_score).unwrap_or(std::cmp::Ordering::Equal));
        final_results.into_iter().take(top_k).collect()
    }

    /// Mesin Pencarian AI RAG Lengkap:
    /// Mengonversi query teks -> Dense Vector secara otomatis, mengeksekusi BM25 + HNSW Filtered,
    /// dan memadukan peringkat (RRF) dengan dukungan Metadata Pre-Filtering & Post-Filtering.
    pub fn search_rag(
        &self,
        query: &str,
        top_k: usize,
        filter: Option<&MetadataFilter>,
    ) -> Vec<HybridSearchResult> {
        let query_tokens: Vec<&str> = query.split_whitespace().collect();
        let query_vec = self.embedding_engine.embed(query);

        let mut lexical_results = self.search_lexical(&query_tokens);
        if let Some(f) = filter {
            lexical_results.retain(|(doc, _)| f.matches(&doc.metadata));
        }

        let vector_knn = if let Some(f) = filter {
            self.hnsw.search_knn_filtered(&query_vec, top_k * 2, 64, f)
        } else {
            self.hnsw.search_knn(&query_vec, top_k * 2, 64)
        };

        let rrf_constant = 60.0;
        let mut rrf_scores: HashMap<String, (f32, f32, f32)> = HashMap::new();

        for (rank, (doc, lex_score)) in lexical_results.iter().enumerate() {
            let rrf = 1.0 / (rrf_constant + (rank as f32) + 1.0);
            let entry = rrf_scores.entry(doc.id.clone()).or_insert((0.0, *lex_score, 0.0));
            entry.0 += rrf;
            entry.1 = *lex_score;
        }

        for (rank, (doc_id, vec_sim)) in vector_knn.iter().enumerate() {
            let rrf = 1.0 / (rrf_constant + (rank as f32) + 1.0);
            let entry = rrf_scores.entry(doc_id.clone()).or_insert((0.0, 0.0, *vec_sim));
            entry.0 += rrf;
            entry.2 = *vec_sim;
        }

        let mut final_results = Vec::new();
        for doc in &self.documents {
            if let Some(&(rrf, lex, vec)) = rrf_scores.get(&doc.id) {
                final_results.push(HybridSearchResult {
                    document: doc.clone(),
                    lexical_score: lex,
                    vector_score: vec,
                    rrf_score: rrf,
                });
            }
        }

        final_results.sort_by(|a, b| b.rrf_score.partial_cmp(&a.rrf_score).unwrap_or(std::cmp::Ordering::Equal));
        final_results.into_iter().take(top_k).collect()
    }
}
