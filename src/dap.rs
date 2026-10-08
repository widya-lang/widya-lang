use crate::error::Galat;
use crate::timetravel_debugger::{ExecutionSnapshot, TimeTravelDebugger};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBreakpoint {
    pub line: usize,
    pub column: Option<usize>,
    pub condition: Option<String>,
    pub hit_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrameInfo {
    pub id: usize,
    pub name: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableScope {
    pub name: String,
    pub value: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebuggerState {
    Initialized,
    Running,
    Paused,
    Terminated,
}

pub struct DapEngine {
    pub state: DebuggerState,
    pub breakpoints: HashMap<String, Vec<SourceBreakpoint>>,
    pub current_line: usize,
    pub call_stack: Vec<StackFrameInfo>,
    pub local_variables: HashMap<String, VariableScope>,
    pub time_travel: TimeTravelDebugger,
    pub watchpoints: HashMap<String, Option<String>>,
}

impl DapEngine {
    pub fn new() -> Self {
        Self {
            state: DebuggerState::Initialized,
            breakpoints: HashMap::new(),
            current_line: 1,
            call_stack: Vec::new(),
            local_variables: HashMap::new(),
            time_travel: TimeTravelDebugger::new(),
            watchpoints: HashMap::new(),
        }
    }

    pub fn set_breakpoints(&mut self, file: &str, lines: Vec<usize>) -> Vec<SourceBreakpoint> {
        let bps: Vec<SourceBreakpoint> = lines
            .into_iter()
            .map(|l| SourceBreakpoint {
                line: l,
                column: None,
                condition: None,
                hit_count: 0,
            })
            .collect();

        self.breakpoints.insert(file.to_string(), bps.clone());
        bps
    }

    pub fn set_conditional_breakpoint(&mut self, file: &str, line: usize, condition: &str) -> SourceBreakpoint {
        let bp = SourceBreakpoint {
            line,
            column: None,
            condition: Some(condition.to_string()),
            hit_count: 0,
        };
        self.breakpoints.entry(file.to_string()).or_default().push(bp.clone());
        bp
    }

    pub fn eval_condition(&self, condition: &str) -> bool {
        let trimmed = condition.trim();
        if trimmed == "benar" || trimmed == "true" {
            return true;
        }
        if trimmed == "salah" || trimmed == "palsu" || trimmed == "false" {
            return false;
        }
        if trimmed.contains("==") {
            let parts: Vec<&str> = trimmed.split("==").collect();
            if parts.len() == 2 {
                let var_name = parts[0].trim();
                let expected = parts[1].trim().trim_matches('"');
                if let Some(var) = self.local_variables.get(var_name) {
                    return var.value == expected;
                }
            }
        } else if trimmed.contains("!=") {
            let parts: Vec<&str> = trimmed.split("!=").collect();
            if parts.len() == 2 {
                let var_name = parts[0].trim();
                let expected = parts[1].trim().trim_matches('"');
                if let Some(var) = self.local_variables.get(var_name) {
                    return var.value != expected;
                }
            }
        } else if trimmed.contains('>') {
            let parts: Vec<&str> = trimmed.split('>').collect();
            if parts.len() == 2 {
                let var_name = parts[0].trim();
                if let (Some(var), Ok(threshold)) = (self.local_variables.get(var_name), parts[1].trim().parse::<i64>()) {
                    if let Ok(cur_val) = var.value.parse::<i64>() {
                        return cur_val > threshold;
                    }
                }
            }
        } else if trimmed.contains('<') {
            let parts: Vec<&str> = trimmed.split('<').collect();
            if parts.len() == 2 {
                let var_name = parts[0].trim();
                if let (Some(var), Ok(threshold)) = (self.local_variables.get(var_name), parts[1].trim().parse::<i64>()) {
                    if let Ok(cur_val) = var.value.parse::<i64>() {
                        return cur_val < threshold;
                    }
                }
            }
        } else if let Some(var) = self.local_variables.get(trimmed) {
            return var.value == "benar" || var.value == "true";
        }
        false
    }

    pub fn add_watchpoint(&mut self, variable: &str, condition: Option<String>) {
        self.watchpoints.insert(variable.to_string(), condition);
    }

    pub fn check_watchpoints(&mut self) -> Option<String> {
        for (var_name, cond) in &self.watchpoints {
            if let Some(var) = self.local_variables.get(var_name) {
                if let Some(c) = cond {
                    if self.eval_condition(c) {
                        self.state = DebuggerState::Paused;
                        return Some(format!("Watchpoint dipicu: {} memenuhi '{}'", var_name, c));
                    }
                } else {
                    self.state = DebuggerState::Paused;
                    return Some(format!("Watchpoint dipicu: nilai variabel {} berubah menjadi {}", var_name, var.value));
                }
            }
        }
        None
    }

    pub fn check_breakpoint(&mut self, file: &str, line: usize) -> bool {
        let (found, cond_to_check) = if let Some(bps) = self.breakpoints.get(file) {
            let mut matched = false;
            let mut cond = None;
            for bp in bps {
                if bp.line == line {
                    matched = true;
                    cond = bp.condition.clone();
                    break;
                }
            }
            (matched, cond)
        } else {
            (false, None)
        };

        if !found {
            return false;
        }

        if let Some(ref cond) = cond_to_check {
            if !self.eval_condition(cond) {
                return false;
            }
        }

        if let Some(bps) = self.breakpoints.get_mut(file) {
            for bp in bps {
                if bp.line == line {
                    bp.hit_count += 1;
                    break;
                }
            }
        }

        self.state = DebuggerState::Paused;
        self.current_line = line;
        true
    }

    pub fn step_over(&mut self) {
        self.current_line += 1;
        self.state = DebuggerState::Paused;
    }

    pub fn continue_exec(&mut self) {
        self.state = DebuggerState::Running;
    }

    pub fn push_frame(&mut self, frame: StackFrameInfo) {
        self.call_stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<StackFrameInfo> {
        self.call_stack.pop()
    }

    pub fn set_variable(&mut self, name: &str, value: &str, type_name: &str) {
        self.local_variables.insert(
            name.to_string(),
            VariableScope {
                name: name.to_string(),
                value: value.to_string(),
                type_name: type_name.to_string(),
            },
        );
    }

    pub fn evaluate_expr(&self, expr: &str) -> Result<String, Galat> {
        if let Some(var) = self.local_variables.get(expr) {
            Ok(format!("{} ({})", var.value, var.type_name))
        } else {
            Ok(format!("(ekspresi '{}' dievaluasi ke nilai)", expr))
        }
    }

    pub fn record_execution_step(&mut self, ip: usize, vars: HashMap<String, String>, timestamp_ms: u64) {
        self.current_line = ip;
        self.time_travel.record_step(ip, vars.clone(), timestamp_ms);
        for (k, v) in vars {
            self.set_variable(&k, &v, "dynamic");
        }
    }

    /// Step Back (Mundur ke snapshot sebelumnya dalam riwayat eksekusi)
    pub fn step_back(&mut self) -> Option<ExecutionSnapshot> {
        let snap = self.time_travel.step_backward().cloned();
        if let Some(ref s) = snap {
            self.current_line = s.instruction_pointer;
            self.local_variables.clear();
            for (k, v) in &s.variables {
                self.set_variable(k, v, "dynamic");
            }
        }
        snap
    }

    /// Rewind ke titik terakhir di mana suatu variabel dimutasi
    pub fn rewind_to_mutation(&mut self, variable_name: &str) -> Option<ExecutionSnapshot> {
        let snap = self.time_travel.rewind_to_last_mutation(variable_name).cloned();
        if let Some(ref s) = snap {
            self.current_line = s.instruction_pointer;
            self.local_variables.clear();
            for (k, v) in &s.variables {
                self.set_variable(k, v, "dynamic");
            }
        }
        snap
    }

    pub fn get_dap_capabilities(&self) -> serde_json::Value {
        serde_json::json!({
            "supportsConfigurationDoneRequest": true,
            "supportsFunctionBreakpoints": true,
            "supportsConditionalBreakpoints": true,
            "supportsEvaluateForHovers": true,
            "supportsStepBack": true,
            "supportsSetVariable": true,
            "supportsRestartFrame": true,
        })
    }
}

impl Default for DapEngine {
    fn default() -> Self {
        Self::new()
    }
}
