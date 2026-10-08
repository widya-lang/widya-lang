use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// --- WidyaSIMD Implementation (High-Performance Vectorization) ---

fn extract_vector_f32x4(val: &Value, span: &Span) -> Result<[f32; 4], Galat> {
    match val {
        Value::Map(m) => {
            let map = m.borrow();
            let x0 = match map.get("x") { Some(Value::Number(n)) => *n as f32, _ => 0.0 };
            let x1 = match map.get("y") { Some(Value::Number(n)) => *n as f32, _ => 0.0 };
            let x2 = match map.get("z") { Some(Value::Number(n)) => *n as f32, _ => 0.0 };
            let x3 = match map.get("w") { Some(Value::Number(n)) => *n as f32, _ => 0.0 };
            Ok([x0, x1, x2, x3])
        }
        Value::Array(arr) => {
            let a = arr.borrow();
            let mut out = [0.0f32; 4];
            for i in 0..4 {
                if i < a.len() {
                    if let Value::Number(n) = a[i] {
                        out[i] = n as f32;
                    }
                }
            }
            Ok(out)
        }
        _ => Err(Galat::runtime(
            "Diharapkan objek VektorF32x4 atau Larik numerik 4-elemen".to_string(),
            span,
        )),
    }
}

fn make_vector_f32x4_value(v: [f32; 4]) -> Value {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("VektorF32x4".to_string()));
    map.insert("x".to_string(), Value::Number(v[0] as f64));
    map.insert("y".to_string(), Value::Number(v[1] as f64));
    map.insert("z".to_string(), Value::Number(v[2] as f64));
    map.insert("w".to_string(), Value::Number(v[3] as f64));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_simd_vektor_f32x4(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let x = match &args[0] { Value::Number(n) => *n as f32, _ => 0.0 };
    let y = match &args[1] { Value::Number(n) => *n as f32, _ => 0.0 };
    let z = match &args[2] { Value::Number(n) => *n as f32, _ => 0.0 };
    let w = match &args[3] { Value::Number(n) => *n as f32, _ => 0.0 };
    Ok(make_vector_f32x4_value([x, y, z, w]))
}

fn builtin_simd_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let a = extract_vector_f32x4(&args[0], span)?;
    let b = extract_vector_f32x4(&args[1], span)?;
    let res = [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]];
    Ok(make_vector_f32x4_value(res))
}

fn builtin_simd_kali(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let a = extract_vector_f32x4(&args[0], span)?;
    let b = extract_vector_f32x4(&args[1], span)?;
    let res = [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]];
    Ok(make_vector_f32x4_value(res))
}

fn builtin_simd_dot(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let a = extract_vector_f32x4(&args[0], span)?;
    let b = extract_vector_f32x4(&args[1], span)?;
    let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
    Ok(Value::Number(dot as f64))
}

fn builtin_simd_jarak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let a = extract_vector_f32x4(&args[0], span)?;
    let b = extract_vector_f32x4(&args[1], span)?;
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    let dw = a[3] - b[3];
    let sum_sq = dx * dx + dy * dy + dz * dz + dw * dw;
    Ok(Value::Number((sum_sq as f64).sqrt()))
}

fn builtin_simd_ke_daftar(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let v = extract_vector_f32x4(&args[0], span)?;
    let list = vec![
        Value::Number(v[0] as f64),
        Value::Number(v[1] as f64),
        Value::Number(v[2] as f64),
        Value::Number(v[3] as f64),
    ];
    Ok(Value::Array(Rc::new(RefCell::new(list))))
}


pub fn register_simd(env: &mut Environment) {
    register_fn(env, "VektorF32x4", Some(4), builtin_simd_vektor_f32x4);
    register_fn(env, "simd_tambah", Some(2), builtin_simd_tambah);
    register_fn(env, "simd_kali", Some(2), builtin_simd_kali);
    register_fn(env, "simd_dot", Some(2), builtin_simd_dot);
    register_fn(env, "simd_jarak", Some(2), builtin_simd_jarak);
    register_fn(env, "simd_ke_daftar", Some(1), builtin_simd_ke_daftar);
}
