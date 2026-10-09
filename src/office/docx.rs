use crate::office::security::{DigitalSignature, DocumentWatermark, OfficeSecurity};
use crate::office::zip::ZipPackage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Gaya Format Teks pada Paragraf DOCX
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub font_size_pt: f32,
    pub color_hex: Option<String>,
    pub font_family: String,
    pub is_rtl: bool,
    pub is_monospace: bool,
    pub link_url: Option<String>,
}

impl TextStyle {
    pub fn new() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            font_size_pt: 11.0,
            color_hex: None,
            font_family: "Calibri".to_string(),
            is_rtl: false,
            is_monospace: false,
            link_url: None,
        }
    }

    pub fn bold() -> Self {
        let mut s = Self::new();
        s.bold = true;
        s
    }

    pub fn italic() -> Self {
        let mut s = Self::new();
        s.italic = true;
        s
    }

    pub fn bold_italic() -> Self {
        let mut s = Self::new();
        s.bold = true;
        s.italic = true;
        s
    }

    pub fn code() -> Self {
        let mut s = Self::new();
        s.is_monospace = true;
        s.font_family = "Courier New".to_string();
        s.color_hex = Some("#0f766e".to_string());
        s
    }

    pub fn link(url: &str) -> Self {
        let mut s = Self::new();
        s.underline = true;
        s.color_hex = Some("#0284c7".to_string());
        s.link_url = Some(url.to_string());
        s
    }

    pub fn rtl() -> Self {
        let mut s = Self::new();
        s.is_rtl = true;
        s.font_family = "Amiri".to_string();
        s
    }
}

/// Bagian Teks (Run) di dalam Paragraf
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
    pub style: TextStyle,
}

/// Jenis Elemen Dokumen Word
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocxElement {
    Heading { level: u8, text: String },
    Paragraph { runs: Vec<TextRun>, align: String, is_rtl: bool },
    Table {
        rows: Vec<Vec<String>>,
        has_header: bool,
        alignments: Vec<crate::office::markdown::ColumnAlignment>,
    },
    PageBreak,
    Divider,
    BulletItem { runs: Vec<TextRun>, indent_level: u8 },
    NumberedItem { number: usize, runs: Vec<TextRun>, indent_level: u8 },
    Callout { title: String, text: String, border_color: String },
    Diagram { raw_mermaid: String },
    Footnote { label: String, text: String },
    TableOfContents,
}

/// Dokumen Microsoft Word (.docx / OpenXML Document) Kelas Industri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocxDocument {
    pub title: String,
    pub author: String,
    pub organization: String,
    pub elements: Vec<DocxElement>,
    pub watermark: Option<DocumentWatermark>,
    pub digital_signatures: Vec<DigitalSignature>,
}

