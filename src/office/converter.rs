use crate::office::diagram::WidyaDiagramEngine;
use crate::office::docx::{parse_markdown_runs, DocxDocument, DocxElement, TextStyle, TextRun};
use crate::office::epub::{markdown_to_epub_doc, EpubDocument};
use crate::office::latex::{LatexDocument, LatexElement};
use crate::office::markdown::{MarkdownBlock, MarkdownDocument};
use crate::office::pdf::{styled_string_width, wrap_runs_to_styled_lines, wrap_text, PdfDocument, StyledFragment};
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
                    let clean_text = clean_markdown_inline(text);
                    let page_idx = pdf.pages.len().saturating_sub(1);
                    pdf.add_bookmark(&clean_text, page_idx, current_y, *level);

                    let (font_size, line_height) = match level {
                        1 => (16.0, 22.0),
                        2 => (13.5, 18.0),
                        3 => (11.5, 16.0),
                        _ => (10.5, 14.0),
                    };

                    let wrapped = wrap_text(&clean_text, content_width, font_size, true);
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
                    let total_text: String = runs.iter().map(|r| r.text.as_str()).collect();
                    if total_text.trim().is_empty() {
                        current_y -= 6.0;
                        continue;
                    }

                    let font_size = 10.0;
                    let line_height = 14.0;
                    let default_color = (0.15, 0.2, 0.25);

                    if *is_rtl {
                        let cleaned = clean_markdown_inline(&total_text);
                        let wrapped = wrap_text(&cleaned, content_width, font_size, false);
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
                            pdf.current_page_mut().add_rtl_text(&line, margin_x + content_width, current_y, font_size, false);
                        }
                    } else {
                        let lines = wrap_runs_to_styled_lines(runs, content_width, font_size, default_color);
                        for line in lines {
                            if line.is_empty() {
                                current_y -= 4.0;
                                continue;
                            }
                            if current_y < 80.0 {
                                pdf.add_page();
                                current_y = page_height - 70.0;
                            }
                            current_y -= line_height;
                            let mut curr_x = margin_x;
                            for frag in &line {
                                if !frag.text.is_empty() {
                                    pdf.current_page_mut().add_formatted_text(
                                        &frag.text,
                                        curr_x,
                                        current_y,
                                        frag.font_size,
                                        frag.bold,
                                        frag.italic,
                                        frag.monospace,
                                        frag.color_rgb,
                                    );
                                    let frag_w = styled_string_width(&frag.text, frag.font_size, frag.bold, frag.monospace);
                                    if let Some(ref url) = frag.link_url {
                                        pdf.current_page_mut().add_link(
                                            curr_x,
                                            current_y - 2.0,
                                            curr_x + frag_w,
                                            current_y + frag.font_size + 1.0,
                                            url,
                                        );
                                    }
                                    if frag.underline {
                                        pdf.current_page_mut().add_styled_line(
                                            curr_x,
                                            current_y - 1.5,
                                            curr_x + frag_w,
                                            current_y - 1.5,
                                            0.6,
                                            frag.color_rgb,
                                        );
                                    }
                                    curr_x += frag_w;
                                }
                            }
                        }
                    }
                    current_y -= 4.0;
                }
                DocxElement::BulletItem { runs, indent_level } => {
                    let font_size = 9.5;
                    let line_height = 13.5;
                    let indent_px = 16.0 + (*indent_level as f32 * 12.0);
                    let bullet_content_w = (content_width - indent_px).max(100.0);
                    let default_color = (0.2, 0.25, 0.3);
                    let lines = wrap_runs_to_styled_lines(runs, bullet_content_w, font_size, default_color);

                    for (idx, line) in lines.iter().enumerate() {
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
                                "•",
                                margin_x + indent_px - 10.0,
                                current_y,
                                font_size,
                                true,
                                (0.15, 0.35, 0.7),
                            );
                        }
                        let mut curr_x = margin_x + indent_px;
                        for frag in line {
                            if !frag.text.is_empty() {
                                pdf.current_page_mut().add_formatted_text(
                                    &frag.text,
                                    curr_x,
                                    current_y,
                                    frag.font_size,
                                    frag.bold,
                                    frag.italic,
                                    frag.monospace,
                                    frag.color_rgb,
                                );
                                let frag_w = styled_string_width(&frag.text, frag.font_size, frag.bold, frag.monospace);
                                if let Some(ref url) = frag.link_url {
                                    pdf.current_page_mut().add_link(
                                        curr_x,
                                        current_y - 2.0,
                                        curr_x + frag_w,
                                        current_y + frag.font_size + 1.0,
                                        url,
                                    );
                                }
                                if frag.underline {
                                    pdf.current_page_mut().add_styled_line(
                                        curr_x,
                                        current_y - 1.5,
                                        curr_x + frag_w,
                                        current_y - 1.5,
                                        0.6,
                                        frag.color_rgb,
                                    );
                                }
                                curr_x += frag_w;
                            }
                        }
                    }
                    current_y -= 2.0;
                }
                DocxElement::NumberedItem { number, runs, indent_level } => {
                    let font_size = 9.5;
                    let line_height = 13.5;
                    let indent_px = 18.0 + (*indent_level as f32 * 12.0);
                    let num_content_w = (content_width - indent_px).max(100.0);
                    let default_color = (0.2, 0.25, 0.3);
                    let lines = wrap_runs_to_styled_lines(runs, num_content_w, font_size, default_color);

                    for (idx, line) in lines.iter().enumerate() {
                        if line.is_empty() {
                            continue;
                        }
                        if current_y < 80.0 {
                            pdf.add_page();
                            current_y = page_height - 70.0;
                        }
                        current_y -= line_height;
                        if idx == 0 {
                            let num_label = format!("{}.", number);
                            pdf.current_page_mut().add_styled_text(
                                &num_label,
                                margin_x + indent_px - 14.0,
                                current_y,
                                font_size,
                                true,
                                (0.15, 0.35, 0.7),
                            );
                        }
                        let mut curr_x = margin_x + indent_px;
                        for frag in line {
                            if !frag.text.is_empty() {
                                pdf.current_page_mut().add_formatted_text(
                                    &frag.text,
                                    curr_x,
                                    current_y,
                                    frag.font_size,
                                    frag.bold,
                                    frag.italic,
                                    frag.monospace,
                                    frag.color_rgb,
                                );
                                let frag_w = styled_string_width(&frag.text, frag.font_size, frag.bold, frag.monospace);
                                if let Some(ref url) = frag.link_url {
                                    pdf.current_page_mut().add_link(
                                        curr_x,
                                        current_y - 2.0,
                                        curr_x + frag_w,
                                        current_y + frag.font_size + 1.0,
                                        url,
                                    );
                                }
                                if frag.underline {
                                    pdf.current_page_mut().add_styled_line(
                                        curr_x,
                                        current_y - 1.5,
                                        curr_x + frag_w,
                                        current_y - 1.5,
                                        0.6,
                                        frag.color_rgb,
                                    );
                                }
                                curr_x += frag_w;
                            }
                        }
                    }
                    current_y -= 2.0;
                }
                DocxElement::Table { rows, has_header, alignments } => {
                    if !rows.is_empty() {
                        let col_count = rows[0].len().max(1);
                        let header_slice: Vec<String> = if *has_header {
                            rows[0].clone()
                        } else {
                            (1..=col_count).map(|i| format!("Kolom {}", i)).collect()
                        };
                        let header_refs: Vec<&str> = header_slice.iter().map(|s| s.as_str()).collect();

                        let data_rows_raw = if *has_header && rows.len() > 1 {
                            &rows[1..]
                        } else {
                            &rows[..]
                        };

                        let widths = calculate_table_col_widths(&header_refs, data_rows_raw, content_width);

                        // Beri jarak atas sebelum tabel agar tidak menimpa teks paragraf sebelumnya
                        current_y -= 14.0;
                        current_y = pdf.add_table_aligned(&header_refs, data_rows_raw, margin_x, current_y, &widths, alignments);
                        current_y -= 14.0;
                    }
                }
                DocxElement::Callout { title, text, border_color } => {
                    let clean_title = clean_markdown_inline(title);
                    let is_code = title.starts_with("KODE");
                    let font_size = 8.5;
                    let line_height = 12.0;
                    let inner_w = content_width - 24.0;
                    let default_color = (0.15, 0.2, 0.25);

                    let lines: Vec<Vec<StyledFragment>> = if is_code {
                        let mut all_code_lines = Vec::new();
                        for raw_line in text.lines() {
                            let highlighted = highlight_code_line(raw_line, font_size);
                            let wrapped_sublines = wrap_code_fragments(highlighted, inner_w);
                            all_code_lines.extend(wrapped_sublines);
                        }
                        if all_code_lines.is_empty() {
                            all_code_lines.push(Vec::new());
                        }
                        all_code_lines
                    } else {
                        let runs = parse_markdown_runs(text);
                        wrap_runs_to_styled_lines(&runs, inner_w, font_size, default_color)
                    };

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
                    page.add_styled_text(&clean_title, margin_x + 10.0, current_y - 12.0, 9.0, true, (0.1, 0.25, 0.45));
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
                        let mut curr_x = margin_x + 10.0;
                        for frag in line {
                            if !frag.text.is_empty() {
                                page.add_formatted_text(
                                    &frag.text,
                                    curr_x,
                                    current_y - 9.0,
                                    frag.font_size,
                                    frag.bold,
                                    frag.italic,
                                    frag.monospace,
                                    frag.color_rgb,
                                );
                                let frag_w = styled_string_width(&frag.text, frag.font_size, frag.bold, frag.monospace);
                                if let Some(ref url) = frag.link_url {
                                    page.add_link(
                                        curr_x,
                                        current_y - 11.0,
                                        curr_x + frag_w,
                                        current_y - 9.0 + frag.font_size,
                                        url,
                                    );
                                }
                                curr_x += frag_w;
                            }
                        }
                        current_y -= line_height;
                    }

                    // Garis penutup bawah kotak
                    let page = pdf.current_page_mut();
                    page.add_styled_line(margin_x, current_y, margin_x + content_width, current_y, 0.5, (0.85, 0.88, 0.9));
                    current_y -= 8.0;
                }
                DocxElement::Divider => {
                    current_y -= 8.0;
                    if current_y < 80.0 {
                        pdf.add_page();
                        current_y = page_height - 70.0;
                    }
                    pdf.current_page_mut().add_styled_line(
                        margin_x,
                        current_y,
                        margin_x + content_width,
                        current_y,
                        0.8,
                        (0.80, 0.85, 0.90),
                    );
                    current_y -= 12.0;
                }
                DocxElement::PageBreak => {
                    pdf.add_page();
                    current_y = page_height - 70.0;
                }
                DocxElement::Diagram { raw_mermaid } => {
                    current_y -= 8.0;
                    current_y = WidyaDiagramEngine::render_to_pdf(&mut pdf, raw_mermaid, margin_x, current_y, content_width);
                    current_y -= 12.0;
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

                let widths = calculate_table_col_widths(&header_refs, &data_rows, content_width);
                current_y -= 10.0;
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
        let raw_title = first_heading.unwrap_or("Dokumen Markdown");
        let mut doc = DocxDocument::new(&clean_markdown_inline(raw_title));

        for block in &md.blocks {
            match block {
                MarkdownBlock::Heading { level, text } => {
                    doc.add_heading(&clean_markdown_inline(text), *level);
                }
                MarkdownBlock::Paragraph(text) => {
                    let runs = parse_markdown_runs(text);
                    doc.add_paragraph_runs(runs);
                }
                MarkdownBlock::UnorderedList(items) => {
                    for item in items {
                        let runs = parse_markdown_runs(item);
                        doc.add_bullet_runs(runs, 0);
                    }
                }
                MarkdownBlock::OrderedList(items) => {
                    for (num, item) in items {
                        let runs = parse_markdown_runs(item);
                        doc.add_numbered_runs(*num, runs, 0);
                    }
                }
                MarkdownBlock::Checklist(items) => {
                    for (checked, item) in items {
                        let prefix = if *checked { "[x] " } else { "[ ] " };
                        let mut runs = vec![TextRun {
                            text: prefix.to_string(),
                            style: TextStyle::bold(),
                        }];
                        runs.extend(parse_markdown_runs(item));
                        doc.add_bullet_runs(runs, 0);
                    }
                }
                MarkdownBlock::CodeBlock { lang, code } => {
                    if lang.trim().to_lowercase() == "mermaid" {
                        doc.add_diagram(code);
                    } else {
                        doc.add_callout(&format!("KODE ({})", lang.to_uppercase()), code, "#64748b");
                    }
                }
                MarkdownBlock::Table { headers, alignments, rows } => {
                    let mut all_rows = Vec::new();
                    all_rows.push(headers.clone());
                    all_rows.extend(rows.clone());
                    doc.add_table_aligned(all_rows, true, alignments.clone());
                }
                MarkdownBlock::Blockquote(text) => {
                    doc.add_callout("KUTIPAN", text, "#3b82f6");
                }
                MarkdownBlock::HorizontalRule => {
                    doc.add_divider();
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

    /// Konversi Dokumen Markdown ke Buku Digital EPUB 3.2 Mandiri
    pub fn markdown_to_epub(
        md: &MarkdownDocument,
        title: Option<&str>,
        author: Option<&str>,
    ) -> EpubDocument {
        markdown_to_epub_doc(md, title, author)
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
                        let runs = parse_markdown_runs(it);
                        doc.add_bullet_runs(runs, 0);
                    }
                }
            }
        }

        doc
    }
}

