const fs = require('fs');
let code = fs.readFileSync('web_3d/app.js', 'utf8');

// 1. Update agen_humas in AGENTS_DATA
code = code.replace(
  `    peran: 'Humas & Stakeholder Comms',
    gelar: 'Communications Officer • Fonnte Gateway Lead',
    kluster: 'Business & Comms Pod',
    sop: 'SOP-09',
    sopFile: 'SOP-09_Humas_WhatsApp.md',
    avatar: '👩‍💼',
    warna: '#22c55e',
    status: '🟢 Standby Gateway',
    tugas: 'Formatting Pesan WhatsApp & Dispatch ke +6281324000714',`,
  `    peran: 'Humas & Telegram Bot Gateway',
    gelar: 'Communications Officer • Telegram Gateway Lead',
    kluster: 'Business & Comms Pod',
    sop: 'SOP-09',
    sopFile: 'SOP-09_Humas_Telegram.md',
    avatar: '👩‍💼',
    warna: '#0284c7',
    status: '🟢 Standby Telegram Bot',
    tugas: 'Formatting & Dispatch Broadcast via @WidyaDigitalBot',`
);

// 2. Add playTelegramPing to SoundFX and keep playWhatsAppPing as alias
code = code.replace(
  `  // Nada Notifikasi WhatsApp "Ting!"
  playWhatsAppPing() {`,
  `  // Nada Notifikasi Telegram "Chime!"
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
  }`
);

// 3. Update holo-wa-target to holo-tg-target
code = code.replace(
  `document.getElementById('holo-wa-target').textContent = '+6281324000714 (Terkirim via Fonnte)';`,
  `const tgTgt1 = document.getElementById('holo-tg-target') || document.getElementById('holo-wa-target'); if (tgTgt1) tgTgt1.textContent = '@WidyaDigitalBot (Terkirim via Bot API)';`
);
code = code.replace(
  `document.getElementById('holo-wa-target').textContent = '+6281324000714 (Terkirim via Fonnte)';`,
  `const tgTgt2 = document.getElementById('holo-tg-target') || document.getElementById('holo-wa-target'); if (tgTgt2) tgTgt2.textContent = '@WidyaDigitalBot (Terkirim via Bot API)';`
);

code = code.replace(
  `setTimeout(() => sound.playWhatsAppPing(), 1800);`,
  `setTimeout(() => sound.playTelegramPing(), 1800);`
);
code = code.replace(
  `setTimeout(() => sound.playWhatsAppPing(), 1800);`,
  `setTimeout(() => sound.playTelegramPing(), 1800);`
);

