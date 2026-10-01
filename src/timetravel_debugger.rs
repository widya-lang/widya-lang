//! Time-Travel Debugger & Reverse Execution Engine (timetravel_debugger.rs)
//! Menyimpan snapshot frame eksekusi & register untuk memungkinkan pelacakan mundur (Step Backward & Rewind) dalam debugging.

use std::collections::HashMap;

/// Snapshot Status Eksekusi pada Titik Waktu Tertentu
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionSnapshot {
    pub step_index: usize,
    pub instruction_pointer: usize,
    pub variables: HashMap<String, String>,
    pub timestamp_ms: u64,
}

/// Time-Travel Debugger Session
pub struct TimeTravelDebugger {
    snapshots: Vec<ExecutionSnapshot>,
    current_cursor: usize,
}

impl TimeTravelDebugger {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            current_cursor: 0,
        }
    }

    /// Rekam snapshot baru saat program berjalan maju
    pub fn record_step(&mut self, ip: usize, vars: HashMap<String, String>, timestamp_ms: u64) {
        let step = self.snapshots.len();
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

    pub fn current_snapshot(&self) -> Option<&ExecutionSnapshot> {
        self.snapshots.get(self.current_cursor)
    }
}
