// Projects/arenasilat/src/scoring_engine.rs
// ==============================================================================
// ARENA SILAT SAAS — SOVEREIGN SCORING & CONSENSUS ENGINE (IPSI/PERSILAT 2026)
// 100% Sovereign In-House Code on Widya-Lang Ecosystem
// ==============================================================================

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Sudut Pesilat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SudutPesilat {
    Merah,
    Biru,
}

/// Tipe Aksi Serangan / Poin Berdasarkan Regulasi IPSI/PERSILAT 2026
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TipePoin {
    Pukulan,      // +1 Poin (Pukulan Masuk Sah)
    Tendangan,    // +2 Poin (Tendangan Masuk Sah)
    Jatuhan,      // +3 Poin (Bantingan / Jatuhan Sah)
    Kuncian,      // +4 Poin (Kuncian Sah Selesai / Tahan)
}

impl TipePoin {
    pub fn nilai(&self) -> i32 {
        match self {
            TipePoin::Pukulan => 1,
            TipePoin::Tendangan => 2,
            TipePoin::Jatuhan => 3,
            TipePoin::Kuncian => 4,
        }
    }
}

/// Tipe Hukuman dan Pelanggaran
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TipeHukuman {
    Teguran1,     // -1 Poin
    Teguran2,     // -2 Poin
    Peringatan1,  // -5 Poin (Kartu Kuning 1)
    Peringatan2,  // -10 Poin (Kartu Kuning 2)
    Peringatan3,  // Diskualifikasi (Kartu Merah)
}

impl TipeHukuman {
    pub fn pengurangan_poin(&self) -> i32 {
        match self {
            TipeHukuman::Teguran1 => -1,
            TipeHukuman::Teguran2 => -2,
            TipeHukuman::Peringatan1 => -5,
            TipeHukuman::Peringatan2 => -10,
            TipeHukuman::Peringatan3 => 0, // Diskualifikasi langsung
        }
    }
}

/// Input Tekanan Tombol Juri
#[derive(Debug, Clone)]
pub struct InputTombolJuri {
    pub juri_id: u8,          // Juri 1, 2, atau 3
    pub sudut: SudutPesilat,
    pub tipe: TipePoin,
    pub timestamp: Instant,
}

/// Rekod Skor Pertandingan
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RekodSkorSudut {
    pub poin_babak: [i32; 3],  // Babak 1, 2, 3
    pub total_poin: i32,
    pub jumlah_pukulan: u32,
    pub jumlah_tendangan: u32,
    pub jumlah_jatuhan: u32,
    pub hukuman: Vec<String>,
    pub kartu_kuning: u8,
    pub kartu_merah: bool,
    pub kartu_var_tersedia: u8, // Kuota kartu review VAR (default 2 per partai)
}

/// Status Pertandingan
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusPartai {
    Persiapan,
    Berlangsung,
    JedaBabak,
    TinjauanVAR,
    WaktuMedis,
    Selesai,
}

/// Mesin Penilaian & Konsensus Juri IPSI 2026
pub struct ArenaScoringEngine {
    pub partai_id: String,
    pub babak_aktif: usize, // 1, 2, atau 3
    pub waktu_detik_tersisa: u32,
    pub status: StatusPartai,
    pub skor_merah: RekodSkorSudut,
    pub skor_biru: RekodSkorSudut,
    
    // Buffer input tombol juri untuk konsensus window (1.5 detik)
    antrian_input: VecDeque<InputTombolJuri>,
    consensus_window: Duration,
    
    // Log audit konsensus yang berhasil disahkan
    pub riwayat_poin_sah: Vec<String>,
}

impl ArenaScoringEngine {
    pub fn baru(partai_id: &str) -> Self {
        let mut skor_merah = RekodSkorSudut::default();
        let mut skor_biru = RekodSkorSudut::default();
        skor_merah.kartu_var_tersedia = 2;
        skor_biru.kartu_var_tersedia = 2;

        Self {
            partai_id: partai_id.to_string(),
            babak_aktif: 1,
            waktu_detik_tersisa: 120, // 2 menit per babak
            status: StatusPartai::Persiapan,
            skor_merah,
            skor_biru,
            antrian_input: VecDeque::new(),
            consensus_window: Duration::from_millis(1500), // Window 1.5 detik
            riwayat_poin_sah: Vec::new(),
        }
    }

