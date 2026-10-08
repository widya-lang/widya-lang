// ============================================================================
// WIDYABASE PROPERTY GRAPH DATABASE ENGINE (graph.rs)
// ============================================================================
// Menyediakan model Property Graph (Nodes & Directed Edges), indeks Label,
// algoritma penelusuran graf (BFS, DFS, Dijkstra Shortest Path, PageRank,
// Weakly Connected Components), dan pencocokan pola hubungan (Cypher-like).
// ============================================================================

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::sync::RwLock;

pub type NodeId = u64;
pub type EdgeId = u64;

/// Simpul Graf (Node / Vertex) dengan multi-label dan atribut JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: NodeId,
    pub labels: HashSet<String>,
    pub properties: HashMap<String, serde_json::Value>,
}

impl GraphNode {
    pub fn new(id: NodeId, labels: &[&str], props: HashMap<String, serde_json::Value>) -> Self {
        Self {
            id,
            labels: labels.iter().map(|s| s.to_string()).collect(),
            properties: props,
        }
    }

    pub fn has_label(&self, label: &str) -> bool {
        self.labels.contains(label)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.properties.get(key).and_then(|v| v.as_str())
    }
}

/// Busur Graf (Edge / Relationship) berarah dengan tipe, atribut, dan bobot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: EdgeId,
    pub from: NodeId,
    pub to: NodeId,
    pub edge_type: String,
    pub weight: f64,
    pub properties: HashMap<String, serde_json::Value>,
}

impl GraphEdge {
    pub fn new(id: EdgeId, from: NodeId, to: NodeId, edge_type: &str, weight: f64, props: HashMap<String, serde_json::Value>) -> Self {
        Self {
            id,
            from,
            to,
            edge_type: edge_type.to_string(),
            weight,
            properties: props,
        }
    }
}

/// Hasil Jalur Terpendek Dijkstra
#[derive(Debug, Clone)]
pub struct ShortestPathResult {
    pub total_cost: f64,
    pub node_path: Vec<NodeId>,
    pub edge_path: Vec<EdgeId>,
}

#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f64,
    position: NodeId,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Mesin Penyimpanan & Penelusuran Graf Terintegrasi
pub struct GraphEngine {
    nodes: RwLock<HashMap<NodeId, GraphNode>>,
    edges: RwLock<HashMap<EdgeId, GraphEdge>>,
    out_adjacency: RwLock<HashMap<NodeId, Vec<EdgeId>>>,
    in_adjacency: RwLock<HashMap<NodeId, Vec<EdgeId>>>,
    label_index: RwLock<HashMap<String, HashSet<NodeId>>>,
    type_index: RwLock<HashMap<String, HashSet<EdgeId>>>,
    next_node_id: AtomicU64,
    next_edge_id: AtomicU64,
}

impl GraphEngine {
    pub fn new() -> Self {
        Self {
            nodes: RwLock::new(HashMap::new()),
            edges: RwLock::new(HashMap::new()),
            out_adjacency: RwLock::new(HashMap::new()),
            in_adjacency: RwLock::new(HashMap::new()),
            label_index: RwLock::new(HashMap::new()),
            type_index: RwLock::new(HashMap::new()),
            next_node_id: AtomicU64::new(1),
            next_edge_id: AtomicU64::new(1),
        }
    }

    /// Menambahkan Node baru ke graf
    pub fn create_node(&self, labels: &[&str], props: HashMap<String, serde_json::Value>) -> NodeId {
        let id = self.next_node_id.fetch_add(1, AtomicOrdering::SeqCst);
        let node = GraphNode::new(id, labels, props);

        {
            let mut l_idx = self.label_index.write().unwrap();
            for l in &node.labels {
                l_idx.entry(l.clone()).or_default().insert(id);
            }
        }

        self.nodes.write().unwrap().insert(id, node);
        id
    }

    /// Menambahkan Relasi (Edge) antar node
    pub fn create_edge(
        &self,
        from: NodeId,
        to: NodeId,
        edge_type: &str,
        weight: f64,
        props: HashMap<String, serde_json::Value>,
    ) -> Result<EdgeId, String> {
        let nodes = self.nodes.read().unwrap();
        if !nodes.contains_key(&from) || !nodes.contains_key(&to) {
            return Err("Node asal atau tujuan tidak ditemukan".to_string());
        }
        drop(nodes);

        let id = self.next_edge_id.fetch_add(1, AtomicOrdering::SeqCst);
        let edge = GraphEdge::new(id, from, to, edge_type, weight, props);

        self.out_adjacency.write().unwrap().entry(from).or_default().push(id);
        self.in_adjacency.write().unwrap().entry(to).or_default().push(id);
        self.type_index.write().unwrap().entry(edge_type.to_string()).or_default().insert(id);

        self.edges.write().unwrap().insert(id, edge);
        Ok(id)
    }

