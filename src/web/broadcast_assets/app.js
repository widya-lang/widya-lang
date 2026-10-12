// ==============================================================================
// WidyaWeb Sovereign Broadcast Studio MCR - Core Application Engine
// ==============================================================================

const state = {
  activeProgScene: 'cam_a_wide',
  activePrevScene: 'cam_a_close',
  tBarValue: 0,
  isLive: true,
  smpteFrames: 0,
  smpteSeconds: 0,
  smpteMinutes: 0,
  smpteHours: 14,
  
  // Scoreboard
  match: {
    kategori: 'FINAL KELAS C PUTRA - 60KG',
    babak: 2,
    timerSeconds: 114,
    timerRunning: true,
    merah: { nama: 'Bayu Pratama (JATIM)', poin: 31, binaan: 0, teguran: 0 },
    biru: { nama: 'Asep Sunandar (JABAR)', poin: 24, binaan: 0, teguran: 0 }
  },

  // VAR Session
  var: {
    active: false,
    challengeBy: 'SUDUT_MERAH',
    reason: 'Bantingan Sah Masuk +3 Poin',
    scrubOffset: -15,
    playbackSpeed: 0.25,
    decision: null
  },

  // AI & Biomechanics
  ai: {
    showSkeleton: true,
    strikeSpeedKmh: 64.8,
    reactionTimeMs: 140,
    impactForceG: 18.5,
    hawkEyeDistanceM: 3.4,
    hawkEyeSafe: true,
    virtualDirectorAutoCut: true
  },

  // BCI Telemetry
  bci: {
    merah: { focus: 78.4, panic: 14.2, alpha: 26.5, beta: 38.2, theta: 11.4, state: 'FLOW_STATE' },
    biru: { focus: 54.0, panic: 62.5, alpha: 12.0, beta: 58.5, theta: 16.0, state: 'PRESSURE' }
  },

  // Saweria Tips
  tips: {
    merahIdr: 1250000,
    biruIdr: 850000,
    lastTip: { sender: 'Sultan_Surabaya', corner: 'MERAH', amount: 500000, msg: 'Semangat Mas Bayu!' }
  },

  // Bonded Cellular
  bonded: {
    modems: [
      { name: 'Telkomsel 5G', bw: 185, healthy: true },
      { name: 'Indosat 5G', bw: 140, healthy: true },
      { name: 'XL Axiata LTE', bw: 65, healthy: true },
      { name: 'Starlink Mini LEO', bw: 220, healthy: true }
    ],
    totalBw: 610,
    lossPct: 0.0
  }
};

// Canvas references
let prevCanvas, prevCtx;
let progCanvas, progCtx;
const thumbCanvases = {};

// Initialization
document.addEventListener('DOMContentLoaded', () => {
  initCanvases();
  setupEventListeners();
  startMasterLoops();
  broadcastTallyState();
});

function initCanvases() {
  prevCanvas = document.getElementById('canvas-preview');
  prevCtx = prevCanvas.getContext('2d');
  progCanvas = document.getElementById('canvas-program');
  progCtx = progCanvas.getContext('2d');

  const camIds = ['cam_a_wide', 'cam_a_close', 'cam_b_wide', 'cam_b_close', 'cam_c_wide', 'skycam_3d'];
  camIds.forEach(id => {
    const el = document.getElementById(`thumb-${id}`);
    if (el) {
      thumbCanvases[id] = { canvas: el, ctx: el.getContext('2d') };
    }
  });
}

function setupEventListeners() {
  // Tabs
  document.querySelectorAll('.tab-btn').forEach(btn => {
    btn.addEventListener('click', (e) => {
      document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
      e.target.classList.add('active');
      const tabId = e.target.dataset.tab;
      document.getElementById(`tab-${tabId}`).classList.add('active');
    });
  });

  // T-Bar
  const tbar = document.getElementById('tbar-slider');
  if (tbar) {
    tbar.addEventListener('input', (e) => {
      state.tBarValue = parseFloat(e.target.value) / 100;
      if (state.tBarValue >= 0.98) {
        executeCut();
        tbar.value = 0;
        state.tBarValue = 0;
      }
    });
  }
}

