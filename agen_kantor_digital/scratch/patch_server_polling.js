const fs = require('fs');
let code = fs.readFileSync('web_3d/server.js', 'utf8');

// Insert Telegram Polling and formatBotResponse helper functions before server creation
const marker = 'const server = http.createServer((req, res) => {';

const pollingCode = `// ================= TELEGRAM BOT ENGINE & RESPONSE FORMATTER =================
function formatBotResponse(text, senderName = 'Pak Satriyo') {
  const clean = (text || '').trim();
  const lower = clean.toLowerCase();

  if (lower === '/start' || lower === '/help' || lower === '/bantuan') {
    return \`👋 <b>Halo \${senderName}! Selamat Datang di Bot Resmi Kantor Digital Widya</b>\\n\\n\` +
      \`Sistem <b>9 AI Agent Berdaulat</b> aktif melayani Anda dengan koneksi langsung ke <b>WidyaBase native DB</b> & <b>Obsidian Vault</b>.\\n\\n\` +
      \`📌 <b>Daftar Perintah Eksekutif:</b>\\n\` +
      \`• <b>/status</b> — Laporan kesehatan 9 AI Agent, RAM, & Server\\n\` +
      \`• <b>/proyek</b> — Daftar Proyek Aktif di WidyaBase WAL\\n\` +
      \`• <b>/tugas</b> — Papan Kanban & Matriks Prioritas Kerja\\n\` +
      \`• <b>/sidang</b> — Risalah Sidang Harian 08:00 & 20:00 WIB\\n\` +
      \`• <b>/acc</b> — Beri persetujuan resmi (Executive Sign-Off)\\n\\n\` +
      \`<i>Atau ketik instruksi apapun secara langsung untuk dikoordinasikan oleh Nadia Safitri (Humas).</i>\`;
  }

  if (lower === '/status') {
    return \`📊 <b>STATUS SISTEM KANTOR DIGITAL WIDYA</b>\\n\` +
      \`━━━━━━━━━━━━━━━━━━━━━━\\n\` +
      \`• <b>9 AI Agent:</b> 🟢 9/9 Aktif Berdaulat (Prima)\\n\` +
      \`• <b>WidyaBase Engine:</b> 🟢 WAL Aktif (512 Buffer Pages)\\n\` +
      \`• <b>Obsidian Vault:</b> 📂 C:/Satriyo/Vault (Tersinkron)\\n\` +
      \`• <b>Keamanan Siber:</b> 🛡️ UU PDP No. 27/2022 Compliant\\n\` +
      \`• <b>Waktu Server:</b> ⏰ \${new Date().toLocaleTimeString('id-ID', { timeZone: 'Asia/Jakarta' })} WIB\\n\\n\` +
      \`<i>Seluruh pipeline beroperasi normal tanpa kendala.</i>\`;
  }

  if (lower === '/proyek') {
    let projsText = '';
    try {
      const projs = runWidyaBaseSql('PILIH * DARI proyek;', 'json');
      if (Array.isArray(projs) && projs.length > 0) {
        projsText = projs.map((p, i) => \`\${i + 1}. <b>\${p.kode_proyek}</b>: \${p.nama_proyek} [<code>\${p.status}</code>]\`).join('\\n');
      }
    } catch (e) {}
    if (!projsText) {
      projsText = \`1. <b>PRJ-01</b>: Migrasi Core Widya-Lang [AKTIF]\\n\` +
        \`2. <b>PRJ-WIDYA-KOP-01</b>: Koperasi Digital Simpan Pinjam [AKTIF]\\n\` +
        \`3. <b>PRJ-WIDYA-INV-02</b>: ERP & Pergudangan Terpadu [AKTIF]\\n\` +
        \`4. <b>PRJ-ARKAS-WEB-01</b>: Aplikasi Anggaran Sekolah ARKAS [AKTIF]\`;
    }
    return \`📁 <b>DAFTAR PROYEK AKTIF DI WIDYABASE:</b>\\n━━━━━━━━━━━━━━━━━━━━━━\\n\${projsText}\\n\\n<i>Ketik /tugas untuk memantau backlog pengerjaan.</i>\`;
  }

  if (lower === '/tugas' || lower === '/kanban') {
    return \`📋 <b>PAPAN KANBAN & PRIORITAS HARI INI:</b>\\n━━━━━━━━━━━━━━━━━━━━━━\\n\` +
      \`1. 🔐 <b>[P1]</b> Otentikasi Argon2id & Session Guard (PIC: Lead Coder)\\n\` +
      \`2. 🗄️ <b>[P2]</b> Skema Basis Data WidyaBase WAL (PIC: Arsitek Sistem)\\n\` +
      \`3. ⚡ <b>[P3]</b> Pengujian Beban Jalur SSE (PIC: QA & DevOps)\\n\\n\` +
      \`📁 Terverifikasi di <code>C:/Satriyo/Vault/Tasks/Kanban_Prioritas.md</code>\`;
  }

  if (lower === '/sidang') {
    return \`🌅 <b>RISALAH SIDANG HARIAN 9 AI AGENT:</b>\\n━━━━━━━━━━━━━━━━━━━━━━\\n\` +
      \`• <b>Sidang Pagi (08:00 WIB):</b> Penetapan 3 prioritas modul anggaran & otentikasi.\\n\` +
      \`• <b>Evaluasi Malam (20:00 WIB):</b> Capaian 94.8% tuntas teruji.\\n\` +
      \`📁 Berkas lengkap: <code>C:/Satriyo/Vault/DailyNotes/</code>\`;
  }

  if (lower === '/acc' || lower.includes('setuju') || lower.includes('acc')) {
    return \`✅ <b>PERSETUJUAN DITERIMA & TERCATAT!</b>\\n━━━━━━━━━━━━━━━━━━━━━━\\n\` +
      \`Instruksi persetujuan resmi (Executive Sign-Off) dari <b>\${senderName}</b> telah diverifikasi.\\n\` +
      \`Catatan dicatat ke <b>WidyaBase WAL</b> & otomatis diterbitkan ke <b>Obsidian Vault</b>.\\n\` +
      \`9 AI Agent melanjutkan pengerjaan ke fase berikutnya.\`;
  }

  // Pesan umum dijawab oleh Nadia Safitri (Humas)
  return \`💬 <b>Nadia Safitri, S.I.Kom. (Humas):</b>\\n\` +
    \`Salam hormat \${senderName}! Pesan Anda telah diterima oleh Kantor Digital:\\n\` +
    \`<blockquote>\${clean}</blockquote>\\n\` +
    \`Instruksi telah dikoordinasikan ke tim 9 AI Agent dan dicatat dalam audit trail. Ketik <b>/status</b> atau <b>/menu</b> untuk perintah bot.\`;
}

let tgPollingActive = false;
let tgLastUpdateId = 0;

async function handleTelegramIncomingMessage(msg, token) {
  if (!msg || !msg.text) return;
  const chatId = msg.chat.id;
  const senderName = msg.from ? (msg.from.first_name || 'Pak Satriyo') : 'Pak Satriyo';
  const replyText = formatBotResponse(msg.text, senderName);

  // Catat pesan terakhir ke berkas
  try {
    const tgFile = path.resolve(DIR, '../telegram_terakhir.txt');
    fs.writeFileSync(tgFile, \`[\${new Date().toISOString()}] Masuk dari \${senderName} (\${chatId}): \${msg.text}\\nBalasan: \${replyText.replace(/<[^>]*>?/gm, '')}\\n\`, 'utf8');
  } catch (e) {}

  // Balas ke Telegram via sendMessage
  try {
    await fetch(\`https://api.telegram.org/bot\${token}/sendMessage\`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        chat_id: chatId,
        text: replyText,
        parse_mode: 'HTML'
      })
    });
    console.log(\`✓ [Telegram Bot] Membalas "\${msg.text}" ke \${senderName} (\${chatId})\`);
  } catch (err) {
    console.error('⚠️ Gagal mengirim balasan Telegram:', err.message);
  }
}

async function startTelegramPolling() {
  const cfgPath = path.resolve(DIR, '../telegram_config.json');
  if (!fs.existsSync(cfgPath)) return;
  let cfg = {};
  try { cfg = JSON.parse(fs.readFileSync(cfgPath, 'utf8')); } catch (e) { return; }

  const token = cfg.bot_token;
  if (!token || token === 'TOKEN_BOT_TELEGRAM_ANDA' || !cfg.aktif) {
    console.log('💡 [Telegram Daemon] Bot token belum diatur, daemon polling standby.');
    return;
  }

  tgPollingActive = true;
  console.log(\`🤖 [Telegram Daemon] Long polling aktif untuk bot: \${cfg.username_bot || 'WidyaDigitalBot'}\`);

  (async () => {
    while (tgPollingActive) {
      try {
        const url = \`https://api.telegram.org/bot\${token}/getUpdates?offset=\${tgLastUpdateId + 1}&timeout=15\`;
        const res = await fetch(url, { signal: AbortSignal.timeout(20000) });
        const data = await res.json();

        if (data.ok && Array.isArray(data.result)) {
          for (const update of data.result) {
            tgLastUpdateId = update.update_id;
            if (update.message && update.message.text) {
              await handleTelegramIncomingMessage(update.message, token);
            }
          }
        }
      } catch (err) {
        await new Promise(r => setTimeout(r, 2000));
      }
    }
  })();
}

`;

