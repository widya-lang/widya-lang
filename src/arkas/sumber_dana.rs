use serde::{Deserialize, Serialize};

/// ============================================================================
/// MODUL MULTI-SUMBER DANA PENDIDIKAN (ARKAS - KEMENDIKDASMEN RI)
/// Mendukung BOSP Reguler, BOS Kinerja, BOS Afirmasi, BOSDA, Banper APBN & Lainnya
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JenisSumberDana {
    BospReguler,              // Bantuan Operasional Satuan Pendidikan Reguler (Cut-off Dapodik)
    BosKinerja,               // BOS Kinerja Sekolah Berkemajuan Terbaik / Sekolah Penggerak
    BosAfirmasi,              // BOS Afirmasi Sekolah Daerah 3T / Khusus
    BosdaProvinsi,            // Bantuan Operasional Sekolah Daerah Tingkat I (Provinsi)
    BosdaKabupatenKota,       // Bantuan Operasional Sekolah Daerah Tingkat II (Kabupaten/Kota)
    BantuanPemerintahApbn,    // Banper APBN (Wajib PKS: Revitalisasi, MBG, Lab, dll.)
    DanaKomiteDanMasyarakat,  // Sumbangan Komite / CSR / Donasi Sukarela Tidak Mengikat
    PendapatanAsliSekolah,    // Unit Produksi SMK / BLUD / Jasa Layanan
    SilpaTahunLalu,           // Sisa Lebih Pembiayaan Anggaran Tahun Anggaran Sebelumnya
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusPencairanDana {
    Diusulkan,
    DiverifikasiDinas,
    DiterbitkanSP2D,
    DanaMasukRekeningSekolah,
    SelesaiDipertanggungjawabkan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RekeningBankSekolah {
    pub nomor_rekening: String,
    pub nama_bank: String,
    pub nama_pemilik_rekening: String,
    pub cabang_bank: String,
    pub npwp_sekolah: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SumberDanaAlokasi {
    pub id_alokasi: String,
    pub npsn_sekolah: String,
    pub tahun_anggaran: u32,
    pub jenis_sumber_dana: JenisSumberDana,
    pub nomor_dipa_atau_sk: String,
    pub termin_tahap: u32, // Tahap 1, Tahap 2, dst.
    pub jumlah_dana_pagu_rp: f64,
    pub jumlah_dana_cair_rp: f64,
    pub sisa_saldo_rp: f64,
    pub tanggal_cair: Option<String>,
    pub status: StatusPencairanDana,
    pub id_pks_banper: Option<String>, // Referensi PKS jika bersumber dari Banper APBN
}

impl SumberDanaAlokasi {
    pub fn new(
        id_alokasi: &str,
        npsn: &str,
        tahun: u32,
        jenis: JenisSumberDana,
        no_sk: &str,
        tahap: u32,
        pagu: f64,
        id_pks: Option<String>,
    ) -> Self {
        Self {
            id_alokasi: id_alokasi.to_string(),
            npsn_sekolah: npsn.to_string(),
            tahun_anggaran: tahun,
            jenis_sumber_dana: jenis,
            nomor_dipa_atau_sk: no_sk.to_string(),
            termin_tahap: tahap,
            jumlah_dana_pagu_rp: pagu,
            jumlah_dana_cair_rp: 0.0,
            sisa_saldo_rp: 0.0,
            tanggal_cair: None,
            status: StatusPencairanDana::Diusulkan,
            id_pks_banper: id_pks,
        }
    }

    /// Cairkan dana ke kas sekolah
    pub fn cairkan_dana(&mut self, tanggal: &str, jumlah_cair: f64) -> Result<(), String> {
        if jumlah_cair > self.jumlah_dana_pagu_rp {
            return Err(format!("Pencairan Rp {} melebihi pagu anggaran Rp {}", jumlah_cair, self.jumlah_dana_pagu_rp));
        }
        self.jumlah_dana_cair_rp = jumlah_cair;
        self.sisa_saldo_rp = jumlah_cair;
        self.tanggal_cair = Some(tanggal.to_string());
        self.status = StatusPencairanDana::DanaMasukRekeningSekolah;
        Ok(())
    }

    /// Kurangi saldo saat ada realisasi BKU
    pub fn debet_realisasi(&mut self, jumlah_belanja: f64) -> Result<(), String> {
        if jumlah_belanja > self.sisa_saldo_rp {
            return Err(format!(
                "Saldo sumber dana {:?} tidak mencukupi! Sisa: Rp {}, Diminta: Rp {}",
                self.jenis_sumber_dana, self.sisa_saldo_rp, jumlah_belanja
            ));
        }
        self.sisa_saldo_rp -= jumlah_belanja;
        Ok(())
    }
}
