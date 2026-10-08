use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub fn invoke_closure_or_fn(func: &Value, args: &[Value], span: &Span) -> Result<Value, Galat> {
    match func {
        Value::Builtin(b) => (b.func)(args, span),
        Value::Function(f) => {
            let mut interp = crate::interpreter::Interpreter::new();
            interp.call_value(Value::Function(f.clone()), args, span)
        }
        other => Err(Galat::runtime(
            format!("Argumen kedua harus berupa fungsi yang dapat dipanggil, ditemukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_panjang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.chars().count() as f64)),
        Value::Array(a) => Ok(Value::Number(a.borrow().len() as f64)),
        Value::Map(m) => Ok(Value::Number(m.borrow().len() as f64)),
        Value::Range(start, end) => Ok(Value::Number(((end - start).abs() + 1) as f64)),
        other => Err(Galat::runtime(
            format!("Fungsi 'panjang' tidak berlaku untuk tipe '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            a.borrow_mut().push(args[1].clone());
            Ok(Value::Array(a.clone()))
        }
        other => Err(Galat::runtime(
            format!("Argumen pertama 'tambah' harus berupa daftar, ditemukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_hapus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Array(a), Value::Number(idx)) => {
            let mut list = a.borrow_mut();
            let i = *idx as usize;
            if i < list.len() {
                Ok(list.remove(i))
            } else {
                Err(Galat::runtime(
                    format!("Indeks {} di luar jangkauan daftar (panjang: {})", i, list.len()),
                    span,
                ))
            }
        }
        (Value::Map(m), Value::String(key)) => {
            let mut map = m.borrow_mut();
            Ok(map.remove(key).unwrap_or(Value::Nil))
        }
        _ => Err(Galat::runtime(
            "Penggunaan 'hapus(daftar, indeks)' atau 'hapus(kamus, kunci)' tidak valid",
            span,
        )),
    }
}

pub fn builtin_sisip(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Array(a), Value::Number(idx)) => {
            let mut list = a.borrow_mut();
            let i = *idx as usize;
            if i <= list.len() {
                list.insert(i, args[2].clone());
                Ok(Value::Array(a.clone()))
            } else {
                Err(Galat::runtime(
                    format!("Indeks sisip {} melebihi panjang daftar {}", i, list.len()),
                    span,
                ))
            }
        }
        _ => Err(Galat::runtime(
            "Fungsi 'sisip' memerlukan (daftar, indeks_angka, nilai)",
            span,
        )),
    }
}

pub fn builtin_gabung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Array(a), Value::String(sep)) => {
            let items = a.borrow();
            let str_items: Vec<String> = items.iter().map(|item| item.to_string_repr()).collect();
            Ok(Value::String(str_items.join(sep)))
        }
        _ => Err(Galat::runtime(
            "Fungsi 'gabung' memerlukan (daftar, pemisah_teks)",
            span,
        )),
    }
}

pub fn builtin_pisah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::String(s), Value::String(sep)) => {
            let parts: Vec<Value> = if sep.is_empty() {
                s.chars().map(|c| Value::String(c.to_string())).collect()
            } else {
                s.split(sep.as_str()).map(|part| Value::String(part.to_string())).collect()
            };
            Ok(Value::Array(Rc::new(RefCell::new(parts))))
        }
        _ => Err(Galat::runtime(
            "Fungsi 'pisah' memerlukan (teks, pemisah_teks)",
            span,
        )),
    }
}