impl DocxDocument {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            author: "Widya Office Suite".to_string(),
            organization: "Pemerintah Republik Indonesia / Korporasi Berdaulat".to_string(),
            elements: Vec::new(),
            watermark: None,
            digital_signatures: Vec::new(),
        }
    }

    pub fn set_watermark(&mut self, text: &str, color_hex: &str, opacity: f32) {
        self.watermark = Some(DocumentWatermark {
            text: text.to_string(),
            color_hex: color_hex.to_string(),
            opacity,
            font_size_pt: 48.0,
            rotation_deg: -45.0,
            is_confidential: true,
        });
    }

    pub fn add_heading(&mut self, text: &str, level: u8) {
        self.elements.push(DocxElement::Heading {
            level: level.clamp(1, 6),
            text: text.to_string(),
        });
    }

    pub fn add_paragraph(&mut self, text: &str) {
        self.elements.push(DocxElement::Paragraph {
            runs: vec![TextRun {
                text: text.to_string(),
                style: TextStyle::new(),
            }],
            align: "left".to_string(),
            is_rtl: false,
        });
    }

    pub fn add_paragraph_runs(&mut self, runs: Vec<TextRun>) {
        self.elements.push(DocxElement::Paragraph {
            runs,
            align: "left".to_string(),
            is_rtl: false,
        });
    }

    pub fn add_styled_paragraph(&mut self, text: &str, bold: bool, italic: bool, font_size_pt: f32, color_hex: Option<&str>) {
        self.elements.push(DocxElement::Paragraph {
            runs: vec![TextRun {
                text: text.to_string(),
                style: TextStyle {
                    bold,
                    italic,
                    underline: false,
                    strikethrough: false,
                    font_size_pt,
                    color_hex: color_hex.map(|s| s.to_string()),
                    font_family: "Calibri".to_string(),
                    is_rtl: false,
                    is_monospace: false,
                    link_url: None,
                },
            }],
            align: "left".to_string(),
            is_rtl: false,
        });
    }

    /// Menambahkan paragraf Kanan-ke-Kiri (RTL) seperti Bahasa Arab / Jawi / Ibrani
    pub fn add_rtl_paragraph(&mut self, text: &str) {
        self.elements.push(DocxElement::Paragraph {
            runs: vec![TextRun {
                text: text.to_string(),
                style: TextStyle {
                    bold: false,
                    italic: false,
                    underline: false,
                    strikethrough: false,
                    font_size_pt: 13.0,
                    color_hex: None,
                    font_family: "Amiri".to_string(),
                    is_rtl: true,
                    is_monospace: false,
                    link_url: None,
                },
            }],
            align: "right".to_string(),
            is_rtl: true,
        });
    }

    /// Menambahkan paragraf RTL dengan format khusus (ukuran, warna, tebal)
    pub fn add_styled_rtl_paragraph(&mut self, text: &str, bold: bool, italic: bool, font_size_pt: f32, font_family: Option<&str>, color_hex: Option<&str>) {
        self.elements.push(DocxElement::Paragraph {
            runs: vec![TextRun {
                text: text.to_string(),
                style: TextStyle {
                    bold,
                    italic,
                    underline: false,
                    strikethrough: false,
                    font_size_pt,
                    color_hex: color_hex.map(|s| s.to_string()),
                    font_family: font_family.unwrap_or("Amiri").to_string(),
                    is_rtl: true,
                    is_monospace: false,
                    link_url: None,
                },
            }],
            align: "right".to_string(),
            is_rtl: true,
        });
    }

    pub fn add_table(&mut self, rows: Vec<Vec<String>>, has_header: bool) {
        self.add_table_aligned(rows, has_header, Vec::new());
    }

    pub fn add_table_aligned(
        &mut self,
        rows: Vec<Vec<String>>,
        has_header: bool,
        alignments: Vec<crate::office::markdown::ColumnAlignment>,
    ) {
        self.elements.push(DocxElement::Table {
            rows,
            has_header,
            alignments,
        });
    }

    pub fn add_numbered_runs(&mut self, number: usize, runs: Vec<TextRun>, indent_level: u8) {
        self.elements.push(DocxElement::NumberedItem {
            number,
            runs,
            indent_level,
        });
    }

    pub fn add_bullet(&mut self, text: &str) {
        self.elements.push(DocxElement::BulletItem {
            runs: vec![TextRun {
                text: text.to_string(),
                style: TextStyle::new(),
            }],
            indent_level: 0,
        });
    }

    pub fn add_bullet_runs(&mut self, runs: Vec<TextRun>, indent_level: u8) {
        self.elements.push(DocxElement::BulletItem {
            runs,
            indent_level,
        });
    }

    pub fn add_callout(&mut self, title: &str, text: &str, border_color: &str) {
        self.elements.push(DocxElement::Callout {
            title: title.to_string(),
            text: text.to_string(),
            border_color: border_color.to_string(),
        });
    }

    pub fn add_page_break(&mut self) {
        self.elements.push(DocxElement::PageBreak);
    }

    pub fn add_divider(&mut self) {
        self.elements.push(DocxElement::Divider);
    }

    pub fn add_diagram(&mut self, raw_mermaid: &str) {
        self.elements.push(DocxElement::Diagram {
            raw_mermaid: raw_mermaid.to_string(),
        });
    }

    pub fn add_footnote(&mut self, label: &str, text: &str) {
        self.elements.push(DocxElement::Footnote {
            label: label.to_string(),
            text: text.to_string(),
        });
    }

    /// Template Engine & Mail Merge: Ganti Placeholder `{{kunci}}` dengan Nilai Nyata
    pub fn apply_template(&mut self, variables: &HashMap<String, String>) {
        for el in &mut self.elements {
            match el {
                DocxElement::Heading { text, .. } => {
                    *text = replace_placeholders(text, variables);
                }
                DocxElement::Paragraph { runs, .. } => {
                    for r in runs {
                        r.text = replace_placeholders(&r.text, variables);
                    }
                }
                DocxElement::Table { rows, .. } => {
                    for row in rows {
                        for cell in row {
                            *cell = replace_placeholders(cell, variables);
                        }
                    }
                }
                DocxElement::BulletItem { runs, .. } => {
                    for r in runs {
                        r.text = replace_placeholders(&r.text, variables);
                    }
                }
                DocxElement::Callout { title, text, .. } => {
                    *title = replace_placeholders(title, variables);
                    *text = replace_placeholders(text, variables);
                }
                _ => {}
            }
        }
    }

    /// Sensor / Redaksi Otomatis Data Sensitif Pribadi (Kepatuhan UU PDP No. 27/2022)
    pub fn auto_redact_sensitive_data(&mut self) {
        for el in &mut self.elements {
            match el {
                DocxElement::Heading { text, .. } => {
                    *text = OfficeSecurity::redact_pdp_sensitive_text(text);
                }
                DocxElement::Paragraph { runs, .. } => {
                    for r in runs {
                        r.text = OfficeSecurity::redact_pdp_sensitive_text(&r.text);
                    }
                }
                DocxElement::Table { rows, .. } => {
                    for row in rows {
                        for cell in row {
                            *cell = OfficeSecurity::redact_pdp_sensitive_text(cell);
                        }
                    }
                }
                DocxElement::BulletItem { runs, .. } => {
                    for r in runs {
                        r.text = OfficeSecurity::redact_pdp_sensitive_text(&r.text);
                    }
                }
                DocxElement::NumberedItem { runs, .. } => {
                    for r in runs {
                        r.text = OfficeSecurity::redact_pdp_sensitive_text(&r.text);
                    }
                }
                DocxElement::Callout { text, .. } => {
                    *text = OfficeSecurity::redact_pdp_sensitive_text(text);
                }
                _ => {}
            }
        }
    }

    /// Tanda Tangan Digital (TTE) Dokumen
    pub fn sign_document(&mut self, signer_name: &str, role: &str, secret_key: &str) -> DigitalSignature {
        let plain_text = self.extract_plain_text();
        let sig = OfficeSecurity::sign_document(
            signer_name,
            role,
            &self.organization,
            plain_text.as_bytes(),
            secret_key,
        );
        self.digital_signatures.push(sig.clone());
        sig
    }

    /// Menambahkan Blok Daftar Isi Otomatis
    pub fn add_table_of_contents(&mut self) {
        self.elements.push(DocxElement::TableOfContents);
    }

    /// Ekstraksi Seluruh Teks Murni dari Dokumen
    pub fn extract_plain_text(&self) -> String {
        let mut out = String::new();
        for el in &self.elements {
            match el {
                DocxElement::Heading { text, .. } => {
                    out.push_str(text);
                    out.push_str("\n\n");
                }
                DocxElement::Paragraph { runs, .. } => {
                    for r in runs {
                        out.push_str(&r.text);
                    }
                    out.push('\n');
                }
                DocxElement::Table { rows, .. } => {
                    for row in rows {
                        out.push_str(&row.join("\t | "));
                        out.push('\n');
                    }
                    out.push('\n');
                }
                DocxElement::BulletItem { runs, .. } => {
                    out.push_str("• ");
                    for r in runs {
                        out.push_str(&r.text);
                    }
                    out.push('\n');
                }
                DocxElement::NumberedItem { number, runs, .. } => {
                    out.push_str(&format!("{}. ", number));
                    for r in runs {
                        out.push_str(&r.text);
                    }
                    out.push('\n');
                }
                DocxElement::Callout { title, text, .. } => {
                    out.push_str(&format!("[{}] {}\n", title, text));
                }
                DocxElement::PageBreak => {
                    out.push_str("\n--- [Page Break] ---\n\n");
                }
                DocxElement::Divider => {
                    out.push_str("\n--------------------\n\n");
                }
                DocxElement::Diagram { raw_mermaid } => {
                    out.push_str(&format!("\n[DIAGRAM ARSITEKTUR]\n{}\n\n", raw_mermaid));
                }
                DocxElement::Footnote { label, text } => {
                    out.push_str(&format!("[{}] {}\n", label, text));
                }
                DocxElement::TableOfContents => {
                    out.push_str("\n[DAFTAR ISI]\n\n");
                }
            }
        }
        out
    }

    /// Menghasilkan XML OpenXML Standar `word/document.xml`
    pub fn generate_document_xml(&self) -> String {
        let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        xml.push_str(r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#);
        xml.push_str("<w:body>");

        for el in &self.elements {
            match el {
                DocxElement::Heading { level, text } => {
                    xml.push_str(&format!(
                        r#"<w:p><w:pPr><w:pStyle w:val="Heading{}"/></w:pPr><w:r><w:t>{}</w:t></w:r></w:p>"#,
                        level, html_escape(text)
                    ));
                }
                DocxElement::Paragraph { runs, align, is_rtl } => {
                    xml.push_str("<w:p>");
                    let mut ppr = String::new();
                    if *is_rtl {
                        ppr.push_str("<w:bidi/>");
                    }
                    if align != "left" || *is_rtl {
                        let jc_val = if *is_rtl && align == "left" { "right" } else { align.as_str() };
                        ppr.push_str(&format!(r#"<w:jc w:val="{}"/>"#, jc_val));
                    }
                    if !ppr.is_empty() {
                        xml.push_str(&format!("<w:pPr>{}</w:pPr>", ppr));
                    }
                    for r in runs {
                        xml.push_str("<w:r>");
                        let mut rpr = String::new();
                        if r.style.is_rtl || *is_rtl { rpr.push_str("<w:rtl/>"); }
                        if r.style.bold { rpr.push_str("<w:b/>"); }
                        if r.style.italic { rpr.push_str("<w:i/>"); }
                        if r.style.underline { rpr.push_str(r#"<w:u w:val="single"/>"#); }
                        if r.style.strikethrough { rpr.push_str(r#"<w:strike/>"#); }
                        if r.style.font_size_pt > 0.0 {
                            rpr.push_str(&format!(r#"<w:sz w:val="{}"/>"#, (r.style.font_size_pt * 2.0) as u32));
                        }
                        if let Some(ref c) = r.style.color_hex {
                            rpr.push_str(&format!(r#"<w:color w:val="{}"/>"#, c.trim_start_matches('#')));
                        }
                        if !r.style.font_family.is_empty() {
                            rpr.push_str(&format!(r#"<w:rFonts w:ascii="{0}" w:hAnsi="{0}" w:cs="{0}"/>"#, r.style.font_family));
                        }
                        if !rpr.is_empty() {
                            xml.push_str(&format!("<w:rPr>{}</w:rPr>", rpr));
                        }
                        xml.push_str(&format!("<w:t>{}</w:t></w:r>", html_escape(&r.text)));
                    }
                    xml.push_str("</w:p>");
                }
                DocxElement::Table { rows, alignments, .. } => {
                    xml.push_str("<w:tbl><w:tblPr><w:tblBorders><w:top w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/><w:left w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/><w:bottom w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/><w:right w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/><w:insideH w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/><w:insideV w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/></w:tblBorders></w:tblPr>");
                    for row in rows {
                        xml.push_str("<w:tr>");
                        for (c_idx, cell) in row.iter().enumerate() {
                            let jc_tag = match alignments.get(c_idx) {
                                Some(crate::office::markdown::ColumnAlignment::Center) => r#"<w:pPr><w:jc w:val="center"/></w:pPr>"#,
                                Some(crate::office::markdown::ColumnAlignment::Right) => r#"<w:pPr><w:jc w:val="right"/></w:pPr>"#,
                                _ => "",
                            };
                            xml.push_str(&format!("<w:tc><w:p>{}", jc_tag));
                            let runs = parse_markdown_runs(cell);
                            for r in runs {
                                xml.push_str("<w:r>");
                                let mut rpr = String::new();
                                if r.style.bold { rpr.push_str("<w:b/>"); }
                                if r.style.italic { rpr.push_str("<w:i/>"); }
                                if r.style.underline { rpr.push_str(r#"<w:u w:val="single"/>"#); }
                                if r.style.strikethrough { rpr.push_str(r#"<w:strike/>"#); }
                                if let Some(ref c) = r.style.color_hex {
                                    rpr.push_str(&format!(r#"<w:color w:val="{}"/>"#, c.trim_start_matches('#')));
                                }
                                if !r.style.font_family.is_empty() {
                                    rpr.push_str(&format!(r#"<w:rFonts w:ascii="{0}" w:hAnsi="{0}" w:cs="{0}"/>"#, r.style.font_family));
                                }
                                if !rpr.is_empty() {
                                    xml.push_str(&format!("<w:rPr>{}</w:rPr>", rpr));
                                }
                                xml.push_str(&format!("<w:t>{}</w:t></w:r>", html_escape(&r.text)));
                            }
                            xml.push_str("</w:p></w:tc>");
                        }
                        xml.push_str("</w:tr>");
                    }
                    xml.push_str("</w:tbl>");
                }
                DocxElement::BulletItem { runs, indent_level } => {
                    xml.push_str(&format!(
                        r#"<w:p><w:pPr><w:numPr><w:ilvl w:val="{}"/><w:numId w:val="1"/></w:numPr></w:pPr>"#,
                        indent_level
                    ));
                    for r in runs {
                        xml.push_str("<w:r>");
                        let mut rpr = String::new();
                        if r.style.is_rtl { rpr.push_str("<w:rtl/>"); }
                        if r.style.bold { rpr.push_str("<w:b/>"); }
                        if r.style.italic { rpr.push_str("<w:i/>"); }
                        if r.style.underline { rpr.push_str(r#"<w:u w:val="single"/>"#); }
                        if r.style.strikethrough { rpr.push_str(r#"<w:strike/>"#); }
                        if r.style.font_size_pt > 0.0 {
                            rpr.push_str(&format!(r#"<w:sz w:val="{}"/>"#, (r.style.font_size_pt * 2.0) as u32));
                        }
                        if let Some(ref c) = r.style.color_hex {
                            rpr.push_str(&format!(r#"<w:color w:val="{}"/>"#, c.trim_start_matches('#')));
                        }
                        if !r.style.font_family.is_empty() {
                            rpr.push_str(&format!(r#"<w:rFonts w:ascii="{0}" w:hAnsi="{0}" w:cs="{0}"/>"#, r.style.font_family));
                        }
                        if !rpr.is_empty() {
                            xml.push_str(&format!("<w:rPr>{}</w:rPr>", rpr));
                        }
                        xml.push_str(&format!("<w:t>{}</w:t></w:r>", html_escape(&r.text)));
                    }
                    xml.push_str("</w:p>");
                }
                DocxElement::NumberedItem { number, runs, indent_level } => {
                    let indent_val = 720 + (*indent_level as usize * 360);
                    xml.push_str(&format!(
                        r#"<w:p><w:pPr><w:numPr><w:ilvl w:val="{}"/><w:numId w:val="2"/></w:numPr><w:ind w:left="{}" w:hanging="360"/></w:pPr><w:r><w:rPr><w:b/><w:color w:val="0369A1"/></w:rPr><w:t>{}. </w:t></w:r>"#,
                        indent_level, indent_val, number
                    ));
                    for r in runs {
                        xml.push_str("<w:r>");
                        let mut rpr = String::new();
                        if r.style.is_rtl { rpr.push_str("<w:rtl/>"); }
                        if r.style.bold { rpr.push_str("<w:b/>"); }
                        if r.style.italic { rpr.push_str("<w:i/>"); }
                        if r.style.underline { rpr.push_str(r#"<w:u w:val="single"/>"#); }
                        if r.style.strikethrough { rpr.push_str(r#"<w:strike/>"#); }
                        if r.style.font_size_pt > 0.0 {
                            rpr.push_str(&format!(r#"<w:sz w:val="{}"/>"#, (r.style.font_size_pt * 2.0) as u32));
                        }
                        if let Some(ref c) = r.style.color_hex {
                            rpr.push_str(&format!(r#"<w:color w:val="{}"/>"#, c.trim_start_matches('#')));
                        }
                        if !r.style.font_family.is_empty() {
                            rpr.push_str(&format!(r#"<w:rFonts w:ascii="{0}" w:hAnsi="{0}" w:cs="{0}"/>"#, r.style.font_family));
                        }
                        if !rpr.is_empty() {
                            xml.push_str(&format!("<w:rPr>{}</w:rPr>", rpr));
                        }
                        xml.push_str(&format!("<w:t>{}</w:t></w:r>", html_escape(&r.text)));
                    }
                    xml.push_str("</w:p>");
                }
                DocxElement::Callout { title, text, .. } => {
                    xml.push_str(&format!(
                        r#"<w:p><w:pPr><w:pBdr><w:left w:val="single" w:sz="24" w:space="15" w:color="0284C7"/></w:pBdr></w:pPr><w:r><w:rPr><w:b/></w:rPr><w:t>{}: </w:t></w:r><w:r><w:t>{}</w:t></w:r></w:p>"#,
                        html_escape(title), html_escape(text)
                    ));
                }
                DocxElement::PageBreak => {
                    xml.push_str(r#"<w:p><w:r><w:br w:type="page"/></w:r></w:p>"#);
                }
                DocxElement::Divider => {
                    xml.push_str(r#"<w:p><w:pPr><w:pBdr><w:bottom w:val="single" w:sz="6" w:space="1" w:color="CBD5E1"/></w:pBdr></w:pPr></w:p>"#);
                }
                DocxElement::Diagram { raw_mermaid } => {
                    xml.push_str(&format!(
                        r#"<w:p><w:pPr><w:pBdr><w:left w:val="single" w:sz="24" w:space="15" w:color="0284C7"/></w:pBdr></w:pPr><w:r><w:rPr><w:b/></w:rPr><w:t>[DIAGRAM ARSITEKTUR MERMAID]: </w:t></w:r><w:r><w:t>{}</w:t></w:r></w:p>"#,
                        html_escape(raw_mermaid)
                    ));
                }
                DocxElement::Footnote { label, text } => {
                    xml.push_str(&format!(
                        r#"<w:p><w:pPr><w:pBdr><w:top w:val="single" w:sz="4" w:space="1" w:color="CBD5E1"/></w:pBdr></w:pPr><w:r><w:rPr><w:vertAlign w:val="superscript"/><w:b/><w:color w:val="0369A1"/></w:rPr><w:t>[{}] </w:t></w:r><w:r><w:rPr><w:sz w:val="18"/><w:color w:val="475569"/></w:rPr><w:t>{}</w:t></w:r></w:p>"#,
                        html_escape(label), html_escape(text)
                    ));
                }
                DocxElement::TableOfContents => {
                    xml.push_str(r#"<w:p><w:pPr><w:pStyle w:val="TOCHeading"/><w:jc w:val="left"/></w:pPr><w:r><w:rPr><w:b/><w:sz w:val="32"/><w:color w:val="0F172A"/></w:rPr><w:t>DAFTAR ISI</w:t></w:r></w:p>"#);
                }
            }
        }

        xml.push_str("</w:body></w:document>");
        xml
    }

    /// Menghasilkan Berkas Biner .DOCX Asli (Zip Packaging Berstandar ECMA-376)
    pub fn export_to_docx_bytes(&self) -> Vec<u8> {
        let mut pkg = ZipPackage::new();

        // 1. [Content_Types].xml
        pkg.add_file_str(
            "[Content_Types].xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
  <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#,
        );

        // 2. _rels/.rels
        pkg.add_file_str(
            "_rels/.rels",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"#,
        );

        // 3. word/document.xml
        pkg.add_file_str("word/document.xml", &self.generate_document_xml());

        // 4. word/_rels/document.xml.rels
        pkg.add_file_str(
            "word/_rels/document.xml.rels",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
</Relationships>"#,
        );

        // 5. docProps/core.xml
        let now = chrono::Utc::now().to_rfc3339();
        let core_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dc:title>{}</dc:title>
  <dc:creator>{}</dc:creator>
  <cp:lastModifiedBy>{}</cp:lastModifiedBy>
  <dcterms:created xsi:type="dcterms:W3CDTF">{}</dcterms:created>
  <dcterms:modified xsi:type="dcterms:W3CDTF">{}</dcterms:modified>
</cp:coreProperties>"#,
            html_escape(&self.title),
            html_escape(&self.author),
            html_escape(&self.author),
            now,
            now
        );
        pkg.add_file_str("docProps/core.xml", &core_xml);

        // 6. docProps/app.xml
        pkg.add_file_str(
            "docProps/app.xml",
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties">
  <Application>WidyaOffice Enterprise Suite</Application>
  <AppVersion>2.0.0</AppVersion>
  <Company>Widya Sovereign Computing Group</Company>
</Properties>"#,
        );

        pkg.to_bytes()
    }

    /// Simpan Langsung ke Berkas Fisik `.docx`
    pub fn save_as_docx<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let bytes = self.export_to_docx_bytes();
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    /// Render ke Pratinjau HTML Interaktif Siap Cetak & Ekspor PDF
    pub fn render_html_preview(&self) -> String {
        let mut html = String::from("<!DOCTYPE html><html><head><meta charset='utf-8'>");
        html.push_str("<link rel='preconnect' href='https://fonts.googleapis.com'><link rel='preconnect' href='https://fonts.gstatic.com' crossorigin><link href='https://fonts.googleapis.com/css2?family=Amiri:ital,wght@0,400;0,700;1,400&family=Scheherazade+New:wght@400;700&display=swap' rel='stylesheet'>");
        html.push_str(&format!("<title>{}</title>", self.title));
        html.push_str(r#"<style>
            @media print {
                body { background: none; padding: 0; }
                .page { box-shadow: none; border: none; width: 100%; padding: 0; }
                .btn-print, .security-banner { display: none; }
            }
            body { font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, Tahoma, sans-serif; background: #0f172a; margin: 0; padding: 40px; display: flex; flex-direction: column; align-items: center; }
            .page { background: white; width: 850px; min-height: 1100px; padding: 70px; box-shadow: 0 25px 50px -12px rgba(0,0,0,0.4); border-radius: 8px; position: relative; overflow: hidden; box-sizing: border-box; }
            .watermark { position: absolute; top: 45%; left: 50%; transform: translate(-50%, -50%) rotate(-45deg); font-size: 52pt; font-weight: 900; color: rgba(220, 38, 38, 0.08); pointer-events: none; text-transform: uppercase; white-space: nowrap; z-index: 1; letter-spacing: 6px; }
            h1 { color: #0f172a; border-bottom: 2px solid #0284c7; padding-bottom: 8px; font-size: 22pt; margin-top: 0; }
            h2 { color: #1e293b; font-size: 16pt; margin-top: 24px; }
            h3 { color: #334155; font-size: 13pt; }
            p { line-height: 1.7; color: #334155; font-size: 11pt; }
            p.rtl-paragraph { direction: rtl; text-align: right; font-family: 'Amiri', 'Scheherazade New', 'Traditional Arabic', 'Segoe UI', sans-serif; font-size: 13pt; line-height: 1.9; }
            table { width: 100%; border-collapse: collapse; margin: 20px 0; font-size: 10pt; }
            th, td { border: 1px solid #cbd5e1; padding: 10px 14px; text-align: left; }
            th { background: #f8fafc; font-weight: 700; color: #0f172a; }
            .callout-box { background: #f0f9ff; border-left: 4px solid #0284c7; padding: 14px 18px; border-radius: 4px; margin: 16px 0; }
            .callout-title { font-weight: bold; color: #0369a1; margin-bottom: 4px; }
            .btn-print { background: linear-gradient(135deg, #0284c7, #2563eb); color: white; border: none; padding: 12px 24px; border-radius: 8px; cursor: pointer; margin-bottom: 24px; font-weight: 700; font-size: 11pt; box-shadow: 0 4px 14px rgba(37,99,235,0.4); }
            .tte-seal { margin-top: 50px; border-top: 2px dashed #cbd5e1; padding-top: 20px; display: flex; justify-content: space-between; align-items: flex-end; }
            .tte-badge { border: 1.5px solid #10b981; background: #ecfdf5; border-radius: 8px; padding: 12px 18px; color: #065f46; font-size: 9pt; }
        </style></head><body>"#);

        html.push_str("<button class='btn-print' onclick='window.print()'>🖨️ Cetak Dokumen / Simpan PDF</button>");
        html.push_str("<div class='page'>");

        if let Some(ref wm) = self.watermark {
            html.push_str(&format!("<div class='watermark' style='color: {}; opacity: {};'>{}</div>", wm.color_hex, wm.opacity, wm.text));
        }

        for el in &self.elements {
            match el {
                DocxElement::Heading { level, text } => {
                    html.push_str(&format!("<h{}>{}</h{}>", level, text, level));
                }
                DocxElement::Paragraph { runs, align, is_rtl } => {
                    let mut p_style = String::new();
                    if *is_rtl {
                        p_style.push_str("direction:rtl;text-align:right;font-family:'Amiri', 'Scheherazade New', serif;font-size:13pt;line-height:1.9;");
                    } else if align != "left" {
                        p_style.push_str(&format!("text-align:{};", align));
                    }
                    if p_style.is_empty() {
                        html.push_str("<p>");
                    } else {
                        html.push_str(&format!("<p style='{}'>", p_style));
                    }
                    for r in runs {
                        let mut style = String::new();
                        if r.style.bold { style.push_str("font-weight:bold;"); }
                        if r.style.italic { style.push_str("font-style:italic;"); }
                        if r.style.font_size_pt > 0.0 { style.push_str(&format!("font-size:{}pt;", r.style.font_size_pt)); }
                        if !r.style.font_family.is_empty() { style.push_str(&format!("font-family:'{}', serif;", r.style.font_family)); }
                        if let Some(ref c) = r.style.color_hex { style.push_str(&format!("color:{};", c)); }
                        html.push_str(&format!("<span style='{}'>{}</span>", style, r.text));
                    }
                    html.push_str("</p>");
                }
                DocxElement::Table { rows, has_header, alignments } => {
                    html.push_str("<table>");
                    for (i, row) in rows.iter().enumerate() {
                        html.push_str("<tr>");
                        for (c_idx, cell) in row.iter().enumerate() {
                            let tag = if i == 0 && *has_header { "th" } else { "td" };
                            let align_style = match alignments.get(c_idx) {
                                Some(crate::office::markdown::ColumnAlignment::Center) => "text-align: center;",
                                Some(crate::office::markdown::ColumnAlignment::Right) => "text-align: right;",
                                _ => "text-align: left;",
                            };
                            html.push_str(&format!("<{} style='{}'>", tag, align_style));
                            let runs = parse_markdown_runs(cell);
                            for r in runs {
                                let mut style = String::new();
                                if r.style.bold { style.push_str("font-weight:bold;"); }
                                if r.style.italic { style.push_str("font-style:italic;"); }
                                if r.style.is_monospace { style.push_str("font-family:'Courier New', monospace;"); }
                                if let Some(ref c) = r.style.color_hex { style.push_str(&format!("color:{};", c)); }
                                html.push_str(&format!("<span style='{}'>{}</span>", style, html_escape(&r.text)));
                            }
                            html.push_str(&format!("</{}>", tag));
                        }
                        html.push_str("</tr>");
                    }
                    html.push_str("</table>");
                }
                DocxElement::BulletItem { runs, .. } => {
                    html.push_str("<ul><li>");
                    for r in runs {
                        let mut style = String::new();
                        if r.style.bold { style.push_str("font-weight:bold;"); }
                        if r.style.italic { style.push_str("font-style:italic;"); }
                        if r.style.is_monospace { style.push_str("font-family:'Courier New', monospace;"); }
                        if let Some(ref c) = r.style.color_hex { style.push_str(&format!("color:{};", c)); }
                        html.push_str(&format!("<span style='{}'>{}</span>", style, html_escape(&r.text)));
                    }
                    html.push_str("</li></ul>");
                }
                DocxElement::NumberedItem { number, runs, indent_level } => {
                    let pad = 20 + (*indent_level as usize * 16);
                    html.push_str(&format!(r#"<div style="margin-left: {}px; margin-bottom: 4px; display: flex;"><span style="min-width: 22px; font-weight: bold; color: #0284c7;">{}. </span><div>"#, pad, number));
                    for r in runs {
                        let mut style = String::new();
                        if r.style.bold { style.push_str("font-weight:bold;"); }
                        if r.style.italic { style.push_str("font-style:italic;"); }
                        if r.style.is_monospace { style.push_str("font-family:'Courier New', monospace;"); }
                        if let Some(ref c) = r.style.color_hex { style.push_str(&format!("color:{};", c)); }
                        html.push_str(&format!("<span style='{}'>{}</span>", style, html_escape(&r.text)));
                    }
                    html.push_str("</div></div>");
                }
                DocxElement::Callout { title, text, border_color } => {
                    html.push_str(&format!(
                        "<div class='callout-box' style='border-left-color: {};'><div class='callout-title'>{}</div><div>{}</div></div>",
                        border_color, title, text
                    ));
                }
                DocxElement::PageBreak => {
                    html.push_str("<hr style='margin: 40px 0; border: 1px dashed #94a3b8;' />");
                }
                DocxElement::Divider => {
                    html.push_str("<hr style='margin: 20px 0; border: none; border-top: 1px solid #cbd5e1;' />");
                }
                DocxElement::Diagram { raw_mermaid } => {
                    html.push_str(&format!(
                        "<div class='mermaid-container' style='margin: 20px 0; text-align: center;'><pre class='mermaid'>\n{}\n</pre></div>",
                        html_escape(raw_mermaid)
                    ));
                }
                DocxElement::Footnote { label, text } => {
                    html.push_str(&format!(
                        "<div class='footnote' style='border-top: 1px solid #cbd5e1; margin-top: 12px; padding-top: 4px; font-size: 8.5pt; color: #475569;'><sup style='color: #0369a1;'>[{}]</sup> {}</div>",
                        html_escape(label), html_escape(text)
                    ));
                }
                DocxElement::TableOfContents => {
                    html.push_str("<div class='toc-box' style='background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 6px; padding: 16px; margin: 20px 0;'><h3 style='margin-top: 0; color: #0f172a;'>Daftar Isi</h3></div>");
                }
            }
        }

        // Tampilkan TTE jika ada
        if !self.digital_signatures.is_empty() {
            html.push_str("<div class='tte-seal'>");
            for sig in &self.digital_signatures {
                html.push_str(&format!(
                    r#"<div class='tte-badge'>
                        <div><b>🛡️ TERTANDA SECARA ELEKTRONIK (TTE)</b></div>
                        <div>Penandatangan: <b>{}</b> ({})</div>
                        <div>Instansi: {}</div>
                        <div>Waktu: {}</div>
                        <div style='font-family: monospace; font-size: 7.5pt; color: #047857;'>Segel SHA256: {}</div>
                    </div>"#,
                    sig.signer_name, sig.signer_role, sig.signer_organization, sig.timestamp, &sig.signature_hex[..16]
                ));
            }
            html.push_str("</div>");
        }

        html.push_str("</div></body></html>");
        html
    }
}

fn replace_placeholders(text: &str, variables: &HashMap<String, String>) -> String {
    let mut res = text.to_string();
    for (k, v) in variables {
        let pattern = format!("{{{{{}}}}}", k);
        res = res.replace(&pattern, v);
    }
    res
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Mengurai sintaks inline Markdown (**bold**, *italic*, ***bold italic***, `code`, [link](url), ~~strike~~)
/// menjadi struktur `Vec<TextRun>` berformat nyata untuk dokumen Word & PDF
pub fn parse_markdown_runs(text: &str) -> Vec<TextRun> {
    if text.is_empty() {
        return Vec::new();
    }
    parse_spans_internal(text, InlineSpanStyle::default())
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct InlineSpanStyle {
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    monospace: bool,
    is_link: bool,
    link_url: Option<String>,
}

impl InlineSpanStyle {
    fn to_text_style(&self) -> TextStyle {
        TextStyle {
            bold: self.bold,
            italic: self.italic,
            underline: self.underline || self.is_link,
            strikethrough: self.strikethrough,
            font_size_pt: 10.0,
            color_hex: if self.is_link {
                Some("#0284c7".to_string())
            } else if self.monospace {
                Some("#0f766e".to_string())
            } else {
                None
            },
            font_family: if self.monospace {
                "Courier New".to_string()
            } else {
                "Calibri".to_string()
            },
            is_rtl: false,
            is_monospace: self.monospace,
            link_url: self.link_url.clone(),
        }
    }
}

fn parse_spans_internal(input: &str, current_style: InlineSpanStyle) -> Vec<TextRun> {
    let mut runs: Vec<TextRun> = Vec::new();
    let mut plain_buf = String::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    let flush_buf = |buf: &mut String, runs: &mut Vec<TextRun>, style: &InlineSpanStyle| {
        if !buf.is_empty() {
            runs.push(TextRun {
                text: std::mem::take(buf),
                style: style.to_text_style(),
            });
        }
    };

    while i < len {
        // 1. Gambar Markdown: ![alt](url) -> [Gambar: alt]
        if chars[i] == '!' && i + 1 < len && chars[i + 1] == '[' {
            if let Some(bracket_end) = chars[i + 2..].iter().position(|&c| c == ']') {
                let abs_b = i + 2 + bracket_end;
                if abs_b + 1 < len && chars[abs_b + 1] == '(' {
                    if let Some(paren_end) = chars[abs_b + 2..].iter().position(|&c| c == ')') {
                        let abs_p = abs_b + 2 + paren_end;
                        flush_buf(&mut plain_buf, &mut runs, &current_style);
                        let alt: String = chars[i + 2..abs_b].iter().collect();
                        let label = if alt.trim().is_empty() {
                            "[Gambar]".to_string()
                        } else {
                            format!("[Gambar: {}]", alt.trim())
                        };
                        runs.push(TextRun {
                            text: label,
                            style: current_style.to_text_style(),
                        });
                        i = abs_p + 1;
                        continue;
                    }
                }
            }
        }

        // 2. Tautan Markdown: [text](url) -> text dengan style link
        if chars[i] == '[' {
            if let Some(bracket_end) = chars[i + 1..].iter().position(|&c| c == ']') {
                let abs_b = i + 1 + bracket_end;
                if abs_b + 1 < len && chars[abs_b + 1] == '(' {
                    if let Some(paren_end) = chars[abs_b + 2..].iter().position(|&c| c == ')') {
                        let abs_p = abs_b + 2 + paren_end;
                        flush_buf(&mut plain_buf, &mut runs, &current_style);
                        let link_text: String = chars[i + 1..abs_b].iter().collect();
                        let link_url: String = chars[abs_b + 2..abs_p].iter().collect();
                        let mut sub_style = current_style.clone();
                        sub_style.is_link = true;
                        sub_style.link_url = Some(link_url);
                        let sub_runs = parse_spans_internal(&link_text, sub_style);
                        runs.extend(sub_runs);
                        i = abs_p + 1;
                        continue;
                    }
                }
            }
        }

        // 3. Inline Code: `code`
        if chars[i] == '`' {
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == '`') {
                let abs_end = i + 1 + end;
                flush_buf(&mut plain_buf, &mut runs, &current_style);
                let code_text: String = chars[i + 1..abs_end].iter().collect();
                let mut code_style = current_style.clone();
                code_style.monospace = true;
                runs.push(TextRun {
                    text: code_text,
                    style: code_style.to_text_style(),
                });
                i = abs_end + 1;
                continue;
            }
        }

        // 4. Bold + Italic: ***text***
        if chars[i] == '*' && i + 2 < len && chars[i + 1] == '*' && chars[i + 2] == '*' {
            let mut found_end = None;
            for j in (i + 3)..len {
                if chars[j] == '*' && j + 2 < len && chars[j + 1] == '*' && chars[j + 2] == '*' {
                    found_end = Some(j);
                    break;
                }
            }
            if let Some(end_idx) = found_end {
                flush_buf(&mut plain_buf, &mut runs, &current_style);
                let inner: String = chars[i + 3..end_idx].iter().collect();
                let mut next_style = current_style.clone();
                next_style.bold = true;
                next_style.italic = true;
                runs.extend(parse_spans_internal(&inner, next_style));
                i = end_idx + 3;
                continue;
            }
        }

        // 5. Bold: **text**
        if chars[i] == '*' && i + 1 < len && chars[i + 1] == '*' {
            let mut found_end = None;
            for j in (i + 2)..len {
                if chars[j] == '*' && j + 1 < len && chars[j + 1] == '*' {
                    found_end = Some(j);
                    break;
                }
            }
            if let Some(end_idx) = found_end {
                flush_buf(&mut plain_buf, &mut runs, &current_style);
                let inner: String = chars[i + 2..end_idx].iter().collect();
                let mut next_style = current_style.clone();
                next_style.bold = true;
                runs.extend(parse_spans_internal(&inner, next_style));
                i = end_idx + 2;
                continue;
            }
        }

        // 6. Bold Underscore: __text__
        if chars[i] == '_' && i + 1 < len && chars[i + 1] == '_' {
            let mut found_end = None;
            for j in (i + 2)..len {
                if chars[j] == '_' && j + 1 < len && chars[j + 1] == '_' {
                    found_end = Some(j);
                    break;
                }
            }
            if let Some(end_idx) = found_end {
                flush_buf(&mut plain_buf, &mut runs, &current_style);
                let inner: String = chars[i + 2..end_idx].iter().collect();
                let mut next_style = current_style.clone();
                next_style.bold = true;
                runs.extend(parse_spans_internal(&inner, next_style));
                i = end_idx + 2;
                continue;
            }
        }

        // 7. Strikethrough: ~~text~~
        if chars[i] == '~' && i + 1 < len && chars[i + 1] == '~' {
            let mut found_end = None;
            for j in (i + 2)..len {
                if chars[j] == '~' && j + 1 < len && chars[j + 1] == '~' {
                    found_end = Some(j);
                    break;
                }
            }
            if let Some(end_idx) = found_end {
                flush_buf(&mut plain_buf, &mut runs, &current_style);
                let inner: String = chars[i + 2..end_idx].iter().collect();
                let mut next_style = current_style.clone();
                next_style.strikethrough = true;
                runs.extend(parse_spans_internal(&inner, next_style));
                i = end_idx + 2;
                continue;
            }
        }

        // 8. Italic Asterisk: *text* (tidak diawali/diakhiri spasi, bukan '*')
        if chars[i] == '*' && i + 1 < len && chars[i + 1] != ' ' && chars[i + 1] != '*' {
            let mut found_end = None;
            for j in (i + 2)..len {
                if chars[j] == '*' && chars[j - 1] != ' ' {
                    if j + 1 < len && chars[j + 1] == '*' {
                        continue;
                    }
                    found_end = Some(j);
                    break;
                }
            }
            if let Some(end_idx) = found_end {
                flush_buf(&mut plain_buf, &mut runs, &current_style);
                let inner: String = chars[i + 1..end_idx].iter().collect();
                let mut next_style = current_style.clone();
                next_style.italic = true;
                runs.extend(parse_spans_internal(&inner, next_style));
                i = end_idx + 1;
                continue;
            }
        }

        // 9. Italic Underscore: _text_ (hindari identifier variabel seperti pure_emitter)
        let prev_is_alnum = if i > 0 { chars[i - 1].is_alphanumeric() } else { false };
        if !prev_is_alnum && chars[i] == '_' && i + 1 < len && chars[i + 1] != ' ' && chars[i + 1] != '_' {
            let mut found_end = None;
            for j in (i + 2)..len {
                if chars[j] == '_' && chars[j - 1] != ' ' {
                    let next_is_alnum = if j + 1 < len { chars[j + 1].is_alphanumeric() } else { false };
                    if !next_is_alnum {
                        found_end = Some(j);
                        break;
                    }
                }
            }
            if let Some(end_idx) = found_end {
                flush_buf(&mut plain_buf, &mut runs, &current_style);
                let inner: String = chars[i + 1..end_idx].iter().collect();
                let mut next_style = current_style.clone();
                next_style.italic = true;
                runs.extend(parse_spans_internal(&inner, next_style));
                i = end_idx + 1;
                continue;
            }
        }

        // 10. Tag HTML format: <b>, <strong>, <i>, <em>, <code>
        let remaining: String = chars[i..].iter().collect();
        if remaining.starts_with("<b>") || remaining.starts_with("<strong>") {
            let tag_len = if remaining.starts_with("<b>") { 3 } else { 8 };
            let close_tag = if remaining.starts_with("<b>") { "</b>" } else { "</strong>" };
            if let Some(end_pos) = remaining[tag_len..].find(close_tag) {
                flush_buf(&mut plain_buf, &mut runs, &current_style);
                let inner = &remaining[tag_len..tag_len + end_pos];
                let mut next_style = current_style.clone();
                next_style.bold = true;
                runs.extend(parse_spans_internal(inner, next_style));
                let advance = tag_len + end_pos + close_tag.len();
                i += advance;
                continue;
            }
        }
        if remaining.starts_with("<i>") || remaining.starts_with("<em>") {
            let tag_len = if remaining.starts_with("<i>") { 3 } else { 4 };
            let close_tag = if remaining.starts_with("<i>") { "</i>" } else { "</em>" };
            if let Some(end_pos) = remaining[tag_len..].find(close_tag) {
                flush_buf(&mut plain_buf, &mut runs, &current_style);
                let inner = &remaining[tag_len..tag_len + end_pos];
                let mut next_style = current_style.clone();
                next_style.italic = true;
                runs.extend(parse_spans_internal(inner, next_style));
                let advance = tag_len + end_pos + close_tag.len();
                i += advance;
                continue;
            }
        }
        if remaining.starts_with("<code>") {
            if let Some(end_pos) = remaining[6..].find("</code>") {
                flush_buf(&mut plain_buf, &mut runs, &current_style);
                let inner = &remaining[6..6 + end_pos];
                let mut code_style = current_style.clone();
                code_style.monospace = true;
                runs.push(TextRun {
                    text: inner.to_string(),
                    style: code_style.to_text_style(),
                });
                let advance = 6 + end_pos + 7;
                i += advance;
                continue;
            }
        }

        // Karakter biasa
        plain_buf.push(chars[i]);
        i += 1;
    }

    flush_buf(&mut plain_buf, &mut runs, &current_style);

    // Gabungkan run berurutan dengan style persis sama
    merge_adjacent_runs(runs)
}

fn merge_adjacent_runs(runs: Vec<TextRun>) -> Vec<TextRun> {
    let mut merged: Vec<TextRun> = Vec::with_capacity(runs.len());
    for r in runs {
        if r.text.is_empty() {
            continue;
        }
        if let Some(last) = merged.last_mut() {
            if last.style.bold == r.style.bold
                && last.style.italic == r.style.italic
                && last.style.underline == r.style.underline
                && last.style.strikethrough == r.style.strikethrough
                && last.style.is_monospace == r.style.is_monospace
                && last.style.color_hex == r.style.color_hex
                && last.style.font_family == r.style.font_family
                && last.style.font_size_pt == r.style.font_size_pt
            {
                last.text.push_str(&r.text);
                continue;
            }
        }
        merged.push(r);
    }
    merged
}
