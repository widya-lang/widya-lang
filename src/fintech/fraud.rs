use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FdsDecision {
    Approve,
    Challenge, // Memerlukan 3D-Secure / OTP tambahan
    Reject,    // Blokir transaksi langsung
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdsRuleResult {
    pub rule_name: String,
    pub triggered: bool,
    pub score_impact: u32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdsAssessment {
    pub total_risk_score: u32, // Rentang 0 - 100
    pub decision: FdsDecision,
    pub rules: Vec<FdsRuleResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRiskContext {
    pub pan: Option<String>,
    pub amount_idr: i64,
    pub ip_address: String,
    pub user_agent: String,
    pub velocity_tx_count_last_10m: u32,
    pub failed_attempts_last_hour: u32,
    pub is_vpn_or_proxy: bool,
    pub card_country: Option<String>,
    pub merchant_country: String,
}

/// Evaluasi Risiko Transaksi dengan Engine Anti-Fraud Terintegrasi
pub fn evaluasi_risiko_transaksi(ctx: &TransactionRiskContext) -> FdsAssessment {
    let mut rules = Vec::new();
    let mut total_score = 0u32;

    // Rule 1: High Velocity Spurt (Banyak transaksi dalam 10 menit terakhir)
    if ctx.velocity_tx_count_last_10m >= 5 {
        let impact = if ctx.velocity_tx_count_last_10m >= 10 { 50 } else { 25 };
        rules.push(FdsRuleResult {
            rule_name: "HIGH_VELOCITY_TX".to_string(),
            triggered: true,
            score_impact: impact,
            description: format!("Terdeteksi {} transaksi dalam 10 menit terakhir", ctx.velocity_tx_count_last_10m),
        });
        total_score += impact;
    }

    // Rule 2: Multiple Failed Attempts (Potensi Brute Force Carding / PIN Guessing)
    if ctx.failed_attempts_last_hour >= 3 {
        let impact = 40;
        rules.push(FdsRuleResult {
            rule_name: "BRUTE_FORCE_FAILED_ATTEMPTS".to_string(),
            triggered: true,
            score_impact: impact,
            description: format!("{} kegagalan otentikasi dalam 1 jam terakhir", ctx.failed_attempts_last_hour),
        });
        total_score += impact;
    }

    // Rule 3: Anonymous IP / Proxy / VPN
    if ctx.is_vpn_or_proxy {
        let impact = 20;
        rules.push(FdsRuleResult {
            rule_name: "PROXY_OR_VPN_DETECTED".to_string(),
            triggered: true,
            score_impact: impact,
            description: "Alamat IP teridentifikasi sebagai Open VPN/Tor/Proxy".to_string(),
        });
        total_score += impact;
    }

    // Rule 4: Cross-Border Country Mismatch
    if let Some(card_cc) = &ctx.card_country {
        if !card_cc.eq_ignore_ascii_case(&ctx.merchant_country) {
            let impact = 15;
            rules.push(FdsRuleResult {
                rule_name: "CROSS_BORDER_GEO_MISMATCH".to_string(),
                triggered: true,
                score_impact: impact,
                description: format!("Negara kartu ({}) berbeda dengan negara merchant ({})", card_cc, ctx.merchant_country),
            });
            total_score += impact;
        }
    }

    // Rule 5: Transaksi Nilai Ekstrem (> 50 Juta IDR tanpa verifikasi tambahan)
    if ctx.amount_idr > 50_000_000 {
        let impact = 20;
        rules.push(FdsRuleResult {
            rule_name: "HIGH_TICKET_AMOUNT".to_string(),
            triggered: true,
            score_impact: impact,
            description: format!("Nominal transaksi sangat besar: Rp {}", ctx.amount_idr),
        });
        total_score += impact;
    }

    total_score = total_score.min(100);

    let decision = if total_score >= 65 {
        FdsDecision::Reject
    } else if total_score >= 30 {
        FdsDecision::Challenge
    } else {
        FdsDecision::Approve
    };

    FdsAssessment {
        total_risk_score: total_score,
        decision,
        rules,
    }
}
