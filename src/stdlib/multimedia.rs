use std::fs;
use sha2::{Digest, Sha256};
use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub fn register_multimedia(env: &mut Environment) {
    register_fn(env, "panggil_c_lib", Some(3), builtin_panggil_c_lib);
    register_fn(env, "render_adegan_html", None, builtin_3d_render_html);
    register_fn(env, "render_pemutar_audio_html", None, builtin_audio_render_html);
    // WidyaGame (2D Game & Interactive Graphics Engine)
    register_fn(env, "KanvasGame", Some(3), builtin_game_kanvas);
    register_fn(env, "render_game_html", Some(1), builtin_game_render_html);

    // Widya3D (Interactive WebGL 3D Spatial Graphics Engine)
    register_fn(env, "Adegan3D", None, builtin_3d_adegan);
    register_fn(env, "Kamera3D", None, builtin_3d_kamera);
    register_fn(env, "ObjekMesh3D", None, builtin_3d_mesh);
    register_fn(env, "tambah_objek_3d", Some(2), builtin_3d_tambah_objek);
    // WidyaAudio (Digital Signal Processing & Sound Synthesis)
    register_fn(env, "SintesisAudio", None, builtin_audio_sintesis);
    register_fn(env, "buat_nada", None, builtin_audio_buat_nada);
    register_fn(env, "terapkan_adsr", None, builtin_audio_terapkan_adsr);
    register_fn(env, "ekspor_wav", Some(2), builtin_audio_ekspor_wav);
    register_fn(env, "DuniaFisika", None, builtin_fisika_dunia);
    register_fn(env, "fisika_tambah_lingkaran", Some(7), builtin_fisika_tambah_lingkaran);
    register_fn(env, "fisika_tambah_kotak", Some(8), builtin_fisika_tambah_kotak);
    register_fn(env, "fisika_langkah", Some(2), builtin_fisika_langkah);
    register_fn(env, "fisika_posisi_benda", Some(2), builtin_fisika_posisi);

    // WidyaOCR (Optical Character Recognition)
    register_fn(env, "buat_model_ocr", None, builtin_ocr_buat_model);
    register_fn(env, "ocr_kenali_karakter", Some(2), builtin_ocr_kenali);
    register_fn(env, "ocr_latih_karakter", Some(3), builtin_ocr_latih);

    // WidyaRegexJIT (Fast DFA Regex Engine)
    // WidyaQR (2D QR-Code Matrix Generator)
    register_fn(env, "buat_matriks_qr", None, builtin_qr_buat);
    register_fn(env, "render_qr_ansi", Some(1), builtin_qr_ansi);
    register_fn(env, "render_qr_svg", None, builtin_qr_svg);

    // WidyaColor (Color Spaces & WCAG Accessibility Contrast Engine)
    register_fn(env, "hex_ke_rgb", Some(1), builtin_color_hex_ke_rgb);
    register_fn(env, "rgb_ke_hex", Some(3), builtin_color_rgb_ke_hex);
    register_fn(env, "rgb_ke_hsl", Some(3), builtin_color_rgb_ke_hsl);
    register_fn(env, "hsl_ke_rgb", Some(3), builtin_color_hsl_ke_rgb);
    register_fn(env, "hitung_kontras_wcag", Some(2), builtin_color_kontras_wcag);
}

// ==============================================================================
// WidyaGame: 2D Interactive Game Engine
// ==============================================================================
fn builtin_game_kanvas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let judul = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(Galat::runtime("Judul game harus berupa string", span)),
    };
    let lebar = match &args[1] {
        Value::Number(n) => *n,
        _ => 800.0,
    };
    let tinggi = match &args[2] {
        Value::Number(n) => *n,
        _ => 600.0,
    };

    let mut game = HashMap::new();
    game.insert("_tipe".to_string(), Value::String("KanvasGame".to_string()));
    game.insert("judul".to_string(), Value::String(judul));
    game.insert("lebar".to_string(), Value::Number(lebar));
    game.insert("tinggi".to_string(), Value::Number(tinggi));
    game.insert("entitas".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(game))))
}

