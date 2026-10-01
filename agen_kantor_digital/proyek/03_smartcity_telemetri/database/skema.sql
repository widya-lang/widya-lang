-- ==============================================================================
-- skema.sql - Skema Basis Data SmartCity Telemetry
-- Engine: WidyaBase v1.0.0
-- ==============================================================================

BUAT_TABEL JIKA_TIDAK_ADA stasiun_sensor (
    id_stasiun TEKS KUNCI_UTAMA,
    nama_lokasi TEKS,
    lat REAL,
    lon REAL,
    status_aktif TEKS
);

BUAT_TABEL JIKA_TIDAK_ADA log_ispu (
    id_log TEKS KUNCI_UTAMA,
    id_stasiun TEKS,
    pm25 REAL,
    nilai_ispu INT,
    kategori TEKS,
    waktu_rekam TEKS
);

SISIP_KE stasiun_sensor NILAI ('ISPU-01', 'Stasiun Barat - Taman Kota', -7.4241, 109.2301, 'AKTIF');
SISIP_KE stasiun_sensor NILAI ('ISPU-02', 'Stasiun Pusat - Alun-Alun', -7.4285, 109.2355, 'AKTIF');
SISIP_KE stasiun_sensor NILAI ('ISPU-03', 'Stasiun Timur - Kawasan Industri', -7.4312, 109.2488, 'AKTIF');
