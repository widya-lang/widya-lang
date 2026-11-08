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

#[test]
fn test_mermaid_state_and_class_diagram_in_markdown_to_pdf() {
    let md_content = r#"# Desain Sistem & Siklus Hidup

## Diagram Status
```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Running : RunCommand
    Running --> Terminated : ExitSuccess
    Running --> Error : Panic
    Error --> [*]
```

## Diagram Kelas
```mermaid
classDiagram
    class CompilerEngine {
        +String arch
        +emit_binary() Vec~u8~
    }
    class ElfLinker {
        +link_symbols() bool
    }
    CompilerEngine <|-- ElfLinker
```
"#;

    let md = MarkdownDocument::parse(md_content);
    let docx = DocumentConverter::markdown_to_docx(&md);
    let pdf = DocumentConverter::docx_to_pdf(&docx);
    let bytes = pdf.generate_pdf_bytes();

    assert!(!bytes.is_empty(), "PDF harus ter-generate tanpa error");
    let pdf_str = String::from_utf8_lossy(&bytes);

    // Verifikasi elemen State Diagram
    let pdf_upper = pdf_str.to_uppercase();
    assert!(pdf_upper.contains("DIAGRAM STATUS"), "PDF harus memuat header diagram status");
    assert!(pdf_str.contains("Idle"), "PDF harus memuat state Idle");
    assert!(pdf_str.contains("Running"), "PDF harus memuat state Running");
    assert!(pdf_str.contains("RunCommand"), "PDF harus memuat transisi RunCommand");

    // Verifikasi elemen Class Diagram
    assert!(pdf_upper.contains("DIAGRAM KELAS"), "PDF harus memuat header diagram kelas");
    assert!(pdf_str.contains("CompilerEngine"), "PDF harus memuat kelas CompilerEngine");
    assert!(pdf_str.contains("ElfLinker"), "PDF harus memuat kelas ElfLinker");
    assert!(pdf_str.contains("emit_binary"), "PDF harus memuat method emit_binary");
}

#[test]
fn test_epub_sovereign_generator_and_converter() {
    let md_content = r#"# Bab 1: Visi Kedaulatan Digital
Indonesia Berdaulat 2045 membutuhkan kemandirian teknologi mendalam.

* Bebas dari dependensi toolchain asing
* Kompiler mandiri tanpa LLVM/GCC

# Bab 2: Ekosistem Dokumen Cerdas
Format EPUB 3.2 memungkinkan pembacaan optimal di semua perangkat baca digital.

| Fitur | Status | Standar |
| :--- | :---: | ---: |
| Open Packaging | Aktif | OCF 3.2 |
| XHTML5 Semantic | Aktif | W3C / IDPF |
| Offline Reader | Mandiri | 100% Rust |

```rust
fn verifikasi() -> bool {
    true
}
```
"#;

    let md = MarkdownDocument::parse(md_content);
    let epub = DocumentConverter::markdown_to_epub(&md, Some("Kedaulatan Digital Nusantara"), Some("Tim Pengembang Widya"));

    assert_eq!(epub.title, "Kedaulatan Digital Nusantara");
    assert_eq!(epub.author, "Tim Pengembang Widya");
    assert_eq!(epub.chapters.len(), 2);

    let bytes = epub.export_to_epub_bytes();
    assert!(!bytes.is_empty());
    assert_eq!(&bytes[0..4], b"PK\x03\x04");

    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("mimetypeapplication/epub+zip"));
    assert!(text.contains("META-INF/container.xml"));
    assert!(text.contains("OEBPS/content.opf"));
    assert!(text.contains("OEBPS/toc.xhtml"));
    assert!(text.contains("OEBPS/style.css"));
    assert!(text.contains("Bab 1: Visi Kedaulatan Digital"));
    assert!(text.contains("Bab 2: Ekosistem Dokumen Cerdas"));
    assert!(text.contains("Kompiler mandiri tanpa LLVM/GCC"));
    assert!(text.contains("OCF 3.2"));
}

