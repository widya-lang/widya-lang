use widya::library::{
    DublinCoreMetadata, IiifManifest, JenisKaryaIlmiah, JenisKoleksiDaerah,
    JenisKoleksiSekolah, KlasifikasiDdc, KoleksiBukuSekolah, LayananPerpustakaanKeliling,
    Marc21Record, NaskahKunoManuskrip, PerpustakaanDaerahEngine, PerpustakaanInternasionalHub,
    PerpustakaanSekolahEngine, PerpustakaanTinggiEngine, StatusInterlibraryLoan,
    StatusKonservasiManuskrip, Z3950QueryRequest,
};

#[test]
fn test_ekosistem_perpustakaan_4_level_terpadu() {
    // =========================================================================
    // 1. PERPUSTAKAAN SEKOLAH (SNP PERPUSNAS & BOSP BUKU PELAJARAN)
    // =========================================================================
    let mut perpus_sekolah = PerpustakaanSekolahEngine::new("20301234", "Perpustakaan Pelita SDN 1 Somagede");

    let mut marc_buku_sd = Marc21Record::new("PERPUS-SD-2026-001");
    marc_buku_sd.tambah_field("020", ' ', ' ', vec![('a', "978-602-244-123-4")]);
    marc_buku_sd.tambah_field("100", '1', ' ', vec![('a', "Susanto, Budi")]);
    marc_buku_sd.tambah_field("245", '1', '0', vec![
        ('a', "Ilmu Pengetahuan Alam dan Sosial (IPAS) SD Kelas 4"),
        ('c', "Budi Susanto & Tim"),
    ]);

    let buku_ipas = KoleksiBukuSekolah {
        id_buku: "BUKU-IPAS-SD4".to_string(),
        isbn: "978-602-244-123-4".to_string(),
        judul: "Ilmu Pengetahuan Alam dan Sosial (IPAS) SD Kelas 4".to_string(),
        pengarang: "Budi Susanto".to_string(),
        penerbit: "Pusat Kurikulum dan Perbukuan Kemendikdasmen".to_string(),
        tahun_terbit: 2026,
        jenis_koleksi: JenisKoleksiSekolah::BukuTeksUtamaKurikulum,
        peruntukan_kelas: "Kelas 4 SD".to_string(),
        klasifikasi_ddc: KlasifikasiDdc {
            nomor_kelas: "372.357".to_string(),
            label_divisi: "Pendidikan Dasar - Sains & Sosial".to_string(),
            rak_lokasi: "Rak Buku Pelajaran Utama A-01".to_string(),
        },
        jumlah_eksemplar: 50,
        eksemplar_tersedia: 50,
        sumber_pengadaan: "BOSP Reguler 2026".to_string(),
        metadata_marc: Some(marc_buku_sd),
    };

    perpus_sekolah.tambah_koleksi(buku_ipas);
    perpus_sekolah.daftarkan_siswa("0123456789", "Ahmad Fauzi", "Kelas 4A");

    // Transaksi Peminjaman Siswa
    let pinjam = perpus_sekolah.pinjam_buku(
        "TRX-PINJAM-202601-001",
        "0123456789",
        "BUKU-IPAS-SD4",
        "2026-01-10",
        "2026-01-24",
    ).expect("Peminjaman buku gagal");

    assert_eq!(pinjam.judul_buku, "Ilmu Pengetahuan Alam dan Sosial (IPAS) SD Kelas 4");
    assert_eq!(perpus_sekolah.katalog_buku[0].eksemplar_tersedia, 49);

    // Pengembalian Buku
    assert!(perpus_sekolah.kembalikan_buku("TRX-PINJAM-202601-001", "2026-01-22", 0.0).is_ok());
    assert_eq!(perpus_sekolah.katalog_buku[0].eksemplar_tersedia, 50);

    // =========================================================================
    // 2. PERPUSTAKAAN DAERAH (UU 13/2018 KCKR, NASKAH KUNO & PERPUS KELILING)
    // =========================================================================
    let mut perpusda = PerpustakaanDaerahEngine::new(
        "Dinas Arsip dan Perpustakaan Daerah Kabupaten Banyumas",
        "Kabupaten/Kota",
    );

    // Serah Simpan Karya Cetak & Rekam (KCKR)
    let bukti_kckr = perpusda.terima_deposit_kckr(
        "KCKR-BMS-2026-0045",
        "Penerbit Banyumas Aksara Mandiri",
        "Jl. Jenderal Soedirman No. 120 Purwokerto",
        "Sejarah Lengkap Kadipaten Banyumas",
        "978-623-7788-99-1",
        "2026-02-15",
        2, // 2 Eksemplar
    );
    assert_eq!(bukti_kckr.jumlah_salinan_diserahkan, 2);

    // Preservasi Naskah Kuno Nusantara
    let manuskrip_babad = NaskahKunoManuskrip {
        id_manuskrip: "MS-BMS-BABAD-01".to_string(),
        judul_naskah: "Babad Pasir Luhur & Kamandaka".to_string(),
        bahasa: "Jawa Kuno / Kawi".to_string(),
        aksara: "Hanacaraka Jawa Pegon".to_string(),
        media_tulis: "Kertas Daluang Pohon Saeh".to_string(),
        perkiraan_abad: "Abad ke-17 Masehi".to_string(),
        asal_daerah: "Kadipaten Pasir Luhur Banyumas".to_string(),
        kondisi_fisik: StatusKonservasiManuskrip::TerkonservasiDigitalTinggi,
        resolusi_scan_dpi: 600,
        uri_repositori_iiif: "https://iiif.dispussipda.banyumaskab.go.id/manuskrip/babad-pasir-luhur".to_string(),
    };
    perpusda.daftarkan_manuskrip(manuskrip_babad);
    assert_eq!(perpusda.koleksi_manuskrip.len(), 1);

    // Perpustakaan Keliling ke Desa
    perpusda.catat_layanan_keliling(LayananPerpustakaanKeliling {
        id_armada: "MOBIL-PERPUS-02".to_string(),
        nomor_polisi: "R 9988 GA".to_string(),
        wilayah_rute_desa: "Kecamatan Somagede & Tambak".to_string(),
        jumlah_koleksi_dibawa: 1200,
        jumlah_pengunjung_terlayani: 340,
        tanggal_kegiatan: "2026-02-20".to_string(),
    });
    assert_eq!(perpusda.rute_keliling[0].jumlah_pengunjung_terlayani, 340);

    // =========================================================================
    // 3. PERPUSTAKAAN PERGURUAN TINGGI (OAI-PMH, REPOSITORI, TURNITIN, DOI)
    // =========================================================================
    let mut perpus_univ = PerpustakaanTinggiEngine::new(
        "001045",
        "Universitas Jenderal Soedirman",
        "https://repository.unsoed.ac.id/oai/request",
    );

    let dc_skripsi = DublinCoreMetadata {
        title: "Penerapan Algoritma Kuantum Hibrida untuk Optimasi Rantai Pasok Pertanian".to_string(),
        creator: vec!["Rizky Pratama".to_string()],
        subject: vec!["Quantum Computing".to_string(), "Supply Chain Optimization".to_string()],
        description: "Penelitian skripsi mengenai optimasi logistik hasil tani presisi.".to_string(),
        publisher: "Universitas Jenderal Soedirman".to_string(),
        contributor: vec!["Dr. Eng. Hendro Wicaksono (Pembimbing I)".to_string()],
        date: "2026-02-28".to_string(),
        type_resource: "Skripsi / Undergraduate Thesis".to_string(),
        format: "application/pdf".to_string(),
        identifier: "10.20884/skripsi.fasilkom.2026.0123".to_string(),
        source: None,
        language: "ind".to_string(),
        relation: None,
        coverage: Some("Indonesia".to_string()),
        rights: "Creative Commons Attribution-NonCommercial (CC-BY-NC 4.0)".to_string(),
    };

    // Uji Kemiripan Turnitin 11.2% (Lolos karena <= 20.0%)
    let karya = perpus_univ.unggah_karya_ilmiah(
        "KARYA-SKRIPSI-2026-0123",
        "H1D022001",
        "Rizky Pratama",
        "Informatika",
        "Fakultas Teknik",
        JenisKaryaIlmiah::SkripsiSarjana,
        dc_skripsi,
        11.2,
        Some("10.20884/skripsi.fasilkom.2026.0123"),
    ).expect("Unggah skripsi gagal");

    assert!(karya.lolos_uji_kemiripan);
    assert!(karya.status_oai_pmh_harvestable);

    // OAI-PMH Harvesting untuk RAMA / GARUDA Kemendiktisaintek
    let oai_records = perpus_univ.harvest_oai_pmh(None);
    assert_eq!(oai_records.len(), 1);
    assert!(oai_records[0].dublin_core_xml.contains("Penerapan Algoritma Kuantum"));

    // =========================================================================
    // 4. PERPUSTAKAAN INTERNASIONAL (Z39.50, IIIF MANIFEST, INTERLIBRARY LOAN)
    // =========================================================================
    let mut perpus_global = PerpustakaanInternasionalHub::new();

    // Z39.50 Query Search
    let z3950_req = Z3950QueryRequest {
        host: "z3950.loc.gov".to_string(),
        port: 210,
        database_name: "Voyager".to_string(),
        syntax_format: "MARC21".to_string(),
        rpn_query: "@attr 1=7 9780132350884".to_string(),
    };
    let z3950_res = perpus_global.eksekusi_z3950_search(z3950_req);
    assert_eq!(z3950_res.status_koneksi, "CONNECTED");
    assert_eq!(z3950_res.total_records_found, 1);
    assert_eq!(
        z3950_res.records[0].get_subfield("245", 'a').unwrap(),
        "Clean Code: A Handbook of Agile Software Craftsmanship"
    );

    // Interlibrary Loan (ILL) Internasional (Perpusnas RI <-> Leiden University Library)
    let ill = perpus_global.buat_permohonan_ill(
        "ILL-GLOBAL-2026-0089",
        "Perpustakaan Nasional Republik Indonesia (Jakarta)",
        "Universiteit Leiden Bibliotheek (Netherlands)",
        "Manuskrip Asli Babad Tanah Jawi (Cod. Or. 1786)",
        "OCLC: 65432198 / Leiden Cod.Or. 1786",
        "2026-03-01",
        "Digital Scan IIIF High Resolution Access",
    );
    assert_eq!(ill.status, StatusInterlibraryLoan::PermintaanDiajukan);

    // IIIF Manifest
    let iiif = IiifManifest {
        iiif_version: "3.0".to_string(),
        id_manifest_uri: "https://digitalcollections.universiteitleiden.nl/iiif/3/manifest/1786.json".to_string(),
        label: "Babad Tanah Jawi - Leiden Cod. Or. 1786".to_string(),
        viewing_direction: "left-to-right".to_string(),
        total_canvases: 450,
        thumbnail_uri: "https://digitalcollections.universiteitleiden.nl/iiif/3/image/1786_cover/full/300,/0/default.jpg".to_string(),
    };
    perpus_global.daftarkan_iiif_manifest(iiif);
    assert_eq!(perpus_global.daftar_iiif.len(), 1);
}
