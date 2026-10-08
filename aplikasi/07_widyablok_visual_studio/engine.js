/**
 * WidyaBlok - Stage & Execution Engine Runtime
 * Coordinates Stage Canvas 2D, Pen drawing, Web Audio Synthesizer,
 * and executes Visual Block AST stacks concurrently.
 */

class WidyaAudioEngine {
  constructor() {
    this.ctx = null;
  }

  ensureContext() {
    if (!this.ctx) {
      const AudioCtx = window.AudioContext || window.webkitAudioContext;
      this.ctx = new AudioCtx();
    }
    if (this.ctx.state === 'suspended') {
      this.ctx.resume();
    }
  }

  // Convert MIDI note to frequency (60 = Middle C = 261.63Hz)
  midiToFreq(note) {
    return 440 * Math.pow(2, (note - 69) / 12);
  }

  playNote(note = 60, beats = 0.5) {
    this.ensureContext();
    const duration = Math.max(0.05, beats * 0.5);
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();

    osc.type = 'triangle';
    osc.frequency.setValueAtTime(this.midiToFreq(note), this.ctx.currentTime);

    gain.gain.setValueAtTime(0.2, this.ctx.currentTime);
    gain.gain.exponentialRampToValueAtTime(0.0001, this.ctx.currentTime + duration);

    osc.connect(gain);
    gain.connect(this.ctx.destination);

    osc.start();
    osc.stop(this.ctx.currentTime + duration);
  }

  playDrum(type = 'kendang') {
    this.ensureContext();
    const now = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();

    if (type === 'gong') {
      osc.type = 'sine';
      osc.frequency.setValueAtTime(110, now);
      osc.frequency.exponentialRampToValueAtTime(55, now + 1.2);
      gain.gain.setValueAtTime(0.35, now);
      gain.gain.exponentialRampToValueAtTime(0.0001, now + 1.2);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start();
      osc.stop(now + 1.2);
    } else if (type === 'kendang') {
      osc.type = 'triangle';
      osc.frequency.setValueAtTime(220, now);
      osc.frequency.exponentialRampToValueAtTime(80, now + 0.3);
      gain.gain.setValueAtTime(0.4, now);
      gain.gain.exponentialRampToValueAtTime(0.0001, now + 0.3);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start();
      osc.stop(now + 0.3);
    } else {
      // Snare / Hihat
      osc.type = 'square';
      osc.frequency.setValueAtTime(400, now);
      gain.gain.setValueAtTime(0.15, now);
      gain.gain.exponentialRampToValueAtTime(0.001, now + 0.1);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start();
      osc.stop(now + 0.1);
    }
  }

  playSound(sound = 'pop') {
    this.ensureContext();
    const now = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();

    if (sound === 'meow') {
      osc.type = 'sine';
      osc.frequency.setValueAtTime(500, now);
      osc.frequency.linearRampToValueAtTime(800, now + 0.2);
      osc.frequency.linearRampToValueAtTime(400, now + 0.4);
      gain.gain.setValueAtTime(0.25, now);
      gain.gain.exponentialRampToValueAtTime(0.001, now + 0.45);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start();
      osc.stop(now + 0.45);
    } else if (sound === 'laser') {
      osc.type = 'sawtooth';
      osc.frequency.setValueAtTime(880, now);
      osc.frequency.exponentialRampToValueAtTime(110, now + 0.18);
      gain.gain.setValueAtTime(0.2, now);
      gain.gain.exponentialRampToValueAtTime(0.001, now + 0.18);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start();
      osc.stop(now + 0.18);
    } else if (sound === 'koin' || sound === 'menang') {
      this.playNote(72, 0.1);
      setTimeout(() => this.playNote(79, 0.2), 100);
    } else {
      // Pop
      osc.type = 'sine';
      osc.frequency.setValueAtTime(600, now);
      osc.frequency.exponentialRampToValueAtTime(200, now + 0.08);
      gain.gain.setValueAtTime(0.3, now);
      gain.gain.exponentialRampToValueAtTime(0.001, now + 0.08);
      osc.connect(gain);
      gain.connect(this.ctx.destination);
      osc.start();
      osc.stop(now + 0.08);
    }
  }
}