// ==============================================================================
// MASTER RENDERING & ANIMATION LOOP (60 FPS)
// ==============================================================================
let animTime = 0;

function startMasterLoops() {
  // SMPTE Timecode generator
  setInterval(() => {
    state.smpteFrames++;
    if (state.smpteFrames >= 60) {
      state.smpteFrames = 0;
      state.smpteSeconds++;
      if (state.smpteSeconds >= 60) {
        state.smpteSeconds = 0;
        state.smpteMinutes++;
      }
    }
    updateTimecodeDisplay();
  }, 1000 / 60);

  // Match Round Timer
  setInterval(() => {
    if (state.match.timerRunning && state.match.timerSeconds > 0) {
      state.match.timerSeconds--;
      updateScoreboardDisplay();
    }
  }, 1000);

  // BCI Oscillations
  setInterval(simulateBciFluctuations, 800);

  // 60 FPS Render Loop
  function renderFrame() {
    animTime += 0.03;
    drawThumbnails();
    drawMasterMonitors();
    requestAnimationFrame(renderFrame);
  }
  requestAnimationFrame(renderFrame);
}

function updateTimecodeDisplay() {
  const pad = (n) => String(n).padStart(2, '0');
  const tcStr = `${pad(state.smpteHours)}:${pad(state.smpteMinutes)}:${pad(state.smpteSeconds)}:${pad(state.smpteFrames)}`;
  const el = document.getElementById('smpte-display');
  if (el) el.textContent = tcStr;
}

function updateScoreboardDisplay() {
  const m = state.match;
  const min = Math.floor(m.timerSeconds / 60);
  const sec = m.timerSeconds % 60;
  const timerStr = `${min}:${String(sec).padStart(2, '0')}`;
  
  const elTimer = document.getElementById('hud-timer');
  if (elTimer) elTimer.textContent = timerStr;
  
  const elMerah = document.getElementById('hud-score-red');
  if (elMerah) elMerah.textContent = m.merah.poin;
  
  const elBiru = document.getElementById('hud-score-blue');
  if (elBiru) elBiru.textContent = m.biru.poin;

  // Sidebar scoreboard values
  const sideM = document.getElementById('side-score-red');
  if (sideM) sideM.textContent = m.merah.poin;
  const sideB = document.getElementById('side-score-blue');
  if (sideB) sideB.textContent = m.biru.poin;
}