/// Menghitung pembagian lebar kolom tabel secara proporsional cerdas berdasarkan kepadatan isi
pub fn calculate_table_col_widths(
    headers: &[&str],
    rows: &[Vec<String>],
    total_width: f32,
) -> Vec<f32> {
    let col_count = headers.len().max(rows.iter().map(|r| r.len()).max().unwrap_or(0)).max(1);
    if col_count == 1 {
        return vec![total_width];
    }

    // Hitung perkiraan panjang karakter maksimum tiap kolom (menggunakan panjang teks bersih tanpa format markup)
    let mut col_max_lens: Vec<usize> = Vec::with_capacity(col_count);
    for i in 0..col_count {
        let h_len = headers.get(i).map(|h| clean_markdown_inline(h).chars().count()).unwrap_or(4);
        let max_data_len = rows
            .iter()
            .filter_map(|r| r.get(i))
            .map(|cell| clean_markdown_inline(cell).chars().count())
            .max()
            .unwrap_or(0);
        col_max_lens.push(h_len.max(max_data_len));
    }

    // Tentukan bobot dan batas minimum per kolom
    let mut weights: Vec<f32> = Vec::with_capacity(col_count);
    let mut min_widths: Vec<f32> = Vec::with_capacity(col_count);

    for &len in &col_max_lens {
        if len <= 3 {
            // Kolom sangat sempit seperti "No", "ID", "#"
            weights.push(22.0);
            min_widths.push(25.0);
        } else if len <= 8 {
            // Kolom sempit seperti "Versi", "1.0.0", "Status"
            weights.push(42.0);
            min_widths.push(40.0);
        } else if len <= 16 {
            // Kolom sedang seperti "Nama Paket", "Perintah"
            weights.push(75.0);
            min_widths.push(65.0);
        } else if len <= 30 {
            // Kolom panjang sedang
            weights.push(110.0);
            min_widths.push(80.0);
        } else {
            // Kolom deskripsi lebar
            let extra = ((len.saturating_sub(30)) as f32 * 1.5).min(120.0);
            weights.push(140.0 + extra);
            min_widths.push(100.0);
        }
    }

    let total_weight: f32 = weights.iter().sum();
    let mut widths: Vec<f32> = weights
        .iter()
        .map(|w| (w / total_weight) * total_width)
        .collect();

    // Pastikan setiap kolom memenuhi batas minimum
    for i in 0..col_count {
        if widths[i] < min_widths[i] {
            widths[i] = min_widths[i];
        }
    }

    // Normalisasi ulang agar total tepat sama dengan total_width
    let current_total: f32 = widths.iter().sum();
    let diff = total_width - current_total;
    if diff.abs() > 0.1 {
        let max_idx = widths
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(col_count - 1);
        widths[max_idx] += diff;
    }

    widths
}

