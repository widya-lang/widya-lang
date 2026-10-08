use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::{get_startup_instant, register_fn};
use crate::value::Value;
use chrono::{DateTime, Datelike, FixedOffset, Timelike, Utc};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn builtin_waktu(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0));
    Ok(Value::Number(since_the_epoch.as_secs_f64()))
}

pub fn cari_info_zona_waktu(zona: &str) -> (&'static str, &'static str, &'static str, f64, i32) {
    let key = zona.to_lowercase().replace(' ', "_").replace('-', "_");
    match key.as_str() {
        "wib" | "jakarta" | "asia/jakarta" | "surabaya" | "bandung" | "medan" | "semarang" | "pontianak" | "jawa" | "sumatera" => {
            ("Asia/Jakarta", "WIB", "Jakarta (Waktu Indonesia Barat)", 7.0, 7 * 3600)
        }
        "wita" | "makassar" | "asia/makassar" | "denpasar" | "bali" | "balikpapan" | "manado" | "mataram" | "banjarmasin" | "sulawesi" => {
            ("Asia/Makassar", "WITA", "Makassar (Waktu Indonesia Tengah)", 8.0, 8 * 3600)
        }
        "wit" | "jayapura" | "asia/jayapura" | "ambon" | "sorong" | "merauke" | "manokwari" | "maluku" | "papua" => {
            ("Asia/Jayapura", "WIT", "Jayapura (Waktu Indonesia Timur)", 9.0, 9 * 3600)
        }
        "singapore" | "singapura" | "sgt" | "asia/singapore" => {
            ("Asia/Singapore", "SGT", "Singapura (Singapore Time)", 8.0, 8 * 3600)
        }
        "kuala_lumpur" | "malaysia" | "myt" | "asia/kuala_lumpur" => {
            ("Asia/Kuala_Lumpur", "MYT", "Kuala Lumpur (Malaysia Time)", 8.0, 8 * 3600)
        }
        "bangkok" | "hanoi" | "ict" | "asia/bangkok" | "vietnam" | "thailand" => {
            ("Asia/Bangkok", "ICT", "Bangkok / Hanoi (Indochina Time)", 7.0, 7 * 3600)
        }
        "tokyo" | "jst" | "asia/tokyo" | "jepang" | "osaka" | "kyoto" => {
            ("Asia/Tokyo", "JST", "Tokyo (Japan Standard Time)", 9.0, 9 * 3600)
        }
        "seoul" | "kst" | "asia/seoul" | "korea" => {
            ("Asia/Seoul", "KST", "Seoul (Korea Standard Time)", 9.0, 9 * 3600)
        }
        "beijing" | "hong_kong" | "shanghai" | "hkt" | "cst_asia" | "asia/hong_kong" | "asia/shanghai" | "china" | "tiongkok" => {
            ("Asia/Shanghai", "CST", "Beijing / Shanghai (China Standard Time)", 8.0, 8 * 3600)
        }
        "delhi" | "new_delhi" | "mumbai" | "ist" | "india" | "asia/kolkata" => {
            ("Asia/Kolkata", "IST", "New Delhi / Mumbai (India Standard Time)", 5.5, (5 * 3600) + 1800)
        }
        "dubai" | "gst" | "uae" | "asia/dubai" | "abu_dhabi" => {
            ("Asia/Dubai", "GST", "Dubai (Gulf Standard Time)", 4.0, 4 * 3600)
        }
        "riyadh" | "mecca" | "mekkah" | "ast" | "asia/riyadh" | "saudi" => {
            ("Asia/Riyadh", "AST", "Mekkah / Riyadh (Arabia Standard Time)", 3.0, 3 * 3600)
        }
        "london" | "gmt" | "utc" | "bst" | "europe/london" | "inggris" | "uk" => {
            ("Europe/London", "GMT", "London (Greenwich Mean Time / UTC)", 0.0, 0)
        }
        "paris" | "cet" | "berlin" | "amsterdam" | "roma" | "madrid" | "europe/paris" | "europe/berlin" | "eropa" => {
            ("Europe/Paris", "CET", "Paris / Berlin (Central European Time)", 1.0, 3600)
        }
        "moscow" | "moskow" | "msk" | "europe/moscow" | "rusia" => {
            ("Europe/Moscow", "MSK", "Moskow (Moscow Standard Time)", 3.0, 3 * 3600)
        }
        "cairo" | "kairo" | "eet" | "africa/cairo" | "mesir" => {
            ("Africa/Cairo", "EET", "Kairo (Eastern European Time)", 2.0, 2 * 3600)
        }
        "new_york" | "est" | "edt" | "america/new_york" | "amerika" | "ny" => {
            ("America/New_York", "EST", "New York (Eastern Standard Time)", -5.0, -5 * 3600)
        }
        "chicago" | "cst_us" | "cdt" | "america/chicago" => {
            ("America/Chicago", "CST", "Chicago (Central Standard Time)", -6.0, -6 * 3600)
        }
        "denver" | "mst" | "mdt" | "america/denver" => {
            ("America/Denver", "MST", "Denver (Mountain Standard Time)", -7.0, -7 * 3600)
        }
        "los_angeles" | "pst" | "pdt" | "america/los_angeles" | "california" | "la" => {
            ("America/Los_Angeles", "PST", "Los Angeles / California (Pacific Standard Time)", -8.0, -8 * 3600)
        }
        "sao_paulo" | "brt" | "america/sao_paulo" | "brazil" => {
            ("America/Sao_Paulo", "BRT", "São Paulo (Brasília Time)", -3.0, -3 * 3600)
        }
        "sydney" | "aest" | "aedt" | "australia/sydney" | "australia" => {
            ("Australia/Sydney", "AEST", "Sydney (Australian Eastern Time)", 10.0, 10 * 3600)
        }
        "auckland" | "nzst" | "pacific/auckland" | "new_zealand" => {
            ("Pacific/Auckland", "NZST", "Auckland (New Zealand Standard Time)", 12.0, 12 * 3600)
        }
        "honolulu" | "hst" | "pacific/honolulu" | "hawaii" => {
            ("Pacific/Honolulu", "HST", "Honolulu (Hawaii-Aleutian Standard Time)", -10.0, -10 * 3600)
        }
        _ => {
            // Default to local/WIB if unrecognized
            ("Asia/Jakarta", "WIB", "Jakarta (Waktu Indonesia Barat)", 7.0, 7 * 3600)
        }
    }
}

