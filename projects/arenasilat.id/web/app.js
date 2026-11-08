/**
 * ============================================================================
 * ARENASILAT.ID - LOGIKA ENGINE RESMI PLATFORM PENCAK SILAT BERDAULAT
 * Versi 1.0.0 (Standard PERSILAT & IPSI 2026 Compliant)
 * ============================================================================
 */

// Web Audio API Synthesizer (Zero External Audio File Dependencies)
let audioCtx = null;
let soundEnabled = true;

function getAudioContext() {
  if (!audioCtx) {
    const AudioContextClass = window.AudioContext || window.webkitAudioContext;
    if (AudioContextClass) {
      audioCtx = new AudioContextClass();
    }
  }
  if (audioCtx && audioCtx.state === 'suspended') {
    audioCtx.resume();
  }
  return audioCtx;
}

/**
 * Sintesis Suara Gong Tradisional Silat (Frekuensi Rendah & Resonansi Panjang)
 */
function playGongSound() {
  if (!soundEnabled) return;
  const ctx = getAudioContext();
  if (!ctx) return;

  const now = ctx.currentTime;
  const osc1 = ctx.createOscillator();
  const osc2 = ctx.createOscillator();
  const osc3 = ctx.createOscillator();
  const gain = ctx.createGain();

  // Frekuensi resonansi Gong Jawa / Silat (85 Hz, 172 Hz, 258 Hz)
  osc1.type = 'sine';
  osc1.frequency.setValueAtTime(85, now);
  osc1.frequency.exponentialRampToValueAtTime(78, now + 3.0);

  osc2.type = 'triangle';
  osc2.frequency.setValueAtTime(172, now);
  osc2.frequency.exponentialRampToValueAtTime(160, now + 2.5);

  osc3.type = 'sine';
  osc3.frequency.setValueAtTime(258, now);
  osc3.frequency.exponentialRampToValueAtTime(245, now + 1.8);

  // Amplop suara (Attack instan, Decay panjang 3.5 detik)
  gain.gain.setValueAtTime(0, now);
  gain.gain.linearRampToValueAtTime(0.8, now + 0.04);
  gain.gain.exponentialRampToValueAtTime(0.001, now + 3.5);

  osc1.connect(gain);
  osc2.connect(gain);
  osc3.connect(gain);
  gain.connect(ctx.destination);

  osc1.start(now);
  osc2.start(now);
  osc3.start(now);
  osc1.stop(now + 3.6);
  osc2.stop(now + 3.6);
  osc3.stop(now + 3.6);
}

/**
 * Sintesis Suara Peluit Wasit Matras (Frekuensi Tinggi Dual-Tone)
 */
function playWhistleSound() {
  if (!soundEnabled) return;
  const ctx = getAudioContext();
  if (!ctx) return;

  const now = ctx.currentTime;
  const osc1 = ctx.createOscillator();
  const osc2 = ctx.createOscillator();
  const gain = ctx.createGain();

  // Dual tone melengking peluit Fox 40 (2400 Hz & 2520 Hz)
  osc1.type = 'sine';
  osc1.frequency.setValueAtTime(2400, now);
  osc2.type = 'triangle';
  osc2.frequency.setValueAtTime(2520, now);

  gain.gain.setValueAtTime(0, now);
  gain.gain.linearRampToValueAtTime(0.5, now + 0.02);
  gain.gain.setValueAtTime(0.5, now + 0.35);
  gain.gain.exponentialRampToValueAtTime(0.001, now + 0.5);

  osc1.connect(gain);
  osc2.connect(gain);
  gain.connect(ctx.destination);

  osc1.start(now);
  osc2.start(now);
  osc1.stop(now + 0.52);
  osc2.stop(now + 0.52);
}

/**
 * Chime Singkat untuk Sentuhan Skor
 */
function playScoreChime(corner) {
  if (!soundEnabled) return;
  const ctx = getAudioContext();
  if (!ctx) return;

  const now = ctx.currentTime;
  const osc = ctx.createOscillator();
  const gain = ctx.createGain();

  const freq = corner === 'red' ? 523.25 : 659.25; // C5 untuk Merah, E5 untuk Biru
  osc.type = 'sine';
  osc.frequency.setValueAtTime(freq, now);

  gain.gain.setValueAtTime(0.25, now);
  gain.gain.exponentialRampToValueAtTime(0.001, now + 0.25);

  osc.connect(gain);
  gain.connect(ctx.destination);

  osc.start(now);
  osc.stop(now + 0.26);
}

function toggleSound() {
  soundEnabled = !soundEnabled;
  const btn = document.getElementById('btnSoundToggle');
  const icon = document.getElementById('soundIcon');
  if (soundEnabled) {
    btn.classList.add('active');
    icon.textContent = '🔊';
  } else {
    btn.classList.remove('active');
    icon.textContent = '🔇';
  }
}

// ============================================================================
// DATA STATE 4 GELANGGANG MULTI-COURT
// ============================================================================
const courtsState = [
  {
    id: 1,
    name: "Gelanggang 1 (Utama)",
    partai: "#14",
    category: "Tanding Dewasa Putra Kelas C (55-60 Kg)",
    round: 1,
    timeLeft: 120,
    isTimerRunning: false,
    red: {
      name: "Arya Wira Sena",
      school: "Tapak Suci Putera Muhammadiyah",
      contingent: "DKI JAKARTA",
      avatar: "🥋",
      score: 0,
      pukulan: 0,
      tendangan: 0,
      bantingan: 0,
      hukuman: 0,
      varCards: 2
    },
    blue: {
      name: "Bagas Danuarta",
      school: "PPS Betako Merpati Putih",
      contingent: "JAWA BARAT",
      avatar: "🥋",
      score: 0,
      pukulan: 0,
      tendangan: 0,
      bantingan: 0,
      hukuman: 0,
      varCards: 2
    },
    juryVotes: {
      red: [false, false, false, false, false],
      blue: [false, false, false, false, false]
    }
  },
  {
    id: 2,
    name: "Gelanggang 2",
    partai: "#11",
    category: "Tanding Dewasa Putra Kelas D (60-65 Kg)",
    round: 2,
    timeLeft: 85,
    isTimerRunning: false,
    red: {
      name: "Reza Pratama",
      school: "Persaudaraan Setia Hati Terate",
      contingent: "JAWA TIMUR",
      avatar: "🥊",
      score: 6,
      pukulan: 2,
      tendangan: 2,
      bantingan: 0,
      hukuman: 0,
      varCards: 2
    },
    blue: {
      name: "I Gusti Ngurah Agung",
      school: "Perisai Diri",
      contingent: "BALI",
      avatar: "🥊",
      score: 8,
      pukulan: 2,
      tendangan: 0,
      bantingan: 2,
      hukuman: 0,
      varCards: 1
    },
    juryVotes: {
      red: [false, false, false, false, false],
      blue: [false, false, false, false, false]
    }
  },
  {
    id: 3,
    name: "Gelanggang 3",
    partai: "#08",
    category: "Tanding Remaja Putri Kelas B (47-51 Kg)",
    round: 3,
    timeLeft: 40,
    isTimerRunning: false,
    red: {
      name: "Siti Rahmawati",
      school: "KPS Nusantara",
      contingent: "SUMATERA BARAT",
      avatar: "🥋",
      score: 11,
      pukulan: 3,
      tendangan: 4,
      bantingan: 0,
      hukuman: 0,
      varCards: 2
    },
    blue: {
      name: "Dewi Sekar Kedaton",
      school: "Satria Muda Indonesia",
      contingent: "JAWA TENGAH",
      avatar: "🥋",
      score: 9,
      pukulan: 1,
      tendangan: 4,
      bantingan: 0,
      hukuman: 0,
      varCards: 2
    },
    juryVotes: {
      red: [false, false, false, false, false],
      blue: [false, false, false, false, false]
    }
  },
  {
    id: 4,
    name: "Gelanggang 4",
    partai: "#05",
    category: "Tanding Usia Dini Putra Kelas Bebas",
    round: 1,
    timeLeft: 120,
    isTimerRunning: false,
    red: {
      name: "Muhammad Fatih",
      school: "Tapak Suci",
      contingent: "DI YOGYAKARTA",
      avatar: "🥋",
      score: 3,
      pukulan: 1,
      tendangan: 1,
      bantingan: 0,
      hukuman: 0,
      varCards: 2
    },
    blue: {
      name: "Alvaro Al Ghifari",
      school: "Merpati Putih",
      contingent: "BANTEN",
      avatar: "🥋",
      score: 2,
      pukulan: 0,
      tendangan: 1,
      bantingan: 0,
      hukuman: 0,
      varCards: 2
    },
    juryVotes: {
      red: [false, false, false, false, false],
      blue: [false, false, false, false, false]
    }
  }
];

