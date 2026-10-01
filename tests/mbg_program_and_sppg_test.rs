use widya::mbg::{
    DapurSentralSppg, JenisLembagaPenerima, KelompokSasaranMbg, KomposisiGiziPorsi,
    ManajemenBantuanYayasan, ManajemenPenerimaManfaat, StandarGiziEngine, StatusImtSiswa,
};

#[test]
fn test_program_mbg_dan_dapur_sppg_terpadu() {
    // 1. Validasi Standar AKG Menu Gizi
    let menu_sd = KomposisiGiziPorsi {
        energi_kalori_kkal: 620.0,
        protein_gram: 22.5, // Ayam + Telur + Tempe
        karbohidrat_gram: 85.0,
        lemak_gram: 18.0,
        serat_pangan_gram: 6.5,
        kalsium_mg: 350.0,
        zat_besi_mg: 4.5,
        vitamin_c_mg: 45.0,
    };
    let (lolos, catatan) = StandarGiziEngine::validasi_kelayakan_gizi(KelompokSasaranMbg::SdKelas4Sampai6, &menu_sd);
    assert!(lolos);
    assert!(catatan.is_empty());

    // 2. Dapur Sentral SPPG & Penyerapan Bahan Lokal
    let mut dapur = DapurSentralSppg::new(
        "SPPG-BANYUMAS-01",
        "Dapur SPPG Sentral Kecamatan Somagede",
        "Kabupaten Banyumas, Jawa Tengah",
        3000,
    );

    let pasokan = dapur.terima_pasokan_lokal("Beras Organik", "Koperasi Desa Merah Putih Somagede", 500.0, 13500);
    assert_eq!(pasokan.kuantitas_kg, 500.0);
    assert!(pasokan.hasil_inspeksi_kebersihan);

    // Proses masak batch 1 (2.500 porsi)
    let batch = dapur.proses_batch_masak(
        "BATCH-20261001-01",
        "Shift 1 (03:00 - 06:00 Subuh)",
        2500,
        72.5, // Suhu matang 72.5 C (Standar HACCP >= 65 C)
    ).expect("Batch masak failed");
    assert_eq!(batch.jumlah_porsi_diproduksi, 2500);
    assert!(batch.sampel_makanan_disimpan_24jam);

    // 3. Yayasan / Lembaga Sekolah Penerima Bantuan APBN & SPJ Digital
    let mut yayasan_mgr = ManajemenBantuanYayasan::new();
    let y1 = yayasan_mgr.daftarkan_lembaga(
        "YAYASAN-SDN-01",
        "20301234",
        "SD Negeri 1 Somagede",
        JenisLembagaPenerima::SekolahNegeri,
        "AKTA-KEMENKUMHAM-123",
        "140-00-1234567-8 (Bank Penyalur BPD)",
        "Drs. Slamet Riyadi, M.Pd",
        250, // 250 Siswa sasaran
    );
    assert_eq!(y1.total_sasaran_siswa, 250);

    // Salurkan dana 20 hari makan (250 siswa x 20 hari x Rp 15.000 = Rp 75.000.000)
    let total_dana = yayasan_mgr.salurkan_dana_termin("YAYASAN-SDN-01", 20, 15000).expect("Salur dana failed");
    assert_eq!(total_dana, 75_000_000);

    // Unggah SPJ Digital Hari ke-1 (250 porsi x Rp 15.000 = Rp 3.750.000)
    let spj = yayasan_mgr.unggah_spj_digital(
        "SPJ-MBG-2026-0001",
        "YAYASAN-SDN-01",
        "Dapur SPPG Sentral Kecamatan Somagede",
        250,
        3_750_000,
        "E-FAKTUR-99887766",
    ).expect("Unggah SPJ failed");
    assert_eq!(spj.status_audit_inspektorat, "VALID_TERVERIFIKASI");

    // 4. Penerima Manfaat & Presensi QR Tapping
    let mut penerima_mgr = ManajemenPenerimaManfaat::new();
    let (imt, status_imt) = ManajemenPenerimaManfaat::hitung_status_imt(135.0, 32.0); // TB 135 cm, BB 32 kg -> IMT 17.6 (Normal)
    assert_eq!(status_imt, StatusImtSiswa::NormalIdeal);
    assert!((imt - 17.6).abs() < 1e-1);

    let siswa = penerima_mgr.daftarkan_siswa(
        "0123456789",
        "Ahmad Fauzi",
        "YAYASAN-SDN-01",
        "Kelas 5A",
        "2015-05-20",
        "Laki-laki",
        135.0,
        32.0,
        None,
    );
    assert_eq!(siswa.status_imt, StatusImtSiswa::NormalIdeal);

    // Tapping Presensi Penerimaan Porsi Hari Ini
    let log_presensi = penerima_mgr.catat_presensi_porsi("0123456789", "QR_CARD_SISWA").expect("Tapping presensi failed");
    assert!(log_presensi.status_penerimaan);

    // Coba tapping porsi ganda (harus ditolak secara otomatis)
    let double_tap = penerima_mgr.catat_presensi_porsi("0123456789", "QR_CARD_SISWA");
    assert!(double_tap.is_err());
}