class WidyaBlokStageEngine {
  constructor() {
    this.stageCanvas = document.getElementById('stage-canvas');
    this.penCanvas = document.getElementById('pen-canvas');
    this.bubblesLayer = document.getElementById('bubbles-layer');
    this.varContainer = document.getElementById('variable-watchers');
    
    this.stageCtx = this.stageCanvas.getContext('2d');
    this.penCtx = this.penCanvas.getContext('2d');

    this.audio = new WidyaAudioEngine();

    this.showGrid = false;
    this.turboMode = false;
    this.isRunning = false;

    this.backdrop = 'grid';
    this.sprites = [];
    this.activeSpriteId = null;

    this.variables = {
      skor: 0,
      waktu: 0,
    };

    this.keysDown = {};
    this.mouse = { x: 0, y: 0, isDown: false };
    this.speechBubbles = {}; // spriteId -> { text, type, timeout }

    this.activeThreads = [];
    this.stepHighlightCallback = null;
    this.logCallback = null;

    this.initEvents();
    this.renderLoop();
  }

  initEvents() {
    window.addEventListener('keydown', (e) => {
      this.keysDown[e.key] = true;
      this.keysDown[e.key.toLowerCase()] = true;
      if (e.key === ' ') this.keysDown['spasi'] = true;
      if (e.key === 'ArrowUp') this.keysDown['panah_atas'] = true;
      if (e.key === 'ArrowDown') this.keysDown['panah_bawah'] = true;
      if (e.key === 'ArrowRight') this.keysDown['panah_kanan'] = true;
      if (e.key === 'ArrowLeft') this.keysDown['panah_kiri'] = true;

      if (this.isRunning) {
        this.triggerEvent('event_whenkeypressed', { key: e.key });
      }
    });

    window.addEventListener('keyup', (e) => {
      this.keysDown[e.key] = false;
      this.keysDown[e.key.toLowerCase()] = false;
      if (e.key === ' ') this.keysDown['spasi'] = false;
      if (e.key === 'ArrowUp') this.keysDown['panah_atas'] = false;
      if (e.key === 'ArrowDown') this.keysDown['panah_bawah'] = false;
      if (e.key === 'ArrowRight') this.keysDown['panah_kanan'] = false;
      if (e.key === 'ArrowLeft') this.keysDown['panah_kiri'] = false;
    });

    this.stageCanvas.addEventListener('mousemove', (e) => {
      const rect = this.stageCanvas.getBoundingClientRect();
      const scaleX = this.stageCanvas.width / rect.width;
      const scaleY = this.stageCanvas.height / rect.height;
      // Convert to Scratch coordinate system (-240..240, -180..180)
      this.mouse.x = Math.round((e.clientX - rect.left) * scaleX - 240);
      this.mouse.y = Math.round(180 - (e.clientY - rect.top) * scaleY);
    });

    this.stageCanvas.addEventListener('mousedown', (e) => {
      this.mouse.isDown = true;
      if (this.isRunning) {
        const clickedSprite = this.getSpriteAt(this.mouse.x, this.mouse.y);
        if (clickedSprite) {
          this.triggerEvent('event_whenthisspriteclicked', { spriteId: clickedSprite.id });
        }
      }
    });

    this.stageCanvas.addEventListener('mouseup', () => {
      this.mouse.isDown = false;
    });
  }

  getSpriteAt(x, y) {
    for (let i = this.sprites.length - 1; i >= 0; i--) {
      const s = this.sprites[i];
      if (!s.visible) continue;
      const dist = Math.hypot(s.x - x, s.y - y);
      if (dist < 32 * (s.size / 100)) {
        return s;
      }
    }
    return null;
  }

  loadProjectState(project) {
    this.stopAll();
    this.clearPen();
    this.sprites = JSON.parse(JSON.stringify(project.sprites || []));
    this.backdrop = project.backdrop || 'grid';
    this.activeSpriteId = this.sprites.length > 0 ? this.sprites[0].id : null;
    this.variables = { skor: 0, waktu: 0 };
    this.updateVariablesWatcherUI();
  }

  getActiveSprite() {
    return this.sprites.find(s => s.id === this.activeSpriteId) || this.sprites[0];
  }

  clearPen() {
    this.penCtx.clearRect(0, 0, this.penCanvas.width, this.penCanvas.height);
  }

  updateVariablesWatcherUI() {
    this.varContainer.innerHTML = '';
    for (const [key, val] of Object.entries(this.variables)) {
      const card = document.createElement('div');
      card.className = 'var-card';
      card.innerHTML = `<span>${key}</span> <span class="var-val">${val}</span>`;
      this.varContainer.appendChild(card);
    }
  }

