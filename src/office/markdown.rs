use serde::{Deserialize, Serialize};

/// Perataan Kolom Tabel Markdown
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnAlignment {
    Left,
    Center,
    Right,
}

/// AST Blok Markdown GFM
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkdownBlock {
    Heading { level: u8, text: String },
    Paragraph(String),
    CodeBlock { lang: String, code: String },
    Blockquote(String),
    UnorderedList(Vec<String>),
    OrderedList(Vec<(usize, String)>),
    Checklist(Vec<(bool, String)>), // (checked, text)
    Table {
        headers: Vec<String>,
        alignments: Vec<ColumnAlignment>,
        rows: Vec<Vec<String>>,
    },
    HorizontalRule,
}

/// Dokumen Markdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownDocument {
    pub raw_text: String,
    pub blocks: Vec<MarkdownBlock>,
}

impl MarkdownDocument {
    pub fn parse(content: &str) -> Self {
        let mut blocks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() {
                i += 1;
                continue;
            }

            // 1. Heading (# Heading)
            if line.starts_with('#') {
                let level = line.chars().take_while(|c| *c == '#').count() as u8;
                let text = line.trim_start_matches('#').trim().to_string();
                blocks.push(MarkdownBlock::Heading { level, text });
                i += 1;
                continue;
            }

            // 2. Horizontal Rule (--- atau ***)
            if line == "---" || line == "***" || line == "___" {
                blocks.push(MarkdownBlock::HorizontalRule);
                i += 1;
                continue;
            }

            // 3. Fenced Code Block (```)
            if line.starts_with("```") {
                let lang = line.trim_start_matches("```").trim().to_string();
                let mut code_lines = Vec::new();
                i += 1;
                while i < lines.len() && !lines[i].trim().starts_with("```") {
                    code_lines.push(lines[i]);
                    i += 1;
                }
                if i < lines.len() { i += 1; } // Skip closing ```
                blocks.push(MarkdownBlock::CodeBlock {
                    lang,
                    code: code_lines.join("\n"),
                });
                continue;
            }

            // 4. Blockquote (> quote)
            if line.starts_with('>') {
                let text = line.trim_start_matches('>').trim().to_string();
                blocks.push(MarkdownBlock::Blockquote(text));
                i += 1;
                continue;
            }

            // 5. Checklist (- [ ] atau - [x])
            if line.starts_with("- [ ] ") || line.starts_with("- [x] ") || line.starts_with("- [X] ") {
                let mut check_items = Vec::new();
                while i < lines.len() {
                    let l = lines[i].trim();
                    if l.starts_with("- [x] ") || l.starts_with("- [X] ") {
                        check_items.push((true, l[6..].trim().to_string()));
                    } else if l.starts_with("- [ ] ") {
                        check_items.push((false, l[6..].trim().to_string()));
                    } else {
                        break;
                    }
                    i += 1;
                }
                blocks.push(MarkdownBlock::Checklist(check_items));
                continue;
            }

            // 5b. Unordered List (- item, * item, + item)
            if (line.starts_with("- ") || line.starts_with("* ") || line.starts_with("+ "))
                && !line.starts_with("---") && !line.starts_with("***") {
                let mut list_items = Vec::new();
                while i < lines.len() {
                    let l = lines[i].trim();
                    if (l.starts_with("- ") || l.starts_with("* ") || l.starts_with("+ "))
                        && !l.starts_with("- [ ] ") && !l.starts_with("- [x] ") && !l.starts_with("- [X] ")
                        && !l.starts_with("---") && !l.starts_with("***") {
                        list_items.push(l[2..].trim().to_string());
                        i += 1;
                    } else {
                        break;
                    }
                }
                blocks.push(MarkdownBlock::UnorderedList(list_items));
                continue;
            }

            // 5c. Ordered List (1. item, 2. item)
            let is_digit_dot = |s: &str| -> Option<(usize, String)> {
                let bytes = s.as_bytes();
                let mut num_end = 0;
                while num_end < bytes.len() && bytes[num_end].is_ascii_digit() {
                    num_end += 1;
                }
                if num_end > 0 && num_end < bytes.len() && bytes[num_end] == b'.' {
                    if num_end + 1 < bytes.len() && bytes[num_end + 1] == b' ' {
                        if let Ok(num) = s[..num_end].parse::<usize>() {
                            return Some((num, s[num_end + 2..].trim().to_string()));
                        }
                    }
                }
                None
            };

            if let Some((num, text)) = is_digit_dot(line) {
                let mut list_items = vec![(num, text)];
                i += 1;
                while i < lines.len() {
                    let l = lines[i].trim();
                    if let Some((n, t)) = is_digit_dot(l) {
                        list_items.push((n, t));
                        i += 1;
                    } else {
                        break;
                    }
                }
                blocks.push(MarkdownBlock::OrderedList(list_items));
                continue;
            }

            // 6. Table (| Col1 | Col2 |)
            if line.starts_with('|') && line.ends_with('|') {
                let headers: Vec<String> = line.split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();
                i += 1; // Skip header delimiter (|---|---|)
                let mut alignments = Vec::new();
                if i < lines.len() && lines[i].contains("---") {
                    let align_parts: Vec<&str> = lines[i].split('|')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .collect();
                    for p in align_parts {
                        let left_colon = p.starts_with(':');
                        let right_colon = p.ends_with(':');
                        if left_colon && right_colon {
                            alignments.push(ColumnAlignment::Center);
                        } else if right_colon {
                            alignments.push(ColumnAlignment::Right);
                        } else {
                            alignments.push(ColumnAlignment::Left);
                        }
                    }
                    i += 1;
                }
                while alignments.len() < headers.len() {
                    alignments.push(ColumnAlignment::Left);
                }
                let mut rows = Vec::new();
                while i < lines.len() && lines[i].trim().starts_with('|') {
                    let row: Vec<String> = lines[i].split('|')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect();
                    rows.push(row);
                    i += 1;
                }
                blocks.push(MarkdownBlock::Table { headers, alignments, rows });
                continue;
            }

            // 7. Regular Paragraph
            blocks.push(MarkdownBlock::Paragraph(line.to_string()));
            i += 1;
        }

