use super::gizi_akg::MenuHarianMbg;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 2. DAPUR SENTRAL SPPG (SATUAN PELAYANAN PEMENUHAN GIZI) KECAMATAN / DESA
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasokanBahanBakuLokal {
    pub nama_bahan: String,
    pub pemasok_koperasi_atau_gapoktan: String,
    pub kuantitas_kg: f64,
    pub harga_satuan_idr: u64,
    pub tanggal_panen_atau_kirim_iso: String,
    pub hasil_inspeksi_kebersihan: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMasakSppg {
    pub id_batch_masak: String,
    pub tanggal_iso: String,
    pub shift_masak: String, // "Shift 1 (03:00 - 06:00 Subuh)", "Shift 2 (08:00 - 10:00 Pagi)"
    pub jumlah_porsi_diproduksi: u32,
    pub suhu_makanan_matang_celsius: f64, // Standar HACCP: >= 65 C saat matang
    pub sampel_makanan_disimpan_24jam: bool, // Food Sample Retention untuk uji lab keamanan
    pub jam_mulai_distribusi_iso: String,
    pub batas_waktu_konsumsi_iso: String, // Maksimal 4 jam setelah matang
}

pub struct DapurSentralSppg {
    pub id_sppg: String,
    pub nama_dapur: String,
    pub wilayah_kecamatan_desa: String,
    pub kapasitas_harian_porsi: u32, // e.g. 3.000 porsi/hari
    pub pasokan_bahan: Vec<PasokanBahanBakuLokal>,
    pub riwayat_batch_masak: HashMap<String, BatchMasakSppg>,
}

impl DapurSentralSppg {
    pub fn new(id: &str, nama: &str, wilayah: &str, kapasitas: u32) -> Self {
        Self {
            id_sppg: id.to_string(),
            nama_dapur: nama.to_string(),
            wilayah_kecamatan_desa: wilayah.to_string(),
            kapasitas_harian_porsi: kapasitas,
            pasokan_bahan: Vec::new(),
            riwayat_batch_masak: HashMap::new(),
        }
    }

    /// Penyerapan bahan pangan dari Koperasi Merah Putih & Petani/Peternak Lokal
    pub fn terima_pasokan_lokal(
        &mut self,
        bahan: &str,
        pemasok: &str,
        qty_kg: f64,
        harga_kg: u64,
    ) -> PasokanBahanBakuLokal {
        let p = PasokanBahanBakuLokal {
            nama_bahan: bahan.to_string(),
            pemasok_koperasi_atau_gapoktan: pemasok.to_string(),
            kuantitas_kg: qty_kg,
            harga_satuan_idr: harga_kg,
            tanggal_panen_atau_kirim_iso: "2026-10-01T04:00:00Z".to_string(),
            hasil_inspeksi_kebersihan: true,
        };
        self.pasokan_bahan.push(p.clone());
        p
    }

    /// Proses produksi masak batch dapur SPPG dengan protokol HACCP & kontrol suhu
    pub fn proses_batch_masak(
        &mut self,
        id_batch: &str,
        shift: &str,
        porsi: u32,
        suhu_matang: f64,
    ) -> Result<BatchMasakSppg, String> {
        if porsi > self.kapasitas_harian_porsi {
            return Err(format!("Permintaan porsi {} melampaui kapasitas maksimum dapur ({})", porsi, self.kapasitas_harian_porsi));
        }

        if suhu_matang < 65.0 {
            return Err(format!("Suhu makanan matang {:.1}°C di bawah standar sanitasi HACCP (min 65.0°C). Makanan belum matang sempurna!", suhu_matang));
        }

        let batch = BatchMasakSppg {
            id_batch_masak: id_batch.to_string(),
            tanggal_iso: "2026-10-01".to_string(),
            shift_masak: shift.to_string(),
            jumlah_porsi_diproduksi: porsi,
            suhu_makanan_matang_celsius: suhu_matang,
            sampel_makanan_disimpan_24jam: true,
            jam_mulai_distribusi_iso: "2026-10-01T09:30:00Z".to_string(),
            batas_waktu_konsumsi_iso: "2026-10-01T13:30:00Z".to_string(),
        };

        self.riwayat_batch_masak.insert(id_batch.to_string(), batch.clone());
        Ok(batch)
    }
}
