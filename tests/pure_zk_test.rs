use std::collections::HashMap;
use widya::kripto::pure_zk::{
    FiatShamirZkProver, FieldElement, LinearCombination, PedersenCommitment, PreimageZkProof,
    R1csCircuit,
};

#[test]
fn test_finite_field_arithmetic_and_inversion() {
    let a = FieldElement::new(123456789);
    let b = FieldElement::new(987654321);

    // Addition & Subtraction
    let sum = a.add(b);
    assert_eq!(sum.sub(b), a);

    // Inversion: a * a^-1 == 1
    let inv_a = a.inv().expect("Inversion failed");
    assert_eq!(a.mul(inv_a), FieldElement::ONE);

    // Exponentiation
    let base = FieldElement::new(3);
    assert_eq!(base.pow(4), FieldElement::new(81));
}

#[test]
fn test_r1cs_arithmetic_circuit_satisfaction() {
    // Circuit proving: (x + 5) * y = z
    let mut circuit = R1csCircuit::new();

    // A = 1*x + 5
    let mut a_lc = LinearCombination::from_var("x");
    a_lc.constant = FieldElement::new(5);

    // B = 1*y
    let b_lc = LinearCombination::from_var("y");

    // C = 1*z
    let c_lc = LinearCombination::from_var("z");

    circuit.add_constraint(a_lc, b_lc, c_lc);

    // Valid Witness: x=3, y=4 => (3+5)*4 = 32
    let mut valid_witness = HashMap::new();
    valid_witness.insert("x".to_string(), FieldElement::new(3));
    valid_witness.insert("y".to_string(), FieldElement::new(4));
    valid_witness.insert("z".to_string(), FieldElement::new(32));

    assert!(circuit.is_satisfied(&valid_witness));

    // Invalid Witness: z=33
    let mut invalid_witness = valid_witness.clone();
    invalid_witness.insert("z".to_string(), FieldElement::new(33));
    assert!(!circuit.is_satisfied(&invalid_witness));
}

#[test]
fn test_fiat_shamir_nizk_proof_and_verification() {
    let secret = 9876543210u64;
    let nonce = 123456789u64;

    // 1. Prover generates proof without revealing secret
    let (public_y, proof) = FiatShamirZkProver::prove(secret, nonce);

    // 2. Verifier checks proof
    assert!(FiatShamirZkProver::verify(public_y, &proof));

    // 3. Tampered proof response must fail
    let bad_proof = PreimageZkProof {
        commitment_t: proof.commitment_t,
        response_s: proof.response_s ^ 0x01,
    };
    assert!(!FiatShamirZkProver::verify(public_y, &bad_proof));
}

#[test]
fn test_pedersen_commitment_homomorphism() {
    let v1 = 100u64;
    let r1 = 12345u64;
    let c1 = PedersenCommitment::commit(v1, r1);

    let v2 = 250u64;
    let r2 = 67890u64;
    let c2 = PedersenCommitment::commit(v2, r2);

    // Homomorphic sum
    let v_sum = v1 + v2;
    let r_sum = r1 + r2;
    let c_sum = PedersenCommitment::commit(v_sum, r_sum);

    assert!(PedersenCommitment::verify_sum(c1, c2, c_sum));
}
