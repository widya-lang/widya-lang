// ==============================================================================
// app.js - WidyaOS Sovereign Web Desktop & REPL Terminal Logic
// ==============================================================================

let highestZ = 10;
let dragElement = null;
let dragOffset = { x: 0, y: 0 };

document.addEventListener('DOMContentLoaded', () => {
  startClock();
  setupDragging();
});

function startClock() {
  const update = () => {
    const now = new Date();
    const t = now.toLocaleTimeString('id-ID', { hour12: false }) + ' WIB';
    const el = document.getElementById('os-clock');
    if (el) el.innerText = t;
  };
  update();
  setInterval(update, 1000);
}

// WINDOW CONTROLS
function openWindow(id) {
  const win = document.getElementById('win-' + id);
  const btn = document.getElementById('task-btn-' + id);
  if (win) {
    win.style.display = 'flex';
    win.classList.add('active');
    highestZ += 1;
    win.style.zIndex = highestZ;
  }
  if (btn) btn.classList.add('active');
}

function closeWindow(id) {
  const win = document.getElementById('win-' + id);
  const btn = document.getElementById('task-btn-' + id);
  if (win) {
    win.style.display = 'none';
    win.classList.remove('active');
  }
  if (btn) btn.classList.remove('active');
}

function minimizeWindow(id) {
  closeWindow(id);
}

function toggleWindow(id) {
  const win = document.getElementById('win-' + id);
  if (win && win.style.display === 'flex') {
    closeWindow(id);
  } else {
    openWindow(id);
  }
}

function toggleStartMenu() {
  const m = document.getElementById('start-menu');
  m.classList.toggle('active');
}

// DRAG ENGINE
function setupDragging() {
  document.addEventListener('mousemove', (e) => {
    if (!dragElement) return;
    dragElement.style.left = (e.clientX - dragOffset.x) + 'px';
    dragElement.style.top = (e.clientY - dragOffset.y) + 'px';
  });

  document.addEventListener('mouseup', () => {
    dragElement = null;
  });
}

function startDrag(e, winId) {
  const win = document.getElementById(winId);
  if (!win) return;

  highestZ += 1;
  win.style.zIndex = highestZ;

  dragElement = win;
  dragOffset.x = e.clientX - win.offsetLeft;
  dragOffset.y = e.clientY - win.offsetTop;
}

// REPL TERMINAL ENGINE
function focusTerminalInput() {
  const input = document.getElementById('term-input');
  if (input) input.focus();
}

