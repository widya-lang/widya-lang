use crate::ai::tensor::Tensor;
use serde::{Deserialize, Serialize};

/// Dokumen dalam Indeks Pengetahuan RAG (Retrieval-Augmented Generation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagDocument {
    pub id: String,
    pub title: String,
    pub content: String,
    pub embedding: Vec<f32>,
}

/// Mesin Pencarian Semantik RAG Mandiri
#[derive(Debug, Clone, Default)]
pub struct RagIndex {
    pub documents: Vec<RagDocument>,
}

impl RagIndex {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
        }
    }

    pub fn add_document(&mut self, id: &str, title: &str, content: &str, embedding: Vec<f32>) {
        self.documents.push(RagDocument {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
            embedding,
        });
    }

    /// Cari Dokumen Paling Relevan berdasarkan Kedekatan Cosine Similarity
    pub fn search_top_k(&self, query_embedding: &[f32], top_k: usize) -> Vec<(&RagDocument, f32)> {
        let query_tensor = Tensor::new(vec![query_embedding.len()], query_embedding.to_vec());
        let mut scores: Vec<(&RagDocument, f32)> = self
            .documents
            .iter()
            .map(|doc| {
                let doc_tensor = Tensor::new(vec![doc.embedding.len()], doc.embedding.clone());
                let sim = query_tensor.cosine_similarity(&doc_tensor);
                (doc, sim)
            })
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(top_k);
        scores
    }
}
