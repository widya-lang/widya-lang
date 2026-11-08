//! Widya-Lang Document DSL & Universal Document Automation Engine
//! Sovereign WordCraft-grade Document Model for WidyaWriter, DOCX, and PDF.

use crate::office::converter::DocumentConverter;
use crate::office::docx::{DocxDocument, DocxElement, TextStyle, TextRun};
use crate::office::markdown::{ColumnAlignment, MarkdownBlock, MarkdownDocument};
use crate::office::pdf::PdfDocument;
use serde::{Deserialize, Serialize};

/// Satuan Margin Lembar Dokumen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMargins {
    pub top_cm: f32,
    pub bottom_cm: f32,
    pub left_cm: f32,
    pub right_cm: f32,
}

impl Default for DocumentMargins {
    fn default() -> Self {
        Self {
            top_cm: 2.5,
            bottom_cm: 2.5,
            left_cm: 3.0,
            right_cm: 2.5,
        }
    }
}

/// Fragmen Teks Berformat di dalam Paragraf (Run)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslRun {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub code: bool,
    pub math: bool,
    pub footnote_ref: Option<usize>,
}

impl DslRun {
    pub fn text(t: &str) -> Self {
        Self {
            text: t.to_string(),
            bold: false,
            italic: false,
            underline: false,
            code: false,
            math: false,
            footnote_ref: None,
        }
    }

    pub fn bold(t: &str) -> Self {
        Self {
            text: t.to_string(),
            bold: true,
            italic: false,
            underline: false,
            code: false,
            math: false,
            footnote_ref: None,
        }
    }

    pub fn italic(t: &str) -> Self {
        Self {
            text: t.to_string(),
            bold: false,
            italic: true,
            underline: false,
            code: false,
            math: false,
            footnote_ref: None,
        }
    }

    pub fn code(t: &str) -> Self {
        Self {
            text: t.to_string(),
            bold: false,
            italic: false,
            underline: false,
            code: true,
            math: false,
            footnote_ref: None,
        }
    }

    pub fn footnote(id: usize) -> Self {
        Self {
            text: format!("[{}]", id),
            bold: false,
            italic: false,
            underline: false,
            code: false,
            math: false,
            footnote_ref: Some(id),
        }
    }
}

/// Catatan Kaki Akademis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslFootnote {
    pub id: usize,
    pub label: String,
    pub text: String,
}

/// Blok Elemen Dokumen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DslElement {
    Heading {
        level: u8,
        text: String,
        id: String,
    },
    Paragraph {
        runs: Vec<DslRun>,
        align: String,
    },
    Table {
        title: Option<String>,
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Equation {
        latex: String,
        is_block: bool,
    },
    Callout {
        title: String,
        text: String,
        kind: String, // "info", "warning", "tip", "important"
    },
    PageBreak,
    Divider,
    BulletList {
        items: Vec<String>,
    },
    NumberedList {
        items: Vec<String>,
    },
}

/// Model Dokumen Berdaulat Widya-Lang (WordCraft Ir / Sovereign Doc Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidyaDocumentDsl {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: String,
    pub organization: String,
    pub paper_size: String,
    pub margins: DocumentMargins,
    pub has_toc: bool,
    pub elements: Vec<DslElement>,
    pub footnotes: Vec<DslFootnote>,
}

impl Default for WidyaDocumentDsl {
    fn default() -> Self {
        Self::new("Naskah Dokumen Baru")
    }
}

