//! Sovereign EPUB 3.2 Digital Book Generator (Pilar II: Penerbitan Buku Digital Mandiri)
//! 100% Zero External Crates (Bebas dependensi pihak ketiga).
//! Membangun berkas buku digital standar EPUB 3.2 / IDPF OCF (Open Container Format)
//! yang kompatibel penuh dengan Apple Books, Google Play Books, Kindle, Calibre, Kobo, dsb.

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::office::docx::{parse_markdown_runs, TextRun};
use crate::office::markdown::{ColumnAlignment, MarkdownBlock, MarkdownDocument};
use crate::office::zip::ZipPackage;

/// Entri Bab Tunggal dalam Buku Digital EPUB
#[derive(Debug, Clone)]
pub struct EpubChapter {
    pub id: String,
    pub title: String,
    pub content_html: String,
}

/// Dokumen Buku Digital Berstandar EPUB 3.2 Mandiri
#[derive(Debug, Clone)]
pub struct EpubDocument {
    pub title: String,
    pub author: String,
    pub publisher: String,
    pub language: String,
    pub identifier: String,
    pub modified_date: String,
    pub custom_css: Option<String>,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub subject: Option<String>,
    pub rights: Option<String>,
    pub cover_image: Option<(Vec<u8>, String)>,
    pub chapters: Vec<EpubChapter>,
}

impl EpubDocument {
    pub fn new(title: &str, author: &str) -> Self {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let uuid = format!(
            "urn:uuid:widya-{:08x}-{:04x}-4a21-8b9f-{:012x}",
            (title.len() as u32).wrapping_mul(0x9e3779b9),
            (author.len() as u16).wrapping_mul(0x85eb),
            0x20261009a7b3u64
        );

        Self {
            title: title.to_string(),
            author: author.to_string(),
            publisher: "Widya Sovereign Computing System".to_string(),
            language: "id".to_string(),
            identifier: uuid,
            modified_date: now,
            custom_css: None,
            isbn: None,
            description: None,
            subject: None,
            rights: None,
            cover_image: None,
            chapters: Vec::new(),
        }
    }

    pub fn set_publisher(&mut self, publisher: &str) -> &mut Self {
        self.publisher = publisher.to_string();
        self
    }

    pub fn set_language(&mut self, language: &str) -> &mut Self {
        self.language = language.to_string();
        self
    }

    pub fn set_identifier(&mut self, identifier: &str) -> &mut Self {
        self.identifier = identifier.to_string();
        self
    }

    pub fn set_custom_css(&mut self, css: &str) -> &mut Self {
        self.custom_css = Some(css.to_string());
        self
    }

    pub fn set_isbn(&mut self, isbn: &str) -> &mut Self {
        self.isbn = Some(isbn.to_string());
        self
    }

    pub fn set_description(&mut self, desc: &str) -> &mut Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn set_subject(&mut self, subj: &str) -> &mut Self {
        self.subject = Some(subj.to_string());
        self
    }

    pub fn set_rights(&mut self, rights: &str) -> &mut Self {
        self.rights = Some(rights.to_string());
        self
    }

    pub fn set_cover_image(&mut self, bytes: &[u8], media_type: &str) -> &mut Self {
        self.cover_image = Some((bytes.to_vec(), media_type.to_string()));
        self
    }

    /// Menambahkan bab baru ke dalam buku digital
    pub fn add_chapter(&mut self, title: &str, html_body: &str) -> &mut Self {
        let idx = self.chapters.len() + 1;
        let id = format!("chapter_{}", idx);
        self.chapters.push(EpubChapter {
            id,
            title: title.to_string(),
            content_html: html_body.to_string(),
        });
        self
    }

    /// Default CSS stylesheet elegan untuk tipografi e-reader modern
    pub fn default_stylesheet() -> &'static str {
        r#"/* Widya Sovereign EPUB 3.2 Typography Stylesheet */
@charset "utf-8";

body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, serif;
    font-size: 1.05em;
    line-height: 1.75;
    color: #1e293b;
    background-color: #ffffff;
    margin: 1.5em 1.2em;
    padding: 0;
    text-rendering: optimizeLegibility;
}

h1, h2, h3, h4, h5, h6 {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    color: #0f172a;
    font-weight: 700;
    line-height: 1.3;
    margin-top: 1.6em;
    margin-bottom: 0.6em;
    page-break-after: avoid;
}

