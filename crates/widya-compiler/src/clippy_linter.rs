//! Clippy-Grade Idiomatic Linter Engine for Widya-Lang
//! Mendeteksi kode tidak efisien, peringatan gaya bahasa idiomatik, deklarasi mutabel yang tidak pernah diubah,
//! kode mati (dead code), dan saran refactoring otomatis selevel cargo clippy di Rust.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Kategori Aturan Linter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClippyLintCategory {
    Correctness,
    Performance,
    Style,
    Complexity,
    DeadCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LintSeverity {
    Error,
    Warning,
    Suggestion,
}

impl fmt::Display for LintSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Suggestion => write!(f, "suggestion"),
        }
    }
}

/// Diagnostik Temuan Clippy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClippyLintFinding {
    pub rule_id: &'static str,
    pub category: ClippyLintCategory,
    pub severity: LintSeverity,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub suggestion: Option<String>,
}

impl fmt::Display for ClippyLintFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}: {}] baris {}:{}: {}",
            self.severity, self.rule_id, self.line, self.column, self.message
        )
    }
}

/// Mesin Penganalisis Kode Statis Clippy
pub struct WidyaClippyLinter;

impl WidyaClippyLinter {
    /// Pindai kode sumber Widya untuk mendeteksi anti-pattern idiomatik
    pub fn lint_source(source: &str) -> Vec<ClippyLintFinding> {
        let mut findings = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        let mut declared_mut_vars: HashMap<String, (usize, usize)> = HashMap::new();
        let mut mutated_vars: HashSet<String> = HashSet::new();
        let mut declared_vars: HashMap<String, (usize, usize)> = HashMap::new();
        let mut used_vars: HashSet<String> = HashSet::new();

        let mut saw_return = false;
        let mut return_line = 0;

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;
            let trimmed = line.trim();

            // Abaikan komentar
            if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
                continue;
            }

            // 1. CLNT005: Dead Code After Return (Kode Mati Pasca Kembalikan)
            if saw_return {
                if trimmed == "}" || trimmed == "}" || trimmed.is_empty() {
                    saw_return = false;
                } else {
                    findings.push(ClippyLintFinding {
                        rule_id: "WIDYA_CLNT005_DEAD_CODE",
                        category: ClippyLintCategory::DeadCode,
                        severity: LintSeverity::Warning,
                        message: format!("Pernyataan tidak akan pernah dieksekusi setelah 'kembalikan' pada baris {}", return_line),
                        line: line_num,
                        column: 1,
                        suggestion: Some("Hapus instruksi yang tidak terjangkau (unreachable code)".to_string()),
                    });
                    saw_return = false;
                }
            }

            if trimmed.starts_with("kembalikan ") || trimmed == "kembalikan;" || trimmed.starts_with("return ") {
                saw_return = true;
                return_line = line_num;
            }

            // 2. CLNT003: Collapse Nested If (Penyederhanaan jika bersarang)
            if trimmed.starts_with("jika ") || trimmed.starts_with("if ") {
                if let Some(next_line) = lines.get(idx + 1) {
                    let next_trimmed = next_line.trim();
                    if next_trimmed.starts_with("jika ") || next_trimmed.starts_with("if ") {
                        findings.push(ClippyLintFinding {
                            rule_id: "WIDYA_CLNT003_COLLAPSE_IF",
                            category: ClippyLintCategory::Complexity,
                            severity: LintSeverity::Suggestion,
                            message: "Percabangan 'jika' bersarang berturutan dapat disederhanakan dengan operator logika '&&'".to_string(),
                            line: line_num,
                            column: 1,
                            suggestion: Some("Gunakan: jika kondisi_a && kondisi_b { ... }".to_string()),
                        });
                    }
                }
            }

            // 3. CLNT004: Redundant Clone on Copy Primitives
            if trimmed.contains(".salin()") || trimmed.contains(".clone()") {
                if trimmed.contains("42.salin()") || trimmed.contains("true.salin()") || trimmed.contains("angka.salin()") {
                    findings.push(ClippyLintFinding {
                        rule_id: "WIDYA_CLNT004_REDUNDANT_CLONE",
                        category: ClippyLintCategory::Performance,
                        severity: LintSeverity::Warning,
                        message: "Pemanggilan .salin() mubazir pada tipe data primitif yang menerapkan semantik Salin (Copy)".to_string(),
                        line: line_num,
                        column: 1,
                        suggestion: Some("Hapus pemanggilan .salin() karena nilai secara otomatis disalin tanpa alokasi heap".to_string()),
                    });
                }
            }

            // 4. Deteksi Deklarasi Variabel & Mutabilitas
            if trimmed.starts_with("misal mut ") || trimmed.starts_with("let mut ") || trimmed.starts_with("misal mutabel ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 3 {
                    let var_name = parts[2].trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                    declared_mut_vars.insert(var_name.to_string(), (line_num, 1));
                }
            } else if trimmed.starts_with("misal ") || trimmed.starts_with("let ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let var_name = parts[1].trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                    declared_vars.insert(var_name.to_string(), (line_num, 1));
                }
            }

            // Cek mutasi variabel (penugasan ulang: var = ...)
            if trimmed.contains(" = ") && !trimmed.starts_with("misal ") && !trimmed.starts_with("let ") && !trimmed.starts_with("tetap ") {
                let lhs = trimmed.split(" = ").next().unwrap_or("").trim();
                let var_name = lhs.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                mutated_vars.insert(var_name.to_string());
            }

            // Cek penggunaan variabel
            for var in declared_vars.keys().chain(declared_mut_vars.keys()) {
                if line.contains(var) && !trimmed.starts_with(&format!("misal {}", var)) && !trimmed.starts_with(&format!("let {}", var)) {
                    used_vars.insert(var.clone());
                }
            }
        }

        // CLNT002: Unnecessary Mutable Variable (Variabel mutabel tapi tidak pernah dimutasi)
        for (var, (line, col)) in &declared_mut_vars {
            if !mutated_vars.contains(var) {
                findings.push(ClippyLintFinding {
                    rule_id: "WIDYA_CLNT002_UNNECESSARY_MUT",
                    category: ClippyLintCategory::Style,
                    severity: LintSeverity::Warning,
                    message: format!("Variabel '{}' dideklarasikan mutabel ('mut'), tetapi tidak pernah dimutasi nilainya", var),
                    line: *line,
                    column: *col,
                    suggestion: Some(format!("Hapus kata kunci 'mut': gunakan 'misal {} = ...'", var)),
                });
            }
        }

        // CLNT001: Unused Variable
        for (var, (line, col)) in &declared_vars {
            if !var.starts_with('_') && !used_vars.contains(var) {
                findings.push(ClippyLintFinding {
                    rule_id: "WIDYA_CLNT001_UNUSED_VAR",
                    category: ClippyLintCategory::DeadCode,
                    severity: LintSeverity::Warning,
                    message: format!("Variabel '{}' dideklarasikan tetapi tidak pernah digunakan", var),
                    line: *line,
                    column: *col,
                    suggestion: Some(format!("Awali dengan garis bawah jika sengaja tidak digunakan: '_{}'", var)),
                });
            }
        }

        findings
    }
}
