use super::rkas_anggaran::{DokumenRkasTahunan, ItemKegiatanRkas};
use super::sumber_dana::JenisSumberDana;
use super::bku_transaksi::{BukuKasUmumSekolah, JenisTransaksiBku, MetodePembayaranBku, RincianPajakTransaksi, TransaksiBkuEntry};
use serde::{Deserialize, Serialize};

/// ============================================================================
/// MODUL INTEGRASI SIPLah & PBJ SATUAN PENDIDIKAN (PERMENDIKDASMEN 15/2026)
/// 6 Dokumen PBJ: Rencana Belanja, Perbandingan Harga, Negosiasi, SP, BAST & Bayar
/// Terintegrasi Penuh dengan Pagu RKAS & Buku Kas Umum (BKU) ARKAS
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetodePbjSiplah {
    PembelianLangsung,      // Nilai s.d Rp 50 Juta
    PerbandinganHarga,      // Nilai Rp 50 Juta s.d Rp 200 Juta (Min. 2 Penyedia)
    NegosiasiHarga,         // Nilai > Rp 50 Juta atau Produk Tanpa Pembanding Identik
    TenderDaringSIPLah,     // Nilai > Rp 200 Juta (Pekerjaan Konstruksi / Paket Besar)
}

/// ----------------------------------------------------------------------------
/// DOKUMEN 1: DOKUMEN RENCANA BELANJA (DRB)
/// Diturunkan langsung dari kode kegiatan & pagu anggaran yang sah di RKAS
/// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemRencanaBelanja {
    pub id_item_rkas: String,
    pub nama_barang_jasa: String,
    pub spesifikasi_teknis: String,
    pub volume: f64,
    pub satuan: String,
    pub harga_satuan_hps_rp: f64,
    pub total_hps_rp: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DokumenRencanaBelanja {
    pub nomor_drb: String,
    pub npsn_sekolah: String,
    pub tanggal_drb: String,
    pub id_kegiatan_rkas: String,
    pub sumber_dana: JenisSumberDana,
    pub metode_pengadaan: MetodePbjSiplah,
    pub items: Vec<ItemRencanaBelanja>,
    pub total_perkiraan_biaya_rp: f64,
    pub nama_pejabat_pengadaan: String, // Kepala Satuan Pendidikan / Pelaksana PBJ
}

/// ----------------------------------------------------------------------------
/// DOKUMEN 2: DOKUMEN PERBANDINGAN HARGA (DPH)
/// Minimal 2 - 3 calon penyedia mitra SIPLah terdaftar
/// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PenawaranCalonPenyedia {
    pub id_penyedia: String,
    pub nama_toko_mitra: String,
    pub nama_marketplace_siplah: String,
    pub harga_satuan_penawaran_rp: f64,
    pub ongkos_kirim_rp: f64,
    pub total_biaya_rp: f64,
    pub estimasi_hari_kirim: u32,
    pub rating_reputasi: f64,
    pub terpilih_sebagai_pemenang: bool,
    pub alasan_pemilihan: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DokumenPerbandinganHarga {
    pub nomor_dph: String,
    pub nomor_drb: String,
    pub tanggal_perbandingan: String,
    pub daftar_penawaran: Vec<PenawaranCalonPenyedia>,
    pub penyedia_terpilih_id: String,
    pub kesimpulan_pemilihan: String,
}

/// ----------------------------------------------------------------------------
/// DOKUMEN 3: DOKUMEN NEGOSIASI HARGA (DNH)
/// Riwayat tawar-menawar harga dan kesepakatan final
/// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiwayatTawarMenawar {
    pub putaran_ke: u32,
    pub pengaju: String, // "Sekolah" atau "Penyedia"
    pub harga_diajukan_rp: f64,
    pub catatan_negosiasi: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DokumenNegosiasiHarga {
    pub nomor_dnh: String,
    pub nomor_drb: String,
    pub id_penyedia: String,
    pub tanggal_negosiasi: String,
    pub harga_awal_penyedia_rp: f64,
    pub riwayat_tawar: Vec<RiwayatTawarMenawar>,
    pub harga_kesepakatan_final_rp: f64,
    pub efisiensi_penghematan_rp: f64,
    pub status_kesepakatan: bool,
}

