use crate::office::docx::{parse_markdown_runs, TextRun};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Dimensi Halaman PDF Standar (Points: 1/72 inch)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PageSize {
    A4,     // 595.28 x 841.89 points
    Letter, // 612.0 x 792.0 points
    Legal,  // 612.0 x 1008.0 points
}

impl PageSize {
    pub fn dimensions_points(&self) -> (f32, f32) {
        match self {
            PageSize::A4 => (595.28, 841.89),
            PageSize::Letter => (612.0, 792.0),
            PageSize::Legal => (612.0, 1008.0),
        }
    }
}

/// Elemen Konten Dokumen PDF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PdfElement {
    Text {
        text: String,
        font_size: f32,
        x: f32,
        y: f32,
        bold: bool,
        italic: bool,
        monospace: bool,
        color_rgb: (f32, f32, f32),
    },
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        width: f32,
        color_rgb: (f32, f32, f32),
    },
    Rectangle {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        fill: bool,
        color_rgb: (f32, f32, f32),
    },
    Polygon {
        points: Vec<(f32, f32)>,
        fill: bool,
        color_rgb: (f32, f32, f32),
    },
}

/// Anotasi Hyperlink Interaktif PDF (ISO 32000-1 Subtype /Link)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfLinkAnnotation {
    pub rect: [f32; 4], // [llx, lly, urx, ury]
    pub uri: String,
}

/// Item Bookmark / Outline Navigasi PDF (ISO 32000-1 /Outlines)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfBookmark {
    pub title: String,
    pub page_index: usize,
    pub y_pos: f32,
    pub level: u8,
}

/// Halaman Tunggal PDF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfPage {
    pub elements: Vec<PdfElement>,
    pub links: Vec<PdfLinkAnnotation>,
}

impl PdfPage {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn add_link(&mut self, llx: f32, lly: f32, urx: f32, ury: f32, uri: &str) {
        if !uri.trim().is_empty() {
            self.links.push(PdfLinkAnnotation {
                rect: [llx, lly, urx, ury],
                uri: uri.trim().to_string(),
            });
        }
    }

    pub fn add_text(&mut self, text: &str, x: f32, y: f32, font_size: f32, bold: bool) {
        self.add_formatted_text(text, x, y, font_size, bold, false, false, (0.1, 0.1, 0.1));
    }

    pub fn add_styled_text(&mut self, text: &str, x: f32, y: f32, font_size: f32, bold: bool, color_rgb: (f32, f32, f32)) {
        self.add_formatted_text(text, x, y, font_size, bold, false, false, color_rgb);
    }

    pub fn add_formatted_text(
        &mut self,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
        bold: bool,
        italic: bool,
        monospace: bool,
        color_rgb: (f32, f32, f32),
    ) {
        self.elements.push(PdfElement::Text {
            text: text.to_string(),
            font_size,
            x,
            y,
            bold,
            italic,
            monospace,
            color_rgb,
        });
    }

    /// Menambahkan teks dengan arah Kanan-ke-Kiri (RTL / Bahasa Arab)
    pub fn add_rtl_text(&mut self, text: &str, right_x: f32, y: f32, font_size: f32, bold: bool) {
        // Pada dokumen PDF, perataan kanan teks dihitung mundur dari right_x berdasarkan perkiraan lebar karakter
        let char_count = text.chars().count() as f32;
        let est_char_width = font_size * 0.45;
        let calculated_x = (right_x - (char_count * est_char_width)).max(20.0);
        self.elements.push(PdfElement::Text {
            text: text.to_string(),
            font_size,
            x: calculated_x,
            y,
            bold,
            italic: false,
            monospace: false,
            color_rgb: (0.1, 0.1, 0.1),
        });
    }

    /// Menambahkan teks dengan pembungkusan otomatis (word-wrapping) agar tidak melampaui margin kanan
    pub fn add_wrapped_text(
        &mut self,
        text: &str,
        x: f32,
        mut y: f32,
        max_width: f32,
        font_size: f32,
        line_height: f32,
        bold: bool,
        color_rgb: (f32, f32, f32),
    ) -> f32 {
        let lines = wrap_text(text, max_width, font_size, bold);
        for line in lines {
            if !line.is_empty() {
                self.add_styled_text(&line, x, y, font_size, bold, color_rgb);
            }
            y -= line_height;
        }
        y
    }

    pub fn add_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32) {
        self.elements.push(PdfElement::Line {
            x1, y1, x2, y2, width,
            color_rgb: (0.7, 0.7, 0.7),
        });
    }

    pub fn add_styled_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32, color_rgb: (f32, f32, f32)) {
        self.elements.push(PdfElement::Line {
            x1, y1, x2, y2, width, color_rgb,
        });
    }

    pub fn add_rect(&mut self, x: f32, y: f32, w: f32, h: f32, fill: bool, color_rgb: (f32, f32, f32)) {
        self.elements.push(PdfElement::Rectangle {
            x, y, w, h, fill, color_rgb,
        });
    }

    pub fn add_polygon(&mut self, points: Vec<(f32, f32)>, fill: bool, color_rgb: (f32, f32, f32)) {
        self.elements.push(PdfElement::Polygon { points, fill, color_rgb });
    }

    /// Gambar panah vektor berarah dari (x1, y1) ke (x2, y2) dengan mata panah segitiga terisi
    pub fn add_arrow(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        width: f32,
        color_rgb: (f32, f32, f32),
    ) {
        self.add_styled_line(x1, y1, x2, y2, width, color_rgb);
        let dx = x2 - x1;
        let dy = y2 - y1;
        let len = (dx * dx + dy * dy).sqrt();
        if len > 1.0 {
            let u_x = dx / len;
            let u_y = dy / len;
            let arrow_len = 6.0;
            let arrow_width = 3.5;

            let tip = (x2, y2);
            let base_x = x2 - arrow_len * u_x;
            let base_y = y2 - arrow_len * u_y;
            let left_wing = (base_x - arrow_width * u_y, base_y + arrow_width * u_x);
            let right_wing = (base_x + arrow_width * u_y, base_y - arrow_width * u_x);

            self.add_polygon(vec![tip, left_wing, right_wing], true, color_rgb);
        }
    }
}

