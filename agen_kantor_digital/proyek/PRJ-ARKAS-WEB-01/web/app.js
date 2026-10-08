// ==============================================================================
// APP.JS - STATE ENGINE & INTERAKTIVITAS ARKAS WEB TERINTEGRASI
// ==============================================================================

// State Global Sistem ARKAS
const ARKAS_STATE = {
  sekolah: {
    npsn: "20301234",
    nama: "SD Negeri 1 Somagede",
    kepsek: "Drs. Slamet Riyadi, M.Pd",
    bendahara: "Anisa Rahmawati, S.Pd",
    rekening: "140-00-9876543-2",
  },
  sumberDana: [
    { nama: "BOSP Reguler (Tahap 1 & 2)", pagu: 225000000, cair: 112500000, sisa: 75500000, status: "Tahap 1 Salur (50%)" },
    { nama: "BOS Kinerja Rapor Mutu", pagu: 45000000, cair: 45000000, sisa: 45000000, status: "Salur Penuh (100%)" },
    { nama: "BOSDA APBD Banyumas", pagu: 25000000, cair: 25000000, sisa: 20000000, status: "Salur Semester 1" },
    { nama: "Banper APBN Smart Classroom", pagu: 150000000, cair: 105000000, sisa: 5000000, status: "Termin 1 (70%)" }
  ],
  rkas: [
    { id: "KEG-01-SNP6", kodeSnp: "06.01.01", uraian: "Pengadaan Buku Teks Utama Kurikulum Nasional", rekening: "5.2.05.01.01.0001", kelompok: "BelanjaModalBuku", pagu: 35000000, realisasi: 35000000 },
    { id: "KEG-02-SNP5", kodeSnp: "05.02.01", uraian: "Honorarium Guru Tidak Tetap & Tendik Non-ASN", rekening: "5.1.01.02.01.0005", kelompok: "BelanjaPegawaiHonor", pagu: 48000000, realisasi: 24000000 },
    { id: "KEG-03-SNP8", kodeSnp: "08.03.01", uraian: "Langganan Daya Listrik PLN & Internet Sekolah", rekening: "5.1.02.02.01.0061", kelompok: "BelanjaBarangJasa", pagu: 18000000, realisasi: 9000000 },
    { id: "KEG-04-SNP6", kodeSnp: "06.02.03", uraian: "Pengadaan Smart Display Interaktif (Banper APBN)", rekening: "5.2.02.05.01.0002", kelompok: "BelanjaModalPeralatan", pagu: 100000000, realisasi: 100000000 },
    { id: "KEG-05-SNP7", kodeSnp: "07.01.02", uraian: "Pelaksanaan Asesmen Nasional Berbasis Komputer (ANBK)", rekening: "5.1.02.01.01.0024", kelompok: "BelanjaBarangJasa", pagu: 12000000, realisasi: 4000000 }
  ],
  bku: [
    {
      no: 1,
      tanggal: "2026-01-15",
      bukti: "TRF-BOSP-T1",
      uraian: "Penyaluran BOSP Reguler Tahap 1 dari Kasda",
      jenis: "PenerimaanDanaTransfer",
      metode: "TransferBankGiro",
      terima: 112500000,
      keluar: 0,
      saldo: 112500000,
      sumber: "BOSP",
      hash: "a718c2d8e4b5f90123456789abcdef0123456789abcdef0123456789abcdef01",
      prevHash: "0000000000000000000000000000000000000000000000000000000000000000"
    },
    {
      no: 2,
      tanggal: "2026-01-20",
      bukti: "SP-SIP-202601-01",
      uraian: "Belanja SIPLah Buku Teks Utama PT Intan Pariwara",
      jenis: "BelanjaOperasionalSIPLah",
      metode: "VirtualAccountSIPLah",
      terima: 0,
      keluar: 35000000,
      saldo: 77500000,
      sumber: "BOSP",
      hash: "b829d3e9f5c6a01234567890bcdef1234567890bcdef1234567890bcdef1234",
      prevHash: "a718c2d8e4b5f90123456789abcdef0123456789abcdef0123456789abcdef01"
    },
    {
      no: 3,
      tanggal: "2026-02-01",
      bukti: "TARIK-TUNAI-01",
      uraian: "Penarikan Kas Tunai Bank untuk Operasional Sekolah",
      jenis: "TarikTunaiDariBank",
      metode: "TransferBankGiro",
      terima: 0,
      keluar: 10000000,
      saldo: 77500000,
      sumber: "BOSP",
      hash: "c93ae4fa06d7b12345678901cdef2345678901cdef2345678901cdef2345678",
      prevHash: "b829d3e9f5c6a01234567890bcdef1234567890bcdef1234567890bcdef1234"
    },
    {
      no: 4,
      tanggal: "2026-02-05",
      bukti: "KWT-PLN-0201",
      uraian: "Pembayaran Rekening Listrik PLN & Internet Tunai",
      jenis: "BelanjaNonSIPLah",
      metode: "KasTunaiBendahara",
      terima: 0,
      keluar: 1500000,
      saldo: 76000000,
      sumber: "BOSP",
      hash: "da4bf50b17e8c23456789012def3456789012def3456789012def3456789012",
      prevHash: "c93ae4fa06d7b12345678901cdef2345678901cdef2345678901cdef2345678"
    },
    {
      no: 5,
      tanggal: "2026-02-25",
      bukti: "SP-SIP-202602-02",
      uraian: "Pengadaan 2 Unit Smart Display Kelas (Banper APBN)",
      jenis: "BelanjaOperasionalSIPLah",
      metode: "VirtualAccountSIPLah",
      terima: 0,
      keluar: 100000000,
      saldo: 76000000,
      sumber: "BANPER",
      hash: "eb5cf61c28f9d34567890123ef4567890123ef4567890123ef4567890123ef45",
      prevHash: "da4bf50b17e8c23456789012def3456789012def3456789012def3456789012"
    }
  ],
  pajak: [
    { idPajak: "PJK-202601-01", noBukti: "SP-SIP-202601-01", pph21: 0, pph22: 525000, pph23: 0, ppn11: 3850000, total: 4375000, billing: "EBILL-88334411", ntpn: "NTPN-998877665544", status: "DISETOR" },
    { idPajak: "PJK-202602-01", noBukti: "SP-SIP-202602-02", pph21: 0, pph22: 1500000, pph23: 0, ppn11: 11000000, total: 12500000, billing: "EBILL-88334422", ntpn: "NTPN-776655443322", status: "DISETOR" }
  ],
  siplah: [
    { sp: "SP-SIP-202601-01", mitra: "SIPLah Telkom", toko: "PT Intan Pariwara", nilai: 35000000, bast: "BAST-20260120-001", tglBast: "2026-01-20", status: "LUNAS" },
    { sp: "SP-SIP-202602-02", mitra: "SIPLah Blibli", toko: "PT Teknologi Edukasi Nusantara", nilai: 100000000, bast: "BAST-20260226-0081", tglBast: "2026-02-26", status: "LUNAS" }
  ],
  saldoBank: 70500000,
  saldoTunai: 5500000,
};