let activeCourtId = 1;
let activeJuryId = 1;
let timerInterval = null;
let medTimerInterval = null;
let medTimeLeft = 180;
let isMedTimerRunning = false;

function getCurrentCourt() {
  return courtsState.find(c => c.id === activeCourtId) || courtsState[0];
}

// ============================================================================
// NAVIGATION & TABS
// ============================================================================
function switchTab(tabName) {
  const tabs = [
    'scoreboard', 'seni', 'jurypad', 'dewanhakim',
    'multicourt', 'bracket', 'weighin', 'kontingen',
    'athletes', 'certificate', 'medical'
  ];
  tabs.forEach(t => {
    const pane = document.getElementById('tab' + capitalize(t));
    const link = document.getElementById('tabLink' + capitalize(t));
    if (pane) pane.classList.remove('active');
    if (link) link.classList.remove('active');
  });

  const targetPane = document.getElementById('tab' + capitalize(tabName));
  const targetLink = document.getElementById('tabLink' + capitalize(tabName));
  if (targetPane) targetPane.classList.add('active');
  if (targetLink) targetLink.classList.add('active');

  if (tabName === 'multicourt') {
    renderMultiCourtGrid();
  } else if (tabName === 'bracket') {
    renderBracket();
  } else if (tabName === 'athletes') {
    renderAthletesList();
  } else if (tabName === 'weighin') {
    renderMarshallingQueue();
  } else if (tabName === 'seni') {
    renderSeniJuryTable();
  } else if (tabName === 'dewanhakim') {
    renderBlackboxLogs();
  }
}

function capitalize(s) {
  return s.charAt(0).toUpperCase() + s.slice(1);
}

function switchCourt(courtId) {
  activeCourtId = courtId;
  for (let i = 1; i <= 4; i++) {
    const btn = document.getElementById('btnCourt' + i);
    if (btn) {
      if (i === courtId) btn.classList.add('active');
      else btn.classList.remove('active');
    }
  }
  updateScoreboardUI();
}

function toggleObsMode() {
  document.body.classList.toggle('obs-mode-active');
  const btn = document.getElementById('btnObsMode');
  if (document.body.classList.contains('obs-mode-active')) {
    btn.classList.add('active');
    btn.innerHTML = '📺 Keluar Mode OBS';
  } else {
    btn.classList.remove('active');
    btn.innerHTML = '📺 Mode OBS';
  }
}

function toggleFullscreen() {
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen().catch(() => {});
  } else {
    document.exitFullscreen().catch(() => {});
  }
}

// ============================================================================
// SCOREBOARD LOGIC & RENDERING
// ============================================================================
function updateScoreboardUI() {
  const c = getCurrentCourt();
  if (!c) return;

  // Header info
  document.getElementById('matchCourtLabel').textContent = c.name;
  document.getElementById('matchPartaiLabel').textContent = c.partai;
  document.getElementById('matchClassBadge').textContent = c.category;
  document.getElementById('centerCourtTag').textContent = c.name.toUpperCase();
  document.getElementById('centerRoundTitle').textContent = 'BABAK ' + c.round;

  // Round Pills
  for (let r = 1; r <= 3; r++) {
    const pill = document.getElementById('pillRound' + r);
    if (pill) {
      if (r === c.round) pill.classList.add('active');
      else pill.classList.remove('active');
    }
  }

  // Red Corner
  document.getElementById('redFighterName').textContent = c.red.name;
  document.getElementById('redSchoolName').textContent = c.red.school;
  document.getElementById('redContingentTag').textContent = c.red.contingent;
  document.getElementById('redScoreDigits').textContent = c.red.score;
  document.getElementById('redStatPukulan').textContent = c.red.pukulan;
  document.getElementById('redStatTendangan').textContent = c.red.tendangan;
  document.getElementById('redStatBantingan').textContent = c.red.bantingan;
  document.getElementById('redStatHukuman').textContent = c.red.hukuman;
  updateVarCards('red', c.red.varCards);

  // Blue Corner
  document.getElementById('blueFighterName').textContent = c.blue.name;
  document.getElementById('blueSchoolName').textContent = c.blue.school;
  document.getElementById('blueContingentTag').textContent = c.blue.contingent;
  document.getElementById('blueScoreDigits').textContent = c.blue.score;
  document.getElementById('blueStatPukulan').textContent = c.blue.pukulan;
  document.getElementById('blueStatTendangan').textContent = c.blue.tendangan;
  document.getElementById('blueStatBantingan').textContent = c.blue.bantingan;
  document.getElementById('blueStatHukuman').textContent = c.blue.hukuman;
  updateVarCards('blue', c.blue.varCards);

  // Jury Pad Sync Names
  const jpRed = document.getElementById('jpRedFighterName');
  const jpBlue = document.getElementById('jpBlueFighterName');
  if (jpRed) jpRed.textContent = c.red.name;
  if (jpBlue) jpBlue.textContent = c.blue.name;

  // Certificate Sync
  const certName = document.getElementById('certWinnerName');
  if (certName) certName.textContent = c.red.score >= c.blue.score ? c.red.name : c.blue.name;

  // Update Timer
  renderTimerDisplay();
  renderJuryDots();
}

function updateVarCards(corner, count) {
  const card1 = document.getElementById(corner + 'VarCard1');
  const card2 = document.getElementById(corner + 'VarCard2');
  if (card1 && card2) {
    if (count >= 2) {
      card1.className = 'var-card-tag active';
      card2.className = 'var-card-tag active';
    } else if (count === 1) {
      card1.className = 'var-card-tag active';
      card2.className = 'var-card-tag spent';
    } else {
      card1.className = 'var-card-tag spent';
      card2.className = 'var-card-tag spent';
    }
  }
}

function selectRound(roundNum) {
  const c = getCurrentCourt();
  c.round = roundNum;
  c.timeLeft = 120;
  pauseTimer();
  updateScoreboardUI();
  playGongSound();
}

function addScore(corner, technique) {
  const c = getCurrentCourt();
  const fighter = corner === 'red' ? c.red : c.blue;
  let points = 1;

  if (technique === 'pukulan') {
    points = 1;
    fighter.pukulan += 1;
  } else if (technique === 'tendangan') {
    points = 2;
    fighter.tendangan += 1;
  } else if (technique === 'bantingan') {
    points = 3;
    fighter.bantingan += 1;
  }

  fighter.score += points;
  playScoreChime(corner);

  // Trigger score pop animation
  const digitsEl = document.getElementById(corner + 'ScoreDigits');
  if (digitsEl) {
    digitsEl.classList.add('pop');
    setTimeout(() => digitsEl.classList.remove('pop'), 200);
  }

  updateScoreboardUI();
}

function addPenalty(corner, points) {
  const c = getCurrentCourt();
  const fighter = corner === 'red' ? c.red : c.blue;

  if (points > 0) {
    fighter.hukuman -= points;
    fighter.score -= points;
    playWhistleSound();
  } else {
    // Binaan
    playWhistleSound();
  }
  updateScoreboardUI();
}

function undoScore(corner) {
  const c = getCurrentCourt();
  const fighter = corner === 'red' ? c.red : c.blue;
  if (fighter.score > 0) {
    fighter.score -= 1;
  }
  updateScoreboardUI();
}

