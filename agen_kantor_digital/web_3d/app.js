/**
 * app.js - Logika Interaktif & State Management Kantor Digital 3D v1.2.0-GA
 * 10 AI Agent Berdaulat | 10 Proyek Industri / Hari | Shared-Pool Async Queue
 */

// 1. DATA MASTER 10 AI AGENT DENGAN PROFIL REALISTIK INDONESIA & GENDER (Dynamic Sourced)
const AGENTS_DATA = [
  {
    id: 'agen_orkestrator',
    nama: 'Arya Wicaksana, PMP',
    panggilan: 'Arya',
    gender: 'Pria',
    peran: 'Mandor Utama (Project Director)',
    gelar: 'Chief Orchestrator • PMP® Certified',
    kluster: 'Executive Suite',
    sop: 'SOP-01',
    sopFile: 'SOP-01_Orkestrator_PM.md',
    avatar: '👨‍💼',
    warna: '#3b82f6',
    status: '🟡 IDLE — Santai & Siaga',
    tugas: '— Tidak ada beban kerja aktif —',
    beban: 0
  },
  {
    id: 'agen_analis',
    nama: 'Dian Kartika, MBA',
    panggilan: 'Dian',
    gender: 'Wanita',
    peran: 'Analis Bisnis & Produk',
    gelar: 'Lead Business Analyst • CBAP®',
    kluster: 'Business & Comms Pod',
    sop: 'SOP-02',
    sopFile: 'SOP-02_Analis_Bisnis.md',
    avatar: '👩‍💼',
    warna: '#06b6d4',
    status: '🟡 IDLE — Santai & Siaga',
    tugas: '— Tidak ada beban kerja aktif —',
    beban: 0
  },
  {
    id: 'agen_arsitek',
    nama: 'Bambang Prasetyo, M.T.',
    panggilan: 'Bambang',
    gender: 'Pria',
    peran: 'Arsitek Sistem & Database',
    gelar: 'Principal System Architect • TOGAF®',
    kluster: 'Engineering Pod 2x2',
    sop: 'SOP-03',
    sopFile: 'SOP-03_Arsitek_Sistem.md',
    avatar: '👨‍💻',
    warna: '#6366f1',
    status: '🟡 IDLE — Santai & Siaga',
    tugas: '— Tidak ada beban kerja aktif —',
    beban: 0
  },
  {
    id: 'agen_coder',
    nama: 'Rizky Firmansyah',
    panggilan: 'Rizky',
    gender: 'Pria',
    peran: 'Lead Software Engineer',
    gelar: 'Core Widya-Lang Developer • Rust & C',
    kluster: 'Engineering Pod 2x2',
    sop: 'SOP-04',
    sopFile: 'SOP-04_Lead_Developer.md',
    avatar: '👨‍💻',
    warna: '#10b981',
    status: '🟡 IDLE — Santai & Siaga',
    tugas: '— Tidak ada beban kerja aktif —',
    beban: 0
  },
  {
    id: 'agen_qa',
    nama: 'Siti Rahmawati, S.Kom.',
    panggilan: 'Siti',
    gender: 'Wanita',
    peran: 'Lead QA & Automation Tester',
    gelar: 'QA Automation Lead • ISTQB®',
    kluster: 'Engineering Pod 2x2',
    sop: 'SOP-05',
    sopFile: 'SOP-05_QA_Tester.md',
    avatar: '👩‍🔬',
    warna: '#ec4899',
    status: '🟡 IDLE — Santai & Siaga',
    tugas: '— Tidak ada beban kerja aktif —',
    beban: 0
  },
  {
    id: 'agen_devops',
    nama: 'Fajar Nugroho',
    panggilan: 'Fajar',
    gender: 'Pria',
    peran: 'DevOps & SRE Specialist',
    gelar: 'Site Reliability Engineer • CKA / SRE',
    kluster: 'Engineering Pod 2x2',
    sop: 'SOP-06',
    sopFile: 'SOP-06_DevOps_SRE.md',
    avatar: '👨‍🔧',
    warna: '#f59e0b',
    status: '🟡 IDLE — Santai & Siaga',
    tugas: '— Tidak ada beban kerja aktif —',
    beban: 0
  },
  {
    id: 'agen_keamanan',
    nama: 'Hendra Gunawan, CISA',
    panggilan: 'Hendra',
    gender: 'Pria',
    peran: 'Auditor Siber & UU PDP',
    gelar: 'Cyber Security Officer • CISSP / CISA',
    kluster: 'Business & Comms Pod',
    sop: 'SOP-07',
    sopFile: 'SOP-07_Auditor_Keamanan.md',
    avatar: '👮‍♂️',
    warna: '#ef4444',
    status: '🟡 IDLE — Santai & Siaga',
    tugas: '— Tidak ada beban kerja aktif —',
    beban: 0
  },
  {
    id: 'agen_notulis',
    nama: 'Ratna Sari',
    panggilan: 'Ratna',
    gender: 'Wanita',
    peran: 'Pustakawan & Notulis',
    gelar: 'Pustakawan Digital & Obsidian Scribe Lead',
    kluster: 'Business & Comms Pod',
    sop: 'SOP-08',
    sopFile: 'SOP-08_Notulis_Obsidian.md',
    avatar: '👩‍🏫',
    warna: '#a855f7',
    status: '🟡 IDLE — Santai & Siaga',
    tugas: '— Tidak ada beban kerja aktif —',
    beban: 0
  },
  {
    id: 'agen_humas',
    nama: 'Bayu Pratama',
    panggilan: 'Bayu',
    gender: 'Pria',
    peran: 'Humas & Telegram Bot Gateway',
    gelar: 'Communications Officer • Telegram & WhatsApp Lead',
    kluster: 'Business & Comms Pod',
    sop: 'SOP-09',
    sopFile: 'SOP-09_Humas_Telegram.md',
    avatar: '👨‍💼',
    warna: '#0284c7',
    status: '🟡 IDLE — Santai & Siaga',
    tugas: '— Tidak ada beban kerja aktif —',
    beban: 0
  },
  {
    id: 'agen_analis_data',
    nama: 'Dewi Lestari, M.Sc.',
    panggilan: 'Dewi',
    gender: 'Wanita',
    peran: 'Kepala Intelijen Data & KPI',
    gelar: 'Analytics Lead • Balanced Scorecard • DAMA-DMBOK',
    kluster: 'Analytics Hub',
    sop: 'SOP-10',
    sopFile: 'SOP-10_Analis_Data_KPI.md',
    avatar: '👩‍💻',
    warna: '#f97316',
    status: '🟡 IDLE — Santai & Siaga',
    tugas: '— Tidak ada beban kerja aktif —',
    beban: 0,
    proyekAktif: ''
  }
];

// 2. KANBAN STATE — Dinamis terhubung ke WidyaBase
let kanbanTasks = {
  todo: [],
  progress: [],
  done: []
};

// 3. SINTESIS SUARA WEB AUDIO API (Zero File Eksternal)
class SoundSystem {
  constructor() {
    this.ctx = null;
    this.enabled = true;
  }

  init() {
    if (!this.ctx) {
      const AudioCtx = window.AudioContext || window.webkitAudioContext;
      this.ctx = new AudioCtx();
    }
    if (this.ctx.state === 'suspended') {
      this.ctx.resume();
    }
  }

  // Lonceng Rapat Harmonis (Dual Sine Chime)
  playMeetingChime() {
    if (!this.enabled) return;
    this.init();
    const now = this.ctx.currentTime;
    
    // Nada 1 (C5 - 523.25 Hz)
    const osc1 = this.ctx.createOscillator();
    const gain1 = this.ctx.createGain();
    osc1.type = 'sine';
    osc1.frequency.setValueAtTime(523.25, now);
    gain1.gain.setValueAtTime(0.25, now);
    gain1.gain.exponentialRampToValueAtTime(0.001, now + 1.2);
    osc1.connect(gain1);
    gain1.connect(this.ctx.destination);
    osc1.start(now);
    osc1.stop(now + 1.2);

    // Nada 2 (G5 - 783.99 Hz)
    const osc2 = this.ctx.createOscillator();
    const gain2 = this.ctx.createGain();
    osc2.type = 'sine';
    osc2.frequency.setValueAtTime(783.99, now + 0.15);
    gain2.gain.setValueAtTime(0.25, now + 0.15);
    gain2.gain.exponentialRampToValueAtTime(0.001, now + 1.5);
    osc2.connect(gain2);
    gain2.connect(this.ctx.destination);
    osc2.start(now + 0.15);
    osc2.stop(now + 1.5);
  }

  // Nada Notifikasi Telegram "Chime!"
  playTelegramPing() {
    if (!this.enabled) return;
    this.init();
    const now = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'triangle';
    osc.frequency.setValueAtTime(880, now); // A5
    osc.frequency.exponentialRampToValueAtTime(1318.51, now + 0.1); // E6
    gain.gain.setValueAtTime(0.2, now);
    gain.gain.exponentialRampToValueAtTime(0.001, now + 0.4);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(now);
    osc.stop(now + 0.4);
  }

  // Backward compatibility alias
  playWhatsAppPing() {
    this.playTelegramPing();
  }

  // Suara Klik Ringan
  playClick() {
    if (!this.enabled) return;
    this.init();
    const now = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(600, now);
    gain.gain.setValueAtTime(0.08, now);
    gain.gain.exponentialRampToValueAtTime(0.001, now + 0.05);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(now);
    osc.stop(now + 0.05);
  }

  // Chime Menutup Kantor (Nada Menurun Santai)
  playOfficeClose() {
    if (!this.enabled) return;
    this.init();
    const now = this.ctx.currentTime;
    [659.25, 523.25, 392.00, 261.63].forEach((freq, idx) => {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'sine';
      osc.frequency.setValueAtTime(freq, now + idx * 0.15);
      gain.gain.setValueAtTime(0.18, now + idx * 0.15);
      gain.gain.exponentialRampToValueAtTime(0.001, now + idx * 0.15 + 0.8);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start(now + idx * 0.15);
      osc.stop(now + idx * 0.15 + 0.8);
    });
  }

  // Chime Membuka Kembali Kantor (Nada Menaik Energik)
  playOfficeOpen() {
    if (!this.enabled) return;
    this.init();
    const now = this.ctx.currentTime;
    [261.63, 329.63, 392.00, 523.25].forEach((freq, idx) => {
      const osc = this.ctx.createOscillator();
      const gain = this.ctx.createGain();
      osc.type = 'triangle';
      osc.frequency.setValueAtTime(freq, now + idx * 0.12);
      gain.gain.setValueAtTime(0.2, now + idx * 0.12);
      gain.gain.exponentialRampToValueAtTime(0.001, now + idx * 0.12 + 0.9);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start(now + idx * 0.12);
      osc.stop(now + idx * 0.12 + 0.9);
    });
  }
}

