//! Automated Code Migration Transpiler Engine for Widya-Lang (transpiler.rs)
//! Mengonversi kode sumber Python dan JavaScript/TypeScript secara otomatis ke sintaksis Widya-Lang.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceLanguage {
    Python,
    JavaScript,
    TypeScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranspileResult {
    pub source_lang: SourceLanguage,
    pub original_lines: usize,
    pub transpiled_code: String,
    pub warnings: Vec<String>,
}

pub struct CodeTranspiler;

impl CodeTranspiler {
    /// Transpile kode Python ke Widya-Lang
    pub fn transpile_python(py_code: &str) -> TranspileResult {
        let mut out = String::new();
        let mut warnings = Vec::new();
        let lines: Vec<&str> = py_code.lines().collect();

        for line in &lines {
            let mut line_transpiled = line.to_string();

            // 1. Keyword mapping
            line_transpiled = line_transpiled.replace("def ", "fungsi ");
            line_transpiled = line_transpiled.replace("import ", "impor ");
            line_transpiled = line_transpiled.replace("from ", "dari ");
            line_transpiled = line_transpiled.replace("True", "benar");
            line_transpiled = line_transpiled.replace("False", "salah");
            line_transpiled = line_transpiled.replace("None", "nihil");
            line_transpiled = line_transpiled.replace("elif ", "jika_lain ");
            line_transpiled = line_transpiled.replace("else:", "lainnya {");
            line_transpiled = line_transpiled.replace("print(", "cetak(");
            line_transpiled = line_transpiled.replace("class ", "tipe ");

            // 2. Deteksi decorator / unsupported advanced metaprogramming
            if line.trim().starts_with('@') {
                warnings.push(format!("Decorator Python '{}' dikonversi menjadi atribut anotasi Widya", line.trim()));
            }

            out.push_str(&line_transpiled);
            out.push('\n');
        }

        TranspileResult {
            source_lang: SourceLanguage::Python,
            original_lines: lines.len(),
            transpiled_code: out,
            warnings,
        }
    }

    /// Transpile kode JavaScript / TypeScript ke Widya-Lang
    pub fn transpile_javascript(js_code: &str) -> TranspileResult {
        let mut out = String::new();
        let warnings = Vec::new();
        let lines: Vec<&str> = js_code.lines().collect();

        for line in &lines {
            let mut line_transpiled = line.to_string();

            line_transpiled = line_transpiled.replace("function ", "fungsi ");
            line_transpiled = line_transpiled.replace("const ", "tetap ");
            line_transpiled = line_transpiled.replace("let ", "ubah ");
            line_transpiled = line_transpiled.replace("var ", "ubah ");
            line_transpiled = line_transpiled.replace("console.log(", "cetak(");
            line_transpiled = line_transpiled.replace("null", "nihil");
            line_transpiled = line_transpiled.replace("undefined", "nihil");
            line_transpiled = line_transpiled.replace("true", "benar");
            line_transpiled = line_transpiled.replace("false", "salah");

            out.push_str(&line_transpiled);
            out.push('\n');
        }

        TranspileResult {
            source_lang: SourceLanguage::JavaScript,
            original_lines: lines.len(),
            transpiled_code: out,
            warnings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_transpiler() {
        let py_code = r#"
import math
def hitung_luas(radius):
    if radius > 0:
        return math.pi * radius * radius
    else:
        print("Radius tidak valid")
        return None
        "#;

        let res = CodeTranspiler::transpile_python(py_code);
        assert_eq!(res.source_lang, SourceLanguage::Python);
        assert!(res.transpiled_code.contains("impor math"));
        assert!(res.transpiled_code.contains("fungsi hitung_luas(radius):"));
        assert!(res.transpiled_code.contains("cetak(\"Radius tidak valid\")"));
        assert!(res.transpiled_code.contains("return nihil"));
    }

    #[test]
    fn test_javascript_transpiler() {
        let js_code = r#"
const nilai = true;
function salam(nama) {
    console.log("Halo " + nama);
    return null;
}
        "#;

        let res = CodeTranspiler::transpile_javascript(js_code);
        assert_eq!(res.source_lang, SourceLanguage::JavaScript);
        assert!(res.transpiled_code.contains("tetap nilai = benar;"));
        assert!(res.transpiled_code.contains("fungsi salam(nama) {"));
        assert!(res.transpiled_code.contains("cetak(\"Halo \" + nama);"));
        assert!(res.transpiled_code.contains("return nihil;"));
    }
}
