use crate::adengine::types::{AdCreative, AdFormat, BannerPosition, ScreenCorner};

/// Mesin Penyusun Komponen Overlay Interaktif (Video & In-App Dynamic Injector)
pub struct OverlayEngine;

impl OverlayEngine {
    /// Hasilkan Kode HTML & CSS Standar untuk Overlay di Atas Video
    pub fn render_video_overlay_html(creative: &AdCreative) -> String {
        match &creative.format {
            AdFormat::VideoOverlayBanner { height_pct, auto_close_sec } => {
                format!(
r#"<div id="widya-video-overlay-{id}" class="widya-video-ad-overlay" style="position:absolute; bottom:12px; left:50%; transform:translateX(-50%); width:92%; max-width:720px; height:{h}%; min-height:80px; background:rgba(15, 23, 42, 0.88); backdrop-filter:blur(16px); border:1px solid rgba(99, 102, 241, 0.4); border-radius:12px; box-shadow:0 12px 32px rgba(0,0,0,0.6); display:flex; align-items:center; justify-content:space-between; padding:12px 18px; z-index:9999; color:#f8fafc; font-family:'Plus Jakarta Sans',sans-serif; animation:widyaFadeUp 0.3s ease;">
  <div style="display:flex; align-items:center; gap:14px;">
    <div style="font-size:24px; background:rgba(99,102,241,0.2); width:44px; height:44px; border-radius:10px; display:flex; align-items:center; justify-content:center; border:1px solid rgba(99,102,241,0.3);">📢</div>
    <div>
      <div style="font-size:10px; text-transform:uppercase; color:#818cf8; font-weight:700; letter-spacing:0.5px;">Sponsor • {sponsor}</div>
      <div style="font-size:13px; font-weight:700; color:#ffffff; margin-top:2px;">{title}</div>
      <div style="font-size:11px; color:#94a3b8; margin-top:1px;">{desc}</div>
    </div>
  </div>
  <div style="display:flex; align-items:center; gap:10px;">
    <a href="{cta_url}" target="_blank" style="background:linear-gradient(135deg, #6366f1, #4f46e5); color:#fff; text-decoration:none; padding:8px 16px; border-radius:8px; font-size:12px; font-weight:700; display:inline-flex; align-items:center; gap:6px; box-shadow:0 4px 12px rgba(99,102,241,0.3); transition:all 0.2s;">
      {cta_text} ↗
    </a>
    <button onclick="document.getElementById('widya-video-overlay-{id}').remove()" style="background:rgba(255,255,255,0.1); border:none; color:#94a3b8; width:28px; height:28px; border-radius:6px; cursor:pointer; font-size:14px; display:flex; align-items:center; justify-content:center;">✕</button>
  </div>
</div>"#,
                    id = creative.id,
                    h = height_pct,
                    sponsor = creative.advertiser_name,
                    title = creative.title,
                    desc = creative.description,
                    cta_url = creative.cta_url,
                    cta_text = creative.cta_text
                )
            }
            AdFormat::ShoppableVideoHotspot { timestamp_sec, product_id, price_idr } => {
                format!(
r#"<div id="widya-hotspot-{id}" class="widya-shoppable-pin" data-timestamp="{t}" style="position:absolute; top:25%; right:24px; background:rgba(16, 185, 129, 0.95); backdrop-filter:blur(12px); border:1px solid #34d399; border-radius:10px; padding:10px 14px; box-shadow:0 8px 24px rgba(16,185,129,0.35); display:flex; align-items:center; gap:12px; z-index:9999; color:#fff; font-family:'Plus Jakarta Sans',sans-serif;">
  <span style="font-size:20px;">🛍️</span>
  <div>
    <div style="font-size:9.5px; text-transform:uppercase; font-weight:700; color:#d1fae5;">Produk di Video Ini ({t}s)</div>
    <div style="font-size:12px; font-weight:700;">{title}</div>
    <div style="font-size:11px; font-weight:800; color:#a7f3d0;">Rp {price}</div>
  </div>
  <a href="{cta_url}" target="_blank" style="background:#ffffff; color:#065f46; text-decoration:none; padding:5px 10px; border-radius:6px; font-size:10.5px; font-weight:700; margin-left:4px;">Beli</a>
</div>"#,
                    id = creative.id,
                    t = timestamp_sec,
                    title = creative.title,
                    price = price_idr,
                    cta_url = creative.cta_url
                )
            }
            _ => String::new(),
        }
    }

