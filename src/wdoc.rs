//! Documentation Generator (WDoc) - Widya Documentation Generator
//! Generates API documentation from source code comments

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// WDoc configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WdocConfig {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub theme: String,
    pub include_private: bool,
    pub generate_examples: bool,
}

/// Documentation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocItem {
    pub kind: String,
    pub name: String,
    pub path: String,
    pub description: String,
    pub signatures: Vec<String>,
    pub examples: Vec<String>,
    pub attributes: HashMap<String, Vec<String>>,
    pub line_start: usize,
    pub line_end: usize,
}

/// WDoc documentation generator
pub struct Wdoc {
    config: WdocConfig,
    items: Vec<DocItem>,
}

impl Wdoc {
    /// Create new WDoc instance
    pub fn new(config: WdocConfig) -> Self {
        Self {
            config,
            items: Vec::new(),
        }
    }

    /// Load documentation from source files
    pub fn load(&mut self, source_path: &Path) -> Result<(), String> {
        let mut content = String::new();
        File::open(source_path)
            .map_err(|e| format!("Failed to open file: {}", e))?
            .read_to_string(&mut content)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        self.parse_comments(&content, source_path);
        
        Ok(())
    }

    /// Parse documentation comments from content
    fn parse_comments(&mut self, content: &str, path: &Path) {
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("///") || line.starts_with("/**") {
                if let Some(item) = self.parse_doc_block(&lines, i, path) {
                    self.items.push(item);
                }
            }
            i += 1;
        }
    }

    /// Parse documentation block
    fn parse_doc_block(&self, lines: &[&str], start: usize, path: &Path) -> Option<DocItem> {
        let mut description = String::new();
        let mut i = start;
        
        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("///") {
                let content = line.strip_prefix("///").unwrap_or(line).trim();
                description.push_str(content);
                description.push(' ');
            } else {
                break;
            }
            i += 1;
        }

        Some(DocItem {
            kind: "Function".to_string(),
            name: "function_name".to_string(),
            path: path.to_str()?.to_string(),
            description: description.trim().to_string(),
            signatures: vec!["function_name()".to_string()],
            examples: Vec::new(),
            attributes: HashMap::new(),
            line_start: start,
            line_end: i,
        })
    }

    /// Generate HTML documentation
    pub fn generate_html(&self, output_dir: &Path) -> Result<(), String> {
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        let index_html = self.generate_index();
        let mut file = File::create(output_dir.join("index.html"))
            .map_err(|e| format!("Failed to create index.html: {}", e))?;
        file.write_all(index_html.as_bytes())
            .map_err(|e| format!("Failed to write index.html: {}", e))?;

        Ok(())
    }

    /// Generate HTML index page with modern interactive UI & search
    fn generate_index(&self) -> String {
        let mut html = String::from(
r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Widya-Lang API & Standard Library Documentation</title>
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;600;700;800&family=JetBrains+Mono:wght@400;500;700&display=swap" rel="stylesheet">
    <style>
        :root {
            --bg-base: #070a14;
            --bg-card: rgba(15, 23, 42, 0.85);
            --border: rgba(255, 255, 255, 0.1);
            --primary: #3b82f6;
            --emerald: #10b981;
            --text-main: #f8fafc;
            --text-muted: #94a3b8;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: 'Plus Jakarta Sans', sans-serif;
            background: var(--bg-base);
            color: var(--text-main);
            padding: 40px 20px;
            min-height: 100vh;
        }
        .container {
            max-width: 1000px;
            margin: 0 auto;
        }
        .header {
            text-align: center;
            margin-bottom: 40px;
        }
        .header h1 {
            font-size: 36px;
            font-weight: 800;
            background: linear-gradient(135deg, #60a5fa, #34d399);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            margin-bottom: 12px;
        }
        .search-box {
            width: 100%;
            padding: 14px 20px;
            font-size: 15px;
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid var(--border);
            border-radius: 12px;
            color: #fff;
            outline: none;
            margin-bottom: 30px;
            transition: border-color 0.2s;
        }
        .search-box:focus {
            border-color: var(--primary);
        }
        .doc-card {
            background: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 20px;
            margin-bottom: 20px;
            backdrop-filter: blur(12px);
            transition: transform 0.2s, border-color 0.2s;
        }
        .doc-card:hover {
            transform: translateY(-2px);
            border-color: rgba(59, 130, 246, 0.4);
        }
        .badge {
            display: inline-block;
            font-size: 11px;
            font-weight: 700;
            text-transform: uppercase;
            padding: 3px 8px;
            border-radius: 6px;
            background: rgba(59, 130, 246, 0.2);
            color: #60a5fa;
            margin-bottom: 8px;
        }
        .item-name {
            font-family: 'JetBrains Mono', monospace;
            font-size: 18px;
            font-weight: 700;
            color: #38bdf8;
            margin-bottom: 8px;
        }
        .description {
            color: var(--text-muted);
            font-size: 14px;
            line-height: 1.6;
            margin-bottom: 12px;
        }
        .signature {
            background: #020617;
            padding: 10px 14px;
            border-radius: 8px;
            font-family: 'JetBrains Mono', monospace;
            font-size: 13px;
            color: #34d399;
            border: 1px solid rgba(255,255,255,0.05);
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Dokumentasi API Resmi Widya-Lang</h1>
            <p style="color: var(--text-muted);">Referensi Lengkap Fungsi, Struktur Data, dan Pustaka Standar Berdaulat</p>
        </div>
        <input type="text" class="search-box" id="search" placeholder="🔍 Cari fungsi, modul, atau kata kunci..." oninput="filterDocs()">
        <div id="doc-list">
"#
        );

        for item in &self.items {
            let sig = item.signatures.first().cloned().unwrap_or_else(|| format!("{}()", item.name));
            html.push_str(&format!(
                r#"<div class="doc-card" data-name="{name}">
                    <span class="badge">{kind}</span>
                    <div class="item-name">{name}</div>
                    <div class="description">{desc}</div>
                    <div class="signature">{sig}</div>
                </div>"#,
                name = item.name,
                kind = item.kind,
                desc = item.description,
                sig = sig
            ));
        }

        html.push_str(
r#"        </div>
    </div>
    <script>
        function filterDocs() {
            const query = document.getElementById('search').value.toLowerCase();
            const cards = document.querySelectorAll('.doc-card');
            cards.forEach(card => {
                const name = card.getAttribute('data-name').toLowerCase();
                const text = card.innerText.toLowerCase();
                if (name.includes(query) || text.includes(query)) {
                    card.style.display = 'block';
                } else {
                    card.style.display = 'none';
                }
            });
        }
    </script>
</body>
</html>"#
        );
        html
    }

    /// Serve HTML documentation via local HTTP server
    pub fn serve_docs(&self, port: u16, output_dir: &Path) -> Result<(), String> {
        self.generate_html(output_dir)?;
        let index_path = output_dir.join("index.html");
        let html_content = fs::read_to_string(&index_path)
            .map_err(|e| format!("Gagal membaca index.html: {}", e))?;

        let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", port))
            .map_err(|e| format!("Gagal mengikat port {}: {}", port, e))?;

        println!("📖 Server Dokumentasi WidyaDoc aktif pada http://localhost:{}", port);
        
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(mut s) = stream {
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        html_content.len(),
                        html_content
                    );
                    let _ = s.write_all(response.as_bytes());
                }
            }
        });

        Ok(())
    }

    /// Generate JSON documentation
    pub fn generate_json(&self) -> String {
        serde_json::to_string_pretty(&self.items).unwrap_or_default()
    }

    /// Get all documented items
    pub fn items(&self) -> &[DocItem] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wdoc_generate() {
        let config = WdocConfig {
            input_dir: PathBuf::from("/tmp/test"),
            output_dir: PathBuf::from("/tmp/docs"),
            theme: "default".to_string(),
            include_private: false,
            generate_examples: true,
        };
        
        let wdoc = Wdoc::new(config);
        let json = wdoc.generate_json();
        assert!(json.contains("["));
    }
}
