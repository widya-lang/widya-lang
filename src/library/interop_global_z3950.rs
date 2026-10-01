use super::marc21_dublin_core::Marc21Record;
use serde::{Deserialize, Serialize};

/// ============================================================================
/// INTEROPERABILITAS PERPUSTAKAAN INTERNASIONAL (Z39.50, SRU/SRW, IIIF, ILL)
/// Standar Global untuk WorldCat OCLC, Library of Congress & Pinjam Antar-Perpus
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Z3950QueryRequest {
    pub host: String,              // e.g. "z3950.loc.gov", "worldcat.org"
    pub port: u16,                 // Default: 210
    pub database_name: String,     // e.g. "Voyager", "OCLC_WorldCat"
    pub syntax_format: String,     // "MARC21", "XML", "UNIMARC"
    pub rpn_query: String,         // e.g. "@attr 1=7 9780132350884" (Pencarian ISBN)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Z3950QueryResponse {
    pub status_koneksi: String,    // "CONNECTED", "TIMEOUT", "AUTH_ERROR"
    pub total_records_found: u32,
    pub records: Vec<Marc21Record>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IiifManifest {
    pub iiif_version: String,      // "3.0"
    pub id_manifest_uri: String,   // e.g. "https://iiif.perpusda.go.id/manuskrip/babad-banyumas/manifest.json"
    pub label: String,
    pub viewing_direction: String, // "left-to-right", "right-to-left" (untuk Arab Pegon)
    pub total_canvases: u32,       // Jumlah lembar halaman scan
    pub thumbnail_uri: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusInterlibraryLoan {
    PermintaanDiajukan,
    DisetujuiPerpustakaanPengirim,
    DalamPengirimanKurirDiplomatik,
    TibaDiPerpustakaanTujuan,
    SelesaiDikembalikan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterlibraryLoanRequest {
    pub nomor_ill: String,
    pub perpustakaan_pemohon: String,  // e.g. "Perpustakaan Nasional RI (Jakarta)"
    pub perpustakaan_pemberi: String,   // e.g. "British Library (London)" / "Leiden University Library"
    pub judul_item: String,
    pub nomor_katalog_internasional: String, // e.g. "OCLC: 12345678" / "Leiden Cod.Or. 1234"
    pub tanggal_permohonan: String,
    pub metode_pengiriman: String,     // "Digital Scan IIIF Secure", "Fisik Berasuransi"
    pub status: StatusInterlibraryLoan,
}

pub struct PerpustakaanInternasionalHub {
    pub nama_jaringan: String,
    pub daftar_server_z3950: Vec<String>,
    pub daftar_ill: Vec<InterlibraryLoanRequest>,
    pub daftar_iiif: Vec<IiifManifest>,
}

impl PerpustakaanInternasionalHub {
    pub fn new() -> Self {
        Self {
            nama_jaringan: "Global Library Interoperability Network (Z39.50 / OCLC / IIIF)".to_string(),
            daftar_server_z3950: vec![
                "z3950.loc.gov:210/Voyager".to_string(),
                "worldcat.org:210/WorldCat".to_string(),
                "katalog.perpusnas.go.id:210/INLISLite".to_string(),
            ],
            daftar_ill: Vec::new(),
            daftar_iiif: Vec::new(),
        }
    }

    /// Simulasi eksekusi Z39.50 Federated Search
    pub fn eksekusi_z3950_search(&self, req: Z3950QueryRequest) -> Z3950QueryResponse {
        let mut sample_rec = Marc21Record::new("LOC-2026-99881");
        sample_rec.tambah_field("020", ' ', ' ', vec![('a', "978-0-13-235088-4")]);
        sample_rec.tambah_field("100", '1', ' ', vec![('a', "Martin, Robert C.")]);
        sample_rec.tambah_field("245", '1', '0', vec![
            ('a', "Clean Code: A Handbook of Agile Software Craftsmanship"),
            ('c', "Robert C. Martin"),
        ]);
        sample_rec.tambah_field("650", ' ', '0', vec![('a', "Software engineering")]);

        Z3950QueryResponse {
            status_koneksi: "CONNECTED".to_string(),
            total_records_found: 1,
            records: vec![sample_rec],
        }
    }

    /// Buat permohonan Interlibrary Loan (ILL) internasional
    pub fn buat_permohonan_ill(
        &mut self,
        no_ill: &str,
        pemohon: &str,
        pemberi: &str,
        judul: &str,
        no_katalog: &str,
        tgl: &str,
        metode: &str,
    ) -> InterlibraryLoanRequest {
        let ill = InterlibraryLoanRequest {
            nomor_ill: no_ill.to_string(),
            perpustakaan_pemohon: pemohon.to_string(),
            perpustakaan_pemberi: pemberi.to_string(),
            judul_item: judul.to_string(),
            nomor_katalog_internasional: no_katalog.to_string(),
            tanggal_permohonan: tgl.to_string(),
            metode_pengiriman: metode.to_string(),
            status: StatusInterlibraryLoan::PermintaanDiajukan,
        };
        self.daftar_ill.push(ill.clone());
        ill
    }

    /// Daftarkan manifest IIIF untuk naskah kuno / buku langka
    pub fn daftarkan_iiif_manifest(&mut self, manifest: IiifManifest) {
        self.daftar_iiif.push(manifest);
    }
}
