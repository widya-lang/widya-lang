use crate::office::docx::{DocxDocument, DocxElement};
use crate::office::latex::{LatexDocument, LatexElement};
use crate::office::markdown::{MarkdownBlock, MarkdownDocument};
use crate::office::pdf::PdfDocument;
use crate::office::xlsx::XlsxWorkbook;

/// Mesin Pipeline Konversi Dokumen Universal Lintas Format
pub struct DocumentConverter;

impl DocumentConverter {
    /// Konversi Dokumen Word (`DocxDocument`) ke Dokumen PDF Vektor Asli (`PdfDocument`)
    pub fn docx_to_pdf(doc: &DocxDocument) -> PdfDocument {
        let mut pdf = PdfDocument::new(&doc.title);
        pdf.author = doc.author.clone();
        pdf.organization = doc.organization.clone();

        if let Some(ref wm) = doc.watermark {
            pdf.set_watermark(&wm.text);
        }

        let (_, page_height) = pdf.page_size.dimensions_points();
        let mut current_y = page_height - 70.0;
        let margin_x = 50.0;
        let content_width = 495.0;

        for el in &doc.elements {
            if current_y < 90.0 {
                pdf.add_page();
                current_y = page_height - 70.0;
            }

            match el {
                DocxElement::Heading { level, text } => {
                    let font_size = match level {
                        1 => 18.0,
                        2 => 14.0,
                        3 => 12.0,
                        _ => 11.0,
                    };
                    current_y -= font_size + 4.0;
                    pdf.current_page_mut().add_styled_text(
                        text,
                        margin_x,
                        current_y,
                        font_size,
                        true,
                        (0.05, 0.15, 0.35),
                    );
                    if *level == 1 {
                        current_y -= 4.0;
                        pdf.current_page_mut().add_styled_line(
                            margin_x,
                            current_y,
                            margin_x + content_width,
                            current_y,
                            1.2,
                            (0.1, 0.4, 0.8),
                        );
                    }
                    current_y -= 10.0;
                }
                DocxElement::Paragraph { runs, .. } => {
                    let mut line_text = String::new();
                    for r in runs {
                        line_text.push_str(&r.text);
                    }
                    current_y -= 14.0;
                    pdf.current_page_mut().add_styled_text(
                        &line_text,
                        margin_x,
                        current_y,
                        10.0,
                        false,
                        (0.15, 0.2, 0.25),
                    );
                    current_y -= 6.0;
                }
                DocxElement::BulletItem { text, .. } => {
                    current_y -= 14.0;
                    let bullet_text = format!("* {}", text);
                    pdf.current_page_mut().add_styled_text(
                        &bullet_text,
                        margin_x + 12.0,
                        current_y,
                        9.5,
                        false,
                        (0.2, 0.25, 0.3),
                    );
                    current_y -= 4.0;
                }
                DocxElement::Table { rows, has_header } => {
                    if !rows.is_empty() {
                        let col_count = rows[0].len().max(1);
                        let col_w = content_width / col_count as f32;
                        let widths: Vec<f32> = vec![col_w; col_count];

                        let header_slice: Vec<&str> = if *has_header {
                            rows[0].iter().map(|s| s.as_str()).collect()
                        } else {
                            (1..=col_count).map(|_| "Kolom").collect()
                        };

                        let data_rows = if *has_header && rows.len() > 1 {
                            &rows[1..]
                        } else {
                            &rows[..]
                        };

                        current_y = pdf.add_table(&header_slice, data_rows, margin_x, current_y, &widths);
                        current_y -= 12.0;
                    }
                }
                DocxElement::Callout { title, text, .. } => {
                    current_y -= 36.0;
                    let page = pdf.current_page_mut();
                    page.add_rect(margin_x, current_y - 8.0, content_width, 36.0, true, (0.94, 0.97, 1.0));
                    page.add_styled_line(margin_x, current_y - 8.0, margin_x, current_y + 28.0, 3.5, (0.1, 0.5, 0.85));
                    page.add_styled_text(&format!("{}: {}", title, text), margin_x + 10.0, current_y + 6.0, 9.5, true, (0.05, 0.25, 0.5));
                    current_y -= 14.0;
                }
                DocxElement::PageBreak => {
                    pdf.add_page();
                    current_y = page_height - 70.0;
                }
            }
        }

        // TTE Seal Badge di akhir PDF jika ada
        if !doc.digital_signatures.is_empty() {
            if current_y < 120.0 {
                pdf.add_page();
                current_y = page_height - 70.0;
            }
            current_y -= 20.0;
            let page = pdf.current_page_mut();
            page.add_styled_line(margin_x, current_y, margin_x + content_width, current_y, 0.8, (0.7, 0.8, 0.7));
            current_y -= 18.0;

            for sig in &doc.digital_signatures {
                let badge_text = format!("TERTANDA SECARA ELEKTRONIK (TTE) - {} ({})", sig.signer_name, sig.signer_role);
                page.add_styled_text(&badge_text, margin_x, current_y, 9.0, true, (0.05, 0.5, 0.3));
                current_y -= 12.0;
                let hash_text = format!("Segel SHA256: {} | {}", &sig.signature_hex[..24], sig.timestamp);
                page.add_styled_text(&hash_text, margin_x, current_y, 7.5, false, (0.3, 0.4, 0.35));
                current_y -= 16.0;
            }
        }

        pdf
    }