#[test]
fn test_code_block_syntax_highlighting_in_pdf() {
    let md_content = r#"# Contoh Kode Widya

```rust
pub fn hitung_luas(panjang: i64, lebar: i64) -> i64 {
    // Menghitung luas persegi panjang
    let hasil = panjang * lebar;
    hasil
}
```
"#;

    let md = MarkdownDocument::parse(md_content);
    let docx = DocumentConverter::markdown_to_docx(&md);
    let pdf = DocumentConverter::docx_to_pdf(&docx);
    let bytes = pdf.generate_pdf_bytes();

    assert!(!bytes.is_empty());
    let pdf_str = String::from_utf8_lossy(&bytes);
    // Verifikasi keyword dan komentar termuat dalam biner PDF
    assert!(pdf_str.contains("hitung_luas"));
    assert!(pdf_str.contains("Menghitung luas"));
}

#[test]
fn test_mermaid_er_diagram_in_markdown_to_pdf() {
    let md_content = r#"# Desain Skema Basis Data

```mermaid
erDiagram
    title: Model Relasi Transaksi WidyaPay
    NASABAH ||--o{ REKENING : memiliki
    REKENING ||--|{ MUTASI : mencatat

    NASABAH {
        string nik PK
        string nama_lengkap
        string email UK
    }

    REKENING {
        string nomor_rekening PK
        string nik FK
        float saldo
    }
```
"#;

    let md = MarkdownDocument::parse(md_content);
    let docx = DocumentConverter::markdown_to_docx(&md);
    let pdf = DocumentConverter::docx_to_pdf(&docx);
    let bytes = pdf.generate_pdf_bytes();

    assert!(!bytes.is_empty());
    let pdf_str = String::from_utf8_lossy(&bytes);

    // Verifikasi elemen ER Diagram
    assert!(pdf_str.contains("DIAGRAM ENTITAS-RELASI"), "PDF harus memuat header ER diagram");
    assert!(pdf_str.contains("NASABAH"), "PDF harus memuat entitas NASABAH");
    assert!(pdf_str.contains("REKENING"), "PDF harus memuat entitas REKENING");
    assert!(pdf_str.contains("nik"), "PDF harus memuat atribut nik");
    assert!(pdf_str.contains("nomor_rekening"), "PDF harus memuat atribut nomor_rekening");
    assert!(pdf_str.contains("PK"), "PDF harus memuat badge PK");
    assert!(pdf_str.contains("FK"), "PDF harus memuat badge FK");
    assert!(pdf_str.contains("memiliki"), "PDF harus memuat label relasi memiliki");
}

#[test]
fn test_footnotes_in_markdown_docx_pdf_and_epub() {
    let md_content = r#"# Naskah Kepatuhan Hukum

Berdasarkan regulasi keamanan data nasional[^1], seluruh enkripsi wajib menggunakan algoritma mandiri[^2].

[^1]: Undang-Undang Perlindungan Data Pribadi (UU PDP No. 27 Tahun 2022).
[^2]: Algoritma Kriptografi FIPS 180-4 dan RFC 8439.
"#;

    let md = MarkdownDocument::parse(md_content);

    // 1. Verifikasi AST Markdown
    assert_eq!(md.blocks.len(), 4);
    assert!(matches!(&md.blocks[2], MarkdownBlock::FootnoteDefinition { label, .. } if label == "1"));
    assert!(matches!(&md.blocks[3], MarkdownBlock::FootnoteDefinition { label, .. } if label == "2"));

    // 2. Verifikasi Konversi DOCX
    let docx = DocumentConverter::markdown_to_docx(&md);
    let docx_xml = docx.generate_document_xml();
    assert!(docx_xml.contains("[1]"), "DOCX harus memuat label catatan kaki [1]");
    assert!(docx_xml.contains("Undang-Undang Perlindungan Data Pribadi"), "DOCX harus memuat teks catatan kaki");

    // 3. Verifikasi Konversi PDF
    let pdf = DocumentConverter::docx_to_pdf(&docx);
    let pdf_bytes = pdf.generate_pdf_bytes();
    assert!(!pdf_bytes.is_empty());
    let pdf_str = String::from_utf8_lossy(&pdf_bytes);
    assert!(pdf_str.contains("[1]"));
    assert!(pdf_str.contains("Undang-Undang Perlindungan Data Pribadi"));
    assert!(pdf_str.contains("[2]"));

    // 4. Verifikasi Konversi EPUB
    let epub = DocumentConverter::markdown_to_epub(&md, Some("Naskah Kepatuhan Hukum"), Some("Biro Hukum"));
    let epub_bytes = epub.export_to_epub_bytes();
    assert!(!epub_bytes.is_empty());
    let epub_str = String::from_utf8_lossy(&epub_bytes);
    assert!(epub_str.contains("class=\"footnote\""));
    assert!(epub_str.contains("id=\"fn_1\""));
    assert!(epub_str.contains("Undang-Undang Perlindungan Data Pribadi"));
}

