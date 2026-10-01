use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// Industri Obat Tradisional, Jamu, OHT, Fitofarmaka & FOHI (Kemenkes & BPOM)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KategoriObatBahanAlam {
    Jamu,                  // BPOM POM TR (Khasiat empiris turun-temurun)
    ObatHerbalTerstandar,  // BPOM POM HT (Uji Praklinik & Standarisasi Bahan Baku)
    Fitofarmaka,           // BPOM POM FF (Uji Klinik Manusia & Kesetaraan Obat Kimia)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenyawaAktif {
    pub nama_senyawa: String,     // e.g. "Kurkuminoid", "Andrografolida", "Gingerol"
    pub persentase_kadar: f64,    // e.g. 5.2%
    pub metode_pengujian: String, // e.g. "HPLC (High-Performance Liquid Chromatography)"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplisiaHerbal {
    pub id_simplisia: String,
    pub nama_latin: String,       // e.g. "Curcuma xanthorrhiza Rhizoma"
    pub nama_daerah: String,      // e.g. "Temulawak"
    pub bagian_tanaman: String,   // "Rimpang", "Daun", "Kulit Batang", "Biji"
    pub senyawa_aktif: Vec<SenyawaAktif>,
    pub kadar_air_pct: f64,       // Standar Farmakope Herbal: <= 10.0%
    pub kadar_abu_total_pct: f64, // Standar: <= 8.0%
    pub bebas_cemaran_logam: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProdukHerbalMarketplace {
    pub id_produk: String,
    pub nama_produk: String,
    pub kategori: KategoriObatBahanAlam,
    pub nomor_izin_edar_bpom: String, // e.g. "POM TR123456781"
    pub produsen: String,
    pub sertifikasi_halal: bool,
    pub indikasi_khasiat: Vec<String>,
    pub kontraindikasi: Vec<String>,
    pub aturan_pakai: String,
    pub harga_idr: u64,
    pub stok_unit: u32,
    pub batch_number: String,
}

pub struct HerbDrugInteractionEngine {
    // Interaksi antara herbal dengan obat kimia (e.g. Sambiloto + Antidiabetes, Kunyit + Antikoagulan)
    pub interaction_rules: HashMap<String, Vec<(String, &'static str)>>,
}

impl HerbDrugInteractionEngine {
    pub fn new() -> Self {
        let mut rules = HashMap::new();

        rules.insert(
            "Curcuma xanthorrhiza / Kunyit".to_string(),
            vec![(
                "Warfarin / Aspirin".to_string(),
                "Peringatan: Potensi peningkatan efek antikoagulan (pengencer darah) dan risiko pendarahan.",
            )],
        );

        rules.insert(
            "Andrographis paniculata / Sambiloto".to_string(),
            vec![(
                "Metformin / Glibenklamid".to_string(),
                "Perhatian: Potensi efek sinergis penurunan glukosa darah berlebihan (hipoglikemia). Pantau kadar gula.",
            )],
        );

        rules.insert(
            "Zingiber officinale / Jahe Merah".to_string(),
            vec![(
                "Kaptopril / Amlodipin".to_string(),
                "Informasi: Memiliki efek hipotensif ringan, dapat membantu stabilitas tekanan darah bersama obat antihipertensi.",
            )],
        );

        Self {
            interaction_rules: rules,
        }
    }

    /// Periksa apakah ada potensi interaksi antara herbal dengan obat kimia resep
    pub fn check_interaction(&self, herb_name: &str, drug_name: &str) -> Option<&'static str> {
        for (herb, interactions) in &self.interaction_rules {
            if herb.to_lowercase().contains(&herb_name.to_lowercase()) {
                for (drug, advice) in interactions {
                    if drug.to_lowercase().contains(&drug_name.to_lowercase()) {
                        return Some(advice);
                    }
                }
            }
        }
        None
    }
}
