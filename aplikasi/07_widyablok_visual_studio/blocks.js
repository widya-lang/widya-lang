/**
 * WidyaBlok - Block Definitions & Widya-Lang Transpiler
 * Defines available blocks, their UI template, execution opcode, and Widya-Lang code generator.
 */

const BLOCK_DEFINITIONS = {
  // =========================================================================
  // 1. GERAKAN (MOTION) - #4C97FF
  // =========================================================================
  motion_movesteps: {
    category: 'motion',
    type: 'statement',
    label: 'gerak <input type="number" data-arg="STEPS" value="10"> langkah',
    defaultArgs: { STEPS: 10 },
    toWidya: (args) => `sprite.gerak_langkah(${args.STEPS});`,
  },
  motion_turnright: {
    category: 'motion',
    type: 'statement',
    label: 'putar kanan ↷ <input type="number" data-arg="DEGREES" value="15"> derajat',
    defaultArgs: { DEGREES: 15 },
    toWidya: (args) => `sprite.putar_kanan(${args.DEGREES});`,
  },
  motion_turnleft: {
    category: 'motion',
    type: 'statement',
    label: 'putar kiri ↶ <input type="number" data-arg="DEGREES" value="15"> derajat',
    defaultArgs: { DEGREES: 15 },
    toWidya: (args) => `sprite.putar_kiri(${args.DEGREES});`,
  },
  motion_gotoxy: {
    category: 'motion',
    type: 'statement',
    label: 'pergi ke x: <input type="number" data-arg="X" value="0"> y: <input type="number" data-arg="Y" value="0">',
    defaultArgs: { X: 0, Y: 0 },
    toWidya: (args) => `sprite.atur_posisi(${args.X}, ${args.Y});`,
  },
  motion_pointindirection: {
    category: 'motion',
    type: 'statement',
    label: 'mengarah ke arah <input type="number" data-arg="DIR" value="90">°',
    defaultArgs: { DIR: 90 },
    toWidya: (args) => `sprite.atur_arah(${args.DIR});`,
  },
  motion_ifonedgebounce: {
    category: 'motion',
    type: 'statement',
    label: 'jika di pinggir, pantulkan',
    defaultArgs: {},
    toWidya: () => `sprite.pantul_jika_di_pinggir();`,
  },
  motion_changexby: {
    category: 'motion',
    type: 'statement',
    label: 'ubah x sebesar <input type="number" data-arg="DX" value="10">',
    defaultArgs: { DX: 10 },
    toWidya: (args) => `sprite.ubah_x(${args.DX});`,
  },
  motion_changeyby: {
    category: 'motion',
    type: 'statement',
    label: 'ubah y sebesar <input type="number" data-arg="DY" value="10">',
    defaultArgs: { DY: 10 },
    toWidya: (args) => `sprite.ubah_y(${args.DY});`,
  },

  // =========================================================================
  // 2. TAMPILAN (LOOKS) - #9966FF
  // =========================================================================
  looks_sayforsecs: {
    category: 'looks',
    type: 'statement',
    label: 'katakan <input type="text" data-arg="TEXT" value="Halo Dunia!"> selama <input type="number" data-arg="SECS" value="2"> detik',
    defaultArgs: { TEXT: 'Halo Dunia!', SECS: 2 },
    toWidya: (args) => `sprite.katakan("${args.TEXT}", ${args.SECS});`,
  },
  looks_say: {
    category: 'looks',
    type: 'statement',
    label: 'katakan <input type="text" data-arg="TEXT" value="Halo! Widya-Lang">',
    defaultArgs: { TEXT: 'Halo! Widya-Lang' },
    toWidya: (args) => `sprite.katakan("${args.TEXT}");`,
  },
  looks_thinkforsecs: {
    category: 'looks',
    type: 'statement',
    label: 'pikirkan <input type="text" data-arg="TEXT" value="Hmm..."> selama <input type="number" data-arg="SECS" value="2"> detik',
    defaultArgs: { TEXT: 'Hmm...', SECS: 2 },
    toWidya: (args) => `sprite.pikirkan("${args.TEXT}", ${args.SECS});`,
  },
  looks_changesizeby: {
    category: 'looks',
    type: 'statement',
    label: 'ubah ukuran sebesar <input type="number" data-arg="CHANGE" value="10">%',
    defaultArgs: { CHANGE: 10 },
    toWidya: (args) => `sprite.ubah_ukuran(${args.CHANGE});`,
  },
  looks_setsizeto: {
    category: 'looks',
    type: 'statement',
    label: 'atur ukuran ke <input type="number" data-arg="SIZE" value="100">%',
    defaultArgs: { SIZE: 100 },
    toWidya: (args) => `sprite.atur_ukuran(${args.SIZE});`,
  },
  looks_show: {
    category: 'looks',
    type: 'statement',
    label: 'tampilkan',
    defaultArgs: {},
    toWidya: () => `sprite.tampilkan();`,
  },
  looks_hide: {
    category: 'looks',
    type: 'statement',
    label: 'sembunyikan',
    defaultArgs: {},
    toWidya: () => `sprite.sembunyikan();`,
  },

  // =========================================================================
  // 3. SUARA (SOUND) - #CF63CF
  // =========================================================================
  sound_playnote: {
    category: 'sound',
    type: 'statement',
    label: 'mainkan nada <input type="number" data-arg="NOTE" value="60"> selama <input type="number" data-arg="BEATS" value="0.5"> ketukan',
    defaultArgs: { NOTE: 60, BEATS: 0.5 },
    toWidya: (args) => `audio.mainkan_nada(${args.NOTE}, ${args.BEATS});`,
  },
  sound_playdrum: {
    category: 'sound',
    type: 'statement',
    label: 'bunyikan perkusi <select data-arg="DRUM"><option value="kendang">Kendang</option><option value="gong">Gong</option><option value="snare">Snare</option><option value="hihat">Hi-Hat</option></select>',
    defaultArgs: { DRUM: 'kendang' },
    toWidya: (args) => `audio.bunyikan_perkusi("${args.DRUM}");`,
  },
  sound_playsound: {
    category: 'sound',
    type: 'statement',
    label: 'mainkan suara <select data-arg="SOUND"><option value="meow">Meow</option><option value="pop">Pop</option><option value="laser">Laser</option><option value="menang">Menang</option><option value="koin">Koin</option></select>',
    defaultArgs: { SOUND: 'pop' },
    toWidya: (args) => `audio.mainkan_efek("${args.SOUND}");`,
  },
  sound_setvolumeto: {
    category: 'sound',
    type: 'statement',
    label: 'atur volume ke <input type="number" data-arg="VOL" value="100">%',
    defaultArgs: { VOL: 100 },
    toWidya: (args) => `audio.atur_volume(${args.VOL});`,
  },

  // =========================================================================
  // 4. KEJADIAN (EVENTS) - #FFBF00
  // =========================================================================
  event_whenflagclicked: {
    category: 'events',
    type: 'hat',
    label: '🚩 ketika bendera hijau diklik',
    defaultArgs: {},
    toWidya: () => `// Event: Ketika Bendera Hijau Diklik\nfungsi ketika_mulai()`,
  },
  event_whenkeypressed: {
    category: 'events',
    type: 'hat',
    label: '⌨️ ketika tombol <select data-arg="KEY"><option value="spasi">Spasi</option><option value="panah_atas">Panah Atas</option><option value="panah_bawah">Panah Bawah</option><option value="panah_kanan">Panah Kanan</option><option value="panah_kiri">Panah Kiri</option><option value="a">a</option><option value="d">d</option><option value="w">w</option><option value="s">s</option></select> ditekan',
    defaultArgs: { KEY: 'spasi' },
    toWidya: (args) => `// Event: Tombol ${args.KEY} ditekan\nfungsi ketika_tombol_${args.KEY}_ditekan()`,
  },
  event_whenthisspriteclicked: {
    category: 'events',
    type: 'hat',
    label: '👆 ketika sprite ini diklik',
    defaultArgs: {},
    toWidya: () => `// Event: Sprite diklik\nfungsi ketika_sprite_diklik()`,
  },
  event_broadcast: {
    category: 'events',
    type: 'statement',
    label: '📢 siarkan pesan <input type="text" data-arg="MSG" value="mulai_game">',
    defaultArgs: { MSG: 'mulai_game' },
    toWidya: (args) => `sistem.siarkan_pesan("${args.MSG}");`,
  },

  // =========================================================================
  // 5. KONTROL (CONTROL) - #FFAB19
  // =========================================================================
  control_wait: {
    category: 'control',
    type: 'statement',
    label: '⏳ tunggu <input type="number" data-arg="DURATION" value="1"> detik',
    defaultArgs: { DURATION: 1 },
    toWidya: (args) => `tunggu(${args.DURATION});`,
  },
  control_repeat: {
    category: 'control',
    type: 'c_block',
    label: '🔁 ulang <input type="number" data-arg="TIMES" value="10"> kali',
    defaultArgs: { TIMES: 10 },
    toWidya: (args, innerCode) => `untuk _langkah dalam 1..${args.TIMES} {\n${innerCode}\n}`,
  },
  control_forever: {
    category: 'control',
    type: 'c_block',
    label: '♾️ selamanya',
    defaultArgs: {},
    toWidya: (args, innerCode) => `selama (benar) {\n${innerCode}\n}`,
  },
  control_if: {
    category: 'control',
    type: 'c_block',
    label: '❓ jika <input type="text" data-arg="COND" value="menyentuh_tepi"> maka',
    defaultArgs: { COND: 'menyentuh_tepi' },
    toWidya: (args, innerCode) => `jika (${args.COND}) {\n${innerCode}\n}`,
  },
  control_stop: {
    category: 'control',
    type: 'statement',
    label: '🛑 hentikan semua',
    defaultArgs: {},
    toWidya: () => `keluar;`,
  },

  // =========================================================================
  // 6. SENSOR (SENSING) - #5CB1D6
  // =========================================================================
  sensing_touchingobject: {
    category: 'sensing',
    type: 'statement',
    label: 'sensor periksa menyentuh <select data-arg="TARGET"><option value="pinggir">pinggir</option><option value="mouse">penunjuk mouse</option><option value="sprite_lain">sprite lain</option></select>',
    defaultArgs: { TARGET: 'pinggir' },
    toWidya: (args) => `misal sentuh = sensor.menyentuh("${args.TARGET}");`,
  },
  sensing_askandwait: {
    category: 'sensing',
    type: 'statement',
    label: '💬 tanya <input type="text" data-arg="QUESTION" value="Siapa namamu?"> dan tunggu',
    defaultArgs: { QUESTION: 'Siapa namamu?' },
    toWidya: (args) => `misal jawaban = sensor.tanya_pengguna("${args.QUESTION}");`,
  },
  sensing_reset_timer: {
    category: 'sensing',
    type: 'statement',
    label: '⏱️ atur ulang penghitung waktu',
    defaultArgs: {},
    toWidya: () => `waktu.atur_ulang();`,
  },

  // =========================================================================
  // 7. OPERATOR & MATEMATIKA (OPERATORS) - #59C059
  // =========================================================================
  operator_random: {
    category: 'operators',
    type: 'statement',
    label: 'tetapkan nilai acak dari <input type="number" data-arg="MIN" value="1"> hingga <input type="number" data-arg="MAX" value="10">',
    defaultArgs: { MIN: 1, MAX: 10 },
    toWidya: (args) => `misal acak = matematika.acak_rentang(${args.MIN}, ${args.MAX});`,
  },
  operator_join: {
    category: 'operators',
    type: 'statement',
    label: 'gabungkan teks <input type="text" data-arg="STR1" value="Halo "> dan <input type="text" data-arg="STR2" value="Widya">',
    defaultArgs: { STR1: 'Halo ', STR2: 'Widya' },
    toWidya: (args) => `misal gabung = "${args.STR1}" + "${args.STR2}";`,
  },

  // =========================================================================
  // 8. VARIABEL (VARIABLES) - #FF8C1A
  // =========================================================================
  data_setvariableto: {
    category: 'variables',
    type: 'statement',
    label: 'tetapkan <input type="text" data-arg="VAR" value="skor"> = <input type="text" data-arg="VALUE" value="0">',
    defaultArgs: { VAR: 'skor', VALUE: '0' },
    toWidya: (args) => `tetapkan ${args.VAR} = ${args.VALUE};`,
  },
  data_changevariableby: {
    category: 'variables',
    type: 'statement',
    label: 'ubah <input type="text" data-arg="VAR" value="skor"> sebesar <input type="number" data-arg="VALUE" value="1">',
    defaultArgs: { VAR: 'skor', VALUE: 1 },
    toWidya: (args) => `tetapkan ${args.VAR} = ${args.VAR} + ${args.VALUE};`,
  },

  // =========================================================================
  // 9. PENA SENI & GRAFIKA (PEN) - #0FBD8C
  // =========================================================================
  pen_clear: {
    category: 'pen',
    type: 'statement',
    label: '🧹 hapus semua coretan pena',
    defaultArgs: {},
    toWidya: () => `pena.hapus_semua();`,
  },
  pen_pendown: {
    category: 'pen',
    type: 'statement',
    label: '✏️ turunkan pena (mulai gambar)',
    defaultArgs: {},
    toWidya: () => `pena.turunkan();`,
  },
  pen_penup: {
    category: 'pen',
    type: 'statement',
    label: '🖊️ angkat pena (berhenti gambar)',
    defaultArgs: {},
    toWidya: () => `pena.angkat();`,
  },
  pen_setpencolorto: {
    category: 'pen',
    type: 'statement',
    label: '🎨 atur warna pena ke <input type="color" data-arg="COLOR" value="#EF4444">',
    defaultArgs: { COLOR: '#EF4444' },
    toWidya: (args) => `pena.atur_warna("${args.COLOR}");`,
  },
  pen_setpensizeto: {
    category: 'pen',
    type: 'statement',
    label: '📏 atur ketebalan pena ke <input type="number" data-arg="SIZE" value="3">',
    defaultArgs: { SIZE: 3 },
    toWidya: (args) => `pena.atur_ukuran(${args.SIZE});`,
  },

  // =========================================================================
  // 10. WIDYA KHUSUS (WIDYA SPECIAL) - #EF4444
  // =========================================================================
  widya_print: {
    category: 'widya',
    type: 'statement',
    label: '🖨️ cetak ke konsol <input type="text" data-arg="TEXT" value="Widya Sovereign Engine">',
    defaultArgs: { TEXT: 'Widya Sovereign Engine' },
    toWidya: (args) => `cetak("${args.TEXT}");`,
  },
  widya_aksara: {
    category: 'widya',
    type: 'statement',
    label: '📜 transliterasi aksara Jawa <input type="text" data-arg="TEXT" value="Nusantara">',
    defaultArgs: { TEXT: 'Nusantara' },
    toWidya: (args) => `misal aksara = aksara_jawa("${args.TEXT}");\ncetak("Aksara: " + aksara);`,
  },
  widya_crypto_hash: {
    category: 'widya',
    type: 'statement',
    label: '🔐 hitung Hash SHA256 <input type="text" data-arg="TEXT" value="WidyaSecurity">',
    defaultArgs: { TEXT: 'WidyaSecurity' },
    toWidya: (args) => `misal hash = sha256("${args.TEXT}");\ncetak("Hash: " + hash);`,
  },
};

