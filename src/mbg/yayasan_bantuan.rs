use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 3. YAYASAN & SATUAN PENDIDIKAN PENERIMA BANTUAN PEMERINTAH (BANSOS / APBN MBG)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JenisLembagaPenerima {
    SekolahNegeri,
    SekolahSwastaYayasan,
    PondokPesantren,
    PantiAsuhanSosial,
    MadrasahDiniyah,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YayasanPenerimaBantuan {
    pub id_yayasan_sekolah: String,
    pub npsn_atau_nsa: String,       // Nomor Pokok Sekolah Nasional / Nomor Statistik Asrama
    pub nama_lembaga: String,
    pub jenis_lembaga: JenisLembagaPenerima,
    pub nomor_akta_kemenkumham: String,
    pub rekening_bank_penyalur: String,
    pub nama_kepala_atau_ketua: String,
    pub total_sasaran_siswa: u32,
    pub total_dana_bantuan_diterima_idr: u64,
    pub total_dana_terealisasi_idr: u64,
    pub status_verifikasi_kemenkeu_bapanas: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpjBuktiBelanjaDigital {
    pub nomor_spj: String,
    pub id_yayasan_sekolah: String,
    pub tanggal_iso: String,
    pub nama_penyedia_koperasi_sppg: String,
    pub jumlah_porsi_dipesan: u32,
    pub total_nominal_belanja_idr: u64,
    pub nomor_e_faktur_pajak_atau_kuitansi: String,
    pub link_foto_bukti_distribusi: String,
    pub status_audit_inspektorat: String, // "VALID_TERVERIFIKASI", "PERLU_REVISI"
}

pub struct ManajemenBantuanYayasan {
    pub daftar_lembaga: HashMap<String, YayasanPenerimaBantuan>,
    pub laporan_spj: Vec<SpjBuktiBelanjaDigital>,
}

impl ManajemenBantuanYayasan {
    pub fn new() -> Self {
        Self {
            daftar_lembaga: HashMap::new(),
            laporan_spj: Vec::new(),
        }
    }

    /// Registrasi & verifikasi lembaga/yayasan penerima bantuan
    pub fn daftarkan_lembaga(
        &mut self,
        id: &str,
        npsn: &str,
        nama: &str,
        jenis: JenisLembagaPenerima,
        akta: &str,
        rek_bank: &str,
        kepala: &str,
        siswa_count: u32,
    ) -> YayasanPenerimaBantuan {
        let y = YayasanPenerimaBantuan {
            id_yayasan_sekolah: id.to_string(),
            npsn_atau_nsa: npsn.to_string(),
            nama_lembaga: nama.to_string(),
            jenis_lembaga: jenis,
            nomor_akta_kemenkumham: akta.to_string(),
            rekening_bank_penyalur: rek_bank.to_string(),
            nama_kepala_atau_ketua: kepala.to_string(),
            total_sasaran_siswa: siswa_count,
            total_dana_bantuan_diterima_idr: 0,
            total_dana_terealisasi_idr: 0,
            status_verifikasi_kemenkeu_bapanas: true,
        };
        self.daftar_lembaga.insert(id.to_string(), y.clone());
        y
    }

    /// Penyaluran pagu alokasi dana bantuan APBN per termin (e.g. 20 hari makan x Rp 15.000 x jumlah siswa)
    pub fn salurkan_dana_termin(&mut self, id: &str, hari_efektif: u32, tarif_porsi: u64) -> Result<u64, String> {
        if let Some(y) = self.daftar_lembaga.get_mut(id) {
            let total_alokasi = (y.total_sasaran_siswa as u64) * (hari_efektif as u64) * tarif_porsi;
            y.total_dana_bantuan_diterima_idr += total_alokasi;
            Ok(y.total_dana_bantuan_diterima_idr)
        } else {
            Err(format!("Lembaga ID {} tidak ditemukan", id))
        }
    }

    /// Pengunggahan SPJ Digital bukti penyerapan anggaran makan bergizi
    pub fn unggah_spj_digital(
        &mut self,
        no_spj: &str,
        id_lembaga: &str,
        penyedia_sppg: &str,
        porsi: u32,
        total_nominal: u64,
        faktur: &str,
    ) -> Result<SpjBuktiBelanjaDigital, String> {
        if let Some(y) = self.daftar_lembaga.get_mut(id_lembaga) {
            y.total_dana_terealisasi_idr += total_nominal;

            let spj = SpjBuktiBelanjaDigital {
                nomor_spj: no_spj.to_string(),
                id_yayasan_sekolah: id_lembaga.to_string(),
                tanggal_iso: "2026-10-01T12:00:00Z".to_string(),
                nama_penyedia_koperasi_sppg: penyedia_sppg.to_string(),
                jumlah_porsi_dipesan: porsi,
                total_nominal_belanja_idr: total_nominal,
                nomor_e_faktur_pajak_atau_kuitansi: faktur.to_string(),
                link_foto_bukti_distribusi: format!("https://mbg.kemendikdasmen.go.id/dokumen/spj/{}.jpg", no_spj),
                status_audit_inspektorat: "VALID_TERVERIFIKASI".to_string(),
            };

            self.laporan_spj.push(spj.clone());
            Ok(spj)
        } else {
            Err(format!("Lembaga ID {} tidak ditemukan", id_lembaga))
        }
    }
}
