// Projects/arenasilat/src/perguruan_and_notifikasi.rs
// ==============================================================================
// ARENA SILAT SAAS — MANAJEMEN PERGURUAN SILAT, UKT SABUK & NOTIFIKASI WHATSAPP
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};

/// Tingkatan Sabuk Pesilat dalam Perguruan
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TingkatSabuk {
    SiswaPolos,
    SiswaJambon,
    SiswaHijau,
    SiswaPutih,
    WargaTingkat1,
    PendekarWargaTingkat2,
    DewanPendekarTingkat3,
}

/// Struktur Data Perguruan Silat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerguruanSilat {
    pub perguruan_id: String,
    pub nama_perguruan: String, // "Tapak Suci", "PSHT", "Perisai Diri", "Merpati Putih", dll
    pub ketua_umum: String,
    pub sekretariat_dpp: String,
    pub total_cabang: usize,
    pub total_ranting: usize,
    pub total_anggota_terdaftar: usize,
}

/// Pesan Notifikasi WhatsApp Otomatis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifikasiWhatsApp {
    pub target_telepon: String,
    pub pesan_teks: String,
    pub tipe_notifikasi: String, // "H-3_CALL_ROOM", "TIMBANGAN_BERHASIL", "SERTIFIKAT_TERBIT"
    pub terkirim: bool,
}

/// Mesin Notifikasi & Tata Kelola Perguruan
pub struct GuildAndNotificationEngine;

impl GuildAndNotificationEngine {
    /// Buat Pesan Pengingat H-3 Pertandingan ke Pelatih
    pub fn buat_notifikasi_h3_laga(
        no_telp: &str,
        nama_atlet: &str,
        partai: u32,
        gelanggang: u8,
        estimasi_waktu: &str,
    ) -> NotifikasiWhatsApp {
        let pesan = format!(
            "🔔 *ARENA SILAT CALL ROOM REMINDER*\n\nYth. Pelatih/Manager,\nPesilat *{}* dijadwalkan bertanding pada:\n- Partai: #{} (Gelanggang {})\n- Estimasi Waktu: {}\n\nMohon segera hadir ke *Ruang Marshalling (H-3)* untuk verifikasi perlengkapan dan uji timbang badan.\n\n_Sistem Otomasi Arena Silat SaaS_",
            nama_atlet, partai, gelanggang, estimasi_waktu
        );

        NotifikasiWhatsApp {
            target_telepon: no_telp.to_string(),
            pesan_teks: pesan,
            tipe_notifikasi: "H-3_CALL_ROOM".to_string(),
            terkirim: true,
        }
    }
}
