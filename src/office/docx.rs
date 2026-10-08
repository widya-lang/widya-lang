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
        }
    }

    pub fn bold() -> Self {
        let mut s = Self::new();
        s.bold = true;
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
    Table { rows: Vec<Vec<String>>, has_header: bool },
    PageBreak,
    Divider,
    BulletItem { text: String, indent_level: u8 },
    Callout { title: String, text: String, border_color: String },
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
                },
            }],
            align: "right".to_string(),
            is_rtl: true,
        });
    }

    pub fn add_table(&mut self, rows: Vec<Vec<String>>, has_header: bool) {
        self.elements.push(DocxElement::Table { rows, has_header });
    }

    pub fn add_bullet(&mut self, text: &str) {
        self.elements.push(DocxElement::BulletItem {
            text: text.to_string(),
            indent_level: 0,
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
                DocxElement::BulletItem { text, .. } => {
                    *text = replace_placeholders(text, variables);
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
                DocxElement::BulletItem { text, .. } => {
                    *text = OfficeSecurity::redact_pdp_sensitive_text(text);
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
                DocxElement::BulletItem { text, .. } => {
                    out.push_str("• ");
                    out.push_str(text);
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
                DocxElement::Table { rows, .. } => {
                    xml.push_str("<w:tbl><w:tblPr><w:tblBorders><w:top w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/><w:left w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/><w:bottom w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/><w:right w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/><w:insideH w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/><w:insideV w:val=\"single\" w:sz=\"4\" w:space=\"0\" w:color=\"auto\"/></w:tblBorders></w:tblPr>");
                    for row in rows {
                        xml.push_str("<w:tr>");
                        for cell in row {
                            xml.push_str(&format!("<w:tc><w:p><w:r><w:t>{}</w:t></w:r></w:p></w:tc>", html_escape(cell)));
                        }
                        xml.push_str("</w:tr>");
                    }
                    xml.push_str("</w:tbl>");
                }
                DocxElement::BulletItem { text, .. } => {
                    xml.push_str(&format!(
                        r#"<w:p><w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr><w:r><w:t>{}</w:t></w:r></w:p>"#,
                        html_escape(text)
                    ));
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
                DocxElement::Table { rows, has_header } => {
                    html.push_str("<table>");
                    for (i, row) in rows.iter().enumerate() {
                        html.push_str("<tr>");
                        for cell in row {
                            if i == 0 && *has_header {
                                html.push_str(&format!("<th>{}</th>", cell));
                            } else {
                                html.push_str(&format!("<td>{}</td>", cell));
                            }
                        }
                        html.push_str("</tr>");
                    }
                    html.push_str("</table>");
                }
                DocxElement::BulletItem { text, .. } => {
                    html.push_str(&format!("<ul><li>{}</li></ul>", text));
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