/// ----------------------------------------------------------------------------
/// DOKUMEN 4: SURAT PESANAN (SP / PO DIGITAL SIPLAH)
/// Kontrak perikatan elektronik resmi sekolah dengan mitra penyedia
/// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemSuratPesanan {
    pub id_item: String,
    pub nama_barang_jasa: String,
    pub spesifikasi: String,
    pub volume: f64,
    pub satuan: String,
    pub harga_satuan_rp: f64,
    pub subtotal_rp: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuratPesananSiplahPermen15 {
    pub nomor_surat_pesanan: String,
    pub nomor_drb: String,
    pub npsn_sekolah: String,
    pub nama_sekolah: String,
    pub id_penyedia: String,
    pub nama_penyedia: String,
    pub npwp_penyedia: String,
    pub virtual_account_pembayaran: String,
    pub tanggal_surat_pesanan: String,
    pub batas_akhir_pengiriman: String,
    pub items: Vec<ItemSuratPesanan>,
    pub total_nilai_pesanan_rp: f64,
    pub ongkos_kirim_rp: f64,
    pub grand_total_rp: f64,
    pub denda_keterlambatan_per_mil: f64, // Standar Permendikdasmen: 1/1000 per hari
    pub status_pesanan: String, // "DITERBITKAN", "DIPROSES_PENYEDIA", "DIKIRIM", "SELESAI_BAST"
}

/// ----------------------------------------------------------------------------
/// DOKUMEN 5: BERITA ACARA SERAH TERIMA (BAST DIGITAL) & UJI TERIMA
/// Pemeriksaan fisik, kesesuaian spek, kelayakan fungsi & tanda tangan elektronik
/// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UjiKualitasFisikBarang {
    pub kesesuaian_spesifikasi: bool,
    pub kuantitas_sesuai_sp: bool,
    pub kondisi_fisik_baik_tanpa_cacat: bool,
    pub uji_fungsi_alat_berjalan: bool,
    pub masa_garansi_bulan: u32,
    pub catatan_pemeriksaan: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BastDigitalSiplahPermen15 {
    pub nomor_bast: String,
    pub nomor_surat_pesanan: String,
    pub tanggal_penerimaan: String,
    pub nama_pemeriksa_sekolah: String,
    pub nama_kurir_penyedia: String,
    pub hasil_uji: UjiKualitasFisikBarang,
    pub status_barang_diterima_lengkap: bool,
    pub tanda_tangan_elektronik_sah: bool,
}

/// ----------------------------------------------------------------------------
/// DOKUMEN 6: BUKTI PEMBAYARAN & PELUNASAN KAS BKU ARKAS
/// Transfer dana via rekening escrow SIPLah & pemungutan pajak otomatis
/// ----------------------------------------------------------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuktiPembayaranSiplah {
    pub nomor_bukti_bayar: String,
    pub nomor_surat_pesanan: String,
    pub nomor_bast: String,
    pub tanggal_bayar: String,
    pub bank_pengirim: String,
    pub nomor_rekening_sekolah: String,
    pub virtual_account_tujuan: String,
    pub nominal_bruto_rp: f64,
    pub rincian_pajak: RincianPajakTransaksi,
    pub nominal_transfer_netto_rp: f64,
    pub nomor_referensi_bank: String,
    pub status_pelunasan_escrow: String, // "LUNAS_TERVERIFIKASI"
}

/// ============================================================================
/// ENGINE ORKESTRASI INTEGRASI PBJ SIPLAH KE ARKAS (PERMENDIKDASMEN 15/2026)
/// ============================================================================

pub struct SiplahPbjEngine {
    pub nama_mitra_platform: String,
    pub daftar_drb: Vec<DokumenRencanaBelanja>,
    pub daftar_dph: Vec<DokumenPerbandinganHarga>,
    pub daftar_dnh: Vec<DokumenNegosiasiHarga>,
    pub daftar_sp: Vec<SuratPesananSiplahPermen15>,
    pub daftar_bast: Vec<BastDigitalSiplahPermen15>,
    pub daftar_bukti_bayar: Vec<BuktiPembayaranSiplah>,
}

