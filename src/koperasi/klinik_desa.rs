use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 4. UNIT KLINIK DESA & FASILITAS KESEHATAN PRIMER (RME SATUSEHAT & KIA)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusGiziBalita {
    GiziBaik,
    BeresikoStunting,
    StuntingTinggiBadanRendah,
    GiziKurangWasting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RekamMedisDesa {
    pub nomor_rekam_medis: String,
    pub nik_pasien: String,
    pub nama_pasien: String,
    pub tanggal_kunjungan_iso: String,
    pub keluhan: String,
    pub tekanan_darah: String,      // e.g. "120/80 mmHg"
    pub gula_darah_sewaktu_mg_dl: Option<u32>,
    pub asam_urat_mg_dl: Option<f64>,
    pub kolesterol_mg_dl: Option<u32>,
    pub diagnosis_icd10: String,    // e.g. "I10 Essential Hypertension"
    pub resep_obat_ids: Vec<String>,
    pub status_gizi_balita: Option<StatusGiziBalita>,
    pub integrasi_satusehat_ihs_id: String,
}

pub struct UnitKlinikDesa {
    pub nama_klinik: String,
    pub rekam_medis_list: HashMap<String, Vec<RekamMedisDesa>>, // Key: NIK
    pub total_kunjungan: u64,
}

impl UnitKlinikDesa {
    pub fn new(nama: &str) -> Self {
        Self {
            nama_klinik: nama.to_string(),
            rekam_medis_list: HashMap::new(),
            total_kunjungan: 0,
        }
    }

    /// Evaluasi antropometri pertumbuhan balita terhadap standar WHO (Pencegahan Stunting Desa)
    pub fn evaluasi_stunting_balita(umur_bulan: u32, tinggi_cm: f64) -> StatusGiziBalita {
        // Standar median tinggi badan WHO anak 24 bulan ~ 87 cm
        let median_tinggi = 75.0 + (umur_bulan as f64 * 0.55);
        if tinggi_cm < (median_tinggi - 6.0) {
            StatusGiziBalita::StuntingTinggiBadanRendah
        } else if tinggi_cm < (median_tinggi - 3.0) {
            StatusGiziBalita::BeresikoStunting
        } else {
            StatusGiziBalita::GiziBaik
        }
    }

    /// Catat pemeriksaan dokter/bidan desa
    pub fn catat_pemeriksaan(
        &mut self,
        no_rm: &str,
        nik: &str,
        nama: &str,
        keluhan: &str,
        tensi: &str,
        gula: Option<u32>,
        diag_icd10: &str,
        obat_ids: Vec<String>,
        stunting: Option<StatusGiziBalita>,
    ) -> RekamMedisDesa {
        self.total_kunjungan += 1;
        let rm = RekamMedisDesa {
            nomor_rekam_medis: no_rm.to_string(),
            nik_pasien: nik.to_string(),
            nama_pasien: nama.to_string(),
            tanggal_kunjungan_iso: "2026-10-01T09:00:00Z".to_string(),
            keluhan: keluhan.to_string(),
            tekanan_darah: tensi.to_string(),
            gula_darah_sewaktu_mg_dl: gula,
            asam_urat_mg_dl: None,
            kolesterol_mg_dl: None,
            diagnosis_icd10: diag_icd10.to_string(),
            resep_obat_ids: obat_ids,
            status_gizi_balita: stunting,
            integrasi_satusehat_ihs_id: format!("IHS-DESA-{}", nik),
        };

        self.rekam_medis_list.entry(nik.to_string()).or_default().push(rm.clone());
        rm
    }
}
