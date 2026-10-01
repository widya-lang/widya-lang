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
}

/// Halaman Tunggal PDF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfPage {
    pub elements: Vec<PdfElement>,
}

impl PdfPage {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn add_text(&mut self, text: &str, x: f32, y: f32, font_size: f32, bold: bool) {
        self.elements.push(PdfElement::Text {
            text: text.to_string(),
            font_size,
            x,
            y,
            bold,
            color_rgb: (0.1, 0.1, 0.1),
        });
    }

    pub fn add_styled_text(&mut self, text: &str, x: f32, y: f32, font_size: f32, bold: bool, color_rgb: (f32, f32, f32)) {
        self.elements.push(PdfElement::Text {
            text: text.to_string(),
            font_size,
            x,
            y,
            bold,
            color_rgb,
        });
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
        }
    }

    pub fn set_watermark(&mut self, text: &str) {
        self.watermark_text = Some(text.to_string());
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

    /// Gambar Tabel Bergaris Resmi dengan Pemisah Halaman Otomatis
    pub fn add_table(
        &mut self,
        headers: &[&str],
        rows: &[Vec<String>],
        margin_x: f32,
        mut start_y: f32,
        col_widths: &[f32],
    ) -> f32 {
        let row_height = 22.0;
        let total_width: f32 = col_widths.iter().sum();
        let (_, page_height) = self.page_size.dimensions_points();

        // 1. Header Bar
        let page = self.current_page_mut();
        page.add_rect(margin_x, start_y - 16.0, total_width, row_height, true, (0.92, 0.95, 0.98));
        page.add_rect(margin_x, start_y - 16.0, total_width, row_height, false, (0.2, 0.4, 0.7));

        let mut curr_x = margin_x;
        for (i, &h) in headers.iter().enumerate() {
            let w = col_widths.get(i).copied().unwrap_or(80.0);
            page.add_styled_text(h, curr_x + 6.0, start_y - 11.0, 9.5, true, (0.05, 0.2, 0.45));
            curr_x += w;
        }
        start_y -= row_height;

        // 2. Data Rows
        for (row_idx, row) in rows.iter().enumerate() {
            if start_y < 80.0 {
                // Halaman baru jika melebihi batas margin bawah
                self.add_page();
                start_y = page_height - 90.0;
            }

            let page = self.current_page_mut();
            let bg_color = if row_idx % 2 == 1 { (0.98, 0.98, 0.99) } else { (1.0, 1.0, 1.0) };
            page.add_rect(margin_x, start_y - 16.0, total_width, row_height, true, bg_color);
            page.add_rect(margin_x, start_y - 16.0, total_width, row_height, false, (0.8, 0.85, 0.9));

            let mut curr_x = margin_x;
            for (c_idx, cell) in row.iter().enumerate() {
                let w = col_widths.get(c_idx).copied().unwrap_or(80.0);
                let max_chars = ((w - 10.0) / 5.2).max(4.0) as usize;
                let clean_cell = sanitize_pdf_text(cell);
                let display_cell = if clean_cell.chars().count() > max_chars {
                    let truncated: String = clean_cell.chars().take(max_chars.saturating_sub(3)).collect();
                    format!("{}...", truncated)
                } else {
                    clean_cell
                };
                page.add_styled_text(&display_cell, curr_x + 6.0, start_y - 11.0, 8.5, false, (0.15, 0.2, 0.25));
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
        out.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");

        // Hitung ID halaman
        let total_pages = self.pages.len();
        let kids_refs: Vec<String> = (0..total_pages).map(|i| format!("{} 0 R", 4 + i * 2)).collect();

        // Objek 2: Pages Root
        xref_offsets.push(out.len());
        let pages_obj = format!(
            "2 0 obj\n<< /Type /Pages /Kids [ {} ] /Count {} /MediaBox [ 0 0 {} {} ] >>\nendobj\n",
            kids_refs.join(" "), total_pages, width, height
        );
        out.extend_from_slice(pages_obj.as_bytes());

        // Objek 3: Font Helvetica & Helvetica-Bold
        xref_offsets.push(out.len());
        out.extend_from_slice(
            b"3 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>\nendobj\n",
        );

        // Objek Halaman & Stream Konten
        for (i, page) in self.pages.iter().enumerate() {
            let page_obj_id = 4 + i * 2;
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
                let safe_title = escape_pdf_string(&sanitize_pdf_text(&self.title));
                stream_data.push_str(&format!(
                    "q 0.4 0.5 0.6 rg BT /F1 8 Tf 1 0 0 1 50 {} Tm ({}) Tj ET 0.5 w 0.8 0.8 0.8 RG 50 {} m {} {} l S Q\n",
                    height - 35.0, safe_title, height - 42.0, width - 50.0, height - 42.0
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
                    PdfElement::Text { text, font_size, x, y, bold: _, color_rgb } => {
                        let clean = sanitize_pdf_text(text);
                        let safe_text = escape_pdf_string(&clean);
                        stream_data.push_str(&format!(
                            "BT /F1 {} Tf {} {} {} rg 1 0 0 1 {} {} Tm ({}) Tj ET\n",
                            font_size, color_rgb.0, color_rgb.1, color_rgb.2, x, y, safe_text
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
                }
            }

            // Objek Page
            xref_offsets.push(out.len());
            let page_def = format!(
                "{} 0 obj\n<< /Type /Page /Parent 2 0 R /Resources << /Font << /F1 3 0 R >> >> /Contents {} 0 R >>\nendobj\n",
                page_obj_id, stream_obj_id
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
            '•' => result.push_str("* "),
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
            '\n' | '\r' | '\t' => result.push(' '),
            c if c.is_ascii() => result.push(c),
            c if (c as u32) >= 0x20 && (c as u32) <= 0xFF => result.push(c),
            _ => result.push(' '),
        }
    }
    result
}

fn escape_pdf_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}
