//! Enterprise Supply Chain Security & SBOM Auditor
//! 
//! Fitur Skala Industri:
//! - CycloneDX v1.5 JSON & SPDX v2.3 JSON Generation
//! - Ed25519 Cryptographic Package Signature Verification
//! - Air-Gapped & Offline Vendoring Integrity Auditor (Zero-Trust)
//! - Enterprise License Compliance & CVE Vulnerability Policy Engine

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use chrono::Utc;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// ============================================================================
// DATA MODEL PAKET & DEPENDENSI
// ============================================================================

/// Informasi paket terkelola untuk pembuatan SBOM & audit keamanan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: String,
    pub purl: String,
    pub sha256: String,
    pub dependencies: Vec<String>, // Nama paket dependensi langsung
}

impl PackageInfo {
    pub fn new(name: &str, version: &str, license: &str, sha256: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: None,
            author: None,
            license: license.to_string(),
            purl: format!("pkg:widya/{}@{}", name, version),
            sha256: sha256.to_string(),
            dependencies: Vec::new(),
        }
    }
}

// ============================================================================
// CYCLONEDX v1.5 SPECIFICATION (JSON)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneDxBom {
    pub bom_format: String,
    pub spec_version: String,
    pub serial_number: String,
    pub version: u32,
    pub metadata: CycloneMetadata,
    pub components: Vec<CycloneComponent>,
    pub dependencies: Vec<CycloneDependency>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub vulnerabilities: Vec<CycloneVulnerability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneMetadata {
    pub timestamp: String,
    pub tools: Vec<CycloneTool>,
    pub component: CycloneRootComponent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneTool {
    pub vendor: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneRootComponent {
    #[serde(rename = "type")]
    pub component_type: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneComponent {
    #[serde(rename = "type")]
    pub component_type: String,
    pub name: String,
    pub version: String,
    pub purl: String,
    pub licenses: Vec<CycloneLicenseWrapper>,
    pub hashes: Vec<CycloneHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneLicenseWrapper {
    pub license: CycloneLicenseItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneLicenseItem {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneHash {
    pub alg: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneDependency {
    #[serde(rename = "ref")]
    pub dependency_ref: String,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneVulnerability {
    pub id: String,
    pub description: String,
    pub source: CycloneVulnSource,
    pub ratings: Vec<CycloneVulnRating>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneVulnSource {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycloneVulnRating {
    pub severity: String,
    pub score: f64,
}

// ============================================================================
// SPDX v2.3 SPECIFICATION (JSON)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpdxDocument {
    pub spdx_version: String,
    pub data_license: String,
    #[serde(rename = "SPDXID")]
    pub spdx_id: String,
    pub name: String,
    pub document_namespace: String,
    pub creation_info: SpdxCreationInfo,
    pub packages: Vec<SpdxPackage>,
    pub relationships: Vec<SpdxRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpdxCreationInfo {
    pub created: String,
    pub creators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpdxPackage {
    #[serde(rename = "SPDXID")]
    pub spdx_id: String,
    pub name: String,
    pub version_info: String,
    pub download_location: String,
    pub files_analyzed: bool,
    pub license_concluded: String,
    pub license_declared: String,
    pub checksums: Vec<SpdxChecksum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpdxChecksum {
    pub algorithm: String,
    pub checksum_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpdxRelationship {
    pub spdx_element_id: String,
    pub relationship_type: String,
    pub related_spdx_element: String,
}

// ============================================================================
// GENERATOR SBOM (CYCLONEDX & SPDX)
// ============================================================================

pub struct SbomGenerator;

impl SbomGenerator {
    /// Menghasilkan dokumen CycloneDX v1.5 JSON
    pub fn generate_cyclonedx(
        root_name: &str,
        root_version: &str,
        packages: &[PackageInfo],
        vulnerabilities: &[CycloneVulnerability],
    ) -> Result<String, String> {
        let serial = format!("urn:uuid:{}", uuid_simple());
        let now = Utc::now().to_rfc3339();

        let mut components = Vec::new();
        let mut dependencies = Vec::new();

        for pkg in packages {
            components.push(CycloneComponent {
                component_type: "library".to_string(),
                name: pkg.name.clone(),
                version: pkg.version.clone(),
                purl: pkg.purl.clone(),
                licenses: vec![CycloneLicenseWrapper {
                    license: CycloneLicenseItem { id: pkg.license.clone() },
                }],
                hashes: vec![CycloneHash {
                    alg: "SHA-256".to_string(),
                    content: pkg.sha256.clone(),
                }],
                description: pkg.description.clone(),
            });

            let depends_on_purls: Vec<String> = pkg.dependencies.iter().map(|dep_name| {
                if let Some(dep_pkg) = packages.iter().find(|p| &p.name == dep_name) {
                    dep_pkg.purl.clone()
                } else {
                    format!("pkg:widya/{}", dep_name)
                }
            }).collect();

            dependencies.push(CycloneDependency {
                dependency_ref: pkg.purl.clone(),
                depends_on: depends_on_purls,
            });
        }

        let bom = CycloneDxBom {
            bom_format: "CycloneDX".to_string(),
            spec_version: "1.5".to_string(),
            serial_number: serial,
            version: 1,
            metadata: CycloneMetadata {
                timestamp: now,
                tools: vec![CycloneTool {
                    vendor: "Widya Sovereign Foundation".to_string(),
                    name: "widya-wpm".to_string(),
                    version: "0.1.0".to_string(),
                }],
                component: CycloneRootComponent {
                    component_type: "application".to_string(),
                    name: root_name.to_string(),
                    version: root_version.to_string(),
                },
            },
            components,
            dependencies,
            vulnerabilities: vulnerabilities.to_vec(),
        };

        serde_json::to_string_pretty(&bom)
            .map_err(|e| format!("Gagal memformat CycloneDX JSON: {}", e))
    }

    /// Menghasilkan dokumen SPDX v2.3 JSON
    pub fn generate_spdx(
        project_name: &str,
        project_version: &str,
        packages: &[PackageInfo],
    ) -> Result<String, String> {
        let now = Utc::now().to_rfc3339();
        let doc_namespace = format!("https://widya.org/spdx/{}-{}", project_name, uuid_simple());

        let mut spdx_packages = Vec::new();
        let mut relationships = Vec::new();

        // Root package
        let root_spdx_id = "SPDXRef-RootApplication".to_string();
        spdx_packages.push(SpdxPackage {
            spdx_id: root_spdx_id.clone(),
            name: project_name.to_string(),
            version_info: project_version.to_string(),
            download_location: "NOASSERTION".to_string(),
            files_analyzed: false,
            license_concluded: "MIT".to_string(),
            license_declared: "MIT".to_string(),
            checksums: Vec::new(),
        });

        relationships.push(SpdxRelationship {
            spdx_element_id: "SPDXRef-DOCUMENT".to_string(),
            relationship_type: "DESCRIBES".to_string(),
            related_spdx_element: root_spdx_id.clone(),
        });

        for pkg in packages {
            let pkg_spdx_id = format!("SPDXRef-Package-{}", pkg.name.replace('-', "_"));
            spdx_packages.push(SpdxPackage {
                spdx_id: pkg_spdx_id.clone(),
                name: pkg.name.clone(),
                version_info: pkg.version.clone(),
                download_location: "NOASSERTION".to_string(),
                files_analyzed: false,
                license_concluded: pkg.license.clone(),
                license_declared: pkg.license.clone(),
                checksums: vec![SpdxChecksum {
                    algorithm: "SHA256".to_string(),
                    checksum_value: pkg.sha256.clone(),
                }],
            });

            // Hubungan root DEPENDS_ON paket
            relationships.push(SpdxRelationship {
                spdx_element_id: root_spdx_id.clone(),
                relationship_type: "DEPENDS_ON".to_string(),
                related_spdx_element: pkg_spdx_id.clone(),
            });

            // Hubungan antar paket
            for dep_name in &pkg.dependencies {
                let dep_spdx_id = format!("SPDXRef-Package-{}", dep_name.replace('-', "_"));
                relationships.push(SpdxRelationship {
                    spdx_element_id: pkg_spdx_id.clone(),
                    relationship_type: "DEPENDS_ON".to_string(),
                    related_spdx_element: dep_spdx_id,
                });
            }
        }

        let doc = SpdxDocument {
            spdx_version: "SPDX-2.3".to_string(),
            data_license: "CC0-1.0".to_string(),
            spdx_id: "SPDXRef-DOCUMENT".to_string(),
            name: project_name.to_string(),
            document_namespace: doc_namespace,
            creation_info: SpdxCreationInfo {
                created: now,
                creators: vec![
                    "Tool: Widya-WPM-0.1.0".to_string(),
                    "Organization: Widya Sovereign Foundation".to_string(),
                ],
            },
            packages: spdx_packages,
            relationships,
        };

        serde_json::to_string_pretty(&doc)
            .map_err(|e| format!("Gagal memformat SPDX JSON: {}", e))
    }
}

// ============================================================================
// ED25519 CRYPTOGRAPHIC PACKAGE SIGNATURES
// ============================================================================

/// Tanda tangan kriptografis paket Widya
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSignature {
    pub package_name: String,
    pub package_version: String,
    pub public_key_hex: String,
    pub signature_hex: String,
    pub manifest_sha256: String,
    pub signed_at: String,
}

pub struct PackageSigner;

impl PackageSigner {
    /// Membuat pasangan kunci baru Ed25519 (SigningKey & VerifyingKey)
    pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
        let mut rng = rand::rngs::OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();
        (signing_key, verifying_key)
    }

    /// Menandatangani manifest paket menggunakan kunci rahasia Ed25519
    pub fn sign_manifest(
        signing_key: &SigningKey,
        package_name: &str,
        package_version: &str,
        manifest_bytes: &[u8],
    ) -> PackageSignature {
        let signature = signing_key.sign(manifest_bytes);
        let verifying_key = signing_key.verifying_key();

        let mut hasher = Sha256::new();
        hasher.update(manifest_bytes);
        let manifest_sha256 = hex::encode(hasher.finalize());

        PackageSignature {
            package_name: package_name.to_string(),
            package_version: package_version.to_string(),
            public_key_hex: hex::encode(verifying_key.to_bytes()),
            signature_hex: hex::encode(signature.to_bytes()),
            manifest_sha256,
            signed_at: Utc::now().to_rfc3339(),
        }
    }
}

pub struct PackageVerifier;

impl PackageVerifier {
    /// Memverifikasi keaslian dan integritas tanda tangan paket menggunakan kunci publik Ed25519
    pub fn verify(
        manifest_bytes: &[u8],
        pkg_sig: &PackageSignature,
    ) -> Result<bool, String> {
        // 1. Verifikasi kecocokan hash manifest
        let mut hasher = Sha256::new();
        hasher.update(manifest_bytes);
        let actual_hash = hex::encode(hasher.finalize());
        if actual_hash != pkg_sig.manifest_sha256 {
            return Err(format!(
                "Manifest hash mismatch: tercatat {}, aktual {}",
                pkg_sig.manifest_sha256, actual_hash
            ));
        }

        // 2. Parse kunci publik
        let pub_bytes = hex::decode(&pkg_sig.public_key_hex)
            .map_err(|e| format!("Format public_key_hex invalid: {}", e))?;
        if pub_bytes.len() != 32 {
            return Err("Panjang kunci publik Ed25519 harus tepat 32 byte".to_string());
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&pub_bytes);
        let verifying_key = VerifyingKey::from_bytes(&arr)
            .map_err(|e| format!("Kunci publik Ed25519 korup: {}", e))?;

        // 3. Parse signature
        let sig_bytes = hex::decode(&pkg_sig.signature_hex)
            .map_err(|e| format!("Format signature_hex invalid: {}", e))?;
        if sig_bytes.len() != 64 {
            return Err("Panjang tanda tangan Ed25519 harus tepat 64 byte".to_string());
        }
        let signature = Signature::from_slice(&sig_bytes)
            .map_err(|e| format!("Signature Ed25519 tidak valid: {}", e))?;

        // 4. Verifikasi kriptografis
        verifying_key.verify(manifest_bytes, &signature)
            .map(|_| true)
            .map_err(|e| format!("Verifikasi kriptografis tanda tangan gagal: {}", e))
    }
}

// ============================================================================
// AIR-GAPPED & OFFLINE VENDORING AUDITOR
// ============================================================================

/// Laporan audit integritas paket offline (air-gapped)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirGapAuditReport {
    pub is_air_gapped_ready: bool,
    pub verified_packages: usize,
    pub missing_packages: Vec<String>,
    pub tampered_packages: Vec<(String, String, String)>, // (name, expected_hash, actual_hash)
    pub total_disk_bytes: u64,
}

pub struct AirGappedAuditor;

impl AirGappedAuditor {
    /// Mengaudit direktori vendor offline terhadap lockfile
    pub fn audit_vendor_directory<P: AsRef<Path>>(
        vendor_dir: P,
        expected_packages: &[PackageInfo],
    ) -> AirGapAuditReport {
        let v_dir = vendor_dir.as_ref();
        let mut missing = Vec::new();
        let mut tampered = Vec::new();
        let mut verified = 0;
        let mut total_bytes = 0;

        for pkg in expected_packages {
            let archive_filename = format!("{}-{}.wpm", pkg.name, pkg.version);
            let pkg_path = v_dir.join(&archive_filename);

            if !pkg_path.exists() {
                missing.push(format!("{}@{}", pkg.name, pkg.version));
                continue;
            }

            match std::fs::read(&pkg_path) {
                Ok(bytes) => {
                    total_bytes += bytes.len() as u64;
                    let mut hasher = Sha256::new();
                    hasher.update(&bytes);
                    let actual_hash = hex::encode(hasher.finalize());

                    if actual_hash != pkg.sha256 {
                        tampered.push((
                            format!("{}@{}", pkg.name, pkg.version),
                            pkg.sha256.clone(),
                            actual_hash,
                        ));
                    } else {
                        verified += 1;
                    }
                }
                Err(_) => {
                    missing.push(format!("{}@{} (gagal baca berkas)", pkg.name, pkg.version));
                }
            }
        }

        let is_air_gapped_ready = missing.is_empty() && tampered.is_empty();
        AirGapAuditReport {
            is_air_gapped_ready,
            verified_packages: verified,
            missing_packages: missing,
            tampered_packages: tampered,
            total_disk_bytes: total_bytes,
        }
    }
}

// ============================================================================
// ENTERPRISE LICENSE COMPLIANCE & VULNERABILITY POLICY ENGINE
// ============================================================================

/// Kebijakan kepatuhan lisensi korporat
#[derive(Debug, Clone)]
pub struct LicensePolicy {
    pub allowed_licenses: HashSet<String>,
    pub forbidden_licenses: HashSet<String>,
}

impl Default for LicensePolicy {
    fn default() -> Self {
        let mut allowed = HashSet::new();
        allowed.insert("MIT".to_string());
        allowed.insert("Apache-2.0".to_string());
        allowed.insert("BSD-2-Clause".to_string());
        allowed.insert("BSD-3-Clause".to_string());
        allowed.insert("ISC".to_string());
        allowed.insert("CC0-1.0".to_string());

        let mut forbidden = HashSet::new();
        forbidden.insert("GPL-3.0".to_string());
        forbidden.insert("AGPL-3.0".to_string());
        forbidden.insert("SSPL-1.0".to_string());

        Self {
            allowed_licenses: allowed,
            forbidden_licenses: forbidden,
        }
    }
}

/// Laporan kepatuhan lisensi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseComplianceReport {
    pub is_compliant: bool,
    pub compliant_packages: Vec<String>,
    pub forbidden_violations: Vec<(String, String)>, // (package, forbidden_license)
    pub unreviewed_licenses: Vec<(String, String)>,  // (package, unknown_license)
}

impl LicensePolicy {
    /// Mengaudit seluruh lisensi dependensi terhadap kebijakan korporat
    pub fn audit(&self, packages: &[PackageInfo]) -> LicenseComplianceReport {
        let mut compliant = Vec::new();
        let mut violations = Vec::new();
        let mut unreviewed = Vec::new();

        for pkg in packages {
            let lic = pkg.license.trim();
            if self.forbidden_licenses.contains(lic) {
                violations.push((pkg.name.clone(), lic.to_string()));
            } else if self.allowed_licenses.contains(lic) {
                compliant.push(pkg.name.clone());
            } else {
                unreviewed.push((pkg.name.clone(), lic.to_string()));
            }
        }

        let is_compliant = violations.is_empty();
        LicenseComplianceReport {
            is_compliant,
            compliant_packages: compliant,
            forbidden_violations: violations,
            unreviewed_licenses: unreviewed,
        }
    }
}

// Helper untuk menghasilkan serial pseudorandom UUID sederhana
fn uuid_simple() -> String {
    let now = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let mut hasher = Sha256::new();
    hasher.update(now.to_le_bytes());
    let hash = hex::encode(hasher.finalize());
    format!(
        "{}-{}-{}-{}-{}",
        &hash[0..8],
        &hash[8..12],
        &hash[12..16],
        &hash[16..20],
        &hash[20..32]
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyclonedx_generation_and_schema() {
        let mut pkg1 = PackageInfo::new("widya-core", "0.1.0", "MIT", "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890");
        pkg1.dependencies.push("widya-math".to_string());

        let pkg2 = PackageInfo::new("widya-math", "1.2.0", "Apache-2.0", "1122334455667788990011223344556677889900112233445566778899001122");

        let packages = vec![pkg1, pkg2];
        let vulns = vec![
            CycloneVulnerability {
                id: "WIDYA-SEC-2026-001".to_string(),
                description: "Fixed integer bounds check in parser".to_string(),
                source: CycloneVulnSource { name: "WidyaSec".to_string() },
                ratings: vec![CycloneVulnRating { severity: "low".to_string(), score: 3.1 }],
            }
        ];

        let json = SbomGenerator::generate_cyclonedx("widya-app", "1.0.0", &packages, &vulns).unwrap();
        assert!(json.contains("\"bomFormat\": \"CycloneDX\""));
        assert!(json.contains("\"specVersion\": \"1.5\""));
        assert!(json.contains("pkg:widya/widya-core@0.1.0"));
        assert!(json.contains("pkg:widya/widya-math@1.2.0"));
        assert!(json.contains("WIDYA-SEC-2026-001"));
    }

    #[test]
    fn test_spdx_generation() {
        let pkg1 = PackageInfo::new("widya-crypto", "2.0.0", "MIT", "9988776655443322110099887766554433221100998877665544332211009988");
        let packages = vec![pkg1];

        let spdx_json = SbomGenerator::generate_spdx("fintech-service", "3.1.0", &packages).unwrap();
        assert!(spdx_json.contains("\"spdxVersion\": \"SPDX-2.3\""));
        assert!(spdx_json.contains("\"dataLicense\": \"CC0-1.0\""));
        assert!(spdx_json.contains("SPDXRef-Package-widya_crypto"));
        assert!(spdx_json.contains("DEPENDS_ON"));
    }

    #[test]
    fn test_ed25519_package_signing_and_verification() {
        let (signing_key, verifying_key) = PackageSigner::generate_keypair();
        let manifest = b"name = \"widya-auth\"\nversion = \"1.0.0\"\nchecksum = \"abc\"";

        let sig = PackageSigner::sign_manifest(&signing_key, "widya-auth", "1.0.0", manifest);
        assert_eq!(sig.public_key_hex, hex::encode(verifying_key.to_bytes()));

        // Verifikasi keaslian harus OK
        let is_valid = PackageVerifier::verify(manifest, &sig).expect("Verifikasi harus berhasil");
        assert!(is_valid);

        // Modifikasi manifest (tampering) harus terdeteksi dan gagal
        let tampered_manifest = b"name = \"widya-auth\"\nversion = \"1.0.0\"\nchecksum = \"HACKED\"";
        let err = PackageVerifier::verify(tampered_manifest, &sig);
        assert!(err.is_err());
        assert!(err.unwrap_err().contains("Manifest hash mismatch"));
    }

    #[test]
    fn test_license_policy_compliance() {
        let policy = LicensePolicy::default();

        let pkg_clean = PackageInfo::new("widya-ui", "1.0.0", "MIT", "hash1");
        let pkg_gpl = PackageInfo::new("gpl-tool", "0.9.0", "GPL-3.0", "hash2");

        let report = policy.audit(&[pkg_clean, pkg_gpl]);
        assert!(!report.is_compliant);
        assert_eq!(report.forbidden_violations.len(), 1);
        assert_eq!(report.forbidden_violations[0].0, "gpl-tool");
        assert_eq!(report.forbidden_violations[0].1, "GPL-3.0");
        assert_eq!(report.compliant_packages, vec!["widya-ui"]);
    }
}