/// Membersihkan sintaks inline Markdown (bold, italic, code, link, gambar, strikethrough, tag HTML sederhana)
/// agar teks bersih berstandar penerbitan profesional tanpa karakter mentah yang bocor.
pub fn clean_markdown_inline(s: &str) -> String {
    let mut result = s.to_string();

    // 1. Gambar Markdown: ![alt](url) -> [Gambar: alt]
    while let Some(img_start) = result.find("![") {
        if let Some(bracket_end) = result[img_start..].find(']') {
            let abs_b_end = img_start + bracket_end;
            if result[abs_b_end..].starts_with("](") {
                if let Some(paren_end) = result[abs_b_end..].find(')') {
                    let abs_p_end = abs_b_end + paren_end;
                    let alt = &result[img_start + 2..abs_b_end];
                    let replacement = if alt.trim().is_empty() {
                        String::new()
                    } else {
                        format!("[Gambar: {}]", alt.trim())
                    };
                    result.replace_range(img_start..=abs_p_end, &replacement);
                    continue;
                }
            }
        }
        break;
    }

    // 2. Tautan Markdown: [text](url) -> text
    while let Some(link_start) = result.find('[') {
        if link_start > 0 && result.as_bytes()[link_start - 1] == b'!' {
            break;
        }
        if let Some(bracket_end) = result[link_start..].find(']') {
            let abs_b_end = link_start + bracket_end;
            if result[abs_b_end..].starts_with("](") {
                if let Some(paren_end) = result[abs_b_end..].find(')') {
                    let abs_p_end = abs_b_end + paren_end;
                    let inner_text = result[link_start + 1..abs_b_end].to_string();
                    result.replace_range(link_start..=abs_p_end, &inner_text);
                    continue;
                }
            }
        }
        break;
    }

    // 3. Autolink: <http://...> atau <https://...>
    while let Some(start) = result.find("<http") {
        if let Some(end) = result[start..].find('>') {
            let url = result[start + 1..start + end].to_string();
            result.replace_range(start..=start + end, &url);
        } else {
            break;
        }
    }

    // 4. Bold & Italic: ***text***
    while let Some(start) = result.find("***") {
        if let Some(end) = result[start + 3..].find("***") {
            let abs_end = start + 3 + end;
            let inner = result[start + 3..abs_end].to_string();
            result.replace_range(start..abs_end + 3, &inner);
        } else {
            break;
        }
    }

    // 5. Bold: **text**
    while let Some(start) = result.find("**") {
        if let Some(end) = result[start + 2..].find("**") {
            let abs_end = start + 2 + end;
            let inner = result[start + 2..abs_end].to_string();
            result.replace_range(start..abs_end + 2, &inner);
        } else {
            break;
        }
    }

    // 6. Bold underscore: __text__
    while let Some(start) = result.find("__") {
        if let Some(end) = result[start + 2..].find("__") {
            let abs_end = start + 2 + end;
            let inner = result[start + 2..abs_end].to_string();
            result.replace_range(start..abs_end + 2, &inner);
        } else {
            break;
        }
    }

    // 7. Inline code: `code`
    while let Some(start) = result.find('`') {
        if let Some(end) = result[start + 1..].find('`') {
            let abs_end = start + 1 + end;
            let inner = result[start + 1..abs_end].to_string();
            result.replace_range(start..abs_end + 1, &inner);
        } else {
            break;
        }
    }

    // 8. Strikethrough: ~~text~~
    while let Some(start) = result.find("~~") {
        if let Some(end) = result[start + 2..].find("~~") {
            let abs_end = start + 2 + end;
            let inner = result[start + 2..abs_end].to_string();
            result.replace_range(start..abs_end + 2, &inner);
        } else {
            break;
        }
    }

    // 9. Italic: *text* (hindari bullet point '* ' atau ekspresi matematika)
    let mut cleaned_italic = String::with_capacity(result.len());
    let chars: Vec<char> = result.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '*' && i + 1 < chars.len() && chars[i + 1] != ' ' && chars[i + 1] != '*' {
            let mut found_end = None;
            for j in (i + 2)..chars.len() {
                if chars[j] == '*' && chars[j - 1] != ' ' {
                    found_end = Some(j);
                    break;
                }
            }
            if let Some(end_idx) = found_end {
                for c in &chars[i + 1..end_idx] {
                    cleaned_italic.push(*c);
                }
                i = end_idx + 1;
                continue;
            }
        }
        cleaned_italic.push(chars[i]);
        i += 1;
    }
    result = cleaned_italic;

    // 10. Tag HTML format sederhana
    let html_tags = [
        "<b>", "</b>", "<i>", "</i>", "<em>", "</em>",
        "<strong>", "</strong>", "<code>", "</code>",
        "<kbd>", "</kbd>", "<mark>", "</mark>",
    ];
    for tag in html_tags {
        result = result.replace(tag, "");
    }
    result = result.replace("<br>", " ")
        .replace("<br/>", " ")
        .replace("<br />", " ");

    result
}