// ============================================================================
// TIMER ENGINE
// ============================================================================
function renderTimerDisplay() {
  const c = getCurrentCourt();
  const minutes = Math.floor(c.timeLeft / 60);
  const seconds = c.timeLeft % 60;
  const timeStr = `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
  
  const digitsEl = document.getElementById('timerDigits');
  if (digitsEl) digitsEl.textContent = timeStr;

  const btnText = document.getElementById('timerBtnText');
  const playIcon = document.getElementById('timerPlayIcon');
  const statusLabel = document.getElementById('timerStatusLabel');
  const primaryBtn = document.getElementById('btnTimerPrimary');

  if (c.isTimerRunning) {
    if (btnText) btnText.textContent = 'Jeda Laga';
    if (playIcon) playIcon.textContent = '⏸';
    if (statusLabel) statusLabel.textContent = 'LAGA BERJALAN';
  } else {
    if (btnText) btnText.textContent = 'Mulai Babak';
    if (playIcon) playIcon.textContent = '▶';
    if (statusLabel) statusLabel.textContent = c.timeLeft === 0 ? 'BABAK SELESAI' : 'JEDA WAKTU';
  }
}

function toggleTimer() {
  const c = getCurrentCourt();
  if (c.isTimerRunning) {
    pauseTimer();
  } else {
    startTimer();
  }
}

function startTimer() {
  const c = getCurrentCourt();
  if (c.timeLeft <= 0) c.timeLeft = 120;
  c.isTimerRunning = true;
  playWhistleSound();

  if (timerInterval) clearInterval(timerInterval);
  timerInterval = setInterval(() => {
    if (c.isTimerRunning && c.timeLeft > 0) {
      c.timeLeft -= 1;
      renderTimerDisplay();
      if (c.timeLeft === 0) {
        pauseTimer();
        playGongSound();
        renderTimerDisplay();
      }
    }
  }, 1000);

  renderTimerDisplay();
}

function pauseTimer() {
  const c = getCurrentCourt();
  c.isTimerRunning = false;
  if (timerInterval) clearInterval(timerInterval);
  renderTimerDisplay();
}

function resetTimer() {
  const c = getCurrentCourt();
  pauseTimer();
  c.timeLeft = 120;
  renderTimerDisplay();
}

function addTime(seconds) {
  const c = getCurrentCourt();
  c.timeLeft += seconds;
  renderTimerDisplay();
}

// ============================================================================
// 5-JURI CONSENSUS SYSTEM & JURY PAD
// ============================================================================
function selectJuryNumber(juryNum) {
  activeJuryId = juryNum;
  for (let j = 1; j <= 5; j++) {
    const btn = document.getElementById('btnSelectJ' + j);
    if (btn) {
      if (j === juryNum) btn.classList.add('active');
      else btn.classList.remove('active');
    }
  }
}

function toggleJuryDot(corner, juryNum) {
  const c = getCurrentCourt();
  c.juryVotes[corner][juryNum - 1] = !c.juryVotes[corner][juryNum - 1];
  checkConsensus(corner);
  renderJuryDots();
}

function juryInputTouch(corner, technique) {
  const c = getCurrentCourt();
  c.juryVotes[corner][activeJuryId - 1] = true;
  renderJuryDots();
  checkConsensus(corner, technique);

  // Auto clear this jury vote after 2s
  setTimeout(() => {
    c.juryVotes[corner][activeJuryId - 1] = false;
    renderJuryDots();
  }, 2000);
}

function checkConsensus(corner, technique = 'pukulan') {
  const c = getCurrentCourt();
  const votes = c.juryVotes[corner];
  const count = votes.filter(v => v === true).length;
  const banner = document.getElementById('consensusBanner');

  if (count >= 3) {
    if (banner) {
      banner.className = 'consensus-banner valid';
      banner.textContent = `⚡ KONSENSUS TERCAPAI (${count}/5 JURI) -> POIN ${corner.toUpperCase()} SAH!`;
    }
    addScore(corner, technique);

    // Auto reset jury lights after valid consensus
    setTimeout(() => {
      c.juryVotes[corner] = [false, false, false, false, false];
      renderJuryDots();
      if (banner) {
        banner.className = 'consensus-banner';
        banner.textContent = '⚖️ Menunggu Masukan Kuorum Juri (Min. 3 dari 5)';
      }
    }, 1200);
  } else {
    if (banner) {
      banner.className = 'consensus-banner';
      banner.textContent = `⚖️ Suara Masuk: ${count}/5 Juri Sepakat (${corner.toUpperCase()})`;
    }
  }
}

function renderJuryDots() {
  const c = getCurrentCourt();
  for (let j = 1; j <= 5; j++) {
    const redDot = document.getElementById('dotRedJ' + j);
    const blueDot = document.getElementById('dotBlueJ' + j);
    if (redDot) {
      if (c.juryVotes.red[j - 1]) redDot.className = 'jury-indicator-dot active-red';
      else redDot.className = 'jury-indicator-dot';
    }
    if (blueDot) {
      if (c.juryVotes.blue[j - 1]) blueDot.className = 'jury-indicator-dot active-blue';
      else blueDot.className = 'jury-indicator-dot';
    }
  }
}

// ============================================================================
// VAR (VIDEO ASSISTANT REFEREE) MODAL & CANVAS SIMULATION
// ============================================================================
let varAnimationId = null;
let varSpeed = 0.5;
let varApplicant = 'MERAH';

function openVarModal(corner) {
  varApplicant = corner;
  const modal = document.getElementById('varModal');
  const applicantEl = document.getElementById('varCornerApplicant');
  if (applicantEl) applicantEl.textContent = 'SUDUT ' + corner;
  if (modal) modal.classList.add('open');

  pauseTimer();
  startVarCanvasAnimation();
}

function closeVarModal() {
  const modal = document.getElementById('varModal');
  if (modal) modal.classList.remove('open');
  if (varAnimationId) cancelAnimationFrame(varAnimationId);
}

function setVarSpeed(speed) {
  varSpeed = speed;
}

function toggleVarPlayback() {
  const btn = document.getElementById('btnVarPlay');
  if (varAnimationId) {
    cancelAnimationFrame(varAnimationId);
    varAnimationId = null;
    if (btn) btn.textContent = '▶ Putar';
  } else {
    startVarCanvasAnimation();
    if (btn) btn.textContent = '⏸ Jeda';
  }
}

function decideVar(isAccepted) {
  const c = getCurrentCourt();
  const cornerKey = varApplicant === 'MERAH' ? 'red' : 'blue';

  if (isAccepted) {
    addScore(cornerKey, 'bantingan'); // +3 points
    alert(`✅ KEPUTUSAN VAR RESMI:\nProtes DITERIMA oleh Dewan Hakim!\nPoin Bantingan Sah (+3) ditambahkan ke Sudut ${varApplicant}. Kuota kartu tetap utuh.`);
  } else {
    if (c[cornerKey].varCards > 0) {
      c[cornerKey].varCards -= 1;
    }
    alert(`❌ KEPUTUSAN VAR RESMI:\nProtes DITOLAK oleh Dewan Hakim!\n1 Kartu Video Review Sudut ${varApplicant} HANGUS.`);
  }

  closeVarModal();
  updateScoreboardUI();
}

function startVarCanvasAnimation() {
  const canvas = document.getElementById('varVideoCanvas');
  if (!canvas) return;
  const ctx = canvas.getContext('2d');
  canvas.width = canvas.parentElement.clientWidth || 800;
  canvas.height = canvas.parentElement.clientHeight || 420;

  let frame = 0;

  function renderFrame() {
    ctx.fillStyle = '#0a0f1d';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Matras Gelanggang Silat
    const cx = canvas.width / 2;
    const cy = canvas.height / 2 + 40;

    ctx.save();
    ctx.fillStyle = '#1e3a5f';
    ctx.beginPath();
    ctx.ellipse(cx, cy, 260, 120, 0, 0, Math.PI * 2);
    ctx.fill();
    ctx.strokeStyle = '#f59e0b';
    ctx.lineWidth = 3;
    ctx.stroke();

    // Garis Sikap Pasang Merah & Biru
    ctx.strokeStyle = '#ff2a5f';
    ctx.beginPath();
    ctx.moveTo(cx - 100, cy - 20);
    ctx.lineTo(cx - 100, cy + 20);
    ctx.stroke();

    ctx.strokeStyle = '#00e5ff';
    ctx.beginPath();
    ctx.moveTo(cx + 100, cy - 20);
    ctx.lineTo(cx + 100, cy + 20);
    ctx.stroke();

    // Animasi Gerak Pesilat (Bantingan Slow-Mo)
    const animCycle = (frame * varSpeed) % 200;
    const dropY = animCycle > 100 ? Math.min(60, (animCycle - 100) * 1.5) : 0;

    // Pesilat Merah (Penyerang)
    ctx.fillStyle = '#ff2a5f';
    ctx.beginPath();
    ctx.arc(cx - 20, cy - 30, 16, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillRect(cx - 28, cy - 14, 16, 40);

    // Pesilat Biru (Terbanting)
    ctx.fillStyle = '#00e5ff';
    ctx.beginPath();
    ctx.arc(cx + 10 + dropY * 0.5, cy - 30 + dropY, 16, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillRect(cx + 2 + dropY * 0.5, cy - 14 + dropY, 16, 40);

    // OSD HUD Overlay Standar Penyiaran
    ctx.fillStyle = 'rgba(0, 0, 0, 0.6)';
    ctx.fillRect(20, 20, 280, 50);
    ctx.fillStyle = '#38bdf8';
    ctx.font = 'bold 14px "JetBrains Mono", monospace';
    ctx.fillText('REC ● VAR REPLAY 60 FPS', 30, 42);
    ctx.fillStyle = '#94a3b8';
    ctx.font = '11px sans-serif';
    ctx.fillText(`SLOW-MO ${varSpeed}x • KAMERA SUDUT 1 DEPAN`, 30, 58);

    ctx.restore();

    frame += 1;
    varAnimationId = requestAnimationFrame(renderFrame);
  }

  if (varAnimationId) cancelAnimationFrame(varAnimationId);
  varAnimationId = requestAnimationFrame(renderFrame);
}

// ============================================================================
// MULTI-COURT GRID RENDERER
// ============================================================================
function renderMultiCourtGrid() {
  const container = document.getElementById('multiCourtGridContainer');
  if (!container) return;

  container.innerHTML = courtsState.map(c => `
    <div class="court-card-mini ${c.id === activeCourtId ? 'active' : ''}" onclick="switchCourt(${c.id}); switchTab('scoreboard');">
      <div class="mini-court-header">
        <div>
          <h4 style="font-size: 16px; font-weight: 800;">${c.name}</h4>
          <div style="font-size: 11px; color: var(--gold);">${c.category}</div>
        </div>
        <span class="live-badge"><span class="pulse-dot"></span> BABAK ${c.round}</span>
      </div>

      <div class="mini-score-row red">
        <div>
          <div style="font-size: 13px; font-weight: 800;">${c.red.name}</div>
          <div style="font-size: 11px; color: var(--text-dim);">${c.red.contingent}</div>
        </div>
        <div style="font-family: var(--font-mono); font-size: 24px; font-weight: 800; color: var(--red-corner);">${c.red.score}</div>
      </div>

      <div class="mini-score-row blue">
        <div>
          <div style="font-size: 13px; font-weight: 800;">${c.blue.name}</div>
          <div style="font-size: 11px; color: var(--text-dim);">${c.blue.contingent}</div>
        </div>
        <div style="font-family: var(--font-mono); font-size: 24px; font-weight: 800; color: var(--blue-corner);">${c.blue.score}</div>
      </div>

      <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 14px; font-size: 12px; color: var(--text-dim);">
        <span>Partai: ${c.partai}</span>
        <button class="btn-utility" style="padding: 4px 10px; font-size: 11px;">Masuk Gelanggang ➔</button>
      </div>
    </div>
  `).join('');
}

// ============================================================================
// TOURNAMENT TREE BRACKET ENGINE (8 PESERTA ELIMINASI TUNGGAL)
// ============================================================================
const bracketMatches = [
  // Perempat Final
  { id: 'QF1', a: 'Arya Wira Sena (DKI)', b: 'Fajar Nugraha (Jateng)', winner: 'Arya Wira Sena (DKI)' },
  { id: 'QF2', a: 'Dimas Satria (Banten)', b: 'Budi Santoso (Jatim)', winner: 'Dimas Satria (Banten)' },
  { id: 'QF3', a: 'Bagas Danuarta (Jabar)', b: 'Irfan Hakim (DIY)', winner: 'Bagas Danuarta (Jabar)' },
  { id: 'QF4', a: 'Reza Pahlevi (Sumbar)', b: 'Made Arya (Bali)', winner: 'Made Arya (Bali)' },
  // Semi Final
  { id: 'SF1', a: 'Arya Wira Sena (DKI)', b: 'Dimas Satria (Banten)', winner: 'Arya Wira Sena (DKI)' },
  { id: 'SF2', a: 'Bagas Danuarta (Jabar)', b: 'Made Arya (Bali)', winner: 'Bagas Danuarta (Jabar)' },
  // Grand Final
  { id: 'GF', a: 'Arya Wira Sena (DKI)', b: 'Bagas Danuarta (Jabar)', winner: 'Arya Wira Sena (DKI)' }
];

let activeBracketMode = 'prestasi'; // 'prestasi' or 'pemasalan'

// Data Pool Pemasalan (Pool 4 Pesilat Ramah Anak - Semua Raih Medali)
let pemasalanPools = [
  {
    id: 101,
    ageCategory: 'Usia Dini 2 (SD)',
    weightClass: 'Tanding Kelas B Putra (28-30 Kg)',
    semi1: { a: 'Kenzo Alvaro (Jateng)', b: 'Aditya Pratama (DIY)', winner: 'Kenzo Alvaro (Jateng)' },
    semi2: { a: 'Gibran Maulana (Jabar)', b: 'Daniswara (DKI)', winner: 'Daniswara (DKI)' },
    final: { a: 'Kenzo Alvaro (Jateng)', b: 'Daniswara (DKI)', winner: 'Kenzo Alvaro (Jateng)' },
    bronze1: 'Aditya Pratama (DIY)',
    bronze2: 'Gibran Maulana (Jabar)'
  },
  {
    id: 102,
    ageCategory: 'Pra-Remaja (SMP)',
    weightClass: 'Tanding Kelas C Putra (36-39 Kg)',
    semi1: { a: 'Rafa Al-Ghifari (Banten)', b: 'M. Fatih (Sumsel)', winner: 'Rafa Al-Ghifari (Banten)' },
    semi2: { a: 'Bima Sakti (Jatim)', b: 'Satria Dewa (Bali)', winner: 'Bima Sakti (Jatim)' },
    final: { a: 'Rafa Al-Ghifari (Banten)', b: 'Bima Sakti (Jatim)', winner: 'Rafa Al-Ghifari (Banten)' },
    bronze1: 'M. Fatih (Sumsel)',
    bronze2: 'Satria Dewa (Bali)'
  }
];

function setBracketMode(mode) {
  activeBracketMode = mode;
  const btnP = document.getElementById('btnModePrestasi');
  const btnM = document.getElementById('btnModePemasalan');
  const wrapP = document.getElementById('bracketPrestasiWrapper');
  const wrapM = document.getElementById('bracketPemasalanWrapper');

  if (mode === 'prestasi') {
    if (btnP) btnP.classList.add('active');
    if (btnM) btnM.classList.remove('active');
    if (wrapP) wrapP.style.display = 'block';
    if (wrapM) wrapM.style.display = 'none';
    renderBracketTree();
  } else {
    if (btnP) btnP.classList.remove('active');
    if (btnM) btnM.classList.add('active');
    if (wrapP) wrapP.style.display = 'none';
    if (wrapM) wrapM.style.display = 'block';
    renderPoolPemasalan();
  }
}

function renderBracket() {
  if (activeBracketMode === 'prestasi') {
    renderBracketTree();
  } else {
    renderPoolPemasalan();
  }
}

function renderBracketTree() {
  const container = document.getElementById('bracketTreeContainer');
  if (!container) return;

  container.innerHTML = `
    <!-- Perempat Final -->
    <div class="bracket-round-col">
      <div class="bracket-col-title">Perempat Final (8 Besar)</div>
      ${renderBracketCard('QF1', bracketMatches[0])}
      ${renderBracketCard('QF2', bracketMatches[1])}
      ${renderBracketCard('QF3', bracketMatches[2])}
      ${renderBracketCard('QF4', bracketMatches[3])}
    </div>

    <!-- Semi Final -->
    <div class="bracket-round-col">
      <div class="bracket-col-title">Semi Final</div>
      ${renderBracketCard('SF1', bracketMatches[4])}
      ${renderBracketCard('SF2', bracketMatches[5])}
    </div>

    <!-- Grand Final -->
    <div class="bracket-round-col">
      <div class="bracket-col-title">Perebutan Medali Emas (Final)</div>
      ${renderBracketCard('GF', bracketMatches[6], true)}
    </div>
  `;
}

function renderPoolPemasalan() {
  const container = document.getElementById('poolPemasalanGrid');
  if (!container) return;

  container.innerHTML = pemasalanPools.map(pool => `
    <div class="pool-card">
      <div class="pool-header">
        <div>
          <strong style="color: var(--gold); font-size: 15px;">Pool #${pool.id}</strong>
          <span style="font-size: 12px; color: var(--text-muted); margin-left: 6px;">${pool.ageCategory}</span>
        </div>
        <span class="status-tag live" style="font-size: 10px;">4 Pesilat • 100% Medali</span>
      </div>

      <div style="font-size: 12px; color: var(--text-muted); margin-bottom: 10px;">${pool.weightClass}</div>

      <!-- Semifinal 1 -->
      <div class="pool-bracket-match">
        <div style="font-size: 10px; color: var(--text-dim); margin-bottom: 4px;">SEMIFINAL 1 (Klik untuk majukan)</div>
        <div class="pool-fighter-row" onclick="advancePoolFighter(${pool.id}, 'semi1', '${pool.semi1.a}')">
          <span>🔴 ${pool.semi1.a}</span>
          ${pool.semi1.winner === pool.semi1.a ? '<span style="color: var(--emerald); font-weight:800;">✓ Menang</span>' : ''}
        </div>
        <div class="pool-fighter-row" onclick="advancePoolFighter(${pool.id}, 'semi1', '${pool.semi1.b}')">
          <span>🔵 ${pool.semi1.b}</span>
          ${pool.semi1.winner === pool.semi1.b ? '<span style="color: var(--emerald); font-weight:800;">✓ Menang</span>' : ''}
        </div>
      </div>

      <!-- Semifinal 2 -->
      <div class="pool-bracket-match">
        <div style="font-size: 10px; color: var(--text-dim); margin-bottom: 4px;">SEMIFINAL 2 (Klik untuk majukan)</div>
        <div class="pool-fighter-row" onclick="advancePoolFighter(${pool.id}, 'semi2', '${pool.semi2.a}')">
          <span>🔴 ${pool.semi2.a}</span>
          ${pool.semi2.winner === pool.semi2.a ? '<span style="color: var(--emerald); font-weight:800;">✓ Menang</span>' : ''}
        </div>
        <div class="pool-fighter-row" onclick="advancePoolFighter(${pool.id}, 'semi2', '${pool.semi2.b}')">
          <span>🔵 ${pool.semi2.b}</span>
          ${pool.semi2.winner === pool.semi2.b ? '<span style="color: var(--emerald); font-weight:800;">✓ Menang</span>' : ''}
        </div>
      </div>

      <!-- Final Perebutan Emas/Perak -->
      <div class="pool-bracket-match" style="border-color: var(--gold); background: rgba(245, 158, 11, 0.08);">
        <div style="font-size: 10px; color: var(--gold); font-weight: 800; margin-bottom: 4px;">FINAL PEREBUTAN EMAS & PERAK</div>
        <div class="pool-fighter-row" onclick="advancePoolFighter(${pool.id}, 'final', '${pool.final.a}')">
          <span>🥇 ${pool.final.a}</span>
          ${pool.final.winner === pool.final.a ? '<span style="color: var(--gold); font-weight:900;">🏆 EMAS</span>' : ''}
        </div>
        <div class="pool-fighter-row" onclick="advancePoolFighter(${pool.id}, 'final', '${pool.final.b}')">
          <span>🥈 ${pool.final.b}</span>
          ${pool.final.winner === pool.final.b ? '<span style="color: var(--gold); font-weight:900;">🏆 EMAS</span>' : ''}
        </div>
      </div>

      <!-- Podium Medali Bersama -->
      <div class="pool-podium-strip">
        <div class="podium-box">
          <div class="podium-badge" style="color: var(--gold);">🥇 EMAS</div>
          <div class="podium-name">${pool.final.winner}</div>
        </div>
        <div class="podium-box">
          <div class="podium-badge" style="color: #cbd5e1;">🥈 PERAK</div>
          <div class="podium-name">${pool.final.winner === pool.final.a ? pool.final.b : pool.final.a}</div>
        </div>
        <div class="podium-box">
          <div class="podium-badge" style="color: #d97706;">🥉 PERUNGGU</div>
          <div class="podium-name">${pool.bronze1}</div>
        </div>
        <div class="podium-box">
          <div class="podium-badge" style="color: #d97706;">🥉 PERUNGGU</div>
          <div class="podium-name">${pool.bronze2}</div>
        </div>
      </div>
    </div>
  `).join('');
}

function advancePoolFighter(poolId, matchKey, winnerName) {
  const p = pemasalanPools.find(item => item.id === poolId);
  if (!p) return;

  if (matchKey === 'semi1') {
    p.semi1.winner = winnerName;
    p.bronze1 = (winnerName === p.semi1.a) ? p.semi1.b : p.semi1.a;
    p.final.a = winnerName;
  } else if (matchKey === 'semi2') {
    p.semi2.winner = winnerName;
    p.bronze2 = (winnerName === p.semi2.a) ? p.semi2.b : p.semi2.a;
    p.final.b = winnerName;
  } else if (matchKey === 'final') {
    p.final.winner = winnerName;
  }

  renderPoolPemasalan();
  playScoreChime('blue');
}

function exportPuspresnasSimt() {
  const csvRows = [
    ['NISN', 'Nama_Pesilat', 'Kategori', 'Kelas_Tanding', 'Sekolah_NPSN', 'Kontingen_Daerah', 'Prestasi_Juara', 'Nomor_SK_Turnamen', 'Hash_HMAC_SHA256'],
    ['0082910291', 'Arya Wira Sena', 'Tanding Dewasa', 'Kelas C (55-60 Kg)', 'SMA Negeri 8 Jakarta (20101821)', 'DKI Jakarta (Tapak Suci)', 'Juara 1 (Medali Emas)', 'SK-IPSI/2026/089/KEJURNAS', 'a9b8c7d6e5f40192837465aabbccddeeff001122'],
    ['0083918291', 'Bagas Danuarta', 'Tanding Dewasa', 'Kelas C (55-60 Kg)', 'SMA Negeri 3 Bandung (20219281)', 'Jawa Barat (Merpati Putih)', 'Juara 2 (Medali Perak)', 'SK-IPSI/2026/089/KEJURNAS', 'f4e3d2c1b0a98765432100ffeeddccbbaa998877'],
    ['0129840192', 'Kenzo Alvaro', 'Tanding Pemasalan', 'Usia Dini 2 Kelas B', 'SD Negeri Menteng 01 (20108920)', 'Jawa Tengah (Persinas ASAD)', 'Juara 1 Pool 101', 'SK-IPSI/2026/089/KEJURNAS', '11223344556677889900aabbccddeeffaabbccdd'],
    ['0119283819', 'Farhan Al-Fatih', 'Seni Tunggal Baku', 'Tunggal Putra Dewasa', 'SMA Negeri 1 Surabaya (20501289)', 'Jawa Timur (Tapak Suci)', 'Juara 1 (Medali Emas)', 'SK-IPSI/2026/089/KEJURNAS', '77889900aabbccddeeff11223344556677889900']
  ];

  const csvContent = 'data:text/csv;charset=utf-8,' + csvRows.map(e => e.join(',')).join('\n');
  const encodedUri = encodeURI(csvContent);
  const link = document.createElement('a');
  link.setAttribute('href', encodedUri);
  link.setAttribute('download', 'kurasi_simt_puspresnas_arenasilat_2026.csv');
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);

  alert('📥 EKSPOR SIMT PUSPRESNAS BERHASIL!\nBerkas "kurasi_simt_puspresnas_arenasilat_2026.csv" berhasil diunduh. Format ini 100% kompatibel dengan portal SIMT Kemendikdasmen untuk jalur prestasi PPDB.');
}

function renderBracketCard(matchId, match, isGrandFinal = false) {
  return `
    <div class="match-bracket-node" style="${isGrandFinal ? 'border: 2px solid var(--gold); box-shadow: 0 0 25px var(--gold-glow);' : ''}">
      <div style="padding: 6px 12px; background: rgba(0,0,0,0.5); font-size: 10px; color: var(--text-dim); display: flex; justify-content: space-between;">
        <span>Partai ${matchId}</span>
        ${isGrandFinal ? '<span style="color: var(--gold); font-weight: 800;">🥇 EMAS</span>' : ''}
      </div>
      <div class="bracket-slot ${match.winner === match.a ? 'winner' : ''}" onclick="advanceWinner('${matchId}', '${match.a}')">
        <span><span class="slot-seed">🔴</span> ${match.a}</span>
        ${match.winner === match.a ? '<span>✓</span>' : ''}
      </div>
      <div class="bracket-slot ${match.winner === match.b ? 'winner' : ''}" onclick="advanceWinner('${matchId}', '${match.b}')">
        <span><span class="slot-seed">🔵</span> ${match.b}</span>
        ${match.winner === match.b ? '<span>✓</span>' : ''}
      </div>
    </div>
  `;
}

function advanceWinner(matchId, winnerName) {
  const m = bracketMatches.find(item => item.id === matchId);
  if (!m) return;
  m.winner = winnerName;

  if (matchId === 'QF1' || matchId === 'QF2') {
    bracketMatches[4].a = bracketMatches[0].winner;
    bracketMatches[4].b = bracketMatches[1].winner;
  } else if (matchId === 'QF3' || matchId === 'QF4') {
    bracketMatches[5].a = bracketMatches[2].winner;
    bracketMatches[5].b = bracketMatches[3].winner;
  } else if (matchId === 'SF1' || matchId === 'SF2') {
    bracketMatches[6].a = bracketMatches[4].winner;
    bracketMatches[6].b = bracketMatches[5].winner;
  }

  renderBracketTree();
  playScoreChime('red');
}

function resetTournamentBracket() {
  bracketMatches[0].winner = 'Arya Wira Sena (DKI)';
  bracketMatches[1].winner = 'Dimas Satria (Banten)';
  bracketMatches[2].winner = 'Bagas Danuarta (Jabar)';
  bracketMatches[3].winner = 'Made Arya (Bali)';
  bracketMatches[4].winner = 'Arya Wira Sena (DKI)';
  bracketMatches[5].winner = 'Bagas Danuarta (Jabar)';
  bracketMatches[6].winner = 'Arya Wira Sena (DKI)';
  renderBracketTree();
}

// ============================================================================
// TIMBANGAN BADAN BLUETOOTH BLE & MARSHALLING
// ============================================================================
function simulateBleWeighin() {
  const scaleValEl = document.getElementById('scaleWeightVal');
  const badgeEl = document.getElementById('scaleStatusBadge');

  if (scaleValEl) {
    scaleValEl.innerHTML = `Menghubungkan BLE...`;
  }

  setTimeout(() => {
    // Generate random realistic weight between 57.5 and 61.2
    const weight = (57.0 + Math.random() * 3.5).toFixed(1);
    const isPassed = weight >= 55.0 && weight <= 60.0;

    if (scaleValEl) {
      scaleValEl.innerHTML = `${weight} <span style="font-size: 24px;">kg</span>`;
    }

    if (badgeEl) {
      if (isPassed) {
        badgeEl.className = 'scale-status-badge passed';
        badgeEl.textContent = 'LOLOS TIMBANG (KELAS C 55-60 KG)';
      } else {
        badgeEl.className = 'scale-status-badge failed';
        badgeEl.textContent = 'OVERWEIGHT - TIDAK LOLOS TIMBANG';
      }
    }
  }, 800);
}

function confirmWeighin() {
  alert('✅ Data hasil timbang badan berhasil disahkan dan disinkronkan ke meja Marshalling!');
}

const marshallingQueue = [
  { partai: '#14', red: 'Arya Wira Sena', blue: 'Bagas Danuarta', status: 'Di Gelanggang (Live)' },
  { partai: '#15', red: 'Rizki Ramadan', blue: 'Fahrul Rozi', status: 'Siap Masuk (H-1)' },
  { partai: '#16', red: 'Teguh Prakoso', blue: 'Wahyu Hidayat', status: 'Pemeriksaan Berkas (H-2)' },
  { partai: '#17', red: 'Andi Saputra', blue: 'Galang Danendra', status: 'Ruang Timbang (H-3)' }
];

function renderMarshallingQueue() {
  const container = document.getElementById('marshallingListContainer');
  if (!container) return;

  container.innerHTML = marshallingQueue.map(item => `
    <div style="background: rgba(15, 23, 42, 0.7); border: 1px solid var(--border-glass); border-radius: var(--radius-sm); padding: 10px 14px; display: flex; justify-content: space-between; align-items: center;">
      <div>
        <div style="font-size: 13px; font-weight: 800; color: #fff;">Partai ${item.partai}</div>
        <div style="font-size: 12px; color: var(--text-muted); margin-top: 2px;">
          <span style="color: var(--red-corner);">●</span> ${item.red} vs <span style="color: var(--blue-corner);">●</span> ${item.blue}
        </div>
      </div>
      <span class="live-badge" style="background: rgba(56, 189, 248, 0.15); color: #38bdf8; border-color: rgba(56, 189, 248, 0.3);">
        ${item.status}
      </span>
    </div>
  `).join('');
}

// ============================================================================
// DATABASE ATLET & SPORT SCIENCE
// ============================================================================
const athletesDB = [
  { name: "Arya Wira Sena", school: "Tapak Suci", contingent: "DKI Jakarta", class: "Kelas C Putra (55-60kg)", elo: 1845, speed: 92, power: 88, vo2max: 95 },
  { name: "Bagas Danuarta", school: "Merpati Putih", contingent: "Jawa Barat", class: "Kelas C Putra (55-60kg)", elo: 1810, speed: 89, power: 91, vo2max: 90 },
  { name: "Reza Pratama", school: "PSHT", contingent: "Jawa Timur", class: "Kelas D Putra (60-65kg)", elo: 1790, speed: 85, power: 94, vo2max: 88 },
  { name: "I Gusti Ngurah Agung", school: "Perisai Diri", contingent: "Bali", class: "Kelas D Putra (60-65kg)", elo: 1775, speed: 91, power: 86, vo2max: 89 },
  { name: "Siti Rahmawati", school: "KPS Nusantara", contingent: "Sumatera Barat", class: "Kelas B Putri (47-51kg)", elo: 1760, speed: 94, power: 82, vo2max: 92 },
  { name: "Dewi Sekar Kedaton", school: "Satria Muda Indonesia", contingent: "Jawa Tengah", class: "Kelas B Putri (47-51kg)", elo: 1745, speed: 88, power: 85, vo2max: 91 }
];

function renderAthletesList(filterQuery = '') {
  const container = document.getElementById('athleteCardsContainer');
  if (!container) return;

  const filtered = athletesDB.filter(a => 
    a.name.toLowerCase().includes(filterQuery.toLowerCase()) || 
    a.school.toLowerCase().includes(filterQuery.toLowerCase()) ||
    a.contingent.toLowerCase().includes(filterQuery.toLowerCase())
  );

  container.innerHTML = filtered.map(a => `
    <div class="court-card-mini">
      <div style="display: flex; gap: 14px; align-items: center; margin-bottom: 12px;">
        <div class="fighter-avatar" style="width: 52px; height: 52px; font-size: 24px; border-color: var(--gold);">🥋</div>
        <div>
          <h4 style="font-size: 16px; font-weight: 800; color: #fff;">${a.name}</h4>
          <div style="font-size: 12px; color: var(--gold);">${a.school} • ${a.contingent}</div>
          <div style="font-size: 11px; color: var(--text-dim);">${a.class}</div>
        </div>
      </div>

      <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 6px; text-align: center; margin-top: 12px; background: rgba(0,0,0,0.3); padding: 8px; border-radius: var(--radius-sm);">
        <div>
          <div style="font-size: 10px; color: var(--text-dim);">RATING ELO</div>
          <div style="font-family: var(--font-mono); font-size: 14px; font-weight: 800; color: #38bdf8;">${a.elo}</div>
        </div>
        <div>
          <div style="font-size: 10px; color: var(--text-dim);">KECEPATAN</div>
          <div style="font-family: var(--font-mono); font-size: 14px; font-weight: 800; color: #10b981;">${a.speed}%</div>
        </div>
        <div>
          <div style="font-size: 10px; color: var(--text-dim);">VO2MAX</div>
          <div style="font-family: var(--font-mono); font-size: 14px; font-weight: 800; color: #f59e0b;">${a.vo2max}</div>
        </div>
      </div>
    </div>
  `).join('');
}

function filterAthletes() {
  const query = document.getElementById('athleteSearchInput').value;
  renderAthletesList(query);
}

// ============================================================================
// KONSOL MEDIS & PROTOKOL CEDERA
// ============================================================================
function toggleMedTimer() {
  const btn = document.getElementById('btnMedTimerToggle');
  const badge = document.getElementById('medStatusBadge');

  if (isMedTimerRunning) {
    isMedTimerRunning = false;
    clearInterval(medTimerInterval);
    if (btn) btn.textContent = '▶ Lanjutkan Timer Medis';
    if (badge) badge.textContent = 'TIMER MEDIS DIJEDA';
  } else {
    isMedTimerRunning = true;
    if (btn) btn.textContent = '⏸ Jeda Timer Medis';
    if (badge) badge.textContent = 'DOKTER SEDANG MENANGANI DI MATRAS';
    playWhistleSound();

    medTimerInterval = setInterval(() => {
      if (medTimeLeft > 0) {
        medTimeLeft -= 1;
        const minutes = Math.floor(medTimeLeft / 60);
        const seconds = medTimeLeft % 60;
        document.getElementById('medTimerDigits').textContent = 
          `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
        
        if (medTimeLeft === 0) {
          clearInterval(medTimerInterval);
          isMedTimerRunning = false;
          alert('🚨 WAKTU MEDIS 3 MENIT HABIS!\nSesuai Aturan PERSILAT, dokter harus memutuskan apakah laga dihentikan (WMP/TKO Medis).');
        }
      }
    }, 1000);
  }
}

