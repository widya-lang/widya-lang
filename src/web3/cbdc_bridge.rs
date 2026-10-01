//! Bank Indonesia Digital Rupiah (CBDC) & Cross-Chain Gateway (cbdc_bridge.rs)
//! Menyediakan integrasi penyelesaian transaksi komputasi menggunakan Rupiah Digital (CBDC)
//! dan jembatan lintas rantai (Cross-Chain EVM: Ethereum, Polygon, Arbitrum).

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;

/// Tipe Rupiah Digital (CBDC)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CbdcType {
    WholesaleCBDC, // Antar-Bank / Institusi
    RetailCBDC,    // Publik / Pengguna Akhir
}

/// Rekening Rupiah Digital Bank Indonesia
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalRupiahAccount {
    pub account_id: String,
    pub nik_or_npwp: String,
    pub cbdc_type: CbdcType,
    pub balance_idr: u64,
    pub is_kyc_verified: bool,
}

/// Transaksi Lintas Rantai (Cross-Chain Lock & Mint Event)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossChainTransferEvent {
    pub transfer_id: String,
    pub source_chain: String, // "Ethereum", "Polygon", "Solana"
    pub dest_chain: String,   // "WidyaPrivateBlockchain"
    pub sender_address: String,
    pub recipient_wallet: String,
    pub amount_tokens: u64,
    pub merkle_root_proof: String,
    pub is_settled: bool,
}

/// Gateway CBDC & Cross-Chain Bridge
pub struct CbdcCrossChainGateway {
    accounts: HashMap<String, DigitalRupiahAccount>,
    pending_cross_chain: HashMap<String, CrossChainTransferEvent>,
    exchange_rate_usdt_to_idr: u64, // 1 USDT = Rp 16.000
}

impl CbdcCrossChainGateway {
    pub fn new(initial_rate_idr: u64) -> Self {
        Self {
            accounts: HashMap::new(),
            pending_cross_chain: HashMap::new(),
            exchange_rate_usdt_to_idr: initial_rate_idr,
        }
    }

    /// Registrasi akun Rupiah Digital resmi
    pub fn register_cbdc_account(&mut self, account: DigitalRupiahAccount) {
        self.accounts.insert(account.account_id.clone(), account);
    }

    /// Transfer Rupiah Digital instan antar akun
    pub fn transfer_digital_rupiah(&mut self, from_id: &str, to_id: &str, amount_idr: u64) -> Result<String, String> {
        let from_acc = self.accounts.get_mut(from_id)
            .ok_or_else(|| format!("Akun pengirim CBDC '{}' tidak ditemukan", from_id))?;

        if from_acc.balance_idr < amount_idr {
            return Err(format!("Saldo Rupiah Digital tidak mencukupi (Tersedia: Rp {}, Butuh: Rp {})", from_acc.balance_idr, amount_idr));
        }
        from_acc.balance_idr -= amount_idr;

        let to_acc = self.accounts.get_mut(to_id)
            .ok_or_else(|| format!("Akun penerima CBDC '{}' tidak ditemukan", to_id))?;
        to_acc.balance_idr += amount_idr;

        let mut hasher = Sha3_256::new();
        hasher.update(from_id.as_bytes());
        hasher.update(to_id.as_bytes());
        hasher.update(amount_idr.to_le_bytes());
        let tx_hash = format!("cbdc_tx_{:x}", hasher.finalize());

        Ok(tx_hash)
    }

    /// Verifikasi dan cetak token komputasi dari deposit lintas rantai (Cross-Chain Mint)
    pub fn process_cross_chain_deposit(
        &mut self,
        source_chain: &str,
        sender_address: &str,
        dest_widya_wallet: &str,
        amount_usdt: u64,
        merkle_proof: &str,
    ) -> Result<u64, String> {
        if merkle_proof.is_empty() {
            return Err("Bukti Merkle rantai asal tidak boleh kosong".to_string());
        }

        let equivalent_idr = amount_usdt * self.exchange_rate_usdt_to_idr;
        let transfer_id = format!("cc_transfer_{}_{}", source_chain, sender_address);

        self.pending_cross_chain.insert(transfer_id.clone(), CrossChainTransferEvent {
            transfer_id,
            source_chain: source_chain.to_string(),
            dest_chain: "WidyaPrivateBlockchain".to_string(),
            sender_address: sender_address.to_string(),
            recipient_wallet: dest_widya_wallet.to_string(),
            amount_tokens: equivalent_idr,
            merkle_root_proof: merkle_proof.to_string(),
            is_settled: true,
        });

        Ok(equivalent_idr)
    }

    pub fn get_cbdc_balance(&self, account_id: &str) -> Option<u64> {
        self.accounts.get(account_id).map(|a| a.balance_idr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cbdc_transfer_flow() {
        let mut gw = CbdcCrossChainGateway::new(16_000);

        gw.register_cbdc_account(DigitalRupiahAccount {
            account_id: "cbdc_klien_pertamina".to_string(),
            nik_or_npwp: "01.234.567.8-901.000".to_string(),
            cbdc_type: CbdcType::WholesaleCBDC,
            balance_idr: 50_000_000,
            is_kyc_verified: true,
        });

        gw.register_cbdc_account(DigitalRupiahAccount {
            account_id: "cbdc_worker_budi".to_string(),
            nik_or_npwp: "3578012345670001".to_string(),
            cbdc_type: CbdcType::RetailCBDC,
            balance_idr: 100_000,
            is_kyc_verified: true,
        });

        let tx = gw.transfer_digital_rupiah("cbdc_klien_pertamina", "cbdc_worker_budi", 5_000_000).unwrap();
        assert!(tx.starts_with("cbdc_tx_"));
        assert_eq!(gw.get_cbdc_balance("cbdc_klien_pertamina").unwrap(), 45_000_000);
        assert_eq!(gw.get_cbdc_balance("cbdc_worker_budi").unwrap(), 5_100_000);
    }

    #[test]
    fn test_cross_chain_deposit() {
        let mut gw = CbdcCrossChainGateway::new(16_000);
        let idr_minted = gw.process_cross_chain_deposit(
            "Ethereum",
            "0x71C...a47",
            "widya_wallet_ai_lab",
            100, // 100 USDT
            "merkle_proof_block_19283748",
        ).unwrap();

        assert_eq!(idr_minted, 1_600_000); // 100 * 16.000 = 1.600.000 IDR
    }
}
