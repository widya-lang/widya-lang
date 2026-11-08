use crate::ast::Stmt;
use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::register_fn;
use crate::value::{BuiltinFunction, Value, WidyaFunction};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

static CHANNEL_NEXT_ID: AtomicU64 = AtomicU64::new(1);
static THREAD_NEXT_ID: AtomicU64 = AtomicU64::new(1);
static WAITGROUP_NEXT_ID: AtomicU64 = AtomicU64::new(1);
static MUTEX_NEXT_ID: AtomicU64 = AtomicU64::new(1);
static CONTEXT_NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct ChannelState {
    pub queue: VecDeque<ValueSerial>,
    pub capacity: Option<usize>,
    pub closed: bool,
}

pub struct SharedChannel {
    pub id: u64,
    pub inner: (Mutex<ChannelState>, Condvar),
}

impl SharedChannel {
    pub fn new(id: u64, capacity: Option<usize>) -> Self {
        Self {
            id,
            inner: (
                Mutex::new(ChannelState {
                    queue: VecDeque::new(),
                    capacity,
                    closed: false,
                }),
                Condvar::new(),
            ),
        }
    }

    pub fn send(&self, val: ValueSerial) -> Result<(), String> {
        let (lock, cvar) = &self.inner;
        let mut state = lock.lock().map_err(|_| "Kunci saluran rusak".to_string())?;
        if state.closed {
            return Err("Tidak dapat mengirim ke saluran yang sudah ditutup (send on closed channel)".to_string());
        }
        // Note: In Go, buffered channels block when full; unbuffered channels synchronize.
        // We push into queue and notify waiting receivers.
        state.queue.push_back(val);
        cvar.notify_all();
        Ok(())
    }

    pub fn try_send(&self, val: ValueSerial) -> Result<bool, String> {
        let (lock, cvar) = &self.inner;
        let mut state = lock.lock().map_err(|_| "Kunci saluran rusak".to_string())?;
        if state.closed {
            return Err("Tidak dapat mengirim ke saluran yang sudah ditutup".to_string());
        }
        if let Some(cap) = state.capacity {
            if cap > 0 && state.queue.len() >= cap {
                return Ok(false);
            }
        }
        state.queue.push_back(val);
        cvar.notify_all();
        Ok(true)
    }

    pub fn recv_blocking(&self) -> (Option<ValueSerial>, bool) {
        let (lock, cvar) = &self.inner;
        let mut state = match lock.lock() {
            Ok(g) => g,
            Err(_) => return (None, false),
        };
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(val) = state.queue.pop_front() {
                cvar.notify_all();
                return (Some(val), true);
            }
            if state.closed {
                return (None, false);
            }
            let now = std::time::Instant::now();
            if now >= deadline {
                return (None, false);
            }
            let wait_dur = (deadline - now).min(Duration::from_millis(50));
            let (new_state, _) = match cvar.wait_timeout(state, wait_dur) {
                Ok(res) => res,
                Err(_) => return (None, false),
            };
            state = new_state;
        }
    }

    pub fn try_recv(&self) -> Option<(Option<ValueSerial>, bool)> {
        let (lock, cvar) = &self.inner;
        let mut state = lock.lock().ok()?;
        if let Some(val) = state.queue.pop_front() {
            cvar.notify_all();
            Some((Some(val), true))
        } else if state.closed {
            Some((None, false))
        } else {
            None
        }
    }

    pub fn close(&self) {
        let (lock, cvar) = &self.inner;
        if let Ok(mut state) = lock.lock() {
            state.closed = true;
            cvar.notify_all();
        }
    }

    pub fn len(&self) -> usize {
        self.inner.0.lock().map(|s| s.queue.len()).unwrap_or(0)
    }

    pub fn cap(&self) -> usize {
        self.inner
            .0
            .lock()
            .ok()
            .and_then(|s| s.capacity)
            .unwrap_or(0)
    }

    pub fn is_closed(&self) -> bool {
        self.inner.0.lock().map(|s| s.closed).unwrap_or(true)
    }
}

