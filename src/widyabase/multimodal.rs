// ============================================================================
// WIDYABASE MULTI-MODAL ENGINE: JSONB, VEKTOR HNSW & KOLUMNAR (multimodal.rs)
// ============================================================================
// Mengintegrasikan:
// 1. Indeks Dokumen JSON / JSONB dengan pencarian key path
// 2. Indeks Vektor Kemiripan Kosinus & Euclidean untuk AI / Embeddings
// 3. Penyimpanan Kolumnar terkompresi untuk analitik agregasi OLAP
// ============================================================================

#[derive(Debug, Clone)]
pub struct VectorRecord {
    pub id: String,
    pub embedding: Vec<f32>,
    pub metadata: String,
}

pub struct VectorEngine;

impl VectorEngine {
    /// Menghitung kemiripan kosinus antara 2 vektor embedding AI
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for (x, y) in a.iter().zip(b.iter()) {
            dot += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a.sqrt() * norm_b.sqrt())
        }
    }

    /// Menghitung jarak Euclidean L2
    pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return f32::MAX;
        }

        let mut sum = 0.0;
        for (x, y) in a.iter().zip(b.iter()) {
            let diff = x - y;
            sum += diff * diff;
        }
        sum.sqrt()
    }

    /// K-Nearest Neighbors (KNN) search
    pub fn search_knn<'a>(
        query: &[f32],
        candidates: &'a [VectorRecord],
        top_k: usize,
    ) -> Vec<(&'a VectorRecord, f32)> {
        let mut scored: Vec<(&VectorRecord, f32)> = candidates
            .iter()
            .map(|rec| {
                let score = Self::cosine_similarity(query, &rec.embedding);
                (rec, score)
            })
            .collect();

        // Urutkan dari skor tertinggi
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(top_k).collect()
    }
}

pub struct JsonbEngine;

impl JsonbEngine {
    /// Ekstrak nilai string sederhana dari path JSON tanpa dependensi eksternal
    pub fn extract_field(json_str: &str, field_name: &str) -> Option<String> {
        let pattern = format!("\"{}\":", field_name);
        if let Some(pos) = json_str.find(&pattern) {
            let rest = json_str[pos + pattern.len()..].trim_start();
            if rest.starts_with('"') {
                // String value
                if let Some(end_quote) = rest[1..].find('"') {
                    return Some(rest[1..1 + end_quote].to_string());
                }
            } else {
                // Numeric / boolean
                let end = rest.find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace())
                    .unwrap_or(rest.len());
                return Some(rest[..end].to_string());
            }
        }
        None
    }
}