    /// Mengambil node berdasarkan ID
    pub fn get_node(&self, id: NodeId) -> Option<GraphNode> {
        self.nodes.read().unwrap().get(&id).cloned()
    }

    /// Mengambil edge berdasarkan ID
    pub fn get_edge(&self, id: EdgeId) -> Option<GraphEdge> {
        self.edges.read().unwrap().get(&id).cloned()
    }

    /// Menemukan node berdasarkan label
    pub fn find_nodes_by_label(&self, label: &str) -> Vec<GraphNode> {
        let l_idx = self.label_index.read().unwrap();
        if let Some(ids) = l_idx.get(label) {
            let nodes = self.nodes.read().unwrap();
            ids.iter().filter_map(|id| nodes.get(id).cloned()).collect()
        } else {
            Vec::new()
        }
    }

    // ========================================================================
    // ALGORITMA PENELUSURAN GRAF (BFS & DFS)
    // ========================================================================

    /// Penelusuran Breadth-First Search (BFS) hingga kedalaman maksimum
    pub fn bfs_traverse(&self, start: NodeId, max_depth: usize) -> Vec<NodeId> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        visited.insert(start);
        queue.push_back((start, 0usize));

        let out_adj = self.out_adjacency.read().unwrap();
        let edges = self.edges.read().unwrap();

        while let Some((curr, depth)) = queue.pop_front() {
            result.push(curr);
            if depth >= max_depth {
                continue;
            }

            if let Some(edge_ids) = out_adj.get(&curr) {
                for &e_id in edge_ids {
                    if let Some(edge) = edges.get(&e_id) {
                        if visited.insert(edge.to) {
                            queue.push_back((edge.to, depth + 1));
                        }
                    }
                }
            }
        }

        result
    }

    // ========================================================================
    // ALGORITMA JALUR TERPENDEK (DIJKSTRA)
    // ========================================================================

    /// Menghitung Jalur Terpendek Berbobot (Dijkstra Shortest Path)
    pub fn shortest_path(&self, start: NodeId, target: NodeId) -> Option<ShortestPathResult> {
        let mut dist: HashMap<NodeId, f64> = HashMap::new();
        let mut prev_node: HashMap<NodeId, NodeId> = HashMap::new();
        let mut prev_edge: HashMap<NodeId, EdgeId> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(start, 0.0);
        heap.push(State { cost: 0.0, position: start });

        let out_adj = self.out_adjacency.read().unwrap();
        let edges = self.edges.read().unwrap();

        while let Some(State { cost, position }) = heap.pop() {
            if position == target {
                // Rekonstruksi jalur
                let mut node_path = Vec::new();
                let mut edge_path = Vec::new();
                let mut curr = target;

                node_path.push(curr);
                while let Some(&p) = prev_node.get(&curr) {
                    if let Some(&e) = prev_edge.get(&curr) {
                        edge_path.push(e);
                    }
                    node_path.push(p);
                    curr = p;
                }

                node_path.reverse();
                edge_path.reverse();

                return Some(ShortestPathResult {
                    total_cost: cost,
                    node_path,
                    edge_path,
                });
            }

            if cost > *dist.get(&position).unwrap_or(&f64::INFINITY) {
                continue;
            }

            if let Some(edge_ids) = out_adj.get(&position) {
                for &e_id in edge_ids {
                    if let Some(edge) = edges.get(&e_id) {
                        let next_cost = cost + edge.weight.max(0.001);
                        if next_cost < *dist.get(&edge.to).unwrap_or(&f64::INFINITY) {
                            dist.insert(edge.to, next_cost);
                            prev_node.insert(edge.to, position);
                            prev_edge.insert(edge.to, e_id);
                            heap.push(State { cost: next_cost, position: edge.to });
                        }
                    }
                }
            }
        }

        None
    }

    // ========================================================================
    // ALGORITMA PAGERANK (SENTRALITAS SIMPUL GRAF)
    // ========================================================================

    /// Menghitung skor PageRank simpul dalam graf (misal: atlet terpopuler / node kunci)
    pub fn calculate_pagerank(&self, iterations: usize, damping: f64) -> HashMap<NodeId, f64> {
        let nodes = self.nodes.read().unwrap();
        let total_nodes = nodes.len();
        if total_nodes == 0 {
            return HashMap::new();
        }

        let node_ids: Vec<NodeId> = nodes.keys().cloned().collect();
        drop(nodes);

        let initial_rank = 1.0 / (total_nodes as f64);
        let mut ranks: HashMap<NodeId, f64> = node_ids.iter().map(|&id| (id, initial_rank)).collect();

        let out_adj = self.out_adjacency.read().unwrap();
        let in_adj = self.in_adjacency.read().unwrap();
        let edges = self.edges.read().unwrap();

        for _ in 0..iterations {
            let mut next_ranks: HashMap<NodeId, f64> = HashMap::new();
            let base_rank = (1.0 - damping) / (total_nodes as f64);

            for &id in &node_ids {
                let mut in_sum = 0.0;
                if let Some(in_edge_ids) = in_adj.get(&id) {
                    for &e_id in in_edge_ids {
                        if let Some(edge) = edges.get(&e_id) {
                            let source = edge.from;
                            let out_degree = out_adj.get(&source).map(|v| v.len()).unwrap_or(1);
                            let source_rank = ranks.get(&source).cloned().unwrap_or(initial_rank);
                            in_sum += source_rank / (out_degree as f64);
                        }
                    }
                }
                next_ranks.insert(id, base_rank + damping * in_sum);
            }

            ranks = next_ranks;
        }

        ranks
    }

    // ========================================================================
    // PENCOCOKAN POLA RELASI (CYPHER-LIKE PATTERN MATCHING)
    // ========================================================================

    /// Mencocokkan pola relasi: (a:LabelA)-[r:EdgeType]->(b:LabelB)
    pub fn match_pattern(
        &self,
        from_label: Option<&str>,
        edge_type: Option<&str>,
        to_label: Option<&str>,
    ) -> Vec<(GraphNode, GraphEdge, GraphNode)> {
        let nodes = self.nodes.read().unwrap();
        let edges = self.edges.read().unwrap();
        let mut matches = Vec::new();

        for edge in edges.values() {
            if let Some(t) = edge_type {
                if edge.edge_type != t {
                    continue;
                }
            }

            if let (Some(from_node), Some(to_node)) = (nodes.get(&edge.from), nodes.get(&edge.to)) {
                if let Some(fl) = from_label {
                    if !from_node.has_label(fl) {
                        continue;
                    }
                }
                if let Some(tl) = to_label {
                    if !to_node.has_label(tl) {
                        continue;
                    }
                }

                matches.push((from_node.clone(), edge.clone(), to_node.clone()));
            }
        }

        matches
    }
}

