use widya::journal::{
    AkreditasiJurnalArjuna, ArtikelJurnalIlmiah, BibliometricEngine, MetrikScopusJurnal,
    MetrikWebOfScienceJurnal, PenulisJurnal, PeringkatSinta, ProfilPenelitiSinta,
    ReviewerRecord, ScopusQuartile, StatusDoajIndex, StatusReviewArtikel, WebOfScienceIndex,
};

#[test]
fn test_scholarly_journal_sinta_scopus_and_bibliometrics() {
    // =========================================================================
    // 1. SIKLUS PENERBITAN ARTIKEL OJS & CROSSREF DOI XML
    // =========================================================================
    let mut artikel = ArtikelJurnalIlmiah::new(
        "ART-2026-0042",
        "10.1234/widya.v10i2.2026",
        "Widya-Lang: A Sovereign Next-Generation Multi-Paradigm Programming Language for Critical Industrial and Scientific Systems",
        "Widya-Lang provides memory safety without garbage collection latency, formal verification, and bilingual native syntax.",
        "Journal of Advanced Computer Science and Technology",
        "2655-9876",
        10,
        2,
        2026,
    );

    artikel.tambah_penulis(PenulisJurnal {
        nama_lengkap: "Pratama, Rizky".to_string(),
        email: "rizky@widya-lang.org".to_string(),
        afiliasi: "Department of Computer Science, Universitas Gadjah Mada".to_string(),
        ror_id_institusi: Some("https://ror.org/03ydzp604".to_string()),
        orcid_id: Some("0000-0002-1825-0097".to_string()),
        corresponding_author: true,
    });

    artikel.kirim_ke_reviewer();
    assert_eq!(artikel.status, StatusReviewArtikel::InReviewWithReviewers);

    // Hasil Double-Blind Peer Review
    artikel.tambahkan_hasil_review(ReviewerRecord {
        nama_reviewer: "Prof. John Doe, Ph.D.".to_string(),
        afiliasi: "MIT EECS".to_string(),
        tanggal_selesai_review: "2026-02-10".to_string(),
        skor_kualitas_metodologi: 95,
        rekomendasi: "Accept".to_string(),
        catatan_ke_penulis: "Excellent contribution to memory safety and formal verification language design.".to_string(),
    });

    artikel.terbitkan_online("2026-02-28");
    assert_eq!(artikel.status, StatusReviewArtikel::PublishedOnline);

    // Verifikasi Crossref DOI Deposit XML
    let xml = artikel.generate_crossref_xml();
    assert!(xml.contains("10.1234/widya.v10i2.2026"));
    assert!(xml.contains("0000-0002-1825-0097"));
    assert!(xml.contains("Journal of Advanced Computer Science and Technology"));

    // =========================================================================
    // 2. AKREDITASI SINTA ARJUNA & PROFIL PENELITI
    // =========================================================================
    let jurnal_sinta1 = AkreditasiJurnalArjuna {
        nama_jurnal: "Indonesian Journal of Computing and Cybernetics Systems".to_string(),
        e_issn: "2460-7258".to_string(),
        nomor_sk_akreditasi: "SK-DIRJEN-DIKTI-2026-012".to_string(),
        peringkat_sinta: PeringkatSinta::Sinta1, // SINTA 1 Terindeks Scopus Q2
        masa_berlaku_mulai_tahun: 2024,
        masa_berlaku_sampai_tahun: 2029,
        nilai_evaluasi_arjuna: 92.5,
        penerbit_lembaga: "Universitas Gadjah Mada".to_string(),
    };
    assert_eq!(jurnal_sinta1.peringkat_sinta, PeringkatSinta::Sinta1);

    let mut dosen_sinta = ProfilPenelitiSinta {
        sinta_id: "6001234".to_string(),
        nidn_nidk: "0012058001".to_string(),
        nama_dosen: "Dr. Eng. Hendro Wicaksono".to_string(),
        perguruan_tinggi: "Universitas Jenderal Soedirman".to_string(),
        program_studi: "Teknik Informatika".to_string(),
        sinta_score_overall: 450.0,
        sinta_score_3yr: 180.0,
        sinta_score_productivity: 85.0,
        total_sitasi_google_scholar: 1420,
        h_index_google_scholar: 18,
        scopus_author_id: Some("57200112233".to_string()),
        total_dokumen_scopus: 24,
        total_sitasi_scopus: 380,
        h_index_scopus: 11,
        wos_researcher_id: Some("AAC-1234-2026".to_string()),
        garuda_id: Some("GARUDA-445566".to_string()),
    };

    // Tambah 1 publikasi Q1 Scopus (+40) dan 1 SINTA 2 (+25)
    dosen_sinta.perbarui_skor_sinta(1, 1);
    assert_eq!(dosen_sinta.sinta_score_3yr, 245.0); // 180 + 65 = 245
    assert_eq!(dosen_sinta.sinta_score_overall, 515.0); // 450 + 65 = 515

    // =========================================================================
    // 3. INDEKSASI INTERNASIONAL BEREPUTASI (SCOPUS QUARTILE & WEB OF SCIENCE)
    // =========================================================================
    let scopus_q1 = MetrikScopusJurnal {
        nama_jurnal: "IEEE Transactions on Software Engineering".to_string(),
        scopus_source_id: "19300156903".to_string(),
        issn: "0098-5589".to_string(),
        bidang_subjek: "Software Engineering & Computer Science".to_string(),
        quartile: ScopusQuartile::Q1,
        citescore: 12.8,
        scimago_sjr: 3.24,
        snip: 3.85,
        persentil_peringkat: 99.0,
        penerbit_internasional: "IEEE Computer Society".to_string(),
    };
    assert_eq!(scopus_q1.quartile, ScopusQuartile::Q1);

    let wos_scie = MetrikWebOfScienceJurnal {
        nama_jurnal: "ACM Computing Surveys".to_string(),
        wos_accession_id: "WOS:00088991122".to_string(),
        indeks_koleksi: WebOfScienceIndex::ScienceCitationIndexExpanded,
        journal_impact_factor_jif: 16.6,
        journal_citation_indicator_jci: 4.15,
        kategori_wos: "Computer Science, Theory & Methods".to_string(),
    };
    assert_eq!(wos_scie.journal_impact_factor_jif, 16.6);

    let doaj = StatusDoajIndex {
        terdaftar_doaj: true,
        doaj_seal_awarded: true,
        apc_amount_usd: 0.0, // Diamond Open Access
        lisensi_resmi: "CC BY 4.0".to_string(),
    };
    assert!(doaj.doaj_seal_awarded);

    // =========================================================================
    // 4. ANALISIS BIBLIOMETRIK & SITASI MULTI-FORMAT
    // =========================================================================
    // H-Index test: sitasi [50, 25, 12, 10, 8, 5, 2, 1] -> 5 artikel memiliki >= 5 sitasi -> H-Index = 5
    let daftar_sitasi = vec![50, 25, 12, 10, 8, 5, 2, 1];
    let h_idx = BibliometricEngine::hitung_h_index(daftar_sitasi.clone());
    assert_eq!(h_idx, 5);

    // i10-Index test: artikel dengan >= 10 sitasi adalah 50, 25, 12, 10 -> i10-Index = 4
    let i10_idx = BibliometricEngine::hitung_i10_index(&daftar_sitasi);
    assert_eq!(i10_idx, 4);

    // Format APA 7th Edition
    let apa = BibliometricEngine::format_apa_7th(
        "Pratama, R., & Wicaksono, H.",
        2026,
        "Widya-Lang: A Sovereign Next-Generation Multi-Paradigm Programming Language",
        "Journal of Advanced Computer Science",
        10,
        2,
        1,
        15,
        "10.1234/widya.v10i2.2026",
    );
    assert!(apa.contains("https://doi.org/10.1234/widya.v10i2.2026"));

    // Format IEEE
    let ieee = BibliometricEngine::format_ieee(
        "R. Pratama and H. Wicaksono",
        "Widya-Lang: A Sovereign Next-Generation Multi-Paradigm Programming Language",
        "Journal of Advanced Computer Science",
        10,
        2,
        1,
        15,
        2026,
        "10.1234/widya.v10i2.2026",
    );
    assert!(ieee.contains("vol. 10, no. 2"));

    // Format BibTeX
    let bibtex = BibliometricEngine::format_bibtex(
        "pratama2026widya",
        "Pratama, Rizky and Wicaksono, Hendro",
        "Widya-Lang: A Sovereign Next-Generation Multi-Paradigm Programming Language",
        "Journal of Advanced Computer Science",
        10,
        2,
        "1--15",
        2026,
        "10.1234/widya.v10i2.2026",
    );
    assert!(bibtex.starts_with("@article{pratama2026widya"));
}