/// Dokumen PDF Standar ISO 32000-1 (%PDF-1.7) Tingkat Industri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfDocument {
    pub title: String,
    pub author: String,
    pub organization: String,
    pub page_size: PageSize,
    pub pages: Vec<PdfPage>,
    pub enable_auto_header_footer: bool,
    pub watermark_text: Option<String>,
    pub bookmarks: Vec<PdfBookmark>,
}

impl PdfDocument {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            author: "Widya Office Suite".to_string(),
            organization: "Pemerintah Republik Indonesia / Korporasi Berdaulat".to_string(),
            page_size: PageSize::A4,
            pages: vec![PdfPage::new()],
            enable_auto_header_footer: true,
            watermark_text: None,
            bookmarks: Vec::new(),
        }
    }

    pub fn set_watermark(&mut self, text: &str) {
        self.watermark_text = Some(text.to_string());
    }

    pub fn add_bookmark(&mut self, title: &str, page_index: usize, y_pos: f32, level: u8) {
        let clean = sanitize_pdf_text(title);
        if !clean.trim().is_empty() {
            self.bookmarks.push(PdfBookmark {
                title: clean.trim().to_string(),
                page_index,
                y_pos,
                level,
            });
        }
    }

    pub fn current_page_mut(&mut self) -> &mut PdfPage {
        if self.pages.is_empty() {
            self.pages.push(PdfPage::new());
        }
        self.pages.last_mut().unwrap()
    }

    pub fn add_page(&mut self) -> &mut PdfPage {
        self.pages.push(PdfPage::new());
        self.pages.last_mut().unwrap()
    }

    /// Gambar Tabel Bergaris Resmi Tingkat Penerbitan dengan Pembungkusan Teks Multi-baris & Header Berulang
    pub fn add_table(
        &mut self,
        headers: &[&str],
        rows: &[Vec<String>],
        margin_x: f32,
        start_y: f32,
        col_widths: &[f32],
    ) -> f32 {
        self.add_table_aligned(headers, rows, margin_x, start_y, col_widths, &[])
    }

    /// Gambar Tabel Bergaris Resmi Tingkat Penerbitan dengan Perataan Kolom (Left, Center, Right) & Dukungan Hyperlink
    pub fn add_table_aligned(
        &mut self,
        headers: &[&str],
        rows: &[Vec<String>],
        margin_x: f32,
        mut start_y: f32,
        col_widths: &[f32],
        alignments: &[crate::office::markdown::ColumnAlignment],
    ) -> f32 {
        let total_width: f32 = col_widths.iter().sum();
        let (_, page_height) = self.page_size.dimensions_points();

        // Helper lokal untuk menggambar Header Bar pada halaman tertentu
        fn draw_header(
            page: &mut PdfPage,
            headers: &[&str],
            col_widths: &[f32],
            margin_x: f32,
            top_y: f32,
            total_width: f32,
            alignments: &[crate::office::markdown::ColumnAlignment],
        ) -> f32 {
            let header_cells: Vec<Vec<Vec<StyledFragment>>> = headers
                .iter()
                .enumerate()
                .map(|(i, &h)| {
                    let w = col_widths.get(i).copied().unwrap_or(80.0);
                    let cell_w = (w - 10.0).max(15.0);
                    let mut runs = parse_markdown_runs(h);
                    if runs.is_empty() {
                        runs.push(TextRun {
                            text: h.to_string(),
                            style: crate::office::docx::TextStyle::bold(),
                        });
                    } else {
                        for r in &mut runs {
                            r.style.bold = true;
                        }
                    }
                    wrap_runs_to_styled_lines(&runs, cell_w, 9.0, (0.05, 0.20, 0.45))
                })
                .collect();

            let max_h_lines = header_cells.iter().map(|l| l.len()).max().unwrap_or(1).max(1);
            let h_line_height = 11.5;
            let header_h = (max_h_lines as f32 * h_line_height) + 10.0;

            // Background & Outer Border
            page.add_rect(margin_x, top_y - header_h, total_width, header_h, true, (0.92, 0.95, 0.98));
            page.add_rect(margin_x, top_y - header_h, total_width, header_h, false, (0.22, 0.42, 0.72));

            // Garis pembagi kolom vertikal header & teks
            let mut curr_x = margin_x;
            for (i, lines) in header_cells.iter().enumerate() {
                let w = col_widths.get(i).copied().unwrap_or(80.0);
                if i > 0 {
                    page.add_styled_line(curr_x, top_y - header_h, curr_x, top_y, 0.75, (0.22, 0.42, 0.72));
                }
                let align = alignments.get(i).copied().unwrap_or(crate::office::markdown::ColumnAlignment::Left);
                for (l_idx, line) in lines.iter().enumerate() {
                    let text_y = top_y - 10.5 - (l_idx as f32 * h_line_height);
                    let line_w: f32 = line.iter().map(|f| styled_string_width(&f.text, f.font_size, f.bold, f.monospace)).sum();
                    let padding = 5.0;
                    let avail_w = (w - (padding * 2.0)).max(0.0);
                    let start_offset = match align {
                        crate::office::markdown::ColumnAlignment::Left => padding,
                        crate::office::markdown::ColumnAlignment::Center => padding + ((avail_w - line_w).max(0.0) / 2.0),
                        crate::office::markdown::ColumnAlignment::Right => padding + (avail_w - line_w).max(0.0),
                    };
                    let mut frag_x = curr_x + start_offset;
                    for frag in line {
                        if !frag.text.is_empty() {
                            page.add_formatted_text(
                                &frag.text,
                                frag_x,
                                text_y,
                                frag.font_size,
                                frag.bold,
                                frag.italic,
                                frag.monospace,
                                frag.color_rgb,
                            );
                            let frag_w = styled_string_width(&frag.text, frag.font_size, frag.bold, frag.monospace);
                            if let Some(ref url) = frag.link_url {
                                page.add_link(frag_x, text_y - 2.0, frag_x + frag_w, text_y + frag.font_size + 1.0, url);
                            }
                            frag_x += frag_w;
                        }
                    }
                }
                curr_x += w;
            }

            top_y - header_h
        }

        // 1. Header Bar Pertama
        start_y = draw_header(self.current_page_mut(), headers, col_widths, margin_x, start_y, total_width, alignments);

        // 2. Data Rows
        for (row_idx, row) in rows.iter().enumerate() {
            // Hitung teks terbungkus dengan parsing Markdown styled runs untuk tiap sel
            let cell_lines: Vec<Vec<Vec<StyledFragment>>> = row
                .iter()
                .enumerate()
                .map(|(c_idx, cell)| {
                    let w = col_widths.get(c_idx).copied().unwrap_or(80.0);
                    let cell_w = (w - 10.0).max(15.0);
                    let runs = parse_markdown_runs(cell);
                    wrap_runs_to_styled_lines(&runs, cell_w, 8.5, (0.15, 0.20, 0.25))
                })
                .collect();

            let max_lines = cell_lines.iter().map(|l| l.len()).max().unwrap_or(1).max(1);
            let line_height = 11.0;
            let row_height = (max_lines as f32 * line_height) + 8.0;

            // Jika baris ini melampaui batas bawah margin halaman, buat halaman baru & ulang header
            if start_y - row_height < 75.0 {
                self.add_page();
                start_y = page_height - 70.0;
                start_y = draw_header(self.current_page_mut(), headers, col_widths, margin_x, start_y, total_width, alignments);
            }

            let page = self.current_page_mut();
            let bg_color = if row_idx % 2 == 1 { (0.97, 0.98, 0.99) } else { (1.0, 1.0, 1.0) };

            // Background & Outer Border
            page.add_rect(margin_x, start_y - row_height, total_width, row_height, true, bg_color);
            page.add_rect(margin_x, start_y - row_height, total_width, row_height, false, (0.80, 0.85, 0.90));

            // Garis pembagi kolom vertikal & teks sel
            let mut curr_x = margin_x;
            for (c_idx, lines) in cell_lines.iter().enumerate() {
                let w = col_widths.get(c_idx).copied().unwrap_or(80.0);
                if c_idx > 0 {
                    page.add_styled_line(curr_x, start_y - row_height, curr_x, start_y, 0.5, (0.80, 0.85, 0.90));
                }
                let align = alignments.get(c_idx).copied().unwrap_or(crate::office::markdown::ColumnAlignment::Left);
                for (l_idx, line) in lines.iter().enumerate() {
                    let text_y = start_y - 9.5 - (l_idx as f32 * line_height);
                    let line_w: f32 = line.iter().map(|f| styled_string_width(&f.text, f.font_size, f.bold, f.monospace)).sum();
                    let padding = 5.0;
                    let avail_w = (w - (padding * 2.0)).max(0.0);
                    let start_offset = match align {
                        crate::office::markdown::ColumnAlignment::Left => padding,
                        crate::office::markdown::ColumnAlignment::Center => padding + ((avail_w - line_w).max(0.0) / 2.0),
                        crate::office::markdown::ColumnAlignment::Right => padding + (avail_w - line_w).max(0.0),
                    };
                    let mut frag_x = curr_x + start_offset;
                    for frag in line {
                        if !frag.text.is_empty() {
                            page.add_formatted_text(
                                &frag.text,
                                frag_x,
                                text_y,
                                frag.font_size,
                                frag.bold,
                                frag.italic,
                                frag.monospace,
                                frag.color_rgb,
                            );
                            let frag_w = styled_string_width(&frag.text, frag.font_size, frag.bold, frag.monospace);
                            if let Some(ref url) = frag.link_url {
                                page.add_link(frag_x, text_y - 2.0, frag_x + frag_w, text_y + frag.font_size + 1.0, url);
                            }
                            if frag.underline {
                                page.add_styled_line(
                                    frag_x,
                                    text_y - 1.5,
                                    frag_x + frag_w,
                                    text_y - 1.5,
                                    0.6,
                                    frag.color_rgb,
                                );
                            }
                            frag_x += frag_w;
                        }
                    }
                }
                curr_x += w;
            }

            start_y -= row_height;
        }

        start_y
    }

    /// Hasilkan Berkas Biner PDF Asli (%PDF-1.7 ISO 32000-1)
    pub fn generate_pdf_bytes(&self) -> Vec<u8> {
        let (width, height) = self.page_size.dimensions_points();
        let mut out = Vec::new();
        let mut xref_offsets = Vec::new();

        // 1. Header
        out.extend_from_slice(b"%PDF-1.7\n%\xE2\xE3\xCF\xD3\n");

        // Objek 1: Catalog
        xref_offsets.push(out.len());
        let total_pages = self.pages.len();
        let outlines_obj_id = 8 + total_pages * 2;
        let catalog_obj = if !self.bookmarks.is_empty() {
            format!("1 0 obj\n<< /Type /Catalog /Pages 2 0 R /Outlines {} 0 R >>\nendobj\n", outlines_obj_id)
        } else {
            "1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n".to_string()
        };
        out.extend_from_slice(catalog_obj.as_bytes());

        // Hitung ID halaman: Objek 1 (Catalog), Objek 2 (Pages), Objek 3..7 (Fonts)
        // Jadi halaman dimulai dari objek 8: page_obj_id = 8 + i * 2
        let kids_refs: Vec<String> = (0..total_pages).map(|i| format!("{} 0 R", 8 + i * 2)).collect();

        // Objek 2: Pages Root
        xref_offsets.push(out.len());
        let pages_obj = format!(
            "2 0 obj\n<< /Type /Pages /Kids [ {} ] /Count {} /MediaBox [ 0 0 {} {} ] >>\nendobj\n",
            kids_refs.join(" "), total_pages, width, height
        );
        out.extend_from_slice(pages_obj.as_bytes());

        // Objek 3: Font Helvetica (/F1 - Regular)
        xref_offsets.push(out.len());
        out.extend_from_slice(
            b"3 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>\nendobj\n",
        );

        // Objek 4: Font Helvetica-Bold (/F2 - Tebal)
        xref_offsets.push(out.len());
        out.extend_from_slice(
            b"4 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding /WinAnsiEncoding >>\nendobj\n",
        );

        // Objek 5: Font Helvetica-Oblique (/F3 - Miring)
        xref_offsets.push(out.len());
        out.extend_from_slice(
            b"5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique /Encoding /WinAnsiEncoding >>\nendobj\n",
        );

        // Objek 6: Font Courier (/F4 - Monospace / Kode)
        xref_offsets.push(out.len());
        out.extend_from_slice(
            b"6 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Courier /Encoding /WinAnsiEncoding >>\nendobj\n",
        );

        // Objek 7: Font Helvetica-BoldOblique (/F5 - Tebal & Miring)
        xref_offsets.push(out.len());
        out.extend_from_slice(
            b"7 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-BoldOblique /Encoding /WinAnsiEncoding >>\nendobj\n",
        );

        // Objek Halaman & Stream Konten
        for (i, page) in self.pages.iter().enumerate() {
            let page_obj_id = 8 + i * 2;
            let stream_obj_id = page_obj_id + 1;

            let mut stream_data = String::new();

            // Watermark (jika ada)
            if let Some(ref wm) = self.watermark_text {
                let safe_wm = escape_pdf_string(&sanitize_pdf_text(wm));
                stream_data.push_str(&format!(
                    "q 0.9 0.9 0.9 rg BT /F1 36 Tf 0.707 0.707 -0.707 0.707 150 250 Tm ({}) Tj ET Q\n",
                    safe_wm
                ));
            }

            // Header Otomatis (jika diaktifkan)
            if self.enable_auto_header_footer {
                let clean_title = sanitize_pdf_text(&self.title).replace('\n', " ");
                let safe_title = if clean_title.chars().count() > 80 {
                    let trunc: String = clean_title.chars().take(77).collect();
                    format!("{}...", trunc)
                } else {
                    clean_title
                };
                let safe_title_esc = escape_pdf_string(&safe_title);
                stream_data.push_str(&format!(
                    "q 0.4 0.5 0.6 rg BT /F1 8 Tf 1 0 0 1 50 {} Tm ({}) Tj ET 0.5 w 0.8 0.8 0.8 RG 50 {} m {} {} l S Q\n",
                    height - 35.0, safe_title_esc, height - 42.0, width - 50.0, height - 42.0
                ));

                // Footer Otomatis: "Halaman X dari Y" + Stempel Resmi
                let page_info = format!("Halaman {} dari {}", i + 1, total_pages);
                stream_data.push_str(&format!(
                    "q 0.5 0.5 0.5 rg BT /F1 8 Tf 1 0 0 1 50 35 Tm (Dokumen Resmi WidyaOffice - Kedaulatan Data Lokal) Tj ET 1 0 0 1 {} 35 Tm ({}) Tj ET 0.5 w 0.8 0.8 0.8 RG 50 45 m {} 45 l S Q\n",
                    width - 150.0, page_info, width - 50.0
                ));
            }

            // Elemen halaman
            for el in &page.elements {
                match el {
                    PdfElement::Text { text, font_size, x, y, bold, italic, monospace, color_rgb } => {
                        let clean = sanitize_pdf_text(text).replace('\n', " ");
                        let safe_text = escape_pdf_string(&clean);
                        let font_id = if *monospace {
                            "/F4"
                        } else if *bold && *italic {
                            "/F5"
                        } else if *bold {
                            "/F2"
                        } else if *italic {
                            "/F3"
                        } else {
                            "/F1"
                        };
                        stream_data.push_str(&format!(
                            "BT {} {} Tf {} {} {} rg 1 0 0 1 {} {} Tm ({}) Tj ET\n",
                            font_id, font_size, color_rgb.0, color_rgb.1, color_rgb.2, x, y, safe_text
                        ));
                    }
                    PdfElement::Line { x1, y1, x2, y2, width, color_rgb } => {
                        stream_data.push_str(&format!(
                            "{} {} {} RG {} w {} {} m {} {} l S\n",
                            color_rgb.0, color_rgb.1, color_rgb.2, width, x1, y1, x2, y2
                        ));
                    }
                    PdfElement::Rectangle { x, y, w, h, fill, color_rgb } => {
                        if *fill {
                            stream_data.push_str(&format!(
                                "{} {} {} rg {} {} {} {} re f\n",
                                color_rgb.0, color_rgb.1, color_rgb.2, x, y, w, h
                            ));
                        } else {
                            stream_data.push_str(&format!(
                                "{} {} {} RG 0.75 w {} {} {} {} re S\n",
                                color_rgb.0, color_rgb.1, color_rgb.2, x, y, w, h
                            ));
                        }
                    }
                    PdfElement::Polygon { points, fill, color_rgb } => {
                        if !points.is_empty() {
                            let mut poly_str = String::new();
                            if *fill {
                                poly_str.push_str(&format!("{} {} {} rg ", color_rgb.0, color_rgb.1, color_rgb.2));
                            } else {
                                poly_str.push_str(&format!("{} {} {} RG 0.75 w ", color_rgb.0, color_rgb.1, color_rgb.2));
                            }
                            poly_str.push_str(&format!("{:.2} {:.2} m ", points[0].0, points[0].1));
                            for pt in &points[1..] {
                                poly_str.push_str(&format!("{:.2} {:.2} l ", pt.0, pt.1));
                            }
                            if *fill {
                                poly_str.push_str("f\n");
                            } else {
                                poly_str.push_str("s\n");
                            }
                            stream_data.push_str(&poly_str);
                        }
                    }
                }
            }

            // Objek Page
            xref_offsets.push(out.len());
            let mut annots_str = String::new();
            if !page.links.is_empty() {
                annots_str.push_str(" /Annots [ ");
                for link in &page.links {
                    let safe_uri = escape_pdf_string(&link.uri);
                    annots_str.push_str(&format!(
                        "<< /Type /Annot /Subtype /Link /Rect [ {:.2} {:.2} {:.2} {:.2} ] /Border [ 0 0 0 ] /A << /S /URI /URI ({}) >> >> ",
                        link.rect[0], link.rect[1], link.rect[2], link.rect[3], safe_uri
                    ));
                }
                annots_str.push_str("] ");
            }

            let page_def = format!(
                "{} 0 obj\n<< /Type /Page /Parent 2 0 R /Resources << /Font << /F1 3 0 R /F2 4 0 R /F3 5 0 R /F4 6 0 R /F5 7 0 R >> >>{} /Contents {} 0 R >>\nendobj\n",
                page_obj_id, annots_str, stream_obj_id
            );
            out.extend_from_slice(page_def.as_bytes());

            // Objek Stream
            xref_offsets.push(out.len());
            let stream_obj = format!(
                "{} 0 obj\n<< /Length {} >>\nstream\n{}endstream\nendobj\n",
                stream_obj_id, stream_data.len(), stream_data
            );
            out.extend_from_slice(stream_obj.as_bytes());
        }

        // Objek Outlines (Daftar Isi / PDF Bookmarks) jika ada
        if !self.bookmarks.is_empty() {
            let count = self.bookmarks.len();
            let first_item_id = outlines_obj_id + 1;
            let last_item_id = outlines_obj_id + count;

            xref_offsets.push(out.len());
            let outlines_obj = format!(
                "{} 0 obj\n<< /Type /Outlines /First {} 0 R /Last {} 0 R /Count {} >>\nendobj\n",
                outlines_obj_id, first_item_id, last_item_id, count
            );
            out.extend_from_slice(outlines_obj.as_bytes());

            for (idx, b) in self.bookmarks.iter().enumerate() {
                let item_id = outlines_obj_id + 1 + idx;
                let target_page_idx = b.page_index.min(total_pages.saturating_sub(1));
                let target_page_obj_id = 8 + target_page_idx * 2;
                let safe_title = escape_pdf_string(&sanitize_pdf_text(&b.title));

                let prev_entry = if idx > 0 {
                    format!(" /Prev {} 0 R", item_id - 1)
                } else {
                    String::new()
                };

                let next_entry = if idx + 1 < count {
                    format!(" /Next {} 0 R", item_id + 1)
                } else {
                    String::new()
                };

                xref_offsets.push(out.len());
                let item_obj = format!(
                    "{} 0 obj\n<< /Title ({}) /Parent {} 0 R{}{} /Dest [ {} 0 R /XYZ 0 {:.2} null ] >>\nendobj\n",
                    item_id, safe_title, outlines_obj_id, prev_entry, next_entry, target_page_obj_id, b.y_pos
                );
                out.extend_from_slice(item_obj.as_bytes());
            }
        }

        // XREF Table
        let xref_start = out.len();
        let total_objects = xref_offsets.len() + 1;
        out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", total_objects).as_bytes());
        for offset in xref_offsets {
            out.extend_from_slice(format!("{:010} 00000 n \n", offset).as_bytes());
        }

        // Trailer
        let now = chrono::Utc::now().format("%Y%m%d%H%M%SZ").to_string();
        let trailer = format!(
            "trailer\n<< /Size {} /Root 1 0 R /Info << /Title ({}) /Author ({}) /Producer (Widya PDF Engine v2.5.0 Enterprise) /CreationDate (D:{}) >> >>\nstartxref\n{}\n%%EOF\n",
            total_objects, escape_pdf_string(&sanitize_pdf_text(&self.title)), escape_pdf_string(&sanitize_pdf_text(&self.author)), now, xref_start
        );
        out.extend_from_slice(trailer.as_bytes());

        out
    }

    /// Simpan Dokumen Langsung ke Berkas Fisik `.pdf`
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let bytes = self.generate_pdf_bytes();
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}

