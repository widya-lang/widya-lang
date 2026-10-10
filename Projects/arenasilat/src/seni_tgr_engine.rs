// Projects/arenasilat/src/seni_tgr_engine.rs
// ==============================================================================
// ARENA SILAT SAAS — SOVEREIGN SCORING ENGINE UNTUK SENI TGR & FESTIVAL (IPSI 2026)
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// Tunggal Baku, Ganda Senjata, Regu Jurus, Solo Kreatif Musik
// ==============================================================================

use serde::{Deserialize, Serialize};

/// Kategori Nomor Seni
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NomorSeni {
    TunggalBaku,
    GandaSenjata,
    ReguJurus,
    SoloKreatif,
}

/// Input Penilaian 1 Juri Seni
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NilaiJuriSeni {
    pub juri_id: usize,
    pub skor_kebenaran_gerak: f32, // Skor awal 9.90 - akumulasi salah gerak
    pub skor_kemantapan_stamina: f32, // Rentang 0.01 - 0.10
    pub pengurangan_senjata_lepas: bool, // -0.50 per kejadian
    pub pengurangan_senjata_keluar: bool, // -0.50
    pub pengurangan_pakaian: bool, // -0.05
    pub total_skor_juri: f32,
}

/// Rekod Penampilan Pesilat Seni
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RekodPenampilanSeni {
    pub nomor_partai: String,
    pub nomor_seni: NomorSeni,
    pub nama_pesilat: String,
    pub kontingen: String,
    pub durasi_aktual_detik: u32, // Target: 180 detik (3 Menit). Toleransi: 175 - 185s
    pub pinalti_waktu: f32,
    pub daftar_nilai_juri: Vec<NilaiJuriSeni>,
    pub skor_akhir_terpangkas: f32, // Trimmed Mean (buang nilai tertinggi & terendah)
}

/// Mesin Penilaian Seni TGR IPSI 2026
pub struct SeniTGREngine;

impl SeniTGREngine {
    /// Hitung Pinalti Deviasi Waktu (Toleransi 2:55 s.d. 3:05 -> 175s s.d. 185s)
    pub fn hitung_pinalti_waktu(durasi_detik: u32) -> f32 {
        if durasi_detik < 175 {
            (175 - durasi_detik) as f32 * 0.01
        } else if durasi_detik > 185 {
            (durasi_detik - 185) as f32 * 0.01
        } else {
            0.00
        }
    }

    /// Evaluasi Skor Juri Tunggal (Standar Dasar: 9.90)
    pub fn kalkulasi_skor_juri(
        juri_id: usize,
        salah_gerak_count: u32,
        kemantapan: f32,
        senjata_lepas: bool,
        senjata_keluar: bool,
        pakaian_salah: bool,
    ) -> NilaiJuriSeni {
        let mut skor = 9.90;
        skor -= salah_gerak_count as f32 * 0.01;
        skor += kemantapan.clamp(0.0, 0.10);

        if senjata_lepas { skor -= 0.50; }
        if senjata_keluar { skor -= 0.50; }
        if pakaian_salah { skor -= 0.05; }

        NilaiJuriSeni {
            juri_id,
            skor_kebenaran_gerak: (9.90 - (salah_gerak_count as f32 * 0.01)),
            skor_kemantapan_stamina: kemantapan,
            pengurangan_senjata_lepas: senjata_lepas,
            pengurangan_senjata_keluar: senjata_keluar,
            pengurangan_pakaian: pakaian_salah,
            total_skor_juri: skor.max(0.0),
        }
    }

    /// Hitung Skor Akhir Trimmed Mean dari 3 atau 5 Juri
    pub fn hitung_skor_akhir_seni(
        partai: &str,
        nomor: NomorSeni,
        pesilat: &str,
        kontingen: &str,
        durasi: u32,
        mut juri_scores: Vec<NilaiJuriSeni>,
    ) -> RekodPenampilanSeni {
        let pinalti_waktu = Self::hitung_pinalti_waktu(durasi);

        let skor_terpangkas = if juri_scores.len() >= 5 {
            // Urutkan nilai juri, buang skor tertinggi dan terendah
            juri_scores.sort_by(|a, b| a.total_skor_juri.partial_cmp(&b.total_skor_juri).unwrap());
            let middle_sum: f32 = juri_scores[1..4].iter().map(|j| j.total_skor_juri).sum();
            (middle_sum / 3.0) - pinalti_waktu
        } else if !juri_scores.is_empty() {
            let sum: f32 = juri_scores.iter().map(|j| j.total_skor_juri).sum();
            (sum / juri_scores.len() as f32) - pinalti_waktu
        } else {
            0.0
        };

        RekodPenampilanSeni {
            nomor_partai: partai.to_string(),
            nomor_seni: nomor,
            nama_pesilat: pesilat.to_string(),
            kontingen: kontingen.to_string(),
            durasi_aktual_detik: durasi,
            pinalti_waktu,
            daftar_nilai_juri: juri_scores,
            skor_akhir_terpangkas: (skor_terpangkas * 1000.0).round() / 1000.0,
        }
    }
}
