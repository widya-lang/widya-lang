// ==============================================================================
// app.js - WidyaPay Ultra-Fast Switching Core & EMVCo/ISO 20022 Logic
// ==============================================================================

let currentTps = 52480;
let totalTrx = 18492105;
let trafficHistory = [];
const maxSamples = 60;

// CRC-16 CCITT (0xFFFF) Implementation for EMVCo
function calcCRC16(str) {
  let crc = 0xFFFF;
  for (let i = 0; i < str.length; i++) {
    crc ^= (str.charCodeAt(i) << 8);
    for (let j = 0; j < 8; j++) {
      if ((crc & 0x8000) !== 0) {
        crc = ((crc << 1) ^ 0x1021) & 0xFFFF;
      } else {
        crc = (crc << 1) & 0xFFFF;
      }
    }
  }
  let hex = (crc & 0xFFFF).toString(16).toUpperCase();
  return hex.padStart(4, '0');
}

// FORMAT RUPIAH
function formatRupiah(num) {
  return 'Rp ' + Number(num).toLocaleString('id-ID');
}

document.addEventListener('DOMContentLoaded', () => {
  initChart();
  generateQRIS();
  startTrafficSimulation();
  startRoutingFeed();
});

// TAB SWITCHER
function setTab(tabId) {
  document.querySelectorAll('.tab-pane').forEach(p => p.classList.remove('active'));
  document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));

  const pane = document.getElementById('pane-' + tabId);
  const btn = document.getElementById('tab-btn-' + tabId);
  if (pane) pane.classList.add('active');
  if (btn) btn.classList.add('active');
}

// REAL-TIME TRAFFIC SIMULATION
function startTrafficSimulation() {
  for (let i = 0; i < maxSamples; i++) {
    trafficHistory.push(50000 + Math.random() * 4000);
  }

  setInterval(() => {
    // Jitter around 52k
    const delta = (Math.random() - 0.48) * 1200;
    currentTps = Math.min(58000, Math.max(48000, Math.round(currentTps + delta)));
    totalTrx += Math.round(currentTps / 10);

    trafficHistory.push(currentTps);
    if (trafficHistory.length > maxSamples) trafficHistory.shift();

    document.getElementById('live-tps-num').innerText = currentTps.toLocaleString('id-ID');
    document.getElementById('hdr-tps').innerText = currentTps.toLocaleString('id-ID') + ' TPS';
    document.getElementById('total-trx-today').innerText = totalTrx.toLocaleString('id-ID');

    const pct = Math.min(100, Math.round((currentTps / 60000) * 100));
    document.getElementById('tps-bar').style.width = pct + '%';

    drawCanvas();
  }, 200);
}

// CANVAS DRAW
function initChart() {
  drawCanvas();
}

function drawCanvas() {
  const canvas = document.getElementById('trafficCanvas');
  if (!canvas) return;
  const ctx = canvas.getContext('2d');
  const w = canvas.width;
  const h = canvas.height;

  ctx.clearRect(0, 0, w, h);

  // Grid lines
  ctx.strokeStyle = 'rgba(255, 255, 255, 0.05)';
  ctx.lineWidth = 1;
  for (let y = 0; y < h; y += 30) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(w, y);
    ctx.stroke();
  }

  // Draw Wave
  ctx.beginPath();
  const step = w / (maxSamples - 1);
  const min = 40000;
  const max = 60000;

  trafficHistory.forEach((val, i) => {
    const norm = (val - min) / (max - min);
    const y = h - (norm * (h - 20) + 10);
    const x = i * step;
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  });

  ctx.strokeStyle = '#3b82f6';
  ctx.lineWidth = 2.5;
  ctx.stroke();

  // Gradient under wave
  ctx.lineTo(w, h);
  ctx.lineTo(0, h);
  const grad = ctx.createLinearGradient(0, 0, 0, h);
  grad.addColorStop(0, 'rgba(59, 130, 246, 0.35)');
  grad.addColorStop(1, 'rgba(59, 130, 246, 0.0)');
  ctx.fillStyle = grad;
  ctx.fill();
}