/// Sanitasi teks Unicode agar kompatibel 100% dengan standard PDF Type 1 WinAnsi/ASCII Font
pub fn sanitize_pdf_text(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '—' | '–' => result.push_str(" - "),
            '•' => result.push('•'),
            '“' | '”' => result.push('"'),
            '‘' | '’' => result.push('\''),
            '🔒' => result.push_str("[TTE] "),
            '✅' => result.push_str("[OK] "),
            '🛡' => {
                if chars.peek() == Some(&'\u{fe0f}') { chars.next(); }
                result.push_str("[SEC] ");
            }
            '⚡' => result.push_str("[DEV] "),
            '🔍' => result.push_str("[QA] "),
            '🏷' => {
                if chars.peek() == Some(&'\u{fe0f}') { chars.next(); }
                result.push_str("[TAG] ");
            }
            '🚀' => result.push_str("[SPRINT] "),
            '🏢' => result.push_str("[CORP] "),
            '💡' => result.push_str("[INFO] "),
            '\u{26a0}' => {
                if chars.peek() == Some(&'\u{fe0f}') { chars.next(); }
                result.push_str("[WARN] ");
            }
            '✓' => result.push_str("[V] "),
            '…' => result.push_str("..."),
            '→' => result.push_str("->"),
            '←' => result.push_str("<-"),
            '↔' => result.push_str("<->"),
            '⇒' => result.push_str("=>"),
            '≤' => result.push_str("<="),
            '≥' => result.push_str(">="),
            '≠' => result.push_str("!="),
            '×' => result.push_str("x"),
            '÷' => result.push_str("/"),
            '±' => result.push_str("+/-"),
            'µ' => result.push_str("u"),
            '👑' => result.push_str("[MASTER] "),
            '📦' => result.push_str("[PKG] "),
            '📋' => result.push_str("[INFO] "),
            '📌' => result.push_str("[PIN] "),
            '✨' => result.push_str("[*] "),
            '🎉' => result.push_str("[*] "),
            '🌐' => result.push_str("[WEB] "),
            '💻' => result.push_str("[PC] "),
            '📱' => result.push_str("[APP] "),
            '🖥' => { if chars.peek() == Some(&'\u{fe0f}') { chars.next(); } result.push_str("[MCR] "); }
            '⚖' => { if chars.peek() == Some(&'\u{fe0f}') { chars.next(); } result.push_str("[VAR] "); }
            '📡' => result.push_str("[RADIO] "),
            '🎥' => result.push_str("[CAM] "),
            '⚙' => { if chars.peek() == Some(&'\u{fe0f}') { chars.next(); } result.push_str("[SYS] "); }
            '🧠' => result.push_str("[AI] "),
            '📄' => result.push_str("[DOC] "),
            '📜' => result.push_str("[SCRIPT] "),
            '📁' => result.push_str("[DIR] "),
            '🎯' => result.push_str("[TARGET] "),
            '🧪' => result.push_str("[TEST] "),
            '❌' => result.push_str("[FAIL] "),
            '🇮' => {
                if chars.peek() == Some(&'🇩') {
                    chars.next();
                    result.push_str("[ID] ");
                }
            }
            '\r' => result.push(' '),
            '\t' => result.push_str("    "),
            '\n' => result.push('\n'),
            c if c.is_ascii() => result.push(c),
            c if (c as u32) >= 0x20 && (c as u32) <= 0xFF => result.push(c),
            _ => result.push(' '),
        }
    }
    result
}