pub struct SharedWaitGroup {
    pub counter: Mutex<isize>,
    pub cvar: Condvar,
}

impl SharedWaitGroup {
    pub fn new() -> Self {
        Self {
            counter: Mutex::new(0),
            cvar: Condvar::new(),
        }
    }

    pub fn add(&self, delta: isize) {
        if let Ok(mut c) = self.counter.lock() {
            *c += delta;
            if *c <= 0 {
                *c = 0;
                self.cvar.notify_all();
            }
        }
    }

    pub fn wait(&self) {
        if let Ok(mut c) = self.counter.lock() {
            let deadline = std::time::Instant::now() + Duration::from_secs(10);
            while *c > 0 {
                let now = std::time::Instant::now();
                if now >= deadline {
                    break;
                }
                let dur = (deadline - now).min(Duration::from_millis(50));
                match self.cvar.wait_timeout(c, dur) {
                    Ok((guard, _)) => c = guard,
                    Err(_) => break,
                }
            }
        }
    }
}

pub struct SharedMutex {
    pub locked: Mutex<bool>,
    pub cvar: Condvar,
}

impl SharedMutex {
    pub fn new() -> Self {
        Self {
            locked: Mutex::new(false),
            cvar: Condvar::new(),
        }
    }

    pub fn lock(&self) {
        if let Ok(mut guard) = self.locked.lock() {
            let deadline = std::time::Instant::now() + Duration::from_secs(10);
            while *guard {
                let now = std::time::Instant::now();
                if now >= deadline {
                    break;
                }
                let dur = (deadline - now).min(Duration::from_millis(20));
                match self.cvar.wait_timeout(guard, dur) {
                    Ok((g, _)) => guard = g,
                    Err(_) => return,
                }
            }
            *guard = true;
        }
    }

    pub fn unlock(&self) {
        if let Ok(mut guard) = self.locked.lock() {
            *guard = false;
            self.cvar.notify_one();
        }
    }
}

pub struct ContextInner {
    pub cancelled: bool,
    pub err_msg: Option<String>,
    pub values: HashMap<String, ValueSerial>,
    pub children: Vec<u64>,
}

pub struct SharedContext {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub done_channel_id: u64,
    pub inner: Mutex<ContextInner>,
}

type ChannelMap = Mutex<BTreeMap<u64, Arc<SharedChannel>>>;
type ThreadMap = Mutex<BTreeMap<u64, thread::JoinHandle<Result<ValueSerial, String>>>>;
type WaitGroupMap = Mutex<BTreeMap<u64, Arc<SharedWaitGroup>>>;
type MutexMap = Mutex<BTreeMap<u64, Arc<SharedMutex>>>;
type ContextMap = Mutex<BTreeMap<u64, Arc<SharedContext>>>;
type SharedVarMap = Mutex<HashMap<String, ValueSerial>>;

