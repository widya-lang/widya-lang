const { execSync } = require('child_process');
const path = require('path');
const sync = require('./sync_obsidian');

const WIDYABASE_EXE = path.resolve(__dirname, '../../dist/releases/v1.0.0/WidyaBase.exe');
const WIDYABASE_DB = path.resolve(__dirname, '../kantor_digital.wdb');

function queryDb(sql) {
  try {
    const raw = execSync(`"${WIDYABASE_EXE}" kueri "${sql}" --data "${WIDYABASE_DB}" --format json`, { encoding: 'utf8' });
    return JSON.parse(raw);
  } catch (e) {
    console.error('Kueri error:', sql, e.message);
    return [];
  }
}

console.log('=== MEMULAI SINKRONISASI LENGKAP KE OBSIDIAN VAULT ===');
const proyekList = queryDb('PILIH * DARI proyek;');
const tugasList = queryDb('PILIH * DARI tugas_kerja;');

console.log(`Ditemukan ${proyekList.length} proyek dan ${tugasList.length} tugas di WidyaBase.`);

// 1. Sinkronkan Daftar Seluruh Proyek
sync.syncDaftarProyek(proyekList);

// 2. Buat Dossier untuk masing-masing proyek
proyekList.forEach(p => {
  sync.syncProyekDossier(p, tugasList);
  sync.syncBukuDokumentasi(p.kode_proyek, p.nama_proyek);
});

// 3. Sinkronkan Papan Kanban
sync.syncKanbanVault(tugasList);

console.log('=== SINKRONISASI OBSIDIAN TUNTAS 100% ===');