// ==============================================================================
// DRAWING ARENA AND SILAT MATCH (CANVAS RENDERER)
// ==============================================================================
function drawSceneToCanvas(ctx, width, height, sceneId, isProgram) {
  ctx.save();
  ctx.clearRect(0, 0, width, height);

  // Background Studio / Matras
  const grad = ctx.createRadialGradient(width/2, height/2, 20, width/2, height/2, width*0.7);
  grad.addColorStop(0, '#0f2038');
  grad.addColorStop(1, '#050a14');
  ctx.fillStyle = grad;
  ctx.fillRect(0, 0, width, height);

  // Matras Silat (10m x 10m Green circle + Red boundary)
  const cx = width / 2;
  const cy = height / 2 + 20;
  const rMatras = Math.min(width, height) * 0.42;

  // Outer safety zone (grey)
  ctx.fillStyle = '#1e293b';
  ctx.beginPath();
  ctx.arc(cx, cy, rMatras * 1.15, 0, Math.PI * 2);
  ctx.fill();

  // Matras hijau
  ctx.fillStyle = '#065f46';
  ctx.beginPath();
  ctx.arc(cx, cy, rMatras, 0, Math.PI * 2);
  ctx.fill();
  ctx.strokeStyle = '#ef4444';
  ctx.lineWidth = 4;
  ctx.stroke();

  // Matras tengah (kuning 3m)
  ctx.strokeStyle = '#f59e0b';
  ctx.lineWidth = 3;
  ctx.beginPath();
  ctx.arc(cx, cy, rMatras * 0.4, 0, Math.PI * 2);
  ctx.stroke();

  // Sudut Merah & Sudut Biru indicators
  ctx.fillStyle = 'rgba(239, 68, 68, 0.7)';
  ctx.fillRect(cx - rMatras * 0.9, cy - 25, 30, 50);
  ctx.fillStyle = 'rgba(59, 130, 246, 0.7)';
  ctx.fillRect(cx + rMatras * 0.9 - 30, cy - 25, 30, 50);

  // Animate Pesilat Merah & Pesilat Biru
  const swayMerah = Math.sin(animTime * 3) * 15;
  const swayBiru = Math.cos(animTime * 3.2) * 12;

  const merahX = cx - 45 + swayMerah;
  const merahY = cy + Math.sin(animTime * 2) * 6;
  const biruX = cx + 45 + swayBiru;
  const biruY = cy + Math.cos(animTime * 2.5) * 6;

  // Shadow
  ctx.fillStyle = 'rgba(0,0,0,0.5)';
  ctx.beginPath();
  ctx.ellipse(merahX, merahY + 35, 18, 6, 0, 0, Math.PI * 2);
  ctx.fill();
  ctx.beginPath();
  ctx.ellipse(biruX, biruY + 35, 18, 6, 0, 0, Math.PI * 2);
  ctx.fill();

  // Draw Pesilat Merah
  ctx.fillStyle = '#ef4444';
  ctx.beginPath();
  ctx.arc(merahX, merahY - 10, 10, 0, Math.PI * 2); // Kepala
  ctx.fill();
  ctx.fillStyle = '#b91c1c';
  ctx.fillRect(merahX - 8, merahY, 16, 26); // Badan
  ctx.fillRect(merahX - 10, merahY + 26, 8, 14); // Kaki 1
  ctx.fillRect(merahX + 2, merahY + 26, 8, 14); // Kaki 2

  // Draw Pesilat Biru
  ctx.fillStyle = '#3b82f6';
  ctx.beginPath();
  ctx.arc(biruX, biruY - 10, 10, 0, Math.PI * 2); // Kepala
  ctx.fill();
  ctx.fillStyle = '#1d4ed8';
  ctx.fillRect(biruX - 8, biruY, 16, 26); // Badan
  ctx.fillRect(biruX - 10, biruY + 26, 8, 14); // Kaki 1
  ctx.fillRect(biruX + 2, biruY + 26, 8, 14); // Kaki 2

  // AI Skeleton Overlay (MediaPipe 33 Keypoints)
  if (state.ai.showSkeleton) {
    ctx.strokeStyle = '#22d3ee';
    ctx.lineWidth = 2;
    // Skeleton Merah
    ctx.beginPath();
    ctx.moveTo(merahX, merahY - 10);
    ctx.lineTo(merahX, merahY + 15);
    ctx.lineTo(merahX - 18, merahY + 8);
    ctx.moveTo(merahX, merahY + 15);
    ctx.lineTo(merahX + 16, merahY + 6);
    ctx.stroke();

    // Keypoint dots
    ctx.fillStyle = '#fbbf24';
    [ [merahX, merahY - 10], [merahX - 18, merahY + 8], [merahX + 16, merahY + 6], [merahX - 6, merahY + 38] ].forEach(pt => {
      ctx.beginPath();
      ctx.arc(pt[0], pt[1], 3, 0, Math.PI * 2);
      ctx.fill();
    });
  }

  // Camera perspective overlay
  ctx.fillStyle = 'rgba(255,255,255,0.7)';
  ctx.font = 'bold 11px JetBrains Mono';
  let camLabel = sceneId.toUpperCase().replace('_', ' ');
  if (sceneId === 'skycam_3d') camLabel = '🛸 SKYCAM 3D WIRECAM ORBIT';
  ctx.fillText(`CAM: ${camLabel}`, 12, 24);

  // VAR Overlay badge if active
  if (state.var.active && isProgram) {
    ctx.fillStyle = 'rgba(239, 68, 68, 0.9)';
    ctx.fillRect(width - 170, 12, 160, 26);
    ctx.fillStyle = '#fff';
    ctx.font = 'bold 11px JetBrains Mono';
    ctx.fillText('🔴 VAR REVIEW LIVE', width - 155, 30);
  }

  ctx.restore();
}