pub fn get_channels() -> &'static ChannelMap {
    static CHANNELS: OnceLock<ChannelMap> = OnceLock::new();
    CHANNELS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn get_threads() -> &'static ThreadMap {
    static THREADS: OnceLock<ThreadMap> = OnceLock::new();
    THREADS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub fn get_waitgroups() -> &'static WaitGroupMap {
    static WGS: OnceLock<WaitGroupMap> = OnceLock::new();
    WGS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub fn get_mutexes() -> &'static MutexMap {
    static MUTEXES: OnceLock<MutexMap> = OnceLock::new();
    MUTEXES.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub fn get_contexts() -> &'static ContextMap {
    static CONTEXTS: OnceLock<ContextMap> = OnceLock::new();
    CONTEXTS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub fn get_dirty_goroutine_vars() -> &'static SharedVarMap {
    static DIRTY_VARS: OnceLock<SharedVarMap> = OnceLock::new();
    DIRTY_VARS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueSerial {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<ValueSerial>),
    Map(Vec<(String, ValueSerial)>),
    Function {
        name: Option<String>,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
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
                ValueSerial::Map(
                    map.iter()
                        .map(|(k, val)| (k.clone(), ValueSerial::from_value(val)))
                        .collect(),
                )
            }
            Value::Function(f) => ValueSerial::Function {
                name: f.name.clone(),
                params: f.params.clone(),
                body: f.body.clone(),
            },
            other => ValueSerial::String(other.to_string_repr()),
        }
    }

    pub fn to_value(&self) -> Value {
        self.to_value_with_env(None)
    }

    pub fn to_value_with_env(&self, env_opt: Option<&Rc<RefCell<Environment>>>) -> Value {
        match self {
            ValueSerial::Nil => Value::Nil,
            ValueSerial::Bool(b) => Value::Bool(*b),
            ValueSerial::Number(n) => Value::Number(*n),
            ValueSerial::String(s) => Value::String(s.clone()),
            ValueSerial::Array(arr) => {
                let items: Vec<Value> = arr
                    .iter()
                    .map(|item| item.to_value_with_env(env_opt))
                    .collect();
                Value::Array(Rc::new(RefCell::new(items)))
            }
            ValueSerial::Map(pairs) => {
                let mut map = HashMap::new();
                for (k, v) in pairs {
                    map.insert(k.clone(), v.to_value_with_env(env_opt));
                }
                Value::Map(Rc::new(RefCell::new(map)))
            }
            ValueSerial::Function { name, params, body } => {
                let closure = env_opt
                    .cloned()
                    .unwrap_or_else(|| Rc::new(RefCell::new(Environment::new())));
                Value::Function(Rc::new(WidyaFunction {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure,
                }))
            }
        }
    }
}

pub fn sync_dirty_vars_to_env(env: &Rc<RefCell<Environment>>) {
    if let Ok(dirty) = get_dirty_goroutine_vars().lock() {
        if dirty.is_empty() {
            return;
        }
        let dummy_span = Span::new(0, 0);
        let mut env_mut = env.borrow_mut();
        for (k, v_serial) in dirty.iter() {
            let val = v_serial.to_value();
            if env_mut.assign(k, val.clone(), &dummy_span).is_err() {
                env_mut.define(k.clone(), val, false);
            }
        }
    }
}

pub fn extract_id_from_arg(val: &Value, expected_type: &str, span: &Span) -> Result<u64, Galat> {
    match val {
        Value::Number(n) => Ok(*n as u64),
        Value::Map(m) => {
            let map = m.borrow();
            match map.get("id") {
                Some(Value::Number(n)) => Ok(*n as u64),
                _ => Err(Galat::runtime(
                    format!("Objek {} tidak valid (tidak memiliki id)", expected_type),
                    span,
                )),
            }
        }
        other => Err(Galat::runtime(
            format!(
                "Memerlukan objek {} atau ID angka, ditemukan '{}'",
                expected_type,
                other.type_name()
            ),
            span,
        )),
    }
}

pub fn get_channel_arc(ch_id: u64, span: &Span) -> Result<Arc<SharedChannel>, Galat> {
    let channels = get_channels()
        .lock()
        .map_err(|_| Galat::runtime("Kunci lock saluran bermasalah", span))?;
    channels
        .get(&ch_id)
        .cloned()
        .ok_or_else(|| Galat::runtime(format!("Saluran dengan ID {} tidak ditemukan", ch_id), span))
}

pub fn create_channel_object(capacity: Option<usize>) -> (u64, Value) {
    let ch_id = CHANNEL_NEXT_ID.fetch_add(1, Ordering::SeqCst);
    let shared = Arc::new(SharedChannel::new(ch_id, capacity));
    if let Ok(mut channels) = get_channels().lock() {
        channels.insert(ch_id, shared);
    }
    let mut result_map = HashMap::new();
    result_map.insert("_tipe".to_string(), Value::String("Saluran".to_string()));
    result_map.insert("id".to_string(), Value::Number(ch_id as f64));
    result_map.insert(
        "kapasitas".to_string(),
        Value::Number(capacity.unwrap_or(0) as f64),
    );
    (ch_id, Value::Map(Rc::new(RefCell::new(result_map))))
}

