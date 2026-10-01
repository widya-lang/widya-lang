use super::marc21_dublin_core::DublinCoreMetadata;
use serde::{Deserialize, Serialize};

/// ============================================================================
/// SISTEM PERPUSTAKAAN PERGURUAN TINGGI (INSTITUTIONAL REPOSITORY & OAI-PMH)
/// RAMA / GARUDA Kemendiktisaintek, Crossref DOI & Deteksi Anti-Plagiarisme
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JenisKaryaIlmiah {
    SkripsiSarjana,
    TesisMagister,
    DisertasiDoktor,
    ArtikelJurnalInternasional,
    LaporanPenelitianDosen,
    BukuAjarPendidikanTinggi,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KaryaIlmiahRepository {
    pub id_karya: String,
    pub nim_nidn_penulis: String,
    pub nama_penulis: String,
    pub program_studi: String,
    pub fakultas: String,
    pub jenis_karya: JenisKaryaIlmiah,
    pub metadata_dc: DublinCoreMetadata,
    pub persentase_kemiripan_plagiarisme: f64, // e.g. 12.5% (Ambang batas max: 20.0%)
    pub lolos_uji_kemiripan: bool,
    pub doi_crossref: Option<String>,
    pub status_oai_pmh_harvestable: bool,      // Siap di-harvest oleh RAMA / GARUDA
    pub aksesibilitas: String,                 // "Open Access", "Restricted / Embargo 12 Bulan"
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponOaiPmhRecord {
    pub oai_identifier: String,  // e.g. "oai:repository.kampus.ac.id:123456"
    pub datestamp: String,       // ISO 8601
    pub set_spec: String,        // e.g. "com_skripsi_fasilkom"
    pub dublin_core_xml: String, // Representasi XML OAI-DC
}

pub struct PerpustakaanTinggiEngine {
    pub kode_pt: String,          // e.g. "001002" (Kode PDDIKTI)
    pub nama_universitas: String,
    pub repositori_karya: Vec<KaryaIlmiahRepository>,
    pub oai_pmh_base_url: String,
}

impl PerpustakaanTinggiEngine {
    pub fn new(kode_pt: &str, nama_univ: &str, base_url: &str) -> Self {
        Self {
            kode_pt: kode_pt.to_string(),
            nama_universitas: nama_univ.to_string(),
            repositori_karya: Vec::new(),
            oai_pmh_base_url: base_url.to_string(),
        }
    }

    /// Unggah karya ilmiah & uji kemiripan plagiarisme otomatis
    pub fn unggah_karya_ilmiah(
        &mut self,
        id_karya: &str,
        nim_nidn: &str,
        penulis: &str,
        prodi: &str,
        fakultas: &str,
        jenis: JenisKaryaIlmiah,
        dc: DublinCoreMetadata,
        similarity_persen: f64,
        doi: Option<&str>,
    ) -> Result<KaryaIlmiahRepository, String> {
        let lolos = similarity_persen <= 20.0; // Maksimal toleransi 20%

        let karya = KaryaIlmiahRepository {
            id_karya: id_karya.to_string(),
            nim_nidn_penulis: nim_nidn.to_string(),
            nama_penulis: penulis.to_string(),
            program_studi: prodi.to_string(),
            fakultas: fakultas.to_string(),
            jenis_karya: jenis,
            metadata_dc: dc,
            persentase_kemiripan_plagiarisme: similarity_persen,
            lolos_uji_kemiripan: lolos,
            doi_crossref: doi.map(|d| d.to_string()),
            status_oai_pmh_harvestable: lolos, // Hanya karya lolos plagiarisme yang dipublikasikan
            aksesibilitas: if lolos { "Open Access".to_string() } else { "DitahanPerbaikan".to_string() },
        };

        if !lolos {
            return Err(format!(
                "Tingkat kemiripan ({:.1}%) melebihi ambang batas maksimal 20.0%! Karya perlu revisi parafrase.",
                similarity_persen
            ));
        }

        self.repositori_karya.push(karya.clone());
        Ok(karya)
    }

    /// Harvest metadata OAI-PMH untuk integrasi Portal RAMA / GARUDA Kemendiktisaintek
    pub fn harvest_oai_pmh(&self, set_filter: Option<&str>) -> Vec<ResponOaiPmhRecord> {
        let mut hasil = Vec::new();
        for k in &self.repositori_karya {
            if k.status_oai_pmh_harvestable {
                let xml = format!(
                    "<oai_dc:dc xmlns:oai_dc=\"http://www.openarchives.org/OAI/2.0/oai_dc/\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\">\n  <dc:title>{}</dc:title>\n  <dc:creator>{}</dc:creator>\n  <dc:identifier>{}</dc:identifier>\n  <dc:date>{}</dc:date>\n</oai_dc:dc>",
                    k.metadata_dc.title,
                    k.nama_penulis,
                    k.metadata_dc.identifier,
                    k.metadata_dc.date
                );

                hasil.push(ResponOaiPmhRecord {
                    oai_identifier: format!("oai:{}:{}", self.kode_pt, k.id_karya),
                    datestamp: k.metadata_dc.date.clone(),
                    set_spec: format!("prodi_{}", k.program_studi.to_lowercase().replace(' ', "_")),
                    dublin_core_xml: xml,
                });
            }
        }
        hasil
    }
}