    /// Kirim tombol input dari Juri (Juri 1, 2, atau 3)
    /// Mengembalikan Some((SudutPesilat, TipePoin, poin)) jika tercapai konsensus minimal 2 juri sepakat
    pub fn tekan_tombol_juri(&mut self, juri_id: u8, sudut: SudutPesilat, tipe: TipePoin) -> Option<(SudutPesilat, TipePoin, i32)> {
        if self.status != StatusPartai::Berlangsung {
            return None;
        }

        let now = Instant::now();
        
        // Bersihkan input usang di luar consensus_window
        let window = self.consensus_window;
        self.antrian_input.retain(|inp| now.duration_since(inp.timestamp) <= window);

        // Periksa apakah juri ini sudah menekan untuk sudut & tipe yang sama dalam window
        let sudah_ada = self.antrian_input.iter().any(|inp| {
            inp.juri_id == juri_id && inp.sudut == sudut && inp.tipe == tipe
        });

        if !sudah_ada {
            self.antrian_input.push_back(InputTombolJuri {
                juri_id,
                sudut,
                tipe,
                timestamp: now,
            });
        }

        // Cek Konsensus: Hitung berapa juri berbeda yang menekan tombol yang sama dalam window
        let mut juri_set = HashMap::new();
        for inp in self.antrian_input.iter() {
            if inp.sudut == sudut && inp.tipe == tipe {
                juri_set.insert(inp.juri_id, true);
            }
        }

        // Syarat Konsensus IPSI 2026: Minimal 2 dari 3 juri sepakat
        if juri_set.len() >= 2 {
            let nilai = tipe.nilai();
            self.tambahkan_poin(sudut, tipe, nilai);
            
            // Bersihkan input yang sudah dikonsensuskan
            self.antrian_input.retain(|inp| !(inp.sudut == sudut && inp.tipe == tipe));

            let log_entry = format!(
                "Babak {} [Consensus {} Juri]: Sudut {:?} mendapat {:?} (+{} poin)",
                self.babak_aktif, juri_set.len(), sudut, tipe, nilai
            );
            self.riwayat_poin_sah.push(log_entry);

            Some((sudut, tipe, nilai))
        } else {
            None
        }
    }

    /// Tambahkan poin sah ke skor sudut
    pub fn tambahkan_poin(&mut self, sudut: SudutPesilat, tipe: TipePoin, nilai: i32) {
        let b_idx = (self.babak_aktif.max(1).min(3)) - 1;
        let skor = match sudut {
            SudutPesilat::Merah => &mut self.skor_merah,
            SudutPesilat::Biru => &mut self.skor_biru,
        };

        skor.poin_babak[b_idx] += nilai;
        skor.total_poin += nilai;

        match tipe {
            TipePoin::Pukulan => skor.jumlah_pukulan += 1,
            TipePoin::Tendangan => skor.jumlah_tendangan += 1,
            TipePoin::Jatuhan => skor.jumlah_jatuhan += 1,
            TipePoin::Kuncian => {},
        }
    }

    /// Berikan Hukuman dari Ketua Pertandingan (KP)
    pub fn berikan_hukuman(&mut self, sudut: SudutPesilat, hukuman: TipeHukuman) {
        let b_idx = (self.babak_aktif.max(1).min(3)) - 1;
        let skor = match sudut {
            SudutPesilat::Merah => &mut self.skor_merah,
            SudutPesilat::Biru => &mut self.skor_biru,
        };

        let minus = hukuman.pengurangan_poin();
        skor.poin_babak[b_idx] += minus;
        skor.total_poin += minus;
        skor.hukuman.push(format!("{:?}", hukuman));

        match hukuman {
            TipeHukuman::Peringatan1 => skor.kartu_kuning = 1,
            TipeHukuman::Peringatan2 => skor.kartu_kuning = 2,
            TipeHukuman::Peringatan3 => skor.kartu_merah = true,
            _ => {},
        }

        self.riwayat_poin_sah.push(format!(
            "Babak {} [HUKUMAN KP]: Sudut {:?} dikenakan {:?} ({} poin)",
            self.babak_aktif, sudut, hukuman, minus
        ));
    }

