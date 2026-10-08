use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn register_industry(env: &mut Environment) {
    // HealthTech & SatuSehat
    register_fn(env, "kesehatan_satusehat_pasien", Some(5), builtin_kesehatan_satusehat_pasien);
    register_fn(env, "kesehatan_satusehat_observasi_sistolik", Some(3), builtin_kesehatan_satusehat_observasi_sistolik);
    register_fn(env, "kesehatan_cari_icd10", Some(1), builtin_kesehatan_cari_icd10);
    // IoT & Agroindustri
    register_fn(env, "iot_cocokkan_topik_mqtt", Some(2), builtin_iot_cocokkan_topik_mqtt);
    register_fn(env, "iot_periksa_geofence", Some(5), builtin_iot_periksa_geofence);
    // AI & Machine Learning
    register_fn(env, "ai_tensor_matmul", Some(2), builtin_ai_tensor_matmul);
    register_fn(env, "ai_tensor_cosine_similarity", Some(2), builtin_ai_tensor_cosine_similarity);
    // Otomotif & CAN Bus
    register_fn(env, "otomotif_dekode_obd2", Some(1), builtin_otomotif_dekode_obd2);
}

// --- HealthTech & SatuSehat Builtin Implementations ---

fn builtin_kesehatan_satusehat_pasien(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nik = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 1 NIK harus string", span)) };
    let nama = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 2 nama harus string", span)) };
    let gender = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 3 gender ('male'/'female') harus string", span)) };
    let birth_date = match &args[3] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 4 tgl lahir ('YYYY-MM-DD') harus string", span)) };
    let telepon = match &args[4] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 5 telepon harus string", span)) };

    let patient = crate::healthtech::FhirPatient::new_satusehat(nik, nama, gender, birth_date, telepon);
    let json_val = serde_json::to_value(&patient).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_kesehatan_satusehat_observasi_sistolik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let patient_id = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 1 ID Pasien harus string", span)) };
    let value_mmhg = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Argumen 2 nilai sistolik harus angka", span)) };
    let waktu = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 3 waktu ISO harus string", span)) };

    let obs = crate::healthtech::FhirObservation::sistolik(patient_id, value_mmhg, waktu);
    let json_val = serde_json::to_value(&obs).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

fn builtin_kesehatan_cari_icd10(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let query = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 1 kata kunci harus string", span)) };
    let results = crate::healthtech::cari_diagnosis_icd10(query);
    let json_val = serde_json::to_value(&results).unwrap_or_default();
    Ok(json_ke_widya_val(json_val))
}

// --- IoT & Smart Cities Builtin Implementations ---

fn builtin_iot_cocokkan_topik_mqtt(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let sub_filter = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 1 filter topik harus string", span)) };
    let pub_topic = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 2 publish topic harus string", span)) };

    let matched = crate::iot::cocokkan_topik_mqtt(sub_filter, pub_topic);
    Ok(Value::Bool(matched))
}

fn builtin_iot_periksa_geofence(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lat = match &args[0] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Argumen 1 lat harus angka", span)) };
    let lon = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Argumen 2 lon harus angka", span)) };
    let c_lat = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Argumen 3 center lat harus angka", span)) };
    let c_lon = match &args[3] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Argumen 4 center lon harus angka", span)) };
    let radius = match &args[4] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Argumen 5 radius meter harus angka", span)) };

    let inside = crate::iot::periksa_geofence(lat, lon, c_lat, c_lon, radius);
    Ok(Value::Bool(inside))
}

// --- AI & Machine Learning Builtin Implementations ---

fn builtin_ai_tensor_matmul(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let (m1, k1, d1) = match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let rows = map.get("baris").and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(1);
            let cols = map.get("kolom").and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(1);
            let data = map.get("data").and_then(|v| if let Value::Array(arr) = v {
                Some(arr.borrow().iter().filter_map(|val| if let Value::Number(n) = val { Some(*n as f32) } else { None }).collect())
            } else { None }).unwrap_or_default();
            (rows, cols, data)
        }
        _ => return Err(Galat::runtime("Argumen 1 harus berupa objek tensor", span)),
    };

    let (k2, n2, d2) = match &args[1] {
        Value::Map(m) => {
            let map = m.borrow();
            let rows = map.get("baris").and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(1);
            let cols = map.get("kolom").and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(1);
            let data = map.get("data").and_then(|v| if let Value::Array(arr) = v {
                Some(arr.borrow().iter().filter_map(|val| if let Value::Number(n) = val { Some(*n as f32) } else { None }).collect())
            } else { None }).unwrap_or_default();
            (rows, cols, data)
        }
        _ => return Err(Galat::runtime("Argumen 2 harus berupa objek tensor", span)),
    };

    let t1 = crate::ai::Tensor::new(vec![m1, k1], d1);
    let t2 = crate::ai::Tensor::new(vec![k2, n2], d2);

    match t1.matmul(&t2) {
        Ok(res) => {
            let mut map = HashMap::new();
            map.insert("baris".to_string(), Value::Number(res.shape[0] as f64));
            map.insert("kolom".to_string(), Value::Number(res.shape[1] as f64));
            let arr_vals: Vec<Value> = res.data.into_iter().map(|n| Value::Number(n as f64)).collect();
            map.insert("data".to_string(), Value::Array(Rc::new(RefCell::new(arr_vals))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Err(Galat::runtime(format!("Gagal perkalian matriks tensor: {}", e), span)),
    }
}

fn builtin_ai_tensor_cosine_similarity(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let d1: Vec<f32> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().filter_map(|v| if let Value::Number(n) = v { Some(*n as f32) } else { None }).collect(),
        _ => return Err(Galat::runtime("Argumen 1 vektor embedding harus array angka", span)),
    };
    let d2: Vec<f32> = match &args[1] {
        Value::Array(arr) => arr.borrow().iter().filter_map(|v| if let Value::Number(n) = v { Some(*n as f32) } else { None }).collect(),
        _ => return Err(Galat::runtime("Argumen 2 vektor embedding harus array angka", span)),
    };

    let t1 = crate::ai::Tensor::new(vec![d1.len()], d1);
    let t2 = crate::ai::Tensor::new(vec![d2.len()], d2);
    let sim = t1.cosine_similarity(&t2);
    Ok(Value::Number(sim as f64))
}

// --- Otomotif & CAN Bus Builtin Implementations ---

fn builtin_otomotif_dekode_obd2(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let bytes: Vec<u8> = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().filter_map(|v| if let Value::Number(n) = v { Some(*n as u8) } else { None }).collect(),
        _ => return Err(Galat::runtime("Argumen 1 byte CAN harus array angka", span)),
    };

    let frame = crate::automotive::CanFrame::new_standard(0x7E8, &bytes);
    let mut map = HashMap::new();
    if let Some((pid_name, val)) = crate::automotive::dekode_obd2_can_frame(&frame) {
        map.insert("valid".to_string(), Value::Bool(true));
        map.insert("pid".to_string(), Value::String(pid_name.to_string()));
        map.insert("nilai".to_string(), Value::Number(val));
    } else {
        map.insert("valid".to_string(), Value::Bool(false));
    }

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

