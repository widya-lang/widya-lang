use serde::{Deserialize, Serialize};

/// ============================================================================
/// MARKETPLACE SIPLah & E-PENGADAAN BAHAN MAKANAN PROGRAM MBG / PENDIDIKAN
/// Sistem Informasi Pengadaan Sekolah / Dapur SPPG MBG untuk Penyedia Pangan
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KategoriBahanPangan {
    BerasDanKarbohidrat,
    ProteinAyamDanUnggas,
    ProteinDagingSapi,
    ProteinIkanDanSeafood,
    TelurAyamSegar,
    SayurMayurSegar,
    BuahBuahanLokal,
    SusuSegarDanOlahan,
    BumbuDapurDanMinyak,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusVerifikasiPenyedia {
    MenungguKurasi,
    TerverifikasiSIPLah,
    SertifikasiLengkap, // Halal + Higiene Sanitasi Pangan
    Ditangguhkan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalitasPenyedia {
    pub nib: String,
    pub npwp: String,
    pub nomor_sertifikat_halal: Option<String>,
    pub sertifikat_laik_higiene: Option<String>,
    pub rekening_bank: String,
    pub nama_bank: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PenyediaBahanMakanan {
    pub id_penyedia: String,
    pub nama_usaha: String,
    pub jenis_badan_usaha: String, // e.g. "Koperasi Desa Merah Putih", "Kelompok Tani", "CV", "PT"
    pub nama_pemilik: String,
    pub alamat: String,
    pub desa_kelurahan: String,
    pub kabupaten_kota: String,
    pub radius_antar_km: f64,
    pub legalitas: LegalitasPenyedia,
    pub status: StatusVerifikasiPenyedia,
    pub rating_reputasi: f64, // Skala 1.0 - 5.0
    pub total_transaksi_selesai: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProdukPanganSIPLah {
    pub id_produk: String,
    pub id_penyedia: String,
    pub nama_produk: String,
    pub kategori: KategoriBahanPangan,
    pub satuan: String, // "Kg", "Liter", "Tray (30 butir)", "Ikat"
    pub harga_satuan_rp: f64,
    pub stok_tersedia: f64,
    pub umur_simpan_hari: u32,
    pub suhu_penyimpanan_celsius: f64, // e.g. 4.0 untuk daging dingin, 25.0 untuk beras
    pub asal_komoditas_lokal: String, // e.g. "Petani Padi Desa Sukamaju"
    pub kandungan_gizi_per_100g: Option<KandunganGiziSingkat>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KandunganGiziSingkat {
    pub kalori_kkal: f64,
    pub protein_gram: f64,
    pub karbohidrat_gram: f64,
    pub lemak_gram: f64,
    pub serat_gram: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPesananPangan {
    pub id_produk: String,
    pub nama_produk: String,
    pub jumlah_satuan: f64,
    pub harga_satuan_rp: f64,
    pub subtotal_rp: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusPesananSIPLah {
    DibuatMenungguPersetujuan,
    TerkonfirmasiPenyedia,
    DalamPengirimanColdChain,
    TibaDiDapurSPPG,
    SelesaiBASTDiverifikasi,
    Dibatalkan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuratPesananSIPLah {
    pub nomor_surat_pesanan: String,
    pub id_pemesan: String, // ID Dapur SPPG / Sekolah / Yayasan
    pub nama_pemesan: String,
    pub id_penyedia: String,
    pub tanggal_pesan: String,
    pub tanggal_rencana_kirim: String,
    pub items: Vec<ItemPesananPangan>,
    pub total_nilai_bruto_rp: f64,
    pub tarif_ppn: f64, // 0.0 jika bahan pokok strategis bebas PPN, atau 0.11
    pub nilai_ppn_rp: f64,
    pub tarif_pph_22: f64, // 0.005 (0.5%) untuk rekanan SIPLah ber-NPWP
    pub nilai_pph_22_rp: f64,
    pub total_tagihan_netto_rp: f64,
    pub status: StatusPesananSIPLah,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UjiKualitasTerimaPangan {
    pub parameter_kesegaran: bool, // Bau, Warna, Tekstur memenuhi syarat
    pub suhu_saat_diterima_celsius: f64,
    pub berat_aktual_timbangan_kg: f64,
    pub selisih_berat_persen: f64,
    pub catatan_pemeriksaan: String,
    pub hasil_layak_konsumsi: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BastDigitalSIPLah {
    pub nomor_bast: String,
    pub nomor_surat_pesanan: String,
    pub tanggal_serah_terima: String,
    pub id_petugas_penerima_sppg: String,
    pub nama_petugas_penerima: String,
    pub id_kurir_penyedia: String,
    pub nama_kurir_penyedia: String,
    pub uji_kualitas: UjiKualitasTerimaPangan,
    pub tanda_tangan_elektronik_sah: bool,
    pub status_pembayaran_escrow: String, // "DanaDiteruskanKePenyedia", "DitahanInvestigasi"
}

/// ============================================================================
/// MARKETPLACE ENGINE SIPLAH PANGAN MBG
/// ============================================================================

pub struct SIPLahPanganEngine {
    pub nama_marketplace: String,
    pub penyedia_list: Vec<PenyediaBahanMakanan>,
    pub katalog_produk: Vec<ProdukPanganSIPLah>,
    pub pesanan_list: Vec<SuratPesananSIPLah>,
    pub bast_list: Vec<BastDigitalSIPLah>,
}

impl SIPLahPanganEngine {
    pub fn new(nama_marketplace: &str) -> Self {
        Self {
            nama_marketplace: nama_marketplace.to_string(),
            penyedia_list: Vec::new(),
            katalog_produk: Vec::new(),
            pesanan_list: Vec::new(),
            bast_list: Vec::new(),
        }
    }

    /// Daftarkan penyedia bahan makanan / mitra lokal
    pub fn daftarkan_penyedia(&mut self, penyedia: PenyediaBahanMakanan) {
        self.penyedia_list.push(penyedia);
    }

    /// Kurasi & verifikasi legalitas penyedia
    pub fn verifikasi_penyedia(&mut self, id_penyedia: &str) -> bool {
        if let Some(p) = self.penyedia_list.iter_mut().find(|p| p.id_penyedia == id_penyedia) {
            if p.legalitas.nomor_sertifikat_halal.is_some() && p.legalitas.sertifikat_laik_higiene.is_some() {
                p.status = StatusVerifikasiPenyedia::SertifikasiLengkap;
            } else if !p.legalitas.nib.is_empty() && !p.legalitas.npwp.is_empty() {
                p.status = StatusVerifikasiPenyedia::TerverifikasiSIPLah;
            }
            true
        } else {
            false
        }
    }

    /// Tambah produk ke katalog SIPLah
    pub fn tambah_produk_katalog(&mut self, produk: ProdukPanganSIPLah) -> Result<(), String> {
        let penyedia = self.penyedia_list.iter().find(|p| p.id_penyedia == produk.id_penyedia);
        match penyedia {
            Some(p) => {
                if p.status == StatusVerifikasiPenyedia::Ditangguhkan || p.status == StatusVerifikasiPenyedia::MenungguKurasi {
                    return Err("Penyedia belum terverifikasi di SIPLah!".to_string());
                }
                self.katalog_produk.push(produk);
                Ok(())
            }
            None => Err("ID Penyedia tidak ditemukan!".to_string()),
        }
    }

    /// Cari komoditas pangan berdasarkan kategori dan radius jangkauan
    pub fn cari_bahan_pangan(
        &self,
        kategori: KategoriBahanPangan,
        max_jarak_km: f64,
    ) -> Vec<(&ProdukPanganSIPLah, &PenyediaBahanMakanan)> {
        let mut hasil = Vec::new();
        for prod in &self.katalog_produk {
            if prod.kategori == kategori && prod.stok_tersedia > 0.0 {
                if let Some(penyedia) = self.penyedia_list.iter().find(|p| p.id_penyedia == prod.id_penyedia) {
                    if penyedia.radius_antar_km >= max_jarak_km {
                        hasil.push((prod, penyedia));
                    }
                }
            }
        }
        hasil
    }

    /// Buat Surat Pesanan (SP) Pengadaan Pangan Sekolah / Dapur SPPG
    pub fn buat_surat_pesanan(
        &mut self,
        nomor_sp: &str,
        id_pemesan: &str,
        nama_pemesan: &str,
        id_penyedia: &str,
        tanggal_pesan: &str,
        tanggal_kirim: &str,
        item_pesanan: Vec<(&str, f64)>, // (id_produk, jumlah)
    ) -> Result<SuratPesananSIPLah, String> {
        let mut items = Vec::new();
        let mut subtotal_kumulatif = 0.0;

        for (id_prod, qty) in item_pesanan {
            let prod = self.katalog_produk.iter().find(|p| p.id_produk == id_prod)
                .ok_or_else(|| format!("Produk {} tidak ditemukan di katalog", id_prod))?;

            if prod.stok_tersedia < qty {
                return Err(format!("Stok produk {} tidak mencukupi (Tersedia: {}, Diminta: {})", prod.nama_produk, prod.stok_tersedia, qty));
            }

            let subtotal = prod.harga_satuan_rp * qty;
            subtotal_kumulatif += subtotal;

            items.push(ItemPesananPangan {
                id_produk: prod.id_produk.clone(),
                nama_produk: prod.nama_produk.clone(),
                jumlah_satuan: qty,
                harga_satuan_rp: prod.harga_satuan_rp,
                subtotal_rp: subtotal,
            });
        }

        // Pajak SIPLah: Bahan pangan pokok mentah bebas PPN (0%), PPh 22 rekanan 0.5%
        let tarif_ppn = 0.0;
        let nilai_ppn = subtotal_kumulatif * tarif_ppn;
        let tarif_pph_22 = 0.005;
        let nilai_pph_22 = subtotal_kumulatif * tarif_pph_22;
        let total_tagihan = subtotal_kumulatif + nilai_ppn; // PPh dipotong saat pencairan

        // Potong stok
        for item in &items {
            if let Some(p) = self.katalog_produk.iter_mut().find(|p| p.id_produk == item.id_produk) {
                p.stok_tersedia -= item.jumlah_satuan;
            }
        }

        let sp = SuratPesananSIPLah {
            nomor_surat_pesanan: nomor_sp.to_string(),
            id_pemesan: id_pemesan.to_string(),
            nama_pemesan: nama_pemesan.to_string(),
            id_penyedia: id_penyedia.to_string(),
            tanggal_pesan: tanggal_pesan.to_string(),
            tanggal_rencana_kirim: tanggal_kirim.to_string(),
            items,
            total_nilai_bruto_rp: subtotal_kumulatif,
            tarif_ppn,
            nilai_ppn_rp: nilai_ppn,
            tarif_pph_22,
            nilai_pph_22_rp: nilai_pph_22,
            total_tagihan_netto_rp: total_tagihan,
            status: StatusPesananSIPLah::TerkonfirmasiPenyedia,
        };

        self.pesanan_list.push(sp.clone());
        Ok(sp)
    }

    /// Penerimaan Bahan & Penerbitan BAST Digital di Dapur SPPG
    pub fn proses_bast_penerimaan(
        &mut self,
        nomor_bast: &str,
        nomor_sp: &str,
        petugas_penerima_id: &str,
        nama_petugas: &str,
        kurir_id: &str,
        nama_kurir: &str,
        tanggal_serah_terima: &str,
        suhu_terima_c: f64,
        berat_aktual_kg: f64,
        berat_pesanan_kg: f64,
    ) -> Result<BastDigitalSIPLah, String> {
        let sp = self.pesanan_list.iter_mut().find(|sp| sp.nomor_surat_pesanan == nomor_sp)
            .ok_or_else(|| "Surat Pesanan tidak ditemukan!".to_string())?;

        let selisih_persen = ((berat_aktual_kg - berat_pesanan_kg) / berat_pesanan_kg) * 100.0;
        let layak = selisih_persen.abs() <= 5.0 && suhu_terima_c <= 8.0; // Toleransi berat 5%, suhu cold chain max 8C

        let uji = UjiKualitasTerimaPangan {
            parameter_kesegaran: true,
            suhu_saat_diterima_celsius: suhu_terima_c,
            berat_aktual_timbangan_kg: berat_aktual_kg,
            selisih_berat_persen: selisih_persen,
            catatan_pemeriksaan: if layak {
                "Bahan segar, bau normal, suhu sesuai rantai dingin, berat akurat".to_string()
            } else {
                "Suhu/berat menyimpang dari standar kualitas".to_string()
            },
            hasil_layak_konsumsi: layak,
        };

        let status_escrow = if layak {
            sp.status = StatusPesananSIPLah::SelesaiBASTDiverifikasi;
            // Update transaksi selesai penyedia
            if let Some(penyedia) = self.penyedia_list.iter_mut().find(|p| p.id_penyedia == sp.id_penyedia) {
                penyedia.total_transaksi_selesai += 1;
            }
            "DanaDiteruskanKePenyedia".to_string()
        } else {
            "DitahanInvestigasi".to_string()
        };

        let bast = BastDigitalSIPLah {
            nomor_bast: nomor_bast.to_string(),
            nomor_surat_pesanan: nomor_sp.to_string(),
            tanggal_serah_terima: tanggal_serah_terima.to_string(),
            id_petugas_penerima_sppg: petugas_penerima_id.to_string(),
            nama_petugas_penerima: nama_petugas.to_string(),
            id_kurir_penyedia: kurir_id.to_string(),
            nama_kurir_penyedia: nama_kurir.to_string(),
            uji_kualitas: uji,
            tanda_tangan_elektronik_sah: true,
            status_pembayaran_escrow: status_escrow,
        };

        self.bast_list.push(bast.clone());
        Ok(bast)
    }
}
