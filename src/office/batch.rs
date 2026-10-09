use crate::office::docx::DocxDocument;
use crate::office::markdown::MarkdownDocument;
use crate::office::converter::DocumentConverter;
use crate::office::pdf::PdfDocument;
use crate::office::qrcode::WidyaQrCode;
use crate::office::zip::ZipPackage;
use serde::{Deserialize, Serialize};
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