pub fn builtin_buat_utas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Function(f) => {
            let body = f.body.clone();
            // Snapshot non-builtin variables and user functions from the lexical closure chain
            let flat_env = f.closure.borrow().flatten_chain();
            let mut captured_env: Vec<(String, ValueSerial)> = Vec::new();
            for (k, v) in flat_env {
                if !matches!(v, Value::Builtin(_)) {
                    captured_env.push((k, ValueSerial::from_value(&v)));
                }
            }

            let thread_id = THREAD_NEXT_ID.fetch_add(1, Ordering::SeqCst);
            let handle = thread::spawn(move || {
                let mut interp = crate::interpreter::Interpreter::new();
                let mut initial_snapshot: HashMap<String, ValueSerial> = HashMap::new();
                // Restore captured environment into the goroutine's interpreter
                for (k, v_ser) in &captured_env {
                    let restored = v_ser.to_value_with_env(Some(&interp.environment));
                    interp.environment.borrow_mut().define(k.clone(), restored, false);
                    initial_snapshot.insert(k.clone(), v_ser.clone());
                }
                interp.goroutine_initial_snapshot = Some(initial_snapshot);

                let res = interp.execute_goroutine_body(&body);
                interp.flush_goroutine_dirty_vars();
                match res {
                    Ok(v) => Ok(ValueSerial::from_value(&v)),
                    Err(e) => Err(e.to_string()),
                }
            });

            if let Ok(mut handles) = get_threads().lock() {
                handles.insert(thread_id, handle);
            }

            let mut res = HashMap::new();
            res.insert("_tipe".to_string(), Value::String("Utas".to_string()));
            res.insert("id".to_string(), Value::Number(thread_id as f64));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        other => Err(Galat::runtime(
            format!("'buat_utas' / 'jalan' memerlukan fungsi, ditemukan '{}'", other.type_name()),
            span,
        )),
    }
}

pub fn builtin_gabung_utas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let thread_id = extract_id_from_arg(&args[0], "Utas", span)?;

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
        Err(Galat::runtime(
            format!("Utas dengan ID {} tidak ditemukan atau sudah digabung", thread_id),
            span,
        ))
    }
}

pub fn builtin_saluran(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let capacity = match args.first() {
        Some(Value::Number(n)) if *n > 0.0 => Some(*n as usize),
        _ => None,
    };
    let (_, ch_val) = create_channel_object(capacity);
    Ok(ch_val)
}

pub fn builtin_saluran_kirim(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("saluran_kirim memerlukan 2 argumen (saluran, nilai)", span));
    }
    let ch_id = extract_id_from_arg(&args[0], "Saluran", span)?;
    let ch = get_channel_arc(ch_id, span)?;
    let val_serial = ValueSerial::from_value(&args[1]);
    ch.send(val_serial).map_err(|e| Galat::runtime(e, span))?;
    Ok(Value::Bool(true))
}

pub fn builtin_saluran_terima(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("saluran_terima memerlukan 1 argumen (saluran)", span));
    }
    let ch_id = extract_id_from_arg(&args[0], "Saluran", span)?;
    let ch = get_channel_arc(ch_id, span)?;
    let (val_opt, _ok) = ch.recv_blocking();
    Ok(val_opt.map(|v| v.to_value()).unwrap_or(Value::Nil))
}

pub fn builtin_saluran_terima_ok(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("saluran_terima_ok memerlukan 1 argumen (saluran)", span));
    }
    let ch_id = extract_id_from_arg(&args[0], "Saluran", span)?;
    let ch = get_channel_arc(ch_id, span)?;
    let (val_opt, ok) = ch.recv_blocking();
    let val = val_opt.map(|v| v.to_value()).unwrap_or(Value::Nil);
    Ok(Value::Array(Rc::new(RefCell::new(vec![
        val,
        Value::Bool(ok),
    ]))))
}

