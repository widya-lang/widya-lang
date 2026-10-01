//! Enterprise Security Linter & UU PDP Compliance Scanner (security_linter.rs)
//! Mendeteksi kerentanan kode (SQL Injection, Hardcoded Secrets) dan memvalidasi kepatuhan enkripsi data pribadi (UU PDP).

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VulnerabilitySeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityFinding {
    pub rule_id: String,
    pub severity: VulnerabilitySeverity,
    pub message: String,
    pub line: usize,
    pub remediation: String,
}

pub struct SecurityLinter;

impl SecurityLinter {
    /// Pindai baris kode untuk potensi kerentanan keamanan dan kepatuhan UU PDP
    pub fn scan_source(source_code: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        let pii_keywords = ["nik", "nomor_ktp", "no_hp", "rekam_medis", "kata_sandi", "password", "pin_atm"];

        for (line_idx, line) in source_code.lines().enumerate() {
            let line_num = line_idx + 1;
            let line_lower = line.to_lowercase();

            // 1. Deteksi Potensi SQL Injection
            if (line_lower.contains("exec(") || line_lower.contains("query(") || line_lower.contains("execute(")
                || line_lower.contains("select ") || line_lower.contains("insert ") || line_lower.contains("update ") || line_lower.contains("delete "))
                && (line_lower.contains(" + ") || line_lower.contains(".format(") || line_lower.contains("format!"))
            {
                findings.push(SecurityFinding {
                    rule_id: "WIDYA-SEC-001".to_string(),
                    severity: VulnerabilitySeverity::Critical,
                    message: "Terdeteksi potensi SQL Injection melalui penggabungan string (concatenation)".to_string(),
                    line: line_num,
                    remediation: "Gunakan parameter binding query (? atau :param) yang disediakan oleh WidyaBase".to_string(),
                });
            }

            // 2. Deteksi Hardcoded Secret Key / Token
            if (line_lower.contains("api_key") || line_lower.contains("secret") || line_lower.contains("private_key"))
                && (line.contains("=\"") || line.contains("= \"") || line.contains(": \""))
                && line.len() > 30
            {
                findings.push(SecurityFinding {
                    rule_id: "WIDYA-SEC-002".to_string(),
                    severity: VulnerabilitySeverity::High,
                    message: "Terdeteksi kunci rahasia / API Key yang ditulis langsung di dalam kode (Hardcoded Secret)".to_string(),
                    line: line_num,
                    remediation: "Pindahkan kredensial rahasia ke Environment Variable atau Secure Vault".to_string(),
                });
            }

            // 3. Deteksi Pelanggaran UU PDP (Penyimpanan Data Pribadi Plaintext)
            for pii in &pii_keywords {
                if line_lower.contains(pii) && (line_lower.contains("print(") || line_lower.contains("cetak(") || line_lower.contains("tulis_berkas(")) {
                    findings.push(SecurityFinding {
                        rule_id: "WIDYA-PDP-001".to_string(),
                        severity: VulnerabilitySeverity::High,
                        message: format!("Pelanggaran UU PDP: Data Pribadi Sensitif ('{}') dicetak/disimpan tanpa enkripsi", pii),
                        line: line_num,
                        remediation: "Terapkan penyortiran atau penyamaran (Masking / AES-GCM Enkripsi) sesuai standar UU PDP".to_string(),
                    });
                }
            }
        }

        findings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_injection_detection() {
        let code = r#"
            fungsi cari_user(nama) {
                db.exec("SELECT * FROM users WHERE nama = " + nama)
            }
        "#;
        let findings = SecurityLinter::scan_source(code);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].rule_id, "WIDYA-SEC-001");
        assert_eq!(findings[0].severity, VulnerabilitySeverity::Critical);
    }

    #[test]
    fn test_hardcoded_secret_detection() {
        let code = r#"
            let api_key = "sk_live_1234567890abcdef1234567890"
        "#;
        let findings = SecurityLinter::scan_source(code);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].rule_id, "WIDYA-SEC-002");
        assert_eq!(findings[0].severity, VulnerabilitySeverity::High);
    }

    #[test]
    fn test_pdp_compliance_detection() {
        let code = r#"
            fungsi simpan_nik(nik) {
                cetak("NIK Anda: " + nik)
            }
        "#;
        let findings = SecurityLinter::scan_source(code);
        assert!(!findings.is_empty());
        assert_eq!(findings[0].rule_id, "WIDYA-PDP-001");
        assert_eq!(findings[0].severity, VulnerabilitySeverity::High);
    }
}