    /// Hasilkan Kode HTML untuk In-App Sticky Banner & Interstitial Modal
    pub fn render_app_overlay_html(creative: &AdCreative) -> String {
        match &creative.format {
            AdFormat::AppStickyBanner { position } => {
                let pos_css = match position {
                    BannerPosition::Top => "top:0; border-bottom:1px solid rgba(99,102,241,0.3);",
                    BannerPosition::Bottom => "bottom:0; border-top:1px solid rgba(99,102,241,0.3);",
                };
                format!(
r#"<div id="widya-app-banner-{id}" class="widya-sticky-app-banner" style="position:fixed; left:0; width:100%; height:60px; {pos_css} background:rgba(10,13,20,0.92); backdrop-filter:blur(20px); z-index:99999; display:flex; align-items:center; justify-content:space-between; padding:0 24px; color:#f8fafc; font-family:'Plus Jakarta Sans',sans-serif; box-shadow:0 10px 30px rgba(0,0,0,0.5);">
  <div style="display:flex; align-items:center; gap:12px;">
    <div style="font-size:20px; background:linear-gradient(135deg,#6366f1,#06b6d4); width:36px; height:36px; border-radius:8px; display:flex; align-items:center; justify-content:center;">⚡</div>
    <div>
      <div style="font-size:12px; font-weight:700; color:#f8fafc;">{title} <span style="font-size:9.5px; background:rgba(99,102,241,0.2); color:#818cf8; padding:2px 6px; border-radius:4px; margin-left:6px;">PROMOSI</span></div>
      <div style="font-size:10.5px; color:#94a3b8;">{desc}</div>
    </div>
  </div>
  <div style="display:flex; align-items:center; gap:10px;">
    <a href="{cta_url}" target="_blank" style="background:#6366f1; color:#fff; text-decoration:none; padding:6px 14px; border-radius:6px; font-size:11px; font-weight:700;">{cta_text}</a>
    <button onclick="document.getElementById('widya-app-banner-{id}').remove()" style="background:transparent; border:none; color:#64748b; font-size:16px; cursor:pointer; padding:4px;">✕</button>
  </div>
</div>"#,
                    id = creative.id,
                    pos_css = pos_css,
                    title = creative.title,
                    desc = creative.description,
                    cta_url = creative.cta_url,
                    cta_text = creative.cta_text
                )
            }
            AdFormat::AppInterstitialModal { duration_sec } => {
                format!(
r#"<div id="widya-interstitial-{id}" class="widya-interstitial-modal" style="position:fixed; top:0; left:0; width:100vw; height:100vh; background:rgba(2,6,23,0.85); backdrop-filter:blur(24px); z-index:999999; display:flex; align-items:center; justify-content:center; padding:20px; font-family:'Plus Jakarta Sans',sans-serif;">
  <div style="width:100%; max-width:540px; background:linear-gradient(135deg,#0f172a,#1e293b); border:1px solid rgba(99,102,241,0.4); border-radius:18px; padding:32px; box-shadow:0 25px 60px rgba(0,0,0,0.8); text-align:center; position:relative;">
    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:18px;">
      <span style="font-size:11px; color:#818cf8; font-weight:700; text-transform:uppercase; letter-spacing:1px;">✨ Penawaran Eksklusif Mitra</span>
      <span id="widya-interstitial-timer-{id}" style="font-size:11px; background:rgba(255,255,255,0.1); padding:3px 8px; border-radius:6px; color:#94a3b8;">Tutup dalam {dur}s</span>
    </div>
    <div style="font-size:48px; margin-bottom:12px;">🎁</div>
    <h3 style="font-size:20px; color:#f8fafc; font-weight:800; margin-bottom:8px;">{title}</h3>
    <p style="font-size:13px; color:#94a3b8; line-height:1.5; margin-bottom:24px;">{desc}</p>
    <div style="display:flex; gap:12px; justify-content:center;">
      <a href="{cta_url}" target="_blank" style="background:#6366f1; color:#fff; text-decoration:none; padding:10px 24px; border-radius:8px; font-size:13px; font-weight:700; box-shadow:0 4px 14px rgba(99,102,241,0.4);">{cta_text}</a>
      <button id="widya-interstitial-close-btn-{id}" onclick="document.getElementById('widya-interstitial-{id}').remove()" style="background:rgba(255,255,255,0.08); border:1px solid rgba(255,255,255,0.15); color:#f8fafc; padding:10px 18px; border-radius:8px; font-size:13px; font-weight:600; cursor:pointer;">Lewati</button>
    </div>
  </div>
</div>"#,
                    id = creative.id,
                    dur = duration_sec,
                    title = creative.title,
                    desc = creative.description,
                    cta_url = creative.cta_url,
                    cta_text = creative.cta_text
                )
            }
            AdFormat::FloatingPipWidget { corner } => {
                let corner_css = match corner {
                    ScreenCorner::BottomRight => "bottom:24px; right:24px;",
                    ScreenCorner::BottomLeft => "bottom:24px; left:24px;",
                    ScreenCorner::TopRight => "top:24px; right:24px;",
                    ScreenCorner::TopLeft => "top:24px; left:24px;",
                };
                format!(
r#"<div id="widya-pip-{id}" class="widya-floating-pip" style="position:fixed; {corner_css} width:280px; background:rgba(15,23,42,0.95); backdrop-filter:blur(18px); border:1px solid rgba(6,182,212,0.4); border-radius:14px; padding:14px; box-shadow:0 12px 36px rgba(0,0,0,0.7); z-index:99999; font-family:'Plus Jakarta Sans',sans-serif; cursor:grab;">
  <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:8px;">
    <span style="font-size:10px; font-weight:700; color:#22d3ee; text-transform:uppercase;">PiP Sponsor</span>
    <button onclick="document.getElementById('widya-pip-{id}').remove()" style="background:none; border:none; color:#64748b; cursor:pointer; font-size:12px;">✕</button>
  </div>
  <div style="font-size:12px; font-weight:700; color:#f8fafc; margin-bottom:4px;">{title}</div>
  <div style="font-size:10.5px; color:#94a3b8; margin-bottom:10px;">{desc}</div>
  <a href="{cta_url}" target="_blank" style="background:#0284c7; color:#fff; text-decoration:none; display:block; text-align:center; padding:6px; border-radius:6px; font-size:11px; font-weight:700;">{cta_text}</a>
</div>"#,
                    id = creative.id,
                    corner_css = corner_css,
                    title = creative.title,
                    desc = creative.description,
                    cta_url = creative.cta_url,
                    cta_text = creative.cta_text
                )
            }
            _ => String::new(),
        }
    }
}