pub fn builtin_huruf_besar(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        other => Err(Galat::runtime(
            format!("'huruf_besar' memerlukan teks, bukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_huruf_kecil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        other => Err(Galat::runtime(
            format!("'huruf_kecil' memerlukan teks, bukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_potong(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let awal = match args[1] {
        Value::Number(n) => n.max(0.0) as usize,
        _ => return Err(Galat::runtime("Indeks awal harus berupa angka", span)),
    };
    let akhir = match args[2] {
        Value::Number(n) => n.max(0.0) as usize,
        _ => return Err(Galat::runtime("Indeks akhir harus berupa angka", span)),
    };

    match &args[0] {
        Value::String(s) => {
            let chars: Vec<char> = s.chars().collect();
            let end = akhir.min(chars.len());
            if awal <= end {
                let sub: String = chars[awal..end].iter().collect();
                Ok(Value::String(sub))
            } else {
                Ok(Value::String(String::new()))
            }
        }
        Value::Array(a) => {
            let list = a.borrow();
            let end = akhir.min(list.len());
            if awal <= end {
                let sub: Vec<Value> = list[awal..end].to_vec();
                Ok(Value::Array(Rc::new(RefCell::new(sub))))
            } else {
                Ok(Value::Array(Rc::new(RefCell::new(Vec::new()))))
            }
        }
        other => Err(Galat::runtime(
            format!("'potong' memerlukan teks atau daftar, bukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_kunci(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let mut keys: Vec<Value> = map.keys().map(|k| Value::String(k.clone())).collect();
            keys.sort_by(|a, b| a.to_string_repr().cmp(&b.to_string_repr()));
            Ok(Value::Array(Rc::new(RefCell::new(keys))))
        }
        other => Err(Galat::runtime(
            format!("'kunci' memerlukan kamus, bukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_nilai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let mut pairs: Vec<(String, Value)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            pairs.sort_by(|a, b| a.0.cmp(&b.0));
            let values: Vec<Value> = pairs.into_iter().map(|(_, v)| v).collect();
            Ok(Value::Array(Rc::new(RefCell::new(values))))
        }
        other => Err(Galat::runtime(
            format!("'nilai' memerlukan kamus, bukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_ada_kunci(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Map(m), Value::String(k)) => {
            Ok(Value::Bool(m.borrow().contains_key(k)))
        }
        _ => Err(Galat::runtime(
            "Fungsi 'ada_kunci' memerlukan (kamus, kunci_teks)",
            span,
        )),
    }
}

pub fn builtin_urutkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            let mut items = a.borrow().clone();
            items.sort_by(|x, y| match (x, y) {
                (Value::Number(n1), Value::Number(n2)) => n1.partial_cmp(n2).unwrap_or(std::cmp::Ordering::Equal),
                (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                _ => x.to_string_repr().cmp(&y.to_string_repr()),
            });
            Ok(Value::Array(Rc::new(RefCell::new(items))))
        }
        other => Err(Galat::runtime(
            format!("'urutkan' memerlukan daftar, bukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_balik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            let mut items = a.borrow().clone();
            items.reverse();
            Ok(Value::Array(Rc::new(RefCell::new(items))))
        }
        Value::String(s) => {
            let rev: String = s.chars().rev().collect();
            Ok(Value::String(rev))
        }
        other => Err(Galat::runtime(
            format!("'balik' memerlukan daftar atau teks, bukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_petakan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            let items = a.borrow().clone();
            let mut mapped = Vec::new();
            for item in items {
                let res = invoke_closure_or_fn(&args[1], &[item], span)?;
                mapped.push(res);
            }
            Ok(Value::Array(Rc::new(RefCell::new(mapped))))
        }
        other => Err(Galat::runtime(
            format!("'petakan' memerlukan daftar sebagai argumen pertama, ditemukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_saring(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            let items = a.borrow().clone();
            let mut filtered = Vec::new();
            for item in items {
                let res = invoke_closure_or_fn(&args[1], &[item.clone()], span)?;
                if res.is_truthy() {
                    filtered.push(item);
                }
            }
            Ok(Value::Array(Rc::new(RefCell::new(filtered))))
        }
        other => Err(Galat::runtime(
            format!("'saring' memerlukan daftar sebagai argumen pertama, ditemukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_lipat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            let items = a.borrow().clone();
            let mut acc = args[1].clone();
            for item in items {
                acc = invoke_closure_or_fn(&args[2], &[acc, item], span)?;
            }
            Ok(acc)
        }
        other => Err(Galat::runtime(
            format!("'lipat' memerlukan daftar sebagai argumen pertama, ditemukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_untuk_setiap(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            let items = a.borrow().clone();
            for item in items {
                invoke_closure_or_fn(&args[1], &[item], span)?;
            }
            Ok(Value::Nil)
        }
        other => Err(Galat::runtime(format!("'untuk_setiap' memerlukan daftar sebagai argumen pertama, ditemukan '{}'", other.type_name()), span)),
    }
}

pub fn builtin_temukan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            let items = a.borrow().clone();
            for item in items {
                let res = invoke_closure_or_fn(&args[1], &[item.clone()], span)?;
                if res.is_truthy() {
                    return Ok(item);
                }
            }
            Ok(Value::Nil)
        }
        other => Err(Galat::runtime(
            format!("'temukan' memerlukan daftar sebagai argumen pertama, ditemukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_gabungkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match (&args[0], &args[1]) {
        (Value::Array(a1), Value::Array(a2)) => {
            let mut combined = a1.borrow().clone();
            combined.extend(a2.borrow().clone());
            Ok(Value::Array(Rc::new(RefCell::new(combined))))
        }
        _ => Err(Galat::runtime("'gabungkan' memerlukan dua buah daftar", span)),
    }
}

pub fn builtin_ambil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let count = match &args[1] {
        Value::Number(n) => (*n as i64).max(0) as usize,
        _ => return Err(Galat::runtime("Argumen kedua 'ambil' harus berupa angka", span)),
    };
    match &args[0] {
        Value::Array(a) => {
            let items = a.borrow();
            let taken: Vec<Value> = items.iter().take(count).cloned().collect();
            Ok(Value::Array(Rc::new(RefCell::new(taken))))
        }
        Value::String(s) => {
            let taken: String = s.chars().take(count).collect();
            Ok(Value::String(taken))
        }
        other => Err(Galat::runtime(format!("'ambil' memerlukan daftar atau teks, ditemukan '{}'", other.type_name()), span)),
    }
}

pub fn builtin_lewati(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let count = match &args[1] {
        Value::Number(n) => (*n as i64).max(0) as usize,
        _ => return Err(Galat::runtime("Argumen kedua 'lewati' harus berupa angka", span)),
    };
    match &args[0] {
        Value::Array(a) => {
            let items = a.borrow();
            let skipped: Vec<Value> = items.iter().skip(count).cloned().collect();
            Ok(Value::Array(Rc::new(RefCell::new(skipped))))
        }
        Value::String(s) => {
            let skipped: String = s.chars().skip(count).collect();
            Ok(Value::String(skipped))
        }
        other => Err(Galat::runtime(format!("'lewati' memerlukan daftar atau teks, ditemukan '{}'", other.type_name()), span)),
    }
}

pub fn builtin_apakah_ada(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            let items = a.borrow().clone();
            for item in items {
                let res = invoke_closure_or_fn(&args[1], &[item], span)?;
                if res.is_truthy() {
                    return Ok(Value::Bool(true));
                }
            }
            Ok(Value::Bool(false))
        }
        other => Err(Galat::runtime(format!("'apakah_ada' memerlukan daftar sebagai argumen pertama, ditemukan '{}'", other.type_name()), span)),
    }
}

pub fn builtin_semua(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Array(a) => {
            let items = a.borrow().clone();
            for item in items {
                let res = invoke_closure_or_fn(&args[1], &[item], span)?;
                if !res.is_truthy() {
                    return Ok(Value::Bool(false));
                }
            }
            Ok(Value::Bool(true))
        }
        other => Err(Galat::runtime(format!("'semua' memerlukan daftar sebagai argumen pertama, ditemukan '{}'", other.type_name()), span)),
    }
}

pub fn builtin_panjang_byte(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s = match &args[0] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'panjang_byte' memerlukan teks, ditemukan '{}'", other.type_name()), span)),
    };
    Ok(Value::Number(s.as_bytes().len() as f64))
}

pub fn builtin_panjang_karakter(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s = match &args[0] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'panjang_karakter' memerlukan teks, ditemukan '{}'", other.type_name()), span)),
    };
    Ok(Value::Number(s.chars().count() as f64))
}

pub fn builtin_potong_karakter(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s = match &args[0] {
        Value::String(s) => s.as_str(),
        other => return Err(Galat::runtime(format!("'potong_karakter' argumen 1 memerlukan teks, ditemukan '{}'", other.type_name()), span)),
    };
    let awal = match args[1] {
        Value::Number(n) => (n as usize).max(0),
        ref other => return Err(Galat::runtime(format!("'potong_karakter' argumen 2 (indeks awal) memerlukan angka, ditemukan '{}'", other.type_name()), span)),
    };
    let panjang = match args[2] {
        Value::Number(n) => (n as usize).max(0),
        ref other => return Err(Galat::runtime(format!("'potong_karakter' argumen 3 (panjang) memerlukan angka, ditemukan '{}'", other.type_name()), span)),
    };

    let hasil: String = s.chars().skip(awal).take(panjang).collect();
    Ok(Value::String(hasil))
}

pub fn builtin_ke_bytes_utf8(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let s = match &args[0] {
        Value::String(s) => s.as_bytes(),
        other => return Err(Galat::runtime(format!("'ke_bytes_utf8' memerlukan teks, ditemukan '{}'", other.type_name()), span)),
    };
    let bytes: Vec<Value> = s.iter().map(|b| Value::Number(*b as f64)).collect();
    Ok(Value::Array(Rc::new(RefCell::new(bytes))))
}

pub fn builtin_dari_bytes_utf8(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let raw = match &args[0] {
        Value::Array(arr) => {
            let mut b = Vec::new();
            for v in arr.borrow().iter() {
                match v {
                    Value::Number(n) => b.push(*n as u8),
                    _ => return Err(Galat::runtime("Array bytes harus berisi angka byte (0-255)", span)),
                }
            }
            b
        }
        Value::String(s) => {
            if let Ok(bytes) = hex::decode(s) {
                bytes
            } else {
                s.as_bytes().to_vec()
            }
        }
        other => return Err(Galat::runtime(format!("'dari_bytes_utf8' memerlukan array bytes atau hex string, ditemukan '{}'", other.type_name()), span)),
    };

    let teks = String::from_utf8_lossy(&raw).to_string();
    Ok(Value::String(teks))
}

pub fn register_collections(env: &mut Environment) {
    register_fn(env, "panjang", Some(1), builtin_panjang);
    register_fn(env, "tambah", Some(2), builtin_tambah);
    register_fn(env, "hapus", Some(2), builtin_hapus);
    register_fn(env, "sisip", Some(3), builtin_sisip);
    register_fn(env, "gabung", Some(2), builtin_gabung);
    register_fn(env, "pisah", Some(2), builtin_pisah);
    register_fn(env, "huruf_besar", Some(1), builtin_huruf_besar);
    register_fn(env, "huruf_kecil", Some(1), builtin_huruf_kecil);
    register_fn(env, "potong", Some(3), builtin_potong);
    register_fn(env, "kunci", Some(1), builtin_kunci);
    register_fn(env, "nilai", Some(1), builtin_nilai);
    register_fn(env, "ada_kunci", Some(2), builtin_ada_kunci);
    register_fn(env, "urutkan", Some(1), builtin_urutkan);
    register_fn(env, "balik", Some(1), builtin_balik);

    register_fn(env, "petakan", Some(2), builtin_petakan);
    register_fn(env, "saring", Some(2), builtin_saring);
    register_fn(env, "lipat", Some(3), builtin_lipat);
    register_fn(env, "untuk_setiap", Some(2), builtin_untuk_setiap);
    register_fn(env, "temukan", Some(2), builtin_temukan);
    register_fn(env, "gabungkan", Some(2), builtin_gabungkan);
    register_fn(env, "ambil", Some(2), builtin_ambil);
    register_fn(env, "lewati", Some(2), builtin_lewati);
    register_fn(env, "apakah_ada", Some(2), builtin_apakah_ada);
    register_fn(env, "semua", Some(2), builtin_semua);

    register_fn(env, "panjang_byte", Some(1), builtin_panjang_byte);
    register_fn(env, "panjang_karakter", Some(1), builtin_panjang_karakter);
    register_fn(env, "potong_karakter", Some(3), builtin_potong_karakter);
    register_fn(env, "ke_bytes_utf8", Some(1), builtin_ke_bytes_utf8);
    register_fn(env, "dari_bytes_utf8", Some(1), builtin_dari_bytes_utf8);
}