#[test]
fn test_epub_metadata_and_cover_image() {
    let mut doc = EpubDocument::new("Buku Putih Kedaulatan AI", "Pusat Riset Komputasi");
    doc.set_isbn("978-602-0000-00-1")
        .set_description("Panduan strategis kecerdasan buatan berdaulat nasional.")
        .set_subject("Teknologi, Komputasi, Kecerdasan Buatan")
        .set_rights("Hak Cipta Dilindungi Undang-Undang Republik Indonesia");

    // Simulasi gambar sampul PNG 1x1 piksel
    let dummy_png = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG Magic
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4,
        0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41,
        0x54, 0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00,
        0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00,
        0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE,
        0x42, 0x60, 0x82,
    ];
    doc.set_cover_image(&dummy_png, "image/png");

    doc.add_chapter("Bab 1: Pendahuluan", "<p>Pengantar ekosistem AI berdaulat.</p>");

    let bytes = doc.export_to_epub_bytes();
    assert!(!bytes.is_empty());
    assert_eq!(&bytes[0..4], b"PK\x03\x04");

    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("urn:isbn:978-602-0000-00-1"), "OPF harus memuat identifier ISBN");
    assert!(text.contains("Panduan strategis kecerdasan buatan"), "OPF harus memuat dc:description");
    assert!(text.contains("Teknologi, Komputasi"), "OPF harus memuat dc:subject");
    assert!(text.contains("Hak Cipta Dilindungi"), "OPF harus memuat dc:rights");
    assert!(text.contains("cover.xhtml"), "Manifes harus memuat cover.xhtml");
    assert!(text.contains("properties=\"cover-image\""), "Manifes harus memuat properti cover-image");
    assert!(text.contains("itemref idref=\"cover\""), "Spine harus memuat cover page di awal");
}

