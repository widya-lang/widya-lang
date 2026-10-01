use widya::arkas::{
    ArkasSekolahEngine, BukuKasUmumSekolah, DokumenRkasTahunan, ItemKegiatanRkas,
    JenisBanperApbn, JenisSumberDana, KelompokBelanjaAkuntansi, LaporanSpjMarkasBulanan,
    ManajemenMarkasDinas, PerjanjianKerjaSamaBanper, PihakKeduaPenerima, PihakPertamaPPK,
    RekapRealisasiPerSumberDana, RekeningBankSekolah, StandarNasionalPendidikan,
    StatusPksBanper, SumberDanaAlokasi,
};

#[test]
fn test_arkas_multi_sumber_dana_dan_pks_banper_apbn() {
    let rekening = RekeningBankSekolah {
        nomor_rekening: "140-00-9876543-2".to_string(),
        nama_bank: "Bank BPD Jateng Cabang Banyumas".to_string(),
        nama_pemilik_rekening: "BOS SDN 1 Somagede".to_string(),
        cabang_bank: "KCP Somagede".to_string(),
        npwp_sekolah: "00.123.456.7-521.000".to_string(),
    };

    let mut arkas = ArkasSekolahEngine::new(
        "20301234",
        "SD Negeri 1 Somagede",
        2026,
        rekening,
    );

    // =========================================================================
    // 1. PENGELOLAAN MULTI-SUMBER DANA PENDIDIKAN
    // =========================================================================
    // a. BOSP Reguler (250 siswa x Rp 900.000/thn = Rp 225.000.000)
    let mut bosp_reguler = SumberDanaAlokasi::new(
        "DANA-BOSP-2026-T1",
        "20301234",
        2026,
        JenisSumberDana::BospReguler,
        "SK-DIRJEN-BOSP-2026-0012",
        1,
        112_500_000.0, // Tahap 1 (50%)
        None,
    );
    bosp_reguler.cairkan_dana("2026-01-15", 112_500_000.0).unwrap();
    arkas.tambah_sumber_dana(bosp_reguler);

    // b. BOS Kinerja (Prestasi Rapor Pendidikan Mutu Terbaik)
    let mut bos_kinerja = SumberDanaAlokasi::new(
        "DANA-BOSKIN-2026",
        "20301234",
        2026,
        JenisSumberDana::BosKinerja,
        "SK-MENDIKDASMEN-KINERJA-089",
        1,
        45_000_000.0,
        None,
    );
    bos_kinerja.cairkan_dana("2026-03-10", 45_000_000.0).unwrap();
    arkas.tambah_sumber_dana(bos_kinerja);

    // c. BOSDA Kabupaten Banyumas (Pendampingan APBD)
    let mut bosda = SumberDanaAlokasi::new(
        "DANA-BOSDA-2026-T1",
        "20301234",
        2026,
        JenisSumberDana::BosdaKabupatenKota,
        "PERBUP-BANYUMAS-BOSDA-04",
        1,
        25_000_000.0,
        None,
    );
    bosda.cairkan_dana("2026-02-01", 25_000_000.0).unwrap();
    arkas.tambah_sumber_dana(bosda);

    // =========================================================================
    // 2. PERJANJIAN KERJA SAMA (PKS) BANTUAN PEMERINTAH (BANPER) APBN
    // =========================================================================
    let pihak_ppk = PihakPertamaPPK {
        nama_pejabat_ppk: "Dr. H. Bambang Irawan, M.Pd".to_string(),
        nip_ppk: "197505121999031002".to_string(),
        jabatan: "PPK Direktorat Sekolah Dasar Ditjen PAUD Dikdasmen".to_string(),
        satuan_kerja_kementerian: "Kemendikdasmen Republik Indonesia".to_string(),
        alamat_kantor: "Gedung E Kemendikbudristek, Senayan, Jakarta Pusat".to_string(),
    };

    let pihak_sekolah = PihakKeduaPenerima {
        npsn: "20301234".to_string(),
        nama_sekolah: "SD Negeri 1 Somagede".to_string(),
        nama_kepala_sekolah_atau_ketua: "Drs. Slamet Riyadi, M.Pd".to_string(),
        nip_atau_nik: "196908151994121001".to_string(),
        jabatan: "Kepala Satuan Pendidikan".to_string(),
        nomor_rekening_khusus_banper: "140-00-9876543-9".to_string(),
        nama_bank_penyalur: "Bank BPD Jateng".to_string(),
    };

    let mut pks = PerjanjianKerjaSamaBanper::buat_pks(
        "PKS-BANPER-APBN/2026/0412",
        "421.2/088/PKS-SDN1/2026",
        "2026-02-10",
        2026,
        JenisBanperApbn::DigitalisasiSmartClassroom,
        pihak_ppk,
        pihak_sekolah,
        150_000_000.0, // Total Bantuan Rp 150 Juta
    );

    // Penandatanganan PKS & Pakta Integritas (Bisa oleh Direktorat Pusat atau Delegasi BPMP / Dinas Pendidikan)
    let otoritas_pusat = widya::arkas::OtoritasPengesahanApbn::DirektoratPusat {
        nama_direktorat: "Direktorat Sekolah Dasar Ditjen PAUD Dikdasmen".to_string(),
        nama_pejabat_direktur_atau_ppk: "Dr. H. Bambang Irawan, M.Pd".to_string(),
        nip_pejabat: "197505121999031002".to_string(),
    };
    pks.tandatangani_pks(true, true, otoritas_pusat).expect("Tanda tangan PKS failed");
    assert_eq!(pks.status, StatusPksBanper::DitandatanganiParaPihak);

    // Realisasi Pencairan Termin 1 (70% = Rp 105.000.000) via SP2D KPPN
    let cair_t1 = pks.realisasi_termin_1("2026-02-20", "SP2D-KPPN-2026-00445").unwrap();
    assert_eq!(cair_t1, 105_000_000.0);
    assert_eq!(pks.status, StatusPksBanper::TerminPertamaDicairkan);

    // Tambah sumber dana Banper APBN ke ARKAS
    let mut banper_alokasi = SumberDanaAlokasi::new(
        "DANA-BANPER-APBN-SMARTCLASS",
        "20301234",
        2026,
        JenisSumberDana::BantuanPemerintahApbn,
        "PKS-BANPER-APBN/2026/0412",
        1,
        150_000_000.0,
        Some("PKS-BANPER-APBN/2026/0412".to_string()),
    );
    banper_alokasi.cairkan_dana("2026-02-20", 105_000_000.0).unwrap();
    arkas.tambah_sumber_dana(banper_alokasi);
    arkas.tambah_pks_banper(pks);

    // Total dana masuk ke sekolah
    assert_eq!(arkas.total_dana_cair_keseluruhan(), 287_500_000.0);

    // =========================================================================
    // 3. PERENCANAAN RKAS & VALIDASI JUKNIS (8 SNP)
    // =========================================================================
    let mut rkas = DokumenRkasTahunan::new("RKAS-2026-01", "20301234", "SD Negeri 1 Somagede", 2026);

    // a. Honor Guru Honorer (SNP 5 - Batas juknis <= 50% BOSP)
    rkas.tambah_kegiatan(ItemKegiatanRkas {
        id_kegiatan: "KEG-HONOR-01".to_string(),
        kode_kegiatan_snp: "05.01.01".to_string(),
        standar_snp: StandarNasionalPendidikan::StandarPendidikDanTendik,
        uraian_kegiatan: "Honorarium Guru Tidak Tetap (GTT) Non-ASN 3 Orang".to_string(),
        kode_rekening: "5.1.01.02.01.0001".to_string(),
        kelompok_belanja: KelompokBelanjaAkuntansi::BelanjaPegawaiHonor,
        sumber_dana: JenisSumberDana::BospReguler,
        volume: 12.0,
        satuan: "Bulan".to_string(),
        tarif_harga_satuan_rp: 3_000_000.0,
        total_anggaran_tahunan_rp: 36_000_000.0, // 36 Juta dari 112.5 Juta BOSP (32.0% < 50%) -> Lolos
        alokasi_triwulan_1_rp: 9_000_000.0,
        alokasi_triwulan_2_rp: 9_000_000.0,
        alokasi_triwulan_3_rp: 9_000_000.0,
        alokasi_triwulan_4_rp: 9_000_000.0,
        total_realisasi_rp: 0.0,
    });

    // Item tambahan BOSP: Pembelajaran & Asesmen (Rp 76.500.000) agar total BOSP = 112.500.000 (Rasio Honor 36/112.5 = 32%)
    rkas.tambah_kegiatan(ItemKegiatanRkas {
        id_kegiatan: "KEG-ASESMEN-01".to_string(),
        kode_kegiatan_snp: "03.02.01".to_string(),
        standar_snp: StandarNasionalPendidikan::StandarProses,
        uraian_kegiatan: "Pelaksanaan Asesmen Nasional & Pembelajaran Interaktif Siswa".to_string(),
        kode_rekening: "5.1.02.01.01.0024".to_string(),
        kelompok_belanja: KelompokBelanjaAkuntansi::BelanjaBarangDanJasa,
        sumber_dana: JenisSumberDana::BospReguler,
        volume: 1.0,
        satuan: "Paket".to_string(),
        tarif_harga_satuan_rp: 76_500_000.0,
        total_anggaran_tahunan_rp: 76_500_000.0,
        alokasi_triwulan_1_rp: 20_000_000.0,
        alokasi_triwulan_2_rp: 20_000_000.0,
        alokasi_triwulan_3_rp: 20_000_000.0,
        alokasi_triwulan_4_rp: 16_500_000.0,
        total_realisasi_rp: 0.0,
    });

    // b. Belanja Smart Classroom dari Banper APBN (SNP 6)
    rkas.tambah_kegiatan(ItemKegiatanRkas {
        id_kegiatan: "KEG-SMARTCLASS-01".to_string(),
        kode_kegiatan_snp: "06.03.02".to_string(),
        standar_snp: StandarNasionalPendidikan::StandarSaranaDanPrasarana,
        uraian_kegiatan: "Pengadaan Smart Board & Interactive Display Kelas Vokasi/Digital".to_string(),
        kode_rekening: "5.2.02.08.01.0005".to_string(),
        kelompok_belanja: KelompokBelanjaAkuntansi::BelanjaModalPeralatanMesin,
        sumber_dana: JenisSumberDana::BantuanPemerintahApbn,
        volume: 2.0,
        satuan: "Unit".to_string(),
        tarif_harga_satuan_rp: 50_000_000.0,
        total_anggaran_tahunan_rp: 100_000_000.0,
        alokasi_triwulan_1_rp: 100_000_000.0,
        alokasi_triwulan_2_rp: 0.0,
        alokasi_triwulan_3_rp: 0.0,
        alokasi_triwulan_4_rp: 0.0,
        total_realisasi_rp: 0.0,
    });

    // Validasi aturan juknis & Pengesahan Dinas MARKAS
    assert!(rkas.sahkan_rkas_dinas("2026-01-20").is_ok());
    assert!(rkas.status_pengesahan_dinas_murni);
    arkas.set_rkas(rkas);

    // =========================================================================
    // 4. BUKU KAS UMUM (BKU), BELANJA SIPLAH & PAJAK
    // =========================================================================
    let mut bku = BukuKasUmumSekolah::new("20301234", 2026, 2, 287_500_000.0, 0.0);

    // Belanja Pengadaan SIPLah Smart Display menggunakan dana Banper APBN
    let belanja = bku.catat_belanja_siplah(
        "2026-02-25",
        "SP-SIPLah-20260225-0102",
        "BAST-SIPLah-20260226-0081",
        "Pembelian 2 Unit Interactive Flat Panel Smart Display Kelas",
        JenisSumberDana::BantuanPemerintahApbn,
        "KEG-SMARTCLASS-01",
        100_000_000.0,
        500_000.0,   // PPh 22 0.5% (Rp 500.000)
        11_000_000.0, // PPN 11% (Rp 11.000.000)
    ).expect("Belanja SIPLah di BKU failed");

    assert_eq!(belanja.pengeluaran_rp, 100_000_000.0);
    assert_eq!(bku.saldo_total_saat_ini(), 187_500_000.0);

    // =========================================================================
    // 5. PELAPORAN SPJ & VERIFIKASI MARKAS DINAS PENDIDIKAN
    // =========================================================================
    let mut markas = ManajemenMarkasDinas::new("Dinas Pendidikan Kabupaten Banyumas");

    let laporan_spj = LaporanSpjMarkasBulanan {
        nomor_rekap_spj: "SPJ-MARKAS-2026-02-0042".to_string(),
        npsn_sekolah: "20301234".to_string(),
        tahun_anggaran: 2026,
        bulan_laporan: 2,
        total_penerimaan_semua_sumber_rp: 287_500_000.0,
        total_pengeluaran_semua_sumber_rp: 100_000_000.0,
        total_sisa_kas_bank_rp: 187_500_000.0,
        total_sisa_kas_tunai_rp: 0.0,
        rekap_per_sumber_dana: vec![
            RekapRealisasiPerSumberDana {
                sumber_dana: JenisSumberDana::BantuanPemerintahApbn,
                total_penerimaan_rp: 105_000_000.0,
                total_belanja_rp: 100_000_000.0,
                sisa_kas_rp: 5_000_000.0,
                persentase_serapan: 95.23, // > 50% Progres Banper tercapai!
            },
        ],
        status_verifikasi_markas: "MENUNGGU_VERIFIKASI".to_string(),
        tanggal_sinkronisasi: "2026-02-28 16:00 WIB".to_string(),
        catatan_auditor_dinas: None,
    };

    markas.terima_sinkronisasi_arkas(laporan_spj);
    assert!(markas.validasi_laporan_sekolah("SPJ-MARKAS-2026-02-0042", true, "Laporan realisasi dan BAST SIPLah terverifikasi lengkap."));

    // Karena progres belanja Banper Termin 1 sudah 95.23% (> 50%), maka sekolah berhak mencairkan Termin 2 (30%)
    let mut pks_ref = arkas.daftar_pks_banper.pop().unwrap();
    let cair_t2 = pks_ref.realisasi_termin_2("2026-03-05", "SP2D-KPPN-2026-00890", 95.23).unwrap();
    assert_eq!(cair_t2, 45_000_000.0);
    assert_eq!(pks_ref.status, StatusPksBanper::TerminKeduaDicairkan);
}