// 4. Update modal preview event listeners
const oldModalBlock = `  // ================= MODAL WHATSAPP PREVIEW =================
  document.getElementById('btn-open-whatsapp').addEventListener('click', () => {
    sound.playClick();
    const chatContainer = document.getElementById('wa-chat-container');
    chatContainer.innerHTML = \`
      <div class="wa-bubble">
        🌅 *BRIEFING PAGI KANTOR AI WIDYA*<br>
        📅 *2026-09-26* | ⏰ *08:00 WIB*<br>
        ━━━━━━━━━━━━━━━━━━━━━━<br>
        Halo Komandan! 9 AI Agent telah menyelesaikan Sidang Pagi. Berikut rencana fokus hari ini:<br><br>
        🎯 *3 PRIORITAS UTAMA HARI INI:*<br>
        1. 🔐 *P1:* Implementasi Otentikasi Argon2id & Session Guard<br>
        2. 🗄️ *P2:* Penyusunan Skema Basis Data SQLite & Migrasi Tabel<br>
        3. ⚡ *P3:* Pengujian Beban Jalur Real-Time Server-Sent Events<br><br>
        📋 *TARGET DEADLINE & PIC:*<br>
        • 11:30 WIB — Finalisasi Migrasi DB (\\\`agen_arsitek\\\`)<br>
        • 14:00 WIB — Selesai Endpoint Login & Argon2id (\\\`agen_coder\\\`)<br>
        • 16:30 WIB — Uji Penetrasi & Audit UU PDP (\\\`agen_qa\\\` & \\\`agen_keamanan\\\`)<br>
        • 17:30 WIB — Backup Snapshot Harian Server (\\\`agen_devops\\\`)<br><br>
        📁 *Catatan Lengkap Tersimpan di Obsidian:*<br>
        \\\`C:/Satriyo/Vault/DailyNotes/2026-09-26-Pagi.md\\\`<br><br>
        _Balas chat ini dengan *ACC* untuk setuju, atau ketik revisi prioritas._
        <div class="wa-bubble-time">08:01 &bull; Terkirim ✓✓</div>
      </div>

      <div class="wa-bubble">
        🌙 *REKAP MALAM & EVALUASI HARIAN*<br>
        📅 *2026-09-26* | ⏰ *20:00 WIB*<br>
        ━━━━━━━━━━━━━━━━━━━━━━<br>
        Halo Komandan! Seluruh divisi telah merampungkan jam kerja. Berikut hasil evaluasinya:<br><br>
        📊 *STATUS CAPAIAN PEKERJAAN:*<br>
        ✅ Otentikasi Argon2id & Session Guard [VERIFIED QA]<br>
        ✅ Skema Basis Data SQLite Inventaris [VERIFIED ARCH]<br>
        ✅ Backup Snapshot Harian Basis Data [VERIFIED DEVOPS]<br><br>
        ⚠️ *KENDALA:* \\\`agen_qa\\\` butuh penyesuaian buffer pool streaming SSE.<br><br>
        🔮 *USULAN 3 PRIORITAS BESOK:*<br>
        1. Optimalisasi Buffer Pool SSE<br>
        2. UI Obsidian Glassmorphism<br>
        3. Audit Log Kriptografis HMAC-SHA256<br><br>
        ⭐ Efektivitas Tim: *88.5%*<br>
        📁 Arsip: \\\`C:/Satriyo/Vault/DailyNotes/2026-09-26-Malam.md\\\`
        <div class="wa-bubble-time">20:01 &bull; Terkirim ✓✓</div>
      </div>
    \`;
    document.getElementById('wa-modal').style.display = 'flex';
  });

  document.getElementById('btn-close-wa').addEventListener('click', () => {
    sound.playClick();
    document.getElementById('wa-modal').style.display = 'none';
  });`;

