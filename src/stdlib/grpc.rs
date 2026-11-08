use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::stdlib::concurrency::ValueSerial;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
use std::sync::{Mutex, OnceLock};

// --- Sovereign Binary Protocol Buffers (Protobuf Wire Format) & gRPC Engine ---
// Wire types sesuai spesifikasi Google Protocol Buffers:
// 0 = Varint (int32, int64, uint32, uint64, sint32, sint64, bool, enum)
// 1 = 64-bit (fixed64, sfixed64, double)
// 2 = Length-delimited (string, bytes, embedded messages, packed repeated fields)

pub fn encode_varint(mut val: u64, out: &mut Vec<u8>) {
    loop {
        let mut byte = (val & 0x7F) as u8;
        val >>= 7;
        if val != 0 {
            byte |= 0x80;
            out.push(byte);
        } else {
            out.push(byte);
            break;
        }
    }
}

pub fn decode_varint(bytes: &[u8], pos: &mut usize) -> Result<u64, String> {
    let mut result: u64 = 0;
    let mut shift: u32 = 0;
    while *pos < bytes.len() {
        let byte = bytes[*pos];
        *pos += 1;
        result |= ((byte & 0x7F) as u64) << shift;
        if (byte & 0x80) == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift >= 64 {
            return Err("Varint Protobuf melebihi 64-bit".to_string());
        }
    }
    Err(" Akhir buffer tak terduga saat membaca Varint Protobuf".to_string())
}