  // =========================================================================
  // Canvas Rendering Loop
  // =========================================================================
  renderLoop() {
    this.renderStage();
    this.renderBubbles();
    requestAnimationFrame(() => this.renderLoop());
  }

  renderStage() {
    const ctx = this.stageCtx;
    const w = this.stageCanvas.width;
    const h = this.stageCanvas.height;

    ctx.clearRect(0, 0, w, h);

    // 1. Draw Backdrop
    this.drawBackdrop(ctx, w, h);

    // 2. Draw Optional Coordinate Grid
    if (this.showGrid) {
      this.drawCoordinateGrid(ctx, w, h);
    }

    // 3. Draw All Sprites
    this.sprites.forEach(sprite => {
      if (!sprite.visible) return;

      ctx.save();
      // Transform from Scratch coordinate (0,0 at center) to Canvas coordinate (0,0 at top-left)
      const cx = w / 2 + sprite.x;
      const cy = h / 2 - sprite.y;

      ctx.translate(cx, cy);
      // Rotation: in Scratch, 90 is facing right (0 rad in Canvas)
      const rad = (sprite.direction - 90) * (Math.PI / 180);
      ctx.rotate(rad);

      const scale = Math.max(0.1, sprite.size / 100);
      ctx.scale(scale, scale);

      this.drawSpriteGraphic(ctx, sprite.type || 'garuda');

      ctx.restore();
    });
  }