impl SiplahPbjEngine {
    pub fn new(nama_platform: &str) -> Self {
        Self {
            nama_mitra_platform: nama_platform.to_string(),
            daftar_drb: Vec::new(),
            daftar_dph: Vec::new(),
            daftar_dnh: Vec::new(),
            daftar_sp: Vec::new(),
            daftar_bast: Vec::new(),
            daftar_bukti_bayar: Vec::new(),
        }
    }

    /// [Dokumen 1] Buat Dokumen Rencana Belanja (DRB) dari Pagu RKAS
    pub fn buat_rencana_belanja(
        &mut self,
        nomor_drb: &str,
        rkas: &DokumenRkasTahunan,
        id_kegiatan: &str,
        tanggal: &str,
        nama_pejabat: &str,
    ) -> Result<DokumenRencanaBelanja, String> {
        let kegiatan = rkas.daftar_kegiatan.iter().find(|k| k.id_kegiatan == id_kegiatan)
            .ok_or_else(|| format!("Kegiatan RKAS '{}' tidak ditemukan!", id_kegiatan))?;

        let metode = if kegiatan.total_anggaran_tahunan_rp <= 50_000_000.0 {
            MetodePbjSiplah::PembelianLangsung
        } else if kegiatan.total_anggaran_tahunan_rp <= 200_000_000.0 {
            MetodePbjSiplah::PerbandinganHarga
        } else {
            MetodePbjSiplah::TenderDaringSIPLah
        };

        let item_drb = ItemRencanaBelanja {
            id_item_rkas: kegiatan.id_kegiatan.clone(),
            nama_barang_jasa: kegiatan.uraian_kegiatan.clone(),
            spesifikasi_teknis: format!("Standar SNP: {:?}, Kode Rekening: {}", kegiatan.standar_snp, kegiatan.kode_rekening),
            volume: kegiatan.volume,
            satuan: kegiatan.satuan.clone(),
            harga_satuan_hps_rp: kegiatan.tarif_harga_satuan_rp,
            total_hps_rp: kegiatan.total_anggaran_tahunan_rp,
        };

        let drb = DokumenRencanaBelanja {
            nomor_drb: nomor_drb.to_string(),
            npsn_sekolah: rkas.npsn_sekolah.clone(),
            tanggal_drb: tanggal.to_string(),
            id_kegiatan_rkas: id_kegiatan.to_string(),
            sumber_dana: kegiatan.sumber_dana.clone(),
            metode_pengadaan: metode,
            items: vec![item_drb],
            total_perkiraan_biaya_rp: kegiatan.total_anggaran_tahunan_rp,
            nama_pejabat_pengadaan: nama_pejabat.to_string(),
        };

        self.daftar_drb.push(drb.clone());
        Ok(drb)
    }

    /// [Dokumen 2] Catat Dokumen Perbandingan Harga (DPH) minimal 2-3 penyedia
    pub fn catat_perbandingan_harga(
        &mut self,
        nomor_dph: &str,
        nomor_drb: &str,
        tanggal: &str,
        penawaran: Vec<PenawaranCalonPenyedia>,
    ) -> Result<DokumenPerbandinganHarga, String> {
        if penawaran.is_empty() {
            return Err("Minimal 1 penawaran untuk pembelian langsung atau 2 pembanding untuk DPH!".to_string());
        }

        let terpilih = penawaran.iter().find(|p| p.terpilih_sebagai_pemenang)
            .ok_or_else(|| "Belum ada penyedia yang ditetapkan sebagai pemenang!".to_string())?;

        let dph = DokumenPerbandinganHarga {
            nomor_dph: nomor_dph.to_string(),
            nomor_drb: nomor_drb.to_string(),
            tanggal_perbandingan: tanggal.to_string(),
            penyedia_terpilih_id: terpilih.id_penyedia.clone(),
            kesimpulan_pemilihan: format!("Penyedia '{}' terpilih karena {}", terpilih.nama_toko_mitra, terpilih.alasan_pemilihan),
            daftar_penawaran: penawaran,
        };

        self.daftar_dph.push(dph.clone());
        Ok(dph)
    }