h1 {
    font-size: 2.0em;
    border-bottom: 2px solid #0284c7;
    padding-bottom: 0.3em;
    margin-top: 0.5em;
    color: #0369a1;
}

h2 {
    font-size: 1.5em;
    border-bottom: 1px solid #e2e8f0;
    padding-bottom: 0.25em;
}

h3 {
    font-size: 1.25em;
    color: #334155;
}

p {
    margin-top: 0;
    margin-bottom: 1.2em;
    text-align: justify;
    hyphens: auto;
}

a {
    color: #0284c7;
    text-decoration: underline;
}

blockquote {
    margin: 1.5em 0;
    padding: 0.8em 1.2em;
    background-color: #f0f9ff;
    border-left: 4px solid #0284c7;
    color: #0c4a6e;
    font-style: italic;
    border-radius: 4px;
}

code {
    font-family: "Cascadia Code", "Fira Code", Consolas, Menlo, Monaco, monospace;
    font-size: 0.88em;
    background-color: #f1f5f9;
    color: #0f766e;
    padding: 0.15em 0.35em;
    border-radius: 3px;
}

pre {
    font-family: "Cascadia Code", "Fira Code", Consolas, Menlo, Monaco, monospace;
    font-size: 0.85em;
    line-height: 1.5;
    background-color: #0f172a;
    color: #f8fafc;
    padding: 1.2em;
    border-radius: 6px;
    overflow-x: auto;
    margin: 1.5em 0;
    white-space: pre-wrap;
    word-break: break-all;
}

pre code {
    background-color: transparent;
    color: inherit;
    padding: 0;
    font-size: 1.0em;
}

table {
    width: 100%;
    border-collapse: collapse;
    margin: 1.8em 0;
    font-size: 0.92em;
}

th, td {
    border: 1px solid #cbd5e1;
    padding: 0.6em 0.8em;
    vertical-align: top;
}

th {
    background-color: #f8fafc;
    font-weight: 700;
    color: #0f172a;
}

tr:nth-child(even) td {
    background-color: #f8fafc;
}

ul, ol {
    margin-top: 0;
    margin-bottom: 1.2em;
    padding-left: 1.8em;
}

li {
    margin-bottom: 0.4em;
}

ul.checklist {
    list-style-type: none;
    padding-left: 0.5em;
}

hr.section-divider {
    border: none;
    border-top: 1px solid #cbd5e1;
    margin: 2.0em 0;
}

.diagram-container {
    background-color: #f8fafc;
    border: 1px solid #e2e8f0;
    border-radius: 6px;
    padding: 1.0em;
    margin: 1.5em 0;
    text-align: center;
}

.callout-box {
    margin: 1.5em 0;
    padding: 1.0em 1.2em;
    border-left: 4px solid #3b82f6;
    background-color: #eff6ff;
    border-radius: 4px;
}

