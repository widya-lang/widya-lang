-- ==============================================================================
-- skema.sql - Skema Basis Data Switcher WidyaPay
-- Engine: WidyaBase v1.0.0
-- ==============================================================================

BUAT_TABEL JIKA_TIDAK_ADA merchant (
    nmid TEKS KUNCI_UTAMA,
    nama_merchant TEKS,
    mcc_code TEKS,
    kota TEKS,
    postal_code TEKS,
    status_aktif TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA transaksi_switching (
    id_transaksi TEKS KUNCI_UTAMA,
    jenis_layanan TEKS,
    bank_asal TEKS,
    bank_tujuan TEKS,
    nominal INT,
    latensi_ms REAL,
    status_transaksi TEKS,
    waktu_settlement TEKS
);

SISIP_KE merchant NILAI ('ID102026093012345', 'KOPERASI DESA MERAH PUTIH #01', '5411', 'BANYUMAS', '53111', 'AKTIF');
