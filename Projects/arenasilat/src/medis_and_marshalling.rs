// Projects/arenasilat/src/medis_and_marshalling.rs
// ==============================================================================
// ARENA SILAT SAAS — KONSOL DOKTER MEDIS & MARSHALLING TIMBANGAN BLE (IPSI 2026)
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

/// Bagian Tubuh Cedera
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BagianTubuhCedera {
    KepalaWajah,
    BahuKlavikula,
    DadaRusuk,
    SikuLengan,
    LututKaki,
    PergelanganKaki,
}

/// Diagnosis Insiden Medis
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KategoriDiagnosis {
    MemarRingan,
    SprainStrain,
    DislokasiSendi,
    FrakturTulang,
    GegarOtakKO, // Concussion - Wajib Skorsing Medis 30 Hari
    LukaRobekPendarahan,
}

/// Keputusan Dokter Pertandingan (Doctor Stoppage Authority)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeputusanDokter {
    FitLanjutTanding,
    UnfitHentikanLagaWMP, // Wasit Menghentikan Pertandingan karena medis
    RujukIgdRsud,
}

/// Rekod Insiden Medis di Gelanggang
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RekodInsidenMedis {
    pub insiden_id: String,
    pub partai_id: String,
    pub atlet_id: String,
    pub nama_atlet: String,
    pub sudut: String,
    pub bagian_tubuh: BagianTubuhCedera,
    pub diagnosis: KategoriDiagnosis,
    pub durasi_penanganan_detik: u32, // Maksimal 180 detik (3 Menit)
    pub keputusan: KeputusanDokter,
    pub skorsing_medis_hari: u32,
    pub rujukan_bpjs_id: Option<String>,
}

/// Mesin Dokter Pertandingan & Tim Medis
pub struct MedicalEngine {
    pub daftar_insiden: Vec<RekodInsidenMedis>,
}

impl MedicalEngine {
    pub fn baru() -> Self {
        Self {
            daftar_insiden: Vec::new(),
        }
    }

    /// Catat Insiden Medis dan Ambil Keputusan Dokter
    pub fn catat_insiden(
        &mut self,
        partai_id: &str,
        atlet_id: &str,
        nama: &str,
        sudut: &str,
        bagian: BagianTubuhCedera,
        diag: KategoriDiagnosis,
        durasi_detik: u32,
        keputusan: KeputusanDokter,
    ) -> RekodInsidenMedis {
        let skorsing = if diag == KategoriDiagnosis::GegarOtakKO { 30 } else { 0 };
        let rujukan = if keputusan == KeputusanDokter::RujukIgdRsud {
            Some(format!("BPJS-SJP-RSUD-{}-{}", partai_id, atlet_id))
        } else {
            None
        };

        let insiden = RekodInsidenMedis {
            insiden_id: format!("MED-{}-{}", partai_id, self.daftar_insiden.len() + 1),
            partai_id: partai_id.to_string(),
            atlet_id: atlet_id.to_string(),
            nama_atlet: nama.to_string(),
            sudut: sudut.to_string(),
            bagian_tubuh: bagian,
            diagnosis: diag,
            durasi_penanganan_detik: durasi_detik.min(180),
            keputusan,
            skorsing_medis_hari: skorsing,
            rujukan_bpjs_id: rujukan,
        };

        self.daftar_insiden.push(insiden.clone());
        insiden
    }
}

/// Status Antrean Call Room Marshalling
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusMarshalling {
    MenungguDiRuangPemanasan, // H-3 Partai
    PemeriksaanPerlengkapan,   // H-2 Partai (Body protector, Sabuk, Deker, Cap)
    SiapDiPintuMasukGelanggang,// H-1 Partai
    Bertanding,
    WalkoverCandidate,        // Tidak hadir saat panggilan ke-3
}

/// Data Pesilat di Call Room Marshalling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntreanMarshalling {
    pub nomor_antrean: u32,
    pub nomor_partai: u32,
    pub gelanggang_id: u8,
    pub atlet_id: String,
    pub nama_atlet: String,
    pub kontingen: String,
    pub sudut: String,
    pub berat_badan_kg: f32,
    pub batas_min_kg: f32,
    pub batas_max_kg: f32,
    pub status_timbangan: bool,
    pub perlengkapan_lengkap: bool,
    pub status: StatusMarshalling,
}

/// Mesin Petugas Marshalling & Timbangan Digital
pub struct MarshallingEngine {
    pub daftar_antrean: Vec<AntreanMarshalling>,
}

impl MarshallingEngine {
    pub fn baru() -> Self {
        Self {
            daftar_antrean: Vec::new(),
        }
    }

    /// Tambahkan Atlet ke Antrean Call Room
    pub fn tambah_antrean(
        &mut self,
        partai: u32,
        gelanggang: u8,
        atlet_id: &str,
        nama: &str,
        kontingen: &str,
        sudut: &str,
        bb: f32,
        min_bb: f32,
        max_bb: f32,
    ) {
        let lolos_timbang = bb >= min_bb && bb <= max_bb;
        let item = AntreanMarshalling {
            nomor_antrean: (self.daftar_antrean.len() + 1) as u32,
            nomor_partai: partai,
            gelanggang_id: gelanggang,
            atlet_id: atlet_id.to_string(),
            nama_atlet: nama.to_string(),
            kontingen: kontingen.to_string(),
            sudut: sudut.to_string(),
            berat_badan_kg: bb,
            batas_min_kg: min_bb,
            batas_max_kg: max_bb,
            status_timbangan: lolos_timbang,
            perlengkapan_lengkap: false,
            status: StatusMarshalling::MenungguDiRuangPemanasan,
        };
        self.daftar_antrean.push(item);
    }

    /// Verifikasi Kelengkapan Pelindung Atlet
    pub fn verifikasi_pelindung(&mut self, atlet_id: &str) -> bool {
        if let Some(a) = self.daftar_antrean.iter_mut().find(|a| a.atlet_id == atlet_id) {
            a.perlengkapan_lengkap = true;
            a.status = StatusMarshalling::SiapDiPintuMasukGelanggang;
            return true;
        }
        false
    }

    /// Tandai Sebagai Kandidat Walkover (WO)
    pub fn tandai_walkover(&mut self, atlet_id: &str) -> bool {
        if let Some(a) = self.daftar_antrean.iter_mut().find(|a| a.atlet_id == atlet_id) {
            a.status = StatusMarshalling::WalkoverCandidate;
            return true;
        }
        false
    }
}