pub fn escape_pdf_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            '•' => out.push_str("\\225"), // WinAnsi Typographic Bullet (•)
            _ => out.push(c),
        }
    }
    out
}

/// Estimasi lebar karakter pada font Helvetica (PDF Type 1) per unit font_size
pub fn helvetica_char_width(c: char, bold: bool) -> f32 {
    let base_width: f32 = match c {
        ' ' => 0.285,
        '!' | '.' | ',' | ':' | ';' | '/' | '\\' | '[' | ']' | 'I' | '|' => 0.285,
        '\'' | '`' => 0.230,
        'i' | 'j' | 'l' => 0.230,
        'f' | 't' | 'r' => 0.345,
        '(' | ')' | '{' | '}' | '-' => 0.345,
        '"' => 0.365,
        '*' | '•' => 0.400,
        'c' | 'k' | 's' | 'v' | 'x' | 'y' | 'z' | 'J' => 0.515,
        'a' | 'b' | 'd' | 'e' | 'g' | 'h' | 'n' | 'o' | 'p' | 'q' | 'u' => 0.575,
        '0'..='9' | '$' | '#' | '?' | '_' => 0.575,
        '+' | '<' | '=' | '>' | '~' | 'L' => 0.600,
        'F' | 'T' | 'Z' => 0.630,
        'A' | 'B' | 'E' | 'K' | 'P' | 'S' | 'V' | 'X' | 'Y' => 0.685,
        '&' => 0.685,
        'w' => 0.745,
        'C' | 'D' | 'H' | 'N' | 'R' | 'U' => 0.745,
        'G' | 'O' | 'Q' => 0.800,
        'm' | 'M' => 0.860,
        '%' => 0.915,
        'W' => 0.970,
        '@' => 1.045,
        _ => 0.575,
    };
    if bold {
        base_width * 1.08
    } else {
        base_width
    }
}

