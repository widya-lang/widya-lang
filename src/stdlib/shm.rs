use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{OnceLock, RwLock};

// --- WidyaShm (Zero-Copy Shared Memory IPC) Implementation ---

fn get_shm_pool() -> &'static RwLock<HashMap<String, Vec<u8>>> {
    static SHM_POOL: std::sync::OnceLock<RwLock<HashMap<String, Vec<u8>>>> = std::sync::OnceLock::new();
    SHM_POOL.get_or_init(|| RwLock::new(HashMap::new()))
}

fn builtin_shm_buka(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args[0].to_string_repr();
    let ukuran = match &args[1] { Value::Number(n) => *n as usize, _ => 1024 };

    let mut pool = get_shm_pool().write().unwrap();
    pool.entry(nama.clone()).or_insert_with(|| vec![0u8; ukuran]);

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MemoriBersama".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("ukuran".to_string(), Value::Number(ukuran as f64));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_shm_tulis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama = match &args[0] {
        Value::Map(m) => m.borrow().get("nama").map(|v| v.to_string_repr()).unwrap_or_default(),
        _ => args[0].to_string_repr(),
    };
    let data = args[1].to_string_repr();

    let mut pool = get_shm_pool().write().unwrap();
    if let Some(buf) = pool.get_mut(&nama) {
        let bytes = data.as_bytes();
        let copy_len = bytes.len().min(buf.len());
        buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
        Ok(Value::Number(copy_len as f64))
    } else {
        Err(Galat::runtime(format!("Segmen memori bersama '{}' tidak ditemukan", nama), span))
    }
}

fn builtin_shm_baca(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama = match &args[0] {
        Value::Map(m) => m.borrow().get("nama").map(|v| v.to_string_repr()).unwrap_or_default(),
        _ => args[0].to_string_repr(),
    };

    let pool = get_shm_pool().read().unwrap();
    if let Some(buf) = pool.get(&nama) {
        let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        let s = String::from_utf8_lossy(&buf[..end]).to_string();
        Ok(Value::String(s))
    } else {
        Err(Galat::runtime(format!("Segmen memori bersama '{}' tidak ditemukan", nama), span))
    }
}


pub fn register_shm(env: &mut Environment) {
    register_fn(env, "buka_memori_bersama", Some(2), builtin_shm_buka);
    register_fn(env, "tulis_memori_bersama", Some(2), builtin_shm_tulis);
    register_fn(env, "baca_memori_bersama", Some(1), builtin_shm_baca);
}
