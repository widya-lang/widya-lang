use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::value::{BuiltinFunction, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;
use std::sync::{mpsc, Mutex, OnceLock};
use std::time::Instant;

static STARTUP_INSTANT: OnceLock<Instant> = OnceLock::new();

pub fn get_startup_instant() -> Instant {
    *STARTUP_INSTANT.get_or_init(Instant::now)
}

thread_local! {
    static KELUARAN_TERTANGKAP: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
    static SINK_ALIRAN: RefCell<Option<mpsc::Sender<String>>> = const { RefCell::new(None) };
}

pub fn mulai_tangkap_keluaran() {
    KELUARAN_TERTANGKAP.with(|c| *c.borrow_mut() = Some(Vec::new()));
}

pub fn mulai_sink_aliran(pengirim: mpsc::Sender<String>) {
    SINK_ALIRAN.with(|c| *c.borrow_mut() = Some(pengirim));
}

pub fn ambil_keluaran_tertangkan() -> Vec<String> {
    KELUARAN_TERTANGKAP.with(|c| c.borrow_mut().take().unwrap_or_default())
}

pub fn emit_baris(baris: String) {
    if SINK_ALIRAN.with(|c| c.borrow().is_some()) {
        SINK_ALIRAN.with(|c| {
            if let Some(tx) = c.borrow().as_ref() {
                let _ = tx.send(baris);
            }
        });
        return;
    }
    KELUARAN_TERTANGKAP.with(|c| {
        let mut b = c.borrow_mut();
        if let Some(col) = b.as_mut() {
            col.push(baris);
        } else {
            println!("{}", baris);
        }
    });
}

pub fn emit_teks(teks: String) {
    if SINK_ALIRAN.with(|c| c.borrow().is_some()) {
        SINK_ALIRAN.with(|c| {
            if let Some(tx) = c.borrow().as_ref() {
                let _ = tx.send(teks);
            }
        });
        return;
    }
    KELUARAN_TERTANGKAP.with(|c| {
        let mut b = c.borrow_mut();
        if let Some(col) = b.as_mut() {
            col.push(teks);
        } else {
            print!("{}", teks);
            let _ = io::stdout().flush();
        }
    });
}

pub fn register_fn(
    env: &mut Environment,
    name: &str,
    arity: Option<usize>,
    func: crate::value::BuiltinFn,
) {
    let builtin = BuiltinFunction {
        name: name.to_string(),
        arity,
        func,
    };
    env.define(name.to_string(), Value::Builtin(Rc::new(builtin)), true);
}

pub fn value_to_json(val: &Value) -> serde_json::Value {
    match val {
        Value::Nil => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Number(n) => {
            if n.fract() == 0.0 && !n.is_nan() && !n.is_infinite() {
                if *n >= 0.0 && *n <= (u64::MAX as f64) {
                    serde_json::Value::Number((*n as u64).into())
                } else if *n >= (i64::MIN as f64) && *n <= (i64::MAX as f64) {
                    serde_json::Value::Number((*n as i64).into())
                } else if let Some(num) = serde_json::Number::from_f64(*n) {
                    serde_json::Value::Number(num)
                } else {
                    serde_json::Value::Null
                }
            } else if let Some(num) = serde_json::Number::from_f64(*n) {
                serde_json::Value::Number(num)
            } else {
                serde_json::Value::Null
            }
        }
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Array(a) => {
            let items = a.borrow();
            let json_arr: Vec<serde_json::Value> = items.iter().map(value_to_json).collect();
            serde_json::Value::Array(json_arr)
        }
        Value::Map(m) => {
            let map = m.borrow();
            let mut json_obj = serde_json::Map::new();
            for (k, v) in map.iter() {
                json_obj.insert(k.clone(), value_to_json(v));
            }
            serde_json::Value::Object(json_obj)
        }
        Value::Instance(inst) => {
            let instance = inst.borrow();
            let mut json_obj = serde_json::Map::new();
            for (k, v) in instance.fields.iter() {
                json_obj.insert(k.clone(), value_to_json(v));
            }
            serde_json::Value::Object(json_obj)
        }
        other => serde_json::Value::String(other.to_string_repr()),
    }
}

pub fn json_to_value(json: serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            let items: Vec<Value> = arr.into_iter().map(json_to_value).collect();
            Value::Array(Rc::new(RefCell::new(items)))
        }
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::new();
            for (k, v) in obj {
                map.insert(k, json_to_value(v));
            }
            Value::Map(Rc::new(RefCell::new(map)))
        }
    }
}

pub fn json_ke_widya_val(json: serde_json::Value) -> Value {
    json_to_value(json)
}

pub use value_to_json as widya_val_ke_json;

fn get_mem_pool() -> &'static Mutex<HashMap<usize, f64>> {
    static MEM_POOL: OnceLock<Mutex<HashMap<usize, f64>>> = OnceLock::new();
    MEM_POOL.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn baca_memori_volatil(alamat: usize, _span: &Span) -> Result<Value, Galat> {
    let pool = get_mem_pool().lock().unwrap();
    let val = pool.get(&alamat).copied().unwrap_or(0.0);
    Ok(Value::Number(val))
}

pub fn tulis_memori_volatil(alamat: usize, nilai: &Value, span: &Span) -> Result<Value, Galat> {
    let num = match nilai {
        Value::Number(n) => *n,
        Value::Bool(b) => if *b { 1.0 } else { 0.0 },
        _ => return Err(Galat::runtime("Nilai memori volatil harus berupa angka atau boolean", span)),
    };
    let mut pool = get_mem_pool().lock().unwrap();
    pool.insert(alamat, num);
    Ok(Value::Nil)
}