pub fn nama_hari_indonesia(weekday: u32) -> &'static str {
    match weekday {
        1 => "Senin",
        2 => "Selasa",
        3 => "Rabu",
        4 => "Kamis",
        5 => "Jumat",
        6 => "Sabtu",
        7 | 0 => "Minggu",
        _ => "Senin",
    }
}

pub fn nama_bulan_indonesia(month: u32) -> &'static str {
    match month {
        1 => "Januari", 2 => "Februari", 3 => "Maret", 4 => "April",
        5 => "Mei", 6 => "Juni", 7 => "Juli", 8 => "Agustus",
        9 => "September", 10 => "Oktober", 11 => "November", 12 => "Desember",
        _ => "Januari",
    }
}

pub fn bangun_map_waktu(dt: DateTime<FixedOffset>, nama_zona: &str, singkatan: &str, deskripsi: &str, offset_jam: f64) -> Value {
    let year = dt.year();
    let month = dt.month();
    let day = dt.day();
    let weekday = dt.weekday().number_from_monday();
    let hour = dt.hour();
    let min = dt.minute();
    let sec = dt.second();
    let epoch_sec = dt.timestamp();

    let offset_str = if offset_jam >= 0.0 {
        let h = offset_jam.floor() as i32;
        let m = ((offset_jam - (h as f64)) * 60.0).round() as i32;
        format!("+{h:02}:{m:02}")
    } else {
        let abs = offset_jam.abs();
        let h = abs.floor() as i32;
        let m = ((abs - (h as f64)) * 60.0).round() as i32;
        format!("-{h:02}:{m:02}")
    };

    let hari_teks = nama_hari_indonesia(weekday);
    let bulan_teks = nama_bulan_indonesia(month);
    let iso = format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}{offset_str}");
    let lengkap = format!("{hari_teks}, {day} {bulan_teks} {year:04} {hour:02}:{min:02}:{sec:02} {singkatan}");

    let ampm = if hour >= 12 { "PM" } else { "AM" };
    let hour12 = match hour % 12 {
        0 => 12,
        h => h,
    };
    let format_12 = format!("{hour12:02}:{min:02}:{sec:02} {ampm}");

    let mut map = HashMap::new();
    map.insert("nama_zona".to_string(), Value::String(nama_zona.to_string()));
    map.insert("singkatan".to_string(), Value::String(singkatan.to_string()));
    map.insert("deskripsi".to_string(), Value::String(deskripsi.to_string()));
    map.insert("utc_offset_jam".to_string(), Value::Number(offset_jam));
    map.insert("utc_offset_teks".to_string(), Value::String(offset_str));
    map.insert("tahun".to_string(), Value::Number(year as f64));
    map.insert("bulan".to_string(), Value::Number(month as f64));
    map.insert("hari".to_string(), Value::Number(day as f64));
    map.insert("nama_hari".to_string(), Value::String(hari_teks.to_string()));
    map.insert("nama_bulan".to_string(), Value::String(bulan_teks.to_string()));
    map.insert("jam".to_string(), Value::Number(hour as f64));
    map.insert("menit".to_string(), Value::Number(min as f64));
    map.insert("detik".to_string(), Value::Number(sec as f64));
    map.insert("epoch_detik".to_string(), Value::Number(epoch_sec as f64));
    map.insert("format_iso".to_string(), Value::String(iso));
    map.insert("format_lengkap".to_string(), Value::String(lengkap));
    map.insert("format_12jam".to_string(), Value::String(format_12));

    Value::Map(Rc::new(RefCell::new(map)))
}

