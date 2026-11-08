use crate::office::docx::DocxDocument;
use crate::office::markdown::MarkdownDocument;
use crate::office::converter::DocumentConverter;
use crate::office::pdf::PdfDocument;
use crate::office::qrcode::WidyaQrCode;
use crate::office::zip::ZipPackage;
use crate::kripto::pqc::{PqcEngine, PqcSignature};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
use std::time::Instant;

/// Laporan Metrik Kinerja Batch Generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGenerationMetrics {
    pub total_documents: usize,
    pub elapsed_millis: u128,
    pub docs_per_second: f64,
    pub total_bytes: usize,
}

/// Mesin Cetak Massal Berkecepatan Tinggi (Batch Document Engine)
pub struct BatchDocumentEngine;

impl BatchDocumentEngine {
    /// Mengurai string CSV menjadi kumpulan record HashMap
    pub fn parse_csv_records(csv_text: &str) -> Vec<HashMap<String, String>> {
        let mut records = Vec::new();
        let lines: Vec<&str> = csv_text.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
        if lines.is_empty() {
            return records;
        }

        let headers: Vec<String> = lines[0]
            .split(',')
            .map(|h| h.trim().trim_matches('"').to_string())
            .collect();

        for line in &lines[1..] {
            let cols: Vec<&str> = line.split(',').collect();
            let mut map = HashMap::new();
            for (i, h) in headers.iter().enumerate() {
                let val = cols.get(i).map(|s| s.trim().trim_matches('"')).unwrap_or("");
                map.insert(h.clone(), val.to_string());
            }
            records.push(map);
        }

        records
    }

    /// Hasilkan N Dokumen Word Secara Cepat dari Master Template & Kumpulan Data
    pub fn generate_batch_docx(
        template: &DocxDocument,
        records: &[HashMap<String, String>],
    ) -> (Vec<DocxDocument>, BatchGenerationMetrics) {
        let start = Instant::now();
        let mut results = Vec::with_capacity(records.len());
        let mut total_bytes = 0;

        for record in records {
            let mut doc = template.clone();
            doc.apply_template(record);
            let bytes = doc.export_to_docx_bytes();
            total_bytes += bytes.len();
            results.push(doc);
        }

        let elapsed = start.elapsed().as_millis();
        let elapsed_sec = elapsed.max(1) as f64 / 1000.0;
        let docs_per_second = records.len() as f64 / elapsed_sec;

        let metrics = BatchGenerationMetrics {
            total_documents: records.len(),
            elapsed_millis: elapsed,
            docs_per_second,
            total_bytes,
        };

        (results, metrics)
    }

    /// Hasilkan N Dokumen PDF Resmi dari Template Markdown & Kumpulan Data dengan QR Code opsional
    pub fn generate_batch_pdf(
        template_md: &str,
        records: &[HashMap<String, String>],
        qr_field: Option<&str>,
    ) -> (Vec<PdfDocument>, BatchGenerationMetrics) {
        let start = Instant::now();
        let mut results = Vec::with_capacity(records.len());
        let mut total_bytes = 0;

        for record in records {
            let mut rendered_md = template_md.to_string();
            for (k, v) in record {
                rendered_md = rendered_md.replace(&format!("{{{{{}}}}}", k), v);
            }

            let md = MarkdownDocument::parse(&rendered_md);
            let mut pdf = DocumentConverter::markdown_to_pdf(&md);

            // Jika field QR Code disediakan, sematkan QR Code di pojok bawah halaman akhir
            if let Some(field) = qr_field {
                if let Some(qr_data) = record.get(field) {
                    if !qr_data.is_empty() {
                        if let Ok(qr) = WidyaQrCode::encode(qr_data) {
                            let qr_size = 50.0;
                            let margin_x = 50.0;
                            let current_y = 120.0;
                            qr.render_to_pdf(&mut pdf, margin_x, current_y, qr_size);
                            pdf.current_page_mut().add_styled_text(
                                "Pindai Verifikasi",
                                margin_x,
                                current_y - qr_size - 10.0,
                                7.0,
                                false,
                                (0.3, 0.4, 0.5),
                            );
                        }
                    }
                }
            }

            let bytes = pdf.generate_pdf_bytes();
            total_bytes += bytes.len();
            results.push(pdf);
        }

        let elapsed = start.elapsed().as_millis();
        let elapsed_sec = elapsed.max(1) as f64 / 1000.0;
        let docs_per_second = records.len() as f64 / elapsed_sec;

        let metrics = BatchGenerationMetrics {
            total_documents: records.len(),
            elapsed_millis: elapsed,
            docs_per_second,
            total_bytes,
        };

        (results, metrics)
    }