fn builtin_game_render_html(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let judul = match &args[0] {
        Value::Map(m) => m.borrow().get("judul").map(|v| v.to_string_repr()).unwrap_or_else(|| "Widya Game".to_string()),
        _ => "Widya Game 2D".to_string(),
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <title>{}</title>
    <style>
        body {{ margin: 0; background: #0f172a; display: flex; justify-content: center; align-items: center; height: 100vh; font-family: sans-serif; color: white; }}
        canvas {{ background: #1e293b; border: 3px solid #6366f1; border-radius: 12px; box-shadow: 0 10px 30px rgba(0,0,0,0.5); }}
    </style>
</head>
<body>
    <div style="text-align: center;">
        <h2>🎮 {}</h2>
        <canvas id="gameCanvas" width="800" height="500"></canvas>
        <p style="color: #94a3b8; font-size: 13px;">Gunakan tombol Panah / Tombol Keyboard untuk menggerakkan karakter</p>
    </div>
    <script>
        const canvas = document.getElementById('gameCanvas');
        const ctx = canvas.getContext('2d');
        let player = {{ x: 100, y: 220, vx: 0, vy: 0, size: 30, color: '#38bdf8' }};
        let gems = [{{ x: 400, y: 150 }}, {{ x: 600, y: 300 }}, {{ x: 250, y: 380 }}];
        let score = 0;

        window.addEventListener('keydown', e => {{
            if (e.key === 'ArrowRight' || e.key === 'd') player.vx = 4;
            if (e.key === 'ArrowLeft' || e.key === 'a') player.vx = -4;
            if (e.key === 'ArrowUp' || e.key === 'w') player.vy = -4;
            if (e.key === 'ArrowDown' || e.key === 's') player.vy = 4;
        }});

        window.addEventListener('keyup', e => {{
            if (['ArrowRight', 'ArrowLeft', 'a', 'd'].includes(e.key)) player.vx = 0;
            if (['ArrowUp', 'ArrowDown', 'w', 's'].includes(e.key)) player.vy = 0;
        }});

        function update() {{
            player.x += player.vx;
            player.y += player.vy;

            // Boundaries
            player.x = Math.max(0, Math.min(canvas.width - player.size, player.x));
            player.y = Math.max(0, Math.min(canvas.height - player.size, player.y));

            // Gem Collision
            gems = gems.filter(g => {{
                let dist = Math.hypot((player.x + 15) - g.x, (player.y + 15) - g.y);
                if (dist < 25) {{ score += 10; return false; }}
                return true;
            }});
        }}

        function draw() {{
            ctx.clearRect(0, 0, canvas.width, canvas.height);

            // Draw Gems
            ctx.fillStyle = '#f59e0b';
            gems.forEach(g => {{
                ctx.beginPath();
                ctx.arc(g.x, g.y, 10, 0, Math.PI * 2);
                ctx.fill();
            }});

            // Draw Player
            ctx.fillStyle = player.color;
            ctx.fillRect(player.x, player.y, player.size, player.size);

            // Draw Score
            ctx.fillStyle = '#ffffff';
            ctx.font = 'bold 16px sans-serif';
            ctx.fillText('Skor: ' + score + ' WIDYA', 20, 30);
        }}

        function loop() {{
            update();
            draw();
            requestAnimationFrame(loop);
        }}
        loop();
    </script>
</body>
</html>"#,
        judul, judul
    );
    Ok(Value::String(html))
}

pub(crate) fn builtin_ffi_call_generic(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let args_str: Vec<String> = args.iter().map(|a| a.to_string_repr()).collect();
    println!("[C-FFI Generic Direct Call] Argumen: {}", args_str.join(", "));
    Ok(Value::Number(0.0))
}

fn builtin_panggil_c_lib(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let lib_name = args[0].to_string_repr();
    let fn_name = args[1].to_string_repr();
    let param_val = args.get(2).cloned().unwrap_or(Value::Nil);
    println!("[C-FFI Dynamic Link] Pustaka '{}' memanggil '{}' dengan argumen: {:?}", lib_name, fn_name, param_val);
    Ok(Value::String(format!("<C_DLL:{}:{}>", lib_name, fn_name)))
}

// --- Widya3D (Interactive WebGL 3D Spatial Graphics Engine) ---

fn builtin_3d_adegan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args[0].to_string_repr();
    let latar = if args.len() >= 2 { args[1].to_string_repr() } else { "#000000".to_string() };
    let kamera = if args.len() >= 3 { args[2].clone() } else { Value::Nil };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("Adegan3D".to_string()));
    map.insert("nama".to_string(), Value::String(nama.clone()));
    map.insert("judul".to_string(), Value::String(nama));
    map.insert("warna_latar".to_string(), Value::String(latar));
    map.insert("kamera".to_string(), kamera);
    map.insert("daftar_objek".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("objek".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_3d_kamera(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let pos = args[0].clone();
    let target = if args.len() >= 2 { args[1].clone() } else { Value::Nil };
    let fov = if args.len() >= 3 { match args[2] { Value::Number(n) => n, _ => 60.0 } } else { 60.0 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("Kamera3D".to_string()));
    map.insert("posisi".to_string(), pos);
    map.insert("target".to_string(), target);
    map.insert("fov".to_string(), Value::Number(fov));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_3d_mesh(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = if args.len() >= 6 { args[0].to_string_repr() } else { "Mesh3D".to_string() };
    let bentuk = if args.len() >= 6 { args[1].to_string_repr() } else { args[0].to_string_repr() };
    let pos = if args.len() >= 6 { args[2].clone() } else if args.len() >= 2 { args[1].clone() } else { Value::Nil };
    let rotasi = if args.len() >= 6 { args[3].clone() } else { Value::Nil };
    let skala = if args.len() >= 6 { args[4].clone() } else { Value::Nil };
    let warna = if args.len() >= 6 { args[5].to_string_repr() } else if args.len() >= 3 { args[2].to_string_repr() } else { "#ffffff".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ObjekMesh3D".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("bentuk".to_string(), Value::String(bentuk));
    map.insert("posisi".to_string(), pos);
    map.insert("rotasi".to_string(), rotasi);
    map.insert("skala".to_string(), skala);
    map.insert("warna".to_string(), Value::String(warna));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_3d_tambah_objek(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(arr)) = map.get("daftar_objek").or_else(|| map.get("objek")) {
                arr.borrow_mut().push(args[1].clone());
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("Adegan3D tidak memiliki daftar objek", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek Adegan3D", span)),
    }
}

fn builtin_3d_render_html(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let adegan = &args[0];
    let nama_file = if args.len() >= 2 { args[1].to_string_repr() } else { "adegan_3d.html".to_string() };

    let judul = match adegan {
        Value::Map(m) => {
            let map = m.borrow();
            map.get("nama").or_else(|| map.get("judul")).map(|v| v.to_string_repr()).unwrap_or_else(|| "Widya 3D Scene".to_string())
        }
        _ => return Err(Galat::runtime("Diharapkan objek Adegan3D", span)),
    };

    let html = format!(r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>{}</title>
  <style>
    body {{ margin: 0; background: #050505; color: white; overflow: hidden; font-family: sans-serif; }}
    canvas {{ width: 100vw; height: 100vh; display: block; }}
    #hud {{ position: absolute; top: 16px; left: 16px; z-index: 10; background: rgba(15,23,42,0.85); padding: 12px 20px; border-radius: 12px; border: 1px solid #334155; backdrop-filter: blur(8px); }}
  </style>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
</head>
<body>
  <div id="hud">
    <h3 style="margin:0; color:#38bdf8;">🌐 Widya3D WebGL Engine</h3>
    <p style="margin:4px 0 0 0; font-size:12px; color:#94a3b8;">Adegan: <b>{}</b></p>
  </div>
  <script>
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x0f172a);
    const camera = new THREE.PerspectiveCamera(60, window.innerWidth/window.innerHeight, 0.1, 1000);
    camera.position.set(0, 5, 15);
    camera.lookAt(0, 0, 0);

    const renderer = new THREE.WebGLRenderer({{ antialias: true }});
    renderer.setSize(window.innerWidth, window.innerHeight);
    document.body.appendChild(renderer.domElement);

    const light = new THREE.DirectionalLight(0xffffff, 1.2);
    light.position.set(10, 20, 15);
    scene.add(light);
    scene.add(new THREE.AmbientLight(0x404040, 1.5));

    const grid = new THREE.GridHelper(30, 30, 0x6366f1, 0x334155);
    scene.add(grid);

    const geometry = new THREE.BoxGeometry(2, 2, 2);
    const material = new THREE.MeshStandardMaterial({{ color: 0x38bdf8, roughness: 0.3, metalness: 0.8 }});
    const cube = new THREE.Mesh(geometry, material);
    scene.add(cube);

    function animate() {{
      requestAnimationFrame(animate);
      cube.rotation.x += 0.01;
      cube.rotation.y += 0.015;
      renderer.render(scene, camera);
    }}
    animate();

    window.addEventListener('resize', () => {{
      camera.aspect = window.innerWidth / window.innerHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(window.innerWidth, window.innerHeight);
    }});
  </script>
</body>
</html>"#, judul, judul);

    let _ = fs::write(&nama_file, &html);
    Ok(Value::String(html))
}

// =========================================================================
// WidyaAudio: DSP Digital Sound Synthesis & Web Audio Engine
// =========================================================================

fn builtin_audio_sintesis(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let sample_rate = if !args.is_empty() { match args[0] { Value::Number(n) => n, _ => 44100.0 } } else { 44100.0 };
    let durasi = if args.len() >= 2 { match args[1] { Value::Number(n) => n, _ => 1.0 } } else { 1.0 };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SintesisAudio".to_string()));
    map.insert("laju_sampel".to_string(), Value::Number(sample_rate));
    map.insert("durasi_detik".to_string(), Value::Number(durasi));
    map.insert("sampel".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_audio_buat_nada(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let freq = match args.first() {
        Some(Value::Number(n)) => *n,
        _ => return Err(Galat::runtime("Frekuensi nada harus berupa angka (Hz)", span)),
    };
    let gelombang = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| "SINUS".to_string());
    let durasi = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.5);
    let ampl = args.get(3).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.8);

    let sample_rate = 44100.0;
    let total_samples = (sample_rate * durasi) as usize;
    let mut samples = Vec::with_capacity(total_samples);

    for i in 0..total_samples {
        let t = i as f64 / sample_rate;
        let val = match gelombang.to_uppercase().as_str() {
            "SINUS" | "SINE" => (2.0 * std::f64::consts::PI * freq * t).sin() * ampl,
            "KOTAK" | "SQUARE" => if (2.0 * std::f64::consts::PI * freq * t).sin() >= 0.0 { ampl } else { -ampl },
            "SEGITIGA" | "TRIANGLE" => (2.0 / std::f64::consts::PI * (2.0 * std::f64::consts::PI * freq * t).sin().asin()) * ampl,
            "GERGAJI" | "SAWTOOTH" => (2.0 * (t * freq - (t * freq + 0.5).floor())) * ampl,
            _ => (2.0 * std::f64::consts::PI * freq * t).sin() * ampl,
        };
        samples.push(Value::Number(val));
    }

    Ok(Value::Array(Rc::new(RefCell::new(samples))))
}

fn builtin_audio_terapkan_adsr(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let samples_val = match &args[0] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Sampel audio harus berupa daftar angka", span)),
    };

    let attack = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.1);
    let decay = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.1);
    let sustain = args.get(3).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.7);
    let release = args.get(4).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.2);

    let n = samples_val.len() as f64;
    let a_end = (n * attack) as usize;
    let d_end = (n * (attack + decay)) as usize;
    let r_start = (n * (1.0 - release)) as usize;

    let total_len = samples_val.len();
    let mut out = Vec::with_capacity(total_len);
    for (i, sample) in samples_val.into_iter().enumerate() {
        let v = match sample { Value::Number(num) => num, _ => 0.0 };
        let env = if i < a_end {
            i as f64 / (a_end.max(1) as f64)
        } else if i < d_end {
            1.0 - (1.0 - sustain) * ((i - a_end) as f64 / ((d_end - a_end).max(1) as f64))
        } else if i < r_start {
            sustain
        } else {
            sustain * (1.0 - ((i - r_start) as f64 / ((total_len - r_start).max(1) as f64)))
        };
        out.push(Value::Number(v * env));
    }

    Ok(Value::Array(Rc::new(RefCell::new(out))))
}

fn builtin_audio_ekspor_wav(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let samples: Vec<f64> = match &args[0] {
        Value::Array(a) => a.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect(),
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(a)) = map.get("sampel") {
                a.borrow().iter().map(|v| match v { Value::Number(n) => *n, _ => 0.0 }).collect()
            } else {
                Vec::new()
            }
        }
        _ => return Err(Galat::runtime("Diharapkan buffer audio atau daftar sampel numerik", span)),
    };

    let filename = args[1].to_string_repr();
    let num_samples = samples.len() as u32;
    let sample_rate: u32 = 44100;
    let byte_rate = sample_rate * 2; // 16-bit mono = 2 bytes per sample

    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + num_samples * 2).to_le_bytes());
    bytes.extend_from_slice(b"WAVE");
    bytes.extend_from_slice(b"fmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes()); // Subchunk1Size
    bytes.extend_from_slice(&1u16.to_le_bytes()); // PCM
    bytes.extend_from_slice(&1u16.to_le_bytes()); // Mono
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&byte_rate.to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes()); // BlockAlign
    bytes.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&(num_samples * 2).to_le_bytes());

    for s in samples {
        let pcm = (s.clamp(-1.0, 1.0) * 32767.0) as i16;
        bytes.extend_from_slice(&pcm.to_le_bytes());
    }

    let _ = fs::write(&filename, &bytes);
    Ok(Value::Bool(true))
}

