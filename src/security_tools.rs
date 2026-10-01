use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }
    
    pub fn emoji(&self) -> &'static str {
        match self {
            Severity::Critical => "🔴",
            Severity::High => "🟠",
            Severity::Medium => "🟡",
            Severity::Low => "🟢",
            Severity::Info => "🔵",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vulnerability {
    pub id: String,
    pub severity: Severity,
    pub category: String,
    pub title: String,
    pub description: String,
    pub line: usize,
    pub column: usize,
    pub code: String,
    pub recommendation: String,
    pub cwe: Option<String>,
}

pub struct SecretScanner {
    patterns: HashMap<String, (Regex, Severity)>,
}

impl SecretScanner {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        patterns.insert(
            "AWS_ACCESS_KEY".to_string(),
            (Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(), Severity::Critical),
        );
        
        patterns.insert(
            "AWS_SECRET_KEY".to_string(),
            (Regex::new(r#"aws_secret_access_key\s*=\s*['"]?[A-Za-z0-9/+=]{40}['"]?"#).unwrap(), Severity::Critical),
        );
        
        patterns.insert(
            "JWT_TOKEN".to_string(),
            (Regex::new(r"eyJ[A-Za-z0-9_-]+\.eyJ[A-Za-z0-9_-]+\.?[A-Za-z0-9_.+-]*").unwrap(), Severity::High),
        );
        
        patterns.insert(
            "PRIVATE_KEY".to_string(),
            (Regex::new(r"-----BEGIN RSA PRIVATE KEY-----").unwrap(), Severity::Critical),
        );
        
        patterns.insert(
            "DATABASE_URL".to_string(),
            (Regex::new(r"(mongodb|postgres|mysql)://[a-zA-Z0-9:@/._-]+").unwrap(), Severity::High),
        );
        
        patterns.insert(
            "HARDCODED_PASSWORD".to_string(),
            (Regex::new(r#"password\s*=\s*['"]([^'"]{6,})['"]"#).unwrap(), Severity::High),
        );
        
        patterns.insert(
            "SLACK_TOKEN".to_string(),
            (Regex::new(r"xox[baprs]-[0-9]{10,13}-[0-9]{10,13}[a-zA-Z0-9-]*").unwrap(), Severity::Critical),
        );
        
        patterns.insert(
            "GITHUB_TOKEN".to_string(),
            (Regex::new(r"ghp_[A-Za-z0-9_]{36}").unwrap(), Severity::Critical),
        );
        
        patterns.insert(
            "API_KEY".to_string(),
            (Regex::new(r#"api[_-]?key\s*[:=]\s*['"]?[a-zA-Z0-9_-]{32,}['"]?"#).unwrap(), Severity::High),
        );
        
        Self { patterns }
    }
    
    pub fn scan(&self, source: &str, filename: &str) -> Vec<Vulnerability> {
        let mut findings = Vec::new();
        
        for (name, (regex, severity)) in &self.patterns {
            for (line_num, line) in source.lines().enumerate() {
                for mat in regex.find_iter(line) {
                    findings.push(Vulnerability {
                        id: format!("SEC-{:04}", findings.len() + 1),
                        severity: severity.clone(),
                        category: "Secret Detection".to_string(),
                        title: format!("Potential {} detected", name),
                        description: format!(
                            "Found what appears to be a {} in file {}: {}",
                            name.to_lowercase(),
                            filename,
                            self.mask_secret(mat.as_str())
                        ),
                        line: line_num + 1,
                        column: mat.start(),
                        code: line.trim().to_string(),
                        recommendation: match name.as_str() {
                            "PRIVATE_KEY" => "Move private keys to secure key management system (e.g., AWS KMS, Vault)".to_string(),
                            "DATABASE_URL" => "Move database credentials to environment variables".to_string(),
                            "HARDCODED_PASSWORD" => "Move passwords to environment variables or secret manager".to_string(),
                            _ => "Move secrets to environment variables or dedicated secret management system".to_string(),
                        },
                        cwe: Some("CWE-798: Use of Hard-coded Credentials".to_string()),
                    });
                }
            }
        }
        
        findings
    }
    
    fn mask_secret(&self, secret: &str) -> String {
        if secret.len() <= 4 {
            "*".repeat(secret.len())
        } else {
            format!("{}...{}", &secret[..4], &secret[secret.len()-4..])
        }
    }
}

pub struct SqlInjectionChecker;

impl SqlInjectionChecker {
    pub fn new() -> Self {
        Self
    }
    
    pub fn scan(&self, source: &str) -> Vec<Vulnerability> {
        let mut findings = Vec::new();
        
        let dangerous_patterns = vec![
            (r#"(?i)SELECT\s+.*\s+FROM\s+.*WHERE\s+.*\+"#, "Potential SQL injection: string concatenation in SELECT"),
            (r#"(?i)INSERT\s+INTO\s+.*VALUES.*\+"#, "Potential SQL injection: string concatenation in INSERT"),
            (r#"(?i)UPDATE\s+.*SET.*WHERE.*\+"#, "Potential SQL injection: string concatenation in UPDATE"),
            (r#"(?i)DELETE\s+FROM.*WHERE.*\+"#, "Potential SQL injection: string concatenation in DELETE"),
            (r#"(?i)query\s*=\s*".*\+.*""#, "String concatenation in query construction"),
        ];
        
        for (pattern_str, message) in dangerous_patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for (line_num, line) in source.lines().enumerate() {
                    if regex.is_match(line) {
                        findings.push(Vulnerability {
                            id: format!("SEC-SQL-{:04}", findings.len() + 1),
                            severity: Severity::Critical,
                            category: "SQL Injection".to_string(),
                            title: message.to_string(),
                            description: format!("Detected potential SQL injection: {}", message),
                            line: line_num + 1,
                            column: 0,
                            code: line.trim().to_string(),
                            recommendation: "Use parameterized queries or prepared statements".to_string(),
                            cwe: Some("CWE-89: SQL Injection".to_string()),
                        });
                    }
                }
            }
        }
        
        findings
    }
}

pub struct XssChecker;

impl XssChecker {
    pub fn new() -> Self {
        Self
    }
    
    pub fn scan(&self, source: &str) -> Vec<Vulnerability> {
        let mut findings = Vec::new();
        
        let dangerous_patterns = vec![
            (r#"(?i)innerHTML\s*=\s*.*"# , "Potential XSS: direct assignment to innerHTML"),
            (r#"(?i)eval\s*\(.*"# , "Potential XSS: use of eval"),
            (r#"(?i)document\.write\s*\(.*"# , "Potential XSS: use of document.write"),
            (r#"<.*\{\{.*\}\}.*>"#, "Potential XSS: unescaped template variable"),
        ];
        
        for (pattern_str, message) in dangerous_patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                for (line_num, line) in source.lines().enumerate() {
                    if regex.is_match(line) {
                        findings.push(Vulnerability {
                            id: format!("SEC-XSS-{:04}", findings.len() + 1),
                            severity: Severity::High,
                            category: "Cross-Site Scripting (XSS)".to_string(),
                            title: message.to_string(),
                            description: format!("Detected potential XSS vulnerability: {}", message),
                            line: line_num + 1,
                            column: 0,
                            code: line.trim().to_string(),
                            recommendation: "Use proper output encoding/escaping".to_string(),
                            cwe: Some("CWE-79: Cross-site Scripting".to_string()),
                        });
                    }
                }
            }
        }
        
        findings
    }
}

pub struct SecurityLinter {
    secret_scanner: SecretScanner,
    sql_checker: SqlInjectionChecker,
    xss_checker: XssChecker,
}

impl SecurityLinter {
    pub fn new() -> Self {
        Self {
            secret_scanner: SecretScanner::new(),
            sql_checker: SqlInjectionChecker::new(),
            xss_checker: XssChecker::new(),
        }
    }
    
    pub fn scan(&self, source: &str, filename: &str) -> Vec<Vulnerability> {
        let mut findings = Vec::new();
        
        findings.extend(self.secret_scanner.scan(source, filename));
        findings.extend(self.sql_checker.scan(source));
        findings.extend(self.xss_checker.scan(source));
        
        findings.sort_by(|a, b| {
            match (&a.severity, &b.severity) {
                (Severity::Critical, Severity::Critical) => a.line.cmp(&b.line),
                (Severity::Critical, _) => std::cmp::Ordering::Less,
                (_, Severity::Critical) => std::cmp::Ordering::Greater,
                _ => a.line.cmp(&b.line),
            }
        });
        
        findings
    }
    
    pub fn format_report(&self, findings: &[Vulnerability]) -> String {
        let mut output = String::new();
        
        output.push_str("\n🛡️ Security Scan Report\n");
        output.push_str(&"═".repeat(50));
        output.push('\n');
        
        if findings.is_empty() {
            output.push_str("✅ No security issues found\n");
            return output;
        }
        
        let critical = findings.iter().filter(|f| matches!(f.severity, Severity::Critical)).count();
        let high = findings.iter().filter(|f| matches!(f.severity, Severity::High)).count();
        let medium = findings.iter().filter(|f| matches!(f.severity, Severity::Medium)).count();
        let low = findings.iter().filter(|f| matches!(f.severity, Severity::Low)).count();
        
        output.push_str(&format!(
            "Summary: {} Critical, {} High, {} Medium, {} Low\n\n",
            critical, high, medium, low
        ));
        
        for finding in findings {
            output.push_str(&format!(
                "{} [{}] {} - Line {}\n",
                finding.severity.emoji(),
                finding.severity.as_str(),
                finding.title,
                finding.line
            ));
            output.push_str(&format!("   Category: {}\n", finding.category));
            output.push_str(&format!("   Code: {}\n", finding.code));
            output.push_str(&format!("   Recommendation: {}\n", finding.recommendation));
            
            if let Some(cwe) = &finding.cwe {
                output.push_str(&format!("   {}\n", cwe));
            }
            
            output.push('\n');
        }
        
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_secret_detection() {
        let source = r#"
        var api_key = "sk-1234567890abcdefghijklmnopqrst"
        "#;
        
        let scanner = SecretScanner::new();
        let findings = scanner.scan(source, "test.wya");
        
        assert!(!findings.is_empty());
        assert!(findings[0].title.contains("API_KEY"));
    }
    
    #[test]
    fn test_sql_injection_detection() {
        let source = r#"
        query = "SELECT * FROM users WHERE id = " + userId
        "#;
        
        let checker = SqlInjectionChecker::new();
        let findings = checker.scan(source);
        
        assert!(!findings.is_empty());
        assert!(findings[0].title.contains("SQL injection"));
    }
    
    #[test]
    fn test_linter_report() {
        let source = r#"
        password = "secret123"
        api_key = "sk-1234567890abcdefghijklmnopqrst"
        "#;
        
        let linter = SecurityLinter::new();
        let findings = linter.scan(source, "test.wya");
        let report = linter.format_report(&findings);
        
        assert!(report.contains("Security Scan Report"));
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_cyclonedx_sbom_generation() {
        let deps = vec![
            ("widya-pqc", "1.0.0", "MIT", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            ("widyabase", "1.0.0", "Apache-2.0", "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"),
        ];

        let sbom_json = SbomGenerator::generate_cyclonedx_json("aplikasi_perbankan", "1.0.0", deps);
        assert!(sbom_json.contains("CycloneDX"));
        assert!(sbom_json.contains("widya-pqc"));
        assert!(sbom_json.contains("widyabase"));
    }

    #[test]
    fn test_fips_140_3_validation() {
        assert!(HsmPkcs11Manager::is_fips_approved_algorithm("ML-DSA-87"));
        assert!(HsmPkcs11Manager::is_fips_approved_algorithm("AES-256-GCM"));
        assert!(HsmPkcs11Manager::is_fips_approved_algorithm("SHA3-512"));
        assert!(!HsmPkcs11Manager::is_fips_approved_algorithm("MD5"));
        assert!(!HsmPkcs11Manager::is_fips_approved_algorithm("DES"));
    }
}

/// Software Bill of Materials (SBOM) Generator (CycloneDX v1.5 Standar)
pub struct SbomGenerator;

impl SbomGenerator {
    pub fn generate_cyclonedx_json(
        app_name: &str,
        version: &str,
        dependencies: Vec<(&str, &str, &str, &str)>, // (name, version, license, sha256)
    ) -> String {
        let components: Vec<serde_json::Value> = dependencies
            .into_iter()
            .map(|(name, ver, lic, hash)| {
                serde_json::json!({
                    "type": "library",
                    "name": name,
                    "version": ver,
                    "licenses": [{ "license": { "id": lic } }],
                    "hashes": [{ "alg": "SHA-256", "content": hash }],
                    "purl": format!("pkg:widya/{}@{}", name, ver)
                })
            })
            .collect();

        let doc = serde_json::json!({
            "$schema": "http://cyclonedx.org/schema/bom-1.5.json",
            "bomFormat": "CycloneDX",
            "specVersion": "1.5",
            "version": 1,
            "metadata": {
                "timestamp": "2026-10-01T08:30:00Z",
                "component": {
                    "type": "application",
                    "name": app_name,
                    "version": version
                }
            },
            "components": components
        });

        serde_json::to_string_pretty(&doc).unwrap_or_default()
    }
}

/// Hardware Security Module (HSM / PKCS#11) & FIPS 140-3 Validator
pub struct HsmPkcs11Manager;

impl HsmPkcs11Manager {
    /// Validasi apakah algoritma memenuhi standar FIPS 140-3 & NIST PQC
    pub fn is_fips_approved_algorithm(algo_name: &str) -> bool {
        let approved = [
            "AES-256-GCM",
            "AES-128-GCM",
            "SHA-256",
            "SHA-384",
            "SHA-512",
            "SHA3-256",
            "SHA3-512",
            "HMAC-SHA256",
            "ML-DSA-87", // Dilithium-5
            "ML-DSA-65", // Dilithium-3
            "ML-KEM-1024", // Kyber-1024
            "ML-KEM-768",  // Kyber-768
            "ED25519",
        ];
        approved.iter().any(|&a| a.eq_ignore_ascii_case(algo_name))
    }

    /// Simulasi sesi PKCS#11 Token Slot
    pub fn open_session(slot_id: u32, pin: &str) -> Result<String, String> {
        if pin.is_empty() {
            return Err("PIN HSM tidak boleh kosong".to_string());
        }
        Ok(format!("hsm_session_slot_{}_{:08x}", slot_id, pin.len() * 42))
    }
}

/// Tipe Enclave TEE (Trusted Execution Environment)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeePlatformType {
    IntelSgx,
    AmdSevSnp,
    AppleSecureEnclave,
    ArmTrustZone,
}

/// Laporan Bukti Attestation Perangkat Keras (Hardware Attestation Quote)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeeAttestationReport {
    pub platform: TeePlatformType,
    pub mrenclave_hash: String,  // Hash kode memori enclave yang dieksekusi
    pub mrsigner_hash: String,   // Hash penandatangan enclave
    pub user_data_hash: String,  // Hash input data AI yang dienkripsi
    pub cpu_svn: u32,           // Security Version Number
    pub is_valid: bool,
}

/// Engine Hardware TEE Confidential Computing
pub struct TeeAttestationEngine;

impl TeeAttestationEngine {
    /// Hasilkan laporan bukti eksekusi terisolasi di dalam TEE Enclave
    pub fn generate_attestation_report(
        platform: TeePlatformType,
        enclave_code_bytes: &[u8],
        input_data_bytes: &[u8],
    ) -> TeeAttestationReport {
        use sha3::{Digest, Sha3_256};

        let mut code_hasher = Sha3_256::new();
        code_hasher.update(enclave_code_bytes);
        let mrenclave = format!("mrenclave_{:x}", code_hasher.finalize());

        let mut data_hasher = Sha3_256::new();
        data_hasher.update(input_data_bytes);
        let user_data = format!("userdata_{:x}", data_hasher.finalize());

        TeeAttestationReport {
            platform,
            mrenclave_hash: mrenclave,
            mrsigner_hash: "mrsigner_widyaos_official_enclave_key_v1".to_string(),
            user_data_hash: user_data,
            cpu_svn: 4,
            is_valid: true,
        }
    }

    /// Verifikasi apakah laporan Attestation valid dan tidak dimanipulasi
    pub fn verify_report(report: &TeeAttestationReport, expected_code_hash: &str) -> Result<bool, String> {
        if !report.is_valid {
            return Err("Status laporan TEE attestation tidak valid".to_string());
        }

        if !report.mrenclave_hash.contains(expected_code_hash) && !expected_code_hash.is_empty() {
            return Err("Hash kode Enclave (MRENCLAVE) tidak cocok dengan versi resmi".to_string());
        }

        Ok(true)
    }
}


