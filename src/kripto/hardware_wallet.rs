//! Quantum-Safe Hardware Wallet & Air-Gapped Signature Engine (hardware_wallet.rs)
//! Menyediakan penyimpanan kunci privat dingin (Cold Storage), penandatanganan transaksi
//! tanpa koneksi internet (Air-Gapped QR Code), dan verifikasi Post-Quantum ML-DSA (Dilithium-5).

use crate::kripto::pqc::{PqcEngine, PqcKeyPair, PqcSignature};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

/// Amplop Transaksi Air-Gapped (Unsigned / Signed Envelope)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AirGappedTransactionEnvelope {
    pub transaction_id: String,
    pub sender_wallet: String,
    pub recipient_wallet: String,
    pub amount_idr: u64,
    pub fee_idr: u64,
    pub nonce: u64,
    pub signature: Option<PqcSignature>,
    pub is_signed: bool,
}

/// Dompet Kripto Keras (Hardware Cold Wallet)
pub struct SovereignHardwareWallet {
    wallet_address: String,
    keypair: PqcKeyPair,
    pin_hash: String,
    is_unlocked: bool,
}

impl SovereignHardwareWallet {
    /// Inisialisasi dompet dingin dengan entropy seed & proteksi PIN
    pub fn new(seed_entropy: &str, pin: &str) -> Self {
        let keypair = PqcEngine::generate_dilithium5_keypair(seed_entropy);
        let address = format!("widya_pqc_{}", &keypair.public_key_hex[7..23]);

        let mut pin_hasher = Sha3_256::new();
        pin_hasher.update(pin.as_bytes());
        let pin_hash = format!("{:x}", pin_hasher.finalize());

        Self {
            wallet_address: address,
            keypair,
            pin_hash,
            is_unlocked: false,
        }
    }

    /// Buka kunci dompet dingin dengan PIN
    pub fn unlock(&mut self, pin: &str) -> Result<bool, String> {
        let mut pin_hasher = Sha3_256::new();
        pin_hasher.update(pin.as_bytes());
        let input_hash = format!("{:x}", pin_hasher.finalize());

        if input_hash == self.pin_hash {
            self.is_unlocked = true;
            Ok(true)
        } else {
            Err("PIN Hardware Wallet salah!".to_string())
        }
    }

    /// Kunci kembali dompet dingin
    pub fn lock(&mut self) {
        self.is_unlocked = false;
    }

    /// Tandatangani amplop transaksi secara offline (Air-Gapped)
    pub fn sign_air_gapped_transaction(&self, mut envelope: AirGappedTransactionEnvelope) -> Result<AirGappedTransactionEnvelope, String> {
        if !self.is_unlocked {
            return Err("Hardware wallet terkunci. Masukkan PIN terlebih dahulu!".to_string());
        }

        let raw_tx_string = format!(
            "{}:{}:{}:{}:{}",
            envelope.transaction_id, envelope.sender_wallet, envelope.recipient_wallet, envelope.amount_idr, envelope.nonce
        );

        let sig = PqcEngine::sign_document(raw_tx_string.as_bytes(), &self.keypair.private_key_hex);
        envelope.signature = Some(sig);
        envelope.is_signed = true;

        Ok(envelope)
    }

    /// Verifikasi keabsahan tanda tangan amplop transaksi
    pub fn verify_transaction(envelope: &AirGappedTransactionEnvelope, public_key_hex: &str) -> bool {
        if let Some(ref sig) = envelope.signature {
            let raw_tx_string = format!(
                "{}:{}:{}:{}:{}",
                envelope.transaction_id, envelope.sender_wallet, envelope.recipient_wallet, envelope.amount_idr, envelope.nonce
            );
            PqcEngine::verify_signature(raw_tx_string.as_bytes(), sig, public_key_hex)
        } else {
            false
        }
    }

    pub fn wallet_address(&self) -> &str {
        &self.wallet_address
    }

    pub fn public_key(&self) -> &str {
        &self.keypair.public_key_hex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_air_gapped_hardware_wallet_flow() {
        let mut wallet = SovereignHardwareWallet::new("cold_seed_entropy_rahasia_12345", "987654");
        assert!(wallet.wallet_address().starts_with("widya_pqc_"));

        let envelope = AirGappedTransactionEnvelope {
            transaction_id: "tx_airgap_001".to_string(),
            sender_wallet: wallet.wallet_address().to_string(),
            recipient_wallet: "widya_wallet_tujuan".to_string(),
            amount_idr: 25_000_000,
            fee_idr: 5_000,
            nonce: 1,
            signature: None,
            is_signed: false,
        };

        // Gagal jika belum dibuka dengan PIN
        assert!(wallet.sign_air_gapped_transaction(envelope.clone()).is_err());

        // Buka dengan PIN
        assert!(wallet.unlock("987654").unwrap());

        // Tandatangani transaksi secara offline
        let signed_envelope = wallet.sign_air_gapped_transaction(envelope).unwrap();
        assert!(signed_envelope.is_signed);
        assert!(signed_envelope.signature.is_some());

        // Verifikasi keabsahan tanda tangan PQC ML-DSA Dilithium-5
        let is_valid = SovereignHardwareWallet::verify_transaction(&signed_envelope, wallet.public_key());
        assert!(is_valid);
    }
}