function resetMedTimer() {
  clearInterval(medTimerInterval);
  isMedTimerRunning = false;
  medTimeLeft = 180;
  document.getElementById('medTimerDigits').textContent = '03:00';
  document.getElementById('btnMedTimerToggle').textContent = '▶ Mulai Timer Medis';
  document.getElementById('medStatusBadge').textContent = 'TIM MEDIS SIAGA';
}

function triggerDoctorStoppage() {
  const reason = prompt('Masukkan alasan keputusan Doctor Stoppage (Unfit to Continue):', 'Gegar Otak Ringan / Dislokasi Sendi Bahu');
  if (reason) {
    playWhistleSound();
    alert(`🛑 DOCTOR STOPPAGE DISAHKAN!\nPertandingan dihentikan atas wewenang dokter pertandingan.\nAlasan: ${reason}\nPesilat lawan dinyatakan Menang Medis (WMP). Pesilat cedera dikenakan skorsing medis 30 hari.`);
  }
}

function logInjury(injuryPart) {
  const display = document.getElementById('injuryLogDisplay');
  const time = new Date().toLocaleTimeString('id-ID');
  display.innerHTML = `
    <div style="color: #ef4444; font-weight: 700; margin-bottom: 4px;">⚠️ Insiden Tercatat [${time}]</div>
    <div>Lokasi Cedera: <strong>${injuryPart}</strong></div>
    <div style="font-size: 11px; color: var(--text-dim); margin-top: 4px;">Surat Pengantar Rujukan RSUD & Reklamasi BPJS Ketenagakerjaan dicetak otomatis.</div>
  `;
}

