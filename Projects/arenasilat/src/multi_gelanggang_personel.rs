// Projects/arenasilat/src/multi_gelanggang_personel.rs
// ==============================================================================
// ARENA SILAT SAAS — MANAJEMEN MULTI-GELANGGANG & ROSTER PENUGASAN PERSONEL
// 100% In-House Sovereign Code on Widya-Lang Ecosystem
// Mendukung N-Gelanggang Paralel (Gelanggang A, B, C, D, E, F, G, H...)
// ==============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Peran Personel Teknis Lapangan
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PeranPersonel {
    KetuaPertandingan,
    DewanWasitPengawas,
    WasitMatras,
    Juri1,
    Juri2,
    Juri3,
    Juri4,
    Juri5,
    OperatorTimerIT,
    OperatorVAR,
    DokterMedisGelanggang,
    PetugasMarshalling,
}

/// Data Profil Personel / Wasit Juri Berlisensi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonelTeknis {
    pub personel_id: String,
    pub nama_lengkap: String,
    pub nomor_lisensi_ipsi: String,
    pub tingkat_lisensi: String, // "Wasit Juri Nasional 1", "Nasional 2", "Daerah/Pengprov", "Cabang"
    pub asal_pengprov: String,
    pub nomor_telepon: String,
    pub status_aktif: bool,
}

/// Struktur Roster Penugasan Personel pada 1 Gelanggang
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RosterGelanggang {
    pub ketua_pertandingan: Option<PersonelTeknis>,
    pub dewan_wasit: Option<PersonelTeknis>,
    pub wasit_matras: Option<PersonelTeknis>,
    pub juri_list: Vec<PersonelTeknis>, // 3 atau 5 Juri
    pub operator_timer: Option<PersonelTeknis>,
    pub operator_var: Option<PersonelTeknis>,
    pub tim_medis: Option<PersonelTeknis>,
    pub marshalling: Option<PersonelTeknis>,
    pub shift_nama: String, // "Shift 1 (Pagi 08:00 - 12:00)", "Shift 2 (Siang 13:00 - 17:00)", "Shift 3 (Malam)"
}

/// Status Operasional Gelanggang
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusGelanggang {
    Siap,
    SedangBertanding,
    IstirahatShift,
    PembersihanMatras,
    Selesai,
}

/// Struktur 1 Unit Gelanggang Pertandingan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GelanggangTurnamen {
    pub gelanggang_id: u8,       // 1, 2, 3, 4, 5, 6, 7, 8...
    pub kode_gelanggang: String, // "Gelanggang A", "Gelanggang B", "Gelanggang C", "Gelanggang D"...
    pub lokasi_matras: String,   // "Matras Barat - Sisi Utama", "Matras Timur"...
    pub status: StatusGelanggang,
    pub partai_aktif_nomor: Option<u32>,
    pub partai_aktif_kategori: Option<String>,
    pub daftar_antrean_partai: Vec<u32>, // Daftar nomor partai yang dijadwalkan
    pub roster_tugas: RosterGelanggang,
    pub total_partai_selesai: u32,
    pub url_stream_obs: String, // "http://127.0.0.1:8088/overlay/court-A"
}

/// Mesin Pengelola Multi-Gelanggang & Rotasi Personel Wasit Juri
pub struct MultiCourtManager {
    pub tenant_id: String,
    pub nama_event: String,
    pub daftar_gelanggang: HashMap<u8, GelanggangTurnamen>,
    pub database_personel: HashMap<String, PersonelTeknis>,
}

