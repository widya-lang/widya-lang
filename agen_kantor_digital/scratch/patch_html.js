const fs = require('fs');
let html = fs.readFileSync('web_3d/index.html', 'utf8');
html = html.replace('<option value="agen_humas">📲 Nadia Safitri (Humas WhatsApp)</option>', '<option value="agen_humas">✈️ Nadia Safitri (Humas Telegram Bot)</option>');
html = html.replace('<option value="agen_humas">📲 Nadia Safitri, S.I.Kom. (Humas & WhatsApp Gateway)</option>', '<option value="agen_humas">✈️ Nadia Safitri, S.I.Kom. (Humas & Telegram Gateway)</option>');
html = html.replace('<button type="button" class="chip-prompt" data-prompt="Siapkan draf pesan WhatsApp untuk Pak Satriyo via Fonnte">📲 Draf WhatsApp</button>', '<button type="button" class="chip-prompt" data-prompt="Siapkan draf broadcast Telegram untuk Pak Satriyo via @WidyaDigitalBot">✈️ Draf Telegram</button>');
fs.writeFileSync('web_3d/index.html', html, 'utf8');
console.log('web_3d/index.html updated successfully!');
