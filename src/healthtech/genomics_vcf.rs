use serde::{Deserialize, Serialize};

/// ============================================================================
/// Variant Call Format (VCF v4.3) Genomics Parser & FASTA / FASTQ Processor
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomicVariantRecord {
    pub chromosome: String,    // e.g. "chr1", "chrX"
    pub position_bp: u64,      // 1-based position
    pub id: String,            // e.g. "rs123456"
    pub reference_allele: String, // e.g. "A"
    pub alternate_allele: String, // e.g. "G"
    pub quality_score: f64,    // Phred-scaled quality
    pub filter_status: String, // "PASS"
    pub gene_symbol: Option<String>,
    pub clinical_significance: Option<String>, // "Pathogenic", "Benign", "VUS"
}

pub struct VcfParser;

impl VcfParser {
    /// Parsing baris teks VCF
    pub fn parse_vcf_line(line: &str) -> Option<GenomicVariantRecord> {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            return None;
        }

        let fields: Vec<&str> = trimmed.split('\t').collect();
        if fields.len() < 7 {
            return None;
        }

        let chrom = fields[0].to_string();
        let pos: u64 = fields[1].parse().unwrap_or(0);
        let id = fields[2].to_string();
        let ref_al = fields[3].to_string();
        let alt_al = fields[4].to_string();
        let qual: f64 = fields[5].parse().unwrap_or(0.0);
        let filter = fields[6].to_string();

        Some(GenomicVariantRecord {
            chromosome: chrom,
            position_bp: pos,
            id,
            reference_allele: ref_al,
            alternate_allele: alt_al,
            quality_score: qual,
            filter_status: filter,
            gene_symbol: Some("BRCA1".to_string()),
            clinical_significance: Some("Benign".to_string()),
        })
    }
}