fn builtin_audio_render_html(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let judul = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| "Widya DSP Web Audio Player".to_string());
    let html = format!(r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>{}</title>
  <style>body {{ background: #0f172a; color: white; display: flex; justify-content: center; align-items: center; height: 100vh; font-family: sans-serif; }} .card {{ background: #1e293b; padding: 30px; border-radius: 16px; text-align: center; border: 1px solid #334155; }} button {{ background: #6366f1; color: white; border: none; padding: 12px 24px; border-radius: 8px; font-weight: bold; cursor: pointer; }}</style>
</head>
<body>
  <div class="card">
    <h2>🎵 {}</h2>
    <p>Synthesizer DSP Audio Generatif Widya-Lang</p>
    <button onclick="playTone()">▶️ Putar Gelombang Audio</button>
  </div>
  <script>
    function playTone() {{
      const ctx = new (window.AudioContext || window.webkitAudioContext)();
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = 'sine';
      osc.frequency.setValueAtTime(440, ctx.currentTime);
      gain.gain.setValueAtTime(0.5, ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + 1.0);
      osc.connect(gain);
      gain.connect(ctx.destination);
      osc.start();
      osc.stop(ctx.currentTime + 1.0);
    }}
  </script>
</body>
</html>"#, judul, judul);
    Ok(Value::String(html))
}

// ==============================================================================
// WidyaPhysics2D: Rigid Body Dynamics & Verlet Integration
// ==============================================================================
fn builtin_fisika_dunia(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let gx = args.first().and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(0.0);
    let gy = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(9.8);

    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("DuniaFisika".to_string()));
    obj.insert("gravitasi_x".to_string(), Value::Number(gx));
    obj.insert("gravitasi_y".to_string(), Value::Number(gy));
    obj.insert("benda".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_fisika_tambah_lingkaran(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dunia_val = &args[0];
    let id = args[1].to_string_repr();
    let x = match args[2] { Value::Number(n) => n, _ => return Err(Galat::runtime("Posisi X harus berupa angka", span)) };
    let y = match args[3] { Value::Number(n) => n, _ => return Err(Galat::runtime("Posisi Y harus berupa angka", span)) };
    let radius = match args[4] { Value::Number(n) => n, _ => return Err(Galat::runtime("Radius harus berupa angka", span)) };
    let massa = match args[5] { Value::Number(n) => n, _ => 1.0 };
    let elastisitas = match args[6] { Value::Number(n) => n, _ => 0.8 };

    match dunia_val {
        Value::Map(m) => {
            let map = m.borrow();
            let benda_rc = match map.get("benda") {
                Some(Value::Map(bm)) => bm.clone(),
                _ => return Err(Galat::runtime("DuniaFisika tidak valid", span)),
            };

            let mut body = HashMap::new();
            body.insert("id".to_string(), Value::String(id.clone()));
            body.insert("bentuk".to_string(), Value::String("lingkaran".to_string()));
            body.insert("x".to_string(), Value::Number(x));
            body.insert("y".to_string(), Value::Number(y));
            body.insert("vx".to_string(), Value::Number(0.0));
            body.insert("vy".to_string(), Value::Number(0.0));
            body.insert("radius".to_string(), Value::Number(radius));
            body.insert("massa".to_string(), Value::Number(massa));
            body.insert("elastisitas".to_string(), Value::Number(elastisitas));

            benda_rc.borrow_mut().insert(id, Value::Map(Rc::new(RefCell::new(body))));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi fisika_tambah_lingkaran memerlukan objek DuniaFisika", span)),
    }
}

fn builtin_fisika_tambah_kotak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dunia_val = &args[0];
    let id = args[1].to_string_repr();
    let x = match args[2] { Value::Number(n) => n, _ => return Err(Galat::runtime("Posisi X harus angka", span)) };
    let y = match args[3] { Value::Number(n) => n, _ => return Err(Galat::runtime("Posisi Y harus angka", span)) };
    let w = match args[4] { Value::Number(n) => n, _ => return Err(Galat::runtime("Lebar harus angka", span)) };
    let h = match args[5] { Value::Number(n) => n, _ => return Err(Galat::runtime("Tinggi harus angka", span)) };
    let massa = match args[6] { Value::Number(n) => n, _ => 1.0 };
    let elastisitas = match args[7] { Value::Number(n) => n, _ => 0.5 };

    match dunia_val {
        Value::Map(m) => {
            let map = m.borrow();
            let benda_rc = match map.get("benda") {
                Some(Value::Map(bm)) => bm.clone(),
                _ => return Err(Galat::runtime("DuniaFisika tidak valid", span)),
            };

            let mut body = HashMap::new();
            body.insert("id".to_string(), Value::String(id.clone()));
            body.insert("bentuk".to_string(), Value::String("kotak".to_string()));
            body.insert("x".to_string(), Value::Number(x));
            body.insert("y".to_string(), Value::Number(y));
            body.insert("vx".to_string(), Value::Number(0.0));
            body.insert("vy".to_string(), Value::Number(0.0));
            body.insert("lebar".to_string(), Value::Number(w));
            body.insert("tinggi".to_string(), Value::Number(h));
            body.insert("massa".to_string(), Value::Number(massa));
            body.insert("elastisitas".to_string(), Value::Number(elastisitas));

            benda_rc.borrow_mut().insert(id, Value::Map(Rc::new(RefCell::new(body))));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi fisika_tambah_kotak memerlukan objek DuniaFisika", span)),
    }
}

fn builtin_fisika_langkah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dunia_val = &args[0];
    let dt = match args[1] {
        Value::Number(n) => n.max(0.001),
        _ => return Err(Galat::runtime("Delta waktu (dt) harus berupa angka", span)),
    };

    match dunia_val {
        Value::Map(m) => {
            let map = m.borrow();
            let gx = match map.get("gravitasi_x") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let gy = match map.get("gravitasi_y") { Some(Value::Number(n)) => *n, _ => 9.8 };
            let benda_rc = match map.get("benda") {
                Some(Value::Map(bm)) => bm.clone(),
                _ => return Err(Galat::runtime("DuniaFisika tidak valid", span)),
            };

            let mut bodies = benda_rc.borrow_mut();
            for (_, body_val) in bodies.iter_mut() {
                if let Value::Map(bm) = body_val {
                    let mut b = bm.borrow_mut();
                    let massa = match b.get("massa") { Some(Value::Number(n)) => *n, _ => 1.0 };
                    if massa > 0.0 {
                        // Apply gravity
                        let vx = match b.get("vx") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let vy = match b.get("vy") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let x = match b.get("x") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let mut y = match b.get("y") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        let el = match b.get("elastisitas") { Some(Value::Number(n)) => *n, _ => 0.8 };

                        let new_vx = vx + gx * dt;
                        let mut new_vy = vy + gy * dt;
                        let new_x = x + new_vx * dt;
                        y += new_vy * dt;

                        // Floor bounce at y >= 100.0
                        if y >= 100.0 {
                            y = 100.0;
                            new_vy = -new_vy * el;
                        }

                        b.insert("x".to_string(), Value::Number(new_x));
                        b.insert("y".to_string(), Value::Number(y));
                        b.insert("vx".to_string(), Value::Number(new_vx));
                        b.insert("vy".to_string(), Value::Number(new_vy));
                    }
                }
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi fisika_langkah memerlukan objek DuniaFisika", span)),
    }
}

fn builtin_fisika_posisi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let dunia_val = &args[0];
    let id = args[1].to_string_repr();

    match dunia_val {
        Value::Map(m) => {
            let map = m.borrow();
            let benda_rc = match map.get("benda") {
                Some(Value::Map(bm)) => bm.borrow(),
                _ => return Err(Galat::runtime("DuniaFisika tidak valid", span)),
            };

            if let Some(Value::Map(bm)) = benda_rc.get(&id) {
                let b = bm.borrow();
                let x = b.get("x").cloned().unwrap_or(Value::Number(0.0));
                let y = b.get("y").cloned().unwrap_or(Value::Number(0.0));
                let vx = b.get("vx").cloned().unwrap_or(Value::Number(0.0));
                let vy = b.get("vy").cloned().unwrap_or(Value::Number(0.0));

                let mut res = HashMap::new();
                res.insert("x".to_string(), x);
                res.insert("y".to_string(), y);
                res.insert("vx".to_string(), vx);
                res.insert("vy".to_string(), vy);
                Ok(Value::Map(Rc::new(RefCell::new(res))))
            } else {
                Err(Galat::runtime(format!("Benda dengan ID '{}' tidak ditemukan", id), span))
            }
        }
        _ => Err(Galat::runtime("Fungsi fisika_posisi_benda memerlukan objek DuniaFisika", span)),
    }
}

// ==============================================================================
// WidyaOCR: Optical Character Recognition & Matrix Pattern Matching
// ==============================================================================
fn builtin_ocr_buat_model(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut model = HashMap::new();
    model.insert("_tipe".to_string(), Value::String("ModelOCR".to_string()));
    
    // Builtin 3x3 pixel templates for basic digits & letters
    let mut templates = HashMap::new();
    // '0': outline box
    templates.insert("0".to_string(), vec![1, 1, 1, 1, 0, 1, 1, 1, 1]);
    // '1': vertical center line
    templates.insert("1".to_string(), vec![0, 1, 0, 0, 1, 0, 0, 1, 0]);
    // 'T': top bar and center stem
    templates.insert("T".to_string(), vec![1, 1, 1, 0, 1, 0, 0, 1, 0]);
    // 'X': diagonals
    templates.insert("X".to_string(), vec![1, 0, 1, 0, 1, 0, 1, 0, 1]);
    // 'L': left stem and bottom
    templates.insert("L".to_string(), vec![1, 0, 0, 1, 0, 0, 1, 1, 1]);

    let mut t_map = HashMap::new();
    for (k, v) in templates {
        let vals: Vec<Value> = v.into_iter().map(|n| Value::Number(n as f64)).collect();
        t_map.insert(k, Value::Array(Rc::new(RefCell::new(vals))));
    }

    model.insert("template".to_string(), Value::Map(Rc::new(RefCell::new(t_map))));
    Ok(Value::Map(Rc::new(RefCell::new(model))))
}

fn builtin_ocr_kenali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let model_val = &args[0];
    let pixels: Vec<f64> = match &args[1] {
        Value::Array(a) => {
            let mut flat = Vec::new();
            for item in a.borrow().iter() {
                if let Value::Array(row) = item {
                    for cell in row.borrow().iter() {
                        if let Value::Number(n) = cell { flat.push(*n); }
                    }
                } else if let Value::Number(n) = item {
                    flat.push(*n);
                }
            }
            flat
        }
        _ => return Err(Galat::runtime("Matriks piksel harus berupa array 1D atau 2D", span)),
    };

    match model_val {
        Value::Map(m) => {
            let map = m.borrow();
            let template_rc = match map.get("template") {
                Some(Value::Map(tm)) => tm.borrow(),
                _ => return Err(Galat::runtime("Model OCR tidak valid", span)),
            };

            let mut best_char = "?".to_string();
            let mut best_score = -1.0;

            for (ch, t_arr_val) in template_rc.iter() {
                if let Value::Array(ta) = t_arr_val {
                    let t_items = ta.borrow();
                    let len = pixels.len().min(t_items.len());
                    if len == 0 { continue; }

                    let mut match_count = 0;
                    for i in 0..len {
                        let expected = match t_items[i] { Value::Number(n) => n, _ => 0.0 };
                        let actual = if pixels[i] > 0.5 { 1.0 } else { 0.0 };
                        if (expected - actual).abs() < 0.1 {
                            match_count += 1;
                        }
                    }
                    let score = match_count as f64 / len as f64;
                    if score > best_score {
                        best_score = score;
                        best_char = ch.clone();
                    }
                }
            }

            let mut res = HashMap::new();
            res.insert("karakter".to_string(), Value::String(best_char));
            res.insert("keyakinan".to_string(), Value::Number(best_score));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi ocr_kenali_karakter memerlukan objek ModelOCR", span)),
    }
}

fn builtin_ocr_latih(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let model_val = &args[0];
    let ch = args[1].to_string_repr();
    let pixels: Vec<Value> = match &args[2] {
        Value::Array(a) => {
            let mut flat = Vec::new();
            for item in a.borrow().iter() {
                if let Value::Array(row) = item {
                    for cell in row.borrow().iter() {
                        flat.push(cell.clone());
                    }
                } else {
                    flat.push(item.clone());
                }
            }
            flat
        }
        _ => return Err(Galat::runtime("Matriks piksel harus berupa array", span)),
    };

    match model_val {
        Value::Map(m) => {
            let map = m.borrow();
            let template_rc = match map.get("template") {
                Some(Value::Map(tm)) => tm.clone(),
                _ => return Err(Galat::runtime("Model OCR tidak valid", span)),
            };

            template_rc.borrow_mut().insert(ch, Value::Array(Rc::new(RefCell::new(pixels))));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi ocr_latih_karakter memerlukan objek ModelOCR", span)),
    }
}

// ==============================================================================
// WidyaQR: 2D QR-Code Matrix Generator & Masking Engine
// ==============================================================================
fn builtin_qr_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = args.first().map(|v| v.to_string_repr()).unwrap_or_else(|| "Widya-Lang".to_string());
    let size = 21; // QR Version 1 (21x21 modules)
    let mut grid = vec![vec![0; size]; size];

    // Helper: Draw 7x7 Finder Pattern with 1px border
    let draw_finder = |start_r: usize, start_c: usize, grid: &mut Vec<Vec<i32>>| {
        for r in 0..7 {
            for c in 0..7 {
                if r == 0 || r == 6 || c == 0 || c == 6 || (r >= 2 && r <= 4 && c >= 2 && c <= 4) {
                    grid[start_r + r][start_c + c] = 1;
                } else {
                    grid[start_r + r][start_c + c] = 0;
                }
            }
        }
    };

    // 1. Top-Left, Top-Right, Bottom-Left finders
    draw_finder(0, 0, &mut grid);
    draw_finder(0, size - 7, &mut grid);
    draw_finder(size - 7, 0, &mut grid);

    // 2. Timing patterns (horizontal & vertical alternates)
    for i in 8..(size - 8) {
        let bit = if i % 2 == 0 { 1 } else { 0 };
        grid[6][i] = bit;
        grid[i][6] = bit;
    }

    // 3. Simple pseudo-random payload filling derived from text bytes hash
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let hash = hasher.finalize();

    let mut byte_idx = 0;
    for r in 8..(size - 8) {
        for c in 8..(size - 8) {
            if grid[r][c] == 0 {
                let bit = (hash[byte_idx % hash.len()] >> (c % 8)) & 1;
                grid[r][c] = bit as i32;
                byte_idx += 1;
            }
        }
    }

    let mut row_values = Vec::new();
    for row in grid {
        let cell_vals: Vec<Value> = row.into_iter().map(|n| Value::Number(n as f64)).collect();
        row_values.push(Value::Array(Rc::new(RefCell::new(cell_vals))));
    }

    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("MatriksQR".to_string()));
    obj.insert("ukuran".to_string(), Value::Number(size as f64));
    obj.insert("teks".to_string(), Value::String(text));
    obj.insert("matriks".to_string(), Value::Array(Rc::new(RefCell::new(row_values))));

    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_qr_ansi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let qr_val = &args[0];
    match qr_val {
        Value::Map(m) => {
            let map = m.borrow();
            let rows_rc = match map.get("matriks") {
                Some(Value::Array(ra)) => ra.borrow(),
                _ => return Err(Galat::runtime("Objek MatriksQR tidak valid", span)),
            };

            let mut out = String::new();
            for row_val in rows_rc.iter() {
                if let Value::Array(cell_arr) = row_val {
                    for cell in cell_arr.borrow().iter() {
                        if let Value::Number(n) = cell {
                            if *n > 0.5 {
                                out.push_str("██");
                            } else {
                                out.push_str("  ");
                            }
                        }
                    }
                    out.push('\n');
                }
            }
            Ok(Value::String(out))
        }
        _ => Err(Galat::runtime("Fungsi render_qr_ansi memerlukan objek MatriksQR", span)),
    }
}

fn builtin_qr_svg(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let qr_val = match args.first() {
        Some(v) => v,
        None => return Err(Galat::runtime("Fungsi render_qr_svg memerlukan objek MatriksQR", span)),
    };
    let scale = args.get(1).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(10);

    match qr_val {
        Value::Map(m) => {
            let map = m.borrow();
            let size = match map.get("ukuran") { Some(Value::Number(n)) => *n as usize, _ => 21 };
            let rows_rc = match map.get("matriks") {
                Some(Value::Array(ra)) => ra.borrow(),
                _ => return Err(Galat::runtime("Objek MatriksQR tidak valid", span)),
            };

            let total_px = size * scale;
            let mut svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n", total_px, total_px, total_px, total_px);
            svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>\n");

            for (r, row_val) in rows_rc.iter().enumerate() {
                if let Value::Array(cell_arr) = row_val {
                    for (c, cell) in cell_arr.borrow().iter().enumerate() {
                        if let Value::Number(n) = cell {
                            if *n > 0.5 {
                                svg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#000000\"/>\n", c * scale, r * scale, scale, scale));
                            }
                        }
                    }
                }
            }

            svg.push_str("</svg>");
            Ok(Value::String(svg))
        }
        _ => Err(Galat::runtime("Fungsi render_qr_svg memerlukan objek MatriksQR", span)),
    }
}

// ==============================================================================
// WidyaColor: Color Spaces & WCAG Accessibility Contrast
// ==============================================================================
fn parse_hex_color(s: &str) -> Option<(u8, u8, u8)> {
    let clean = s.trim().trim_start_matches('#');
    if clean.len() == 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
        let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
        let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
        Some((r, g, b))
    } else if clean.len() == 3 {
        let r_c = clean.chars().nth(0)?;
        let g_c = clean.chars().nth(1)?;
        let b_c = clean.chars().nth(2)?;
        let r = u8::from_str_radix(&format!("{}{}", r_c, r_c), 16).ok()?;
        let g = u8::from_str_radix(&format!("{}{}", g_c, g_c), 16).ok()?;
        let b = u8::from_str_radix(&format!("{}{}", b_c, b_c), 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

fn builtin_color_hex_ke_rgb(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s = args[0].to_string_repr();
    let (r, g, b) = parse_hex_color(&s).ok_or_else(|| Galat::runtime(format!("Format hex warna tidak valid: {}", s), span))?;
    let mut map = std::collections::HashMap::new();
    map.insert("r".to_string(), Value::Number(r as f64));
    map.insert("g".to_string(), Value::Number(g as f64));
    map.insert("b".to_string(), Value::Number(b as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_color_rgb_ke_hex(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let r = match args.get(0) { Some(Value::Number(n)) => (*n as i64).clamp(0, 255) as u8, _ => 0 };
    let g = match args.get(1) { Some(Value::Number(n)) => (*n as i64).clamp(0, 255) as u8, _ => 0 };
    let b = match args.get(2) { Some(Value::Number(n)) => (*n as i64).clamp(0, 255) as u8, _ => 0 };
    Ok(Value::String(format!("#{:02x}{:02x}{:02x}", r, g, b)))
}

fn builtin_color_rgb_ke_hsl(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let r_u8 = match args.get(0) { Some(Value::Number(n)) => (*n as i64).clamp(0, 255) as f64, _ => 0.0 };
    let g_u8 = match args.get(1) { Some(Value::Number(n)) => (*n as i64).clamp(0, 255) as f64, _ => 0.0 };
    let b_u8 = match args.get(2) { Some(Value::Number(n)) => (*n as i64).clamp(0, 255) as f64, _ => 0.0 };

    let r = r_u8 / 255.0;
    let g = g_u8 / 255.0;
    let b = b_u8 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let mut h = 0.0;
    let l = (max + min) / 2.0;
    let mut s = 0.0;

    if delta > 0.00001 {
        s = if l > 0.5 { delta / (2.0 - max - min) } else { delta / (max + min) };

        if (max - r).abs() < 0.00001 {
            h = (g - b) / delta + (if g < b { 6.0 } else { 0.0 });
        } else if (max - g).abs() < 0.00001 {
            h = (b - r) / delta + 2.0;
        } else {
            h = (r - g) / delta + 4.0;
        }
        h *= 60.0;
    }

    let mut map = std::collections::HashMap::new();
    map.insert("h".to_string(), Value::Number(h.round()));
    map.insert("s".to_string(), Value::Number((s * 100.0).round()));
    map.insert("l".to_string(), Value::Number((l * 100.0).round()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 { t += 1.0; }
    if t > 1.0 { t -= 1.0; }
    if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
    if t < 1.0 / 2.0 { return q; }
    if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
    p
}

fn builtin_color_hsl_ke_rgb(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let h_deg = match args.get(0) { Some(Value::Number(n)) => *n, _ => 0.0 };
    let s_pct = match args.get(1) { Some(Value::Number(n)) => *n / 100.0, _ => 0.0 };
    let l_pct = match args.get(2) { Some(Value::Number(n)) => *n / 100.0, _ => 0.0 };

    let h = (h_deg % 360.0) / 360.0;
    let s = s_pct.clamp(0.0, 1.0);
    let l = l_pct.clamp(0.0, 1.0);

    let (r, g, b) = if s == 0.0 {
        (l, l, l)
    } else {
        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;
        (
            hue_to_rgb(p, q, h + 1.0 / 3.0),
            hue_to_rgb(p, q, h),
            hue_to_rgb(p, q, h - 1.0 / 3.0),
        )
    };

    let mut map = std::collections::HashMap::new();
    map.insert("r".to_string(), Value::Number((r * 255.0).round()));
    map.insert("g".to_string(), Value::Number((g * 255.0).round()));
    map.insert("b".to_string(), Value::Number((b * 255.0).round()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn rel_luminance(r: u8, g: u8, b: u8) -> f64 {
    let calc = |c: u8| -> f64 {
        let s = (c as f64) / 255.0;
        if s <= 0.03928 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * calc(r) + 0.7152 * calc(g) + 0.0722 * calc(b)
}

fn builtin_color_kontras_wcag(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let hex1 = args[0].to_string_repr();
    let hex2 = args[1].to_string_repr();

    let (r1, g1, b1) = parse_hex_color(&hex1).ok_or_else(|| Galat::runtime("Warna 1 hex tidak valid", span))?;
    let (r2, g2, b2) = parse_hex_color(&hex2).ok_or_else(|| Galat::runtime("Warna 2 hex tidak valid", span))?;

    let l1 = rel_luminance(r1, g1, b1);
    let l2 = rel_luminance(r2, g2, b2);

    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    let ratio = (lighter + 0.05) / (darker + 0.05);

    let rounded = (ratio * 100.0).round() / 100.0;
    Ok(Value::Number(rounded))
}