function drawMasterMonitors() {
  if (!prevCtx || !progCtx) return;

  // Preview
  drawSceneToCanvas(prevCtx, prevCanvas.width, prevCanvas.height, state.activePrevScene, false);

  // Program
  if (state.tBarValue > 0 && state.tBarValue < 1) {
    // Cross-fade composite
    drawSceneToCanvas(progCtx, progCanvas.width, progCanvas.height, state.activeProgScene, true);
    progCtx.save();
    progCtx.globalAlpha = state.tBarValue;
    drawSceneToCanvas(progCtx, progCanvas.width, progCanvas.height, state.activePrevScene, true);
    progCtx.restore();
  } else {
    drawSceneToCanvas(progCtx, progCanvas.width, progCanvas.height, state.activeProgScene, true);
  }
}

function drawThumbnails() {
  Object.keys(thumbCanvases).forEach(camId => {
    const { canvas, ctx } = thumbCanvases[camId];
    drawSceneToCanvas(ctx, canvas.width, canvas.height, camId, false);
    
    // Tally border on thumb
    const el = document.getElementById(`card-${camId}`);
    if (el) {
      el.classList.remove('active-prog', 'active-prev');
      if (camId === state.activeProgScene) el.classList.add('active-prog');
      else if (camId === state.activePrevScene) el.classList.add('active-prev');
    }
  });
}

// ==============================================================================
// MCR TRANSITION LOGIC
// ==============================================================================
function executeCut() {
  const temp = state.activeProgScene;
  state.activeProgScene = state.activePrevScene;
  state.activePrevScene = temp;
  updateSceneLabels();
  broadcastTallyState();
}

function executeAutoFade() {
  let progress = 0;
  const interval = setInterval(() => {
    progress += 0.08;
    state.tBarValue = progress;
    const tbar = document.getElementById('tbar-slider');
    if (tbar) tbar.value = progress * 100;
    
    if (progress >= 1.0) {
      clearInterval(interval);
      executeCut();
      state.tBarValue = 0;
      if (tbar) tbar.value = 0;
    }
  }, 25);
}

function selectScenePreview(sceneId) {
  state.activePrevScene = sceneId;
  updateSceneLabels();
  broadcastTallyState();
}

function updateSceneLabels() {
  const prevLbl = document.getElementById('lbl-prev-scene');
  if (prevLbl) prevLbl.textContent = state.activePrevScene.toUpperCase();
  const progLbl = document.getElementById('lbl-prog-scene');
  if (progLbl) progLbl.textContent = state.activeProgScene.toUpperCase();
}

// ==============================================================================
// REAL-TIME SIGNALING & TALLY BROADCAST HUB (CROSS-TAB & WEBSOCKET LAN SYNC)
// ==============================================================================
const mcrChannel = typeof BroadcastChannel !== 'undefined' ? new BroadcastChannel('widya_broadcast_mcr_hub') : null;

