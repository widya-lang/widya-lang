use super::marc21_dublin_core::{DublinCoreMetadata, KlasifikasiDdc, Marc21Record};
use serde::{Deserialize, Serialize};

/// ============================================================================
/// SISTEM PERPUSTAKAAN SEKOLAH (SNP PERPUSNAS & KEMENDIKDASMEN)
/// Pengelolaan Buku Teks Pelajaran BOSP, Kartu Anggota NISN, dan Sirkulasi
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JenisKoleksiSekolah {
    BukuTeksUtamaKurikulum,      // Pengadaan BOSP Buku Kurikulum Nasional
    BukuPengayaanDanReferensi,   // Ensiklopedia, Kamus, Buku Pengayaan
    BukuBacaanFiksiDanSastra,    // Cerita Rakyat, Novel Edukatif
    BukuDigitalEbook,            // Koleksi Pojok Baca Digital
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KoleksiBukuSekolah {
    pub id_buku: String,
    pub isbn: String,
    pub judul: String,
    pub pengarang: String,
    pub penerbit: String,
    pub tahun_terbit: u32,
    pub jenis_koleksi: JenisKoleksiSekolah,
    pub peruntukan_kelas: String, // e.g. "Kelas 4 SD", "Kelas 7 SMP", "Kelas 10 SMA"
    pub klasifikasi_ddc: KlasifikasiDdc,
    pub jumlah_eksemplar: u32,
    pub eksemplar_tersedia: u32,
    pub sumber_pengadaan: String, // "BOSP Reguler 2026", "Hibah Banper APBN", "Donasi Komite"
    pub metadata_marc: Option<Marc21Record>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnggotaPerpustakaanSekolah {
    pub nisn: String,
    pub nama_siswa: String,
    pub kelas: String,
    pub status_aktif: bool,
    pub batas_maksimal_pinjam: u32,
    pub jumlah_buku_dipinjam: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransaksiPeminjamanSekolah {
    pub nomor_transaksi: String,
    pub nisn_peminjam: String,
    pub id_buku: String,
    pub judul_buku: String,
    pub tanggal_pinjam: String,
    pub batas_kembali: String,
    pub tanggal_dikembalikan: Option<String>,
    pub denda_keterlambatan_rp: f64,
    pub status_selesai: bool,
}

pub struct PerpustakaanSekolahEngine {
    pub npsn: String,
    pub nama_sekolah: String,
    pub katalog_buku: Vec<KoleksiBukuSekolah>,
    pub daftar_anggota: Vec<AnggotaPerpustakaanSekolah>,
    pub riwayat_sirkulasi: Vec<TransaksiPeminjamanSekolah>,
}

impl PerpustakaanSekolahEngine {
    pub fn new(npsn: &str, nama_sekolah: &str) -> Self {
        Self {
            npsn: npsn.to_string(),
            nama_sekolah: nama_sekolah.to_string(),
            katalog_buku: Vec::new(),
            daftar_anggota: Vec::new(),
            riwayat_sirkulasi: Vec::new(),
        }
    }

    /// Tambahkan koleksi buku ke perpustakaan sekolah
    pub fn tambah_koleksi(&mut self, buku: KoleksiBukuSekolah) {
        self.katalog_buku.push(buku);
    }

    /// Daftarkan siswa sebagai anggota perpustakaan
    pub fn daftarkan_siswa(&mut self, nisn: &str, nama: &str, kelas: &str) {
        self.daftar_anggota.push(AnggotaPerpustakaanSekolah {
            nisn: nisn.to_string(),
            nama_siswa: nama.to_string(),
            kelas: kelas.to_string(),
            status_aktif: true,
            batas_maksimal_pinjam: 5,
            jumlah_buku_dipinjam: 0,
        });
    }

    /// Peminjaman buku oleh siswa
    pub fn pinjam_buku(
        &mut self,
        no_trx: &str,
        nisn: &str,
        id_buku: &str,
        tgl_pinjam: &str,
        tgl_kembali: &str,
    ) -> Result<TransaksiPeminjamanSekolah, String> {
        let siswa = self.daftar_anggota.iter_mut().find(|s| s.nisn == nisn)
            .ok_or_else(|| format!("Siswa NISN '{}' tidak terdaftar!", nisn))?;

        if siswa.jumlah_buku_dipinjam >= siswa.batas_maksimal_pinjam {
            return Err("Batas maksimal peminjaman telah tercapai!".to_string());
        }

        let buku = self.katalog_buku.iter_mut().find(|b| b.id_buku == id_buku)
            .ok_or_else(|| format!("Buku ID '{}' tidak ditemukan!", id_buku))?;

        if buku.eksemplar_tersedia == 0 {
            return Err(format!("Stok eksemplar buku '{}' sedang habis dipinjam!", buku.judul));
        }

        buku.eksemplar_tersedia -= 1;
        siswa.jumlah_buku_dipinjam += 1;

        let trx = TransaksiPeminjamanSekolah {
            nomor_transaksi: no_trx.to_string(),
            nisn_peminjam: nisn.to_string(),
            id_buku: id_buku.to_string(),
            judul_buku: buku.judul.clone(),
            tanggal_pinjam: tgl_pinjam.to_string(),
            batas_kembali: tgl_kembali.to_string(),
            tanggal_dikembalikan: None,
            denda_keterlambatan_rp: 0.0,
            status_selesai: false,
        };

        self.riwayat_sirkulasi.push(trx.clone());
        Ok(trx)
    }

    /// Pengembalian buku
    pub fn kembalikan_buku(&mut self, no_trx: &str, tgl_kembali: &str, denda: f64) -> Result<(), String> {
        let trx = self.riwayat_sirkulasi.iter_mut().find(|t| t.nomor_transaksi == no_trx)
            .ok_or_else(|| "Transaksi tidak ditemukan!".to_string())?;

        if trx.status_selesai {
            return Err("Transaksi sudah diselesaikan sebelumnya!".to_string());
        }

        trx.tanggal_dikembalikan = Some(tgl_kembali.to_string());
        trx.denda_keterlambatan_rp = denda;
        trx.status_selesai = true;

        if let Some(buku) = self.katalog_buku.iter_mut().find(|b| b.id_buku == trx.id_buku) {
            buku.eksemplar_tersedia += 1;
        }

        if let Some(siswa) = self.daftar_anggota.iter_mut().find(|s| s.nisn == trx.nisn_peminjam) {
            if siswa.jumlah_buku_dipinjam > 0 {
                siswa.jumlah_buku_dipinjam -= 1;
            }
        }

        Ok(())
    }
}
