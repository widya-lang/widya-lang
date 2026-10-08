// ==============================================================================
// app.js - Logika Frontend & Simulasi Mandiri PRJ-WIDYA-INV-02
// Kompatibel dengan Runtime WidyaWeb Port 8011 & Mesin Basis Data WidyaBase
// ==============================================================================

// State Lokal In-Memory
let state = {
  barang: [
    { sku: "BRG-KOP-001", ean: "8992753123456", nama: "Beras Rojo Lele Super 5kg", kategori: "Sembako", satuan: "Sak", stok: 120, beli: 68000, jual: 75000 },
    { sku: "BRG-KOP-002", ean: "8993175402014", nama: "Minyak Goreng Sawit 2L", kategori: "Sembako", satuan: "Pouch", stok: 85, beli: 29000, jual: 33500 },
    { sku: "BRG-KOP-003", ean: "8999999195028", nama: "Gula Pasir Kristal Putih 1kg", kategori: "Sembako", satuan: "Bks", stok: 240, beli: 14500, jual: 17000 },
    { sku: "BRG-KOP-004", ean: "8996001301019", nama: "Kopi Robusta Temanggung 250g", kategori: "Komoditas", satuan: "Bks", stok: 60, beli: 22000, jual: 28000 },
    { sku: "BRG-KOP-005", ean: "8991001402026", nama: "Pupuk Organik Granul 10kg", kategori: "Pertanian", satuan: "Sak", stok: 45, beli: 45000, jual: 52000 }
  ],
  rfid: [
    { epc: "3014257BF7194E4000001A85", sku: "BRG-KOP-001", serial: "SER-0001", rssi: -42.5, status: "AKTIF", waktu: "2026-10-02 10:15:00" },
    { epc: "3014257BF7194E4000001A86", sku: "BRG-KOP-001", serial: "SER-0002", rssi: -43.1, status: "AKTIF", waktu: "2026-10-02 10:15:02" },
    { epc: "3014257BF7194E4000002B10", sku: "BRG-KOP-002", serial: "SER-0010", rssi: -50.2, status: "AKTIF", waktu: "2026-10-02 10:16:30" }
  ],
  rak: [
    { bin: "BIN-A-01-01-01", zona: "Zona A", lorong: 1, rak: 1, tingkat: 1, kapasitas: 150, terisi: 120, sku: "BRG-KOP-001" },
    { bin: "BIN-A-01-01-02", zona: "Zona A", lorong: 1, rak: 1, tingkat: 2, kapasitas: 150, terisi: 45, sku: "BRG-KOP-005" },
    { bin: "BIN-B-02-01-01", zona: "Zona B", lorong: 2, rak: 1, tingkat: 1, kapasitas: 200, terisi: 85, sku: "BRG-KOP-002" },
    { bin: "BIN-B-02-02-01", zona: "Zona B", lorong: 2, rak: 2, tingkat: 1, kapasitas: 200, terisi: 240, sku: "BRG-KOP-003" },
    { bin: "BIN-C-03-01-01", zona: "Zona C", lorong: 3, rak: 1, tingkat: 1, kapasitas: 100, terisi: 60, sku: "BRG-KOP-004" }
  ],
  mutasi: [
    { id: "MUT-20261002-0001", sku: "BRG-KOP-001", jenis: "MASUK", jumlah: 50, dari: "Supplier PT Tani Makmur", ke: "BIN-A-01-01-01", waktu: "2026-10-02 08:30:00", operator: "Rizky Firmansyah" },
    { id: "MUT-20261002-0002", sku: "BRG-KOP-002", jenis: "MASUK", jumlah: 40, dari: "Supplier CV Sawit Jaya", ke: "BIN-B-02-01-01", waktu: "2026-10-02 09:15:00", operator: "Rizky Firmansyah" },
    { id: "MUT-20261002-0003", sku: "BRG-KOP-001", jenis: "KELUAR", jumlah: 10, dari: "BIN-A-01-01-01", ke: "Warung KDMP Unit 1", waktu: "2026-10-02 11:00:00", operator: "Budi Santoso" }
  ],
  opname: [
    { id: "OPN-20261002-001", sku: "BRG-KOP-001", tercatat: 120, fisik: 120, selisih: 0, status: "SESUAI", waktu: "2026-10-02 14:00:00" },
    { id: "OPN-20261002-002", sku: "BRG-KOP-002", tercatat: 85, fisik: 83, selisih: -2, status: "TERTUNDA_APPROVAL", waktu: "2026-10-02 14:15:00" }
  ]
};

