// Projects/arenasilat/src/dewan_hakim_td.rs
// ==============================================================================
// ARENA SILAT SAAS — DEWAN HAKIM, TECHNICAL DELEGATE & BLACK BOX AUDIT (IPSI 2026)
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Status Pengesahan Bagan Drawing oleh TD
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusOtorisasiBagan {
    DraftBelumDisahkan,
    TelahDisahkanOlehTD,
    DitolakPerluDrawingUlang,
}

/// Permohonan Sengketa & Sidang Banding Dewan Hakim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermohonanProtesBanding {
    pub protes_id: String,
    pub partai_id: String,
    pub kontingen_pemohon: String,
    pub nama_ofisial: String,
    pub babak_kejadian: u8,
    pub waktu_laga: String,
    pub alasan_protes: String,
    pub uang_jaminan_idr: u64, // Rp 3.000.000,- sesuai aturan IPSI 2026
    pub uang_jaminan_diterima: bool,
    pub status_putusan: StatusPutusanBanding,
    pub catatan_dewan_hakim: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusPutusanBanding {
    DalamPemeriksaan,
    DiterimaPoinDirevisi,
    DitolakUangJaminanHangus,
}

/// Black Box Audit Trail Entry untuk Setiap Input Tombol Juri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackBoxAuditEntry {
    pub log_id: u64,
    pub partai_id: String,
    pub juri_id: usize,
    pub sudut: String,
    pub teknik: String,
    pub timestamp_ms: u64,
    pub status_konsensus: bool,
}

/// Mesin Dewan Hakim & Technical Delegate (TD)
pub struct DewanHakimEngine {
    pub nama_td: String,
    pub lisensi_ipsi_td: String,
    pub status_bagan: StatusOtorisasiBagan,
    pub daftar_protes: Vec<PermohonanProtesBanding>,
    pub black_box_logs: Vec<BlackBoxAuditEntry>,
}

impl DewanHakimEngine {
    pub fn baru(nama_td: &str, lisensi: &str) -> Self {
        Self {
            nama_td: nama_td.to_string(),
            lisensi_ipsi_td: lisensi.to_string(),
            status_bagan: StatusOtorisasiBagan::DraftBelumDisahkan,
            daftar_protes: Vec::new(),
            black_box_logs: Vec::new(),
        }
    }

    /// Sahkan Bagan Pertandingan oleh Technical Delegate
    pub fn sahkan_bagan_turnamen(&mut self) -> bool {
        self.status_bagan = StatusOtorisasiBagan::TelahDisahkanOlehTD;
        true
    }

    /// Catat Raw Input Juri ke dalam Black Box Audit Trail
    pub fn rekam_black_box(&mut self, partai: &str, juri: u8, sudut: &str, teknik: &str, ts: u64, konsensus: bool) {
        let entry = BlackBoxAuditEntry {
            log_id: (self.black_box_logs.len() + 1) as u64,
            partai_id: partai.to_string(),
            juri_id: juri as usize,
            sudut: sudut.to_string(),
            teknik: teknik.to_string(),
            timestamp_ms: ts,
            status_konsensus: konsensus,
        };
        self.black_box_logs.push(entry);
    }

    /// Ajukan Protes Resmi Kontingen ke Dewan Hakim
    pub fn ajukan_protes(
        &mut self,
        partai: &str,
        kontingen: &str,
        ofisial: &str,
        babak: u8,
        waktu: &str,
        alasan: &str,
        uang_jaminan: u64,
    ) -> Result<String, &'static str> {
        if uang_jaminan < 3_000_000 {
            return Err("Uang jaminan protes minimal Rp 3.000.000,- sesuai regulasi IPSI!");
        }

        let protes_id = format!("PROTES-{}-{}", partai, self.daftar_protes.len() + 1);
        let permohonan = PermohonanProtesBanding {
            protes_id: protes_id.clone(),
            partai_id: partai.to_string(),
            kontingen_pemohon: kontingen.to_string(),
            nama_ofisial: ofisial.to_string(),
            babak_kejadian: babak,
            waktu_laga: waktu.to_string(),
            alasan_protes: alasan.to_string(),
            uang_jaminan_idr: uang_jaminan,
            uang_jaminan_diterima: true,
            status_putusan: StatusPutusanBanding::DalamPemeriksaan,
            catatan_dewan_hakim: "Sedang meninjau rekaman Black Box & Video VAR".to_string(),
        };

        self.daftar_protes.push(permohonan);
        Ok(protes_id)
    }

    /// Putuskan Hasil Sidang Banding Dewan Hakim
    pub fn putuskan_protes(&mut self, protes_id: &str, diterima: bool, catatan: &str) -> bool {
        if let Some(p) = self.daftar_protes.iter_mut().find(|p| p.protes_id == protes_id) {
            if diterima {
                p.status_putusan = StatusPutusanBanding::DiterimaPoinDirevisi;
            } else {
                p.status_putusan = StatusPutusanBanding::DitolakUangJaminanHangus;
            }
            p.catatan_dewan_hakim = catatan.to_string();
            return true;
        }
        false
    }
}
