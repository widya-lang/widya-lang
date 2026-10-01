use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// STANDAR BIBLIOGRAFI INTERNASIONAL (MARC21, DUBLIN CORE, DDC, ISBN, DOI)
/// Format Standar Mesin-Baca untuk Perpustakaan Sekolah s.d Internasional
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marc21Field {
    pub tag: String,              // e.g. "020" (ISBN), "100" (Pengarang), "245" (Judul), "650" (Topik)
    pub ind1: char,               // Indikator 1
    pub ind2: char,               // Indikator 2
    pub subfields: HashMap<char, String>, // Subfields e.g. 'a' => Judul, 'c' => Pernyataan Tanggung Jawab
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marc21Record {
    pub leader: String,           // 24-character MARC leader
    pub control_number_001: String,
    pub fields: Vec<Marc21Field>,
}

impl Marc21Record {
    pub fn new(control_number: &str) -> Self {
        Self {
            leader: "00000nam a2200000 i 4500".to_string(),
            control_number_001: control_number.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn tambah_field(&mut self, tag: &str, ind1: char, ind2: char, subfields: Vec<(char, &str)>) {
        let mut map = HashMap::new();
        for (k, v) in subfields {
            map.insert(k, v.to_string());
        }
        self.fields.push(Marc21Field {
            tag: tag.to_string(),
            ind1,
            ind2,
            subfields: map,
        });
    }

    pub fn get_subfield(&self, tag: &str, subfield_code: char) -> Option<String> {
        for f in &self.fields {
            if f.tag == tag {
                if let Some(val) = f.subfields.get(&subfield_code) {
                    return Some(val.clone());
                }
            }
        }
        None
    }
}

/// ----------------------------------------------------------------------------
/// STANDAR DUBLIN CORE METADATA (DCMI - 15 Unsur Standar Global)
/// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DublinCoreMetadata {
    pub title: String,            // Judul
    pub creator: Vec<String>,     // Pengarang / Pencipta
    pub subject: Vec<String>,     // Kata Kunci / Subjek DDC
    pub description: String,      // Abstrak / Deskripsi Fisik
    pub publisher: String,        // Penerbit
    pub contributor: Vec<String>, // Editor / Pembimbing / Penerjemah
    pub date: String,             // Tanggal Publikasi (ISO 8601)
    pub type_resource: String,    // Jenis (Buku, Skripsi, Jurnal, Naskah Kuno)
    pub format: String,           // Format (application/pdf, text/html, printed)
    pub identifier: String,       // ISBN / ISSN / DOI / URI
    pub source: Option<String>,   // Sumber rujukan
    pub language: String,         // Bahasa (ind, eng, jav, ara)
    pub relation: Option<String>, // Relasi koleksi
    pub coverage: Option<String>, // Cakupan spasial / temporal
    pub rights: String,           // Hak Cipta / Lisensi (Open Access / CC-BY / All Rights Reserved)
}

/// ----------------------------------------------------------------------------
/// KLASIFIKASI DDC (DEWEY DECIMAL CLASSIFICATION)
/// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KlasifikasiDdc {
    pub nomor_kelas: String,      // e.g. "005.133" (Rust/Pemrograman), "297" (Islam), "370" (Pendidikan)
    pub label_divisi: String,     // e.g. "Ilmu Komputer, Informasi & Karya Umum"
    pub rak_lokasi: String,       // Lokasi fisik di perpustakaan
}