/// Estimasi lebar karakter terformat (Helvetica, Helvetica-Bold, Helvetica-Oblique, Courier)
pub fn styled_char_width(c: char, bold: bool, monospace: bool) -> f32 {
    if monospace {
        0.600 // Standar Courier Type 1 (advance width konstan 600/1000)
    } else {
        helvetica_char_width(c, bold)
    }
}

/// Menghitung lebar string terformat dalam satuan points (1/72 inci)
pub fn styled_string_width(s: &str, font_size: f32, bold: bool, monospace: bool) -> f32 {
    let mut total = 0.0;
    for c in s.chars() {
        total += styled_char_width(c, bold, monospace) * font_size;
    }
    total
}

/// Menghitung lebar string dalam satuan points (1/72 inci)
pub fn string_width(s: &str, font_size: f32, bold: bool) -> f32 {
    styled_string_width(s, font_size, bold, false)
}

/// Fragmen teks terformat untuk perataan & pembungkusan multi-run pada baris PDF
#[derive(Debug, Clone)]
pub struct StyledFragment {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub monospace: bool,
    pub underline: bool,
    pub color_rgb: (f32, f32, f32),
    pub font_size: f32,
    pub link_url: Option<String>,
}

/// Konversi warna hex CSS (#rrggbb atau #rgb) ke triplet f32 RGB (0.0 .. 1.0)
pub fn parse_hex_color(hex: &str) -> Option<(f32, f32, f32)> {
    let clean = hex.trim_start_matches('#');
    if clean.len() == 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).ok()? as f32 / 255.0;
        let g = u8::from_str_radix(&clean[2..4], 16).ok()? as f32 / 255.0;
        let b = u8::from_str_radix(&clean[4..6], 16).ok()? as f32 / 255.0;
        Some((r, g, b))
    } else if clean.len() == 3 {
        let r = u8::from_str_radix(&clean[0..1], 16).ok()? as f32 / 15.0;
        let g = u8::from_str_radix(&clean[1..2], 16).ok()? as f32 / 15.0;
        let b = u8::from_str_radix(&clean[2..3], 16).ok()? as f32 / 15.0;
        Some((r, g, b))
    } else {
        None
    }
}