let wsMcr = null;
function initMcrWebSocket() {
  if (typeof location !== 'undefined' && (location.protocol === 'http:' || location.protocol === 'https:')) {
    try {
      const wsProto = location.protocol === 'https:' ? 'wss:' : 'ws:';
      wsMcr = new WebSocket(`${wsProto}//${location.host}/ws`);
      wsMcr.onopen = () => {
        console.log('⚡ Connected to Widya MCR WebSocket Signaling Server');
        broadcastTallyState();
      };
      wsMcr.onmessage = (event) => {
        try {
          const msg = JSON.parse(event.data);
          handleMcrMessage(msg);
        } catch (e) {}
      };
      wsMcr.onclose = () => {
        setTimeout(initMcrWebSocket, 3000);
      };
    } catch (e) {
      console.log('WS fallback to BroadcastChannel');
    }
  }
}
initMcrWebSocket();

function handleMcrMessage(msg) {
  if (!msg) return;
  if (msg.type === 'CAM_TELEMETRY') {
    const telemEl = document.getElementById(`telem-${msg.camId}`);
    if (telemEl) {
      telemEl.textContent = `🔋 ${msg.battery}% | 🧭 ${msg.pitch > 0 ? '+' : ''}${msg.pitch}°`;
      telemEl.style.color = msg.battery < 20 ? '#ef4444' : '#10b981';
    }
  } else if (msg.type === 'REQUEST_INITIAL_TALLY') {
    broadcastTallyState();
  }
}

if (mcrChannel) {
  mcrChannel.onmessage = (event) => {
    handleMcrMessage(event.data);
  };
}

function broadcastTallyState() {
  const payload = {
    type: 'TALLY_UPDATE',
    program: state.activeProgScene,
    preview: state.activePrevScene,
    timestamp: Date.now()
  };
  if (mcrChannel) {
    mcrChannel.postMessage(payload);
  }
  if (wsMcr && wsMcr.readyState === WebSocket.OPEN) {
    wsMcr.send(JSON.stringify(payload));
  }
}

// ==============================================================================
// SCOREBOARD ACTIONS
// ==============================================================================
function addScore(corner, points) {
  if (corner === 'merah') {
    state.match.merah.poin = Math.max(0, state.match.merah.poin + points);
  } else {
    state.match.biru.poin = Math.max(0, state.match.biru.poin + points);
  }
  updateScoreboardDisplay();
}

function toggleRoundTimer() {
  state.match.timerRunning = !state.match.timerRunning;
  const btn = document.getElementById('btn-timer-toggle');
  if (btn) btn.textContent = state.match.timerRunning ? 'PAUSE' : 'START';
}

function resetRoundTimer() {
  state.match.timerSeconds = 120;
  updateScoreboardDisplay();
}

// ==============================================================================
// VAR (VIDEO ASSISTANT REFEREE)
// ==============================================================================
function triggerVarChallenge(corner) {
  state.var.active = true;
  state.var.challengeBy = corner;
  state.var.scrubOffset = -15;
  state.var.decision = null;

  const banner = document.getElementById('var-status-banner');
  if (banner) {
    banner.style.display = 'block';
    banner.className = 'ctrl-card';
    banner.style.borderColor = '#ef4444';
    banner.innerHTML = `
      <div style="display:flex; justify-content:space-between; align-items:center;">
        <div>
          <span style="background:#ef4444; color:white; font-size:10px; font-weight:800; padding:2px 6px; border-radius:4px;">VAR REVIEW IN PROGRESS</span>
          <div style="font-size:12px; font-weight:700; margin-top:4px;">Protes oleh: <b>${corner}</b></div>
          <div style="font-size:11px; color:#94a3b8;">Alasan: ${state.var.reason} (-15 frames @ 0.25x)</div>
        </div>
        <button class="btn-var btn-var-deny" onclick="closeVarBanner()">Tutup</button>
      </div>
    `;
  }
}

function scrubVar(val) {
  state.var.scrubOffset = parseInt(val);
  const lbl = document.getElementById('var-scrub-val');
  if (lbl) lbl.textContent = `${state.var.scrubOffset} Frames`;
}

