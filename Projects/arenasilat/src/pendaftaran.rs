// Projects/arenasilat/src/pendaftaran.rs
// ==============================================================================
// ARENA SILAT SAAS — MODUL PENDAFTARAN & MANAJEMEN KONTINGEN (IPSI 2026)
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Kategori Usia Resmi IPSI/PERSILAT
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KategoriUsia {
    UsiaDini1,    // 7 - 9 Tahun
    UsiaDini2,    // 10 - 11 Tahun
    PraRemaja,    // 12 - 14 Tahun (SMP)
    Remaja,       // 14 - 17 Tahun (SMA)
    Dewasa,       // 17 - 35 Tahun (Mahasiswa/Umum)
    Master,       // Di atas 35 Tahun
}

impl KategoriUsia {
    pub fn label(&self) -> &'static str {
        match self {
            KategoriUsia::UsiaDini1 => "Usia Dini 1 (7-9 Thn)",
            KategoriUsia::UsiaDini2 => "Usia Dini 2 (10-11 Thn)",
            KategoriUsia::PraRemaja => "Pra-Remaja (12-14 Thn)",
            KategoriUsia::Remaja => "Remaja (14-17 Thn)",
            KategoriUsia::Dewasa => "Dewasa (17-35 Thn)",
            KategoriUsia::Master => "Master (>35 Thn)",
        }
    }
}

/// Jenis Kompetisi Silat
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JenisKompetisi {
    Tanding(String),    // Misal: "Kelas A", "Kelas B", "Kelas Bebas"
    SeniTunggal,        // Jurus Tunggal Baku
    SeniGanda,          // Ganda Tangan Kosong & Senjata
    SeniRegu,           // Regu 3 Pesilat
    SoloKreatif,        // Solo Kreatif Musik Bebas
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JenisKelamin {
    Putra,
    Putri,
}

/// Data Kontingen / Tim Perguruan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kontingen {
    pub id: String,
    pub nama: String,
    pub asal_daerah: String,
    pub nama_official: String,
    pub no_telepon: String,
    pub email: String,
    pub status_pembayaran: bool,
    pub total_tagihan_idr: u64,
}

/// Data Pendaftaran Atlet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtletRegistrasi {
    pub id: String,
    pub kontingen_id: String,
    pub nama_lengkap: String,
    pub jenis_kelamin: JenisKelamin,
    pub tanggal_lahir: String,
    pub umur_tahun: u8,
    pub berat_badan_kg: f32,
    pub tinggi_badan_cm: f32,
    pub kategori_usia: KategoriUsia,
    pub kompetisi: JenisKompetisi,
    pub dokumen_ktp_kia: bool,
    pub dokumen_akte: bool,
    pub dokumen_surat_sehat: bool,
    pub terverifikasi: bool,
    pub seed_unggulan: Option<u32>,
}

/// Status Rekapitulasi Pendaftaran Event
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RekapPendaftaran {
    pub total_kontingen: usize,
    pub total_atlet: usize,
    pub total_tanding: usize,
    pub total_seni: usize,
    pub terverifikasi: usize,
    pub total_pendapatan_idr: u64,
}

/// Mesin Pendaftaran & Verifikasi Kontingen
pub struct RegistrationEngine {
    pub event_id: String,
    pub event_nama: String,
    pub biaya_tanding_idr: u64,
    pub biaya_seni_tunggal_idr: u64,
    pub biaya_kontingen_idr: u64,
    pub daftar_kontingen: HashMap<String, Kontingen>,
    pub daftar_atlet: HashMap<String, AtletRegistrasi>,
}

impl RegistrationEngine {
    pub fn baru(event_id: &str, event_nama: &str) -> Self {
        Self {
            event_id: event_id.to_string(),
            event_nama: event_nama.to_string(),
            biaya_tanding_idr: 250_000,
            biaya_seni_tunggal_idr: 250_000,
            biaya_kontingen_idr: 100_000,
            daftar_kontingen: HashMap::new(),
            daftar_atlet: HashMap::new(),
        }
    }

