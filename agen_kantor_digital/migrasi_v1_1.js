/**
 * migrasi_v1_1.js — Migrasi WidyaBase v1.1: Tabel Lifecycle Engine
 * Menambahkan: status_agen, log_penulisan_vault, log_komunikasi_agen
 * Aman dijalankan berulang kali (cek eksistensi dulu via PILIH)
 */
const { execSync } = require('child_process');
const path = require('path');

const exe = path.resolve(__dirname, '../dist/releases/v1.0.0/WidyaBase.exe');
const db  = path.resolve(__dirname, 'kantor_digital.wdb');

function runSql(q, label) {
  try {
    const res = execSync(`"${exe}" kueri "${q}" --data "${db}"`, { encoding: 'utf8' });
    console.log('✓', (label || q).substring(0, 55).padEnd(56), '→ OK');
    return res;
  } catch (e) {
    const msg = e.message || '';
    // Jika error adalah "tabel sudah ada", itu bukan masalah
    if (msg.includes('sudah ada') || msg.includes('already exists')) {
      console.log('⚠', (label || q).substring(0, 55).padEnd(56), '→ SUDAH ADA (skip)');
      return null;
    }
    console.error('✗', (label || q).substring(0, 55).padEnd(56), '→ ERROR:', msg.split('\n')[0]);
    return null;
  }
}

function tableExists(tableName) {
  try {
    execSync(`"${exe}" kueri "PILIH * DARI ${tableName} BATAS 1;" --data "${db}"`, { encoding: 'utf8' });
    return true;
  } catch (e) {
    return false;
  }
}

console.log('\n══════════════════════════════════════════════════════');
console.log('  MIGRASI WIDYABASE v1.1 — Lifecycle Engine Tables  ');
console.log('══════════════════════════════════════════════════════\n');

// ─── Tabel 1: status_agen ───
if (tableExists('status_agen')) {
  console.log('⚠  status_agen       → SUDAH ADA (skip)');
} else {
  runSql(
    'BUAT_TABEL status_agen (id_agen TEKS, nama_agen TEKS, state TEKS, beban_dinamis INT, tugas_aktif_count INT, terakhir_update TEKS);',
    'BUAT_TABEL status_agen'
  );
}

// ─── Tabel 2: log_penulisan_vault ───
if (tableExists('log_penulisan_vault')) {
  console.log('⚠  log_penulisan_vault → SUDAH ADA (skip)');
} else {
  runSql(
    'BUAT_TABEL log_penulisan_vault (id_log TEKS, waktu TEKS, path_target TEKS, path_fallback TEKS, sukses INT, checksum_md5 TEKS, ukuran_bita INT, keterangan TEKS);',
    'BUAT_TABEL log_penulisan_vault'
  );
}

// ─── Tabel 3: log_komunikasi_agen ───
if (tableExists('log_komunikasi_agen')) {
  console.log('⚠  log_komunikasi_agen → SUDAH ADA (skip)');
} else {
  runSql(
    'BUAT_TABEL log_komunikasi_agen (id_pesan TEKS, waktu TEKS, dari_agen TEKS, ke_agen TEKS, tipe_pesan TEKS, konten TEKS, id_tugas_terkait TEKS, state_setelah TEKS);',
    'BUAT_TABEL log_komunikasi_agen'
  );
}

// ─── Inisialisasi state_agen: 10 agen semua IDLE ───
console.log('\n─── Inisialisasi status_agen (10 agen → IDLE, beban = 0%) ───');
const AGEN_LIST = [
  { id: 'agen_orkestrator', nama: 'Mandor Utama / PM' },
  { id: 'agen_analis',      nama: 'Analis Bisnis' },
  { id: 'agen_arsitek',     nama: 'Arsitek Sistem' },
  { id: 'agen_coder',       nama: 'Lead Developer' },
  { id: 'agen_qa',          nama: 'QA Tester' },
  { id: 'agen_devops',      nama: 'DevOps SRE' },
  { id: 'agen_keamanan',    nama: 'Auditor Siber' },
  { id: 'agen_notulis',     nama: 'Notulis Obsidian' },
  { id: 'agen_humas',       nama: 'Humas Telegram' },
  { id: 'agen_analis_data', nama: 'Analis Data & KPI' },
];

const now = new Date().toISOString();
AGEN_LIST.forEach(a => {
  // Hapus entri lama (idempoten)
  runSql(`HAPUS DARI status_agen DIMANA id_agen = '${a.id}';`, `HAPUS ${a.id}`);
  // Sisipkan fresh dengan state IDLE
  runSql(
    `SISIP_KE status_agen NILAI ('${a.id}', '${a.nama}', 'IDLE', 0, 0, '${now}');`,
    `IDLE ${a.id}`
  );
});

// ─── Update field progres di tabel proyek ───
console.log('\n─── Update field progres proyek ───');
runSql("PERBARUI proyek ATUR progres = 82 DIMANA kode_proyek = 'PRJ-WIDYA-KOP-01';", 'Progres KOP-01: 82%');
runSql("PERBARUI proyek ATUR progres = 55 DIMANA kode_proyek = 'PRJ-ARKAS-WEB-01';", 'Progres ARKAS-01: 55%');
runSql("PERBARUI proyek ATUR progres = 88 DIMANA kode_proyek = 'PRJ-WIDYA-INV-02';", 'Progres INV-02: 88%');

// ─── Verifikasi akhir ───
console.log('\n─── Verifikasi Tabel ───');
runSql('PILIH * DARI status_agen;', 'CHECK status_agen');
runSql('PILIH * DARI proyek;', 'CHECK proyek');

console.log('\n══════════════════════════════════════════════════════');
console.log('  ✅ MIGRASI v1.1 SELESAI                          ');
console.log('══════════════════════════════════════════════════════\n');
