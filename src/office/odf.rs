//! Pure Sovereign OpenDocument Format (ODF) Engine (odf.rs)
//!
//! Implementasi 100% mandiri format dokumen standar ISO/IEC 26300 (OASIS OpenDocument v1.2/1.3)
//! untuk kepatuhan standar format terbuka nasional (Perpres & Kebijakan Dokumen Publik):
//! - 📝 OpenDocument Text (.odt): Dokumen teks, judul (H1-H6), paragraf, gaya teks, tabel
//! - 📊 OpenDocument Spreadsheet (.ods): Lembar kerja, baris, sel numerik/teks/mata uang
//! - 📽️ OpenDocument Presentation (.odp): Slide presentasi, bingkai teks, kotak judul & peluru
//! - 📦 Kontainer ZIP Standar ODF: Berkas `mimetype` tanpa kompresi pada offset 0,
//!   `META-INF/manifest.xml`, `content.xml`, `styles.xml`, dan `meta.xml`.

use std::fs;
use std::path::{Path, PathBuf};
use crate::office::zip::ZipPackage;

pub const ODF_MIME_TEXT: &str = "application/vnd.oasis.opendocument.text";
pub const ODF_MIME_SPREADSHEET: &str = "application/vnd.oasis.opendocument.spreadsheet";
pub const ODF_MIME_PRESENTATION: &str = "application/vnd.oasis.opendocument.presentation";

// ============================================================================
// 1. ELEMEN DOKUMEN ODF TEXT (.ODT)
// ============================================================================

#[derive(Debug, Clone)]
pub enum OdtBlock {
    Heading { level: u8, text: String },
    Paragraph(String),
    BoldParagraph(String),
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
}

#[derive(Debug, Clone, Default)]
pub struct OdtDocument {
    pub title: String,
    pub author: String,
    pub blocks: Vec<OdtBlock>,
}

impl OdtDocument {
    pub fn baru(title: &str) -> Self {
        Self {
            title: title.to_string(),
            author: "WidyaOffice Sovereign Suite".to_string(),
            blocks: Vec::new(),
        }
    }

    pub fn dengan_penulis(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }

    pub fn tambah_judul(&mut self, level: u8, text: &str) {
        self.blocks.push(OdtBlock::Heading {
            level: level.clamp(1, 6),
            text: text.to_string(),
        });
    }

    pub fn tambah_paragraf(&mut self, text: &str) {
        self.blocks.push(OdtBlock::Paragraph(text.to_string()));
    }

    pub fn tambah_paragraf_tebal(&mut self, text: &str) {
        self.blocks.push(OdtBlock::BoldParagraph(text.to_string()));
    }

    pub fn tambah_tabel(&mut self, headers: Vec<&str>, rows: Vec<Vec<&str>>) {
        let h: Vec<String> = headers.into_iter().map(|s| s.to_string()).collect();
        let r: Vec<Vec<String>> = rows
            .into_iter()
            .map(|row| row.into_iter().map(|s| s.to_string()).collect())
            .collect();
        self.blocks.push(OdtBlock::Table { headers: h, rows: r });
    }

