use serde::{Deserialize, Serialize};

/// ============================================================================
/// PERJANJIAN KERJA SAMA (PKS) BANTUAN PEMERINTAH (BANPER) APBN
/// Modul Legalitas, Komitmen PPK, Otoritas Pengesahan & Pendelegasian BPMP/Dinas
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JenisBanperApbn {
    RevitalisasiSarprasDanGedung,
    DigitalisasiSmartClassroom,
    LaboratoriumVokasiSmk,
    DukunganDapurSentralMbgSekolah,
    BantuanAfirmasiPeralatanBelajar,
    BantuanOperasionalRintisanSekolah,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusPksBanper {
    DraftPengajuan,
    VerifikasiKelayakanAdministrasi,
    DitandatanganiParaPihak,
    TerminPertamaDicairkan,
    TerminKeduaDicairkan,
    SelesaiBastPekerjaan,
    DibatalkanWanprestasi,
}

/// ============================================================================
/// OTORITAS PENGESAHAN DANA APBN: DIREKTORAT PUSAT, DELEGASI BPMP / DINAS
/// ============================================================================
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OtoritasPengesahanApbn {
    /// Pengesahan langsung oleh Direktorat Teknis Kemendikdasmen RI
    DirektoratPusat {
        nama_direktorat: String, // e.g. "Direktorat Sekolah Dasar", "Direktorat SMK"
        nama_pejabat_direktur_atau_ppk: String,
        nip_pejabat: String,
    },
    /// Pendelegasian wewenang ke BPMP (Balai Penjaminan Mutu Pendidikan) Provinsi
    DelegasiBpmpProvinsi {
        nama_bpmp: String, // e.g. "BPMP Provinsi Jawa Tengah"
        nomor_sk_delegasi_kementerian: String,
        nama_kepala_bpmp_atau_verifikator: String,
        nip_pejabat: String,
    },
    /// Pendelegasian wewenang ke Dinas Pendidikan Kabupaten/Kota/Provinsi
    DelegasiDinasPendidikan {
        nama_dinas: String, // e.g. "Dinas Pendidikan Kabupaten Banyumas"
        nomor_surat_pelimpahan_tugas: String,
        nama_kepala_dinas_atau_tim_bos: String,
        nip_pejabat: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PihakPertamaPPK {
    pub nama_pejabat_ppk: String,
    pub nip_ppk: String,
    pub jabatan: String, // e.g. "Pejabat Pembuat Komitmen Direktorat Sekolah Menengah"
    pub satuan_kerja_kementerian: String,
    pub alamat_kantor: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PihakKeduaPenerima {
    pub npsn: String,
    pub nama_sekolah: String,
    pub nama_kepala_sekolah_atau_ketua: String,
    pub nip_atau_nik: String,
    pub jabatan: String, // e.g. "Kepala Satuan Pendidikan", "Ketua Yayasan Pendidikan"
    pub nomor_rekening_khusus_banper: String,
    pub nama_bank_penyalur: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerminPenyaluranBanper {
    pub termin_ke: u32,
    pub persentase_pencairan: f64, // e.g. 70.0% dan 30.0%
    pub nominal_cair_rp: f64,
    pub syarat_progres_minimal_persen: f64,
    pub tanggal_realisasi_cair: Option<String>,
    pub nomor_sp2d_kppn: Option<String>,
    pub status_lunas: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerjanjianKerjaSamaBanper {
    pub nomor_pks_kementerian: String,
    pub nomor_pks_sekolah: String,
    pub tanggal_pengesahan: String,
    pub tahun_anggaran: u32,
    pub jenis_banper: JenisBanperApbn,
    pub pihak_pertama: PihakPertamaPPK,
    pub pihak_kedua: PihakKeduaPenerima,
    pub total_nilai_bantuan_rp: f64,
    pub termin_jadwal: Vec<TerminPenyaluranBanper>,
    pub pakta_integritas_ditandatangani: bool,
    pub kesiapan_rekening_terverifikasi: bool,
    pub status: StatusPksBanper,
    pub catatan_komitmen: String,
    pub otoritas_pengesah: Option<OtoritasPengesahanApbn>,
}

impl PerjanjianKerjaSamaBanper {
    /// Membuat draf PKS baru untuk Bantuan Pemerintah APBN
    pub fn buat_pks(
        no_pks_kemen: &str,
        no_pks_sek: &str,
        tgl_sah: &str,
        tahun: u32,
        jenis: JenisBanperApbn,
        pihak1: PihakPertamaPPK,
        pihak2: PihakKeduaPenerima,
        total_nilai_rp: f64,
    ) -> Self {
        // Skema standar Banper APBN: Termin I (70%), Termin II (30% setelah progres >= 50%)
        let nominal_t1 = total_nilai_rp * 0.70;
        let nominal_t2 = total_nilai_rp * 0.30;

        let termin_jadwal = vec![
            TerminPenyaluranBanper {
                termin_ke: 1,
                persentase_pencairan: 70.0,
                nominal_cair_rp: nominal_t1,
                syarat_progres_minimal_persen: 0.0,
                tanggal_realisasi_cair: None,
                nomor_sp2d_kppn: None,
                status_lunas: false,
            },
            TerminPenyaluranBanper {
                termin_ke: 2,
                persentase_pencairan: 30.0,
                nominal_cair_rp: nominal_t2,
                syarat_progres_minimal_persen: 50.0,
                tanggal_realisasi_cair: None,
                nomor_sp2d_kppn: None,
                status_lunas: false,
            },
        ];

        Self {
            nomor_pks_kementerian: no_pks_kemen.to_string(),
            nomor_pks_sekolah: no_pks_sek.to_string(),
            tanggal_pengesahan: tgl_sah.to_string(),
            tahun_anggaran: tahun,
            jenis_banper: jenis,
            pihak_pertama: pihak1,
            pihak_kedua: pihak2,
            total_nilai_bantuan_rp: total_nilai_rp,
            termin_jadwal,
            pakta_integritas_ditandatangani: false,
            kesiapan_rekening_terverifikasi: false,
            status: StatusPksBanper::DraftPengajuan,
            catatan_komitmen: "Wajib mematuhi Juknis Bantuan Pemerintah APBN & pelaporan berkala".to_string(),
            otoritas_pengesah: None,
        }
    }

    /// Verifikasi dan penandatanganan PKS oleh PPK / Pihak yang didelegasikan
    pub fn tandatangani_pks(
        &mut self,
        pakta_integritas: bool,
        rekening_valid: bool,
        pengesah: OtoritasPengesahanApbn,
    ) -> Result<(), String> {
        if !pakta_integritas {
            return Err("Pakta Integritas anti-korupsi & gratifikasi wajib ditandatangani!".to_string());
        }
        if !rekening_valid {
            return Err("Rekening bank khusus bantuan belum terverifikasi oleh KPPN!".to_string());
        }
        self.pakta_integritas_ditandatangani = true;
        self.kesiapan_rekening_terverifikasi = true;
        self.otoritas_pengesah = Some(pengesah);
        self.status = StatusPksBanper::DitandatanganiParaPihak;
        Ok(())
    }

    /// Realisasi pencairan Termin 1 melalui SP2D KPPN
    pub fn realisasi_termin_1(&mut self, tanggal: &str, no_sp2d: &str) -> Result<f64, String> {
        if self.status != StatusPksBanper::DitandatanganiParaPihak {
            return Err("PKS belum sah ditandatangani kedua belah pihak!".to_string());
        }
        if let Some(t1) = self.termin_jadwal.get_mut(0) {
            t1.tanggal_realisasi_cair = Some(tanggal.to_string());
            t1.nomor_sp2d_kppn = Some(no_sp2d.to_string());
            t1.status_lunas = true;
            self.status = StatusPksBanper::TerminPertamaDicairkan;
            Ok(t1.nominal_cair_rp)
        } else {
            Err("Jadwal Termin 1 tidak ditemukan!".to_string())
        }
    }

    /// Realisasi pencairan Termin 2 dengan validasi progres fisik & belanja minimal 50%
    pub fn realisasi_termin_2(&mut self, tanggal: &str, no_sp2d: &str, progres_aktual_persen: f64) -> Result<f64, String> {
        if self.status != StatusPksBanper::TerminPertamaDicairkan {
            return Err("Termin 1 belum dicairkan!".to_string());
        }
        if progres_aktual_persen < 50.0 {
            return Err(format!(
                "Progres fisik/SPJ belum memenuhi syarat (Aktual: {:.1}%, Syarat minimal: 50.0%)!",
                progres_aktual_persen
            ));
        }

        if let Some(t2) = self.termin_jadwal.get_mut(1) {
            t2.tanggal_realisasi_cair = Some(tanggal.to_string());
            t2.nomor_sp2d_kppn = Some(no_sp2d.to_string());
            t2.status_lunas = true;
            self.status = StatusPksBanper::TerminKeduaDicairkan;
            Ok(t2.nominal_cair_rp)
        } else {
            Err("Jadwal Termin 2 tidak ditemukan!".to_string())
        }
    }
}
