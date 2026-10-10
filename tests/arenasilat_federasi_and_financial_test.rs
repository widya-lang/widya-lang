// tests/arenasilat_federasi_and_financial_test.rs
// ==============================================================================
// ARENA SILAT SAAS — FEDERASI, PAYMENT, AI BIOMECHANICS & ASURANSI TEST SUITE
// 100% Zero External Dependencies — Verified on Widya-Lang Ecosystem
// ==============================================================================

#[path = "../Projects/arenasilat/src/federasi_and_perizinan.rs"]
mod federasi_and_perizinan;
#[path = "../Projects/arenasilat/src/payment_and_split_settlement.rs"]
mod payment_and_split_settlement;
#[path = "../Projects/arenasilat/src/ai_biomekanik_and_cv.rs"]
mod ai_biomekanik_and_cv;
#[path = "../Projects/arenasilat/src/asuransi_and_klaim_bpjs.rs"]
mod asuransi_and_klaim_bpjs;
#[path = "../Projects/arenasilat/src/portal_publik_and_h2h.rs"]
mod portal_publik_and_h2h;

use federasi_and_perizinan::{
    FederasiEngine, TingkatFederasi, TingkatLisensiWasitJuri, WasitJuriTervalidasi, StatusIzinTurnamen,
};
use payment_and_split_settlement::PaymentEngine;
use ai_biomekanik_and_cv::{
    AIVisionAndBiomechanics, JenisTeknikSilat, ZonaSasaranSerangan,
};
use asuransi_and_klaim_bpjs::{
    InsuranceEngine, TingkatCederaKlaim, StatusKlaimMedis,
};
use portal_publik_and_h2h::{
    PublicPortalEngine, PertandinganPublikLive,
};

#[test]
fn test_federasi_licensing_and_tournament_audit() {
    println!(">>> Testing Arena Silat: IPSI Federation Licensing & Audit...");

    // 1. Validasi Wasit Juri
    let wasit_nasional = WasitJuriTervalidasi {
        lisensi_id: "WJ-NAS-2024-001".to_string(),
        nama_lengkap: "Drs. H. Mulyono, M.Pd.".to_string(),
        tingkat_lisensi: TingkatLisensiWasitJuri::WasitJuriNasionalKelas1,
        perguruan_asal: "Tapak Suci".to_string(),
        pengprov_asal: "Jawa Tengah".to_string(),
        tahun_kelulusan_penataran: 2024,
        masa_berlaku_sampai: 2028,
        total_partai_dipimpin: 142,
        status_skorsing: false,
    };

    let (sah_kejurnas, msg) = FederasiEngine::validasi_kelayakan_wasit(
        &wasit_nasional,
        2026,
        &TingkatFederasi::PbIpsi,
    );
    assert!(sah_kejurnas, "{}", msg);

    // 2. Audit Sarana Gelanggang
    let audit = FederasiEngine::audit_sarana_dan_kelayakan(
        "KEJURNAS-2026",
        true, // Matras standar
        4,    // 4 Gelanggang
        true, // Tim dokter siap
        true, // Ambulans standby
        true, // Timbangan digital BLE
        true, // Wasit juri cukup
    );
    assert!(audit.status_rekomendasi_kelayakan);
    assert_eq!(audit.skor_kelayakan_persen, 100.0);

    // 3. Surat Rekomendasi
    let surat = FederasiEngine::proses_surat_rekomendasi(
        "REK-PB-IPSI/X/2026/089",
        "KEJURNAS-2026",
        "Kejuaraan Nasional Pencak Silat Indonesia 2026",
        "Panitia PB IPSI",
        TingkatFederasi::PbIpsi,
        "GOR Among Rogo Yogyakarta",
        1200,
        true,
    );
    assert_eq!(surat.status_izin, StatusIzinTurnamen::DisetujuiRekomendasi);
    println!("    [PASS] IPSI Federation Licensing & Venue Audit verified!");
}

#[test]
fn test_payment_split_settlement_and_protest_deposit() {
    println!(">>> Testing Arena Silat: Payment Gateway & Split Settlement...");

    // 1. Split settlement pendaftaran 20 atlet (Biaya Rp 350.000 / atlet = Rp 7.000.000)
    let total_tagihan = 7_000_000;
    let split = PaymentEngine::hitung_split_settlement("INV-KONT-001", total_tagihan, 20);

    assert_eq!(split.porsi_platform_saas_idr, 20 * 15_000); // Rp 300.000
    assert_eq!(split.iuran_kas_federasi_ipsi_idr, 20 * 10_000); // Rp 200.000
    assert_eq!(split.biaya_gerbang_pembayaran_idr, 3_500);
    assert_eq!(split.porsi_eo_panitia_idr, 7_000_000 - 300_000 - 200_000 - 3_500); // Rp 6.496.500

    // 2. Deposit Banding Protes Dewan Hakim (Rp 3 Juta)
    let deposit = PaymentEngine::ajukan_deposit_banding("BANDING-01", "KONT-JATENG", "PARTAI-A-04");
    assert_eq!(deposit.nominal_deposit_idr, 3_000_000);

    // Kasus 1: Protes DITERIMA -> Refund 100%
    let deposit_diterima = PaymentEngine::selesaikan_deposit_banding(deposit.clone(), true);
    assert!(deposit_diterima.status_kembali_dana);

    // Kasus 2: Protes DITOLAK -> Hangus
    let deposit_ditolak = PaymentEngine::selesaikan_deposit_banding(deposit, false);
    assert!(!deposit_ditolak.status_kembali_dana);

    // 3. QRIS Payload
    let qris = PaymentEngine::generate_qris_payload("INV-KONT-001", total_tagihan, "ARENA SILAT SAAS");
    assert!(qris.contains("INV-KONT-001"));
    println!("    [PASS] Payment Split Settlement & Protest Deposit verified!");
}