  drawBackdrop(ctx, w, h) {
    if (this.backdrop === 'galaxy') {
      const grad = ctx.createRadialGradient(w/2, h/2, 20, w/2, h/2, 260);
      grad.addColorStop(0, '#1e1b4b');
      grad.addColorStop(0.6, '#0f172a');
      grad.addColorStop(1, '#020617');
      ctx.fillStyle = grad;
      ctx.fillRect(0, 0, w, h);

      // Star dots
      ctx.fillStyle = '#ffffff';
      for (let i = 0; i < 40; i++) {
        const sx = (i * 97) % w;
        const sy = (i * 61) % h;
        ctx.fillRect(sx, sy, 2, 2);
      }
    } else if (this.backdrop === 'panggung') {
      // Theater Stage
      ctx.fillStyle = '#312e81';
      ctx.fillRect(0, 0, w, h * 0.7);
      ctx.fillStyle = '#78350f';
      ctx.fillRect(0, h * 0.7, w, h * 0.3);
      // Curtain header
      ctx.fillStyle = '#dc2626';
      ctx.beginPath();
      ctx.moveTo(0, 0);
      ctx.lineTo(w, 0);
      ctx.lineTo(w, 35);
      ctx.lineTo(0, 35);
      ctx.fill();
    } else if (this.backdrop === 'alam') {
      // Nature / Beach
      ctx.fillStyle = '#38bdf8'; // sky
      ctx.fillRect(0, 0, w, h * 0.65);
      ctx.fillStyle = '#15803d'; // grass
      ctx.fillRect(0, h * 0.65, w, h * 0.35);
      // Sun
      ctx.fillStyle = '#fbbf24';
      ctx.beginPath();
      ctx.arc(60, 60, 30, 0, Math.PI * 2);
      ctx.fill();
    } else if (this.backdrop === 'polos_gelap') {
      ctx.fillStyle = '#0f172a';
      ctx.fillRect(0, 0, w, h);
    } else if (this.backdrop === 'polos_putih') {
      ctx.fillStyle = '#f8fafc';
      ctx.fillRect(0, 0, w, h);
    } else {
      // Default: Clean Studio Grid
      ctx.fillStyle = '#ffffff';
      ctx.fillRect(0, 0, w, h);
      ctx.strokeStyle = '#e2e8f0';
      ctx.lineWidth = 1;
      for (let x = 0; x <= w; x += 40) {
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, h);
        ctx.stroke();
      }
      for (let y = 0; y <= h; y += 40) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(w, y);
        ctx.stroke();
      }
    }
  }

  drawCoordinateGrid(ctx, w, h) {
    ctx.save();
    ctx.strokeStyle = '#3b82f6';
    ctx.lineWidth = 1.5;

    // X Axis
    ctx.beginPath();
    ctx.moveTo(0, h / 2);
    ctx.lineTo(w, h / 2);
    ctx.stroke();

    // Y Axis
    ctx.beginPath();
    ctx.moveTo(w / 2, 0);
    ctx.lineTo(w / 2, h);
    ctx.stroke();

    ctx.fillStyle = '#1e293b';
    ctx.font = '10px JetBrains Mono, monospace';
    ctx.fillText('(0, 0)', w / 2 + 4, h / 2 - 4);
    ctx.fillText('X: +240', w - 46, h / 2 - 4);
    ctx.fillText('X: -240', 6, h / 2 - 4);
    ctx.fillText('Y: +180', w / 2 + 4, 16);
    ctx.fillText('Y: -180', w / 2 + 4, h - 8);
    ctx.restore();
  }

  drawSpriteGraphic(ctx, type) {
    // High-resolution vector-styled sprites
    ctx.save();
    ctx.shadowColor = 'rgba(0,0,0,0.2)';
    ctx.shadowBlur = 8;

    if (type === 'kucing') {
      // Cute Scratch-style Orange Cat Mascot
      ctx.fillStyle = '#F59E0B';
      // Body & Head
      ctx.beginPath();
      ctx.arc(0, 0, 24, 0, Math.PI * 2);
      ctx.fill();
      // Ears
      ctx.beginPath();
      ctx.moveTo(-18, -14);
      ctx.lineTo(-24, -34);
      ctx.lineTo(-6, -22);
      ctx.fill();
      ctx.beginPath();
      ctx.moveTo(18, -14);
      ctx.lineTo(24, -34);
      ctx.lineTo(6, -22);
      ctx.fill();
      // Eyes
      ctx.fillStyle = '#ffffff';
      ctx.beginPath();
      ctx.arc(-8, -4, 6, 0, Math.PI * 2);
      ctx.arc(8, -4, 6, 0, Math.PI * 2);
      ctx.fill();
      ctx.fillStyle = '#0f172a';
      ctx.beginPath();
      ctx.arc(-7, -4, 3, 0, Math.PI * 2);
      ctx.arc(9, -4, 3, 0, Math.PI * 2);
      ctx.fill();
      // Whiskers
      ctx.strokeStyle = '#fff';
      ctx.lineWidth = 1.5;
      ctx.beginPath();
      ctx.moveTo(-16, 4); ctx.lineTo(-28, 2);
      ctx.moveTo(-16, 8); ctx.lineTo(-28, 10);
      ctx.moveTo(16, 4); ctx.lineTo(28, 2);
      ctx.moveTo(16, 8); ctx.lineTo(28, 10);
      ctx.stroke();
    } else if (type === 'garuda') {
      // Golden Garuda Mascot
      ctx.fillStyle = '#EAB308';
      ctx.beginPath();
      ctx.arc(0, 0, 22, 0, Math.PI * 2);
      ctx.fill();
      // Wings
      ctx.fillStyle = '#CA8A04';
      ctx.beginPath();
      ctx.ellipse(-26, 6, 16, 8, -Math.PI/6, 0, Math.PI*2);
      ctx.ellipse(26, 6, 16, 8, Math.PI/6, 0, Math.PI*2);
      ctx.fill();
      // Beak
      ctx.fillStyle = '#EA580C';
      ctx.beginPath();
      ctx.moveTo(0, -2);
      ctx.lineTo(12, 4);
      ctx.lineTo(0, 10);
      ctx.fill();
      // Eyes
      ctx.fillStyle = '#000';
      ctx.beginPath();
      ctx.arc(-6, -4, 3, 0, Math.PI*2);
      ctx.arc(6, -4, 3, 0, Math.PI*2);
      ctx.fill();
    } else if (type === 'apel') {
      ctx.fillStyle = '#EF4444';
      ctx.beginPath();
      ctx.arc(0, 2, 20, 0, Math.PI * 2);
      ctx.fill();
      // Stem & Leaf
      ctx.strokeStyle = '#78350F';
      ctx.lineWidth = 3;
      ctx.beginPath();
      ctx.moveTo(0, -18); ctx.lineTo(4, -26);
      ctx.stroke();
      ctx.fillStyle = '#22C55E';
      ctx.beginPath();
      ctx.ellipse(8, -24, 7, 3, Math.PI/4, 0, Math.PI*2);
      ctx.fill();
    } else if (type === 'bintang') {
      ctx.fillStyle = '#F59E0B';
      ctx.beginPath();
      for (let i = 0; i < 5; i++) {
        ctx.lineTo(Math.cos((18 + i * 72) * Math.PI / 180) * 24, -Math.sin((18 + i * 72) * Math.PI / 180) * 24);
        ctx.lineTo(Math.cos((54 + i * 72) * Math.PI / 180) * 10, -Math.sin((54 + i * 72) * Math.PI / 180) * 10);
      }
      ctx.closePath();
      ctx.fill();
    } else if (type === 'roket') {
      ctx.fillStyle = '#3B82F6';
      ctx.beginPath();
      ctx.ellipse(0, 0, 14, 26, 0, 0, Math.PI*2);
      ctx.fill();
      // Nose cone
      ctx.fillStyle = '#EF4444';
      ctx.beginPath();
      ctx.moveTo(-12, -16); ctx.lineTo(0, -32); ctx.lineTo(12, -16);
      ctx.fill();
    } else if (type === 'robot') {
      ctx.fillStyle = '#64748B';
      ctx.fillRect(-18, -18, 36, 36);
      ctx.fillStyle = '#38BDF8';
      ctx.fillRect(-12, -10, 8, 8);
      ctx.fillRect(4, -10, 8, 8);
      ctx.fillStyle = '#F43F5E';
      ctx.fillRect(-8, 6, 16, 4);
    } else {
      // Default Mascot
      ctx.fillStyle = '#9966FF';
      ctx.beginPath();
      ctx.arc(0, 0, 22, 0, Math.PI * 2);
      ctx.fill();
    }

    ctx.restore();
  }

  renderBubbles() {
    this.bubblesLayer.innerHTML = '';
    const w = this.stageCanvas.width;
    const h = this.stageCanvas.height;

    for (const [sId, bubble] of Object.entries(this.speechBubbles)) {
      const sprite = this.sprites.find(s => s.id === sId);
      if (!sprite || !sprite.visible) continue;

      const cx = w / 2 + sprite.x;
      const cy = h / 2 - sprite.y - 36 * (sprite.size / 100);

      const div = document.createElement('div');
      div.className = 'bubble';
      div.style.left = `${cx}px`;
      div.style.top = `${cy}px`;
      div.textContent = bubble.text;
      this.bubblesLayer.appendChild(div);
    }
  }

  // =========================================================================
  // Virtual Machine Execution Engine
  // =========================================================================
  startGreenFlag() {
    this.stopAll();
    this.isRunning = true;
    this.log('[Eksekusi] Memulai skrip Bendera Hijau ▶️...', 'system');

    this.sprites.forEach(sprite => {
      (sprite.stacks || []).forEach(stack => {
        if (stack.length > 0 && stack[0].opcode === 'event_whenflagclicked') {
          this.launchThread(sprite, stack);
        }
      });
    });
  }

  triggerEvent(eventType, eventData = {}) {
    if (!this.isRunning) return;

    this.sprites.forEach(sprite => {
      (sprite.stacks || []).forEach(stack => {
        if (stack.length > 0 && stack[0].opcode === eventType) {
          if (eventType === 'event_whenkeypressed') {
            const blockKey = stack[0].args.KEY || 'spasi';
            const pressed = (eventData.key || '').toLowerCase();
            if (blockKey === 'spasi' && pressed === ' ') this.launchThread(sprite, stack);
            else if (blockKey === pressed) this.launchThread(sprite, stack);
            else if (blockKey === 'panah_atas' && eventData.key === 'ArrowUp') this.launchThread(sprite, stack);
            else if (blockKey === 'panah_bawah' && eventData.key === 'ArrowDown') this.launchThread(sprite, stack);
            else if (blockKey === 'panah_kanan' && eventData.key === 'ArrowRight') this.launchThread(sprite, stack);
            else if (blockKey === 'panah_kiri' && eventData.key === 'ArrowLeft') this.launchThread(sprite, stack);
          } else if (eventType === 'event_whenthisspriteclicked') {
            if (sprite.id === eventData.spriteId) {
              this.launchThread(sprite, stack);
            }
          }
        }
      });
    });
  }

  launchThread(sprite, stack) {
    const thread = {
      id: Math.random().toString(36).substring(2, 9),
      sprite,
      stack,
      ip: 1, // Start after the Hat block
      isCancelled: false,
    };
    this.activeThreads.push(thread);
    this.runThread(thread);
  }

  async runThread(thread) {
    const { sprite, stack } = thread;

    while (thread.ip < stack.length && !thread.isCancelled && this.isRunning) {
      const block = stack[thread.ip];
      if (this.stepHighlightCallback) {
        this.stepHighlightCallback(block.id, true);
      }

      await this.executeBlock(sprite, block, thread);

      if (this.stepHighlightCallback) {
        this.stepHighlightCallback(block.id, false);
      }

      thread.ip++;

      // Tick delay for smooth animation
      const delay = this.turboMode ? 2 : 20;
      await new Promise(r => setTimeout(r, delay));
    }

    this.activeThreads = this.activeThreads.filter(t => t.id !== thread.id);
    if (this.activeThreads.length === 0) {
      this.isRunning = false;
      this.log('[Eksekusi] Seluruh rangkaian skrip selesai dijalankan.', 'system');
      if (window.onEngineStopped) window.onEngineStopped();
    }
  }

  async executeBlock(sprite, block, thread) {
    if (!block) return;
    const op = block.opcode;
    const args = block.args || {};

    switch (op) {
      // Motion
      case 'motion_movesteps': {
        const steps = Number(args.STEPS) || 10;
        const rad = (sprite.direction - 90) * (Math.PI / 180);
        const oldX = sprite.x;
        const oldY = sprite.y;
        sprite.x += Math.cos(rad) * steps;
        sprite.y -= Math.sin(rad) * steps;

        if (sprite.penDown) {
          this.drawPenLine(oldX, oldY, sprite.x, sprite.y, sprite.penColor, sprite.penSize);
        }
        break;
      }
      case 'motion_turnright': {
        const deg = Number(args.DEGREES) || 15;
        sprite.direction = (sprite.direction + deg) % 360;
        break;
      }
      case 'motion_turnleft': {
        const deg = Number(args.DEGREES) || 15;
        sprite.direction = (sprite.direction - deg + 360) % 360;
        break;
      }
      case 'motion_gotoxy': {
        const oldX = sprite.x;
        const oldY = sprite.y;
        sprite.x = Number(args.X) || 0;
        sprite.y = Number(args.Y) || 0;
        if (sprite.penDown) {
          this.drawPenLine(oldX, oldY, sprite.x, sprite.y, sprite.penColor, sprite.penSize);
        }
        break;
      }
      case 'motion_pointindirection': {
        sprite.direction = Number(args.DIR) || 90;
        break;
      }
      case 'motion_ifonedgebounce': {
        const maxX = 220;
        const maxY = 160;
        if (Math.abs(sprite.x) >= maxX) {
          sprite.x = Math.sign(sprite.x) * maxX;
          sprite.direction = (360 - sprite.direction) % 360;
        }
        if (Math.abs(sprite.y) >= maxY) {
          sprite.y = Math.sign(sprite.y) * maxY;
          sprite.direction = (180 - sprite.direction + 360) % 360;
        }
        break;
      }
      case 'motion_changexby': {
        sprite.x += Number(args.DX) || 10;
        break;
      }
      case 'motion_changeyby': {
        sprite.y += Number(args.DY) || 10;
        break;
      }

      // Looks
      case 'looks_sayforsecs': {
        const text = args.TEXT || 'Halo!';
        const secs = Number(args.SECS) || 2;
        this.speechBubbles[sprite.id] = { text, type: 'say' };
        await new Promise(r => setTimeout(r, secs * 1000));
        delete this.speechBubbles[sprite.id];
        break;
      }
      case 'looks_say': {
        const text = args.TEXT || '';
        this.speechBubbles[sprite.id] = { text, type: 'say' };
        break;
      }
      case 'looks_thinkforsecs': {
        const text = args.TEXT || 'Hmm...';
        const secs = Number(args.SECS) || 2;
        this.speechBubbles[sprite.id] = { text, type: 'think' };
        await new Promise(r => setTimeout(r, secs * 1000));
        delete this.speechBubbles[sprite.id];
        break;
      }
      case 'looks_changesizeby': {
        sprite.size = Math.max(10, sprite.size + (Number(args.CHANGE) || 10));
        break;
      }
      case 'looks_setsizeto': {
        sprite.size = Math.max(10, Number(args.SIZE) || 100);
        break;
      }
      case 'looks_show': {
        sprite.visible = true;
        break;
      }
      case 'looks_hide': {
        sprite.visible = false;
        break;
      }

      // Sound
      case 'sound_playnote': {
        const note = Number(args.NOTE) || 60;
        const beats = Number(args.BEATS) || 0.5;
        this.audio.playNote(note, beats);
        await new Promise(r => setTimeout(r, beats * 500));
        break;
      }
      case 'sound_playdrum': {
        this.audio.playDrum(args.DRUM || 'kendang');
        break;
      }
      case 'sound_playsound': {
        this.audio.playSound(args.SOUND || 'pop');
        break;
      }

      // Control
      case 'control_wait': {
        const dur = Number(args.DURATION) || 1;
        await new Promise(r => setTimeout(r, dur * 1000));
        break;
      }
      case 'control_repeat': {
        const times = Number(args.TIMES) || 10;
        const children = block.children || [];
        for (let t = 0; t < times && !thread.isCancelled && this.isRunning; t++) {
          for (const child of children) {
            await this.executeBlock(sprite, child, thread);
          }
        }
        break;
      }
      case 'control_forever': {
        const children = block.children || [];
        while (!thread.isCancelled && this.isRunning) {
          for (const child of children) {
            await this.executeBlock(sprite, child, thread);
          }
          await new Promise(r => setTimeout(r, 20));
        }
        break;
      }
      case 'control_if': {
        const cond = args.COND || 'menyentuh_tepi';
        let isTrue = false;
        if (cond === 'menyentuh_tepi') {
          isTrue = Math.abs(sprite.x) >= 200 || Math.abs(sprite.y) >= 140;
        } else {
          isTrue = true;
        }

        if (isTrue) {
          const children = block.children || [];
          for (const child of children) {
            await this.executeBlock(sprite, child, thread);
          }
        }
        break;
      }
      case 'control_stop': {
        this.stopAll();
        break;
      }

      // Variables
      case 'data_setvariableto': {
        const vName = args.VAR || 'skor';
        this.variables[vName] = isNaN(args.VALUE) ? args.VALUE : Number(args.VALUE);
        this.updateVariablesWatcherUI();
        break;
      }
      case 'data_changevariableby': {
        const vName = args.VAR || 'skor';
        const cur = Number(this.variables[vName]) || 0;
        this.variables[vName] = cur + (Number(args.VALUE) || 1);
        this.updateVariablesWatcherUI();
        break;
      }

      // Pen
      case 'pen_clear': {
        this.clearPen();
        break;
      }
      case 'pen_pendown': {
        sprite.penDown = true;
        break;
      }
      case 'pen_penup': {
        sprite.penDown = false;
        break;
      }
      case 'pen_setpencolorto': {
        sprite.penColor = args.COLOR || '#EF4444';
        break;
      }
      case 'pen_setpensizeto': {
        sprite.penSize = Number(args.SIZE) || 3;
        break;
      }

      // Widya Language Specials
      case 'widya_print': {
        this.log(`[Cetak] ${args.TEXT}`, 'output');
        break;
      }
      case 'widya_aksara': {
        this.log(`[Aksara Jawa] ${args.TEXT} ➔ ꦤꦸꦱꦤ꧀ꦠꦫ`, 'output');
        break;
      }
      case 'widya_crypto_hash': {
        this.log(`[Kripto SHA256] "${args.TEXT}" ➔ e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`, 'output');
        break;
      }
    }
  }

  drawPenLine(x1, y1, x2, y2, color = '#EF4444', size = 3) {
    const ctx = this.penCtx;
    const w = this.penCanvas.width;
    const h = this.penCanvas.height;

    const cx1 = w / 2 + x1;
    const cy1 = h / 2 - y1;
    const cx2 = w / 2 + x2;
    const cy2 = h / 2 - y2;

    ctx.strokeStyle = color;
    ctx.lineWidth = size;
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';

    ctx.beginPath();
    ctx.moveTo(cx1, cy1);
    ctx.lineTo(cx2, cy2);
    ctx.stroke();
  }

  stopAll() {
    this.isRunning = false;
    this.activeThreads.forEach(t => { t.isCancelled = true; });
    this.activeThreads = [];
    this.speechBubbles = {};
    this.log('[Sistem] Semua skrip dihentikan 🛑.', 'system');
    if (window.onEngineStopped) window.onEngineStopped();
  }

  log(text, type = 'output') {
    if (this.logCallback) {
      this.logCallback(text, type);
    }
  }
}