pub fn builtin_tutup_saluran(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.is_empty() {
        return Err(Galat::runtime("tutup_saluran memerlukan 1 argumen (saluran)", span));
    }
    let ch_id = extract_id_from_arg(&args[0], "Saluran", span)?;
    let ch = get_channel_arc(ch_id, span)?;
    ch.close();
    Ok(Value::Nil)
}

pub fn builtin_grup_tunggu(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let wg_id = WAITGROUP_NEXT_ID.fetch_add(1, Ordering::SeqCst);
    if let Ok(mut wgs) = get_waitgroups().lock() {
        wgs.insert(wg_id, Arc::new(SharedWaitGroup::new()));
    }
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("GrupTunggu".to_string()));
    map.insert("id".to_string(), Value::Number(wg_id as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_wg_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let wg_id = extract_id_from_arg(&args[0], "GrupTunggu", span)?;
    let delta = match args.get(1) {
        Some(Value::Number(n)) => *n as isize,
        _ => 1,
    };
    let wg = get_waitgroups()
        .lock()
        .ok()
        .and_then(|w| w.get(&wg_id).cloned())
        .ok_or_else(|| Galat::runtime("GrupTunggu tidak ditemukan", span))?;
    wg.add(delta);
    Ok(Value::Nil)
}

pub fn builtin_wg_selesai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let wg_id = extract_id_from_arg(&args[0], "GrupTunggu", span)?;
    let wg = get_waitgroups()
        .lock()
        .ok()
        .and_then(|w| w.get(&wg_id).cloned())
        .ok_or_else(|| Galat::runtime("GrupTunggu tidak ditemukan", span))?;
    wg.add(-1);
    Ok(Value::Nil)
}

pub fn builtin_wg_tunggu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let wg_id = extract_id_from_arg(&args[0], "GrupTunggu", span)?;
    let wg = get_waitgroups()
        .lock()
        .ok()
        .and_then(|w| w.get(&wg_id).cloned())
        .ok_or_else(|| Galat::runtime("GrupTunggu tidak ditemukan", span))?;
    wg.wait();
    Ok(Value::Nil)
}

pub fn builtin_kunci_mutex(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mu_id = MUTEX_NEXT_ID.fetch_add(1, Ordering::SeqCst);
    if let Ok(mut mus) = get_mutexes().lock() {
        mus.insert(mu_id, Arc::new(SharedMutex::new()));
    }
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KunciMutex".to_string()));
    map.insert("id".to_string(), Value::Number(mu_id as f64));
    if let Some(init_val) = args.first() {
        map.insert("_punya_nilai".to_string(), Value::Bool(true));
        map.insert("nilai".to_string(), init_val.clone());
    }
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

pub fn builtin_mutex_kunci(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mu_id = extract_id_from_arg(&args[0], "KunciMutex", span)?;
    let mu = get_mutexes()
        .lock()
        .ok()
        .and_then(|m| m.get(&mu_id).cloned())
        .ok_or_else(|| Galat::runtime("KunciMutex tidak ditemukan", span))?;
    mu.lock();
    Ok(Value::Nil)
}

pub fn builtin_mutex_buka(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mu_id = extract_id_from_arg(&args[0], "KunciMutex", span)?;
    let mu = get_mutexes()
        .lock()
        .ok()
        .and_then(|m| m.get(&mu_id).cloned())
        .ok_or_else(|| Galat::runtime("KunciMutex tidak ditemukan", span))?;
    mu.unlock();
    Ok(Value::Nil)
}

// --- Go `context.Context` (`Konteks`) Implementation ---

fn make_context_map(ctx_id: u64, done_ch_val: Value) -> Value {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("Konteks".to_string()));
    map.insert("id".to_string(), Value::Number(ctx_id as f64));
    map.insert("_saluran_selesai".to_string(), done_ch_val);
    Value::Map(Rc::new(RefCell::new(map)))
}