// ============================================================================
// MODUL SENI TGR (TUNGGAL, GANDA, REGU & SOLO KREATIF) PERSILAT 2026
// ============================================================================
let seniTimeLeft = 180; // 3 Menit
let isSeniTimerRunning = false;
let seniTimerInterval = null;
let activeSeniJuryIndex = 1;

let seniJuries = [
  { j: 1, base: 9.90, mistakes: 2, kemantapan: 0.08, penalties: 0, finalScore: 9.96 },
  { j: 2, base: 9.90, mistakes: 1, kemantapan: 0.07, penalties: 0, finalScore: 9.96 },
  { j: 3, base: 9.90, mistakes: 2, kemantapan: 0.08, penalties: 0, finalScore: 9.96 },
  { j: 4, base: 9.90, mistakes: 3, kemantapan: 0.09, penalties: 0, finalScore: 9.96 },
  { j: 5, base: 9.90, mistakes: 0, kemantapan: 0.09, penalties: 0, finalScore: 9.99 }
];

let activeSeniPenalties = {
  senjata: false,
  garis: false,
  pakaian: false,
  suara: false
};

function toggleSeniTimer() {
  const btn = document.getElementById('btnSeniTimerToggle');
  if (isSeniTimerRunning) {
    clearInterval(seniTimerInterval);
    isSeniTimerRunning = false;
    if (btn) btn.textContent = '▶ Lanjutkan Timer';
  } else {
    isSeniTimerRunning = true;
    if (btn) btn.textContent = '⏸ Jeda Timer';
    playGongSound();

    seniTimerInterval = setInterval(() => {
      if (seniTimeLeft > 0) {
        seniTimeLeft -= 1;
        updateSeniTimerDisplay();

        if (seniTimeLeft === 0) {
          clearInterval(seniTimerInterval);
          isSeniTimerRunning = false;
          playGongSound();
          alert('🥋 WAKTU PENAMPILAN SENI 3 MENIT SELESAI!');
        }
      }
    }, 1000);
  }
}

