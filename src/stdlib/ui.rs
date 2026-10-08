use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// =========================================================================
// WIDYAUI (FLUTTER-LIKE WIDGET FRAMEWORK)
// =========================================================================

fn make_widget(tipe: &str, properti: HashMap<String, Value>) -> Value {
    let mut map = properti;
    map.insert("_widget_tipe".to_string(), Value::String(tipe.to_string()));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_ui_aplikasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => HashMap::new(),
    };
    if !prop.contains_key("judul") {
        prop.insert("judul".to_string(), Value::String("Aplikasi Widya".to_string()));
    }
    Ok(make_widget("Aplikasi", prop))
}

fn builtin_ui_halaman(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => HashMap::new(),
    };
    Ok(make_widget("Halaman", prop))
}

fn builtin_ui_kolom(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut prop = HashMap::new();
    if args.len() == 1 {
        match &args[0] {
            Value::Array(a) => {
                prop.insert("anak".to_string(), Value::Array(a.clone()));
            }
            Value::Map(m) => {
                prop = m.borrow().clone();
            }
            _ => return Err(Galat::runtime("Kolom menerima daftar anak atau kamus properti", span)),
        }
    } else if args.len() >= 2 {
        if let Value::Map(m) = &args[0] {
            prop = m.borrow().clone();
        }
        if let Value::Array(a) = &args[1] {
            prop.insert("anak".to_string(), Value::Array(a.clone()));
        }
    }
    Ok(make_widget("Kolom", prop))
}

fn builtin_ui_baris(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut prop = HashMap::new();
    if args.len() == 1 {
        match &args[0] {
            Value::Array(a) => {
                prop.insert("anak".to_string(), Value::Array(a.clone()));
            }
            Value::Map(m) => {
                prop = m.borrow().clone();
            }
            _ => return Err(Galat::runtime("Baris menerima daftar anak atau kamus properti", span)),
        }
    } else if args.len() >= 2 {
        if let Value::Map(m) = &args[0] {
            prop = m.borrow().clone();
        }
        if let Value::Array(a) = &args[1] {
            prop.insert("anak".to_string(), Value::Array(a.clone()));
        }
    }
    Ok(make_widget("Baris", prop))
}

fn builtin_ui_teks(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("TeksWidget memerlukan minimal 1 argumen teks", span));
    }
    let mut prop = HashMap::new();
    prop.insert("teks".to_string(), Value::String(args[0].to_string_repr()));
    if let Some(Value::Map(gaya)) = args.get(1) {
        prop.insert("gaya".to_string(), Value::Map(gaya.clone()));
    }
    Ok(make_widget("Teks", prop))
}

fn builtin_ui_tombol(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut prop = HashMap::new();
    if !args.is_empty() {
        prop.insert("label".to_string(), Value::String(args[0].to_string_repr()));
    }
    if args.len() > 1 {
        prop.insert("pada_klik".to_string(), args[1].clone());
    }
    Ok(make_widget("Tombol", prop))
}

fn builtin_ui_tombol_utama(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut prop = HashMap::new();
    if !args.is_empty() {
        prop.insert("label".to_string(), Value::String(args[0].to_string_repr()));
    }
    if args.len() > 1 {
        prop.insert("pada_klik".to_string(), args[1].clone());
    }
    prop.insert("utama".to_string(), Value::Bool(true));
    Ok(make_widget("TombolUtama", prop))
}

fn builtin_ui_bilah_atas(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match args.first() {
        Some(Value::Map(m)) => m.borrow().clone(),
        Some(Value::String(s)) => {
            let mut p = HashMap::new();
            p.insert("judul".to_string(), Value::String(s.clone()));
            p
        }
        _ => HashMap::new(),
    };
    Ok(make_widget("BilahAtas", prop))
}

fn builtin_ui_bidang_teks(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        Value::String(hint) => {
            let mut p = HashMap::new();
            p.insert("petunjuk".to_string(), Value::String(hint.clone()));
            p
        }
        _ => HashMap::new(),
    };
    Ok(make_widget("BidangTeks", prop))
}

fn builtin_ui_kontainer(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => HashMap::new(),
    };
    Ok(make_widget("Kontainer", prop))
}

fn builtin_ui_kartu(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => HashMap::new(),
    };
    Ok(make_widget("Kartu", prop))
}

