/**
 * ============================================================================
 * ARENASILAT.ID - LOGIKA INTERAKTIF LANDING PAGE PUBLIK
 * ============================================================================
 */

// Data Mock Jadwal Pertandingan Real-time
const SCHEDULE_DATA = [
  {
    id: 1,
    court: 1,
    partai: 14,
    stage: 'Semifinal',
    category: 'Dewasa Putra Kelas C (55-60 Kg)',
    redName: 'Arya Wira Sena',
    redSchool: 'Tapak Suci',
    redContingent: 'DKI Jakarta',
    redScore: 24,
    blueName: 'Bagas Danuarta',
    blueSchool: 'Merpati Putih',
    blueContingent: 'Jawa Barat',
    blueScore: 22,
    status: 'live',
    statusText: 'Babak 2 Berjalan',
    timeEst: '10:45 WIB'
  },
  {
    id: 2,
    court: 2,
    partai: 18,
    stage: 'Perempat Final',
    category: 'Remaja Putri Kelas B (45-50 Kg)',
    redName: 'Siti Nurhaliza',
    redSchool: 'PSHT',
    redContingent: 'Jawa Tengah',
    redScore: 18,
    blueName: 'Dewi Lestari',
    blueSchool: 'Perisai Diri',
    blueContingent: 'Jawa Timur',
    blueScore: 12,
    status: 'live',
    statusText: 'Babak 3 Berjalan',
    timeEst: '10:50 WIB'
  },
  {
    id: 3,
    court: 3,
    partai: 9,
    stage: 'Grand Final',
    category: 'Seni Tunggal Putra Dewasa (TGR)',
    redName: 'Raden Fajar',
    redSchool: 'Tapak Suci Solo',
    redContingent: 'Jawa Tengah',
    redScore: '9.920',
    blueName: 'Ahmad Faiz',
    blueSchool: 'Perisai Diri Bali',
    blueContingent: 'Bali',
    blueScore: '9.890',
    status: 'live',
    statusText: 'Evaluasi Juri',
    timeEst: '11:00 WIB'
  },
  {
    id: 4,
    court: 4,
    partai: 22,
    stage: 'Babak Penyisihan',
    category: 'Usia Dini Putra Kelas Bebas',
    redName: 'Alif Pratama',
    redSchool: 'Pagar Nusa',
    redContingent: 'DI Yogyakarta',
    redScore: 0,
    blueName: 'Rizky Maulana',
    blueSchool: 'KPS Nusantara',
    blueContingent: 'Banten',
    blueScore: 0,
    status: 'calling',
    statusText: 'Panggilan H-3',
    timeEst: '11:15 WIB'
  },
  {
    id: 5,
    court: 1,
    partai: 15,
    stage: 'Semifinal',
    category: 'Dewasa Putra Kelas D (60-65 Kg)',
    redName: 'Dimas Wicaksono',
    redSchool: 'Merpati Putih',
    redContingent: 'Jawa Timur',
    redScore: 0,
    blueName: 'Ferry Ardiansyah',
    blueSchool: 'Tapak Suci',
    blueContingent: 'Sumatera Utara',
    blueScore: 0,
    status: 'ready',
    statusText: 'Siap di Call Room',
    timeEst: '11:30 WIB'
  },
  {
    id: 6,
    court: 2,
    partai: 19,
    stage: 'Semifinal',
    category: 'Dewasa Putri Kelas A (45-50 Kg)',
    redName: 'Anisa Rahmawati',
    redSchool: 'Perisai Diri',
    redContingent: 'DKI Jakarta',
    redScore: 0,
    blueName: 'Putri Handayani',
    blueSchool: 'PSHT',
    blueContingent: 'Jawa Barat',
    blueScore: 0,
    status: 'ready',
    statusText: 'Timbang Lolos',
    timeEst: '11:35 WIB'
  },
  {
    id: 7,
    court: 1,
    partai: 13,
    stage: 'Perempat Final',
    category: 'Dewasa Putra Kelas B (50-55 Kg)',
    redName: 'Reza Fahlevi',
    redSchool: 'Setia Hati',
    redContingent: 'Riau',
    redScore: 31,
    blueName: 'Bayu Segara',
    blueSchool: 'Sinar Putih',
    blueContingent: 'Kalimantan Timur',
    blueScore: 19,
    status: 'finished',
    statusText: 'Selesai (Menang Merah)',
    timeEst: '10:20 WIB'
  }
];

