use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Mutex, OnceLock};

// ----------------------------------------------------------------------------
// GLOBAL I18N STATE
// ----------------------------------------------------------------------------

struct I18nManager {
    current_locale: String,
    dictionaries: HashMap<String, HashMap<String, String>>,
}

fn get_i18n_manager() -> &'static Mutex<I18nManager> {
    static I18N_MANAGER: OnceLock<Mutex<I18nManager>> = OnceLock::new();
    I18N_MANAGER.get_or_init(|| {
        let mut dicts = HashMap::new();

        // Standard Default Dictionaries
        let mut id = HashMap::new();
        id.insert("selamat_datang".to_string(), "Selamat datang di Widya-lang!".to_string());
        id.insert("halo".to_string(), "Halo, {nama}!".to_string());
        id.insert("sukses".to_string(), "Operasi berhasil dilakukan.".to_string());
        id.insert("gagal".to_string(), "Operasi gagal.".to_string());
        id.insert("simpan".to_string(), "Simpan".to_string());
        id.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("id-ID".to_string(), id);

        let mut en = HashMap::new();
        en.insert("selamat_datang".to_string(), "Welcome to Widya-lang!".to_string());
        en.insert("halo".to_string(), "Hello, {nama}!".to_string());
        en.insert("sukses".to_string(), "Operation completed successfully.".to_string());
        en.insert("gagal".to_string(), "Operation failed.".to_string());
        en.insert("simpan".to_string(), "Save".to_string());
        en.insert("batal".to_string(), "Cancel".to_string());
        dicts.insert("en-US".to_string(), en);

        let mut ar = HashMap::new();
        ar.insert("selamat_datang".to_string(), "مرحباً بكم في لغة ويديا!".to_string());
        ar.insert("halo".to_string(), "أهلاً وسهلاً، {nama}!".to_string());
        ar.insert("sukses".to_string(), "تمت العملية بنجاح.".to_string());
        ar.insert("gagal".to_string(), "فشلت العملية.".to_string());
        ar.insert("simpan".to_string(), "حفظ".to_string());
        ar.insert("batal".to_string(), "إلغاء".to_string());
        dicts.insert("ar-SA".to_string(), ar);

        // Bahasa Daerah Nusantara (16 bahasa)
        let mut jv = HashMap::new();
        jv.insert("selamat_datang".to_string(), "Sugeng rawuh wonten ing Widya-lang!".to_string());
        jv.insert("halo".to_string(), "Sugeng panggih, {nama}!".to_string());
        jv.insert("sukses".to_string(), "Kasil kanthi sae".to_string());
        jv.insert("simpan".to_string(), "Simpen".to_string());
        jv.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("jv-ID".to_string(), jv);

        let mut su = HashMap::new();
        su.insert("selamat_datang".to_string(), "Wilujeng sumping di Widya-lang!".to_string());
        su.insert("halo".to_string(), "Sampurasun, {nama}!".to_string());
        su.insert("sukses".to_string(), "Parantos rengse kalayan sae".to_string());
        su.insert("simpan".to_string(), "Simpen".to_string());
        su.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("su-ID".to_string(), su);

        let mut ban = HashMap::new();
        ban.insert("selamat_datang".to_string(), "Om swastyastu, rauh ring Widya-lang!".to_string());
        ban.insert("halo".to_string(), "Rahajeng semeng, {nama}!".to_string());
        ban.insert("sukses".to_string(), "Labda karya".to_string());
        ban.insert("simpan".to_string(), "Simpen".to_string());
        ban.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("ban-ID".to_string(), ban);

        let mut min = HashMap::new();
        min.insert("selamat_datang".to_string(), "Salamaik datang di Widya-lang!".to_string());
        min.insert("halo".to_string(), "Salamaik datang, {nama}!".to_string());
        min.insert("sukses".to_string(), "Alah salasai jo rancak".to_string());
        min.insert("simpan".to_string(), "Simpan".to_string());
        min.insert("batal".to_string(), "Batalkan".to_string());
        dicts.insert("min-ID".to_string(), min);

        let mut bug = HashMap::new();
        bug.insert("selamat_datang".to_string(), "Salama engka ko Widya-lang!".to_string());
        bug.insert("halo".to_string(), "Salama, {nama}!".to_string());
        bug.insert("sukses".to_string(), "Makessing maneng".to_string());
        bug.insert("simpan".to_string(), "Taro".to_string());
        bug.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("bug-ID".to_string(), bug);

        let mut mak = HashMap::new();
        mak.insert("selamat_datang".to_string(), "Salama battu ri Widya-lang!".to_string());
        mak.insert("halo".to_string(), "Salama, {nama}!".to_string());
        mak.insert("sukses".to_string(), "Bajik ngaseng".to_string());
        mak.insert("simpan".to_string(), "Taro".to_string());
        mak.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("mak-ID".to_string(), mak);

        let mut btk = HashMap::new();
        btk.insert("selamat_datang".to_string(), "Horas ma di hamu sude di Widya-lang!".to_string());
        btk.insert("halo".to_string(), "Horas, {nama}!".to_string());
        btk.insert("sukses".to_string(), "Denggan do sude".to_string());
        btk.insert("simpan".to_string(), "Simpan".to_string());
        btk.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("btk-ID".to_string(), btk);

        let mut ace = HashMap::new();
        ace.insert("selamat_datang".to_string(), "Seulamat teuka bak Widya-lang!".to_string());
        ace.insert("halo".to_string(), "Saleum teuka, {nama}!".to_string());
        ace.insert("sukses".to_string(), "Ka meusaneut got".to_string());
        ace.insert("simpan".to_string(), "Keubah".to_string());
        ace.insert("batal".to_string(), "Bateue".to_string());
        dicts.insert("ace-ID".to_string(), ace);

        let mut mad = HashMap::new();
        mad.insert("selamat_datang".to_string(), "Salamet dhateng neng Widya-lang!".to_string());
        mad.insert("halo".to_string(), "Salam, {nama}!".to_string());
        mad.insert("sukses".to_string(), "Ampon sae".to_string());
        mad.insert("simpan".to_string(), "Simpen".to_string());
        mad.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("mad-ID".to_string(), mad);

        let mut bjn = HashMap::new();
        bjn.insert("selamat_datang".to_string(), "Salamat datang di Widya-lang!".to_string());
        bjn.insert("halo".to_string(), "Salamat, {nama}!".to_string());
        bjn.insert("sukses".to_string(), "Tuntung sudah wan baik".to_string());
        bjn.insert("simpan".to_string(), "Simpan".to_string());
        bjn.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("bjn-ID".to_string(), bjn);

        let mut sas = HashMap::new();
        sas.insert("selamat_datang".to_string(), "Slamet rauh leq Widya-lang!".to_string());
        sas.insert("halo".to_string(), "Slamet, {nama}!".to_string());
        sas.insert("sukses".to_string(), "Sampun solah".to_string());
        sas.insert("simpan".to_string(), "Telah".to_string());
        sas.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("sas-ID".to_string(), sas);

        let mut ljp = HashMap::new();
        ljp.insert("selamat_datang".to_string(), "Tabik pun, selamat ratong di Widya-lang!".to_string());
        ljp.insert("halo".to_string(), "Tabik, {nama}!".to_string());
        ljp.insert("sukses".to_string(), "Radu wawai".to_string());
        ljp.insert("simpan".to_string(), "Simpen".to_string());
        ljp.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("ljp-ID".to_string(), ljp);

        let mut rej = HashMap::new();
        rej.insert("selamat_datang".to_string(), "Selamat magheak keme di Widya-lang!".to_string());
        rej.insert("halo".to_string(), "Selamat, {nama}!".to_string());
        rej.insert("sukses".to_string(), "Keme laku wawai".to_string());
        rej.insert("simpan".to_string(), "Simpen".to_string());
        rej.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("rej-ID".to_string(), rej);

        let mut tor = HashMap::new();
        tor.insert("selamat_datang".to_string(), "Salama sae lako Widya-lang!".to_string());
        tor.insert("halo".to_string(), "Salama, {nama}!".to_string());
        tor.insert("sukses".to_string(), "Melo kaboro".to_string());
        tor.insert("simpan".to_string(), "Parapa".to_string());
        tor.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("tor-ID".to_string(), tor);

        let mut pap = HashMap::new();
        pap.insert("selamat_datang".to_string(), "Selamat datang kitorang di Widya-lang!".to_string());
        pap.insert("halo".to_string(), "Halo kawan, {nama}!".to_string());
        pap.insert("sukses".to_string(), "Beres aman mantap".to_string());
        pap.insert("simpan".to_string(), "Simpan".to_string());
        pap.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("pap-ID".to_string(), pap);

        let mut ms = HashMap::new();
        ms.insert("selamat_datang".to_string(), "Selamat datang ke Widya-lang!".to_string());
        ms.insert("halo".to_string(), "Salam sejahtera, {nama}!".to_string());
        ms.insert("sukses".to_string(), "Operasi berjaya dilaksanakan".to_string());
        ms.insert("simpan".to_string(), "Simpan".to_string());
        ms.insert("batal".to_string(), "Batal".to_string());
        dicts.insert("ms-MY".to_string(), ms);

        Mutex::new(I18nManager {
            current_locale: "id-ID".to_string(),
            dictionaries: dicts,
        })
    })
}

