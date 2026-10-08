//! Time-Travel Debugger & Reverse Execution Engine (timetravel_debugger.rs)
//! Menyimpan snapshot frame eksekusi & delta register untuk memungkinkan pelacakan mundur
//! (Step Backward, Rewind, dan Reverse Conditional Watchpoints) dengan konsumsi RAM hemat (differential snapshots).

use std::collections::HashMap;

/// Snapshot Status Eksekusi pada Titik Waktu Tertentu
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionSnapshot {
    pub step_index: usize,
    pub instruction_pointer: usize,
    pub variables: HashMap<String, String>,
    pub timestamp_ms: u64,
}

/// Snapshot Diferensial (Menyimpan hanya delta perubahan variabel dari langkah sebelumnya)
#[derive(Debug, Clone, PartialEq)]
pub struct DeltaExecutionSnapshot {
    pub step_index: usize,
    pub instruction_pointer: usize,
    pub changed_variables: HashMap<String, String>, // Key -> Nilai Baru
    pub timestamp_ms: u64,
}

/// Time-Travel Debugger Session
pub struct TimeTravelDebugger {
    snapshots: Vec<ExecutionSnapshot>,
    delta_snapshots: Vec<DeltaExecutionSnapshot>,
    current_cursor: usize,
}

impl TimeTravelDebugger {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            delta_snapshots: Vec::new(),
            current_cursor: 0,
        }
    }

    /// Rekam snapshot baru saat program berjalan maju
    pub fn record_step(&mut self, ip: usize, vars: HashMap<String, String>, timestamp_ms: u64) {
        let step = self.snapshots.len();

        // Hitung delta perubahan dari snapshot sebelumnya
        let mut changes = HashMap::new();
        if let Some(prev) = self.snapshots.last() {
            for (k, v) in &vars {
                if prev.variables.get(k) != Some(v) {
                    changes.insert(k.clone(), v.clone());
                }
            }
        } else {
            changes = vars.clone();
        }

        self.delta_snapshots.push(DeltaExecutionSnapshot {
            step_index: step,
            instruction_pointer: ip,
            changed_variables: changes,
            timestamp_ms,
        });

        self.snapshots.push(ExecutionSnapshot {
            step_index: step,
            instruction_pointer: ip,
            variables: vars,
            timestamp_ms,
        });
        self.current_cursor = self.snapshots.len() - 1;
    }

    /// Melangkah Mundur Satu Langkah (Step Backward / Undo Execution)
    pub fn step_backward(&mut self) -> Option<&ExecutionSnapshot> {
        if self.current_cursor > 0 {
            self.current_cursor -= 1;
            Some(&self.snapshots[self.current_cursor])
        } else {
            None
        }
    }

    /// Melangkah Maju Satu Langkah (Step Forward / Redo)
    pub fn step_forward(&mut self) -> Option<&ExecutionSnapshot> {
        if self.current_cursor + 1 < self.snapshots.len() {
            self.current_cursor += 1;
            Some(&self.snapshots[self.current_cursor])
        } else {
            None
        }
    }

    /// Loncat Langsung ke Langkah Tertentu (Rewind to Step Index)
    pub fn rewind_to(&mut self, step_index: usize) -> Option<&ExecutionSnapshot> {
        if step_index < self.snapshots.len() {
            self.current_cursor = step_index;
            Some(&self.snapshots[self.current_cursor])
        } else {
            None
        }
    }

    /// Reverse Conditional Watchpoint: Mundur ke titik terakhir di mana variabel diubah
    pub fn rewind_to_last_mutation(&mut self, variable_name: &str) -> Option<&ExecutionSnapshot> {
        if self.current_cursor == 0 {
            return None;
        }

        let start = self.current_cursor - 1;
        for i in (0..=start).rev() {
            if self.delta_snapshots[i].changed_variables.contains_key(variable_name) {
                self.current_cursor = i;
                return Some(&self.snapshots[i]);
            }
        }
        None
    }

    /// Reverse Conditional Watchpoint: Mundur ke titik di mana predikat kondisi terpenuhi
    pub fn rewind_until_predicate<F>(&mut self, predicate: F) -> Option<&ExecutionSnapshot>
    where
        F: Fn(&HashMap<String, String>) -> bool,
    {
        if self.current_cursor == 0 {
            return None;
        }

        let start = self.current_cursor - 1;
        for i in (0..=start).rev() {
            if predicate(&self.snapshots[i].variables) {
                self.current_cursor = i;
                return Some(&self.snapshots[i]);
            }
        }
        None
    }

    pub fn current_snapshot(&self) -> Option<&ExecutionSnapshot> {
        self.snapshots.get(self.current_cursor)
    }

    pub fn total_steps(&self) -> usize {
        self.snapshots.len()
    }
}