#[test]
fn test_pergeseran_dan_perubahan_anggaran_dengan_sandingan() {
    use widya::arkas::{
        DokumenRkasTahunan, ItemKegiatanRkas, JenisRevisiAnggaran, JenisSumberDana,
        KelompokBelanjaAkuntansi, StandarNasionalPendidikan,
    };

    let mut rkas = DokumenRkasTahunan::new("RKAS-2026-02", "20301234", "SD Negeri 1 Somagede", 2026);

    // RKAS Murni Awal: Total Pagu Rp 50.000.000
    // Item 1: Belanja Kertas & ATK (Rp 30.000.000)
    rkas.tambah_kegiatan(ItemKegiatanRkas {
        id_kegiatan: "KEG-ATK-01".to_string(),
        kode_kegiatan_snp: "07.01.01".to_string(),
        standar_snp: StandarNasionalPendidikan::StandarPengelolaan,
        uraian_kegiatan: "Pengadaan Kertas HVS & ATK Kantor".to_string(),
        kode_rekening: "5.1.02.01.01.0024".to_string(),
        kelompok_belanja: KelompokBelanjaAkuntansi::BelanjaBarangDanJasa,
        sumber_dana: JenisSumberDana::BospReguler,
        volume: 60.0,
        satuan: "Rim".to_string(),
        tarif_harga_satuan_rp: 500_000.0,
        total_anggaran_tahunan_rp: 30_000_000.0,
        alokasi_triwulan_1_rp: 10_000_000.0,
        alokasi_triwulan_2_rp: 10_000_000.0,
        alokasi_triwulan_3_rp: 5_000_000.0,
        alokasi_triwulan_4_rp: 5_000_000.0,
        total_realisasi_rp: 0.0,
    });

    // Item 2: Pemeliharaan Sarana Sanitasi MCK (Rp 20.000.000)
    rkas.tambah_kegiatan(ItemKegiatanRkas {
        id_kegiatan: "KEG-MCK-01".to_string(),
        kode_kegiatan_snp: "06.02.01".to_string(),
        standar_snp: StandarNasionalPendidikan::StandarSaranaDanPrasarana,
        uraian_kegiatan: "Pemeliharaan Kebersihan & Sanitasi MCK Siswa".to_string(),
        kode_rekening: "5.1.02.03.01.0010".to_string(),
        kelompok_belanja: KelompokBelanjaAkuntansi::BelanjaBarangDanJasa,
        sumber_dana: JenisSumberDana::BospReguler,
        volume: 1.0,
        satuan: "Paket".to_string(),
        tarif_harga_satuan_rp: 20_000_000.0,
        total_anggaran_tahunan_rp: 20_000_000.0,
        alokasi_triwulan_1_rp: 5_000_000.0,
        alokasi_triwulan_2_rp: 5_000_000.0,
        alokasi_triwulan_3_rp: 5_000_000.0,
        alokasi_triwulan_4_rp: 5_000_000.0,
        total_realisasi_rp: 0.0,
    });

    assert_eq!(rkas.total_pagu_tahunan(), 50_000_000.0);

    // =========================================================================
    // 1. UJI PERGESERAN ANGGARAN (TIDAK PERLU PENGESAHAN DINAS)
    // Skenario: Menggeser Rp 5.000.000 dari ATK ke Sanitasi MCK (Total tetap Rp 50 Juta)
    // =========================================================================
    let update_atk = ItemKegiatanRkas {
        id_kegiatan: "KEG-ATK-01".to_string(),
        kode_kegiatan_snp: "07.01.01".to_string(),
        standar_snp: StandarNasionalPendidikan::StandarPengelolaan,
        uraian_kegiatan: "Pengadaan Kertas HVS & ATK Kantor (Disesuaikan)".to_string(),
        kode_rekening: "5.1.02.01.01.0024".to_string(),
        kelompok_belanja: KelompokBelanjaAkuntansi::BelanjaBarangDanJasa,
        sumber_dana: JenisSumberDana::BospReguler,
        volume: 50.0, // Turun dari 60 jadi 50
        satuan: "Rim".to_string(),
        tarif_harga_satuan_rp: 500_000.0,
        total_anggaran_tahunan_rp: 25_000_000.0, // Berkurang Rp 5.000.000
        alokasi_triwulan_1_rp: 10_000_000.0,
        alokasi_triwulan_2_rp: 5_000_000.0,
        alokasi_triwulan_3_rp: 5_000_000.0,
        alokasi_triwulan_4_rp: 5_000_000.0,
        total_realisasi_rp: 0.0,
    };

    let update_mck = ItemKegiatanRkas {
        id_kegiatan: "KEG-MCK-01".to_string(),
        kode_kegiatan_snp: "06.02.01".to_string(),
        standar_snp: StandarNasionalPendidikan::StandarSaranaDanPrasarana,
        uraian_kegiatan: "Pemeliharaan Kebersihan & Sanitasi MCK Siswa (Ditambah)".to_string(),
        kode_rekening: "5.1.02.03.01.0010".to_string(),
        kelompok_belanja: KelompokBelanjaAkuntansi::BelanjaBarangDanJasa,
        sumber_dana: JenisSumberDana::BospReguler,
        volume: 1.0,
        satuan: "Paket".to_string(),
        tarif_harga_satuan_rp: 25_000_000.0,
        total_anggaran_tahunan_rp: 25_000_000.0, // Bertambah Rp 5.000.000
        alokasi_triwulan_1_rp: 5_000_000.0,
        alokasi_triwulan_2_rp: 10_000_000.0,
        alokasi_triwulan_3_rp: 5_000_000.0,
        alokasi_triwulan_4_rp: 5_000_000.0,
        total_realisasi_rp: 0.0,
    };

    let log_pergeseran = rkas.lakukan_pergeseran_anggaran(
        "2026-04-15",
        "Optimasi pemeliharaan sanitasi sekolah menjelang akreditasi",
        vec![update_atk, update_mck],
    ).expect("Pergeseran anggaran failed");

    // Verifikasi aturan: Pergeseran TIDAK butuh pengesahan dinas
    assert_eq!(log_pergeseran.jenis_revisi, JenisRevisiAnggaran::PergeseranAnggaran);
    assert!(!log_pergeseran.butuh_pengesahan_dinas);
    assert_eq!(log_pergeseran.status_pengesahan, "BerlakuInternalSekolah");
    assert_eq!(rkas.total_pagu_tahunan(), 50_000_000.0); // Total pagu tetap sama!

    // Verifikasi Tabel Sandingan Before-After
    assert_eq!(log_pergeseran.tabel_sandingan.len(), 2);
    let sandingan_atk = &log_pergeseran.tabel_sandingan[0];
    assert_eq!(sandingan_atk.total_sebelum_rp, 30_000_000.0);
    assert_eq!(sandingan_atk.total_sesudah_rp, 25_000_000.0);
    assert_eq!(sandingan_atk.selisih_rp, -5_000_000.0); // Turun 5 Juta

    let sandingan_mck = &log_pergeseran.tabel_sandingan[1];
    assert_eq!(sandingan_mck.total_sebelum_rp, 20_000_000.0);
    assert_eq!(sandingan_mck.total_sesudah_rp, 25_000_000.0);
    assert_eq!(sandingan_mck.selisih_rp, 5_000_000.0); // Naik 5 Juta

    // =========================================================================
    // 2. UJI PERUBAHAN ANGGARAN (WAJIB PENGESAHAN DINAS MARKAS)
    // Skenario: Penambahan Pagu Pendapatan BOS Kinerja Rp 25 Juta (Pagu baru Rp 75 Juta)
    // =========================================================================
    let mut daftar_perubahan = rkas.daftar_kegiatan.clone();
    daftar_perubahan.push(ItemKegiatanRkas {
        id_kegiatan: "KEG-DIGITAL-01".to_string(),
        kode_kegiatan_snp: "06.01.01".to_string(),
        standar_snp: StandarNasionalPendidikan::StandarSaranaDanPrasarana,
        uraian_kegiatan: "Pengadaan Tablet Pembelajaran Digital dari BOS Kinerja".to_string(),
        kode_rekening: "5.2.02.08.01.0012".to_string(),
        kelompok_belanja: KelompokBelanjaAkuntansi::BelanjaModalPeralatanMesin,
        sumber_dana: JenisSumberDana::BosKinerja,
        volume: 10.0,
        satuan: "Unit".to_string(),
        tarif_harga_satuan_rp: 2_500_000.0,
        total_anggaran_tahunan_rp: 25_000_000.0,
        alokasi_triwulan_1_rp: 0.0,
        alokasi_triwulan_2_rp: 0.0,
        alokasi_triwulan_3_rp: 25_000_000.0,
        alokasi_triwulan_4_rp: 0.0,
        total_realisasi_rp: 0.0,
    });

    let log_perubahan = rkas.ajukan_perubahan_anggaran(
        "2026-08-10",
        "Penetapan Pagu BOS Kinerja berdasarkan SK Rapor Pendidikan",
        daftar_perubahan,
    ).expect("Ajukan perubahan anggaran failed");

    // Verifikasi aturan: Perubahan WAJIB butuh pengesahan dinas
    assert_eq!(log_perubahan.jenis_revisi, JenisRevisiAnggaran::PerubahanAnggaran);
    assert!(log_perubahan.butuh_pengesahan_dinas);
    assert_eq!(log_perubahan.status_pengesahan, "MenungguPengesahanDinas");

    // Verifikasi Tabel Sandingan Before-After
    let sandingan_tablet = log_perubahan.tabel_sandingan.iter().find(|s| s.id_kegiatan == "KEG-DIGITAL-01").unwrap();
    assert_eq!(sandingan_tablet.total_sebelum_rp, 0.0);
    assert_eq!(sandingan_tablet.total_sesudah_rp, 25_000_000.0);
    assert_eq!(sandingan_tablet.selisih_rp, 25_000_000.0);

    // Pengesahan oleh Dinas Pendidikan (MARKAS)
    assert!(rkas.sahkan_perubahan_oleh_dinas(log_perubahan.nomor_revisi, "2026-08-15").is_ok());
    assert_eq!(rkas.riwayat_revisi.last().unwrap().status_pengesahan, "DisahkanDinas");
    assert_eq!(rkas.total_pagu_tahunan(), 75_000_000.0);
}

