/**
 * avatars.js - Koleksi Avatar Potret Realistik 10 AI Agent Indonesia v1.2.0-GA
 * Menghasilkan avatar vektor definisi tinggi (HD SVG) dengan pencahayaan studio,
 * busana formal korporat, fitur wajah, gaya rambut, dan aksen warna departemen.
 */

const AGENT_AVATARS = {
  // 1. Arya Wicaksana, PMP (Mandor Utama / Chief Orchestrator)
  // Karakter: Pria eksekutif 35 tahun, jas navy formal, dasi sutra biru, rambut pompadour belah samping rapi
  agen_orkestrator: `
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="avatar-svg">
      <defs>
        <radialGradient id="bg-arya" cx="50%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#1e3a8a"/>
          <stop offset="100%" stop-color="#090d16"/>
        </radialGradient>
        <linearGradient id="skin-arya" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#fed7aa"/>
          <stop offset="60%" stop-color="#f59e0b"/>
          <stop offset="100%" stop-color="#d97706"/>
        </linearGradient>
        <linearGradient id="suit-arya" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stop-color="#1e293b"/>
          <stop offset="100%" stop-color="#0f172a"/>
        </linearGradient>
      </defs>
      <!-- Background Circle with Glow -->
      <circle cx="50" cy="50" r="48" fill="url(#bg-arya)" stroke="#3b82f6" stroke-width="2.5"/>
      <!-- Jas Navy Korporat -->
      <path d="M18 100 C20 76, 32 68, 50 68 C68 68, 80 76, 82 100 Z" fill="url(#suit-arya)"/>
      <!-- Kerah Kemeja Putih -->
      <polygon points="40,68 50,82 60,68 56,66 44,66" fill="#f8fafc"/>
      <!-- Dasi Biru Sutra -->
      <polygon points="47,70 53,70 52,88 50,92 48,88" fill="#2563eb"/>
      <!-- Kerah Jas Lapel -->
      <path d="M30 76 L44 68 L47 84 Z" fill="#334155"/>
      <path d="M70 76 L56 68 L53 84 Z" fill="#334155"/>
      <!-- Leher -->
      <rect x="43" y="52" width="14" height="18" rx="4" fill="url(#skin-arya)"/>
      <ellipse cx="50" cy="62" rx="7" ry="3" fill="#b45309" opacity="0.3"/>
      <!-- Kepala / Wajah Oval Pria -->
      <path d="M32 38 C32 25, 68 25, 68 38 C68 55, 60 62, 50 62 C40 62, 32 55, 32 38 Z" fill="url(#skin-arya)"/>
      <!-- Telinga -->
      <ellipse cx="32" cy="40" rx="3.5" ry="6" fill="#f59e0b"/>
      <ellipse cx="68" cy="40" rx="3.5" ry="6" fill="#d97706"/>
      <!-- Rambut Pompadour Rapi Bertekstur -->
      <path d="M30 35 C28 20, 42 12, 58 13 C68 14, 72 22, 70 34 C66 28, 62 26, 52 26 C40 26, 34 30, 30 35 Z" fill="#1c1917"/>
      <path d="M30 35 C32 38, 32 44, 31 46 C29 44, 29 38, 30 35 Z" fill="#1c1917"/>
      <path d="M70 34 C68 38, 68 44, 69 46 C71 44, 71 38, 70 34 Z" fill="#1c1917"/>
      <!-- Alis Pria Tegas -->
      <path d="M37 34 Q43 32 47 34" stroke="#0c0a09" stroke-width="2" stroke-linecap="round" fill="none"/>
      <path d="M53 34 Q57 32 63 34" stroke="#0c0a09" stroke-width="2" stroke-linecap="round" fill="none"/>
      <!-- Mata Tajam & Percaya Diri -->
      <ellipse cx="42" cy="38" rx="3" ry="1.8" fill="#0f172a"/>
      <circle cx="42.8" cy="37.5" r="0.8" fill="#ffffff"/>
      <ellipse cx="58" cy="38" rx="3" ry="1.8" fill="#0f172a"/>
      <circle cx="58.8" cy="37.5" r="0.8" fill="#ffffff"/>
      <!-- Hidung Pria Berkarakter -->
      <path d="M49 39 L48 47 L52 47" stroke="#b45309" stroke-width="1.4" stroke-linecap="round" fill="none"/>
      <!-- Senyum Kepemimpinan Hangat -->
      <path d="M44 53 Q50 57 56 53" stroke="#9a3412" stroke-width="1.6" stroke-linecap="round" fill="none"/>
      <!-- Pin Badge Mandor Utama -->
      <circle cx="34" cy="82" r="3" fill="#eab308" stroke="#fef08a" stroke-width="0.8"/>
    </svg>
  `,

  // 2. Dian Kartika, MBA (Analis Bisnis & Produk)
  // Karakter: Wanita profesional 32 tahun, blazer teal/cyan anggun, blus sutra, kacamata modern chic, rambut bob terurai rapi
  agen_analis: `
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="avatar-svg">
      <defs>
        <radialGradient id="bg-dian" cx="50%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#0891b2"/>
          <stop offset="100%" stop-color="#090d16"/>
        </radialGradient>
        <linearGradient id="skin-dian" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#ffedd5"/>
          <stop offset="60%" stop-color="#fcd34d"/>
          <stop offset="100%" stop-color="#f59e0b"/>
        </linearGradient>
      </defs>
      <circle cx="50" cy="50" r="48" fill="url(#bg-dian)" stroke="#06b6d4" stroke-width="2.5"/>
      <!-- Rambut Belakang Panjang Terurai -->
      <path d="M26 40 C22 65, 26 85, 34 94 C34 94, 66 94, 66 94 C74 85, 78 65, 74 40 Z" fill="#18181b"/>
      <!-- Blazer Cyan / Teal Elegan -->
      <path d="M20 100 C22 76, 32 69, 50 69 C68 69, 78 76, 80 100 Z" fill="#0e7490"/>
      <!-- Blus Silk Putih -->
      <polygon points="42,69 50,84 58,69" fill="#f8fafc"/>
      <path d="M30 76 L44 69 L48 84 Z" fill="#155e75"/>
      <path d="M70 76 L56 69 L52 84 Z" fill="#155e75"/>
      <!-- Leher Ramping -->
      <rect x="44" y="52" width="12" height="19" rx="4" fill="url(#skin-dian)"/>
      <!-- Kepala / Wajah Wanita Tirus -->
      <path d="M34 38 C34 26, 66 26, 66 38 C66 53, 58 61, 50 61 C42 61, 34 53, 34 38 Z" fill="url(#skin-dian)"/>
      <!-- Rambut Bob Depan Bertekstur -->
      <path d="M32 34 C30 18, 45 14, 58 14 C70 15, 71 25, 68 36 C64 24, 56 22, 48 23 C38 24, 34 29, 32 34 Z" fill="#27272a"/>
      <path d="M32 34 C32 46, 36 54, 38 58 C35 52, 33 44, 32 34 Z" fill="#27272a"/>
      <path d="M68 36 C68 46, 64 54, 62 58 C65 52, 67 44, 68 36 Z" fill="#27272a"/>
      <!-- Kacamata Chic Elegan -->
      <rect x="36" y="34" width="11" height="8" rx="2.5" fill="none" stroke="#22d3ee" stroke-width="1.4"/>
      <rect x="53" y="34" width="11" height="8" rx="2.5" fill="none" stroke="#22d3ee" stroke-width="1.4"/>
      <line x1="47" y1="38" x2="53" y2="38" stroke="#22d3ee" stroke-width="1.4"/>
      <!-- Mata Cerdas di Balik Kacamata -->
      <ellipse cx="41.5" cy="38" rx="2.5" ry="1.8" fill="#18181b"/>
      <circle cx="42.2" cy="37.5" r="0.7" fill="#ffffff"/>
      <ellipse cx="58.5" cy="38" rx="2.5" ry="1.8" fill="#18181b"/>
      <circle cx="59.2" cy="37.5" r="0.7" fill="#ffffff"/>
      <!-- Alis Lentik Rapi -->
      <path d="M37 32 Q42 29 46 31" stroke="#27272a" stroke-width="1.6" stroke-linecap="round" fill="none"/>
      <path d="M54 31 Q58 29 63 32" stroke="#27272a" stroke-width="1.6" stroke-linecap="round" fill="none"/>
      <!-- Hidung & Bibir Elegan -->
      <path d="M49 40 L48 47 L51 47" stroke="#d97706" stroke-width="1.2" stroke-linecap="round" fill="none"/>
      <path d="M45 53 Q50 56 55 53" stroke="#e11d48" stroke-width="1.8" stroke-linecap="round" fill="none"/>
    </svg>
  `,

  // 3. Bambang Prasetyo, M.T. (Principal System & Database Architect)
  // Karakter: Pria tech lead 38 tahun, jaket indigo, kacamata titanium, jenggot rapi berkarakter
  agen_arsitek: `
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="avatar-svg">
      <defs>
        <radialGradient id="bg-bambang" cx="50%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#3730a3"/>
          <stop offset="100%" stop-color="#090d16"/>
        </radialGradient>
        <linearGradient id="skin-bambang" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#fed7aa"/>
          <stop offset="60%" stop-color="#f59e0b"/>
          <stop offset="100%" stop-color="#b45309"/>
        </linearGradient>
      </defs>
      <circle cx="50" cy="50" r="48" fill="url(#bg-bambang)" stroke="#6366f1" stroke-width="2.5"/>
      <!-- Jaket Indigo & Turtleneck Hitam -->
      <path d="M18 100 C20 76, 32 68, 50 68 C68 68, 80 76, 82 100 Z" fill="#312e81"/>
      <path d="M41 68 C41 60, 59 60, 59 68 Z" fill="#0f172a"/>
      <!-- Leher & Rahang Kokoh -->
      <rect x="43" y="50" width="14" height="19" rx="4" fill="url(#skin-bambang)"/>
      <path d="M33 38 C33 25, 67 25, 67 38 C67 56, 59 62, 50 62 C41 62, 33 56, 33 38 Z" fill="url(#skin-bambang)"/>
      <!-- Rambut Belah Samping Matang -->
      <path d="M31 35 C30 18, 45 12, 60 13 C69 14, 71 24, 68 34 C64 26, 58 24, 48 24 C38 24, 34 29, 31 35 Z" fill="#292524"/>
      <!-- Jenggot Rapi Tipis (Tech Lead Beard) -->
      <path d="M41 54 C44 58, 56 58, 59 54 C58 60, 54 62, 50 62 C46 62, 42 60, 41 54 Z" fill="#292524" opacity="0.85"/>
      <!-- Kacamata Persegi Titanium -->
      <rect x="36" y="34" width="11" height="7" rx="1.5" fill="none" stroke="#a5b4fc" stroke-width="1.5"/>
      <rect x="53" y="34" width="11" height="7" rx="1.5" fill="none" stroke="#a5b4fc" stroke-width="1.5"/>
      <line x1="47" y1="37" x2="53" y2="37" stroke="#a5b4fc" stroke-width="1.5"/>
      <!-- Mata Visioner Database Architect -->
      <circle cx="41.5" cy="37.5" r="1.8" fill="#0f172a"/>
      <circle cx="42.2" cy="37" r="0.6" fill="#ffffff"/>
      <circle cx="58.5" cy="37.5" r="1.8" fill="#0f172a"/>
      <circle cx="59.2" cy="37" r="0.6" fill="#ffffff"/>
      <!-- Hidung & Garis Senyum Dewasa -->
      <path d="M49 39 L48 46 L52 46" stroke="#9a3412" stroke-width="1.4" stroke-linecap="round" fill="none"/>
      <path d="M45 52 Q50 54 55 52" stroke="#451a03" stroke-width="1.5" stroke-linecap="round" fill="none"/>
    </svg>
  `,

  // 4. Rizky Firmansyah (Lead Software Engineer / Coder)
  // Karakter: Pria muda jenius 28 tahun, jaket tech hitam aksen hijau emerald, headphone melingkar di leher, potongan rambut modern fade
  agen_coder: `
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="avatar-svg">
      <defs>
        <radialGradient id="bg-rizky" cx="50%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#064e3b"/>
          <stop offset="100%" stop-color="#090d16"/>
        </radialGradient>
        <linearGradient id="skin-rizky" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#ffedd5"/>
          <stop offset="60%" stop-color="#fcd34d"/>
          <stop offset="100%" stop-color="#d97706"/>
        </linearGradient>
      </defs>
      <circle cx="50" cy="50" r="48" fill="url(#bg-rizky)" stroke="#10b981" stroke-width="2.5"/>
      <!-- Hoodie / Tech Jacket Charcoal -->
      <path d="M18 100 C20 75, 32 68, 50 68 C68 68, 80 75, 82 100 Z" fill="#1e293b"/>
      <!-- Aksen Ritsleting Hijau Emerald -->
      <line x1="50" y1="72" x2="50" y2="100" stroke="#10b981" stroke-width="2"/>
      <!-- Headphone Nirkabel di Leher -->
      <path d="M30 65 Q50 74 70 65" stroke="#334155" stroke-width="5" fill="none" stroke-linecap="round"/>
      <ellipse cx="30" cy="65" rx="4" ry="7" fill="#10b981"/>
      <ellipse cx="70" cy="65" rx="4" ry="7" fill="#10b981"/>
      <!-- Leher & Wajah -->
      <rect x="44" y="50" width="12" height="19" rx="4" fill="url(#skin-rizky)"/>
      <path d="M33 37 C33 24, 67 24, 67 37 C67 54, 59 61, 50 61 C41 61, 33 54, 33 37 Z" fill="url(#skin-rizky)"/>
      <!-- Gaya Rambut Modern Fade Bertekstur -->
      <path d="M31 34 C30 18, 44 14, 58 14 C70 14, 71 22, 69 32 C65 25, 59 23, 49 23 C39 23, 34 28, 31 34 Z" fill="#0f172a"/>
      <path d="M35 18 L39 12 L43 17 L48 11 L53 17 L57 12 L61 18" stroke="#0f172a" stroke-width="2" fill="none"/>
      <!-- Alis & Sorot Mata Fokus Pemrograman -->
      <path d="M37 33 Q42 30 46 32" stroke="#0f172a" stroke-width="2" stroke-linecap="round" fill="none"/>
      <path d="M54 32 Q58 30 63 33" stroke="#0f172a" stroke-width="2" stroke-linecap="round" fill="none"/>
      <ellipse cx="41.5" cy="37" rx="3" ry="2" fill="#0f172a"/>
      <circle cx="42.3" cy="36.5" r="0.8" fill="#34d399"/>
      <ellipse cx="58.5" cy="37" rx="3" ry="2" fill="#0f172a"/>
      <circle cx="59.3" cy="36.5" r="0.8" fill="#34d399"/>
      <!-- Hidung & Senyum Khas Coder -->
      <path d="M49 39 L48 46 L52 46" stroke="#b45309" stroke-width="1.3" stroke-linecap="round" fill="none"/>
      <path d="M45 52 Q50 56 55 52" stroke="#b45309" stroke-width="1.6" stroke-linecap="round" fill="none"/>
    </svg>
  `,

  // 5. Siti Nurhaliza, S.Kom. (QA Automation Lead)
  // Karakter: Wanita teliti 29 tahun, hijab modern anggun warna pastel, blazer dusty-pink, headset QA
  agen_qa: `
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="avatar-svg">
      <defs>
        <radialGradient id="bg-siti" cx="50%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#831843"/>
          <stop offset="100%" stop-color="#090d16"/>
        </radialGradient>
        <linearGradient id="skin-siti" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#fff7ed"/>
          <stop offset="60%" stop-color="#fed7aa"/>
          <stop offset="100%" stop-color="#f59e0b"/>
        </linearGradient>
      </defs>
      <circle cx="50" cy="50" r="48" fill="url(#bg-siti)" stroke="#ec4899" stroke-width="2.5"/>
      <!-- Blazer Dusty Pink Korporat -->
      <path d="M18 100 C20 76, 32 70, 50 70 C68 70, 80 76, 82 100 Z" fill="#9d174d"/>
      <polygon points="42,70 50,84 58,70" fill="#fbcfe8"/>
      <!-- Hijab Anggun Menutup Leher -->
      <path d="M28 42 C24 64, 32 76, 50 76 C68 76, 76 64, 72 42 C70 20, 30 20, 28 42 Z" fill="#be185d"/>
      <!-- Wajah Bersih Teliti -->
      <path d="M35 40 C35 30, 65 30, 65 40 C65 54, 58 60, 50 60 C42 60, 35 54, 35 40 Z" fill="url(#skin-siti)"/>
      <!-- Headset Mikrofon Pengujian Otomasi -->
      <path d="M32 30 C40 22, 60 22, 68 30" stroke="#f472b6" stroke-width="2.2" fill="none"/>
      <ellipse cx="32" cy="38" rx="2.5" ry="4" fill="#ec4899"/>
      <path d="M32 40 Q36 49 43 49" stroke="#f472b6" stroke-width="1.4" fill="none"/>
      <circle cx="44" cy="49" r="1.5" fill="#f43f5e"/>
      <!-- Alis Halus & Mata Teliti -->
      <path d="M38 34 Q42 32 46 33" stroke="#831843" stroke-width="1.6" stroke-linecap="round" fill="none"/>
      <path d="M54 33 Q58 32 62 34" stroke="#831843" stroke-width="1.6" stroke-linecap="round" fill="none"/>
      <ellipse cx="42" cy="38" rx="2.6" ry="1.9" fill="#0f172a"/>
      <circle cx="42.6" cy="37.5" r="0.7" fill="#ffffff"/>
      <ellipse cx="58" cy="38" rx="2.6" ry="1.9" fill="#0f172a"/>
      <circle cx="58.6" cy="37.5" r="0.7" fill="#ffffff"/>
      <!-- Hidung & Senyum Manis -->
      <path d="M49 41 L48 46 L51 46" stroke="#d97706" stroke-width="1.2" stroke-linecap="round" fill="none"/>
      <path d="M45 52 Q50 55 55 52" stroke="#e11d48" stroke-width="1.6" stroke-linecap="round" fill="none"/>
    </svg>
  `,

  // 6. Fajar Nugroho (DevOps & SRE Specialist)
  // Karakter: Pria praktis 31 tahun, polo shirt abu-abu aksen amber cloud, rambut spike rapi, ekspresi sigap 24/7
  agen_devops: `
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="avatar-svg">
      <defs>
        <radialGradient id="bg-fajar" cx="50%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#78350f"/>
          <stop offset="100%" stop-color="#090d16"/>
        </radialGradient>
        <linearGradient id="skin-fajar" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#fed7aa"/>
          <stop offset="60%" stop-color="#f59e0b"/>
          <stop offset="100%" stop-color="#b45309"/>
        </linearGradient>
      </defs>
      <circle cx="50" cy="50" r="48" fill="url(#bg-fajar)" stroke="#f59e0b" stroke-width="2.5"/>
      <!-- Polo Shirt Dark Charcoal -->
      <path d="M18 100 C20 75, 32 68, 50 68 C68 68, 80 75, 82 100 Z" fill="#1e293b"/>
      <!-- Kerah Polo Shirt -->
      <polygon points="40,68 50,78 60,68 54,65 46,65" fill="#334155"/>
      <circle cx="36" cy="80" r="3" fill="#f59e0b"/> <!-- Badge Cloud SRE -->
      <!-- Leher & Kepala -->
      <rect x="44" y="50" width="12" height="19" rx="4" fill="url(#skin-fajar)"/>
      <path d="M33 37 C33 24, 67 24, 67 37 C67 55, 59 61, 50 61 C41 61, 33 55, 33 37 Z" fill="url(#skin-fajar)"/>
      <!-- Rambut Spiky Rapi DevOps -->
      <path d="M31 34 C30 18, 44 14, 58 14 C69 15, 71 22, 69 32 C65 26, 59 23, 49 23 C39 23, 34 28, 31 34 Z" fill="#1c1917"/>
      <polygon points="44,14 48,8 52,14" fill="#1c1917"/>
      <polygon points="50,14 54,9 58,14" fill="#1c1917"/>
      <!-- Mata Siaga 24/7 Monitoring -->
      <ellipse cx="42" cy="37" rx="3" ry="1.9" fill="#0f172a"/>
      <circle cx="42.7" cy="36.5" r="0.7" fill="#fbbf24"/>
      <ellipse cx="58" cy="37" rx="3" ry="1.9" fill="#0f172a"/>
      <circle cx="58.7" cy="36.5" r="0.7" fill="#fbbf24"/>
      <!-- Alis Tegas -->
      <path d="M37 32 Q42 30 46 32" stroke="#1c1917" stroke-width="1.8" stroke-linecap="round" fill="none"/>
      <path d="M54 32 Q58 30 63 32" stroke="#1c1917" stroke-width="1.8" stroke-linecap="round" fill="none"/>
      <!-- Hidung & Senyum Tangguh -->
      <path d="M49 39 L48 46 L52 46" stroke="#9a3412" stroke-width="1.4" stroke-linecap="round" fill="none"/>
      <path d="M45 52 Q50 56 55 52" stroke="#78350f" stroke-width="1.6" stroke-linecap="round" fill="none"/>
    </svg>
  `,

  // 7. Hendra Gunawan, CISA (Auditor Siber & UU PDP Officer)
  // Karakter: Pria tegas 36 tahun, setelan hitam rapi, pin lencana cyber-shield merah, tatapan tajam mengaudit
  agen_keamanan: `
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="avatar-svg">
      <defs>
        <radialGradient id="bg-hendra" cx="50%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#7f1d1d"/>
          <stop offset="100%" stop-color="#090d16"/>
        </radialGradient>
        <linearGradient id="skin-hendra" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#fed7aa"/>
          <stop offset="60%" stop-color="#f59e0b"/>
          <stop offset="100%" stop-color="#b45309"/>
        </linearGradient>
      </defs>
      <circle cx="50" cy="50" r="48" fill="url(#bg-hendra)" stroke="#ef4444" stroke-width="2.5"/>
      <!-- Setelan Hitam Jas Formal Keamanan -->
      <path d="M18 100 C20 75, 32 68, 50 68 C68 68, 80 75, 82 100 Z" fill="#0f172a"/>
      <polygon points="42,68 50,82 58,68 54,65 46,65" fill="#f8fafc"/>
      <polygon points="48,70 52,70 51,88 50,91 49,88" fill="#dc2626"/>
      <!-- Pin Lencana Cyber Shield UU PDP -->
      <polygon points="34,77 38,77 38,82 36,85 34,82" fill="#ef4444" stroke="#fca5a5" stroke-width="0.7"/>
      <!-- Leher & Kepala Tegas -->
      <rect x="43" y="50" width="14" height="19" rx="4" fill="url(#skin-hendra)"/>
      <path d="M33 37 C33 24, 67 24, 67 37 C67 55, 59 62, 50 62 C41 62, 33 55, 33 37 Z" fill="url(#skin-hendra)"/>
      <!-- Rambut Samping Rapi Disisir Formal -->
      <path d="M31 34 C30 18, 44 13, 59 13 C69 14, 71 22, 69 32 C65 25, 58 24, 48 24 C38 24, 34 29, 31 34 Z" fill="#0c0a09"/>
      <!-- Alis Tegas Berwibawa -->
      <path d="M37 32 Q42 29 46 32" stroke="#0c0a09" stroke-width="2.2" stroke-linecap="round" fill="none"/>
      <path d="M54 32 Q58 29 63 32" stroke="#0c0a09" stroke-width="2.2" stroke-linecap="round" fill="none"/>
      <!-- Mata Tajam Auditor Forensik -->
      <ellipse cx="42" cy="37" rx="3" ry="1.8" fill="#0f172a"/>
      <circle cx="42.6" cy="36.5" r="0.7" fill="#f87171"/>
      <ellipse cx="58" cy="37" rx="3" ry="1.8" fill="#0f172a"/>
      <circle cx="58.6" cy="36.5" r="0.7" fill="#f87171"/>
      <!-- Hidung & Mulut Serius -->
      <path d="M49 39 L48 46 L52 46" stroke="#7f1d1d" stroke-width="1.5" stroke-linecap="round" fill="none"/>
      <line x1="45" y1="53" x2="55" y2="53" stroke="#450a0a" stroke-width="1.8" stroke-linecap="round"/>
    </svg>
  `,

  // 8. Ratna Sari (Pustakawan & Notulis Obsidian Scribe)
  // Karakter: Wanita intelektual 28 tahun, blazer ungu lilac, kacamata bulat baca, rambut terikat anggun
  agen_notulis: `
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="avatar-svg">
      <defs>
        <radialGradient id="bg-anisa" cx="50%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#581c87"/>
          <stop offset="100%" stop-color="#090d16"/>
        </radialGradient>
        <linearGradient id="skin-anisa" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#fff7ed"/>
          <stop offset="60%" stop-color="#fed7aa"/>
          <stop offset="100%" stop-color="#f59e0b"/>
        </linearGradient>
      </defs>
      <circle cx="50" cy="50" r="48" fill="url(#bg-anisa)" stroke="#a855f7" stroke-width="2.5"/>
      <!-- Rambut Panjang Bergelombang Terikat Belakang -->
      <path d="M26 38 C22 65, 26 86, 34 94 C34 94, 66 94, 66 94 C74 86, 78 65, 74 38 Z" fill="#2e1065"/>
      <!-- Blazer Lilac Ungu Elegan -->
      <path d="M18 100 C20 76, 32 70, 50 70 C68 70, 80 76, 82 100 Z" fill="#7c3aed"/>
      <polygon points="42,70 50,83 58,70" fill="#ede9fe"/>
      <!-- Leher & Wajah -->
      <rect x="44" y="52" width="12" height="19" rx="4" fill="url(#skin-anisa)"/>
      <path d="M34 38 C34 26, 66 26, 66 38 C66 53, 58 60, 50 60 C42 60, 34 53, 34 38 Z" fill="url(#skin-anisa)"/>
      <!-- Poni Rambut Halus & Bando -->
      <path d="M32 34 C30 18, 44 14, 58 14 C70 14, 71 24, 68 34 C64 24, 56 22, 48 23 C38 24, 34 28, 32 34 Z" fill="#3b0764"/>
      <!-- Kacamata Bulat Estetik Obsidian Scribe -->
      <circle cx="41.5" cy="37.5" r="5" fill="none" stroke="#c084fc" stroke-width="1.3"/>
      <circle cx="58.5" cy="37.5" r="5" fill="none" stroke="#c084fc" stroke-width="1.3"/>
      <line x1="46.5" y1="37.5" x2="53.5" y2="37.5" stroke="#c084fc" stroke-width="1.3"/>
      <!-- Mata Penuh Rasa Ingin Tahu -->
      <ellipse cx="41.5" cy="37.5" rx="2.4" ry="1.8" fill="#1e1b4b"/>
      <circle cx="42.2" cy="37" r="0.7" fill="#ffffff"/>
      <ellipse cx="58.5" cy="37.5" rx="2.4" ry="1.8" fill="#1e1b4b"/>
      <circle cx="59.2" cy="37" r="0.7" fill="#ffffff"/>
      <!-- Hidung & Senyum Manis -->
      <path d="M49 40 L48 46 L51 46" stroke="#c2410c" stroke-width="1.2" stroke-linecap="round" fill="none"/>
      <path d="M45 52 Q50 55 55 52" stroke="#db2777" stroke-width="1.6" stroke-linecap="round" fill="none"/>
    </svg>
  `,

  // 9. Bayu Pratama (Humas & Telegram Gateway Lead)
  // Karakter: Pria komunikatif 29 tahun, blazer formal, senyum ramah, pin resmi Telegram Gateway
  agen_humas: `
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="avatar-svg">
      <defs>
        <radialGradient id="bg-nadia" cx="50%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#0284c7"/>
          <stop offset="100%" stop-color="#090d16"/>
        </radialGradient>
        <linearGradient id="skin-nadia" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#fff7ed"/>
          <stop offset="60%" stop-color="#fcd34d"/>
          <stop offset="100%" stop-color="#f59e0b"/>
        </linearGradient>
      </defs>
      <circle cx="50" cy="50" r="48" fill="url(#bg-nadia)" stroke="#24A1DE" stroke-width="2.5"/>
      <!-- Rambut Pendek Rapi -->
      <path d="M26 40 C24 64, 28 84, 35 94 C35 94, 65 94, 65 94 C72 84, 76 64, 74 40 Z" fill="#1c1917"/>
      <!-- Blazer Biru Korporat Elegan -->
      <path d="M18 100 C20 76, 32 70, 50 70 C68 70, 80 76, 82 100 Z" fill="#0369a1"/>
      <polygon points="42,70 50,83 58,70" fill="#f0f9ff"/>
      <!-- Leher & Kepala -->
      <rect x="44" y="52" width="12" height="19" rx="4" fill="url(#skin-nadia)"/>
      <path d="M34 38 C34 26, 66 26, 66 38 C66 53, 58 60, 50 60 C42 60, 34 53, 34 38 Z" fill="url(#skin-nadia)"/>
      <!-- Gaya Rambut Pria Korporat -->
      <path d="M32 34 C30 18, 44 14, 58 14 C70 14, 71 24, 68 34 C64 24, 56 22, 48 23 C38 24, 34 28, 32 34 Z" fill="#262626"/>
      <!-- Alis Rapi Melengkung -->
      <path d="M37 32 Q42 29 46 31" stroke="#1c1917" stroke-width="1.6" stroke-linecap="round" fill="none"/>
      <path d="M54 31 Q58 29 63 32" stroke="#1c1917" stroke-width="1.6" stroke-linecap="round" fill="none"/>
      <!-- Mata Ceria Komunikatif -->
      <ellipse cx="41.5" cy="37.5" rx="2.8" ry="1.9" fill="#0f172a"/>
      <circle cx="42.2" cy="37" r="0.8" fill="#38bdf8"/>
      <ellipse cx="58.5" cy="37.5" rx="2.8" ry="1.9" fill="#0f172a"/>
      <circle cx="59.2" cy="37" r="0.8" fill="#38bdf8"/>
      <!-- Hidung & Senyum Ramah -->
      <path d="M49 40 L48 46 L51 46" stroke="#b45309" stroke-width="1.2" stroke-linecap="round" fill="none"/>
      <path d="M44 52 Q50 57 56 52" stroke="#dc2626" stroke-width="1.8" stroke-linecap="round" fill="none"/>
      <!-- Pin Telegram Bot Paper Plane -->
      <circle cx="34" cy="80" r="3.5" fill="#24A1DE" stroke="#ffffff" stroke-width="0.8"/>
      <polygon points="32.3,80.5 35.8,78.8 33.2,81.4" fill="#ffffff"/>
      <polygon points="35.8,78.8 33.6,80.0 33.6,81.2" fill="#e0f2fe"/>
    </svg>
  `,

  // 10. Dewi Lestari, M.Sc. (Analis Data & KPI Intelligence)
  // Karakter: Wanita analis data & KPI, blazer oranye modern, kacamata data scientist
  agen_analis_data: `
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" class="avatar-svg">
      <defs>
        <radialGradient id="bg-reza" cx="50%" cy="30%" r="70%">
          <stop offset="0%" stop-color="#c2410c"/>
          <stop offset="100%" stop-color="#090d16"/>
        </radialGradient>
        <linearGradient id="skin-reza" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stop-color="#fed7aa"/>
          <stop offset="60%" stop-color="#f59e0b"/>
          <stop offset="100%" stop-color="#d97706"/>
        </linearGradient>
        <linearGradient id="suit-reza" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" stop-color="#9a3412"/>
          <stop offset="100%" stop-color="#7c2d12"/>
        </linearGradient>
      </defs>
      <!-- Background Circle -->
      <circle cx="50" cy="50" r="48" fill="url(#bg-reza)" stroke="#f97316" stroke-width="2.5"/>
      <!-- Blazer Oranye Analytics -->
      <path d="M18 100 C20 76, 32 68, 50 68 C68 68, 80 76, 82 100 Z" fill="url(#suit-reza)"/>
      <!-- Kemeja Putih -->
      <polygon points="40,68 50,82 60,68 56,66 44,66" fill="#f8fafc"/>
      <!-- Dasi Abu Slate -->
      <polygon points="47,70 53,70 52,88 50,92 48,88" fill="#475569"/>
      <!-- Kerah Jas Lapel Oranye -->
      <path d="M30 76 L44 68 L47 84 Z" fill="#c2410c"/>
      <path d="M70 76 L56 68 L53 84 Z" fill="#c2410c"/>
      <!-- Leher -->
      <rect x="43" y="52" width="14" height="18" rx="4" fill="url(#skin-reza)"/>
      <!-- Kepala Oval Pria Muda -->
      <path d="M32 38 C32 25, 68 25, 68 38 C68 55, 60 62, 50 62 C40 62, 32 55, 32 38 Z" fill="url(#skin-reza)"/>
      <!-- Telinga -->
      <ellipse cx="32" cy="40" rx="3.5" ry="6" fill="#f59e0b"/>
      <ellipse cx="68" cy="40" rx="3.5" ry="6" fill="#d97706"/>
      <!-- Rambut Undercut Modern -->
      <path d="M30 36 C28 18, 42 11, 60 12 C70 13, 74 22, 70 34 C65 26, 55 22, 48 23 C38 24, 33 30, 30 36 Z" fill="#111827"/>
      <!-- Kacamata Data Scientist Rectangular -->
      <rect x="36" y="34" width="11" height="7" rx="2" fill="none" stroke="#f97316" stroke-width="1.5"/>
      <rect x="53" y="34" width="11" height="7" rx="2" fill="none" stroke="#f97316" stroke-width="1.5"/>
      <line x1="47" y1="37" x2="53" y2="37" stroke="#f97316" stroke-width="1.5"/>
      <line x1="36" y1="37" x2="31" y2="38" stroke="#f97316" stroke-width="1.5"/>
      <line x1="64" y1="37" x2="69" y2="38" stroke="#f97316" stroke-width="1.5"/>
      <!-- Alis Tegas di balik kacamata -->
      <path d="M37 33 Q43 31 47 33" stroke="#111827" stroke-width="1.8" stroke-linecap="round" fill="none"/>
      <path d="M53 33 Q57 31 63 33" stroke="#111827" stroke-width="1.8" stroke-linecap="round" fill="none"/>
      <!-- Mata Fokus di balik kacamata -->
      <ellipse cx="41.5" cy="37.5" rx="2.5" ry="1.6" fill="#0f172a"/>
      <circle cx="42.2" cy="37" r="0.7" fill="#f8fafc"/>
      <ellipse cx="58.5" cy="37.5" rx="2.5" ry="1.6" fill="#0f172a"/>
      <circle cx="59.2" cy="37" r="0.7" fill="#f8fafc"/>
      <!-- Hidung & Senyum Analitik -->
      <path d="M49 40 L48 47 L52 47" stroke="#b45309" stroke-width="1.4" stroke-linecap="round" fill="none"/>
      <path d="M44 53 Q50 57 56 53" stroke="#9a3412" stroke-width="1.6" stroke-linecap="round" fill="none"/>
      <!-- Pin Badge Chart Analytics -->
      <circle cx="34" cy="81" r="3.5" fill="#f97316" stroke="#fed7aa" stroke-width="0.8"/>
      <rect x="32" y="82" width="1.5" height="2.5" fill="#fff"/>
      <rect x="34" y="80.5" width="1.5" height="4" fill="#fff"/>
      <rect x="36" y="79" width="1.5" height="5.5" fill="#fff"/>
    </svg>
  `
};


// Helper menghasilkan SVG Avatar
function getAgentAvatarSvg(agentId, size = 36) {
  const svgRaw = AGENT_AVATARS[agentId] || AGENT_AVATARS.agen_orkestrator;
  return svgRaw.replace('<svg ', `<svg width="${size}" height="${size}" style="vertical-align: middle; display: block; border-radius: 50%;" `);
}

// Ekspor untuk browser dan Node.js
if (typeof window !== 'undefined') {
  window.AGENT_AVATARS = AGENT_AVATARS;
  window.getAgentAvatarSvg = getAgentAvatarSvg;
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { AGENT_AVATARS, getAgentAvatarSvg };
}