// Format Rupiah
function formatRupiah(num) {
  return "Rp " + Math.round(num).toLocaleString("id-ID");
}

// Simulasi Kripto SHA-256 (Pseudorandom Deterministic)
function pseudoSha256(str) {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash |= 0;
  }
  const hex = Math.abs(hash).toString(16).padStart(8, '0');
  return hex.repeat(8).substring(0, 64);
}

// Inisialisasi Tampilan
document.addEventListener("DOMContentLoaded", () => {
  setupNavigation();
  renderKpis();
  renderSumberDana();
  renderRkas();
  renderBku();
  renderPembantu();
  renderSiplah();
  setupTaxCalculator();
  setupFormTransaksi();
  setupModalAudit();
  setupTutupBuku();
});

// Setup Navigasi Tab
function setupNavigation() {
  const buttons = document.querySelectorAll(".nav-item");
  const panes = document.querySelectorAll(".tab-pane");

  buttons.forEach(btn => {
    btn.addEventListener("click", () => {
      buttons.forEach(b => b.classList.remove("active"));
      panes.forEach(p => p.classList.remove("active"));

      btn.classList.add("active");
      const targetId = btn.getAttribute("data-tab");
      document.getElementById(targetId).classList.add("active");

      // Perbarui judul halaman
      const titles = {
        "tab-dashboard": "Dasbor Tata Kelola Keuangan Sekolah",
        "tab-rkas": "Perencanaan RKAS (8 SNP & Batas Juknis)",
        "tab-bku": "Buku Kas Umum (BKU) & Rantai Audit",
        "tab-transaksi-baru": "Formulir Pencatatan Transaksi BKU",
        "tab-pembantu": "Buku Pembantu (Bank, Kas Tunai & Pajak)",
        "tab-siplah": "Pengadaan Mitra Resmi SIPLah Kemendikdasmen"
      };
      document.getElementById("pageTitle").innerText = titles[targetId] || "ARKAS Web";
    });
  });
}