pub fn cancel_context_by_id(ctx_id: u64, reason: &str) {
    let ctx_arc_opt = get_contexts()
        .lock()
        .ok()
        .and_then(|c| c.get(&ctx_id).cloned());
    if let Some(ctx_arc) = ctx_arc_opt {
        let (children_to_cancel, done_ch_id) = {
            let mut inner = match ctx_arc.inner.lock() {
                Ok(g) => g,
                Err(_) => return,
            };
            if inner.cancelled {
                return;
            }
            inner.cancelled = true;
            inner.err_msg = Some(reason.to_string());
            (inner.children.clone(), ctx_arc.done_channel_id)
        };
        if let Ok(channels) = get_channels().lock() {
            if let Some(ch) = channels.get(&done_ch_id).cloned() {
                drop(channels);
                let _ = ch.send(ValueSerial::String(reason.to_string()));
                ch.close();
            }
        }
        for child_id in children_to_cancel {
            cancel_context_by_id(child_id, reason);
        }
    }
}

fn spawn_child_context(
    parent_id_opt: Option<u64>,
    extra_values: HashMap<String, ValueSerial>,
) -> (u64, Value) {
    let ctx_id = CONTEXT_NEXT_ID.fetch_add(1, Ordering::SeqCst);
    let (done_ch_id, done_ch_val) = create_channel_object(Some(1));

    let mut inherited_values = HashMap::new();
    let mut parent_cancelled = None;

    if let Some(pid) = parent_id_opt {
        if let Ok(contexts) = get_contexts().lock() {
            if let Some(p_arc) = contexts.get(&pid).cloned() {
                drop(contexts);
                if let Ok(mut p_inner) = p_arc.inner.lock() {
                    for (k, v) in &p_inner.values {
                        inherited_values.insert(k.clone(), v.clone());
                    }
                    if p_inner.cancelled {
                        parent_cancelled = Some(
                            p_inner
                                .err_msg
                                .clone()
                                .unwrap_or_else(|| "context canceled".to_string()),
                        );
                    } else {
                        p_inner.children.push(ctx_id);
                    }
                }
            }
        }
    }

    for (k, v) in extra_values {
        inherited_values.insert(k, v);
    }

    let shared_ctx = Arc::new(SharedContext {
        id: ctx_id,
        parent_id: parent_id_opt,
        done_channel_id: done_ch_id,
        inner: Mutex::new(ContextInner {
            cancelled: false,
            err_msg: None,
            values: inherited_values,
            children: Vec::new(),
        }),
    });

    if let Ok(mut contexts) = get_contexts().lock() {
        contexts.insert(ctx_id, shared_ctx);
    }

    if let Some(reason) = parent_cancelled {
        cancel_context_by_id(ctx_id, &reason);
    }

    (ctx_id, make_context_map(ctx_id, done_ch_val))
}

pub fn builtin_konteks_latar(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let (_, ctx_val) = spawn_child_context(None, HashMap::new());
    Ok(ctx_val)
}

pub fn builtin_konteks_dengan_batal(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let parent_id = args
        .first()
        .map(|v| extract_id_from_arg(v, "Konteks", span))
        .transpose()?;
    let (ctx_id, ctx_val) = spawn_child_context(parent_id, HashMap::new());

    let cancel_fn = Value::Builtin(Rc::new(BuiltinFunction {
        name: format!("__batal_konteks::{}", ctx_id),
        arity: Some(0),
        func: |_a, _s| Ok(Value::Nil),
    }));

    Ok(Value::Array(Rc::new(RefCell::new(vec![ctx_val, cancel_fn]))))
}

pub fn builtin_konteks_dengan_batas_waktu(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime(
            "konteks_dengan_batas_waktu memerlukan 2 argumen (induk, milidetik)",
            span,
        ));
    }
    let parent_id = extract_id_from_arg(&args[0], "Konteks", span)?;
    let timeout_ms = match &args[1] {
        Value::Number(n) => (*n).max(0.0) as u64,
        _ => 100,
    };
    let (ctx_id, ctx_val) = spawn_child_context(Some(parent_id), HashMap::new());

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(timeout_ms));
        cancel_context_by_id(ctx_id, "context deadline exceeded");
    });

    let cancel_fn = Value::Builtin(Rc::new(BuiltinFunction {
        name: format!("__batal_konteks::{}", ctx_id),
        arity: Some(0),
        func: |_a, _s| Ok(Value::Nil),
    }));

    Ok(Value::Array(Rc::new(RefCell::new(vec![ctx_val, cancel_fn]))))
}