// Data 9 Peran RBAC Lengkap
const ROLES_INFO = {
  superadmin: {
    title: '1. Super Admin Platform',
    icon: '👑',
    scope: 'Seluruh Tenant Platform SaaS',
    device: 'PC Desktop / Laptop (Web Master Console)',
    desc: 'Pemilik infrastruktur cloud SaaS Arena Silat yang bertanggung jawab atas uptime server, billing multi-tenant, dan kepatuhan sistem audit global.',
    allowed: [
      'Manajemen seluruh akun tenant Event Organizer (EO)',
      'Konfigurasi fee platform per atlet & komisi payment gateway',
      'Pemantauan performa server WebSocket, WebRTC, dan Redis',
      'Pemeriksaan log audit keamanan global dan restore database'
    ],
    forbidden: [
      'Mengubah skor pertandingan yang sedang berjalan di venue',
      'Mengintervensi keputusan wasit juri atau dewan hakim di arena'
    ],
    launchLink: 'arena.html'
  },
  eo: {
    title: '2. Event Organizer (EO Panitia)',
    icon: '🏢',
    scope: '1 Event Kejuaraan Miliknya',
    device: 'Laptop / PC (Management Console)',
    desc: 'Penanggung jawab operasional penyelenggaraan kejuaraan dari fase pra-event, hari-H, hingga laporan pertanggungjawaban (LPJ).',
    allowed: [
      'Setup turnamen, aturan IPSI/Puspresnas, dan kuota atlet',
      'Verifikasi pembayaran pendaftaran kontingen (Lunas/Pending)',
      'Generate bagan drawing otomatis (Seeding Separation se-perguruan)',
      'Penjadwalan partai gelanggang dan re-balancing waktu',
      'Cetak massal ID Card peserta dan e-piagam ber-QR Code'
    ],
    forbidden: [
      'Mengintervensi tombol input juri saat ronde aktif',
      'Mengakses data keuangan milik EO lain (proteksi RLS)'
    ],
    launchLink: 'arena.html#bracket'
  },
  td: {
    title: '3. Technical Delegate (TD) / Dewan Hakim IPSI',
    icon: '⚖️',
    scope: 'Pengawasan Teknis Kejuaraan',
    device: 'Tablet 10-12" / Laptop (Approval Console)',
    desc: 'Otoritas tertinggi perwakilan Pengurus IPSI yang memastikan turnamen berjalan sah sesuai regulasi PERSILAT 2026.',
    allowed: [
      'Otorisasi & approval bagan drawing sebelum pertandingan dimulai',
      'Memimpin sidang banding tertutup jika ada protes resmi kontingen',
      'Memeriksa audit trail scoring dan rekaman VAR video review',
      'Otorisasi kebijakan darurat jika terjadi force majeure / molor'
    ],
    forbidden: [
      'Menambah/menghapus atlet peserta tanpa koordinasi sekretariat EO'
    ],
    launchLink: 'arena.html#scoreboard'
  },
  kp: {
    title: '4. Ketua Pertandingan (KP & Meja Gelanggang)',
    icon: '⏱️',
    scope: '1 Gelanggang Matras Tugasnya',
    device: 'Laptop 14-15" + Monitor Meja Gelanggang',
    desc: 'Pengendali operasional laga di gelanggang spesifik, mengontrol waktu timer, mengesahkan poin jatuhan dan sanksi wasit matras.',
    allowed: [
      'Start, Pause, Reset, dan Stop timer ronde pertandingan',
      'Sahkan poin jatuhan bantingan (+3) sudut merah/biru',
      'Input sanksi wasit: Teguran 1/2, Peringatan 1/2 (-5/-10), Binaan',
      'Eksekusi pemutaran Video Review (VAR) slow-motion 0.25x/0.5x',
      'Batalkan poin keliru juri (Undo Score) dan sahkan pemenang'
    ],
    forbidden: [
      'Mengubah jadwal atau skor di luar gelanggang tugasnya'
    ],
    launchLink: 'arena.html#scoreboard'
  },
  jury: {
    title: '5. Wasit Juri (Jury 1 s.d. 5)',
    icon: '📱',
    scope: '1 Perangkat Jury Pad',
    device: 'Smartphone 6-7" / Tablet 8" (PWA Kiosk Mode)',
    desc: 'Memberikan penilaian independen secara real-time atas teknik pukulan, tendangan, bantingan, atau nilai seni TGR.',
    allowed: [
      'Input pukulan (+1), tendangan (+2), jatuhan (+3) via sentuhan',
      'Input nilai kebenaran dan kemantapan gerak seni TGR',
      'Pairing instan ke sesi gelanggang via scan Dynamic QR Code'
    ],
    forbidden: [
      'Melihat total akumulasi skor berjalan (Blind Scoring Protection)',
      'Mengubah timer atau mengintervensi hasil laga'
    ],
    launchLink: 'arena.html#jurypad'
  },
  medical: {
    title: '6. Dokter Pertandingan & Tim Medis',
    icon: '🏥',
    scope: 'Keamanan Fisik & Kesehatan Atlet',
    device: 'Tablet / Smartphone (Medical Console)',
    desc: 'Menjaga keselamatan nyawa pesilat, menangani cedera di atas matras, dan mengevaluasi kelayakan tanding atlet pasca benturan.',
    allowed: [
      'Aktivasi Countdown Recovery Timer kumulatif 3 menit di matras',
      'Keputusan Doctor Stoppage: FIT TO CONTINUE atau UNFIT (WMP)',
      'Input rekam medis cedera dan penerbitan rujukan SJP BPJS',
      'Memberikan status skorsing medis 30 hari pasca K.O benturan kepala'
    ],
    forbidden: [
      'Mengubah nilai skor atau susunan bagan turnamen'
    ],
    launchLink: 'arena.html#medical'
  },
  marshal: {
    title: '7. Petugas Marshalling & Timbang Badan',
    icon: '⚖️',
    scope: 'Sesi Timbang & Call Room',
    device: 'Smartphone / Tablet (Scanner Terminal)',
    desc: 'Memastikan uji timbang resmi akurat dan atlet hadir tepat waktu di matras dengan perlengkapan pelindung lengkap.',
    allowed: [
      'Koneksi Web Bluetooth BLE ke timbangan digital tanpa ketik',
      'Validasi berat aktual terhadap batas kelas (contoh: 55-60 Kg)',
      'Pemanggilan antrean atlet H-3 partai dan notifikasi WhatsApp',
      'Checklist perlengkapan (Body protector, Cup, Kuku, Deker)'
    ],
    forbidden: [
      'Mengubah skor pertandingan atau bagan drawing'
    ],
    launchLink: 'arena.html#weighin'
  },
  coach: {
    title: '8. Manager Kontingen / Pelatih',
    icon: '📋',
    scope: '1 Kontingen Perguruan / Daerah',
    device: 'Smartphone (Mobile Portal Web)',
    desc: 'Mendaftarkan atlet kontingen, mendampingi pesilat di sudut matras, dan memperjuangkan hak atlet melalui saluran resmi.',
    allowed: [
      'Pendaftaran pesilat, upload berkas NIK/BPJS/Surat Sehat',
      'Pembayaran tagihan via QRIS / Virtual Account Bank',
      'Live tracking jadwal tanding seluruh atlet kontingennya',
      'Mengangkat kartu Video Review (VAR) kuota 2 kartu per partai',
      'Download e-piagam juara ber-barcode resmi'
    ],
    forbidden: [
      'Mengakses konsol gelanggang atau touchpad wasit juri'
    ],
    launchLink: 'arena.html#bracket'
  },
  public: {
    title: '9. Publik, Orang Tua & Suporter',
    icon: '👥',
    scope: 'Akses Terbuka (Read-Only)',
    device: 'Smartphone / PC / Smart TV (Public Portal)',
    desc: 'Masyarakat umum dan suporter yang ingin menonton pertandingan, memantau skor langsung, dan memverifikasi sertifikat prestasi.',
    allowed: [
      'Melihat Live Scoreboard multi-gelanggang secara real-time',
      'Menonton live stream siaran laga dengan overlay skor OBS',
      'Mengecek bagan eliminasi dan klasemen medali kontingen',
      'Memindai QR Code untuk memverifikasi keaslian e-sertifikat'
    ],
    forbidden: [
      'Mengirimkan input atau data apa pun ke server pertandingan'
    ],
    launchLink: 'arena.html#multicourt'
  }
};

