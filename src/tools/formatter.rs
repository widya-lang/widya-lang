use colored::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use crate::ast::{Expr, Program, Stmt};
use crate::lexer::Lexer;
use crate::parser::Parser;

/// Pilihan gaya kata kunci dwibahasa (Bilingual Keyword Style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordStyle {
    /// Kosakata Nusantara / Bahasa Indonesia asli (fungsi, jika, kembalikan, misal, tetap, dll.)
    Nusantara,
    /// Kosakata Internasional / Bahasa Inggris standar (fn, if, return, let, const, dll.)
    Internasional,
}

impl std::str::FromStr for KeywordStyle {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nusantara" | "id" | "indo" | "indonesia" => Ok(KeywordStyle::Nusantara),
            "internasional" | "international" | "en" | "eng" | "english" => Ok(KeywordStyle::Internasional),
            _ => Err(format!("Gaya kata kunci '{}' tidak dikenali. Pilih: 'nusantara' atau 'internasional'", s)),
        }
    }
}

/// Menyelaraskan gaya kata kunci dwibahasa tanpa mengubah string literal, komentar, atau identifier
pub fn transform_keyword_style(sumber: &str, target_style: KeywordStyle) -> String {
    let nusantara_to_intl: &[(&str, &str)] = &[
        ("fungsi", "fn"),
        ("def", "fn"),
        ("kembalikan", "return"),
        ("jika", "if"),
        ("kalau", "elif"),
        ("selain_itu", "else"),
        ("kalau_tidak", "else"),
        ("jika_tidak", "else"),
        ("selain", "else"),
        ("lainnya", "else"),
        ("selama", "while"),
        ("untuk", "for"),
        ("dalam", "in"),
        ("di", "in"),
        ("ulang", "loop"),
        ("misal", "let"),
        ("biar", "let"),
        ("diketahui", "let"),
        ("diket", "let"),
        ("tetap", "const"),
        ("konst", "const"),
        ("struktur", "struct"),
        ("kelas", "class"),
        ("pilihan", "enum"),
        ("varian", "enum"),
        ("opsi", "enum"),
        ("cocokkan", "match"),
        ("sesuaikan", "match"),
        ("cocok", "match"),
        ("benar", "true"),
        ("salah", "false"),
        ("kosong", "null"),
        ("nihil", "null"),
        ("impor", "import"),
        ("muat", "import"),
        ("sebagai", "as"),
        ("coba", "try"),
        ("tangkap", "catch"),
        ("lempar", "throw"),
        ("asinkron", "async"),
        ("tunggu_hasil", "await"),
        ("cetak", "print"),
        ("lanjut", "continue"),
        ("henti", "break"),
        ("berhenti", "break"),
        ("tipe", "type"),
        ("sifat", "trait"),
        ("terapkan", "impl"),
        ("implementasi", "impl"),
        ("baru", "new"),
        ("ini", "self"),
        ("pinjam", "borrow"),
        ("mutabel", "mut"),
    ];

    let intl_to_nusantara: &[(&str, &str)] = &[
        ("fn", "fungsi"),
        ("func", "fungsi"),
        ("function", "fungsi"),
        ("def", "fungsi"),
        ("return", "kembalikan"),
        ("if", "jika"),
        ("elif", "kalau"),
        ("else", "selain_itu"),
        ("while", "selama"),
        ("for", "untuk"),
        ("in", "dalam"),
        ("loop", "ulang"),
        ("let", "misal"),
        ("var", "misal"),
        ("const", "tetap"),
        ("struct", "struktur"),
        ("class", "kelas"),
        ("enum", "pilihan"),
        ("match", "cocokkan"),
        ("true", "benar"),
        ("false", "salah"),
        ("null", "kosong"),
        ("nil", "kosong"),
        ("import", "impor"),
        ("as", "sebagai"),
        ("try", "coba"),
        ("catch", "tangkap"),
        ("throw", "lempar"),
        ("async", "asinkron"),
        ("await", "tunggu_hasil"),
        ("print", "cetak"),
        ("println", "cetak"),
        ("continue", "lanjut"),
        ("break", "berhenti"),
        ("type", "tipe"),
        ("trait", "sifat"),
        ("interface", "sifat"),
        ("impl", "terapkan"),
        ("new", "baru"),
        ("self", "ini"),
        ("this", "ini"),
        ("borrow", "pinjam"),
        ("mut", "mutabel"),
    ];

    let mut result = String::with_capacity(sumber.len());
    let chars: Vec<char> = sumber.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // 1. Cek string literal ganda "..."
        if chars[i] == '"' {
            result.push(chars[i]);
            i += 1;
            while i < len {
                result.push(chars[i]);
                if chars[i] == '\\' && i + 1 < len {
                    i += 1;
                    result.push(chars[i]);
                } else if chars[i] == '"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // 2. Cek string literal tunggal '...'
        if chars[i] == '\'' {
            result.push(chars[i]);
            i += 1;
            while i < len {
                result.push(chars[i]);
                if chars[i] == '\\' && i + 1 < len {
                    i += 1;
                    result.push(chars[i]);
                } else if chars[i] == '\'' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // 3. Cek komentar satu baris //...
        if chars[i] == '/' && i + 1 < len && chars[i + 1] == '/' {
            while i < len && chars[i] != '\n' {
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // 4. Cek komentar blok /* ... */
        if chars[i] == '/' && i + 1 < len && chars[i + 1] == '*' {
            result.push(chars[i]);
            result.push(chars[i + 1]);
            i += 2;
            while i < len {
                if chars[i] == '*' && i + 1 < len && chars[i + 1] == '/' {
                    result.push('*');
                    result.push('/');
                    i += 2;
                    break;
                }
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // 5. Cek token kata (identifier / keyword)
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            let mapped_word = match target_style {
                KeywordStyle::Internasional => {
                    let mut found = None;
                    for (nus, intl) in nusantara_to_intl {
                        if word == *nus {
                            found = Some(*intl);
                            break;
                        }
                    }
                    found.unwrap_or(&word)
                }
                KeywordStyle::Nusantara => {
                    let mut found = None;
                    for (intl, nus) in intl_to_nusantara {
                        if word == *intl {
                            found = Some(*nus);
                            break;
                        }
                    }
                    found.unwrap_or(&word)
                }
            };

            result.push_str(mapped_word);
            continue;
        }

        // Karakter lainnya (spasi, simbol, angka mandiri)
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Memformat string kode sumber sesuai konvensi indentasi resmi Widya-Lang
pub fn format_code(sumber: &str) -> String {
    format_code_with_style(sumber, None)
}

/// Memformat string kode sumber dengan opsi gaya kata kunci dwibahasa
pub fn format_code_with_style(sumber: &str, style: Option<KeywordStyle>) -> String {
    let source_aligned = match style {
        Some(s) => transform_keyword_style(sumber, s),
        None => sumber.to_string(),
    };

    let mut formatted = String::new();
    let lines = source_aligned.lines();
    let mut indent_level: usize = 0;

    for raw_line in lines {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            formatted.push('\n');
            continue;
        }

        // Sesuaikan indentasi untuk kurung kurawal/siku penutup di awal baris
        let leading_close = trimmed.starts_with('}') || trimmed.starts_with(']');
        let effective_indent = if leading_close && indent_level > 0 {
            indent_level - 1
        } else {
            indent_level
        };

        let indent_str = "    ".repeat(effective_indent);
        formatted.push_str(&indent_str);
        formatted.push_str(trimmed);
        formatted.push('\n');

        // Hitung pembukaan dan penutupan kurung untuk baris selanjutnya
        let open_count = trimmed.chars().filter(|c| *c == '{' || *c == '[').count();
        let close_count = trimmed.chars().filter(|c| *c == '}' || *c == ']').count();

        if open_count > close_count {
            indent_level += open_count - close_count;
        } else if close_count > open_count {
            indent_level = indent_level.saturating_sub(close_count - open_count);
        }
    }

    formatted
}

/// Memformat berkas sumber Widya
/// - `overwrite`: Timpa berkas langsung jika true
/// - `check_only`: Mode CI, kembalikan `true` jika berkas sudah terformat, `false` jika belum
/// - `style`: Opsi gaya kata kunci Nusantara vs Internasional
pub fn format_berkas(path: &PathBuf, overwrite: bool, check_only: bool, style: Option<KeywordStyle>) -> bool {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Gagal membaca berkas '{}': {}", "❌".bright_red(), path.display(), e);
            return false;
        }
    };

    let formatted = format_code_with_style(&sumber, style);

    if check_only {
        if sumber.trim() == formatted.trim() {
            println!("{} Berkas '{}' sudah terformat rapi sesuai standar Widya.", "✅".bright_green(), path.display());
            true
        } else {
            eprintln!("{} Berkas '{}' belum terformat sesuai standar Widya (Jalankan 'widya format -t {}' untuk memformat).", "❌".bright_red(), path.display(), path.display());
            false
        }
    } else if overwrite {
        if let Err(e) = fs::write(path, &formatted) {
            eprintln!("{} Gagal menyimpan pemformatan ke berkas: {}", "❌".bright_red(), e);
            false
        } else {
            println!("{} Berkas '{}' berhasil diformat dan disimpan ulang.", "✅".bright_green(), path.display());
            true
        }
    } else {
        println!("{}", "=== HASIL PEMFORMATAN STANDAR WIDYA ===".bright_green().bold());
        println!("{}", formatted);
        true
    }
}

/// Tingkat keparahan isu linter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintSeverity {
    Warning,
    Error,
    Info,
}

/// Representasi isu yang terdeteksi oleh linter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintIssue {
    pub code: &'static str,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: LintSeverity,
    pub suggestion: Option<String>,
}

/// Mesin Linter Widya-Lang berstandar industri
pub struct WidyaLinter {
    issues: Vec<LintIssue>,
    declared_vars: HashMap<String, (usize, usize)>, // nama -> (line, col)
    used_vars: HashSet<String>,
}

impl WidyaLinter {
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            declared_vars: HashMap::new(),
            used_vars: HashSet::new(),
        }
    }

    pub fn inspect_program(&mut self, program: &Program) -> Vec<LintIssue> {
        self.issues.clear();
        self.declared_vars.clear();
        self.used_vars.clear();

        self.inspect_statements(&program.statements);

        // Periksa WIDYA-L01: Unused Variables
        let mut unused_issues = Vec::new();
        for (var_name, (line, col)) in &self.declared_vars {
            if !self.used_vars.contains(var_name) && !var_name.starts_with('_') {
                unused_issues.push(LintIssue {
                    code: "WIDYA-L01",
                    message: format!("Variabel '{}' dideklarasikan namun tidak pernah digunakan", var_name),
                    line: *line,
                    column: *col,
                    severity: LintSeverity::Warning,
                    suggestion: Some(format!("Gunakan prefix '_{}' jika variabel sengaja diabaikan", var_name)),
                });
            }
        }

        self.issues.extend(unused_issues);
        self.issues.sort_by_key(|i| (i.line, i.column));
        self.issues.clone()
    }

    fn inspect_statements(&mut self, stmts: &[Stmt]) {
        let mut encountered_terminator = false;

        for stmt in stmts {
            // WIDYA-L03: Dead Code After Return / Break / Continue
            if encountered_terminator {
                let span = stmt.span();
                self.issues.push(LintIssue {
                    code: "WIDYA-L03",
                    message: "Kode tidak terjangkau (dead code) setelah pernyataan return / break".to_string(),
                    line: span.line,
                    column: span.column,
                    severity: LintSeverity::Warning,
                    suggestion: Some("Hapus kode yang tidak pernah dieksekusi ini".to_string()),
                });
            }

            match stmt {
                Stmt::VarDecl { name, span, initializer, .. } => {
                    // WIDYA-L04: Uninitialized Variable
                    if initializer.is_none() {
                        self.issues.push(LintIssue {
                            code: "WIDYA-L04",
                            message: format!("Variabel '{}' dideklarasikan tanpa inisialisasi awal", name),
                            line: span.line,
                            column: span.column,
                            severity: LintSeverity::Warning,
                            suggestion: Some(format!("Inisialisasi nilai awal, contoh: misal {} = kosong;", name)),
                        });
                    }

                    // WIDYA-L02: Naming Convention (variabel harus snake_case)
                    if Self::is_camel_case(name) {
                        self.issues.push(LintIssue {
                            code: "WIDYA-L02",
                            message: format!("Nama variabel '{}' sebaiknya menggunakan format snake_case", name),
                            line: span.line,
                            column: span.column,
                            severity: LintSeverity::Info,
                            suggestion: Some(format!("Ganti dengan '{}'", Self::to_snake_case(name))),
                        });
                    }

                    self.declared_vars.insert(name.clone(), (span.line, span.column));

                    if let Some(expr) = initializer {
                        self.inspect_expression(expr);
                    }
                }
                Stmt::FunctionDecl { name, params, body, span, .. } => {
                    // WIDYA-L02: Naming Convention (fungsi harus snake_case)
                    if Self::is_camel_case(name) {
                        self.issues.push(LintIssue {
                            code: "WIDYA-L02",
                            message: format!("Nama fungsi '{}' sebaiknya menggunakan format snake_case", name),
                            line: span.line,
                            column: span.column,
                            severity: LintSeverity::Info,
                            suggestion: Some(format!("Ganti dengan '{}'", Self::to_snake_case(name))),
                        });
                    }

                    // WIDYA-L05: Empty Block
                    if body.is_empty() {
                        self.issues.push(LintIssue {
                            code: "WIDYA-L05",
                            message: format!("Fungsi '{}' memiliki blok pernyataan kosong", name),
                            line: span.line,
                            column: span.column,
                            severity: LintSeverity::Warning,
                            suggestion: Some("Tambahkan implementasi atau kembalikan nilai default".to_string()),
                        });
                    }

                    // Parameter fungsi didaftarkan
                    for param in params {
                        self.declared_vars.insert(param.clone(), (span.line, span.column));
                    }

                    self.inspect_statements(body);
                }
                Stmt::StructDecl { name, span, .. } => {
                    // WIDYA-L02: Struct harus PascalCase
                    if !Self::is_pascal_case(name) {
                        self.issues.push(LintIssue {
                            code: "WIDYA-L02",
                            message: format!("Nama struktur data '{}' sebaiknya menggunakan format PascalCase", name),
                            line: span.line,
                            column: span.column,
                            severity: LintSeverity::Info,
                            suggestion: Some(format!("Ganti dengan '{}'", Self::to_pascal_case(name))),
                        });
                    }
                }
                Stmt::EnumDecl { name, span, .. } => {
                    // WIDYA-L02: Enum harus PascalCase
                    if !Self::is_pascal_case(name) {
                        self.issues.push(LintIssue {
                            code: "WIDYA-L02",
                            message: format!("Nama enum/pilihan '{}' sebaiknya menggunakan format PascalCase", name),
                            line: span.line,
                            column: span.column,
                            severity: LintSeverity::Info,
                            suggestion: Some(format!("Ganti dengan '{}'", Self::to_pascal_case(name))),
                        });
                    }
                }
                Stmt::Block(inner_stmts, span) => {
                    if inner_stmts.is_empty() {
                        self.issues.push(LintIssue {
                            code: "WIDYA-L05",
                            message: "Blok pernyataan kurung kurawal kosong tanpa operasi".to_string(),
                            line: span.line,
                            column: span.column,
                            severity: LintSeverity::Info,
                            suggestion: Some("Hapus blok kosong ini jika tidak diperlukan".to_string()),
                        });
                    }
                    self.inspect_statements(inner_stmts);
                }
                Stmt::If { condition, then_branch, elif_branches, else_branch, .. } => {
                    self.inspect_expression(condition);
                    self.inspect_statement_box(then_branch);
                    for (c, b) in elif_branches {
                        self.inspect_expression(c);
                        self.inspect_statement_box(&Box::new(b.clone()));
                    }
                    if let Some(eb) = else_branch {
                        self.inspect_statement_box(eb);
                    }
                }
                Stmt::While { condition, body, .. } => {
                    self.inspect_expression(condition);
                    self.inspect_statement_box(body);
                }
                Stmt::ForIn { var_name, iterable, body, span } => {
                    self.declared_vars.insert(var_name.clone(), (span.line, span.column));
                    self.inspect_expression(iterable);
                    self.inspect_statement_box(body);
                }
                Stmt::Expression(expr) => {
                    self.inspect_expression(expr);
                }
                Stmt::Assignment { value, .. } => {
                    self.inspect_expression(value);
                }
                Stmt::Return { value, .. } => {
                    if let Some(expr) = value {
                        self.inspect_expression(expr);
                    }
                    encountered_terminator = true;
                }
                Stmt::Break(_) | Stmt::Continue(_) => {
                    encountered_terminator = true;
                }
                _ => {}
            }
        }
    }

    fn inspect_statement_box(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block(stmts, _) => self.inspect_statements(stmts),
            single => self.inspect_statements(&[single.clone()]),
        }
    }

    fn inspect_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Identifier(name, _) => {
                self.used_vars.insert(name.clone());
            }
            Expr::Binary { left, right, .. } => {
                self.inspect_expression(left);
                self.inspect_expression(right);
            }
            Expr::Unary { right, .. } => {
                self.inspect_expression(right);
            }
            Expr::Grouping(inner, _) => {
                self.inspect_expression(inner);
            }
            Expr::Array(items, _) => {
                for item in items {
                    self.inspect_expression(item);
                }
            }
            Expr::Map(pairs, _) => {
                for (k, v) in pairs {
                    self.inspect_expression(k);
                    self.inspect_expression(v);
                }
            }
            Expr::Index { target, index, .. } => {
                self.inspect_expression(target);
                self.inspect_expression(index);
            }
            Expr::Call { callee, arguments, .. } => {
                self.inspect_expression(callee);
                for arg in arguments {
                    self.inspect_expression(arg);
                }
            }
            Expr::FunctionExpr { body, .. } => {
                self.inspect_statements(body);
            }
            _ => {}
        }
    }

    fn is_camel_case(s: &str) -> bool {
        if s.starts_with('_') || s.is_empty() {
            return false;
        }
        let first_char = s.chars().next().unwrap();
        first_char.is_lowercase() && s.chars().any(|c| c.is_uppercase()) && !s.contains('_')
    }

    fn is_pascal_case(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let first_char = s.chars().next().unwrap();
        first_char.is_uppercase() && !s.contains('_')
    }

    fn to_snake_case(s: &str) -> String {
        let mut res = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 {
                    res.push('_');
                }
                res.push(c.to_ascii_lowercase());
            } else {
                res.push(c);
            }
        }
        res
    }

    fn to_pascal_case(s: &str) -> String {
        let mut res = String::new();
        let mut capitalize_next = true;
        for c in s.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                res.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                res.push(c);
            }
        }
        res
    }
}

