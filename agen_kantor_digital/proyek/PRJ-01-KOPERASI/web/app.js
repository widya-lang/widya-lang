// ==============================================================================
// app.js - Logika Interaktif Sistem Koperasi Desa Merah Putih (KDMP) 7 Pilar
// ==============================================================================

// STATE GLOBAL
let currentTab = 'dashboard';
let kasOperasional = 148650000;
let omzetHariIni = 24850000;
let shuBerjalan = 42120000;
let selectedPayment = 'qris';

// DATA PRODUK WARUNG POS
const PRODUK_LIST = [
  { id: 'P01', nama: 'Beras Rojolele Organik (5kg)', kategori: 'sembako', harga: 68000, stok: 45, emoji: '🌾' },
  { id: 'P02', nama: 'Minyak Goreng Sawit Desa (2L)', kategori: 'sembako', harga: 32000, stok: 80, emoji: '🛢️' },
  { id: 'P03', nama: 'Gula Pasir Kristal Putih (1kg)', kategori: 'sembako', harga: 17500, stok: 120, emoji: '🍚' },
  { id: 'P04', nama: 'Telur Ayam Ras Segar (1kg)', kategori: 'sembako', harga: 28000, stok: 60, emoji: '🥚' },
  { id: 'P05', nama: 'Pupuk NPK Mutiara 16-16 (5kg)', kategori: 'pertanian', harga: 55000, stok: 35, emoji: '🌱' },
  { id: 'P06', nama: 'Bibit Padi Inpari 32 (5kg)', kategori: 'pertanian', harga: 75000, stok: 20, emoji: '🌾' },
  { id: 'P07', nama: 'Pestisida Hayati Alami (500ml)', kategori: 'pertanian', harga: 42000, stok: 18, emoji: '🧪' },
  { id: 'P08', nama: 'Sabun Mandi Keluarga (Pack 4)', kategori: 'kebutuhan', harga: 14000, stok: 90, emoji: '🧼' },
  { id: 'P09', nama: 'Deterjen Ramah Lingkungan (1kg)', kategori: 'kebutuhan', harga: 22000, stok: 50, emoji: '🫧' },
  { id: 'P10', nama: 'Kopi Robusta Lereng Desa (250g)', kategori: 'sembako', harga: 35000, stok: 40, emoji: '☕' }
];

let cart = [];
let currentCategory = 'semua';

// FORMAT RUPIAH
function formatRupiah(num) {
  return 'Rp ' + Number(num).toLocaleString('id-ID');
}

// INISIALISASI
document.addEventListener('DOMContentLoaded', () => {
  renderProduk();
  renderLogAwal();
  kalkulasiPinjaman();
  startLiveClock();
});

// CLOCK
function startLiveClock() {
  const update = () => {
    const now = new Date();
    const timeStr = now.toLocaleTimeString('id-ID', { hour12: false }) + ' WIB';
    const clockEl = document.getElementById('live-clock');
    if (clockEl) clockEl.innerText = timeStr;
  };
  update();
  setInterval(update, 1000);
}

// SWITCH TAB
function switchTab(tabId) {
  currentTab = tabId;
  
  // Hide all panes
  document.querySelectorAll('.tab-pane').forEach(p => p.classList.remove('active'));
  document.querySelectorAll('.nav-item').forEach(n => n.classList.remove('active'));

  // Show target
  const targetPane = document.getElementById('tab-' + tabId);
  const targetNav = document.getElementById('nav-' + tabId);
  if (targetPane) targetPane.classList.add('active');
  if (targetNav) targetNav.classList.add('active');

  // Title update
  const titleMap = {
    dashboard: ['Dashboard Eksekutif Koperasi KDMP', 'Sinergi 7 Pilar Usaha Terpadu Desa Mandiri • Sinkronisasi Real-Time'],
    pos: ['Kasir Point of Sales (POS) Warung Sembako Desa', 'Barcode Scanner, Diskon 5% Anggota, & QRIS Terintegrasi WidyaPay'],
    simpanpinjam: ['Unit Simpan Pinjam Syariah & Konvensional', 'Bunga Flat 0.8%, Tanpa Biaya Tersembunyi, Proteksi Dana Anggota'],
    apotik: ['Apotik & Distribusi Obat Generik Desa', 'Pengadaan Obat Terjangkau, Vitamin Subsidi, & Resep Dokter'],
    klinik: ['Klinik Pratama Desa & Rekam Medis SatuSehat', 'Interoperabilitas HL7/FHIR Kemenkes RI & Bridging BPJS V-Claim'],
    gudang: ['Sistem Resi Gudang (SRG) & Tokenisasi Komoditas', 'Sertifikat Digital Anti-Tengkulak untuk Petani & Peternak Desa'],
    logistik: ['Manajemen Armada Logistik & Distribusi Panen', 'Pelacakan Rute Armada Truk & Pikap Desa Real-Time']
  };

  if (titleMap[tabId]) {
    document.getElementById('page-title').innerText = titleMap[tabId][0];
    document.getElementById('page-subtitle').innerText = titleMap[tabId][1];
  }
}