#[test]
fn test_siplah_pbj_permendikdasmen_15_2026_lengkap_6_dokumen() {
    use widya::arkas::{
        BukuKasUmumSekolah, DokumenRkasTahunan, ItemKegiatanRkas, JenisSumberDana,
        KelompokBelanjaAkuntansi, MetodePbjSiplah, PenawaranCalonPenyedia,
        RiwayatTawarMenawar, SiplahPbjEngine, StandarNasionalPendidikan,
        UjiKualitasFisikBarang,
    };

    let mut engine = SiplahPbjEngine::new("SIPLah Kemendikdasmen RI");

    // Persiapan RKAS dengan pagu pengadaan Laptop Pembelajaran
    let mut rkas = DokumenRkasTahunan::new("RKAS-2026-03", "20301234", "SD Negeri 1 Somagede", 2026);
    rkas.tambah_kegiatan(ItemKegiatanRkas {
        id_kegiatan: "KEG-TIK-LAPTOP-01".to_string(),
        kode_kegiatan_snp: "06.01.02".to_string(),
        standar_snp: StandarNasionalPendidikan::StandarSaranaDanPrasarana,
        uraian_kegiatan: "Pengadaan Laptop Chromebook & Perangkat Asesmen Digital".to_string(),
        kode_rekening: "5.2.02.08.01.0002".to_string(),
        kelompok_belanja: KelompokBelanjaAkuntansi::BelanjaModalPeralatanMesin,
        sumber_dana: JenisSumberDana::BospReguler,
        volume: 10.0,
        satuan: "Unit".to_string(),
        tarif_harga_satuan_rp: 8_000_000.0,
        total_anggaran_tahunan_rp: 80_000_000.0,
        alokasi_triwulan_1_rp: 80_000_000.0,
        alokasi_triwulan_2_rp: 0.0,
        alokasi_triwulan_3_rp: 0.0,
        alokasi_triwulan_4_rp: 0.0,
        total_realisasi_rp: 0.0,
    });

    // =========================================================================
    // DOKUMEN 1: DOKUMEN RENCANA BELANJA (DRB)
    // =========================================================================
    let drb = engine.buat_rencana_belanja(
        "DRB-SIPLah/2026/001",
        &rkas,
        "KEG-TIK-LAPTOP-01",
        "2026-03-01",
        "Drs. Slamet Riyadi, M.Pd (Kepala Satdik)",
    ).expect("Buat DRB failed");

    assert_eq!(drb.metode_pengadaan, MetodePbjSiplah::PerbandinganHarga); // Rp 80 Jt (50-200 Jt)
    assert_eq!(drb.total_perkiraan_biaya_rp, 80_000_000.0);

    // =========================================================================
    // DOKUMEN 2: DOKUMEN PERBANDINGAN HARGA (DPH)
    // =========================================================================
    let penawaran_1 = PenawaranCalonPenyedia {
        id_penyedia: "PENYEDIA-A-01".to_string(),
        nama_toko_mitra: "PT Mitra Edukasi Nusantara (SIPLah Blibli)".to_string(),
        nama_marketplace_siplah: "SIPLah Blibli".to_string(),
        harga_satuan_penawaran_rp: 7_900_000.0,
        ongkos_kirim_rp: 200_000.0,
        total_biaya_rp: 79_200_000.0,
        estimasi_hari_kirim: 3,
        rating_reputasi: 4.9,
        terpilih_sebagai_pemenang: true,
        alasan_pemilihan: "Harga paling ekonomis, spesifikasi sesuai SNI, rating 4.9".to_string(),
    };

    let penawaran_2 = PenawaranCalonPenyedia {
        id_penyedia: "PENYEDIA-B-02".to_string(),
        nama_toko_mitra: "CV Graha TIK Prima (SIPLah Eureka)".to_string(),
        nama_marketplace_siplah: "SIPLah Eureka".to_string(),
        harga_satuan_penawaran_rp: 8_000_000.0,
        ongkos_kirim_rp: 500_000.0,
        total_biaya_rp: 80_500_000.0,
        estimasi_hari_kirim: 5,
        rating_reputasi: 4.7,
        terpilih_sebagai_pemenang: false,
        alasan_pemilihan: "Harga lebih tinggi dari pembanding".to_string(),
    };

    let dph = engine.catat_perbandingan_harga(
        "DPH-SIPLah/2026/001",
        &drb.nomor_drb,
        "2026-03-02",
        vec![penawaran_1, penawaran_2],
    ).expect("Catat DPH failed");

    assert_eq!(dph.penyedia_terpilih_id, "PENYEDIA-A-01");

    // =========================================================================
    // DOKUMEN 3: DOKUMEN NEGOSIASI HARGA (DNH)
    // =========================================================================
    let riwayat_nego = vec![
        RiwayatTawarMenawar {
            putaran_ke: 1,
            pengaju: "Sekolah".to_string(),
            harga_diajukan_rp: 76_000_000.0,
            catatan_negosiasi: "Permohonan diskon volume 10 unit laptop".to_string(),
        },
        RiwayatTawarMenawar {
            putaran_ke: 2,
            pengaju: "Penyedia".to_string(),
            harga_diajukan_rp: 77_000_000.0,
            catatan_negosiasi: "Penyedia menyetujui harga Rp 77 Juta + Free Mouse & Bag".to_string(),
        },
    ];

    let dnh = engine.catat_negosiasi_harga(
        "DNH-SIPLah/2026/001",
        &drb.nomor_drb,
        "PENYEDIA-A-01",
        "2026-03-03",
        79_000_000.0, // Awal
        77_000_000.0, // Kesepakatan Final
        riwayat_nego,
    );

    assert_eq!(dnh.harga_kesepakatan_final_rp, 77_000_000.0);
    assert_eq!(dnh.efisiensi_penghematan_rp, 2_000_000.0);

    // =========================================================================
    // DOKUMEN 4: SURAT PESANAN (SP / PO DIGITAL SIPLAH)
    // =========================================================================
    let sp = engine.terbitkan_surat_pesanan(
        "SP-SIPLah-20260303-0099",
        &drb,
        "PENYEDIA-A-01",
        "PT Mitra Edukasi Nusantara",
        "01.999.888.7-001.000",
        "8888-01-20301234-01", // VA Pembayaran
        "2026-03-03",
        "2026-03-08",
        77_000_000.0,
        200_000.0, // Ongkir
    );

    assert_eq!(sp.grand_total_rp, 77_200_000.0);
    assert_eq!(sp.status_pesanan, "DITERBITKAN");

    // =========================================================================
    // DOKUMEN 5: BERITA ACARA SERAH TERIMA (BAST DIGITAL) & UJI TERIMA
    // =========================================================================
    let hasil_uji = UjiKualitasFisikBarang {
        kesesuaian_spesifikasi: true,
        kuantitas_sesuai_sp: true,
        kondisi_fisik_baik_tanpa_cacat: true,
        uji_fungsi_alat_berjalan: true,
        masa_garansi_bulan: 12,
        catatan_pemeriksaan: "10 unit Laptop berfungsi normal, segel utuh, garansi 1 tahun resmi".to_string(),
    };

    let bast = engine.proses_bast_digital(
        "BAST-SIPLah-20260307-0055",
        &sp.nomor_surat_pesanan,
        "2026-03-07",
        "Wahyudi, S.Pd (Pemeriksa Barang Satdik)",
        "Dedi Hermawan (Kurir Logistik Mitra)",
        hasil_uji,
    ).expect("Proses BAST failed");

    assert!(bast.status_barang_diterima_lengkap);
    assert!(bast.tanda_tangan_elektronik_sah);

    // =========================================================================
    // DOKUMEN 6: BUKTI PEMBAYARAN & POSTING OTOMATIS KE BKU ARKAS
    // =========================================================================
    let mut bku = BukuKasUmumSekolah::new("20301234", 2026, 3, 100_000_000.0, 0.0);

    let bukti_bayar = engine.bayar_dan_posting_bku(
        "BAYAR-SIPLah-20260308-01",
        &sp.nomor_surat_pesanan,
        &bast.nomor_bast,
        "2026-03-08",
        &mut bku,
        JenisSumberDana::BospReguler,
        "KEG-TIK-LAPTOP-01",
    ).expect("Pembayaran dan posting BKU failed");

    assert_eq!(bukti_bayar.nominal_bruto_rp, 77_200_000.0);
    assert_eq!(bukti_bayar.rincian_pajak.pph_22_rp, 385_000.0); // 0.5% dari Rp 77 Jt
    assert_eq!(bukti_bayar.rincian_pajak.ppn_11_rp, 8_470_000.0); // 11% dari Rp 77 Jt

    // Verifikasi saldo BKU berkurang dari Rp 100 Jt -> Rp 22.8 Jt
    assert_eq!(bku.saldo_total_saat_ini(), 22_800_000.0);
    assert_eq!(bku.daftar_transaksi.len(), 1);
    assert_eq!(bku.daftar_transaksi[0].pengeluaran_rp, 77_200_000.0);
}

