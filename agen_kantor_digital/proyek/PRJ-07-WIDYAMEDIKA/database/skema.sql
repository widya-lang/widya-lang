-- ==============================================================================
-- skema.sql - Skema Basis Data WidyaMedika SIMRS
-- Engine: WidyaBase v1.0.0
-- ==============================================================================

BUAT_TABEL JIKA_TIDAK_ADA pasien (
    no_rm TEKS KUNCI_UTAMA,
    nik TEKS,
    no_bpjs TEKS,
    nama_pasien TEKS,
    umur INT,
    gender TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA kunjungan_rme (
    id_kunjungan TEKS KUNCI_UTAMA,
    no_rm TEKS,
    tgl_periksa TEKS,
    keluhan TEKS,
    icd10 TEKS,
    resep TEKS,
    status_bridging TEKS
);

SISIP_KE pasien NILAI ('RM-2026-08412', '3302150804720002', '0001849201948', 'Bpk. Slamet Riyadi', 54, 'L');
SISIP_KE pasien NILAI ('RM-2026-08413', '3302154402780001', '0002948192019', 'Ibu Siti Aminah', 48, 'P');