impl MultiCourtManager {
    pub fn baru(tenant_id: &str, nama_event: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_string(),
            nama_event: nama_event.to_string(),
            daftar_gelanggang: HashMap::new(),
            database_personel: HashMap::new(),
        }
    }

    /// Daftarkan Personel Baru ke Bank Data Wasit Juri Turnamen
    pub fn daftarkan_personel(
        &mut self,
        id: &str,
        nama: &str,
        lisensi: &str,
        tingkat: &str,
        provinsi: &str,
        telp: &str,
    ) {
        let p = PersonelTeknis {
            personel_id: id.to_string(),
            nama_lengkap: nama.to_string(),
            nomor_lisensi_ipsi: lisensi.to_string(),
            tingkat_lisensi: tingkat.to_string(),
            asal_pengprov: provinsi.to_string(),
            nomor_telepon: telp.to_string(),
            status_aktif: true,
        };
        self.database_personel.insert(id.to_string(), p);
    }

    /// Tambahkan Gelanggang Baru (Mendukung N-Gelanggang: A, B, C, D, E, F, G, H...)
    pub fn tambah_gelanggang(&mut self, gelanggang_id: u8, kode: &str, lokasi: &str) {
        let g = GelanggangTurnamen {
            gelanggang_id,
            kode_gelanggang: kode.to_string(),
            lokasi_matras: lokasi.to_string(),
            status: StatusGelanggang::Siap,
            partai_aktif_nomor: None,
            partai_aktif_kategori: None,
            daftar_antrean_partai: Vec::new(),
            roster_tugas: RosterGelanggang::default(),
            total_partai_selesai: 0,
            url_stream_obs: format!("http://127.0.0.1:8088/overlay/court-{}", kode.replace("Gelanggang ", "").trim()),
        };
        self.daftar_gelanggang.insert(gelanggang_id, g);
    }

    /// Pasang Roster Personel ke Gelanggang Tertentu
    pub fn tetapkan_roster_gelanggang(
        &mut self,
        gelanggang_id: u8,
        kp_id: &str,
        wasit_id: &str,
        juri_ids: &[&str],
        timer_id: &str,
        medis_id: &str,
        shift: &str,
    ) -> Result<(), &'static str> {
        let gelanggang = self.daftar_gelanggang.get_mut(&gelanggang_id).ok_or("Gelanggang tidak ditemukan!")?;

        let kp = self.database_personel.get(kp_id).cloned();
        let wasit = self.database_personel.get(wasit_id).cloned();
        let timer = self.database_personel.get(timer_id).cloned();
        let medis = self.database_personel.get(medis_id).cloned();

        let mut juries = Vec::new();
        for &jid in juri_ids {
            if let Some(j) = self.database_personel.get(jid) {
                juries.push(j.clone());
            }
        }

        gelanggang.roster_tugas = RosterGelanggang {
            ketua_pertandingan: kp,
            dewan_wasit: None,
            wasit_matras: wasit,
            juri_list: juries,
            operator_timer: timer,
            operator_var: None,
            tim_medis: medis,
            marshalling: None,
            shift_nama: shift.to_string(),
        };

        Ok(())
    }

    /// Alokasikan Partai ke Gelanggang Tertentu
    pub fn alokasikan_partai(&mut self, gelanggang_id: u8, nomor_partai: u32) -> Result<(), &'static str> {
        let g = self.daftar_gelanggang.get_mut(&gelanggang_id).ok_or("Gelanggang tidak ditemukan!")?;
        g.daftar_antrean_partai.push(nomor_partai);
        Ok(())
    }

    /// Dynamic Court Load Balancing: Pindahkan Partai dari Gelanggang Padat ke Gelanggang yang Lebih Cepat Selesai
    pub fn pindahkan_partai_penyeimbang(&mut self, dari_gelanggang: u8, ke_gelanggang: u8) -> Result<u32, &'static str> {
        if dari_gelanggang == ke_gelanggang {
            return Err("Gelanggang asal dan tujuan tidak boleh sama!");
        }

        let partai_pindah = {
            let g_asal = self.daftar_gelanggang.get_mut(&dari_gelanggang).ok_or("Gelanggang asal tidak ditemukan!")?;
            if g_asal.daftar_antrean_partai.is_empty() {
                return Err("Tidak ada partai yang bisa dipindahkan dari gelanggang asal!");
            }
            g_asal.daftar_antrean_partai.pop().unwrap()
        };

        let g_tujuan = self.daftar_gelanggang.get_mut(&ke_gelanggang).ok_or("Gelanggang tujuan tidak ditemukan!")?;
        g_tujuan.daftar_antrean_partai.push(partai_pindah);

        Ok(partai_pindah)
    }

    /// Dapatkan Status Ringkasan Semua Gelanggang
    pub fn ringkasan_semua_gelanggang(&self) -> Vec<GelanggangTurnamen> {
        let mut list: Vec<GelanggangTurnamen> = self.daftar_gelanggang.values().cloned().collect();
        list.sort_by_key(|g| g.gelanggang_id);
        list
    }
}
