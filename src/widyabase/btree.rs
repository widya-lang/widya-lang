// ============================================================================
// WIDYABASE ADVANCED B+TREE INDEX (btree.rs)
// ============================================================================
// Indeks pohon B+Tree terurut untuk pencarian kunci utama O(log N),
// Composite Indexing (Multi-Column), Desimal Indexing i128, dan Range Scan.
// ============================================================================

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use crate::compiler::decimal::Desimal;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IndexKey {
    Integer(i64),
    Text(String),
    Decimal(Desimal),
    Composite(Vec<IndexKey>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordPointer {
    pub page_id: u32,
    pub slot_id: u16,
}

pub struct BTreeIndex {
    pub name: String,
    // Penyimpanan in-memory terstruktur B+Tree dengan sinkronisasi thread-safe
    tree: Arc<RwLock<BTreeMap<IndexKey, RecordPointer>>>,
}

impl BTreeIndex {
    pub fn new(name: &str) -> Self {
        BTreeIndex {
            name: name.to_string(),
            tree: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub fn insert(&self, key: IndexKey, ptr: RecordPointer) {
        let mut guard = self.tree.write().unwrap();
        guard.insert(key, ptr);
    }

    pub fn search(&self, key: &IndexKey) -> Option<RecordPointer> {
        let guard = self.tree.read().unwrap();
        guard.get(key).copied()
    }

    pub fn delete(&self, key: &IndexKey) -> bool {
        let mut guard = self.tree.write().unwrap();
        guard.remove(key).is_some()
    }

    pub fn range_scan(&self, min_key: Option<&IndexKey>, max_key: Option<&IndexKey>) -> Vec<(IndexKey, RecordPointer)> {
        let guard = self.tree.read().unwrap();
        let mut results = Vec::new();

        for (k, v) in guard.iter() {
            let match_min = match min_key {
                Some(min) => k >= min,
                None => true,
            };
            let match_max = match max_key {
                Some(max) => k <= max,
                None => true,
            };

            if match_min && match_max {
                results.push((k.clone(), *v));
            }
        }
        results
    }

    pub fn len(&self) -> usize {
        let guard = self.tree.read().unwrap();
        guard.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