#[test]
fn test_ai_pose_estimation_and_biomechanical_analytics() {
    println!(">>> Testing Arena Silat: Computer Vision & Biomechanics...");

    // 1. Serangan Sah ke Body Protector Dada
    let strike_sah = AIVisionAndBiomechanics::analisis_legalitas_serangan(
        "Merah",
        JenisTeknikSilat::TendanganSabit,
        ZonaSasaranSerangan::BodyProtectorDada,
        14.2, // 14.2 m/s
        180.0, // 180 ms reaksi lawan
        0.5,  // tangkapan < 2s
        0.98,
    );
    assert!(strike_sah.is_serangan_sah);
    assert!(!strike_sah.tarikan_terlarang_terdeteksi);

    // 2. Serangan Terlarang (Tendangan ke Leher/Kepala)
    let strike_foul = AIVisionAndBiomechanics::analisis_legalitas_serangan(
        "Biru",
        JenisTeknikSilat::TendanganT,
        ZonaSasaranSerangan::LeherKepala,
        15.5,
        120.0,
        0.0,
        0.95,
    );
    assert!(!strike_foul.is_serangan_sah);

    // 3. Profil Biomotorik
    let speeds = vec![12.5, 14.0, 15.2, 13.8];
    let reactions = vec![190.0, 175.0, 182.0, 168.0];
    let profil = AIVisionAndBiomechanics::hitung_performa_biomotorik(
        "ATL-001",
        "Eko Prasetyo",
        10,
        8,
        &speeds,
        &reactions,
    );
    assert_eq!(profil.akurasi_serangan_masuk_persen, 80.0);
    assert!(profil.daya_ledak_tendangan_watt > 5000.0);
    println!("    [PASS] AI Computer Vision & Biomechanics verified!");
}

#[test]
fn test_insurance_coverage_and_bpjs_claim_issuance() {
    println!(">>> Testing Arena Silat: BPJS Health Protection & Hospital Referral...");

    // 1. Polis Atlet
    let polis = InsuranceEngine::daftarkan_polis_atlet(
        "0001234567890",
        "ATL-001",
        "Eko Prasetyo",
        "Kontingen Jawa Tengah",
    );
    assert!(polis.status_aktif);
    assert_eq!(polis.plafon_maksimal_idr, 50_000_000);

    // 2. Penerbitan Klaim Rujukan RSUD
    let berkas_klaim = InsuranceEngine::terbitkan_klaim_rujukan_rsud(
        "KLM-2026-004",
        &polis,
        "RSUD dr. Soedirman Kebumen",
        TingkatCederaKlaim::SedangDislokasiSprain,
        "Dislokasi Articulatio Genu Sinistra",
        "dr. Anita Rahayu, Sp.KO",
        "STR-DOKTER-3301.99281",
        8_500_000,
    );
    assert_eq!(berkas_klaim.status_klaim, StatusKlaimMedis::CoverPenuhDisetujui);
    assert_eq!(berkas_klaim.biaya_ditanggung_idr, 8_500_000);
    assert!(berkas_klaim.qr_verifikasi_klaim.contains("KLM-2026-004"));
    println!("    [PASS] BPJS Insurance & Green Line Hospital Referral verified!");
}

#[test]
fn test_public_portal_h2h_and_live_ticker() {
    println!(">>> Testing Arena Silat: Public Portal H2H & Live Ticker...");

    // 1. Head to Head Analytics
    let h2h = PublicPortalEngine::kalkulasi_h2h(
        "FINAL-KELAS-E",
        "Eko Prasetyo", "Jawa Tengah", 1620.0, 18, 2, "Tendangan Sabit Cepat",
        "Bambang Sudarsono", "Jawa Timur", 1540.0, 14, 4, "Bantingan Tangkapan",
    );

    assert!(h2h.win_probability_merah_persen > 50.0);
    assert_eq!(h2h.win_probability_merah_persen + h2h.win_probability_biru_persen, 100.0);

    // 2. Live Match Ticker
    let live_match = PertandinganPublikLive {
        nomor_partai: "PARTAI-FINAL-01".to_string(),
        gelanggang_id: "GEL-A".to_string(),
        gelanggang_nama: "Gelanggang A".to_string(),
        estimasi_jam_mulai: "14:30 WIB".to_string(),
        kategori_tanding: "Tanding Dewasa Putra Kelas E".to_string(),
        nama_atlet_merah: "Eko Prasetyo".to_string(),
        kontingen_merah: "Jateng".to_string(),
        skor_merah: 14,
        nama_atlet_biru: "Bambang Sudarsono".to_string(),
        kontingen_biru: "Jatim".to_string(),
        skor_biru: 9,
        babak_ronde_ke: 3,
        sisa_waktu_detik: 45,
        status_pertandingan: "Sedang Berlangsung".to_string(),
    };

    let ticker = PublicPortalEngine::format_live_ticker(&live_match);
    assert!(ticker.contains("Gelanggang A"));
    assert!(ticker.contains("Eko Prasetyo"));
    assert!(ticker.contains("14"));
    println!("    [PASS] Public Portal H2H & Live Ticker verified!");
}
