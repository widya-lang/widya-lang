//! Enterprise Security Linter & UU PDP Compliance Scanner (security_linter.rs)
//! Mendeteksi kerentanan kode (SQL Injection, Hardcoded Secrets, Command Injection, Memory Leaks)
//! dan memvalidasi kepatuhan privasi data pribadi (UU PDP No. 27 Tahun 2022).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VulnerabilitySeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl std::fmt::Display for VulnerabilitySeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Critical => write!(f, "CRITICAL"),
            Self::High => write!(f, "HIGH"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::Low => write!(f, "LOW"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityFinding {
    pub rule_id: String,
    pub severity: VulnerabilitySeverity,
    pub message: String,
    pub line: usize,
    pub remediation: String,
}

pub struct SecurityLinter;

impl SecurityLinter {
    /// Pindai baris kode untuk potensi kerentanan keamanan, kebocoran memori, dan kepatuhan UU PDP
    pub fn scan_source(source_code: &str) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();
        let pii_keywords = [
            "nik", "nomor_ktp", "no_hp", "nomor_telepon", "rekam_medis",
            "kata_sandi", "password", "pin_atm", "cvv", "nomor_rekening", "bpjs"
        ];

        let mut in_infinite_loop = false;
        let mut loop_start_line = 0;
        let mut loop_has_break = false;
        let mut loop_mutates_collection = false;

        let lines: Vec<&str> = source_code.lines().collect();
        for (line_idx, line) in lines.iter().enumerate() {
            let line_num = line_idx + 1;
            let line_trimmed = line.trim();
            let line_lower = line_trimmed.to_lowercase();

            // Abaikan komentar
            if line_trimmed.starts_with("//") || line_trimmed.starts_with("/*") || line_trimmed.starts_with('*') {
                continue;
            }

            // 1. Deteksi Potensi SQL Injection (WIDYA-SEC-001)
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

            // 2. Deteksi Hardcoded Secret Key / Token (WIDYA-SEC-002)
            let is_secret_var = line_lower.contains("api_key")
                || line_lower.contains("secret")
                || line_lower.contains("private_key")
                || line_lower.contains("token")
                || line_lower.contains("auth_key");

            let has_assignment = line.contains("=\"") || line.contains("= \"") || line.contains(": \"") || line.contains("='") || line.contains("= '");

            let contains_key_pattern = line.contains("AKIA") // AWS Access Key
                || line.contains("eyJ") // JWT token header
                || line.contains("-----BEGIN PRIVATE KEY-----")
                || line.contains("-----BEGIN RSA PRIVATE KEY-----");

            if (is_secret_var && has_assignment && line.len() > 30) || contains_key_pattern {
                findings.push(SecurityFinding {
                    rule_id: "WIDYA-SEC-002".to_string(),
                    severity: VulnerabilitySeverity::High,
                    message: "Terdeteksi kunci rahasia / API Key yang ditulis langsung di dalam kode (Hardcoded Secret)".to_string(),
                    line: line_num,
                    remediation: "Pindahkan kredensial rahasia ke Environment Variable atau Secure Vault / Widya Secret".to_string(),
                });
            }

            // 3. Deteksi Command Injection / Unsanitized Shell Execution (WIDYA-SEC-003)
            if (line_lower.contains("eksekusi(") || line_lower.contains("shell(") || line_lower.contains("system(") || line_lower.contains("exec_cmd("))
                && (line_lower.contains(" + ") || line_lower.contains(".format("))
            {
                findings.push(SecurityFinding {
                    rule_id: "WIDYA-SEC-003".to_string(),
                    severity: VulnerabilitySeverity::Critical,
                    message: "Terdeteksi potensi Command Injection via eksekusi shell tanpa sanitasi argumen".to_string(),
                    line: line_num,
                    remediation: "Gunakan array argumen terpisah (misal: eksekusi([program, arg1, arg2])) alih-alih penggabungan string mentah".to_string(),
                });
            }

            // 4. Deteksi Insecure Deserialization / Untrusted Code Eval (WIDYA-SEC-004)
            if line_lower.contains("eval(") || line_lower.contains("evaluasi(") || line_lower.contains("eval_untrusted(") {
                findings.push(SecurityFinding {
                    rule_id: "WIDYA-SEC-004".to_string(),
                    severity: VulnerabilitySeverity::High,
                    message: "Terdeteksi penggunaan eval() / evaluasi kode dinamis yang berisiko Arbitrary Code Execution".to_string(),
                    line: line_num,
                    remediation: "Gunakan parser data aman (JSON / CBOR) dan hindari evaluasi string arbitrer sebagai kode".to_string(),
                });
            }

            // 5. Deteksi Pelanggaran UU PDP (WIDYA-PDP-001)
            for pii in &pii_keywords {
                if line_lower.contains(pii) && (line_lower.contains("print(") || line_lower.contains("cetak(") || line_lower.contains("tulis_berkas(") || line_lower.contains("log(")) {
                    findings.push(SecurityFinding {
                        rule_id: "WIDYA-PDP-001".to_string(),
                        severity: VulnerabilitySeverity::High,
                        message: format!("Pelanggaran UU PDP No. 27/2022: Data Pribadi Sensitif ('{}') dicetak/dicatat dalam log tanpa penyamaran (masking)", pii),
                        line: line_num,
                        remediation: "Terapkan penyamaran (masking: misal '3201****') atau enkripsi AES-256-GCM sesuai standar kepatuhan UU PDP".to_string(),
                    });
                }
            }

            // 6. Deteksi Potensi Kebocoran Memori (WIDYA-MEM-001)
            if line_lower.contains("selama benar") || line_lower.contains("while true") || line_lower.contains("selama 1 == 1") {
                in_infinite_loop = true;
                loop_start_line = line_num;
                loop_has_break = false;
                loop_mutates_collection = false;
            }

            if in_infinite_loop {
                if line_lower.contains("henti") || line_lower.contains("break") || line_lower.contains("kembalikan") || line_lower.contains("return") {
                    loop_has_break = true;
                }
                if line_lower.contains(".push(") || line_lower.contains(".tambah(") || line_lower.contains(".insert(") || line_lower.contains("+= [") {
                    loop_mutates_collection = true;
                }
                if line_trimmed == "}" {
                    if loop_mutates_collection && !loop_has_break {
                        findings.push(SecurityFinding {
                            rule_id: "WIDYA-MEM-001".to_string(),
                            severity: VulnerabilitySeverity::High,
                            message: "Terdeteksi potensi kebocoran memori (Unbounded Collection Growth): Loop tak hingga menambahkan data ke koleksi tanpa henti/break".to_string(),
                            line: loop_start_line,
                            remediation: "Beri batas ukuran maksimum koleksi atau tambahkan kondisi terminasi (henti/break)".to_string(),
                        });
                    }
                    in_infinite_loop = false;
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

    #[test]
    fn test_command_injection_detection() {
        let code = r#"
            fungsi jalankan(perintah_user) {
                eksekusi("rm -rf " + perintah_user);
            }
        "#;
        let findings = SecurityLinter::scan_source(code);
        assert!(findings.iter().any(|f| f.rule_id == "WIDYA-SEC-003"));
    }

    #[test]
    fn test_memory_leak_unbounded_collection_detection() {
        let code = r#"
            misal buffer = [];
            selama benar {
                buffer.tambah("data_sensor");
            }
        "#;
        let findings = SecurityLinter::scan_source(code);
        assert!(findings.iter().any(|f| f.rule_id == "WIDYA-MEM-001"));
    }
}
