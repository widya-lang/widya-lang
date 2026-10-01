/**
 * bersihkan_proyek.js — Reset tabel proyek ke kondisi bersih tanpa duplikat
 * Dijalankan sekali untuk memperbaiki data setelah migrasi ganda
 */
const { execSync } = require('child_process');
const path = require('path');

const exe = path.resolve(__dirname, '../dist/releases/v1.0.0/WidyaBase.exe');
const db  = path.resolve(__dirname, 'kantor_digital.wdb');

function q(sql, label) {
  try {
    const r = execSync(`"${exe}" kueri "${sql}" --data "${db}"`, { encoding: 'utf8' });
    console.log('✓', label || sql.substring(0, 50));
    return r;
  } catch (e) {
    console.log('  →', (e.message || '').split('\n')[0].substring(0, 80));
    return null;
  }
}

console.log('\n— RESET TABEL PROYEK —\n');

// Hapus semua entri dan insert ulang 3 proyek bersih
q('HAPUS DARI proyek DIMANA status = \'AKTIF\';', 'Hapus semua proyek AKTIF');

q("SISIP_KE proyek NILAI ('PRJ-WKD-001', 'PRJ-WIDYA-KOP-01', 'Sistem Koperasi Digital Berdaulat Multi-Agent', 'AKTIF', 82, 'Sistem koperasi dengan 10 AI Agent dan WidyaBase', '2026-09-01', '2026-12-31');",
  'INSERT PRJ-WIDYA-KOP-01 (82%)');

q("SISIP_KE proyek NILAI ('PRJ-ARK-001', 'PRJ-ARKAS-WEB-01', 'Aplikasi ARKAS Web Terintegrasi', 'AKTIF', 55, 'ARKAS web terintegrasi Dapodik dan BOS', '2026-09-15', '2026-11-30');",
  'INSERT PRJ-ARKAS-WEB-01 (55%)');

q("SISIP_KE proyek NILAI ('PRJ-INV-002', 'PRJ-WIDYA-INV-02', 'Manajemen Inventaris UMKM Nusantara', 'AKTIF', 88, 'Barcode, RFID, dan IoT sensor gudang', '2026-08-01', '2026-10-31');",
  'INSERT PRJ-WIDYA-INV-02 (88%)');

console.log('\n— VERIFIKASI AKHIR —\n');
q('PILIH * DARI proyek;', 'Daftar proyek final');

try {
  const res = execSync(`"${exe}" kueri "PILIH * DARI proyek;" --data "${db}" --format json`, { encoding: 'utf8' });
  const data = JSON.parse(res);
  console.log(`\n✅ Total: ${data.length} proyek`);
  data.forEach(p => {
    console.log(`   • [${p.kode_proyek}] ${p.nama_proyek} — ${p.progres}% (${p.status})`);
  });
} catch(e) {
  console.log('Lihat output di atas.');
}