        Self {
            raw_text: content.to_string(),
            blocks,
        }
    }

    /// Render ke Dokumen HTML5 Bersih & Elegan
    pub fn to_html(&self) -> String {
        let mut html = String::from("<div class='widya-markdown'>\n");
        for b in &self.blocks {
            match b {
                MarkdownBlock::Heading { level, text } => {
                    html.push_str(&format!("<h{}>{}</h{}>\n", level, parse_inline_styles(text), level));
                }
                MarkdownBlock::Paragraph(p) => {
                    html.push_str(&format!("<p>{}</p>\n", parse_inline_styles(p)));
                }
                MarkdownBlock::CodeBlock { lang, code } => {
                    html.push_str(&format!("<pre><code class='language-{}'>{}</code></pre>\n", lang, html_escape(code)));
                }
                MarkdownBlock::Blockquote(q) => {
                    html.push_str(&format!("<blockquote>{}</blockquote>\n", parse_inline_styles(q)));
                }
                MarkdownBlock::UnorderedList(items) => {
                    html.push_str("<ul>\n");
                    for it in items { html.push_str(&format!("  <li>{}</li>\n", parse_inline_styles(it))); }
                    html.push_str("</ul>\n");
                }
                MarkdownBlock::OrderedList(items) => {
                    html.push_str("<ol>\n");
                    for (_num, it) in items { html.push_str(&format!("  <li>{}</li>\n", parse_inline_styles(it))); }
                    html.push_str("</ol>\n");
                }
                MarkdownBlock::Checklist(items) => {
                    html.push_str("<ul class='task-list'>\n");
                    for (checked, text) in items {
                        let chk = if *checked { "checked" } else { "" };
                        html.push_str(&format!("  <li><input type='checkbox' {} disabled/> {}</li>\n", chk, parse_inline_styles(text)));
                    }
                    html.push_str("</ul>\n");
                }
                MarkdownBlock::Table { headers, alignments, rows } => {
                    html.push_str("<table><thead><tr>\n");
                    for (idx, h) in headers.iter().enumerate() {
                        let align_css = match alignments.get(idx) {
                            Some(ColumnAlignment::Center) => " style='text-align: center;'",
                            Some(ColumnAlignment::Right) => " style='text-align: right;'",
                            _ => " style='text-align: left;'",
                        };
                        html.push_str(&format!("  <th{}>{}</th>\n", align_css, parse_inline_styles(h)));
                    }
                    html.push_str("</tr></thead><tbody>\n");
                    for r in rows {
                        html.push_str("<tr>\n");
                        for (idx, c) in r.iter().enumerate() {
                            let align_css = match alignments.get(idx) {
                                Some(ColumnAlignment::Center) => " style='text-align: center;'",
                                Some(ColumnAlignment::Right) => " style='text-align: right;'",
                                _ => " style='text-align: left;'",
                            };
                            html.push_str(&format!("  <td{}>{}</td>\n", align_css, parse_inline_styles(c)));
                        }
                        html.push_str("</tr>\n");
                    }
                    html.push_str("</tbody></table>\n");
                }
                MarkdownBlock::HorizontalRule => {
                    html.push_str("<hr />\n");
                }
            }
        }
        html.push_str("</div>\n");
        html
    }
}

fn parse_inline_styles(text: &str) -> String {
    use crate::office::docx::parse_markdown_runs;
    let runs = parse_markdown_runs(text);
    if runs.is_empty() {
        return html_escape(text);
    }
    let mut html = String::new();
    for r in runs {
        let escaped = html_escape(&r.text);
        let mut styled = escaped;
        if r.style.is_monospace {
            styled = format!("<code>{}</code>", styled);
        }
        if r.style.bold && r.style.italic {
            styled = format!("<strong><em>{}</em></strong>", styled);
        } else if r.style.bold {
            styled = format!("<strong>{}</strong>", styled);
        } else if r.style.italic {
            styled = format!("<em>{}</em>", styled);
        }
        if r.style.strikethrough {
            styled = format!("<del>{}</del>", styled);
        }
        if r.style.underline && !r.style.bold {
            styled = format!("<u>{}</u>", styled);
        }
        if let Some(ref color) = r.style.color_hex {
            styled = format!(r#"<span style="color: {};">{}</span>"#, color, styled);
        }
        html.push_str(&styled);
    }
    html
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
