use super::sumber_dana::JenisSumberDana;
use serde::{Deserialize, Serialize};

/// ============================================================================
/// BUKU KAS UMUM (BKU) & TRANSAKSI REALISASI KEUANGAN SEKOLAH (ARKAS)
/// Pembukuan Kas Tunai, Kas Bank, Pajak (PPh 21/22/23, PPN) & Integrasi SIPLah
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JenisTransaksiBku {
    PenerimaanDanaTransfer,       // Penerimaan dana cair BOSP/BOSDA/Banper APBN
    TarikTunaiDariBank,           // Pindah buku Kas Bank -> Kas Tunai
    SetorTunaiKeBank,             // Pindah buku Kas Tunai -> Kas Bank
    BelanjaOperasionalSIPLah,     // Belanja daring via SIPLah (Nomor SP & BAST)
    BelanjaNonSIPLah,             // Belanja langsung / tagihan rutin PLN/PDAM/Honor
    PungutPajakBelanja,           // Pemotongan PPh 21/22/23 atau PPN
    SetorPajakKeKasNegara,        // Pembayaran NTPN melalui e-Billing Pajak
    BungaBank,                    // Pendapatan jasa giro / bunga bank
    PajakBungaDanBiayaAdminBank,  // Biaya administrasi bank
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetodePembayaranBku {
    TransferBankGiro,
    KasTunaiBendahara,
    VirtualAccountSIPLah,
    QrPayment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RincianPajakTransaksi {
    pub pph_21_rp: f64,
    pub pph_22_rp: f64,
    pub pph_23_rp: f64,
    pub ppn_11_rp: f64,
    pub kode_billing_pajak: Option<String>,
    pub ntpn_bukti_setor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransaksiBkuEntry {
    pub nomor_urut_bku: u32,
    pub tanggal_transaksi: String,
    pub nomor_bukti: String, // e.g. "KWT-202610-001", "SP-SIPLah-20261001-0089"
    pub uraian: String,
    pub jenis_transaksi: JenisTransaksiBku,
    pub sumber_dana: JenisSumberDana,
    pub id_kegiatan_rkas: Option<String>,
    pub metode_bayar: MetodePembayaranBku,
    pub penerimaan_rp: f64,
    pub pengeluaran_rp: f64,
    pub saldo_kumulatif_rp: f64,
    pub rincian_pajak: Option<RincianPajakTransaksi>,
    pub id_sp_siplah: Option<String>,
    pub nomor_bast_siplah: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BukuKasUmumSekolah {
    pub npsn_sekolah: String,
    pub tahun_anggaran: u32,
    pub bulan_buku: u32, // 1 - 12
    pub saldo_awal_bank_rp: f64,
    pub saldo_awal_tunai_rp: f64,
    pub daftar_transaksi: Vec<TransaksiBkuEntry>,
    pub status_tutup_buku: bool,
    pub tanggal_tutup_buku: Option<String>,
}

impl BukuKasUmumSekolah {
    pub fn new(npsn: &str, tahun: u32, bulan: u32, saldo_awal_bank: f64, saldo_awal_tunai: f64) -> Self {
        Self {
            npsn_sekolah: npsn.to_string(),
            tahun_anggaran: tahun,
            bulan_buku: bulan,
            saldo_awal_bank_rp: saldo_awal_bank,
            saldo_awal_tunai_rp: saldo_awal_tunai,
            daftar_transaksi: Vec::new(),
            status_tutup_buku: false,
            tanggal_tutup_buku: None,
        }
    }

    /// Saldo kas total saat ini (Bank + Tunai)
    pub fn saldo_total_saat_ini(&self) -> f64 {
        let mut total = self.saldo_awal_bank_rp + self.saldo_awal_tunai_rp;
        for t in &self.daftar_transaksi {
            total += t.penerimaan_rp;
            total -= t.pengeluaran_rp;
        }
        total
    }

    /// Catat Penerimaan Dana Masuk ke Kas Bank
    pub fn catat_penerimaan_dana(
        &mut self,
        tanggal: &str,
        no_bukti: &str,
        uraian: &str,
        sumber_dana: JenisSumberDana,
        nominal: f64,
    ) {
        let nomor_urut = (self.daftar_transaksi.len() + 1) as u32;
        let saldo_baru = self.saldo_total_saat_ini() + nominal;

        self.daftar_transaksi.push(TransaksiBkuEntry {
            nomor_urut_bku: nomor_urut,
            tanggal_transaksi: tanggal.to_string(),
            nomor_bukti: no_bukti.to_string(),
            uraian: uraian.to_string(),
            jenis_transaksi: JenisTransaksiBku::PenerimaanDanaTransfer,
            sumber_dana,
            id_kegiatan_rkas: None,
            metode_bayar: MetodePembayaranBku::TransferBankGiro,
            penerimaan_rp: nominal,
            pengeluaran_rp: 0.0,
            saldo_kumulatif_rp: saldo_baru,
            rincian_pajak: None,
            id_sp_siplah: None,
            nomor_bast_siplah: None,
        });
    }

    /// Catat Belanja Transaksi SIPLah dengan Pungutan Pajak Otomatis
    pub fn catat_belanja_siplah(
        &mut self,
        tanggal: &str,
        no_sp: &str,
        no_bast: &str,
        uraian: &str,
        sumber_dana: JenisSumberDana,
        id_kegiatan_rkas: &str,
        nilai_bruto: f64,
        pph_22: f64,
        ppn_11: f64,
    ) -> Result<TransaksiBkuEntry, String> {
        let saldo = self.saldo_total_saat_ini();
        if nilai_bruto > saldo {
            return Err(format!("Saldo Kas tidak mencukupi untuk belanja SIPLah! Saldo: Rp {}, Belanja: Rp {}", saldo, nilai_bruto));
        }

        let nomor_urut = (self.daftar_transaksi.len() + 1) as u32;
        let saldo_baru = saldo - nilai_bruto;

        let entry = TransaksiBkuEntry {
            nomor_urut_bku: nomor_urut,
            tanggal_transaksi: tanggal.to_string(),
            nomor_bukti: no_sp.to_string(),
            uraian: uraian.to_string(),
            jenis_transaksi: JenisTransaksiBku::BelanjaOperasionalSIPLah,
            sumber_dana,
            id_kegiatan_rkas: Some(id_kegiatan_rkas.to_string()),
            metode_bayar: MetodePembayaranBku::VirtualAccountSIPLah,
            penerimaan_rp: 0.0,
            pengeluaran_rp: nilai_bruto,
            saldo_kumulatif_rp: saldo_baru,
            rincian_pajak: Some(RincianPajakTransaksi {
                pph_21_rp: 0.0,
                pph_22_rp: pph_22,
                pph_23_rp: 0.0,
                ppn_11_rp: ppn_11,
                kode_billing_pajak: Some(format!("EBILLING-{}", no_sp)),
                ntpn_bukti_setor: None,
            }),
            id_sp_siplah: Some(no_sp.to_string()),
            nomor_bast_siplah: Some(no_bast.to_string()),
        };

        self.daftar_transaksi.push(entry.clone());
        Ok(entry)
    }

    /// Tutup Buku Bulanan BKU
    pub fn tutup_buku(&mut self, tanggal: &str) {
        self.status_tutup_buku = true;
        self.tanggal_tutup_buku = Some(tanggal.to_string());
    }
}
