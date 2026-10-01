use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 2. UNIT SIMPAN PINJAM (USP / KSP SYARIAH & KONVENSIONAL DESA)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JenisAkadPembiayaan {
    MurabahahJualBeli, // Syariah: Jual beli barang dengan margin disepakati
    QardhKebajikan,    // Syariah: Pinjaman tanpa bunga untuk darurat sosial
    MudharabahBagiHasil,// Syariah: Bagi hasil usaha produktif
    KonvensionalAnuitas,// Bunga anuitas flat ringan koperasi
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusPinjaman {
    Diajukan,
    Disetujui,
    Dicairkan,
    Lunas,
    Macet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RekeningTabungan {
    pub nomor_rekening: String,
    pub nomor_pokok_anggota: String,
    pub saldo_idr: u64,
    pub jenis_tabungan: String, // "Tabungan Sukarela", "Tabungan Qurban", "Tabungan Pendidikan"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AngsuranBulanan {
    pub bulan_ke: u32,
    pub pokok_idr: u64,
    pub bagi_hasil_margin_idr: u64,
    pub total_angsuran_idr: u64,
    pub sisa_pokok_idr: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinjamanAnggota {
    pub id_pinjaman: String,
    pub nomor_pokok_anggota: String,
    pub nominal_pengajuan_idr: u64,
    pub tenor_bulan: u32,
    pub akad: JenisAkadPembiayaan,
    pub margin_atau_bunga_tahunan_pct: f64,
    pub tujuan_penggunaan: String, // e.g. "Modal Tani Padi", "Modal Warung", "Renovasi Rumah"
    pub status: StatusPinjaman,
    pub jadwal_angsuran: Vec<AngsuranBulanan>,
    pub total_terbayar_idr: u64,
}

pub struct UnitSimpanPinjam {
    pub rekening_tabungan: HashMap<String, RekeningTabungan>,
    pub pinjaman: HashMap<String, PinjamanAnggota>,
    pub total_dana_kelolaan_idr: u64,
}

impl UnitSimpanPinjam {
    pub fn new() -> Self {
        Self {
            rekening_tabungan: HashMap::new(),
            pinjaman: HashMap::new(),
            total_dana_kelolaan_idr: 0,
        }
    }

    /// Buka rekening tabungan sukarela anggota
    pub fn buka_tabungan(&mut self, no_rek: &str, npa: &str, jenis: &str) -> RekeningTabungan {
        let rek = RekeningTabungan {
            nomor_rekening: no_rek.to_string(),
            nomor_pokok_anggota: npa.to_string(),
            saldo_idr: 0,
            jenis_tabungan: jenis.to_string(),
        };
        self.rekening_tabungan.insert(no_rek.to_string(), rek.clone());
        rek
    }

    /// Setor tabungan
    pub fn setor_tabungan(&mut self, no_rek: &str, nominal: u64) -> Result<u64, String> {
        if let Some(r) = self.rekening_tabungan.get_mut(no_rek) {
            r.saldo_idr += nominal;
            self.total_dana_kelolaan_idr += nominal;
            Ok(r.saldo_idr)
        } else {
            Err(format!("Rekening {} tidak ditemukan", no_rek))
        }
    }

    /// Pengajuan dan simulasi pembiayaan mikro usaha desa (Amortisasi Flat/Murabahah)
    pub fn ajukan_pinjaman(
        &mut self,
        id_pinjaman: &str,
        npa: &str,
        nominal: u64,
        tenor: u32,
        akad: JenisAkadPembiayaan,
        margin_pct: f64,
        tujuan: &str,
    ) -> PinjamanAnggota {
        let total_margin = (nominal as f64 * (margin_pct / 100.0) * (tenor as f64 / 12.0)).round() as u64;
        let total_kewajiban = nominal + total_margin;
        let cicilan_bulanan = (total_kewajiban as f64 / tenor as f64).round() as u64;
        let pokok_bulanan = (nominal as f64 / tenor as f64).round() as u64;
        let margin_bulanan = cicilan_bulanan - pokok_bulanan;

        let mut jadwal = Vec::with_capacity(tenor as usize);
        let mut sisa_pokok = nominal;

        for b in 1..=tenor {
            sisa_pokok = if sisa_pokok > pokok_bulanan { sisa_pokok - pokok_bulanan } else { 0 };
            jadwal.push(AngsuranBulanan {
                bulan_ke: b,
                pokok_idr: pokok_bulanan,
                bagi_hasil_margin_idr: margin_bulanan,
                total_angsuran_idr: cicilan_bulanan,
                sisa_pokok_idr: sisa_pokok,
            });
        }

        let p = PinjamanAnggota {
            id_pinjaman: id_pinjaman.to_string(),
            nomor_pokok_anggota: npa.to_string(),
            nominal_pengajuan_idr: nominal,
            tenor_bulan: tenor,
            akad,
            margin_atau_bunga_tahunan_pct: margin_pct,
            tujuan_penggunaan: tujuan.to_string(),
            status: StatusPinjaman::Disetujui,
            jadwal_angsuran: jadwal,
            total_terbayar_idr: 0,
        };

        self.pinjaman.insert(id_pinjaman.to_string(), p.clone());
        p
    }
}
