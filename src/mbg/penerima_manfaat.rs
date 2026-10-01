use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 4. PENERIMA MANFAAT (BENEFICIARIES), PRESENSI QR & PEMANTAUAN TUMBUH KEMBANG
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusImtSiswa {
    GiziBurukSangatKurus,
    KurusUnderweight,
    NormalIdeal,
    KelebihanBeratBadanOverweight,
    Obesitas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiswaPenerimaManfaat {
    pub nisn_atau_nik: String,
    pub nama_lengkap: String,
    pub sekolah_id: String,
    pub kelas: String,               // e.g. "Kelas 4A", "Kelas 7B", "Santri Aliyah 1"
    pub tanggal_lahir_iso: String,
    pub jenis_kelamin: String,       // "Laki-laki", "Perempuan"
    pub tinggi_badan_cm: f64,
    pub berat_badan_kg: f64,
    pub status_imt: StatusImtSiswa,
    pub riwayat_alergi_makanan: Option<String>, // e.g. "Alergi Seafood / Udang", "Intoleransi Laktosa"
    pub total_porsi_diterima: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogPresensiPorsiHarian {
    pub id_log: String,
    pub nisn_atau_nik: String,
    pub tanggal_iso: String,
    pub jam_distribusi_iso: String,
    pub status_penerimaan: bool,
    pub metode_tapping: String, // "QR_CARD_SISWA", "FACE_BIOMETRIC", "MANUAL_GURU"
}

pub struct ManajemenPenerimaManfaat {
    pub daftar_siswa: HashMap<String, SiswaPenerimaManfaat>, // Key: NISN / NIK
    pub log_harian: Vec<LogPresensiPorsiHarian>,
}

impl ManajemenPenerimaManfaat {
    pub fn new() -> Self {
        Self {
            daftar_siswa: HashMap::new(),
            log_harian: Vec::new(),
        }
    }

    /// Hitung Indeks Massa Tubuh (IMT / BMI = BB / TB_m^2) dan tentukan status gizi
    pub fn hitung_status_imt(tb_cm: f64, bb_kg: f64) -> (f64, StatusImtSiswa) {
        let tb_m = tb_cm / 100.0;
        let imt = if tb_m > 0.0 { bb_kg / (tb_m * tb_m) } else { 0.0 };

        let status = if imt < 14.5 {
            StatusImtSiswa::GiziBurukSangatKurus
        } else if imt < 17.0 {
            StatusImtSiswa::KurusUnderweight
        } else if imt <= 23.5 {
            StatusImtSiswa::NormalIdeal
        } else if imt <= 27.0 {
            StatusImtSiswa::KelebihanBeratBadanOverweight
        } else {
            StatusImtSiswa::Obesitas
        };

        ((imt * 10.0).round() / 10.0, status)
    }

    /// Mendaftarkan penerima manfaat siswa/santri
    pub fn daftarkan_siswa(
        &mut self,
        nisn: &str,
        nama: &str,
        sekolah_id: &str,
        kelas: &str,
        dob: &str,
        jk: &str,
        tb_cm: f64,
        bb_kg: f64,
        alergi: Option<&str>,
    ) -> SiswaPenerimaManfaat {
        let (_, status_imt) = Self::hitung_status_imt(tb_cm, bb_kg);

        let s = SiswaPenerimaManfaat {
            nisn_atau_nik: nisn.to_string(),
            nama_lengkap: nama.to_string(),
            sekolah_id: sekolah_id.to_string(),
            kelas: kelas.to_string(),
            tanggal_lahir_iso: dob.to_string(),
            jenis_kelamin: jk.to_string(),
            tinggi_badan_cm: tb_cm,
            berat_badan_kg: bb_kg,
            status_imt,
            riwayat_alergi_makanan: alergi.map(|s| s.to_string()),
            total_porsi_diterima: 0,
        };

        self.daftar_siswa.insert(nisn.to_string(), s.clone());
        s
    }

    /// Tapping QR penerimaan porsi makan siang siswa (Mencegah klaim porsi ganda)
    pub fn catat_presensi_porsi(
        &mut self,
        nisn: &str,
        metode: &str,
    ) -> Result<LogPresensiPorsiHarian, String> {
        let tanggal_hari_ini = "2026-10-01";
        
        // Cek apakah siswa sudah menerima makan hari ini
        let sudah_terima = self.log_harian.iter().any(|l| l.nisn_atau_nik == nisn && l.tanggal_iso == tanggal_hari_ini);
        if sudah_terima {
            return Err(format!("Peringatan: Siswa NISN {} sudah tercatat menerima porsi makan siang hari ini!", nisn));
        }

        if let Some(s) = self.daftar_siswa.get_mut(nisn) {
            s.total_porsi_diterima += 1;

            let log = LogPresensiPorsiHarian {
                id_log: format!("LOG-MBG-{}-{}", tanggal_hari_ini, self.log_harian.len() + 1),
                nisn_atau_nik: nisn.to_string(),
                tanggal_iso: tanggal_hari_ini.to_string(),
                jam_distribusi_iso: "11:45:00".to_string(),
                status_penerimaan: true,
                metode_tapping: metode.to_string(),
            };

            self.log_harian.push(log.clone());
            Ok(log)
        } else {
            Err(format!("Siswa NISN {} tidak ditemukan di database penerima manfaat", nisn))
        }
    }
}