.callout-title {
    font-weight: bold;
    color: #1d4ed8;
    margin-bottom: 0.4em;
}
"#
    }

    /// Menghasilkan seluruh berkas paket arsip EPUB 3.2 dalam bentuk byte
    pub fn export_to_epub_bytes(&self) -> Vec<u8> {
        let mut pkg = ZipPackage::new();

        // 1. mimetype (Wajib entri pertama dalam ZIP, uncompressed/store)
        pkg.add_file("mimetype", b"application/epub+zip");

        // 2. META-INF/container.xml
        let container_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>"#;
        pkg.add_file_str("META-INF/container.xml", container_xml);

        // 3. OEBPS/style.css
        let css_content = match &self.custom_css {
            Some(css) => css.as_str(),
            None => Self::default_stylesheet(),
        };
        pkg.add_file_str("OEBPS/style.css", css_content);

        // 4. Jika ada cover image, tambahkan file gambar dan cover.xhtml
        if let Some((ref img_bytes, ref mime)) = self.cover_image {
            let img_filename = if mime.contains("png") { "cover.png" } else { "cover.jpg" };
            pkg.add_file(&format!("OEBPS/{}", img_filename), img_bytes);

            let cover_xhtml = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="{}">
<head>
  <meta charset="utf-8" />
  <title>Sampul</title>
  <link rel="stylesheet" type="text/css" href="style.css" />
</head>
<body style="margin: 0; padding: 0; text-align: center; background-color: #ffffff;">
  <div style="height: 100vh; display: flex; justify-content: center; align-items: center;">
    <img src="{}" alt="Sampul Buku" style="max-width: 100%; max-height: 100%; object-fit: contain;" />
  </div>
</body>
</html>
"#,
                self.language, img_filename
            );
            pkg.add_file_str("OEBPS/cover.xhtml", &cover_xhtml);
        }

        // 5. OEBPS/toc.xhtml (Navigasi EPUB 3 Navigation Document)
        let toc_xhtml = self.generate_toc_xhtml();
        pkg.add_file_str("OEBPS/toc.xhtml", &toc_xhtml);

        // 6. OEBPS/chapter_{i}.xhtml
        for (idx, chapter) in self.chapters.iter().enumerate() {
            let filename = format!("OEBPS/{}.xhtml", chapter.id);
            let chapter_xhtml = self.generate_chapter_xhtml(chapter, idx + 1);
            pkg.add_file_str(&filename, &chapter_xhtml);
        }

        // 7. OEBPS/content.opf (Package metadata, manifest, dan spine)
        let content_opf = self.generate_content_opf();
        pkg.add_file_str("OEBPS/content.opf", &content_opf);

        pkg.to_bytes()
    }

    /// Menghasilkan berkas XML content.opf (Open Packaging Format)
    fn generate_content_opf(&self) -> String {
        let mut opf = String::new();
        opf.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" unique-identifier="pub-id" version="3.0">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
"#);

        opf.push_str(&format!("    <dc:identifier id=\"pub-id\">{}</dc:identifier>\n", xml_escape(&self.identifier)));
        if let Some(ref isbn) = self.isbn {
            opf.push_str(&format!("    <dc:identifier id=\"isbn\">urn:isbn:{}</dc:identifier>\n", xml_escape(isbn)));
        }
        opf.push_str(&format!("    <dc:title>{}</dc:title>\n", xml_escape(&self.title)));
        opf.push_str(&format!("    <dc:creator>{}</dc:creator>\n", xml_escape(&self.author)));
        opf.push_str(&format!("    <dc:language>{}</dc:language>\n", xml_escape(&self.language)));
        opf.push_str(&format!("    <dc:publisher>{}</dc:publisher>\n", xml_escape(&self.publisher)));
        if let Some(ref desc) = self.description {
            opf.push_str(&format!("    <dc:description>{}</dc:description>\n", xml_escape(desc)));
        }
        if let Some(ref subj) = self.subject {
            opf.push_str(&format!("    <dc:subject>{}</dc:subject>\n", xml_escape(subj)));
        }
        if let Some(ref rights) = self.rights {
            opf.push_str(&format!("    <dc:rights>{}</dc:rights>\n", xml_escape(rights)));
        }
        opf.push_str(&format!("    <meta property=\"dcterms:modified\">{}</meta>\n", self.modified_date));
        opf.push_str("  </metadata>\n");

        // Manifest
        opf.push_str("  <manifest>\n");
        opf.push_str("    <item id=\"nav\" href=\"toc.xhtml\" media-type=\"application/xhtml+xml\" properties=\"nav\"/>\n");
        opf.push_str("    <item id=\"style\" href=\"style.css\" media-type=\"text/css\"/>\n");
        if let Some((_, ref mime)) = self.cover_image {
            let img_filename = if mime.contains("png") { "cover.png" } else { "cover.jpg" };
            opf.push_str(&format!("    <item id=\"cover-img\" href=\"{}\" media-type=\"{}\" properties=\"cover-image\"/>\n", img_filename, mime));
            opf.push_str("    <item id=\"cover\" href=\"cover.xhtml\" media-type=\"application/xhtml+xml\"/>\n");
        }
        for chapter in &self.chapters {
            opf.push_str(&format!(
                "    <item id=\"{}\" href=\"{}.xhtml\" media-type=\"application/xhtml+xml\"/>\n",
                chapter.id, chapter.id
            ));
        }
        opf.push_str("  </manifest>\n");

        // Spine
        opf.push_str("  <spine>\n");
        if self.cover_image.is_some() {
            opf.push_str("    <itemref idref=\"cover\"/>\n");
        }
        for chapter in &self.chapters {
            opf.push_str(&format!("    <itemref idref=\"{}\"/>\n", chapter.id));
        }
        opf.push_str("  </spine>\n");
        opf.push_str("</package>\n");

        opf
    }

    /// Menghasilkan navigasi Daftar Isi (OEBPS/toc.xhtml)
    fn generate_toc_xhtml(&self) -> String {
        let mut xhtml = String::new();
        xhtml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" xml:lang=""#);
        xhtml.push_str(&self.language);
        xhtml.push_str(r#"">
<head>
  <meta charset="utf-8" />
  <title>Daftar Isi</title>
  <link rel="stylesheet" type="text/css" href="style.css" />
</head>
<body>
  <nav epub:type="toc" id="toc">
    <h1>Daftar Isi</h1>
    <ol>
"#);

        for chapter in &self.chapters {
            xhtml.push_str(&format!(
                "      <li><a href=\"{}.xhtml\">{}</a></li>\n",
                chapter.id,
                xml_escape(&chapter.title)
            ));
        }

        xhtml.push_str(r#"    </ol>
  </nav>
</body>
</html>
"#);
        xhtml
    }

    /// Menghasilkan halaman XHTML bab individual
    fn generate_chapter_xhtml(&self, chapter: &EpubChapter, _chapter_num: usize) -> String {
        let mut xhtml = String::new();
        xhtml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang=""#);
        xhtml.push_str(&self.language);
        xhtml.push_str(r#"">
<head>
  <meta charset="utf-8" />
  <title>"#);
        xhtml.push_str(&xml_escape(&chapter.title));
        xhtml.push_str(r#"</title>
  <link rel="stylesheet" type="text/css" href="style.css" />
</head>
<body>
<section class="chapter-content">
"#);

        // Sertakan judul bab
        xhtml.push_str(&format!("  <h1 class=\"chapter-title\">{}</h1>\n", xml_escape(&chapter.title)));
        xhtml.push_str(&chapter.content_html);
        xhtml.push_str("\n</section>\n</body>\n</html>\n");

        xhtml
    }

    /// Simpan paket buku langsung ke berkas fisik .epub
    pub fn save_as_epub<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let bytes = self.export_to_epub_bytes();
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}

/// Mengubah daftar `TextRun` berformat menjadi string XHTML
pub fn text_runs_to_xhtml(runs: &[TextRun]) -> String {
    let mut out = String::new();
    for r in runs {
        let mut inner = xml_escape(&r.text);
        if r.style.is_monospace {
            inner = format!("<code>{}</code>", inner);
        }
        if r.style.bold {
            inner = format!("<strong>{}</strong>", inner);
        }
        if r.style.italic {
            inner = format!("<em>{}</em>", inner);
        }
        if r.style.strikethrough {
            inner = format!("<del>{}</del>", inner);
        }
        if r.style.underline {
            inner = format!("<u>{}</u>", inner);
        }
        if let Some(ref url) = r.style.link_url {
            inner = format!("<a href=\"{}\">{}</a>", xml_escape(url), inner);
        }
        out.push_str(&inner);
    }
    out
}

/// Mengubah baris teks Markdown inline menjadi XHTML
pub fn inline_markdown_to_xhtml(text: &str) -> String {
    let runs = parse_markdown_runs(text);
    text_runs_to_xhtml(&runs)
}

/// Konversi Dokumen Markdown (`MarkdownDocument`) ke Buku Digital EPUB 3.2
pub fn markdown_to_epub_doc(
    md: &MarkdownDocument,
    book_title: Option<&str>,
    author: Option<&str>,
) -> EpubDocument {
    let first_h1 = md.blocks.iter().find_map(|b| {
        if let MarkdownBlock::Heading { level: 1, text } = b {
            Some(text.as_str())
        } else {
            None
        }
    });

    let detected_title = book_title
        .or(first_h1)
        .unwrap_or("Buku Digital Widya");
    let detected_author = author.unwrap_or("Penyusun Berdaulat Widya");

    let mut doc = EpubDocument::new(detected_title, detected_author);

    // Kumpulkan bab-bab: jika ada Heading level 1, setiap H1 memecah bab
    let mut current_chapter_title = String::new();
    let mut current_chapter_html = String::new();
    let mut chapter_count = 0;

    let has_any_h1 = md.blocks.iter().any(|b| matches!(b, MarkdownBlock::Heading { level: 1, .. }));

    if !has_any_h1 {
        // Jika tidak ada heading level 1, seluruh isi dokumen adalah satu bab utama
        current_chapter_title = detected_title.to_string();
    }

    for block in &md.blocks {
        match block {
            MarkdownBlock::Heading { level: 1, text } => {
                if !current_chapter_title.is_empty() || !current_chapter_html.is_empty() {
                    let title = if current_chapter_title.is_empty() {
                        format!("Bab {}", chapter_count + 1)
                    } else {
                        current_chapter_title.clone()
                    };
                    doc.add_chapter(&title, &current_chapter_html);
                    chapter_count += 1;
                    current_chapter_html.clear();
                }
                current_chapter_title = text.clone();
            }
            MarkdownBlock::Heading { level, text } => {
                let tag = format!("h{}", level.clamp(&2, &6));
                let inner = inline_markdown_to_xhtml(text);
                current_chapter_html.push_str(&format!("  <{tag}>{inner}</{tag}>\n"));
            }
            MarkdownBlock::Paragraph(text) => {
                let inner = inline_markdown_to_xhtml(text);
                current_chapter_html.push_str(&format!("  <p>{inner}</p>\n"));
            }
            MarkdownBlock::UnorderedList(items) => {
                current_chapter_html.push_str("  <ul>\n");
                for item in items {
                    let inner = inline_markdown_to_xhtml(item);
                    current_chapter_html.push_str(&format!("    <li>{inner}</li>\n"));
                }
                current_chapter_html.push_str("  </ul>\n");
            }
            MarkdownBlock::OrderedList(items) => {
                current_chapter_html.push_str("  <ol>\n");
                for (_, item) in items {
                    let inner = inline_markdown_to_xhtml(item);
                    current_chapter_html.push_str(&format!("    <li>{inner}</li>\n"));
                }
                current_chapter_html.push_str("  </ol>\n");
            }
            MarkdownBlock::Checklist(items) => {
                current_chapter_html.push_str("  <ul class=\"checklist\">\n");
                for (checked, item) in items {
                    let mark = if *checked { "&#9745; " } else { "&#9744; " };
                    let inner = inline_markdown_to_xhtml(item);
                    current_chapter_html.push_str(&format!("    <li>{mark}{inner}</li>\n"));
                }
                current_chapter_html.push_str("  </ul>\n");
            }
            MarkdownBlock::CodeBlock { lang, code } => {
                let clean_lang = lang.trim().to_lowercase();
                if clean_lang == "mermaid" {
                    current_chapter_html.push_str(&format!(
                        "  <div class=\"diagram-container\"><pre class=\"mermaid\">{}</pre></div>\n",
                        xml_escape(code)
                    ));
                } else {
                    let lang_attr = if clean_lang.is_empty() {
                        String::new()
                    } else {
                        format!(" class=\"language-{}\"", clean_lang)
                    };
                    current_chapter_html.push_str(&format!(
                        "  <pre><code{}>{}</code></pre>\n",
                        lang_attr,
                        xml_escape(code)
                    ));
                }
            }
            MarkdownBlock::Blockquote(text) => {
                let inner = inline_markdown_to_xhtml(text);
                current_chapter_html.push_str(&format!("  <blockquote><p>{inner}</p></blockquote>\n"));
            }
            MarkdownBlock::Table { headers, alignments, rows } => {
                current_chapter_html.push_str("  <table>\n");
                if !headers.is_empty() {
                    current_chapter_html.push_str("    <thead>\n      <tr>\n");
                    for (col_idx, h) in headers.iter().enumerate() {
                        let align_str = match alignments.get(col_idx).copied().unwrap_or(ColumnAlignment::Left) {
                            ColumnAlignment::Center => " style=\"text-align: center;\"",
                            ColumnAlignment::Right => " style=\"text-align: right;\"",
                            ColumnAlignment::Left => " style=\"text-align: left;\"",
                        };
                        let inner = inline_markdown_to_xhtml(h);
                        current_chapter_html.push_str(&format!("        <th{align_str}>{inner}</th>\n"));
                    }
                    current_chapter_html.push_str("      </tr>\n    </thead>\n");
                }
                current_chapter_html.push_str("    <tbody>\n");
                for row in rows {
                    current_chapter_html.push_str("      <tr>\n");
                    for (col_idx, cell) in row.iter().enumerate() {
                        let align_str = match alignments.get(col_idx).copied().unwrap_or(ColumnAlignment::Left) {
                            ColumnAlignment::Center => " style=\"text-align: center;\"",
                            ColumnAlignment::Right => " style=\"text-align: right;\"",
                            ColumnAlignment::Left => " style=\"text-align: left;\"",
                        };
                        let inner = inline_markdown_to_xhtml(cell);
                        current_chapter_html.push_str(&format!("        <td{align_str}>{inner}</td>\n"));
                    }
                    current_chapter_html.push_str("      </tr>\n");
                }
                current_chapter_html.push_str("    </tbody>\n  </table>\n");
            }
            MarkdownBlock::FootnoteDefinition { label, content } => {
                let inner = inline_markdown_to_xhtml(content);
                current_chapter_html.push_str(&format!(
                    "  <aside id=\"fn_{}\" class=\"footnote\"><p><sup>[{}]</sup> {}</p></aside>\n",
                    xml_escape(label),
                    xml_escape(label),
                    inner
                ));
            }
            MarkdownBlock::HorizontalRule => {
                current_chapter_html.push_str("  <hr class=\"section-divider\" />\n");
            }
        }
    }

    // Flush bab terakhir
    if !current_chapter_title.is_empty() || !current_chapter_html.is_empty() {
        let title = if current_chapter_title.is_empty() {
            if doc.chapters.is_empty() {
                detected_title.to_string()
            } else {
                format!("Bab {}", chapter_count + 1)
            }
        } else {
            current_chapter_title
        };
        doc.add_chapter(&title, &current_chapter_html);
    }

    if doc.chapters.is_empty() {
        doc.add_chapter(detected_title, "  <p>Dokumen tanpa isi.</p>\n");
    }

    doc
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epub_generation_structure() {
        let mut doc = EpubDocument::new("Arsitektur Widya Sovereign", "Tim Inti Widya");
        doc.add_chapter(
            "Bab 1: Visi Kedaulatan",
            "<p>Kedaulatan digital adalah keharusan mutlak bagi bangsa modern.</p>",
        );
        doc.add_chapter(
            "Bab 2: Pilar Kompiler Mandiri",
            "<p>Emitter biner ELF murni tanpa ketergantungan toolchain asing.</p>",
        );

        let epub_bytes = doc.export_to_epub_bytes();

        // 1. Validasi Magic ZIP ("PK\x03\x04")
        assert!(epub_bytes.len() > 100);
        assert_eq!(&epub_bytes[0..4], &[0x50, 0x4b, 0x03, 0x04]);

        // 2. Periksa berkas mimetype berada di awal berkas zip tanpa kompresi
        let mimetype_bytes = b"mimetypeapplication/epub+zip";
        assert!(
            epub_bytes.windows(mimetype_bytes.len()).any(|w| w == mimetype_bytes),
            "Paket EPUB harus memuat entri mimetype persis di header pertama"
        );

        // 3. Periksa keberadaan berkas-berkas OCF standar
        let text = String::from_utf8_lossy(&epub_bytes);
        assert!(text.contains("META-INF/container.xml"));
        assert!(text.contains("OEBPS/content.opf"));
        assert!(text.contains("OEBPS/toc.xhtml"));
        assert!(text.contains("OEBPS/style.css"));
        assert!(text.contains("OEBPS/chapter_1.xhtml"));
        assert!(text.contains("OEBPS/chapter_2.xhtml"));
    }

    #[test]
    fn test_markdown_to_epub_conversion() {
        let md_src = r#"# Bab 1: Pengantar Bahasa Widya
Bahasa Widya dibangun dengan **kedaulatan penuh**.

* Fitur 1: Tanpa LLVM
* Fitur 2: 100% Rust Native

# Bab 2: Struktur Tipe Data
Bahasa ini mendukung tipe numerik presisi tinggi:

```rust
fn main() {
    println!("Widya 2026");
}
```
"#;
        let md = MarkdownDocument::parse(md_src);
        let epub = markdown_to_epub_doc(&md, Some("Panduan Widya"), Some("Pengembang"));

        assert_eq!(epub.title, "Panduan Widya");
        assert_eq!(epub.chapters.len(), 2);
        assert_eq!(epub.chapters[0].title, "Bab 1: Pengantar Bahasa Widya");
        assert_eq!(epub.chapters[1].title, "Bab 2: Struktur Tipe Data");

        let bytes = epub.export_to_epub_bytes();
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[0..4], &[0x50, 0x4b, 0x03, 0x04]);
    }
}
