use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 1. UNIT PERKANTORAN, KEANGGOTAAN & TATA KELOLA KOPERASI MERAH PUTIH
/// (UU No. 25 Tahun 1992 tentang Perkoperasian & Permenkop UKM)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusKeanggotaan {
    CalonAnggota,
    AnggotaBiasa,
    AnggotaLuarBiasa,
    NonAktif,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnggotaKoperasi {
    pub nomor_pokok_anggota: String, // e.g. "KMP-3301-0001"
    pub nik: String,                 // 16 digit NIK KTP
    pub nama_lengkap: String,
    pub alamat_rt_rw: String,
    pub desa_kelurahan: String,
    pub nomor_telepon: String,
    pub simpanan_pokok_idr: u64,     // Simpanan awal masuk koperasi
    pub simpanan_wajib_idr: u64,     // Akumulasi simpanan bulanan
    pub status: StatusKeanggotaan,
    pub tanggal_bergabung_iso: String,
    pub total_partisipasi_transaksi_idr: u64, // Akumulasi belanja & transaksi di 7 unit usaha
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlokasiShuAnggota {
    pub nomor_pokok_anggota: String,
    pub nama_lengkap: String,
    pub shu_jasa_modal_idr: u64,     // Berdasarkan persentase simpanan
    pub shu_jasa_usaha_idr: u64,     // Berdasarkan persentase transaksi
    pub total_shu_diterima_idr: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaporanShuKoperasi {
    pub tahun_buku: u32,
    pub total_shu_kotor_idr: u64,
    pub dana_cadangan_idr: u64,      // Minimal 25% untuk cadangan modal
    pub dana_jasa_modal_idr: u64,    // Alokasi untuk jasa simpanan anggota (e.g. 20%)
    pub dana_jasa_usaha_idr: u64,    // Alokasi untuk jasa transaksi anggota (e.g. 25%)
    pub dana_pengurus_pengawas_idr: u64, // (e.g. 10%)
    pub dana_karyawan_idr: u64,      // (e.g. 5%)
    pub dana_sosial_pendidikan_idr: u64, // (e.g. 10%)
    pub dana_pembangunan_desa_idr: u64, // (e.g. 5%)
}

pub struct UnitPerkantoranKoperasi {
    pub nama_koperasi: String,
    pub nomor_badan_hukum: String,
    pub desa_kelurahan: String,
    pub anggota: HashMap<String, AnggotaKoperasi>,
}

impl UnitPerkantoranKoperasi {
    pub fn new(nama: &str, no_badan_hukum: &str, desa: &str) -> Self {
        Self {
            nama_koperasi: nama.to_string(),
            nomor_badan_hukum: no_badan_hukum.to_string(),
            desa_kelurahan: desa.to_string(),
            anggota: HashMap::new(),
        }
    }

    /// Mendaftarkan anggota baru Koperasi Merah Putih
    pub fn daftarkan_anggota(
        &mut self,
        npa: &str,
        nik: &str,
        nama: &str,
        alamat: &str,
        desa: &str,
        telp: &str,
        simpanan_pokok: u64,
    ) -> AnggotaKoperasi {
        let a = AnggotaKoperasi {
            nomor_pokok_anggota: npa.to_string(),
            nik: nik.to_string(),
            nama_lengkap: nama.to_string(),
            alamat_rt_rw: alamat.to_string(),
            desa_kelurahan: desa.to_string(),
            nomor_telepon: telp.to_string(),
            simpanan_pokok_idr: simpanan_pokok,
            simpanan_wajib_idr: 0,
            status: StatusKeanggotaan::AnggotaBiasa,
            tanggal_bergabung_iso: "2026-10-01T00:00:00Z".to_string(),
            total_partisipasi_transaksi_idr: 0,
        };
        self.anggota.insert(npa.to_string(), a.clone());
        a
    }

    /// Setor simpanan wajib bulanan
    pub fn setor_simpanan_wajib(&mut self, npa: &str, nominal: u64) -> Result<u64, String> {
        if let Some(a) = self.anggota.get_mut(npa) {
            a.simpanan_wajib_idr += nominal;
            Ok(a.simpanan_wajib_idr)
        } else {
            Err(format!("Anggota dengan NPA {} tidak ditemukan", npa))
        }
    }

    /// Catat partisipasi transaksi anggota di unit-unit usaha
    pub fn catat_transaksi_anggota(&mut self, npa: &str, nominal_transaksi: u64) {
        if let Some(a) = self.anggota.get_mut(npa) {
            a.total_partisipasi_transaksi_idr += nominal_transaksi;
        }
    }

    /// Hitung alokasi Sisa Hasil Usaha (SHU) tahunan secara matematis
    pub fn hitung_shu_tahunan(&self, tahun: u32, total_shu_kotor: u64) -> (LaporanShuKoperasi, Vec<AlokasiShuAnggota>) {
        let cadangan = (total_shu_kotor as f64 * 0.25).round() as u64;
        let jasa_modal = (total_shu_kotor as f64 * 0.20).round() as u64;
        let jasa_usaha = (total_shu_kotor as f64 * 0.25).round() as u64;
        let pengurus = (total_shu_kotor as f64 * 0.10).round() as u64;
        let karyawan = (total_shu_kotor as f64 * 0.05).round() as u64;
        let sosial_pend = (total_shu_kotor as f64 * 0.10).round() as u64;
        let pbt_desa = (total_shu_kotor as f64 * 0.05).round() as u64;

        let laporan = LaporanShuKoperasi {
            tahun_buku: tahun,
            total_shu_kotor_idr: total_shu_kotor,
            dana_cadangan_idr: cadangan,
            dana_jasa_modal_idr: jasa_modal,
            dana_jasa_usaha_idr: jasa_usaha,
            dana_pengurus_pengawas_idr: pengurus,
            dana_karyawan_idr: karyawan,
            dana_sosial_pendidikan_idr: sosial_pend,
            dana_pembangunan_desa_idr: pbt_desa,
        };

        let total_seluruh_simpanan: u64 = self.anggota.values().map(|a| a.simpanan_pokok_idr + a.simpanan_wajib_idr).sum();
        let total_seluruh_transaksi: u64 = self.anggota.values().map(|a| a.total_partisipasi_transaksi_idr).sum();

        let mut daftar_shu = Vec::new();

        for a in self.anggota.values() {
            let simpanan_a = a.simpanan_pokok_idr + a.simpanan_wajib_idr;
            let shu_modal = if total_seluruh_simpanan > 0 {
                ((simpanan_a as f64 / total_seluruh_simpanan as f64) * jasa_modal as f64).round() as u64
            } else {
                0
            };

            let shu_usaha = if total_seluruh_transaksi > 0 {
                ((a.total_partisipasi_transaksi_idr as f64 / total_seluruh_transaksi as f64) * jasa_usaha as f64).round() as u64
            } else {
                0
            };

            daftar_shu.push(AlokasiShuAnggota {
                nomor_pokok_anggota: a.nomor_pokok_anggota.clone(),
                nama_lengkap: a.nama_lengkap.clone(),
                shu_jasa_modal_idr: shu_modal,
                shu_jasa_usaha_idr: shu_usaha,
                total_shu_diterima_idr: shu_modal + shu_usaha,
            });
        }

        (laporan, daftar_shu)
    }
}