    /// Konversi Spreadsheet Excel (`XlsxWorkbook`) ke Laporan Rekapitulasi PDF
    pub fn xlsx_to_pdf(wb: &XlsxWorkbook) -> PdfDocument {
        let mut pdf = PdfDocument::new(&wb.title);
        pdf.author = wb.author.clone();

        let (_, page_height) = pdf.page_size.dimensions_points();
        let margin_x = 50.0;
        let content_width = 495.0;

        for (sheet_idx, sheet) in wb.sheets.iter().enumerate() {
            if sheet_idx > 0 {
                pdf.add_page();
            }

            let mut current_y = page_height - 70.0;
            let page = pdf.current_page_mut();

            // Judul Sheet
            page.add_styled_text(&format!("Laporan Lembar Kerja: {}", sheet.name), margin_x, current_y, 14.0, true, (0.05, 0.2, 0.4));
            current_y -= 6.0;
            page.add_styled_line(margin_x, current_y, margin_x + content_width, current_y, 1.0, (0.2, 0.4, 0.7));
            current_y -= 16.0;

            if sheet.max_row > 0 && sheet.max_col > 0 {
                let col_count = sheet.max_col;
                let col_w = content_width / col_count as f32;
                let widths = vec![col_w; col_count as usize];

                let mut headers = Vec::new();
                for c in 1..=col_count {
                    headers.push(sheet.get_cell(1, c).to_display_string());
                }
                let header_refs: Vec<&str> = headers.iter().map(|s| s.as_str()).collect();

                let mut data_rows = Vec::new();
                for r in 2..=sheet.max_row {
                    let mut row_vals = Vec::new();
                    for c in 1..=col_count {
                        let val = sheet.get_cell(r, c);
                        let display = if let Some(fmt) = sheet.formats.get(&(r, c)) {
                            val.format_with(fmt)
                        } else {
                            val.to_display_string()
                        };
                        row_vals.push(display);
                    }
                    data_rows.push(row_vals);
                }

                pdf.add_table(&header_refs, &data_rows, margin_x, current_y, &widths);
            }
        }

        pdf
    }

    /// Konversi Dokumen Markdown ke Dokumen Word (.docx)
    pub fn markdown_to_docx(md: &MarkdownDocument) -> DocxDocument {
        let first_heading = md.blocks.iter().find_map(|b| {
            if let MarkdownBlock::Heading { text, .. } = b {
                Some(text.as_str())
            } else {
                None
            }
        });
        let mut doc = DocxDocument::new(first_heading.unwrap_or("Dokumen Markdown"));

        for block in &md.blocks {
            match block {
                MarkdownBlock::Heading { level, text } => {
                    doc.add_heading(text, *level);
                }
                MarkdownBlock::Paragraph(text) => {
                    doc.add_paragraph(text);
                }
                MarkdownBlock::UnorderedList(items) => {
                    for item in items {
                        doc.add_bullet(item);
                    }
                }
                MarkdownBlock::Checklist(items) => {
                    for (checked, item) in items {
                        let prefix = if *checked { "[x] " } else { "[ ] " };
                        doc.add_bullet(&format!("{}{}", prefix, item));
                    }
                }
                MarkdownBlock::CodeBlock { lang, code } => {
                    doc.add_callout(&format!("KODE ({})", lang.to_uppercase()), code, "#64748b");
                }
                MarkdownBlock::Table { headers, rows } => {
                    let mut all_rows = Vec::new();
                    all_rows.push(headers.clone());
                    all_rows.extend(rows.clone());
                    doc.add_table(all_rows, true);
                }
                MarkdownBlock::Blockquote(text) => {
                    doc.add_callout("KUTIPAN", text, "#3b82f6");
                }
                MarkdownBlock::HorizontalRule => {
                    doc.add_page_break();
                }
            }
        }

        doc
    }

    /// Konversi Dokumen Markdown langsung ke PDF
    pub fn markdown_to_pdf(md: &MarkdownDocument) -> PdfDocument {
        let docx = Self::markdown_to_docx(md);
        Self::docx_to_pdf(&docx)
    }

    /// Konversi Dokumen LaTeX Ilmiah ke Dokumen Word (.docx)
    pub fn latex_to_docx(tex: &LatexDocument) -> DocxDocument {
        let mut doc = DocxDocument::new(&tex.title);
        doc.author = tex.author.clone();

        for el in &tex.elements {
            match el {
                LatexElement::Section(title) => {
                    doc.add_heading(title, 1);
                }
                LatexElement::Subsection(title) => {
                    doc.add_heading(title, 2);
                }
                LatexElement::Paragraph(p) => {
                    doc.add_paragraph(p);
                }
                LatexElement::Equation(eq) => {
                    doc.add_callout("PERSAMAAN MATEMATIKA", eq, "#8b5cf6");
                }
                LatexElement::InlineMath(m) => {
                    doc.add_paragraph(&format!("${}$", m));
                }
                LatexElement::Table { headers, rows } => {
                    let mut all_rows = Vec::new();
                    all_rows.push(headers.clone());
                    all_rows.extend(rows.clone());
                    doc.add_table(all_rows, true);
                }
                LatexElement::Itemize(items) => {
                    for it in items {
                        doc.add_bullet(it);
                    }
                }
            }
        }

        doc
    }
}
