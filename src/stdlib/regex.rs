use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub use ::regex::Regex;

// --- WidyaRegex Native Engine Implementation ---

fn builtin_regex_cocok(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pola = args[0].to_string_repr();
    let teks = args[1].to_string_repr();

    let re = Regex::new(&pola)
        .map_err(|e| Galat::runtime(format!("Pola regex tidak valid: {}", e), span))?;

    Ok(Value::Bool(re.is_match(&teks)))
}

fn builtin_regex_temukan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pola = args[0].to_string_repr();
    let teks = args[1].to_string_repr();

    let re = Regex::new(&pola)
        .map_err(|e| Galat::runtime(format!("Pola regex tidak valid: {}", e), span))?;

    if let Some(mat) = re.find(&teks) {
        let mut map = HashMap::new();
        map.insert("ditemukan".to_string(), Value::Bool(true));
        map.insert("hasil".to_string(), Value::String(mat.as_str().to_string()));
        map.insert("mulai".to_string(), Value::Number(mat.start() as f64));
        map.insert("selesai".to_string(), Value::Number(mat.end() as f64));
        Ok(Value::Map(Rc::new(RefCell::new(map))))
    } else {
        Ok(Value::Nil)
    }
}

fn builtin_regex_ganti(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pola = args[0].to_string_repr();
    let pengganti = args[1].to_string_repr();
    let teks = args[2].to_string_repr();

    let re = Regex::new(&pola)
        .map_err(|e| Galat::runtime(format!("Pola regex tidak valid: {}", e), span))?;

    let res = re.replace_all(&teks, pengganti.as_str());
    Ok(Value::String(res.to_string()))
}

fn builtin_regex_tangkap(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pola = args[0].to_string_repr();
    let teks = args[1].to_string_repr();

    let re = Regex::new(&pola)
        .map_err(|e| Galat::runtime(format!("Pola regex tidak valid: {}", e), span))?;

    let matches: Vec<Value> = re.find_iter(&teks)
        .map(|m| Value::String(m.as_str().to_string()))
        .collect();

    Ok(Value::Array(Rc::new(RefCell::new(matches))))
}


pub fn register_regex(env: &mut Environment) {
    register_fn(env, "regex_cocok", Some(2), builtin_regex_cocok);
    register_fn(env, "regex_temukan", Some(2), builtin_regex_temukan);
    register_fn(env, "regex_ganti", Some(3), builtin_regex_ganti);
    register_fn(env, "regex_tangkap", Some(2), builtin_regex_tangkap);
}