function handleTerminalKey(e) {
  if (e.key !== 'Enter') return;
  const input = document.getElementById('term-input');
  const cmd = input.value.trim();
  input.value = '';

  if (!cmd) return;

  const out = document.getElementById('term-output');
  out.innerHTML += `<div class="term-line"><span class="term-prompt">widya@sovereign:~$</span> ${cmd}</div>`;

  // Parse Commands
  const lower = cmd.toLowerCase();

  if (lower === 'clear' || lower === 'bersihkan') {
    out.innerHTML = '';
    return;
  }

  if (lower === 'bantuan' || lower === 'help') {
    out.innerHTML += `
      <div class="term-line text-muted">Perintah bawaan WidyaOS Terminal:</div>
      <div class="term-line">  <strong>cetak(teks)</strong> : Mencetak teks output</div>
      <div class="term-line">  <strong>ps / proses</strong>  : Melihat status 10 AI Agent</div>
      <div class="term-line">  <strong>widyabase</strong>    : Cek status basis data ACID</div>
      <div class="term-line">  <strong>proyek</strong>       : Daftar 10 portofolio industri</div>
      <div class="term-line">  <strong>neofetch</strong>     : Info sistem operasi WidyaOS</div>
      <div class="term-line">  <strong>clear</strong>        : Membersihkan layar terminal</div>
    `;
  } else if (lower.startsWith('cetak(') && lower.endsWith(')')) {
    const content = cmd.slice(6, -1).replace(/^['"]|['"]$/g, '');
    out.innerHTML += `<div class="term-line text-emerald">${content}</div>`;
  } else if (lower === 'ps' || lower === 'proses') {
    out.innerHTML += `
      <div class="term-line">PID   USER             AGENT                STATUS</div>
      <div class="term-line">101   sovereign        agen_orkestrator     IDLE (0%)</div>
      <div class="term-line">102   sovereign        agen_coder           IDLE (0%)</div>
      <div class="term-line">103   sovereign        agen_arsitek         IDLE (0%)</div>
      <div class="term-line">104   sovereign        agen_qa              IDLE (0%)</div>
      <div class="term-line">105   sovereign        agen_keamanan        IDLE (0%)</div>
      <div class="term-line text-emerald">Semua 10 AI Agent dalam kondisi sehat dan siap tugas.</div>
    `;
  } else if (lower === 'widyabase') {
    out.innerHTML += `
      <div class="term-line text-emerald">🗄️  WIDYABASE v1.0.0 (ACID & WAL OK)</div>
      <div class="term-line">   Berkas: /home/devops/widya-lang/agen_kantor_digital/kantor_digital.wdb</div>
      <div class="term-line">   Tabel : 8 Tabel Aktif | Buffer Pool: 512 Pages</div>
    `;
  } else if (lower === 'proyek') {
    out.innerHTML += `
      <div class="term-line">🚀 10 PORTOFOLIO INDUSTRI BERDAULAT:</div>
      <div class="term-line">   1. PRJ-01-KOPERASI  : Sistem Koperasi KDMP 7 Pilar</div>
      <div class="term-line">   2. PRJ-02-SMARTCITY : Telemetri ISPU & GIS Smart City</div>
      <div class="term-line">   3. PRJ-03-COLLAB-DOC: CRDT Collaborative Document Engine</div>
      <div class="term-line">   4. PRJ-04-DAPODIK   : Pelaporan Dapodik & PDF ISO 32000-1</div>
      <div class="term-line">   5. PRJ-05-JOURNAL   : LaTeX Scientific Journal Previewer</div>
      <div class="term-line">   6. PRJ-06-WIDYAPAY  : Switcher QRIS & BI-FAST 50k TPS</div>
      <div class="term-line">   7. PRJ-07-MEDIKA    : SIMRS & SatuSehat HL7 FHIR</div>
      <div class="term-line">   8. PRJ-08-WIDYAOS   : Bare-Metal Kernel & OEM Bundling</div>
      <div class="term-line">   9. PRJ-09-AI-BOX    : WidyaAI Box Offline & Sandi PQC</div>
      <div class="term-line">  10. PRJ-10-LEDGER    : Blockchain RWA Tokenization</div>
    `;
  } else if (lower === 'neofetch') {
    out.innerHTML += `
      <div class="term-line" style="color: #60a5fa;">        ██╗    ██╗██╗██████╗ ██╗   ██╗ █████╗ </div>
      <div class="term-line" style="color: #60a5fa;">        ██║    ██║██║██╔══██╗╚██╗ ██╔╝██╔══██╗</div>
      <div class="term-line" style="color: #60a5fa;">        ██║ █╗ ██║██║██║  ██║ ╚████╔╝ ███████║</div>
      <div class="term-line" style="color: #60a5fa;">        ██║███╗██║██║██║  ██║  ╚██╔╝  ██╔══██║</div>
      <div class="term-line" style="color: #60a5fa;">        ╚███╔███╔╝██║██████╔╝   ██║   ██║  ██║</div>
      <div class="term-line">   OS     : WidyaOS Sovereign Edition (2026)</div>
      <div class="term-line">   Kernel : Rust Bare-Metal SMP x86_64</div>
      <div class="term-line">   Memory : 184MB / 16384MB (Efficient)</div>
      <div class="term-line">   Engine : WidyaBase ACID + WidyaBox Container</div>
    `;
  } else {
    out.innerHTML += `<div class="term-line" style="color: #f87171;">Galat: Perintah '${cmd}' tidak dikenali. Ketik 'bantuan' untuk panduan.</div>`;
  }

  out.parentElement.scrollTop = out.parentElement.scrollHeight;
}
