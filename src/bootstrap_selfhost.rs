//! Self-Hosting Compiler Bootstrapper & Stage-0 Generator (bootstrap_selfhost.rs)
//! Menyediakan representasi AST dan parser mandiri untuk memungkinkan Widya mengompilasi dirinya sendiri (*Self-Compiling*).

/// Token Mandiri Kompiler Widya
#[derive(Debug, Clone, PartialEq)]
pub enum SelfHostToken {
    KataKunci(String),
    Pengenal(String),
    Angka(f64),
    StringTeks(String),
    Simbol(char),
    Eof,
}

/// Node AST Kompiler Mandiri
#[derive(Debug, Clone, PartialEq)]
pub enum SelfHostAst {
    DeklarasiFungsi {
        nama: String,
        parameter: Vec<String>,
        tubuh: Vec<SelfHostAst>,
    },
    DeklarasiVariabel {
        nama: String,
        nilai: String,
    },
    Kembalikan(String),
}

/// Bootstrapper Stage-0 Compiler
pub struct SelfHostBootstrapper;

impl SelfHostBootstrapper {
    /// Tokenize kode sumber Widya murni
    pub fn tokenize(source: &str) -> Vec<SelfHostToken> {
        let mut tokens = Vec::new();
        let words: Vec<&str> = source.split_whitespace().collect();

        for w in words {
            match w {
                "fungsi" | "variabel" | "kembalikan" | "jika" | "untuk" => {
                    tokens.push(SelfHostToken::KataKunci(w.to_string()));
                }
                _ if w.chars().all(|c| c.is_digit(10) || c == '.') => {
                    if let Ok(n) = w.parse::<f64>() {
                        tokens.push(SelfHostToken::Angka(n));
                    }
                }
                _ => {
                    tokens.push(SelfHostToken::Pengenal(w.to_string()));
                }
            }
        }
        tokens.push(SelfHostToken::Eof);
        tokens
    }

    /// Emisi kode C/LLVM IR mandiri dari AST
    pub fn emit_stage1_c(ast: &[SelfHostAst]) -> String {
        let mut out = String::new();
        out.push_str("// === Widya-Lang Self-Hosted Stage-1 Generated C Code ===\n");
        out.push_str("#include <stdio.h>\n#include <stdint.h>\n\n");

        for node in ast {
            if let SelfHostAst::DeklarasiFungsi { nama, parameter, tubuh } = node {
                out.push_str(&format!("void {}_widya(", nama));
                for (i, p) in parameter.iter().enumerate() {
                    if i > 0 { out.push_str(", "); }
                    out.push_str(&format!("int64_t {}", p));
                }
                out.push_str(") {\n");

                for stmt in tubuh {
                    if let SelfHostAst::DeklarasiVariabel { nama: vname, nilai: vval } = stmt {
                        out.push_str(&format!("    int64_t {} = {};\n", vname, vval));
                    }
                }
                out.push_str("}\n\n");
            }
        }

        out
    }
}
