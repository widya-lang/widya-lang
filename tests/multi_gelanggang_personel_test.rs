// tests/multi_gelanggang_personel_test.rs
// ==============================================================================
// ARENA SILAT SAAS — MULTI-GELANGGANG & ROSTER PERSONEL INTEGRATION TEST
// 100% Zero External Dependencies — Verified on Widya-Lang Ecosystem
// ==============================================================================

#[path = "../Projects/arenasilat/src/multi_gelanggang_personel.rs"]
mod multi_gelanggang_personel;

use multi_gelanggang_personel::{MultiCourtManager, StatusGelanggang};

#[test]
fn test_multi_gelanggang_scaling_and_personnel_roster() {
    println!(">>> Testing Arena Silat: Multi-Court Scaling (6 Gelanggang) & Personnel Roster...");

    let mut manager = MultiCourtManager::baru("TENANT-KEJURNAS-01", "Kejurnas Pencak Silat Indonesia 2026");

    // 1. Daftarkan 6 Gelanggang Serentak (A, B, C, D, E, F)
    manager.tambah_gelanggang(1, "Gelanggang A", "Matras Utama Barat");
    manager.tambah_gelanggang(2, "Gelanggang B", "Matras Utama Timur");
    manager.tambah_gelanggang(3, "Gelanggang C", "Matras Sayap Barat");
    manager.tambah_gelanggang(4, "Gelanggang D", "Matras Sayap Timur");
    manager.tambah_gelanggang(5, "Gelanggang E", "Matras Pemasalan 1");
    manager.tambah_gelanggang(6, "Gelanggang F", "Matras Pemasalan 2");

    assert_eq!(manager.daftar_gelanggang.len(), 6);

    // 2. Daftarkan Database Personel Wasit Juri & Medis
    manager.daftarkan_personel("P-KP-01", "Drs. H. Mulyono", "WJ-NAS-01", "Wasit Juri Nasional 1", "Jateng", "081234");
    manager.daftarkan_personel("P-WASIT-01", "Ahmad Fauzi", "WJ-NAS-02", "Wasit Juri Nasional 2", "Jatim", "081235");
    manager.daftarkan_personel("P-JURI-01", "Budi Santoso", "WJ-DAERAH-01", "Daerah", "Jabar", "081236");
    manager.daftarkan_personel("P-JURI-02", "Cecep Supriatna", "WJ-DAERAH-02", "Daerah", "Banten", "081237");
    manager.daftarkan_personel("P-JURI-03", "Dedi Kurniawan", "WJ-DAERAH-03", "Daerah", "DKI", "081238");
    manager.daftarkan_personel("P-TIMER-01", "Operator Dimas", "OP-01", "Operator IT", "Jateng", "081239");
    manager.daftarkan_personel("P-MEDIS-01", "dr. Anita Rahayu", "MED-01", "Dokter Olahraga", "RSUD", "081240");

    // 3. Pasang Roster Tugas ke Gelanggang A (Gelanggang 1)
    let res = manager.tetapkan_roster_gelanggang(
        1, "P-KP-01", "P-WASIT-01", &["P-JURI-01", "P-JURI-02", "P-JURI-03"], "P-TIMER-01", "P-MEDIS-01", "Shift Pagi (08:00 - 12:00)"
    );
    assert!(res.is_ok());

    let g1 = manager.daftar_gelanggang.get(&1).unwrap();
    assert_eq!(g1.roster_tugas.ketua_pertandingan.as_ref().unwrap().nama_lengkap, "Drs. H. Mulyono");
    assert_eq!(g1.roster_tugas.juri_list.len(), 3);
    assert_eq!(g1.roster_tugas.tim_medis.as_ref().unwrap().nama_lengkap, "dr. Anita Rahayu");

    println!("    [PASS] 6 Gelanggang created & Gelanggang A personnel roster verified!");
}

#[test]
fn test_dynamic_court_load_balancing() {
    println!(">>> Testing Arena Silat: Dynamic Court Load Balancing across Arenas...");

    let mut manager = MultiCourtManager::baru("TENANT-KEJURPROV-01", "Kejurprov Jawa Tengah");
    manager.tambah_gelanggang(1, "Gelanggang A", "Matras 1");
    manager.tambah_gelanggang(2, "Gelanggang B", "Matras 2");

    // Gelanggang A padat memiliki 5 partai antrean
    for p in 1..=5 {
        manager.alokasikan_partai(1, p).unwrap();
    }
    // Gelanggang B hanya memiliki 1 partai
    manager.alokasikan_partai(2, 6).unwrap();

    assert_eq!(manager.daftar_gelanggang.get(&1).unwrap().daftar_antrean_partai.len(), 5);
    assert_eq!(manager.daftar_gelanggang.get(&2).unwrap().daftar_antrean_partai.len(), 1);

    // Dynamic Load Balancing: Pindahkan partai paling belakang dari Gelanggang A ke Gelanggang B
    let partai_dipindah = manager.pindahkan_partai_penyeimbang(1, 2).unwrap();
    assert_eq!(partai_dipindah, 5);

    assert_eq!(manager.daftar_gelanggang.get(&1).unwrap().daftar_antrean_partai.len(), 4);
    assert_eq!(manager.daftar_gelanggang.get(&2).unwrap().daftar_antrean_partai.len(), 2);
    assert_eq!(manager.daftar_gelanggang.get(&2).unwrap().daftar_antrean_partai[1], 5);

    println!("    [PASS] Dynamic court load balancing successfully shifted match #5 from Gelanggang A to Gelanggang B!");
}
