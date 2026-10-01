//! Mesin Pencarian Hibrida (Hybrid Search Engine) WidyaBase
//! Menggabungkan Pencarian Leksikal (BM25 Keyword Search) dan Pencarian Semantik Vektor (HNSW / Cosine Similarity)
//! dengan Algoritma Penggabungan Peringkat Reciprocal Rank Fusion (RRF).

use std::collections::HashMap;
use crate::widyabase::multimodal::VectorEngine;
use serde::{Deserialize, Serialize};

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
}

impl HybridSearchEngine {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
        }
    }

    pub fn index_document(&mut self, doc: HybridDocument) {
        self.documents.push(doc);
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
}