// RENDER PRODUK POS
function renderProduk() {
  const grid = document.getElementById('product-grid');
  if (!grid) return;

  const keyword = document.getElementById('pos-search').value.toLowerCase();
  
  grid.innerHTML = '';
  const filtered = PRODUK_LIST.filter(p => {
    const matchCat = currentCategory === 'semua' || p.kategori === currentCategory;
    const matchKey = p.nama.toLowerCase().includes(keyword);
    return matchCat && matchKey;
  });

  if (filtered.length === 0) {
    grid.innerHTML = '<div style="grid-column: 1/-1; text-align: center; color: var(--text-dim); padding: 40px;">Tidak ada produk yang cocok.</div>';
    return;
  }

  filtered.forEach(p => {
    const card = document.createElement('div');
    card.className = 'product-card';
    card.onclick = () => tambahKeKeranjang(p.id);
    card.innerHTML = `
      <div class="product-emoji">${p.emoji}</div>
      <div class="product-title">${p.nama}</div>
      <div class="product-stock">Tersedia: <strong>${p.stok} unit</strong></div>
      <div class="product-price">${formatRupiah(p.harga)}</div>
    `;
    grid.appendChild(card);
  });
}

function filterProduk() {
  renderProduk();
}

function filterKategori(cat) {
  currentCategory = cat;
  document.querySelectorAll('.cat-pill').forEach(btn => btn.classList.remove('active'));
  event.target.classList.add('active');
  renderProduk();
}

// LOGIKA KERANJANG BELANJA
function tambahKeKeranjang(productId) {
  const item = PRODUK_LIST.find(p => p.id === productId);
  if (!item) return;

  const existing = cart.find(c => c.id === productId);
  if (existing) {
    existing.qty += 1;
  } else {
    cart.push({ ...item, qty: 1 });
  }

  renderKeranjang();
}

function updateQty(id, delta) {
  const item = cart.find(c => c.id === id);
  if (!item) return;

  item.qty += delta;
  if (item.qty <= 0) {
    cart = cart.filter(c => c.id !== id);
  }
  renderKeranjang();
}

function kosongkanKeranjang() {
  cart = [];
  renderKeranjang();
}

function renderKeranjang() {
  const container = document.getElementById('cart-items');
  const badge = document.getElementById('badge-cart');
  
  const totalItems = cart.reduce((sum, c) => sum + c.qty, 0);
  if (badge) badge.innerText = totalItems;

  if (cart.length === 0) {
    container.innerHTML = '<div class="cart-empty">Keranjang belanja kosong.<br>Pilih produk di sebelah kiri.</div>';
    hitungTotalPos();
    return;
  }

  container.innerHTML = '';
  cart.forEach(item => {
    const row = document.createElement('div');
    row.className = 'cart-item';
    row.innerHTML = `
      <div>
        <div class="cart-item-name">${item.nama}</div>
        <div style="font-size: 11px; color: #f87171;">${formatRupiah(item.harga)} x ${item.qty}</div>
      </div>
      <div class="cart-item-qty">
        <button class="btn-qty" onclick="updateQty('${item.id}', -1)">-</button>
        <span style="font-size: 12px; font-weight: 700; width: 16px; text-align: center;">${item.qty}</span>
        <button class="btn-qty" onclick="updateQty('${item.id}', 1)">+</button>
      </div>
    `;
    container.appendChild(row);
  });

  hitungTotalPos();
}

