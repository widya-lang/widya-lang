use serde::{Deserialize, Serialize};

/// AST Blok Markdown GFM
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkdownBlock {
    Heading { level: u8, text: String },
    Paragraph(String),
    CodeBlock { lang: String, code: String },
    Blockquote(String),
    UnorderedList(Vec<String>),
    Checklist(Vec<(bool, String)>), // (checked, text)
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
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

            // 6. Table (| Col1 | Col2 |)
            if line.starts_with('|') && line.ends_with('|') {
                let headers: Vec<String> = line.split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();
                i += 1; // Skip header delimiter (|---|---|)
                if i < lines.len() && lines[i].contains("---") {
                    i += 1;
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
                blocks.push(MarkdownBlock::Table { headers, rows });
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
                MarkdownBlock::Checklist(items) => {
                    html.push_str("<ul class='task-list'>\n");
                    for (checked, text) in items {
                        let chk = if *checked { "checked" } else { "" };
                        html.push_str(&format!("  <li><input type='checkbox' {} disabled/> {}</li>\n", chk, parse_inline_styles(text)));
                    }
                    html.push_str("</ul>\n");
                }
                MarkdownBlock::Table { headers, rows } => {
                    html.push_str("<table><thead><tr>\n");
                    for h in headers { html.push_str(&format!("  <th>{}</th>\n", parse_inline_styles(h))); }
                    html.push_str("</tr></thead><tbody>\n");
                    for r in rows {
                        html.push_str("<tr>\n");
                        for c in r { html.push_str(&format!("  <td>{}</td>\n", parse_inline_styles(c))); }
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
    let mut s = html_escape(text);
    // Bold **text**
    while let Some(start) = s.find("**") {
        if let Some(end_offset) = s[start + 2..].find("**") {
            let real_end = start + 2 + end_offset;
            let inner = s[start + 2..real_end].to_string();
            s = format!("{}<strong>{}</strong>{}", &s[..start], inner, &s[real_end + 2..]);
        } else {
            break;
        }
    }
    // Inline Code `code`
    while let Some(start) = s.find('`') {
        if let Some(end_offset) = s[start + 1..].find('`') {
            let real_end = start + 1 + end_offset;
            let inner = s[start + 1..real_end].to_string();
            s = format!("{}<code>{}</code>{}", &s[..start], inner, &s[real_end + 1..]);
        } else {
            break;
        }
    }
    s
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