    /// [Dokumen 3] Catat Dokumen Negosiasi Harga (DNH)
    pub fn catat_negosiasi_harga(
        &mut self,
        nomor_dnh: &str,
        nomor_drb: &str,
        id_penyedia: &str,
        tanggal: &str,
        harga_awal: f64,
        harga_sepakat: f64,
        riwayat: Vec<RiwayatTawarMenawar>,
    ) -> DokumenNegosiasiHarga {
        let efisiensi = harga_awal - harga_sepakat;
        let dnh = DokumenNegosiasiHarga {
            nomor_dnh: nomor_dnh.to_string(),
            nomor_drb: nomor_drb.to_string(),
            id_penyedia: id_penyedia.to_string(),
            tanggal_negosiasi: tanggal.to_string(),
            harga_awal_penyedia_rp: harga_awal,
            riwayat_tawar: riwayat,
            harga_kesepakatan_final_rp: harga_sepakat,
            efisiensi_penghematan_rp: if efisiensi > 0.0 { efisiensi } else { 0.0 },
            status_kesepakatan: true,
        };

        self.daftar_dnh.push(dnh.clone());
        dnh
    }

    /// [Dokumen 4] Penerbitan Surat Pesanan (SP Digital) SIPLah
    pub fn terbitkan_surat_pesanan(
        &mut self,
        nomor_sp: &str,
        drb: &DokumenRencanaBelanja,
        id_penyedia: &str,
        nama_penyedia: &str,
        npwp_penyedia: &str,
        va_tujuan: &str,
        tanggal: &str,
        batas_kirim: &str,
        harga_final: f64,
        ongkir: f64,
    ) -> SuratPesananSiplahPermen15 {
        let item_sp = ItemSuratPesanan {
            id_item: drb.items[0].id_item_rkas.clone(),
            nama_barang_jasa: drb.items[0].nama_barang_jasa.clone(),
            spesifikasi: drb.items[0].spesifikasi_teknis.clone(),
            volume: drb.items[0].volume,
            satuan: drb.items[0].satuan.clone(),
            harga_satuan_rp: harga_final / drb.items[0].volume,
            subtotal_rp: harga_final,
        };

        let grand_total = harga_final + ongkir;

        let sp = SuratPesananSiplahPermen15 {
            nomor_surat_pesanan: nomor_sp.to_string(),
            nomor_drb: drb.nomor_drb.clone(),
            npsn_sekolah: drb.npsn_sekolah.clone(),
            nama_sekolah: "Satuan Pendidikan Pengguna SIPLah".to_string(),
            id_penyedia: id_penyedia.to_string(),
            nama_penyedia: nama_penyedia.to_string(),
            npwp_penyedia: npwp_penyedia.to_string(),
            virtual_account_pembayaran: va_tujuan.to_string(),
            tanggal_surat_pesanan: tanggal.to_string(),
            batas_akhir_pengiriman: batas_kirim.to_string(),
            items: vec![item_sp],
            total_nilai_pesanan_rp: harga_final,
            ongkos_kirim_rp: ongkir,
            grand_total_rp: grand_total,
            denda_keterlambatan_per_mil: 1.0,
            status_pesanan: "DITERBITKAN".to_string(),
        };

        self.daftar_sp.push(sp.clone());
        sp
    }