/// Tokenizer Pewarnaan Sintaks Kode Mandiri (Zero Dependency) untuk Blok Kode PDF
pub fn highlight_code_line(line: &str, font_size: f32) -> Vec<StyledFragment> {
    let mut frags = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    let keywords = [
        "fn", "fungsi", "let", "mut", "pub", "struct", "enum", "impl", "trait", "match",
        "pilih", "if", "jika", "else", "lain", "return", "kembalikan", "def", "class",
        "import", "from", "as", "const", "type", "use", "for", "untuk", "while", "selama",
        "loop", "break", "continue", "async", "await", "select", "where", "insert", "into",
        "values", "update", "set", "delete", "join", "create", "table", "drop", "alter",
        "view", "index", "primary", "key", "foreign", "not", "null", "true", "benar",
        "false", "salah", "and", "or", "in", "is", "echo", "printf", "println", "cargo",
        "widya", "git", "bash", "sudo", "docker", "podman",
    ];

    let types = [
        "i8", "i16", "i32", "i64", "i128", "isize",
        "u8", "u16", "u32", "u64", "u128", "usize",
        "f32", "f64", "bool", "char", "str", "String",
        "Vec", "Option", "Result", "Some", "None", "Ok", "Err",
        "Teks", "Angka", "Daftar", "Pilihan", "Hasil",
    ];

    while i < len {
        // 1. Whitespace
        if chars[i].is_whitespace() {
            let start = i;
            while i < len && chars[i].is_whitespace() {
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            frags.push(StyledFragment {
                text,
                bold: false,
                italic: false,
                monospace: true,
                underline: false,
                color_rgb: (0.15, 0.20, 0.25),
                font_size,
                link_url: None,
            });
            continue;
        }

        // 2. Line Comment (// atau # atau --)
        if (chars[i] == '/' && i + 1 < len && chars[i + 1] == '/')
            || chars[i] == '#'
            || (chars[i] == '-' && i + 1 < len && chars[i + 1] == '-')
        {
            let text: String = chars[i..len].iter().collect();
            frags.push(StyledFragment {
                text,
                bold: false,
                italic: true,
                monospace: true,
                underline: false,
                color_rgb: (0.45, 0.52, 0.60), // Abu-abu miring komentar
                font_size,
                link_url: None,
            });
            break;
        }

        // 3. String Literal ("..." atau '...')
        if chars[i] == '"' || chars[i] == '\'' {
            let quote = chars[i];
            let start = i;
            i += 1;
            while i < len {
                if chars[i] == '\\' && i + 1 < len {
                    i += 2;
                } else if chars[i] == quote {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            let text: String = chars[start..i].iter().collect();
            frags.push(StyledFragment {
                text,
                bold: false,
                italic: false,
                monospace: true,
                underline: false,
                color_rgb: (0.12, 0.55, 0.25), // Hijau daun string
                font_size,
                link_url: None,
            });
            continue;
        }

        // 4. Number Literal
        if chars[i].is_ascii_digit() || (chars[i] == '-' && i + 1 < len && chars[i + 1].is_ascii_digit()) {
            let start = i;
            if chars[i] == '-' {
                i += 1;
            }
            if i + 1 < len && chars[i] == '0' && (chars[i + 1] == 'x' || chars[i + 1] == 'X') {
                i += 2;
                while i < len && (chars[i].is_ascii_hexdigit() || chars[i] == '_') {
                    i += 1;
                }
            } else {
                while i < len && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == '_') {
                    i += 1;
                }
            }
            let text: String = chars[start..i].iter().collect();
            frags.push(StyledFragment {
                text,
                bold: false,
                italic: false,
                monospace: true,
                underline: false,
                color_rgb: (0.78, 0.38, 0.08), // Oranye angka
                font_size,
                link_url: None,
            });
            continue;
        }

        // 5. Identifiers / Keywords
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let (color_rgb, bold) = if keywords.contains(&word.to_lowercase().as_str()) {
                ((0.10, 0.35, 0.85), true) // Biru tua tebal kata kunci
            } else if types.contains(&word.as_str()) {
                ((0.05, 0.55, 0.55), true) // Teal tebal tipe data
            } else {
                ((0.15, 0.20, 0.25), false) // Default identifier
            };

            frags.push(StyledFragment {
                text: word,
                bold,
                italic: false,
                monospace: true,
                underline: false,
                color_rgb,
                font_size,
                link_url: None,
            });
            continue;
        }

        // 6. Symbols / Operators
        let start = i;
        i += 1;
        let text: String = chars[start..i].iter().collect();
        frags.push(StyledFragment {
            text,
            bold: false,
            italic: false,
            monospace: true,
            underline: false,
            color_rgb: (0.35, 0.40, 0.50), // Biru baja operator
            font_size,
            link_url: None,
        });
    }

    frags
}

/// Membungkus daftar fragmen kode berformat warna agar rapi sesuai lebar batas kontainer
pub fn wrap_code_fragments(frags: Vec<StyledFragment>, max_w: f32) -> Vec<Vec<StyledFragment>> {
    let mut lines: Vec<Vec<StyledFragment>> = Vec::new();
    let mut current_line: Vec<StyledFragment> = Vec::new();
    let mut current_w = 0.0;

    for frag in frags {
        let frag_w = styled_string_width(&frag.text, frag.font_size, frag.bold, frag.monospace);
        if current_w + frag_w > max_w && !current_line.is_empty() {
            lines.push(current_line);
            current_line = Vec::new();
            current_w = 0.0;
        }
        current_w += frag_w;
        current_line.push(frag);
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(Vec::new());
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_markdown_inline() {
        assert_eq!(
            clean_markdown_inline("**Pilar I: Kompiler & Linker**"),
            "Pilar I: Kompiler & Linker"
        );
        assert_eq!(
            clean_markdown_inline("[`src/compiler/pure_emitter.rs`](file:///path/to/pure_emitter.rs)"),
            "src/compiler/pure_emitter.rs"
        );
        assert_eq!(
            clean_markdown_inline("*third-party crates / packages*"),
            "third-party crates / packages"
        );
        assert_eq!(
            clean_markdown_inline("`widya wpm init <nama>`"),
            "widya wpm init <nama>"
        );
        assert_eq!(
            clean_markdown_inline("pure_emitter.rs dan pure_crypto.rs"),
            "pure_emitter.rs dan pure_crypto.rs"
        );
        assert_eq!(
            clean_markdown_inline("Teks dengan **tebal** dan `kode` serta [tautan](https://widya-lang.org)"),
            "Teks dengan tebal dan kode serta tautan"
        );
    }

    #[test]
    fn test_highlight_code_line() {
        let line = "pub fn proses_data(nilai: i32) -> String { // komentar penting";
        let frags = highlight_code_line(line, 9.0);

        // 'pub' dan 'fn' harus bold dan berwarna biru keyword
        assert!(frags.iter().any(|f| f.text == "pub" && f.bold && f.color_rgb == (0.10, 0.35, 0.85)));
        assert!(frags.iter().any(|f| f.text == "fn" && f.bold && f.color_rgb == (0.10, 0.35, 0.85)));
        // 'i32' dan 'String' harus tipe
        assert!(frags.iter().any(|f| f.text == "i32" && f.bold && f.color_rgb == (0.05, 0.55, 0.55)));
        // Komentar harus italic abu-abu
        assert!(frags.iter().any(|f| f.text.contains("komentar penting") && f.italic));
    }
}
