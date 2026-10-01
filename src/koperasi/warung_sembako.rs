use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 3. UNIT WARUNG SEMBAKO & RITEL SEMBILAN BAHAN POKOK (POS KOPERASI)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KategoriKomoditasSembako {
    Beras,
    MinyakGoreng,
    GulaPasir,
    TelurAyam,
    TepungTerigu,
    LpgBersubsidi3Kg,
    DagingDanIkan,
    GaramDanBumbu,
    KebutuhanHarianLainnya,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSembako {
    pub sku_barcode: String,
    pub nama_barang: String,
    pub kategori: KategoriKomoditasSembako,
    pub satuan: String, // "kg", "liter", "tabung", "butir", "pack"
    pub harga_beli_idr: u64,
    pub harga_jual_anggota_idr: u64,  // Harga diskon khusus anggota koperasi
    pub harga_jual_umum_idr: u64,
    pub stok_tersedia: f64,
    pub wajib_nik_subsidi: bool,      // true untuk LPG 3kg & Minyakita subsidi
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemKeranjang {
    pub sku_barcode: String,
    pub nama_barang: String,
    pub kuantitas: f64,
    pub harga_satuan_idr: u64,
    pub subtotal_idr: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrukTransaksiSembako {
    pub nomor_struk: String,
    pub nomor_pokok_anggota: Option<String>,
    pub nik_pembeli_subsidi: Option<String>,
    pub items: Vec<ItemKeranjang>,
    pub total_belanja_idr: u64,
    pub metode_pembayaran: String, // "TUNAI", "QRIS_MERAH_PUTIH", "POTONG_SIMPANAN", "PAYLATER"
    pub poin_anggota_diperoleh: u32,
}

pub struct UnitWarungSembako {
    pub inventaris: HashMap<String, ItemSembako>,
    pub riwayat_transaksi: Vec<StrukTransaksiSembako>,
}

impl UnitWarungSembako {
    pub fn new() -> Self {
        let mut warung = Self {
            inventaris: HashMap::new(),
            riwayat_transaksi: Vec::new(),
        };

        // Standard default inventory
        warung.tambah_item(
            "8991234001",
            "Beras Premium Desa Merah Putih 5kg",
            KategoriKomoditasSembako::Beras,
            "karung",
            65000,
            68000,
            72000,
            200.0,
            false,
        );

        warung.tambah_item(
            "8991234002",
            "Minyakita Kemasan 1 Liter",
            KategoriKomoditasSembako::MinyakGoreng,
            "liter",
            13500,
            14000,
            15000,
            500.0,
            true, // Subsidi
        );

        warung.tambah_item(
            "8991234003",
            "Gas LPG 3kg Melon Bersubsidi",
            KategoriKomoditasSembako::LpgBersubsidi3Kg,
            "tabung",
            16000,
            18000,
            20000,
            150.0,
            true, // Subsidi wajib NIK
        );

        warung
    }

    pub fn tambah_item(
        &mut self,
        sku: &str,
        nama: &str,
        kategori: KategoriKomoditasSembako,
        satuan: &str,
        beli: u64,
        jual_anggota: u64,
        jual_umum: u64,
        stok: f64,
        subsidi: bool,
    ) {
        self.inventaris.insert(
            sku.to_string(),
            ItemSembako {
                sku_barcode: sku.to_string(),
                nama_barang: nama.to_string(),
                kategori,
                satuan: satuan.to_string(),
                harga_beli_idr: beli,
                harga_jual_anggota_idr: jual_anggota,
                harga_jual_umum_idr: jual_umum,
                stok_tersedia: stok,
                wajib_nik_subsidi: subsidi,
            },
        );
    }

    /// Proses checkout kasir POS dengan diskon anggota & verifikasi subsidi NIK
    pub fn proses_transaksi(
        &mut self,
        no_struk: &str,
        npa_anggota: Option<&str>,
        nik_subsidi: Option<&str>,
        keranjang_req: &[(&str, f64)],
        metode_bayar: &str,
    ) -> Result<StrukTransaksiSembako, String> {
        let is_anggota = npa_anggota.is_some();
        let mut items_out = Vec::new();
        let mut total = 0;

        for &(sku, qty) in keranjang_req {
            if let Some(item) = self.inventaris.get_mut(sku) {
                if item.stok_tersedia < qty {
                    return Err(format!("Stok barang '{}' tidak mencukupi (sisa: {})", item.nama_barang, item.stok_tersedia));
                }

                if item.wajib_nik_subsidi && nik_subsidi.is_none() {
                    return Err(format!("Barang bersubsidi '{}' wajib menyertakan NIK KTP warga!", item.nama_barang));
                }

                let harga_satuan = if is_anggota {
                    item.harga_jual_anggota_idr
                } else {
                    item.harga_jual_umum_idr
                };

                let subtotal = (harga_satuan as f64 * qty).round() as u64;
                item.stok_tersedia -= qty;
                total += subtotal;

                items_out.push(ItemKeranjang {
                    sku_barcode: sku.to_string(),
                    nama_barang: item.nama_barang.clone(),
                    kuantitas: qty,
                    harga_satuan_idr: harga_satuan,
                    subtotal_idr: subtotal,
                });
            } else {
                return Err(format!("Barang SKU {} tidak ditemukan", sku));
            }
        }

        let poin = if is_anggota { (total / 10000) as u32 } else { 0 };

        let struk = StrukTransaksiSembako {
            nomor_struk: no_struk.to_string(),
            nomor_pokok_anggota: npa_anggota.map(|s| s.to_string()),
            nik_pembeli_subsidi: nik_subsidi.map(|s| s.to_string()),
            items: items_out,
            total_belanja_idr: total,
            metode_pembayaran: metode_bayar.to_string(),
            poin_anggota_diperoleh: poin,
        };

        self.riwayat_transaksi.push(struk.clone());
        Ok(struk)
    }
}