// ROUTING FEED
const BANKS = ['BPD Jateng', 'Bank Mandiri', 'Bank BRI', 'BCA', 'BJB Jabar', 'Bank Jatim', 'Bank DKI', 'BNI'];
const METHODS = ['QRIS Dinamis', 'BI-FAST', 'Card ISO 8583', 'SNAP BI'];

function startRoutingFeed() {
  setInterval(() => {
    const src = BANKS[Math.floor(Math.random() * BANKS.length)];
    let dst = BANKS[Math.floor(Math.random() * BANKS.length)];
    while (dst === src) dst = BANKS[Math.floor(Math.random() * BANKS.length)];

    const m = METHODS[Math.floor(Math.random() * METHODS.length)];
    const amt = Math.floor(10000 + Math.random() * 2500000);
    const lat = (0.15 + Math.random() * 0.2).toFixed(2);

    addFeedItem(src, dst, m, amt, lat);
  }, 600);
}

function addFeedItem(src, dst, method, amt, lat) {
  const container = document.getElementById('routing-feed');
  if (!container) return;

  const now = new Date().toLocaleTimeString('id-ID', { hour12: false }) + '.' + Math.floor(Math.random() * 900 + 100);
  const div = document.createElement('div');
  div.className = 'feed-item ' + (method.includes('QRIS') ? 'qris' : 'bifast');
  div.innerHTML = `
    <div class="feed-top">
      <span>${now} • ${method}</span>
      <span class="text-emerald">${lat} ms</span>
    </div>
    <div class="feed-val">${src} ➔ ${dst}: ${formatRupiah(amt)}</div>
  `;
  container.prepend(div);

  if (container.children.length > 25) {
    container.removeChild(container.lastChild);
  }
}

function clearFeed() {
  const container = document.getElementById('routing-feed');
  if (container) container.innerHTML = '';
}

function simulasiBurstTraffic() {
  currentTps = 59850;
  trafficHistory[trafficHistory.length - 1] = currentTps;
  drawCanvas();
  alert('⚡ Burst Traffic Injected: 59.850 TPS sustained with 0.28ms latency!');
}