fn builtin_ui_pemisah(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut prop = HashMap::new();
    if let Some(Value::Number(n)) = args.first() {
        prop.insert("ukuran".to_string(), Value::Number(*n));
    }
    Ok(make_widget("Pemisah", prop))
}

fn builtin_ui_gambar(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Gambar memerlukan URL", span));
    }
    let mut prop = HashMap::new();
    prop.insert("url".to_string(), Value::String(args[0].to_string_repr()));
    Ok(make_widget("Gambar", prop))
}

fn builtin_ui_daftar_gulir(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mut prop = HashMap::new();
    match &args[0] {
        Value::Array(a) => {
            prop.insert("anak".to_string(), Value::Array(a.clone()));
        }
        Value::Map(m) => {
            prop = m.borrow().clone();
        }
        _ => return Err(Galat::runtime("DaftarGulir memerlukan daftar widget anak", span)),
    }
    Ok(make_widget("DaftarGulir", prop))
}

fn builtin_ui_item_daftar(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => HashMap::new(),
    };
    Ok(make_widget("ItemDaftar", prop))
}

fn builtin_ui_lencana(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        Value::String(s) => {
            let mut p = HashMap::new();
            p.insert("teks".to_string(), Value::String(s.clone()));
            p
        }
        _ => HashMap::new(),
    };
    Ok(make_widget("Lencana", prop))
}

fn builtin_ui_sakelar(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => HashMap::new(),
    };
    Ok(make_widget("Sakelar", prop))
}

fn builtin_ui_progres(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        Value::Number(n) => {
            let mut p = HashMap::new();
            p.insert("nilai".to_string(), Value::Number(*n));
            p
        }
        _ => HashMap::new(),
    };
    Ok(make_widget("IndikatorProgres", prop))
}

fn builtin_ui_avatar(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        Value::String(s) => {
            let mut p = HashMap::new();
            p.insert("nama".to_string(), Value::String(s.clone()));
            p
        }
        _ => HashMap::new(),
    };
    Ok(make_widget("Avatar", prop))
}

fn builtin_ui_bilah_navigasi(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        Value::Array(a) => {
            let mut p = HashMap::new();
            p.insert("items".to_string(), Value::Array(a.clone()));
            p
        }
        _ => HashMap::new(),
    };
    Ok(make_widget("BilahNavigasiBawah", prop))
}

fn builtin_ui_fab(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        Value::String(s) => {
            let mut p = HashMap::new();
            p.insert("ikon".to_string(), Value::String(s.clone()));
            p
        }
        _ => HashMap::new(),
    };
    Ok(make_widget("TombolAksiMengambang", prop))
}

fn builtin_ui_lembar_bawah(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let prop = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => HashMap::new(),
    };
    Ok(make_widget("LembarBawah", prop))
}

