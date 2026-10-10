// Projects/arenasilat/src/asuransi_and_klaim_bpjs.rs
// ==============================================================================
// ARENA SILAT SAAS — ASURANSI ATLET & KLAIM MEDIS BPJS KETENAGAKERJAAN
// 100% Zero External Dependencies — Berdaulat Penuh di atas Widya-Lang
// Sesuai Dokumen:
// - docs/integrasi_asuransi_dan_perlindungan_medis.md
// - docs/sop_penanganan_atlet_cedera_dan_medis.md
// ==============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JenisPerlindunganAsuransi {
    BpjsKetenagakerjaanAtlet,
    AsuransiKecelakaanOlahragaSwasta,
    JaminanPenyelenggaraTurnamen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TingkatCederaKlaim {
    RinganPerawatanMatras,
    SedangDislokasiSprain,
    BeratFrakturAtauGegarOtak,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusKlaimMedis {
    PolisTerdaftarAktif,
    VerifikasiDokterTurnamen,
    RujukanUgdBuktiDiterbitkan,
    CoverPenuhDisetujui,
    SelesaiPelunasanRsud,
}

#[derive(Debug, Clone)]
pub struct PolisAsuransiAtlet {
    pub nomor_polis_bpjs: String,
    pub atlet_id: String,
    pub nama_atlet: String,
    pub kontingen: String,
    pub plafon_maksimal_idr: u64,
    pub status_aktif: bool,
    pub berlaku_dari_tanggal: String,
    pub berlaku_sampai_tanggal: String,
}

#[derive(Debug, Clone)]
pub struct BerkasKlaimRujukanRsud {
    pub klaim_id: String,
    pub nomor_rujukan: String,
    pub nomor_polis: String,
    pub nama_atlet: String,
    pub rumah_sakit_rujukan: String,
    pub tingkat_cedera: TingkatCederaKlaim,
    pub diagnosa_dokter_pertandingan: String,
    pub nama_dokter_pj: String,
    pub no_str_dokter: String,
    pub status_klaim: StatusKlaimMedis,
    pub biaya_ditanggung_idr: u64,
    pub qr_verifikasi_klaim: String,
}

pub struct InsuranceEngine;

impl InsuranceEngine {
    pub const PLAFON_STANDAR_BPJS_ATLET: u64 = 50_000_000; // Rp 50.000.000,- (Unlimited sesuai indikasi medis)

    /// Daftarkan polis asuransi perlindungan atlet kejuaraan
    pub fn daftarkan_polis_atlet(
        nomor_bpjs: &str,
        atlet_id: &str,
        nama: &str,
        kontingen: &str,
    ) -> PolisAsuransiAtlet {
        PolisAsuransiAtlet {
            nomor_polis_bpjs: nomor_bpjs.to_string(),
            atlet_id: atlet_id.to_string(),
            nama_atlet: nama.to_string(),
            kontingen: kontingen.to_string(),
            plafon_maksimal_idr: Self::PLAFON_STANDAR_BPJS_ATLET,
            status_aktif: true,
            berlaku_dari_tanggal: "2026-10-01".to_string(),
            berlaku_sampai_tanggal: "2026-10-31".to_string(),
        }
    }

    /// Terbitkan surat rujukan dan klaim asuransi jalur hijau IGD RSUD
    pub fn terbitkan_klaim_rujukan_rsud(
        klaim_id: &str,
        polis: &PolisAsuransiAtlet,
        rsud_tujuan: &str,
        tingkat_cedera: TingkatCederaKlaim,
        diagnosa: &str,
        dokter_pj: &str,
        no_str: &str,
        estimasi_biaya: u64,
    ) -> BerkasKlaimRujukanRsud {
        let no_rujukan = format!("RUJUKAN-MEDIS-AS/{}", klaim_id);
        let qr_klaim = format!("https://arenasilat.id/klaim-bpjs/{}/{}", polis.nomor_polis_bpjs, klaim_id);

        BerkasKlaimRujukanRsud {
            klaim_id: klaim_id.to_string(),
            nomor_rujukan: no_rujukan,
            nomor_polis: polis.nomor_polis_bpjs.clone(),
            nama_atlet: polis.nama_atlet.clone(),
            rumah_sakit_rujukan: rsud_tujuan.to_string(),
            tingkat_cedera,
            diagnosa_dokter_pertandingan: diagnosa.to_string(),
            nama_dokter_pj: dokter_pj.to_string(),
            no_str_dokter: no_str.to_string(),
            status_klaim: StatusKlaimMedis::CoverPenuhDisetujui,
            biaya_ditanggung_idr: estimasi_biaya.min(polis.plafon_maksimal_idr),
            qr_verifikasi_klaim: qr_klaim,
        }
    }
}
