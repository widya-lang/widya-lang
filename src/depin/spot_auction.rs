//! Dynamic Spot Market & Compute Auction Engine for DePIN Grid (spot_auction.rs)
//! Menyediakan bursa penetapan harga komputasi real-time berbasis Continuous Double Auction dan AMM.

use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Tipe Order di Bursa Komputasi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Bid, // Permintaan sewa dari Klien (beli daya)
    Ask, // Penawaran daya dari Worker Node (jual daya)
}

/// Order Komputasi di Buku Order (Order Book)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputeOrder {
    pub order_id: String,
    pub participant_wallet: String,
    pub order_type: OrderType,
    pub tflops_amount: f64,
    pub price_per_tflops_sec_idr: f64, // Harga per TFLOPS per detik dalam IDR
    pub timestamp: u64,
}

impl Eq for ComputeOrder {}

impl PartialOrd for ComputeOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComputeOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        // Untuk Bid: Prioritas harga tertinggi
        // Untuk Ask: Prioritas harga terendah
        match self.order_type {
            OrderType::Bid => self.price_per_tflops_sec_idr
                .partial_cmp(&other.price_per_tflops_sec_idr)
                .unwrap_or(Ordering::Equal),
            OrderType::Ask => other.price_per_tflops_sec_idr
                .partial_cmp(&self.price_per_tflops_sec_idr)
                .unwrap_or(Ordering::Equal),
        }
    }
}

/// Hasil Pencocokan Transaksi (Matched Trade)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchedTrade {
    pub trade_id: String,
    pub bid_order_id: String,
    pub ask_order_id: String,
    pub client_wallet: String,
    pub worker_wallet: String,
    pub matched_tflops: f64,
    pub settlement_price_idr: f64,
}

/// Mesin Bursa Komputasi DePIN
pub struct SpotComputeAuctionEngine {
    bids: BinaryHeap<ComputeOrder>,
    asks: BinaryHeap<ComputeOrder>,
    base_price_idr: f64,
    total_trades_count: usize,
}

impl SpotComputeAuctionEngine {
    pub fn new(base_price: f64) -> Self {
        Self {
            bids: BinaryHeap::new(),
            asks: BinaryHeap::new(),
            base_price_idr: base_price,
            total_trades_count: 0,
        }
    }

    /// Menambahkan Order baru ke Buku Order
    pub fn place_order(&mut self, order: ComputeOrder) {
        match order.order_type {
            OrderType::Bid => self.bids.push(order),
            OrderType::Ask => self.asks.push(order),
        }
    }

    /// Hitung Harga Spot Dinamis Berdasarkan Utilisasi Jaringan (AMM Model)
    pub fn calculate_dynamic_spot_price(&self, online_supply_tflops: f64, demand_tflops: f64) -> f64 {
        if online_supply_tflops <= 0.0 {
            return self.base_price_idr * 5.0; // Surge maksimum jika supply kosong
        }
        let utilization_ratio = demand_tflops / online_supply_tflops;
        // Formula: Price = Base * (1.0 + (Util^2 * 1.5))
        self.base_price_idr * (1.0 + (utilization_ratio.powi(2) * 1.5))
    }

    /// Eksekusi Pencocokan Order (Order Matching Engine)
    pub fn match_orders(&mut self) -> Vec<MatchedTrade> {
        let mut trades = Vec::new();

        while let (Some(best_bid), Some(best_ask)) = (self.bids.peek(), self.asks.peek()) {
            if best_bid.price_per_tflops_sec_idr >= best_ask.price_per_tflops_sec_idr {
                let mut bid = self.bids.pop().unwrap();
                let mut ask = self.asks.pop().unwrap();

                let matched_tflops = bid.tflops_amount.min(ask.tflops_amount);
                let settlement_price = (bid.price_per_tflops_sec_idr + ask.price_per_tflops_sec_idr) / 2.0;

                self.total_trades_count += 1;
                trades.push(MatchedTrade {
                    trade_id: format!("trade_{}", self.total_trades_count),
                    bid_order_id: bid.order_id.clone(),
                    ask_order_id: ask.order_id.clone(),
                    client_wallet: bid.participant_wallet.clone(),
                    worker_wallet: ask.participant_wallet.clone(),
                    matched_tflops,
                    settlement_price_idr: settlement_price,
                });

                // Jika sisa volume masih ada, masukkan kembali ke antrean
                if bid.tflops_amount > matched_tflops {
                    bid.tflops_amount -= matched_tflops;
                    self.bids.push(bid);
                }
                if ask.tflops_amount > matched_tflops {
                    ask.tflops_amount -= matched_tflops;
                    self.asks.push(ask);
                }
            } else {
                break; // Tidak ada harga yang cocok (Spread belum terpenuhi)
            }
        }

        trades
    }

    pub fn bid_depth(&self) -> usize {
        self.bids.len()
    }

    pub fn ask_depth(&self) -> usize {
        self.asks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spot_market_matching_and_amm_pricing() {
        let mut engine = SpotComputeAuctionEngine::new(10.0); // Base Rp 10/TFLOPS-sec

        // Hitung dynamic spot price pada utilisasi 80% (80 demand / 100 supply)
        let spot_price = engine.calculate_dynamic_spot_price(100.0, 80.0);
        assert!((spot_price - 19.6).abs() < 0.1);

        // Klien menawar beli Rp 15 untuk 50 TFLOPS
        engine.place_order(ComputeOrder {
            order_id: "bid_01".to_string(),
            participant_wallet: "client_wallet".to_string(),
            order_type: OrderType::Bid,
            tflops_amount: 50.0,
            price_per_tflops_sec_idr: 15.0,
            timestamp: 1720000000,
        });

        // Worker menawarkan jual Rp 12 untuk 50 TFLOPS
        engine.place_order(ComputeOrder {
            order_id: "ask_01".to_string(),
            participant_wallet: "worker_wallet".to_string(),
            order_type: OrderType::Ask,
            tflops_amount: 50.0,
            price_per_tflops_sec_idr: 12.0,
            timestamp: 1720000000,
        });

        let trades = engine.match_orders();
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].matched_tflops, 50.0);
        assert_eq!(trades[0].settlement_price_idr, 13.5); // (15 + 12) / 2 = 13.5
        assert_eq!(engine.bid_depth(), 0);
        assert_eq!(engine.ask_depth(), 0);
    }
}
