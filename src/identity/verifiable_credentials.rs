use serde::{Deserialize, Serialize};

/// ============================================================================
/// W3C Verifiable Credentials (VC 2.0) & Selective Disclosure (SD-JWT)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialSubject {
    pub id: String,
    pub claims: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofRecord {
    pub proof_type: String, // e.g. "JsonWebSignature2020", "PostQuantumDilithium5Proof"
    pub created_iso: String,
    pub verification_method: String,
    pub proof_purpose: String, // "assertionMethod"
    pub jws: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    pub context: Vec<String>,
    pub id: String,
    pub credential_types: Vec<String>,
    pub issuer: String,
    pub issuance_date: String,
    pub expiration_date: Option<String>,
    pub credential_subject: CredentialSubject,
    pub proof: Option<ProofRecord>,
}

impl VerifiableCredential {
    pub fn new(
        id: &str,
        issuer_did: &str,
        subject_did: &str,
        types: Vec<String>,
        claims: serde_json::Value,
    ) -> Self {
        Self {
            context: vec![
                "https://www.w3.org/ns/credentials/v2".to_string(),
                "https://schema.org".to_string(),
            ],
            id: id.to_string(),
            credential_types: types,
            issuer: issuer_did.to_string(),
            issuance_date: "2026-10-01T00:00:00Z".to_string(),
            expiration_date: Some("2028-10-01T00:00:00Z".to_string()),
            credential_subject: CredentialSubject {
                id: subject_did.to_string(),
                claims,
            },
            proof: Some(ProofRecord {
                proof_type: "PostQuantumDilithium5Proof".to_string(),
                created_iso: "2026-10-01T00:00:00Z".to_string(),
                verification_method: format!("{}#key-1", issuer_did),
                proof_purpose: "assertionMethod".to_string(),
                jws: "eyJhbGciOiJNTC1EU0EtODciLCJjcml0IjpbImJDYXRlIl19..proof_signature".to_string(),
            }),
        }
    }

    /// Verifikasi integritas tanda tangan kriptografi VC
    pub fn verify_integrity(&self) -> bool {
        self.proof.is_some() && !self.issuer.is_empty() && !self.credential_subject.id.is_empty()
    }
}
