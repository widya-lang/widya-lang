use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// Multi-Currency Forex Cross Rate Matrix & Pip/Spread Engine
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FxQuote {
    pub base_currency: String,  // e.g. "EUR", "USD", "GBP", "IDR", "JPY"
    pub quote_currency: String, // e.g. "USD", "JPY", "IDR"
    pub bid_price: f64,
    pub ask_price: f64,
    pub pip_size: f64,          // e.g. 0.0001 for EUR/USD, 0.01 for USD/JPY, 1.0 for USD/IDR
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarryTradeAnalysis {
    pub currency_pair: String,
    pub base_interest_rate_pct: f64,
    pub quote_interest_rate_pct: f64,
    pub net_carry_yield_pct: f64,
    pub recommended_action: String,
}

pub struct ForexMatrixEngine {
    pub rates: HashMap<String, FxQuote>, // Key: "EUR/USD", "USD/IDR", etc.
    pub central_bank_rates: HashMap<String, f64>, // Policy Interest Rates (BI, Fed, ECB, BOJ)
}

impl ForexMatrixEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            rates: HashMap::new(),
            central_bank_rates: HashMap::new(),
        };

        // Policy interest rates baseline (2026)
        engine.central_bank_rates.insert("IDR".to_string(), 6.00); // Bank Indonesia BI-Rate
        engine.central_bank_rates.insert("USD".to_string(), 4.50); // US Fed Funds
        engine.central_bank_rates.insert("EUR".to_string(), 3.25); // ECB Refi Rate
        engine.central_bank_rates.insert("JPY".to_string(), 0.50); // Bank of Japan

        // Initial default FX majors & IDR
        engine.update_quote("EUR", "USD", 1.0850, 1.0852, 0.0001);
        engine.update_quote("USD", "IDR", 15850.0, 15865.0, 1.0);
        engine.update_quote("USD", "JPY", 152.30, 152.34, 0.01);
        engine.update_quote("GBP", "USD", 1.3020, 1.3023, 0.0001);

        engine
    }

    pub fn update_quote(&mut self, base: &str, quote: &str, bid: f64, ask: f64, pip_size: f64) {
        let pair = format!("{}/{}", base, quote);
        self.rates.insert(pair, FxQuote {
            base_currency: base.to_string(),
            quote_currency: quote.to_string(),
            bid_price: bid,
            ask_price: ask,
            pip_size,
        });
    }

    /// Hitung Spread dalam satuan Pips
    pub fn calculate_spread_pips(&self, base: &str, quote: &str) -> Option<f64> {
        let pair = format!("{}/{}", base, quote);
        self.rates.get(&pair).map(|q| (q.ask_price - q.bid_price) / q.pip_size)
    }

    /// Menghitung Nilai Triangular Cross Rate otomatis (misal: EUR/IDR dari EUR/USD dan USD/IDR)
    pub fn calculate_triangular_cross(&self, base: &str, quote: &str) -> Option<(f64, f64)> {
        let direct_pair = format!("{}/{}", base, quote);
        if let Some(q) = self.rates.get(&direct_pair) {
            return Some((q.bid_price, q.ask_price));
        }

        // Coba sintesis via USD pivot
        let base_usd_pair = format!("{}/USD", base);
        let usd_quote_pair = format!("USD/{}", quote);

        if let (Some(q1), Some(q2)) = (self.rates.get(&base_usd_pair), self.rates.get(&usd_quote_pair)) {
            let synth_bid = q1.bid_price * q2.bid_price;
            let synth_ask = q1.ask_price * q2.ask_price;
            return Some((synth_bid, synth_ask));
        }

        None
    }

    /// Analisis Carry Trade Imbal Hasil Selisih Suku Bunga (Interest Rate Differential)
    pub fn analyze_carry_trade(&self, base: &str, quote: &str) -> Option<CarryTradeAnalysis> {
        let r_base = self.central_bank_rates.get(base)?;
        let r_quote = self.central_bank_rates.get(quote)?;
        let net_yield = r_base - r_quote;

        let action = if net_yield > 2.0 {
            "Strong Positive Carry (Long Base / Short Quote)".to_string()
        } else if net_yield < -2.0 {
            "Negative Carry (Risk of Carry Bleed - Hedging Recommended)".to_string()
        } else {
            "Neutral Carry Differential".to_string()
        };

        Some(CarryTradeAnalysis {
            currency_pair: format!("{}/{}", base, quote),
            base_interest_rate_pct: *r_base,
            quote_interest_rate_pct: *r_quote,
            net_carry_yield_pct: net_yield,
            recommended_action: action,
        })
    }
}
