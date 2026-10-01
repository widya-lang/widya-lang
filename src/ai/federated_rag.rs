//! Federated Vector Search & Decentralized RAG Engine (federated_rag.rs)
//! Menyediakan pencarian semantik terdistribusi di ribuan node instansi/organisasi
//! tanpa memusatkan dokumen sensitif ke satu database terpusat.

use serde::{Deserialize, Serialize};

/// Hasil Pencarian Vektor dari Node Lokal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FederatedSearchResult {
    pub node_id: String,
    pub document_id: String,
    pub title: String,
    pub snippet: String,
    pub similarity_score: f32, // Cosine similarity: 0.0 - 1.0
}

/// Item Dokumen Vektor Lokal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalVectorDoc {
    pub doc_id: String,
    pub title: String,
    pub content: String,
    pub embedding: Vec<f32>,
}

/// Mesin Federated RAG Coordinator
pub struct FederatedRagEngine {
    local_docs: Vec<LocalVectorDoc>,
    min_similarity_threshold: f32,
}

impl FederatedRagEngine {
    pub fn new(min_threshold: f32) -> Self {
        Self {
            local_docs: Vec::new(),
            min_similarity_threshold: min_threshold,
        }
    }

    pub fn index_local_document(&mut self, doc: LocalVectorDoc) {
        self.local_docs.push(doc);
    }

    /// Hitung Cosine Similarity antara dua vektor
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let mut dot = 0.0f32;
        let mut norm_a = 0.0f32;
        let mut norm_b = 0.0f32;

        for (&x, &y) in a.iter().zip(b.iter()) {
            dot += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }

        if norm_a <= 0.0 || norm_b <= 0.0 {
            0.0
        } else {
            dot / (norm_a.sqrt() * norm_b.sqrt())
        }
    }

    /// Eksekusi pencarian vektor lokal
    pub fn search_local(&self, node_id: &str, query_embedding: &[f32], top_k: usize) -> Vec<FederatedSearchResult> {
        let mut scored: Vec<FederatedSearchResult> = self.local_docs
            .iter()
            .map(|doc| {
                let sim = Self::cosine_similarity(&doc.embedding, query_embedding);
                FederatedSearchResult {
                    node_id: node_id.to_string(),
                    document_id: doc.doc_id.clone(),
                    title: doc.title.clone(),
                    snippet: doc.content.chars().take(100).collect(),
                    similarity_score: sim,
                }
            })
            .filter(|r| r.similarity_score >= self.min_similarity_threshold)
            .collect();

        scored.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        scored.truncate(top_k);
        scored
    }

    /// Gabungkan hasil pencarian multi-node (Federated Aggregator)
    pub fn aggregate_multi_node_results(
        node_results: Vec<Vec<FederatedSearchResult>>,
        global_top_k: usize,
    ) -> Vec<FederatedSearchResult> {
        let mut all_results: Vec<FederatedSearchResult> = node_results.into_iter().flatten().collect();
        all_results.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        all_results.truncate(global_top_k);
        all_results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federated_rag_search_and_aggregation() {
        let mut node1 = FederatedRagEngine::new(0.5);
        node1.index_local_document(LocalVectorDoc {
            doc_id: "doc_uu_pdp".to_string(),
            title: "Undang-Undang PDP No 27/2022".to_string(),
            content: "Kewajiban pengendali data pribadi melindungi data spesifik...".to_string(),
            embedding: vec![0.9, 0.1, 0.0],
        });

        let mut node2 = FederatedRagEngine::new(0.5);
        node2.index_local_document(LocalVectorDoc {
            doc_id: "doc_kripto_pqc".to_string(),
            title: "Standar NIST Post-Quantum Kyber".to_string(),
            content: "Kriptografi tahan serangan kuantum menggunakan kisi polinomial...".to_string(),
            embedding: vec![0.1, 0.9, 0.0],
        });

        let query = vec![0.85, 0.15, 0.0]; // Query relevan dengan UU PDP

        let res1 = node1.search_local("node_kemenkominfo", &query, 2);
        let res2 = node2.search_local("node_bssn", &query, 2);

        let global_top = FederatedRagEngine::aggregate_multi_node_results(vec![res1, res2], 1);
        assert_eq!(global_top.len(), 1);
        assert_eq!(global_top[0].document_id, "doc_uu_pdp");
        assert_eq!(global_top[0].node_id, "node_kemenkominfo");
    }
}