// Filter State Jadwal
let currentCourtFilter = 'all';
let currentSearchQuery = '';

function renderScheduleTable() {
  const tbody = document.getElementById('scheduleTableBody');
  if (!tbody) return;

  const filtered = SCHEDULE_DATA.filter(item => {
    // Filter Court
    if (currentCourtFilter !== 'all' && item.court.toString() !== currentCourtFilter) {
      return false;
    }
    // Filter Search
    if (currentSearchQuery.trim() !== '') {
      const q = currentSearchQuery.toLowerCase();
      const matchRed = item.redName.toLowerCase().includes(q) || item.redSchool.toLowerCase().includes(q) || item.redContingent.toLowerCase().includes(q);
      const matchBlue = item.blueName.toLowerCase().includes(q) || item.blueSchool.toLowerCase().includes(q) || item.blueContingent.toLowerCase().includes(q);
      const matchCat = item.category.toLowerCase().includes(q);
      if (!matchRed && !matchBlue && !matchCat) return false;
    }
    return true;
  });

  if (filtered.length === 0) {
    tbody.innerHTML = `
      <tr>
        <td colspan="7" style="text-align: center; padding: 30px; color: var(--text-dim);">
          🔍 Tidak ada partai pertandingan yang cocok dengan pencarian Anda.
        </td>
      </tr>
    `;
    return;
  }

  tbody.innerHTML = filtered.map(item => {
    let statusClass = 'ready';
    if (item.status === 'live') statusClass = 'live';
    else if (item.status === 'calling') statusClass = 'calling';
    else if (item.status === 'finished') statusClass = 'finished';

    return `
      <tr>
        <td>
          <span style="font-weight: 800; color: #fff;">Gel. ${item.court}</span>
          <div style="font-size: 11px; color: var(--text-dim);">Partai #${item.partai}</div>
        </td>
        <td>
          <div style="font-weight: 700; color: #fff;">${item.category}</div>
          <div style="font-size: 11px; color: var(--text-muted);">${item.stage}</div>
        </td>
        <td>
          <div style="font-weight: 700; color: var(--red-corner);">🔴 ${item.redName}</div>
          <div style="font-size: 11px; color: var(--text-dim);">${item.redSchool} • ${item.redContingent}</div>
        </td>
        <td>
          <div style="font-weight: 700; color: var(--blue-corner);">🔵 ${item.blueName}</div>
          <div style="font-size: 11px; color: var(--text-dim);">${item.blueSchool} • ${item.blueContingent}</div>
        </td>
        <td>
          <span style="font-family: var(--font-mono); font-weight: 800; font-size: 14px;">
            ${item.redScore} - ${item.blueScore}
          </span>
        </td>
        <td>
          <span class="status-tag ${statusClass}">
            ${item.status === 'live' ? '<span class="pulse-dot"></span>' : ''} ${item.statusText}
          </span>
        </td>
        <td>
          <a href="arena.html" class="btn-sm-action btn-sm-outline" style="padding: 4px 10px; font-size: 11px;">
            👁️ Pantau
          </a>
        </td>
      </tr>
    `;
  }).join('');
}