fn render_widget_html(w: &Value) -> String {
    if let Value::Map(m) = w {
        let map = m.borrow();
        let tipe = map.get("_widget_tipe").map(|v| v.to_string_repr()).unwrap_or_default();

        match tipe.as_str() {
            "Aplikasi" => {
                let judul = map.get("judul").map(|v| v.to_string_repr()).unwrap_or_else(|| "Widya App".to_string());
                let _tema = map.get("tema").map(|v| v.to_string_repr()).unwrap_or_else(|| "indigo".to_string());
                let badan = map.get("badan").map(render_widget_html).unwrap_or_default();
                let fab = map.get("tombol_mengambang").map(render_widget_html).unwrap_or_default();
                let nav_bawah = map.get("navigasi_bawah").map(render_widget_html).unwrap_or_default();
                format!(
                    r##"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover">
    <title>{}</title>
    <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&display=swap" rel="stylesheet">
    <style>
        :root {{
            --primary: #6366f1;
            --primary-gradient: linear-gradient(135deg, #6366f1, #4f46e5);
            --bg-page: #090d16;
            --bg-frame: #111827;
            --bg-card: rgba(31, 41, 55, 0.7);
            --border-card: rgba(255, 255, 255, 0.08);
            --text-main: #f9fafb;
            --text-muted: #9ca3af;
            --sat: env(safe-area-inset-top, 0px);
            --sab: env(safe-area-inset-bottom, 0px);
            --sar: env(safe-area-inset-right, 0px);
            --sal: env(safe-area-inset-left, 0px);
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; font-family: 'Plus Jakarta Sans', -apple-system, sans-serif; -webkit-tap-highlight-color: transparent; }}
        body {{ background: var(--bg-page); color: var(--text-main); min-height: 100vh; display: flex; justify-content: center; align-items: center; padding: 24px; overflow-x: hidden; }}
        
        .widya-app-frame {{
            width: 100%;
            max-width: 420px;
            height: 840px;
            background: var(--bg-frame);
            border-radius: 44px;
            box-shadow: 0 25px 60px -15px rgba(0,0,0,0.7), 0 0 0 10px #1f2937, 0 0 0 12px rgba(255,255,255,0.08);
            overflow: hidden;
            display: flex;
            flex-direction: column;
            position: relative;
        }}

        .widya-status-bar {{
            height: 38px;
            background: rgba(17, 24, 39, 0.95);
            backdrop-filter: blur(12px);
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 0 24px;
            font-size: 0.78rem;
            font-weight: 600;
            color: var(--text-main);
            z-index: 50;
        }}
        .widya-status-icons {{ display: flex; gap: 6px; align-items: center; }}

        .widya-notch {{
            position: absolute;
            top: 0;
            left: 50%;
            transform: translateX(-50%);
            width: 120px;
            height: 24px;
            background: #000;
            border-bottom-left-radius: 14px;
            border-bottom-right-radius: 14px;
            z-index: 60;
        }}

        .widya-appbar {{
            background: rgba(17, 24, 39, 0.85);
            backdrop-filter: blur(16px);
            padding: 14px 20px;
            border-bottom: 1px solid var(--border-card);
            display: flex;
            align-items: center;
            justify-content: space-between;
            z-index: 40;
        }}
        .widya-appbar-title {{ font-weight: 800; font-size: 1.15rem; letter-spacing: -0.02em; }}
        .widya-appbar-badge {{ font-size: 0.7rem; font-weight: 700; background: rgba(99,102,241,0.2); color: #818cf8; padding: 3px 8px; border-radius: 12px; border: 1px solid rgba(99,102,241,0.3); }}

        .widya-body {{
            flex: 1;
            padding: 20px;
            display: flex;
            flex-direction: column;
            overflow-y: auto;
            -webkit-overflow-scrolling: touch;
            gap: 16px;
        }}
        .widya-body::-webkit-scrollbar {{ width: 0px; }}

        .widya-kolom {{ display: flex; flex-direction: column; gap: 14px; width: 100%; }}
        .widya-baris {{ display: flex; flex-direction: row; gap: 12px; align-items: center; width: 100%; }}
        .widya-teks {{ font-size: 0.95rem; color: #e5e7eb; line-height: 1.5; }}
        
        .widya-kartu {{
            background: var(--bg-card);
            backdrop-filter: blur(12px);
            border-radius: 20px;
            padding: 18px;
            border: 1px solid var(--border-card);
            box-shadow: 0 8px 24px rgba(0,0,0,0.25);
            transition: transform 0.2s ease, border-color 0.2s ease;
        }}
        .widya-kartu:hover {{ border-color: rgba(99,102,241,0.3); }}

        .widya-tombol {{
            background: var(--primary-gradient);
            color: #ffffff;
            border: none;
            padding: 14px 20px;
            border-radius: 16px;
            font-weight: 700;
            font-size: 0.95rem;
            cursor: pointer;
            transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
            box-shadow: 0 6px 20px rgba(99,102,241,0.35);
            text-align: center;
            width: 100%;
            display: inline-flex;
            justify-content: center;
            align-items: center;
            gap: 8px;
        }}
        .widya-tombol:hover {{ transform: translateY(-2px); box-shadow: 0 10px 25px rgba(99,102,241,0.5); }}
        .widya-tombol:active {{ transform: scale(0.97); }}

        .widya-bidang-teks {{
            width: 100%;
            background: rgba(17, 24, 39, 0.8);
            border: 1px solid var(--border-card);
            border-radius: 14px;
            padding: 13px 16px;
            color: #ffffff;
            font-size: 0.92rem;
            outline: none;
            transition: border-color 0.2s ease, box-shadow 0.2s ease;
        }}
        .widya-bidang-teks:focus {{ border-color: var(--primary); box-shadow: 0 0 0 3px rgba(99,102,241,0.2); }}

        .widya-list-tile {{
            display: flex;
            align-items: center;
            padding: 12px 14px;
            background: rgba(255,255,255,0.03);
            border-radius: 14px;
            gap: 12px;
            cursor: pointer;
            transition: background 0.15s ease;
        }}
        .widya-list-tile:active {{ background: rgba(99,102,241,0.15); }}
        .widya-tile-icon {{ width: 38px; height: 38px; border-radius: 10px; background: rgba(99,102,241,0.15); display: flex; align-items: center; justify-content: center; font-size: 1.1rem; }}
        .widya-tile-content {{ flex: 1; }}
        .widya-tile-title {{ font-weight: 600; font-size: 0.9rem; color: #fff; }}
        .widya-tile-sub {{ font-size: 0.75rem; color: var(--text-muted); margin-top: 2px; }}
        .widya-tile-action {{ font-size: 1.1rem; color: var(--text-muted); }}

        .widya-switch-wrapper {{ display: flex; align-items: center; justify-content: space-between; width: 100%; padding: 6px 0; }}
        .widya-switch {{
            width: 48px;
            height: 26px;
            background: #374151;
            border-radius: 13px;
            position: relative;
            cursor: pointer;
            transition: background 0.3s ease;
        }}
        .widya-switch.active {{ background: #6366f1; }}
        .widya-switch-thumb {{
            width: 20px;
            height: 20px;
            background: #ffffff;
            border-radius: 50%;
            position: absolute;
            top: 3px;
            left: 3px;
            transition: transform 0.25s cubic-bezier(0.4, 0, 0.2, 1);
            box-shadow: 0 2px 4px rgba(0,0,0,0.3);
        }}
        .widya-switch.active .widya-switch-thumb {{ transform: translateX(22px); }}

        .widya-progress-container {{ width: 100%; background: #374151; border-radius: 8px; height: 8px; overflow: hidden; }}
        .widya-progress-bar {{ height: 100%; background: var(--primary-gradient); border-radius: 8px; transition: width 0.4s ease; }}

        .widya-badge {{ display: inline-flex; align-items: center; gap: 4px; padding: 4px 10px; border-radius: 20px; font-size: 0.72rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.04em; }}
        .widya-badge-hijau {{ background: rgba(16, 185, 129, 0.15); color: #34d399; border: 1px solid rgba(16, 185, 129, 0.3); }}
        .widya-badge-biru {{ background: rgba(59, 130, 246, 0.15); color: #60a5fa; border: 1px solid rgba(59, 130, 246, 0.3); }}
        .widya-badge-merah {{ background: rgba(239, 68, 68, 0.15); color: #f87171; border: 1px solid rgba(239, 68, 68, 0.3); }}
        .widya-badge-kuning {{ background: rgba(245, 158, 11, 0.15); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.3); }}

        .widya-avatar {{
            width: 44px;
            height: 44px;
            border-radius: 50%;
            background: linear-gradient(135deg, #4f46e5, #ec4899);
            color: #fff;
            font-weight: 700;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1rem;
            position: relative;
            box-shadow: 0 4px 10px rgba(0,0,0,0.3);
        }}
        .widya-avatar-online {{ position: absolute; bottom: 0; right: 0; width: 12px; height: 12px; background: #10b981; border: 2px solid var(--bg-frame); border-radius: 50%; }}

        .widya-fab {{
            position: absolute;
            bottom: 84px;
            right: 20px;
            width: 56px;
            height: 56px;
            border-radius: 28px;
            background: var(--primary-gradient);
            color: #ffffff;
            box-shadow: 0 8px 24px rgba(99,102,241,0.5);
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.5rem;
            border: none;
            cursor: pointer;
            z-index: 45;
            transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
        }}
        .widya-fab:active {{ transform: scale(0.92); }}

        .widya-bottom-nav {{
            height: 68px;
            background: rgba(17, 24, 39, 0.92);
            backdrop-filter: blur(20px);
            border-top: 1px solid var(--border-card);
            display: flex;
            align-items: center;
            justify-content: space-around;
            padding: 0 12px;
            z-index: 50;
        }}
        .widya-nav-item {{
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 4px;
            color: var(--text-muted);
            font-size: 0.7rem;
            font-weight: 600;
            cursor: pointer;
            transition: color 0.2s ease, transform 0.2s ease;
            text-decoration: none;
        }}
        .widya-nav-item.active {{ color: #818cf8; transform: translateY(-2px); }}
        .widya-nav-icon {{ font-size: 1.25rem; }}

        .widya-pemisah {{ width: 100%; height: 16px; }}
        .widya-gambar {{ width: 100%; border-radius: 14px; object-fit: cover; }}
        .widya-daftar-gulir {{ display: flex; flex-direction: column; gap: 12px; overflow-y: auto; }}

        @media (max-width: 600px) {{
            body {{ padding: 0 !important; margin: 0 !important; background: var(--bg-frame); }}
            .widya-app-frame {{
                max-width: 100% !important;
                height: 100vh !important;
                border-radius: 0 !important;
                border: none !important;
                box-shadow: none !important;
            }}
            .widya-notch {{ display: none !important; }}
            .widya-status-bar {{ display: none !important; }}
            .widya-body {{
                padding-top: calc(var(--sat) + 12px) !important;
                padding-bottom: calc(var(--sab) + 72px) !important;
            }}
            .widya-bottom-nav {{
                padding-bottom: var(--sab) !important;
                height: calc(64px + var(--sab)) !important;
            }}
        }}
    </style>
</head>
<body>
    <div class="widya-app-frame">
        <div class="widya-notch"></div>
        <div class="widya-status-bar">
            <span id="widyaLiveClock">09:41</span>
            <div class="widya-status-icons">
                <span>5G</span>
                <span>📶</span>
                <span>98% ⚡</span>
            </div>
        </div>
        <div class="widya-appbar">
            <div class="widya-appbar-title">{}</div>
            <div class="widya-appbar-badge">WidyaUI</div>
        </div>
        <div class="widya-body">
            {}
        </div>
        {}
        {}
    </div>
    <script>
        (function() {{
            function updateClock() {{
                const el = document.getElementById('widyaLiveClock');
                if (!el) return;
                const now = new Date();
                const h = String(now.getHours()).padStart(2, '0');
                const m = String(now.getMinutes()).padStart(2, '0');
                el.textContent = h + ':' + m;
            }}
            updateClock();
            setInterval(updateClock, 30000);
        }})();
    </script>
</body>
</html>"##,
                    judul, judul, badan, fab, nav_bawah
                )
            }
            "Halaman" => {
                let navigasi = map.get("navigasi").map(render_widget_html).unwrap_or_default();
                let badan = map.get("badan").map(render_widget_html).unwrap_or_default();
                format!("{}{}", navigasi, badan)
            }
            "BilahAtas" => {
                let judul = map.get("judul").map(|v| v.to_string_repr()).unwrap_or_else(|| "Widya App".to_string());
                let warna = map.get("warna").map(|v| v.to_string_repr()).unwrap_or_else(|| "#4f46e5".to_string());
                format!(
                    r#"<div class="widya-top-bar" style="background:{};padding:14px 18px;font-weight:700;font-size:16px;color:#ffffff;border-radius:12px;margin-bottom:12px;display:flex;align-items:center;box-shadow:0 4px 12px rgba(0,0,0,0.15);">{}</div>"#,
                    warna, judul
                )
            }
            "Kolom" => {
                let mut children_html = String::new();
                if let Some(Value::Array(a)) = map.get("anak") {
                    for child in a.borrow().iter() {
                        children_html.push_str(&render_widget_html(child));
                    }
                }
                format!("<div class=\"widya-kolom\">{}</div>", children_html)
            }
            "Baris" => {
                let mut children_html = String::new();
                if let Some(Value::Array(a)) = map.get("anak") {
                    for child in a.borrow().iter() {
                        children_html.push_str(&render_widget_html(child));
                    }
                }
                format!("<div class=\"widya-baris\">{}</div>", children_html)
            }
            "Teks" => {
                let teks = map.get("teks").map(|v| v.to_string_repr()).unwrap_or_default();
                format!("<div class=\"widya-teks\">{}</div>", teks)
            }
            "Tombol" => {
                let label = map.get("label").map(|v| v.to_string_repr()).unwrap_or_else(|| "Tombol".to_string());
                format!(r#"<button class="widya-tombol" onclick="if(window.WidyaMobile)window.WidyaMobile.vibrate(40);console.log('Klik: {}');">{}</button>"#, label, label)
            }
            "TombolUtama" => {
                let label = map.get("label").map(|v| v.to_string_repr()).unwrap_or_else(|| "Kirim".to_string());
                format!(
                    r#"<button class="widya-tombol widya-tombol-utama" style="background:var(--primary, #6366f1);color:#ffffff;font-weight:700;padding:12px 18px;border-radius:10px;border:none;cursor:pointer;width:100%;box-shadow:0 4px 14px rgba(99,102,241,0.35);margin-top:8px;" onclick="if(window.WidyaMobile)window.WidyaMobile.vibrate(60);console.log('Klik Utama: {}');">{}</button>"#,
                    label, label
                )
            }
            "BidangTeks" => {
                let hint = map.get("petunjuk").map(|v| v.to_string_repr()).unwrap_or_else(|| "Ketik di sini...".to_string());
                format!("<input class=\"widya-bidang-teks\" placeholder=\"{}\" />", hint)
            }
            "Kartu" => {
                let isi = map.get("isi").or_else(|| map.get("anak")).map(render_widget_html).unwrap_or_default();
                format!("<div class=\"widya-kartu\">{}</div>", isi)
            }
            "ItemDaftar" => {
                let judul = map.get("judul").map(|v| v.to_string_repr()).unwrap_or_default();
                let subjudul = map.get("subjudul").map(|v| v.to_string_repr()).unwrap_or_default();
                let ikon = map.get("ikon_awal").map(|v| v.to_string_repr()).unwrap_or_else(|| "📌".to_string());
                let aksi = map.get("aksi_akhir").map(|v| v.to_string_repr()).unwrap_or_else(|| "›".to_string());
                format!(
                    r#"<div class="widya-list-tile" onclick="if(window.WidyaMobile)window.WidyaMobile.vibrate(30);"><div class="widya-tile-icon">{}</div><div class="widya-tile-content"><div class="widya-tile-title">{}</div><div class="widya-tile-sub">{}</div></div><div class="widya-tile-action">{}</div></div>"#,
                    ikon, judul, subjudul, aksi
                )
            }
            "Lencana" => {
                let teks = map.get("teks").map(|v| v.to_string_repr()).unwrap_or_default();
                let warna = map.get("warna").map(|v| v.to_string_repr()).unwrap_or_else(|| "biru".to_string());
                format!(r#"<span class="widya-badge widya-badge-{}">{}</span>"#, warna, teks)
            }
            "Sakelar" => {
                let label = map.get("label").map(|v| v.to_string_repr()).unwrap_or_default();
                let aktif = map.get("aktif").map(|v| v.is_truthy()).unwrap_or(false);
                let active_cls = if aktif { "active" } else { "" };
                format!(
                    r#"<div class="widya-switch-wrapper"><span>{}</span><div class="widya-switch {}" onclick="this.classList.toggle('active');if(window.WidyaMobile)window.WidyaMobile.vibrate(40);"><div class="widya-switch-thumb"></div></div></div>"#,
                    label, active_cls
                )
            }
            "IndikatorProgres" => {
                let nilai = map.get("nilai").and_then(|v| match v { Value::Number(n) => Some(*n), _ => None }).unwrap_or(50.0).clamp(0.0, 100.0);
                format!(
                    r#"<div class="widya-progress-container"><div class="widya-progress-bar" style="width: {}%;"></div></div>"#,
                    nilai
                )
            }
            "Avatar" => {
                let nama = map.get("nama").map(|v| v.to_string_repr()).unwrap_or_else(|| "W".to_string());
                let online = map.get("status_online").map(|v| v.is_truthy()).unwrap_or(false);
                let initial = nama.chars().next().unwrap_or('W').to_uppercase();
                let dot = if online { r#"<div class="widya-avatar-online"></div>"# } else { "" };
                format!(r#"<div class="widya-avatar">{} {}</div>"#, initial, dot)
            }
            "BilahNavigasiBawah" => {
                let mut items_html = String::new();
                if let Some(Value::Array(a)) = map.get("items") {
                    for (idx, item_val) in a.borrow().iter().enumerate() {
                        if let Value::Map(im) = item_val {
                            let item_map = im.borrow();
                            let label = item_map.get("label").map(|v| v.to_string_repr()).unwrap_or_default();
                            let ikon = item_map.get("ikon").map(|v| v.to_string_repr()).unwrap_or_else(|| "●".to_string());
                            let active_cls = if item_map.get("aktif").map(|v| v.is_truthy()).unwrap_or(idx == 0) { "active" } else { "" };
                            items_html.push_str(&format!(
                                r#"<div class="widya-nav-item {}" onclick="document.querySelectorAll('.widya-nav-item').forEach(e=>e.classList.remove('active'));this.classList.add('active');if(window.WidyaMobile)window.WidyaMobile.vibrate(30);"><div class="widya-nav-icon">{}</div><span>{}</span></div>"#,
                                active_cls, ikon, label
                            ));
                        }
                    }
                }
                format!(r#"<div class="widya-bottom-nav">{}</div>"#, items_html)
            }
            "TombolAksiMengambang" => {
                let ikon = map.get("ikon").map(|v| v.to_string_repr()).unwrap_or_else(|| "+".to_string());
                format!(
                    r#"<button class="widya-fab" onclick="if(window.WidyaMobile)window.WidyaMobile.vibrate(60);">{}</button>"#,
                    ikon
                )
            }
            "LembarBawah" => {
                let judul = map.get("judul").map(|v| v.to_string_repr()).unwrap_or_default();
                let isi = map.get("isi").map(render_widget_html).unwrap_or_default();
                format!(
                    r#"<div class="widya-bottom-sheet"><div class="widya-sheet-handle"></div><div class="widya-sheet-title">{}</div><div class="widya-sheet-body">{}</div></div>"#,
                    judul, isi
                )
            }
            "Gambar" => {
                let url = map.get("url").map(|v| v.to_string_repr()).unwrap_or_default();
                format!("<img class=\"widya-gambar\" src=\"{}\" alt=\"Gambar\" />", url)
            }
            "DaftarGulir" => {
                let mut children_html = String::new();
                if let Some(Value::Array(a)) = map.get("anak") {
                    for child in a.borrow().iter() {
                        children_html.push_str(&render_widget_html(child));
                    }
                }
                format!("<div class=\"widya-daftar-gulir\">{}</div>", children_html)
            }
            "Pemisah" => "<div class=\"widya-pemisah\"></div>".to_string(),
            _ => format!("<div class=\"widya-widget\">{}</div>", w.to_string_repr()),
        }
    } else {
        format!("<div class=\"widya-teks\">{}</div>", w.to_string_repr())
    }
}
fn builtin_ui_render_html(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let html = render_widget_html(&args[0]);
    Ok(Value::String(html))
}


pub fn register_ui(env: &mut Environment) {
    register_fn(env, "Aplikasi", Some(1), builtin_ui_aplikasi);
    register_fn(env, "Halaman", Some(1), builtin_ui_halaman);
    register_fn(env, "Kolom", None, builtin_ui_kolom);
    register_fn(env, "Baris", None, builtin_ui_baris);
    register_fn(env, "TeksWidget", None, builtin_ui_teks);
    register_fn(env, "Tombol", Some(2), builtin_ui_tombol);
    register_fn(env, "BidangTeks", Some(1), builtin_ui_bidang_teks);
    register_fn(env, "Kontainer", Some(1), builtin_ui_kontainer);
    register_fn(env, "Kartu", Some(1), builtin_ui_kartu);
    register_fn(env, "Pemisah", None, builtin_ui_pemisah);
    register_fn(env, "Gambar", None, builtin_ui_gambar);
    register_fn(env, "DaftarGulir", Some(1), builtin_ui_daftar_gulir);
    register_fn(env, "ItemDaftar", Some(1), builtin_ui_item_daftar);
    register_fn(env, "Lencana", Some(1), builtin_ui_lencana);
    register_fn(env, "Sakelar", Some(1), builtin_ui_sakelar);
    register_fn(env, "IndikatorProgres", Some(1), builtin_ui_progres);
    register_fn(env, "BilahNavigasiBawah", Some(1), builtin_ui_bilah_navigasi);
    register_fn(env, "BilahAtas", None, builtin_ui_bilah_atas);
    register_fn(env, "TombolUtama", None, builtin_ui_tombol_utama);
    register_fn(env, "TombolAksiMengambang", Some(1), builtin_ui_fab);
    register_fn(env, "LembarBawah", Some(1), builtin_ui_lembar_bawah);
    register_fn(env, "Avatar", Some(1), builtin_ui_avatar);
    register_fn(env, "AvatarWidget", Some(1), builtin_ui_avatar);
    register_fn(env, "render_html", Some(1), builtin_ui_render_html);
}
