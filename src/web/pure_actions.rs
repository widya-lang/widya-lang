//! Zero-API Server Actions & Form RPC Engine (Zero External Dependencies)
//! Standar Mutasi Modern: Pemanggilan fungsi backend langsung dari frontend,
//! Validasi Token Anti-CSRF, Form Progressive Enhancement, dan Optimistic Responses.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use serde_json::Value;

/// Definisi Aksi Server (Server Action Handler)
pub type AksiHandlerFn = Arc<dyn Fn(HashMap<String, String>) -> Result<Value, String> + Send + Sync>;

/// Registri Aksi Server Terdaftar
pub struct RegistriAksiServer {
    pub handlers: Arc<Mutex<HashMap<String, AksiHandlerFn>>>,
    pub csrf_tokens: Arc<Mutex<HashMap<String, u64>>>, // Token -> Expire Timestamp
}

impl Default for RegistriAksiServer {
    fn default() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            csrf_tokens: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

static GLOBAL_SERVER_ACTIONS: OnceLock<RegistriAksiServer> = OnceLock::new();

pub fn get_global_server_actions() -> &'static RegistriAksiServer {
    GLOBAL_SERVER_ACTIONS.get_or_init(RegistriAksiServer::default)
}

pub type ServerActionDispatcher = RegistriAksiServer;

impl RegistriAksiServer {
    pub fn baru() -> Self {
        Self::default()
    }

    /// Daftarkan Aksi Server
    pub fn daftarkan_aksi<F: Fn(HashMap<String, String>) -> Result<Value, String> + Send + Sync + 'static>(
        &self,
        id_aksi: &str,
        handler: F,
    ) {
        self.handlers.lock().unwrap().insert(id_aksi.to_string(), Arc::new(handler));
    }

    /// Daftarkan Aksi Server dengan return String
    pub fn daftarkan_action<F: Fn(HashMap<String, String>) -> Result<String, String> + Send + Sync + 'static>(
        &self,
        id_aksi: &str,
        handler: F,
    ) {
        self.handlers.lock().unwrap().insert(
            id_aksi.to_string(),
            Arc::new(move |args| handler(args).map(Value::String)),
        );
    }

    /// Buat Token Anti-CSRF baru
    pub fn buat_token_csrf(&self) -> String {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let token = format!("csrf_{:x}_{}", now, fastrand());
        self.csrf_tokens.lock().unwrap().insert(token.clone(), now + 3600); // Valid 1 jam
        token
    }

    /// Buat Token Anti-CSRF untuk session
    pub fn buat_csrf_token(&self, session_id: &str) -> String {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let token = format!("csrf_{}_{:x}_{}", session_id, now, fastrand());
        self.csrf_tokens.lock().unwrap().insert(token.clone(), now + 3600);
        token
    }

    /// Validasi Token Anti-CSRF
    pub fn validasi_token_csrf(&self, token: &str) -> bool {
        let tokens = self.csrf_tokens.lock().unwrap();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        if let Some(&expire) = tokens.get(token) {
            if now <= expire {
                return true;
            }
        }
        false
    }

    /// Validasi Token Anti-CSRF untuk session
    pub fn validasi_csrf_token(&self, session_id: &str, token: &str) -> bool {
        if !token.starts_with(&format!("csrf_{}_", session_id)) {
            return false;
        }
        self.validasi_token_csrf(token)
    }

    /// Eksekusi Aksi Server
    pub fn eksekusi(
        &self,
        id_aksi: &str,
        data_form: HashMap<String, String>,
        csrf_token: Option<&str>,
    ) -> Result<Value, String> {
        // Validasi CSRF
        if let Some(token) = csrf_token {
            if !self.validasi_token_csrf(token) {
                return Err("Token Keamanan CSRF tidak valid atau telah kadaluwarsa".to_string());
            }
        }

        let map = self.handlers.lock().unwrap();
        if let Some(handler) = map.get(id_aksi) {
            handler(data_form)
        } else {
            Err(format!("Aksi Server '{}' tidak ditemukan", id_aksi))
        }
    }

    /// Eksekusi Aksi Server returning String
    pub fn eksekusi_action(&self, id_aksi: &str, data_form: &HashMap<String, String>) -> Result<String, String> {
        let map = self.handlers.lock().unwrap();
        if let Some(handler) = map.get(id_aksi) {
            match handler(data_form.clone()) {
                Ok(Value::String(s)) => Ok(s),
                Ok(other) => Ok(other.to_string()),
                Err(e) => Err(e),
            }
        } else {
            Err(format!("Aksi Server '{}' tidak ditemukan", id_aksi))
        }
    }
}


fn fastrand() -> u32 {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    (now & 0xFFFFFFFF) as u32
}