function resolveVar(accepted) {
  if (accepted) {
    state.var.decision = 'PROTES DITERIMA (+3 Poin Sah PERSILAT)';
    addScore(state.var.challengeBy === 'SUDUT_MERAH' ? 'merah' : 'biru', 3);
  } else {
    state.var.decision = 'PROTES DITOLAK (Bantingan Tidak Sah)';
  }
  
  const banner = document.getElementById('var-status-banner');
  if (banner) {
    banner.innerHTML = `
      <div style="color:#10b981; font-weight:800; font-size:13px;">✓ KEPUTUSAN VAR FINAL:</div>
      <div style="font-size:12px; margin-top:2px;">${state.var.decision}</div>
      <button class="btn-var" style="background:#334155; color:white; margin-top:6px;" onclick="closeVarBanner()">Selesai Review</button>
    `;
  }
}

function closeVarBanner() {
  state.var.active = false;
  const banner = document.getElementById('var-status-banner');
  if (banner) banner.style.display = 'none';
}

// ==============================================================================
// BCI & DIALECT SIMULATION
// ==============================================================================
function simulateBciFluctuations() {
  state.bci.merah.focus = Math.min(98, Math.max(65, state.bci.merah.focus + (Math.random() * 6 - 3)));
  state.bci.biru.panic = Math.min(85, Math.max(40, state.bci.biru.panic + (Math.random() * 8 - 4)));

  const barM = document.getElementById('bci-bar-red');
  if (barM) barM.style.width = `${state.bci.merah.focus}%`;
  const valM = document.getElementById('bci-val-red');
  if (valM) valM.textContent = `${state.bci.merah.focus.toFixed(1)}% (FLOW)`;

  const barB = document.getElementById('bci-bar-blue');
  if (barB) barB.style.width = `${state.bci.biru.panic}%`;
  const valB = document.getElementById('bci-val-blue');
  if (valB) valB.textContent = `${state.bci.biru.panic.toFixed(1)}% (STRESS)`;
}

function playDialectAudio(dialect) {
  let phrase = '';
  switch(dialect) {
    case 'JAWA_KRAMA':
      phrase = 'Bantingan kalawau saestu sae sanget, Sudut Abrit pikantuk poin jangkep tiga!';
      break;
    case 'SUNDA':
      phrase = 'Bantingan telak pisan euy! Sudut Beureum kenging tilu poin mutlak ti wasit!';
      break;
    case 'MINANG':
      phrase = 'Gagah bana bantingannyo! Sudut Merah mandapek tigo poin dari dewan juri!';
      break;
    default:
      phrase = 'Bantingan sah sudut merah mendapatkan tiga poin mutlak!';
  }

  // Synthesize speech if browser supports it
  if ('speechSynthesis' in window) {
    const utter = new SpeechSynthesisUtterance(phrase);
    utter.lang = 'id-ID';
    utter.rate = 1.05;
    window.speechSynthesis.speak(utter);
  } else {
    alert(`[${dialect}] ${phrase}`);
  }
}

// ==============================================================================
// SAWERIA TIPS & CELLULAR FAILOVER
// ==============================================================================
function sendSimulatedTip(corner, amount) {
  if (corner === 'MERAH') {
    state.tips.merahIdr += amount;
  } else {
    state.tips.biruIdr += amount;
  }
  
  const elM = document.getElementById('tip-total-red');
  if (elM) elM.textContent = `Rp ${state.tips.merahIdr.toLocaleString('id-ID')}`;
  const elB = document.getElementById('tip-total-blue');
  if (elB) elB.textContent = `Rp ${state.tips.biruIdr.toLocaleString('id-ID')}`;

  // Flash on-air notification
  const notify = document.getElementById('hud-tip-banner');
  if (notify) {
    notify.style.display = 'block';
    notify.textContent = `💰 TIP DITERIMA: Rp ${amount.toLocaleString('id-ID')} untuk SUDUT ${corner}!`;
    setTimeout(() => { notify.style.display = 'none'; }, 4000);
  }
}

