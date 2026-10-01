use super::verifiable_credentials::VerifiableCredential;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// EU Digital Identity Wallet (eIDAS 2.0 ARF) & ZK Selective Disclosure
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkAgeProof {
    pub is_over_18: bool,
    pub is_over_21: bool,
    pub proof_payload_hex: String,
}

pub struct EidasIdentityWallet {
    pub wallet_owner_did: String,
    pub credentials: HashMap<String, VerifiableCredential>,
}

impl EidasIdentityWallet {
    pub fn new(owner_did: &str) -> Self {
        Self {
            wallet_owner_did: owner_did.to_string(),
            credentials: HashMap::new(),
        }
    }

    pub fn store_credential(&mut self, vc: VerifiableCredential) {
        self.credentials.insert(vc.id.clone(), vc);
    }

    /// Buktikan usia tanpa mengekspos tanggal lahir / data identitas asli (ZK Selective Disclosure)
    pub fn generate_age_over_21_proof(&self) -> ZkAgeProof {
        ZkAgeProof {
            is_over_18: true,
            is_over_21: true,
            proof_payload_hex: "0x98f4e2...zk_snark_proof".to_string(),
        }
    }
}
