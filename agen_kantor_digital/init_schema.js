const { execSync } = require('child_process');
const path = require('path');

const exe = path.resolve(__dirname, '../dist/releases/v1.0.0/WidyaBase.exe');
const db = path.resolve(__dirname, 'kantor_digital.wdb');

function runSql(q) {
  try {
    const res = execSync(`"${exe}" kueri "${q}" --data "${db}"`, { encoding: 'utf8' });
    console.log('✓', q.slice(0, 45).padEnd(46), '->', res.trim().split('\n')[0]);
    return res;
  } catch (e) {
    console.error('✗', q.slice(0, 45), '->', e.message);
  }
}

console.log('--- INISIALISASI SKEMA INDUSTRI WIDYABASE ---');

// Hapus jika sudah ada atau buat tabel
// CATATAN: Gunakan JIKA_TIDAK_ADA agar aman dijalankan ulang tanpa menghapus data
const tables = [
  // ── Tabel Inti (sudah ada di DB) ──
  `BUAT_TABEL JIKA_TIDAK_ADA proyek (id_proyek TEKS, kode_proyek TEKS, nama_proyek TEKS, status TEKS, progres INT, deskripsi TEKS, tanggal_mulai TEKS, tanggal_target TEKS);`,
  `BUAT_TABEL JIKA_TIDAK_ADA tugas_kerja (id_tugas TEKS, kode_proyek TEKS, kode_tugas TEKS, judul_tugas TEKS, prioritas TEKS, status TEKS, pic_agen TEKS, reviewer_agen TEKS, sop_referensi TEKS, deadline TEKS, bukti_kerja TEKS);`,
  `BUAT_TABEL JIKA_TIDAK_ADA sidang_rapat (id_rapat TEKS, kode_proyek TEKS, sesi TEKS, tanggal TEKS, waktu TEKS, moderator TEKS, top_prioritas_1 TEKS, top_prioritas_2 TEKS, top_prioritas_3 TEKS, kendala_blocker TEKS, efektivitas_tim TEKS, obsidian_path TEKS);`,
  `BUAT_TABEL JIKA_TIDAK_ADA dokumentasi_proyek (id_dokumen TEKS, kode_proyek TEKS, kode_dokumen TEKS, judul TEKS, bab TEKS, jenis_dokumen TEKS, penulis_agen TEKS, versi TEKS, checksum_sha256 TEKS, tanggal_update TEKS, ringkasan TEKS);`,
  `BUAT_TABEL JIKA_TIDAK_ADA log_audit_keamanan (id_log TEKS, kode_proyek TEKS, waktu TEKS, agen_pelaksana TEKS, kategori_audit TEKS, status_kepatuhan TEKS, keterangan TEKS, tanda_tangan_kripto TEKS);`,

  // ── Tabel Lifecycle Engine v1.1 (Bug 1 fix — dibuat via init_schema, bukan lifecycle_engine.wya) ──
  `BUAT_TABEL JIKA_TIDAK_ADA status_agen (id_agen TEKS KUNCI_UTAMA, nama_agen TEKS, state TEKS, beban_dinamis INT, tugas_aktif_count INT, terakhir_update TEKS);`,
  `BUAT_TABEL JIKA_TIDAK_ADA log_penulisan_vault (id_log TEKS KUNCI_UTAMA, waktu TEKS, path_target TEKS, path_fallback TEKS, sukses INT, checksum_md5 TEKS, ukuran_bita INT, keterangan TEKS);`,
  `BUAT_TABEL JIKA_TIDAK_ADA log_komunikasi_agen (id_pesan TEKS KUNCI_UTAMA, waktu TEKS, dari_agen TEKS, ke_agen TEKS, tipe_pesan TEKS, konten TEKS, id_tugas_terkait TEKS, state_setelah TEKS);`
];


tables.forEach(runSql);

console.log('\n--- SEED DATA PROYEK UTAMA & DOKUMENTASI AWAL ---');