#[test]
fn test_siplah_supplier_marketplace_mbg() {
    use widya::mbg::{
        KategoriBahanPangan, LegalitasPenyedia, PenyediaBahanMakanan, ProdukPanganSIPLah,
        SIPLahPanganEngine, StatusPesananSIPLah, StatusVerifikasiPenyedia,
    };

    let mut siplah = SIPLahPanganEngine::new("SIPLah Pangan MBG Kemendikdasmen RI");

    // 1. Daftarkan Penyedia / Koperasi Pangan Desa Merah Putih
    let supplier_koperasi = PenyediaBahanMakanan {
        id_penyedia: "SUPPLIER-KOP-01".to_string(),
        nama_usaha: "Koperasi Desa Merah Putih Somagede (Unit Sembako & Pangan)".to_string(),
        jenis_badan_usaha: "Koperasi Produsen Pangan".to_string(),
        nama_pemilik: "H. Sudirman, S.Pt".to_string(),
        alamat: "Jl. Raya Somagede No. 45".to_string(),
        desa_kelurahan: "Somagede".to_string(),
        kabupaten_kota: "Banyumas".to_string(),
        radius_antar_km: 25.0,
        legalitas: LegalitasPenyedia {
            nib: "9120001234567".to_string(),
            npwp: "01.234.567.8-521.000".to_string(),
            nomor_sertifikat_halal: Some("ID3311000012345".to_string()),
            sertifikat_laik_higiene: Some("SKP-DKP-2026-088".to_string()),
            rekening_bank: "0012-01-002345-50-8".to_string(),
            nama_bank: "Bank BRI Mitra SIPLah".to_string(),
        },
        status: StatusVerifikasiPenyedia::MenungguKurasi,
        rating_reputasi: 4.9,
        total_transaksi_selesai: 42,
    };

    siplah.daftarkan_penyedia(supplier_koperasi);
    assert!(siplah.verifikasi_penyedia("SUPPLIER-KOP-01"));

    // 2. Tambah Komoditas Pangan ke Katalog SIPLah
    let telur = ProdukPanganSIPLah {
        id_produk: "PROD-TELUR-01".to_string(),
        id_penyedia: "SUPPLIER-KOP-01".to_string(),
        nama_produk: "Telur Ayam Ras Segar Grade A (Kandang Peternak Lokal)".to_string(),
        kategori: KategoriBahanPangan::TelurAyamSegar,
        satuan: "Tray (30 butir / ~1.8 kg)".to_string(),
        harga_satuan_rp: 48000.0,
        stok_tersedia: 500.0, // 500 tray
        umur_simpan_hari: 14,
        suhu_penyimpanan_celsius: 18.0,
        asal_komoditas_lokal: "Kelompok Peternak Unggas Somagede Mandiri".to_string(),
        kandungan_gizi_per_100g: None,
    };

    let beras = ProdukPanganSIPLah {
        id_produk: "PROD-BERAS-01".to_string(),
        id_penyedia: "SUPPLIER-KOP-01".to_string(),
        nama_produk: "Beras Premium IR64 Organik Bersertifikat".to_string(),
        kategori: KategoriBahanPangan::BerasDanKarbohidrat,
        satuan: "Kg".to_string(),
        harga_satuan_rp: 14500.0,
        stok_tersedia: 2000.0,
        umur_simpan_hari: 180,
        suhu_penyimpanan_celsius: 25.0,
        asal_komoditas_lokal: "Gapoktan Sari Tani Somagede".to_string(),
        kandungan_gizi_per_100g: None,
    };

    siplah.tambah_produk_katalog(telur).unwrap();
    siplah.tambah_produk_katalog(beras).unwrap();

    // 3. Pencarian Bahan Pangan Berdasarkan Kategori & Jarak Radius
    let hasil_cari = siplah.cari_bahan_pangan(KategoriBahanPangan::TelurAyamSegar, 10.0);
    assert_eq!(hasil_cari.len(), 1);
    assert_eq!(hasil_cari[0].0.id_produk, "PROD-TELUR-01");

    // 4. Penerbitan Surat Pesanan (SP / PO Digital) oleh Dapur Sentral SPPG
    let item_order = vec![
        ("PROD-TELUR-01", 100.0), // 100 tray x Rp 48.000 = Rp 4.800.000
        ("PROD-BERAS-01", 200.0), // 200 kg x Rp 14.500 = Rp 2.900.000
    ];

    let sp = siplah.buat_surat_pesanan(
        "SP-SIPLah-20261001-0089",
        "SPPG-BANYUMAS-01",
        "Dapur SPPG Sentral Kecamatan Somagede",
        "SUPPLIER-KOP-01",
        "2026-10-01",
        "2026-10-02",
        item_order,
    ).expect("Buat SP SIPLah failed");

    assert_eq!(sp.total_nilai_bruto_rp, 7_700_000.0);
    assert_eq!(sp.nilai_pph_22_rp, 38_500.0); // 0.5% dari Rp 7.700.000
    assert_eq!(sp.status, StatusPesananSIPLah::TerkonfirmasiPenyedia);

    // 5. Penerimaan Fisik di Dapur SPPG & BAST Digital
    let bast = siplah.proses_bast_penerimaan(
        "BAST-SIPLah-20261002-0042",
        "SP-SIPLah-20261001-0089",
        "PETUGAS-SPPG-01",
        "Budi Santoso (Koordinator Logistik Dapur)",
        "KURIR-KOP-09",
        "Aris Setiawan (Armada Koperasi)",
        "2026-10-02 05:30 WIB",
        7.2,  // Suhu cold chain 7.2 C
        380.0, // Berat aktual total 380 kg (200kg beras + 180kg telur)
        380.0, // Berat pesanan 380 kg
    ).expect("Proses BAST failed");

    assert!(bast.uji_kualitas.hasil_layak_konsumsi);
    assert!(bast.tanda_tangan_elektronik_sah);
    assert_eq!(bast.status_pembayaran_escrow, "DanaDiteruskanKePenyedia");
}

