//! Engine Kompilasi Inkremental & Cache Daemon (incremental.rs)
//! Menyimpan hash SHA-256 berkas, grafik dependensi modul (DAG), dan hanya mengkompilasi modul yang terdampak.

use std::collections::{HashMap, HashSet};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleMetadata {
    pub module_name: String,
    pub content_hash: String,
    pub dependencies: Vec<String>,
    pub compiled_bytecode_size: usize,
    pub last_compiled_timestamp: u64,
}

pub struct IncrementalCache {
    modules: HashMap<String, ModuleMetadata>,
    reverse_dependencies: HashMap<String, HashSet<String>>,
}

impl IncrementalCache {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            reverse_dependencies: HashMap::new(),
        }
    }

    /// Menghitung SHA-256 fingerprint dari konten kode sumber
    pub fn calculate_hash(source_code: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(source_code.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Memeriksa apakah modul perlu dikompilasi ulang berdasarkan konten baru
    pub fn is_dirty(&self, module_name: &str, current_source: &str) -> bool {
        let current_hash = Self::calculate_hash(current_source);
        match self.modules.get(module_name) {
            Some(meta) => meta.content_hash != current_hash,
            None => true, // Modul baru belum ada di cache
        }
    }

    /// Mendaftarkan / memperbarui modul yang sukses dikompilasi
    pub fn update_module(
        &mut self,
        module_name: &str,
        source_code: &str,
        dependencies: Vec<String>,
        bytecode_size: usize,
        timestamp: u64,
    ) {
        let hash = Self::calculate_hash(source_code);

        // Update reverse dependency index
        for dep in &dependencies {
            self.reverse_dependencies
                .entry(dep.clone())
                .or_default()
                .insert(module_name.to_string());
        }

        self.modules.insert(
            module_name.to_string(),
            ModuleMetadata {
                module_name: module_name.to_string(),
                content_hash: hash,
                dependencies,
                compiled_bytecode_size: bytecode_size,
                last_compiled_timestamp: timestamp,
            },
        );
    }

    /// Menemukan seluruh modul hilir yang ikut terpengaruh jika suatu modul berubah (Cascading Invalidation)
    pub fn get_affected_modules(&self, changed_module: &str) -> HashSet<String> {
        let mut affected = HashSet::new();
        let mut queue = vec![changed_module.to_string()];

        while let Some(current) = queue.pop() {
            if affected.insert(current.clone()) {
                if let Some(dependents) = self.reverse_dependencies.get(&current) {
                    for dep in dependents {
                        if !affected.contains(dep) {
                            queue.push(dep.clone());
                        }
                    }
                }
            }
        }

        affected
    }

    pub fn cached_module_count(&self) -> usize {
        self.modules.len()
    }
}