pub fn builtin_daftar_zona_waktu_dunia(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let zones = vec![
        ("Asia/Jakarta", "WIB", "Waktu Indonesia Barat (Jakarta, Surabaya, Medan)", 7.0, "+07:00", "Indonesia / Asia Tenggara"),
        ("Asia/Makassar", "WITA", "Waktu Indonesia Tengah (Makassar, Denpasar, Balikpapan)", 8.0, "+08:00", "Indonesia / Asia Tenggara"),
        ("Asia/Jayapura", "WIT", "Waktu Indonesia Timur (Jayapura, Ambon, Sorong)", 9.0, "+09:00", "Indonesia / Asia Tenggara"),
        ("Asia/Singapore", "SGT", "Singapura (Singapore Time)", 8.0, "+08:00", "Asia Tenggara"),
        ("Asia/Kuala_Lumpur", "MYT", "Kuala Lumpur (Malaysia Time)", 8.0, "+08:00", "Asia Tenggara"),
        ("Asia/Bangkok", "ICT", "Bangkok / Hanoi (Indochina Time)", 7.0, "+07:00", "Asia Tenggara"),
        ("Asia/Tokyo", "JST", "Tokyo (Japan Standard Time)", 9.0, "+09:00", "Asia Timur"),
        ("Asia/Seoul", "KST", "Seoul (Korea Standard Time)", 9.0, "+09:00", "Asia Timur"),
        ("Asia/Shanghai", "CST", "Beijing / Shanghai (China Standard Time)", 8.0, "+08:00", "Asia Timur"),
        ("Asia/Kolkata", "IST", "New Delhi / Mumbai (India Standard Time)", 5.5, "+05:30", "Asia Selatan"),
        ("Asia/Dubai", "GST", "Dubai (Gulf Standard Time)", 4.0, "+04:00", "Timur Tengah"),
        ("Asia/Riyadh", "AST", "Mekkah / Riyadh (Arabia Standard Time)", 3.0, "+03:00", "Timur Tengah"),
        ("Europe/London", "GMT", "London (Greenwich Mean Time / UTC)", 0.0, "+00:00", "Eropa Barat"),
        ("Europe/Paris", "CET", "Paris / Berlin / Roma (Central European Time)", 1.0, "+01:00", "Eropa Barat"),
        ("Europe/Moscow", "MSK", "Moskow (Moscow Standard Time)", 3.0, "+03:00", "Eropa Timur"),
        ("Africa/Cairo", "EET", "Kairo (Eastern European Time)", 2.0, "+02:00", "Afrika Utara"),
        ("America/New_York", "EST", "New York (Eastern Standard Time)", -5.0, "-05:00", "Amerika Utara"),
        ("America/Chicago", "CST", "Chicago (Central Standard Time)", -6.0, "-06:00", "Amerika Utara"),
        ("America/Denver", "MST", "Denver (Mountain Standard Time)", -7.0, "-07:00", "Amerika Utara"),
        ("America/Los_Angeles", "PST", "Los Angeles / San Francisco (Pacific Standard Time)", -8.0, "-08:00", "Amerika Utara"),
        ("America/Sao_Paulo", "BRT", "São Paulo (Brasília Time)", -3.0, "-03:00", "Amerika Selatan"),
        ("Australia/Sydney", "AEST", "Sydney (Australian Eastern Time)", 10.0, "+10:00", "Australia & Pasifik"),
        ("Pacific/Auckland", "NZST", "Auckland (New Zealand Standard Time)", 12.0, "+12:00", "Pasifik"),
        ("Pacific/Honolulu", "HST", "Honolulu (Hawaii-Aleutian Time)", -10.0, "-10:00", "Pasifik"),
        ("UTC", "UTC", "Universal Coordinated Time", 0.0, "+00:00", "Standar Global Universal"),
    ];

    let items: Vec<Value> = zones.into_iter().map(|(id, abbr, desc, offset, offset_str, region)| {
        let mut m = HashMap::new();
        m.insert("id".to_string(), Value::String(id.to_string()));
        m.insert("singkatan".to_string(), Value::String(abbr.to_string()));
        m.insert("deskripsi".to_string(), Value::String(desc.to_string()));
        m.insert("utc_offset_jam".to_string(), Value::Number(offset));
        m.insert("utc_offset_teks".to_string(), Value::String(offset_str.to_string()));
        m.insert("wilayah".to_string(), Value::String(region.to_string()));
        Value::Map(Rc::new(RefCell::new(m)))
    }).collect();

    Ok(Value::Array(Rc::new(RefCell::new(items))))
}

