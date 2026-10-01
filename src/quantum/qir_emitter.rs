use super::qasm_parser::{QuantumCircuit, QuantumGateOp};

/// ============================================================================
/// Quantum Intermediate Representation (QIR - LLVM Bitcode Emitter)
/// ============================================================================

pub struct QirEmitter;

impl QirEmitter {
    /// Emit QIR / LLVM IR string representation for PyQIR / Azure Quantum / AWS Braket
    pub fn emit_llvm_qir(circuit: &QuantumCircuit, entry_name: &str) -> String {
        let mut out = String::new();
        out.push_str("; ModuleID = 'widya_quantum_module'\n");
        out.push_str("source_filename = \"quantum.widya\"\n");
        out.push_str("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n");
        out.push_str("target triple = \"custom-qir-unknown-unknown\"\n\n");

        out.push_str("%Qubit = type opaque\n");
        out.push_str("%Result = type opaque\n\n");

        // External QIR runtime functions
        out.push_str("declare void @__quantum__qis__h__body(%Qubit*)\n");
        out.push_str("declare void @__quantum__qis__x__body(%Qubit*)\n");
        out.push_str("declare void @__quantum__qis__z__body(%Qubit*)\n");
        out.push_str("declare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\n");
        out.push_str("declare void @__quantum__qis__rz__body(double, %Qubit*)\n");
        out.push_str("declare %Result* @__quantum__qis__mz__body(%Qubit*)\n");
        out.push_str("declare %Qubit* @__quantum__rt__qubit_allocate()\n");
        out.push_str("declare void @__quantum__rt__qubit_release(%Qubit*)\n\n");

        out.push_str(&format!("define void @{}() #0 {{\nentry:\n", entry_name));

        // Allocate qubits
        for i in 0..circuit.num_qubits {
            out.push_str(&format!("  %q{} = call %Qubit* @__quantum__rt__qubit_allocate()\n", i));
        }

        // Emit gates
        for op in &circuit.gates {
            match op {
                QuantumGateOp::Hadamard(q) => {
                    out.push_str(&format!("  call void @__quantum__qis__h__body(%Qubit* %q{})\n", q));
                }
                QuantumGateOp::PauliX(q) => {
                    out.push_str(&format!("  call void @__quantum__qis__x__body(%Qubit* %q{})\n", q));
                }
                QuantumGateOp::PauliZ(q) => {
                    out.push_str(&format!("  call void @__quantum__qis__z__body(%Qubit* %q{})\n", q));
                }
                QuantumGateOp::Cnot(c, t) => {
                    out.push_str(&format!("  call void @__quantum__qis__cnot__body(%Qubit* %q{}, %Qubit* %q{})\n", c, t));
                }
                QuantumGateOp::RotationZ(q, theta) => {
                    out.push_str(&format!("  call void @__quantum__qis__rz__body(double {:e}, %Qubit* %q{})\n", theta, q));
                }
                QuantumGateOp::Measure(q, c) => {
                    out.push_str(&format!("  %r{} = call %Result* @__quantum__qis__mz__body(%Qubit* %q{})\n", c, q));
                }
                _ => {}
            }
        }

        // Release qubits
        for i in 0..circuit.num_qubits {
            out.push_str(&format!("  call void @__quantum__rt__qubit_release(%Qubit* %q{})\n", i));
        }

        out.push_str("  ret void\n}\n\n");
        out.push_str("attributes #0 = { \"entry_point\" \"qir_profiles\"=\"custom\" }\n");
        out
    }
}
