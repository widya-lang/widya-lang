use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::{json_to_value, register_fn, value_to_json};
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn builtin_tipe(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::String(args[0].type_name().to_string()))
}

pub fn builtin_ke_angka(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::String(s) => s.trim().parse::<f64>().map(Value::Number).map_err(|_| {
            Galat::runtime(format!("Tidak dapat mengubah teks '{}' menjadi angka", s), span)
        }),
        Value::Bool(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
        other => Err(Galat::runtime(
            format!("Tidak dapat mengubah tipe '{}' menjadi angka", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_ke_teks(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::String(args[0].to_string_repr()))
}

pub fn builtin_ke_boolean(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::Bool(args[0].is_truthy()))
}

pub fn builtin_ok(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe_enum".to_string(), Value::String("Result::Ok".to_string()));
    map.insert("apakah_ok".to_string(), Value::Bool(true));
    map.insert("nilai".to_string(), args[0].clone());
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_err(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe_enum".to_string(), Value::String("Result::Err".to_string()));
    map.insert("apakah_ok".to_string(), Value::Bool(false));
    map.insert("galat".to_string(), args[0].clone());
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_ada(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe_enum".to_string(), Value::String("Option::Ada".to_string()));
    map.insert("ada".to_string(), Value::Bool(true));
    map.insert("nilai".to_string(), args[0].clone());
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_kosong(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe_enum".to_string(), Value::String("Option::Kosong".to_string()));
    map.insert("ada".to_string(), Value::Bool(false));
    map.insert("nilai".to_string(), Value::Nil);
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_buka(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Bool(true)) = map.get("apakah_ok") {
                return Ok(map.get("nilai").cloned().unwrap_or(Value::Nil));
            } else if let Some(Value::Bool(false)) = map.get("apakah_ok") {
                let err_msg = map.get("galat").map(|v| v.to_string_repr()).unwrap_or_else(|| "Galat tidak diketahui".to_string());
                return Err(Galat::runtime(format!("Panic saat membuka (unwrap) Result::Err: {}", err_msg), span));
            } else if let Some(Value::Bool(true)) = map.get("ada") {
                return Ok(map.get("nilai").cloned().unwrap_or(Value::Nil));
            } else if let Some(Value::Bool(false)) = map.get("ada") {
                return Err(Galat::runtime("Panic saat membuka (unwrap) Option::Kosong / None", span));
            }
            Ok(args[0].clone())
        }
        Value::Nil => Err(Galat::runtime("Panic saat membuka (unwrap) nilai kosong/nihil", span)),
        other => Ok(other.clone()),
    }
}

pub fn builtin_buka_atau(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let fallback = &args[1];
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Bool(true)) = map.get("apakah_ok") {
                Ok(map.get("nilai").cloned().unwrap_or_else(|| fallback.clone()))
            } else if let Some(Value::Bool(true)) = map.get("ada") {
                Ok(map.get("nilai").cloned().unwrap_or_else(|| fallback.clone()))
            } else {
                Ok(fallback.clone())
            }
        }
        Value::Nil => Ok(fallback.clone()),
        other => Ok(other.clone()),
    }
}

pub fn builtin_apakah_ok(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Bool(b)) = map.get("apakah_ok") {
                Ok(Value::Bool(*b))
            } else if let Some(Value::Bool(b)) = map.get("ada") {
                Ok(Value::Bool(*b))
            } else {
                Ok(Value::Bool(true))
            }
        }
        Value::Nil => Ok(Value::Bool(false)),
        _ => Ok(Value::Bool(true)),
    }
}

pub fn builtin_apakah_err(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Bool(b)) = map.get("apakah_ok") {
                Ok(Value::Bool(!*b))
            } else if let Some(Value::Bool(b)) = map.get("ada") {
                Ok(Value::Bool(!*b))
            } else {
                Ok(Value::Bool(false))
            }
        }
        Value::Nil => Ok(Value::Bool(true)),
        _ => Ok(Value::Bool(false)),
    }
}

pub fn builtin_pastikan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'pastikan' memerlukan kondisi ekspresi", span));
    }
    if !args[0].is_truthy() {
        let msg = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| "Penegasan (assertion) gagal!".to_string());
        return Err(Galat::runtime(format!("Assertion Panic: {}", msg), span));
    }
    Ok(Value::Bool(true))
}

pub fn builtin_pastikan_sama(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("Fungsi 'pastikan_sama' memerlukan minimal 2 argumen: (ekspektasi, aktual)", span));
    }
    if args[0] != args[1] {
        let msg = args.get(2).map(|v| v.to_string_repr()).unwrap_or_else(|| {
            format!("Penegasan gagal! Diharapkan: {}, Ditemukan: {}", args[0].to_string_repr(), args[1].to_string_repr())
        });
        return Err(Galat::runtime(format!("Assertion Panic: {}", msg), span));
    }
    Ok(Value::Bool(true))
}