impl Default for WidyaLinter {
    fn default() -> Self {
        Self::new()
    }
}

/// Menjalankan pemeriksaan linter pada berkas dan menampilkan laporan visual
pub fn periksa_linter(path: &PathBuf) -> Vec<LintIssue> {
    let sumber = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{} Gagal membaca berkas '{}': {}", "❌".bright_red(), path.display(), e);
            return Vec::new();
        }
    };

    println!("🔍 {} Memeriksa potensi galat & best-practices pada '{}'...", "Widya Linter:".bright_cyan().bold(), path.display().to_string().bright_yellow());

    let mut lexer = Lexer::new(&sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{} Galat Pemindai Token: {}", "❌".bright_red(), e);
            return Vec::new();
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} Galat Parser: {}", "❌".bright_red(), e);
            return Vec::new();
        }
    };

    let mut linter = WidyaLinter::new();
    let mut issues = linter.inspect_program(&program);

    // Jalankan pemeriksaan linter keamanan mendalam, kebocoran memori, dan UU PDP
    let sec_findings = crate::security_linter::SecurityLinter::scan_source(&sumber);
    for sec in sec_findings {
        let severity = match sec.severity {
            crate::security_linter::VulnerabilitySeverity::Critical | crate::security_linter::VulnerabilitySeverity::High => LintSeverity::Error,
            crate::security_linter::VulnerabilitySeverity::Medium => LintSeverity::Warning,
            crate::security_linter::VulnerabilitySeverity::Low => LintSeverity::Info,
        };
        issues.push(LintIssue {
            code: Box::leak(sec.rule_id.into_boxed_str()),
            message: sec.message,
            line: sec.line,
            column: 1,
            severity,
            suggestion: Some(sec.remediation),
        });
    }

    issues.sort_by_key(|i| (i.line, i.column));

    if issues.is_empty() {
        println!("{}", "✨ Kode bersih sempurna! Seluruh standar kode Widya-Lang terpenuhi (0 peringatan).".bright_green().bold());
    } else {
        println!("{}", format!("⚠️  Ditemukan {} isu gaya kode & potensi galat:", issues.len()).bright_yellow().bold());
        for issue in &issues {
            let label = match issue.severity {
                LintSeverity::Warning => "Peringatan".bright_yellow(),
                LintSeverity::Error => "Galat".bright_red(),
                LintSeverity::Info => "Info".bright_blue(),
            };
            println!("  [{}] [{}:{}] ({}) {}", label, issue.line, issue.column, issue.code.bright_magenta(), issue.message);
            if let Some(ref sug) = issue.suggestion {
                println!("    ↳ {}", format!("Saran: {}", sug).bright_black());
            }
        }
        println!();
    }

    issues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_code_indentation() {
        let unformatted = "fungsi hitung(a,b){\nif(a>0){\ncetak(a);\n}\nkembalikan a+b;\n}";
        let formatted = format_code(unformatted);
        assert!(formatted.contains("    if(a>0){\n        cetak(a);\n    }"));
        assert!(formatted.contains("    kembalikan a+b;"));
    }

    #[test]
    fn test_bilingual_keyword_formatter() {
        let nusantara = r#"fungsi hitung(a, b) {
    misal pesan = "fungsi ini jika benar";
    jika a > 0 {
        cetak(pesan);
        kembalikan benar;
    } selain_itu {
        kembalikan salah;
    }
}"#;
        // 1. Transformasi Nusantara -> Internasional
        let intl = format_code_with_style(nusantara, Some(KeywordStyle::Internasional));
        assert!(intl.contains("fn hitung(a, b)"));
        assert!(intl.contains("let pesan = \"fungsi ini jika benar\";"), "String literal tidak boleh diubah!");
        assert!(intl.contains("if a > 0"));
        assert!(intl.contains("return true;"));
        assert!(intl.contains("else {"));
        assert!(intl.contains("return false;"));

        // 2. Transformasi Internasional -> Nusantara
        let back_to_nusantara = format_code_with_style(&intl, Some(KeywordStyle::Nusantara));
        assert!(back_to_nusantara.contains("fungsi hitung(a, b)"));
        assert!(back_to_nusantara.contains("jika a > 0"));
        assert!(back_to_nusantara.contains("kembalikan benar;"));
        assert!(back_to_nusantara.contains("selain_itu {"));
    }

    #[test]
    fn test_linter_unused_variables() {
        let code = "fungsi uji() {\n    misal varTakTerpakai = 42;\n    misal _diabaikan = 100;\n    misal terpakai = 5;\n    cetak(terpakai);\n}";
        let mut lexer = Lexer::new(code);
        let tokens = lexer.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut linter = WidyaLinter::new();
        let issues = linter.inspect_program(&program);

        // Harus mendeteksi WIDYA-L01 untuk varTakTerpakai, tetapi mengabaikan _diabaikan dan terpakai
        let unused = issues.iter().find(|i| i.code == "WIDYA-L01");
        assert!(unused.is_some());
        assert!(unused.unwrap().message.contains("varTakTerpakai"));
    }

    #[test]
    fn test_linter_dead_code_and_naming() {
        let code = "struktur data_pengguna {\n    id: Int\n}\n\nfungsi tes() {\n    kembalikan 10;\n    cetak(\"mati\");\n}";
        let mut lexer = Lexer::new(code);
        let tokens = lexer.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut linter = WidyaLinter::new();
        let issues = linter.inspect_program(&program);

        let dead_code = issues.iter().find(|i| i.code == "WIDYA-L03");
        assert!(dead_code.is_some(), "Harus mendeteksi kode tidak terjangkau setelah return!");

        let naming = issues.iter().find(|i| i.code == "WIDYA-L02");
        assert!(naming.is_some(), "Harus merekomendasikan PascalCase untuk struct data_pengguna!");
    }
}