#[test]
fn test_auto_toc_in_markdown_to_pdf() {
    let md_content = r#"# Naskah Kebijakan Satu Data Indonesia

[TOC]

## Bab 1: Tata Kelola Data
Regulasi arsitektur interoperabilitas basis data lintas kementerian.

### 1.1 Standar Meta Data
Penetapan kamus data seragam.

## Bab 2: Infrastruktur Pusat Data
Implementasi komputasi awan berdaulat nasional.
"#;

    let md = MarkdownDocument::parse(md_content);
    assert!(md.blocks.iter().any(|b| matches!(b, MarkdownBlock::TableOfContents)));

    // Konversi langsung dengan TOC eksplisit dari markdown
    let pdf1 = DocumentConverter::markdown_to_pdf(&md);
    let bytes1 = pdf1.generate_pdf_bytes();
    assert!(!bytes1.is_empty());
    let pdf_str1 = String::from_utf8_lossy(&bytes1);
    assert!(pdf_str1.contains("DAFTAR ISI"), "PDF harus memuat blok header DAFTAR ISI");
    assert!(pdf_str1.contains("Bab 1: Tata Kelola Data"), "TOC harus memuat judul Bab 1");
    assert!(pdf_str1.contains("Infrastruktur Pusat Data"), "TOC harus memuat judul Bab 2");

    // Konversi dengan markdown_to_pdf_with_toc pada markdown tanpa tag [TOC]
    let md_no_toc = MarkdownDocument::parse(r#"# Panduan Teknis Kompiler
## Spesifikasi Arsitektur
Arsitektur target x86-64 murni tanpa libc.
"#);
    let pdf2 = DocumentConverter::markdown_to_pdf_with_toc(&md_no_toc);
    let bytes2 = pdf2.generate_pdf_bytes();
    assert!(!bytes2.is_empty());
    let pdf_str2 = String::from_utf8_lossy(&bytes2);
    assert!(pdf_str2.contains("DAFTAR ISI"), "markdown_to_pdf_with_toc harus otomatis menyisipkan DAFTAR ISI");
    assert!(pdf_str2.contains("Spesifikasi Arsitektur"), "TOC harus memuat subjudul");
}

#[test]
fn test_pptx_to_pdf_landscape_conversion() {
    let mut pres = PptxPresentation::new("Paparan Kedaulatan Digital");
    pres.set_theme(SlideTheme::DarkModern);

    // Slide 1: Judul
    pres.add_slide(PptxSlide::title_slide(
        "Kedaulatan Digital 2045",
        "Kemandirian Penuh Komputasi & Perangkat Lunak Indonesia",
    ));

    // Slide 2: Bullet points
    pres.add_slide(PptxSlide::content_slide(
        "Pilar Kemandirian",
        &[
            "Zero Dependency Toolchain Kompiler",
            "Mesin Dokumen & Desain Vektor Murni",
            "Algoritma Kriptografi Nasional",
        ],
    ));

    // Slide 3: KPI Dashboard
    pres.add_slide(PptxSlide::kpi_slide(
        "Capaian Indikator Kunci",
        vec![
            MetricKpiCard {
                label: "Kecepatan Kompilasi".to_string(),
                value: "0.012s".to_string(),
                subtext: "10x lebih gesit dari C/C++".to_string(),
                is_positive: true,
            },
            MetricKpiCard {
                label: "Ukuran Runtime Biner".to_string(),
                value: "14 KB".to_string(),
                subtext: "Tanpa overhead libc/glibc".to_string(),
                is_positive: true,
            },
        ],
    ));

    let pdf = DocumentConverter::pptx_to_pdf(&pres);
    let bytes = pdf.generate_pdf_bytes();
    assert!(!bytes.is_empty());
    assert_eq!(&bytes[0..4], b"%PDF");

    // Periksa dimensi MediaBox lanskap (842 x 595 pt)
    let pdf_str = String::from_utf8_lossy(&bytes);
    assert!(pdf_str.contains("/MediaBox [ 0 0 842 595 ]"), "PDF presentasi harus berorientasi lanskap 842x595 pt");
    assert!(pdf_str.contains("Kedaulatan Digital 2045"), "PDF harus memuat judul presentasi");
    assert!(pdf_str.contains("Pilar Kemandirian"), "PDF harus memuat judul slide 2");
    assert!(pdf_str.contains("Zero Dependency"), "PDF harus memuat isi slide 2");
    assert!(pdf_str.contains("Capaian Indikator Kunci"), "PDF harus memuat judul dashboard KPI");
    assert!(pdf_str.contains("Kecepatan Kompilasi"), "PDF harus memuat label KPI");
    assert!(pdf_str.contains("0.012s"), "PDF harus memuat nilai metrik KPI");
}

#[test]
fn test_xlsx_and_csv_to_pdf_conversion() {
    // 1. Uji XlsxWorkbook ke PDF
    let mut wb = XlsxWorkbook::new("Laporan Neraca Keuangan");
    let sheet = wb.get_sheet_mut("Sheet1").unwrap();
    sheet.set_cell(1, 1, CellValue::Text("Komponen Biaya".to_string()));
    sheet.set_cell(1, 2, CellValue::Text("Kuartal 1".to_string()));
    sheet.set_cell(1, 3, CellValue::Text("Kuartal 2".to_string()));

    sheet.set_cell(2, 1, CellValue::Text("Server Mandiri".to_string()));
    sheet.set_cell(2, 2, CellValue::Number(12500000.0));
    sheet.set_cell(2, 3, CellValue::Number(14000000.0));

    sheet.set_cell(3, 1, CellValue::Text("Riset Enkripsi".to_string()));
    sheet.set_cell(3, 2, CellValue::Number(8500000.0));
    sheet.set_cell(3, 3, CellValue::Number(9000000.0));

    let pdf = DocumentConverter::workbook_to_pdf(&wb);
    let bytes = pdf.generate_pdf_bytes();
    assert!(!bytes.is_empty());
    let pdf_str = String::from_utf8_lossy(&bytes);
    assert!(pdf_str.contains("Laporan Neraca Keuangan"), "PDF harus memuat judul buku kerja");
    assert!(pdf_str.contains("Server Mandiri"), "PDF harus memuat baris data spreadsheet");
    assert!(pdf_str.contains("Riset Enkripsi"), "PDF harus memuat data kedua");

    // 2. Uji CSV ke PDF via Worksheet CSV loader
    let csv_content = r#"ID,Nama Proyek,Anggaran,Status
101,Widya Kernel,50000000,Selesai
102,Widya Office,35000000,Produksi
103,Widya Crypto,20000000,Audit
"#;
    let mut csv_wb = XlsxWorkbook::new("Daftar Proyek Strategis");
    csv_wb.get_sheet_mut("Sheet1").unwrap().from_csv(csv_content);

    let csv_pdf = DocumentConverter::workbook_to_pdf(&csv_wb);
    let csv_pdf_bytes = csv_pdf.generate_pdf_bytes();
    assert!(!csv_pdf_bytes.is_empty());
    let csv_pdf_str = String::from_utf8_lossy(&csv_pdf_bytes);
    assert!(csv_pdf_str.contains("Nama Proyek"), "PDF dari CSV harus memuat header");
    assert!(csv_pdf_str.contains("Widya Kernel"), "PDF dari CSV harus memuat record 1");
    assert!(csv_pdf_str.contains("Widya Office"), "PDF dari CSV harus memuat record 2");
    assert!(csv_pdf_str.contains("Widya Crypto"), "PDF dari CSV harus memuat record 3");
}

#[test]
fn test_mermaid_pie_and_gantt_diagrams_in_markdown() {
    let md_content = r#"# Analisis Portfolio & Linimasa Proyek

## Distribusi Anggaran Riset
```mermaid
pie title Alokasi Dana Riset 2026
    "Kompiler ELF-64" : 45
    "Mesin Office Native" : 30
    "Kriptografi Nusantara" : 15
    "Edukasi Komunitas" : 10
```

## Jadwal Implementasi Rilis
```mermaid
gantt
    title Jadwal Peluncuran Versi 2.6
    dateFormat YYYY-MM-DD
    section Inti Kompiler
    Parsing AST             :done, 2026-01-01, 2026-01-15
    Penyusunan Kode Mesin   :active, 2026-01-16, 2026-02-15
    section Suite Office
    Konverter PDF & ToC     :done, 2026-02-01, 2026-02-28
    Pengujian Integrasi     :2026-03-01, 2026-03-15
```
"#;

    let md = MarkdownDocument::parse(md_content);
    let docx = DocumentConverter::markdown_to_docx(&md);
    let pdf = DocumentConverter::docx_to_pdf(&docx);
    let bytes = pdf.generate_pdf_bytes();

    assert!(!bytes.is_empty(), "PDF hasil konversi Pie & Gantt harus ter-generate");
    let pdf_str = String::from_utf8_lossy(&bytes);

    // Verifikasi Pie Chart
    assert!(pdf_str.contains("DIAGRAM LINGKARAN"), "PDF harus memuat header Pie Chart");
    assert!(pdf_str.contains("Alokasi Dana Riset 2026"), "PDF harus memuat judul Pie Chart");
    assert!(pdf_str.contains("Kompiler ELF-64"), "PDF harus memuat label irisan 1");
    assert!(pdf_str.contains("Mesin Office Native"), "PDF harus memuat label irisan 2");
    assert!(pdf_str.contains("Kriptografi Nusantara"), "PDF harus memuat label irisan 3");

    // Verifikasi Gantt Chart
    assert!(pdf_str.contains("DIAGRAM JADWAL & LINIMASA"), "PDF harus memuat header Gantt Chart");
    assert!(pdf_str.contains("Jadwal Peluncuran Versi 2.6"), "PDF harus memuat judul Gantt Chart");
    assert!(pdf_str.contains("Parsing AST"), "PDF harus memuat tugas 1 gantt");
    assert!(pdf_str.contains("Penyusunan Kode Mesin"), "PDF harus memuat tugas 2 gantt");
    assert!(pdf_str.contains("Konverter PDF & ToC"), "PDF harus memuat tugas 3 gantt");
    assert!(pdf_str.contains("Pengujian Integrasi"), "PDF harus memuat tugas 4 gantt");
}

#[test]
fn test_qr_code_pdf_rendering_and_svg_generation() {
    let url = "https://widya-lang.org/verify/doc-2026";
    let qr = WidyaQrCode::encode(url).expect("QR code harus berhasil di-encode");

    // 1. Verifikasi SVG Output
    let svg = qr.to_svg(5, 2);
    assert!(svg.starts_with("<svg"), "SVG harus memiliki tag pembuka");
    assert!(svg.contains("shape-rendering=\"crispEdges\""));
    assert!(svg.ends_with("</svg>"));

    // 2. Verifikasi Render ke PDF Vektor
    let mut pdf = PdfDocument::new("Sertifikat Ber-QR Code");
    qr.render_to_pdf(&mut pdf, 50.0, 600.0, 100.0);
    let bytes = pdf.generate_pdf_bytes();
    assert!(!bytes.is_empty());
    let pdf_str = String::from_utf8_lossy(&bytes);
    assert!(pdf_str.contains(" re f\n"), "PDF harus memuat modul-modul persegi QR");
}

#[test]
fn test_tte_visual_seal_with_qr_code() {
    let mut pdf = PdfDocument::new("Surat Keputusan Pengangkatan");
    pdf.current_page_mut().add_styled_text("SURAT KEPUTUSAN NO: 042/SK/DIR/2026", 50.0, 750.0, 14.0, true, (0.05, 0.15, 0.35));

    let dummy_doc_bytes = b"Isi Surat Keputusan Direksi Widya-Lang 2026";
    let sig = OfficeSecurity::sign_document(
        "Satriyo Purnomo",
        "Direktur Utama",
        "PT Inovasi Kedaulatan Digital",
        dummy_doc_bytes,
        "kunci_rahasia_tte_2026",
    );

    let next_y = OfficeSecurity::render_tte_visual_seal(&mut pdf, &sig, 50.0, 700.0, 480.0);
    assert!(next_y < 700.0);

    let bytes = pdf.generate_pdf_bytes();
    assert!(!bytes.is_empty());
    let pdf_str = String::from_utf8_lossy(&bytes);

    // Verifikasi teks segel resmi TTE
    assert!(pdf_str.contains("DOKUMEN INI TELAH DITANDATANGANI SECARA ELEKTRONIK"), "PDF harus memuat header segel resmi");
    assert!(pdf_str.contains("Satriyo Purnomo"), "PDF harus memuat nama penandatangan");
    assert!(pdf_str.contains("Direktur Utama"), "PDF harus memuat jabatan");
    assert!(pdf_str.contains("PT Inovasi Kedaulatan Digital"), "PDF harus memuat instansi");
    assert!(pdf_str.contains("ASLI & TERVERIFIKASI"), "PDF harus memuat status keaslian");
}

#[test]
fn test_pdf_text_extraction_engine() {
    let mut pdf = PdfDocument::new("Naskah Akademik Widya");
    let page = pdf.current_page_mut();
    page.add_styled_text("Kedaulatan Algoritma Nasional", 50.0, 700.0, 14.0, true, (0.1, 0.2, 0.3));
    page.add_styled_text("Kompiler mandiri tanpa LLVM atau GCC.", 50.0, 680.0, 10.0, false, (0.2, 0.3, 0.4));

    // 1. Ekstraksi langsung dari struktur dokumen
    let extracted_from_doc = pdf.extract_text();
    assert!(extracted_from_doc.contains("Kedaulatan Algoritma Nasional"));
    assert!(extracted_from_doc.contains("Kompiler mandiri tanpa LLVM"));

    // 2. Ekstraksi mandiri dari aliran biner byte PDF
    let bytes = pdf.generate_pdf_bytes();
    let extracted_from_bytes = PdfDocument::extract_text_from_bytes(&bytes).expect("Ekstraksi biner PDF harus sukses");
    assert!(extracted_from_bytes.contains("Kedaulatan Algoritma Nasional"));
    assert!(extracted_from_bytes.contains("Kompiler mandiri tanpa LLVM"));
}

#[test]
fn test_batch_document_mail_merge_pdf_with_qr() {
    let csv_data = r#"nik,nama,jabatan,gaji,url_verifikasi
3271012345678901,Budi Santoso,Perekayasa Utama,25000000,https://widya.id/v/101
3271019876543210,Siti Rahmawati,Spesialis Kriptografi,22000000,https://widya.id/v/102
"#;

    let records = BatchDocumentEngine::parse_csv_records(csv_data);
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].get("nama").unwrap(), "Budi Santoso");
    assert_eq!(records[1].get("jabatan").unwrap(), "Spesialis Kriptografi");

    let template_md = r#"# Slip Gaji Resmi Periode Oktober 2026

