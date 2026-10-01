use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Asset,     // Kas, Bank, Settlement Receivable
    Liability, // Saldo Merchant Tersimpan, Hutang MDR
    Equity,    // Modal Pembayaran
    Revenue,   // Pendapatan Platform / Fee MDR
    Expense,   // Biaya Switching / Bank Fee / PPN
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType {
    Debit,
    Credit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub account_id: String,
    pub entry_type: EntryType,
    pub amount_cents: i64, // Nilai dalam satuan integer terkecil (sen/rupiah) tanpa float
    pub currency: String,
    pub memo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerTransaction {
    pub transaction_id: String,
    pub reference_id: String, // e.g. ID Pembayaran / Invoice
    pub timestamp: String,
    pub description: String,
    pub entries: Vec<JournalEntry>,
}

impl LedgerTransaction {
    pub fn new(tx_id: &str, ref_id: &str, description: &str) -> Self {
        Self {
            transaction_id: tx_id.to_string(),
            reference_id: ref_id.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            description: description.to_string(),
            entries: Vec::new(),
        }
    }

    pub fn add_debit(&mut self, account_id: &str, amount_cents: i64, currency: &str, memo: &str) -> &mut Self {
        self.entries.push(JournalEntry {
            account_id: account_id.to_string(),
            entry_type: EntryType::Debit,
            amount_cents,
            currency: currency.to_string(),
            memo: memo.to_string(),
        });
        self
    }

    pub fn add_credit(&mut self, account_id: &str, amount_cents: i64, currency: &str, memo: &str) -> &mut Self {
        self.entries.push(JournalEntry {
            account_id: account_id.to_string(),
            entry_type: EntryType::Credit,
            amount_cents,
            currency: currency.to_string(),
            memo: memo.to_string(),
        });
        self
    }

    /// Memvalidasi keseimbangan buku besar akuntansi ganda: Sum(Debit) == Sum(Credit)
    pub fn validate_balance(&self) -> Result<(), String> {
        if self.entries.is_empty() {
            return Err("Transaksi buku besar tidak boleh kosong".to_string());
        }

        let mut total_debit: i64 = 0;
        let mut total_credit: i64 = 0;

        for entry in &self.entries {
            if entry.amount_cents <= 0 {
                return Err(format!("Nominal jurnal harus lebih dari 0 pada akun {}", entry.account_id));
            }
            match entry.entry_type {
                EntryType::Debit => total_debit += entry.amount_cents,
                EntryType::Credit => total_credit += entry.amount_cents,
            }
        }

        if total_debit != total_credit {
            return Err(format!(
                "Buku besar tidak seimbang! Total Debit ({}) != Total Kredit ({})",
                total_debit, total_credit
            ));
        }

        Ok(())
    }
}

/// Model Perhitungan Fee Split & MDR Pembayaran
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeSplitResult {
    pub gross_amount: i64,      // Total bayar pelanggan
    pub mdr_percent_rate: f64,  // Persentase MDR (e.g. 0.007 untuk 0.7%)
    pub mdr_fixed_fee: i64,     // Biaya tetap (e.g. 2000 untuk VA)
    pub total_fee: i64,         // MDR Nominal + Fixed Fee
    pub vat_tax: i64,           // PPN 11% dari Fee Platform
    pub platform_net_revenue: i64, // Pendapatan bersih gateway
    pub bank_switching_fee: i64,   // Biaya yang dibayarkan ke Bank / Acquirer
    pub merchant_net_payout: i64,  // Saldo bersih yang masuk ke merchant
}

/// Menghitung pembagian fee (Fee Split Engine)
pub fn hitung_fee_split(
    gross_amount: i64,
    mdr_percent_rate: f64,
    mdr_fixed_fee: i64,
    bank_cost_fee: i64,
) -> FeeSplitResult {
    let mdr_nominal = ((gross_amount as f64) * mdr_percent_rate).round() as i64;
    let total_fee = mdr_nominal + mdr_fixed_fee;
    
    // PPN 11% dari fee platform
    let vat_tax = ((total_fee as f64) * 0.11).round() as i64;
    let merchant_net_payout = gross_amount - total_fee;
    let platform_net_revenue = total_fee - bank_cost_fee - vat_tax;

    FeeSplitResult {
        gross_amount,
        mdr_percent_rate,
        mdr_fixed_fee,
        total_fee,
        vat_tax,
        platform_net_revenue: platform_net_revenue.max(0),
        bank_switching_fee: bank_cost_fee,
        merchant_net_payout: merchant_net_payout.max(0),
    }
}

/// Otomatisasi pembentukan Jurnal Akuntansi Ganda dari sebuah Pembayaran Berhasil
pub fn buat_jurnal_pembayaran_sukses(
    tx_id: &str,
    invoice_id: &str,
    merchant_id: &str,
    fee_split: &FeeSplitResult,
) -> Result<LedgerTransaction, String> {
    let mut tx = LedgerTransaction::new(tx_id, invoice_id, "Penerimaan Pembayaran Pelanggan");

    // 1. Kas/Bank bertambah (Debit Asset) sebesar total gross
    tx.add_debit("1010-KAS-BANK-ESCROW", fee_split.gross_amount, "IDR", "Penerimaan dana escrow bank");

    // 2. Kewajiban ke Merchant bertambah (Credit Liability) sebesar net payout
    tx.add_credit(
        &format!("2010-HUTANG-MERCHANT-{}", merchant_id),
        fee_split.merchant_net_payout,
        "IDR",
        "Kewajiban saldo payout merchant",
    );

    // 3. Pendapatan Fee Platform (Credit Revenue)
    let gross_platform_revenue = fee_split.total_fee - fee_split.bank_switching_fee;
    if gross_platform_revenue > 0 {
        tx.add_credit(
            "4010-PENDAPATAN-MDR-GATEWAY",
            gross_platform_revenue,
            "IDR",
            "Pendapatan MDR gateway",
        );
    }

    // 4. Hutang Biaya Switching Bank (Credit Liability)
    if fee_split.bank_switching_fee > 0 {
        tx.add_credit(
            "2020-HUTANG-SWITCHING-BANK",
            fee_split.bank_switching_fee,
            "IDR",
            "Hutang switching/interchange bank",
        );
    }

    tx.validate_balance()?;
    Ok(tx)
}
