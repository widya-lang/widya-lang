-- Skema Basis Data Integrasi Dapodik & Rekapitulasi Sekolah
BUAT_TABEL sekolah (
  npsn TEKS KUNCI_UTAMA,
  nama_sekolah TEKS TIDAK_KOSONG,
  jenjang TEKS,
  provinsi TEKS,
  total_siswa INTEGER,
  status_sinkron TEKS
);

BUAT_TABEL rekap_anggaran (
  id_rekap TEKS KUNCI_UTAMA,
  npsn TEKS REFERENSI sekolah(npsn),
  pos_anggaran TEKS,
  nominal INTEGER,
  status_verifikasi TEKS
);
