use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 7. UNIT LOGISTIK, DISTRIBUSI & KURIR DESA (ARMADA & SURAT JALAN ELEKTRONIK)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JenisKendaraanLogistik {
    MotorRodaTigaKurir, // Pengantaran sembako/obat ke rumah warga
    MobilPickUpL300,    // Angkutan hasil tani & pasokan toko
    TrukEngkel6Roda,    // Angkutan komoditas gabah/pupuk antar-kota
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusPengiriman {
    Dijadwalkan,
    DalamPerjalanan,
    Terkirim,
    GagalKirim,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KendaraanArmada {
    pub nomor_plat: String,
    pub jenis: JenisKendaraanLogistik,
    pub nama_pengemudi: String,
    pub kapasitas_muatan_kg: f64,
    pub lokasi_terkini_gps: (f64, f64),
    pub status_tersedia: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuratJalanElektronik {
    pub nomor_waybill: String, // e.g. "KMP-LOG-2026-0421"
    pub asal_gudang_unit: String,
    pub tujuan_alamat: String,
    pub nomor_plat_armada: String,
    pub rincian_muatan: Vec<(String, f64, String)>, // [(Nama, Qty, Satuan)]
    pub berat_total_kg: f64,
    pub estimasi_waktu_tiba_iso: String,
    pub status: StatusPengiriman,
    pub tanda_tangan_digital_penerima: Option<String>,
}

pub struct UnitLogistikDesa {
    pub armada: HashMap<String, KendaraanArmada>,
    pub surat_jalan: HashMap<String, SuratJalanElektronik>,
}

impl UnitLogistikDesa {
    pub fn new() -> Self {
        let mut logistik = Self {
            armada: HashMap::new(),
            surat_jalan: HashMap::new(),
        };

        logistik.tambah_armada(
            "B 9102 KMP",
            JenisKendaraanLogistik::MotorRodaTigaKurir,
            "Pak Joko (Kurir Desa)",
            400.0,
            -6.2088,
            106.8456,
        );

        logistik.tambah_armada(
            "B 9488 KMP",
            JenisKendaraanLogistik::MobilPickUpL300,
            "Pak Bambang (Driver Logistik)",
            1500.0,
            -6.2088,
            106.8456,
        );

        logistik
    }

    pub fn tambah_armada(&mut self, plat: &str, jenis: JenisKendaraanLogistik, supir: &str, kapasitas: f64, lat: f64, lon: f64) {
        self.armada.insert(
            plat.to_string(),
            KendaraanArmada {
                nomor_plat: plat.to_string(),
                jenis,
                nama_pengemudi: supir.to_string(),
                kapasitas_muatan_kg: kapasitas,
                lokasi_terkini_gps: (lat, lon),
                status_tersedia: true,
            },
        );
    }

    /// Buat surat jalan elektronik pengiriman logistik
    pub fn buat_surat_jalan(
        &mut self,
        no_waybill: &str,
        asal: &str,
        tujuan: &str,
        plat: &str,
        muatan: Vec<(String, f64, String)>,
    ) -> Result<SuratJalanElektronik, String> {
        let armada_entry = self.armada.get_mut(plat)
            .ok_or_else(|| format!("Armada plat {} tidak terdaftar", plat))?;

        let total_berat: f64 = muatan.iter().map(|(_, q, _)| *q).sum();
        if total_berat > armada_entry.kapasitas_muatan_kg {
            return Err(format!("Berat muatan {:.1}kg melebihi kapasitas armada ({:.1}kg)!", total_berat, armada_entry.kapasitas_muatan_kg));
        }

        armada_entry.status_tersedia = false;

        let sj = SuratJalanElektronik {
            nomor_waybill: no_waybill.to_string(),
            asal_gudang_unit: asal.to_string(),
            tujuan_alamat: tujuan.to_string(),
            nomor_plat_armada: plat.to_string(),
            rincian_muatan: muatan,
            berat_total_kg: total_berat,
            estimasi_waktu_tiba_iso: "2026-10-01T15:30:00Z".to_string(),
            status: StatusPengiriman::DalamPerjalanan,
            tanda_tangan_digital_penerima: None,
        };

        self.surat_jalan.insert(no_waybill.to_string(), sj.clone());
        Ok(sj)
    }

    /// Konfirmasi penerimaan pengiriman dengan tanda tangan digital
    pub fn konfirmasi_penerimaan(&mut self, no_waybill: &str, ttd_digital: &str) -> Result<bool, String> {
        if let Some(sj) = self.surat_jalan.get_mut(no_waybill) {
            sj.status = StatusPengiriman::Terkirim;
            sj.tanda_tangan_digital_penerima = Some(ttd_digital.to_string());
            if let Some(a) = self.armada.get_mut(&sj.nomor_plat_armada) {
                a.status_tersedia = true;
            }
            Ok(true)
        } else {
            Err(format!("Surat jalan {} tidak ditemukan", no_waybill))
        }
    }
}
