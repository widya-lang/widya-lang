use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 6. UNIT PERGUDANGAN & SISTEM RESI GUDANG KOMODITAS DESA (SRG BAPPEBTI)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JenisKomoditasGudang {
    GabahKeringPanen,
    BerasMedium,
    BerasPremium,
    JagungPipil,
    BijiKopiRobusta,
    PupukSubsidiUreaNPK,
    PakanTernak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResiGudangSertifikat {
    pub nomor_resi_gudang: String, // e.g. "SRG-KMP-2026-0081"
    pub pemilik_npa: String,
    pub nama_pemilik: String,
    pub komoditas: JenisKomoditasGudang,
    pub kuantitas_kg: f64,
    pub kadar_air_pct: f64,        // Gabah standar BPS/Bulog <= 14.0%
    pub butir_patah_pct: f64,
    pub nilai_taksiran_pasar_idr: u64,
    pub plafon_pembiayaan_bank_idr: u64, // Maksimal 70% dari nilai taksiran pasar
    pub lokasi_blok_gudang: String,
    pub tanggal_masuk_iso: String,
    pub tanggal_jatuh_tempo_iso: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetriSensorSiloGudang {
    pub id_silo: String,
    pub suhu_ruang_celsius: f64,   // Ideal: 18 - 25 C
    pub kelembaban_udara_rh_pct: f64, // Ideal: 55 - 65% RH
    pub kadar_gas_co2_ppm: f64,    // Deteksi respirasi jamur / kutu beras (>1000 ppm = warning)
    pub status_ventilasi_otomatis: bool,
}

pub struct UnitPergudanganDesa {
    pub nama_gudang: String,
    pub kapasitas_total_ton: f64,
    pub srg_aktif: HashMap<String, ResiGudangSertifikat>,
    pub sensor_silo: Vec<TelemetriSensorSiloGudang>,
}

impl UnitPergudanganDesa {
    pub fn new(nama: &str, kapasitas_ton: f64) -> Self {
        Self {
            nama_gudang: nama.to_string(),
            kapasitas_total_ton: kapasitas_ton,
            srg_aktif: HashMap::new(),
            sensor_silo: Vec::new(),
        }
    }

    /// Terbitkan Resi Gudang (SRG) komoditas petani dengan penilaian kualitas mutu
    pub fn terbitkan_resi_gudang(
        &mut self,
        no_srg: &str,
        npa: &str,
        nama: &str,
        komoditas: JenisKomoditasGudang,
        berat_kg: f64,
        kadar_air: f64,
        harga_per_kg: u64,
        blok: &str,
    ) -> Result<ResiGudangSertifikat, String> {
        if kadar_air > 14.5 {
            return Err(format!("Kadar air gabah {:.1}% melampaui batas toleransi mutu (maks 14.0%). Perlu pengeringan ulang!", kadar_air));
        }

        let nilai_pasar = (berat_kg * harga_per_kg as f64).round() as u64;
        let plafon_kredit = (nilai_pasar as f64 * 0.70).round() as u64; // 70% LTV untuk agunan simpan pinjam

        let srg = ResiGudangSertifikat {
            nomor_resi_gudang: no_srg.to_string(),
            pemilik_npa: npa.to_string(),
            nama_pemilik: nama.to_string(),
            komoditas,
            kuantitas_kg: berat_kg,
            kadar_air_pct: kadar_air,
            butir_patah_pct: 3.5,
            nilai_taksiran_pasar_idr: nilai_pasar,
            plafon_pembiayaan_bank_idr: plafon_kredit,
            lokasi_blok_gudang: blok.to_string(),
            tanggal_masuk_iso: "2026-10-01T08:00:00Z".to_string(),
            tanggal_jatuh_tempo_iso: "2027-04-01T08:00:00Z".to_string(),
        };

        self.srg_aktif.insert(no_srg.to_string(), srg.clone());
        Ok(srg)
    }

    /// Pemantauan telemetri IoT kondisi mikroiklim silo penyimpanan gabah
    pub fn pantau_sensor_silo(&mut self, silo_id: &str, temp: f64, rh: f64, co2: f64) -> TelemetriSensorSiloGudang {
        let kipas_aktif = temp > 28.0 || rh > 70.0 || co2 > 1000.0;
        let telem = TelemetriSensorSiloGudang {
            id_silo: silo_id.to_string(),
            suhu_ruang_celsius: temp,
            kelembaban_udara_rh_pct: rh,
            kadar_gas_co2_ppm: co2,
            status_ventilasi_otomatis: kipas_aktif,
        };
        self.sensor_silo.push(telem.clone());
        telem
    }
}
