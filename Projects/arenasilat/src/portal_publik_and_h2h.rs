// Projects/arenasilat/src/portal_publik_and_h2h.rs
// ==============================================================================
// ARENA SILAT SAAS — PUBLIC PORTAL, LIVE TICKER & HEAD-TO-HEAD MATCH ANALYTICS
// 100% Zero External Dependencies — Berdaulat Penuh di atas Widya-Lang
// Sesuai Dokumen:
// - docs/portal_publik_jadwal_dan_event.md
// - docs/portal_publik_statistik_dan_riwayat_laga.md
// - docs/modul_social_sharing_dan_kartu_atlet.md
// ==============================================================================

#[derive(Debug, Clone)]
pub struct PertandinganPublikLive {
    pub nomor_partai: String,
    pub gelanggang_id: String,
    pub gelanggang_nama: String,
    pub estimasi_jam_mulai: String,
    pub kategori_tanding: String,
    pub nama_atlet_merah: String,
    pub kontingen_merah: String,
    pub skor_merah: i32,
    pub nama_atlet_biru: String,
    pub kontingen_biru: String,
    pub skor_biru: i32,
    pub babak_ronde_ke: u32,
    pub sisa_waktu_detik: u32,
    pub status_pertandingan: String,
}

#[derive(Debug, Clone)]
pub struct HeadToHeadAnalisis {
    pub partai_id: String,
    pub atlet_merah_nama: String,
    pub atlet_merah_kontingen: String,
    pub atlet_merah_elo: f64,
    pub atlet_merah_winrate: f64,
    pub atlet_merah_teknik_favorit: String,
    pub atlet_biru_nama: String,
    pub atlet_biru_kontingen: String,
    pub atlet_biru_elo: f64,
    pub atlet_biru_winrate: f64,
    pub atlet_biru_teknik_favorit: String,
    pub win_probability_merah_persen: f64,
    pub win_probability_biru_persen: f64,
}

pub struct PublicPortalEngine;

impl PublicPortalEngine {
    /// Hitung analisis Head-to-Head dan probabilitas kemenangan berbasis ELO dan Winrate
    pub fn kalkulasi_h2h(
        partai_id: &str,
        merah_nama: &str,
        merah_kontingen: &str,
        merah_elo: f64,
        merah_menang: u32,
        merah_kalah: u32,
        merah_teknik: &str,
        biru_nama: &str,
        biru_kontingen: &str,
        biru_elo: f64,
        biru_menang: u32,
        biru_kalah: u32,
        biru_teknik: &str,
    ) -> HeadToHeadAnalisis {
        let total_m = merah_menang + merah_kalah;
        let wr_m = if total_m > 0 { (merah_menang as f64) / (total_m as f64) * 100.0 } else { 50.0 };

        let total_b = biru_menang + biru_kalah;
        let wr_b = if total_b > 0 { (biru_menang as f64) / (total_b as f64) * 100.0 } else { 50.0 };

        // Formula Bradley-Terry / Elo Logistic Win Probability
        let exponent = (biru_elo - merah_elo) / 400.0;
        let prob_m = 1.0 / (1.0 + 10.0_f64.powf(exponent));
        let prob_merah_persen = (prob_m * 1000.0).round() / 10.0;
        let prob_biru_persen = (100.0 - prob_merah_persen * 10.0).round() / 10.0;

        HeadToHeadAnalisis {
            partai_id: partai_id.to_string(),
            atlet_merah_nama: merah_nama.to_string(),
            atlet_merah_kontingen: merah_kontingen.to_string(),
            atlet_merah_elo: merah_elo,
            atlet_merah_winrate: wr_m,
            atlet_merah_teknik_favorit: merah_teknik.to_string(),
            atlet_biru_nama: biru_nama.to_string(),
            atlet_biru_kontingen: biru_kontingen.to_string(),
            atlet_biru_elo: biru_elo,
            atlet_biru_winrate: wr_b,
            atlet_biru_teknik_favorit: biru_teknik.to_string(),
            win_probability_merah_persen: prob_merah_persen,
            win_probability_biru_persen: 100.0 - prob_merah_persen,
        }
    }

    /// Format ticker live pertandingan untuk OBS / Layar Publik
    pub fn format_live_ticker(laga: &PertandinganPublikLive) -> String {
        format!(
            "[{}] {} | 🔴 {} ({}) [{}] vs [{}] ({}) {} 🔵 | Ronde {} ({:02}:{:02})",
            laga.gelanggang_nama,
            laga.kategori_tanding,
            laga.nama_atlet_merah,
            laga.kontingen_merah,
            laga.skor_merah,
            laga.skor_biru,
            laga.kontingen_biru,
            laga.nama_atlet_biru,
            laga.babak_ronde_ke,
            laga.sisa_waktu_detik / 60,
            laga.sisa_waktu_detik % 60
        )
    }
}