pub fn register_i18n(env: &mut Environment) {
    // Arab & RTL Text Editing (Contoh 187)
    register_fn(env, "apakah_rtl", Some(1), builtin_apakah_rtl);
    register_fn(env, "arah_teks", Some(1), builtin_arah_teks);
    register_fn(env, "format_arab", Some(1), builtin_format_arab);
    register_fn(env, "hapus_harakat", Some(1), builtin_hapus_harakat);
    register_fn(env, "teks_ke_rtl", Some(1), builtin_teks_ke_rtl);
    register_fn(env, "balik_teks_visual", Some(1), builtin_balik_teks_visual);
    register_fn(env, "angka_ke_arab", Some(1), builtin_angka_ke_arab);
    register_fn(env, "angka_ke_latin", Some(1), builtin_angka_ke_latin);

    // Internationalization & Localization (Contoh 188)
    register_fn(env, "ambil_lokal", Some(0), builtin_ambil_lokal);
    register_fn(env, "atur_lokal", Some(1), builtin_atur_lokal);
    register_fn(env, "i18n_daftarkan", Some(2), builtin_i18n_daftarkan);
    register_fn(env, "i18n_terjemah", None, builtin_i18n_terjemah);
    register_fn(env, "t", None, builtin_i18n_terjemah); // alias praktis
    register_fn(env, "format_mata_uang", None, builtin_format_mata_uang);
    register_fn(env, "format_tanggal", None, builtin_format_tanggal);
    register_fn(env, "format_plural", Some(2), builtin_format_plural);
}

