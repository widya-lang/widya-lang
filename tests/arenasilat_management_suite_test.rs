// tests/arenasilat_management_suite_test.rs
// ==============================================================================
// ARENA SILAT SAAS — TOURNAMENT MANAGEMENT INTEGRATION TEST
// 100% Zero External Dependencies — Verified on Widya-Lang Ecosystem
// ==============================================================================

#[path = "../Projects/arenasilat/src/pendaftaran.rs"]
mod pendaftaran;
#[path = "../Projects/arenasilat/src/kategori_and_drawing.rs"]
mod kategori_and_drawing;
#[path = "../Projects/arenasilat/src/laporan_and_rekap.rs"]
mod laporan_and_rekap;
#[path = "../Projects/arenasilat/src/scoring_engine.rs"]
mod scoring_engine;
#[path = "../Projects/arenasilat/src/bracket_generator.rs"]
mod bracket_generator;
#[path = "../Projects/arenasilat/src/var_and_hardware.rs"]
mod var_and_hardware;

use pendaftaran::{RegistrationEngine, KategoriUsia, JenisKelamin, JenisKompetisi};
use kategori_and_drawing::CategoryDrawingEngine;
use laporan_and_rekap::{ReportGenerator, RekapMedaliKontingen, NominasiPesilatTerbaik};

#[test]
fn test_tournament_registration_and_financial_flow() {
    println!(">>> Testing Arena Silat: Registration & Financial Flow...");
    let mut reg = RegistrationEngine::baru("EVENT-01", "Kejurda Silat Nusantara 2026");

    // 1. Daftarkan Kontingen
    reg.daftarkan_kontingen("KTG-A", "Tapak Suci", "Yogyakarta", "Pak Guru", "08123456", "ts@silat.id");
    reg.daftarkan_kontingen("KTG-B", "PSHT", "Madiun", "Kang Mas", "08129999", "psht@silat.id");

    // 2. Daftarkan Atlet
    let res1 = reg.daftarkan_atlet(
        "ATL-01", "KTG-A", "Fajar Nugraha", JenisKelamin::Putra, "2005-01-01", 19,
        66.5, 172.0, KategoriUsia::Dewasa, JenisKompetisi::Tanding("Kelas E".to_string())
    );
    assert!(res1.is_ok());

    let res2 = reg.daftarkan_atlet(
        "ATL-02", "KTG-B", "Bayu Pratama", JenisKelamin::Putra, "2004-03-15", 20,
        67.0, 174.0, KategoriUsia::Dewasa, JenisKompetisi::Tanding("Kelas E".to_string())
    );
    assert!(res2.is_ok());

    // 3. Verifikasi berkas atlet
    assert!(reg.verifikasi_atlet("ATL-01"));
    assert!(reg.verifikasi_atlet("ATL-02"));

    // 4. Konfirmasi Pembayaran
    assert!(reg.konfirmasi_pembayaran_kontingen("KTG-A"));
    assert!(reg.konfirmasi_pembayaran_kontingen("KTG-B"));

    let rekap = reg.hitung_rekapitulasi();
    assert_eq!(rekap.total_kontingen, 2);
    assert_eq!(rekap.total_atlet, 2);
    assert_eq!(rekap.terverifikasi, 2);
    assert!(rekap.total_pendapatan_idr > 0);
    println!("    [PASS] Registration & Financial flow verified! Total Pendapatan: Rp {}", rekap.total_pendapatan_idr);
}