    /// Bungkus Seluruh Dokumen Massal DOCX ke dalam Satu Paket ZIP Biner
    pub fn export_batch_to_zip(
        template: &DocxDocument,
        records: &[HashMap<String, String>],
        file_prefix: &str,
    ) -> (Vec<u8>, BatchGenerationMetrics) {
        let start = Instant::now();
        let mut zip = ZipPackage::new();

        for (i, record) in records.iter().enumerate() {
            let mut doc = template.clone();
            doc.apply_template(record);
            let bytes = doc.export_to_docx_bytes();

            let filename = format!("{}_{:04}.docx", file_prefix, i + 1);
            zip.add_file(&filename, &bytes);
        }

        let zip_bytes = zip.to_bytes();
        let elapsed = start.elapsed().as_millis();
        let elapsed_sec = elapsed.max(1) as f64 / 1000.0;
        let docs_per_second = records.len() as f64 / elapsed_sec;

        let metrics = BatchGenerationMetrics {
            total_documents: records.len(),
            elapsed_millis: elapsed,
            docs_per_second,
            total_bytes: zip_bytes.len(),
        };

        (zip_bytes, metrics)
    }

    /// Bungkus Seluruh Dokumen Massal PDF ke dalam Satu Paket ZIP Biner
    pub fn export_batch_pdf_to_zip(
        template_md: &str,
        records: &[HashMap<String, String>],
        file_prefix: &str,
        qr_field: Option<&str>,
    ) -> (Vec<u8>, BatchGenerationMetrics) {
        let start = Instant::now();
        let mut zip = ZipPackage::new();
        let mut total_bytes = 0;

        for (i, record) in records.iter().enumerate() {
            let mut rendered_md = template_md.to_string();
            for (k, v) in record {
                rendered_md = rendered_md.replace(&format!("{{{{{}}}}}", k), v);
            }

            let md = MarkdownDocument::parse(&rendered_md);
            let mut pdf = DocumentConverter::markdown_to_pdf(&md);

            if let Some(field) = qr_field {
                if let Some(qr_data) = record.get(field) {
                    if !qr_data.is_empty() {
                        if let Ok(qr) = WidyaQrCode::encode(qr_data) {
                            let qr_size = 50.0;
                            let margin_x = 50.0;
                            let current_y = 120.0;
                            qr.render_to_pdf(&mut pdf, margin_x, current_y, qr_size);
                            pdf.current_page_mut().add_styled_text(
                                "Pindai Verifikasi",
                                margin_x,
                                current_y - qr_size - 10.0,
                                7.0,
                                false,
                                (0.3, 0.4, 0.5),
                            );
                        }
                    }
                }
            }

            let pdf_bytes = pdf.generate_pdf_bytes();
            total_bytes += pdf_bytes.len();
            let filename = format!("{}_{:04}.pdf", file_prefix, i + 1);
            zip.add_file(&filename, &pdf_bytes);
        }

        let zip_bytes = zip.to_bytes();
        let elapsed = start.elapsed().as_millis();
        let elapsed_sec = elapsed.max(1) as f64 / 1000.0;
        let docs_per_second = records.len() as f64 / elapsed_sec;

        let metrics = BatchGenerationMetrics {
            total_documents: records.len(),
            elapsed_millis: elapsed,
            docs_per_second,
            total_bytes,
        };

        (zip_bytes, metrics)
    }
}

