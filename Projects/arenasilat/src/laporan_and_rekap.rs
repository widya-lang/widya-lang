// Projects/arenasilat/src/laporan_and_rekap.rs
// ==============================================================================
// ARENA SILAT SAAS — LAPORAN REKAPITULASI, MEDALI & KEUANGAN (IPSI 2026)
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rekap Perolehan Medali Kontingen
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RekapMedaliKontingen {
    pub kontingen_id: String,
    pub nama_kontingen: String,
    pub emas: u32,
    pub perak: u32,
    pub perunggu: u32,
    pub total_poin_klasemen: u32, // Emas = 5 poin, Perak = 3 poin, Perunggu = 1 poin
}

impl RekapMedaliKontingen {
    pub fn hitung_poin(&mut self) {
        self.total_poin_klasemen = (self.emas * 5) + (self.perak * 3) + (self.perunggu * 1);
    }
}

/// Nominasi & Penentuan Pesilat Terbaik
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NominasiPesilatTerbaik {
    pub atlet_id: String,
    pub nama_atlet: String,
    pub kontingen: String,
    pub kategori: String,
    pub total_poin_teknik: u32,
    pub total_jatuhan: u32,
    pub total_tendangan: u32,
    pub jumlah_pelanggaran: u32,
    pub skor_indeks_kemenangan: f32,
}

/// Laporan Keuangan & Arus Kas Turnamen (Financial Settlement)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaporanKeuanganTurnamen {
    pub total_registrasi_kontingen_idr: u64,
    pub total_registrasi_atlet_idr: u64,
    pub total_pemasukan_kotor_idr: u64,
    pub honorarium_wasit_juri_idr: u64,
    pub biaya_tim_medis_idr: u64,
    pub sewa_gelanggang_sarana_idr: u64,
    pub platform_fee_saas_idr: u64,
    pub laba_bersih_penyelenggara_idr: i64,
}

/// Mesin Pembuat Laporan Resmi Kejuaraan
pub struct ReportGenerator;

impl ReportGenerator {
    /// Urutkan Klasemen Medali Juara Umum (Prioritas: Emas -> Perak -> Perunggu)
    pub fn urutkan_klasemen_medali(mut daftar: Vec<RekapMedaliKontingen>) -> Vec<RekapMedaliKontingen> {
        for m in &mut daftar {
            m.hitung_poin();
        }

        daftar.sort_by(|a, b| {
            b.emas.cmp(&a.emas)
                .then_with(|| b.perak.cmp(&a.perak))
                .then_with(|| b.perunggu.cmp(&a.perunggu))
                .then_with(|| b.total_poin_klasemen.cmp(&a.total_poin_klasemen))
        });

        daftar
    }

    /// Tentukan Pesilat Terbaik Berdasarkan Indeks Kemenangan & Sportivitas
    pub fn tentukan_pesilat_terbaik(mut kandidat: Vec<NominasiPesilatTerbaik>) -> Option<NominasiPesilatTerbaik> {
        for k in &mut kandidat {
            // Formula Indeks: (Jatuhan * 3) + (Tendangan * 2) + PoinTeknik - (Pelanggaran * 4)
            k.skor_indeks_kemenangan = (k.total_jatuhan as f32 * 3.0)
                + (k.total_tendangan as f32 * 2.0)
                + (k.total_poin_teknik as f32)
                - (k.jumlah_pelanggaran as f32 * 4.0);
        }

        kandidat.sort_by(|a, b| b.skor_indeks_kemenangan.partial_cmp(&a.skor_indeks_kemenangan).unwrap());
        kandidat.into_iter().next()
    }

    /// Hitung Neraca Keuangan Turnamen
    pub fn hitung_neraca_keuangan(
        reg_kontingen: u64,
        reg_atlet: u64,
        wasit_juri: u64,
        medis: u64,
        sewa: u64,
    ) -> LaporanKeuanganTurnamen {
        let total_kotor = reg_kontingen + reg_atlet;
        let platform_fee = (total_kotor as f64 * 0.05) as u64; // 5% SaaS Fee
        let total_beban = wasit_juri + medis + sewa + platform_fee;
        let laba_bersih = total_kotor as i64 - total_beban as i64;

        LaporanKeuanganTurnamen {
            total_registrasi_kontingen_idr: reg_kontingen,
            total_registrasi_atlet_idr: reg_atlet,
            total_pemasukan_kotor_idr: total_kotor,
            honorarium_wasit_juri_idr: wasit_juri,
            biaya_tim_medis_idr: medis,
            sewa_gelanggang_sarana_idr: sewa,
            platform_fee_saas_idr: platform_fee,
            laba_bersih_penyelenggara_idr: laba_bersih,
        }
    }
}
