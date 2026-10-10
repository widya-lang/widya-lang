// tests/arenasilat_sovereign_suite_test.rs
// ==============================================================================
// ARENA SILAT SAAS — SOVEREIGN SUITE INTEGRATION TEST
// 100% Zero External Dependencies — Verified on Widya-Lang Ecosystem
// ==============================================================================

#[path = "../Projects/arenasilat/src/scoring_engine.rs"]
mod scoring_engine;
#[path = "../Projects/arenasilat/src/bracket_generator.rs"]
mod bracket_generator;
#[path = "../Projects/arenasilat/src/var_and_hardware.rs"]
mod var_and_hardware;

use scoring_engine::{
    ArenaScoringEngine, SudutPesilat, TipePoin, TipeHukuman, StatusPartai,
};
use bracket_generator::{
    BracketGenerator, AtletPeserta,
};
use var_and_hardware::{
    VarEngine, KeputusanVAR, ScaleBridge, CertificateEngine,
};

#[test]
fn test_arena_silat_ipsi_2026_consensus_scoring() {
    println!(">>> Testing Arena Silat: IPSI 2026 Consensus Scoring Engine...");

    let mut engine = ArenaScoringEngine::baru("MATCH-FINAL-01");
    engine.mulai_pertandingan();
    assert_eq!(engine.status, StatusPartai::Berlangsung);

    // 1. Juri 1 menekan Pukulan (+1) untuk Sudut Merah -> Belum ada konsensus (0 poin)
    let res1 = engine.tekan_tombol_juri(1, SudutPesilat::Merah, TipePoin::Pukulan);
    assert_eq!(res1, None);
    assert_eq!(engine.skor_merah.total_poin, 0);

    // 2. Juri 2 menekan Pukulan (+1) untuk Sudut Merah dalam window 1.5 detik -> KONSENSUS TERCAPAI (+1 poin)!
    let res2 = engine.tekan_tombol_juri(2, SudutPesilat::Merah, TipePoin::Pukulan);
    assert_eq!(res2, Some((SudutPesilat::Merah, TipePoin::Pukulan, 1)));
    assert_eq!(engine.skor_merah.total_poin, 1);
    assert_eq!(engine.skor_merah.jumlah_pukulan, 1);

    // 3. Juri 1 & Juri 3 menekan Tendangan (+2) untuk Sudut Biru -> KONSENSUS (+2 poin)!
    engine.tekan_tombol_juri(1, SudutPesilat::Biru, TipePoin::Tendangan);
    let res_b = engine.tekan_tombol_juri(3, SudutPesilat::Biru, TipePoin::Tendangan);
    assert_eq!(res_b, Some((SudutPesilat::Biru, TipePoin::Tendangan, 2)));
    assert_eq!(engine.skor_biru.total_poin, 2);
    assert_eq!(engine.skor_biru.jumlah_tendangan, 1);

    // 4. Juri 1, 2, 3 menekan Jatuhan (+3) untuk Sudut Merah -> KONSENSUS (+3 poin)!
    engine.tekan_tombol_juri(1, SudutPesilat::Merah, TipePoin::Jatuhan);
    let res_j = engine.tekan_tombol_juri(2, SudutPesilat::Merah, TipePoin::Jatuhan);
    assert_eq!(res_j, Some((SudutPesilat::Merah, TipePoin::Jatuhan, 3)));
    assert_eq!(engine.skor_merah.total_poin, 4); // 1 + 3 = 4
    assert_eq!(engine.skor_merah.jumlah_jatuhan, 1);

    // 5. Ketua Pertandingan (KP) memberikan Teguran 1 (-1) ke Sudut Merah
    engine.berikan_hukuman(SudutPesilat::Merah, TipeHukuman::Teguran1);
    assert_eq!(engine.skor_merah.total_poin, 3); // 4 - 1 = 3

    // 6. Penentuan Pemenang Sementara
    let (pemenang, keterangan) = engine.tentukan_pemenang();
    assert_eq!(pemenang, Some(SudutPesilat::Merah));
    assert!(keterangan.contains("Menang Angka (3 vs 2)"));
}