    /// Serialisasi dokumen teks ke format biner .odt
    pub fn tulis_ke_bytes(&self) -> Result<Vec<u8>, String> {
        let mut zip = ZipPackage::new();

        // 1. mimetype (Wajib entry pertama tanpa kompresi)
        zip.add_file("mimetype", ODF_MIME_TEXT.as_bytes());

        // 2. META-INF/manifest.xml
        let manifest = OdfManifestBuilder::bangun_manifest(ODF_MIME_TEXT);
        zip.add_file("META-INF/manifest.xml", manifest.as_bytes());

        // 3. meta.xml
        let meta = OdfMetaBuilder::bangun_meta(&self.title, &self.author);
        zip.add_file("meta.xml", meta.as_bytes());

        // 4. styles.xml
        let styles = OdfStylesBuilder::bangun_styles();
        zip.add_file("styles.xml", styles.as_bytes());

        // 5. content.xml
        let mut body_xml = String::new();
        for block in &self.blocks {
            match block {
                OdtBlock::Heading { level, text } => {
                    body_xml.push_str(&format!(
                        r#"<text:h text:outline-level="{}">{}</text:h>"#,
                        level, escape_xml(text)
                    ));
                }
                OdtBlock::Paragraph(text) => {
                    body_xml.push_str(&format!(
                        r#"<text:p text:style-name="Standard">{}</text:p>"#,
                        escape_xml(text)
                    ));
                }
                OdtBlock::BoldParagraph(text) => {
                    body_xml.push_str(&format!(
                        r#"<text:p text:style-name="Standard"><text:span text:style-name="Bold">{}</text:span></text:p>"#,
                        escape_xml(text)
                    ));
                }
                OdtBlock::Table { headers, rows } => {
                    body_xml.push_str(r#"<table:table table:name="Table1">"#);
                    // Header row
                    if !headers.is_empty() {
                        body_xml.push_str(r#"<table:table-header-rows><table:table-row>"#);
                        for h in headers {
                            body_xml.push_str(&format!(
                                r#"<table:table-cell office:value-type="string"><text:p><text:span text:style-name="Bold">{}</text:span></text:p></table:table-cell>"#,
                                escape_xml(h)
                            ));
                        }
                        body_xml.push_str(r#"</table:table-row></table:table-header-rows>"#);
                    }
                    // Data rows
                    for r in rows {
                        body_xml.push_str(r#"<table:table-row>"#);
                        for c in r {
                            body_xml.push_str(&format!(
                                r#"<table:table-cell office:value-type="string"><text:p>{}</text:p></table:table-cell>"#,
                                escape_xml(c)
                            ));
                        }
                        body_xml.push_str(r#"</table:table-row>"#);
                    }
                    body_xml.push_str(r#"</table:table>"#);
                }
            }
        }

        let content_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-content xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0" xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0" xmlns:fo="urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0" office:version="1.2">
  <office:automatic-styles>
    <style:style xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0" style:name="Bold" style:family="text">
      <style:text-properties fo:font-weight="bold"/>
    </style:style>
  </office:automatic-styles>
  <office:body>
    <office:text>
      {}
    </office:text>
  </office:body>
</office:document-content>"#,
            body_xml
        );

        zip.add_file("content.xml", content_xml.as_bytes());

        Ok(zip.to_bytes())
    }

    pub fn ekspor_berkas(&self, path: &Path) -> Result<PathBuf, String> {
        let bytes = self.tulis_ke_bytes()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Gagal membuat direktori {:?}: {}", parent, e))?;
        }
        fs::write(path, &bytes)
            .map_err(|e| format!("Gagal menulis berkas ODT ke {:?}: {}", path, e))?;
        Ok(path.to_path_buf())
    }
}

// ============================================================================
// 2. ELEMEN DOKUMEN ODF SPREADSHEET (.ODS)
// ============================================================================

#[derive(Debug, Clone)]
pub enum OdsCellValue {
    String(String),
    Number(f64),
    Currency(f64),
}

#[derive(Debug, Clone, Default)]
pub struct OdsSheet {
    pub name: String,
    pub rows: Vec<Vec<OdsCellValue>>,
}

#[derive(Debug, Clone, Default)]
pub struct OdsDocument {
    pub title: String,
    pub author: String,
    pub sheets: Vec<OdsSheet>,
}

impl OdsDocument {
    pub fn baru(title: &str) -> Self {
        Self {
            title: title.to_string(),
            author: "WidyaOffice Sovereign Suite".to_string(),
            sheets: Vec::new(),
        }
    }

    pub fn tambah_sheet(&mut self, name: &str) -> &mut OdsSheet {
        self.sheets.push(OdsSheet {
            name: name.to_string(),
            rows: Vec::new(),
        });
        self.sheets.last_mut().unwrap()
    }

    pub fn tulis_ke_bytes(&self) -> Result<Vec<u8>, String> {
        let mut zip = ZipPackage::new();
        zip.add_file("mimetype", ODF_MIME_SPREADSHEET.as_bytes());

        let manifest = OdfManifestBuilder::bangun_manifest(ODF_MIME_SPREADSHEET);
        zip.add_file("META-INF/manifest.xml", manifest.as_bytes());

        let meta = OdfMetaBuilder::bangun_meta(&self.title, &self.author);
        zip.add_file("meta.xml", meta.as_bytes());

        let styles = OdfStylesBuilder::bangun_styles();
        zip.add_file("styles.xml", styles.as_bytes());

        let mut sheets_xml = String::new();
        for sheet in &self.sheets {
            sheets_xml.push_str(&format!(
                r#"<table:table table:name="{}">"#,
                escape_xml(&sheet.name)
            ));
            for row in &sheet.rows {
                sheets_xml.push_str(r#"<table:table-row>"#);
                for cell in row {
                    match cell {
                        OdsCellValue::String(s) => {
                            sheets_xml.push_str(&format!(
                                r#"<table:table-cell office:value-type="string"><text:p>{}</text:p></table:table-cell>"#,
                                escape_xml(s)
                            ));
                        }
                        OdsCellValue::Number(num) => {
                            sheets_xml.push_str(&format!(
                                r#"<table:table-cell office:value-type="float" office:value="{}"><text:p>{}</text:p></table:table-cell>"#,
                                num, num
                            ));
                        }
                        OdsCellValue::Currency(cur) => {
                            sheets_xml.push_str(&format!(
                                r#"<table:table-cell office:value-type="currency" office:currency="IDR" office:value="{}"><text:p>Rp {}</text:p></table:table-cell>"#,
                                cur, cur
                            ));
                        }
                    }
                }
                sheets_xml.push_str(r#"</table:table-row>"#);
            }
            sheets_xml.push_str(r#"</table:table>"#);
        }

        let content_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-content xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0" xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0" office:version="1.2">
  <office:body>
    <office:spreadsheet>
      {}
    </office:spreadsheet>
  </office:body>
</office:document-content>"#,
            sheets_xml
        );

        zip.add_file("content.xml", content_xml.as_bytes());
        Ok(zip.to_bytes())
    }

    pub fn ekspor_berkas(&self, path: &Path) -> Result<PathBuf, String> {
        let bytes = self.tulis_ke_bytes()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Gagal membuat direktori {:?}: {}", parent, e))?;
        }
        fs::write(path, &bytes)
            .map_err(|e| format!("Gagal menulis berkas ODS ke {:?}: {}", path, e))?;
        Ok(path.to_path_buf())
    }
}

// ============================================================================
// 3. ELEMEN DOKUMEN ODF PRESENTATION (.ODP)
// ============================================================================

#[derive(Debug, Clone)]
pub struct OdpSlide {
    pub title: String,
    pub bullet_points: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct OdpDocument {
    pub title: String,
    pub author: String,
    pub slides: Vec<OdpSlide>,
}

impl OdpDocument {
    pub fn baru(title: &str) -> Self {
        Self {
            title: title.to_string(),
            author: "WidyaOffice Sovereign Suite".to_string(),
            slides: Vec::new(),
        }
    }

    pub fn tambah_slide(&mut self, title: &str, bullets: &[&str]) {
        self.slides.push(OdpSlide {
            title: title.to_string(),
            bullet_points: bullets.iter().map(|s| s.to_string()).collect(),
        });
    }

    pub fn tulis_ke_bytes(&self) -> Result<Vec<u8>, String> {
        let mut zip = ZipPackage::new();
        zip.add_file("mimetype", ODF_MIME_PRESENTATION.as_bytes());

        let manifest = OdfManifestBuilder::bangun_manifest(ODF_MIME_PRESENTATION);
        zip.add_file("META-INF/manifest.xml", manifest.as_bytes());

        let meta = OdfMetaBuilder::bangun_meta(&self.title, &self.author);
        zip.add_file("meta.xml", meta.as_bytes());

        let styles = OdfStylesBuilder::bangun_styles();
        zip.add_file("styles.xml", styles.as_bytes());

        let mut slides_xml = String::new();
        for (idx, slide) in self.slides.iter().enumerate() {
            let mut list_items = String::new();
            for bullet in &slide.bullet_points {
                list_items.push_str(&format!(
                    r#"<text:list-item><text:p>• {}</text:p></text:list-item>"#,
                    escape_xml(bullet)
                ));
            }

            slides_xml.push_str(&format!(
                r#"<draw:page draw:name="Slide_{}">
  <draw:frame draw:name="TitleFrame" svg:width="25cm" svg:height="3cm" svg:x="1cm" svg:y="1cm">
    <draw:text-box>
      <text:h text:outline-level="1">{}</text:h>
    </draw:text-box>
  </draw:frame>
  <draw:frame draw:name="BodyFrame" svg:width="25cm" svg:height="12cm" svg:x="1cm" svg:y="4cm">
    <draw:text-box>
      <text:list>
        {}
      </text:list>
    </draw:text-box>
  </draw:frame>
</draw:page>"#,
                idx + 1,
                escape_xml(&slide.title),
                list_items
            ));
        }

        let content_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-content xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0" xmlns:draw="urn:oasis:names:tc:opendocument:xmlns:drawing:1.0" xmlns:svg="urn:oasis:names:tc:opendocument:xmlns:svg-compatible:1.0" office:version="1.2">
  <office:body>
    <office:presentation>
      {}
    </office:presentation>
  </office:body>
</office:document-content>"#,
            slides_xml
        );

        zip.add_file("content.xml", content_xml.as_bytes());
        Ok(zip.to_bytes())
    }

    pub fn ekspor_berkas(&self, path: &Path) -> Result<PathBuf, String> {
        let bytes = self.tulis_ke_bytes()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Gagal membuat direktori {:?}: {}", parent, e))?;
        }
        fs::write(path, &bytes)
            .map_err(|e| format!("Gagal menulis berkas ODP ke {:?}: {}", path, e))?;
        Ok(path.to_path_buf())
    }
}

// ============================================================================
// 4. BUILDER COMMON METADATA & MANIFEST ODF
// ============================================================================

struct OdfManifestBuilder;
impl OdfManifestBuilder {
    fn bangun_manifest(main_mimetype: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<manifest:manifest xmlns:manifest="urn:oasis:names:tc:opendocument:xmlns:manifest:1.0" manifest:version="1.2">
  <manifest:file-entry manifest:full-path="/" manifest:media-type="{}"/>
  <manifest:file-entry manifest:full-path="content.xml" manifest:media-type="text/xml"/>
  <manifest:file-entry manifest:full-path="styles.xml" manifest:media-type="text/xml"/>
  <manifest:file-entry manifest:full-path="meta.xml" manifest:media-type="text/xml"/>
</manifest:manifest>"#,
            main_mimetype
        )
    }
}

struct OdfMetaBuilder;
impl OdfMetaBuilder {
    fn bangun_meta(title: &str, author: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-meta xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:meta="urn:oasis:names:tc:opendocument:xmlns:meta:1.0" office:version="1.2">
  <office:meta>
    <dc:title>{}</dc:title>
    <dc:creator>{}</dc:creator>
    <dc:date>2026-10-10T12:00:00Z</dc:date>
    <meta:generator>WidyaOffice Sovereign Suite v2.6.0</meta:generator>
  </office:meta>
</office:document-meta>"#,
            escape_xml(title), escape_xml(author)
        )
    }
}

struct OdfStylesBuilder;
impl OdfStylesBuilder {
    fn bangun_styles() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-styles xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0" xmlns:fo="urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0" office:version="1.2">
  <office:styles>
    <style:default-style style:family="paragraph">
      <style:text-properties fo:font-size="12pt" fo:font-family="Liberation Sans, Arial, sans-serif"/>
    </style:default-style>
  </office:styles>
</office:document-styles>"#.to_string()
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