// 4. MAIN CONTROLLER
document.addEventListener('DOMContentLoaded', () => {
  let office = null;
  try {
    if (typeof THREE !== 'undefined' && typeof THREE.OrbitControls !== 'undefined') {
      office = new Office3D('canvas-container');
      office.setupWorkstations(AGENTS_DATA);
      console.log('✓ Office3D successfully initialized');
    } else {
      console.warn('Three.js / OrbitControls not loaded, running in UI overlay mode');
    }
  } catch (err) {
    console.error('Office3D initialization error:', err);
  }

  const sound = new SoundSystem();
  refreshProjectsAndKanban();
  initProgressStripInteractions();

  // Tombol Minimize / Expand Roster Kiri
  const btnMinRoster = document.getElementById('btn-minimize-roster');
  const rosterPanel = document.getElementById('roster-panel');
  if (btnMinRoster && rosterPanel) {
    btnMinRoster.addEventListener('click', () => {
      sound.playClick();
      rosterPanel.classList.toggle('minimized');
      btnMinRoster.textContent = rosterPanel.classList.contains('minimized') ? '▶' : '◀';
    });
  }

  // Render Roster Kiri
  const rosterContainer = document.getElementById('roster-list-container');
  AGENTS_DATA.forEach((agent) => {
    const item = document.createElement('div');
    item.className = 'roster-item';
    item.id = `roster-${agent.id}`;
    const avatarGraphic = (typeof getAgentAvatarSvg === 'function')
      ? getAgentAvatarSvg(agent.id, 32)
      : agent.avatar;

    item.innerHTML = `
      <div class="roster-avatar">${avatarGraphic}</div>
      <div class="roster-info">
        <div class="roster-name">${agent.nama}</div>
        <div class="roster-role">${agent.sop} • ${agent.gelar}</div>
      </div>
      <div class="roster-status-dot" style="background: ${agent.warna}"></div>
    `;
    item.addEventListener('click', () => {
      sound.playClick();
      selectAgent(agent.id);
    });
    rosterContainer.appendChild(item);
  });

  // Pilih Agen & Tampilkan Dossier
  function selectAgent(agentId) {
    const agent = AGENTS_DATA.find((a) => a.id === agentId);
    if (!agent) return;

    // Tandai di Roster
    document.querySelectorAll('.roster-item').forEach((el) => el.classList.remove('active'));
    const activeItem = document.getElementById(`roster-${agentId}`);
    if (activeItem) activeItem.classList.add('active');

    // Isi Data di Dossier Card Kanan
    document.getElementById('dossier-sop').textContent = agent.sop;
    const dossierAvatar = document.getElementById('dossier-avatar');
    if (dossierAvatar) {
      dossierAvatar.innerHTML = (typeof getAgentAvatarSvg === 'function')
        ? getAgentAvatarSvg(agent.id, 52)
        : agent.avatar;
    }
    document.getElementById('dossier-name').textContent = agent.nama;
    document.getElementById('dossier-role').textContent = agent.gelar;
    document.getElementById('dossier-status').textContent = agent.status;
    document.getElementById('dossier-task').textContent = agent.tugas;
    document.getElementById('dossier-load-val').textContent = `${agent.beban}%`;
    document.getElementById('dossier-load-fill').style.width = `${agent.beban}%`;

    // Metadata Gender & Kluster Departemen
    const elGender = document.getElementById('dossier-gender');
    if (elGender) {
      elGender.textContent = agent.gender === 'Wanita' ? '👩 Wanita' : '👨 Pria';
      elGender.className = `dossier-tag-pill ${agent.gender === 'Wanita' ? 'gender-wanita' : 'gender-pria'}`;
    }
    const elCluster = document.getElementById('dossier-cluster');
    if (elCluster) {
      elCluster.textContent = agent.kluster || 'Office Pod';
    }

    // Sorot Tag Visual 3D yang aktif
    document.querySelectorAll('.agent-floating-tag').forEach(t => t.classList.remove('active'));
    const activeTag = document.querySelector(`.agent-floating-tag[data-agent-id="${agentId}"]`);
    if (activeTag) activeTag.classList.add('active');

    const dossier = document.getElementById('agent-dossier');
    dossier.style.display = 'block';

    // Tombol Dekati Meja
    document.getElementById('btn-fly-agent').onclick = () => {
      sound.playClick();
      const a3D = office && office.agents3D ? office.agents3D.get(agentId) : null;
      if (office && a3D) {
        office.setCameraView('focus', a3D.isMeeting ? a3D.meetPos : a3D.deskPos);
      }
    };

    // Tombol Baca SOP
    document.getElementById('btn-read-sop').onclick = () => {
      sound.playClick();
      openSOPModal(agent);
    };
  }

  // Tangkap event klik 3D
  window.addEventListener('agent-selected', (e) => {
    selectAgent(e.detail.agentId);
  });

  // Tombol Tutup Dossier
  document.getElementById('btn-close-dossier').addEventListener('click', () => {
    sound.playClick();
    document.getElementById('agent-dossier').style.display = 'none';
  });

  // ================= ALUR RAPAT PAGI (08:00 WIB) =================
  document.getElementById('btn-trigger-morning').addEventListener('click', () => {
    sound.playMeetingChime();
    if (office) office.gatherAgentsForMeeting();

    const holo = document.getElementById('meeting-hologram');
    document.getElementById('holo-badge').textContent = 'SIDANG RESMI PAGI • 08:00 WIB';
    document.getElementById('holo-title').textContent = '🎯 3 Prioritas Utama & Hard Deadline';

    const cardsContainer = document.getElementById('holo-cards-container');
    cardsContainer.innerHTML = `
      <div class="holo-card-item priority-1">
        <strong>🔐 [P1] Implementasi Otentikasi Argon2id & Session Guard</strong><br>
        <span style="color: #94a3b8">PIC: Lead Coder & Auditor Keamanan • Deadline: 14:00 WIB</span>
      </div>
      <div class="holo-card-item priority-2">
        <strong>🗄️ [P2] Penyusunan Skema Basis Data SQLite & Migrasi Tabel</strong><br>
        <span style="color: #94a3b8">PIC: Arsitek Sistem • Deadline: 11:30 WIB</span>
      </div>
      <div class="holo-card-item priority-3">
        <strong>⚡ [P3] Pengujian Beban Jalur Real-Time Server-Sent Events</strong><br>
        <span style="color: #94a3b8">PIC: QA Tester & DevOps SRE • Deadline: 16:30 WIB</span>
      </div>
    `;

    document.getElementById('holo-obsidian-file').textContent = 'DailyNotes/2026-09-26-Pagi.md (Tersimpan)';
    const tgTgt1 = document.getElementById('holo-tg-target') || document.getElementById('holo-wa-target'); if (tgTgt1) tgTgt1.textContent = '@WidyaDigitalBot (Terkirim via Bot API)';

    holo.style.display = 'block';

    // Picu eksekusi riil di backend jika server aktif
    try {
      fetch('/api/rapat/pagi', { method: 'POST' }).catch(() => {});
    } catch (e) {}

    // Picu audio notifikasi WhatsApp sesaat setelah sidang
    setTimeout(() => sound.playTelegramPing(), 1800);
  });

  // ================= ALUR RAPAT MALAM (20:00 WIB) =================
  document.getElementById('btn-trigger-evening').addEventListener('click', () => {
    sound.playMeetingChime();
    if (office) office.toggleDayNight(true); // Redupkan lampu ke suasana malam
    document.getElementById('daynight-icon').textContent = '🌙';
    document.body.className = 'theme-night';

    if (office) office.gatherAgentsForMeeting();

    const holo = document.getElementById('meeting-hologram');
    document.getElementById('holo-badge').textContent = 'EVALUASI MALAM • 20:00 WIB';
    document.getElementById('holo-title').textContent = '🌙 Rekapitulasi Capaian & Usulan Esok';

    const cardsContainer = document.getElementById('holo-cards-container');
    cardsContainer.innerHTML = `
      <div class="holo-card-item" style="border-left-color: #10b981;">
        <strong>✅ Capaian Hari Ini (Verified QA):</strong><br>
        <span style="color: #cbd5e1">• Otentikasi Argon2id & Session Cookies [SUKSES - Lolos UU PDP]</span><br>
        <span style="color: #cbd5e1">• Skema Basis Data SQLite Inventaris [SUKSES - Termigrasi]</span><br>
        <span style="color: #cbd5e1">• Backup Snapshot Harian [SUKSES - 100% Utuh]</span>
      </div>
      <div class="holo-card-item" style="border-left-color: #f59e0b;">
        <strong>⚠️ Kendala Teknis (Blocker):</strong><br>
        <span style="color: #fbbf24">Pengujian beban streaming SSE memerlukan penyesuaian buffer pool memori.</span>
      </div>
      <div class="holo-card-item" style="border-left-color: #a855f7;">
        <strong>🔮 Usulan 3 Prioritas Besok:</strong><br>
        <span style="color: #e2e8f0">1. Buffer pool & tuntas uji beban SSE • 2. UI Glassmorphism • 3. Audit Log Kripto</span>
      </div>
    `;

    document.getElementById('holo-obsidian-file').textContent = 'DailyNotes/2026-09-26-Malam.md (Tersimpan)';
    const tgTgt2 = document.getElementById('holo-tg-target') || document.getElementById('holo-wa-target'); if (tgTgt2) tgTgt2.textContent = '@WidyaDigitalBot (Terkirim via Bot API)';

    holo.style.display = 'block';

    // Picu eksekusi riil di backend jika server aktif
    try {
      fetch('/api/rapat/malam', { method: 'POST' }).catch(() => {});
    } catch (e) {}

    setTimeout(() => sound.playTelegramPing(), 1800);
  });

  // Fungsi Tutup / Bubarkan Rapat
  function closeMeetingPopup() {
    sound.playClick();
    const holo = document.getElementById('meeting-hologram');
    if (holo) holo.style.display = 'none';
    if (office) office.dismissMeetingToDesks();
  }

  const btnDismissMeeting = document.getElementById('btn-dismiss-meeting');
  if (btnDismissMeeting) btnDismissMeeting.addEventListener('click', closeMeetingPopup);

  const btnCloseHoloX = document.getElementById('btn-close-meeting-x');
  if (btnCloseHoloX) btnCloseHoloX.addEventListener('click', closeMeetingPopup);

  // Keyboard Escape Handler untuk Menutup Seluruh Modal / Popup yang Sedang Aktif
  window.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      const holo = document.getElementById('meeting-hologram');
      if (holo && holo.style.display !== 'none') {
        closeMeetingPopup();
        return;
      }
      ['sop-modal', 'widyabase-modal', 'kanban-modal', 'telegram-modal', 'wa-modal', 'modal-direct-comms', 'modal-add-task', 'modal-add-project'].forEach(id => {
        const m = document.getElementById(id);
        if (m && m.style.display !== 'none') {
          m.style.display = 'none';
        }
      });
    }
  });

  // ================= MODAL KANBAN OBSIDIAN =================
  document.getElementById('btn-open-kanban').addEventListener('click', () => {
    sound.playClick();
    renderKanban();
    document.getElementById('kanban-modal').style.display = 'flex';
  });

  document.getElementById('btn-close-kanban').addEventListener('click', () => {
    sound.playClick();
    document.getElementById('kanban-modal').style.display = 'none';
  });

  // ================= MODAL KANBAN OBSIDIAN & WIDYABASE =================
  let cachedProjects = [];
  let cachedTasks = [];
  let selectedKanbanProject = 'ALL';

  function refreshProjectsAndKanban() {
    // 1. Ambil seluruh proyek dari WidyaBase
    fetch('/api/widyabase/proyek')
      .then(r => r.json())
      .then(projects => {
        if (Array.isArray(projects)) {
          cachedProjects = projects;
          const activeProjects = projects.filter(p => (p.status || '').toUpperCase() !== 'ARSIP');
          const archivedProjects = projects.filter(p => (p.status || '').toUpperCase() === 'ARSIP');

          const hudPrjLabel = document.getElementById('hud-projects-label');
          if (hudPrjLabel) {
            hudPrjLabel.textContent = `${activeProjects.length} Proyek Aktif${archivedProjects.length > 0 ? ` (${archivedProjects.length} Arsip)` : ''}`;
          }

          const filterSel = document.getElementById('kanban-project-filter');
          if (filterSel) {
            const currentVal = selectedKanbanProject;
            filterSel.innerHTML = '<option value="ALL">🌐 Semua Proyek</option>' + 
              projects.map(p => {
                const isArsip = (p.status || '').toUpperCase() === 'ARSIP';
                return `<option value="${p.kode_proyek}" ${p.kode_proyek === currentVal ? 'selected' : ''}>${isArsip ? '📦 [ARSIP] ' : ''}${p.kode_proyek} • ${p.nama_proyek}</option>`;
              }).join('');
          }

          const taskPrjSel = document.getElementById('input-task-project');
          if (taskPrjSel) {
            taskPrjSel.innerHTML = activeProjects.map(p => `<option value="${p.kode_proyek}">${p.kode_proyek} • ${p.nama_proyek}</option>`).join('');
          }
        }
      })
      .catch(() => {});

    // 2. Ambil seluruh tugas kerja dari WidyaBase & Persistent Store
    fetch('/api/widyabase/tugas')
      .then(r => r.json())
      .then(tugasList => {
        if (Array.isArray(tugasList) && tugasList.length > 0) {
          cachedTasks = tugasList;
          
          // Perbarui indikator ringkasan portofolio di HUD atas (hanya hitung proyek aktif)
          const activeProjects = (cachedProjects || []).filter(p => (p.status || '').toUpperCase() !== 'ARSIP');
          const activePrjCodes = new Set(activeProjects.map(p => p.kode_proyek));
          
          const relevantTasks = cachedTasks.filter(t => activePrjCodes.size === 0 || activePrjCodes.has(t.kode_proyek));
          const totalAll = relevantTasks.length;
          const doneAll = relevantTasks.filter(t => ['SELESAI', 'DONE', 'VERIFIED'].includes((t.status || '').toUpperCase())).length;
          const overallPct = totalAll > 0 ? Math.round((doneAll / totalAll) * 100) : 0;
          const hudProgress = document.getElementById('hud-progress-label');
          if (hudProgress) {
            hudProgress.textContent = `Portofolio: ${overallPct}% (${doneAll}/${totalAll} Selesai)`;
          }

          updateAgentWorkloads();
          renderProjectProgressStrip();
          renderKanbanUI();
        } else {
          updateAgentWorkloads();
          renderKanbanFallback();
        }
      })
      .catch(() => {
        updateAgentWorkloads();
        renderKanbanFallback();
      });
  }

  // Hitung Beban Kerja Dinamis Agen Berdasarkan Antrean Tugas Riil (Kecuali Proyek yang Diarsipkan)
  function updateAgentWorkloads() {
    window.AGENT_WORKLOAD = window.AGENT_WORKLOAD || {};

    const activeProjects = (cachedProjects || []).filter(p => (p.status || '').toUpperCase() !== 'ARSIP');
    const activePrjCodes = new Set(activeProjects.map(p => p.kode_proyek));

    AGENTS_DATA.forEach(ag => {
      // Filter tugas aktif untuk agen ini (hanya dari proyek aktif non-arsip)
      const activeTasks = (cachedTasks || []).filter(t => {
        const isAgent = (t.pic_agen === ag.id || t.pic_agen === ag.nama || t.sop_referensi === ag.sop || (t.pic_agen && t.pic_agen.toLowerCase().includes(ag.panggilan.toLowerCase())));
        const status = (t.status || '').toUpperCase();
        const isActive = ['PROSES', 'IN PROGRESS', 'AKTIF', 'TINJAUAN', 'TODO'].includes(status);
        const isProjectActive = activePrjCodes.size === 0 || activePrjCodes.has(t.kode_proyek);

        return isAgent && isActive && isProjectActive;
      });

      if (activeTasks.length === 0) {
        ag.beban = 0;
        ag.status = '🟡 IDLE — Santai & Siap Tugas';
        ag.tugas = '— Tidak ada beban kerja aktif (idle / santai di kantor) —';
      } else {
        const inProg = activeTasks.filter(t => ['PROSES', 'IN PROGRESS', 'AKTIF'].includes((t.status || '').toUpperCase()));
        ag.beban = Math.min(100, Math.max(25, activeTasks.length * 30));
        ag.status = inProg.length > 0 ? '🟢 Sedang Eksekusi Tugas' : '🔵 Antrean Tugas';
        ag.tugas = (inProg[0] || activeTasks[0]).judul_tugas || (inProg[0] || activeTasks[0]).nama_tugas || 'Mengerjakan tugas proyek';
      }

      window.AGENT_WORKLOAD[ag.id] = ag.beban;

      // Update badge dot di roster jika ada
      const rosterItem = document.getElementById(`roster-${ag.id}`);
      if (rosterItem) {
        const dot = rosterItem.querySelector('.roster-status-dot');
        if (dot) {
          dot.style.background = ag.beban === 0 ? '#eab308' : ag.warna;
          dot.title = `Beban Kerja: ${ag.beban}%`;
        }
      }
    });

    // Update panel dossier jika ada yang sedang terbuka
    const activeRoster = document.querySelector('.roster-item.active');
    if (activeRoster) {
      const activeId = activeRoster.id.replace('roster-', '');
      const curAgent = AGENTS_DATA.find(a => a.id === activeId);
      if (curAgent) {
        const loadVal = document.getElementById('dossier-load-val');
        const loadFill = document.getElementById('dossier-load-fill');
        const taskEl = document.getElementById('dossier-task');
        const statEl = document.getElementById('dossier-status');
        if (loadVal) loadVal.textContent = `${curAgent.beban}%`;
        if (loadFill) loadFill.style.width = `${curAgent.beban}%`;
        if (taskEl) taskEl.textContent = curAgent.tugas;
        if (statEl) statEl.textContent = curAgent.status;
      }
    }
  }

  // Fungsi Toggle Arsip Proyek (Arsipkan / Aktifkan Kembali)

  // Render Visual Strip Progres Portofolio Per Proyek (Hanya Menampilkan Proyek Aktif)
  function renderProjectProgressStrip() {
    const stripEl = document.getElementById('project-progress-strip');
    const archiveListEl = document.getElementById('project-archive-list');
    const archiveCountBadge = document.getElementById('archive-count-badge');
    if (!stripEl) return;

    const allProjects = (cachedProjects && cachedProjects.length > 0) ? cachedProjects : [
      { kode_proyek: 'PRJ-01-KOPERASI', nama_proyek: 'Koperasi Merah Putih & Simpan Pinjam Syariah', status: 'AKTIF' },
      { kode_proyek: 'PRJ-02-SMARTCITY', nama_proyek: 'Platform Telemetri ISPU & GIS Smart City', status: 'AKTIF' },
      { kode_proyek: 'PRJ-03-COLLAB-DOC', nama_proyek: 'Kolaborasi Dokumen Realtime CRDT & Markdown', status: 'AKTIF' },
      { kode_proyek: 'PRJ-04-DAPODIK-AUTO', nama_proyek: 'Otomasi Integrasi Data Pokok Pendidikan', status: 'AKTIF' },
      { kode_proyek: 'PRJ-05-LATEX-JOURNAL', nama_proyek: 'Generator Jurnal Ilmiah & Kompilator LaTeX', status: 'AKTIF' },
      { kode_proyek: 'PRJ-06-WIDYAPAY', nama_proyek: 'WidyaPay Switcher QRIS & Core Banking', status: 'AKTIF' },
      { kode_proyek: 'PRJ-07-WIDYAMEDIKA', nama_proyek: 'WidyaMedika SIMRS & SATUSEHAT HL7 FHIR', status: 'AKTIF' },
      { kode_proyek: 'PRJ-08-WIDYAOS', nama_proyek: 'WidyaOS Bare-Metal Rust Kernel & OEM', status: 'AKTIF' },
      { kode_proyek: 'PRJ-09-WIDYAAI-BOX', nama_proyek: 'WidyaAI Box Offline & Cyber Shield PQC', status: 'AKTIF' },
      { kode_proyek: 'PRJ-10-WIDYALEDGER', nama_proyek: 'WidyaLedger Sovereign Blockchain & RWA', status: 'AKTIF' }
    ];

    const activeProjects = allProjects.filter(p => (p.status || '').toUpperCase() !== 'ARSIP');
    const archivedProjects = allProjects.filter(p => (p.status || '').toUpperCase() === 'ARSIP');

    if (archiveCountBadge) archiveCountBadge.textContent = archivedProjects.length;

    const activePrjCodes = new Set(activeProjects.map(p => p.kode_proyek));
    const relevantTasks = cachedTasks.filter(t => activePrjCodes.size === 0 || activePrjCodes.has(t.kode_proyek));

    const totalAll = relevantTasks.length;
    const doneAll = relevantTasks.filter(t => ['SELESAI', 'DONE', 'VERIFIED'].includes((t.status || '').toUpperCase())).length;
    const progAll = relevantTasks.filter(t => ['PROSES', 'IN PROGRESS'].includes((t.status || '').toUpperCase())).length;
    const todoAll = relevantTasks.filter(t => (t.status || '').toUpperCase() === 'TODO').length;
    const overallPct = totalAll > 0 ? Math.round((doneAll / totalAll) * 100) : (activeProjects.length === 0 ? 100 : 0);

    // 1. Render Kartu Semua Proyek & Kartu Proyek Aktif
    let html = `
      <div class="project-progress-card ${selectedKanbanProject === 'ALL' ? 'active' : ''}" data-prj-code="ALL">
        <div class="card-prj-header">
          <span class="card-prj-code">🌐 SEMUA PROYEK AKTIF</span>
          <span class="card-prj-pct">${overallPct}%</span>
        </div>
        <div class="card-prj-name">Portofolio Berjalan (${activeProjects.length} Proyek)</div>
        <div class="card-prog-bar-bg">
          <div class="card-prog-bar-fill" style="width: ${overallPct}%;"></div>
        </div>
        <div class="card-prj-counts">
          <span>✅ ${doneAll} Selesai</span>
          <span>⏳ ${progAll}</span>
          <span>📝 ${todoAll}</span>
        </div>
      </div>
    `;

    if (activeProjects.length === 0) {
      html += `
        <div class="empty-active-strip">
          <span style="font-size: 16pt;">🎉</span>
          <div>
            <div style="font-weight: 700; color: #34d399; font-size: 8pt;">Seluruh Proyek Telah Tuntas & Diarsipkan!</div>
            <div style="font-size: 7pt; color: #94a3b8;">Beban kerja tim saat ini 0% (Mode IDLE). Proyek yang telah selesai tersimpan rapi di <b>Brankas Arsip</b> di atas.</div>
          </div>
        </div>
      `;
    } else {
      activeProjects.forEach(p => {
        const prjTasks = cachedTasks.filter(t => t.kode_proyek === p.kode_proyek);
        const prjTotal = prjTasks.length;
        const prjDone = prjTasks.filter(t => ['SELESAI', 'DONE', 'VERIFIED'].includes((t.status || '').toUpperCase())).length;
        const prjProg = prjTasks.filter(t => ['PROSES', 'IN PROGRESS'].includes((t.status || '').toUpperCase())).length;
        const prjTodo = prjTasks.filter(t => (t.status || '').toUpperCase() === 'TODO').length;
        const pct = prjTotal > 0 ? Math.round((prjDone / prjTotal) * 100) : 0;

        html += `
          <div class="project-progress-card ${selectedKanbanProject === p.kode_proyek ? 'active' : ''}" data-prj-code="${p.kode_proyek}">
            <div class="card-prj-header">
              <span class="card-prj-code">${p.kode_proyek}</span>
              <span class="card-prj-pct">${pct}%</span>
            </div>
            <div class="card-prj-name" title="${p.nama_proyek}">${p.nama_proyek}</div>
            <div class="card-prog-bar-bg">
              <div class="card-prog-bar-fill" style="width: ${pct}%;"></div>
            </div>
            <div class="card-prj-counts">
              <span>✅ ${prjDone}/${prjTotal}</span>
              <span>⏳ ${prjProg}</span>
              <button class="btn-archive-toggle" data-kode="${p.kode_proyek}" data-status="AKTIF" title="Pindahkan proyek ini ke Brankas Arsip">
                ${pct === 100 ? '📦 Arsipkan' : '📦 Arsip'}
              </button>
            </div>
          </div>
        `;
      });
    }

    stripEl.innerHTML = html;

    // 2. Render Brankas Arsip Khusus
    if (archiveListEl) {
      if (archivedProjects.length === 0) {
        archiveListEl.innerHTML = '<div style="font-size: 7.2pt; color: #64748b; padding: 6px 10px;">Belum ada proyek yang diarsipkan. Proyek yang telah selesai (100%) dapat dipindahkan ke sini melalui tombol <b>Arsipkan</b>.</div>';
      } else {
        archiveListEl.innerHTML = archivedProjects.map(ap => {
          const prjTasks = cachedTasks.filter(t => t.kode_proyek === ap.kode_proyek);
          const prjTotal = prjTasks.length;
          const prjDone = prjTasks.filter(t => ['SELESAI', 'DONE', 'VERIFIED'].includes((t.status || '').toUpperCase())).length;
          return `
            <div class="archive-project-card" data-prj-code="${ap.kode_proyek}">
              <div class="archive-card-top">
                <span class="archive-card-code">${ap.kode_proyek}</span>
                <span class="archive-card-badge">✅ SELESAI</span>
              </div>
              <div class="archive-card-name" title="${ap.nama_proyek}">${ap.nama_proyek}</div>
              <div class="archive-card-actions">
                <span>📁 ${prjDone}/${prjTotal} Tiket</span>
                <button class="btn-restore-project" data-kode="${ap.kode_proyek}" title="Aktifkan kembali proyek ke bilah progres">
                  📂 Pulihkan
                </button>
              </div>
            </div>
          `;
        }).join('');
      }
    }

    // Pasang listener klik pada kartu untuk menyaring kanban
    stripEl.querySelectorAll('.project-progress-card').forEach(c => {
      c.addEventListener('click', (e) => {
        if (e.target.closest('.btn-archive-toggle')) return;
        sound.playClick();
        selectedKanbanProject = c.getAttribute('data-prj-code');
        const filterSel = document.getElementById('kanban-project-filter');
        if (filterSel) filterSel.value = selectedKanbanProject;
        renderProjectProgressStrip();
        renderKanbanUI();
      });
    });

    // Pasang listener tombol arsip di bar aktif
    stripEl.querySelectorAll('.btn-archive-toggle').forEach(btn => {
      btn.addEventListener('click', (e) => {
        e.stopPropagation();
        const kode = btn.getAttribute('data-kode');
        const status = btn.getAttribute('data-status');
        toggleArchiveProject(kode, status);
      });
    });

    // Pasang listener tombol pulihkan di brankas arsip
    if (archiveListEl) {
      archiveListEl.querySelectorAll('.btn-restore-project').forEach(btn => {
        btn.addEventListener('click', (e) => {
          e.stopPropagation();
          const kode = btn.getAttribute('data-kode');
          toggleArchiveProject(kode, 'ARSIP');
        });
      });
    }
  }

  // Fungsi Arsipkan / Pulihkan Proyek dari Bilah Progres & Brankas
  async function toggleArchiveProject(kodePrj, currentStatus) {
    if (!kodePrj) return;
    sound.playClick();
    const nextStatus = (currentStatus || '').toUpperCase() === 'ARSIP' ? 'AKTIF' : 'ARSIP';

    // 1. Perbarui status proyek di cache lokal
    const pIdx = cachedProjects.findIndex(p => p.kode_proyek === kodePrj);
    if (pIdx >= 0) {
      cachedProjects[pIdx].status = nextStatus;
    } else {
      cachedProjects.push({ kode_proyek: kodePrj, nama_proyek: kodePrj, status: nextStatus });
    }

    // Jika proyek yang difilter diarsipkan, kembalikan filter kanban ke SEMUA
    if (selectedKanbanProject === kodePrj && nextStatus === 'ARSIP') {
      selectedKanbanProject = 'ALL';
    }

    // 2. Render ulang antarmuka progres, kanban, dan beban agen
    renderProjectProgressStrip();
    updateAgentWorkloads();
    renderKanbanUI();

    // 3. Perbarui dropdown filter proyek
    const activeProjects = cachedProjects.filter(p => (p.status || '').toUpperCase() !== 'ARSIP');
    const archivedProjects = cachedProjects.filter(p => (p.status || '').toUpperCase() === 'ARSIP');

    const hudPrjLabel = document.getElementById('hud-projects-label');
    if (hudPrjLabel) {
      hudPrjLabel.textContent = `${activeProjects.length} Proyek Aktif${archivedProjects.length > 0 ? ` (${archivedProjects.length} Arsip)` : ''}`;
    }

    const filterSel = document.getElementById('kanban-project-filter');
    if (filterSel) {
      filterSel.innerHTML = '<option value="ALL">🌐 Semua Proyek</option>' + 
        cachedProjects.map(p => {
          const isArsip = (p.status || '').toUpperCase() === 'ARSIP';
          return `<option value="${p.kode_proyek}" ${p.kode_proyek === selectedKanbanProject ? 'selected' : ''}>${isArsip ? '📦 [ARSIP] ' : ''}${p.kode_proyek} • ${p.nama_proyek}</option>`;
        }).join('');
    }

    // 4. Siarkan notifikasi percakapan tim
    postAgentChatter({
      from: 'agen_orkestrator',
      msg: nextStatus === 'ARSIP'
        ? `📦 Proyek "${kodePrj}" telah resmi dipindahkan ke Brankas Arsip. Beban kerja tim disesuaikan secara dinamis.`
        : `📂 Proyek "${kodePrj}" telah dipulihkan kembali ke daftar proyek aktif!`
    });

    // 5. Sinkronisasi persistensi backend
    try {
      await fetch('/api/widyabase/arsip-proyek', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ kode_proyek: kodePrj, status: nextStatus })
      });
    } catch (e) {
      console.warn('Gagal sinkron status arsip ke server:', e);
    }
  }

  // Tombol Buka / Tutup Drawer Brankas Arsip
  const btnToggleArchive = document.getElementById('btn-toggle-archive-drawer');
  const drawerArchive = document.getElementById('project-archive-drawer');
  if (btnToggleArchive && drawerArchive) {
    btnToggleArchive.addEventListener('click', () => {
      sound.playClick();
      const isClosed = drawerArchive.style.display === 'none' || !drawerArchive.style.display;
      drawerArchive.style.display = isClosed ? 'block' : 'none';
      btnToggleArchive.classList.toggle('active', isClosed);
    });
  }

  // Interaktivitas Geser / Drag-to-Scroll & Mouse Wheel pada Strip Progres Proyek
  function initProgressStripInteractions() {
    const stripEl = document.getElementById('project-progress-strip');
    if (!stripEl) return;

    let isDown = false;
    let startX = 0;
    let scrollLeft = 0;
    let hasDragged = false;

    // 1. Mouse Drag to Scroll
    stripEl.addEventListener('mousedown', (e) => {
      if (e.target.closest('.btn-archive-toggle') || e.target.closest('.btn-card-advance')) return;
      isDown = true;
      hasDragged = false;
      stripEl.classList.add('is-dragging');
      startX = e.pageX - stripEl.offsetLeft;
      scrollLeft = stripEl.scrollLeft;
    });

    window.addEventListener('mouseup', () => {
      if (isDown) {
        isDown = false;
        stripEl.classList.remove('is-dragging');
      }
    });

    stripEl.addEventListener('mouseleave', () => {
      if (isDown) {
        isDown = false;
        stripEl.classList.remove('is-dragging');
      }
    });

    stripEl.addEventListener('mousemove', (e) => {
      if (!isDown) return;
      e.preventDefault();
      const x = e.pageX - stripEl.offsetLeft;
      const walk = (x - startX) * 1.5; // multiplier kecepatan geser
      if (Math.abs(walk) > 4) {
        hasDragged = true;
      }
      stripEl.scrollLeft = scrollLeft - walk;
    });

    // Cegah klik kartu terpicu saat pengguna melakukan drag
    stripEl.addEventListener('click', (e) => {
      if (hasDragged) {
        e.stopPropagation();
        e.preventDefault();
        hasDragged = false;
      }
    }, true);

    // 2. Mouse Wheel to Horizontal Scroll
    stripEl.addEventListener('wheel', (e) => {
      if (e.deltaY !== 0) {
        e.preventDefault();
        stripEl.scrollLeft += e.deltaY;
      }
    }, { passive: false });

    // 3. Tombol Navigasi Kiri / Kanan
    const btnLeft = document.getElementById('btn-strip-scroll-left');
    const btnRight = document.getElementById('btn-strip-scroll-right');
    if (btnLeft) {
      btnLeft.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        stripEl.scrollBy({ left: -240, behavior: 'smooth' });
      });
    }
    if (btnRight) {
      btnRight.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        stripEl.scrollBy({ left: 240, behavior: 'smooth' });
      });
    }
  }

  function renderKanbanUI() {
    const listTodo = document.getElementById('kanban-list-todo');
    const listProg = document.getElementById('kanban-list-progress');
    const listDone = document.getElementById('kanban-list-done');

    const filterVal = selectedKanbanProject;
    const filtered = filterVal === 'ALL' ? cachedTasks : cachedTasks.filter(t => t.kode_proyek === filterVal);

    const todoItems = filtered.filter(t => (t.status || '').toUpperCase() === 'TODO');
    const progItems = filtered.filter(t => ['PROSES', 'IN PROGRESS'].includes((t.status || '').toUpperCase()));
    const doneItems = filtered.filter(t => ['SELESAI', 'DONE', 'VERIFIED'].includes((t.status || '').toUpperCase()));

    if (listTodo) {
      listTodo.innerHTML = todoItems.map(t => {
        const ag = AGENTS_DATA.find(a => a.id === t.pic_agen);
        const agName = ag ? ag.nama : t.pic_agen;
        return `
          <div class="kanban-card">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
              <span class="card-tag" style="color: #60a5fa;">${t.prioritas || 'P1'} • ${t.sop_referensi || 'SOP'}</span>
              <span style="font-size: 6.8pt; color: #94a3b8; font-family: monospace;">${t.kode_proyek}</span>
            </div>
            <div style="font-weight: 700; color: #f8fafc; font-size: 8.5pt;">[${t.kode_tugas}] ${t.judul_tugas}</div>
            <div style="font-size: 7.2pt; color: #38bdf8; margin-top: 5px;">👤 ${agName} • ⏰ ${t.deadline}</div>
            <div class="kanban-card-actions">
              <button class="btn-card-advance" data-task-id="${t.id_tugas}" data-task-code="${t.kode_tugas}" data-action="PROSES" title="Mulai kerjakan tugas ini">▶️ Kerjakan (Mulai)</button>
            </div>
          </div>
        `;
      }).join('');
      const countEl = document.getElementById('count-todo');
      if (countEl) countEl.textContent = todoItems.length;
    }

    if (listProg) {
      listProg.innerHTML = progItems.map(t => {
        const ag = AGENTS_DATA.find(a => a.id === t.pic_agen);
        const agName = ag ? ag.nama : t.pic_agen;
        return `
          <div class="kanban-card">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
              <span class="card-tag" style="color: #f59e0b;">${t.prioritas || 'P2'} • ${t.sop_referensi || 'SOP'}</span>
              <span style="font-size: 6.8pt; color: #94a3b8; font-family: monospace;">${t.kode_proyek}</span>
            </div>
            <div style="font-weight: 700; color: #f8fafc; font-size: 8.5pt;">[${t.kode_tugas}] ${t.judul_tugas}</div>
            <div style="font-size: 7.2pt; color: #fbbf24; margin-top: 5px;">⏳ ${agName} • ⏰ ${t.deadline}</div>
            <div class="kanban-card-actions">
              <button class="btn-card-advance btn-done-action" data-task-id="${t.id_tugas}" data-task-code="${t.kode_tugas}" data-action="SELESAI" title="Verifikasi dan selesaikan tugas ini">✅ Selesaikan (Lulus QA)</button>
            </div>
          </div>
        `;
      }).join('');
      const countEl = document.getElementById('count-progress');
      if (countEl) countEl.textContent = progItems.length;
    }

    if (listDone) {
      listDone.innerHTML = doneItems.map(t => `
        <div class="kanban-card">
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;">
            <span class="card-tag" style="color: #10b981;">VERIFIED • ${t.sop_referensi || 'SOP'}</span>
            <span style="font-size: 6.8pt; color: #94a3b8; font-family: monospace;">${t.kode_proyek}</span>
          </div>
          <div style="font-weight: 700; color: #f8fafc; font-size: 8.5pt;">[${t.kode_tugas}] ${t.judul_tugas}</div>
          <div style="font-size: 7.2pt; color: #34d399; margin-top: 5px;">✅ ${t.bukti_kerja || 'Lulus Verifikasi QA'}</div>
          <div class="kanban-card-actions">
            <button class="btn-card-advance btn-reopen-action" data-task-id="${t.id_tugas}" data-task-code="${t.kode_tugas}" data-action="PROSES" title="Buka kembali pengerjaan tugas">↩️ Buka Kembali</button>
          </div>
        </div>
      `).join('');
      const countEl = document.getElementById('count-done');
      if (countEl) countEl.textContent = doneItems.length;
    }
  }

  // Fungsi Memajukan Status Tugas secara Interaktif
  async function advanceTaskStatus(taskId, taskCode, nextStatus) {
    sound.playClick();
    const idx = cachedTasks.findIndex(t => (taskId && String(t.id_tugas) === String(taskId)) || (taskCode && (t.kode_tugas || '').toUpperCase() === taskCode.toUpperCase()));
    if (idx >= 0) {
      cachedTasks[idx].status = nextStatus;
      if (nextStatus === 'SELESAI') cachedTasks[idx].bukti_kerja = 'Lulus verifikasi QA & uji regresi otomatis';
      renderProjectProgressStrip();
      updateAgentWorkloads();
      renderKanbanUI();
    }

    try {
      await fetch('/api/widyabase/update-task-status', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id_tugas: taskId, kode_tugas: taskCode, status: nextStatus })
      });
    } catch (e) {
      console.warn('Gagal sinkron update status tugas:', e);
    }
  }

  // Delegasi Event Klik untuk Tombol Aksi Kartu Kanban
  const kanbanModalEl = document.getElementById('kanban-modal');
  if (kanbanModalEl) {
    kanbanModalEl.addEventListener('click', (e) => {
      const btn = e.target.closest('.btn-card-advance');
      if (btn) {
        const taskId = btn.getAttribute('data-task-id');
        const taskCode = btn.getAttribute('data-task-code');
        const action = btn.getAttribute('data-action');
        advanceTaskStatus(taskId, taskCode, action);
      }
    });
  }

  function renderKanbanFallback() {
    const listTodo = document.getElementById('kanban-list-todo');
    const listProg = document.getElementById('kanban-list-progress');
    const listDone = document.getElementById('kanban-list-done');
    if (listTodo) listTodo.innerHTML = kanbanTasks.todo.map(t => `<div class="kanban-card"><div class="card-tag">${t.tag}</div><div>${t.text}</div></div>`).join('');
    if (listProg) listProg.innerHTML = kanbanTasks.progress.map(t => `<div class="kanban-card"><div class="card-tag" style="color: #f59e0b;">${t.tag}</div><div>${t.text}</div></div>`).join('');
    if (listDone) listDone.innerHTML = kanbanTasks.done.map(t => `<div class="kanban-card"><div class="card-tag" style="color: #10b981;">${t.tag}</div><div>${t.text}</div></div>`).join('');
  }

  function renderKanban() {
    refreshProjectsAndKanban();
  }

  const kanbanFilter = document.getElementById('kanban-project-filter');
  if (kanbanFilter) {
    kanbanFilter.addEventListener('change', () => {
      selectedKanbanProject = kanbanFilter.value;
      renderProjectProgressStrip();
      renderKanbanUI();
    });
  }

  const btnHudPrj = document.getElementById('btn-hud-projects');
  if (btnHudPrj) {
    btnHudPrj.addEventListener('click', () => {
      sound.playClick();
      openWidyaBaseModal();
      loadWidyaBaseTable('proyek');
    });
  }

  // ================= MODAL TELEGRAM PREVIEW (Dinamis) =================
  const btnOpenTelegram = document.getElementById('btn-open-telegram') || document.getElementById('btn-open-whatsapp');
  if (btnOpenTelegram) {
    btnOpenTelegram.addEventListener('click', async () => {
      sound.playClick();
      const chatContainer = document.getElementById('telegram-chat-container') || document.getElementById('wa-chat-container');
      if (chatContainer) {
        chatContainer.innerHTML = `<div class="telegram-loading" style="color: #94a3b8; font-size: 8pt; text-align: center; padding: 15px;">⏳ Mengambil transmisi Telegram terbaru...</div>`;
        
        try {
          const res = await fetch('/api/status');
          const data = await res.json();
          const todayStr = new Date().toISOString().slice(0, 10);
          const timeNow = new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }).replace('.', ':');
          
          if (data && data.telegram_terakhir && data.telegram_terakhir.trim().length > 0) {
            const formattedMsg = data.telegram_terakhir.replace(/\n/g, '<br>');
            chatContainer.innerHTML = `
              <div class="telegram-bubble">
                ${formattedMsg}
                <div class="telegram-bubble-time">${timeNow} &bull; Terkirim ✓✓</div>
              </div>
            `;
          } else {
            // Komposisi dinamis berdasarkan progres tugas riil
            const total = (cachedTasks || []).length;
            const done = (cachedTasks || []).filter(t => ['SELESAI', 'DONE', 'VERIFIED'].includes((t.status || '').toUpperCase())).length;
            const active = (cachedTasks || []).filter(t => ['PROSES', 'IN PROGRESS', 'AKTIF'].includes((t.status || '').toUpperCase()));
            const pct = total > 0 ? Math.round((done / total) * 100) : 0;
            
            chatContainer.innerHTML = `
              <div class="telegram-bubble">
                ✈️ <b>WIDYA ENTERPRISE LABS • STATUS TELEGRAM LIVE</b><br>
                📅 <b>${todayStr}</b> | ⏰ <b>${timeNow} WIB</b> | Channel: <code>@WidyaDigitalBot</code><br>
                ━━━━━━━━━━━━━━━━━━━━━━<br>
                Halo Pak Satriyo! Laporan real-time status 10 AI Agent Berdaulat:<br><br>
                📊 <b>STATUS PORTOFOLIO KERJA:</b><br>
                • Total Tiket Tugas: <b>${total}</b><br>
                • Selesai Terverifikasi: <b>${done} (${pct}%)</b><br>
                • Tugas Sedang Aktif: <b>${active.length}</b><br><br>
                🎯 <b>TUGAS AKTIF SAAT INI:</b><br>
                ${active.length > 0 
                  ? active.map(t => `• <b>[${t.kode_proyek}]</b> ${t.judul_tugas} (PIC: <code>${t.pic_agen}</code>)`).join('<br>')
                  : '<i>(Seluruh tugas saat ini selesai — tim dalam status IDLE / santai)</i>'}<br><br>
                📁 <b>Sinkronisasi Vault:</b> <code>C:/Satriyo/Vault</code><br>
                <div class="telegram-bubble-time">${timeNow} &bull; Terkirim ✓✓</div>
              </div>
            `;
          }
        } catch (e) {
          chatContainer.innerHTML = `<div class="telegram-bubble" style="color:#f87171;">Gagal memuat status live Telegram: ${e.message}</div>`;
        }
      }
      const tgModal = document.getElementById('telegram-modal') || document.getElementById('wa-modal');
      if (tgModal) {
        tgModal.style.display = 'flex';
        setTimeout(() => {
          const inp = document.getElementById('telegram-preview-input');
          if (inp) inp.focus();
        }, 100);
      }
    });
  }

  const btnCloseTg = document.getElementById('btn-close-tg') || document.getElementById('btn-close-wa');
  if (btnCloseTg) {
    btnCloseTg.addEventListener('click', () => {
      sound.playClick();
      const tgModal = document.getElementById('telegram-modal') || document.getElementById('wa-modal');
      if (tgModal) tgModal.style.display = 'none';
    });
  }

  // Interaktivitas Pengetikan & Balasan Bot di Pratinjau Smartphone Telegram
  const tgPreviewForm = document.getElementById('telegram-preview-form');
  const tgPreviewInput = document.getElementById('telegram-preview-input');
  if (tgPreviewForm && tgPreviewInput) {
    tgPreviewForm.addEventListener('submit', async (e) => {
      e.preventDefault();
      const text = tgPreviewInput.value.trim();
      if (!text) return;

      sound.playClick();
      const chatContainer = document.getElementById('telegram-chat-container') || document.getElementById('wa-chat-container');
      const timeStr = new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }).replace('.', ':');

      // 1. Tampilkan Balon Pesan Pengguna (User Bubble di kanan)
      if (chatContainer) {
        const userBubble = document.createElement('div');
        userBubble.className = 'telegram-bubble user-msg';
        userBubble.innerHTML = `
          ${text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')}
          <div class="telegram-bubble-time">${timeStr} &bull; Terkirim ✓✓</div>
        `;
        chatContainer.appendChild(userBubble);
        chatContainer.scrollTop = chatContainer.scrollHeight;
      }

      tgPreviewInput.value = '';

      // 2. Hubungi Backend /api/telegram/interact untuk menghasilkan balasan bot instan
      try {
        const res = await fetch('/api/telegram/interact', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ pesan: text, forwardReal: true })
        });
        const data = await res.json();
        const replyHtml = data.balasan || `💬 Pesan diterima: ${text}`;

        setTimeout(() => {
          if (chatContainer) {
            const botBubble = document.createElement('div');
            botBubble.className = 'telegram-bubble';
            botBubble.innerHTML = `
              ${replyHtml}
              <div class="telegram-bubble-time">${timeStr} &bull; Terkirim ✓✓</div>
            `;
            chatContainer.appendChild(botBubble);
            chatContainer.scrollTop = chatContainer.scrollHeight;
            sound.playTelegramPing();
          }
        }, 400);
      } catch (err) {
        console.error('Gagal interaksi telegram preview:', err);
      }
    });
  }

  // ================= MODAL SOP VIEWER (10 AI AGENT RESMI) =================
  const SOP_TAXONOMY = {
    'agen_orkestrator': {
      standar: 'PMI Agile / Scrum Framework & CSP Actor Model',
      mandat: 'Moderator Sidang Pagi/Malam, Penentu Prioritas (P1/P2/P3), Resolusi Deadlock & Pengatur Siklus Beban Kerja Tim Multi-Proyek.',
      keahlian: [
        'Agile Sprint Leadership & Daily Standup Moderation',
        'Eisenhower Matrix Prioritization (P1, P2, P3 Hard Deadlines)',
        'Task State Machine Controller (IDLE ➔ DITUGASKAN ➔ AKTIF ➔ TINJAUAN ➔ SELESAI)',
        'Multi-Project Portfolio Orchestration (10 Proyek Industri / Hari)'
      ],
      tools: 'Widya Task State Machine, Obsidian Kanban Engine, Multi-Project Queue Router'
    },
    'agen_analis': {
      standar: 'BABOK (Business Analysis Body of Knowledge)',
      mandat: 'Dekomposisi Kebutuhan Bisnis, Perumusan User Story & Kriteria Penerimaan Formal (Acceptance Criteria).',
      keahlian: [
        'BRD / PRD Decompilation & Scope Clarification',
        'User Story & Acceptance Criteria Formal Gherkin (Given-When-Then)',
        'MoSCoW Feature Categorization & Feasibility Assessment',
        'BPMN Process Flow Modeling & Value vs Effort Ratio'
      ],
      tools: 'Gherkin BDD Syntax, Value vs Effort Matrix, BPMN Flow Modeler'
    },
    'agen_arsitek': {
      standar: 'TOGAF Architecture Framework & ACID Storage Principles',
      mandat: 'Desain Arsitektur Sistem, Perancangan Skema SQL Normalisasi, Optimasi B+Tree Index & Kontrak API OpenAPI 3.1.',
      keahlian: [
        'Domain-Driven Design (DDD) & Bounded Contexts',
        'Relational Schema Modeling (SQL DDL 3NF) & Foreign Key Constraints',
        'B+Tree & Secondary Index Query Optimization (O(log N))',
        'WAL & MVCC Transaction Design, REST / SSE OpenAPI 3.1 Specs'
      ],
      tools: 'WidyaBase Engine, SQLite / Postgres Dialects, OpenAPI 3.1 Spec'
    },
    'agen_coder': {
      standar: 'Clean Architecture & SOLID Principles',
      mandat: 'Sintesis Kode Sumber Native Widya-Lang, Implementasi Logika Bisnis, Enkripsi Payload & Hashing Kriptografis.',
      keahlian: [
        'Widya-Lang & Rust Native Programming (High Throughput, Zero GC Overhead)',
        'Cryptographic Integration (Argon2id Password Hashing & HMAC-SHA256)',
        'Concurrency & Memory Safety Management (Zero Leaks, No Null Deref)',
        'Clean Architecture Layering (Controller ➔ Service ➔ Repository ➔ Entity)'
      ],
      tools: 'Widya Toolchain & Compiler, Cargo Engine, SSE Streaming Runtime'
    },
    'agen_qa': {
      standar: 'ISTQB Software Testing Standards',
      mandat: 'Verifikasi Fungsional, Pengujian Beban/Stress Test Concurrency, Fuzzing Injeksi & Sertifikasi Kelayakan Rilis.',
      keahlian: [
        'Automated Test Suite Design (Unit, Integration, & Regression Testing)',
        'Stress & High-Concurrency Load Testing (500+ Conn Benchmark)',
        'Edge Case, Boundary Value, SQL Injection & XSS Fuzzing',
        'Release Gatekeeping & Zero Blocker Enforcement'
      ],
      tools: 'Widya Test Runner, Benchmark Harness, Network Chaos Simulator'
    },
    'agen_devops': {
      standar: 'Google SRE Principles & ITIL v4',
      mandat: 'Automasi CI/CD, Cadangan Data Harian (Snapshot Backup), Pemantauan Kesehatan Server & Pemulihan Darurat WAL.',
      keahlian: [
        'Continuous Integration & Deployment (CI/CD) Automation',
        'Automated Snapshot & Backup Management dengan Hash SHA-256',
        'WAL Checkpoint Verification & Disaster Recovery Failover (<1.5 detik)',
        'Server Telemetry & Process Daemonization (Uptime Guard 99.9%)'
      ],
      tools: 'Widya Observability Platform, Task Scheduler / Cron, Health Daemon'
    },
    'agen_keamanan': {
      standar: 'ISO 27001:2022 & UU No. 27/2022 (Perlindungan Data Pribadi)',
      mandat: 'Audit Keamanan OWASP Top 10, Penegakan UU Perlindungan Data Pribadi (UU PDP), Audit Log Forensik & Verifikasi HMAC.',
      keahlian: [
        'OWASP Top 10 Vulnerability Audit (SQLi, XSS, SSRF, Auth Bypass)',
        'UU PDP Compliance & PII Data Masking / Zero Raw Data Leakage',
        'Secret & Credential Leakage Scanning (Repo & Memory Shield)',
        'HMAC-SHA256 Cryptographic Signature Verification & Forensic Logging'
      ],
      tools: 'Widya Security Scanner, Argon2id Verifier, Forensic Audit Logger'
    },
    'agen_notulis': {
      standar: 'Zettelkasten Method & Personal Knowledge Management (PKM)',
      mandat: 'Dokumentasi Notulensi Rapat, Pemutakhiran Papan Kanban, Manajemen Knowledge Base Vault di Obsidian.',
      keahlian: [
        'Structured Markdown Documentation (GFM) & YAML Frontmatter',
        'Bi-directional Linking & Knowledge Graph Synthesis ([[...]])',
        'Automated Kanban Board Sync (TODO ➔ IN PROGRESS ➔ DONE)',
        'Write-then-Verify Documentation Version Control'
      ],
      tools: 'Obsidian Vault Bridge (C:/Satriyo/Vault), GFM Transpiler, Kanban Engine'
    },
    'agen_humas': {
      standar: 'Event-Driven Notification Architecture & Executive Reporting',
      mandat: 'Sintesis Laporan Eksekutif, Pemformatan Pesan Visual, Pengiriman Briefing via Telegram Bot Gateway ke Pimpinan.',
      keahlian: [
        'Executive Briefing Synthesis & High-Impact Summary Composition',
        'Visual Markdown Formatting for Smartphone Viewports',
        'Telegram Bot API Gateway Integration (@WidyaDigitalBot)',
        'Rate Limiting, Retry Backoff & Multi-Channel Broadcast'
      ],
      tools: 'Telegram Bot Gateway, Webhook Dispatcher, Fallback Logger'
    },
    'agen_analis_data': {
      standar: 'Balanced Scorecard & DAMA-DMBOK Data Management Framework',
      mandat: 'Analitika Portofolio Proyek, Algoritma Penilaian Efektivitas Tim, Deteksi Anomali Beban Kerja & Dashboard KPI Harian.',
      keahlian: [
        'Project Portfolio Health Analytics & Completion Forecasting',
        'Team Productivity Scoring Algorithm (0–100% Verified Sprint Ratio)',
        'Workload Distribution Anomaly Detection (>80% Saturation Alert)',
        'Executive KPI Dashboard Generation (DailyNotes/*-Analitika.md)'
      ],
      tools: 'Widya Analytics Bridge, SQL Aggregation Engine, Obsidian Dashboard Exporter'
    }
  };

  let activeSopAgentId = 'agen_orkestrator';
  let isSopControlsInitialized = false;

  function initSopHeaderControls() {
    if (isSopControlsInitialized) return;
    const sopSelect = document.getElementById('sop-modal-agent-select');
    const btnPrev = document.getElementById('btn-sop-prev');
    const btnNext = document.getElementById('btn-sop-next');

    if (sopSelect) {
      sopSelect.innerHTML = AGENTS_DATA.map(ag => `
        <option value="${ag.id}">${ag.sop} • ${ag.nama} (${ag.peran})</option>
      `).join('');

      sopSelect.addEventListener('change', () => {
        sound.playClick();
        const target = AGENTS_DATA.find(a => a.id === sopSelect.value);
        if (target) openSOPModal(target);
      });
    }

    if (btnPrev) {
      btnPrev.addEventListener('click', () => {
        sound.playClick();
        const curIdx = AGENTS_DATA.findIndex(a => a.id === activeSopAgentId);
        const nextIdx = (curIdx - 1 + AGENTS_DATA.length) % AGENTS_DATA.length;
        openSOPModal(AGENTS_DATA[nextIdx]);
      });
    }

    if (btnNext) {
      btnNext.addEventListener('click', () => {
        sound.playClick();
        const curIdx = AGENTS_DATA.findIndex(a => a.id === activeSopAgentId);
        const nextIdx = (curIdx + 1) % AGENTS_DATA.length;
        openSOPModal(AGENTS_DATA[nextIdx]);
      });
    }

    isSopControlsInitialized = true;
  }

  function renderSopSwitcher(currentAgentId) {
    initSopHeaderControls();

    const sopSelect = document.getElementById('sop-modal-agent-select');
    if (sopSelect) sopSelect.value = currentAgentId;

    const switcherEl = document.getElementById('sop-agent-switcher');
    if (!switcherEl) return;
    switcherEl.innerHTML = AGENTS_DATA.map(ag => {
      const isActive = ag.id === currentAgentId;
      return `
        <button class="sop-pill-btn ${isActive ? 'active' : ''}" data-agent-id="${ag.id}" title="${ag.nama} (${ag.gelar})">
          <span>${ag.avatar}</span>
          <span>${ag.sop} • ${ag.panggilan}</span>
        </button>
      `;
    }).join('');

    switcherEl.querySelectorAll('.sop-pill-btn').forEach(btn => {
      btn.addEventListener('click', () => {
        sound.playClick();
        const agId = btn.getAttribute('data-agent-id');
        const ag = AGENTS_DATA.find(a => a.id === agId);
        if (ag) openSOPModal(ag);
      });
    });
  }

  function openSOPModal(agent = null) {
    const targetAgent = agent || AGENTS_DATA.find(a => a.id === activeSopAgentId) || AGENTS_DATA[0];
    activeSopAgentId = targetAgent.id;

    renderSopSwitcher(targetAgent.id);

    const tax = SOP_TAXONOMY[targetAgent.id] || {
      standar: 'Standar Industri Terverifikasi',
      mandat: targetAgent.tugas || 'Pelaksanaan tugas operasional kantor digital.',
      keahlian: ['Eksekusi tugas terspesialisasi', 'Kepatuhan UU PDP No. 27/2022'],
      tools: 'Widya Toolchain'
    };

    const modalTitle = document.getElementById('sop-modal-title');
    if (modalTitle) {
      modalTitle.textContent = `📜 ${targetAgent.sop}: Standard Operating Procedure — ${targetAgent.nama}`;
    }

    const contentEl = document.getElementById('sop-modal-content');
    if (contentEl) {
      contentEl.innerHTML = `
        <div class="sop-card-header">
          <div class="sop-agent-identity">
            <div class="sop-agent-avatar-wrap" style="border-color: ${targetAgent.warna};">
              ${typeof getAgentAvatarSvg === 'function' ? getAgentAvatarSvg(targetAgent.id, 38) : targetAgent.avatar}
            </div>
            <div class="sop-identity-text">
              <h4>${targetAgent.nama}</h4>
              <div class="sop-role-label">${targetAgent.gelar}</div>
              <div style="font-size: 7pt; color: #94a3b8; margin-top: 3px;">
                Kluster: <b>${targetAgent.kluster || 'Engineering Pod'}</b> • Gender: <b>${targetAgent.gender}</b>
              </div>
            </div>
          </div>
          <div style="text-align: right;">
            <span class="sop-badge-tag">STATUS: RESMI (LEVEL A)</span>
            <div style="font-size: 6.8pt; color: #64748b; margin-top: 4px;">Berkas: <code>C:/Satriyo/Vault/SOP/${targetAgent.sopFile}</code></div>
          </div>
        </div>

        <div class="sop-section-grid">
          <div class="sop-section-card">
            <h5>🏛️ Standar Industri Rujukan</h5>
            <div style="font-weight: 700; color: #38bdf8; font-size: 8pt; margin-bottom: 4px;">${tax.standar}</div>
            <div style="font-size: 7.5pt; color: #94a3b8;">Mematuhi ISO 27001:2022, UU PDP No. 27/2022, dan Arsitektur Bersih Widya-Lang.</div>
          </div>
          <div class="sop-section-card">
            <h5>🛠️ Perkakas & Framework Kerja (Tools)</h5>
            <div style="font-size: 7.5pt; color: #e2e8f0; font-family: monospace;">${tax.tools}</div>
          </div>
        </div>

        <div style="background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.06); border-radius: 10px; padding: 12px; margin-bottom: 12px;">
          <h4 style="color: #38bdf8; font-size: 8.5pt; margin-bottom: 6px;">1. Mandat & Tanggung Jawab Utama</h4>
          <p style="font-size: 8pt; color: #cbd5e1; margin-bottom: 8px;">${tax.mandat}</p>
          
          <h5 style="color: #f59e0b; font-size: 7.8pt; margin-bottom: 4px;">Daftar Kompetensi & Spesifikasi Skills:</h5>
          <ul style="padding-left: 16px; font-size: 7.5pt; color: #94a3b8; line-height: 1.5;">
            ${tax.keahlian.map(k => `<li>${k}</li>`).join('')}
          </ul>
        </div>

        <div style="background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.06); border-radius: 10px; padding: 12px; margin-bottom: 12px;">
          <h4 style="color: #a855f7; font-size: 8.5pt; margin-bottom: 6px;">2. Alur Kerja Sidang Rapat Pagi & Evaluasi Malam</h4>
          <ul style="padding-left: 16px; font-size: 7.8pt; color: #cbd5e1; line-height: 1.55;">
            <li><strong>08:00 WIB (Sidang Resmi Pagi):</strong> Mengajukan 3 prioritas kerja, menetapkan hard deadline, konfirmasi reviewer QA, dan menjamin kepatuhan UU PDP No. 27/2022.</li>
            <li><strong>20:00 WIB (Evaluasi Malam):</strong> Menyerahkan bukti kerja (*evidence* teruji), melaporkan kendala teknis (*blockers*), dan memberikan skor kesiapan produksi esok hari.</li>
          </ul>
        </div>

        <div style="background: rgba(16, 185, 129, 0.05); border: 1px solid rgba(16, 185, 129, 0.2); border-radius: 10px; padding: 12px;">
          <h4 style="color: #34d399; font-size: 8.5pt; margin-bottom: 4px;">3. Protokol Keamanan & Kepatuhan UU PDP No. 27/2022</h4>
          <p style="font-size: 7.5pt; color: #a7f3d0; margin: 0;">
            Seluruh eksekusi tugas diaudit secara otomatis oleh <code>agen_keamanan</code> menggunakan tanda tangan HMAC-SHA256 dengan prinsip <em>Zero Raw Data Leakage</em> (semua kredensial & data sensitif disamarkan).
          </p>
        </div>
      `;
    }

    const modal = document.getElementById('sop-modal');
    if (modal) modal.style.display = 'flex';
  }

  document.getElementById('btn-open-sop-modal').addEventListener('click', () => {
    sound.playClick();
    // Buka SOP agen yang sedang aktif di roster atau agen terakhir yang dilihat
    const currentActiveRoster = document.querySelector('.roster-item.active');
    let targetAgent = null;
    if (currentActiveRoster && currentActiveRoster.id) {
      const agId = currentActiveRoster.id.replace('roster-', '');
      targetAgent = AGENTS_DATA.find(a => a.id === agId);
    }
    if (!targetAgent) {
      targetAgent = AGENTS_DATA.find(a => a.id === activeSopAgentId) || AGENTS_DATA[0];
    }
    openSOPModal(targetAgent);
  });

  document.getElementById('btn-close-sop').addEventListener('click', () => {
    sound.playClick();
    document.getElementById('sop-modal').style.display = 'none';
  });

  // ================= MODAL WIDYABASE ENTERPRISE & DOKUMENTASI =================
  const widyaModal = document.getElementById('widyabase-modal');
  let currentWbTable = 'proyek';
  let cachedDocsProjectList = [];
  let selectedDocProject = 'PRJ-01-KOPERASI';
  let selectedDocItem = 'DOC-ALL';

  function openWidyaBaseModal() {
    sound.playClick();
    if (widyaModal) {
      widyaModal.style.display = 'flex';
      loadWidyaBaseStatus();
      loadWidyaBaseTable('proyek');
      loadWidyaBaseDocs();
    }
  }

  function closeWidyaBaseModal() {
    sound.playClick();
    if (widyaModal) widyaModal.style.display = 'none';
  }

  const btnOpenWb = document.getElementById('btn-open-widyabase');
  if (btnOpenWb) btnOpenWb.addEventListener('click', openWidyaBaseModal);

  const btnHudWb = document.getElementById('btn-hud-widyabase');
  if (btnHudWb) btnHudWb.addEventListener('click', openWidyaBaseModal);

  const btnCloseWb = document.getElementById('btn-close-widyabase');
  if (btnCloseWb) btnCloseWb.addEventListener('click', closeWidyaBaseModal);

  // Tangkap klik 3D pada Server Rack Data Center
  window.addEventListener('open-widyabase-modal', () => {
    openWidyaBaseModal();
  });

  // Tab Switcher
  document.querySelectorAll('.db-tab-btn').forEach((btn) => {
    btn.addEventListener('click', () => {
      sound.playClick();
      document.querySelectorAll('.db-tab-btn').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('.db-tab-content').forEach(c => c.classList.remove('active'));
      btn.classList.add('active');
      const target = document.getElementById(btn.getAttribute('data-tab'));
      if (target) target.classList.add('active');
      if (btn.getAttribute('data-tab') === 'tab-docs') {
        loadWidyaBaseDocs();
      }
    });
  });

  // Load Status Engine
  function loadWidyaBaseStatus() {
    fetch('/api/widyabase/status')
      .then(r => r.json())
      .then(d => {
        if (d.sukses) {
          const valEl = document.getElementById('wb-status-val');
          const subEl = document.getElementById('wb-status-sub');
          if (valEl) valEl.textContent = d.status;
          if (subEl) subEl.textContent = `${d.buffer_pool} • ${d.page_size}`;
        }
      })
      .catch(() => {});
  }

  // Load Database Table View
  function loadWidyaBaseTable(tableName) {
    currentWbTable = tableName;
    const container = document.getElementById('db-table-display');
    if (!container) return;
    container.innerHTML = '<div style="padding: 20px; color: #94a3b8; font-family: monospace;">⏳ Memuat data dari WidyaBase...</div>';

    fetch(`/api/widyabase/${tableName}`)
      .then(r => {
        if (!r.ok) throw new Error(`HTTP ${r.status} ${r.statusText}`);
        return r.json();
      })
      .then(rows => {
        if (!Array.isArray(rows) || rows.length === 0) {
          container.innerHTML = '<div style="padding: 20px; color: #94a3b8; font-family: monospace;">Tabel kosong atau belum ada entri data.</div>';
          return;
        }
        const cols = Object.keys(rows[0]);
        let html = '<table class="wb-custom-table"><thead><tr>';
        cols.forEach(c => html += `<th>${c}</th>`);
        html += '</tr></thead><tbody>';

        rows.forEach(row => {
          html += '<tr>';
          cols.forEach(c => {
            let val = row[c] !== undefined && row[c] !== null ? String(row[c]) : '';
            if (val.length > 70) val = val.substring(0, 67) + '...';
            html += `<td>${val}</td>`;
          });
          html += '</tr>';
        });
        html += '</tbody></table>';
        container.innerHTML = html;
      })
      .catch(err => {
        container.innerHTML = `<div style="padding: 20px; color: #ef4444; font-family: monospace;">Galat memuat tabel: ${err.message}</div>`;
      });
  }

  // Table Switcher Pills
  document.querySelectorAll('.table-pill').forEach(btn => {
    btn.addEventListener('click', () => {
      sound.playClick();
      document.querySelectorAll('.table-pill').forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      loadWidyaBaseTable(btn.getAttribute('data-table'));
    });
  });

  // Load & Format Documentation Markdown (Multi-Proyek & Multi-Dokumen)
  let isDocsSelectorInitialized = false;

  function initDocsSelectors() {
    const prjSelect = document.getElementById('doc-project-select');
    const itemSelect = document.getElementById('doc-item-select');
    if (!prjSelect || !itemSelect) return;

    prjSelect.addEventListener('change', () => {
      sound.playClick();
      selectedDocProject = prjSelect.value;
      updateDocsItemSelect(selectedDocProject);
      fetchAndRenderCurrentDoc();
    });

    itemSelect.addEventListener('change', () => {
      sound.playClick();
      selectedDocItem = itemSelect.value;
      fetchAndRenderCurrentDoc();
    });

    isDocsSelectorInitialized = true;
  }

  function updateDocsItemSelect(prjCode) {
    const itemSelect = document.getElementById('doc-item-select');
    if (!itemSelect) return;

    const prjData = cachedDocsProjectList.find(p => p.kode_proyek === prjCode);
    const docs = (prjData && Array.isArray(prjData.dokumen_list)) ? prjData.dokumen_list : [
      { id: 'DOC-ALL', judul: '📖 Dokumen Lengkap Proyek (Full Specification)' },
      { id: 'DOC-ARCH-01', judul: '📐 Bab 1: Arsitektur Sistem & Topologi 10 AI Agent' },
      { id: 'DOC-SEC-01', judul: '🛡️ Bab 2: Kepatuhan Keamanan & UU PDP No. 27/2022' },
      { id: 'DOC-API-01', judul: '🔌 Bab 3: Kontrak REST API & Real-Time SSE Streams' },
      { id: 'DOC-QA-01', judul: '🧪 Bab 4: Matriks Pengujian Mutu QA & Stress Test' },
      { id: 'DOC-OPS-01', judul: '⚡ Bab 5: Panduan Operasional, WAL & Recovery SRE' }
    ];

    itemSelect.innerHTML = docs.map(d => `<option value="${d.id}" ${d.id === selectedDocItem ? 'selected' : ''}>${d.judul}</option>`).join('');
    if (!docs.some(d => d.id === selectedDocItem)) {
      selectedDocItem = 'DOC-ALL';
      itemSelect.value = 'DOC-ALL';
    }
  }

  function fetchAndRenderCurrentDoc() {
    const docContainer = document.getElementById('docs-markdown-view');
    const sourceLabel = document.getElementById('doc-source-label');
    if (!docContainer) return;

    docContainer.innerHTML = '<div style="color: #94a3b8; font-family: monospace;">⏳ Mengambil dokumen proyek dari WidyaBase & Obsidian Vault...</div>';

    fetch(`/api/widyabase/dokumentasi/lengkap?proyek=${encodeURIComponent(selectedDocProject)}&dokumen=${encodeURIComponent(selectedDocItem)}`)
      .then(r => r.json())
      .then(d => {
        if (d.sukses && d.konten_markdown) {
          if (sourceLabel) {
            sourceLabel.innerHTML = `Berkas Sumber: <code>${d.file_path || `C:/Satriyo/Vault/Dokumentasi_Proyek/${selectedDocProject}_Dokumentasi_Lengkap.md`}</code>`;
          }
          renderMarkdown(docContainer, d.konten_markdown);
        } else {
          docContainer.innerHTML = '<div style="color: #f59e0b;">Dokumentasi belum tersedia. Klik tombol "Susun Ulang" untuk generate.</div>';
        }
      })
      .catch(err => {
        docContainer.innerHTML = `<div style="color: #ef4444;">Gagal memuat dokumen: ${err.message}</div>`;
      });
  }

  function loadWidyaBaseDocs() {
    if (!isDocsSelectorInitialized) initDocsSelectors();

    fetch('/api/widyabase/dokumentasi/daftar')
      .then(r => r.json())
      .then(res => {
        if (res.sukses && Array.isArray(res.daftar_proyek)) {
          cachedDocsProjectList = res.daftar_proyek;
          const prjSelect = document.getElementById('doc-project-select');
          if (prjSelect) {
            prjSelect.innerHTML = res.daftar_proyek.map(p => {
              const isArsip = (p.status || '').toUpperCase() === 'ARSIP';
              return `<option value="${p.kode_proyek}" ${p.kode_proyek === selectedDocProject ? 'selected' : ''}>${isArsip ? '📦 [ARSIP] ' : ''}${p.kode_proyek} • ${p.nama_proyek}</option>`;
            }).join('');
            if (!res.daftar_proyek.some(p => p.kode_proyek === selectedDocProject)) {
              selectedDocProject = res.daftar_proyek[0].kode_proyek;
              prjSelect.value = selectedDocProject;
            }
          }
          updateDocsItemSelect(selectedDocProject);
          fetchAndRenderCurrentDoc();
        } else {
          fetchAndRenderCurrentDoc();
        }
      })
      .catch(() => {
        fetchAndRenderCurrentDoc();
      });
  }

  // Parser Markdown Sederhana & Cepat untuk Preview Buku Dokumen
  function renderMarkdown(container, md) {
    let lines = md.split('\n');
    let html = '';
    let inTable = false;
    let tableHead = false;

    lines.forEach(line => {
      let l = line.trim();
      if (l.startsWith('# ')) {
        html += `<h1>${l.slice(2)}</h1>`;
      } else if (l.startsWith('## ')) {
        html += `<h2>${l.slice(3)}</h2>`;
      } else if (l.startsWith('### ')) {
        html += `<h3>${l.slice(4)}</h3>`;
      } else if (l.startsWith('#### ')) {
        html += `<h4>${l.slice(5)}</h4>`;
      } else if (l.startsWith('> ')) {
        html += `<blockquote style="border-left: 3px solid #06b6d4; padding-left: 10px; margin: 8px 0; color: #94a3b8;">${l.slice(2)}</blockquote>`;
      } else if (l.startsWith('|')) {
        if (!inTable) {
          html += '<table><tbody>';
          inTable = true;
          tableHead = true;
        }
        if (l.includes('---')) {
          tableHead = false;
          return;
        }
        let cells = l.split('|').filter((_, i, a) => i > 0 && i < a.length - 1);
        html += '<tr>';
        cells.forEach(cell => {
          let tag = tableHead ? 'th' : 'td';
          html += `<${tag}>${cell.trim().replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>').replace(/`(.*?)`/g, '<code>$1</code>')}</${tag}>`;
        });
        html += '</tr>';
      } else {
        if (inTable) {
          html += '</tbody></table>';
          inTable = false;
        }
        if (l === '---') {
          html += '<hr style="border: none; border-top: 1px solid rgba(255,255,255,0.1); margin: 16px 0;">';
        } else if (l.startsWith('1. ') || l.startsWith('2. ') || l.startsWith('3. ') || l.startsWith('4. ') || l.startsWith('5. ') || l.startsWith('6. ') || l.startsWith('7. ') || l.startsWith('8. ') || l.startsWith('9. ')) {
          html += `<div style="margin: 4px 0 4px 14px;">${l.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>').replace(/`(.*?)`/g, '<code>$1</code>')}</div>`;
        } else if (l.startsWith('- ')) {
          html += `<div style="margin: 3px 0 3px 14px;">• ${l.slice(2).replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>').replace(/`(.*?)`/g, '<code>$1</code>')}</div>`;
        } else if (l.length > 0) {
          html += `<p style="margin: 6px 0;">${l.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>').replace(/`(.*?)`/g, '<code>$1</code>')}</p>`;
        }
      }
    });
    if (inTable) html += '</tbody></table>';
    container.innerHTML = html;
  }

  // Tombol Re-generate Dokumentasi Proyek
  const btnRegenDocs = document.getElementById('btn-regen-docs');
  if (btnRegenDocs) {
    btnRegenDocs.addEventListener('click', () => {
      sound.playClick();
      btnRegenDocs.textContent = '⏳ Mengompilasi Dokumen...';
      btnRegenDocs.disabled = true;

      fetch('/api/widyabase/generate-docs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          kode_proyek: selectedDocProject,
          dokumen: selectedDocItem
        })
      })
        .then(r => r.json())
        .then(d => {
          btnRegenDocs.textContent = '⚡ Susun Ulang dari Database';
          btnRegenDocs.disabled = false;
          sound.playWhatsAppPing();
          fetchAndRenderCurrentDoc();
          loadWidyaBaseTable(currentWbTable);
        })
        .catch(() => {
          btnRegenDocs.textContent = '⚡ Susun Ulang dari Database';
          btnRegenDocs.disabled = false;
        });
    });
  }

  // SQL Terminal Execution
  const sqlInput = document.getElementById('sql-input');
  const btnExecSql = document.getElementById('btn-execute-sql');
  const sqlOutput = document.getElementById('sql-output');

  function executeSql(sql) {
    sound.playClick();
    if (!sql || !sql.trim()) return;
    if (sqlOutput) sqlOutput.innerHTML = '<pre style="color: #94a3b8;">Menjalankan kueri pada WidyaBase...</pre>';

    fetch('/api/widyabase/sql', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ kueri: sql.trim(), format: 'json' })
    })
      .then(r => r.json())
      .then(d => {
        if (sqlOutput) {
          if (d.sukses) {
            sqlOutput.innerHTML = `<pre style="color: #34d399;">widyabase> ${d.kueri}\n\n${JSON.stringify(d.hasil, null, 2)}</pre>`;
          } else {
            sqlOutput.innerHTML = `<pre style="color: #ef4444;">[GALAT SQL]\n${d.error}</pre>`;
          }
        }
      })
      .catch(e => {
        if (sqlOutput) sqlOutput.innerHTML = `<pre style="color: #ef4444;">[KONEKSI GAGAL]\n${e.message}</pre>`;
      });
  }

  if (btnExecSql && sqlInput) {
    btnExecSql.addEventListener('click', () => executeSql(sqlInput.value));
    sqlInput.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') executeSql(sqlInput.value);
    });
  }

  document.querySelectorAll('.btn-quick-sql').forEach(btn => {
    btn.addEventListener('click', () => {
      const sql = btn.getAttribute('data-sql');
      if (sqlInput) sqlInput.value = sql;
      executeSql(sql);
    });
  });

  // Klik di luar jendela modal (backdrop) untuk menutup
  ['kanban-modal', 'telegram-modal', 'wa-modal', 'sop-modal', 'widyabase-modal', 'modal-add-task', 'modal-add-project', 'modal-security'].forEach((modalId) => {
    const modalEl = document.getElementById(modalId);
    if (modalEl) {
      modalEl.addEventListener('click', (e) => {
        if (e.target === modalEl) {
          sound.playClick();
          modalEl.style.display = 'none';
        }
      });
    }
  });

  // Tombol Keyboard ESC untuk menutup modal apapun
  window.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      ['kanban-modal', 'telegram-modal', 'wa-modal', 'sop-modal', 'meeting-hologram', 'widyabase-modal', 'modal-add-task', 'modal-add-project', 'modal-security'].forEach((id) => {
        const el = document.getElementById(id);
        if (el) el.style.display = 'none';
      });
      const dossier = document.getElementById('agent-dossier');
      if (dossier) dossier.style.display = 'none';
    }
  });

  // ================= MODAL AUDIT KEAMANAN (HENDRA GUNAWAN) =================
  const modalSecurity = document.getElementById('modal-security');
  const btnCloseSecurity = document.getElementById('btn-close-security');
  const btnHudSecurity = document.getElementById('btn-hud-security');

  function openSecurityModal() {
    sound.playClick();
    if (!modalSecurity) return;
    modalSecurity.style.display = 'flex';

    const bodyEl = document.getElementById('security-modal-body');
    if (bodyEl) {
      bodyEl.innerHTML = '<div style="padding: 24px; text-align: center; color: #94a3b8;">Menjalankan inspeksi audit keamanan siber & kepatuhan UU PDP...</div>';
      
      fetch('/api/keamanan/audit')
        .then(r => r.json())
        .then(d => {
          if (!d.sukses) {
            bodyEl.innerHTML = `<div style="color: #ef4444; padding: 20px;">Gagal memuat audit: ${d.error}</div>`;
            return;
          }

          // Perbarui teks pill HUD
          const hudSec = document.getElementById('hud-security-label');
          if (hudSec) hudSec.textContent = `Keamanan: Skor ${d.skor_keamanan} (${d.tingkat_kepatuhan.split('•')[0].trim()})`;

          const rowsHtml = d.inspeksi.map(item => `
            <div style="background: rgba(255,255,255,0.03); border: 1px solid rgba(255,255,255,0.08); border-radius: 8px; padding: 12px 16px; margin-bottom: 10px; display: flex; justify-content: space-between; align-items: center;">
              <div>
                <div style="font-weight: 700; color: #f8fafc; font-size: 9pt; display: flex; align-items: center; gap: 8px;">
                  <span>${item.aspek}</span>
                  <span style="font-size: 7.2pt; padding: 2px 6px; border-radius: 4px; background: rgba(16, 185, 129, 0.2); color: #34d399; border: 1px solid #10b981;">${item.status}</span>
                </div>
                <div style="font-size: 8pt; color: #94a3b8; margin-top: 4px;">${item.detail}</div>
              </div>
              <div style="font-family: monospace; font-weight: 800; color: #38bdf8; font-size: 11pt;">+${item.skor} Pts</div>
            </div>
          `).join('');

          bodyEl.innerHTML = `
            <div style="display: flex; gap: 16px; align-items: center; margin-bottom: 18px; padding: 16px; background: linear-gradient(135deg, rgba(16, 185, 129, 0.15), rgba(6, 182, 212, 0.15)); border: 1px solid rgba(16, 185, 129, 0.3); border-radius: 10px;">
              <div style="font-size: 32pt; line-height: 1;">🛡️</div>
              <div>
                <div style="font-size: 14pt; font-weight: 800; color: #34d399;">Skor Keamanan Siber: ${d.skor_keamanan}/100</div>
                <div style="font-size: 8.5pt; color: #f8fafc; font-weight: 600; margin-top: 2px;">${d.tingkat_kepatuhan}</div>
                <div style="font-size: 7.8pt; color: #94a3b8; margin-top: 4px;">Auditor Resmi: <b>${d.auditor}</b></div>
              </div>
            </div>

            <div style="margin-bottom: 12px; font-size: 8.5pt; font-weight: 700; color: #cbd5e1; text-transform: uppercase; letter-spacing: 0.5px;">
              Hasil 6 Titik Pemeriksaan Integritas & Kedaulatan:
            </div>
            
            <div style="max-height: 320px; overflow-y: auto; padding-right: 6px;">
              ${rowsHtml}
            </div>

            <div style="margin-top: 14px; padding: 10px 14px; background: rgba(56, 189, 248, 0.1); border: 1px solid rgba(56, 189, 248, 0.25); border-radius: 8px; font-size: 8pt; color: #bae6fd;">
              💡 <b>Rekomendasi Hendra:</b> ${d.rekomendasi}
            </div>
          `;
        })
        .catch(err => {
          bodyEl.innerHTML = `<div style="color: #ef4444; padding: 20px;">Gagal memuat audit keamanan: ${err.message}</div>`;
        });
    }
  }

  if (btnHudSecurity) btnHudSecurity.addEventListener('click', openSecurityModal);
  if (btnCloseSecurity && modalSecurity) {
    btnCloseSecurity.addEventListener('click', () => {
      sound.playClick();
      modalSecurity.style.display = 'none';
    });
  }

  // ================= FORM TAMBAH PROYEK & TUGAS KERJA =================
  const modalAddTask = document.getElementById('modal-add-task');
  const modalAddProject = document.getElementById('modal-add-project');

  function openAddTaskModal() {
    sound.playClick();
    if (modalAddTask) {
      modalAddTask.style.display = 'flex';
      fetch('/api/widyabase/proyek')
        .then(r => r.json())
        .then(projects => {
          if (Array.isArray(projects) && projects.length > 0) {
            const sel = document.getElementById('input-task-project');
            if (sel) {
              sel.innerHTML = projects.map(p => `<option value="${p.kode_proyek}">${p.kode_proyek} • ${p.nama_proyek}</option>`).join('');
            }
          }
        })
        .catch(() => {});
      const codeInput = document.getElementById('input-task-code');
      if (codeInput && !codeInput.value) {
        codeInput.value = `TSK-${String(Math.floor(Math.random() * 800) + 100)}`;
      }
    }
  }

  function openAddProjectModal() {
    sound.playClick();
    if (modalAddProject) {
      modalAddProject.style.display = 'flex';
      const codeInput = document.getElementById('input-prj-code');
      if (codeInput && !codeInput.value) {
        codeInput.value = `PRJ-WIDYA-${String(Math.floor(Math.random() * 80) + 10)}`;
      }
    }
  }

  // Tombol Buka Modal Tambah Tugas
  ['btn-open-add-task', 'btn-wb-add-task'].forEach(id => {
    const btn = document.getElementById(id);
    if (btn) btn.addEventListener('click', openAddTaskModal);
  });

  // Tombol Tutup Modal Tambah Tugas
  ['btn-close-add-task', 'btn-cancel-add-task'].forEach(id => {
    const btn = document.getElementById(id);
    if (btn) btn.addEventListener('click', () => {
      sound.playClick();
      if (modalAddTask) modalAddTask.style.display = 'none';
    });
  });

  // Tombol Buka Modal Tambah Proyek
  ['btn-open-add-project', 'btn-wb-add-project'].forEach(id => {
    const btn = document.getElementById(id);
    if (btn) btn.addEventListener('click', openAddProjectModal);
  });

  // Tombol Tutup Modal Tambah Proyek
  ['btn-close-add-project', 'btn-cancel-add-project'].forEach(id => {
    const btn = document.getElementById(id);
    if (btn) btn.addEventListener('click', () => {
      sound.playClick();
      if (modalAddProject) modalAddProject.style.display = 'none';
    });
  });

  // Submit Form Tambah Proyek
  const formAddProject = document.getElementById('form-add-project');
  if (formAddProject) {
    formAddProject.addEventListener('submit', async (e) => {
      e.preventDefault();
      sound.playClick();

      const kode = document.getElementById('input-prj-code').value.trim();
      const nama = document.getElementById('input-prj-name').value.trim();
      const status = document.getElementById('input-prj-status').value;

      try {
        const res = await fetch('/api/widyabase/tambah-proyek', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ kode_proyek: kode, nama_proyek: nama, status })
        });
        
        const contentType = res.headers.get('content-type') || '';
        if (!contentType.includes('application/json')) {
          throw new Error('Server backend (port 8085) tidak merespons JSON. Pastikan Anda membuka melalui server backend Node.js (http://localhost:8085), bukan Live Server/file statis biasa.');
        }

        const data = await res.json();
        if (data.sukses) {
          if (modalAddProject) modalAddProject.style.display = 'none';
          formAddProject.reset();

          postAgentChatter({
            from: 'agen_orkestrator',
            msg: `📁 Proyek baru "${nama}" (${kode}) telah resmi didaftarkan ke WidyaBase!`
          });

          loadWidyaBaseTable('proyek');
          refreshProjectsAndKanban();
        } else {
          alert('Gagal menambahkan proyek: ' + (data.error || 'Terjadi kesalahan pada database.'));
        }
      } catch (err) {
        alert('Gagal menambahkan proyek: ' + err.message);
      }
    });
  }

  // Submit Form Tambah Tugas
  const formAddTask = document.getElementById('form-add-task');
  if (formAddTask) {
    formAddTask.addEventListener('submit', async (e) => {
      e.preventDefault();
      sound.playClick();

      const prj = document.getElementById('input-task-project').value;
      const kode = document.getElementById('input-task-code').value.trim();
      const prio = document.getElementById('input-task-prio').value;
      const judul = document.getElementById('input-task-title').value.trim();
      const pic = document.getElementById('input-task-pic').value;
      const reviewer = document.getElementById('input-task-reviewer').value;
      const sop = document.getElementById('input-task-sop').value;
      const deadline = document.getElementById('input-task-deadline').value.trim();

      const targetAgent = AGENTS_DATA.find(a => a.id === pic);
      const agentNama = targetAgent ? targetAgent.nama : pic;

      try {
        const res = await fetch('/api/widyabase/tambah-tugas', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            kode_proyek: prj,
            kode_tugas: kode,
            prioritas: prio,
            judul_tugas: judul,
            pic_agen: pic,
            reviewer_agen: reviewer,
            sop_referensi: sop,
            deadline: deadline
          })
        });

        const contentType = res.headers.get('content-type') || '';
        if (!contentType.includes('application/json')) {
          throw new Error('Server backend (port 8085) tidak merespons JSON. Pastikan Anda membuka melalui server backend Node.js (http://localhost:8085), bukan Live Server/file statis biasa.');
        }

        const data = await res.json();
        if (data.sukses) {
          if (modalAddTask) modalAddTask.style.display = 'none';
          formAddTask.reset();

          refreshProjectsAndKanban();

          postAgentChatter({
            from: 'agen_orkestrator',
            msg: `⚡ Penugasan baru [${kode}] "${judul}" didelegasikan ke @${agentNama} (Deadline: ${deadline})`
          });

          if (office && office.agents3D) {
            const a3D = office.agents3D.get(pic);
            if (a3D) office.setCameraView('focus', a3D.deskPos);
          }

          loadWidyaBaseTable('tugas');
        }
      } catch (err) {
        alert('Gagal mendaftarkan tugas: ' + err.message);
      }
    });
  }

  // ================= CONTROLLER MODE KANTOR (BUKA / TUTUP & ISTIRAHAT) =================
  let currentOfficeMode = 'BUKA';

  function applyOfficeMode(mode, userTriggered = false) {
    const isClosed = mode === 'TUTUP';
    currentOfficeMode = mode;

    const btnKantor = document.getElementById('btn-toggle-kantor');
    const txtKantor = document.getElementById('kantor-status-text');
    const dotKantor = document.getElementById('kantor-status-dot');
    const bannerClosed = document.getElementById('office-closed-banner');

    if (btnKantor) {
      if (isClosed) {
        btnKantor.className = 'btn-pill btn-office-status closed';
        if (txtKantor) txtKantor.textContent = 'Kantor: TUTUP (ISTIRAHAT)';
        if (dotKantor) { 
          dotKantor.className = 'pulse-dot'; 
          dotKantor.style.background = '#a855f7'; 
          dotKantor.style.boxShadow = '0 0 8px #a855f7'; 
        }
      } else {
        btnKantor.className = 'btn-pill btn-office-status active';
        if (txtKantor) txtKantor.textContent = 'Kantor: BUKA';
        if (dotKantor) { 
          dotKantor.className = 'pulse-dot green'; 
          dotKantor.style.background = '#10b981'; 
          dotKantor.style.boxShadow = '0 0 8px #10b981'; 
        }
      }
    }

    if (bannerClosed) {
      bannerClosed.style.display = isClosed ? 'block' : 'none';
    }

    // Ubah pencahayaan 3D kantor
    if (office) {
      office.toggleDayNight(isClosed);
    }
    const dayNightIcon = document.getElementById('daynight-icon');
    if (dayNightIcon) dayNightIcon.textContent = isClosed ? '🌙' : '☀️';

    // Perbarui Tag Visual Agen di Ruang 3D
    AGENTS_DATA.forEach(ag => {
      const tagRole = document.getElementById(`tag-role-${ag.id}`);
      if (tagRole) {
        tagRole.textContent = isClosed ? '💤 Istirahat' : ag.peran.split('/')[0].trim();
        tagRole.style.color = isClosed ? '#c084fc' : (ag.warna || '#38bdf8');
      }
      const tagDot = document.getElementById(`tag-dot-${ag.id}`);
      if (tagDot) {
        tagDot.style.background = isClosed ? '#a855f7' : '#10b981';
        tagDot.style.boxShadow = isClosed ? '0 0 6px #a855f7' : '0 0 6px #10b981';
      }
    });

    // Perbarui Roster Daftar Agen
    document.querySelectorAll('.roster-item').forEach(item => {
      const statusEl = item.querySelector('.roster-status');
      if (statusEl) {
        statusEl.textContent = isClosed ? '💤 Istirahat' : '🟢 Bekerja di Meja';
        statusEl.style.color = isClosed ? '#c084fc' : '#34d399';
      }
    });

    // Mainkan efek suara
    if (userTriggered) {
      if (isClosed) {
        sound.playOfficeClose();
        postAgentChatter({
          from: 'agen_orkestrator',
          msg: '🌙 Kantor resmi ditutup untuk jam istirahat. Seluruh 10 AI Agent standby berdaulat.'
        });
      } else {
        sound.playOfficeOpen();
        postAgentChatter({
          from: 'agen_orkestrator',
          msg: '☀️ Kantor digital dibuka kembali! 10 AI Agent aktif kembali di meja kerja masing-masing.'
        });
      }
    }
  }

  async function toggleOfficeMode() {
    sound.playClick();
    const nextMode = currentOfficeMode === 'BUKA' ? 'TUTUP' : 'BUKA';
    applyOfficeMode(nextMode, true);

    try {
      await fetch('/api/kantor/mode', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ mode: nextMode, diubah_oleh: 'Pak Satriyo (Web UI)' })
      });
    } catch (e) {
      console.warn('Gagal simpan mode kantor:', e);
    }
  }

  // Listener Tombol Buka/Tutup Kantor
  const btnToggleKantor = document.getElementById('btn-toggle-kantor');
  if (btnToggleKantor) btnToggleKantor.addEventListener('click', toggleOfficeMode);

  const btnWakeOffice = document.getElementById('btn-wake-office');
  if (btnWakeOffice) btnWakeOffice.addEventListener('click', () => {
    applyOfficeMode('BUKA', true);
    fetch('/api/kantor/mode', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ mode: 'BUKA', diubah_oleh: 'Pak Satriyo (Web UI Wake)' })
    }).catch(() => {});
  });

  // Sinkronisasi status mode kantor dari server
  function fetchOfficeModeSync() {
    fetch('/api/kantor/mode')
      .then(r => r.json())
      .then(d => {
        if (d && d.status && d.status !== currentOfficeMode) {
          applyOfficeMode(d.status, false);
        }
      })
      .catch(() => {});
  }

  fetchOfficeModeSync();
  setInterval(fetchOfficeModeSync, 3500);

  // Sinkronisasi periodik progres portofolio & kanban dari basis data
  refreshProjectsAndKanban();
  setInterval(refreshProjectsAndKanban, 3000);

  // Listener Klik pada Badge Progres Portofolio di HUD
  const btnHudProgress = document.getElementById('btn-hud-progress');
  if (btnHudProgress) {
    btnHudProgress.addEventListener('click', () => {
      sound.playClick();
      const kanbanModal = document.getElementById('kanban-modal');
      if (kanbanModal) {
        kanbanModal.style.display = 'flex';
        refreshProjectsAndKanban();
      }
    });
  }

  // ================= CONTROLLER AUTO-WORKER PROGRES MANDIRI =================
  const btnToggleAutoWorker = document.getElementById('btn-toggle-autoworker');
  const txtAutoWorker = document.getElementById('autoworker-text');
  const btnTriggerAutoWork = document.getElementById('btn-trigger-autowork');

  function fetchAutoWorkerStatus() {
    fetch('/api/kantor/autoworker')
      .then(r => r.json())
      .then(d => {
        if (btnToggleAutoWorker && txtAutoWorker) {
          if (d.aktif) {
            btnToggleAutoWorker.className = 'btn-pill active';
            txtAutoWorker.textContent = 'Auto-Progres: ON';
          } else {
            btnToggleAutoWorker.className = 'btn-pill';
            txtAutoWorker.textContent = 'Auto-Progres: OFF';
          }
        }
      })
      .catch(() => {});
  }

  if (btnToggleAutoWorker) {
    btnToggleAutoWorker.addEventListener('click', async () => {
      sound.playClick();
      try {
        const res = await fetch('/api/kantor/autoworker/toggle', { method: 'POST' });
        const d = await res.json();
        if (btnToggleAutoWorker && txtAutoWorker) {
          if (d.aktif) {
            btnToggleAutoWorker.className = 'btn-pill active';
            txtAutoWorker.textContent = 'Auto-Progres: ON';
            postAgentChatter({
              from: 'agen_orkestrator',
              msg: '⚡ Mode Auto-Progres diaktifkan! Agen akan memajukan tugas secara berkala.'
            });
          } else {
            btnToggleAutoWorker.className = 'btn-pill';
            txtAutoWorker.textContent = 'Auto-Progres: OFF';
            postAgentChatter({
              from: 'agen_orkestrator',
              msg: '⏸️ Mode Auto-Progres dijeda. Pengerjaan tugas beralih ke kendali manual.'
            });
          }
        }
      } catch (e) {}
    });
  }

  if (btnTriggerAutoWork) {
    btnTriggerAutoWork.addEventListener('click', async () => {
      sound.playClick();
      try {
        btnTriggerAutoWork.disabled = true;
        const res = await fetch('/api/kantor/autoworker/step', { method: 'POST' });
        const d = await res.json();
        btnTriggerAutoWork.disabled = false;
        if (d.sukses) {
          sound.playMeetingChime();
          refreshProjectsAndKanban();
          const agId = (d.tugas && d.tugas.pic_agen) ? d.tugas.pic_agen : 'agen_orkestrator';
          postAgentChatter({
            from: agId,
            msg: d.deskripsi || 'Satu langkah progres tugas berhasil diselesaikan.'
          });
          if (office && office.agents3D) {
            const a3d = office.agents3D.get(agId);
            if (a3d) office.setCameraView('focus', a3d.deskPos);
          }
        } else {
          alert(d.alasan || 'Tidak dapat memajukan tugas.');
        }
      } catch (e) {
        btnTriggerAutoWork.disabled = false;
      }
    });
  }

  fetchAutoWorkerStatus();

  // ================= TOGGLE SIANG / MALAM =================
  document.getElementById('btn-toggle-daynight').addEventListener('click', () => {
    sound.playClick();
    const isNowNight = office ? !office.isNight : !document.body.classList.contains('theme-night');
    if (office) office.toggleDayNight(isNowNight);
    document.getElementById('daynight-icon').textContent = isNowNight ? '🌙' : '☀️';
    document.body.className = isNowNight ? 'theme-night' : 'theme-day';
  });

  // ================= TOGGLE AUDIO =================
  document.getElementById('btn-toggle-sound').addEventListener('click', () => {
    sound.enabled = !sound.enabled;
    document.getElementById('sound-icon').textContent = sound.enabled ? '🔊' : '🔇';
    if (sound.enabled) sound.playClick();
  });

  // ================= TOGGLE KAMERA =================
  document.getElementById('btn-toggle-cam').addEventListener('click', () => {
    sound.playClick();
    const txt = document.getElementById('cam-mode-text');
    if (office) {
      if (office.camMode === 'iso') {
        office.setCameraView('top');
        txt.textContent = 'Top-Down';
      } else {
        office.setCameraView('iso');
        txt.textContent = 'Isometrik';
      }
    }
  });

  document.getElementById('btn-reset-view').addEventListener('click', () => {
    sound.playClick();
    if (office) office.setCameraView('iso');
    document.getElementById('cam-mode-text').textContent = 'Isometrik';
  });

  // ================= TOGGLE TAG VISUAL AGEN =================
  const btnToggleTags = document.getElementById('btn-toggle-tags');
  if (btnToggleTags) {
    btnToggleTags.addEventListener('click', () => {
      sound.playClick();
      if (office) {
        const isVis = office.toggleTags();
        btnToggleTags.classList.toggle('active', isVis);
        const txt = document.getElementById('tags-mode-text');
        if (txt) txt.textContent = isVis ? 'Tag: ON' : 'Tag: OFF';
      }
    });
  }

  // ================= TOGGLE TINGKAT KECERAHAN RUANGAN =================
  const btnToggleLighting = document.getElementById('btn-toggle-lighting');
  let currentLightLevel = 'bright'; // 'bright' -> 'normal' -> 'dim'
  if (btnToggleLighting) {
    btnToggleLighting.addEventListener('click', () => {
      sound.playClick();
      if (office) {
        const txt = document.getElementById('light-mode-text');
        if (currentLightLevel === 'bright') {
          currentLightLevel = 'normal';
          office.setBrightness('normal');
          if (txt) txt.textContent = 'Normal';
        } else if (currentLightLevel === 'normal') {
          currentLightLevel = 'dim';
          office.setBrightness('dim');
          if (txt) txt.textContent = 'Redup';
        } else {
          currentLightLevel = 'bright';
          office.setBrightness('bright');
          if (txt) txt.textContent = 'Terang';
        }
      }
    });
  }

  // ================= LIVE INTER-AGENT CHATTER MONITOR =================
  const CHATTER_POOL = [
    {
      from: 'agen_orkestrator',
      msg: 'Dian, mohon pastikan user acceptance criteria transaksi tabungan koperasi sudah final untuk sprint ini.'
    },
    {
      from: 'agen_analis',
      msg: 'Sudah final Mas Arya! Kebutuhan validasi transaksi & limit harian sudah dicocokkan dengan SOP-02.'
    },
    {
      from: 'agen_arsitek',
      msg: 'Skema DDL WidyaBase kantor_digital.wdb sudah aktif. Buffer pool 512 pages siap untuk throughput tinggi.'
    },
    {
      from: 'agen_coder',
      msg: 'Endpoint transfer & penarikan simpanan tuntas dikoding dengan hashing Argon2id & session guard.'
    },
    {
      from: 'agen_qa',
      msg: 'Eksekusi automated fuzzing & test injection... 100% pass! Beban 500 concurrent request stabil tanpa lag.'
    },
    {
      from: 'agen_devops',
      msg: 'Daemon SSE aktif di port 8085. Resource CPU 3.8%, snapshot backup otomatis terverifikasi utuh.'
    },
    {
      from: 'agen_keamanan',
      msg: 'Audit kepatuhan UU PDP: Seluruh payload terenkripsi, zero raw credential leakage terverifikasi bersih!'
    },
    {
      from: 'agen_notulis',
      msg: 'Notulensi sidang & progress kanban sudah otomatis disinkronkan ke Obsidian Vault (C:/Satriyo/Vault).'
    },
    {
      from: 'agen_humas',
      msg: 'Jalur Telegram Bot Gateway (@WidyaDigitalBot) siaga penuh untuk memancarkan laporan resmi ke Pak Satriyo.'
    }
  ];

  let chatterIndex = 0;
  const chatterFeed = document.getElementById('chatter-feed');
  const chatterDock = document.getElementById('hud-chatter-dock');

  function postAgentChatter(forcedItem = null) {
    // Cek apakah ada pekerjaan aktif di tim kantor
    const workloadMap = window.AGENT_WORKLOAD || {};
    const hasActiveWork = Object.values(workloadMap).some(v => v > 0);

    // Jika seluruh tim nol kerjaan (beban = 0) dan bukan dipaksa secara manual, jangan ada percakapan antar agen
    if (!hasActiveWork && !forcedItem) {
      return;
    }

    const item = forcedItem || CHATTER_POOL[chatterIndex % CHATTER_POOL.length];
    if (!forcedItem) chatterIndex++;

    const agent = AGENTS_DATA.find(a => a.id === item.from);
    if (!agent) return;

    // Jika agen ini bebannya 0 (sedang santai/idle walk), jangan biarkan dia bicara urusan tugas proyek kecuali dipaksa
    if (workloadMap[agent.id] === 0 && !forcedItem) {
      return;
    }

    const now = new Date();
    const timeStr = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}:${String(now.getSeconds()).padStart(2, '0')}`;

    // 1. Tambahkan ke Feed UI
    if (chatterFeed) {
      const el = document.createElement('div');
      el.className = 'chatter-item';
      el.style.setProperty('--agent-color', agent.warna);
      const chatterAvatarGraphic = (typeof getAgentAvatarSvg === 'function')
        ? getAgentAvatarSvg(agent.id, 26)
        : agent.avatar;

      el.innerHTML = `
        <div class="chatter-avatar">${chatterAvatarGraphic}</div>
        <div class="chatter-body">
          <div class="chatter-meta">
            <span class="chatter-author">${agent.nama}</span>
            <span class="chatter-time">${timeStr}</span>
          </div>
          <div class="chatter-msg">${item.msg}</div>
        </div>
      `;
      chatterFeed.appendChild(el);
      chatterFeed.scrollTop = chatterFeed.scrollHeight;

      while (chatterFeed.children.length > 25) {
        chatterFeed.removeChild(chatterFeed.firstChild);
      }
    }

    // 2. Picu Balon Ucapan 3D di Atas Kepala Agen
    if (office && typeof office.showSpeechBubble === 'function') {
      office.showSpeechBubble(item.from, item.msg, 6500);
    }
  }

  // Ticker otomatis setiap 8 detik — hanya aktif jika ada agen yang sedang memiliki beban kerja aktif
  setInterval(() => {
    const workloadMap = window.AGENT_WORKLOAD || {};
    const hasActiveWork = Object.values(workloadMap).some(v => v > 0);
    if (hasActiveWork) {
      postAgentChatter();
    }
  }, 8000);

  // Tombol Pancing Diskusi Tim di header chatter dock
  const btnTriggerChatter = document.getElementById('btn-trigger-chatter');
  if (btnTriggerChatter) {
    btnTriggerChatter.addEventListener('click', () => {
      sound.playClick();
      const workloadMap = window.AGENT_WORKLOAD || {};
      const hasActiveWork = Object.values(workloadMap).some(v => v > 0);
      if (hasActiveWork) {
        postAgentChatter();
      } else {
        if (chatterFeed) {
          const el = document.createElement('div');
          el.className = 'chatter-item';
          el.style.setProperty('--agent-color', '#fbbf24');
          el.innerHTML = `
            <div class="chatter-avatar">☕</div>
            <div class="chatter-body">
              <div class="chatter-meta">
                <span class="chatter-author" style="color: #fbbf24;">Status Kantor</span>
                <span class="chatter-time">IDLE</span>
              </div>
              <div class="chatter-msg" style="color: #fde68a; font-style: italic;">Semua beban kerja selesai (0%). Agen sedang santai di pantry, lounge, dan jendela. Tidak ada percakapan tugas aktif.</div>
            </div>
          `;
          chatterFeed.appendChild(el);
          chatterFeed.scrollTop = chatterFeed.scrollHeight;
        }
      }
    });
  }

  // Tombol Minimalkan Chatter Dock
  const btnMinChatter = document.getElementById('btn-min-chatter');
  if (btnMinChatter && chatterDock) {
    btnMinChatter.addEventListener('click', () => {
      sound.playClick();
      chatterDock.classList.toggle('minimized');
      btnMinChatter.textContent = chatterDock.classList.contains('minimized') ? '+' : '−';
    });
  }

  // Tombol di Bottom Toolbar
  const btnDockChatterToggle = document.getElementById('btn-dock-chatter-toggle');
  if (btnDockChatterToggle && chatterDock) {
    btnDockChatterToggle.addEventListener('click', () => {
      sound.playClick();
      if (chatterDock.classList.contains('minimized')) {
        chatterDock.classList.remove('minimized');
        if (btnMinChatter) btnMinChatter.textContent = '−';
      } else {
        postAgentChatter();
      }
    });
  }

  // Tombol Sinkronisasi Obsidian di Top HUD
  const btnHudObsidian = document.getElementById('btn-hud-obsidian');
  if (btnHudObsidian) {
    btnHudObsidian.addEventListener('click', async () => {
      sound.playClick();
      const statusLabel = document.getElementById('obsidian-status');
      if (statusLabel) statusLabel.textContent = '⏳ Menyinkronkan...';
      try {
        const res = await fetch('/api/obsidian/sync');
        const data = await res.json();
        if (data.sukses) {
          if (statusLabel) statusLabel.textContent = `✓ C:/Satriyo/Vault (${data.total_proyek} Proyek)`;
          postAgentChatter({
            from: 'agen_notulis',
            msg: `✓ Sinkronisasi Obsidian tuntas! Seluruh ${data.total_proyek} proyek & ${data.total_tugas} penugasan telah tercatat rapi di C:/Satriyo/Vault.`
          });
        } else {
          if (statusLabel) statusLabel.textContent = 'C:/Satriyo/Vault Terhubung';
        }
      } catch (err) {
        if (statusLabel) statusLabel.textContent = 'C:/Satriyo/Vault Terhubung';
      }
    });
  }

  // ================= DIRECT AGENT COMMUNICATIONS MANAGER =================
  const modalComms = document.getElementById('modal-direct-comms');
  const btnOpenComms = document.getElementById('btn-hud-comms');
  const btnCloseComms = document.getElementById('btn-close-comms');
  const btnDossierChat = document.getElementById('btn-dossier-chat');
  const commsSelect = document.getElementById('comms-agent-select');
  const commsHeaderAvatar = document.getElementById('comms-header-avatar');
  const commsStream = document.getElementById('comms-messages-stream');
  const formComms = document.getElementById('form-comms');
  const inputCommsText = document.getElementById('input-comms-text');

  let commsInitialized = false;

  function updateCommsHeaderAvatar(agentId) {
    if (!commsHeaderAvatar) return;
    if (agentId === 'semua_agen') {
      commsHeaderAvatar.innerHTML = typeof getAgentAvatarSvg === 'function' ? getAgentAvatarSvg('agen_orkestrator', 46) : '👥';
      commsHeaderAvatar.style.borderColor = '#38bdf8';
    } else {
      const ag = AGENTS_DATA.find(a => a.id === agentId);
      commsHeaderAvatar.innerHTML = typeof getAgentAvatarSvg === 'function' ? getAgentAvatarSvg(agentId, 46) : (ag ? ag.avatar : '🤖');
      commsHeaderAvatar.style.borderColor = ag ? ag.warna : '#38bdf8';
    }
  }

  function openDirectComms(preselectAgentId = null) {
    if (!modalComms) return;
    sound.playClick();
    modalComms.style.display = 'flex';

    if (preselectAgentId && commsSelect) {
      commsSelect.value = preselectAgentId;
    }
    const currentTarget = commsSelect ? commsSelect.value : 'semua_agen';
    updateCommsHeaderAvatar(currentTarget);

    // Initial greeting if stream is empty
    if (!commsInitialized && commsStream) {
      commsInitialized = true;
      appendAgentCommsMessage({
        agentId: 'agen_orkestrator',
        nama: 'Arya Wicaksana, PMP',
        peran: 'Mandor Utama / PM',
        jawaban: 'Selamat datang di Saluran Komunikasi Langsung Kantor Digital Widya, Pak Satriyo! Saya Arya Wicaksana bersama 8 rekan agen spesialis siap menerima instruksi strategis, pendelegasian tugas, audit keamanan, atau pertanyaan teknis terkait seluruh portofolio proyek industri kita.',
        tindakan: 'Saluran enkripsi sesi lokal aktif • Terkoneksi ke WidyaBase WAL & Obsidian'
      });
    }

    if (inputCommsText) {
      setTimeout(() => inputCommsText.focus(), 100);
    }
  }

  if (btnOpenComms) {
    btnOpenComms.addEventListener('click', () => openDirectComms());
  }

  if (btnCloseComms && modalComms) {
    btnCloseComms.addEventListener('click', () => {
      sound.playClick();
      modalComms.style.display = 'none';
    });
  }

  if (btnDossierChat) {
    btnDossierChat.addEventListener('click', () => {
      const currentSelectedAgent = document.querySelector('.roster-item.active');
      let targetId = 'agen_orkestrator';
      if (currentSelectedAgent && currentSelectedAgent.id) {
        targetId = currentSelectedAgent.id.replace('roster-', '');
      }
      openDirectComms(targetId);
    });
  }

  if (commsSelect) {
    commsSelect.addEventListener('change', () => {
      sound.playClick();
      const val = commsSelect.value;
      updateCommsHeaderAvatar(val);
      if (val !== 'semua_agen' && office && office.agents3D) {
        const a3d = office.agents3D.get(val);
        if (a3d) office.setCameraView('focus', a3d.isMeeting ? a3d.meetPos : a3d.deskPos);
      }
    });
  }

  // Quick Prompt Chips
  document.querySelectorAll('.chip-prompt').forEach(chip => {
    chip.addEventListener('click', () => {
      sound.playClick();
      const promptText = chip.getAttribute('data-prompt');
      if (inputCommsText) {
        inputCommsText.value = promptText;
        if (formComms) formComms.requestSubmit();
      }
    });
  });

  function appendUserCommsMessage(text) {
    if (!commsStream) return;
    const now = new Date();
    const timeStr = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}`;

    const row = document.createElement('div');
    row.className = 'comms-bubble-row user';
    row.innerHTML = `
      <div class="comms-bubble-avatar user-av">👨‍💼</div>
      <div class="comms-bubble-content">
        <div class="comms-bubble-header">
          <span class="comms-bubble-time">${timeStr}</span>
          <span class="comms-bubble-author" style="color: #60a5fa;">Pak Satriyo (User)</span>
        </div>
        <div class="comms-bubble-card">${text}</div>
      </div>
    `;
    commsStream.appendChild(row);
    commsStream.scrollTop = commsStream.scrollHeight;
  }

  function appendAgentCommsMessage(data) {
    if (!commsStream) return;
    const now = new Date();
    const timeStr = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}`;

    const agent = AGENTS_DATA.find(a => a.id === data.agentId) || AGENTS_DATA[0];
    const avatarSvg = typeof getAgentAvatarSvg === 'function' ? getAgentAvatarSvg(data.agentId, 38) : agent.avatar;

    const row = document.createElement('div');
    row.className = 'comms-bubble-row agent';
    row.style.setProperty('--agent-color', agent.warna);

    let actionHtml = '';
    if (data.tindakan) {
      actionHtml = `<div class="comms-action-tag">⚡ ${data.tindakan}</div>`;
    }

    row.innerHTML = `
      <div class="comms-bubble-avatar">${avatarSvg}</div>
      <div class="comms-bubble-content">
        <div class="comms-bubble-header">
          <span class="comms-bubble-author">${data.nama || agent.nama}</span>
          <span class="comms-bubble-time">${timeStr} • ${data.peran || agent.peran}</span>
        </div>
        <div class="comms-bubble-card">
          ${(data.jawaban || '').replace(/\n/g, '<br>')}
          ${actionHtml}
        </div>
      </div>
    `;
    commsStream.appendChild(row);
    commsStream.scrollTop = commsStream.scrollHeight;
  }

  if (formComms) {
    formComms.addEventListener('submit', async (e) => {
      e.preventDefault();
      const text = (inputCommsText.value || '').trim();
      if (!text) return;
      inputCommsText.value = '';

      const targetAgentId = commsSelect ? commsSelect.value : 'agen_orkestrator';
      sound.playClick();
      appendUserCommsMessage(text);

      // Typing Indicator
      const typingIndicator = document.createElement('div');
      typingIndicator.className = 'comms-typing-indicator';
      typingIndicator.id = 'comms-typing-ind';
      typingIndicator.innerHTML = `
        <span class="typing-dot"></span>
        <span class="typing-dot"></span>
        <span class="typing-dot"></span>
        <span>Agen sedang menyusun respon berdasarkan basis data WidyaBase...</span>
      `;
      commsStream.appendChild(typingIndicator);
      commsStream.scrollTop = commsStream.scrollHeight;

      try {
        const response = await fetch('/api/agen/chat', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            agentId: targetAgentId,
            pesan: text,
            kodeProyek: 'PRJ-ARKAS-WEB-01'
          })
        });
        const result = await response.json();
        const ind = document.getElementById('comms-typing-ind');
        if (ind) ind.remove();

        sound.playMeetingChime();
        appendAgentCommsMessage(result);

        // Reaksi 3D Kantor Digital
        const realAgentId = (result.agentId && result.agentId !== 'semua_agen') ? result.agentId : 'agen_orkestrator';
        if (office && typeof office.showSpeechBubble === 'function') {
          office.showSpeechBubble(realAgentId, result.speechText || result.jawaban.slice(0, 80), 8000);
        }
        if (office && office.agents3D) {
          const a3d = office.agents3D.get(realAgentId);
          if (a3d) {
            office.setCameraView('focus', a3d.isMeeting ? a3d.meetPos : a3d.deskPos);
          }
        }
        // Tambahkan ke Chatter Feed
        postAgentChatter({
          from: realAgentId,
          msg: `[Instruksi User]: ${result.speechText || result.jawaban.slice(0, 90)}`
        });

      } catch (err) {
        const ind = document.getElementById('comms-typing-ind');
        if (ind) ind.remove();
        appendAgentCommsMessage({
          agentId: targetAgentId,
          nama: 'Sistem Terdistribusi',
          jawaban: `Maaf, gagal memproses komunikasi: ${err.message}`
        });
      }
    });
  }

  // JAM DIGITAL RUNNER
  function updateClock() {
    const now = new Date();
    const h = String(now.getHours()).padStart(2, '0');
    const m = String(now.getMinutes()).padStart(2, '0');
    const s = String(now.getSeconds()).padStart(2, '0');
    document.getElementById('hud-clock').textContent = `${h}:${m}:${s} WIB`;
  }
  setInterval(updateClock, 1000);
  updateClock();
});
