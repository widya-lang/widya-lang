use super::sumber_dana::JenisSumberDana;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// ============================================================================
/// BUKU KAS UMUM (BKU) & TRANSAKSI REALISASI KEUANGAN SEKOLAH (ARKAS)
/// Pembukuan Kas Tunai, Kas Bank, Pajak (PPh 21/22/23, PPN) & Integrasi SIPLah
/// Dilengkapi Rantai Hash Audit Kriptografis (Tamper-Evident Ledger BPK/Inspektorat)
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

impl RincianPajakTransaksi {
    pub fn total_pajak(&self) -> f64 {
        self.pph_21_rp + self.pph_22_rp + self.pph_23_rp + self.ppn_11_rp
    }
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
    #[serde(default)]
    pub hash_transaksi: Option<String>,
    #[serde(default)]
    pub hash_sebelumnya: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BukuPembantuKas {
    pub npsn_sekolah: String,
    pub tahun_anggaran: u32,
    pub bulan_buku: u32,
    pub saldo_awal_rp: f64,
    pub total_penerimaan_rp: f64,
    pub total_pengeluaran_rp: f64,
    pub saldo_akhir_rp: f64,
    pub daftar_transaksi: Vec<TransaksiBkuEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BukuPembantuBank {
    pub npsn_sekolah: String,
    pub tahun_anggaran: u32,
    pub bulan_buku: u32,
    pub saldo_awal_rp: f64,
    pub total_penerimaan_rp: f64,
    pub total_pengeluaran_rp: f64,
    pub saldo_akhir_rp: f64,
    pub daftar_transaksi: Vec<TransaksiBkuEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BukuPembantuPajak {
    pub npsn_sekolah: String,
    pub tahun_anggaran: u32,
    pub bulan_buku: u32,
    pub total_pph_21_rp: f64,
    pub total_pph_22_rp: f64,
    pub total_pph_23_rp: f64,
    pub total_ppn_11_rp: f64,
    pub total_pajak_dipungut_rp: f64,
    pub total_pajak_disetor_rp: f64,
    pub sisa_saldo_titipan_pajak_rp: f64,
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

fn hitung_hash_entry(
    prev_hash: &str,
    nomor_urut: u32,
    tanggal: &str,
    nomor_bukti: &str,
    terima: f64,
    keluar: f64,
    saldo: f64,
) -> String {
    let mut hasher = Sha256::new();
    let data = format!(
        "{}:{}:{}:{}:{:.2}:{:.2}:{:.2}",
        prev_hash, nomor_urut, tanggal, nomor_bukti, terima, keluar, saldo
    );
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

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

    fn ambil_prev_hash(&self) -> String {
        self.daftar_transaksi
            .last()
            .and_then(|t| t.hash_transaksi.clone())
            .unwrap_or_else(|| GENESIS_HASH.to_string())
    }

    /// Saldo kas bank saat ini
    pub fn saldo_bank_saat_ini(&self) -> f64 {
        let mut total = self.saldo_awal_bank_rp;
        for t in &self.daftar_transaksi {
            match t.jenis_transaksi {
                JenisTransaksiBku::PenerimaanDanaTransfer | JenisTransaksiBku::BungaBank => {
                    total += t.penerimaan_rp;
                }
                JenisTransaksiBku::TarikTunaiDariBank => {
                    total -= t.pengeluaran_rp;
                }
                JenisTransaksiBku::SetorTunaiKeBank => {
                    total += t.penerimaan_rp;
                }
                JenisTransaksiBku::BelanjaOperasionalSIPLah | JenisTransaksiBku::PajakBungaDanBiayaAdminBank => {
                    total -= t.pengeluaran_rp;
                }
                JenisTransaksiBku::BelanjaNonSIPLah | JenisTransaksiBku::SetorPajakKeKasNegara => {
                    if t.metode_bayar != MetodePembayaranBku::KasTunaiBendahara {
                        total -= t.pengeluaran_rp;
                    }
                }
                JenisTransaksiBku::PungutPajakBelanja => {}
            }
        }
        total
    }

    /// Saldo kas tunai saat ini
    pub fn saldo_tunai_saat_ini(&self) -> f64 {
        let mut total = self.saldo_awal_tunai_rp;
        for t in &self.daftar_transaksi {
            match t.jenis_transaksi {
                JenisTransaksiBku::TarikTunaiDariBank => {
                    total += t.pengeluaran_rp;
                }
                JenisTransaksiBku::SetorTunaiKeBank => {
                    total -= t.pengeluaran_rp;
                }
                JenisTransaksiBku::BelanjaNonSIPLah | JenisTransaksiBku::SetorPajakKeKasNegara => {
                    if t.metode_bayar == MetodePembayaranBku::KasTunaiBendahara {
                        total -= t.pengeluaran_rp;
                    }
                }
                _ => {}
            }
        }
        total
    }

    /// Saldo kas total saat ini (Bank + Tunai)
    pub fn saldo_total_saat_ini(&self) -> f64 {
        self.saldo_bank_saat_ini() + self.saldo_tunai_saat_ini()
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
        let prev_hash = self.ambil_prev_hash();
        let hash_cur = hitung_hash_entry(
            &prev_hash,
            nomor_urut,
            tanggal,
            no_bukti,
            nominal,
            0.0,
            saldo_baru,
        );

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
            hash_transaksi: Some(hash_cur),
            hash_sebelumnya: Some(prev_hash),
        });
    }

    /// Catat Penarikan Tunai dari Bank ke Kas Tunai Bendahara
    pub fn tarik_tunai_bank(
        &mut self,
        tanggal: &str,
        no_bukti: &str,
        nominal: f64,
    ) -> Result<TransaksiBkuEntry, String> {
        let saldo_bank = self.saldo_bank_saat_ini();
        if nominal > saldo_bank {
            return Err(format!(
                "Saldo Bank tidak mencukupi untuk penarikan tunai! Saldo Bank: Rp {:.2}, Tarik: Rp {:.2}",
                saldo_bank, nominal
            ));
        }

        let nomor_urut = (self.daftar_transaksi.len() + 1) as u32;
        let saldo_total = self.saldo_total_saat_ini();
        let prev_hash = self.ambil_prev_hash();
        let hash_cur = hitung_hash_entry(
            &prev_hash,
            nomor_urut,
            tanggal,
            no_bukti,
            0.0,
            nominal,
            saldo_total,
        );

        let entry = TransaksiBkuEntry {
            nomor_urut_bku: nomor_urut,
            tanggal_transaksi: tanggal.to_string(),
            nomor_bukti: no_bukti.to_string(),
            uraian: format!("Penarikan Tunai dari Rekening Bank untuk Operasional Kas Tunai (Rp {:.2})", nominal),
            jenis_transaksi: JenisTransaksiBku::TarikTunaiDariBank,
            sumber_dana: JenisSumberDana::BospReguler,
            id_kegiatan_rkas: None,
            metode_bayar: MetodePembayaranBku::TransferBankGiro,
            penerimaan_rp: 0.0,
            pengeluaran_rp: nominal,
            saldo_kumulatif_rp: saldo_total,
            rincian_pajak: None,
            id_sp_siplah: None,
            nomor_bast_siplah: None,
            hash_transaksi: Some(hash_cur),
            hash_sebelumnya: Some(prev_hash),
        };

        self.daftar_transaksi.push(entry.clone());
        Ok(entry)
    }

    /// Catat Setor Tunai dari Kas Bendahara ke Bank
    pub fn setor_tunai_ke_bank(
        &mut self,
        tanggal: &str,
        no_bukti: &str,
        nominal: f64,
    ) -> Result<TransaksiBkuEntry, String> {
        let saldo_tunai = self.saldo_tunai_saat_ini();
        if nominal > saldo_tunai {
            return Err(format!(
                "Saldo Tunai tidak mencukupi untuk setor ke bank! Saldo Tunai: Rp {:.2}, Setor: Rp {:.2}",
                saldo_tunai, nominal
            ));
        }

        let nomor_urut = (self.daftar_transaksi.len() + 1) as u32;
        let saldo_total = self.saldo_total_saat_ini();
        let prev_hash = self.ambil_prev_hash();
        let hash_cur = hitung_hash_entry(
            &prev_hash,
            nomor_urut,
            tanggal,
            no_bukti,
            nominal,
            0.0,
            saldo_total,
        );

        let entry = TransaksiBkuEntry {
            nomor_urut_bku: nomor_urut,
            tanggal_transaksi: tanggal.to_string(),
            nomor_bukti: no_bukti.to_string(),
            uraian: format!("Setor Kas Tunai ke Rekening Bank (Rp {:.2})", nominal),
            jenis_transaksi: JenisTransaksiBku::SetorTunaiKeBank,
            sumber_dana: JenisSumberDana::BospReguler,
            id_kegiatan_rkas: None,
            metode_bayar: MetodePembayaranBku::TransferBankGiro,
            penerimaan_rp: nominal,
            pengeluaran_rp: nominal,
            saldo_kumulatif_rp: saldo_total,
            rincian_pajak: None,
            id_sp_siplah: None,
            nomor_bast_siplah: None,
            hash_transaksi: Some(hash_cur),
            hash_sebelumnya: Some(prev_hash),
        };

        self.daftar_transaksi.push(entry.clone());
        Ok(entry)
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
        let saldo = self.saldo_bank_saat_ini();
        if nilai_bruto > saldo {
            return Err(format!("Saldo Kas tidak mencukupi untuk belanja SIPLah! Saldo: Rp {}, Belanja: Rp {}", saldo, nilai_bruto));
        }

        let nomor_urut = (self.daftar_transaksi.len() + 1) as u32;
        let saldo_baru = self.saldo_total_saat_ini() - nilai_bruto;
        let prev_hash = self.ambil_prev_hash();
        let hash_cur = hitung_hash_entry(
            &prev_hash,
            nomor_urut,
            tanggal,
            no_sp,
            0.0,
            nilai_bruto,
            saldo_baru,
        );

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
            hash_transaksi: Some(hash_cur),
            hash_sebelumnya: Some(prev_hash),
        };

        self.daftar_transaksi.push(entry.clone());
        Ok(entry)
    }

    /// Catat Belanja Non-SIPLah (Langsung/Honor/Langganan Daya & Jasa)
    pub fn catat_belanja_non_siplah(
        &mut self,
        tanggal: &str,
        no_bukti: &str,
        uraian: &str,
        sumber_dana: JenisSumberDana,
        id_kegiatan_rkas: &str,
        metode_bayar: MetodePembayaranBku,
        nilai_bruto: f64,
        rincian_pajak: Option<RincianPajakTransaksi>,
    ) -> Result<TransaksiBkuEntry, String> {
        let saldo_sumber = match metode_bayar {
            MetodePembayaranBku::KasTunaiBendahara => self.saldo_tunai_saat_ini(),
            _ => self.saldo_bank_saat_ini(),
        };

        if nilai_bruto > saldo_sumber {
            return Err(format!(
                "Saldo tidak mencukupi untuk belanja Non-SIPLah! Saldo Tersedia: Rp {:.2}, Kebutuhan: Rp {:.2}",
                saldo_sumber, nilai_bruto
            ));
        }

        let nomor_urut = (self.daftar_transaksi.len() + 1) as u32;
        let saldo_baru = self.saldo_total_saat_ini() - nilai_bruto;
        let prev_hash = self.ambil_prev_hash();
        let hash_cur = hitung_hash_entry(
            &prev_hash,
            nomor_urut,
            tanggal,
            no_bukti,
            0.0,
            nilai_bruto,
            saldo_baru,
        );

        let entry = TransaksiBkuEntry {
            nomor_urut_bku: nomor_urut,
            tanggal_transaksi: tanggal.to_string(),
            nomor_bukti: no_bukti.to_string(),
            uraian: uraian.to_string(),
            jenis_transaksi: JenisTransaksiBku::BelanjaNonSIPLah,
            sumber_dana,
            id_kegiatan_rkas: Some(id_kegiatan_rkas.to_string()),
            metode_bayar,
            penerimaan_rp: 0.0,
            pengeluaran_rp: nilai_bruto,
            saldo_kumulatif_rp: saldo_baru,
            rincian_pajak,
            id_sp_siplah: None,
            nomor_bast_siplah: None,
            hash_transaksi: Some(hash_cur),
            hash_sebelumnya: Some(prev_hash),
        };

        self.daftar_transaksi.push(entry.clone());
        Ok(entry)
    }

    /// Catat Penyetoran Pajak ke Kas Negara via e-Billing & NTPN
    pub fn catat_setor_pajak(
        &mut self,
        tanggal: &str,
        kode_billing: &str,
        ntpn: &str,
        uraian: &str,
        nominal_pajak: f64,
        metode_bayar: MetodePembayaranBku,
    ) -> Result<TransaksiBkuEntry, String> {
        let nomor_urut = (self.daftar_transaksi.len() + 1) as u32;
        let saldo_total = self.saldo_total_saat_ini();
        let prev_hash = self.ambil_prev_hash();
        let hash_cur = hitung_hash_entry(
            &prev_hash,
            nomor_urut,
            tanggal,
            ntpn,
            0.0,
            nominal_pajak,
            saldo_total,
        );

        let entry = TransaksiBkuEntry {
            nomor_urut_bku: nomor_urut,
            tanggal_transaksi: tanggal.to_string(),
            nomor_bukti: ntpn.to_string(),
            uraian: format!("Penyetoran Pajak ke Kas Negara (NTPN: {}): {}", ntpn, uraian),
            jenis_transaksi: JenisTransaksiBku::SetorPajakKeKasNegara,
            sumber_dana: JenisSumberDana::BospReguler,
            id_kegiatan_rkas: None,
            metode_bayar,
            penerimaan_rp: 0.0,
            pengeluaran_rp: 0.0, // Pajak belanja dicatat sebagai penyetoran titipan pajak
            saldo_kumulatif_rp: saldo_total,
            rincian_pajak: Some(RincianPajakTransaksi {
                pph_21_rp: 0.0,
                pph_22_rp: 0.0,
                pph_23_rp: 0.0,
                ppn_11_rp: nominal_pajak,
                kode_billing_pajak: Some(kode_billing.to_string()),
                ntpn_bukti_setor: Some(ntpn.to_string()),
            }),
            id_sp_siplah: None,
            nomor_bast_siplah: None,
            hash_transaksi: Some(hash_cur),
            hash_sebelumnya: Some(prev_hash),
        };

        self.daftar_transaksi.push(entry.clone());
        Ok(entry)
    }

    /// Verifikasi Integritas Rantai Hash Audit BKU (Tamper-Evident Ledger)
    pub fn verifikasi_integritas_rantai(&self) -> bool {
        let mut expected_prev = GENESIS_HASH.to_string();
        for entry in &self.daftar_transaksi {
            if let Some(prev) = &entry.hash_sebelumnya {
                if prev != &expected_prev {
                    return false;
                }
            } else {
                return false;
            }

            let computed = hitung_hash_entry(
                &expected_prev,
                entry.nomor_urut_bku,
                &entry.tanggal_transaksi,
                &entry.nomor_bukti,
                entry.penerimaan_rp,
                entry.pengeluaran_rp,
                entry.saldo_kumulatif_rp,
            );

            if entry.hash_transaksi.as_deref() != Some(&computed) {
                return false;
            }

            expected_prev = computed;
        }
        true
    }

    /// Ekstraksi Buku Pembantu Kas Tunai
    pub fn buat_buku_pembantu_kas(&self) -> BukuPembantuKas {
        let mut total_terima = 0.0;
        let mut total_keluar = 0.0;
        let mut mutasi = Vec::new();

        for t in &self.daftar_transaksi {
            let mut relevant = false;
            if t.jenis_transaksi == JenisTransaksiBku::TarikTunaiDariBank {
                total_terima += t.pengeluaran_rp;
                relevant = true;
            } else if t.jenis_transaksi == JenisTransaksiBku::SetorTunaiKeBank {
                total_keluar += t.pengeluaran_rp;
                relevant = true;
            } else if (t.jenis_transaksi == JenisTransaksiBku::BelanjaNonSIPLah
                || t.jenis_transaksi == JenisTransaksiBku::SetorPajakKeKasNegara)
                && t.metode_bayar == MetodePembayaranBku::KasTunaiBendahara
            {
                total_keluar += t.pengeluaran_rp;
                relevant = true;
            }

            if relevant {
                mutasi.push(t.clone());
            }
        }

        BukuPembantuKas {
            npsn_sekolah: self.npsn_sekolah.clone(),
            tahun_anggaran: self.tahun_anggaran,
            bulan_buku: self.bulan_buku,
            saldo_awal_rp: self.saldo_awal_tunai_rp,
            total_penerimaan_rp: total_terima,
            total_pengeluaran_rp: total_keluar,
            saldo_akhir_rp: self.saldo_tunai_saat_ini(),
            daftar_transaksi: mutasi,
        }
    }

    /// Ekstraksi Buku Pembantu Bank
    pub fn buat_buku_pembantu_bank(&self) -> BukuPembantuBank {
        let mut total_terima = 0.0;
        let mut total_keluar = 0.0;
        let mut mutasi = Vec::new();

        for t in &self.daftar_transaksi {
            let mut relevant = false;
            match t.jenis_transaksi {
                JenisTransaksiBku::PenerimaanDanaTransfer | JenisTransaksiBku::BungaBank => {
                    total_terima += t.penerimaan_rp;
                    relevant = true;
                }
                JenisTransaksiBku::TarikTunaiDariBank => {
                    total_keluar += t.pengeluaran_rp;
                    relevant = true;
                }
                JenisTransaksiBku::SetorTunaiKeBank => {
                    total_terima += t.penerimaan_rp;
                    relevant = true;
                }
                JenisTransaksiBku::BelanjaOperasionalSIPLah
                | JenisTransaksiBku::PajakBungaDanBiayaAdminBank => {
                    total_keluar += t.pengeluaran_rp;
                    relevant = true;
                }
                JenisTransaksiBku::BelanjaNonSIPLah | JenisTransaksiBku::SetorPajakKeKasNegara => {
                    if t.metode_bayar != MetodePembayaranBku::KasTunaiBendahara {
                        total_keluar += t.pengeluaran_rp;
                        relevant = true;
                    }
                }
                _ => {}
            }

            if relevant {
                mutasi.push(t.clone());
            }
        }

        BukuPembantuBank {
            npsn_sekolah: self.npsn_sekolah.clone(),
            tahun_anggaran: self.tahun_anggaran,
            bulan_buku: self.bulan_buku,
            saldo_awal_rp: self.saldo_awal_bank_rp,
            total_penerimaan_rp: total_terima,
            total_pengeluaran_rp: total_keluar,
            saldo_akhir_rp: self.saldo_bank_saat_ini(),
            daftar_transaksi: mutasi,
        }
    }

    /// Ekstraksi Buku Pembantu Pajak
    pub fn buat_buku_pembantu_pajak(&self) -> BukuPembantuPajak {
        let mut pph21 = 0.0;
        let mut pph22 = 0.0;
        let mut pph23 = 0.0;
        let mut ppn11 = 0.0;
        let mut disetor = 0.0;

        for t in &self.daftar_transaksi {
            if let Some(ref p) = t.rincian_pajak {
                if t.jenis_transaksi == JenisTransaksiBku::SetorPajakKeKasNegara {
                    disetor += p.total_pajak();
                } else {
                    pph21 += p.pph_21_rp;
                    pph22 += p.pph_22_rp;
                    pph23 += p.pph_23_rp;
                    ppn11 += p.ppn_11_rp;
                }
            }
        }

        let dipungut = pph21 + pph22 + pph23 + ppn11;
        let sisa = (dipungut - disetor).max(0.0);

        BukuPembantuPajak {
            npsn_sekolah: self.npsn_sekolah.clone(),
            tahun_anggaran: self.tahun_anggaran,
            bulan_buku: self.bulan_buku,
            total_pph_21_rp: pph21,
            total_pph_22_rp: pph22,
            total_pph_23_rp: pph23,
            total_ppn_11_rp: ppn11,
            total_pajak_dipungut_rp: dipungut,
            total_pajak_disetor_rp: disetor,
            sisa_saldo_titipan_pajak_rp: sisa,
        }
    }

    /// Hitung Pajak Belanja Otomatis Berdasarkan Aturan Perpajakan Nasional (HPP & Juknis BOSP)
    pub fn hitung_pajak_belanja(nilai_bruto: f64, kategori: &str, ber_npwp: bool) -> RincianPajakTransaksi {
        let mut ppn = 0.0;
        let mut pph21 = 0.0;
        let mut pph22 = 0.0;
        let mut pph23 = 0.0;

        match kategori {
            "Barang" => {
                if nilai_bruto >= 2_000_000.0 {
                    ppn = (nilai_bruto * 0.11).round();
                    pph22 = if ber_npwp {
                        (nilai_bruto * 0.015).round()
                    } else {
                        (nilai_bruto * 0.03).round()
                    };
                }
            }
            "Jasa" => {
                if nilai_bruto >= 2_000_000.0 {
                    ppn = (nilai_bruto * 0.11).round();
                }
                pph23 = if ber_npwp {
                    (nilai_bruto * 0.02).round()
                } else {
                    (nilai_bruto * 0.04).round()
                };
            }
            "HonorPNS_Gol3" => {
                pph21 = (nilai_bruto * 0.05).round();
            }
            "HonorPNS_Gol4" => {
                pph21 = (nilai_bruto * 0.15).round();
            }
            "HonorNonPNS" => {
                pph21 = if ber_npwp {
                    (nilai_bruto * 0.05).round()
                } else {
                    (nilai_bruto * 0.06).round()
                };
            }
            _ => {}
        }

        RincianPajakTransaksi {
            pph_21_rp: pph21,
            pph_22_rp: pph22,
            pph_23_rp: pph23,
            ppn_11_rp: ppn,
            kode_billing_pajak: None,
            ntpn_bukti_setor: None,
        }
    }

    /// Tutup Buku Bulanan BKU
    pub fn tutup_buku(&mut self, tanggal: &str) {
        self.status_tutup_buku = true;
        self.tanggal_tutup_buku = Some(tanggal.to_string());
    }
}