#[test]
fn test_delegasi_pengesahan_apbn_ke_bpmp_dan_dinas() {
    use widya::arkas::{
        JenisBanperApbn, OtoritasPengesahanApbn, PerjanjianKerjaSamaBanper,
        PihakKeduaPenerima, PihakPertamaPPK, StatusPksBanper,
    };

    let pihak_ppk = PihakPertamaPPK {
        nama_pejabat_ppk: "Direktur SMK Ditjen Pendidikan Vokasi".to_string(),
        nip_ppk: "197203101997021001".to_string(),
        jabatan: "PPK Pengadaan Lab Vokasi Kemendikdasmen".to_string(),
        satuan_kerja_kementerian: "Kemendikdasmen RI".to_string(),
        alamat_kantor: "Kompleks Kemendikbud Senayan".to_string(),
    };

    let pihak_smk = PihakKeduaPenerima {
        npsn: "20399881".to_string(),
        nama_sekolah: "SMK Negeri 1 Purwokerto".to_string(),
        nama_kepala_sekolah_atau_ketua: "Drs. Dani Haryanto, M.T".to_string(),
        nip_atau_nik: "197001011995031002".to_string(),
        jabatan: "Kepala Satuan Pendidikan".to_string(),
        nomor_rekening_khusus_banper: "140-00-1122334-5".to_string(),
        nama_bank_penyalur: "Bank Jateng".to_string(),
    };

    // 1. Skenario Pendelegasian ke BPMP Provinsi Jawa Tengah
    let mut pks_bpmp = PerjanjianKerjaSamaBanper::buat_pks(
        "PKS-BANPER-VOKASI/2026/088",
        "421.5/102/SMKN1/2026",
        "2026-03-15",
        2026,
        JenisBanperApbn::LaboratoriumVokasiSmk,
        pihak_ppk.clone(),
        pihak_smk.clone(),
        250_000_000.0,
    );

    let delegasi_bpmp = OtoritasPengesahanApbn::DelegasiBpmpProvinsi {
        nama_bpmp: "BPMP (Balai Penjaminan Mutu Pendidikan) Provinsi Jawa Tengah".to_string(),
        nomor_sk_delegasi_kementerian: "SK-MENDIKDASMEN-DELEGASI-2026-015".to_string(),
        nama_kepala_bpmp_atau_verifikator: "Nugroho S., M.Ed (Ketua Tim Mutu BPMP)".to_string(),
        nip_pejabat: "197806202002121003".to_string(),
    };

    pks_bpmp.tandatangani_pks(true, true, delegasi_bpmp.clone()).unwrap();
    assert_eq!(pks_bpmp.status, StatusPksBanper::DitandatanganiParaPihak);
    assert_eq!(pks_bpmp.otoritas_pengesah, Some(delegasi_bpmp));

    // 2. Skenario Pendelegasian ke Dinas Pendidikan Kabupaten/Kota
    let mut pks_dinas = PerjanjianKerjaSamaBanper::buat_pks(
        "PKS-BANPER-REVITALISASI/2026/099",
        "421.5/103/SMKN1/2026",
        "2026-03-20",
        2026,
        JenisBanperApbn::RevitalisasiSarprasDanGedung,
        pihak_ppk,
        pihak_smk,
        180_000_000.0,
    );

    let delegasi_dinas = OtoritasPengesahanApbn::DelegasiDinasPendidikan {
        nama_dinas: "Dinas Pendidikan Provinsi Jawa Tengah Cabang Dinas Wilayah X".to_string(),
        nomor_surat_pelimpahan_tugas: "ST-KEMENDIKDASMEN-WIL10-2026-042".to_string(),
        nama_kepala_dinas_atau_tim_bos: "Ir. Joko Sulistyo, M.M (Kacabdin Wilayah X)".to_string(),
        nip_pejabat: "197109151998031005".to_string(),
    };

    pks_dinas.tandatangani_pks(true, true, delegasi_dinas.clone()).unwrap();
    assert_eq!(pks_dinas.status, StatusPksBanper::DitandatanganiParaPihak);
    assert_eq!(pks_dinas.otoritas_pengesah, Some(delegasi_dinas));
}