pub fn builtin_konteks_dengan_nilai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 3 {
        return Err(Galat::runtime(
            "konteks_dengan_nilai memerlukan 3 argumen (induk, kunci, nilai)",
            span,
        ));
    }
    let parent_id = extract_id_from_arg(&args[0], "Konteks", span)?;
    let key = args[1].to_string_repr();
    let val = ValueSerial::from_value(&args[2]);
    let mut map = HashMap::new();
    map.insert(key, val);
    let (_, ctx_val) = spawn_child_context(Some(parent_id), map);
    Ok(ctx_val)
}

pub fn context_is_cancelled(ctx_id: u64) -> bool {
    get_contexts()
        .lock()
        .ok()
        .and_then(|c| c.get(&ctx_id).cloned())
        .and_then(|arc| arc.inner.lock().ok().map(|i| i.cancelled))
        .unwrap_or(false)
}

pub fn context_err_value(ctx_id: u64) -> Value {
    get_contexts()
        .lock()
        .ok()
        .and_then(|c| c.get(&ctx_id).cloned())
        .and_then(|arc| {
            arc.inner
                .lock()
                .ok()
                .and_then(|i| i.err_msg.clone().map(Value::String))
        })
        .unwrap_or(Value::Nil)
}

pub fn context_get_value(ctx_id: u64, key: &str) -> Value {
    get_contexts()
        .lock()
        .ok()
        .and_then(|c| c.get(&ctx_id).cloned())
        .and_then(|arc| {
            arc.inner
                .lock()
                .ok()
                .and_then(|i| i.values.get(key).map(|v| v.to_value()))
        })
        .unwrap_or(Value::Nil)
}

pub fn register_concurrency(env: &mut Environment) {
    register_fn(env, "buat_utas", Some(1), builtin_buat_utas);
    register_fn(env, "_jalan_goroutine", Some(1), builtin_buat_utas);
    register_fn(env, "gabung_utas", Some(1), builtin_gabung_utas);

    for name in &["saluran", "buat_saluran", "chan"] {
        register_fn(env, name, None, builtin_saluran);
    }
    register_fn(env, "saluran_kirim", Some(2), builtin_saluran_kirim);
    register_fn(env, "saluran_terima", Some(1), builtin_saluran_terima);
    register_fn(env, "saluran_terima_ok", Some(1), builtin_saluran_terima_ok);
    for name in &["tutup_saluran", "close"] {
        register_fn(env, name, Some(1), builtin_tutup_saluran);
    }

    for name in &["GrupTunggu", "WaitGroup"] {
        register_fn(env, name, None, builtin_grup_tunggu);
    }
    register_fn(env, "wg_tambah", None, builtin_wg_tambah);
    register_fn(env, "wg_selesai", Some(1), builtin_wg_selesai);
    register_fn(env, "wg_tunggu", Some(1), builtin_wg_tunggu);

    for name in &["KunciMutex", "Mutex"] {
        register_fn(env, name, None, builtin_kunci_mutex);
    }
    register_fn(env, "mutex_kunci", Some(1), builtin_mutex_kunci);
    register_fn(env, "mutex_buka", Some(1), builtin_mutex_buka);

    for name in &["konteks_latar", "context_background", "Konteks"] {
        register_fn(env, name, None, builtin_konteks_latar);
    }
    for name in &["konteks_dengan_batal", "context_with_cancel"] {
        register_fn(env, name, None, builtin_konteks_dengan_batal);
    }
    for name in &["konteks_dengan_batas_waktu", "context_with_timeout"] {
        register_fn(env, name, Some(2), builtin_konteks_dengan_batas_waktu);
    }
    for name in &["konteks_dengan_nilai", "context_with_value"] {
        register_fn(env, name, Some(3), builtin_konteks_dengan_nilai);
    }
}
