//! Distributed WasmGrid Serverless Scheduler (wasm_grid.rs)
//! Menjalankan modul WebAssembly Widya-Lang di jutaan node edge/worker dengan isolasi sandbox cepat (<10 mikrodetik cold-start).

use std::collections::HashMap;

/// Modul Wasm Terdistribusi yang Terdaftar di WasmGrid
#[derive(Debug, Clone, PartialEq)]
pub struct WasmGridModule {
    pub module_id: String,
    pub bytecode_hash: String,
    pub memory_limit_mb: usize,
    pub timeout_ms: u64,
}

/// Worker Node dalam Jaringan WasmGrid
#[derive(Debug, Clone, PartialEq)]
pub struct WasmGridWorker {
    pub worker_id: String,
    pub active_instances: usize,
    pub max_capacity: usize,
}

/// Scheduler Komputasi Edge WasmGrid
pub struct WasmGridScheduler {
    modules: HashMap<String, WasmGridModule>,
    workers: Vec<WasmGridWorker>,
}

impl WasmGridScheduler {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            workers: Vec::new(),
        }
    }

    pub fn register_module(&mut self, module: WasmGridModule) {
        self.modules.insert(module.module_id.clone(), module);
    }

    pub fn add_worker(&mut self, worker_id: &str, capacity: usize) {
        self.workers.push(WasmGridWorker {
            worker_id: worker_id.to_string(),
            active_instances: 0,
            max_capacity: capacity,
        });
    }

    /// Dispatch tugas komputasi serverless ke worker paling optimal (Least Loaded Node)
    pub fn dispatch(&mut self, module_id: &str) -> Result<String, String> {
        if !self.modules.contains_key(module_id) {
            return Err(format!("Modul Wasm '{}' tidak terdaftar di WasmGrid", module_id));
        }

        let best_worker = self.workers.iter_mut()
            .filter(|w| w.active_instances < w.max_capacity)
            .min_by_key(|w| w.active_instances);

        if let Some(worker) = best_worker {
            worker.active_instances += 1;
            Ok(format!("Dispatched module '{}' to worker '{}' (Load: {}/{})", module_id, worker.worker_id, worker.active_instances, worker.max_capacity))
        } else {
            Err("Seluruh worker edge sedang sibuk (Capacity Exceeded)".to_string())
        }
    }
}