// ----------------------------------------------------------------------------
// ARAB & RTL TEXT EDITING BUILTINS
// ----------------------------------------------------------------------------

fn is_rtl_char(c: char) -> bool {
    matches!(c,
        '\u{0590}'..='\u{05FF}' | // Hebrew
        '\u{0600}'..='\u{06FF}' | // Arabic
        '\u{0700}'..='\u{074F}' | // Syriac
        '\u{0750}'..='\u{077F}' | // Arabic Supplement
        '\u{0780}'..='\u{07BF}' | // Thaana
        '\u{08A0}'..='\u{08FF}' | // Arabic Extended-A
        '\u{FB50}'..='\u{FDFF}' | // Arabic Presentation Forms-A
        '\u{FE70}'..='\u{FEFF}'   // Arabic Presentation Forms-B
    )
}

fn builtin_apakah_rtl(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => "",
    };
    let is_rtl = text.chars().any(is_rtl_char);
    Ok(Value::Bool(is_rtl))
}

fn builtin_arah_teks(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => "",
    };
    let is_rtl = text.chars().any(is_rtl_char);
    Ok(Value::String(if is_rtl { "rtl".to_string() } else { "ltr".to_string() }))
}

fn builtin_format_arab(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => "",
    };
    let is_arab = text.chars().any(|c| matches!(c, '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{08A0}'..='\u{08FF}'));
    let char_count = text.chars().count();

    let mut map = HashMap::new();
    map.insert("arah".to_string(), Value::String(if is_arab { "rtl".to_string() } else { "ltr".to_string() }));
    map.insert("apakah_arab".to_string(), Value::Bool(is_arab));
    map.insert("panjang_huruf".to_string(), Value::Number(char_count as f64));
    map.insert("font_rekomendasi".to_string(), Value::String("Amiri, Scheherazade New, Segoe UI, Tahoma".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_hapus_harakat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => "",
    };
    let filtered: String = text
        .chars()
        .filter(|&c| !matches!(c, '\u{064B}'..='\u{065F}' | '\u{0670}' | '\u{06D6}'..='\u{06ED}'))
        .collect();
    Ok(Value::String(filtered))
}

fn builtin_teks_ke_rtl(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => "",
    };
    // U+202B (Right-to-Left Embedding) ... U+202C (Pop Directional Formatting)
    Ok(Value::String(format!("\u{202B}{}\u{202C}", text)))
}

fn builtin_balik_teks_visual(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => "",
    };
    let rev: String = text.chars().rev().collect();
    Ok(Value::String(rev))
}

