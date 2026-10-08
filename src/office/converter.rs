use crate::office::docx::{DocxDocument, DocxElement};
use crate::office::latex::{LatexDocument, LatexElement};
use crate::office::markdown::{MarkdownBlock, MarkdownDocument};
use crate::office::pdf::{wrap_text, PdfDocument};
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
        let content_width = 480.0;

        for el in &doc.elements {
            if current_y < 90.0 {
                pdf.add_page();
                current_y = page_height - 70.0;
            }

            match el {
                DocxElement::Heading { level, text } => {
                    let (font_size, line_height) = match level {
                        1 => (16.0, 22.0),
                        2 => (13.5, 18.0),
                        3 => (11.5, 16.0),
                        _ => (10.5, 14.0),
                    };

                    let wrapped = wrap_text(text, content_width, font_size, true);
                    current_y -= 8.0;

                    for line in wrapped {
                        if line.is_empty() {
                            continue;
                        }
                        if current_y < 80.0 {
                            pdf.add_page();
                            current_y = page_height - 70.0;
                        }
                        current_y -= line_height;
                        pdf.current_page_mut().add_styled_text(
                            &line,
                            margin_x,
                            current_y,
                            font_size,
                            true,
                            (0.05, 0.15, 0.35),
                        );
                    }

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
                    current_y -= 6.0;
                }
                DocxElement::Paragraph { runs, is_rtl, .. } => {
                    let mut line_text = String::new();
                    for r in runs {
                        line_text.push_str(&r.text);
                    }
                    if line_text.trim().is_empty() {
                        current_y -= 6.0;
                        continue;
                    }

                    let font_size = 10.0;
                    let line_height = 14.0;
                    let wrapped = wrap_text(&line_text, content_width, font_size, false);

                    for line in wrapped {
                        if line.is_empty() {
                            current_y -= 4.0;
                            continue;
                        }
                        if current_y < 80.0 {
                            pdf.add_page();
                            current_y = page_height - 70.0;
                        }
                        current_y -= line_height;
                        if *is_rtl {
                            pdf.current_page_mut().add_rtl_text(&line, margin_x + content_width, current_y, font_size, false);
                        } else {
                            pdf.current_page_mut().add_styled_text(
                                &line,
                                margin_x,
                                current_y,
                                font_size,
                                false,
                                (0.15, 0.2, 0.25),
                            );
                        }
                    }
                    current_y -= 4.0;
                }
                DocxElement::BulletItem { text, indent_level } => {
                    let font_size = 9.5;
                    let line_height = 13.5;
                    let indent_px = 16.0 + (*indent_level as f32 * 12.0);
                    let bullet_content_w = (content_width - indent_px).max(100.0);
                    let wrapped = wrap_text(text, bullet_content_w, font_size, false);

                    for (idx, line) in wrapped.iter().enumerate() {
                        if line.is_empty() {
                            continue;
                        }
                        if current_y < 80.0 {
                            pdf.add_page();
                            current_y = page_height - 70.0;
                        }
                        current_y -= line_height;
                        if idx == 0 {
                            pdf.current_page_mut().add_styled_text(
                                "*",
                                margin_x + indent_px - 10.0,
                                current_y,
                                font_size,
                                true,
                                (0.15, 0.35, 0.7),
                            );
                        }
                        pdf.current_page_mut().add_styled_text(
                            line,
                            margin_x + indent_px,
                            current_y,
                            font_size,
                            false,
                            (0.2, 0.25, 0.3),
                        );
                    }
                    current_y -= 2.0;
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
                DocxElement::Callout { title, text, border_color } => {
                    let font_size = 8.5;
                    let line_height = 12.0;
                    let inner_w = content_width - 24.0;
                    let lines = wrap_text(text, inner_w, font_size, false);

                    let (border_rgb, bg_rgb) = if border_color.contains("3b82f6") || border_color.contains("blue") {
                        ((0.15, 0.45, 0.85), (0.94, 0.97, 1.0))
                    } else if border_color.contains("8b5cf6") || border_color.contains("purple") {
                        ((0.45, 0.25, 0.85), (0.97, 0.95, 1.0))
                    } else {
                        ((0.35, 0.40, 0.48), (0.96, 0.97, 0.98))
                    };

                    current_y -= 8.0;

                    // Header Callout (Judul)
                    if current_y < 85.0 {
                        pdf.add_page();
                        current_y = page_height - 70.0;
                    }
                    let header_h = 18.0;
                    let page = pdf.current_page_mut();
                    page.add_rect(margin_x, current_y - header_h, content_width, header_h, true, bg_rgb);
                    page.add_styled_line(margin_x, current_y - header_h, margin_x, current_y, 3.5, border_rgb);
                    page.add_styled_line(margin_x, current_y, margin_x + content_width, current_y, 0.5, (0.85, 0.88, 0.9));
                    page.add_styled_line(margin_x + content_width, current_y - header_h, margin_x + content_width, current_y, 0.5, (0.85, 0.88, 0.9));
                    page.add_styled_text(title, margin_x + 10.0, current_y - 12.0, 9.0, true, (0.1, 0.25, 0.45));
                    current_y -= header_h;

                    for line in &lines {
                        if current_y < 80.0 {
                            pdf.add_page();
                            current_y = page_height - 70.0;
                        }
                        let page = pdf.current_page_mut();
                        page.add_rect(margin_x, current_y - line_height, content_width, line_height, true, bg_rgb);
                        page.add_styled_line(margin_x, current_y - line_height, margin_x, current_y, 3.5, border_rgb);
                        page.add_styled_line(margin_x + content_width, current_y - line_height, margin_x + content_width, current_y, 0.5, (0.85, 0.88, 0.9));
                        if !line.is_empty() {
                            page.add_styled_text(line, margin_x + 10.0, current_y - 9.0, font_size, false, (0.15, 0.2, 0.25));
                        }
                        current_y -= line_height;
                    }

                    // Garis penutup bawah kotak
                    let page = pdf.current_page_mut();
                    page.add_styled_line(margin_x, current_y, margin_x + content_width, current_y, 0.5, (0.85, 0.88, 0.9));
                    current_y -= 8.0;
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
