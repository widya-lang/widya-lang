use serde::{Deserialize, Serialize};

/// ============================================================================
/// 1. STANDAR GIZI & ANGKA KECUKUPAN GIZI (AKG) PROGRAM MAKAN BERGIZI GRATIS (MBG)
/// Permenkes No. 28 Tahun 2019 tentang Angka Kecukupan Gizi yang Dianjurkan
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KelompokSasaranMbg {
    PaudTk,         // Usia 4-6 tahun (Porsi ~400-500 kkal)
    SdKelas1Sampai3,// Usia 7-9 tahun (Porsi ~500-600 kkal)
    SdKelas4Sampai6,// Usia 10-12 tahun (Porsi ~600-700 kkal)
    SmpSmaSantri,   // Usia 13-18 tahun (Porsi ~700-850 kkal)
    IbuHamilMenyusui, // Porsi tinggi protein, asam folat & zat besi
    BalitaNonSekolah, // Balita stunting/gizi kurang
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KomposisiGiziPorsi {
    pub energi_kalori_kkal: f64,
    pub protein_gram: f64,      // Protein Hewani (Ayam, Ikan, Telur, Daging) + Nabati
    pub karbohidrat_gram: f64,  // Nasi, Jagung, Ubi
    pub lemak_gram: f64,
    pub serat_pangan_gram: f64,
    pub kalsium_mg: f64,        // Dari Susu Segar Koperasi Desa
    pub zat_besi_mg: f64,       // Pencegahan Anemia Remaja Putri
    pub vitamin_c_mg: f64,      // Buah Segar (Pisang, Jeruk, Pepaya)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuHarianMbg {
    pub hari: &'static str,      // "Senin", "Selasa", dll.
    pub makanan_pokok: String,  // e.g. "Nasi Pulen Desa Merah Putih"
    pub lauk_hewani: String,    // e.g. "Ayam Goreng Lengkuas / Telur Rebus"
    pub lauk_nabati: String,    // e.g. "Tempe Mendoan / Tahu Bacem"
    pub sayur: String,          // e.g. "Sayur Sop Wortel Buncis"
    pub buah: String,           // e.g. "Pisang Cavendish / Jeruk Manis"
    pub minuman: String,        // e.g. "Susu Pasteurisasi Segar Koperasi Merah Putih"
    pub nilai_gizi: KomposisiGiziPorsi,
    pub estimasi_biaya_porsi_idr: u64, // e.g. Rp 15.000 / porsi
}

pub struct StandarGiziEngine;

impl StandarGiziEngine {
    /// Validasi apakah menu makan siang MBG memenuhi standar AKG target
    pub fn validasi_kelayakan_gizi(target: KelompokSasaranMbg, gizi: &KomposisiGiziPorsi) -> (bool, Vec<String>) {
        let mut catatan = Vec::new();
        let (min_kal, max_kal, min_prot) = match target {
            KelompokSasaranMbg::PaudTk => (400.0, 550.0, 12.0),
            KelompokSasaranMbg::SdKelas1Sampai3 => (500.0, 650.0, 15.0),
            KelompokSasaranMbg::SdKelas4Sampai6 => (600.0, 750.0, 20.0),
            KelompokSasaranMbg::SmpSmaSantri => (700.0, 900.0, 25.0),
            KelompokSasaranMbg::IbuHamilMenyusui => (750.0, 950.0, 30.0),
            KelompokSasaranMbg::BalitaNonSekolah => (350.0, 500.0, 10.0),
        };

        if gizi.energi_kalori_kkal < min_kal {
            catatan.push(format!("Energi {:.1} kkal kurang dari batas minimum {:.1} kkal", gizi.energi_kalori_kkal, min_kal));
        } else if gizi.energi_kalori_kkal > max_kal {
            catatan.push(format!("Energi {:.1} kkal melebihi batas toleransi {:.1} kkal", gizi.energi_kalori_kkal, max_kal));
        }

        if gizi.protein_gram < min_prot {
            catatan.push(format!("Kandungan protein {:.1}g kurang dari standar {:.1}g", gizi.protein_gram, min_prot));
        }

        let lolos = catatan.is_empty();
        (lolos, catatan)
    }
}