impl WidyaDocumentDsl {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            subtitle: None,
            author: "Satriyo".to_string(),
            organization: "Widya Sovereign Academic Engine".to_string(),
            paper_size: "A4".to_string(),
            margins: DocumentMargins::default(),
            has_toc: true,
            elements: Vec::new(),
            footnotes: Vec::new(),
        }
    }

    pub fn add_heading(&mut self, text: &str, level: u8) {
        let slug = text
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-");
        let id = if slug.is_empty() {
            format!("heading-ref-{}", self.elements.len() + 1)
        } else {
            slug
        };

        self.elements.push(DslElement::Heading {
            level: level.clamp(1, 6),
            text: text.to_string(),
            id,
        });
    }

    pub fn add_paragraph(&mut self, text: &str) {
        let runs = parse_inline_dsl_runs(text);
        self.elements.push(DslElement::Paragraph {
            runs,
            align: "justify".to_string(),
        });
    }

    pub fn add_footnote(&mut self, text: &str) -> usize {
        let next_id = self.footnotes.len() + 1;
        self.footnotes.push(DslFootnote {
            id: next_id,
            label: next_id.to_string(),
            text: text.to_string(),
        });
        next_id
    }

    pub fn add_equation(&mut self, latex: &str, is_block: bool) {
        self.elements.push(DslElement::Equation {
            latex: latex.to_string(),
            is_block,
        });
    }

    pub fn add_table(&mut self, title: Option<&str>, headers: Vec<String>, rows: Vec<Vec<String>>) {
        self.elements.push(DslElement::Table {
            title: title.map(|s| s.to_string()),
            headers,
            rows,
        });
    }

    pub fn add_page_break(&mut self) {
        self.elements.push(DslElement::PageBreak);
    }

    /// Parser dari Naskah Markdown Komprehensif (seperti draf disertasi)
    pub fn from_markdown(md_content: &str, default_title: Option<&str>) -> Self {
        let md_doc = MarkdownDocument::parse(md_content);
        let mut title = default_title.unwrap_or("Draf Disertasi Akademis").to_string();
        let mut doc = Self::new(&title);

        let mut footnote_counter = 0usize;
        let mut footnote_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        // Lintasan 1: Ekstrak definisi footnote [^1]: teks
        for block in &md_doc.blocks {
            if let MarkdownBlock::FootnoteDefinition { label, content } = block {
                footnote_counter += 1;
                footnote_map.insert(label.clone(), footnote_counter);
                doc.footnotes.push(DslFootnote {
                    id: footnote_counter,
                    label: label.clone(),
                    text: content.clone(),
                });
            }
        }

        // Lintasan 2: Ekstrak blok konten
        let mut first_h1 = true;
        for block in md_doc.blocks {
            match block {
                MarkdownBlock::Heading { level, text } => {
                    if level == 1 && first_h1 {
                        title = text.clone();
                        doc.title = title.clone();
                        first_h1 = false;
                    }
                    doc.add_heading(&text, level);
                }
                MarkdownBlock::Paragraph(raw_p) => {
                    // Deteksi rujukan catatan kaki markdown [^1]
                    let mut runs = parse_inline_dsl_runs(&raw_p);
                    for r in &mut runs {
                        if r.text.starts_with("[^") && r.text.ends_with(']') {
                            let label = &r.text[2..r.text.len() - 1];
                            if let Some(&fn_id) = footnote_map.get(label) {
                                r.footnote_ref = Some(fn_id);
                                r.text = format!("[{}]", fn_id);
                            }
                        }
                    }
                    doc.elements.push(DslElement::Paragraph {
                        runs,
                        align: "justify".to_string(),
                    });
                }
                MarkdownBlock::Table { headers, rows, .. } => {
                    doc.elements.push(DslElement::Table {
                        title: None,
                        headers,
                        rows,
                    });
                }
                MarkdownBlock::TableOfContents => {
                    doc.has_toc = true;
                }
                MarkdownBlock::HorizontalRule => {
                    doc.elements.push(DslElement::Divider);
                }
                MarkdownBlock::CodeBlock { lang, code } => {
                    if lang == "latex" || lang == "math" {
                        doc.add_equation(&code, true);
                    } else {
                        doc.elements.push(DslElement::Callout {
                            title: format!("Kode Sumber ({})", if lang.is_empty() { "Plain" } else { &lang }),
                            text: code,
                            kind: "tip".to_string(),
                        });
                    }
                }
                MarkdownBlock::Blockquote(b) => {
                    doc.elements.push(DslElement::Callout {
                        title: "Catatan".to_string(),
                        text: b,
                        kind: "info".to_string(),
                    });
                }
                MarkdownBlock::UnorderedList(items) => {
                    doc.elements.push(DslElement::BulletList { items });
                }
                MarkdownBlock::OrderedList(items) => {
                    let text_items = items.into_iter().map(|(_, t)| t).collect();
                    doc.elements.push(DslElement::NumberedList { items: text_items });
                }
                MarkdownBlock::Checklist(items) => {
                    let text_items = items
                        .into_iter()
                        .map(|(c, t)| format!("[{}] {}", if c { "✓" } else { " " }, t))
                        .collect();
                    doc.elements.push(DslElement::BulletList { items: text_items });
                }
                MarkdownBlock::FootnoteDefinition { .. } => {
                    // Sudah diproses di lintasan 1
                }
            }
        }

        doc
    }

    /// Parser dari Skrip Kode Bahasa Widya-Lang (`.widya` / `.wdoc`)
    pub fn from_widya_dsl(code: &str) -> Result<Self, String> {
        let mut doc = Self::new("Dokumen Widya-Lang");
        let lines: Vec<&str> = code.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() || line.starts_with("//") {
                i += 1;
                continue;
            }

            if line.starts_with("judul:") {
                let val = extract_quoted_value(line);
                if !val.is_empty() {
                    doc.title = val;
                }
            } else if line.starts_with("penulis:") {
                let val = extract_quoted_value(line);
                if !val.is_empty() {
                    doc.author = val;
                }
            } else if line.starts_with("institusi:") || line.starts_with("organisasi:") {
                let val = extract_quoted_value(line);
                if !val.is_empty() {
                    doc.organization = val;
                }
            } else if line.starts_with("ukuran:") {
                let val = extract_quoted_value(line);
                if !val.is_empty() {
                    doc.paper_size = val;
                }
            } else if line.starts_with("daftar_isi:") {
                doc.has_toc = line.contains("benar") || line.contains("true");
            } else if line.starts_with("bab(") {
                let title = extract_arg_between(line, "bab(", ")");
                doc.add_heading(&title, 1);
            } else if line.starts_with("subbab(") {
                let title = extract_arg_between(line, "subbab(", ")");
                doc.add_heading(&title, 2);
            } else if line.starts_with("subsubbab(") {
                let title = extract_arg_between(line, "subsubbab(", ")");
                doc.add_heading(&title, 3);
            } else if line.starts_with("paragraf(") {
                let content = extract_arg_between(line, "paragraf(", ")");
                doc.add_paragraph(&content);
            } else if line.starts_with("catatan_kaki(") {
                let fn_text = extract_arg_between(line, "catatan_kaki(", ")");
                let fid = doc.add_footnote(&fn_text);
                // Tambahkan rujukan di paragraf terakhir jika ada
                if let Some(DslElement::Paragraph { runs, .. }) = doc.elements.last_mut() {
                    runs.push(DslRun::footnote(fid));
                }
            } else if line.starts_with("persamaan(") || line.starts_with("rumus(") {
                let formula = extract_arg_between(line, "(", ")");
                doc.add_equation(&formula, true);
            } else if line.starts_with("pemisah_halaman()") {
                doc.add_page_break();
            } else if line.starts_with("garis_pemisah()") {
                doc.elements.push(DslElement::Divider);
            } else if line.starts_with("tabel(") {
                // Parsing blok tabel sederhana
                let headers = extract_array_strings(line, "kolom:");
                let mut rows = Vec::new();

                let mut j = i + 1;
                while j < lines.len() {
                    let subline = lines[j].trim();
                    if subline.starts_with("baris:") || subline.starts_with("baris(") {
                        let row_vals = extract_array_strings(subline, "");
                        if !row_vals.is_empty() {
                            rows.push(row_vals);
                        }
                    } else if subline.starts_with('}') || subline.starts_with(")") {
                        break;
                    }
                    j += 1;
                }
                i = j;

                let title = if line.contains("judul:") {
                    let t = extract_arg_between(line, "judul:", ",");
                    Some(t.trim_matches(|c| c == '"' || c == '\'').to_string())
                } else {
                    None
                };

                doc.elements.push(DslElement::Table {
                    title,
                    headers,
                    rows,
                });
            }

            i += 1;
        }

        Ok(doc)
    }

    /// Render ke Struktur HTML Semantik Lengkap untuk WidyaWriter
    pub fn to_writer_html(&self) -> String {
        let mut html = String::new();

        // 1. Judul Dokumen Utama
        html.push_str(&format!(
            "<h1 class=\"doc-main-title\" style=\"text-align:center; margin-bottom: 24px;\">{}</h1>\n",
            html_escape(&self.title)
        ));

        // 2. Daftar Isi Otomatis
        if self.has_toc {
            html.push_str("<div class=\"doc-toc\" contenteditable=\"false\">\n");
            html.push_str("  <div class=\"doc-toc-header\">\n");
            html.push_str("    <span class=\"doc-toc-title\">DAFTAR ISI</span>\n");
            html.push_str("    <button type=\"button\" class=\"doc-toc-update-btn\" onclick=\"OfficeCore.updateTableOfContents()\" title=\"Perbarui nomor halaman & judul\">⟳ Perbarui Daftar Isi</button>\n");
            html.push_str("  </div>\n");
            html.push_str("  <div class=\"doc-toc-list\">\n");

            let mut h_idx = 1;
            for el in &self.elements {
                if let DslElement::Heading { level, text, id } = el {
                    let prefix = if *level == 1 {
                        let p = format!("{}. ", h_idx);
                        h_idx += 1;
                        p
                    } else {
                        "".to_string()
                    };
                    html.push_str(&format!(
                        "    <a class=\"toc-item level-{}\" href=\"#{}\" onclick=\"event.preventDefault(); document.getElementById('{}')?.scrollIntoView({{behavior:'smooth', block:'center'}});\">\n",
                        level, id, id
                    ));
                    html.push_str(&format!("      <span class=\"toc-title-text\">{}{}</span>\n", prefix, html_escape(text)));
                    html.push_str("      <span class=\"toc-dots\"></span>\n");
                    html.push_str("      <span class=\"toc-page-num\">1</span>\n");
                    html.push_str("    </a>\n");
                }
            }
            html.push_str("  </div>\n");
            html.push_str("</div>\n<p><br></p>\n");
        }

        // 3. Elemen Konten Dokumen
        for el in &self.elements {
            match el {
                DslElement::Heading { level, text, id } => {
                    let tag = format!("h{}", level);
                    html.push_str(&format!(
                        "<{} id=\"{}\" style=\"margin-top:20px; margin-bottom:10px;\">{}</{}>\n",
                        tag, id, html_escape(text), tag
                    ));
                }
                DslElement::Paragraph { runs, align } => {
                    html.push_str(&format!("<p style=\"text-align:{}; line-height:1.6; margin-bottom:12px;\">", align));
                    for run in runs {
                        if let Some(fn_id) = run.footnote_ref {
                            html.push_str(&format!(
                                "<sup class=\"doc-footnote-ref\" data-note-num=\"{}\"><a href=\"#footnote-{}\">[{}]</a></sup>&nbsp;",
                                fn_id, fn_id, fn_id
                            ));
                        } else {
                            let mut tag_open = String::new();
                            let mut tag_close = String::new();
                            if run.bold {
                                tag_open.push_str("<strong>");
                                tag_close.insert_str(0, "</strong>");
                            }
                            if run.italic {
                                tag_open.push_str("<em>");
                                tag_close.insert_str(0, "</em>");
                            }
                            if run.underline {
                                tag_open.push_str("<u>");
                                tag_close.insert_str(0, "</u>");
                            }
                            if run.code {
                                tag_open.push_str("<code style=\"background:#f1f5f9; padding:2px 4px; border-radius:3px;\">");
                                tag_close.insert_str(0, "</code>");
                            }
                            if run.math {
                                tag_open.push_str("<span class=\"equation-inline\" style=\"font-style:italic;\">");
                                tag_close.insert_str(0, "</span>");
                            }
                            html.push_str(&format!("{}{}{}", tag_open, html_escape(&run.text), tag_close));
                        }
                    }
                    html.push_str("</p>\n");
                }
                DslElement::Table { title, headers, rows } => {
                    if let Some(t) = title {
                        html.push_str(&format!(
                            "<p style=\"font-weight:bold; margin-top:16px; margin-bottom:6px;\">{}</p>\n",
                            html_escape(t)
                        ));
                    }
                    html.push_str("<table border=\"1\" style=\"width:100%; border-collapse:collapse; margin-bottom:16px;\">\n");
                    if !headers.is_empty() {
                        html.push_str("  <thead>\n    <tr style=\"background:#e2e8f0;\">\n");
                        for h in headers {
                            html.push_str(&format!(
                                "      <th style=\"padding:8px; border:1px solid #cbd5e1; text-align:center;\">{}</th>\n",
                                html_escape(h)
                            ));
                        }
                        html.push_str("    </tr>\n  </thead>\n");
                    }
                    html.push_str("  <tbody>\n");
                    for row in rows {
                        html.push_str("    <tr>\n");
                        for cell in row {
                            html.push_str(&format!(
                                "      <td style=\"padding:6px; border:1px solid #cbd5e1;\">{}</td>\n",
                                html_escape(cell)
                            ));
                        }
                        html.push_str("    </tr>\n");
                    }
                    html.push_str("  </tbody>\n</table>\n");
                }
                DslElement::Equation { latex, is_block } => {
                    if *is_block {
                        html.push_str(&format!(
                            "<div class=\"doc-equation-block\" style=\"text-align:center; padding:12px; background:#f8fafc; border:1px dashed #cbd5e1; border-radius:4px; margin:14px 0;\"><code>{}</code></div>\n",
                            html_escape(latex)
                        ));
                    } else {
                        html.push_str(&format!("<code>{}</code>", html_escape(latex)));
                    }
                }
                DslElement::Callout { title, text, kind } => {
                    let (border_color, bg_color) = match kind.as_str() {
                        "warning" => ("#eab308", "#fefce8"),
                        "tip" => ("#06b6d4", "#ecfeff"),
                        "important" => ("#ef4444", "#fef2f2"),
                        _ => ("#3b82f6", "#eff6ff"),
                    };
                    html.push_str(&format!(
                        "<div style=\"border-left:4px solid {}; background:{}; padding:12px; border-radius:4px; margin:14px 0;\">\n  <strong>{}</strong><br><span>{}</span>\n</div>\n",
                        border_color, bg_color, html_escape(title), html_escape(text)
                    ));
                }
                DslElement::BulletList { items } => {
                    html.push_str("<ul style=\"margin-left:24px; margin-bottom:12px;\">\n");
                    for it in items {
                        html.push_str(&format!("  <li>{}</li>\n", html_escape(it)));
                    }
                    html.push_str("</ul>\n");
                }
                DslElement::NumberedList { items } => {
                    html.push_str("<ol style=\"margin-left:24px; margin-bottom:12px;\">\n");
                    for it in items {
                        html.push_str(&format!("  <li>{}</li>\n", html_escape(it)));
                    }
                    html.push_str("</ol>\n");
                }
                DslElement::Divider => {
                    html.push_str("<hr style=\"border:none; border-top:1px solid #cbd5e1; margin:20px 0;\">\n");
                }
                DslElement::PageBreak => {
                    html.push_str("<div class=\"doc-page-break\" data-page-break=\"true\" style=\"page-break-after:always;\"></div>\n");
                }
            }
        }

        // 4. Catatan Kaki (Footnotes Container)
        if !self.footnotes.is_empty() {
            html.push_str("<div class=\"doc-page-footnotes\" style=\"display:block; margin-top:20px;\">\n");
            html.push_str("  <hr class=\"doc-footnote-rule\">\n");
            for fn_item in &self.footnotes {
                html.push_str(&format!(
                    "  <div class=\"doc-footnote-item\" id=\"footnote-{}\" data-note-num=\"{}\"><sup class=\"footnote-item-num\">{}</sup> <span class=\"footnote-item-text\">{}</span></div>\n",
                    fn_item.id, fn_item.id, fn_item.id, html_escape(&fn_item.text)
                ));
            }
            html.push_str("</div>\n");
        }

        html
    }

    /// Konversi Dokumen DSL ke OpenXML DOCX Asli (`DocxDocument`)
    pub fn to_docx(&self) -> DocxDocument {
        let mut doc = DocxDocument::new(&self.title);
        doc.author = self.author.clone();
        doc.organization = self.organization.clone();

        if self.has_toc {
            doc.elements.push(DocxElement::TableOfContents);
        }

        for el in &self.elements {
            match el {
                DslElement::Heading { level, text, .. } => {
                    doc.add_heading(text, *level);
                }
                DslElement::Paragraph { runs, align } => {
                    let mut text_runs = Vec::new();
                    for r in runs {
                        let mut style = TextStyle::new();
                        style.bold = r.bold;
                        style.italic = r.italic;
                        style.underline = r.underline;
                        if r.code {
                            style.is_monospace = true;
                        }
                        text_runs.push(TextRun {
                            text: r.text.clone(),
                            style,
                        });
                    }
                    doc.elements.push(DocxElement::Paragraph {
                        runs: text_runs,
                        align: align.clone(),
                        is_rtl: false,
                    });
                }
                DslElement::Table { headers, rows, .. } => {
                    let mut all_rows = Vec::new();
                    if !headers.is_empty() {
                        all_rows.push(headers.clone());
                    }
                    for r in rows {
                        all_rows.push(r.clone());
                    }
                    doc.elements.push(DocxElement::Table {
                        rows: all_rows,
                        has_header: !headers.is_empty(),
                        alignments: vec![ColumnAlignment::Left; headers.len().max(1)],
                    });
                }
                DslElement::Equation { latex, is_block } => {
                    doc.elements.push(DocxElement::Equation {
                        latex_math: latex.clone(),
                        is_block: *is_block,
                    });
                }
                DslElement::Callout { title, text, .. } => {
                    doc.elements.push(DocxElement::Callout {
                        title: title.clone(),
                        text: text.clone(),
                        border_color: "#2563eb".to_string(),
                    });
                }
                DslElement::BulletList { items } => {
                    for it in items {
                        doc.elements.push(DocxElement::BulletItem {
                            runs: vec![TextRun {
                                text: it.clone(),
                                style: TextStyle::new(),
                            }],
                            indent_level: 0,
                        });
                    }
                }
                DslElement::NumberedList { items } => {
                    for (idx, it) in items.iter().enumerate() {
                        doc.elements.push(DocxElement::NumberedItem {
                            number: idx + 1,
                            runs: vec![TextRun {
                                text: it.clone(),
                                style: TextStyle::new(),
                            }],
                            indent_level: 0,
                        });
                    }
                }
                DslElement::Divider => {
                    doc.elements.push(DocxElement::Divider);
                }
                DslElement::PageBreak => {
                    doc.elements.push(DocxElement::PageBreak);
                }
            }
        }

        for fn_item in &self.footnotes {
            doc.elements.push(DocxElement::Footnote {
                label: fn_item.label.clone(),
                text: fn_item.text.clone(),
            });
        }

        doc
    }

    /// Konversi Dokumen DSL ke Format PDF Vektor Murni (`PdfDocument`)
    pub fn to_pdf(&self) -> PdfDocument {
        let docx = self.to_docx();
        DocumentConverter::docx_to_pdf(&docx)
    }

    /// Ekspor Kembali ke Kode Sintaks Murni Widya-Lang DSL (`.widya`)
    pub fn to_widya_dsl_code(&self) -> String {
        let mut code = String::new();
        code.push_str("// Widya-Lang Sovereign Document DSL\n");
        code.push_str("dokumen {\n");
        code.push_str(&format!("    judul: \"{}\",\n", self.title));
        code.push_str(&format!("    penulis: \"{}\",\n", self.author));
        code.push_str(&format!("    institusi: \"{}\",\n", self.organization));
        code.push_str(&format!("    ukuran: \"{}\",\n", self.paper_size));
        code.push_str(&format!("    daftar_isi: {},\n\n", if self.has_toc { "benar" } else { "salah" }));

        for el in &self.elements {
            match el {
                DslElement::Heading { level, text, .. } => {
                    let fn_name = match level {
                        1 => "bab",
                        2 => "subbab",
                        _ => "subsubbab",
                    };
                    code.push_str(&format!("    {}(\"{}\")\n", fn_name, text));
                }
                DslElement::Paragraph { runs, .. } => {
                    let p_text = runs.iter().map(|r| r.text.as_str()).collect::<Vec<&str>>().join("");
                    code.push_str(&format!("    paragraf(\"{}\")\n", p_text.replace('"', "\\\"")));
                }
                DslElement::Table { title, headers, rows } => {
                    code.push_str("    tabel(\n");
                    if let Some(t) = title {
                        code.push_str(&format!("        judul: \"{}\",\n", t));
                    }
                    code.push_str(&format!(
                        "        kolom: {:?},\n",
                        headers
                    ));
                    code.push_str("        baris: [\n");
                    for r in rows {
                        code.push_str(&format!("            {:?},\n", r));
                    }
                    code.push_str("        ]\n    )\n");
                }
                DslElement::Equation { latex, .. } => {
                    code.push_str(&format!("    persamaan(\"{}\")\n", latex.replace('"', "\\\"")));
                }
                DslElement::PageBreak => {
                    code.push_str("    pemisah_halaman()\n");
                }
                _ => {}
            }
        }

        for fn_item in &self.footnotes {
            code.push_str(&format!("    catatan_kaki(\"{}\")\n", fn_item.text.replace('"', "\\\"")));
        }

        code.push_str("}\n");
        code
    }
}

