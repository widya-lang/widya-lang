pub mod citygml;
pub mod ifc_parser;
pub mod pure_smartcity;

pub use citygml::{CityGmlBuildingLod2, FloodEvacuationRoute, MetropolitanDigitalTwin};
pub use ifc_parser::{BimModel, IfcBuildingElement, IfcElementType};
pub use pure_smartcity::{
    LandCoverType, RooftopSolarEngine, TrafficApproachPhase, UrbanCatchmentZone,
    UrbanHydrologyEngine, WebsterSignalOptimizer,
};

use serde::{Deserialize, Serialize};

/// ============================================================================
/// 1. SISTEM INDEKS STANDAR PENCEMAR UDARA (ISPU) - PERMEN LHK
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KategoriIspu {
    Baik,
    Sedang,
    TidakSehat,
    SangatTidakSehat,
    Berbahaya,
}

impl KategoriIspu {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Baik => "Baik",
            Self::Sedang => "Sedang",
            Self::TidakSehat => "Tidak Sehat",
            Self::SangatTidakSehat => "Sangat Tidak Sehat",
            Self::Berbahaya => "Berbahaya",
        }
    }

    pub fn warna_hex(&self) -> &'static str {
        match self {
            Self::Baik => "#22c55e",          // Hijau
            Self::Sedang => "#3b82f6",        // Biru
            Self::TidakSehat => "#eab308",    // Kuning
            Self::SangatTidakSehat => "#ef4444", // Merah
            Self::Berbahaya => "#18181b",     // Hitam
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasilIspu {
    pub parameter: String,
    pub konsentrasi_ug_m3: f64,
    pub nilai_ispu: u32,
    pub kategori: KategoriIspu,
    pub saran_kesehatan: String,
}

/// Menghitung ISPU untuk parameter PM2.5 (ug/m3) berdasarkan Permen LHK
pub fn hitung_ispu_pm25(konsentrasi: f64) -> HasilIspu {
    let (xa, xb, ia, ib) = if konsentrasi <= 15.5 {
        (15.5, 0.0, 50.0, 0.0)
    } else if konsentrasi <= 55.4 {
        (55.4, 15.6, 100.0, 51.0)
    } else if konsentrasi <= 150.4 {
        (150.4, 55.5, 200.0, 101.0)
    } else if konsentrasi <= 250.4 {
        (250.4, 150.5, 300.0, 201.0)
    } else {
        (500.0, 250.5, 500.0, 301.0)
    };

    let ispu_val = if xa == xb {
        0
    } else {
        (((ia - ib) / (xa - xb)) * (konsentrasi - xb) + ib).round() as u32
    };

    let (kategori, saran) = if ispu_val <= 50 {
        (KategoriIspu::Baik, "Kualitas udara sangat baik, aman untuk aktivitas luar ruangan bagi seluruh masyarakat.".to_string())
    } else if ispu_val <= 100 {
        (KategoriIspu::Sedang, "Kualitas udara dapat diterima, kelompok sangat sensitif disarankan membatasi paparan fisik berlebihan.".to_string())
    } else if ispu_val <= 200 {
        (KategoriIspu::TidakSehat, "Udara merugikan bagi kelompok sensitif (anak-anak/lansia/penderita asma). Gunakan masker medis/N95.".to_string())
    } else if ispu_val <= 300 {
        (KategoriIspu::SangatTidakSehat, "Kualitas udara sangat berbahaya bagi kesehatan. Hindari aktivitas di luar ruangan.".to_string())
    } else {
        (KategoriIspu::Berbahaya, "Status Darurat Polusi Udara. Semua warga wajib berada di dalam ruangan dengan air purifier.".to_string())
    };

    HasilIspu {
        parameter: "PM2.5".to_string(),
        konsentrasi_ug_m3: konsentrasi,
        nilai_ispu: ispu_val,
        kategori,
        saran_kesehatan: saran,
    }
}

/// ============================================================================
/// 2. SPATIAL GEO-INTELLIGENCE & SISTEM INFORMASI GEOGRAFIS (SIG)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitikKoordinat {
    pub lintang: f64,  // Latitude (-90 s/d +90)
    pub bujur: f64,    // Longitude (-180 s/d +180)
}

impl TitikKoordinat {
    pub fn new(lintang: f64, bujur: f64) -> Self {
        Self { lintang, bujur }
    }
}

