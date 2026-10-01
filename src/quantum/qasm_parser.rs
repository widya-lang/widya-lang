use serde::{Deserialize, Serialize};

/// ============================================================================
/// OpenQASM 3.0 Parser & AST
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuantumGateOp {
    Hadamard(usize),
    PauliX(usize),
    PauliY(usize),
    PauliZ(usize),
    Cnot(usize, usize), // Control, Target
    RotationZ(usize, f64),
    Measure(usize, usize), // Qubit index, Classical bit index
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub num_clbits: usize,
    pub gates: Vec<QuantumGateOp>,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize, num_clbits: usize) -> Self {
        Self {
            num_qubits,
            num_clbits,
            gates: Vec::new(),
        }
    }

    /// Parse OpenQASM 2.0 / 3.0 string subset
    pub fn parse_qasm(source: &str) -> Result<Self, String> {
        let mut num_qubits = 0;
        let mut num_clbits = 0;
        let mut gates = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("OPENQASM") || trimmed.starts_with("include") {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "qreg" | "qubit" => {
                    // e.g., qreg q[4]; or qubit[4] q;
                    if let Some(bracket) = trimmed.find('[') {
                        if let Some(end_bracket) = trimmed.find(']') {
                            let count: usize = trimmed[bracket + 1..end_bracket].parse().unwrap_or(2);
                            num_qubits = count;
                        }
                    }
                }
                "creg" | "bit" => {
                    // e.g., creg c[4]; or bit[4] c;
                    if let Some(bracket) = trimmed.find('[') {
                        if let Some(end_bracket) = trimmed.find(']') {
                            let count: usize = trimmed[bracket + 1..end_bracket].parse().unwrap_or(2);
                            num_clbits = count;
                        }
                    }
                }
                "h" => {
                    // h q[0];
                    let q = extract_qubit_index(parts[1])?;
                    gates.push(QuantumGateOp::Hadamard(q));
                }
                "x" => {
                    let q = extract_qubit_index(parts[1])?;
                    gates.push(QuantumGateOp::PauliX(q));
                }
                "z" => {
                    let q = extract_qubit_index(parts[1])?;
                    gates.push(QuantumGateOp::PauliZ(q));
                }
                "cx" => {
                    // cx q[0], q[1];
                    let raw = trimmed[2..].replace(';', "");
                    let q_parts: Vec<&str> = raw.split(',').map(|s| s.trim()).collect();
                    if q_parts.len() == 2 {
                        let c = extract_qubit_index(q_parts[0])?;
                        let t = extract_qubit_index(q_parts[1])?;
                        gates.push(QuantumGateOp::Cnot(c, t));
                    }
                }
                "measure" => {
                    // measure q[0] -> c[0];
                    let raw = trimmed[7..].replace(';', "");
                    let q_parts: Vec<&str> = raw.split("->").map(|s| s.trim()).collect();
                    if q_parts.len() == 2 {
                        let q = extract_qubit_index(q_parts[0])?;
                        let c = extract_qubit_index(q_parts[1])?;
                        gates.push(QuantumGateOp::Measure(q, c));
                    }
                }
                _ => {}
            }
        }

        if num_qubits == 0 {
            num_qubits = 2;
        }

        Ok(Self {
            num_qubits,
            num_clbits,
            gates,
        })
    }
}

fn extract_qubit_index(s: &str) -> Result<usize, String> {
    let clean = s.trim().trim_end_matches(';');
    if let Some(start) = clean.find('[') {
        if let Some(end) = clean.find(']') {
            let num_str = &clean[start + 1..end];
            num_str.parse::<usize>().map_err(|e| e.to_string())
        } else {
            Err("Format qubit index tidak valid".to_string())
        }
    } else {
        clean.parse::<usize>().map_err(|e| e.to_string())
    }
}