pub fn builtin_waktu_dunia(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let zona_str = if !args.is_empty() {
        match &args[0] {
            Value::String(s) => s.as_str(),
            other => return Err(Galat::runtime(format!("'waktu_dunia' memerlukan teks nama zona/kota, ditemukan '{}'", other.type_name()), span)),
        }
    } else {
        "WIB"
    };

    let (nama_zona, singkatan, deskripsi, offset_jam, offset_secs) = cari_info_zona_waktu(zona_str);
    let now_utc = Utc::now();
    let offset = FixedOffset::east_opt(offset_secs).unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
    let dt = now_utc.with_timezone(&offset);

    Ok(bangun_map_waktu(dt, nama_zona, singkatan, deskripsi, offset_jam))
}

pub fn builtin_konversi_waktu_dunia(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let zona_asal = match &args[1] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'konversi_waktu_dunia' argumen zona_asal memerlukan teks, ditemukan '{}'", other.type_name()), span)),
    };
    let zona_tujuan = match &args[2] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'konversi_waktu_dunia' argumen zona_tujuan memerlukan teks, ditemukan '{}'", other.type_name()), span)),
    };

    let (_asal_id, _asal_abbr, _asal_desc, _asal_offset_h, asal_offset_s) = cari_info_zona_waktu(zona_asal);
    let (tujuan_id, tujuan_abbr, tujuan_desc, tujuan_offset_h, tujuan_offset_s) = cari_info_zona_waktu(zona_tujuan);

    let epoch_sec = match &args[0] {
        Value::Number(n) => *n as i64,
        Value::String(s) => {
            if let Ok(parsed) = DateTime::parse_from_rfc3339(s) {
                parsed.timestamp()
            } else if let Ok(parsed) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                let offset_asal = FixedOffset::east_opt(asal_offset_s).unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
                let local_dt = parsed.and_local_timezone(offset_asal).single();
                if let Some(ldt) = local_dt {
                    ldt.timestamp()
                } else {
                    Utc::now().timestamp()
                }
            } else {
                Utc::now().timestamp()
            }
        }
        other => return Err(Galat::runtime(format!("'konversi_waktu_dunia' argumen waktu memerlukan angka epoch atau teks ISO, ditemukan '{}'", other.type_name()), span)),
    };

    let utc_dt = DateTime::from_timestamp(epoch_sec, 0).unwrap_or_else(|| Utc::now());
    let offset_tujuan = FixedOffset::east_opt(tujuan_offset_s).unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
    let dt_tujuan = utc_dt.with_timezone(&offset_tujuan);

    Ok(bangun_map_waktu(dt_tujuan, tujuan_id, tujuan_abbr, tujuan_desc, tujuan_offset_h))
}

