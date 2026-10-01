use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Format Iklan yang Didukung oleh WidyaAd Engine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdFormat {
    /// Iklan Video Linear Sebelum Konten (Skippable setelah N detik)
    LinearPreRoll { skip_offset_sec: u32, duration_sec: u32 },
    /// Iklan Video Linear di Tengah Konten pada Timestamp Tertentu
    LinearMidRoll { cue_time_sec: u32, skip_offset_sec: u32, duration_sec: u32 },
    /// Iklan Video Linear di Akhir Konten
    LinearPostRoll { duration_sec: u32 },
    /// Banner Non-Linear di Atas Video (Bottom 20%, Glassmorphism)
    VideoOverlayBanner { height_pct: u32, auto_close_sec: u32 },
    /// Pin Interaktif Produk di Video pada Timestamp Tertentu (Shoppable Hotspot)
    ShoppableVideoHotspot { timestamp_sec: u32, product_id: String, price_idr: u64 },
    /// Banner Menempel di Atas / Bawah Aplikasi Web / Mobile (Sticky Smart Banner)
    AppStickyBanner { position: BannerPosition },
    /// Modal Layar Penuh Interstitial dengan Tombol Tutup & Hitung Mundur
    AppInterstitialModal { duration_sec: u32 },
    /// Widget Melayang Picture-in-Picture (PiP) yang Dapat Digeser (Draggable)
    FloatingPipWidget { corner: ScreenCorner },
    /// Drawer Sponsor yang Meluncur dari Samping (Slide-Out)
    SlideOutDrawer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BannerPosition {
    Top,
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScreenCorner {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

/// Materi Kreatif Iklan (Creative Asset)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdCreative {
    pub id: String,
    pub campaign_id: String,
    pub title: String,
    pub description: String,
    pub advertiser_name: String,
    pub format: AdFormat,
    pub media_url: String,       // URL Video MP4 / WebM / Image SVG/PNG
    pub cta_text: String,        // misal "Kunjungi Situs", "Beli Sekarang"
    pub cta_url: String,         // Landing page URL
    pub companion_banner_url: Option<String>,
    pub fallback_text: String,
    pub tags: Vec<String>,
}

/// Penargetan & Aturan Penayangan (Targeting Rules)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingRules {
    pub target_categories: Vec<String>, // misal "fintech", "edutech", "health"
    pub geo_provinces: Vec<String>,    // misal "DKI Jakarta", "Jawa Barat", "Semua"
    pub min_cpm_idr: u64,              // Batas bawah lelang CPM
    pub daily_budget_idr: u64,
    pub frequency_capping_per_user: u32, // Maksimal tampil per pengguna / hari
}

/// Tawaran Lelang Real-Time Bidding (RTB Bid)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtbBid {
    pub bidder_id: String,
    pub campaign_id: String,
    pub bid_price_cpm_idr: u64,
    pub creative_id: String,
    pub timestamp: u64,
}

/// Hasil Lelang Real-Time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionResult {
    pub auction_id: String,
    pub winning_bid: Option<RtbBid>,
    pub second_price_cpm_idr: u64,
    pub total_bidders: usize,
    pub creative: Option<AdCreative>,
    pub cryptographic_seal: String,
}

/// Metrik Viewability & Telemetri Standar MRC (Media Rating Council)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewabilityMetric {
    pub impression_id: String,
    pub creative_id: String,
    pub is_mrc_compliant: bool,     // >=50% piksel terlihat selama >=1s (display) atau >=2s (video)
    pub duration_viewed_ms: u64,
    pub pixels_visible_pct: f32,
    pub click_registered: bool,
    pub sound_enabled: bool,
    pub user_interaction: bool,
    pub fraud_risk_score: f32,       // 0.0 (Aman) - 1.0 (Bot Fraud)
    pub verification_hash: String,
}
