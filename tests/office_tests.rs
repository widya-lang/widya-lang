use std::collections::HashMap;
use widya::office::*;

#[test]
fn test_docx_document_creation_and_text_extraction() {
    let mut doc = DocxDocument::new("Laporan Tahunan 2026");
    doc.add_heading("Bab 1: Pendahuluan", 1);
    doc.add_styled_paragraph("Widya-Lang adalah bahasa masa depan.", true, false, 12.0, Some("#003366"));
    doc.add_bullet("Kinerja tinggi tanpa runtime overhead.");
    doc.add_bullet("Dukungan ekosistem multi-industri.");
    doc.add_table(
        vec![
            vec!["Komponen".to_string(), "Status".to_string()],
            vec!["WidyaOffice".to_string(), "Produksi".to_string()],
            vec!["WidyaBase".to_string(), "Produksi".to_string()],
        ],
        true,
    );

    let plain_text = doc.extract_plain_text();
    assert!(plain_text.contains("Bab 1: Pendahuluan"));
    assert!(plain_text.contains("Widya-Lang adalah bahasa masa depan."));
    assert!(plain_text.contains("WidyaOffice"));

    let xml = doc.generate_document_xml();
    assert!(xml.starts_with(r#"<?xml version="1.0""#));
    assert!(xml.contains("<w:body>"));
    assert!(xml.contains("Heading1"));

    let html = doc.render_html_preview();
    assert!(html.contains("Laporan Tahunan 2026"));
    assert!(html.contains("btn-print"));
}

#[test]
fn test_docx_template_mail_merge_and_watermark() {
    let mut doc = DocxDocument::new("Surat Perjanjian Kerja Sama");
    doc.set_watermark("RAHASIA NEGARA", "#dc2626", 0.12);
    doc.add_heading("PERJANJIAN KERJASAMA NO: {{nomor_surat}}", 1);
    doc.add_paragraph("Pihak Pertama: {{nama_pihak_1}} ({{instansi_1}})");
    doc.add_paragraph("Pihak Kedua: {{nama_pihak_2}} ({{instansi_2}})");
    doc.add_table(
        vec![
            vec!["Pos Anggaran".to_string(), "Nilai Kontrak".to_string()],
            vec!["Alokasi Server & Cloud".to_string(), "{{nilai_kontrak}}".to_string()],
        ],
        true,
    );

    let mut vars = HashMap::new();
    vars.insert("nomor_surat".to_string(), "SPK/WIDYA/2026/001".to_string());
    vars.insert("nama_pihak_1".to_string(), "Dr. Satriyo P.".to_string());
    vars.insert("instansi_1".to_string(), "Kementerian Komdigi RI".to_string());
    vars.insert("nama_pihak_2".to_string(), "PT Widya Inovasi".to_string());
    vars.insert("instansi_2".to_string(), "Industri Berdaulat".to_string());
    vars.insert("nilai_kontrak".to_string(), "Rp 500.000.000".to_string());

    doc.apply_template(&vars);

    let plain = doc.extract_plain_text();
    assert!(plain.contains("SPK/WIDYA/2026/001"));
    assert!(plain.contains("Dr. Satriyo P."));
    assert!(plain.contains("Kementerian Komdigi RI"));
    assert!(plain.contains("Rp 500.000.000"));

    // Verifikasi ekspor biner .docx asli
    let docx_bytes = doc.export_to_docx_bytes();
    assert!(!docx_bytes.is_empty());
    assert_eq!(&docx_bytes[0..4], b"PK\x03\x04"); // Valid ZIP header
}

#[test]
fn test_docx_tte_digital_signature_and_verification() {
    let mut doc = DocxDocument::new("Keputusan Direksi");
    doc.add_heading("SURAT KEPUTUSAN PENGANGKATAN", 1);
    doc.add_paragraph("Menetapkan pengangkatan tim auditor independen.");

    let secret = "kunci_rahasia_tte_2026";
    let sig = doc.sign_document("Satriyo", "Direktur Utama", secret);

    assert_eq!(sig.signer_name, "Satriyo");
    assert_eq!(sig.signer_role, "Direktur Utama");
    assert!(!sig.signature_hex.is_empty());

    let plain_bytes = doc.extract_plain_text();
    let is_valid = OfficeSecurity::verify_signature(&sig, plain_bytes.as_bytes(), secret);
    assert!(is_valid, "Tanda tangan elektronik harus terverifikasi valid");

    let is_invalid = OfficeSecurity::verify_signature(&sig, b"Teks dimanipulasi", secret);
    assert!(!is_invalid, "Tanda tangan harus gagal jika konten diubah");
}

#[test]
fn test_uupdp_sensitive_data_auto_redaction() {
    let raw = "Data Pegawai: NIK 3271012345678901, No Rekening 1234567890, HP 081234567890, Email budi.santoso@kemdikbud.go.id";
    let redacted = OfficeSecurity::redact_pdp_sensitive_text(raw);

    assert!(redacted.contains("327101******8901"), "NIK harus disensor");
    assert!(redacted.contains("1234****90"), "Rekening harus disensor");
    assert!(redacted.contains("0812****7890"), "Nomor HP harus disensor");
    assert!(redacted.contains("b***o@kemdikbud.go.id"), "Email harus disensor");
}

#[test]
fn test_xlsx_extended_industry_formulas() {
    let mut sheet = Worksheet::new("Keuangan");
    sheet.set_cell(1, 1, CellValue::Text("Karyawan".to_string())); // A1
    sheet.set_cell(1, 2, CellValue::Number(15_000_000.0)); // B1 (Gaji)
    sheet.set_cell(2, 1, CellValue::Text("Budi".to_string())); // A2
    sheet.set_cell(2, 2, CellValue::Number(8_000_000.0)); // B2
    sheet.set_cell(3, 1, CellValue::Text("Siti".to_string())); // A3
    sheet.set_cell(3, 2, CellValue::Number(12_000_000.0)); // B3

    // 1. IF Formula
    let if_res = sheet.evaluate_formula("=IF(B1>=10000000, 2000000, 500000)").unwrap();
    assert_eq!(if_res, CellValue::Number(2_000_000.0));

    // 2. VLOOKUP Formula
    let vlookup_res = sheet.evaluate_formula("=VLOOKUP(\"Siti\", A1:B3, 2)").unwrap();
    assert_eq!(vlookup_res, CellValue::Number(12_000_000.0));

    // 3. CONCAT Formula
    let concat_res = sheet.evaluate_formula("=CONCAT(A2, \" - \", \"Staff\")").unwrap();
    assert_eq!(concat_res, CellValue::Text("Budi - Staff".to_string()));

    // 4. UPPER & LOWER & LEN
    let upper_res = sheet.evaluate_formula("=UPPER(A2)").unwrap();
    assert_eq!(upper_res, CellValue::Text("BUDI".to_string()));
    let len_res = sheet.evaluate_formula("=LEN(A2)").unwrap();
    assert_eq!(len_res, CellValue::Number(4.0));

    // 5. ROUND & ABS
    sheet.set_cell(4, 1, CellValue::Number(-125.768));
    let round_res = sheet.evaluate_formula("=ROUND(A4, 2)").unwrap();
    assert_eq!(round_res, CellValue::Number(-125.77));
    let abs_res = sheet.evaluate_formula("=ABS(A4)").unwrap();
    assert_eq!(abs_res, CellValue::Number(125.768));

    // 6. PMT Financial (Cicilan Pinjaman)
    let pmt_res = sheet.evaluate_formula("=PMT(0.01, 12, 10000000)").unwrap();
    if let CellValue::Number(val) = pmt_res {
        assert!(val < 0.0, "PMT menghasilkan nilai negatif untuk arus kas keluar");
        assert!((val.abs() - 888487.88).abs() < 10.0);
    } else {
        panic!("PMT harus menghasilkan angka");
    }

    // 7. Format Mata Uang Rupiah (IDR)
    let fmt = CellFormat {
        currency: Some("IDR".to_string()),
        decimals: Some(0),
        ..Default::default()
    };
    let display_str = CellValue::Number(15_000_000.0).format_with(&fmt);
    assert_eq!(display_str, "Rp 15.000.000");
}

#[test]
fn test_xlsx_native_binary_zip_export() {
    let mut wb = XlsxWorkbook::new("Buku Kas Utama 2026");
    let sheet = wb.get_sheet_mut("Sheet1").unwrap();
    sheet.set_cell(1, 1, CellValue::Text("Bulan".to_string()));
    sheet.set_cell(1, 2, CellValue::Text("Omset".to_string()));
    sheet.set_cell(2, 1, CellValue::Text("Januari".to_string()));
    sheet.set_cell(2, 2, CellValue::Number(50_000_000.0));

    let xlsx_bytes = wb.export_to_xlsx_bytes();
    assert!(!xlsx_bytes.is_empty());
    assert_eq!(&xlsx_bytes[0..4], b"PK\x03\x04"); // Valid ZIP header
}

#[test]
fn test_pptx_kpi_cards_and_native_binary_export() {
    let mut pres = PptxPresentation::new("Executive Dashboard 2026");
    pres.set_theme(SlideTheme::EmeraldFintech);
    pres.add_slide(PptxSlide::title_slide("Laporan Eksekutif Koperasi Digital", "Oleh Widya-Lang"));
    pres.add_slide(PptxSlide::kpi_slide(
        "Kinerja Portofolio Q3",
        vec![
            MetricKpiCard {
                label: "Total Aset".to_string(),
                value: "Rp 14.5 M".to_string(),
                subtext: "+18.2% YoY".to_string(),
                is_positive: true,
            },
            MetricKpiCard {
                label: "NPL (Kredit Macet)".to_string(),
                value: "0.4%".to_string(),
                subtext: "Sehat & Berdaulat".to_string(),
                is_positive: true,
            },
        ],
    ));

    assert_eq!(pres.slides.len(), 2);

    let html_slideshow = pres.render_html_slideshow();
    assert!(html_slideshow.contains("kpi-card"));
    assert!(html_slideshow.contains("Rp 14.5 M"));

    let pptx_bytes = pres.export_to_pptx_bytes();
    assert!(!pptx_bytes.is_empty());
    assert_eq!(&pptx_bytes[0..4], b"PK\x03\x04"); // Valid ZIP header
}

#[test]
fn test_pdf_document_tables_headers_and_watermark() {
    let mut pdf = PdfDocument::new("Surat Keputusan Resmi 2026");
    pdf.organization = "Kementerian Komdigi RI".to_string();
    pdf.set_watermark("SALINAN SAH");
    
    let headers = ["No.", "Nama Modul", "Tingkat Kesiapan"];
    let table_data = vec![
        vec!["1".to_string(), "WidyaOffice Engine".to_string(), "Tingkat 9 (Komersial)".to_string()],
        vec!["2".to_string(), "WidyaBase Transactional".to_string(), "Tingkat 9 (Komersial)".to_string()],
        vec!["3".to_string(), "WidyaTTE Digital Signature".to_string(), "Tingkat 9 (Komersial)".to_string()],
    ];
    pdf.add_table(&headers, &table_data, 50.0, 750.0, &[40.0, 250.0, 200.0]);

    let pdf_bytes = pdf.generate_pdf_bytes();
    assert!(!pdf_bytes.is_empty());
    assert!(pdf_bytes.starts_with(b"%PDF-1.7"));
    
    let pdf_str = String::from_utf8_lossy(&pdf_bytes);
    assert!(pdf_str.contains("Surat Keputusan Resmi 2026"));
    assert!(pdf_str.contains("SALINAN SAH"));
    assert!(pdf_str.contains("WidyaOffice Engine"));
}

#[test]
fn test_document_converter_docx_xlsx_markdown() {
    // 1. DOCX to PDF
    let mut doc = DocxDocument::new("Kontrak Kerja");
    doc.add_heading("SURAT PERJANJIAN KERJA", 1);
    doc.add_paragraph("Karyawan menyetujui seluruh ketentuan standar industri.");
    
    let pdf = DocumentConverter::docx_to_pdf(&doc);
    assert_eq!(pdf.title, "Kontrak Kerja");
    assert_eq!(pdf.pages.len(), 1);

    // 2. XLSX to PDF
    let mut wb = XlsxWorkbook::new("Laporan Laba Rugi");
    let sheet = wb.get_sheet_mut("Sheet1").unwrap();
    sheet.set_cell(1, 1, CellValue::Text("Pendapatan".to_string()));
    sheet.set_cell(1, 2, CellValue::Number(500_000_000.0));
    sheet.set_cell(2, 1, CellValue::Text("Beban Operasional".to_string()));
    sheet.set_cell(2, 2, CellValue::Number(200_000_000.0));

    let pdf_sheet = DocumentConverter::xlsx_to_pdf(&wb);
    assert_eq!(pdf_sheet.title, "Laporan Laba Rugi");

    // 3. Markdown to DOCX
    let md_text = "# Panduan Arsitektur\n\nWidya-Lang dirancang dengan performa maksimal.\n\n- Fitur 1: Fast Execution\n- Fitur 2: Memory Safe";
    let md = MarkdownDocument::parse(md_text);
    let doc_from_md = DocumentConverter::markdown_to_docx(&md);
    assert!(doc_from_md.elements.len() >= 3);

    // 4. LaTeX to DOCX
    let mut tex = LatexDocument::new("Paper Ilmiah", "Dr. Satriyo");
    tex.add_section("Pengantar Sistem");
    tex.add_paragraph("Widya-Lang adalah bahasa masa depan.");
    
    let doc_from_latex = DocumentConverter::latex_to_docx(&tex);
    assert_eq!(doc_from_latex.title, "Paper Ilmiah");
    assert!(doc_from_latex.extract_plain_text().contains("Pengantar Sistem"));
}

#[test]
fn test_crdt_track_changes_and_threaded_comments() {
    let mut session = DocumentReviewSession::new("Draft Regulasi AI");
    
    // Track Change: Tambahkan catatan perubahan
    session.add_change("Dr. Satriyo", ChangeType::Insertion, "Sistem AI", 0);
    
    // Tambah Komentar Bertingkat
    let c1_id = session.add_comment(
        "Dr. Siti",
        "paragraf_1",
        "Perlu penegasan mengenai kepatuhan UU PDP No 27/2022.",
    );
    
    // Balas komentar
    let replied = session.reply_comment(&c1_id, "Budi Hartono", "Setuju, saya tambahkan di ayat 2.");
    assert!(replied);

    assert_eq!(session.changes.len(), 1);
    assert_eq!(session.comments.len(), 1);
    assert_eq!(session.comments[0].replies.len(), 1);
    assert_eq!(session.comments[0].replies[0].author, "Budi Hartono");

    let mut collab = CollabSession::new("room-1", "Draft Regulasi AI", "user-1", "Dr. Satriyo");
    collab.crdt_engine.insert_text(0, "Draft Regulasi AI Nasional");
    let html_editor = collab.render_html_editor();
    assert!(html_editor.contains("Draft Regulasi AI Nasional"));
}

#[test]
fn test_batch_document_generation_engine() {
    let mut template = DocxDocument::new("Sertifikat Kompetensi");
    template.add_heading("SERTIFIKAT KELULUSAN", 1);
    template.add_paragraph("Diberikan kepada: {{nama_peserta}}");
    template.add_paragraph("Dengan predikat: {{predikat}}");
    template.add_paragraph("Tanggal Terbit: {{tanggal}}");

    let records = vec![
        [
            ("nama_peserta".to_string(), "Ahmad Fauzi".to_string()),
            ("predikat".to_string(), "Dengan Pujian (Cumlaude)".to_string()),
            ("tanggal".to_string(), "30 September 2026".to_string()),
        ].into_iter().collect::<HashMap<_, _>>(),
        [
            ("nama_peserta".to_string(), "Dewi Lestari".to_string()),
            ("predikat".to_string(), "Sangat Memuaskan".to_string()),
            ("tanggal".to_string(), "30 September 2026".to_string()),
        ].into_iter().collect::<HashMap<_, _>>(),
        [
            ("nama_peserta".to_string(), "Rian Pratama".to_string()),
            ("predikat".to_string(), "Memuaskan".to_string()),
            ("tanggal".to_string(), "30 September 2026".to_string()),
        ].into_iter().collect::<HashMap<_, _>>(),
    ];

    let (results, metrics) = BatchDocumentEngine::generate_batch_docx(&template, &records);
    assert_eq!(results.len(), 3);
    assert_eq!(metrics.total_documents, 3);

    assert!(results[0].extract_plain_text().contains("Ahmad Fauzi"));
    assert!(results[1].extract_plain_text().contains("Dewi Lestari"));
    assert!(results[2].extract_plain_text().contains("Rian Pratama"));

    // Batch ZIP export
    let (zip_bytes, zip_metrics) = BatchDocumentEngine::export_batch_to_zip(&template, &records, "sertifikat");
    assert_eq!(zip_metrics.total_documents, 3);
    assert!(!zip_bytes.is_empty());
    assert_eq!(&zip_bytes[0..4], b"PK\x03\x04");
}

#[test]
fn test_markdown_inline_runs_parsing_and_styling() {
    let text = "Ini **tebal**, ini *miring*, dan ini ***tebal miring*** serta `kode_fungsi()`. Underscore seperti pure_emitter.rs tetap aman.";
    let runs = parse_markdown_runs(text);

    assert!(runs.iter().any(|r| r.text == "tebal" && r.style.bold && !r.style.italic));
    assert!(runs.iter().any(|r| r.text == "miring" && r.style.italic && !r.style.bold));
    assert!(runs.iter().any(|r| r.text == "tebal miring" && r.style.bold && r.style.italic));
    assert!(runs.iter().any(|r| r.text == "kode_fungsi()" && r.style.is_monospace));
    assert!(runs.iter().any(|r| r.text.contains("pure_emitter.rs") && !r.style.italic));
}

#[test]
fn test_markdown_to_docx_and_pdf_with_styled_fonts() {
    let md_content = r#"# Laporan Arsitektur Berdaulat

Pilar utama mencakup **Kompiler Mandiri** dan *Sistem Operasi*.

- Gunakan `widya wpm init` untuk inisialisasi.
- Kunjungi [`Dokumentasi Resmi`](https://widya-lang.org) untuk panduan.

| Komponen | Status | Perintah |
|---|---|---|
| **Pilar I** | *Stabil* | `widya build` |
| **Pilar II** | *Aktif* | `widya run` |
"#;

    let md = MarkdownDocument::parse(md_content);
    let docx = DocumentConverter::markdown_to_docx(&md);

    // 1. Verifikasi XML DOCX memiliki tag <w:b/> dan <w:rFonts>
    let docx_xml = docx.generate_document_xml();
    assert!(docx_xml.contains("<w:b/>"), "DOCX harus memuat tag bold untuk teks tebal");
    assert!(docx_xml.contains("<w:i/>"), "DOCX harus memuat tag italic untuk teks miring");
    assert!(docx_xml.contains("Courier New"), "DOCX harus memuat font monospace untuk kode inline");

    // 2. Verifikasi PDF Vektor Asli memuat Type 1 font references (/F1, /F2, /F3, /F4)
    let pdf = DocumentConverter::docx_to_pdf(&docx);
    let pdf_bytes = pdf.generate_pdf_bytes();
    let pdf_str = String::from_utf8_lossy(&pdf_bytes);

    assert!(pdf_str.contains("/BaseFont /Helvetica"), "PDF harus mendefinisikan font Helvetica");
    assert!(pdf_str.contains("/BaseFont /Helvetica-Bold"), "PDF harus mendefinisikan font Helvetica-Bold");
    assert!(pdf_str.contains("/BaseFont /Helvetica-Oblique"), "PDF harus mendefinisikan font Helvetica-Oblique");
    assert!(pdf_str.contains("/BaseFont /Courier"), "PDF harus mendefinisikan font Courier");

    // Periksa bahwa stream teks benar-benar memanggil /F2 (bold) dan /F4 (code)
    assert!(pdf_str.contains("/F2"), "PDF harus menggunakan /F2 untuk teks bold");
    assert!(pdf_str.contains("/F4"), "PDF harus menggunakan /F4 untuk teks inline code");
}

#[test]
fn test_pdf_bookmarks_links_and_typographic_bullet() {
    let md_content = r#"# Panduan Penggunaan Widya

Bab ini menjelaskan cara mengunduh pustaka.

- Poin satu: Menggunakan standar WinAnsi.
- Poin dua: Silakan kunjungi [Portal Resmi Widya](https://widya-lang.org).

## Sub-Bab 1.1: Arsitektur

Berikut rincian teknis.
"#;

    let md = MarkdownDocument::parse(md_content);
    let pdf = DocumentConverter::markdown_to_pdf(&md);
    let bytes = pdf.generate_pdf_bytes();
    let pdf_str = String::from_utf8_lossy(&bytes);

    // 1. Verifikasi Interactive PDF Bookmarks (/Outlines & Outline Item)
    assert!(pdf_str.contains("/Outlines"), "PDF harus memiliki dictionary /Outlines di Catalog");
    assert!(pdf_str.contains("/Title (Panduan Penggunaan Widya)"), "PDF harus memiliki bookmark judul");
    assert!(pdf_str.contains("/Title (Sub-Bab 1.1: Arsitektur)"), "PDF harus memiliki bookmark sub-bab");

    // 2. Verifikasi Clickable Link Annotation (/Annots, /Subtype /Link, /URI)
    assert!(pdf_str.contains("/Annots"), "PDF Page harus memiliki array /Annots");
    assert!(pdf_str.contains("/Subtype /Link"), "PDF harus memiliki annotation tipe Link");
    assert!(pdf_str.contains("/URI (https://widya-lang.org)"), "PDF harus memiliki target URI tautan");

    // 3. Verifikasi Typographic Bullet Glyph (\225 WinAnsi bullet)
    assert!(pdf_str.contains(r"\225"), "PDF harus merender simbol bullet tipografis WinAnsi standar");
}

#[test]
fn test_ordered_list_and_table_column_alignment() {
    let md_content = r#"# Prosedur Rilis

1. Kompilasi biner dengan `cargo build --release`.
2. Uji suite dengan tautan [Dokumentasi](https://widya-lang.org/docs).
3. Tandatangani segel rilis.

| Nomor | Nama Komponen | Status | Skor |
| :--- | :---: | :---: | ---: |
| 1 | Kompiler ELF-64 | Aktif | 100 |
| 2 | Pure Crypto | Stabil | 98 |
"#;

    let md = MarkdownDocument::parse(md_content);

    // 1. Verifikasi Markdown Parsing
    assert!(md.blocks.iter().any(|b| matches!(b, MarkdownBlock::OrderedList(_))));
    if let Some(MarkdownBlock::Table { alignments, .. }) = md.blocks.iter().find(|b| matches!(b, MarkdownBlock::Table { .. })) {
        assert_eq!(alignments.len(), 4);
        assert_eq!(alignments[0], ColumnAlignment::Left);
        assert_eq!(alignments[1], ColumnAlignment::Center);
        assert_eq!(alignments[2], ColumnAlignment::Center);
        assert_eq!(alignments[3], ColumnAlignment::Right);
    } else {
        panic!("Tabel markdown harus terdeteksi dengan alignments");
    }

    // 2. Verifikasi Konversi ke HTML
    let html = md.to_html();
    assert!(html.contains("<ol>"), "HTML harus memuat tag <ol>");
    assert!(html.contains("text-align: center"), "HTML harus memuat text-align: center");
    assert!(html.contains("text-align: right"), "HTML harus memuat text-align: right");

    // 3. Verifikasi Konversi ke DOCX
    let docx = DocumentConverter::markdown_to_docx(&md);
    let docx_xml = docx.generate_document_xml();
    assert!(docx_xml.contains("<w:numPr>"), "DOCX harus memuat format list bernomor");
    assert!(docx_xml.contains(r#"<w:jc w:val="center"/>"#), "DOCX harus memuat alignment kolom tabel tengah");
    assert!(docx_xml.contains(r#"<w:jc w:val="right"/>"#), "DOCX harus memuat alignment kolom tabel kanan");

    // 4. Verifikasi Konversi ke PDF
    let pdf = DocumentConverter::docx_to_pdf(&docx);
    let bytes = pdf.generate_pdf_bytes();
    assert!(!bytes.is_empty(), "PDF harus ter-generate tanpa error");
    let pdf_str = String::from_utf8_lossy(&bytes);
    assert!(pdf_str.contains("1."), "PDF harus memuat nomor urut '1.'");
    assert!(pdf_str.contains("Kompiler ELF-64"), "PDF harus memuat teks tabel");
}

#[test]
fn test_mermaid_sequence_diagram_in_markdown_to_pdf() {
    let md_content = r#"# Alur Interaksi Layanan

Berikut adalah diagram urutan:

```mermaid
sequenceDiagram
    participant Browser
    participant Gateway as API Gateway
    participant DB as Postgres DB

    Browser->>Gateway: GET /status
    Gateway->>DB: SELECT health
    DB-->>Gateway: OK
    Gateway-->>Browser: 200 Health OK
```
"#;

    let md = MarkdownDocument::parse(md_content);
    let docx = DocumentConverter::markdown_to_docx(&md);
    let pdf = DocumentConverter::docx_to_pdf(&docx);
    let bytes = pdf.generate_pdf_bytes();

    assert!(!bytes.is_empty());
    let pdf_str = String::from_utf8_lossy(&bytes);
    assert!(pdf_str.contains("API Gateway"), "PDF harus memuat nama partisipan sequence diagram");
    assert!(pdf_str.contains("Postgres DB"), "PDF harus memuat label partisipan DB");
    assert!(pdf_str.contains("GET /status"), "PDF harus memuat pesan urutan request");
    assert!(pdf_str.contains("200 Health OK"), "PDF harus memuat pesan urutan response");
}