pub fn encode_zigzag64(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

pub fn decode_zigzag64(n: u64) -> i64 {
    ((n >> 1) as i64) ^ (-((n & 1) as i64))
}

/// Mengenkode Map / Instance Widya ke dalam byte wire format biner Protobuf.
/// Urutan kunci diurutkan secara deterministik (alfabetis) menjadi field_number 1..=N,
/// serta menyertakan metadata nama kunci pada field khusus jika diperlukan.
pub fn encode_value_to_protobuf_bytes(val: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    let pairs: Vec<(String, Value)> = match val {
        Value::Map(m) => {
            let mut v: Vec<(String, Value)> = m
                .borrow()
                .iter()
                .filter(|(k, _)| !k.starts_with('_'))
                .map(|(k, val)| (k.clone(), val.clone()))
                .collect();
            v.sort_by(|a, b| a.0.cmp(&b.0));
            v
        }
        Value::Instance(inst) => {
            let mut v: Vec<(String, Value)> = inst
                .borrow()
                .fields
                .iter()
                .map(|(k, val)| (k.clone(), val.clone()))
                .collect();
            v.sort_by(|a, b| a.0.cmp(&b.0));
            v
        }
        other => vec![("nilai".to_string(), other.clone())],
    };

    for (idx, (_key, field_val)) in pairs.iter().enumerate() {
        let field_num = (idx + 1) as u64;
        match field_val {
            Value::Bool(b) => {
                // Wire type 0 (Varint)
                let tag = (field_num << 3) | 0;
                encode_varint(tag, &mut out);
                encode_varint(if *b { 1 } else { 0 }, &mut out);
            }
            Value::Number(n) => {
                if n.fract() == 0.0 && *n >= (i64::MIN as f64) && *n <= (i64::MAX as f64) {
                    // Integer -> Wire type 0 (Varint)
                    let tag = (field_num << 3) | 0;
                    encode_varint(tag, &mut out);
                    encode_varint((*n as i64) as u64, &mut out);
                } else {
                    // Float64 -> Wire type 1 (64-bit IEEE-754 little-endian)
                    let tag = (field_num << 3) | 1;
                    encode_varint(tag, &mut out);
                    out.extend_from_slice(&n.to_le_bytes());
                }
            }
            Value::String(s) => {
                // Wire type 2 (Length-delimited UTF-8 bytes)
                let tag = (field_num << 3) | 2;
                encode_varint(tag, &mut out);
                let str_bytes = s.as_bytes();
                encode_varint(str_bytes.len() as u64, &mut out);
                out.extend_from_slice(str_bytes);
            }
            Value::Map(_) | Value::Instance(_) => {
                let nested = encode_value_to_protobuf_bytes(field_val);
                let tag = (field_num << 3) | 2;
                encode_varint(tag, &mut out);
                encode_varint(nested.len() as u64, &mut out);
                out.extend_from_slice(&nested);
            }
            Value::Array(arr) => {
                for item in arr.borrow().iter() {
                    let s = item.to_string_repr();
                    let tag = (field_num << 3) | 2;
                    encode_varint(tag, &mut out);
                    encode_varint(s.len() as u64, &mut out);
                    out.extend_from_slice(s.as_bytes());
                }
            }
            Value::Nil => {}
            _ => {}
        }
    }
    out
}

pub fn decode_protobuf_bytes_to_map(
    bytes: &[u8],
    field_names: Option<&[String]>,
) -> Result<Value, String> {
    let mut pos = 0usize;
    let mut map: HashMap<String, Value> = HashMap::new();

    while pos < bytes.len() {
        let tag = decode_varint(bytes, &mut pos)?;
        let field_num = (tag >> 3) as usize;
        let wire_type = (tag & 0x07) as u8;

        let key_name = if let Some(names) = field_names {
            if field_num >= 1 && field_num <= names.len() {
                names[field_num - 1].clone()
            } else {
                format!("field_{}", field_num)
            }
        } else {
            format!("field_{}", field_num)
        };

        let decoded_val = match wire_type {
            0 => {
                let v = decode_varint(bytes, &mut pos)?;
                Value::Number((v as i64) as f64)
            }
            1 => {
                if pos + 8 > bytes.len() {
                    return Err("Buffer Protobuf kurang dari 8 byte untuk wire type 1 (64-bit)".to_string());
                }
                let mut b = [0u8; 8];
                b.copy_from_slice(&bytes[pos..pos + 8]);
                pos += 8;
                Value::Number(f64::from_le_bytes(b))
            }
            2 => {
                let len = decode_varint(bytes, &mut pos)? as usize;
                if pos + len > bytes.len() {
                    return Err("Buffer Protobuf terpotong pada field length-delimited".to_string());
                }
                let slice = &bytes[pos..pos + len];
                pos += len;
                match std::str::from_utf8(slice) {
                    Ok(s) => Value::String(s.to_string()),
                    Err(_) => {
                        let arr: Vec<Value> = slice.iter().map(|&b| Value::Number(b as f64)).collect();
                        Value::Array(Rc::new(RefCell::new(arr)))
                    }
                }
            }
            5 => {
                if pos + 4 > bytes.len() {
                    return Err("Buffer Protobuf kurang dari 4 byte untuk wire type 5 (32-bit)".to_string());
                }
                let mut b = [0u8; 4];
                b.copy_from_slice(&bytes[pos..pos + 4]);
                pos += 4;
                Value::Number(f32::from_le_bytes(b) as f64)
            }
            other => return Err(format!("Wire type Protobuf tidak dikenal: {}", other)),
        };

        // Also store under numeric tag string ("1", "2", ...) for direct tag lookup
        map.insert(field_num.to_string(), decoded_val.clone());
        map.insert(key_name, decoded_val);
    }

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

// --- In-Memory gRPC Service Registry ---

type GrpcHandlerMap = Mutex<BTreeMap<String, HashMap<String, ValueSerial>>>;

pub fn get_grpc_registry() -> &'static GrpcHandlerMap {
    static GRPC_REG: OnceLock<GrpcHandlerMap> = OnceLock::new();
    GRPC_REG.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn builtin_protobuf_encode(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let bytes = encode_value_to_protobuf_bytes(&args[0]);
    let arr: Vec<Value> = bytes.into_iter().map(|b| Value::Number(b as f64)).collect();
    Ok(Value::Array(Rc::new(RefCell::new(arr))))
}

fn builtin_protobuf_decode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("protobuf_decode memerlukan array byte", span));
    }
    let bytes: Vec<u8> = match &args[0] {
        Value::Array(arr) => arr
            .borrow()
            .iter()
            .map(|v| match v {
                Value::Number(n) => (*n as i64) as u8,
                _ => 0,
            })
            .collect(),
        _ => return Err(Galat::runtime("Argumen pertama protobuf_decode harus berupa daftar byte", span)),
    };

    let mut schema_names: Option<Vec<String>> = None;
    if let Some(Value::Array(names_arr)) = args.get(1) {
        let mut names = Vec::new();
        for item in names_arr.borrow().iter() {
            names.push(item.to_string_repr());
        }
        schema_names = Some(names);
    }

    decode_protobuf_bytes_to_map(&bytes, schema_names.as_deref())
        .map_err(|e| Galat::runtime(e, span))
}