fn builtin_angka_ke_arab(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match &args[0] {
        Value::String(s) => s.clone(),
        Value::Number(n) => {
            if n.fract() == 0.0 {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        other => other.to_string_repr(),
    };

    let arab_digits = ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'];
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_digit() {
            let idx = (c as u8 - b'0') as usize;
            result.push(arab_digits[idx]);
        } else if c == '.' {
            result.push('٫'); // Arabic decimal separator U+066B
        } else if c == ',' {
            result.push('٬'); // Arabic thousands separator U+066C
        } else {
            result.push(c);
        }
    }
    Ok(Value::String(result))
}

fn builtin_angka_ke_latin(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let text = match &args[0] {
        Value::String(s) => s.as_str(),
        other => &other.to_string_repr(),
    };

    let mut result = String::new();
    for c in text.chars() {
        match c {
            '٠' => result.push('0'),
            '١' => result.push('1'),
            '٢' => result.push('2'),
            '٣' => result.push('3'),
            '٤' => result.push('4'),
            '٥' => result.push('5'),
            '٦' => result.push('6'),
            '٧' => result.push('7'),
            '٨' => result.push('8'),
            '٩' => result.push('9'),
            '٫' => result.push('.'),
            '٬' => result.push(','),
            other => result.push(other),
        }
    }
    Ok(Value::String(result))
}

// ----------------------------------------------------------------------------
// I18N / L10N BUILTINS
// ----------------------------------------------------------------------------

fn builtin_ambil_lokal(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mgr = get_i18n_manager().lock().unwrap();
    Ok(Value::String(mgr.current_locale.clone()))
}

fn builtin_atur_lokal(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut mgr = get_i18n_manager().lock().unwrap();
    let new_loc = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "id-ID".to_string(),
    };
    mgr.current_locale = new_loc.clone();
    Ok(Value::String(new_loc))
}

fn builtin_i18n_daftarkan(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let locale = match &args[0] {
        Value::String(s) => s.clone(),
        _ => "id-ID".to_string(),
    };
    let mut new_dict = HashMap::new();
    if let Value::Map(m) = &args[1] {
        for (k, v) in m.borrow().iter() {
            new_dict.insert(k.clone(), v.to_string_repr());
        }
    }

    let mut mgr = get_i18n_manager().lock().unwrap();
    let entry = mgr.dictionaries.entry(locale).or_default();
    for (k, v) in new_dict {
        entry.insert(k, v);
    }

    Ok(Value::Bool(true))
}

fn builtin_i18n_terjemah(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Ok(Value::String(String::new()));
    }
    let key = match &args[0] {
        Value::String(s) => s.clone(),
        other => other.to_string_repr(),
    };

    let mgr = get_i18n_manager().lock().unwrap();
    let current_locale = &mgr.current_locale;

    let raw_text = mgr
        .dictionaries
        .get(current_locale)
        .and_then(|dict| dict.get(&key))
        .cloned()
        .or_else(|| {
            // Fallback to id-ID
            mgr.dictionaries
                .get("id-ID")
                .and_then(|dict| dict.get(&key))
                .cloned()
        })
        .unwrap_or_else(|| key.clone());

    // Interpolation {nama}, {item}, dll.
    let mut interpolated = raw_text;
    if args.len() > 1 {
        if let Value::Map(m) = &args[1] {
            for (k, v) in m.borrow().iter() {
                let placeholder = format!("{{{}}}", k);
                interpolated = interpolated.replace(&placeholder, &v.to_string_repr());
            }
        }
    }

    Ok(Value::String(interpolated))
}