function hitungTotalPos() {
  const subtotal = cart.reduce((sum, c) => sum + (c.harga * c.qty), 0);
  const memberSelect = document.getElementById('select-member');
  const isMember = memberSelect && memberSelect.value !== 'umum';
  
  const diskon = isMember ? subtotal * 0.05 : 0;
  const total = subtotal - diskon;

  document.getElementById('pos-subtotal').innerText = formatRupiah(subtotal);
  document.getElementById('pos-diskon').innerText = '- ' + formatRupiah(diskon);
  document.getElementById('pos-total').innerText = formatRupiah(total);

  return { subtotal, diskon, total };
}

function setPaymentMethod(method) {
  selectedPayment = method;
  document.querySelectorAll('.btn-method').forEach(b => b.classList.remove('active'));
  document.getElementById('btn-' + method).classList.add('active');
}

// PROSES TRANSAKSI POS
function prosesPembayaranPOS() {
  if (cart.length === 0) {
    alert('Keranjang masih kosong!');
    return;
  }

  const { total, diskon } = hitungTotalPos();
  const trxId = 'TRX-' + Math.floor(100000 + Math.random() * 900000);
  const timeNow = new Date().toLocaleTimeString('id-ID', { hour12: false });

  // Update Kas & Omzet
  kasOperasional += total;
  omzetHariIni += total;
  shuBerjalan += (total * 0.08); // 8% margin SHU

  document.getElementById('total-kas').innerText = formatRupiah(kasOperasional);
  document.getElementById('val-omzet').innerText = formatRupiah(omzetHariIni);
  document.getElementById('val-shu').innerText = formatRupiah(shuBerjalan);

  // Buat Receipt
  const receiptEl = document.getElementById('modal-receipt');
  receiptEl.innerHTML = `
    ========================================
    KOPERASI DESA MERAH PUTIH (KDMP)
    Unit Usaha: Warung Sembako POS #01
    No. Transaksi : ${trxId}
    Waktu         : ${timeNow} WIB
    Kasir         : Ahmad Dahlan
    ----------------------------------------
    ${cart.map(c => `${c.nama.padEnd(26).slice(0, 26)} x${c.qty} = ${formatRupiah(c.harga * c.qty)}`).join('\n    ')}
    ----------------------------------------
    Hemat Diskon  : ${formatRupiah(diskon)}
    TOTAL BAYAR   : ${formatRupiah(total)}
    Metode Bayar  : ${selectedPayment.toUpperCase()} WIDYAPAY
    Status DB     : ACID RECORDED & WAL OK
    ========================================
  `;

  // Catat ke Log Real-time
  tambahLog('Warung POS', `Penjualan ${cart.length} item (${trxId})`, total, selectedPayment.toUpperCase());

  // Buka Modal
  document.getElementById('modal-checkout').classList.add('active');
  kosongkanKeranjang();
}

function tutupModal() {
  document.getElementById('modal-checkout').classList.remove('active');
}

// SIMULASI PINJAMAN
function kalkulasiPinjaman() {
  const nominal = Number(document.getElementById('sp-nominal').value) || 0;
  const tenor = Number(document.getElementById('sp-tenor').value) || 12;

  const pokokBulan = Math.round(nominal / tenor);
  const bungaBulan = Math.round(nominal * 0.008); // 0.8%
  const totalBulan = pokokBulan + bungaBulan;

  document.getElementById('res-pokok').innerText = formatRupiah(pokokBulan);
  document.getElementById('res-bunga').innerText = formatRupiah(bungaBulan);
  document.getElementById('res-total-angsuran').innerText = formatRupiah(totalBulan);
}

function ajukanPinjaman() {
  const nominal = Number(document.getElementById('sp-nominal').value);
  const peruntukan = document.getElementById('sp-peruntukan').value;
  const pinjamId = 'PJ-2026-' + Math.floor(100 + Math.random() * 900);

  const tbody = document.getElementById('sp-tbody');
  const tr = document.createElement('tr');
  tr.innerHTML = `
    <td><code>${pinjamId}</code></td>
    <td>Bpk. Pemohon Baru (${peruntukan.slice(0, 15)}...)</td>
    <td>${formatRupiah(nominal)}</td>
    <td>${formatRupiah(nominal)}</td>
    <td><span class="badge-success">Disetujui (1/12)</span></td>
  `;
  tbody.prepend(tr);

  tambahLog('Simpan Pinjam', `Pencairan Modal Usaha ${pinjamId}`, nominal, 'TRANSFER WIDYAPAY');
  alert(`Pengajuan Pinjaman ${pinjamId} sebesar ${formatRupiah(nominal)} berhasil disetujui & dicatat di WidyaBase!`);
}