function filterScheduleByCourt(court, buttonElement) {
  currentCourtFilter = court;
  document.querySelectorAll('.filter-pill').forEach(btn => btn.classList.remove('active'));
  if (buttonElement) buttonElement.classList.add('active');
  renderScheduleTable();
}

function handleScheduleSearch(event) {
  currentSearchQuery = event.target.value;
  renderScheduleTable();
}

// Simulasi Verifikasi E-Sertifikat Kriptografis
function verifyCertificatePublic() {
  const input = document.getElementById('certInputNumber');
  const resultCard = document.getElementById('certResultBox');
  const certId = input ? input.value.trim() : '';

  if (!certId) {
    alert('Silakan masukkan nomor registrasi sertifikat!');
    return;
  }

  // Animasi loading sebentar
  if (resultCard) {
    resultCard.style.display = 'block';
    resultCard.innerHTML = `
      <div style="display: flex; align-items: center; justify-content: center; gap: 10px; padding: 20px;">
        <span class="pulse-dot" style="background: var(--gold);"></span>
        <span>Memverifikasi tanda tangan digital HMAC-SHA256 pada Ledger PB IPSI...</span>
      </div>
    `;

    setTimeout(() => {
      resultCard.innerHTML = `
        <div style="display: flex; align-items: flex-start; justify-content: space-between; flex-wrap: wrap; gap: 16px;">
          <div>
            <div style="display: inline-flex; align-items: center; gap: 6px; background: rgba(16, 185, 129, 0.2); border: 1px solid rgba(16, 185, 129, 0.5); color: #34d399; padding: 4px 12px; border-radius: 9999px; font-size: 12px; font-weight: 800; margin-bottom: 12px;">
              ✅ PIAGAM RESMI & TERVERIFIKASI ASLI (AUTHENTIC)
            </div>
            <h3 style="font-size: 20px; font-weight: 800; color: #fff; margin-bottom: 4px;">Arya Wira Sena</h3>
            <div style="font-size: 14px; color: var(--gold); font-weight: 700; margin-bottom: 6px;">JUARA 1 - MEDALI EMAS</div>
            <div style="font-size: 13px; color: #cbd5e1;">Kelas C Dewasa Putra (55-60 Kg) • Pengprov IPSI DKI Jakarta</div>
            <div style="font-size: 12px; color: var(--text-dim); margin-top: 4px;">Nomor SK: ${certId}</div>
          </div>

          <div style="text-align: right; font-size: 12px; color: var(--text-muted);">
            <div>Puspresnas Status: <strong style="color: #34d399;">TERKURASI SIMT (A)</strong></div>
            <div>Diterbitkan: 21 Oktober 2026</div>
            <div style="font-family: var(--font-mono); font-size: 11px; color: var(--text-dim); margin-top: 6px;">
              SHA-256: 8a4fe2...7c91e0
            </div>
            <a href="arena.html#certificate" class="btn-sm-action btn-sm-primary" style="margin-top: 10px;">
              📜 Buka Lembar Piagam Resmi
            </a>
          </div>
        </div>
      `;
    }, 600);
  }
}