/// Membungkus kumpulan TextRun menjadi baris-baris terformat (StyledFragment) tanpa melewati `max_width`
pub fn wrap_runs_to_styled_lines(
    runs: &[TextRun],
    max_width: f32,
    base_font_size: f32,
    default_color: (f32, f32, f32),
) -> Vec<Vec<StyledFragment>> {
    struct WordToken {
        text: String,
        has_leading_space: bool,
        bold: bool,
        italic: bool,
        monospace: bool,
        underline: bool,
        color_rgb: (f32, f32, f32),
        font_size: f32,
        link_url: Option<String>,
    }

    let mut tokens: Vec<WordToken> = Vec::new();
    let mut pending_space = false;

    for r in runs {
        let clean = sanitize_pdf_text(&r.text);
        if clean.is_empty() {
            continue;
        }

        let bold = r.style.bold;
        let italic = r.style.italic;
        let monospace = r.style.is_monospace
            || r.style.font_family.contains("Courier")
            || r.style.font_family.contains("Consolas");
        let underline = r.style.underline;
        let link_url = r.style.link_url.clone();
        let font_size = if r.style.font_size_pt > 0.0 && r.style.font_size_pt != 10.0 && r.style.font_size_pt != 11.0 {
            r.style.font_size_pt
        } else {
            base_font_size
        };

        let color_rgb = if let Some(ref hex) = r.style.color_hex {
            parse_hex_color(hex).unwrap_or(default_color)
        } else {
            default_color
        };

        let words: Vec<&str> = clean.split_whitespace().collect();
        if words.is_empty() {
            if clean.chars().any(|c| c.is_whitespace()) {
                pending_space = true;
            }
            continue;
        }

        let starts_with_space = clean.chars().next().map(|c| c.is_whitespace()).unwrap_or(false);
        let ends_with_space = clean.chars().last().map(|c| c.is_whitespace()).unwrap_or(false);

        for (w_idx, word) in words.into_iter().enumerate() {
            let is_first = w_idx == 0;
            let lead_space = if is_first {
                pending_space || starts_with_space
            } else {
                true
            };
            pending_space = false;

            tokens.push(WordToken {
                text: word.to_string(),
                has_leading_space: lead_space && !tokens.is_empty(),
                bold,
                italic,
                monospace,
                underline,
                color_rgb,
                font_size,
                link_url: link_url.clone(),
            });
        }

        if ends_with_space {
            pending_space = true;
        }
    }

    if tokens.is_empty() {
        return Vec::new();
    }

    let mut lines: Vec<Vec<StyledFragment>> = Vec::new();
    let mut cur_line: Vec<StyledFragment> = Vec::new();
    let mut cur_w = 0.0;

    for tok in tokens {
        let space_w = if tok.has_leading_space {
            styled_char_width(' ', tok.bold, tok.monospace) * tok.font_size
        } else {
            0.0
        };
        let word_w = styled_string_width(&tok.text, tok.font_size, tok.bold, tok.monospace);

        // Jika satu kata melebihi seluruh max_width, potong karakter per karakter
        if word_w > max_width {
            if !cur_line.is_empty() {
                lines.push(std::mem::take(&mut cur_line));
                cur_w = 0.0;
            }
            let mut cur_chunk = String::new();
            let mut chunk_w = 0.0;
            for ch in tok.text.chars() {
                let ch_w = styled_char_width(ch, tok.bold, tok.monospace) * tok.font_size;
                if chunk_w + ch_w > max_width && !cur_chunk.is_empty() {
                    lines.push(vec![StyledFragment {
                        text: std::mem::take(&mut cur_chunk),
                        bold: tok.bold,
                        italic: tok.italic,
                        monospace: tok.monospace,
                        underline: tok.underline,
                        color_rgb: tok.color_rgb,
                        font_size: tok.font_size,
                        link_url: tok.link_url.clone(),
                    }]);
                    chunk_w = 0.0;
                }
                cur_chunk.push(ch);
                chunk_w += ch_w;
            }
            if !cur_chunk.is_empty() {
                cur_line.push(StyledFragment {
                    text: cur_chunk,
                    bold: tok.bold,
                    italic: tok.italic,
                    monospace: tok.monospace,
                    underline: tok.underline,
                    color_rgb: tok.color_rgb,
                    font_size: tok.font_size,
                    link_url: tok.link_url.clone(),
                });
                cur_w = chunk_w;
            }
            continue;
        }

        if cur_line.is_empty() {
            cur_line.push(StyledFragment {
                text: tok.text,
                bold: tok.bold,
                italic: tok.italic,
                monospace: tok.monospace,
                underline: tok.underline,
                color_rgb: tok.color_rgb,
                font_size: tok.font_size,
                link_url: tok.link_url.clone(),
            });
            cur_w = word_w;
        } else if cur_w + space_w + word_w <= max_width {
            let last = cur_line.last_mut().unwrap();
            if last.bold == tok.bold
                && last.italic == tok.italic
                && last.monospace == tok.monospace
                && last.underline == tok.underline
                && last.color_rgb == tok.color_rgb
                && last.font_size == tok.font_size
                && last.link_url == tok.link_url
            {
                if tok.has_leading_space {
                    last.text.push(' ');
                }
                last.text.push_str(&tok.text);
            } else {
                let mut frag_text = String::new();
                if tok.has_leading_space {
                    frag_text.push(' ');
                }
                frag_text.push_str(&tok.text);
                cur_line.push(StyledFragment {
                    text: frag_text,
                    bold: tok.bold,
                    italic: tok.italic,
                    monospace: tok.monospace,
                    underline: tok.underline,
                    color_rgb: tok.color_rgb,
                    font_size: tok.font_size,
                    link_url: tok.link_url.clone(),
                });
            }
            cur_w += space_w + word_w;
        } else {
            lines.push(std::mem::take(&mut cur_line));
            cur_line.push(StyledFragment {
                text: tok.text,
                bold: tok.bold,
                italic: tok.italic,
                monospace: tok.monospace,
                underline: tok.underline,
                color_rgb: tok.color_rgb,
                font_size: tok.font_size,
                link_url: tok.link_url,
            });
            cur_w = word_w;
        }
    }

    if !cur_line.is_empty() {
        lines.push(cur_line);
    }

    lines
}

