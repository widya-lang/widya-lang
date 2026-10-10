use widya::quantum::pure_quantum::{BellStateType, PureQasmEngine, PureQuantumSim};
use std::f64::consts::PI;

#[test]
fn test_single_qubit_gates_and_reversibility() {
    let mut sim = PureQuantumSim::new(1);
    assert!(sim.is_normalized(1e-12));

    // Initial state |0> -> Prob(0) = 1.0
    let (p0, p1) = sim.qubit_prob(0);
    assert!((p0 - 1.0).abs() < 1e-12);
    assert!(p1 < 1e-12);

    // H gate: Superposition
    sim.h(0);
    let (p0, p1) = sim.qubit_prob(0);
    assert!((p0 - 0.5).abs() < 1e-12);
    assert!((p1 - 0.5).abs() < 1e-12);

    // H again -> returns to |0> (H^2 = I)
    sim.h(0);
    let (p0, _) = sim.qubit_prob(0);
    assert!((p0 - 1.0).abs() < 1e-12);

    // X gate (NOT) -> |1>
    sim.x(0);
    let (p0, p1) = sim.qubit_prob(0);
    assert!(p0 < 1e-12);
    assert!((p1 - 1.0).abs() < 1e-12);

    // Y gate on |1> -> i|0> (Prob 0 = 1.0)
    sim.y(0);
    let (p0, _) = sim.qubit_prob(0);
    assert!((p0 - 1.0).abs() < 1e-12);

    // Z gate on |0> -> |0>
    sim.z(0);
    let (p0, _) = sim.qubit_prob(0);
    assert!((p0 - 1.0).abs() < 1e-12);

    // T and S gate test: T^2 = S, S^2 = Z
    sim.reset();
    sim.x(0); // state is |1>
    sim.t(0);
    sim.t(0); // T^2 = S
    let mut sim2 = PureQuantumSim::new(1);
    sim2.x(0);
    sim2.s(0);
    assert!((sim.fidelity(&sim2) - 1.0).abs() < 1e-12);

    // Rotations RX, RY, RZ
    sim.reset();
    sim.rx(0, PI); // RX(pi) flips |0> to -i|1> (Prob 1 = 1.0)
    let (_, p1) = sim.qubit_prob(0);
    assert!((p1 - 1.0).abs() < 1e-12);
}

#[test]
fn test_bell_states_and_ghz_entanglement() {
    let mut sim = PureQuantumSim::new(2);

    // PhiPlus (|00> + |11>) / sqrt(2)
    sim.generate_bell_state(BellStateType::PhiPlus);
    assert!(sim.is_normalized(1e-12));
    let probs = sim.get_probabilities();
    assert!((probs[0] - 0.5).abs() < 1e-12); // |00>
    assert!(probs[1] < 1e-12);               // |01>
    assert!(probs[2] < 1e-12);               // |10>
    assert!((probs[3] - 0.5).abs() < 1e-12); // |11>

    // PsiPlus (|01> + |10>) / sqrt(2)
    sim.generate_bell_state(BellStateType::PsiPlus);
    let probs = sim.get_probabilities();
    assert!(probs[0] < 1e-12);
    assert!((probs[1] - 0.5).abs() < 1e-12); // |01>
    assert!((probs[2] - 0.5).abs() < 1e-12); // |10>
    assert!(probs[3] < 1e-12);

    // 4-qubit GHZ state (|0000> + |1111>) / sqrt(2)
    let mut ghz_sim = PureQuantumSim::new(4);
    ghz_sim.generate_ghz_state();
    assert!(ghz_sim.is_normalized(1e-12));
    let ghz_probs = ghz_sim.get_probabilities();
    assert!((ghz_probs[0] - 0.5).abs() < 1e-12);   // index 0: |0000>
    assert!((ghz_probs[15] - 0.5).abs() < 1e-12);  // index 15: |1111>
    for i in 1..15 {
        assert!(ghz_probs[i] < 1e-12);
    }
}

