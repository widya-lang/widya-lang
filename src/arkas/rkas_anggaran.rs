use super::sumber_dana::JenisSumberDana;
use serde::{Deserialize, Serialize};

/// ============================================================================
/// PERENCANAAN ANGGARAN SEKOLAH (RKAS) & REVISI ANGGARAN (ARKAS)
/// Mendukung Pergeseran (Tanpa Pengesahan Dinas) vs Perubahan (Wajib Pengesahan)
/// Serta Tabel Sandingan Before-After Historis Lengkap
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StandarNasionalPendidikan {
    StandarKompetensiLulusan,      // SNP 1
    StandarIsi,                    // SNP 2
    StandarProses,                 // SNP 3
    StandarPenilaianPendidikan,    // SNP 4
    StandarPendidikDanTendik,      // SNP 5 (Honorarium, Diklat Guru)
    StandarSaranaDanPrasarana,     // SNP 6 (Buku, Lab, Alat Peraga, Pemeliharaan)
    StandarPengelolaan,            // SNP 7 (PPDB, Rapat, Sistem Informasi)
    StandarPembiayaan,             // SNP 8 (Langganan Daya/Jasa, Operasional Rutin)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KelompokBelanjaAkuntansi {
    BelanjaPegawaiHonor,           // 5.1.01 (Honor GTT/PTT, maksimal batas juknis e.g. 50%)
    BelanjaBarangDanJasa,          // 5.1.02 (ATK, Konsumsi, Langganan Internet, Listrik, SIPLah)
    BelanjaModalPeralatanMesin,    // 5.2.02 (Komputer, Laptop, AC, Alat Lab)
    BelanjaModalGedungBangunan,    // 5.2.03 (Renovasi Ruang Kelas, MCK/Sanitasi)
    BelanjaModalBukuDanAsetLain,   // 5.2.05 (Buku Teks Utama, Koleksi Perpustakaan)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemKegiatanRkas {
    pub id_kegiatan: String,
    pub kode_kegiatan_snp: String, // e.g. "03.02.01" (Pelaksanaan Kegiatan Asesmen Nasional)
    pub standar_snp: StandarNasionalPendidikan,
    pub uraian_kegiatan: String,
    pub kode_rekening: String, // e.g. "5.1.02.01.01.0024"
    pub kelompok_belanja: KelompokBelanjaAkuntansi,
    pub sumber_dana: JenisSumberDana,
    pub volume: f64,
    pub satuan: String, // "Bulan", "Paket", "Unit", "Orang/Bulan"
    pub tarif_harga_satuan_rp: f64,
    pub total_anggaran_tahunan_rp: f64,
    pub alokasi_triwulan_1_rp: f64,
    pub alokasi_triwulan_2_rp: f64,
    pub alokasi_triwulan_3_rp: f64,
    pub alokasi_triwulan_4_rp: f64,
    pub total_realisasi_rp: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JenisRevisiAnggaran {
    /// Pergeseran: Penataan ulang alokasi antar kegiatan/rekening TANPA mengubah total pagu pendapatan.
    /// Aturan: TIDAK MEMERLUKAN pengesahan Dinas Pendidikan (cukup internal sekolah).
    PergeseranAnggaran,
    
    /// Perubahan: Terjadi penambahan/pengurangan total pagu pendapatan atau perubahan prioritas strategis.
    /// Aturan: WAJIB MEMERLUKAN pengesahan Dinas Pendidikan (MARKAS).
    PerubahanAnggaran,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SandinganItemBeforeAfter {
    pub id_kegiatan: String,
    pub uraian_kegiatan: String,
    pub kode_rekening: String,
    pub sumber_dana: JenisSumberDana,
    // Kondisi Sebelum (Before)
    pub volume_sebelum: f64,
    pub tarif_sebelum_rp: f64,
    pub total_sebelum_rp: f64,
    // Kondisi Sesudah (After)
    pub volume_sesudah: f64,
    pub tarif_sesudah_rp: f64,
    pub total_sesudah_rp: f64,
    // Selisih (Delta)
    pub selisih_rp: f64, // (Sesudah - Sebelum)
    pub keterangan_perubahan: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRevisiRkas {
    pub nomor_revisi: u32,
    pub versi_rkas: String, // e.g. "v1.1 (Pergeseran-1)", "v2.0 (Perubahan-1)"
    pub jenis_revisi: JenisRevisiAnggaran,
    pub tanggal_revisi: String,
    pub alasan_dasar_revisi: String,
    pub tabel_sandingan: Vec<SandinganItemBeforeAfter>,
    pub butuh_pengesahan_dinas: bool,
    pub status_pengesahan: String, // "BerlakuInternalSekolah", "MenungguPengesahanDinas", "DisahkanDinas"
    pub tanggal_pengesahan_dinas: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DokumenRkasTahunan {
    pub id_rkas: String,
    pub npsn_sekolah: String,
    pub nama_sekolah: String,
    pub tahun_anggaran: u32,
    pub versi_aktif: String,
    pub daftar_kegiatan: Vec<ItemKegiatanRkas>,
    pub riwayat_revisi: Vec<LogRevisiRkas>,
    pub status_pengesahan_dinas_murni: bool, // Pengesahan RKAS Murni di awal tahun
    pub tanggal_pengesahan_murni: Option<String>,
}

impl DokumenRkasTahunan {
    pub fn new(id_rkas: &str, npsn: &str, nama_sekolah: &str, tahun: u32) -> Self {
        Self {
            id_rkas: id_rkas.to_string(),
            npsn_sekolah: npsn.to_string(),
            nama_sekolah: nama_sekolah.to_string(),
            tahun_anggaran: tahun,
            versi_aktif: "v1.0 (RKAS Murni)".to_string(),
            daftar_kegiatan: Vec::new(),
            riwayat_revisi: Vec::new(),
            status_pengesahan_dinas_murni: false,
            tanggal_pengesahan_murni: None,
        }
    }

    /// Tambahkan item rencana kegiatan anggaran awal (RKAS Murni)
    pub fn tambah_kegiatan(&mut self, item: ItemKegiatanRkas) {
        self.daftar_kegiatan.push(item);
    }

    /// Total seluruh pagu rencana anggaran tahunan
    pub fn total_pagu_tahunan(&self) -> f64 {
        self.daftar_kegiatan.iter().map(|k| k.total_anggaran_tahunan_rp).sum()
    }

    /// Total rencana anggaran per sumber dana
    pub fn total_pagu_per_sumber_dana(&self, jenis: JenisSumberDana) -> f64 {
        self.daftar_kegiatan.iter()
            .filter(|k| k.sumber_dana == jenis)
            .map(|k| k.total_anggaran_tahunan_rp)
            .sum()
    }

    /// Validasi aturan juknis (misal batas belanja honor guru honorer non-ASN <= 50% BOSP)
    pub fn validasi_aturan_juknis_bosp(&self) -> Result<(), String> {
        let total_bosp = self.total_pagu_per_sumber_dana(JenisSumberDana::BospReguler);
        if total_bosp <= 0.0 {
            return Ok(());
        }

        let total_honor: f64 = self.daftar_kegiatan.iter()
            .filter(|k| k.sumber_dana == JenisSumberDana::BospReguler && k.kelompok_belanja == KelompokBelanjaAkuntansi::BelanjaPegawaiHonor)
            .map(|k| k.total_anggaran_tahunan_rp)
            .sum();

        let rasio_honor = (total_honor / total_bosp) * 100.0;
        if rasio_honor > 50.0 {
            return Err(format!(
                "Alokasi Honorarium ({:.1}%) melebihi ambang batas maksimal Juknis BOSP (50.0%)!",
                rasio_honor
            ));
        }
        Ok(())
    }

    /// Pengesahan RKAS Murni awal tahun oleh Manajemen BOS Dinas Pendidikan (MARKAS)
    pub fn sahkan_rkas_dinas(&mut self, tanggal: &str) -> Result<(), String> {
        self.validasi_aturan_juknis_bosp()?;
        self.status_pengesahan_dinas_murni = true;
        self.tanggal_pengesahan_murni = Some(tanggal.to_string());
        Ok(())
    }

    /// =========================================================================
    /// PERGESERAN ANGGARAN (TIDAK PERLU PENGESAHAN DINAS)
    /// Menghasilkan tabel sandingan before-after secara otomatis
    /// =========================================================================
    pub fn lakukan_pergeseran_anggaran(
        &mut self,
        tanggal: &str,
        alasan: &str,
        item_updates: Vec<ItemKegiatanRkas>, // Daftar item baru yang disesuaikan
    ) -> Result<LogRevisiRkas, String> {
        // 1. Validasi Total Pagu tidak boleh berubah saat Pergeseran
        let total_lama = self.total_pagu_tahunan();
        let mut daftar_baru = self.daftar_kegiatan.clone();
        let mut sandingan_list = Vec::new();

        for update in item_updates {
            if let Some(pos) = daftar_baru.iter().position(|k| k.id_kegiatan == update.id_kegiatan) {
                let sebelum = &daftar_baru[pos];
                let sandingan = SandinganItemBeforeAfter {
                    id_kegiatan: update.id_kegiatan.clone(),
                    uraian_kegiatan: update.uraian_kegiatan.clone(),
                    kode_rekening: update.kode_rekening.clone(),
                    sumber_dana: update.sumber_dana.clone(),
                    volume_sebelum: sebelum.volume,
                    tarif_sebelum_rp: sebelum.tarif_harga_satuan_rp,
                    total_sebelum_rp: sebelum.total_anggaran_tahunan_rp,
                    volume_sesudah: update.volume,
                    tarif_sesudah_rp: update.tarif_harga_satuan_rp,
                    total_sesudah_rp: update.total_anggaran_tahunan_rp,
                    selisih_rp: update.total_anggaran_tahunan_rp - sebelum.total_anggaran_tahunan_rp,
                    keterangan_perubahan: format!("Pergeseran anggaran internal: {}", alasan),
                };
                sandingan_list.push(sandingan);
                daftar_baru[pos] = update;
            } else {
                // Item baru ditambahkan dalam pergeseran
                let sandingan = SandinganItemBeforeAfter {
                    id_kegiatan: update.id_kegiatan.clone(),
                    uraian_kegiatan: update.uraian_kegiatan.clone(),
                    kode_rekening: update.kode_rekening.clone(),
                    sumber_dana: update.sumber_dana.clone(),
                    volume_sebelum: 0.0,
                    tarif_sebelum_rp: 0.0,
                    total_sebelum_rp: 0.0,
                    volume_sesudah: update.volume,
                    tarif_sesudah_rp: update.tarif_harga_satuan_rp,
                    total_sesudah_rp: update.total_anggaran_tahunan_rp,
                    selisih_rp: update.total_anggaran_tahunan_rp,
                    keterangan_perubahan: format!("Penambahan pos baru dalam pergeseran: {}", alasan),
                };
                sandingan_list.push(sandingan);
                daftar_baru.push(update);
            }
        }

        let total_baru: f64 = daftar_baru.iter().map(|k| k.total_anggaran_tahunan_rp).sum();
        let delta_total = (total_baru - total_lama).abs();
        if delta_total > 1.0 { // Toleransi selisih pembulatan 1 rupiah
            return Err(format!(
                "Pergeseran Anggaran tidak boleh mengubah total pagu! (Total Lama: Rp {}, Total Baru: Rp {}, Selisih: Rp {})",
                total_lama, total_baru, total_baru - total_lama
            ));
        }

        self.daftar_kegiatan = daftar_baru;
        self.validasi_aturan_juknis_bosp()?;

        let nomor_rev = (self.riwayat_revisi.len() + 1) as u32;
        let versi_baru = format!("v1.{} (Pergeseran-{})", nomor_rev, nomor_rev);
        self.versi_aktif = versi_baru.clone();

        let log = LogRevisiRkas {
            nomor_revisi: nomor_rev,
            versi_rkas: versi_baru,
            jenis_revisi: JenisRevisiAnggaran::PergeseranAnggaran,
            tanggal_revisi: tanggal.to_string(),
            alasan_dasar_revisi: alasan.to_string(),
            tabel_sandingan: sandingan_list,
            butuh_pengesahan_dinas: false, // << TIDAK PERLU PENGESAHAN DINAS
            status_pengesahan: "BerlakuInternalSekolah".to_string(),
            tanggal_pengesahan_dinas: None,
        };

        self.riwayat_revisi.push(log.clone());
        Ok(log)
    }

    /// =========================================================================
    /// PERUBAHAN ANGGARAN (WAJIB PENGESAHAN DINAS MARKAS)
    /// Menghasilkan tabel sandingan before-after dan menunggu validasi dinas
    /// =========================================================================
    pub fn ajukan_perubahan_anggaran(
        &mut self,
        tanggal: &str,
        alasan: &str,
        daftar_kegiatan_baru: Vec<ItemKegiatanRkas>,
    ) -> Result<LogRevisiRkas, String> {
        let mut sandingan_list = Vec::new();

        // Bandingkan setiap item lama dengan yang baru
        for baru in &daftar_kegiatan_baru {
            if let Some(sebelum) = self.daftar_kegiatan.iter().find(|k| k.id_kegiatan == baru.id_kegiatan) {
                if (sebelum.total_anggaran_tahunan_rp - baru.total_anggaran_tahunan_rp).abs() > 0.01
                    || (sebelum.volume - baru.volume).abs() > 0.01
                {
                    sandingan_list.push(SandinganItemBeforeAfter {
                        id_kegiatan: baru.id_kegiatan.clone(),
                        uraian_kegiatan: baru.uraian_kegiatan.clone(),
                        kode_rekening: baru.kode_rekening.clone(),
                        sumber_dana: baru.sumber_dana.clone(),
                        volume_sebelum: sebelum.volume,
                        tarif_sebelum_rp: sebelum.tarif_harga_satuan_rp,
                        total_sebelum_rp: sebelum.total_anggaran_tahunan_rp,
                        volume_sesudah: baru.volume,
                        tarif_sesudah_rp: baru.tarif_harga_satuan_rp,
                        total_sesudah_rp: baru.total_anggaran_tahunan_rp,
                        selisih_rp: baru.total_anggaran_tahunan_rp - sebelum.total_anggaran_tahunan_rp,
                        keterangan_perubahan: format!("Perubahan pagu/volume: {}", alasan),
                    });
                }
            } else {
                // Item baru ditambahkan dalam Perubahan Anggaran
                sandingan_list.push(SandinganItemBeforeAfter {
                    id_kegiatan: baru.id_kegiatan.clone(),
                    uraian_kegiatan: baru.uraian_kegiatan.clone(),
                    kode_rekening: baru.kode_rekening.clone(),
                    sumber_dana: baru.sumber_dana.clone(),
                    volume_sebelum: 0.0,
                    tarif_sebelum_rp: 0.0,
                    total_sebelum_rp: 0.0,
                    volume_sesudah: baru.volume,
                    tarif_sesudah_rp: baru.tarif_harga_satuan_rp,
                    total_sesudah_rp: baru.total_anggaran_tahunan_rp,
                    selisih_rp: baru.total_anggaran_tahunan_rp,
                    keterangan_perubahan: format!("Pos kegiatan baru: {}", alasan),
                });
            }
        }

        let nomor_rev = (self.riwayat_revisi.len() + 1) as u32;
        let versi_baru = format!("v2.{} (Perubahan-{})", nomor_rev, nomor_rev);

        let log = LogRevisiRkas {
            nomor_revisi: nomor_rev,
            versi_rkas: versi_baru,
            jenis_revisi: JenisRevisiAnggaran::PerubahanAnggaran,
            tanggal_revisi: tanggal.to_string(),
            alasan_dasar_revisi: alasan.to_string(),
            tabel_sandingan: sandingan_list,
            butuh_pengesahan_dinas: true, // << WAJIB PENGESAHAN DINAS
            status_pengesahan: "MenungguPengesahanDinas".to_string(),
            tanggal_pengesahan_dinas: None,
        };

        self.daftar_kegiatan = daftar_kegiatan_baru;
        self.riwayat_revisi.push(log.clone());
        Ok(log)
    }

    /// Pengesahan Perubahan Anggaran oleh Dinas Pendidikan (MARKAS)
    pub fn sahkan_perubahan_oleh_dinas(&mut self, nomor_revisi: u32, tanggal_sah: &str) -> Result<(), String> {
        self.validasi_aturan_juknis_bosp()?;
        if let Some(rev) = self.riwayat_revisi.iter_mut().find(|r| r.nomor_revisi == nomor_revisi) {
            if rev.jenis_revisi != JenisRevisiAnggaran::PerubahanAnggaran {
                return Err("Pergeseran anggaran tidak memerlukan pengesahan dinas!".to_string());
            }
            rev.status_pengesahan = "DisahkanDinas".to_string();
            rev.tanggal_pengesahan_dinas = Some(tanggal_sah.to_string());
            self.versi_aktif = rev.versi_rkas.clone();
            Ok(())
        } else {
            Err(format!("Nomor revisi {} tidak ditemukan!", nomor_revisi))
        }
    }
}