/**
 * Transpiles an array of stack nodes into clean, idiomatic Widya-Lang (.wya) code.
 */
function transpileStacksToWidya(stacks) {
  let lines = [
    '// ==============================================================================',
    '// Program Widya-Lang (Dihasilkan Otomatis oleh WidyaBlok Visual Studio)',
    '// ==============================================================================',
    '',
    '// Variabel Global',
    'misal skor = 0;',
    'misal waktu_mulai = 0;',
    '',
  ];

  if (!stacks || stacks.length === 0) {
    lines.push('// Belum ada blok di papan kerja.');
    return lines.join('\n');
  }

  stacks.forEach((stack, sIdx) => {
    lines.push(`// --- Rangkaian Blok #${sIdx + 1} ---`);
    stack.forEach(block => {
      lines.push(transpileBlock(block, ''));
    });
    lines.push('');
  });

  lines.push('// Jalankan loop utama engine');
  lines.push('cetak("Program WidyaBlok selesai dieksekusi.");');

  return lines.join('\n');
}

function transpileBlock(block, indent = '') {
  const def = BLOCK_DEFINITIONS[block.opcode];
  if (!def) return `${indent}// Blok tidak dikenal: ${block.opcode}`;

  if (def.type === 'c_block') {
    let innerCode = '';
    if (block.children && block.children.length > 0) {
      innerCode = block.children.map(ch => transpileBlock(ch, indent + '    ')).join('\n');
    } else {
      innerCode = `${indent}    // (badan blok kosong)`;
    }
    const code = def.toWidya(block.args, innerCode);
    return code.split('\n').map(l => indent + l).join('\n');
  } else {
    const code = def.toWidya(block.args);
    return indent + code;
  }
}

