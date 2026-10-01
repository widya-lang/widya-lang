use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// W3C Decentralized Identifier (DID) Core 1.0 Resolution Engine
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    pub method_type: String, // e.g. "JsonWebKey2020", "Ed25519VerificationKey2020"
    pub controller: String,
    pub public_key_jwk: Option<serde_json::Value>,
    pub public_key_multibase: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidDocument {
    pub id: String,
    pub context: Vec<String>,
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    pub assertion_method: Vec<String>,
    pub service_endpoints: Vec<serde_json::Value>,
}

pub struct DidResolver {
    pub registry: HashMap<String, DidDocument>,
}

impl DidResolver {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    /// Resolve string DID ke W3C DID Document
    pub fn resolve(&self, did: &str) -> Result<DidDocument, String> {
        if let Some(doc) = self.registry.get(did) {
            return Ok(doc.clone());
        }

        if did.starts_with("did:widya:") {
            let identifier = &did[10..];
            let key_id = format!("{}#key-1", did);
            let doc = DidDocument {
                id: did.to_string(),
                context: vec!["https://www.w3.org/ns/did/v1".to_string()],
                verification_method: vec![VerificationMethod {
                    id: key_id.clone(),
                    method_type: "Ed25519VerificationKey2020".to_string(),
                    controller: did.to_string(),
                    public_key_jwk: None,
                    public_key_multibase: Some(format!("z{}", identifier)),
                }],
                authentication: vec![key_id.clone()],
                assertion_method: vec![key_id],
                service_endpoints: Vec::new(),
            };
            Ok(doc)
        } else if did.starts_with("did:key:") {
            let key_id = format!("{}#{}", did, &did[8..]);
            let doc = DidDocument {
                id: did.to_string(),
                context: vec!["https://www.w3.org/ns/did/v1".to_string()],
                verification_method: vec![VerificationMethod {
                    id: key_id.clone(),
                    method_type: "Ed25519VerificationKey2020".to_string(),
                    controller: did.to_string(),
                    public_key_jwk: None,
                    public_key_multibase: Some(did[8..].to_string()),
                }],
                authentication: vec![key_id.clone()],
                assertion_method: vec![key_id],
                service_endpoints: Vec::new(),
            };
            Ok(doc)
        } else {
            Err(format!("Metode DID '{}' tidak didukung oleh resolver", did))
        }
    }
}
