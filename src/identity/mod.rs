pub mod did_resolver;
pub mod eidas_wallet;
pub mod icao_passport;
pub mod verifiable_credentials;

pub use did_resolver::{DidDocument, DidResolver, VerificationMethod};
pub use eidas_wallet::{EidasIdentityWallet, ZkAgeProof};
pub use icao_passport::{IcaoPassportReader, MrzData};
pub use verifiable_credentials::{CredentialSubject, ProofRecord, VerifiableCredential};
