use widya::projectmgmt::{
    CpmPertEngine, EstimasiDurasiPert, EvmEngine, ManajemenRisikoEngine, PrioritasMoscow,
    SprintScrum, StatusKanban, StrategiResponRisiko, TingkatKeparahanRisiko, UserStory,
    WbsAktivitas,
};

#[test]
fn test_project_management_comprehensive_suite() {
    // =========================================================================
    // 1. CPM (CRITICAL PATH METHOD) & PERT SCHEDULING
    // =========================================================================
    // Aktivitas A (Durasi 5) -> B (Durasi 3) & C (Durasi 6) -> D (Durasi 4, butuh B & C)
    // Jalur A -> C -> D = 5 + 6 + 4 = 15 hari (JALUR KRITIS)
    // Jalur A -> B -> D = 5 + 3 + 4 = 12 hari (Slack B = 3 hari)
    let mut cpm = CpmPertEngine::new("Proyek Transformasi Digital ERP Sekolah");

    cpm.tambah_aktivitas(WbsAktivitas {
        id_aktivitas: "A".to_string(),
        nama_tugas: "Analisis Kebutuhan & Desain Sistem".to_string(),
        durasi_hari: 5.0,
        dependensi_pendahulu: vec![],
        biaya_estimasi_rp: 25_000_000.0,
        penanggung_jawab: "Lead Systems Analyst".to_string(),
        early_start: 0.0,
        early_finish: 0.0,
        late_start: 0.0,
        late_finish: 0.0,
        total_slack_float: 0.0,
        adalah_jalur_kritis: false,
    });

    cpm.tambah_aktivitas(WbsAktivitas {
        id_aktivitas: "B".to_string(),
        nama_tugas: "Pengadaan Perangkat Keras Server".to_string(),
        durasi_hari: 3.0,
        dependensi_pendahulu: vec!["A".to_string()],
        biaya_estimasi_rp: 50_000_000.0,
        penanggung_jawab: "Procurement Specialist".to_string(),
        early_start: 0.0,
        early_finish: 0.0,
        late_start: 0.0,
        late_finish: 0.0,
        total_slack_float: 0.0,
        adalah_jalur_kritis: false,
    });

    cpm.tambah_aktivitas(WbsAktivitas {
        id_aktivitas: "C".to_string(),
        nama_tugas: "Pengembangan Modul Perangkat Lunak".to_string(),
        durasi_hari: 6.0,
        dependensi_pendahulu: vec!["A".to_string()],
        biaya_estimasi_rp: 60_000_000.0,
        penanggung_jawab: "Senior Software Engineer".to_string(),
        early_start: 0.0,
        early_finish: 0.0,
        late_start: 0.0,
        late_finish: 0.0,
        total_slack_float: 0.0,
        adalah_jalur_kritis: false,
    });

    cpm.tambah_aktivitas(WbsAktivitas {
        id_aktivitas: "D".to_string(),
        nama_tugas: "Integrasi Sistem, UAT & Go-Live".to_string(),
        durasi_hari: 4.0,
        dependensi_pendahulu: vec!["B".to_string(), "C".to_string()],
        biaya_estimasi_rp: 20_000_000.0,
        penanggung_jawab: "QA & Release Manager".to_string(),
        early_start: 0.0,
        early_finish: 0.0,
        late_start: 0.0,
        late_finish: 0.0,
        total_slack_float: 0.0,
        adalah_jalur_kritis: false,
    });

    let total_durasi = cpm.hitung_jalur_kritis_cpm().expect("Kalkulasi CPM failed");
    assert_eq!(total_durasi, 15.0); // 5 + 6 + 4 = 15 hari

    // Verifikasi Jalur Kritis (A, C, D)
    assert!(cpm.daftar_id_jalur_kritis.contains(&"A".to_string()));
    assert!(cpm.daftar_id_jalur_kritis.contains(&"C".to_string()));
    assert!(cpm.daftar_id_jalur_kritis.contains(&"D".to_string()));
    assert!(!cpm.daftar_id_jalur_kritis.contains(&"B".to_string())); // B bukan jalur kritis

    // Verifikasi Float Aktivitas B = 3.0 hari
    let tugas_b = cpm.daftar_tugas.iter().find(|t| t.id_aktivitas == "B").unwrap();
    assert_eq!(tugas_b.total_slack_float, 3.0);

    // PERT 3-Point Estimate: Optimis 4, Paling Mungkin 6, Pesimis 14 -> TE = (4 + 24 + 14)/6 = 7.0 hari
    let pert = EstimasiDurasiPert {
        durasi_optimis_hari: 4.0,
        durasi_paling_mungkin: 6.0,
        durasi_pesimis_hari: 14.0,
    };
    assert_eq!(pert.hitung_expected_duration(), 7.0);

    // =========================================================================
    // 2. EARNED VALUE MANAGEMENT (EVM) METRICS
    // Skenario: BAC = 200 Juta, PV = 100 Juta, EV = 110 Juta, AC = 100 Juta
    // =========================================================================
    let evm = EvmEngine::evaluasi_kinerja_evm(
        200_000_000.0, // BAC
        100_000_000.0, // PV
        110_000_000.0, // EV
        100_000_000.0, // AC
    );

    assert_eq!(evm.cost_variance_cv_rp, 10_000_000.0);       // CV = EV - AC = +10 Juta (Hemat)
    assert_eq!(evm.schedule_variance_sv_rp, 10_000_000.0);   // SV = EV - PV = +10 Juta (Ahead)
    assert_eq!(evm.cost_performance_index_cpi, 1.10);        // CPI = 1.10 (> 1.0)
    assert_eq!(evm.schedule_performance_index_spi, 1.10);    // SPI = 1.10 (> 1.0)
    assert!((evm.estimate_at_completion_eac_rp - 181_818_181.81).abs() < 1.0); // EAC = 200M / 1.10
    assert!(evm.status_kesehatan_proyek.contains("SANGAT BAIK"));

    // =========================================================================
    // 3. AGILE SCRUM SPRINTS & BURNDOWN
    // =========================================================================
    let mut sprint = SprintScrum::new(1, "Sprint 1: Modul Otentikasi & RBAC", "2026-03-01", "2026-03-14");

    sprint.tambah_story(UserStory {
        id_story: "US-01".to_string(),
        judul: "Autentikasi SSO OAuth2 / OIDC".to_string(),
        sebagai_peran: "Pengguna Aplikasi".to_string(),
        saya_ingin: "Login menggunakan akun Google / Kemendikdasmen".to_string(),
        sehingga_tujuan: "Akses sistem lebih cepat dan aman".to_string(),
        story_points: 5,
        prioritas: PrioritasMoscow::MustHave,
        status: StatusKanban::InProgress,
        assignee: Some("Rian Dev".to_string()),
    });

    sprint.tambah_story(UserStory {
        id_story: "US-02".to_string(),
        judul: "Manajemen Hak Akses RBAC Role-Based".to_string(),
        sebagai_peran: "Administrator Sistem".to_string(),
        saya_ingin: "Mengatur izin akses menu per role pengguna".to_string(),
        sehingga_tujuan: "Keamanan hak akses terisolasi".to_string(),
        story_points: 8,
        prioritas: PrioritasMoscow::MustHave,
        status: StatusKanban::ToDo,
        assignee: Some("Siti Dev".to_string()),
    });

    assert_eq!(sprint.target_komitmen_points, 13); // 5 + 8 = 13 Points
    assert_eq!(sprint.sisa_points_burndown(), 13);

    // Selesaikan US-01 (5 Points)
    assert!(sprint.selesaikan_story("US-01").is_ok());
    assert_eq!(sprint.points_terselesaikan, 5);
    assert_eq!(sprint.sisa_points_burndown(), 8);

    // =========================================================================
    // 4. MANAJEMEN RISIKO (5X5 RISK MATRIX & CONTINGENCY)
    // =========================================================================
    let mut risk_mgr = ManajemenRisikoEngine::new();

    let r1 = risk_mgr.identifikasi_risiko(
        "RSK-01",
        "Keterlambatan pasokan chip server impor dari vendor",
        "Vendor",
        4, // Probabilitas 4 (Tinggi)
        4, // Dampak 4 (Tinggi) -> Skor = 16 (Ekstrem)
        StrategiResponRisiko::Mitigasi,
        "Menyediakan secondary dual-vendor lokal dan pre-order 2 bulan sebelumnya",
        "Procurement Manager",
        15_000_000.0,
    );
    assert_eq!(r1.skor_risiko, 16);
    assert_eq!(r1.level_risiko, TingkatKeparahanRisiko::Ekstrem);

    let r2 = risk_mgr.identifikasi_risiko(
        "RSK-02",
        "Ketidakstabilan jaringan internet satelit saat live deployment",
        "Teknis",
        2, // Probabilitas 2 (Jarang)
        3, // Dampak 3 (Sedang) -> Skor = 6 (Sedang)
        StrategiResponRisiko::Mitigasi,
        "Menggunakan dual-WAN failover dengan jaringan seluler 5G lokal",
        "Network Engineer",
        5_000_000.0,
    );
    assert_eq!(r2.skor_risiko, 6);
    assert_eq!(r2.level_risiko, TingkatKeparahanRisiko::Sedang);

    assert_eq!(risk_mgr.total_dana_cadangan_kontinjensi(), 20_000_000.0);
}