    /// Klaim / Gunakan Kartu Video Review (VAR) oleh Ofisial
    pub fn ajukan_kartu_var(&mut self, sudut: SudutPesilat) -> Result<u8, String> {
        let skor = match sudut {
            SudutPesilat::Merah => &mut self.skor_merah,
            SudutPesilat::Biru => &mut self.skor_biru,
        };

        if skor.kartu_var_tersedia == 0 {
            return Err("Kuota kartu review VAR sudah habis!".to_string());
        }

        skor.kartu_var_tersedia -= 1;
        self.status = StatusPartai::TinjauanVAR;
        
        self.riwayat_poin_sah.push(format!(
            "Babak {} [VAR REQUEST]: Sudut {:?} mengajukan review video! Sisa kuota: {}",
            self.babak_aktif, sudut, skor.kartu_var_tersedia
        ));

        Ok(skor.kartu_var_tersedia)
    }

    /// Kembalikan kartu VAR jika protes ofisial diterima oleh Dewan Hakim
    pub fn kembalikan_kartu_var(&mut self, sudut: SudutPesilat) {
        let skor = match sudut {
            SudutPesilat::Merah => &mut self.skor_merah,
            SudutPesilat::Biru => &mut self.skor_biru,
        };
        skor.kartu_var_tersedia += 1;
    }

    /// Mulai / Lanjutkan Pertandingan
    pub fn mulai_pertandingan(&mut self) {
        self.status = StatusPartai::Berlangsung;
    }

    /// Jeda Babak Pertandingan
    pub fn jeda_pertandingan(&mut self) {
        self.status = StatusPartai::JedaBabak;
    }

    /// Naikkan Babak Pertandingan (Maksimal 3 Babak)
    pub fn lanjut_babak_berikutnya(&mut self) -> Result<usize, String> {
        if self.babak_aktif >= 3 {
            self.status = StatusPartai::Selesai;
            return Err("Pertandingan telah mencapai batas 3 babak!".to_string());
        }
        self.babak_aktif += 1;
        self.waktu_detik_tersisa = 120;
        self.status = StatusPartai::Persiapan;
        Ok(self.babak_aktif)
    }

    /// Tentukan Pemenang Pertandingan
    pub fn tentukan_pemenang(&self) -> (Option<SudutPesilat>, String) {
        if self.skor_merah.kartu_merah {
            return (Some(SudutPesilat::Biru), "Menang Diskualifikasi (Sudut Merah Kartu Merah)".to_string());
        }
        if self.skor_biru.kartu_merah {
            return (Some(SudutPesilat::Merah), "Menang Diskualifikasi (Sudut Biru Kartu Merah)".to_string());
        }

        if self.skor_merah.total_poin > self.skor_biru.total_poin {
            (Some(SudutPesilat::Merah), format!("Menang Angka ({} vs {})", self.skor_merah.total_poin, self.skor_biru.total_poin))
        } else if self.skor_biru.total_poin > self.skor_merah.total_poin {
            (Some(SudutPesilat::Biru), format!("Menang Angka ({} vs {})", self.skor_biru.total_poin, self.skor_merah.total_poin))
        } else {
            // Jika seri, aturan IPSI: utamakan jumlah jatuhan, lalu tendangan, lalu pukulan
            if self.skor_merah.jumlah_jatuhan > self.skor_biru.jumlah_jatuhan {
                (Some(SudutPesilat::Merah), "Menang Keunggulan Teknik Jatuhan (Skor Seri)".to_string())
            } else if self.skor_biru.jumlah_jatuhan > self.skor_merah.jumlah_jatuhan {
                (Some(SudutPesilat::Biru), "Menang Keunggulan Teknik Jatuhan (Skor Seri)".to_string())
            } else if self.skor_merah.jumlah_tendangan > self.skor_biru.jumlah_tendangan {
                (Some(SudutPesilat::Merah), "Menang Keunggulan Teknik Tendangan (Skor Seri)".to_string())
            } else if self.skor_biru.jumlah_tendangan > self.skor_merah.jumlah_tendangan {
                (Some(SudutPesilat::Biru), "Menang Keunggulan Teknik Tendangan (Skor Seri)".to_string())
            } else {
                (None, "Hasil Imbang Sempurna (Perlu Keputusan Dewan Hakim / Timbangan)".to_string())
            }
        }
    }
}