* **Nama Penerima**: {{nama}}
* **Nomor Induk Kependudukan**: {{nik}}
* **Jabatan**: {{jabatan}}
* **Gaji Bersih**: Rp {{gaji}}

Dokumen ini sah dan diterbitkan secara digital oleh Sistem Payroll Berdaulat.
"#;

    // 1. Generate Batch PDF Documents
    let (docs, metrics) = BatchDocumentEngine::generate_batch_pdf(template_md, &records, Some("url_verifikasi"));
    assert_eq!(docs.len(), 2);
    assert_eq!(metrics.total_documents, 2);

    let doc1_bytes = docs[0].generate_pdf_bytes();
    let doc1_str = String::from_utf8_lossy(&doc1_bytes);
    assert!(doc1_str.contains("Budi Santoso"));
    assert!(doc1_str.contains("Perekayasa Utama"));
    assert!(doc1_str.contains("Pindai Verifikasi"));

    // 2. Export Batch PDF ke Arsip ZIP Tunggal
    let (zip_bytes, zip_metrics) = BatchDocumentEngine::export_batch_pdf_to_zip(template_md, &records, "slip_gaji", Some("url_verifikasi"));
    assert_eq!(zip_metrics.total_documents, 2);
    assert!(!zip_bytes.is_empty());
    assert_eq!(&zip_bytes[0..4], b"PK\x03\x04");

    let zip_str = String::from_utf8_lossy(&zip_bytes);
    assert!(zip_str.contains("slip_gaji_0001.pdf"));
    assert!(zip_str.contains("slip_gaji_0002.pdf"));
}

