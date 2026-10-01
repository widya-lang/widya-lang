-- ==============================================================================
-- skema.sql - Skema Basis Data Mandiri Koperasi Desa Merah Putih (KDMP)
-- Engine: WidyaBase v1.0.0 (Slotted-Page 4KB, B+Tree, WAL, ACID)
-- ==============================================================================

BUAT_TABEL JIKA_TIDAK_ADA anggota (
    no_anggota TEKS KUNCI_UTAMA,
    nik TEKS,
    nama_lengkap TEKS,
    alamat_desa TEKS,
    status_keanggotaan TEKS,
    tgl_bergabung TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA pinjaman (
    id_pinjaman TEKS KUNCI_UTAMA,
    no_anggota TEKS,
    nominal_pokok INT,
    bunga_persen REAL,
    tenor_bulan INT,
    status_approval TEKS,
    peruntukan TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA produk_warung (
    kode_produk TEKS KUNCI_UTAMA,
    nama_produk TEKS,
    kategori TEKS,
    harga_jual INT,
    stok_tersedia INT
);

BUAT_TABEL JIKA_TIDAK_ADA transaksi_pos (
    id_transaksi TEKS KUNCI_UTAMA,
    no_anggota TEKS,
    waktu_transaksi TEKS,
    total_bayar INT,
    diskon_anggota INT,
    metode_bayar TEKS,
    status_acid TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA sertifikat_resi_gudang (
    no_srg TEKS KUNCI_UTAMA,
    nama_petani TEKS,
    komoditas TEKS,
    bobot_ton REAL,
    kadar_air REAL,
    taksiran_nilai INT,
    status_jaminan TEKS
);

-- Seed Data Awal
SISIP_KE anggota NILAI ('KOP-0084', '3302150804720002', 'Bpk. Slamet Riyadi', 'Dusun 1 RT 02 RW 01', 'AKTIF', '2026-01-10');
SISIP_KE anggota NILAI ('KOP-0112', '3302154402780001', 'Ibu Sri Wahyuni', 'Dusun 2 RT 04 RW 02', 'AKTIF', '2026-02-15');
SISIP_KE anggota NILAI ('KOP-0341', '3302151206800004', 'Bpk. Joko Supriyanto', 'Dusun 3 RT 01 RW 03', 'AKTIF', '2026-03-01');