/// Memecah teks menjadi baris-baris yang tidak melampaui `max_width` (Word Wrapping)
pub fn wrap_text(text: &str, max_width: f32, font_size: f32, bold: bool) -> Vec<String> {
    let mut lines = Vec::new();
    let sanitized = sanitize_pdf_text(text);

    for raw_line in sanitized.split('\n') {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            lines.push(String::new());
            continue;
        }

        let words: Vec<&str> = trimmed.split_whitespace().collect();
        if words.is_empty() {
            lines.push(String::new());
            continue;
        }

        let space_w = helvetica_char_width(' ', bold) * font_size;
        let mut cur_line = String::new();
        let mut cur_w: f32 = 0.0;

        for word in words {
            let word_w = string_width(word, font_size, bold);

            // Jika satu kata melebihi seluruh lebar baris, potong secara paksa per karakter
            if word_w > max_width {
                if !cur_line.is_empty() {
                    lines.push(cur_line);
                    cur_line = String::new();
                    cur_w = 0.0;
                }
                for ch in word.chars() {
                    let ch_w = helvetica_char_width(ch, bold) * font_size;
                    if cur_w + ch_w > max_width && !cur_line.is_empty() {
                        lines.push(cur_line);
                        cur_line = String::new();
                        cur_w = 0.0;
                    }
                    cur_line.push(ch);
                    cur_w += ch_w;
                }
                continue;
            }

            if cur_line.is_empty() {
                cur_line.push_str(word);
                cur_w = word_w;
            } else if cur_w + space_w + word_w <= max_width {
                cur_line.push(' ');
                cur_line.push_str(word);
                cur_w += space_w + word_w;
            } else {
                lines.push(cur_line);
                cur_line = word.to_string();
                cur_w = word_w;
            }
        }

        if !cur_line.is_empty() {
            lines.push(cur_line);
        }
    }
    lines
}