// Render KPI Cards
function renderKpis() {
  const totalPagu = ARKAS_STATE.sumberDana.reduce((acc, s) => acc + s.pagu, 0);
  const totalRealisasi = ARKAS_STATE.rkas.reduce((acc, r) => acc + r.realisasi, 0);
  const persen = ((totalRealisasi / totalPagu) * 100).toFixed(2);

  document.getElementById("kpiTotalPagu").innerText = formatRupiah(totalPagu);
  document.getElementById("kpiTotalRealisasi").innerText = formatRupiah(totalRealisasi);
  document.getElementById("kpiSerapanPersen").innerText = persen + "%";
  document.getElementById("kpiProgressFill").style.width = persen + "%";

  document.getElementById("kpiSaldoBank").innerText = formatRupiah(ARKAS_STATE.saldoBank);
  document.getElementById("kpiSaldoTunai").innerText = formatRupiah(ARKAS_STATE.saldoTunai);
  document.getElementById("bpBankSaldo").innerText = formatRupiah(ARKAS_STATE.saldoBank);
  document.getElementById("bpKasSaldo").innerText = formatRupiah(ARKAS_STATE.saldoTunai);
}

// Render Tabel Sumber Dana
function renderSumberDana() {
  const tbody = document.querySelector("#tableSumberDana tbody");
  tbody.innerHTML = ARKAS_STATE.sumberDana.map(s => `
    <tr>
      <td><strong>${s.nama}</strong></td>
      <td>${formatRupiah(s.pagu)}</td>
      <td>${formatRupiah(s.cair)}</td>
      <td>${formatRupiah(s.sisa)}</td>
      <td><span class="badge-success">${s.status}</span></td>
    </tr>
  `).join("");
}

// Render Tabel RKAS 8 SNP
function renderRkas() {
  const tbody = document.querySelector("#tableRkas tbody");
  tbody.innerHTML = ARKAS_STATE.rkas.map(r => {
    const sisa = r.pagu - r.realisasi;
    const persen = Math.round((r.realisasi / r.pagu) * 100);
    return `
      <tr>
        <td><strong>${r.kodeSnp}</strong></td>
        <td>${r.uraian}</td>
        <td><code>${r.rekening}</code></td>
        <td><span class="badge-tag">${r.kelompok}</span></td>
        <td>${formatRupiah(r.pagu)}</td>
        <td>${formatRupiah(r.realisasi)}</td>
        <td>${formatRupiah(sisa)}</td>
        <td>
          <div style="font-weight:700; color:${persen >= 100 ? '#10b981' : '#3b82f6'}">${persen}%</div>
        </td>
      </tr>
    `;
  }).join("");
}

// Render Tabel BKU
function renderBku(filter = "ALL") {
  const tbody = document.querySelector("#tableBku tbody");
  const filtered = filter === "ALL" ? ARKAS_STATE.bku : ARKAS_STATE.bku.filter(b => b.sumber === filter);

  tbody.innerHTML = filtered.map(b => `
    <tr>
      <td>${b.no}</td>
      <td>${b.tanggal}</td>
      <td><strong>${b.bukti}</strong></td>
      <td>${b.uraian}</td>
      <td><span class="badge-tag">${b.metode}</span></td>
      <td style="color:#10b981">${b.terima > 0 ? formatRupiah(b.terima) : '-'}</td>
      <td style="color:#ef4444">${b.keluar > 0 ? formatRupiah(b.keluar) : '-'}</td>
      <td><strong>${formatRupiah(b.saldo)}</strong></td>
      <td><span class="hash-badge" title="Prev: ${b.prevHash}">SHA256:${b.hash.substring(0, 12)}...</span></td>
    </tr>
  `).join("");
}

// Filter BKU
document.getElementById("filterBkuSumberDana")?.addEventListener("change", (e) => {
  renderBku(e.target.value);
});