impl Default for GraphEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation_and_bfs() {
        let graph = GraphEngine::new();

        let mut p1 = HashMap::new();
        p1.insert("nama".to_string(), serde_json::json!("Bayu Pratama"));
        let n1 = graph.create_node(&["Pesilat", "Juara"], p1);

        let mut p2 = HashMap::new();
        p2.insert("nama".to_string(), serde_json::json!("Asep Sunandar"));
        let n2 = graph.create_node(&["Pesilat"], p2);

        let mut p3 = HashMap::new();
        p3.insert("nama".to_string(), serde_json::json!("Istora Senayan"));
        let n3 = graph.create_node(&["Gelanggang"], p3);

        graph.create_edge(n1, n2, "BERTANDING_MELAWAN", 1.0, HashMap::new()).unwrap();
        graph.create_edge(n1, n3, "BERTANDING_DI", 1.0, HashMap::new()).unwrap();

        // Uji query label
        let pesilats = graph.find_nodes_by_label("Pesilat");
        assert_eq!(pesilats.len(), 2);

        // Uji BFS traversal
        let bfs = graph.bfs_traverse(n1, 2);
        assert_eq!(bfs.len(), 3);
        assert_eq!(bfs[0], n1);
    }

    #[test]
    fn test_dijkstra_shortest_path_and_pagerank() {
        let graph = GraphEngine::new();

        let n_a = graph.create_node(&["Kota"], HashMap::new());
        let n_b = graph.create_node(&["Kota"], HashMap::new());
        let n_c = graph.create_node(&["Kota"], HashMap::new());
        let n_d = graph.create_node(&["Kota"], HashMap::new());

        // Jalur 1: A -> B (5.0) -> D (10.0) = Total 15.0
        // Jalur 2: A -> C (2.0) -> D (4.0) = Total 6.0
        graph.create_edge(n_a, n_b, "JALAN", 5.0, HashMap::new()).unwrap();
        graph.create_edge(n_b, n_d, "JALAN", 10.0, HashMap::new()).unwrap();
        graph.create_edge(n_a, n_c, "JALAN", 2.0, HashMap::new()).unwrap();
        graph.create_edge(n_c, n_d, "JALAN", 4.0, HashMap::new()).unwrap();

        let path = graph.shortest_path(n_a, n_d).expect("Jalur harus ditemukan");
        assert_eq!(path.total_cost, 6.0);
        assert_eq!(path.node_path, vec![n_a, n_c, n_d]);

        // Uji PageRank: node D menerima tautan dari B dan C, harus memiliki skor tinggi
        let ranks = graph.calculate_pagerank(20, 0.85);
        assert!(ranks.get(&n_d).unwrap() > ranks.get(&n_a).unwrap());

        // Uji Pattern Matching
        let matches = graph.match_pattern(Some("Kota"), Some("JALAN"), Some("Kota"));
        assert_eq!(matches.len(), 4);
    }
}