#[test]
fn test_category_classification_and_drawing() {
    println!(">>> Testing Arena Silat: Category Classification & Drawing...");
    let mut reg = RegistrationEngine::baru("EVENT-02", "Kejuaraan Mahasiswa");
    reg.daftarkan_kontingen("KTG-1", "Merpati Putih", "Jakarta", "Off 1", "0811", "mp@silat.id");
    reg.daftarkan_kontingen("KTG-2", "Perisai Diri", "Surabaya", "Off 2", "0822", "pd@silat.id");

    reg.daftarkan_atlet("A1", "KTG-1", "Ali", JenisKelamin::Putra, "2003-01-01", 21, 68.0, 175.0, KategoriUsia::Dewasa, JenisKompetisi::Tanding("Kelas E".to_string())).unwrap();
    reg.daftarkan_atlet("A2", "KTG-1", "Amir", JenisKelamin::Putra, "2003-02-01", 21, 69.0, 174.0, KategoriUsia::Dewasa, JenisKompetisi::Tanding("Kelas E".to_string())).unwrap();
    reg.daftarkan_atlet("A3", "KTG-2", "Budi", JenisKelamin::Putra, "2003-03-01", 21, 67.5, 172.0, KategoriUsia::Dewasa, JenisKompetisi::Tanding("Kelas E".to_string())).unwrap();
    reg.daftarkan_atlet("A4", "KTG-2", "Bagas", JenisKelamin::Putra, "2003-04-01", 21, 68.5, 176.0, KategoriUsia::Dewasa, JenisKompetisi::Tanding("Kelas E".to_string())).unwrap();

    reg.verifikasi_atlet("A1");
    reg.verifikasi_atlet("A2");
    reg.verifikasi_atlet("A3");
    reg.verifikasi_atlet("A4");

    let atlet_vec: Vec<pendaftaran::AtletRegistrasi> = reg.daftar_atlet.values().cloned().collect();
    let grouped = CategoryDrawingEngine::kelompokkan_atlet(&atlet_vec);
    assert_eq!(grouped.len(), 1); // 1 Kategori: Dewasa Putra Kelas E

    let peserta_kelas = grouped.values().next().unwrap();
    assert_eq!(peserta_kelas.len(), 4);

    let (pool_a, pool_b) = CategoryDrawingEngine::lakukan_drawing_kategori(peserta_kelas);
    assert_eq!(pool_a.len(), 2);
    assert_eq!(pool_b.len(), 2);
    println!("    [PASS] Category classification & drawing pool balanced verified!");
}

#[test]
fn test_medal_standings_and_best_athlete_award() {
    println!(">>> Testing Arena Silat: Medal Standings & Best Athlete Award...");

    let daftar_medali = vec![
        RekapMedaliKontingen { kontingen_id: "K1".to_string(), nama_kontingen: "Jateng".to_string(), emas: 5, perak: 2, perunggu: 1, total_poin_klasemen: 0 },
        RekapMedaliKontingen { kontingen_id: "K2".to_string(), nama_kontingen: "Jatim".to_string(), emas: 3, perak: 4, perunggu: 2, total_poin_klasemen: 0 },
        RekapMedaliKontingen { kontingen_id: "K3".to_string(), nama_kontingen: "Jabar".to_string(), emas: 6, perak: 1, perunggu: 0, total_poin_klasemen: 0 },
    ];

    let sorted = ReportGenerator::urutkan_klasemen_medali(daftar_medali);
    assert_eq!(sorted[0].nama_kontingen, "Jabar"); // 6 Emas -> Peringkat 1
    assert_eq!(sorted[1].nama_kontingen, "Jateng"); // 5 Emas -> Peringkat 2
    assert_eq!(sorted[2].nama_kontingen, "Jatim"); // 3 Emas -> Peringkat 3

    let nominasi = vec![
        NominasiPesilatTerbaik { atlet_id: "A1".to_string(), nama_atlet: "Pesilat Hebat".to_string(), kontingen: "Jabar".to_string(), kategori: "Kelas E".to_string(), total_poin_teknik: 30, total_jatuhan: 4, total_tendangan: 7, jumlah_pelanggaran: 0, skor_indeks_kemenangan: 0.0 },
        NominasiPesilatTerbaik { atlet_id: "A2".to_string(), nama_atlet: "Pesilat Biasa".to_string(), kontingen: "Jateng".to_string(), kategori: "Kelas E".to_string(), total_poin_teknik: 15, total_jatuhan: 1, total_tendangan: 2, jumlah_pelanggaran: 2, skor_indeks_kemenangan: 0.0 },
    ];

    let best = ReportGenerator::tentukan_pesilat_terbaik(nominasi).unwrap();
    assert_eq!(best.nama_atlet, "Pesilat Hebat");
    println!("    [PASS] Medal standings & Best athlete determination verified!");
}