const newModalBlock = `  // ================= MODAL TELEGRAM PREVIEW =================
  const btnOpenTelegram = document.getElementById('btn-open-telegram') || document.getElementById('btn-open-whatsapp');
  if (btnOpenTelegram) {
    btnOpenTelegram.addEventListener('click', () => {
      sound.playClick();
      const chatContainer = document.getElementById('telegram-chat-container') || document.getElementById('wa-chat-container');
      if (chatContainer) {
        chatContainer.innerHTML = \`
          <div class="telegram-bubble">
            ✈️ <b>WIDYA ENTERPRISE LABS • DAILY BRIEFING</b><br>
            📅 <b>2026-09-26</b> | ⏰ <b>08:00 WIB</b> | Channel: <code>@WidyaDigitalBot</code><br>
            ━━━━━━━━━━━━━━━━━━━━━━<br>
            Halo Pak Satriyo! 9 AI Agent Berdaulat telah menyelesaikan Sidang Pagi. Rencana fokus eksekutif hari ini:<br><br>
            🎯 <b>3 PRIORITAS UTAMA HARI INI:</b><br>
            1. 🔐 <b>P1:</b> Implementasi Otentikasi Argon2id & Session Guard<br>
            2. 🗄️ <b>P2:</b> Penyusunan Skema Basis Data WidyaBase WAL & Migrasi Tabel<br>
            3. ⚡ <b>P3:</b> Pengujian Beban Jalur Real-Time Server-Sent Events<br><br>
            📋 <b>TARGET DEADLINE & PIC:</b><br>
            • 11:30 WIB — Finalisasi Migrasi DB (<code>agen_arsitek</code>)<br>
            • 14:00 WIB — Selesai Endpoint Login & Argon2id (<code>agen_coder</code>)<br>
            • 16:30 WIB — Uji Penetrasi & Audit UU PDP (<code>agen_qa</code> & <code>agen_keamanan</code>)<br>
            • 17:30 WIB — Backup Snapshot Harian Server (<code>agen_devops</code>)<br><br>
            📁 <b>Catatan Lengkap Tersimpan di Obsidian:</b><br>
            <code>C:/Satriyo/Vault/DailyNotes/2026-09-26-Pagi.md</code><br><br>
            <i>Balas Telegram ini dengan <b>/acc</b> untuk persetujuan, atau ketik revisi prioritas.</i>
            <div class="telegram-bubble-time">08:01 &bull; Terkirim ✓✓</div>
          </div>

          <div class="telegram-bubble">
            🌙 <b>REKAP EVALUASI HARIAN EKSEKUTIF</b><br>
            📅 <b>2026-09-26</b> | ⏰ <b>20:00 WIB</b> | Bot Gateway<br>
            ━━━━━━━━━━━━━━━━━━━━━━<br>
            Halo Pak Satriyo! Seluruh divisi telah merampungkan jam kerja. Hasil evaluasi terverifikasi:<br><br>
            📊 <b>STATUS CAPAIAN PEKERJAAN:</b><br>
            ✅ Otentikasi Argon2id & Session Guard [VERIFIED QA]<br>
            ✅ Skema Basis Data WidyaBase [VERIFIED ARCH]<br>
            ✅ Backup Snapshot Harian Basis Data [VERIFIED DEVOPS]<br><br>
            ⚠️ <b>CATATAN:</b> <code>agen_qa</code> penyesuaian buffer pool streaming SSE berjalan optimal.<br><br>
            🔮 <b>USULAN 3 PRIORITAS BESOK:</b><br>
            1. Optimalisasi Buffer Pool SSE<br>
            2. UI Obsidian Glassmorphism<br>
            3. Audit Log Kriptografis HMAC-SHA256<br><br>
            ⭐ Efektivitas Tim: <b>94.8%</b><br>
            📁 Arsip Vault: <code>C:/Satriyo/Vault/DailyNotes/2026-09-26-Malam.md</code>
            <div class="telegram-bubble-time">20:01 &bull; Terkirim ✓✓</div>
          </div>
        \`;
      }
      const tgModal = document.getElementById('telegram-modal') || document.getElementById('wa-modal');
      if (tgModal) tgModal.style.display = 'flex';
    });
  }

  const btnCloseTg = document.getElementById('btn-close-tg') || document.getElementById('btn-close-wa');
  if (btnCloseTg) {
    btnCloseTg.addEventListener('click', () => {
      sound.playClick();
      const tgModal = document.getElementById('telegram-modal') || document.getElementById('wa-modal');
      if (tgModal) tgModal.style.display = 'none';
    });
  }`;

code = code.replace(oldModalBlock, newModalBlock);

// 5. Update modal backdrop ID arrays
code = code.replace(
  `['kanban-modal', 'wa-modal', 'sop-modal', 'widyabase-modal', 'modal-add-task', 'modal-add-project']`,
  `['kanban-modal', 'telegram-modal', 'wa-modal', 'sop-modal', 'widyabase-modal', 'modal-add-task', 'modal-add-project']`
);
code = code.replace(
  `['kanban-modal', 'wa-modal', 'sop-modal', 'meeting-hologram', 'widyabase-modal', 'modal-add-task', 'modal-add-project']`,
  `['kanban-modal', 'telegram-modal', 'wa-modal', 'sop-modal', 'meeting-hologram', 'widyabase-modal', 'modal-add-task', 'modal-add-project']`
);

// 6. Update chatter text
code = code.replace(
  `msg: 'Ringkasan eksekutif dan notifikasi WhatsApp berhasil dikirimkan via Fonnte Gateway ke Pak Satriyo.'`,
  `msg: 'Ringkasan eksekutif dan notifikasi resmi berhasil didispatch via Telegram Bot Gateway (@WidyaDigitalBot) ke Pak Satriyo.'`
);

fs.writeFileSync('web_3d/app.js', code, 'utf8');
console.log('web_3d/app.js updated successfully!');