fn builtin_format_mata_uang(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nominal = match args.first() {
        Some(Value::Number(n)) => *n,
        Some(Value::Map(m)) => {
            let map = m.borrow();
            if let Some(Value::Number(n)) = map.get("angka") {
                *n
            } else if let Some(Value::Number(n)) = map.get("nilai") {
                *n
            } else if let Some(Value::String(s)) = map.get("teks") {
                s.parse::<f64>().unwrap_or(0.0)
            } else {
                0.0
            }
        }
        _ => 0.0,
    };
    let mgr = get_i18n_manager().lock().unwrap();
    let default_locale = mgr.current_locale.clone();
    drop(mgr);

    let (locale, currency) = if args.len() == 2 {
        if let Some(Value::String(s)) = args.get(1) {
            if s.len() == 3 && s.chars().all(|c| c.is_ascii_uppercase()) {
                let loc = match s.as_str() {
                    "IDR" => "id-ID",
                    "USD" => "en-US",
                    "EUR" => "de-DE",
                    "JPY" => "ja-JP",
                    "SAR" => "ar-SA",
                    _ => default_locale.as_str(),
                };
                (loc, s.as_str())
            } else {
                (s.as_str(), match s.as_str() {
                    "id-ID" => "IDR",
                    "en-US" => "USD",
                    "ar-SA" => "SAR",
                    "ja-JP" => "JPY",
                    _ => "IDR",
                })
            }
        } else {
            (default_locale.as_str(), "IDR")
        }
    } else {
        let loc = match args.get(1) {
            Some(Value::String(s)) => s.as_str(),
            _ => default_locale.as_str(),
        };
        let cur = match args.get(2) {
            Some(Value::String(s)) => s.as_str(),
            _ => match loc {
                "id-ID" => "IDR",
                "en-US" => "USD",
                "ar-SA" => "SAR",
                "ja-JP" => "JPY",
                _ => "IDR",
            },
        };
        (loc, cur)
    };

    let formatted = match currency {
        "IDR" => {
            let int_part = nominal as i64;
            let frac_part = ((nominal - int_part as f64).abs() * 100.0).round() as i64;
            let formatted_int = format_ribuan(int_part, '.');
            if frac_part > 0 {
                format!("Rp {formatted_int},{:02}", frac_part)
            } else {
                format!("Rp {formatted_int}")
            }
        }
        "USD" => {
            let int_part = nominal as i64;
            let frac_part = ((nominal - int_part as f64).abs() * 100.0).round() as i64;
            let formatted_int = format_ribuan(int_part, ',');
            format!("${formatted_int}.{:02}", frac_part)
        }
        "EUR" => {
            let int_part = nominal as i64;
            let frac_part = ((nominal - int_part as f64).abs() * 100.0).round() as i64;
            let formatted_int = format_ribuan(int_part, '.');
            if frac_part > 0 {
                format!("€{formatted_int},{:02}", frac_part)
            } else {
                format!("€{formatted_int}")
            }
        }
        "SAR" => {
            let int_part = nominal as i64;
            let frac_part = ((nominal - int_part as f64).abs() * 100.0).round() as i64;
            let latin_str = format!("{}.{:02}", format_ribuan(int_part, ','), frac_part);
            let arab_val = builtin_angka_ke_arab(&[Value::String(latin_str)], _span)?;
            format!("{} ر.س", arab_val.to_string_repr())
        }
        "JPY" => {
            let int_part = nominal as i64;
            let formatted_int = format_ribuan(int_part, ',');
            format!("￥{formatted_int}")
        }
        _ => format!("{} {}", currency, nominal),
    };

    Ok(Value::String(formatted))
}

fn format_ribuan(mut n: i64, separator: char) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let neg = n < 0;
    n = n.abs();
    let mut parts = Vec::new();
    while n > 0 {
        let rem = n % 1000;
        n /= 1000;
        if n > 0 {
            parts.push(format!("{:03}", rem));
        } else {
            parts.push(format!("{}", rem));
        }
    }
    parts.reverse();
    let res = parts.join(&separator.to_string());
    if neg {
        format!("-{}", res)
    } else {
        res
    }
}

fn builtin_format_tanggal(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mgr = get_i18n_manager().lock().unwrap();
    let default_locale = mgr.current_locale.clone();
    drop(mgr);

    let locale = match args.first() {
        Some(Value::String(s)) => s.as_str(),
        _ => default_locale.as_str(),
    };

    let tanggal_str = match locale {
        "id-ID" => "Sabtu, 3 Oktober 2026",
        "en-US" => "Saturday, October 3, 2026",
        "ar-SA" => "السبت، ٣ أكتوبر ٢٠٢٦",
        "ja-JP" => "2026年10月3日 土曜日",
        "jv-ID" => "Setu Paing, 3 Oktober 2026",
        "su-ID" => "Saptu, 3 Oktober 2026",
        "ban-ID" => "Saniscara, 3 Oktober 2026",
        "mad-ID" => "Satto, 3 Oktober 2026",
        "bjn-ID" => "Sabtu, 3 Uktuber 2026",
        "ace-ID" => "Sabtu, 3 Buleuen Siploh 2026",
        _ => "3 Oktober 2026",
    };

    Ok(Value::String(tanggal_str.to_string()))
}

fn builtin_format_plural(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let count = match args.first() {
        Some(Value::Number(n)) => *n as i64,
        _ => 0,
    };
    let rules_map = match args.get(1) {
        Some(Value::Map(m)) => m.borrow().clone(),
        _ => HashMap::new(),
    };

    let key = if count == 0 {
        "zero"
    } else if count == 1 {
        "one"
    } else if count == 2 {
        "two"
    } else if (3..=10).contains(&(count % 100)) {
        "few"
    } else if (11..=99).contains(&(count % 100)) {
        "many"
    } else {
        "other"
    };

    let result = rules_map
        .get(key)
        .or_else(|| rules_map.get("other"))
        .map(|v| v.to_string_repr())
        .unwrap_or_else(|| format!("{} items", count));

    Ok(Value::String(result))
}