pub fn builtin_hitung_selisih_waktu_zona(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let zona_a = match &args[0] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'hitung_selisih_waktu_zona' argumen zona_a memerlukan teks, ditemukan '{}'", other.type_name()), span)),
    };
    let zona_b = match &args[1] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'hitung_selisih_waktu_zona' argumen zona_b memerlukan teks, ditemukan '{}'", other.type_name()), span)),
    };

    let (_a_id, a_abbr, _a_desc, a_offset_h, _) = cari_info_zona_waktu(zona_a);
    let (_b_id, b_abbr, _b_desc, b_offset_h, _) = cari_info_zona_waktu(zona_b);

    let selisih_jam = a_offset_h - b_offset_h;

    let mut map = HashMap::new();
    map.insert("zona_a".to_string(), Value::String(a_abbr.to_string()));
    map.insert("zona_b".to_string(), Value::String(b_abbr.to_string()));
    map.insert("selisih_jam".to_string(), Value::Number(selisih_jam));
    let ket = if selisih_jam > 0.0 {
        format!("{a_abbr} lebih cepat {selisih_jam} jam dari {b_abbr}")
    } else if selisih_jam < 0.0 {
        format!("{a_abbr} lebih lambat {} jam dari {b_abbr}", selisih_jam.abs())
    } else {
        format!("{a_abbr} dan {b_abbr} berada di zona waktu yang sama")
    };
    map.insert("keterangan".to_string(), Value::String(ket));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_format_waktu_zona(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (epoch_sec, pola, zona_str) = match args.len() {
        2 => {
            let ep = match &args[0] {
                Value::Number(n) => *n as i64,
                _ => Utc::now().timestamp(),
            };
            let p = match &args[1] {
                Value::String(s) => s.clone(),
                _ => "%Y-%m-%d %H:%M:%S".to_string(),
            };
            (ep, p, "WIB".to_string())
        }
        3.. => {
            let ep = match &args[0] {
                Value::Number(n) => *n as i64,
                _ => Utc::now().timestamp(),
            };
            let p = match &args[1] {
                Value::String(s) => s.clone(),
                _ => "%Y-%m-%d %H:%M:%S".to_string(),
            };
            let z = match &args[2] {
                Value::String(s) => s.clone(),
                _ => "WIB".to_string(),
            };
            (ep, p, z)
        }
        _ => return Err(Galat::runtime("Fungsi 'format_waktu_zona' memerlukan (epoch, pola, opsional_zona)", span)),
    };

    let (_, singkatan, _, _, offset_secs) = cari_info_zona_waktu(&zona_str);
    let utc_dt = DateTime::from_timestamp(epoch_sec, 0).unwrap_or_else(|| Utc::now());
    let offset = FixedOffset::east_opt(offset_secs).unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
    let dt = utc_dt.with_timezone(&offset);

    let formatted = dt.format(&pola).to_string().replace("%Z", singkatan);
    Ok(Value::String(formatted))
}