pub fn builtin_pastikan_beda(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("Fungsi 'pastikan_beda' memerlukan minimal 2 argumen: (nilai1, nilai2)", span));
    }
    if args[0] == args[1] {
        let msg = args.get(2).map(|v| v.to_string_repr()).unwrap_or_else(|| {
            format!("Penegasan gagal! Kedua nilai sama: {}", args[0].to_string_repr())
        });
        return Err(Galat::runtime(format!("Assertion Panic: {}", msg), span));
    }
    Ok(Value::Bool(true))
}

pub fn builtin_pastikan_benar(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'pastikan_benar' memerlukan nilai boolean", span));
    }
    if !args[0].is_truthy() {
        let msg = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| "Diharapkan 'benar', ditemukan 'salah'".to_string());
        return Err(Galat::runtime(format!("Assertion Panic: {}", msg), span));
    }
    Ok(Value::Bool(true))
}

pub fn builtin_pastikan_salah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'pastikan_salah' memerlukan nilai boolean", span));
    }
    if args[0].is_truthy() {
        let msg = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| "Diharapkan 'salah', ditemukan 'benar'".to_string());
        return Err(Galat::runtime(format!("Assertion Panic: {}", msg), span));
    }
    Ok(Value::Bool(true))
}

pub fn builtin_pastikan_nihil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'pastikan_nihil' memerlukan 1 argumen", span));
    }
    if !matches!(args[0], Value::Nil) {
        let msg = args.get(1).map(|v| v.to_string_repr()).unwrap_or_else(|| format!("Diharapkan 'nihil', ditemukan '{}'", args[0].to_string_repr()));
        return Err(Galat::runtime(format!("Assertion Panic: {}", msg), span));
    }
    Ok(Value::Bool(true))
}

pub fn builtin_ke_json(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("Fungsi 'ke_json' memerlukan minimal 1 argumen", span));
    }
    let val = &args[0];
    let pretty = args.get(1).map(|v| v.is_truthy()).unwrap_or(false);
    let json_val = value_to_json(val);

    let res = if pretty {
        serde_json::to_string_pretty(&json_val)
    } else {
        serde_json::to_string(&json_val)
    };

    match res {
        Ok(s) => Ok(Value::String(s)),
        Err(e) => Err(Galat::runtime(format!("Gagal membuat JSON: {}", e), span)),
    }
}

pub fn builtin_dari_json(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::String(s) => match serde_json::from_str::<serde_json::Value>(s) {
            Ok(json) => Ok(json_to_value(json)),
            Err(e) => Err(Galat::runtime(format!("JSON tidak valid: {}", e), span)),
        },
        other => Err(Galat::runtime(
            format!("'dari_json' memerlukan teks JSON, bukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn register_types(env: &mut Environment) {
    register_fn(env, "tipe", Some(1), builtin_tipe);
    register_fn(env, "ke_angka", Some(1), builtin_ke_angka);
    register_fn(env, "ke_teks", Some(1), builtin_ke_teks);
    register_fn(env, "ke_boolean", Some(1), builtin_ke_boolean);
    register_fn(env, "string", Some(1), builtin_ke_teks);
    register_fn(env, "teks", Some(1), builtin_ke_teks);
    register_fn(env, "angka", Some(1), builtin_ke_angka);
    register_fn(env, "boolean", Some(1), builtin_ke_boolean);

    register_fn(env, "Ok", Some(1), builtin_ok);
    register_fn(env, "Sukses", Some(1), builtin_ok);
    register_fn(env, "Err", Some(1), builtin_err);
    register_fn(env, "Gagal", Some(1), builtin_err);
    register_fn(env, "Ada", Some(1), builtin_ada);
    register_fn(env, "Some", Some(1), builtin_ada);
    register_fn(env, "Kosong", Some(0), builtin_kosong);
    register_fn(env, "None", Some(0), builtin_kosong);
    register_fn(env, "buka", Some(1), builtin_buka);
    register_fn(env, "unwrap", Some(1), builtin_buka);
    register_fn(env, "buka_atau", Some(2), builtin_buka_atau);
    register_fn(env, "unwrap_or", Some(2), builtin_buka_atau);
    register_fn(env, "apakah_ok", Some(1), builtin_apakah_ok);
    register_fn(env, "apakah_err", Some(1), builtin_apakah_err);

    register_fn(env, "pastikan", None, builtin_pastikan);
    register_fn(env, "pastikan_sama", None, builtin_pastikan_sama);
    register_fn(env, "pastikan_beda", None, builtin_pastikan_beda);
    register_fn(env, "pastikan_benar", None, builtin_pastikan_benar);
    register_fn(env, "pastikan_salah", None, builtin_pastikan_salah);
    register_fn(env, "pastikan_nihil", None, builtin_pastikan_nihil);

    register_fn(env, "ke_json", None, builtin_ke_json);
    register_fn(env, "dari_json", Some(1), builtin_dari_json);
}
