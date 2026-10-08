use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn register_smartcity(env: &mut Environment) {
    // Smart City & Spatial Geo-intelligence Engine
    register_fn(env, "smartcity_hitung_ispu_pm25", Some(1), builtin_smartcity_hitung_ispu_pm25);
    register_fn(env, "smartcity_jarak_haversine", Some(4), builtin_smartcity_jarak_haversine);
    register_fn(env, "smartcity_titik_dalam_poligon", Some(3), builtin_smartcity_titik_dalam_poligon);
    register_fn(env, "smartcity_optimasi_lalu_lintas", Some(4), builtin_smartcity_optimasi_lalu_lintas);
    register_fn(env, "smartcity_analisis_bauran_energi", Some(4), builtin_smartcity_analisis_bauran_energi);
}

// ============================================================================
// SMART CITY & SPATIAL BUILTIN FUNCTIONS
// ============================================================================

fn builtin_smartcity_hitung_ispu_pm25(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let konsentrasi = match &args[0] {
        Value::Number(n) => *n,
        _ => 0.0,
    };
    let hasil = crate::smartcity::hitung_ispu_pm25(konsentrasi);
    let json_val = serde_json::to_value(&hasil).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_smartcity_jarak_haversine(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lat1 = match &args[0] { Value::Number(n) => *n, _ => 0.0 };
    let lon1 = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
    let lat2 = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
    let lon2 = match &args[3] { Value::Number(n) => *n, _ => 0.0 };

    let p1 = crate::smartcity::TitikKoordinat::new(lat1, lon1);
    let p2 = crate::smartcity::TitikKoordinat::new(lat2, lon2);
    let d = crate::smartcity::jarak_haversine_km(&p1, &p2);
    Ok(Value::Number(d))
}

fn builtin_smartcity_titik_dalam_poligon(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lat = match &args[0] { Value::Number(n) => *n, _ => 0.0 };
    let lon = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
    let titik = crate::smartcity::TitikKoordinat::new(lat, lon);

    let mut poligon = Vec::new();
    if let Value::Array(arr) = &args[2] {
        for v in arr.borrow().iter() {
            if let Value::Array(coords) = v {
                let c = coords.borrow();
                if c.len() >= 2 {
                    let plat = match &c[0] { Value::Number(n) => *n, _ => 0.0 };
                    let plon = match &c[1] { Value::Number(n) => *n, _ => 0.0 };
                    poligon.push(crate::smartcity::TitikKoordinat::new(plat, plon));
                }
            }
        }
    }

    let inside = crate::smartcity::titik_di_dalam_poligon(&titik, &poligon);
    Ok(Value::Bool(inside))
}


fn builtin_smartcity_optimasi_lalu_lintas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama = match &args[0] { Value::String(s) => s.as_str(), _ => "Jalan Utama" };
    let vol = match &args[1] { Value::Number(n) => *n, _ => 1000.0 };
    let cap = match &args[2] { Value::Number(n) => *n, _ => 1500.0 };
    let siklus = match &args[3] { Value::Number(n) => *n as u32, _ => 120 };

    let res = crate::smartcity::optimasi_lalu_lintas(nama, vol, cap, siklus);
    let json_val = serde_json::to_value(&res).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_smartcity_analisis_bauran_energi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let plts = match &args[0] { Value::Number(n) => *n, _ => 0.0 };
    let plta = match &args[1] { Value::Number(n) => *n, _ => 0.0 };
    let pltp = match &args[2] { Value::Number(n) => *n, _ => 0.0 };
    let fosil = match &args[3] { Value::Number(n) => *n, _ => 0.0 };

    let res = crate::smartcity::analisis_bauran_energi(plts, plta, pltp, fosil);
    let json_val = serde_json::to_value(&res).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

































