// EMVCO QRIS BUILDER
function generateQRIS() {
  const merchant = (document.getElementById('qris-merchant').value || 'MERCHANT').toUpperCase();
  const nmid = document.getElementById('qris-nmid').value || 'ID102026093012345';
  const city = (document.getElementById('qris-city').value || 'JAKARTA').toUpperCase();
  const postal = document.getElementById('qris-postal').value || '10110';
  const amount = Number(document.getElementById('qris-amount').value) || 0;
  const qrisType = document.getElementById('qris-type').value;

  document.getElementById('disp-merchant-name').innerText = merchant;
  document.getElementById('disp-nmid').innerText = 'NMID: ' + nmid;
  document.getElementById('qr-center-val').innerText = amount > 0 ? formatRupiah(amount) : 'STATIS';

  // Sub-tag 26 Merchant Info (Globally Unique Identifier)
  const tag26_00 = '00' + String('ID.CO.WIDYAPAY.WWW').length.toString().padStart(2, '0') + 'ID.CO.WIDYAPAY.WWW';
  const tag26_01 = '01' + String(nmid).length.toString().padStart(2, '0') + nmid;
  const tag26_val = tag26_00 + tag26_01;
  const tag26 = '26' + String(tag26_val.length).padStart(2, '0') + tag26_val;

  let payloadWithoutCRC = 
    '000201' + // Payload Format Indicator
    '0102' + qrisType + // Point of Initiation Method (11: Static, 12: Dynamic)
    tag26 +
    '52045411' + // Merchant Category Code (5411 Grocery)
    '5303360' +  // Transaction Currency 360 (IDR)
    (amount > 0 ? ('54' + String(String(amount).length).padStart(2, '0') + String(amount)) : '') +
    '5802ID' +   // Country Code
    '59' + String(merchant.length).padStart(2, '0') + merchant +
    '60' + String(city.length).padStart(2, '0') + city +
    '61' + String(postal.length).padStart(2, '0') + postal +
    '6304';

  const crc = calcCRC16(payloadWithoutCRC);
  const fullPayload = payloadWithoutCRC + crc;

  document.getElementById('raw-qris-text').value = fullPayload;

  // Render TLV Table
  const tlvTbody = document.getElementById('tlv-tbody');
  tlvTbody.innerHTML = `
    <tr><td>00</td><td>Payload Format Indicator</td><td>02</td><td>01</td></tr>
    <tr><td>01</td><td>Point of Initiation Method</td><td>02</td><td>${qrisType} (${qrisType === '12' ? 'Dinamis' : 'Statis'})</td></tr>
    <tr><td>26</td><td>Merchant Account Info</td><td>${tag26_val.length}</td><td>${tag26_val}</td></tr>
    <tr><td>52</td><td>Merchant Category Code</td><td>04</td><td>5411 (Grocery/Retail)</td></tr>
    <tr><td>53</td><td>Transaction Currency</td><td>03</td><td>360 (Indonesian Rupiah)</td></tr>
    ${amount > 0 ? `<tr><td>54</td><td>Transaction Amount</td><td>${String(amount).length}</td><td>${amount}</td></tr>` : ''}
    <tr><td>58</td><td>Country Code</td><td>02</td><td>ID</td></tr>
    <tr><td>59</td><td>Merchant Name</td><td>${merchant.length}</td><td>${merchant}</td></tr>
    <tr><td>60</td><td>Merchant City</td><td>${city.length}</td><td>${city}</td></tr>
    <tr><td>61</td><td>Postal Code</td><td>${postal.length}</td><td>${postal}</td></tr>
    <tr><td>63</td><td>CRC-16 Checksum</td><td>04</td><td><strong class="text-emerald">${crc}</strong> (Valid)</td></tr>
  `;
}

function copyRawQRIS() {
  const t = document.getElementById('raw-qris-text');
  t.select();
  document.execCommand('copy');
  alert('EMVCo QRIS Payload string berhasil disalin ke clipboard!');
}

// BI-FAST TRANSFER SIMULATOR
function eksekusiBIFastTransfer() {
  const src = document.getElementById('bf-bank-src').value;
  const dst = document.getElementById('bf-bank-dst').value;
  const acc = document.getElementById('bf-dest-acc').value;
  const amt = Number(document.getElementById('bf-amount').value);

  const txId = 'TX-BF-' + Math.floor(100000 + Math.random() * 900000);
  const now = new Date().toISOString();

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>WIDYA-${txId}</MsgId>
      <CreDtTm>${now}</CreDtTm>
      <NbOfTxs>1</NbOfTxs>
      <SttlmInf>
        <SttlmMtd>CLRG</SttlmMtd>
        <ClrSys>
          <Prtry>BI-FAST</Prtry>
        </ClrSys>
      </SttlmInf>
    </GrpHdr>
    <CdtTrfTxInf>
      <PmtId>
        <EndToEndId>E2E-${txId}</EndToEndId>
        <TxId>${txId}</TxId>
      </PmtId>
      <IntrBkSttlmAmt Ccy="IDR">${amt.toFixed(2)}</IntrBkSttlmAmt>
      <DbtrAgt>
        <FinInstnId><BICFI>${src}</BICFI></FinInstnId>
      </DbtrAgt>
      <CdtrAgt>
        <FinInstnId><BICFI>${dst}</BICFI></FinInstnId>
      </CdtrAgt>
      <CdtrAcct>
        <Id><Othr><Id>${acc}</Id></Othr></Id>
      </CdtrAcct>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>`;

  document.getElementById('iso-payload-preview').innerText = xml;
  addFeedItem(src, dst, 'BI-FAST', amt, '0.19');
  alert(`Transaksi BI-FAST ${txId} (${formatRupiah(amt)}) ke ${dst} sukses dieksekusi dalam 0.19 ms!`);
}
