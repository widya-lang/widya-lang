use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 5. UNIT APOTEK DESA & FARMASI ESENSIAL (KONTROL FEFO & HET KEMENKES)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GolonganObat {
    ObatBebas,           // Lingkaran Hijau
    ObatBebasTerbatas,   // Lingkaran Biru
    ObatKerasResepDokter,// Lingkaran Merah (K)
    JamuOhtFitofarmaka,  // Herbal Terstandar
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchObat {
    pub batch_number: String,
    pub tanggal_kedaluwarsa_iso: String, // YYYY-MM-DD
    pub stok_tersedia: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemObatApotek {
    pub id_obat: String,
    pub nama_generik: String,
    pub nama_dagang: String,
    pub golongan: GolonganObat,
    pub bentuk_sediaan: String, // "Tablet", "Sirup", "Kapsul", "Salep"
    pub harga_het_kemenkes_idr: u64,
    pub harga_jual_idr: u64,
    pub batches: Vec<BatchObat>, // Sorted by expiration date (FEFO)
}

pub struct UnitApotekDesa {
    pub nama_apotek: String,
    pub nomor_sia: String, // Surat Izin Apotek
    pub inventaris_obat: HashMap<String, ItemObatApotek>,
}

impl UnitApotekDesa {
    pub fn new(nama: &str, sia: &str) -> Self {
        let mut apotek = Self {
            nama_apotek: nama.to_string(),
            nomor_sia: sia.to_string(),
            inventaris_obat: HashMap::new(),
        };

        // Standard Essential Village Medicines
        apotek.tambah_obat(
            "MED-001",
            "Paracetamol 500mg",
            "Paracetamol Generik",
            GolonganObat::ObatBebas,
            "Tablet",
            8000,
            6500,
            "BATCH-2027A",
            "2027-12-31",
            500,
        );

        apotek.tambah_obat(
            "MED-002",
            "Amoxicillin 500mg",
            "Amoxicillin Generik",
            GolonganObat::ObatKerasResepDokter,
            "Kaplet",
            12000,
            10000,
            "BATCH-2027B",
            "2027-08-30",
            300,
        );

        apotek.tambah_obat(
            "MED-003",
            "Curcuma & Zingiberis",
            "Kapsul Temulawak Jahe Merah",
            GolonganObat::JamuOhtFitofarmaka,
            "Kapsul",
            45000,
            38000,
            "BATCH-2028C",
            "2028-05-15",
            150,
        );

        apotek
    }

    pub fn tambah_obat(
        &mut self,
        id: &str,
        generik: &str,
        dagang: &str,
        gol: GolonganObat,
        bentuk: &str,
        het: u64,
        jual: u64,
        batch_no: &str,
        exp_iso: &str,
        stok: u32,
    ) {
        let entry = self.inventaris_obat.entry(id.to_string()).or_insert_with(|| ItemObatApotek {
            id_obat: id.to_string(),
            nama_generik: generik.to_string(),
            nama_dagang: dagang.to_string(),
            golongan: gol,
            bentuk_sediaan: bentuk.to_string(),
            harga_het_kemenkes_idr: het,
            harga_jual_idr: jual,
            batches: Vec::new(),
        });

        entry.batches.push(BatchObat {
            batch_number: batch_no.to_string(),
            tanggal_kedaluwarsa_iso: exp_iso.to_string(),
            stok_tersedia: stok,
        });

        // Urutkan batch berdasarkan tanggal kedaluwarsa terdekat (First Expired First Out - FEFO)
        entry.batches.sort_by(|a, b| a.tanggal_kedaluwarsa_iso.cmp(&b.tanggal_kedaluwarsa_iso));
    }

    /// Pengeluaran obat resep dengan algoritma FEFO otomatis
    pub fn keluarkan_obat_fefo(&mut self, id_obat: &str, jumlah_dibutuhkan: u32) -> Result<(u64, Vec<String>), String> {
        if let Some(item) = self.inventaris_obat.get_mut(id_obat) {
            let total_stok: u32 = item.batches.iter().map(|b| b.stok_tersedia).sum();
            if total_stok < jumlah_dibutuhkan {
                return Err(format!("Stok obat '{}' tidak mencukupi (sisa {})", item.nama_dagang, total_stok));
            }

            let mut sisa_kebutuhan = jumlah_dibutuhkan;
            let mut used_batches = Vec::new();

            for batch in item.batches.iter_mut() {
                if sisa_kebutuhan == 0 {
                    break;
                }
                let ambil = sisa_kebutuhan.min(batch.stok_tersedia);
                batch.stok_tersedia -= ambil;
                sisa_kebutuhan -= ambil;
                used_batches.push(format!("Batch {} (Exp: {}): {} unit", batch.batch_number, batch.tanggal_kedaluwarsa_iso, ambil));
            }

            let total_biaya = item.harga_jual_idr * (jumlah_dibutuhkan as u64);
            Ok((total_biaya, used_batches))
        } else {
            Err(format!("Obat ID {} tidak ditemukan", id_obat))
        }
    }
}