function updateSeniTimerDisplay() {
  const digitsEl = document.getElementById('seniTimerDigits');
  const barEl = document.getElementById('seniTimerBar');
  const badgeEl = document.getElementById('seniTimeStatusBadge');
  if (!digitsEl || !barEl || !badgeEl) return;

  const minutes = Math.floor(seniTimeLeft / 60);
  const seconds = seniTimeLeft % 60;
  digitsEl.textContent = `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;

  // Progress Bar percentage (180s to 0s)
  const pct = (seniTimeLeft / 180) * 100;
  barEl.style.width = `${pct}%`;

  // Waktu tampil berjalan = 180 - seniTimeLeft
  const elapsed = 180 - seniTimeLeft;
  // Zona aman: 175s s.d. 185s
  if (elapsed >= 175 && elapsed <= 185) {
    badgeEl.className = 'scale-status-badge passed';
    badgeEl.textContent = '🟢 ZONA AMAN (02:55 - 03:05)';
    digitsEl.style.color = 'var(--emerald)';
  } else if (elapsed > 185) {
    badgeEl.className = 'scale-status-badge failed';
    badgeEl.textContent = '⚠️ WAKTU LEBIH (>03:05) - PENALTI AKTIF';
    digitsEl.style.color = '#ef4444';
  } else {
    badgeEl.className = 'scale-status-badge';
    badgeEl.style.background = 'rgba(255,255,255,0.1)';
    badgeEl.style.color = '#cbd5e1';
    badgeEl.textContent = 'PENTAS BERJALAN';
    digitsEl.style.color = 'var(--emerald)';
  }
}

function resetSeniTimer() {
  clearInterval(seniTimerInterval);
  isSeniTimerRunning = false;
  seniTimeLeft = 180;
  updateSeniTimerDisplay();
  const btn = document.getElementById('btnSeniTimerToggle');
  if (btn) btn.textContent = '▶ Mulai Timer Seni';
}

function selectSeniJury(jNum) {
  activeSeniJuryIndex = jNum;
  for (let i = 1; i <= 5; i++) {
    const btn = document.getElementById('btnSeniJ' + i);
    if (btn) {
      if (i === jNum) btn.classList.add('active');
      else btn.classList.remove('active');
    }
  }

  const j = seniJuries[activeSeniJuryIndex - 1];
  const countEl = document.getElementById('seniMistakeCount');
  const kebScoreEl = document.getElementById('seniKebenaranScore');
  const kemDispEl = document.getElementById('seniKemantapanDisplay');

  if (countEl) countEl.textContent = j.mistakes;
  if (kebScoreEl) kebScoreEl.textContent = (j.base - (j.mistakes * 0.01)).toFixed(2);
  if (kemDispEl) kemDispEl.textContent = `+${j.kemantapan.toFixed(2)}`;
}

function addSeniMistake() {
  const j = seniJuries[activeSeniJuryIndex - 1];
  j.mistakes += 1;
  selectSeniJury(activeSeniJuryIndex);
}

function undoSeniMistake() {
  const j = seniJuries[activeSeniJuryIndex - 1];
  if (j.mistakes > 0) {
    j.mistakes -= 1;
    selectSeniJury(activeSeniJuryIndex);
  }
}

function updateSeniKemantapan(val) {
  const j = seniJuries[activeSeniJuryIndex - 1];
  j.kemantapan = parseInt(val) / 100;
  selectSeniJury(activeSeniJuryIndex);
}

function toggleSeniPenalty(type, val) {
  activeSeniPenalties[type] = !activeSeniPenalties[type];
  const btnId = {
    senjata: 'btnPenSenjata',
    garis: 'btnPenGaris',
    pakaian: 'btnPenPakaian',
    suara: 'btnPenSuara'
  }[type];

  const btn = document.getElementById(btnId);
  if (btn) {
    if (activeSeniPenalties[type]) {
      btn.style.background = '#dc2626';
      btn.style.borderColor = '#ef4444';
      btn.style.color = '#fff';
    } else {
      btn.style.background = '';
      btn.style.borderColor = '';
      btn.style.color = '';
    }
  }
}

function submitActiveSeniJury() {
  const j = seniJuries[activeSeniJuryIndex - 1];
  let techPenalties = 0;
  if (activeSeniPenalties.senjata) techPenalties += 0.50;
  if (activeSeniPenalties.garis) techPenalties += 0.50;
  if (activeSeniPenalties.pakaian) techPenalties += 0.50;
  if (activeSeniPenalties.suara) techPenalties += 0.50;

  j.penalties = techPenalties;
  const kebenaran = j.base - (j.mistakes * 0.01);
  j.finalScore = (kebenaran - j.penalties) + j.kemantapan;

  renderSeniJuryTable();
  playScoreChime('red');
  alert(`✅ NILAI JURI ${activeSeniJuryIndex} TERSIMPAN!\nKebenaran: ${kebenaran.toFixed(2)} | Kemantapan: +${j.kemantapan.toFixed(2)} | Penalti: -${j.penalties.toFixed(2)}\nSkor Akhir Juri ${activeSeniJuryIndex}: ${j.finalScore.toFixed(3)}`);
}

function renderSeniJuryTable() {
  const tbody = document.getElementById('seniJuryTableBody');
  if (!tbody) return;

  const scores = seniJuries.map(j => j.finalScore);
  const maxScore = Math.max(...scores);
  const minScore = Math.min(...scores);

  let hasDiscardedMax = false;
  let hasDiscardedMin = false;

  tbody.innerHTML = seniJuries.map((j, idx) => {
    let isDiscarded = false;
    let discardReason = '';

    if (j.finalScore === maxScore && !hasDiscardedMax) {
      isDiscarded = true;
      discardReason = 'Dibuang (Tertinggi)';
      hasDiscardedMax = true;
    } else if (j.finalScore === minScore && !hasDiscardedMin) {
      isDiscarded = true;
      discardReason = 'Dibuang (Terendah)';
      hasDiscardedMin = true;
    }

    const kebenaran = (j.base - (j.mistakes * 0.01)).toFixed(2);

    return `
      <tr style="border-bottom: 1px solid var(--border-glass); opacity: ${isDiscarded ? '0.45' : '1.0'};">
        <td style="padding: 10px 8px; font-weight: 800; color: ${idx + 1 === activeSeniJuryIndex ? 'var(--blue-corner)' : '#fff'};">
          Juri ${j.j}
        </td>
        <td style="padding: 10px 8px; font-family: var(--font-mono);">${kebenaran}</td>
        <td style="padding: 10px 8px; font-family: var(--font-mono); color: var(--emerald);">+${j.kemantapan.toFixed(2)}</td>
        <td style="padding: 10px 8px; font-family: var(--font-mono); color: #f87171;">-${j.penalties.toFixed(2)}</td>
        <td style="padding: 10px 8px; font-family: var(--font-mono); font-weight: 900; text-align: right; color: var(--gold); font-size: 15px;">
          ${j.finalScore.toFixed(3)}
        </td>
        <td style="padding: 10px 8px; text-align: center;">
          ${isDiscarded 
            ? `<span style="font-size: 10px; color: #f87171; border: 1px solid rgba(248,113,113,0.3); padding: 2px 6px; border-radius: 4px;">${discardReason}</span>`
            : '<span style="font-size: 10px; color: var(--emerald); border: 1px solid rgba(16,185,129,0.3); padding: 2px 6px; border-radius: 4px;">✓ Dihitung</span>'
          }
        </td>
      </tr>
    `;
  }).join('');

  // Hitung rata-rata 3 juri tengah
  const sorted = [...scores].sort((a, b) => a - b);
  const midScores = sorted.slice(1, 4);
  const median = midScores.reduce((acc, v) => acc + v, 0) / midScores.length;

  const displayOfficial = document.getElementById('seniOfficialResultScore');
  const displayTop = document.getElementById('seniFinalScoreDisplay');
  if (displayOfficial) displayOfficial.textContent = median.toFixed(3);
  if (displayTop) displayTop.textContent = median.toFixed(3);
}

function finalizeSeniMatch() {
  playGongSound();
  alert('🏆 HASIL RESMI SENI DISAHKAN KETUA PERTANDINGAN!\nPartai S-03 Tunggal Putra Dewasa dimenangkan oleh Farhan Al-Fatih (Pengprov Jawa Timur) dengan skor resmi 9.960.\nHasil telah dikirimkan ke bagan juara dan E-Sertifikat.');
}

// ============================================================================
// MODUL DEWAN HAKIM & FORENSIC BLACK-BOX LOGGER
// ============================================================================
let blackboxLogs = [
  { time: '01:24.320', court: 'Gelanggang 1', event: 'QUORUM_TAKEDOWN', actor: 'Juri 1, 2, 3', payload: 'Bantingan Sudut Biru (+3 Poin Sah)', hmac: 'e3b0c44...89a' },
  { time: '01:18.105', court: 'Gelanggang 1', event: 'POINT_KICK', actor: 'Juri 2, 3', payload: 'Tendangan Sudut Merah (+2 Poin Sah)', hmac: '109281a...bc4' },
  { time: '00:45.912', court: 'Gelanggang 1', event: 'POINT_PUNCH', actor: 'Juri 1, 3', payload: 'Pukulan Sudut Merah (+1 Poin Sah)', hmac: '581902f...dd8' },
  { time: '00:30.000', court: 'Gelanggang 1', event: 'ROUND_START', actor: 'Meja KP', payload: 'Babak 2 Dimulai (Gong Dibunyikan)', hmac: '9182390...44a' }
];

function renderBlackboxLogs() {
  const container = document.getElementById('blackboxLogList');
  if (!container) return;

  container.innerHTML = blackboxLogs.map(log => `
    <div style="background: rgba(0,0,0,0.5); border: 1px solid var(--border-glass); border-radius: var(--radius-sm); padding: 10px 14px; font-size: 12px;">
      <div style="display: flex; justify-content: space-between; margin-bottom: 4px;">
        <span style="font-family: var(--font-mono); color: var(--gold); font-weight: 700;">⏱️ ${log.time} • ${log.court}</span>
        <span style="font-size: 10px; color: var(--emerald); font-family: var(--font-mono);">HMAC: ${log.hmac}</span>
      </div>
      <div style="color: #fff; font-weight: 600;">${log.payload}</div>
      <div style="font-size: 11px; color: var(--text-muted); margin-top: 2px;">Aktor: <strong>${log.actor}</strong> [Event: ${log.event}]</div>
    </div>
  `).join('');
}

function refreshBlackboxTimeline() {
  playScoreChime('blue');
  alert('🔄 REPLAY TIMELINE MILIDETIK DIAKTIFKAN!\nData tombol juri disinkronkan ke canvas forensik dengan resolusi 1ms.');
}

function resolveBanding(isAccepted) {
  const noticeEl = document.getElementById('bandingVerdictNotice');
  if (!noticeEl) return;

  if (isAccepted) {
    playWhistleSound();
    noticeEl.style.background = 'rgba(16, 185, 129, 0.2)';
    noticeEl.style.color = 'var(--emerald)';
    noticeEl.innerHTML = '<strong>✅ PUTUSAN DEWAN HAKIM: PROTES DITERIMA!</strong><br>Bantingan disahkan (+3 poin sudut biru). Uang jaminan Rp 3.000.000 dikembalikan utuh ke kontingen.';
  } else {
    playWhistleSound();
    noticeEl.style.background = 'rgba(239, 68, 68, 0.2)';
    noticeEl.style.color = '#ef4444';
    noticeEl.innerHTML = '<strong>❌ PUTUSAN DEWAN HAKIM: PROTES DITOLAK!</strong><br>Hasil skor awal tetap sah. Uang jaminan Rp 3.000.000 HANGUS dan disita ke Kas Federasi IPSI.';
  }
}

function generateBapDocument() {
  alert('📄 BERITA ACARA PERKARA (BAP) RESMI TERCETAK!\nDokumen BAP Sidang Banding dengan Nomor BAP-IPSI/2026/014 telah diterbitkan lengkap dengan Hash Kriptografis HMAC-SHA256 dan stempel digital Dewan Hakim.');
}

// ============================================================================
// MODUL PORTAL KONTINGEN & BILLING XENDIT
// ============================================================================
let isInvoicePaid = false;

function addRegisteredAthlete() {
  const name = document.getElementById('regAthleteName').value.trim();
  const nik = document.getElementById('regAthleteNik').value.trim();
  const nisn = document.getElementById('regAthleteNisn').value.trim();
  const cont = document.getElementById('regContingent').value.trim();
  const cat = document.getElementById('regCategory').value;
  const wClass = document.getElementById('regWeightClass').value;

  if (!name || !cont) {
    alert('Harap isi minimal Nama Pesilat dan Kontingen!');
    return;
  }

  playScoreChime('red');
  alert(`✅ ATLET BERHASIL DIDAFTARKAN!\nNama: ${name}\nKontingen: ${cont}\nKategori: ${cat} - ${wClass}\nStatus: Menunggu Pembayaran Faktur Xendit.`);
  document.getElementById('regAthleteName').value = '';
}

function simulatePayXendit() {
  isInvoicePaid = true;
  const badge = document.getElementById('invoiceStatusBadge');
  const btn = document.getElementById('btnSimulatePayXendit');
  
  if (badge) {
    badge.className = 'scale-status-badge passed';
    badge.textContent = 'LUNAS (PAID VIA QRIS)';
  }
  if (btn) {
    btn.textContent = '✅ Pembayaran Telah Lunas Terverifikasi';
    btn.style.background = '#059669';
  }

  playGongSound();
  alert('⚡ XENDIT WEBHOOK CALLBACK RECEIVED [200 OK]!\nInvoice INV-2026-XENDIT-0891 status PAID.\nSplit Disbursement: Rp 3.325.000 masuk ke Panitia EO (95%), Rp 175.000 ke ArenaSilat SaaS Fee.\nBarcode ID Card 10 atlet kontingen otomatis aktif.');
}

// Inisialisasi Aplikasi Saat Memuat
window.addEventListener('DOMContentLoaded', () => {
  updateScoreboardUI();
  const hash = window.location.hash.replace('#', '');
  if (hash && typeof switchTab === 'function') {
    const validTabs = [
      'scoreboard', 'seni', 'jurypad', 'dewanhakim',
      'multicourt', 'bracket', 'weighin', 'kontingen',
      'athletes', 'certificate', 'medical'
    ];
    if (validTabs.includes(hash)) {
      switchTab(hash);
    }
  }
});