function toggleModemHealth(idx) {
  const m = state.bonded.modems[idx];
  m.healthy = !m.healthy;
  
  // Recalculate
  let total = 0;
  state.bonded.modems.forEach(modem => {
    if (modem.healthy) total += modem.bw;
  });
  state.bonded.totalBw = total;

  const btn = document.getElementById(`btn-modem-${idx}`);
  if (btn) {
    btn.style.background = m.healthy ? '#059669' : '#dc2626';
    btn.textContent = m.healthy ? 'AKTIF' : 'PUTUS';
  }

  const bwLabel = document.getElementById('bonded-bw-total');
  if (bwLabel) bwLabel.textContent = `${state.bonded.totalBw} Mbps`;
}

function openQrModal() {
  const modal = document.getElementById('qr-modal');
  if (modal) modal.style.display = 'flex';
}

function closeQrModal() {
  const modal = document.getElementById('qr-modal');
  if (modal) modal.style.display = 'none';
}

// ==============================================================================
// NDI & VISCA PTZ HARDWARE CONTROLLER
// ==============================================================================
let ptzState = { pan: 0, tilt: 0, zoom: 1.0 };

function sendPtz(deltaPan, deltaTilt, deltaZoom) {
  if (deltaPan === 0 && deltaTilt === 0 && deltaZoom === 0) {
    ptzState = { pan: 0, tilt: 0, zoom: 1.0 };
  } else {
    ptzState.pan = Math.max(-170, Math.min(170, ptzState.pan + deltaPan));
    ptzState.tilt = Math.max(-30, Math.min(90, ptzState.tilt + deltaTilt));
    ptzState.zoom = Math.max(1.0, Math.min(30.0, ptzState.zoom + deltaZoom));
  }
  console.log(`[PTZ VISCA] Pan: ${ptzState.pan}°, Tilt: ${ptzState.tilt}°, Zoom: ${ptzState.zoom.toFixed(1)}x`);
  
  // Kirim perintah lewat WebSocket jika aktif
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify({
      type: 'PTZ_COMMAND',
      pan: ptzState.pan,
      tilt: ptzState.tilt,
      zoom: ptzState.zoom,
      timestamp: Date.now()
    }));
  }
}

// ==============================================================================
// AI AUTOMATED HIGHLIGHT REEL & MULTI-FORMAT EXPORTER
// ==============================================================================
const sampleReelClips = [
  { id: 'hl_01', title: 'Bantingan Telak 3 Poin (Sudut Merah)', src: 'CAM_B_CLOSE', inTc: '14:03:12:10', outTc: '14:03:18:40', dur: 6.5, stars: 5 },
  { id: 'hl_02', title: 'Tendangan Poin Beruntun (Gelanggang B)', src: 'CAM_A_WIDE', inTc: '14:05:40:00', outTc: '14:05:45:30', dur: 5.5, stars: 4 },
  { id: 'hl_03', title: 'Jatuhan Mutlak Babak Penentuan (Sudut Biru)', src: 'CAM_B_CLOSE', inTc: '14:08:22:15', outTc: '14:08:30:15', dur: 8.0, stars: 5 }
];

function generateHighlightReelUI() {
  const listEl = document.getElementById('highlight-clips-list');
  if (listEl) {
    listEl.innerHTML = sampleReelClips.map((c) => `
      <div style="background:#0f172a; padding:6px; border-radius:4px; border-left:3px solid #10b981;">
        <b>${c.title}</b><br>
        <small style="color:#64748b;">Kamera: ${c.src} • Durasi: ${c.dur}s • ⭐⭐⭐⭐⭐</small>
      </div>
    `).join('');
  }
  alert('⚡ AI Highlight Reel Berhasil Disusun! Total 3 Klip Emas (20.0 Detik). Siap diekspor ke EDL, FCPXML, atau HLS.');
}