    /// [Dokumen 5] Penerbitan BAST Digital & Uji Terima Barang
    pub fn proses_bast_digital(
        &mut self,
        nomor_bast: &str,
        nomor_sp: &str,
        tanggal: &str,
        nama_pemeriksa: &str,
        nama_kurir: &str,
        hasil_uji: UjiKualitasFisikBarang,
    ) -> Result<BastDigitalSiplahPermen15, String> {
        let sp = self.daftar_sp.iter_mut().find(|sp| sp.nomor_surat_pesanan == nomor_sp)
            .ok_or_else(|| format!("Surat Pesanan '{}' tidak ditemukan!", nomor_sp))?;

        if !hasil_uji.kesesuaian_spesifikasi || !hasil_uji.kuantitas_sesuai_sp {
            return Err("Barang tidak sesuai spesifikasi atau kuantitas SP!".to_string());
        }

        sp.status_pesanan = "SELESAI_BAST".to_string();

        let bast = BastDigitalSiplahPermen15 {
            nomor_bast: nomor_bast.to_string(),
            nomor_surat_pesanan: nomor_sp.to_string(),
            tanggal_penerimaan: tanggal.to_string(),
            nama_pemeriksa_sekolah: nama_pemeriksa.to_string(),
            nama_kurir_penyedia: nama_kurir.to_string(),
            hasil_uji,
            status_barang_diterima_lengkap: true,
            tanda_tangan_elektronik_sah: true,
        };

        self.daftar_bast.push(bast.clone());
        Ok(bast)
    }

    /// [Dokumen 6] Pembayaran Pelunasan & Posting Otomatis ke BKU ARKAS
    pub fn bayar_dan_posting_bku(
        &mut self,
        nomor_bukti_bayar: &str,
        nomor_sp: &str,
        nomor_bast: &str,
        tanggal: &str,
        bku: &mut BukuKasUmumSekolah,
        sumber_dana: JenisSumberDana,
        id_kegiatan_rkas: &str,
    ) -> Result<BuktiPembayaranSiplah, String> {
        let sp = self.daftar_sp.iter().find(|sp| sp.nomor_surat_pesanan == nomor_sp)
            .ok_or_else(|| format!("Surat Pesanan '{}' tidak ditemukan!", nomor_sp))?;

        // Kalkulasi Pajak PBJ Permendikdasmen 15/2026 & PMK:
        // PPh 22 = 0.5% (rekanan ber-NPWP), PPN = 11%
        let pph_22 = sp.total_nilai_pesanan_rp * 0.005;
        let ppn_11 = sp.total_nilai_pesanan_rp * 0.11;

        // Catat pengeluaran di Buku Kas Umum (BKU)
        bku.catat_belanja_siplah(
            tanggal,
            &sp.nomor_surat_pesanan,
            nomor_bast,
            &format!("Pembayaran Belanja SIPLah Permen 15/2026: {}", sp.items[0].nama_barang_jasa),
            sumber_dana,
            id_kegiatan_rkas,
            sp.grand_total_rp,
            pph_22,
            ppn_11,
        )?;

        let bukti = BuktiPembayaranSiplah {
            nomor_bukti_bayar: nomor_bukti_bayar.to_string(),
            nomor_surat_pesanan: nomor_sp.to_string(),
            nomor_bast: nomor_bast.to_string(),
            tanggal_bayar: tanggal.to_string(),
            bank_pengirim: "Bank Penyalur BOSP (Giro Satdik)".to_string(),
            nomor_rekening_sekolah: bku.npsn_sekolah.clone(),
            virtual_account_tujuan: sp.virtual_account_pembayaran.clone(),
            nominal_bruto_rp: sp.grand_total_rp,
            rincian_pajak: RincianPajakTransaksi {
                pph_21_rp: 0.0,
                pph_22_rp: pph_22,
                pph_23_rp: 0.0,
                ppn_11_rp: ppn_11,
                kode_billing_pajak: Some(format!("BILLING-{}", sp.nomor_surat_pesanan)),
                ntpn_bukti_setor: Some(format!("NTPN-{}", sp.nomor_surat_pesanan)),
            },
            nominal_transfer_netto_rp: sp.grand_total_rp,
            nomor_referensi_bank: format!("TRX-SIPLah-{}", nomor_bukti_bayar),
            status_pelunasan_escrow: "LUNAS_TERVERIFIKASI".to_string(),
        };

        self.daftar_bukti_bayar.push(bukti.clone());
        Ok(bukti)
    }
}