#[test]
fn test_arena_silat_bracket_generator_and_contingent_separation() {
    println!(">>> Testing Arena Silat: Bracket Generator & Contingent Separation...");

    let peserta = vec![
        AtletPeserta {
            id: "AT-01".into(),
            nama: "Budi Santoso".into(),
            kontingen: "PSHT Cabang Solo".into(),
            kelas_tanding: "Kelas A Putra".into(),
            berat_badan_kg: 48.5,
            unggulan_seed: Some(1),
        },
        AtletPeserta {
            id: "AT-02".into(),
            nama: "Joko Anwar".into(),
            kontingen: "PSHT Cabang Solo".into(), // Kontingen sama dengan AT-01!
            kelas_tanding: "Kelas A Putra".into(),
            berat_badan_kg: 49.0,
            unggulan_seed: None,
        },
        AtletPeserta {
            id: "AT-03".into(),
            nama: "Ahmad Dahlan".into(),
            kontingen: "Tapak Suci Sleman".into(),
            kelas_tanding: "Kelas A Putra".into(),
            berat_badan_kg: 47.8,
            unggulan_seed: Some(2),
        },
        AtletPeserta {
            id: "AT-04".into(),
            nama: "I Gede Dharma".into(),
            kontingen: "Keluarga Silat Nasional Perisai Diri".into(),
            kelas_tanding: "Kelas A Putra".into(),
            berat_badan_kg: 48.2,
            unggulan_seed: None,
        },
        AtletPeserta {
            id: "AT-05".into(),
            nama: "Rizky Ramadhan".into(),
            kontingen: "Pagar Nusa Kediri".into(),
            kelas_tanding: "Kelas A Putra".into(),
            berat_badan_kg: 49.5,
            unggulan_seed: None,
        },
    ];

    let mut bagan = BracketGenerator::buat_bagan_eliminasi("Kelas A Putra", peserta);
    assert_eq!(bagan.kapasitas_bracket, 8); // 5 peserta -> bracket 8
    assert_eq!(bagan.jumlah_bye, 3);        // 8 - 5 = 3 BYE
    assert_eq!(bagan.daftar_partai.len(), 4);

    // Jadwalkan ke 2 Gelanggang (Court 1 & Court 2)
    BracketGenerator::jadwalkan_multi_gelanggang(&mut bagan.daftar_partai, 2, 510, 12);
    
    // Pastikan partai terjadwal dengan format waktu WIB
    for partai in bagan.daftar_partai.iter() {
        if !partai.is_bye {
            assert!(partai.gelanggang_id.is_some());
            assert!(partai.estimasi_waktu.ends_with("WIB"));
        }
    }
}

#[test]
fn test_arena_silat_var_video_assistant_referee() {
    println!(">>> Testing Arena Silat: VAR Video Review System...");

    let mut var_engine = VarEngine::baru();
    let sesi = var_engine.buat_sesi_review(
        "MATCH-01",
        SudutPesilat::Merah,
        2,
        45,
        "Tendangan Terakhir Masuk Sebelum Aba-aba Berhenti"
    );
    assert_eq!(sesi.partai_id, "MATCH-01");
    assert_eq!(sesi.sudut_pengaju, SudutPesilat::Merah);

    // Dewan Hakim menetapkan keputusan: Diterima
    let hasil = var_engine.tetapkan_keputusan(
        &sesi.sesi_id,
        KeputusanVAR::DiterimaPoinMasuk,
        "Rekaman slow-motion sudut kamera 2 mengonfirmasi tendangan sah mengenai sasaran"
    );
    assert!(hasil.is_ok());
    let sesi_sah = hasil.unwrap();
    assert_eq!(sesi_sah.keputusan, Some(KeputusanVAR::DiterimaPoinMasuk));
}

#[test]
fn test_arena_silat_weigh_in_scale_validation() {
    println!(">>> Testing Arena Silat: Digital Weigh-in Scale Validation...");

    // Atlet 1: Masuk rentang 45.0 - 50.0 kg (48.5 kg) -> SAH
    let t1 = ScaleBridge::validasi_berat("AT-01", "Budi Santoso", "Kelas A", 45.0, 50.0, 48.5);
    assert!(t1.status_lolos);
    assert!(t1.catatan.contains("SAH"));

    // Atlet 2: Kelebihan berat badan (50.8 kg) -> DISKUALIFIKASI
    let t2 = ScaleBridge::validasi_berat("AT-02", "Joko Anwar", "Kelas A", 45.0, 50.0, 50.8);
    assert!(!t2.status_lolos);
    assert!(t2.catatan.contains("Kelebihan berat badan"));
}

#[test]
fn test_arena_silat_anti_fraud_e_certificate() {
    println!(">>> Testing Arena Silat: Anti-Fraud Digital Certificate QR...");

    let no_sertifikat = "SRT-IPSI-2026-00891";
    let atlet_id = "AT-BUDI-01";
    let event_id = "KEJURNAS-SILAT-2026";

    let url_qr = CertificateEngine::buat_url_verifikasi(no_sertifikat, atlet_id, event_id);
    assert!(url_qr.starts_with("https://arenasilat.id/verifikasi/SRT-IPSI-2026-00891/"));

    // Ambil hash dari URL dan verifikasi
    let parts: Vec<&str> = url_qr.split('/').collect();
    let hash_signature = parts.last().unwrap();
    let is_valid = CertificateEngine::verifikasi_keaslian(no_sertifikat, atlet_id, event_id, hash_signature);
    assert!(is_valid);

    // Sertifikat palsu / hash dimanipulasi -> Harus ditolak
    let is_fake_valid = CertificateEngine::verifikasi_keaslian(no_sertifikat, atlet_id, event_id, "hash_palsu_99");
    assert!(!is_fake_valid);
}