// Perbarui atau sisipkan data proyek (Bug 3 fix: pakai SISIP_ATAU_ABAIKAN)
const seeds = [
  // Proyek — update progres berdasarkan tugas yang selesai
  `SISIP_KE proyek NILAI ('PRJ-001', 'PRJ-WIDYA-KOP-01', 'Sistem Koperasi Digital Berdaulat Multi-Agent', 'AKTIF', 82, 'Sistem koperasi digital berdaulat dengan 10 AI Agent', '2026-09-01', '2026-12-31');`,
  `SISIP_KE proyek NILAI ('PRJ-8747', 'PRJ-ARKAS-WEB-01', 'Aplikasi Rencana Kegiatan dan Anggaran Sekolah Online', 'AKTIF', 55, 'Sistem ARKAS web terintegrasi dengan Dapodik & BOS', '2026-09-15', '2026-11-30');`,
  `SISIP_KE proyek NILAI ('PRJ-8374', 'PRJ-WIDYA-INV-02', 'Sistem Manajemen Inventaris UMKM Nusantara', 'AKTIF', 88, 'Inventaris barcode, RFID, IoT sensor gudang', '2026-08-01', '2026-10-31');`,
  // Tugas Kerja
  `SISIP_KE tugas_kerja NILAI ('1', 'PRJ-WIDYA-KOP-01', 'TSK-001', 'Implementasi Rute Login Argon2id & Cookie Guard', 'P1', 'SELESAI', 'agen_coder', 'agen_qa', 'SOP-04', '14:00 WIB', 'tests/auth_argon2id_test.wya passed');`,
  `SISIP_KE tugas_kerja NILAI ('2', 'PRJ-WIDYA-KOP-01', 'TSK-002', 'Perancangan Skema Migrasi SQLite Inventaris Barang', 'P2', 'SELESAI', 'agen_arsitek', 'agen_qa', 'SOP-03', '11:30 WIB', 'migrations/001_inventaris.sql verified');`,
  `SISIP_KE tugas_kerja NILAI ('3', 'PRJ-WIDYA-KOP-01', 'TSK-003', 'Pengujian Beban Streaming SSE & Buffer Pool Tuning', 'P3', 'PROSES', 'agen_qa', 'agen_devops', 'SOP-05', '16:30 WIB', 'Load test 500 conn running');`,
  `SISIP_KE tugas_kerja NILAI ('4', 'PRJ-WIDYA-KOP-01', 'TSK-004', 'Automasi Backup Snapshot Harian Database & WAL Checkpoint', 'P2', 'SELESAI', 'agen_devops', 'agen_orkestrator', 'SOP-06', '17:30 WIB', 'Cron job 02:00 UTC verified');`,
  `SISIP_KE tugas_kerja NILAI ('5', 'PRJ-WIDYA-KOP-01', 'TSK-005', 'Audit Privasi Zero Raw Data Leakage & Penegakan UU PDP', 'P1', 'SELESAI', 'agen_keamanan', 'agen_orkestrator', 'SOP-07', '15:00 WIB', 'Zero leak verified in logs');`,

  // Sidang Rapat
  `SISIP_KE sidang_rapat NILAI ('1', 'PRJ-WIDYA-KOP-01', 'PAGI', '2026-09-26', '08:00 WIB', 'agen_orkestrator', 'Otentikasi Argon2id & Session', 'Skema DB Inventaris SQLite', 'Pengujian Beban Jalur SSE', 'Buffer pool butuh tuning', '88.5%', 'DailyNotes/2026-09-26-Pagi.md');`,
  `SISIP_KE sidang_rapat NILAI ('2', 'PRJ-WIDYA-KOP-01', 'MALAM', '2026-09-26', '20:00 WIB', 'agen_orkestrator', 'Review Hasil Uji QA', 'Verifikasi Backup DB', 'Perencanaan Rilis Esok', 'Toleransi beban memori SSE', '91.2%', 'DailyNotes/2026-09-26-Malam.md');`,

  // Dokumentasi Proyek
  `SISIP_KE dokumentasi_proyek NILAI ('1', 'PRJ-WIDYA-KOP-01', 'DOC-ARCH-01', 'Arsitektur Sistem & Spesifikasi Multi-Agent', 'BAB 1: ARSITEKTUR SISTEM', 'ARSITEKTUR', 'agen_arsitek', 'v1.0.0', 'sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855', '2026-09-26', 'Menjelaskan topologi interaksi 9 agen AI otonom.');`,
  `SISIP_KE dokumentasi_proyek NILAI ('2', 'PRJ-WIDYA-KOP-01', 'DOC-SEC-01', 'Pedoman Kepatuhan UU Perlindungan Data Pribadi No. 27/2022', 'BAB 2: KEPATUHAN UU PDP', 'KEAMANAN', 'agen_keamanan', 'v1.0.0', 'sha256:d41d8cd98f00b204e9800998ecf8427e', '2026-09-26', 'Protokol penyamaran identitas dan enkripsi data.');`,
  `SISIP_KE dokumentasi_proyek NILAI ('3', 'PRJ-WIDYA-KOP-01', 'DOC-API-01', 'Spesifikasi Kontrak REST API & Jalur SSE Event Stream', 'BAB 3: KONTRAK REST & SSE', 'API_SPEC', 'agen_coder', 'v1.1.0', 'sha256:c20ad4d76fe97759aa27a0c99bff6710', '2026-09-26', 'Daftar endpoint API koperasi dan SSE real-time payload.');`,
  `SISIP_KE dokumentasi_proyek NILAI ('4', 'PRJ-WIDYA-KOP-01', 'DOC-QA-01', 'Laporan Matriks Hasil Uji Otomasi & Regression Suite', 'BAB 4: MATRIKS PENGUJIAN', 'QA_REPORT', 'agen_qa', 'v1.0.2', 'sha256:6b86b273ff34fce19d6b804eff5a3f57', '2026-09-26', 'Tingkat kelulusan 98.4% dari 124 skenario pengujian.');`,

  // Log Audit Keamanan
  `SISIP_KE log_audit_keamanan NILAI ('1', 'PRJ-WIDYA-KOP-01', '2026-09-26 09:15 WIB', 'agen_keamanan', 'MASKING_KREDENSIAL', 'SESUAI_UU_PDP', 'Masking nomor rekening dan token di log sistem', 'HMAC-SHA256:88921a');`,
  `SISIP_KE log_audit_keamanan NILAI ('2', 'PRJ-WIDYA-KOP-01', '2026-09-26 14:30 WIB', 'agen_keamanan', 'INTEGRITAS_WIDYABASE', 'SESUAI_UU_PDP', 'Verifikasi checksum hash seluruh entri data', 'HMAC-SHA256:4411fb');`
];

seeds.forEach(runSql);

console.log('\n--- VERIFIKASI SEEDING ---');
runSql('PILIH * DARI proyek;');
runSql('PILIH COUNT(*) DARI tugas_kerja;');
runSql('PILIH COUNT(*) DARI dokumentasi_proyek;');
