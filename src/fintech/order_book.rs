use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// ============================================================================
/// L2 / L3 Deterministic Limit Order Book (LOB) Matching Engine (HFT Stock & FX)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Bid, // Beli
    Ask, // Jual
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitOrder {
    pub order_id: u64,
    pub client_id: String,
    pub symbol: String, // e.g., "BBCA.JK", "EUR/USD", "AAPL"
    pub side: OrderSide,
    pub price: f64,
    pub quantity: f64,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeExecution {
    pub trade_id: u64,
    pub symbol: String,
    pub maker_order_id: u64,
    pub taker_order_id: u64,
    pub price: f64,
    pub quantity: f64,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BboSnapshot {
    pub symbol: String,
    pub best_bid_price: Option<f64>,
    pub best_bid_qty: f64,
    pub best_ask_price: Option<f64>,
    pub best_ask_qty: f64,
    pub spread: Option<f64>,
    pub mid_price: Option<f64>,
}

pub struct LimitOrderBook {
    pub symbol: String,
    // Bids sorted descending (highest price first), Asks sorted ascending (lowest price first)
    pub bids: BTreeMap<u64, Vec<LimitOrder>>, // Key: price scaled to u64 integer ticks
    pub asks: BTreeMap<u64, Vec<LimitOrder>>,
    pub next_trade_id: u64,
    pub price_scale: f64, // Multiplier to convert float price to integer ticks (e.g., 100.0 for 2 decimals, 10000.0 for pips)
}

impl LimitOrderBook {
    pub fn new(symbol: &str, decimals: u32) -> Self {
        let scale = 10.0_f64.powi(decimals as i32);
        Self {
            symbol: symbol.to_string(),
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            next_trade_id: 1,
            price_scale: scale,
        }
    }

    fn price_to_tick(&self, price: f64) -> u64 {
        (price * self.price_scale).round() as u64
    }

    fn tick_to_price(&self, tick: u64) -> f64 {
        (tick as f64) / self.price_scale
    }

    /// Pasang order baru dan jalankan matching engine secara instan (Price-Time Priority)
    pub fn place_order(&mut self, mut order: LimitOrder) -> Vec<TradeExecution> {
        let mut executions = Vec::new();
        let scale = self.price_scale;
        let order_tick = (order.price * scale).round() as u64;

        match order.side {
            OrderSide::Bid => {
                // Match against best asks where ask_price <= bid_price
                let mut matched_ticks = Vec::new();

                for (&ask_tick, ask_orders) in self.asks.iter_mut() {
                    if ask_tick > order_tick || order.quantity <= 0.0 {
                        break;
                    }

                    let mut i = 0;
                    while i < ask_orders.len() && order.quantity > 0.0 {
                        let fill_qty = order.quantity.min(ask_orders[i].quantity);
                        let trade_price = (ask_tick as f64) / scale;

                        executions.push(TradeExecution {
                            trade_id: self.next_trade_id,
                            symbol: self.symbol.clone(),
                            maker_order_id: ask_orders[i].order_id,
                            taker_order_id: order.order_id,
                            price: trade_price,
                            quantity: fill_qty,
                            timestamp_ns: order.timestamp_ns,
                        });
                        self.next_trade_id += 1;

                        order.quantity -= fill_qty;
                        ask_orders[i].quantity -= fill_qty;

                        if ask_orders[i].quantity <= 0.0 {
                            ask_orders.remove(i);
                        } else {
                            i += 1;
                        }
                    }

                    if ask_orders.is_empty() {
                        matched_ticks.push(ask_tick);
                    }
                }

                for t in matched_ticks {
                    self.asks.remove(&t);
                }

                // Sisa kuantitas masuk ke antrean Bid
                if order.quantity > 0.0 {
                    self.bids.entry(order_tick).or_default().push(order);
                }
            }
            OrderSide::Ask => {
                // Match against best bids where bid_price >= ask_price
                let mut matched_ticks = Vec::new();

                // Iterate bids in reverse order (highest first)
                for (&bid_tick, bid_orders) in self.bids.iter_mut().rev() {
                    if bid_tick < order_tick || order.quantity <= 0.0 {
                        break;
                    }

                    let mut i = 0;
                    while i < bid_orders.len() && order.quantity > 0.0 {
                        let fill_qty = order.quantity.min(bid_orders[i].quantity);
                        let trade_price = (bid_tick as f64) / scale;

                        executions.push(TradeExecution {
                            trade_id: self.next_trade_id,
                            symbol: self.symbol.clone(),
                            maker_order_id: bid_orders[i].order_id,
                            taker_order_id: order.order_id,
                            price: trade_price,
                            quantity: fill_qty,
                            timestamp_ns: order.timestamp_ns,
                        });
                        self.next_trade_id += 1;

                        order.quantity -= fill_qty;
                        bid_orders[i].quantity -= fill_qty;

                        if bid_orders[i].quantity <= 0.0 {
                            bid_orders.remove(i);
                        } else {
                            i += 1;
                        }
                    }

                    if bid_orders.is_empty() {
                        matched_ticks.push(bid_tick);
                    }
                }

                for t in matched_ticks {
                    self.bids.remove(&t);
                }

                // Sisa kuantitas masuk ke antrean Ask
                if order.quantity > 0.0 {
                    self.asks.entry(order_tick).or_default().push(order);
                }
            }
        }

        executions
    }

    /// Ambil snapshot Best Bid Offer (BBO) dan Spread saat ini
    pub fn get_bbo(&self) -> BboSnapshot {
        let best_bid = self.bids.iter().next_back().map(|(&tick, orders)| {
            (self.tick_to_price(tick), orders.iter().map(|o| o.quantity).sum::<f64>())
        });

        let best_ask = self.asks.iter().next().map(|(&tick, orders)| {
            (self.tick_to_price(tick), orders.iter().map(|o| o.quantity).sum::<f64>())
        });

        let (b_price, b_qty) = match best_bid {
            Some((p, q)) => (Some(p), q),
            None => (None, 0.0),
        };

        let (a_price, a_qty) = match best_ask {
            Some((p, q)) => (Some(p), q),
            None => (None, 0.0),
        };

        let spread = match (b_price, a_price) {
            (Some(b), Some(a)) => Some(a - b),
            _ => None,
        };

        let mid = match (b_price, a_price) {
            (Some(b), Some(a)) => Some((a + b) / 2.0),
            _ => None,
        };

        BboSnapshot {
            symbol: self.symbol.clone(),
            best_bid_price: b_price,
            best_bid_qty: b_qty,
            best_ask_price: a_price,
            best_ask_qty: a_qty,
            spread,
            mid_price: mid,
        }
    }
}