pub fn builtin_jam_dunia_multi_kota(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let target_zones: Vec<String> = if !args.is_empty() {
        match &args[0] {
            Value::Array(arr) => {
                arr.borrow().iter().filter_map(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                }).collect()
            }
            _ => vec!["Jakarta".to_string(), "Makassar".to_string(), "Jayapura".to_string(), "Tokyo".to_string(), "London".to_string(), "New_York".to_string()],
        }
    } else {
        vec![
            "Jakarta".to_string(),
            "Makassar".to_string(),
            "Jayapura".to_string(),
            "Singapura".to_string(),
            "Tokyo".to_string(),
            "Mekkah".to_string(),
            "London".to_string(),
            "Paris".to_string(),
            "New_York".to_string(),
            "Los_Angeles".to_string(),
            "Sydney".to_string(),
        ]
    };

    let now_utc = Utc::now();
    let mut items = Vec::new();

    for z_str in target_zones {
        let (nama_zona, singkatan, deskripsi, offset_jam, offset_secs) = cari_info_zona_waktu(&z_str);
        let offset = FixedOffset::east_opt(offset_secs).unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
        let dt = now_utc.with_timezone(&offset);
        items.push(bangun_map_waktu(dt, nama_zona, singkatan, deskripsi, offset_jam));
    }

    Ok(Value::Array(Rc::new(RefCell::new(items))))
}

pub fn builtin_waktu_utc(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let now = SystemTime::now();
    let epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    Ok(Value::Number(epoch.as_secs_f64()))
}

pub fn builtin_waktu_utc_ms(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let now = SystemTime::now();
    let epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    Ok(Value::Number(epoch.as_millis() as f64))
}

pub fn builtin_waktu_monotonik_ns(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let start = get_startup_instant();
    let elapsed = start.elapsed();
    Ok(Value::Number(elapsed.as_nanos() as f64))
}

pub fn builtin_waktu_monotonik_ms(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let start = get_startup_instant();
    let elapsed = start.elapsed();
    Ok(Value::Number(elapsed.as_secs_f64() * 1000.0))
}

pub fn builtin_durasi_monotonik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let awal_ns = match args[0] {
        Value::Number(n) => n,
        ref other => return Err(Galat::runtime(format!("'durasi_monotonik' memerlukan angka timestamp monotonik awal (ns/ms), ditemukan '{}'", other.type_name()), span)),
    };
    let sekarang_ns = get_startup_instant().elapsed().as_nanos() as f64;
    let diff_ns = (sekarang_ns - awal_ns).max(0.0);
    let diff_ms = diff_ns / 1_000_000.0;
    let diff_detik = diff_ns / 1_000_000_000.0;

    let mut map = HashMap::new();
    map.insert("durasi_ns".to_string(), Value::Number(diff_ns));
    map.insert("durasi_ms".to_string(), Value::Number(diff_ms));
    map.insert("durasi_detik".to_string(), Value::Number(diff_detik));
    map.insert("keterangan".to_string(), Value::String(format!("{diff_ms:.3} ms ({diff_detik:.6} detik)")));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_urai_iso8601(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let iso_str = match &args[0] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'urai_iso8601' memerlukan argumen teks format ISO 8601 / RFC 3339, ditemukan '{}'", other.type_name()), span)),
    };

    let parsed = match DateTime::parse_from_rfc3339(iso_str) {
        Ok(dt) => dt,
        Err(_) => {
            if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(iso_str, "%Y-%m-%d %H:%M:%S") {
                let utc = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
                utc.with_timezone(&FixedOffset::east_opt(0).unwrap())
            } else {
                return Err(Galat::runtime(format!("Gagal mengurai teks ISO 8601: '{}'", iso_str), span));
            }
        }
    };

    let utc_dt = parsed.with_timezone(&Utc);
    let offset = parsed.offset();
    let offset_seconds = offset.local_minus_utc();
    let offset_jam = (offset_seconds as f64) / 3600.0;

    let mut map = HashMap::new();
    map.insert("epoch_detik".to_string(), Value::Number(parsed.timestamp() as f64));
    map.insert("epoch_ms".to_string(), Value::Number(parsed.timestamp_millis() as f64));
    map.insert("iso_asli".to_string(), Value::String(iso_str.to_string()));
    map.insert("iso_utc_zulu".to_string(), Value::String(format!("{}.{:03}Z", utc_dt.format("%Y-%m-%dT%H:%M:%S"), utc_dt.timestamp_subsec_millis())));
    map.insert("tahun".to_string(), Value::Number(parsed.year() as f64));
    map.insert("bulan".to_string(), Value::Number(parsed.month() as f64));
    map.insert("hari".to_string(), Value::Number(parsed.day() as f64));
    map.insert("jam".to_string(), Value::Number(parsed.hour() as f64));
    map.insert("menit".to_string(), Value::Number(parsed.minute() as f64));
    map.insert("detik".to_string(), Value::Number(parsed.second() as f64));
    map.insert("milidetik".to_string(), Value::Number(parsed.timestamp_subsec_millis() as f64));
    map.insert("utc_offset_jam".to_string(), Value::Number(offset_jam));
    map.insert("adalah_zulu_utc".to_string(), Value::Bool(offset_seconds == 0));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_format_iso8601_utc(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let epoch_sec = if !args.is_empty() {
        match &args[0] {
            Value::Number(n) => *n as i64,
            other => return Err(Galat::runtime(format!("'format_iso8601_utc' memerlukan angka epoch detik, ditemukan '{}'", other.type_name()), span)),
        }
    } else {
        Utc::now().timestamp()
    };

    let dt = DateTime::from_timestamp(epoch_sec, 0).unwrap_or_else(|| Utc::now());
    Ok(Value::String(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()))
}

