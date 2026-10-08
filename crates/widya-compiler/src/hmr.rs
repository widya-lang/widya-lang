//! Engine Hot-Module Replacement (HMR) & Live State Preservation (hmr.rs)
//! Memungkinkan penukaran kode runtime secara instan tanpa menghentikan aplikasi atau menghilangkan state memori.

use std::collections::HashMap;
use widya_vm::vm::VmValue;

#[derive(Debug, Clone, PartialEq)]
pub struct StateSnapshot {
    pub module_name: String,
    pub version: u32,
    pub global_bindings: HashMap<String, VmValue>,
    pub created_timestamp: u64,
}

pub struct HotModuleReloader {
    module_versions: HashMap<String, u32>,
    persisted_states: HashMap<String, StateSnapshot>,
    patch_history: Vec<(String, u32, String)>, // (module, version, patch_description)
}

impl HotModuleReloader {
    pub fn new() -> Self {
        Self {
            module_versions: HashMap::new(),
            persisted_states: HashMap::new(),
            patch_history: Vec::new(),
        }
    }

    /// Mengambil snapshot state variabel modul sebelum dilakukan hot-swap
    pub fn save_state(&mut self, module_name: &str, bindings: HashMap<String, VmValue>, timestamp: u64) -> u32 {
        let current_version = self.module_versions.entry(module_name.to_string()).or_insert(1);
        let version = *current_version;

        self.persisted_states.insert(
            module_name.to_string(),
            StateSnapshot {
                module_name: module_name.to_string(),
                version,
                global_bindings: bindings,
                created_timestamp: timestamp,
            },
        );

        version
    }

    /// Melakukan hot-patch pada modul dengan versi baru dan mengembalikan state yang tersimpan
    pub fn apply_patch(
        &mut self,
        module_name: &str,
        patch_description: &str,
    ) -> Result<Option<StateSnapshot>, String> {
        let version_entry = self.module_versions.entry(module_name.to_string()).or_insert(0);
        *version_entry += 1;
        let new_version = *version_entry;

        self.patch_history.push((
            module_name.to_string(),
            new_version,
            patch_description.to_string(),
        ));

        // Kembalikan state yang tersimpan untuk re-hydration
        Ok(self.persisted_states.get(module_name).cloned())
    }

    pub fn get_current_version(&self, module_name: &str) -> u32 {
        self.module_versions.get(module_name).copied().unwrap_or(0)
    }

    pub fn total_patches_applied(&self) -> usize {
        self.patch_history.len()
    }
}