// =============================================================================
// PREBUILT TEMPLATE EXAMPLES
// =============================================================================
const PREBUILT_EXAMPLES = {
  animasi_kucing: {
    title: 'Animasi Tari & Sapa Kucing Widya',
    backdrop: 'panggung',
    sprites: [
      {
        id: 'sprite-1',
        name: 'KucingWidya',
        type: 'kucing',
        x: -50,
        y: 0,
        size: 100,
        direction: 90,
        visible: true,
        stacks: [
          [
            { opcode: 'event_whenflagclicked', args: {} },
            { opcode: 'widya_print', args: { TEXT: 'Memulai Pertunjukan Tari Kucing!' } },
            { opcode: 'looks_sayforsecs', args: { TEXT: 'Halo Teman-teman Nusantara!', SECS: 2 } },
            { opcode: 'sound_playsound', args: { SOUND: 'meow' } },
            {
              opcode: 'control_repeat',
              args: { TIMES: 12 },
              children: [
                { opcode: 'motion_movesteps', args: { STEPS: 20 } },
                { opcode: 'motion_turnright', args: { DEGREES: 30 } },
                { opcode: 'sound_playnote', args: { NOTE: 64, BEATS: 0.2 } },
                { opcode: 'motion_ifonedgebounce', args: {} },
                { opcode: 'control_wait', args: { DURATION: 0.15 } },
              ]
            },
            { opcode: 'looks_sayforsecs', args: { TEXT: 'Hebat! Sekarang giliranmu!', SECS: 2 } },
          ]
        ]
      }
    ]
  },

  tangkap_apel: {
    title: 'Game Tangkap Apel Skor Merah Putih',
    backdrop: 'alam',
    sprites: [
      {
        id: 'sprite-1',
        name: 'KeranjangGaruda',
        type: 'garuda',
        x: 0,
        y: -120,
        size: 90,
        direction: 90,
        visible: true,
        stacks: [
          [
            { opcode: 'event_whenflagclicked', args: {} },
            { opcode: 'data_setvariableto', args: { VAR: 'skor', VALUE: '0' } },
            { opcode: 'looks_sayforsecs', args: { TEXT: 'Gunakan tombol Panah / A-D untuk bergerak!', SECS: 2 } },
            {
              opcode: 'control_forever',
              args: {},
              children: [
                { opcode: 'motion_changexby', args: { DX: 5 } },
                { opcode: 'motion_ifonedgebounce', args: {} },
                { opcode: 'control_wait', args: { DURATION: 0.05 } }
              ]
            }
          ]
        ]
      },
      {
        id: 'sprite-2',
        name: 'ApelJatuh',
        type: 'apel',
        x: 0,
        y: 120,
        size: 80,
        direction: 90,
        visible: true,
        stacks: [
          [
            { opcode: 'event_whenflagclicked', args: {} },
            {
              opcode: 'control_forever',
              args: {},
              children: [
                { opcode: 'motion_changeyby', args: { DY: -8 } },
                { opcode: 'control_wait', args: { DURATION: 0.04 } },
                {
                  opcode: 'control_if',
                  args: { COND: 'menyentuh_tepi' },
                  children: [
                    { opcode: 'motion_gotoxy', args: { X: 0, Y: 130 } },
                    { opcode: 'data_changevariableby', args: { VAR: 'skor', VALUE: 1 } },
                    { opcode: 'sound_playsound', args: { SOUND: 'koin' } }
                  ]
                }
              ]
            }
          ]
        ]
      }
    ]
  },

  geometri_fraktal: {
    title: 'Seni Geometri Bintang & Pena Fraktal',
    backdrop: 'polos_gelap',
    sprites: [
      {
        id: 'sprite-1',
        name: 'PenaCerdas',
        type: 'bintang',
        x: 0,
        y: 0,
        size: 70,
        direction: 90,
        visible: true,
        stacks: [
          [
            { opcode: 'event_whenflagclicked', args: {} },
            { opcode: 'pen_clear', args: {} },
            { opcode: 'pen_setpensizeto', args: { SIZE: 3 } },
            { opcode: 'pen_setpencolorto', args: { COLOR: '#F59E0B' } },
            { opcode: 'pen_pendown', args: {} },
            {
              opcode: 'control_repeat',
              args: { TIMES: 24 },
              children: [
                { opcode: 'motion_movesteps', args: { STEPS: 70 } },
                { opcode: 'motion_turnright', args: { DEGREES: 144 } },
                { opcode: 'sound_playnote', args: { NOTE: 60, BEATS: 0.1 } },
                { opcode: 'motion_turnleft', args: { DEGREES: 15 } },
                { opcode: 'control_wait', args: { DURATION: 0.08 } }
              ]
            },
            { opcode: 'pen_penup', args: {} },
            { opcode: 'looks_sayforsecs', args: { TEXT: 'Mahakarya Seni Selesai!', SECS: 2 } }
          ]
        ]
      }
    ]
  },

  piano_musik: {
    title: 'Sintesis Melodi Musik Gamelan & Piano',
    backdrop: 'panggung',
    sprites: [
      {
        id: 'sprite-1',
        name: 'MusisiWidya',
        type: 'burung',
        x: 0,
        y: 0,
        size: 100,
        direction: 90,
        visible: true,
        stacks: [
          [
            { opcode: 'event_whenflagclicked', args: {} },
            { opcode: 'widya_print', args: { TEXT: 'Memainkan Melodi Harmoni Nusantara...' } },
            { opcode: 'sound_playnote', args: { NOTE: 60, BEATS: 0.4 } }, // C
            { opcode: 'sound_playnote', args: { NOTE: 62, BEATS: 0.4 } }, // D
            { opcode: 'sound_playnote', args: { NOTE: 64, BEATS: 0.4 } }, // E
            { opcode: 'sound_playnote', args: { NOTE: 67, BEATS: 0.6 } }, // G
            { opcode: 'sound_playnote', args: { NOTE: 69, BEATS: 0.6 } }, // A
            { opcode: 'sound_playdrum', args: { DRUM: 'gong' } },
            { opcode: 'looks_sayforsecs', args: { TEXT: 'Harmoni Melodi Indah!', SECS: 2 } }
          ]
        ]
      }
    ]
  },

  robot_labirin: {
    title: 'Logika Navigasi Robot & Sensor AI',
    backdrop: 'grid',
    sprites: [
      {
        id: 'sprite-1',
        name: 'RobotPintar',
        type: 'robot',
        x: -150,
        y: -100,
        size: 90,
        direction: 0,
        visible: true,
        stacks: [
          [
            { opcode: 'event_whenflagclicked', args: {} },
            { opcode: 'widya_crypto_hash', args: { TEXT: 'SistemOtonom' } },
            {
              opcode: 'control_repeat',
              args: { TIMES: 4 },
              children: [
                { opcode: 'motion_movesteps', args: { STEPS: 60 } },
                { opcode: 'sound_playsound', args: { SOUND: 'laser' } },
                { opcode: 'control_wait', args: { DURATION: 0.3 } },
                { opcode: 'motion_turnright', args: { DEGREES: 90 } },
                { opcode: 'looks_sayforsecs', args: { TEXT: 'Rotasi Sektor 90°', SECS: 1 } }
              ]
            },
            { opcode: 'looks_sayforsecs', args: { TEXT: 'Misi Patroli Selesai!', SECS: 2 } }
          ]
        ]
      }
    ]
  }
};
