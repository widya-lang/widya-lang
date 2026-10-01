use crate::office::docx::DocxDocument;
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

    /// Bungkus Seluruh Dokumen Massal ke dalam Satu Paket ZIP Biner
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
}
