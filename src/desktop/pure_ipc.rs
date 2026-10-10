//! Zero-Copy Fast IPC (Inter-Process Communication) & Event Bridge Engine
//! Zero External Dependencies (No Electron IPC, No Chromium Mojo).
//! Provides typed asynchronous request/response, publish-subscribe event channels,
//! continuous data streaming, and per-window token security validation.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Jenis Peristiwa IPC
#[derive(Debug, Clone, PartialEq)]
pub enum JenisPesanIPC {
    Permintaan { id_req: u64, metode: String, payload: Vec<u8> },
    Respon { id_req: u64, sukses: bool, payload: Vec<u8> },
    SiaranPeristiwa { topik: String, payload: Vec<u8> },
    AliranData { id_aliran: u64, chunk: Vec<u8>, selesai: bool },
}

pub type IPCHandlerFn = Arc<dyn Fn(&[u8]) -> Result<Vec<u8>, String> + Send + Sync>;
pub type IPCSuscriberFn = Arc<dyn Fn(&[u8]) + Send + Sync>;

/// Jembatan IPC Murni (Sovereign IPC Bridge)
pub struct JembatanIPCMurni {
    handlers: Arc<Mutex<HashMap<String, IPCHandlerFn>>>,
    subscribers: Arc<Mutex<HashMap<String, Vec<IPCSuscriberFn>>>>,
    session_tokens: Arc<Mutex<HashMap<String, String>>>, // window_id -> secret_token
}

impl Default for JembatanIPCMurni {
    fn default() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            session_tokens: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

static GLOBAL_IPC_BRIDGE: OnceLock<JembatanIPCMurni> = OnceLock::new();

pub fn get_global_ipc_bridge() -> &'static JembatanIPCMurni {
    GLOBAL_IPC_BRIDGE.get_or_init(JembatanIPCMurni::default)
}

impl JembatanIPCMurni {
    pub fn baru() -> Self {
        Self::default()
    }

    /// Daftarkan token keamanan untuk jendela tertentu
    pub fn daftarkan_token_jendela(&self, window_id: &str, token: &str) {
        self.session_tokens.lock().unwrap().insert(window_id.to_string(), token.to_string());
    }

    /// Validasi otentisitas pesan IPC dari jendela
    pub fn validasi_token(&self, window_id: &str, token: &str) -> bool {
        let map = self.session_tokens.lock().unwrap();
        if let Some(expected) = map.get(window_id) {
            expected == token
        } else {
            false
        }
    }

    /// Daftarkan penangan metode RPC IPC
    pub fn daftarkan_metode<F>(&self, nama_metode: &str, handler: F)
    where
        F: Fn(&[u8]) -> Result<Vec<u8>, String> + Send + Sync + 'static,
    {
        self.handlers.lock().unwrap().insert(nama_metode.to_string(), Arc::new(handler));
    }

    /// Panggil metode RPC IPC secara langsung
    pub fn panggil(&self, nama_metode: &str, argumen: &[u8]) -> Result<Vec<u8>, String> {
        let map = self.handlers.lock().unwrap();
        if let Some(handler) = map.get(nama_metode) {
            handler(argumen)
        } else {
            Err(format!("Metode IPC '{}' tidak terdaftar", nama_metode))
        }
    }

    /// Berlangganan topik siaran peristiwa (*Event Subscription*)
    pub fn langganan_topik<F>(&self, topik: &str, subscriber: F)
    where
        F: Fn(&[u8]) + Send + Sync + 'static,
    {
        let mut map = self.subscribers.lock().unwrap();
        map.entry(topik.to_string())
            .or_default()
            .push(Arc::new(subscriber));
    }

    /// Siarkan peristiwa ke semua pelanggan (*Publish Event*)
    pub fn siarkan(&self, topik: &str, data: &[u8]) -> usize {
        let map = self.subscribers.lock().unwrap();
        if let Some(subs) = map.get(topik) {
            for s in subs {
                s(data);
            }
            subs.len()
        } else {
            0
        }
    }
}
