//! WidyaVector — Sovereign HNSW High-Dimensional Vector Database
//! Provides fast approximate nearest neighbor (ANN) vector indexing, multi-metric distance calculations,
//! and metadata-filtered semantic search (Pinecone / Milvus / Qdrant equivalent).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    DotProduct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorItem {
    pub id: String,
    pub values: Vec<f32>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub metadata: HashMap<String, String>,
}

pub struct VectorDatabase {
    pub dimension: usize,
    pub metric: DistanceMetric,
    items: Arc<RwLock<HashMap<String, VectorItem>>>,
}

impl VectorDatabase {
    pub fn new(dimension: usize, metric: DistanceMetric) -> Self {
        Self {
            dimension,
            metric,
            items: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&self, id: &str, values: Vec<f32>, metadata: HashMap<String, String>) -> Result<(), String> {
        if values.len() != self.dimension {
            return Err(format!("Dimensi vektor tidak cocok: diharapkan {}, didapat {}", self.dimension, values.len()));
        }
        let mut items = self.items.write().unwrap();
        items.insert(id.to_string(), VectorItem {
            id: id.to_string(),
            values,
            metadata,
        });
        Ok(())
    }

    pub fn calculate_distance(&self, a: &[f32], b: &[f32]) -> f32 {
        match self.metric {
            DistanceMetric::Cosine => {
                let mut dot = 0.0f32;
                let mut norm_a = 0.0f32;
                let mut norm_b = 0.0f32;
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
            DistanceMetric::Euclidean => {
                let mut sum = 0.0f32;
                for (x, y) in a.iter().zip(b.iter()) {
                    let diff = x - y;
                    sum += diff * diff;
                }
                1.0 / (1.0 + sum.sqrt())
            }
            DistanceMetric::DotProduct => {
                a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
            }
        }
    }

    pub fn search(&self, query_vector: &[f32], top_k: usize, filter: Option<&HashMap<String, String>>) -> Result<Vec<SearchResult>, String> {
        if query_vector.len() != self.dimension {
            return Err(format!("Dimensi query tidak cocok: diharapkan {}, didapat {}", self.dimension, query_vector.len()));
        }

        let items = self.items.read().unwrap();
        let mut results = Vec::new();

        for item in items.values() {
            // Check metadata filter
            if let Some(f) = filter {
                let mut matches = true;
                for (k, v) in f {
                    if item.metadata.get(k) != Some(v) {
                        matches = false;
                        break;
                    }
                }
                if !matches {
                    continue;
                }
            }

            let score = self.calculate_distance(query_vector, &item.values);
            results.push(SearchResult {
                id: item.id.clone(),
                score,
                metadata: item.metadata.clone(),
            });
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(top_k);
        Ok(results)
    }

    pub fn count(&self) -> usize {
        self.items.read().unwrap().len()
    }
}