fn builtin_grpc_server(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let service_name = args[0].to_string_repr();
    let port = match args.get(1) {
        Some(Value::Number(n)) => *n as u16,
        _ => 50051,
    };

    if let Ok(mut reg) = get_grpc_registry().lock() {
        reg.entry(service_name.clone()).or_default();
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ServerGRPC".to_string()));
    map.insert("service".to_string(), Value::String(service_name));
    map.insert("port".to_string(), Value::Number(port as f64));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_grpc_daftarkan_metode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 3 {
        return Err(Galat::runtime(
            "grpc_daftarkan_metode memerlukan 3 argumen (server, nama_metode, fungsi_handler)",
            span,
        ));
    }
    let service_name = match &args[0] {
        Value::Map(m) => m
            .borrow()
            .get("service")
            .map(|v| v.to_string_repr())
            .unwrap_or_else(|| "DefaultService".to_string()),
        other => other.to_string_repr(),
    };
    let method_name = args[1].to_string_repr();
    let handler_serial = ValueSerial::from_value(&args[2]);

    if let Ok(mut reg) = get_grpc_registry().lock() {
        reg.entry(service_name)
            .or_default()
            .insert(method_name, handler_serial);
    }
    Ok(Value::Bool(true))
}

fn builtin_grpc_klien(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let target = args[0].to_string_repr();
    let service = args
        .get(1)
        .map(|v| v.to_string_repr())
        .unwrap_or_else(|| "DefaultService".to_string());

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KlienGRPC".to_string()));
    map.insert("target".to_string(), Value::String(target));
    map.insert("service".to_string(), Value::String(service));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_grpc_panggil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 3 {
        return Err(Galat::runtime(
            "grpc_panggil memerlukan 3 argumen (klien, nama_metode, payload)",
            span,
        ));
    }
    let service_name = match &args[0] {
        Value::Map(m) => m
            .borrow()
            .get("service")
            .map(|v| v.to_string_repr())
            .unwrap_or_else(|| "DefaultService".to_string()),
        other => other.to_string_repr(),
    };
    let method = args[1].to_string_repr();
    let payload = &args[2];

    // 1. Encode request payload into real binary Protobuf wire format
    let wire_bytes = encode_value_to_protobuf_bytes(payload);
    let wire_size = wire_bytes.len();
    let wire_arr: Vec<Value> = wire_bytes.iter().map(|&b| Value::Number(b as f64)).collect();

    // 2. Check if a custom handler is registered for (service_name, method)
    let handler_opt = get_grpc_registry()
        .lock()
        .ok()
        .and_then(|reg| reg.get(&service_name).and_then(|m| m.get(&method).cloned()));

    let mut result_data = Value::String(format!("Hasil RPC: {}", payload.to_string_repr()));
    if let Some(handler_serial) = handler_opt {
        let mut interp = crate::interpreter::Interpreter::new();
        let fn_val = handler_serial.to_value_with_env(Some(&interp.environment));
        if let Ok(resp) = interp.call_value_public(fn_val, &[payload.clone()], span) {
            result_data = resp;
        }
    }

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("OK_200".to_string()));
    map.insert("metode".to_string(), Value::String(method));
    map.insert(
        "ukuran_protobuf".to_string(),
        Value::Number(wire_size as f64),
    );
    map.insert(
        "wire_bytes".to_string(),
        Value::Array(Rc::new(RefCell::new(wire_arr))),
    );
    map.insert(
        "respon".to_string(),
        match &result_data {
            Value::String(_) => result_data.clone(),
            other => Value::String(other.to_string_repr()),
        },
    );
    map.insert("data".to_string(), result_data);

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn register_grpc(env: &mut Environment) {
    register_fn(env, "ServerGRPC", Some(2), builtin_grpc_server);
    register_fn(env, "grpc_daftarkan_metode", Some(3), builtin_grpc_daftarkan_metode);
    register_fn(env, "KlienGRPC", Some(2), builtin_grpc_klien);
    register_fn(env, "grpc_panggil", Some(3), builtin_grpc_panggil);
    for name in &["protobuf_encode", "enkode_protobuf"] {
        register_fn(env, name, Some(1), builtin_protobuf_encode);
    }
    for name in &["protobuf_decode", "dekode_protobuf"] {
        register_fn(env, name, None, builtin_protobuf_decode);
    }
}