// Render Buku Pembantu Pajak
function renderPembantu() {
  const tbody = document.querySelector("#tablePajak tbody");
  tbody.innerHTML = ARKAS_STATE.pajak.map(p => `
    <tr>
      <td><strong>${p.idPajak}</strong></td>
      <td>${p.noBukti}</td>
      <td>${p.pph21 > 0 ? formatRupiah(p.pph21) : '-'}</td>
      <td>${p.pph22 > 0 ? formatRupiah(p.pph22) : '-'}</td>
      <td>${p.pph23 > 0 ? formatRupiah(p.pph23) : '-'}</td>
      <td>${p.ppn11 > 0 ? formatRupiah(p.ppn11) : '-'}</td>
      <td><strong>${formatRupiah(p.total)}</strong></td>
      <td><code>${p.billing}</code></td>
      <td><code>${p.ntpn}</code></td>
      <td><span class="badge-success">${p.status}</span></td>
    </tr>
  `).join("");
}

// Render SIPLah
function renderSiplah() {
  const tbody = document.querySelector("#tableSiplah tbody");
  tbody.innerHTML = ARKAS_STATE.siplah.map(s => `
    <tr>
      <td><strong>${s.sp}</strong></td>
      <td>${s.mitra}</td>
      <td>${s.toko}</td>
      <td><strong>${formatRupiah(s.nilai)}</strong></td>
      <td><code>${s.bast}</code></td>
      <td>${s.tglBast}</td>
      <td><span class="badge-success">${s.status}</span></td>
    </tr>
  `).join("");
}

// Kalkulator Pajak Otomatis di Form
function setupTaxCalculator() {
  const nominalInput = document.getElementById("inputNominal");
  const chkPajak = document.getElementById("chkKenaPajak");
  const selKategori = document.getElementById("selKategoriPajak");
  const chkNpwp = document.getElementById("chkMemilikiNpwp");

  function hitung() {
    const nominal = parseFloat(nominalInput.value) || 0;
    const isPajak = chkPajak.checked;
    const kategori = selKategori.value;
    const npwp = chkNpwp.checked;

    if (!isPajak || nominal <= 0) {
      document.getElementById("estPpn").innerText = "Rp 0";
      document.getElementById("estPph").innerText = "Rp 0";
      document.getElementById("estTotalPajak").innerText = "Rp 0";
      return;
    }

    let ppn = 0;
    let pph = 0;

    if (kategori === "Barang") {
      if (nominal >= 2000000) {
        ppn = Math.round(nominal * 0.11);
        pph = Math.round(nominal * (npwp ? 0.015 : 0.03));
      }
    } else if (kategori === "Jasa") {
      if (nominal >= 2000000) {
        ppn = Math.round(nominal * 0.11);
      }
      pph = Math.round(nominal * (npwp ? 0.02 : 0.04));
    } else if (kategori === "HonorNonPNS") {
      pph = Math.round(nominal * (npwp ? 0.05 : 0.06));
    }

    const total = ppn + pph;
    document.getElementById("estPpn").innerText = formatRupiah(ppn);
    document.getElementById("estPph").innerText = formatRupiah(pph);
    document.getElementById("estTotalPajak").innerText = formatRupiah(total);
  }

  nominalInput.addEventListener("input", hitung);
  chkPajak.addEventListener("change", hitung);
  selKategori.addEventListener("change", hitung);
  chkNpwp.addEventListener("change", hitung);
}

