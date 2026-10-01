use widya::koperasi::{
    GolonganObat, JenisAkadPembiayaan, JenisKendaraanLogistik, JenisKomoditasGudang,
    KategoriKomoditasSembako, KoperasiMerahPutihHub, StatusGiziBalita, StatusKeanggotaan,
    StatusPengiriman, UnitKlinikDesa,
};

#[test]
fn test_koperasi_merah_putih_7_unit_usaha_lengkap() {
    let mut hub = KoperasiMerahPutihHub::new(
        "Koperasi Desa Merah Putih Makmur",
        "AHU-0012345.AH.01.26.TAHUN 2026",
        "Desa Sukamaju, Jawa Tengah",
    );

    // ========================================================================
    // 1. UNIT PERKANTORAN & KEANGGOTAAN
    // ========================================================================
    let a1 = hub.unit_perkantoran.daftarkan_anggota(
        "KMP-3301-0001",
        "3301123456780001",
        "Siti Aminah",
        "RT 02 / RW 01",
        "Desa Sukamaju",
        "081234567890",
        500000, // Simpanan Pokok Rp 500.000
    );
    assert_eq!(a1.status, StatusKeanggotaan::AnggotaBiasa);

    let sw = hub.unit_perkantoran.setor_simpanan_wajib("KMP-3301-0001", 50000).expect("SW failed");
    assert_eq!(sw, 50000);

    // Catat belanja transaksi
    hub.unit_perkantoran.catat_transaksi_anggota("KMP-3301-0001", 1200000);

    // Hitung SHU tahunan (SHU kotor Rp 100.000.000)
    let (laporan_shu, alokasi_anggota) = hub.unit_perkantoran.hitung_shu_tahunan(2026, 100_000_000);
    assert_eq!(laporan_shu.dana_cadangan_idr, 25_000_000);
    assert_eq!(laporan_shu.dana_pembangunan_desa_idr, 5_000_000);
    assert_eq!(alokasi_anggota.len(), 1);
    assert!(alokasi_anggota[0].total_shu_diterima_idr > 0);

    // ========================================================================
    // 2. UNIT SIMPAN PINJAM (USP)
    // ========================================================================
    let rek = hub.unit_simpan_pinjam.buka_tabungan("REK-TAB-001", "KMP-3301-0001", "Tabungan Qurban");
    assert_eq!(rek.saldo_idr, 0);

    let saldo = hub.unit_simpan_pinjam.setor_tabungan("REK-TAB-001", 300000).expect("Setor failed");
    assert_eq!(saldo, 300000);

    // Pengajuan Pembiayaan Modal Tani (Murabahah Rp 10.000.000, Tenor 10 Bulan, Margin 6% p.a.)
    let pinjaman = hub.unit_simpan_pinjam.ajukan_pinjaman(
        "PINJ-2026-001",
        "KMP-3301-0001",
        10_000_000,
        10,
        JenisAkadPembiayaan::MurabahahJualBeli,
        6.0,
        "Modal Benih & Pupuk Padi Organik",
    );
    assert_eq!(pinjaman.jadwal_angsuran.len(), 10);
    assert_eq!(pinjaman.jadwal_angsuran[0].pokok_idr, 1_000_000);

    // ========================================================================
    // 3. UNIT WARUNG SEMBAKO (POS RITEL & SUBSIDI NIK)
    // ========================================================================
    let belanja = hub.unit_warung_sembako.proses_transaksi(
        "STRUK-2026-1001",
        Some("KMP-3301-0001"), // Anggota Koperasi
        Some("3301123456780001"), // NIK KTP untuk LPG subsidi
        &[
            ("8991234001", 2.0), // 2 karung Beras Premium 5kg
            ("8991234002", 3.0), // 3 liter Minyakita
            ("8991234003", 1.0), // 1 tabung Gas LPG 3kg
        ],
        "QRIS_MERAH_PUTIH",
    ).expect("Transaksi sembako failed");
    assert_eq!(belanja.items.len(), 3);
    assert!(belanja.total_belanja_idr > 0);
    assert!(belanja.poin_anggota_diperoleh > 0);

    // ========================================================================
    // 4. UNIT KLINIK DESA (RME & PENCEGAHAN STUNTING)
    // ========================================================================
    let stunting_eval = UnitKlinikDesa::evaluasi_stunting_balita(24, 88.5);
    assert_eq!(stunting_eval, StatusGiziBalita::GiziBaik);

    let rm = hub.unit_klinik_desa.catat_pemeriksaan(
        "RM-2026-001",
        "3301123456780001",
        "Siti Aminah",
        "Pusing dan pegal linu",
        "120/80 mmHg",
        Some(105),
        "M79.1 Myalgia",
        vec!["MED-001".to_string(), "MED-003".to_string()],
        None,
    );
    assert_eq!(rm.diagnosis_icd10, "M79.1 Myalgia");
    assert_eq!(hub.unit_klinik_desa.total_kunjungan, 1);

    // ========================================================================
    // 5. UNIT APOTEK DESA (OBAT FEFO & HET KEMENKES)
    // ========================================================================
    let (biaya_obat, batches) = hub.unit_apotek_desa.keluarkan_obat_fefo("MED-001", 10).expect("Apotek failed");
    assert_eq!(biaya_obat, 65000); // 10 x 6500
    assert!(!batches.is_empty());

    // ========================================================================
    // 6. UNIT PERGUDANGAN & RESI GUDANG (SRG)
    // ========================================================================
    let srg = hub.unit_pergudangan.terbitkan_resi_gudang(
        "SRG-KMP-2026-001",
        "KMP-3301-0001",
        "Siti Aminah",
        JenisKomoditasGudang::GabahKeringPanen,
        3500.0, // 3.5 Ton Gabah
        13.2,   // 13.2% Kadar Air (Memenuhi syarat <= 14%)
        6800,   // Rp 6.800 / kg
        "SILO-A1",
    ).expect("SRG failed");
    assert_eq!(srg.nilai_taksiran_pasar_idr, 23_800_000);
    assert_eq!(srg.plafon_pembiayaan_bank_idr, 16_660_000); // 70% Plafon

    let telem_silo = hub.unit_pergudangan.pantau_sensor_silo("SILO-A1", 24.5, 60.0, 450.0);
    assert!(!telem_silo.status_ventilasi_otomatis); // Suhu & kelembaban normal

    // ========================================================================
    // 7. UNIT LOGISTIK, DISTRIBUSI & KURIR DESA
    // ========================================================================
    let waybill = hub.unit_logistik.buat_surat_jalan(
        "KMP-LOG-2026-0001",
        "Gudang Komoditas Koperasi Merah Putih",
        "Pasar Induk Beras Cipinang",
        "B 9488 KMP",
        vec![("Beras Premium 5kg".to_string(), 1200.0, "kg".to_string())],
    ).expect("Surat jalan failed");
    assert_eq!(waybill.status, StatusPengiriman::DalamPerjalanan);

    let konfirmasi = hub.unit_logistik.konfirmasi_penerimaan("KMP-LOG-2026-0001", "TTD-DIGITAL-PENERIMA-PASAR-INDUK").expect("Konfirmasi failed");
    assert!(konfirmasi);
    assert_eq!(hub.unit_logistik.surat_jalan.get("KMP-LOG-2026-0001").unwrap().status, StatusPengiriman::Terkirim);
}
