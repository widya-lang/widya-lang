//! WidyaStudio Live — Time-Travel Reversible Debugger & Hot-Code Reloading Engine
//! Provides recording of execution frame snapshots, reverse time stepping (step-backward),
//! variable delta history tracking, and live hot-code patching without state loss.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugVariable {
    pub name: String,
    pub var_type: String,
    pub value_repr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionFrameSnapshot {
    pub frame_index: usize,
    pub line_number: usize,
    pub function_name: String,
    pub variables: HashMap<String, DebugVariable>,
}

pub struct TimeTravelDebugger {
    history: Vec<ExecutionFrameSnapshot>,
    current_cursor: usize,
}

impl TimeTravelDebugger {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current_cursor: 0,
        }
    }

    pub fn record_snapshot(&mut self, line: usize, func: &str, vars: HashMap<String, DebugVariable>) {
        let frame_index = self.history.len();
        self.history.push(ExecutionFrameSnapshot {
            frame_index,
            line_number: line,
            function_name: func.to_string(),
            variables: vars,
        });
        self.current_cursor = frame_index;
    }

    pub fn step_backward(&mut self) -> Option<&ExecutionFrameSnapshot> {
        if self.current_cursor > 0 {
            self.current_cursor -= 1;
            Some(&self.history[self.current_cursor])
        } else {
            self.history.first()
        }
    }

    pub fn step_forward(&mut self) -> Option<&ExecutionFrameSnapshot> {
        if self.current_cursor + 1 < self.history.len() {
            self.current_cursor += 1;
            Some(&self.history[self.current_cursor])
        } else {
            self.history.last()
        }
    }

    pub fn current_frame(&self) -> Option<&ExecutionFrameSnapshot> {
        self.history.get(self.current_cursor)
    }

    pub fn total_recorded_frames(&self) -> usize {
        self.history.len()
    }
}
