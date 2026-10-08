//! Hierarchical Navigable Small World (HNSW) Vector Index (hnsw.rs)
//! Struktur graf multi-lapis untuk pencarian tetangga terdekat (Approximate Nearest Neighbors / ANN)
//! dengan kompleksitas waktu O(log N) untuk AI Embedding, RAG, dan database semantik WidyaBase.

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

/// Filter Predikat Metadata Dokumen untuk AI Vector Search
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MetadataFilter {
    Exact(String, String),
    Contains(String, String),
    In(String, Vec<String>),
    GreaterThan(String, f64),
    LessThan(String, f64),
    And(Vec<MetadataFilter>),
    Or(Vec<MetadataFilter>),
}

impl MetadataFilter {
    /// Mengevaluasi apakah metadata dokumen memenuhi predikat filter
    pub fn matches(&self, metadata: &HashMap<String, String>) -> bool {
        match self {
            MetadataFilter::Exact(key, val) => {
                metadata.get(key).map(|v| v == val).unwrap_or(false)
            }
            MetadataFilter::Contains(key, sub) => {
                metadata.get(key).map(|v| v.contains(sub)).unwrap_or(false)
            }
            MetadataFilter::In(key, allowed) => {
                metadata.get(key).map(|v| allowed.contains(v)).unwrap_or(false)
            }
            MetadataFilter::GreaterThan(key, threshold) => {
                if let Some(v_str) = metadata.get(key) {
                    if let Ok(num) = v_str.parse::<f64>() {
                        return num > *threshold;
                    }
                }
                false
            }
            MetadataFilter::LessThan(key, threshold) => {
                if let Some(v_str) = metadata.get(key) {
                    if let Ok(num) = v_str.parse::<f64>() {
                        return num < *threshold;
                    }
                }
                false
            }
            MetadataFilter::And(filters) => {
                filters.iter().all(|f| f.matches(metadata))
            }
            MetadataFilter::Or(filters) => {
                filters.iter().any(|f| f.matches(metadata))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct HnswNode {
    pub id: String,
    pub vector: Vec<f32>,
    /// Daftar tetangga pada setiap layer: neighbors[layer] = Vec<node_index>
    pub neighbors: Vec<Vec<usize>>,
    /// Metadata dokumen untuk pre-filtering & post-filtering
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct Candidate {
    distance: f32,
    node_idx: usize,
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Candidate {}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Untuk min-heap (jarak terkecil di atas), balik perbandingannya
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub struct HnswVectorIndex {
    pub nodes: Vec<HnswNode>,
    pub m: usize,                // Maksimum link per layer
    pub m0: usize,               // Maksimum link di ground layer 0
    pub ef_construction: usize,  // Ukuran beam saat konstruksi
    pub entry_point: Option<usize>,
    pub max_level: usize,
    level_mult: f64,
}

impl HnswVectorIndex {
    pub fn new(m: usize, ef_construction: usize) -> Self {
        let m0 = m * 2;
        let level_mult = 1.0 / (m as f64).ln();
        Self {
            nodes: Vec::new(),
            m,
            m0,
            ef_construction,
            entry_point: None,
            max_level: 0,
            level_mult,
        }
    }

    /// Menghitung Cosine Distance: 1.0 - CosineSimilarity (0 = identik, 2 = berlawanan)
    pub fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 2.0;
        }
        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;
        for i in 0..a.len() {
            dot += a[i] * b[i];
            norm_a += a[i] * a[i];
            norm_b += b[i] * b[i];
        }
        let denom = norm_a.sqrt() * norm_b.sqrt();
        if denom <= 1e-9 {
            2.0
        } else {
            1.0 - (dot / denom).clamp(-1.0, 1.0)
        }
    }

    fn random_level(&self) -> usize {
        let mut r: f64 = rand::random::<f64>();
        if r == 0.0 {
            r = 0.0000001;
        }
        let lvl = (-r.ln() * self.level_mult).floor() as usize;
        lvl.min(16) // Batasi maksimum 16 layer
    }

    /// Menyisipkan vektor baru ke dalam indeks graf HNSW tanpa metadata
    pub fn insert(&mut self, id: &str, vector: &[f32]) -> usize {
        self.insert_with_metadata(id, vector, HashMap::new())
    }

    /// Menyisipkan vektor baru ke dalam indeks graf HNSW beserta metadata atribut dokumen
    pub fn insert_with_metadata(
        &mut self,
        id: &str,
        vector: &[f32],
        metadata: HashMap<String, String>,
    ) -> usize {
        let new_idx = self.nodes.len();
        let target_level = self.random_level();

        let mut node = HnswNode {
            id: id.to_string(),
            vector: vector.to_vec(),
            neighbors: vec![Vec::new(); target_level + 1],
            metadata,
        };

        if self.entry_point.is_none() {
            self.nodes.push(node);
            self.entry_point = Some(new_idx);
            self.max_level = target_level;
            return new_idx;
        }

        let mut curr_entry = self.entry_point.unwrap();
        let mut curr_dist = Self::cosine_distance(vector, &self.nodes[curr_entry].vector);

        // 1. Fase Navigasi: Greedy descent dari top layer hingga target_level + 1
        for lvl in (target_level + 1..=self.max_level).rev() {
            let mut changed = true;
            while changed {
                changed = false;
                if lvl < self.nodes[curr_entry].neighbors.len() {
                    for &neighbor_idx in &self.nodes[curr_entry].neighbors[lvl] {
                        let d = Self::cosine_distance(vector, &self.nodes[neighbor_idx].vector);
                        if d < curr_dist {
                            curr_dist = d;
                            curr_entry = neighbor_idx;
                            changed = true;
                        }
                    }
                }
            }
        }

        // 2. Fase Penyambungan Link: Dari min(target_level, max_level) turun ke layer 0
        let connect_top = target_level.min(self.max_level);
        for lvl in (0..=connect_top).rev() {
            let candidates = self.search_layer(vector, curr_entry, self.ef_construction, lvl);
            let max_neighbors = if lvl == 0 { self.m0 } else { self.m };

            // Urutkan kandidat berdasarkan jarak terdekat (ascending distance)
            let mut sorted: Vec<Candidate> = candidates.into_vec();
            sorted.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(Ordering::Equal));

            let mut selected = Vec::new();
            for c in sorted {
                if selected.len() >= max_neighbors {
                    break;
                }
                selected.push(c.node_idx);
            }

            // Tambahkan link dua arah (bidirectional link)
            for &neighbor_idx in &selected {
                node.neighbors[lvl].push(neighbor_idx);
                if lvl < self.nodes[neighbor_idx].neighbors.len() {
                    self.nodes[neighbor_idx].neighbors[lvl].push(new_idx);
                    // Pangkas tetangga berlebih jika melebihi kuota
                    if self.nodes[neighbor_idx].neighbors[lvl].len() > max_neighbors {
                        self.nodes[neighbor_idx].neighbors[lvl].truncate(max_neighbors);
                    }
                }
            }
        }

        self.nodes.push(node);

        if target_level > self.max_level {
            self.max_level = target_level;
            self.entry_point = Some(new_idx);
        }

        new_idx
    }

    fn search_layer(&self, query: &[f32], entry: usize, ef: usize, level: usize) -> BinaryHeap<Candidate> {
        let mut visited = HashSet::new();
        let mut candidates = BinaryHeap::new();
        let mut results = BinaryHeap::new();

        let initial_dist = Self::cosine_distance(query, &self.nodes[entry].vector);
        visited.insert(entry);

        candidates.push(Candidate { distance: initial_dist, node_idx: entry });
        results.push(Candidate { distance: initial_dist, node_idx: entry });

        while let Some(current) = candidates.pop() {
            if results.len() >= ef {
                if let Some(worst) = results.peek() {
                    if current.distance > worst.distance {
                        break;
                    }
                }
            }

            if level < self.nodes[current.node_idx].neighbors.len() {
                for &neighbor_idx in &self.nodes[current.node_idx].neighbors[level] {
                    if visited.insert(neighbor_idx) {
                        let d = Self::cosine_distance(query, &self.nodes[neighbor_idx].vector);
                        if results.len() < ef {
                            candidates.push(Candidate { distance: d, node_idx: neighbor_idx });
                            results.push(Candidate { distance: d, node_idx: neighbor_idx });
                        } else if let Some(worst) = results.peek() {
                            if d < worst.distance {
                                candidates.push(Candidate { distance: d, node_idx: neighbor_idx });
                                results.push(Candidate { distance: d, node_idx: neighbor_idx });
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Pencarian K-Nearest Neighbors (KNN) standar tanpa filter metadata
    pub fn search_knn(&self, query: &[f32], k: usize, ef_search: usize) -> Vec<(String, f32)> {
        if self.nodes.is_empty() || self.entry_point.is_none() {
            return Vec::new();
        }

        let mut curr_entry = self.entry_point.unwrap();
        let mut curr_dist = Self::cosine_distance(query, &self.nodes[curr_entry].vector);

        // 1. Greedy search dari layer atas sampai layer 1
        for lvl in (1..=self.max_level).rev() {
            let mut changed = true;
            while changed {
                changed = false;
                if lvl < self.nodes[curr_entry].neighbors.len() {
                    for &neighbor_idx in &self.nodes[curr_entry].neighbors[lvl] {
                        let d = Self::cosine_distance(query, &self.nodes[neighbor_idx].vector);
                        if d < curr_dist {
                            curr_dist = d;
                            curr_entry = neighbor_idx;
                            changed = true;
                        }
                    }
                }
            }
        }

        // 2. Beam search di ground level 0
        let candidates = self.search_layer(query, curr_entry, ef_search.max(k), 0);

        let mut sorted: Vec<Candidate> = candidates.into_vec();
        sorted.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(Ordering::Equal));
        sorted.truncate(k);

        sorted
            .into_iter()
            .map(|c| {
                let similarity = (1.0 - c.distance).clamp(0.0, 1.0);
                (self.nodes[c.node_idx].id.clone(), similarity)
            })
            .collect()
    }

    /// Pencarian K-Nearest Neighbors dengan Pre-Filtering:
    /// Kandidat dievaluasi terhadap metadata filter saat traversing struktur graf HNSW.
    pub fn search_knn_prefiltered(
        &self,
        query: &[f32],
        k: usize,
        ef_search: usize,
        filter: &MetadataFilter,
    ) -> Vec<(String, f32)> {
        if self.nodes.is_empty() || self.entry_point.is_none() {
            return Vec::new();
        }

        let ef = ef_search.max(k * 4).min(self.nodes.len().max(1));
        let candidates = self.search_layer(query, self.entry_point.unwrap(), ef, 0);

        let mut sorted: Vec<Candidate> = candidates.into_vec();
        sorted.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(Ordering::Equal));

        let mut matched = Vec::new();
        for c in sorted {
            if filter.matches(&self.nodes[c.node_idx].metadata) {
                let similarity = (1.0 - c.distance).clamp(0.0, 1.0);
                matched.push((self.nodes[c.node_idx].id.clone(), similarity));
                if matched.len() >= k {
                    break;
                }
            }
        }

        // Fallback jika beam belum cukup menemukan k dokumen karena selektivitas filter tinggi
        if matched.len() < k {
            for (idx, node) in self.nodes.iter().enumerate() {
                if !matched.iter().any(|(id, _)| id == &node.id) && filter.matches(&node.metadata) {
                    let d = Self::cosine_distance(query, &node.vector);
                    let sim = (1.0 - d).clamp(0.0, 1.0);
                    matched.push((node.id.clone(), sim));
                }
            }
            matched.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            matched.truncate(k);
        }

        matched
    }

    /// Pencarian K-Nearest Neighbors dengan Post-Filtering:
    /// Melakukan pencarian ANN diperluas (expanded beam) kemudian memfilter hasil sesuai metadata.
    pub fn search_knn_postfiltered(
        &self,
        query: &[f32],
        k: usize,
        ef_search: usize,
        filter: &MetadataFilter,
    ) -> Vec<(String, f32)> {
        let mut ef = ef_search.max(k * 2);
        let max_ef = self.nodes.len();

        loop {
            let candidates = self.search_layer(query, self.entry_point.unwrap_or(0), ef.min(max_ef), 0);
            let mut sorted: Vec<Candidate> = candidates.into_vec();
            sorted.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(Ordering::Equal));

            let mut matched = Vec::new();
            for c in sorted {
                if filter.matches(&self.nodes[c.node_idx].metadata) {
                    let sim = (1.0 - c.distance).clamp(0.0, 1.0);
                    matched.push((self.nodes[c.node_idx].id.clone(), sim));
                    if matched.len() >= k {
                        return matched;
                    }
                }
            }

            if ef >= max_ef || matched.len() >= k {
                return matched;
            }
            ef = (ef * 2).min(max_ef);
        }
    }

    /// Pencarian K-Nearest Neighbors Terpadu (Pre/Post-Filtering Hybrid)
    pub fn search_knn_filtered(
        &self,
        query: &[f32],
        k: usize,
        ef_search: usize,
        filter: &MetadataFilter,
    ) -> Vec<(String, f32)> {
        self.search_knn_prefiltered(query, k, ef_search, filter)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