// -----------------------------------------------------------------------------
// Helper Fungsi Parser String & Inline Runs
// -----------------------------------------------------------------------------

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn extract_quoted_value(line: &str) -> String {
    if let Some(first_quote) = line.find('"') {
        if let Some(second_quote) = line[first_quote + 1..].find('"') {
            return line[first_quote + 1..first_quote + 1 + second_quote].to_string();
        }
    }
    String::new()
}

fn extract_arg_between(line: &str, start_token: &str, end_token: &str) -> String {
    if let Some(pos) = line.find(start_token) {
        let rest = &line[pos + start_token.len()..];
        if let Some(end_pos) = rest.rfind(end_token) {
            let val = rest[..end_pos].trim();
            return val.trim_matches(|c| c == '"' || c == '\'').to_string();
        }
    }
    String::new()
}

fn extract_array_strings(line: &str, key: &str) -> Vec<String> {
    let search_area = if key.is_empty() {
        line
    } else if let Some(p) = line.find(key) {
        &line[p + key.len()..]
    } else {
        return Vec::new();
    };

    if let (Some(b_open), Some(b_close)) = (search_area.find('['), search_area.find(']')) {
        let content = &search_area[b_open + 1..b_close];
        return content
            .split(',')
            .map(|s| s.trim().trim_matches(|c| c == '"' || c == '\'' || c == ' ').to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }
    Vec::new()
}

fn parse_inline_dsl_runs(text: &str) -> Vec<DslRun> {
    let mut runs = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    let mut current_buf = String::new();

    while i < chars.len() {
        // Deteksi footnote [^...]
        if i + 1 < chars.len() && chars[i] == '[' && chars[i + 1] == '^' {
            if let Some(close_idx) = chars[i + 2..].iter().position(|&c| c == ']') {
                if !current_buf.is_empty() {
                    runs.push(DslRun::text(&current_buf));
                    current_buf.clear();
                }
                let fn_label: String = chars[i + 2..i + 2 + close_idx].iter().collect();
                let full_tag = format!("[^{}]", fn_label);
                runs.push(DslRun::text(&full_tag));
                i += close_idx + 3;
                continue;
            }
        }

        // Deteksi math $...$
        if chars[i] == '$' && (i == 0 || chars[i - 1] != '\\') {
            if let Some(close_idx) = chars[i + 1..].iter().position(|&c| c == '$') {
                if !current_buf.is_empty() {
                    runs.push(DslRun::text(&current_buf));
                    current_buf.clear();
                }
                let math_content: String = chars[i + 1..i + 1 + close_idx].iter().collect();
                let mut r = DslRun::text(&format!("${}$", math_content));
                r.math = true;
                runs.push(r);
                i += close_idx + 2;
                continue;
            }
        }

        // Deteksi tebal **...**
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            if let Some(close_idx) = chars[i + 2..].windows(2).position(|w| w == ['*', '*']) {
                if !current_buf.is_empty() {
                    runs.push(DslRun::text(&current_buf));
                    current_buf.clear();
                }
                let bold_content: String = chars[i + 2..i + 2 + close_idx].iter().collect();
                runs.push(DslRun::bold(&bold_content));
                i += close_idx + 4;
                continue;
            }
        }

        // Deteksi miring *...*
        if chars[i] == '*' && (i == 0 || chars[i - 1] != '*') {
            if let Some(close_idx) = chars[i + 1..].iter().position(|&c| c == '*') {
                if !current_buf.is_empty() {
                    runs.push(DslRun::text(&current_buf));
                    current_buf.clear();
                }
                let italic_content: String = chars[i + 1..i + 1 + close_idx].iter().collect();
                runs.push(DslRun::italic(&italic_content));
                i += close_idx + 2;
                continue;
            }
        }

        // Deteksi code `...`
        if chars[i] == '`' {
            if let Some(close_idx) = chars[i + 1..].iter().position(|&c| c == '`') {
                if !current_buf.is_empty() {
                    runs.push(DslRun::text(&current_buf));
                    current_buf.clear();
                }
                let code_content: String = chars[i + 1..i + 1 + close_idx].iter().collect();
                runs.push(DslRun::code(&code_content));
                i += close_idx + 2;
                continue;
            }
        }

        current_buf.push(chars[i]);
        i += 1;
    }

    if !current_buf.is_empty() {
        runs.push(DslRun::text(&current_buf));
    }

    runs
}
