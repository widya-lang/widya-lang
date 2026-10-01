use widya::office::*;

#[test]
fn test_native_pdf_iso_binary_generation() {
    let mut pdf = PdfDocument::new("Laporan Resmi ISO PDF");
    pdf.current_page_mut().add_text("Laporan Tahunan Widya-Lang", 50.0, 780.0, 18.0, true);
    pdf.current_page_mut().add_line(50.0, 770.0, 545.0, 770.0, 1.5);
    pdf.current_page_mut().add_text("Halaman 1: Pendahuluan Ekosistem", 50.0, 740.0, 12.0, false);

    let page2 = pdf.add_page();
    page2.add_text("Halaman 2: Rincian Anggaran", 50.0, 780.0, 14.0, true);

    let bytes = pdf.generate_pdf_bytes();
    let pdf_str = String::from_utf8_lossy(&bytes);

    assert!(pdf_str.starts_with("%PDF-1.7"));
    assert!(pdf_str.contains("/Type /Catalog"));
    assert!(pdf_str.contains("/Type /Pages"));
    assert!(pdf_str.contains("Laporan Tahunan Widya-Lang"));
    assert!(pdf_str.contains("xref"));
    assert!(pdf_str.ends_with("%%EOF\n"));
}

#[test]
fn test_latex_tex_generation_and_math_preview() {
    let mut tex = LatexDocument::new("Mekanika Kuantum Lanjutan", "Prof. Widya");
    tex.add_section("Persamaan Gelombang");
    tex.add_paragraph("Persamaan dasar gelombang kuantum Schrodinger:");
    tex.add_equation(r#"i \hbar \frac{\partial}{\partial t} \Psi(\mathbf{r}, t) = \hat{H} \Psi(\mathbf{r}, t)"#);
    tex.add_table(
        vec!["Simbol".to_string(), "Makna".to_string()],
        vec![
            vec![r#"\hbar"#.to_string(), "Konstanta Planck Tereduksi".to_string()],
            vec![r#"\hat{H}"#.to_string(), "Operator Hamiltonian".to_string()],
        ],
    );

    let tex_source = tex.generate_tex_source();
    assert!(tex_source.contains(r#"\documentclass{article}"#));
    assert!(tex_source.contains(r#"\begin{equation}"#));
    assert!(tex_source.contains(r#"\Psi(\mathbf{r}, t)"#));

    let html_preview = tex.render_html_math_preview();
    assert!(html_preview.contains("katex.min.js"));
    assert!(html_preview.contains("Mekanika Kuantum Lanjutan"));
}

#[test]
fn test_markdown_gfm_parser_and_html_transpiler() {
    let md_text = r#"
# Proposal Riset AI
Dokumen ini dibuat menggunakan **Widya Markdown**.

- [x] Selesaikan Modul Office
- [ ] Integrasi Cloud Drive

| Fitur | Status |
|---|---|
| DOCX | Siap |
| PDF | Siap |

```rust
fn main() {
    println!("Widya-Lang");
}
```
"#;

    let doc = MarkdownDocument::parse(md_text);
    assert_eq!(doc.blocks.len(), 5);

    let html = doc.to_html();
    assert!(html.contains("<h1>Proposal Riset AI</h1>"));
    assert!(html.contains("<strong>Widya Markdown</strong>"));
    assert!(html.contains("<ul class='task-list'>"));
    assert!(html.contains("<table>"));
    assert!(html.contains("<code class='language-rust'>"));
}

#[test]
fn test_widyadrive_sovereign_cloud_storage() {
    let mut drive = WidyaDrive::new("DriveProduksi");
    let file_id = drive.upload_file("dokumen_rahasia.docx", b"Isi Rahasia Perusahaan", "admin").unwrap();

    assert!(drive.files.contains_key(&file_id));
    let downloaded = drive.download_file(&file_id).unwrap();
    assert_eq!(downloaded.name, "dokumen_rahasia.docx");
    assert_eq!(downloaded.owner, "admin");

    let explorer_html = drive.render_web_explorer();
    assert!(explorer_html.contains("WidyaDrive Sovereign Cloud"));
    assert!(explorer_html.contains("dokumen_rahasia.docx"));
}

#[test]
fn test_widya_workspace_dashboard_suite() {
    let ws = WidyaWorkspace::new("KementerianDigital", "Pemerintah RI");
    assert_eq!(ws.active_members.len(), 3);
    assert!(!ws.drive.files.is_empty());

    let dashboard_html = ws.render_workspace_dashboard();
    assert!(dashboard_html.contains("KementerianDigital"));
    assert!(dashboard_html.contains("WidyaDocs"));
    assert!(dashboard_html.contains("WidyaSheets"));
    assert!(dashboard_html.contains("100% Data Sovereign"));
}

#[test]
fn test_widya_script_workspace_execution() {
    let script = r##"
        // 1. PDF
        biar pdf = office_pdf_buat("Sertifikat Resmi")
        biar pdf_update = office_pdf_tambah_teks(pdf, "Sertifikat Kelulusan Widya-Lang", 50.0, 750.0, 16.0)

        // 2. LaTeX
        biar tex = office_latex_buat("Fisika Relativitas", "Albert Einstein")
        biar tex_update = office_latex_tambah_persamaan(tex, "E = mc^2")

        // 3. Markdown
        biar html_md = office_markdown_ke_html("# Judul Utama\nTeks deskripsi.")
        pastikan_benar(panjang(html_md) > 10)

        // 4. WidyaDrive
        biar drive = widyadrive_buat("DriveTim")
        biar upload_res = widyadrive_unggah(drive, "data.txt", "123456789")
        pastikan_sama(upload_res["total_berkas"], 1.0)

        // 5. Widya Workspace
        biar ws = widyaworkspace_buat("RuangKerjaUtama", "Inovasi Nusantara")
        biar ws_html = widyaworkspace_render_dashboard(ws)
        pastikan_benar(panjang(ws_html) > 100)
    "##;

    let res = widya::jalankan(script);
    assert!(res.is_ok(), "Eksekusi skrip Workspace gagal: {:?}", res.err());
}
