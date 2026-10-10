// tests/arenasilat_extended_parity_test.rs
// ==============================================================================
// ARENA SILAT SAAS — EXTENDED SPECIFICATION PARITY TEST (53 DOCS COMPLIANCE)
// 100% Zero External Dependencies — Verified on Widya-Lang Ecosystem
// ==============================================================================

#[path = "../Projects/arenasilat/src/seni_tgr_engine.rs"]
mod seni_tgr_engine;
#[path = "../Projects/arenasilat/src/ranking_and_elo.rs"]
mod ranking_and_elo;
#[path = "../Projects/arenasilat/src/puspresnas_and_social.rs"]
mod puspresnas_and_social;
#[path = "../Projects/arenasilat/src/perguruan_and_notifikasi.rs"]
mod perguruan_and_notifikasi;

use seni_tgr_engine::{SeniTGREngine, NomorSeni};
use ranking_and_elo::{SilatEloEngine, TingkatKejuaraan, ProfilRankingPesilat};
use puspresnas_and_social::PuspresnasAndSocialEngine;
use perguruan_and_notifikasi::GuildAndNotificationEngine;

#[test]
fn test_seni_tgr_trimmed_mean_and_penalties() {
    println!(">>> Testing Arena Silat: Seni TGR 5-Jury Trimmed Mean & Penalties...");

    // 1. Uji Pinalti Waktu: 170 detik (target 175-185s -> deviasi 5s = -0.05)
    let pinalti = SeniTGREngine::hitung_pinalti_waktu(170);
    assert_eq!((pinalti * 100.0).round() / 100.0, 0.05);

    // 2. Simulasi 5 Juri
    let mut juri_scores = Vec::new();
    juri_scores.push(SeniTGREngine::kalkulasi_skor_juri(1, 2, 0.05, false, false, false)); // 9.90 - 0.02 + 0.05 = 9.93
    juri_scores.push(SeniTGREngine::kalkulasi_skor_juri(2, 3, 0.04, false, false, false)); // 9.90 - 0.03 + 0.04 = 9.91
    juri_scores.push(SeniTGREngine::kalkulasi_skor_juri(3, 1, 0.06, false, false, false)); // 9.90 - 0.01 + 0.06 = 9.95
    juri_scores.push(SeniTGREngine::kalkulasi_skor_juri(4, 5, 0.02, false, false, false)); // 9.90 - 0.05 + 0.02 = 9.87
    juri_scores.push(SeniTGREngine::kalkulasi_skor_juri(5, 0, 0.08, false, false, false)); // 9.90 - 0.00 + 0.08 = 9.98

    let hasil = SeniTGREngine::hitung_skor_akhir_seni(
        "SENI-TUNGGAL-01", NomorSeni::TunggalBaku, "Pesilat Seni A", "Jawa Tengah", 180, juri_scores
    );

    // Trimmed mean membuang skor tertinggi (9.98) dan terendah (9.87), merata-ratakan 9.91, 9.93, 9.95 -> 9.930
    assert_eq!(hasil.skor_akhir_terpangkas, 9.930);
    println!("    [PASS] Seni TGR 5-Jury trimmed mean verified! Final Score: {:.3}", hasil.skor_akhir_terpangkas);
}

#[test]
fn test_national_ranking_elo_and_tier_multipliers() {
    println!(">>> Testing Arena Silat: Silat Dynamic Elo & National Leaderboard...");

    let (r_a, r_b) = SilatEloEngine::hitung_pembaruan_elo(1500.0, 1500.0, true, TingkatKejuaraan::Tier1DuniaSeagamesPon);
    assert!(r_a > 1500.0);
    assert!(r_b < 1500.0);

    let poin_emas_tier1 = SilatEloEngine::hitung_poin_medali(1, TingkatKejuaraan::Tier1DuniaSeagamesPon);
    assert_eq!(poin_emas_tier1, 1000.0);

    let poin_emas_tier2 = SilatEloEngine::hitung_poin_medali(1, TingkatKejuaraan::Tier2KejurnasPopnas);
    assert_eq!(poin_emas_tier2, 750.0);

    let leaderboard = vec![
        ProfilRankingPesilat { atlet_id: "A1".to_string(), nama_lengkap: "Atlet A".to_string(), perguruan: "TS".to_string(), provinsi: "Jateng".to_string(), kelas_tanding: "E".to_string(), rating_elo: r_a, total_poin_prestasi: 1000.0, jumlah_menang: 5, jumlah_kalah: 0, medali_emas: 1, medali_perak: 0, medali_perunggu: 0 },
        ProfilRankingPesilat { atlet_id: "A2".to_string(), nama_lengkap: "Atlet B".to_string(), perguruan: "PSHT".to_string(), provinsi: "Jatim".to_string(), kelas_tanding: "E".to_string(), rating_elo: r_b, total_poin_prestasi: 600.0, jumlah_menang: 3, jumlah_kalah: 1, medali_emas: 0, medali_perak: 1, medali_perunggu: 0 },
    ];

    let sorted = SilatEloEngine::urutkan_leaderboard(leaderboard);
    assert_eq!(sorted[0].nama_lengkap, "Atlet A");
    println!("    [PASS] Dynamic Elo & National Leaderboard verified!");
}

#[test]
fn test_puspresnas_simt_and_whatsapp_dispatcher() {
    println!(">>> Testing Arena Silat: Puspresnas SIMT Exporter & WhatsApp Dispatcher...");

    let record = PuspresnasAndSocialEngine::buat_rekod_simt(
        "0051234567", "Eko Prasetyo", "20304050", "Kejurnas Silat 2026", "Tanding Kelas E Putra",
        "Juara 1 (Emas)", "2026-10-09", "https://arenasilat.id/sertif/01", "abc123456789"
    );
    assert_eq!(record.nisn, "0051234567");

    let notif = GuildAndNotificationEngine::buat_notifikasi_h3_laga(
        "08123456789", "Eko Prasetyo", 1, 1, "09:30 WIB"
    );
    assert!(notif.pesan_teks.contains("Ruang Marshalling (H-3)"));
    println!("    [PASS] SIMT Puspresnas format & WhatsApp H-3 Dispatcher verified!");
}
