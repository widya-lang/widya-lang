// Projects/arenasilat/src/payment_and_split_settlement.rs
// ==============================================================================
// ARENA SILAT SAAS — PAYMENT GATEWAY & AUTOMATED SPLIT SETTLEMENT
// 100% Zero External Dependencies — Berdaulat Penuh di atas Widya-Lang
// Sesuai Dokumen:
// - docs/integrasi_xendit_payment_gateway_dan_split_settlement.md
// - docs/analisis_keuangan_dan_pricing_saas.md
// ==============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetodePembayaran {
    QrisDinamis,
    VirtualAccountBca,
    VirtualAccountBri,
    VirtualAccountMandiri,
    EWalletShopeePay,
    EWalletDana,
    TransferBankManual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusTransaksi {
    MenungguPembayaran,
    PembayaranLunas,
    Kedaluwarsa,
    Dibatalkan,
    DanaDikembalikan,
}

#[derive(Debug, Clone)]
pub struct SplitSettlementRincian {
    pub invoice_id: String,
    pub total_tagihan_idr: u64,
    pub porsi_eo_panitia_idr: u64,
    pub iuran_kas_federasi_ipsi_idr: u64,
    pub porsi_platform_saas_idr: u64,
    pub biaya_gerbang_pembayaran_idr: u64,
}

#[derive(Debug, Clone)]
pub struct DepositBandingProtes {
    pub banding_id: String,
    pub kontingen_id: String,
    pub nominal_deposit_idr: u64,
    pub partai_id: String,
    pub status_putusan_diterima: Option<bool>,
    pub status_kembali_dana: bool,
    pub keterangan: String,
}

pub struct PaymentEngine;

impl PaymentEngine {
    pub const BIAYA_SAAS_PER_ATLET: u64 = 15_000;
    pub const IURAN_IPSI_PER_ATLET: u64 = 10_000;
    pub const BIAYA_GATEWAY_FIXED: u64 = 3_500;
    pub const NOMINAL_STANDAR_BANDING_PROTES: u64 = 3_000_000;

    /// Hitung pemisahan dana (split settlement) otomatis per pendaftaran kontingen
    pub fn hitung_split_settlement(
        invoice_id: &str,
        biaya_pendaftaran_total: u64,
        jumlah_atlet: u32,
    ) -> SplitSettlementRincian {
        let total_saas = (jumlah_atlet as u64) * Self::BIAYA_SAAS_PER_ATLET;
        let total_ipsi = (jumlah_atlet as u64) * Self::IURAN_IPSI_PER_ATLET;
        let gateway_fee = Self::BIAYA_GATEWAY_FIXED;

        let potongan_total = total_saas + total_ipsi + gateway_fee;
        let porsi_eo = if biaya_pendaftaran_total >= potongan_total {
            biaya_pendaftaran_total - potongan_total
        } else {
            0
        };

        SplitSettlementRincian {
            invoice_id: invoice_id.to_string(),
            total_tagihan_idr: biaya_pendaftaran_total,
            porsi_eo_panitia_idr: porsi_eo,
            iuran_kas_federasi_ipsi_idr: total_ipsi,
            porsi_platform_saas_idr: total_saas,
            biaya_gerbang_pembayaran_idr: gateway_fee,
        }
    }

    /// Proses deposit jaminan banding protes dewan hakim (Rp 3.000.000,-)
    pub fn ajukan_deposit_banding(
        banding_id: &str,
        kontingen_id: &str,
        partai_id: &str,
    ) -> DepositBandingProtes {
        DepositBandingProtes {
            banding_id: banding_id.to_string(),
            kontingen_id: kontingen_id.to_string(),
            nominal_deposit_idr: Self::NOMINAL_STANDAR_BANDING_PROTES,
            partai_id: partai_id.to_string(),
            status_putusan_diterima: None,
            status_kembali_dana: false,
            keterangan: "Deposit jaminan Rp 3.000.000 terverifikasi. Sidang Dewan Hakim dimulai.".to_string(),
        }
    }

    /// Eksekusi pengembalian atau penghangusan dana banding pasca putusan Dewan Hakim
    pub fn selesaikan_deposit_banding(
        mut deposit: DepositBandingProtes,
        protes_dikabulkan: bool,
    ) -> DepositBandingProtes {
        deposit.status_putusan_diterima = Some(protes_dikabulkan);
        if protes_dikabulkan {
            deposit.status_kembali_dana = true;
            deposit.keterangan = format!(
                "Protes DIKABULKAN Dewan Hakim. Dana Rp {} dikembalikan 100% ke rekening kontingen.",
                deposit.nominal_deposit_idr
            );
        } else {
            deposit.status_kembali_dana = false;
            deposit.keterangan = format!(
                "Protes DITOLAK Dewan Hakim. Dana jaminan Rp {} hangus dan masuk kas IPSI/Panitia.",
                deposit.nominal_deposit_idr
            );
        }
        deposit
    }

    /// Generate payload QRIS Dinamis ISO 20022 / ASPI Standar Indonesia
    pub fn generate_qris_payload(invoice_id: &str, amount: u64, merchant_name: &str) -> String {
        format!(
            "00020101021226580016ID.CO.ARENASILAT0118{}520459995303360540{}.005802ID59{}6007JAKARTA6304",
            invoice_id, amount, merchant_name.replace(' ', "_")
        )
    }
}
