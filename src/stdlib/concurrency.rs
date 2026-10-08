use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Mutex, OnceLock};
use std::thread;

static CHANNEL_NEXT_ID: AtomicU64 = AtomicU64::new(1);
static THREAD_NEXT_ID: AtomicU64 = AtomicU64::new(1);

type ChannelMap = Mutex<BTreeMap<u64, (mpsc::Sender<ValueSerial>, mpsc::Receiver<ValueSerial>)>>;
type ThreadMap = Mutex<BTreeMap<u64, thread::JoinHandle<Result<ValueSerial, String>>>>;

fn get_channels() -> &'static ChannelMap {
    static CHANNELS: OnceLock<ChannelMap> = OnceLock::new();
    CHANNELS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn get_threads() -> &'static ThreadMap {
    static THREADS: OnceLock<ThreadMap> = OnceLock::new();
    THREADS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

#[derive(Clone, Debug)]
pub enum ValueSerial {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<ValueSerial>),
    Map(Vec<(String, ValueSerial)>),
}

impl ValueSerial {
    pub fn from_value(v: &Value) -> Self {
        match v {
            Value::Nil => ValueSerial::Nil,
            Value::Bool(b) => ValueSerial::Bool(*b),
            Value::Number(n) => ValueSerial::Number(*n),
            Value::String(s) => ValueSerial::String(s.clone()),
            Value::Array(a) => {
                let items = a.borrow();
                ValueSerial::Array(items.iter().map(ValueSerial::from_value).collect())
            }
            Value::Map(m) => {
                let map = m.borrow();
                ValueSerial::Map(map.iter().map(|(k, val)| (k.clone(), ValueSerial::from_value(val))).collect())
            }
            other => ValueSerial::String(other.to_string_repr()),
        }
    }

    pub fn to_value(&self) -> Value {
        match self {
            ValueSerial::Nil => Value::Nil,
            ValueSerial::Bool(b) => Value::Bool(*b),
            ValueSerial::Number(n) => Value::Number(*n),
            ValueSerial::String(s) => Value::String(s.clone()),
            ValueSerial::Array(arr) => {
                let items: Vec<Value> = arr.iter().map(ValueSerial::to_value).collect();
                Value::Array(Rc::new(RefCell::new(items)))
            }
            ValueSerial::Map(pairs) => {
                let mut map = HashMap::new();
                for (k, v) in pairs {
                    map.insert(k.clone(), v.to_value());
                }
                Value::Map(Rc::new(RefCell::new(map)))
            }
        }
    }
}