// Navigasi Antar Tab
function switchTab(tabId) {
  document.querySelectorAll('.tab-pane').forEach(el => el.classList.remove('active'));
  document.querySelectorAll('.nav-item').forEach(el => el.classList.remove('active'));

  const targetTab = document.getElementById(`tab-${tabId}`);
  const targetNav = document.getElementById(`nav-${tabId}`);
  if (targetTab) targetTab.classList.add('active');
  if (targetNav) targetNav.classList.add('active');

  const titles = {
    dashboard: ["Dashboard Inventaris & Logistik UMKM", "Pemantauan Terpadu Standar EAN-13, GS1-128, dan EPC Gen2 UHF"],
    barcode: ["Generator & Validator Barcode GS1", "Kalkulator Checksum Modulo 10 & Dekode Application Identifiers"],
    rfid: ["Gerbang Pemindai RFID UHF SGTIN-96", "Telemetri Signal Strength (RSSI) & Otomasi Transisi Gerbang"],
    rak: ["Pemetaan Spasial Topologi Bin & Rak", "Visualisasi Utilisasi Kapasitas Gudang 3 Dimensi"],
    mutasi: ["Jurnal Transaksi Pergerakan Mutasi Stok", "Audit Trail ACID Logistik Tanpa Selisih (Zero Phantom Inventory)"],
    opname: ["Rekonsiliasi Audit Siklus (Cycle Count)", "Otomasi Deteksi Variansi Deviasi Stok Buku vs Realitas Fisik"]
  };

  if (titles[tabId]) {
    document.getElementById('header-title').textContent = titles[tabId][0];
    document.getElementById('header-desc').textContent = titles[tabId][1];
  }
}

// Render Master Barang
function renderBarang() {
  const tbody = document.getElementById('tbody-barang');
  tbody.innerHTML = '';
  let totalStok = 0;

  state.barang.forEach(b => {
    totalStok += b.stok;
    const tr = document.createElement('tr');
    tr.innerHTML = `
      <td class="code-cell"><b>${b.sku}</b></td>
      <td class="code-cell" style="color: var(--accent-cyan);">${b.ean}</td>
      <td>${b.nama}</td>
      <td><span class="pill pill-info">${b.kategori}</span></td>
      <td>${b.satuan}</td>
      <td><b style="font-size: 1rem; color: var(--accent-emerald);">${b.stok}</b></td>
      <td>Rp ${b.beli.toLocaleString('id-ID')}</td>
      <td>Rp ${b.jual.toLocaleString('id-ID')}</td>
      <td>
        <button class="btn btn-outline" style="padding: 0.3rem 0.6rem; font-size: 0.75rem;" onclick="pilihBarangBarcode('${b.ean.slice(0, 12)}')">🏷️ Cetak EAN</button>
      </td>
    `;
    tbody.appendChild(tr);
  });

  document.getElementById('kpi-sku').textContent = state.barang.length;
  document.getElementById('kpi-total-stok').textContent = totalStok;
}

// Render Log RFID
function renderRfid() {
  const tbody = document.getElementById('tbody-rfid');
  tbody.innerHTML = '';
  state.rfid.forEach(r => {
    const tr = document.createElement('tr');
    tr.innerHTML = `
      <td class="code-cell"><b>${r.epc}</b></td>
      <td class="code-cell">${r.sku}</td>
      <td class="code-cell">${r.serial}</td>
      <td><span class="pill ${r.rssi > -60 ? 'pill-success' : 'pill-warning'}">${r.rssi} dBm</span></td>
      <td><span class="pill pill-success">${r.status}</span></td>
      <td>${r.waktu}</td>
    `;
    tbody.appendChild(tr);
  });
  document.getElementById('kpi-rfid').textContent = state.rfid.length;
}

// Render Topologi Rak
function renderRak() {
  const container = document.getElementById('rack-container');
  container.innerHTML = '';
  state.rak.forEach(r => {
    const pct = Math.min(100, Math.round((r.terisi / r.kapasitas) * 100));
    const card = document.createElement('div');
    card.className = 'bin-card';
    card.innerHTML = `
      <div class="bin-header">
        <span>${r.bin}</span>
        <span>${r.zona}</span>
      </div>
      <div style="font-size: 0.8rem; color: var(--text-secondary);">SKU: <b>${r.sku}</b></div>
      <div style="display: flex; justify-content: space-between; font-size: 0.75rem; margin-top: 0.25rem;">
        <span>Muatan: ${r.terisi} / ${r.kapasitas}</span>
        <span><b>${pct}%</b></span>
      </div>
      <div class="bin-progress">
        <div class="bin-bar" style="width: ${pct}%;"></div>
      </div>
      <div style="font-size: 0.7rem; color: var(--text-muted); margin-top: 0.25rem;">Lorong ${r.lorong} • Rak ${r.rak} • Tingkat ${r.tingkat}</div>
    `;
    container.appendChild(card);
  });
}