#[test]
fn test_docx_native_equation_omml_and_mail_merge_batch() {
    // ------------------------------------------------------------------------
    // 1. Uji Penyisipan Persamaan Matematika (OMML / LaTeX Equation ke DOCX)
    // ------------------------------------------------------------------------
    let mut doc = DocxDocument::new("Jurnal Fisika Kuantum & Komputasi");
    doc.add_heading("Persamaan Relativitas Khusus & Matematika Gelombang", 1);
    doc.add_paragraph("Berikut adalah formula relativitas massa-energi Einstein:");
    
    // Formula blok: E = mc^2
    doc.add_equation("E = m c^2");
    
    // Formula pecahan & akar kuadrat: x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
    doc.add_paragraph("Solusi rumus kuadratik abc:");
    doc.add_equation("x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}");

    // Formula integral & simbol yunani: \int_{0}^{\infty} e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
    doc.add_paragraph("Integral Gaussian probabilitas:");
    doc.add_equation("\\int e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}");

    // Inline equation
    doc.add_inline_equation("\\alpha + \\beta = \\gamma");

    let xml = doc.generate_document_xml();
    // Verifikasi namespace math Microsoft Word
    assert!(xml.contains(r#"xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math""#));
    // Verifikasi tag OMML
    assert!(xml.contains("<m:oMathPara>"));
    assert!(xml.contains("<m:oMath>"));
    assert!(xml.contains("<m:f>")); // fraction
    assert!(xml.contains("<m:num>")); // numerator
    assert!(xml.contains("<m:den>")); // denominator
    assert!(xml.contains("<m:rad>")); // radical / sqrt
    assert!(xml.contains("<m:sSup>")); // superscript
    assert!(xml.contains("π")); // simbol pi
    assert!(xml.contains("±")); // plus-minus

    // Verifikasi HTML preview mengandung formula
    let html = doc.render_html_preview();
    assert!(html.contains("docx-equation"));
    assert!(html.contains("E = m c^2"));

    // Verifikasi ekspor DOCX ECMA-376 biner
    let docx_bytes = doc.export_to_docx_bytes();
    assert!(!docx_bytes.is_empty());
    assert_eq!(&docx_bytes[0..4], b"PK\x03\x04");

    // ------------------------------------------------------------------------
    // 2. Uji Mail Merge Massal Berdasarkan Dataset
    // ------------------------------------------------------------------------
    let mut template_doc = DocxDocument::new("Surat Keputusan Kelulusan");
    template_doc.add_heading("SURAT KEPUTUSAN NO: {{nomor_sk}}", 1);
    template_doc.add_paragraph("Diberikan kepada: {{nama}} (NIM: {{nim}})");
    template_doc.add_paragraph("Program Studi: {{prodi}} dengan predikat: {{predikat}}");
    template_doc.add_paragraph("Indeks Prestasi Kumulatif (IPK): {{ipk}}");
    template_doc.add_equation("\\text{IPK} = \\frac{\\sum (K_i \\times N_i)}{\\sum K_i} = {{ipk}}");

    let mut records = Vec::new();
    let mut r1 = HashMap::new();
    r1.insert("nomor_sk".to_string(), "SK/001/2026".to_string());
    r1.insert("nama".to_string(), "Ahmad Fauzi".to_string());
    r1.insert("nim".to_string(), "202601001".to_string());
    r1.insert("prodi".to_string(), "Teknik Informatika Berdaulat".to_string());
    r1.insert("predikat".to_string(), "Dengan Pujian (Cum Laude)".to_string());
    r1.insert("ipk".to_string(), "3.92".to_string());
    records.push(r1);

    let mut r2 = HashMap::new();
    r2.insert("nomor_sk".to_string(), "SK/002/2026".to_string());
    r2.insert("nama".to_string(), "Siti Nurhaliza".to_string());
    r2.insert("nim".to_string(), "202601002".to_string());
    r2.insert("prodi".to_string(), "Kriptografi & Keamanan Sistem".to_string());
    r2.insert("predikat".to_string(), "Sangat Memuaskan".to_string());
    r2.insert("ipk".to_string(), "3.85".to_string());
    records.push(r2);

    // Jalankan mail merge massal
    let merged_docs = template_doc.mail_merge_batch(&records);
    assert_eq!(merged_docs.len(), 2);

    // Verifikasi dokumen 1
    let plain1 = merged_docs[0].extract_plain_text();
    assert!(plain1.contains("Ahmad Fauzi"));
    assert!(plain1.contains("202601001"));
    assert!(plain1.contains("SK/001/2026"));
    assert!(plain1.contains("3.92"));

    // Verifikasi dokumen 2
    let plain2 = merged_docs[1].extract_plain_text();
    assert!(plain2.contains("Siti Nurhaliza"));
    assert!(plain2.contains("202601002"));
    assert!(plain2.contains("SK/002/2026"));
    assert!(plain2.contains("3.85"));

    // Verifikasi XML dokumen termerge mengandung formula dengan variabel terisi
    let xml1 = merged_docs[0].generate_document_xml();
    assert!(xml1.contains("3.92"));
    let xml2 = merged_docs[1].generate_document_xml();
    assert!(xml2.contains("3.85"));
}