/// Item Dokumen untuk Penandatanganan Massal Kriptografi Pasca-Kuantum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPqcDocumentItem {
    pub id: String,
    pub title: String,
    pub content: String,
}

/// Hasil Penandatanganan PQC Per Dokumen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPqcSignedItem {
    pub id: String,
    pub title: String,
    pub signature_hex: String,
    pub public_key_hex: String,
    pub salt_hex: String,
    pub timestamp_ms: u64,
    pub algorithm: String,
    pub sha256_hash: String,
    pub is_valid: bool,
}

/// Metrik Kinerja Penandatanganan PQC Massal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPqcSigningMetrics {
    pub total_documents: usize,
    pub elapsed_millis: u128,
    pub signatures_per_second: f64,
    pub algorithm: String,
    pub all_verified_valid: bool,
}

/// Mesin Penandatanganan Pasca-Kuantum Massal (NIST FIPS 204 ML-DSA-87)
pub struct BatchPqcSigner;

impl BatchPqcSigner {
    /// Menandatangani daftar dokumen secara massal dengan kunci pasca-kuantum ML-DSA-87
    pub fn sign_batch(
        items: &[BatchPqcDocumentItem],
        seed: &str,
    ) -> (Vec<BatchPqcSignedItem>, BatchPqcSigningMetrics) {
        let start = Instant::now();
        let keypair = PqcEngine::generate_mldsa87_keypair(seed);
        let mut results = Vec::with_capacity(items.len());

        for item in items {
            let mut hasher = sha2::Sha256::new();
            hasher.update(item.content.as_bytes());
            let sha256_hash = hex::encode(hasher.finalize());

            let sig = PqcEngine::sign_mldsa(item.content.as_bytes(), &keypair.private_key_hex);
            let is_valid = PqcEngine::verify_mldsa(item.content.as_bytes(), &sig, &keypair.public_key_hex);

            results.push(BatchPqcSignedItem {
                id: item.id.clone(),
                title: item.title.clone(),
                signature_hex: sig.signature_hex,
                public_key_hex: keypair.public_key_hex.clone(),
                salt_hex: sig.salt_hex,
                timestamp_ms: sig.timestamp_ms,
                algorithm: keypair.algorithm.clone(),
                sha256_hash,
                is_valid,
            });
        }

        let elapsed = start.elapsed().as_millis();
        let elapsed_sec = elapsed.max(1) as f64 / 1000.0;
        let signatures_per_second = items.len() as f64 / elapsed_sec;
        let all_verified_valid = results.iter().all(|r| r.is_valid);

        let metrics = BatchPqcSigningMetrics {
            total_documents: items.len(),
            elapsed_millis: elapsed,
            signatures_per_second,
            algorithm: keypair.algorithm,
            all_verified_valid,
        };

        (results, metrics)
    }

    /// Verifikasi integritas tanda tangan seluruh berkas yang telah ditandatangani
    pub fn verify_batch(
        items: &[BatchPqcSignedItem],
        original_contents: &HashMap<String, String>,
    ) -> (bool, usize, usize) {
        let mut valid_count = 0;
        let mut invalid_count = 0;

        for item in items {
            let content = original_contents.get(&item.id).map(|s| s.as_str()).unwrap_or("");
            let sig = PqcSignature {
                algorithm: item.algorithm.clone(),
                signature_hex: item.signature_hex.clone(),
                timestamp_ms: item.timestamp_ms,
                salt_hex: item.salt_hex.clone(),
            };
            if PqcEngine::verify_mldsa(content.as_bytes(), &sig, &item.public_key_hex) {
                valid_count += 1;
            } else {
                invalid_count += 1;
            }
        }

        (invalid_count == 0, valid_count, invalid_count)
    }
}

