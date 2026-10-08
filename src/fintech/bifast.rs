// ============================================================================
// WIDYA BI-FAST NATIONAL SETTLEMENT & PROXY ENGINE (src/fintech/bifast.rs)
// ============================================================================
// Standar BI-FAST (Bank Indonesia Fast Payment) Skala Nasional:
// - Resolusi Proksi Pembayaran (Nomor HP / Email / NIK ke Nomor Rekening)
// - Routing Setelmen Real-Time ISO 20022 (pacs.008 & pacs.002)
// - Skema Pembagian Biaya Transaksi Resmi (Fee Split Rp 2.500)
// - SLA Verifikasi Waktu-Nyata (< 2 Detik)
// ============================================================================

use crate::compiler::decimal::Desimal;
use crate::fintech::iso20022::{Iso20022Pacs002, Iso20022Pacs008};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyType {
    MobilePhone,
    Email,
    NationalId, // NIK
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyAccount {
    pub proxy_value: String,
    pub proxy_type: ProxyType,
    pub bank_bic: String,
    pub bank_name: String,
    pub account_number: String,
    pub account_holder_name: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiFastStatus {
    Pending,
    AcceptedSettled, // ACTC
    Rejected,        // RJCT
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiFastTransferRequest {
    pub end_to_end_id: String,
    pub sender_account: String,
    pub sender_name: String,
    pub sender_bank_bic: String,
    pub target_proxy_or_account: String,
    pub amount: Desimal,
    pub purpose_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiFastTransferReceipt {
    pub transaction_id: String,
    pub end_to_end_id: String,
    pub status: BiFastStatus,
    pub status_code: String,
    pub status_reason: String,
    pub amount: Desimal,
    pub total_fee: Desimal,
    pub recipient_bank_fee: Desimal,
    pub central_bank_fee: Desimal,
    pub recipient_name: String,
    pub recipient_bank_name: String,
    pub iso_pacs008_xml: String,
    pub timestamp_ms: u64,
}

pub struct BiFastRouter {
    proxies: HashMap<String, ProxyAccount>,
}

impl BiFastRouter {
    pub fn new() -> Self {
        let mut router = Self {
            proxies: HashMap::new(),
        };

        // Daftarkan contoh rekening proksi default
        router.register_proxy(ProxyAccount {
            proxy_value: "081234567890".to_string(),
            proxy_type: ProxyType::MobilePhone,
            bank_bic: "WIDYIDJA".to_string(),
            bank_name: "Bank Widya Indonesia".to_string(),
            account_number: "9876543210".to_string(),
            account_holder_name: "Dr. Budi Santoso".to_string(),
            is_active: true,
        });

        router.register_proxy(ProxyAccount {
            proxy_value: "koperasi@merdeka.id".to_string(),
            proxy_type: ProxyType::Email,
            bank_bic: "BINAIDJA".to_string(),
            bank_name: "Bank Digital Merdeka".to_string(),
            account_number: "1122334455".to_string(),
            account_holder_name: "Koperasi Unit Desa Merdeka".to_string(),
            is_active: true,
        });

        router
    }

    /// Mendaftarkan atau memperbarui entri proksi nasabah
    pub fn register_proxy(&mut self, proxy: ProxyAccount) {
        self.proxies.insert(proxy.proxy_value.clone(), proxy);
    }

    /// Melakukan pencarian resolusi proksi menjadi nomor rekening dan bank tujuan
    pub fn resolve_proxy(&self, query: &str) -> Option<ProxyAccount> {
        self.proxies.get(query).filter(|p| p.is_active).cloned()
    }

    /// Memproses transfer BI-FAST real-time secara instan
    pub fn execute_transfer(&self, req: BiFastTransferRequest) -> Result<BiFastTransferReceipt, String> {
        let resolved = self.resolve_proxy(&req.target_proxy_or_account);

        let (target_bank_bic, target_acc, target_name, target_bank_name) = match resolved {
            Some(proxy) => (proxy.bank_bic, proxy.account_number, proxy.account_holder_name, proxy.bank_name),
            None => {
                // Asumsikan target langsung berupa nomor rekening umum jika bukan proksi
                ("BNIAIDJA".to_string(), req.target_proxy_or_account.clone(), "Penerima Langsung".to_string(), "Bank Mitra BI-FAST".to_string())
            }
        };

        // Buat pesan ISO 20022 pacs.008 Credit Transfer
        let pacs008 = Iso20022Pacs008::new(
            &format!("MSG-{}", req.end_to_end_id),
            &req.end_to_end_id,
            req.amount,
            &req.sender_name,
            &req.sender_account,
            &target_name,
            &target_acc,
        );

        let iso_xml = pacs008.to_xml();

        // Validasi batas minimum transaksi nominal
        if req.amount <= Desimal::dari_int(0) {
            return Err("Nominal transfer BI-FAST harus lebih besar dari 0".to_string());
        }

        // Simulasi respon clearing BI-FAST (pacs.002 ACTC)
        let total_fee = Desimal::dari_int(2500);
        let recipient_bank_fee = Desimal::dari_int(2000);
        let central_bank_fee = Desimal::dari_int(500);

        let tx_id = format!("BIFAST-{}", req.end_to_end_id);
        let pacs002 = Iso20022Pacs002::new_accepted(&format!("STS-{}", tx_id), &tx_id);

        let status = if pacs002.tx_status == "ACTC" {
            BiFastStatus::AcceptedSettled
        } else {
            BiFastStatus::Rejected
        };

        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        Ok(BiFastTransferReceipt {
            transaction_id: tx_id,
            end_to_end_id: req.end_to_end_id,
            status,
            status_code: pacs002.tx_status,
            status_reason: "Settlement Successful via Bank Indonesia Fast Payment".to_string(),
            amount: req.amount,
            total_fee,
            recipient_bank_fee,
            central_bank_fee,
            recipient_name: target_name,
            recipient_bank_name: target_bank_name,
            iso_pacs008_xml: iso_xml,
            timestamp_ms: now_ms,
        })
    }
}

impl Default for BiFastRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bifast_proxy_resolution() {
        let router = BiFastRouter::new();
        let p_hp = router.resolve_proxy("081234567890").expect("Nomor HP harus ditemukan");
        assert_eq!(p_hp.account_holder_name, "Dr. Budi Santoso");
        assert_eq!(p_hp.bank_bic, "WIDYIDJA");

        let p_email = router.resolve_proxy("koperasi@merdeka.id").expect("Email harus ditemukan");
        assert_eq!(p_email.bank_name, "Bank Digital Merdeka");

        assert!(router.resolve_proxy("tidak_ada@domain.com").is_none());
    }

    #[test]
    fn test_bifast_execute_transfer_lifecycle() {
        let router = BiFastRouter::new();

        let req = BiFastTransferRequest {
            end_to_end_id: "E2E-20261007-001".to_string(),
            sender_account: "1234567890".to_string(),
            sender_name: "Siti Rahmawati".to_string(),
            sender_bank_bic: "WIDYIDJA".to_string(),
            target_proxy_or_account: "081234567890".to_string(),
            amount: Desimal::dari_int(1_500_000),
            purpose_code: "CASH".to_string(),
        };

        let receipt = router.execute_transfer(req).expect("Transfer BI-FAST harus sukses");
        assert_eq!(receipt.status, BiFastStatus::AcceptedSettled);
        assert_eq!(receipt.status_code, "ACTC");
        assert_eq!(receipt.total_fee, Desimal::dari_int(2500));
        assert_eq!(receipt.recipient_name, "Dr. Budi Santoso");
        assert!(receipt.iso_pacs008_xml.contains("<FIToFICstmrCdtTrf>"));
        assert!(receipt.iso_pacs008_xml.contains("1500000"));
    }
}