// Render Mutasi Stok
function renderMutasi() {
  const tbody = document.getElementById('tbody-mutasi');
  tbody.innerHTML = '';
  state.mutasi.forEach(m => {
    const tr = document.createElement('tr');
    tr.innerHTML = `
      <td class="code-cell"><b>${m.id}</b></td>
      <td class="code-cell">${m.sku}</td>
      <td><span class="pill ${m.jenis === 'MASUK' ? 'pill-success' : 'pill-warning'}">${m.jenis}</span></td>
      <td><b>${m.jumlah}</b> unit</td>
      <td>${m.dari}</td>
      <td>${m.ke}</td>
      <td>${m.waktu}</td>
      <td>${m.operator}</td>
    `;
    tbody.appendChild(tr);
  });
}

// Render Stok Opname
function renderOpname() {
  const tbody = document.getElementById('tbody-opname');
  tbody.innerHTML = '';
  state.opname.forEach(o => {
    const tr = document.createElement('tr');
    const isSesuai = o.selisih === 0;
    tr.innerHTML = `
      <td class="code-cell"><b>${o.id}</b></td>
      <td class="code-cell">${o.sku}</td>
      <td>${o.tercatat} unit</td>
      <td>${o.fisik} unit</td>
      <td><b style="color: ${isSesuai ? 'var(--accent-emerald)' : 'var(--accent-rose)};">${o.selisih > 0 ? '+' : ''}${o.selisih}</b></td>
      <td><span class="pill ${o.status === 'SESUAI' || o.status === 'DIREKONSILIASI' ? 'pill-success' : 'pill-warning'}">${o.status}</span></td>
      <td>${o.waktu}</td>
      <td>
        ${o.status === 'TERTUNDA_APPROVAL' ? `<button class="btn btn-emerald" style="padding: 0.3rem 0.6rem; font-size: 0.75rem;" onclick="rekonsiliasiOpname('${o.id}')">Rekonsiliasi</button>` : `<span style="color: var(--text-muted); font-size: 0.8rem;">Selesai</span>`}
      </td>
    `;
    tbody.appendChild(tr);
  });
}

// Algoritma Checksum EAN-13 Modulo 10
function hitungEan13Checksum(raw12) {
  if (raw12.length !== 12 || !/^\d+$/.test(raw12)) return null;
  let sum = 0;
  for (let i = 0; i < 12; i++) {
    const d = parseInt(raw12[i], 10);
    sum += (i % 2 === 0) ? d : d * 3;
  }
  return (10 - (sum % 10)) % 10;
}

function hitungChecksumEan13() {
  const raw = document.getElementById('input-ean12').value.trim();
  const chk = hitungEan13Checksum(raw);
  if (chk === null) {
    alert("Harap masukkan tepat 12 digit angka numerik!");
    return;
  }
  const ean13 = raw + chk;
  document.getElementById('text-ean-lengkap').textContent = ean13;
  gambarBarcodeSvg(ean13);
}

function pilihBarangBarcode(ean12) {
  document.getElementById('input-ean12').value = ean12;
  switchTab('barcode');
  hitungChecksumEan13();
}

function gambarBarcodeSvg(code) {
  const svg = document.getElementById('barcode-svg-element');
  svg.innerHTML = '';
  let x = 10;
  for (let i = 0; i < code.length; i++) {
    const val = parseInt(code[i], 10);
    const barWidth = (val % 3) + 1;
    const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
    rect.setAttribute("x", x);
    rect.setAttribute("y", "5");
    rect.setAttribute("width", barWidth);
    rect.setAttribute("height", "50");
    rect.setAttribute("fill", "#000000");
    svg.appendChild(rect);
    x += barWidth + 2;
  }
}