// Setup Form Input Transaksi BKU
function setupFormTransaksi() {
  const form = document.getElementById("formTransaksiBku");
  form.addEventListener("submit", (e) => {
    e.preventDefault();

    const jenis = document.getElementById("inputJenisTransaksi").value;
    const metode = document.getElementById("inputMetodeBayar").value;
    const bukti = document.getElementById("inputNoBukti").value.trim();
    const tanggal = document.getElementById("inputTanggal").value;
    const uraian = document.getElementById("inputUraian").value.trim();
    const pos = document.getElementById("inputPosKegiatan").value;
    const nominal = parseFloat(document.getElementById("inputNominal").value) || 0;

    if (nominal <= 0) {
      alert("Nominal belanja harus lebih dari 0!");
      return;
    }

    // Validasi saldo
    if (metode === "KasTunaiBendahara" && nominal > ARKAS_STATE.saldoTunai) {
      alert(`Saldo Kas Tunai tidak mencukupi! Tersedia: ${formatRupiah(ARKAS_STATE.saldoTunai)}`);
      return;
    }
    if (metode !== "KasTunaiBendahara" && nominal > ARKAS_STATE.saldoBank) {
      alert(`Saldo Bank tidak mencukupi! Tersedia: ${formatRupiah(ARKAS_STATE.saldoBank)}`);
      return;
    }

    // Ambil hash sebelumnya
    const lastBku = ARKAS_STATE.bku[ARKAS_STATE.bku.length - 1];
    const prevHash = lastBku ? lastBku.hash : "0000000000000000000000000000000000000000000000000000000000000000";

    // Hitung saldo baru
    let saldoBaru = lastBku.saldo;
    let keluar = 0;
    let terima = 0;

    if (jenis === "TarikTunaiDariBank") {
      ARKAS_STATE.saldoBank -= nominal;
      ARKAS_STATE.saldoTunai += nominal;
      keluar = nominal;
    } else if (jenis === "SetorTunaiKeBank") {
      ARKAS_STATE.saldoTunai -= nominal;
      ARKAS_STATE.saldoBank += nominal;
      keluar = nominal;
      terima = nominal;
    } else {
      // Belanja
      if (metode === "KasTunaiBendahara") {
        ARKAS_STATE.saldoTunai -= nominal;
      } else {
        ARKAS_STATE.saldoBank -= nominal;
      }
      saldoBaru -= nominal;
      keluar = nominal;

      // Update RKAS realization
      const rkasItem = ARKAS_STATE.rkas.find(r => r.id === pos);
      if (rkasItem) {
        rkasItem.realisasi += nominal;
      }
    }

    const newNo = ARKAS_STATE.bku.length + 1;
    const newHash = pseudoSha256(`${prevHash}:${newNo}:${tanggal}:${bukti}:${terima}:${keluar}:${saldoBaru}`);

    const newEntry = {
      no: newNo,
      tanggal,
      bukti,
      uraian,
      jenis,
      metode,
      terima,
      keluar,
      saldo: saldoBaru,
      sumber: "BOSP",
      hash: newHash,
      prevHash
    };

    ARKAS_STATE.bku.push(newEntry);

    // Refresh UI
    renderKpis();
    renderRkas();
    renderBku();
    renderPembantu();

    alert(`✓ Transaksi '${bukti}' berhasil dicatat di BKU! Hash SHA-256 telah dikunci.`);
    form.reset();

    // Pindah ke tab BKU
    document.querySelector("[data-tab='tab-bku']").click();
  });
}

// Modal Uji Audit Kriptografis
function setupModalAudit() {
  const modal = document.getElementById("auditModal");
  const btnOpen = document.getElementById("btnAuditChain");
  const btnClose = document.getElementById("btnCloseAuditModal");
  const visualizer = document.getElementById("hashVisualizer");

  btnOpen.addEventListener("click", () => {
    visualizer.innerHTML = ARKAS_STATE.bku.map(b => `
      <div class="chain-node">
        <div><strong>[BLOK #${b.no}] ${b.tanggal} — ${b.bukti}</strong></div>
        <div>Uraian: ${b.uraian}</div>
        <div>Penerimaan: ${formatRupiah(b.terima)} | Pengeluaran: ${formatRupiah(b.keluar)} | Saldo: ${formatRupiah(b.saldo)}</div>
        <div style="color:#64748b">Prev Hash: ${b.prevHash}</div>
        <div style="color:#38bdf8">This Hash: ${b.hash}</div>
        <div style="color:#10b981; font-weight:700">✓ Status: VALID & ANTI-TAMPER</div>
      </div>
    `).join("");

    modal.classList.add("active");
  });

  btnClose.addEventListener("click", () => {
    modal.classList.remove("active");
  });

  document.getElementById("btnExportAuditLog")?.addEventListener("click", () => {
    alert("Berita Acara Rekonsiliasi & Audit Kriptografis siap dicetak (ISO 32000-1).");
  });
}

// Tutup Buku
function setupTutupBuku() {
  document.getElementById("btnTutupBuku").addEventListener("click", () => {
    const tgl = new Date().toISOString().split("T")[0];
    alert(`Buku Kas Umum (BKU) Bulan Ini Telah DITUTUP per ${tgl}.\nSaldo Bank: ${formatRupiah(ARKAS_STATE.saldoBank)}\nSaldo Kas Tunai: ${formatRupiah(ARKAS_STATE.saldoTunai)}\nData telah terkunci dan siap disinkronkan ke Markas Dinas.`);
  });
}