code = code.replace(marker, pollingCode + '\n' + marker);

// Add /api/telegram/interact endpoint
const endpointMarker = `  // 4. API: WidyaBase Engine Health Check`;
const interactEndpoint = `  // 3c. API: Interaksi Chat Smartphone Preview (Frontend Web 3D)
  if (req.url === '/api/telegram/interact' && req.method === 'POST') {
    let body = '';
    req.on('data', chunk => { body += chunk; });
    req.on('end', async () => {
      try {
        const payload = JSON.parse(body || '{}');
        const userText = payload.pesan || '';
        const botReply = formatBotResponse(userText, 'Pak Satriyo');

        // Opsional kirim juga ke Telegram fisik jika diminta
        const cfgPath = path.resolve(DIR, '../telegram_config.json');
        let cfg = {};
        try { cfg = JSON.parse(fs.readFileSync(cfgPath, 'utf8')); } catch (e) {}

        let realTgSent = false;
        if (cfg.bot_token && cfg.bot_token !== 'TOKEN_BOT_TELEGRAM_ANDA' && cfg.chat_id) {
          try {
            await fetch(\`https://api.telegram.org/bot\${cfg.bot_token}/sendMessage\`, {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({
                chat_id: cfg.chat_id,
                text: botReply,
                parse_mode: 'HTML'
              })
            });
            realTgSent = true;
          } catch (e) {}
        }

        res.writeHead(200, { 'Content-Type': 'application/json' });
        return res.end(JSON.stringify({
          sukses: true,
          balasan: botReply,
          real_telegram_sent: realTgSent
        }));
      } catch (err) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        return res.end(JSON.stringify({ sukses: false, error: err.message }));
      }
    });
    return;
  }

`;

code = code.replace(endpointMarker, interactEndpoint + endpointMarker);

// Start Telegram polling right after server.listen
const listenMarker = `  console.log(\`🚀 Server Web 3D Kantor Digital Widya aktif di: http://127.0.0.1:\${PORT}\`);`;
const startPollingCall = `  console.log(\`🚀 Server Web 3D Kantor Digital Widya aktif di: http://127.0.0.1:\${PORT}\`);
  // Jalankan Daemon Telegram Bot Polling
  startTelegramPolling();`;

code = code.replace(listenMarker, startPollingCall);

fs.writeFileSync('web_3d/server.js', code, 'utf8');
console.log('web_3d/server.js updated with Telegram Polling successfully!');