pub fn builtin_buat_utas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Function(f) => {
            let body = f.body.clone();
            let thread_id = THREAD_NEXT_ID.fetch_add(1, Ordering::SeqCst);
            let handle = thread::spawn(move || {
                let mut interp = crate::interpreter::Interpreter::new();
                let mut last = Value::Nil;
                for stmt in &body {
                    match interp.execute(stmt) {
                        Ok(v) => last = v,
                        Err(Galat::Kembalikan(v)) => {
                            last = v;
                            break;
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
                Ok(ValueSerial::from_value(&last))
            });

            if let Ok(mut handles) = get_threads().lock() {
                handles.insert(thread_id, handle);
            }

            let mut res = HashMap::new();
            res.insert("_tipe".to_string(), Value::String("Utas".to_string()));
            res.insert("id".to_string(), Value::Number(thread_id as f64));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        other => Err(Galat::runtime(format!("'buat_utas' memerlukan fungsi, ditemukan '{}'", other.type_name()), span)),
    }
}

pub fn builtin_gabung_utas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let thread_id = match &args[0] {
        Value::Number(n) => *n as u64,
        Value::Map(m) => {
            let map = m.borrow();
            match map.get("id") {
                Some(Value::Number(n)) => *n as u64,
                _ => return Err(Galat::runtime("Objek Utas tidak valid", span)),
            }
        }
        other => return Err(Galat::runtime(format!("'gabung_utas' memerlukan ID utas atau objek Utas, ditemukan '{}'", other.type_name()), span)),
    };

    let handle_opt = if let Ok(mut handles) = get_threads().lock() {
        handles.remove(&thread_id)
    } else {
        None
    };

    if let Some(handle) = handle_opt {
        match handle.join() {
            Ok(Ok(val_serial)) => Ok(val_serial.to_value()),
            Ok(Err(e)) => Err(Galat::runtime(format!("Galat pada utas: {}", e), span)),
            Err(_) => Err(Galat::runtime("Utas (thread) mengalami kepanikan (panicked)", span)),
        }
    } else {
        Err(Galat::runtime(format!("Utas dengan ID {} tidak ditemukan atau sudah digabung", thread_id), span))
    }
}

pub fn builtin_saluran(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let (tx, rx) = mpsc::channel::<ValueSerial>();
    let ch_id = CHANNEL_NEXT_ID.fetch_add(1, Ordering::SeqCst);

    if let Ok(mut channels) = get_channels().lock() {
        channels.insert(ch_id, (tx, rx));
    }

    let mut result_map = HashMap::new();
    result_map.insert("_tipe".to_string(), Value::String("Saluran".to_string()));
    result_map.insert("id".to_string(), Value::Number(ch_id as f64));
    Ok(Value::Map(Rc::new(RefCell::new(result_map))))
}

pub fn builtin_saluran_kirim(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ch_id = match &args[0] {
        Value::Number(n) => *n as u64,
        Value::Map(m) => {
            let map = m.borrow();
            match map.get("id") {
                Some(Value::Number(n)) => *n as u64,
                _ => return Err(Galat::runtime("Objek Saluran tidak valid", span)),
            }
        }
        _ => return Err(Galat::runtime("Argumen pertama 'saluran_kirim' harus berupa Saluran atau ID Saluran", span)),
    };

    let val_serial = ValueSerial::from_value(&args[1]);
    if let Ok(channels) = get_channels().lock() {
        if let Some((tx, _)) = channels.get(&ch_id) {
            match tx.send(val_serial) {
                Ok(_) => Ok(Value::Bool(true)),
                Err(e) => Err(Galat::runtime(format!("Gagal mengirim pesan saluran: {}", e), span)),
            }
        } else {
            Err(Galat::runtime(format!("Saluran dengan ID {} tidak ditemukan", ch_id), span))
        }
    } else {
        Err(Galat::runtime("Kunci lock saluran bermasalah", span))
    }
}

pub fn builtin_saluran_terima(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ch_id = match &args[0] {
        Value::Number(n) => *n as u64,
        Value::Map(m) => {
            let map = m.borrow();
            match map.get("id") {
                Some(Value::Number(n)) => *n as u64,
                _ => return Err(Galat::runtime("Objek Saluran tidak valid", span)),
            }
        }
        _ => return Err(Galat::runtime("Argumen pertama 'saluran_terima' harus berupa Saluran atau ID Saluran", span)),
    };

    if let Ok(channels) = get_channels().lock() {
        if let Some((_, rx)) = channels.get(&ch_id) {
            match rx.recv() {
                Ok(val_serial) => Ok(val_serial.to_value()),
                Err(e) => Err(Galat::runtime(format!("Saluran penerimaan terputus: {}", e), span)),
            }
        } else {
            Err(Galat::runtime(format!("Saluran dengan ID {} tidak ditemukan", ch_id), span))
        }
    } else {
        Err(Galat::runtime("Kunci lock saluran bermasalah", span))
    }
}

pub fn register_concurrency(env: &mut Environment) {
    register_fn(env, "buat_utas", Some(1), builtin_buat_utas);
    register_fn(env, "gabung_utas", Some(1), builtin_gabung_utas);
    register_fn(env, "saluran", Some(0), builtin_saluran);
    register_fn(env, "saluran_kirim", Some(2), builtin_saluran_kirim);
    register_fn(env, "saluran_terima", Some(1), builtin_saluran_terima);
}
