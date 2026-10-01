use serde::{Deserialize, Serialize};

/// ============================================================================
/// Quantitative Finance, Algorithmic Trading Analytics & Risk Engine (VaR & Greeks)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackScholesGreeks {
    pub call_price: f64,
    pub put_price: f64,
    pub delta_call: f64,
    pub delta_put: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta_call: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueAtRiskReport {
    pub confidence_level_pct: f64, // 95.0% or 99.0%
    pub time_horizon_days: u32,
    pub portfolio_value: f64,
    pub parametric_var_amount: f64,
    pub max_expected_loss_pct: f64,
}

pub struct QuantitativeEngine;

impl QuantitativeEngine {
    /// Hitung Volume-Weighted Average Price (VWAP) dari rangkaian transaksi
    pub fn calculate_vwap(trades: &[(f64, f64)]) -> f64 {
        // trades: [(price, volume), ...]
        let mut total_pv = 0.0;
        let mut total_vol = 0.0;

        for &(p, v) in trades {
            total_pv += p * v;
            total_vol += v;
        }

        if total_vol > 0.0 {
            total_pv / total_vol
        } else {
            0.0
        }
    }

    /// Hitung Time-Weighted Average Price (TWAP)
    pub fn calculate_twap(prices: &[f64]) -> f64 {
        if prices.is_empty() {
            return 0.0;
        }
        prices.iter().sum::<f64>() / (prices.len() as f64)
    }

    /// Hitung Order Flow Imbalance (OFI) untuk memprediksi pergerakan harga mikrodetik
    /// OFI = Delta(Bid_Qty) - Delta(Ask_Qty)
    pub fn calculate_order_flow_imbalance(
        prev_bid_p: f64, prev_bid_q: f64, curr_bid_p: f64, curr_bid_q: f64,
        prev_ask_p: f64, prev_ask_q: f64, curr_ask_p: f64, curr_ask_q: f64,
    ) -> f64 {
        let delta_bid = if curr_bid_p > prev_bid_p {
            curr_bid_q
        } else if curr_bid_p == prev_bid_p {
            curr_bid_q - prev_bid_q
        } else {
            0.0
        };

        let delta_ask = if curr_ask_p < prev_ask_p {
            curr_ask_q
        } else if curr_ask_p == prev_ask_p {
            curr_ask_q - prev_ask_q
        } else {
            0.0
        };

        delta_bid - delta_ask
    }

    /// Standard Normal Cumulative Distribution Function approximation (Abramowitz & Stegun)
    fn normal_cdf(x: f64) -> f64 {
        let b1 = 0.319381530;
        let b2 = -0.356563782;
        let b3 = 1.781477937;
        let b4 = -1.821255978;
        let b5 = 1.330274429;
        let p = 0.2316419;
        let c = 0.39894228; // 1 / sqrt(2*pi)

        if x >= 0.0 {
            let t = 1.0 / (1.0 + p * x);
            1.0 - c * (-x * x / 2.0).exp() * t * (t * (t * (t * (t * b5 + b4) + b3) + b2) + b1)
        } else {
            1.0 - Self::normal_cdf(-x)
        }
    }

    /// Pricing Opsi Saham / Valas & Greeks menggunakan Formula Black-Scholes-Merton (1973)
    /// S = Spot Price, K = Strike Price, T = Time to maturity (years), r = Risk-free rate, sigma = Volatility
    pub fn black_scholes_greeks(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> BlackScholesGreeks {
        if t <= 0.0 || sigma <= 0.0 {
            return BlackScholesGreeks {
                call_price: (s - k).max(0.0),
                put_price: (k - s).max(0.0),
                delta_call: if s >= k { 1.0 } else { 0.0 },
                delta_put: if s < k { -1.0 } else { 0.0 },
                gamma: 0.0,
                vega: 0.0,
                theta_call: 0.0,
            };
        }

        let sqrt_t = t.sqrt();
        let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * sqrt_t);
        let d2 = d1 - sigma * sqrt_t;

        let nd1 = Self::normal_cdf(d1);
        let nd2 = Self::normal_cdf(d2);
        let n_neg_d1 = Self::normal_cdf(-d1);
        let n_neg_d2 = Self::normal_cdf(-d2);

        let exp_rt = (-r * t).exp();
        let call = s * nd1 - k * exp_rt * nd2;
        let put = k * exp_rt * n_neg_d2 - s * n_neg_d1;

        let phi_d1 = (-0.5 * d1 * d1).exp() / (2.0 * std::f64::consts::PI).sqrt();
        let gamma = phi_d1 / (s * sigma * sqrt_t);
        let vega = s * sqrt_t * phi_d1 / 100.0; // 1% vol change
        let theta = (-(s * phi_d1 * sigma) / (2.0 * sqrt_t) - r * k * exp_rt * nd2) / 365.0;

        BlackScholesGreeks {
            call_price: call,
            put_price: put,
            delta_call: nd1,
            delta_put: nd1 - 1.0,
            gamma,
            vega,
            theta_call: theta,
        }
    }

    /// Hitung Parametric Value at Risk (VaR 99% / 95% Confidence)
    pub fn calculate_parametric_var(
        portfolio_value: f64,
        daily_volatility_pct: f64,
        confidence_pct: f64,
        days: u32,
    ) -> ValueAtRiskReport {
        // Z-score: 95% -> 1.645, 99% -> 2.326
        let z_score = if confidence_pct >= 99.0 {
            2.3263
        } else {
            1.6448
        };

        let horizon_multiplier = (days as f64).sqrt();
        let var_amount = portfolio_value * (daily_volatility_pct / 100.0) * z_score * horizon_multiplier;
        let loss_pct = (var_amount / portfolio_value) * 100.0;

        ValueAtRiskReport {
            confidence_level_pct: confidence_pct,
            time_horizon_days: days,
            portfolio_value,
            parametric_var_amount: var_amount,
            max_expected_loss_pct: loss_pct,
        }
    }
}
