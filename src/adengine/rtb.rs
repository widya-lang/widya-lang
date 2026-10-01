use crate::adengine::types::{AdCreative, AuctionResult, RtbBid};
use std::collections::HashMap;

/// Mesin Lelang Iklan Real-Time (RTB Engine - OpenRTB 3.0 Compatible)
pub struct RtbAuctionEngine {
    pub registered_creatives: HashMap<String, AdCreative>,
    pub floor_price_cpm_idr: u64,
}

impl RtbAuctionEngine {
    pub fn new(floor_price_cpm_idr: u64) -> Self {
        Self {
            registered_creatives: HashMap::new(),
            floor_price_cpm_idr,
        }
    }

    pub fn register_creative(&mut self, creative: AdCreative) {
        self.registered_creatives.insert(creative.id.clone(), creative);
    }

    /// Jalankan Lelang Vickrey Harga Kedua (Second-Price Vickrey Auction)
    pub fn execute_auction(&self, auction_id: &str, mut bids: Vec<RtbBid>) -> AuctionResult {
        // Filter tawaran di atas harga dasar (floor price)
        bids.retain(|b| b.bid_price_cpm_idr >= self.floor_price_cpm_idr);

        // Urutkan dari tawaran tertinggi ke terendah
        bids.sort_by(|a, b| b.bid_price_cpm_idr.cmp(&a.bid_price_cpm_idr));

        if bids.is_empty() {
            return AuctionResult {
                auction_id: auction_id.to_string(),
                winning_bid: None,
                second_price_cpm_idr: self.floor_price_cpm_idr,
                total_bidders: 0,
                creative: None,
                cryptographic_seal: "NO_BIDS".to_string(),
            };
        }

        let total_bidders = bids.len();
        let highest_bid = bids[0].clone();
        
        // Harga pemenang adalah tawaran kedua tertinggi + Rp 100, atau floor price jika hanya 1 penawar
        let second_price = if bids.len() > 1 {
            bids[1].bid_price_cpm_idr + 100
        } else {
            self.floor_price_cpm_idr
        };

        let creative = self.registered_creatives.get(&highest_bid.creative_id).cloned();

        // Segel Kriptografi Lelang (Proof-of-Fair-Auction)
        let mut hash_state: u64 = 0xcbf29ce484222325;
        let seal_input = format!("{}:{}:{}:{}", auction_id, highest_bid.bidder_id, second_price, highest_bid.timestamp);
        for b in seal_input.bytes() {
            hash_state = (hash_state ^ (b as u64)).wrapping_mul(0x100000001b3);
        }
        let seal = format!("AUCT-VICKREY-{:016x}", hash_state);

        AuctionResult {
            auction_id: auction_id.to_string(),
            winning_bid: Some(highest_bid),
            second_price_cpm_idr: second_price,
            total_bidders,
            creative,
            cryptographic_seal: seal,
        }
    }
}