// Parser GS1-128
function ekstrakGs1() {
  const raw = document.getElementById('input-gs1').value.trim();
  const tbody = document.getElementById('tbody-gs1-result');
  tbody.innerHTML = '';

  const aiPatterns = [
    { ai: "01", label: "GTIN (Global Trade Item Number)", regex: /\(01\)(\d{14})/ },
    { ai: "10", label: "Batch / Lot Number", regex: /\(10\)([A-Za-z0-9]+?)(?=\(|$)/ },
    { ai: "17", label: "Tanggal Kedaluwarsa (YYMMDD)", regex: /\(17\)(\d{6})/ },
    { ai: "21", label: "Nomor Seri Individual (Serial)", regex: /\(21\)([A-Za-z0-9]+?)(?=\(|$)/ }
  ];

  aiPatterns.forEach(p => {
    const match = raw.match(p.regex);
    if (match) {
      const tr = document.createElement('tr');
      tr.innerHTML = `
        <td class="code-cell"><b>(${p.ai})</b></td>
        <td>${p.label}</td>
        <td class="code-cell" style="color: var(--accent-emerald);"><b>${match[1]}</b></td>
      `;
      tbody.appendChild(tr);
    }
  });
}

// Simulasi Scan Gate RFID UHF
function triggerRfidScan(arah) {
  const antLights = document.querySelectorAll('.antenna-light');
  antLights.forEach(l => l.classList.add('active'));

  const pallet = document.getElementById('sim-pallet');
  pallet.style.transform = arah === 'MASUK' ? 'translateX(30px)' : 'translateX(-30px)';
  pallet.style.transition = 'transform 0.4s ease';

  setTimeout(() => {
    pallet.style.transform = 'translateX(0px)';

    const randomSku = state.barang[Math.floor(Math.random() * state.barang.length)].sku;
    const serialNum = `SER-${Math.floor(1000 + Math.random() * 9000)}`;
    const epcHex = `3014257BF7194E40000${Math.floor(10000 + Math.random() * 90000)}`;
    const rssi = parseFloat((-35.0 - Math.random() * 30.0).toFixed(1));
    const now = new Date().toISOString().replace('T', ' ').slice(0, 19);

    state.rfid.unshift({
      epc: epcHex,
      sku: randomSku,
      serial: serialNum,
      rssi: rssi,
      status: "AKTIF",
      waktu: now
    });

    const targetBarang = state.barang.find(b => b.sku === randomSku);
    const qty = 10;
    if (arah === 'MASUK') {
      targetBarang.stok += qty;
      state.mutasi.unshift({
        id: `MUT-${Date.now().toString().slice(-8)}`,
        sku: randomSku,
        jenis: "MASUK",
        jumlah: qty,
        dari: "Portal RFID Gate A",
        ke: "BIN-A-01-01-01",
        waktu: now,
        operator: "Sistem Otomasi RFID"
      });
    } else {
      if (targetBarang.stok >= qty) targetBarang.stok -= qty;
      state.mutasi.unshift({
        id: `MUT-${Date.now().toString().slice(-8)}`,
        sku: randomSku,
        jenis: "KELUAR",
        jumlah: qty,
        dari: "BIN-A-01-01-01",
        ke: "Truk Ekspedisi Logistik",
        waktu: now,
        operator: "Sistem Otomasi RFID"
      });
    }

    renderBarang();
    renderRfid();
    renderMutasi();
    alert(`📡 Sinyal Tag RFID Diterima!\nEPC: ${epcHex}\nSKU: ${randomSku}\nSignal: ${rssi} dBm\nStatus: ${arah === 'MASUK' ? '+10 Unit Masuk' : '-10 Unit Keluar'}`);
  }, 400);
}

function simulasiPemindaianMasuk() {
  triggerRfidScan('MASUK');
}

function rekonsiliasiOpname(id) {
  const item = state.opname.find(o => o.id === id);
  if (item) {
    item.status = "DIREKONSILIASI";
    const targetBarang = state.barang.find(b => b.sku === item.sku);
    if (targetBarang) targetBarang.stok = item.fisik;
    renderBarang();
    renderOpname();
    alert(`✅ Rekonsiliasi Sukses! Stok ${item.sku} disesuaikan ke ${item.fisik} unit.`);
  }
}

function muatUlangData() {
  renderBarang();
  renderRfid();
  renderRak();
  renderMutasi();
  renderOpname();
  hitungChecksumEan13();
  ekstrakGs1();
}

// Inisialisasi Saat Halaman Dimuat
window.addEventListener('DOMContentLoaded', () => {
  muatUlangData();
});