// APOTIK JUAL OBAT
function jualObat(nama, harga) {
  tambahLog('Apotik Desa', `Penjualan Obat: ${nama}`, harga, 'QRIS');
  kasOperasional += harga;
  omzetHariIni += harga;
  document.getElementById('total-kas').innerText = formatRupiah(kasOperasional);
  document.getElementById('val-omzet').innerText = formatRupiah(omzetHariIni);
  alert(`Resep / Penjualan obat '${nama}' (${formatRupiah(harga)}) sukses diproses!`);
}

// SATUSEHAT DISPATCH
function kirimSatuSehat() {
  const icd = document.getElementById('klinik-icd10').value;
  tambahLog('Klinik Desa', `Sinkronisasi RME Pasien SatuSehat (${icd})`, 35000, 'BPJS BRIDGING');
  alert(`Payload FHIR Encounter & Condition (${icd}) berhasil dikirim & tervalidasi di SatuSehat Kemenkes!`);
}

// RESI GUDANG
function terbitkanResiGudang() {
  const srgId = 'SRG-PADI-' + Math.floor(1000 + Math.random() * 9000);
  tambahLog('Resi Gudang', `Penerbitan Sertifikat ${srgId} (20 Ton)`, 130000000, 'ACID LEDGER');
  alert(`Sertifikat Resi Gudang ${srgId} senilai Rp 130.000.000 berhasil diterbitkan & ditokenisasi!`);
}

// LOGISTIK
function jadwalPengiriman() {
  tambahLog('Logistik', 'Penjadwalan Rute Baru Distribusi Hasil Panen', 450000, 'KAS DESA');
  alert('Rute distribusi armada logistik desa berhasil ditambahkan ke jadwal real-time!');
}

// LOG TRANSAKSI REAL-TIME
function renderLogAwal() {
  tambahLog('Warung POS', 'Penjualan Sembako Warga KOP-0084', 185000, 'QRIS WIDYAPAY');
  tambahLog('Simpan Pinjam', 'Angsuran Ke-8 Pinjaman PJ-2026-001', 913333, 'POTONG SIMPANAN');
  tambahLog('Apotik Desa', 'Pengambilan Vitamin Warga Lansia', 15000, 'TUNAI');
  tambahLog('Klinik Desa', 'Pemeriksaan Tensi & RME SatuSehat', 35000, 'BPJS');
}

function tambahLog(unit, uraian, nominal, metode) {
  const tbody = document.getElementById('log-tbody');
  if (!tbody) return;

  const now = new Date().toLocaleTimeString('id-ID', { hour12: false });
  const tr = document.createElement('tr');
  tr.innerHTML = `
    <td><code>${now}</code></td>
    <td><strong>${unit}</strong></td>
    <td>${uraian}</td>
    <td style="color: #f87171; font-weight: 700; font-family: 'JetBrains Mono';">${formatRupiah(nominal)}</td>
    <td><span class="badge-code" style="background: rgba(255,255,255,0.08);">${metode}</span></td>
    <td><span class="badge-success">ACID COMMITTED</span></td>
  `;
  tbody.prepend(tr);
}

function bersihkanLog() {
  renderLogAwal();
}

function simulasiTransaksiBaru() {
  const randItems = [
    { unit: 'Warung POS', uraian: 'Penjualan Beras 10kg & Minyak', nominal: 168000, metode: 'QRIS' },
    { unit: 'Apotik Desa', uraian: 'Penjualan Paracetamol & Vitamin C', nominal: 24500, metode: 'TUNAI' },
    { unit: 'Simpan Pinjam', uraian: 'Setoran Simpanan Wajib Anggota KOP-0341', 50000, metode: 'TRANSFER' }
  ];
  const item = randItems[Math.floor(Math.random() * randItems.length)];
  tambahLog(item.unit, item.uraian, item.nominal, item.metode);
  kasOperasional += item.nominal;
  omzetHariIni += item.nominal;
  document.getElementById('total-kas').innerText = formatRupiah(kasOperasional);
  document.getElementById('val-omzet').innerText = formatRupiah(omzetHariIni);
}
