// Projects/arenasilat/src/federasi_and_perizinan.rs
// ==============================================================================
// ARENA SILAT SAAS — MODUL FEDERASI IPSI BERJENJANG & PERIZINAN TURNAMEN
// 100% Zero External Dependencies — Berdaulat Penuh di atas Widya-Lang
// Sesuai Dokumen:
// - docs/dashboard_federasi_ipsi_berjenjang.md
// - docs/panduan_administrasi_dan_perizinan.md
// ==============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TingkatFederasi {
    Pengcab(String),   // Pengurus Cabang Kabupaten / Kota
    Pengprov(String),  // Pengurus Provinsi
    PbIpsi,            // Pengurus Besar Ikatan Pencak Silat Indonesia
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TingkatLisensiWasitJuri {
    WasitJuriCabangKelas3,
    WasitJuriDaerahKelas2,
    WasitJuriNasionalKelas1,
    WasitJuriInternasionalPersilat,
}

#[derive(Debug, Clone)]
pub struct WasitJuriTervalidasi {
    pub lisensi_id: String,
    pub nama_lengkap: String,
    pub tingkat_lisensi: TingkatLisensiWasitJuri,
    pub perguruan_asal: String,
    pub pengprov_asal: String,
    pub tahun_kelulusan_penataran: u32,
    pub masa_berlaku_sampai: u32,
    pub total_partai_dipimpin: u32,
    pub status_skorsing: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusIzinTurnamen {
    DraftPengajuan,
    MenungguReviewPengcab,
    MenungguReviewPengprov,
    DisetujuiRekomendasi,
    Ditolak(String),
}

#[derive(Debug, Clone)]
pub struct SuratRekomendasiIzin {
    pub nomor_rekomendasi: String,
    pub turnamen_id: String,
    pub nama_turnamen: String,
    pub instansi_penyelenggara_eo: String,
    pub tingkat_kejuaraan: TingkatFederasi,
    pub lokasi_gor: String,
    pub estimasi_peserta: u32,
    pub status_izin: StatusIzinTurnamen,
    pub nomor_surat_polres_polda: Option<String>,
    pub mou_rsud_rujukan: bool,
}

#[derive(Debug, Clone)]
pub struct AuditKelayakanTurnamen {
    pub turnamen_id: String,
    pub matras_standar_ipsi: bool,
    pub jumlah_gelanggang: u32,
    pub dokter_dan_paramedis_siap: bool,
    pub ambulans_standby: bool,
    pub timbangan_digital_terkalibrasi: bool,
    pub wasit_juri_berlisensi_cukup: bool,
    pub skor_kelayakan_persen: f64,
    pub status_rekomendasi_kelayakan: bool,
}

pub struct FederasiEngine;

impl FederasiEngine {
    /// Validasi kelayakan lisensi Wasit/Juri memimpin suatu partai kejuaraan
    pub fn validasi_kelayakan_wasit(
        wasit: &WasitJuriTervalidasi,
        tahun_event: u32,
        tingkat_event: &TingkatFederasi,
    ) -> (bool, String) {
        if wasit.status_skorsing {
            return (false, format!("Wasit {} sedang dalam masa sanksi / skorsing disiplin", wasit.nama_lengkap));
        }

        if tahun_event > wasit.masa_berlaku_sampai {
            return (false, format!("Lisensi wasit {} telah kedaluwarsa pada tahun {}", wasit.nama_lengkap, wasit.masa_berlaku_sampai));
        }

        match (tingkat_event, &wasit.tingkat_lisensi) {
            (TingkatFederasi::PbIpsi, TingkatLisensiWasitJuri::WasitJuriNasionalKelas1)
            | (TingkatFederasi::PbIpsi, TingkatLisensiWasitJuri::WasitJuriInternasionalPersilat) => {
                (true, "Memenuhi syarat memimpin Kejuaraan Nasional / PB IPSI".to_string())
            }
            (TingkatFederasi::PbIpsi, _) => {
                (false, "Kejuaraan Nasional membutuhkan Lisensi Wasit Juri Minimal Kelas 1 Nasional".to_string())
            }
            (TingkatFederasi::Pengprov(_), TingkatLisensiWasitJuri::WasitJuriCabangKelas3) => {
                (false, "Kejuaraan Tingkat Provinsi membutuhkan Lisensi Minimal Kelas 2 Daerah".to_string())
            }
            _ => (true, "Kualifikasi lisensi wasit juri sah dan terverifikasi".to_string()),
        }
    }

    /// Buat pengajuan surat izin dan verifikasi rekomendasi IPSI
    pub fn proses_surat_rekomendasi(
        nomor_rekomendasi: &str,
        turnamen_id: &str,
        nama_turnamen: &str,
        eo: &str,
        tingkat: TingkatFederasi,
        gor: &str,
        peserta: u32,
        mou_rsud: bool,
    ) -> SuratRekomendasiIzin {
        let status = if !mou_rsud {
            StatusIzinTurnamen::Ditolak("Wajib memiliki MoU Tertulis dengan Rumah Sakit Rujukan / UGD terdekat".to_string())
        } else {
            match tingkat {
                TingkatFederasi::Pengcab(_) => StatusIzinTurnamen::MenungguReviewPengcab,
                TingkatFederasi::Pengprov(_) => StatusIzinTurnamen::MenungguReviewPengprov,
                TingkatFederasi::PbIpsi => StatusIzinTurnamen::DisetujuiRekomendasi,
            }
        };

        SuratRekomendasiIzin {
            nomor_rekomendasi: nomor_rekomendasi.to_string(),
            turnamen_id: turnamen_id.to_string(),
            nama_turnamen: nama_turnamen.to_string(),
            instansi_penyelenggara_eo: eo.to_string(),
            tingkat_kejuaraan: tingkat,
            lokasi_gor: gor.to_string(),
            estimasi_peserta: peserta,
            status_izin: status,
            nomor_surat_polres_polda: None,
            mou_rsud_rujukan: mou_rsud,
        }
    }

    /// Audit komprehensif sarana gelanggang sesuai SOP IPSI 2026
    pub fn audit_sarana_dan_kelayakan(
        turnamen_id: &str,
        matras: bool,
        gelanggang_count: u32,
        dokter: bool,
        ambulans: bool,
        timbangan_calibrated: bool,
        wasit_cukup: bool,
    ) -> AuditKelayakanTurnamen {
        let mut poin = 0.0;
        if matras { poin += 20.0; }
        if gelanggang_count > 0 { poin += 15.0; }
        if dokter { poin += 25.0; }
        if ambulans { poin += 15.0; }
        if timbangan_calibrated { poin += 10.0; }
        if wasit_cukup { poin += 15.0; }

        let layak = poin >= 80.0 && dokter && matras && ambulans;

        AuditKelayakanTurnamen {
            turnamen_id: turnamen_id.to_string(),
            matras_standar_ipsi: matras,
            jumlah_gelanggang: gelanggang_count,
            dokter_dan_paramedis_siap: dokter,
            ambulans_standby: ambulans,
            timbangan_digital_terkalibrasi: timbangan_calibrated,
            wasit_juri_berlisensi_cukup: wasit_cukup,
            skor_kelayakan_persen: poin,
            status_rekomendasi_kelayakan: layak,
        }
    }
}