// Buka Modal Detail Role
function openRoleModal(roleKey) {
  const modal = document.getElementById('roleModal');
  const role = ROLES_INFO[roleKey];
  if (!modal || !role) return;

  const titleEl = document.getElementById('roleModalTitle');
  const scopeEl = document.getElementById('roleModalScope');
  const deviceEl = document.getElementById('roleModalDevice');
  const descEl = document.getElementById('roleModalDesc');
  const allowedList = document.getElementById('roleModalAllowedList');
  const forbiddenList = document.getElementById('roleModalForbiddenList');
  const launchBtn = document.getElementById('roleModalLaunchBtn');

  if (titleEl) titleEl.innerHTML = `${role.icon} ${role.title}`;
  if (scopeEl) scopeEl.textContent = role.scope;
  if (deviceEl) deviceEl.textContent = role.device;
  if (descEl) descEl.textContent = role.desc;

  if (allowedList) {
    allowedList.innerHTML = role.allowed.map(item => `<li style="margin-bottom: 6px; color: #cbd5e1;">✅ ${item}</li>`).join('');
  }

  if (forbiddenList) {
    forbiddenList.innerHTML = role.forbidden.map(item => `<li style="margin-bottom: 6px; color: #f87171;">⛔ ${item}</li>`).join('');
  }

  if (launchBtn) {
    launchBtn.href = role.launchLink;
    launchBtn.textContent = `🚀 Buka Workspace / Konsol (${role.title.split('.')[1].trim()})`;
  }

  modal.style.display = 'flex';
}

function closeRoleModal() {
  const modal = document.getElementById('roleModal');
  if (modal) modal.style.display = 'none';
}

// Inisialisasi Saat Halaman Dimuat
document.addEventListener('DOMContentLoaded', () => {
  renderScheduleTable();

  // Close modal when click outside card
  const modal = document.getElementById('roleModal');
  if (modal) {
    modal.addEventListener('click', (e) => {
      if (e.target === modal) closeRoleModal();
    });
  }

  // Smooth scroll for nav anchor links
  document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
      const targetId = this.getAttribute('href');
      if (targetId.length > 1) {
        const targetElement = document.querySelector(targetId);
        if (targetElement) {
          e.preventDefault();
          targetElement.scrollIntoView({ behavior: 'smooth' });
        }
      }
    });
  });
});