/// Menghitung jarak lingkaran besar (Great Circle Distance) rumus Haversine dalam satuan Kilometer (km)
pub fn jarak_haversine_km(titik_a: &TitikKoordinat, titik_b: &TitikKoordinat) -> f64 {
    const R_BUMI_KM: f64 = 6371.0;

    let lat1_rad = titik_a.lintang.to_radians();
    let lat2_rad = titik_b.lintang.to_radians();
    let d_lat = (titik_b.lintang - titik_a.lintang).to_radians();
    let d_lon = (titik_b.bujur - titik_a.bujur).to_radians();

    let a = (d_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    R_BUMI_KM * c
}

/// Menentukan apakah titik koordinat berada di dalam poligon batas wilayah (Ray Casting Algorithm)
pub fn titik_di_dalam_poligon(titik: &TitikKoordinat, poligon: &[TitikKoordinat]) -> bool {
    if poligon.len() < 3 {
        return false;
    }

    let mut inside = false;
    let mut j = poligon.len() - 1;

    for i in 0..poligon.len() {
        let pi = &poligon[i];
        let pj = &poligon[j];

        if ((pi.lintang > titik.lintang) != (pj.lintang > titik.lintang))
            && (titik.bujur < (pj.bujur - pi.bujur) * (titik.lintang - pi.lintang) / (pj.lintang - pi.lintang) + pi.bujur)
        {
            inside = !inside;
        }
        j = i;
    }

    inside
}

/// ============================================================================
/// 3. SMART TRAFFIC & REKAYASA LALU LINTAS CERDAS (V/C RATIO & LoS)
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LevelOfService {
    A, // Bebas lancar
    B, // Arus stabil
    C, // Arus stabil kecepatan terkontrol
    D, // Mendekati kapasitas jenuh
    E, // Arus jenuh / antrean padat
    F, // Macet total / Gridlock
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalisisLaluLintas {
    pub nama_ruas_jalan: String,
    pub volume_kendaraan_smp: f64,
    pub kapasitas_jalan_smp: f64,
    pub vc_ratio: f64,
    pub level_of_service: LevelOfService,
    pub durasi_lampu_hijau_detik: u32,
}

/// Menghitung rasio V/C (Volume-to-Capacity) dan durasi lampu hijau adaptif
pub fn optimasi_lalu_lintas(
    nama_jalan: &str,
    volume_smp: f64,
    kapasitas_smp: f64,
    durasi_siklus_total_detik: u32,
) -> AnalisisLaluLintas {
    let vc_ratio = if kapasitas_smp > 0.0 {
        volume_smp / kapasitas_smp
    } else {
        1.0
    };

    let los = if vc_ratio < 0.60 {
        LevelOfService::A
    } else if vc_ratio < 0.70 {
        LevelOfService::B
    } else if vc_ratio < 0.80 {
        LevelOfService::C
    } else if vc_ratio < 0.90 {
        LevelOfService::D
    } else if vc_ratio <= 1.00 {
        LevelOfService::E
    } else {
        LevelOfService::F
    };

    // Alokasi proporsional lampu hijau adaptif (minimum 15 detik, maksimum 90 detik)
    let proporsi = (vc_ratio / 1.2).clamp(0.2, 0.8);
    let durasi_hijau = (durasi_siklus_total_detik as f64 * proporsi).round().clamp(15.0, 90.0) as u32;

    AnalisisLaluLintas {
        nama_ruas_jalan: nama_jalan.to_string(),
        volume_kendaraan_smp: volume_smp,
        kapasitas_jalan_smp: kapasitas_smp,
        vc_ratio: (vc_ratio * 100.0).round() / 100.0,
        level_of_service: los,
        durasi_lampu_hijau_detik: durasi_hijau,
    }
}

/// ============================================================================
/// 4. SMART ENERGY & ANALISIS MIKROGRID ENERGI BARU TERBARUKAN (EBT)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaporanEnergiEbt {
    pub total_daya_mw: f64,
    pub daya_plts_surya_mw: f64,
    pub daya_plta_hidro_mw: f64,
    pub daya_pltp_panas_bumi_mw: f64,
    pub daya_termal_fosil_mw: f64,
    pub persentase_ebt: f64,
    pub reduksi_emisi_ton_co2_harian: f64,
}

pub fn analisis_bauran_energi(
    plts_mw: f64,
    plta_mw: f64,
    pltp_mw: f64,
    fosil_mw: f64,
) -> LaporanEnergiEbt {
    let total_ebt = plts_mw + plta_mw + pltp_mw;
    let total_daya = total_ebt + fosil_mw;
    let persen_ebt = if total_daya > 0.0 {
        (total_ebt / total_daya) * 100.0
    } else {
        0.0
    };

    // Faktor emisi rata-rata PLTU batubara: ~0.9 kg CO2 per kWh (atau 21.6 ton CO2 per MW/hari)
    let reduksi_co2 = total_ebt * 21.6;

    LaporanEnergiEbt {
        total_daya_mw: total_daya,
        daya_plts_surya_mw: plts_mw,
        daya_plta_hidro_mw: plta_mw,
        daya_pltp_panas_bumi_mw: pltp_mw,
        daya_termal_fosil_mw: fosil_mw,
        persentase_ebt: (persen_ebt * 10.0).round() / 10.0,
        reduksi_emisi_ton_co2_harian: (reduksi_co2 * 10.0).round() / 10.0,
    }
}