function exportFormatUI(format) {
  const card = document.getElementById('export-preview-card');
  const label = document.getElementById('export-title-label');
  const txt = document.getElementById('export-text-area');
  if (!card || !txt) return;

  card.style.display = 'block';

  if (format === 'EDL') {
    label.textContent = '📄 PRATINJAU CMX 3600 EDIT DECISION LIST (EDL)';
    txt.value = `TITLE: KEJURNAS_PENCAK_SILAT_HIGHLIGHTS_2026\r\nFCM: NON-DROP FRAME\r\n\r\n001  CAM_B_CLOSE  V     C        14:03:12:10 14:03:18:40 00:00:00:00 00:00:06:15\r\n* FROM CLIP: BANTINGAN_TELAK_SUDUT_MERAH\r\n* TRANSITION: STINGER_WIPE_3D\r\n* BGM DUCKING: -14.0dB (epic_brass.wav)\r\n\r\n002  CAM_A_WIDE   V     C        14:05:40:00 14:05:45:30 00:00:06:15 00:00:12:00\r\n* FROM CLIP: TENDANGAN_POIN_BERUNTUN\r\n* TRANSITION: STINGER_WIPE_3D\r\n\r\n003  CAM_B_CLOSE  V     C        14:08:22:15 14:08:30:15 00:00:12:00 00:00:20:00\r\n* FROM CLIP: JATUHAN_MUTLAK_BABAK_PENENTUAN`;
  } else if (format === 'FCPXML') {
    label.textContent = '📄 PRATINJAU FINAL CUT PRO / DAVINCI RESOLVE (FCPXML)';
    txt.value = `<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE xmeml>\n<xmeml version="5">\n  <sequence>\n    <name>Kejurnas Pencak Silat Highlights 2026</name>\n    <duration>1200</duration>\n    <rate><timebase>60</timebase><ntsc>FALSE</ntsc></rate>\n    <media>\n      <video>\n        <track>\n          <clipitem>\n            <name>Bantingan Telak 3 Poin (Sudut Merah)</name>\n            <duration>390</duration>\n            <file id="hl_01"><name>hl_01.mp4</name></file>\n          </clipitem>\n        </track>\n      </video>\n      <audio>\n        <track>\n          <clipitem>\n            <name>epic_brass.wav</name>\n            <filter><effect><name>Audio Ducking</name><parameter><value>-14</value></parameter></effect></filter>\n          </clipitem>\n        </track>\n      </audio>\n    </media>\n  </sequence>\n</xmeml>`;
  } else if (format === 'HLS') {
    label.textContent = '📄 PRATINJAU HLS VOD PLAYLIST (M3U8)';
    txt.value = `#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-TARGETDURATION:10\n#EXT-X-PLAYLIST-TYPE:VOD\n#EXT-X-DISCONTINUITY\n#EXTINF:6.5,Bantingan Telak 3 Poin (Sudut Merah)\nclips/hl_01_slowmo.ts\n#EXT-X-DISCONTINUITY\n#EXTINF:5.5,Tendangan Poin Beruntun (Gelanggang B)\nclips/hl_02_slowmo.ts\n#EXT-X-DISCONTINUITY\n#EXTINF:8.0,Jatuhan Mutlak Babak Penentuan (Sudut Biru)\nclips/hl_03_slowmo.ts\n#EXT-X-ENDLIST`;
  }
}

// ==============================================================================
// PWA SERVICE WORKER REGISTRATION
// ==============================================================================
if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('/sw.js').then((reg) => {
      console.log('[MCR PWA SW] Service worker aktif dengan scope:', reg.scope);
    }).catch((err) => {
      console.warn('[MCR PWA SW] Service worker gagal didaftarkan:', err);
    });
  });
}
