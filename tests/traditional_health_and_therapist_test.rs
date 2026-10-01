use widya::healthtech::{
    HerbDrugInteractionEngine, KategoriObatBahanAlam, MarketplaceKesehatanTradisional,
    ModalitasTerapi, ProdukHerbalMarketplace, RekamTerapiTradisional, SenyawaAktif,
    SimplisiaHerbal,
};

#[test]
fn test_traditional_medicine_and_interaction() {
    let simplisia = SimplisiaHerbal {
        id_simplisia: "SIMP-TM-001".to_string(),
        nama_latin: "Curcuma xanthorrhiza Rhizoma".to_string(),
        nama_daerah: "Temulawak".to_string(),
        bagian_tanaman: "Rimpang".to_string(),
        senyawa_aktif: vec![SenyawaAktif {
            nama_senyawa: "Kurkuminoid".to_string(),
            persentase_kadar: 4.8,
            metode_pengujian: "HPLC".to_string(),
        }],
        kadar_air_pct: 8.5,        // Standard <= 10.0%
        kadar_abu_total_pct: 6.2,  // Standard <= 8.0%
        bebas_cemaran_logam: true,
    };
    assert!(simplisia.kadar_air_pct <= 10.0);
    assert_eq!(simplisia.senyawa_aktif[0].nama_senyawa, "Kurkuminoid");

    let produk = ProdukHerbalMarketplace {
        id_produk: "PROD-JAMU-001".to_string(),
        nama_produk: "Kapsul Ekstrak Temulawak Plus".to_string(),
        kategori: KategoriObatBahanAlam::ObatHerbalTerstandar,
        nomor_izin_edar_bpom: "POM HT212345671".to_string(),
        produsen: "PT Herbal Nusantara Berdaulat".to_string(),
        sertifikasi_halal: true,
        indikasi_khasiat: vec!["Membantu memelihara kesehatan fungsi hati (hepatoprotektor)".to_string()],
        kontraindikasi: vec!["Obstruksi saluran empedu".to_string()],
        aturan_pakai: "3 kali sehari 1 kapsul sesudah makan".to_string(),
        harga_idr: 85000,
        stok_unit: 120,
        batch_number: "BATCH-2026-X1".to_string(),
    };
    assert!(produk.sertifikasi_halal);
    assert_eq!(produk.kategori, KategoriObatBahanAlam::ObatHerbalTerstandar);

    // Herb-Drug Interaction Check
    let engine = HerbDrugInteractionEngine::new();
    let advice = engine.check_interaction("Kunyit", "Warfarin");
    assert!(advice.is_some());
    assert!(advice.unwrap().contains("antikoagulan"));

    let safe_check = engine.check_interaction("Jahe", "Paracetamol");
    assert!(safe_check.is_none());
}

#[test]
fn test_therapist_stpt_and_marketplace_booking() {
    let mut marketplace = MarketplaceKesehatanTradisional::new();

    // Register Terapis STPT Kemenkes
    let terapis = marketplace.register_terapis(
        "TERAPIS-001",
        "H. Ahmad Syafii, S.Kep., C.Tr",
        "STPT/440/123/DINKES/2025",
        "Asosiasi Penyehat Tradisional Indonesia (APSTI)",
        vec![ModalitasTerapi::Akupresur, ModalitasTerapi::BekamSteril, ModalitasTerapi::PijatRefleksi],
        150000,
        true, // Homecare
        -6.2088, // Jakarta Pusat
        106.8456,
    );
    assert_eq!(terapis.rating_bintang, 5.0);
    assert!(terapis.verifikasi_higienitas_alat);

    // Cari terapis terdekat (radius 10 km)
    let terapis_dekat = marketplace.cari_terapis_terdekat(
        ModalitasTerapi::BekamSteril,
        -6.2000,
        106.8400,
        10.0,
    );
    assert_eq!(terapis_dekat.len(), 1);
    assert_eq!(terapis_dekat[0].nama_lengkap, "H. Ahmad Syafii, S.Kep., C.Tr");

    // Booking Jasa
    let booking = marketplace.buat_booking(
        "BOOK-2026-0099",
        "USER-KLIEN-88",
        "TERAPIS-001",
        ModalitasTerapi::BekamSteril,
        "2026-10-02T14:00:00Z",
        "Jl. Kebon Sirih No. 45, Jakarta Pusat",
    ).expect("Booking failed");
    assert_eq!(booking.status_layanan, "DIKONFIRMASI");
    assert!(booking.status_pembayaran_escrow);

    // Rekam Terapi Tradisional (SOAP)
    let rekam = RekamTerapiTradisional {
        id_rekam: "REKAM-001".to_string(),
        id_klien: "USER-KLIEN-88".to_string(),
        id_terapis: "TERAPIS-001".to_string(),
        tanggal_sesi_iso: "2026-10-02T15:00:00Z".to_string(),
        keluhan_utama: "Nyeri bahu dan kekakuan leher kronis (Myalgia)".to_string(),
        observasi_anamnesis: "Palpasi otot trapezius spasme, nadi lambat kuat".to_string(),
        modalitas_diberikan: ModalitasTerapi::BekamSteril,
        titik_meridian_terstimulasi: vec![
            "GV14 (Dazhui)".to_string(),
            "BL11 (Dazhu)".to_string(),
            "GB21 (Jianjing)".to_string(),
        ],
        ramuan_herbal_anjuran: Some("Seduhan Jahe Merah + Temulawak + Madu Hutan".to_string()),
        evaluasi_pasca_terapi: "Otot bahu rileks, skala nyeri turun dari 7 ke 2".to_string(),
    };
    marketplace.catat_rekam_terapi(rekam);
    assert_eq!(marketplace.rekam_terapi.len(), 1);
}
