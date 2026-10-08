use crate::error::Galat;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use crate::value::Value;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

pub struct WidyaLsp {
    interpreter: Interpreter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub code: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceEdit {
    pub changes: HashMap<String, Vec<TextEdit>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeAction {
    pub title: String,
    pub kind: String,
    pub edit: Option<WorkspaceEdit>,
}

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Keyword = 14,
    Snippet = 15,
}

#[derive(Debug, Clone)]
pub struct Hover {
    pub contents: String,
    pub range: Option<Range>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub range: Range,
    pub signature: Option<String>,
    pub documentation: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Function = 12,
    Variable = 13,
    Constant = 14,
}

impl WidyaLsp {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }
    
    pub fn get_diagnostics(source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        let mut lexer = Lexer::new(source);
        let tokens = match lexer.scan_tokens() {
            Ok(toks) => toks,
            Err(e) => {
                let (line, col, msg) = Self::extract_error_info(&e);
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line, character: col },
                        end: Position { line, character: col + 1 },
                    },
                    message: format!("Sintaks Lexer: {}", msg),
                    severity: DiagnosticSeverity::Error,
                    code: Some("lexer-error".to_string()),
                    source: Some("widya".to_string()),
                });
                return diagnostics;
            }
        };

        let mut parser = Parser::new(tokens);
        if let Err(e) = parser.parse() {
            let (line, col, msg) = Self::extract_error_info(&e);
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line, character: col },
                    end: Position { line, character: col + 1 },
                },
                message: format!("Sintaks Parser: {}", msg),
                severity: DiagnosticSeverity::Error,
                code: Some("parser-error".to_string()),
                source: Some("widya".to_string()),
            });
        }

        diagnostics
    }
    
    pub fn get_completions(&self, position: Position, source: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        
        let keywords = vec![
            ("var", "Deklarasi variabel", "var ${1:nama} = ${2:nilai}"),
            ("fungsi", "Definisi fungsi", "fungsi ${1:nama}(${2:params}) {\n    ${3:// body}\n}"),
            ("jika", "Kondisional if", "jika ${1:kondisi} {\n    ${2:// body}\n}"),
            ("kalau_tidak", "Kondisional else", "kalau_tidak {\n    ${1:// body}\n}"),
            ("kalau_tidak_jika", "Kondisional else if", "kalau_tidak_jika ${1:kondisi} {\n    ${2:// body}\n}"),
            ("untuk", "Perulangan for", "untuk ${1:i} dalam ${2:rentang} {\n    ${3:// body}\n}"),
            ("selama", "Perulangan while", "selama ${1:kondisi} {\n    ${2:// body}\n}"),
            ("kembalikan", "Return statement", "kembalikan ${1:nilai}"),
            ("cetak", "Cetak ke layar", "cetak(${1:nilai})"),
            ("benar", "Nilai boolean true", "benar"),
            ("salah", "Nilai boolean false", "salah"),
            ("kosong", "Nilai null", "kosong"),
            ("dan", "Operator logika AND", "dan"),
            ("atau", "Operator logika OR", "atau"),
            ("bukan", "Operator logika NOT", "bukan"),
        ];
        
        for (keyword, description, snippet) in keywords {
            items.push(CompletionItem {
                label: keyword.to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some(description.to_string()),
                documentation: None,
                insert_text: Some(snippet.to_string()),
            });
        }
        
        let builtins = vec![
            ("cetak", "Mencetak nilai ke layar", "cetak(${1:nilai})"),
            ("panjang", "Menghitung panjang array/string", "panjang(${1:nilai})"),
            ("tipe", "Mendapatkan tipe data", "tipe(${1:nilai})"),
            ("rentang", "Membuat range angka", "rentang(${1:mulai}, ${2:akhir})"),
            ("push", "Menambah elemen ke array", "push(${1:array}, ${2:nilai})"),
            ("pop", "Menghapus elemen terakhir array", "pop(${1:array})"),
            ("struktur_snippet", "✨ [AI Snippet] Buat struktur data baru", "struktur ${1:NamaModel} {\n    ${2:id}: Int,\n    ${3:nama}: String\n}"),
            ("cocokkan_snippet", "✨ [AI Snippet] Pattern matching exhaustif", "cocokkan ${1:target} {\n    ${2:Pola} => ${3:ekspresi},\n    _ => ${4:fallback}\n}"),
            ("aktor_snippet", "✨ [AI Snippet] Aktor asinkron konkurensi", "aktor ${1:NamaAktor} {\n    terima ${2:Pesan} => {\n        ${3:// logika}\n    }\n}"),
            ("widyabase_crud", "✨ [AI Snippet] Eksekusi query WidyaBase ACID", "misal hasil = widyabase.kueri(\"SELECT * FROM ${1:tabel} WHERE ${2:kondisi}\");"),
            ("pqc_shield_sign", "✨ [AI Snippet] Tandatangani payload dengan Post-Quantum ML-DSA", "misal tte = PqcShield::tandatangan(${1:payload}, ${2:kunci_privat});"),
        ];
        
        for (func, description, snippet) in builtins {
            items.push(CompletionItem {
                label: func.to_string(),
                kind: CompletionItemKind::Function,
                detail: Some(description.to_string()),
                documentation: None,
                insert_text: Some(snippet.to_string()),
            });
        }
        
        let vars = self.interpreter.get_global_variables();
        for (name, value) in vars {
            items.push(CompletionItem {
                label: name.clone(),
                kind: CompletionItemKind::Variable,
                detail: Some(format!(": {}", value.type_name())),
                documentation: None,
                insert_text: Some(name),
            });
        }
        
        items
    }
    
    pub fn get_hover(&self, position: Position, source: &str) -> Option<Hover> {
        let word = self.get_word_at_position(position, source)?;
        
        let keywords = vec![
            ("var", "Deklarasi variabel", "var nama = nilai"),
            ("fungsi", "Definisi fungsi", "fungsi nama(params) { ... }"),
            ("jika", "Kondisional if", "jika kondisi { ... }"),
            ("kalau_tidak", "Kondisional else", "kalau_tidak { ... }"),
            ("untuk", "Perulangan for", "untuk i dalam rentang { ... }"),
            ("selama", "Perulangan while", "selama kondisi { ... }"),
            ("kembalikan", "Return statement", "kembalikan nilai"),
            ("cetak", "Fungsi cetak", "cetak(nilai) - Mencetak nilai ke layar"),
        ];
        
        for (keyword, description, example) in keywords {
            if word == keyword {
                return Some(Hover {
                    contents: format!("**{}**\n\n{}\n\n```widya\n{}\n```", 
                        keyword, description, example),
                    range: None,
                });
            }
        }
        
        let vars = self.interpreter.get_global_variables();
        if let Some(value) = vars.get(&word) {
            return Some(Hover {
                contents: format!("**{}** `: {}`\n\nNilai: `{}`", 
                    word, value.type_name(), value.to_debug_repr()),
                range: None,
            });
        }
        
        None
    }
    
    fn get_word_at_position(&self, position: Position, source: &str) -> Option<String> {
        let lines: Vec<&str> = source.lines().collect();
        if position.line == 0 || position.line > lines.len() {
            return None;
        }
        
        let line = lines[position.line - 1];
        let chars: Vec<char> = line.chars().collect();
        
        if position.character == 0 || position.character > chars.len() {
            return None;
        }
        
        let mut start = position.character - 1;
        let mut end = position.character;
        
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
            start -= 1;
        }
        
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }
        
        let word: String = chars[start..end].iter().collect();
        if word.is_empty() {
            None
        } else {
            Some(word)
        }
    }
    
    pub fn find_definition(&self, position: Position, source: &str) -> Option<Location> {
        let word = self.get_word_at_position(position, source)?;
        
        let lines: Vec<&str> = source.lines().collect();
        for (idx, line) in lines.iter().enumerate() {
            if line.starts_with(&format!("var {}", word)) || 
               line.starts_with(&format!("fungsi {}", word)) {
                let col = line.find(&word)?;
                return Some(Location {
                    uri: String::new(),
                    range: Range {
                        start: Position { line: idx + 1, character: col },
                        end: Position { line: idx + 1, character: col + word.len() },
                    },
                });
            }
        }
        
        None
    }
    
    pub fn find_references(&self, position: Position, source: &str) -> Vec<Location> {
        let word = match self.get_word_at_position(position, source) {
            Some(w) => w,
            None => return Vec::new(),
        };
        
        let mut locations = Vec::new();
        let lines: Vec<&str> = source.lines().collect();
        
        for (idx, line) in lines.iter().enumerate() {
            let mut search_start = 0;
            while let Some(col) = line[search_start..].find(&word) {
                let actual_col = search_start + col;
                
                let before_ok = actual_col == 0 || 
                    !line.chars().nth(actual_col - 1).map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false);
                let after_ok = actual_col + word.len() >= line.len() || 
                    !line.chars().nth(actual_col + word.len()).map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false);
                
                if before_ok && after_ok {
                    locations.push(Location {
                        uri: String::new(),
                        range: Range {
                            start: Position { line: idx + 1, character: actual_col },
                            end: Position { line: idx + 1, character: actual_col + word.len() },
                        },
                    });
                }
                
                search_start = actual_col + word.len();
            }
        }
        
        locations
    }
    
    pub fn get_document_symbols(&self, source: &str) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = source.lines().collect();
        
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("fungsi ") {
                if let Some(name) = trimmed.strip_prefix("fungsi ") {
                    let name = name.split('(').next().unwrap_or("").trim();
                    if !name.is_empty() {
                        symbols.push(Symbol {
                            name: name.to_string(),
                            kind: SymbolKind::Function,
                            range: Range {
                                start: Position { line: idx + 1, character: 1 },
                                end: Position { line: idx + 1, character: line.len() },
                            },
                            signature: Some(format!("fungsi {}(...)", name)),
                            documentation: None,
                        });
                    }
                }
            }
            
            if trimmed.starts_with("var ") || trimmed.starts_with("misal ") {
                let prefix = if trimmed.starts_with("var ") { "var " } else { "misal " };
                if let Some(rest) = trimmed.strip_prefix(prefix) {
                    let name = rest.split('=').next().unwrap_or("").trim();
                    let name = name.strip_prefix("mutabel ").unwrap_or(name).trim();
                    if !name.is_empty() {
                        symbols.push(Symbol {
                            name: name.to_string(),
                            kind: SymbolKind::Variable,
                            range: Range {
                                start: Position { line: idx + 1, character: 1 },
                                end: Position { line: idx + 1, character: line.len() },
                            },
                            signature: None,
                            documentation: None,
                        });
                    }
                }
            }

            if trimmed.starts_with("struktur ") {
                if let Some(rest) = trimmed.strip_prefix("struktur ") {
                    let name = rest.split('{').next().unwrap_or("").trim();
                    if !name.is_empty() {
                        symbols.push(Symbol {
                            name: name.to_string(),
                            kind: SymbolKind::Class,
                            range: Range {
                                start: Position { line: idx + 1, character: 1 },
                                end: Position { line: idx + 1, character: line.len() },
                            },
                            signature: Some(format!("struktur {}", name)),
                            documentation: None,
                        });
                    }
                }
            }

            if trimmed.starts_with("tetap ") {
                if let Some(rest) = trimmed.strip_prefix("tetap ") {
                    let name = rest.split('=').next().unwrap_or("").trim();
                    if !name.is_empty() {
                        symbols.push(Symbol {
                            name: name.to_string(),
                            kind: SymbolKind::Constant,
                            range: Range {
                                start: Position { line: idx + 1, character: 1 },
                                end: Position { line: idx + 1, character: line.len() },
                            },
                            signature: None,
                            documentation: None,
                        });
                    }
                }
            }
        }
        
        symbols
    }
    
    fn extract_error_info(error: &Galat) -> (usize, usize, String) {
        match error {
            Galat::Sintaks { baris, kolom, pesan } |
            Galat::Runtime { baris, kolom, pesan } => (*baris, *kolom, pesan.clone()),
            _ => (1, 1, format!("{}", error)),
        }
    }
    
    /// Menghasilkan Code Actions (Quick Fixes / Lightbulb Suggestions) untuk IDE
    pub fn get_code_actions(&self, uri: &str, range: Range, diagnostics: &[Diagnostic], source: &str) -> Vec<CodeAction> {
        let mut actions = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        if range.start.line > 0 && range.start.line <= lines.len() {
            let line_content = lines[range.start.line - 1];

            // 1. Perbaikan kata kunci usang 'var' -> 'misal'
            if line_content.trim_start().starts_with("var ") {
                let leading_ws = line_content.len() - line_content.trim_start().len();
                let mut changes = HashMap::new();
                changes.insert(uri.to_string(), vec![TextEdit {
                    range: Range {
                        start: Position { line: range.start.line, character: leading_ws },
                        end: Position { line: range.start.line, character: leading_ws + 3 },
                    },
                    new_text: "misal".to_string(),
                }]);

                actions.push(CodeAction {
                    title: "Perbarui sintaks 'var' menjadi 'misal'".to_string(),
                    kind: "quickfix".to_string(),
                    edit: Some(WorkspaceEdit { changes }),
                });
            }

            // 2. Koreksi salah ketik kata kunci bahasa Widya
            let typos = [
                ("fungi ", "fungsi "),
                ("jikalau ", "jika "),
                ("kembali ", "kembalikan "),
                ("cetakln", "cetak"),
            ];

            for (typo, fix) in typos {
                if let Some(col) = line_content.find(typo) {
                    let mut changes = HashMap::new();
                    changes.insert(uri.to_string(), vec![TextEdit {
                        range: Range {
                            start: Position { line: range.start.line, character: col },
                            end: Position { line: range.start.line, character: col + typo.len() },
                        },
                        new_text: fix.to_string(),
                    }]);

                    actions.push(CodeAction {
                        title: format!("Perbaiki ejaan '{}' menjadi '{}'", typo.trim(), fix.trim()),
                        kind: "quickfix".to_string(),
                        edit: Some(WorkspaceEdit { changes }),
                    });
                }
            }

            // 3. Rekomendasi Borrow Checker: Tambahkan 'mutabel'
            for diag in diagnostics {
                if diag.message.contains("imutabel") || diag.message.contains("tidak dapat memutasi") {
                    let mut changes = HashMap::new();
                    if let Some(col) = line_content.find("misal ") {
                        changes.insert(uri.to_string(), vec![TextEdit {
                            range: Range {
                                start: Position { line: range.start.line, character: col + 6 },
                                end: Position { line: range.start.line, character: col + 6 },
                            },
                            new_text: "mutabel ".to_string(),
                        }]);

                        actions.push(CodeAction {
                            title: "Tambahkan kata kunci 'mutabel' pada deklarasi variabel".to_string(),
                            kind: "quickfix".to_string(),
                            edit: Some(WorkspaceEdit { changes }),
                        });
                    }
                }
            }
        }

        actions
    }

    /// Melakukan auto-formatting kode sumber Widya-Lang (Format Document / textDocument/formatting)
    pub fn format_document(&self, _uri: &str, source: &str) -> Vec<TextEdit> {
        let mut formatted_lines = Vec::new();
        let mut indent_level: usize = 0;

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                formatted_lines.push(String::new());
                continue;
            }

            // Kurangi indentasi jika baris dimulai dengan kurung kurawal/siku penutup
            if trimmed.starts_with('}') || trimmed.starts_with(']') || trimmed.starts_with(')') {
                if indent_level > 0 {
                    indent_level -= 1;
                }
            }

            let indent = "    ".repeat(indent_level);
            formatted_lines.push(format!("{}{}", indent, trimmed));

            // Tambah indentasi jika ada kurung buka lebih banyak daripada kurung tutup
            let opens = trimmed.chars().filter(|c| *c == '{' || *c == '[').count();
            let closes = trimmed.chars().filter(|c| *c == '}' || *c == ']').count();
            if opens > closes {
                indent_level += opens - closes;
            }
        }

        let new_text = formatted_lines.join("\n");
        if new_text != source {
            let total_lines = source.lines().count().max(1);
            let last_line_len = source.lines().last().unwrap_or("").len();
            vec![TextEdit {
                range: Range {
                    start: Position { line: 1, character: 0 },
                    end: Position { line: total_lines, character: last_line_len },
                },
                new_text,
            }]
        } else {
            Vec::new()
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "name": "widya-language-server",
            "version": "0.1.0"
        })
    }
}

impl Default for WidyaLsp {
    fn default() -> Self {
        Self::new()
    }
}
