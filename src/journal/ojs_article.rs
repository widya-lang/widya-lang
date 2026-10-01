use serde::{Deserialize, Serialize};

/// ============================================================================
/// PENERBITAN JURNAL ILMIAH ELEKTRONIK (OPEN JOURNAL SYSTEMS / OJS STANDARDS)
/// Metadata Artikel, ORCID, Siklus Double-Blind Review, dan Crossref DOI
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusReviewArtikel {
    Submitted,
    InReviewWithReviewers,
    MinorRevisionRequired,
    MajorRevisionRequired,
    AcceptedForPublication,
    PublishedOnline,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PenulisJurnal {
    pub nama_lengkap: String,
    pub email: String,
    pub afiliasi: String,
    pub ror_id_institusi: Option<String>,
    pub orcid_id: Option<String>, // e.g. "0000-0002-1825-0097"
    pub corresponding_author: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewerRecord {
    pub nama_reviewer: String,
    pub afiliasi: String,
    pub tanggal_selesai_review: String,
    pub skor_kualitas_metodologi: u32, // 1 - 100
    pub rekomendasi: String,           // "Accept", "Minor Revision", "Major Revision", "Reject"
    pub catatan_ke_penulis: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtikelJurnalIlmiah {
    pub id_artikel: String,
    pub doi: String,                    // e.g. "10.1234/widya.v12i1.450"
    pub judul_utama_id: String,
    pub judul_utama_en: String,
    pub abstrak_id: String,
    pub abstrak_en: String,
    pub kata_kunci_en: Vec<String>,
    pub penulis: Vec<PenulisJurnal>,
    pub nama_jurnal: String,
    pub e_issn: String,
    pub p_issn: Option<String>,
    pub volume: u32,
    pub nomor_issue: u32,
    pub tahun_terbit: u32,
    pub halaman_mulai: u32,
    pub halaman_akhir: u32,
    pub tanggal_submit: String,
    pub tanggal_terbit: Option<String>,
    pub lisensi_hak_cipta: String,      // "CC-BY-4.0", "CC-BY-SA-4.0"
    pub status: StatusReviewArtikel,
    pub catatan_review: Vec<ReviewerRecord>,
}

impl ArtikelJurnalIlmiah {
    pub fn new(
        id: &str,
        doi: &str,
        judul_en: &str,
        abstrak_en: &str,
        jurnal: &str,
        e_issn: &str,
        vol: u32,
        no: u32,
        tahun: u32,
    ) -> Self {
        Self {
            id_artikel: id.to_string(),
            doi: doi.to_string(),
            judul_utama_id: "".to_string(),
            judul_utama_en: judul_en.to_string(),
            abstrak_id: "".to_string(),
            abstrak_en: abstrak_en.to_string(),
            kata_kunci_en: Vec::new(),
            penulis: Vec::new(),
            nama_jurnal: jurnal.to_string(),
            e_issn: e_issn.to_string(),
            p_issn: None,
            volume: vol,
            nomor_issue: no,
            tahun_terbit: tahun,
            halaman_mulai: 1,
            halaman_akhir: 15,
            tanggal_submit: "2026-01-01".to_string(),
            tanggal_terbit: None,
            lisensi_hak_cipta: "Creative Commons Attribution 4.0 International (CC BY 4.0)".to_string(),
            status: StatusReviewArtikel::Submitted,
            catatan_review: Vec::new(),
        }
    }

    pub fn tambah_penulis(&mut self, penulis: PenulisJurnal) {
        self.penulis.push(penulis);
    }

    pub fn kirim_ke_reviewer(&mut self) {
        self.status = StatusReviewArtikel::InReviewWithReviewers;
    }

    pub fn tambahkan_hasil_review(&mut self, rev: ReviewerRecord) {
        self.catatan_review.push(rev);
    }

    pub fn terbitkan_online(&mut self, tanggal: &str) {
        self.status = StatusReviewArtikel::PublishedOnline;
        self.tanggal_terbit = Some(tanggal.to_string());
    }

    /// Menghasilkan format Crossref XML Deposit Schema 5.3.1
    pub fn generate_crossref_xml(&self) -> String {
        let first_author = self.penulis.first().cloned().unwrap_or(PenulisJurnal {
            nama_lengkap: "Author, Unknown".to_string(),
            email: "".to_string(),
            afiliasi: "".to_string(),
            ror_id_institusi: None,
            orcid_id: None,
            corresponding_author: true,
        });

        let orcid_tag = match &first_author.orcid_id {
            Some(o) => format!("        <ORCID authenticated=\"true\">https://orcid.org/{}</ORCID>\n", o),
            None => "".to_string(),
        };

        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
            <doi_batch version=\"5.3.1\" xmlns=\"http://www.crossref.org/schema/5.3.1\">\n  \
              <head>\n    \
                <doi_batch_id>BATCH-{}</doi_batch_id>\n    \
                <depositor><depositor_name>Widya Scholarly Press</depositor_name><email_address>editor@widya-lang.org</email_address></depositor>\n    \
                <registrant>Widya-Lang Association</registrant>\n  \
              </head>\n  \
              <body>\n    \
                <journal>\n      \
                  <journal_metadata>\n        \
                    <full_title>{}</full_title>\n        \
                    <issn media_type=\"electronic\">{}</issn>\n      \
                  </journal_metadata>\n      \
                  <journal_issue>\n        \
                    <journal_volume><volume>{}</volume></journal_volume>\n        \
                    <issue>{}</issue>\n      \
                  </journal_issue>\n      \
                  <journal_article publication_type=\"full_text\">\n        \
                    <titles><title>{}</title></titles>\n        \
                    <contributors>\n          \
                      <person_name sequence=\"first\" contributor_role=\"author\">\n            \
                        <surname>{}</surname>\n{}\
                        <affiliation>{}</affiliation>\n          \
                      </person_name>\n        \
                    </contributors>\n        \
                    <publication_date media_type=\"online\">\n          \
                      <year>{}</year>\n        \
                    </publication_date>\n        \
                    <doi_data>\n          \
                      <doi>{}</doi>\n          \
                      <resource>https://journal.widya-lang.org/index.php/article/view/{}</resource>\n        \
                    </doi_data>\n      \
                  </journal_article>\n    \
                </journal>\n  \
              </body>\n\
            </doi_batch>",
            self.id_artikel,
            self.nama_jurnal,
            self.e_issn,
            self.volume,
            self.nomor_issue,
            self.judul_utama_en,
            first_author.nama_lengkap,
            orcid_tag,
            first_author.afiliasi,
            self.tahun_terbit,
            self.doi,
            self.id_artikel
        )
    }
}
