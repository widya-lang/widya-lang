// Projects/arenasilat/src/ranking_and_elo.rs
// ==============================================================================
// ARENA SILAT SAAS — SISTEM RANKING ELO & LEADERBOARD NASIONAL
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};

/// Tingkatan Event Turnamen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TingkatKejuaraan {
    Tier1DuniaSeagamesPon, // Bobot: 1.00
    Tier2KejurnasPopnas,   // Bobot: 0.75
    Tier3KejurcabOpen,     // Bobot: 0.50
    Tier4PemasalanLokal,   // Bobot: 0.25
}

impl TingkatKejuaraan {
    pub fn bobot(&self) -> f32 {
        match self {
            TingkatKejuaraan::Tier1DuniaSeagamesPon => 1.00,
            TingkatKejuaraan::Tier2KejurnasPopnas => 0.75,
            TingkatKejuaraan::Tier3KejurcabOpen => 0.50,
            TingkatKejuaraan::Tier4PemasalanLokal => 0.25,
        }
    }
}

/// Profil Ranking Pesilat Nasional
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilRankingPesilat {
    pub atlet_id: String,
    pub nama_lengkap: String,
    pub perguruan: String,
    pub provinsi: String,
    pub kelas_tanding: String,
    pub rating_elo: f32,
    pub total_poin_prestasi: f32,
    pub jumlah_menang: u32,
    pub jumlah_kalah: u32,
    pub medali_emas: u32,
    pub medali_perak: u32,
    pub medali_perunggu: u32,
}

/// Mesin Kalkulasi Elo & Ranking Nasional
pub struct SilatEloEngine;

impl SilatEloEngine {
    /// Hitung Pembaruan Skor Elo Rating Pasca Pertandingan
    pub fn hitung_pembaruan_elo(rating_a: f32, rating_b: f32, menang_a: bool, tier: TingkatKejuaraan) -> (f32, f32) {
        let k_factor = 32.0 * tier.bobot();

        // Probabilitas Menang Ekspektasi (Formula Standar FIDE/Elo)
        let ea = 1.0 / (1.0 + 10.0f32.powf((rating_b - rating_a) / 400.0));
        let eb = 1.0 / (1.0 + 10.0f32.powf((rating_a - rating_b) / 400.0));

        let sa = if menang_a { 1.0 } else { 0.0 };
        let sb = if menang_a { 0.0 } else { 1.0 };

        let rating_baru_a = rating_a + k_factor * (sa - ea);
        let rating_baru_b = rating_b + k_factor * (sb - eb);

        ((rating_baru_a * 10.0).round() / 10.0, (rating_baru_b * 10.0).round() / 10.0)
    }

    /// Tambahkan Poin Prestasi Medali Berdasarkan Tingkat Kejuaraan
    pub fn hitung_poin_medali(posisi_medali: u8, tier: TingkatKejuaraan) -> f32 {
        let poin_dasar = match posisi_medali {
            1 => 1000.0, // Emas
            2 => 600.0,  // Perak
            3 => 350.0,  // Perunggu
            _ => 100.0,
        };
        poin_dasar * tier.bobot()
    }

    /// Urutkan Leaderboard Nasional Berdasarkan Akumulasi Poin Prestasi & Rating Elo
    pub fn urutkan_leaderboard(mut list: Vec<ProfilRankingPesilat>) -> Vec<ProfilRankingPesilat> {
        list.sort_by(|a, b| {
            b.total_poin_prestasi.partial_cmp(&a.total_poin_prestasi).unwrap()
                .then_with(|| b.rating_elo.partial_cmp(&a.rating_elo).unwrap())
        });
        list
    }
}