#[test]
fn test_toffoli_and_cswap() {
    let mut sim = PureQuantumSim::new(3);

    // Toffoli (CCNOT): c1=0, c2=1, target=2
    // State |000> -> target remains 0
    sim.toffoli(0, 1, 2);
    assert_eq!(sim.get_probabilities()[0], 1.0);

    // Set control 0 to 1 -> State |001> (binary 1)
    sim.reset();
    sim.x(0);
    sim.toffoli(0, 1, 2); // only one control active, target remains 0
    assert_eq!(sim.get_probabilities()[1], 1.0);

    // Set both controls 0 and 1 -> State |011> (binary 3)
    sim.reset();
    sim.x(0);
    sim.x(1);
    sim.toffoli(0, 1, 2); // both controls active! target flipped to 1 -> State |111> (binary 7)
    assert_eq!(sim.get_probabilities()[7], 1.0);

    // CSWAP (Fredkin): control=0, q1=1, q2=2
    // If control is 1, swap qubit 1 and qubit 2
    sim.reset();
    sim.x(0); // control = 1
    sim.x(1); // q1 = 1, q2 = 0 -> state |011> (binary 3)
    sim.cswap(0, 1, 2); // q1 and q2 swap -> state |101> (binary 5)
    assert_eq!(sim.get_probabilities()[5], 1.0);
}

#[test]
fn test_wavefunction_collapse_and_measurement() {
    let mut sim = PureQuantumSim::new(2);
    sim.generate_bell_state(BellStateType::PhiPlus);

    // Measure qubit 0 with random value 0.2 (< 0.5 -> outcome 0)
    let (outcome, prob) = sim.measure_collapse(0, 0.2);
    assert_eq!(outcome, 0);
    assert!((prob - 0.5).abs() < 1e-12);

    // After collapse to 0, entire state MUST be strictly |00>
    assert!(sim.is_normalized(1e-12));
    let probs = sim.get_probabilities();
    assert!((probs[0] - 1.0).abs() < 1e-12);
    assert!(probs[3] < 1e-12);

    // Qubit 1 must now be 0 with 100% probability due to entanglement collapse!
    let (q1_p0, q1_p1) = sim.qubit_prob(1);
    assert!((q1_p0 - 1.0).abs() < 1e-12);
    assert!(q1_p1 < 1e-12);

    // Repeat for outcome 1
    sim.generate_bell_state(BellStateType::PhiPlus);
    let (outcome2, _) = sim.measure_collapse(0, 0.8); // > 0.5 -> outcome 1
    assert_eq!(outcome2, 1);
    let probs2 = sim.get_probabilities();
    assert!((probs2[3] - 1.0).abs() < 1e-12); // strictly |11>
}

#[test]
fn test_qasm_engine_execution() {
    let qasm_src = r#"
        OPENQASM 2.0;
        include "qelib1.inc";
        qreg q[2];
        creg c[2];

        h q[0];
        cx q[0], q[1];
        measure q[0] -> c[0];
        measure q[1] -> c[1];
    "#;

    // Seed 0.1 -> outcome 0 for qubit 0, which collapses qubit 1 to 0 as well
    let res = PureQasmEngine::execute_qasm(qasm_src, &[0.1, 0.1]).expect("QASM execution failed");
    assert_eq!(res.classical_bits[0], 0);
    assert_eq!(res.classical_bits[1], 0);
    assert_eq!(res.measured_states.len(), 2);

    // Seed 0.9 -> outcome 1 for qubit 0, which collapses qubit 1 to 1 as well
    let res2 = PureQasmEngine::execute_qasm(qasm_src, &[0.9, 0.9]).expect("QASM execution failed");
    assert_eq!(res2.classical_bits[0], 1);
    assert_eq!(res2.classical_bits[1], 1);
}
