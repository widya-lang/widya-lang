// Projects/arenasilat/src/puspresnas_and_social.rs
// ==============================================================================
// ARENA SILAT SAAS — PUSPRESNAS SIMT EXPORTER & ATHLETE SOCIAL TRADING CARDS
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};

/// Format Ekspor Kurasi Prestasi Puspresnas / SIMT Kemendikdasmen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimtPuspresnasRecord {
    pub id_prestasi: String,
    pub nisn: String,
    pub nama_atlet: String,
    pub sekolah_npsn: String,
    pub nama_event: String,
    pub penyelenggara: String,
    pub tingkat_event: String, // "Kabupaten/Kota", "Provinsi", "Nasional", "Internasional"
    pub nomor_pertandingan: String,
    pub capaian_juara: String, // "Juara 1 (Emas)", "Juara 2 (Perak)", "Juara 3 (Perunggu)"
    pub tanggal_event: String,
    pub url_sertifikat_resmi: String,
    pub hash_verifikasi_sha256: String,
}

/// Mesin Pembuat Format Puspresnas & Kartu Sosial Atlet
pub struct PuspresnasAndSocialEngine;

impl PuspresnasAndSocialEngine {
    /// Buat Rekod SIMT Puspresnas Resmi
    pub fn buat_rekod_simt(
        nisn: &str,
        nama: &str,
        npsn: &str,
        event: &str,
        nomor: &str,
        juara: &str,
        tgl: &str,
        url_sertif: &str,
        hash: &str,
    ) -> SimtPuspresnasRecord {
        SimtPuspresnasRecord {
            id_prestasi: format!("SIMT-SILAT-{}-{}", nisn, &hash[..8]),
            nisn: nisn.to_string(),
            nama_atlet: nama.to_string(),
            sekolah_npsn: npsn.to_string(),
            nama_event: event.to_string(),
            penyelenggara: "Pengurus Besar / Pengprov IPSI".to_string(),
            tingkat_event: "Nasional".to_string(),
            nomor_pertandingan: nomor.to_string(),
            capaian_juara: juara.to_string(),
            tanggal_event: tgl.to_string(),
            url_sertifikat_resmi: url_sertif.to_string(),
            hash_verifikasi_sha256: hash.to_string(),
        }
    }

    /// Ekspor Rekod ke Format JSON Puspresnas
    pub fn ekspor_json_puspresnas(records: &[SimtPuspresnasRecord]) -> String {
        serde_json::to_string_pretty(records).unwrap_or_else(|_| "[]".to_string())
    }

    /// Generate 9:16 Athlete Digital Trading Card HTML/SVG
    pub fn buat_trading_card_html(
        nama: &str,
        kontingen: &str,
        kategori: &str,
        medali: &str,
        win_streak: u32,
        elo: f32,
    ) -> String {
        format!(r#"<div style="width:360px; height:640px; background:linear-gradient(145deg, #090d16, #1e293b); border:3px solid #f59e0b; border-radius:20px; padding:24px; box-sizing:border-box; color:white; font-family:system-ui, sans-serif; display:flex; flex-direction:column; justify-content:space-between; box-shadow:0 15px 35px rgba(0,0,0,0.6);">
    <div style="display:flex; justify-content:space-between; align-items:center;">
        <div style="font-size:0.8rem; font-weight:bold; letter-spacing:2px; color:#f59e0b;">ARENA SILAT CARD</div>
        <div style="background:#f59e0b; color:black; padding:2px 8px; border-radius:10px; font-size:0.75rem; font-weight:bold;">OFFICIAL</div>
    </div>
    <div style="text-align:center; margin:20px 0;">
        <div style="font-size:4rem;">🥋</div>
        <h2 style="margin:10px 0 2px 0; font-size:1.5rem; text-shadow:0 2px 8px rgba(0,0,0,0.8);">{}</h2>
        <div style="font-size:0.9rem; color:#94a3b8;">{}</div>
        <div style="display:inline-block; margin-top:8px; background:rgba(245, 158, 11, 0.2); border:1px solid #f59e0b; color:#fde68a; padding:4px 12px; border-radius:12px; font-size:0.85rem; font-weight:bold;">{}</div>
    </div>
    <div style="background:rgba(0,0,0,0.4); border-radius:12px; padding:12px; display:grid; grid-template-columns:1fr 1fr; gap:10px; text-align:center;">
        <div><div style="font-size:0.75rem; color:#94a3b8;">CAPAIAN</div><div style="font-size:1.1rem; font-weight:bold; color:#10b981;">{}</div></div>
        <div><div style="font-size:0.75rem; color:#94a3b8;">WIN STREAK</div><div style="font-size:1.1rem; font-weight:bold; color:#38bdf8;">{} Laga</div></div>
        <div><div style="font-size:0.75rem; color:#94a3b8;">ELO RATING</div><div style="font-size:1.1rem; font-weight:bold; color:#f59e0b;">{:.1}</div></div>
        <div><div style="font-size:0.75rem; color:#94a3b8;">VERIFIKASI</div><div style="font-size:1.1rem; font-weight:bold; color:#a855f7;">SHA-256</div></div>
    </div>
    <div style="text-align:center; font-size:0.75rem; color:#64748b;">
        Sovereign IPSI 2026 Engine • arenasilat.id
    </div>
</div>"#, nama, kontingen, kategori, medali, win_streak, elo)
    }
}
