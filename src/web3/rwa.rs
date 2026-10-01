use crate::web3::crypto::{hitung_merkle_root, keccak256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipe Aset Riil (Real World Asset)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RwaAssetType {
    EmasFisik,
    PropertiRealEstate,
    ObligasiSuratBerharga,
    KomoditasPertanian,
    KreditKarbon,
}

/// Token Aset Riil (RWA) Terdesentralisasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwaToken {
    pub name: String,
    pub symbol: String,
    pub asset_type: RwaAssetType,
    pub total_supply: u64,
    pub decimals: u8,
    pub valuation_idr_per_token: u64,
    pub custodian_auditor: String,
    pub proof_of_reserve_root: String,
    pub balances: HashMap<String, u64>,
}

impl RwaToken {
    pub fn new(
        name: &str,
        symbol: &str,
        asset_type: RwaAssetType,
        valuation_idr: u64,
        custodian: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            asset_type,
            total_supply: 0,
            decimals: 18,
            valuation_idr_per_token: valuation_idr,
            custodian_auditor: custodian.to_string(),
            proof_of_reserve_root: String::new(),
            balances: HashMap::new(),
        }
    }

    /// Minting Token Baru Berdasarkan Audit Bukti Cadangan Fisik (Proof of Reserve)
    pub fn mint(&mut self, to_address: &str, amount: u64, audit_serial_numbers: &[&str]) -> Result<(), String> {
        if audit_serial_numbers.is_empty() {
            return Err("Minting RWA membutuhkan minimal 1 nomor seri audit fisik".to_string());
        }

        // Hasilkan Merkle Root dari serial number aset fisik di brankas
        let hashes: Vec<[u8; 32]> = audit_serial_numbers.iter().map(|s| keccak256(s.as_bytes())).collect();
        let root = hitung_merkle_root(&hashes);
        self.proof_of_reserve_root = format!("0x{}", hex::encode(root));

        let current = self.balances.entry(to_address.to_string()).or_insert(0);
        *current += amount;
        self.total_supply += amount;

        Ok(())
    }

    /// Transfer Kepemilikan Token Aset Riil
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let sender_bal = self.balances.get_mut(from).ok_or_else(|| "Pengirim tidak memiliki saldo".to_string())?;
        if *sender_bal < amount {
            return Err(format!("Saldo tidak cukup: punya {}, butuh {}", *sender_bal, amount));
        }
        *sender_bal -= amount;

        let receiver_bal = self.balances.entry(to.to_string()).or_insert(0);
        *receiver_bal += amount;

        Ok(())
    }

    pub fn balance_of(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }

    /// Hitung Total Valuasi Portofolio Fisik dalam Rupiah
    pub fn total_market_cap_idr(&self) -> u64 {
        self.total_supply.saturating_mul(self.valuation_idr_per_token)
    }
}
