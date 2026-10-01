use serde::{Deserialize, Serialize};

/// ============================================================================
/// ANALISIS BIBLIOMETRIK, H-INDEX, SITASI & EKSPOR SITASI MULTI-FORMAT
/// Algoritma Hirsch H-Index, i10-Index, APA 7th, IEEE, BibTeX & RIS Formatter
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiwayatPublikasiSitasi {
    pub id_artikel: String,
    pub judul: String,
    pub tahun: u32,
    pub jumlah_sitasi: u32,
}

pub struct BibliometricEngine;

impl BibliometricEngine {
    /// Menghitung Hirsch H-Index dari daftar publikasi dan sitasi
    /// Definisi: Seorang peneliti memiliki indeks h jika h dari N artikelnya memiliki
    /// setidaknya h sitasi masing-masing, dan (N - h) artikel lainnya memiliki <= h sitasi.
    pub fn hitung_h_index(mut daftar_sitasi: Vec<u32>) -> u32 {
        daftar_sitasi.sort_by(|a, b| b.cmp(a)); // Urutkan descending
        let mut h_index = 0;
        for (i, &sitasi) in daftar_sitasi.iter().enumerate() {
            let rank = (i + 1) as u32;
            if sitasi >= rank {
                h_index = rank;
            } else {
                break;
            }
        }
        h_index
    }

    /// Menghitung i10-Index (Jumlah artikel dengan minimal 10 sitasi)
    pub fn hitung_i10_index(daftar_sitasi: &[u32]) -> u32 {
        daftar_sitasi.iter().filter(|&&s| s >= 10).count() as u32
    }

    /// Format Sitasi Standar APA 7th Edition
    pub fn format_apa_7th(
        penulis: &str,
        tahun: u32,
        judul_artikel: &str,
        nama_jurnal: &str,
        volume: u32,
        issue: u32,
        hal_awal: u32,
        hal_akhir: u32,
        doi: &str,
    ) -> String {
        format!(
            "{}. ({}). {}. {}, {}({}), {}-{}. https://doi.org/{}",
            penulis, tahun, judul_artikel, nama_jurnal, volume, issue, hal_awal, hal_akhir, doi
        )
    }

    /// Format Sitasi Standar IEEE
    pub fn format_ieee(
        penulis: &str,
        judul_artikel: &str,
        nama_jurnal: &str,
        volume: u32,
        issue: u32,
        hal_awal: u32,
        hal_akhir: u32,
        tahun: u32,
        doi: &str,
    ) -> String {
        format!(
            "{}, \"{},\" {}, vol. {}, no. {}, pp. {}-{}, {}, doi: {}.",
            penulis, judul_artikel, nama_jurnal, volume, issue, hal_awal, hal_akhir, tahun, doi
        )
    }

    /// Format Sitasi BibTeX
    pub fn format_bibtex(
        citation_key: &str,
        penulis: &str,
        judul: &str,
        jurnal: &str,
        volume: u32,
        number: u32,
        pages: &str,
        year: u32,
        doi: &str,
    ) -> String {
        format!(
            "@article{{{},\n  author = {{{}}},\n  title = {{{}}},\n  journal = {{{}}},\n  volume = {{{}}},\n  number = {{{}}},\n  pages = {{{}}},\n  year = {{{}}},\n  doi = {{{}}}\n}}",
            citation_key, penulis, judul, jurnal, volume, number, pages, year, doi
        )
    }

    /// Format Sitasi RIS (Research Information Systems)
    pub fn format_ris(
        penulis: &str,
        judul: &str,
        jurnal: &str,
        volume: u32,
        issue: u32,
        hal_awal: u32,
        hal_akhir: u32,
        tahun: u32,
        doi: &str,
    ) -> String {
        format!(
            "TY  - JOUR\nAU  - {}\nTI  - {}\nJO  - {}\nVL  - {}\nIS  - {}\nSP  - {}\nEP  - {}\nPY  - {}\nDO  - {}\nER  - ",
            penulis, judul, jurnal, volume, issue, hal_awal, hal_akhir, tahun, doi
        )
    }
}
