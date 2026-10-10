// Projects/arenasilat/src/var_and_hardware.rs
// ==============================================================================
// ARENA SILAT SAAS — VAR VIDEO REVIEW & HARDWARE SCALE INTEGRATION
// 100% Sovereign In-House Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::scoring_engine::SudutPesilat;

/// Tipe Keputusan Dewan Hakim atas Permintaan VAR
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeputusanVAR {
    DiterimaPoinMasuk,      // Protes Diterima -> Tambah Poin & Kartu VAR Dikembalikan
    DitolakTidakAdaPoin,   // Protes Ditolak -> Kartu VAR Hangus
    DiterimaPelanggaran,    // Protes Diterima -> Lawan Diberi Hukuman
    BatalTanpaAksi,        // Rekaman Video Tidak Konklusif / Blur
}

/// Sesi Tinjauan Video VAR (Video Assistant Referee)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SesiTinjauanVAR {
    pub sesi_id: String,
    pub partai_id: String,
    pub sudut_pengaju: SudutPesilat,
    pub babak: usize,
    pub detik_kejadian: u32,
    pub alasan_protes: String, // Misal: "Tendangan Masuk Sah", "Pukulan Sebelum Gong", "Pelanggaran Tarikan"
    pub keputusan: Option<KeputusanVAR>,
    pub catatan_dewan_hakim: Option<String>,
}

/// Modul Pengendali VAR & Live Review
pub struct VarEngine {
    pub daftar_sesi: Vec<SesiTinjauanVAR>,
}

impl VarEngine {
    pub fn baru() -> Self {
        Self {
            daftar_sesi: Vec::new(),
        }
    }

    /// Ajukan sesi tinjauan VAR baru oleh Ofisial/Pelatih Sudut
    pub fn buat_sesi_review(
        &mut self,
        partai_id: &str,
        sudut: SudutPesilat,
        babak: usize,
        detik: u32,
        alasan: &str,
    ) -> SesiTinjauanVAR {
        let sesi = SesiTinjauanVAR {
            sesi_id: format!("VAR-{}-B{}-{}", partai_id, babak, detik),
            partai_id: partai_id.to_string(),
            sudut_pengaju: sudut,
            babak,
            detik_kejadian: detik,
            alasan_protes: alasan.to_string(),
            keputusan: None,
            catatan_dewan_hakim: None,
        };
        self.daftar_sesi.push(sesi.clone());
        sesi
    }

    /// Dewan Hakim menetapkan keputusan hasil review video
    pub fn tetapkan_keputusan(&mut self, sesi_id: &str, keputusan: KeputusanVAR, catatan: &str) -> Result<SesiTinjauanVAR, String> {
        let sesi = self.daftar_sesi.iter_mut().find(|s| s.sesi_id == sesi_id)
            .ok_or_else(|| "Sesi VAR tidak ditemukan".to_string())?;

        sesi.keputusan = Some(keputusan);
        sesi.catatan_dewan_hakim = Some(catatan.to_string());
        Ok(sesi.clone())
    }
}

/// Hasil Verifikasi Penimbangan Badan Atlet (Weigh-in)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasilTimbangan {
    pub atlet_id: String,
    pub nama_atlet: String,
    pub kelas_tanding: String,
    pub berat_min_kg: f32,
    pub berat_max_kg: f32,
    pub berat_terukur_kg: f32,
    pub status_lolos: bool,
    pub catatan: String,
}

/// Integrator Protokol Timbangan Digital (Bluetooth BLE / ASCII Serial)
pub struct ScaleBridge;

impl ScaleBridge {
    /// Validasi Berat Badan Atlet terhadap Rentang Kelas Tanding IPSI
    pub fn validasi_berat(
        atlet_id: &str,
        nama: &str,
        kelas: &str,
        berat_min: f32,
        berat_max: f32,
        berat_terukur: f32,
    ) -> HasilTimbangan {
        let toleransi_kg = 0.0; // Aturan resmi IPSI: tidak ada toleransi berat badan saat penimbangan resmi H-1
        let lolos = berat_terukur >= (berat_min - toleransi_kg) && berat_terukur <= (berat_max + toleransi_kg);

        let catatan = if lolos {
            "SAH — Memenuhi syarat berat badan kelas tanding".to_string()
        } else if berat_terukur > berat_max {
            format!("DISKUALIFIKASI — Kelebihan berat badan (+{:.2} kg)", berat_terukur - berat_max)
        } else {
            format!("DISKUALIFIKASI — Kekurangan berat badan (-{:.2} kg)", berat_min - berat_terukur)
        };

        HasilTimbangan {
            atlet_id: atlet_id.to_string(),
            nama_atlet: nama.to_string(),
            kelas_tanding: kelas.to_string(),
            berat_min_kg: berat_min,
            berat_max_kg: berat_max,
            berat_terukur_kg: berat_terukur,
            status_lolos: lolos,
            catatan,
        }
    }
}

/// Generator & Verifikator E-Sertifikat Anti-Fraud (BSrE QR Code Dynamic Link)
pub struct CertificateEngine;

impl CertificateEngine {
    fn hitung_hash(payload: &str) -> String {
        let mut hasher = widya::kripto::PureSha256::new();
        hasher.update(payload.as_bytes());
        let digest = hasher.finalize();
        widya::kripto::PureHex::encode(&digest)
    }

    /// Buat URL Verifikasi Sertifikat Resmi dengan Hash Anti-Fraud
    pub fn buat_url_verifikasi(nomor_sertifikat: &str, atlet_id: &str, event_id: &str) -> String {
        let payload = format!("{}:{}:{}", nomor_sertifikat, atlet_id, event_id);
        let hash = Self::hitung_hash(&payload);
        format!("https://arenasilat.id/verifikasi/{}/{}", nomor_sertifikat, &hash[..12])
    }

    /// Verifikasi Keaslian Sertifikat
    pub fn verifikasi_keaslian(nomor_sertifikat: &str, atlet_id: &str, event_id: &str, signature_hash: &str) -> bool {
        let payload = format!("{}:{}:{}", nomor_sertifikat, atlet_id, event_id);
        let expected_hash = Self::hitung_hash(&payload);
        &expected_hash[..12] == signature_hash
    }
}
