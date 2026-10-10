// Projects/arenasilat/src/bracket_generator.rs
// ==============================================================================
// ARENA SILAT SAAS — TOURNAMENT BRACKET & FAIR COURT SCHEDULER
// 100% Sovereign In-House Code on Widya-Lang Ecosystem
// ==============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Data Atlet / Peserta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtletPeserta {
    pub id: String,
    pub nama: String,
    pub kontingen: String,   // Nama Kontingen / Perguruan Silat
    pub kelas_tanding: String, // Misal: "Kelas A Putra (45-50 kg)"
    pub berat_badan_kg: f32,
    pub unggulan_seed: Option<u32>, // Seed unggulan (1, 2, 3, 4)
}

/// Node Partai dalam Bagan Turnamen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartaiBagan {
    pub nomor_partai: u32,
    pub babak: String,          // "Penyisihan", "Perempat Final", "Semi Final", "Final"
    pub atlet_merah: Option<AtletPeserta>,
    pub atlet_biru: Option<AtletPeserta>,
    pub pemenang_id: Option<String>,
    pub gelanggang_id: Option<u8>, // Gelanggang 1, 2, 3, 4
    pub estimasi_waktu: String,
    pub is_bye: bool,
}

/// Struktur Bagan Pohon Turnamen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaganTurnamen {
    pub kelas_tanding: String,
    pub kapasitas_bracket: usize, // 4, 8, 16, 32, 64
    pub jumlah_peserta: usize,
    pub jumlah_bye: usize,
    pub daftar_partai: Vec<PartaiBagan>,
}

/// Generator Bagan & Penjadwalan Adil (Fair Court Scheduler)
pub struct BracketGenerator;

impl BracketGenerator {
    /// Hitung kapasitas bracket terkecil berbasis eksponen 2 ($2^n$)
    pub fn hitung_kapasitas_bracket(jumlah_peserta: usize) -> usize {
        let mut kap = 2;
        while kap < jumlah_peserta {
            kap *= 2;
        }
        kap.max(2)
    }

    /// Susun bagan eliminasi gugur tunggal dengan pemisahan kontingen yang sama
    pub fn buat_bagan_eliminasi(kelas: &str, mut peserta: Vec<AtletPeserta>) -> BaganTurnamen {
        let total = peserta.len();
        let kapasitas = Self::hitung_kapasitas_bracket(total);
        let jumlah_bye = kapasitas - total;

        // Urutkan dan pisahkan kontingen se-perguruan (Contingent Separation)
        // Kelompokkan peserta berdasarkan kontingen
        let mut peta_kontingen: HashMap<String, Vec<AtletPeserta>> = HashMap::new();
        for p in peserta.drain(..) {
            peta_kontingen.entry(p.kontingen.clone()).or_default().push(p);
        }

        // Distribusikan bergantian ke Pool Atas (Ganjil) dan Pool Bawah (Genap)
        let mut slot_bracket: Vec<Option<AtletPeserta>> = vec![None; kapasitas];
        let mut slot_atas = 0;
        let mut slot_bawah = kapasitas / 2;

        for (_kontingen, anggota) in peta_kontingen.iter() {
            for (idx, atlet) in anggota.iter().enumerate() {
                if idx % 2 == 0 && slot_atas < kapasitas / 2 {
                    slot_bracket[slot_atas] = Some(atlet.clone());
                    slot_atas += 1;
                } else if slot_bawah < kapasitas {
                    slot_bracket[slot_bawah] = Some(atlet.clone());
                    slot_bawah += 1;
                } else if slot_atas < kapasitas / 2 {
                    slot_bracket[slot_atas] = Some(atlet.clone());
                    slot_atas += 1;
                }
            }
        }

        // Bangun partai babak penyisihan / ronde 1
        let mut daftar_partai = Vec::new();
        let jumlah_partai_r1 = kapasitas / 2;
        let nama_babak = match kapasitas {
            2 => "Final",
            4 => "Semi Final",
            8 => "Perempat Final",
            16 => "Babak 16 Besar",
            32 => "Babak 32 Besar",
            _ => "Babak Penyisihan",
        };

        for i in 0..jumlah_partai_r1 {
            let merah = slot_bracket[i * 2].clone();
            let biru = slot_bracket[i * 2 + 1].clone();
            let is_bye = merah.is_none() || biru.is_none();

            // Jika salah satu BYE, tentukan pemenang langsung
            let pemenang_id = if is_bye {
                if let Some(ref m) = merah {
                    Some(m.id.clone())
                } else if let Some(ref b) = biru {
                    Some(b.id.clone())
                } else {
                    None
                }
            } else {
                None
            };

            daftar_partai.push(PartaiBagan {
                nomor_partai: (i + 1) as u32,
                babak: nama_babak.to_string(),
                atlet_merah: merah,
                atlet_biru: biru,
                pemenang_id,
                gelanggang_id: Some(((i % 3) + 1) as u8), // Distribusi seimbang Gelanggang 1, 2, 3
                estimasi_waktu: format!("09:{:02} WIB", 15 * i),
                is_bye,
            });
        }

        BaganTurnamen {
            kelas_tanding: kelas.to_string(),
            kapasitas_bracket: kapasitas,
            jumlah_peserta: total,
            jumlah_bye,
            daftar_partai,
        }
    }

    /// Penjadwalan Multi-Gelanggang dengan estimasi waktu dinamis (Fair Court Scheduler)
    pub fn jadwalkan_multi_gelanggang(
        partai_list: &mut [PartaiBagan],
        jumlah_gelanggang: u8,
        waktu_mulai_menit_dari_00: u32, // Misal 08:30 = 8 * 60 + 30 = 510
        durasi_partai_menit: u32,       // Standar 12 menit per partai
    ) {
        let mut waktu_gelanggang: Vec<u32> = vec![waktu_mulai_menit_dari_00; jumlah_gelanggang as usize];

        for partai in partai_list.iter_mut() {
            if partai.is_bye {
                continue;
            }

            // Cari gelanggang dengan antrian paling awal (Load Balancing)
            let mut min_court = 0;
            let mut min_waktu = waktu_gelanggang[0];
            for (c_idx, &waktu) in waktu_gelanggang.iter().enumerate() {
                if waktu < min_waktu {
                    min_waktu = waktu;
                    min_court = c_idx;
                }
            }

            let jam = min_waktu / 60;
            let menit = min_waktu % 60;

            partai.gelanggang_id = Some((min_court + 1) as u8);
            partai.estimasi_waktu = format!("{:02}:{:02} WIB", jam, menit);

            // Tambahkan durasi partai ke gelanggang tersebut
            waktu_gelanggang[min_court] += durasi_partai_menit;
        }
    }
}