    /// Daftarkan Kontingen Baru
    pub fn daftarkan_kontingen(&mut self, id: &str, nama: &str, daerah: &str, official: &str, no_telp: &str, email: &str) {
        let k = Kontingen {
            id: id.to_string(),
            nama: nama.to_string(),
            asal_daerah: daerah.to_string(),
            nama_official: official.to_string(),
            no_telepon: no_telp.to_string(),
            email: email.to_string(),
            status_pembayaran: false,
            total_tagihan_idr: self.biaya_kontingen_idr,
        };
        self.daftar_kontingen.insert(id.to_string(), k);
    }

    /// Daftarkan Atlet ke Kontingen
    pub fn daftarkan_atlet(
        &mut self,
        atlet_id: &str,
        kontingen_id: &str,
        nama: &str,
        jk: JenisKelamin,
        tgl_lahir: &str,
        umur: u8,
        bb: f32,
        tb: f32,
        usia: KategoriUsia,
        kompetisi: JenisKompetisi,
    ) -> Result<(), &'static str> {
        if !self.daftar_kontingen.contains_key(kontingen_id) {
            return Err("Kontingen belum terdaftar!");
        }

        let biaya_item = match &kompetisi {
            JenisKompetisi::Tanding(_) => self.biaya_tanding_idr,
            JenisKompetisi::SeniTunggal | JenisKompetisi::SoloKreatif => self.biaya_seni_tunggal_idr,
            JenisKompetisi::SeniGanda => self.biaya_seni_tunggal_idr * 2,
            JenisKompetisi::SeniRegu => self.biaya_seni_tunggal_idr * 3,
        };

        if let Some(kontingen) = self.daftar_kontingen.get_mut(kontingen_id) {
            kontingen.total_tagihan_idr += biaya_item;
        }

        let atlet = AtletRegistrasi {
            id: atlet_id.to_string(),
            kontingen_id: kontingen_id.to_string(),
            nama_lengkap: nama.to_string(),
            jenis_kelamin: jk,
            tanggal_lahir: tgl_lahir.to_string(),
            umur_tahun: umur,
            berat_badan_kg: bb,
            tinggi_badan_cm: tb,
            kategori_usia: usia,
            kompetisi,
            dokumen_ktp_kia: true,
            dokumen_akte: true,
            dokumen_surat_sehat: true,
            terverifikasi: false,
            seed_unggulan: None,
        };

        self.daftar_atlet.insert(atlet_id.to_string(), atlet);
        Ok(())
    }

    /// Verifikasi Berkas Administratif Atlet
    pub fn verifikasi_atlet(&mut self, atlet_id: &str) -> bool {
        if let Some(a) = self.daftar_atlet.get_mut(atlet_id) {
            if a.dokumen_ktp_kia && a.dokumen_akte && a.dokumen_surat_sehat {
                a.terverifikasi = true;
                return true;
            }
        }
        false
    }

    /// Konfirmasi Pembayaran Kontingen
    pub fn konfirmasi_pembayaran_kontingen(&mut self, kontingen_id: &str) -> bool {
        if let Some(k) = self.daftar_kontingen.get_mut(kontingen_id) {
            k.status_pembayaran = true;
            return true;
        }
        false
    }

    /// Rekapitulasi Statistik Pendaftaran
    pub fn hitung_rekapitulasi(&self) -> RekapPendaftaran {
        let mut rekap = RekapPendaftaran {
            total_kontingen: self.daftar_kontingen.len(),
            total_atlet: self.daftar_atlet.len(),
            ..Default::default()
        };

        for (_, a) in &self.daftar_atlet {
            if a.terverifikasi {
                rekap.terverifikasi += 1;
            }
            match a.kompetisi {
                JenisKompetisi::Tanding(_) => rekap.total_tanding += 1,
                _ => rekap.total_seni += 1,
            }
        }

        for (_, k) in &self.daftar_kontingen {
            if k.status_pembayaran {
                rekap.total_pendapatan_idr += k.total_tagihan_idr;
            }
        }

        rekap
    }
}
