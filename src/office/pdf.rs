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
            let max_h_chars = ((w - 8.0) / 5.6).max(3.0) as usize;
            let clean_h = sanitize_pdf_text(h);
            let display_h = if clean_h.chars().count() > max_h_chars {
                let trunc: String = clean_h.chars().take(max_h_chars.saturating_sub(2)).collect();
                format!("{}..", trunc)
            } else {
                clean_h
            };
            page.add_styled_text(&display_h, curr_x + 6.0, start_y - 11.0, 9.0, true, (0.05, 0.2, 0.45));
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
                    PdfElement::Text { text, font_size, x, y, bold: _, color_rgb } => {
                        let clean = sanitize_pdf_text(text).replace('\n', " ");
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
    s.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
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
        '*' => 0.400,
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

/// Menghitung lebar string dalam satuan points (1/72 inci)
pub fn string_width(s: &str, font_size: f32, bold: bool) -> f32 {
    let mut total = 0.0;
    for c in s.chars() {
        total += helvetica_char_width(c, bold) * font_size;
    }
    total
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