pub fn builtin_cek_status_dst(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let zona = if !args.is_empty() {
        match &args[0] {
            Value::String(s) => s.as_str(),
            other => return Err(Galat::runtime(format!("'cek_status_dst' memerlukan teks nama zona/kota, ditemukan '{}'", other.type_name()), span)),
        }
    } else {
        "America/New_York"
    };

    let dt_target = if args.len() >= 2 {
        match &args[1] {
            Value::Number(n) => DateTime::from_timestamp(*n as i64, 0).unwrap_or_else(|| Utc::now()),
            Value::String(s) => DateTime::parse_from_rfc3339(s).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            _ => Utc::now(),
        }
    } else {
        Utc::now()
    };

    let month = dt_target.month();
    let key = zona.to_lowercase();

    let (berlaku_dst, nama_musim, catatan_gap_overlap) = if key.contains("york") || key.contains("america") || key.contains("us") {
        let is_dst = (3..=10).contains(&month);
        let status_musim = if is_dst { "Daylight Saving Time (EDT/CDT/PDT)" } else { "Standard Time (EST/CST/PST)" };
        let fenomena = "Gap 02:00 -> 03:00 (Maret) | Overlap 02:00 -> 01:00 (November)";
        (is_dst, status_musim, fenomena)
    } else if key.contains("london") || key.contains("paris") || key.contains("berlin") || key.contains("europe") {
        let is_dst = (4..=9).contains(&month);
        let status_musim = if is_dst { "British Summer Time (BST) / Central European Summer Time (CEST)" } else { "Greenwich Mean Time (GMT) / Central European Time (CET)" };
        let fenomena = "Gap 01:00 -> 02:00 (Maret) | Overlap 02:00 -> 01:00 (Oktober)";
        (is_dst, status_musim, fenomena)
    } else if key.contains("sydney") || key.contains("australia") {
        let is_dst = month >= 10 || month <= 3;
        let status_musim = if is_dst { "Australian Eastern Daylight Time (AEDT)" } else { "Australian Eastern Standard Time (AEST)" };
        let fenomena = "Gap 02:00 -> 03:00 (Oktober) | Overlap 03:00 -> 02:00 (April)";
        (is_dst, status_musim, fenomena)
    } else {
        (false, "Tidak Menerapkan DST (Waktu Standar Tetap)", "Zona Khatulistiwa/Tropis bebas komplikasi Gap dan Overlap jam")
    };

    let mut map = HashMap::new();
    map.insert("zona".to_string(), Value::String(zona.to_string()));
    map.insert("status_dst_aktif".to_string(), Value::Bool(berlaku_dst));
    map.insert("nama_musim".to_string(), Value::String(nama_musim.to_string()));
    map.insert("mekanisme_gap_overlap".to_string(), Value::String(catatan_gap_overlap.to_string()));
    map.insert("waktu_pemeriksaan_utc".to_string(), Value::String(dt_target.format("%Y-%m-%dT%H:%M:%SZ").to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_simulasi_leap_smearing(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let durasi_jendela_jam = if !args.is_empty() {
        match &args[0] {
            Value::Number(n) => *n,
            _ => 24.0,
        }
    } else {
        24.0
    };

    let total_detik_jendela = durasi_jendela_jam * 3600.0;
    let laju_penyesuaian_per_detik = 1.0 / total_detik_jendela;

    let mut map = HashMap::new();
    map.insert("metode".to_string(), Value::String("Linear Leap Smearing (Standar Google / AWS / NTP)".to_string()));
    map.insert("jendela_smear_jam".to_string(), Value::Number(durasi_jendela_jam));
    map.insert("total_detik_jendela".to_string(), Value::Number(total_detik_jendela));
    map.insert("penyesuaian_per_detik_ppm".to_string(), Value::Number(laju_penyesuaian_per_detik * 1_000_000.0));
    map.insert("keamanan_transaksi".to_string(), Value::String("Mencegah detik berulang (23:59:60) atau jam mundur yang merusak database & bursa finansial".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_tunggu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match args[0] {
        Value::Number(ms) => {
            if ms < 0.0 {
                return Err(Galat::runtime("Waktu tunggu tidak boleh negatif", span));
            }
            thread::sleep(Duration::from_millis(ms as u64));
            Ok(Value::Nil)
        }
        _ => Err(Galat::runtime("Fungsi 'tunggu' memerlukan waktu dalam milidetik (angka)", span)),
    }
}

pub fn register_time(env: &mut Environment) {
    register_fn(env, "waktu", Some(0), builtin_waktu);
    register_fn(env, "waktu_utc", Some(0), builtin_waktu_utc);
    register_fn(env, "waktu_utc_ms", Some(0), builtin_waktu_utc_ms);
    register_fn(env, "waktu_monotonik_ns", Some(0), builtin_waktu_monotonik_ns);
    register_fn(env, "waktu_monotonik_ms", Some(0), builtin_waktu_monotonik_ms);
    register_fn(env, "durasi_monotonik", Some(1), builtin_durasi_monotonik);
    register_fn(env, "urai_iso8601", Some(1), builtin_urai_iso8601);
    register_fn(env, "format_iso8601_utc", None, builtin_format_iso8601_utc);
    register_fn(env, "cek_status_dst", None, builtin_cek_status_dst);
    register_fn(env, "simulasi_leap_smearing", None, builtin_simulasi_leap_smearing);
    register_fn(env, "waktu_dunia", None, builtin_waktu_dunia);
    register_fn(env, "waktu_sekarang", None, builtin_waktu_dunia);
    register_fn(env, "daftar_zona_waktu_dunia", Some(0), builtin_daftar_zona_waktu_dunia);
    register_fn(env, "konversi_waktu_dunia", Some(3), builtin_konversi_waktu_dunia);
    register_fn(env, "hitung_selisih_waktu_zona", Some(2), builtin_hitung_selisih_waktu_zona);
    register_fn(env, "format_waktu_zona", None, builtin_format_waktu_zona);
    register_fn(env, "jam_dunia_multi_kota", None, builtin_jam_dunia_multi_kota);
    register_fn(env, "tunggu", Some(1), builtin_tunggu);
    register_fn(env, "tidur", Some(1), builtin_tunggu);
}
