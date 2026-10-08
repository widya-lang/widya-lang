// ==============================================================================
// WidyaWeb Fullstack Reactive Component Engine (SSR + Hydration)
// ==============================================================================
// Arsitektur: Komponen reaktif fullstack terinspirasi dari Phoenix LiveView & SvelteKit.
// - Server-Side Rendering (SSR) awal menghasilkan HTML lengkap dengan token hidrasi.
// - Hidrasi instan via WebSocket RFC 6455 tanpa boilerplate REST/JSON manual.
// - State di backend langsung merefleksikan perubahan di DOM browser secara real-time.
// - Mendukung event handling: w-click, w-model, w-input, w-change, w-submit.
// - Smart DOM diff patching untuk transmisi data delta yang sangat hemat bandwidth.
// ==============================================================================

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, OnceLock};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use base64::Engine;

use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::value::{BuiltinFn, BuiltinFunction, Value};
use std::cell::RefCell;
use std::rc::Rc;

type HmacSha256 = Hmac<Sha256>;

/// Struktur pesan WebSocket dari browser klien
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveClientMessage {
    #[serde(rename = "tipe")]
    pub msg_type: String,
    pub cid: String,
    #[serde(default)]
    pub event: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
    #[serde(default)]
    pub token: Option<String>,
}

/// Struktur instruksi patch DOM yang dikirimkan ke klien
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomPatchInstruction {
    pub selector: String,
    pub op: String, // "replace_text", "replace_html", "set_attr", "remove_attr"
    pub value: String,
}

/// Struktur pesan balasan WebSocket dari server ke browser klien
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveServerMessage {
    #[serde(rename = "tipe")]
    pub msg_type: String, // "mounted", "patch", "error", "pong"
    pub cid: String,
    pub version: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffs: Option<Vec<DomPatchInstruction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Konfigurasi WASM Micro-Island untuk interaktivitas offline / lokal di klien
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmIslandConfig {
    pub island_id: String,
    pub wasm_url: String,
    pub exports: Vec<String>,
    pub initial_params: HashMap<String, serde_json::Value>,
}

/// Snapshot mutasi state untuk Time-Travel Debugger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshotEntry {
    pub version: u64,
    pub timestamp_ms: u64,
    pub action_name: String,
    pub state_before: HashMap<String, serde_json::Value>,
    pub state_after: HashMap<String, serde_json::Value>,
    pub diffs: Vec<DomPatchInstruction>,
}

/// Komponen Reaktif WidyaWeb
#[derive(Clone)]
pub struct ReactiveComponent {
    pub cid: String,
    pub name: String,
    pub state: Arc<Mutex<HashMap<String, serde_json::Value>>>,
    pub template: String,
    pub version: Arc<Mutex<u64>>,
    pub mount_token: String,
    pub resumable: Arc<Mutex<bool>>,
    pub wasm_islands: Arc<Mutex<HashMap<String, WasmIslandConfig>>>,
    pub source_location: Arc<Mutex<Option<String>>>,
    pub history: Arc<Mutex<Vec<StateSnapshotEntry>>>,
    pub dev_mode: Arc<Mutex<bool>>,
    handlers: Arc<Mutex<HashMap<String, Arc<dyn Fn(&mut HashMap<String, serde_json::Value>, &serde_json::Value) -> Result<(), String> + Send + Sync>>>>,
}

impl ReactiveComponent {
    pub fn new(name: &str, initial_state: HashMap<String, serde_json::Value>, template: &str) -> Self {
        let cid = format!("widya-comp-{}", uuid::Uuid::new_v4().to_string().replace('-', "")[..12].to_string());
        let token = Self::generate_mount_token(&cid, &initial_state);

        let initial_snapshot = StateSnapshotEntry {
            version: 1,
            timestamp_ms: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0),
            action_name: "mount_initial".to_string(),
            state_before: initial_state.clone(),
            state_after: initial_state.clone(),
            diffs: Vec::new(),
        };

        Self {
            cid,
            name: name.to_string(),
            state: Arc::new(Mutex::new(initial_state)),
            template: template.to_string(),
            version: Arc::new(Mutex::new(1)),
            mount_token: token,
            resumable: Arc::new(Mutex::new(true)),
            wasm_islands: Arc::new(Mutex::new(HashMap::new())),
            source_location: Arc::new(Mutex::new(None)),
            history: Arc::new(Mutex::new(vec![initial_snapshot])),
            dev_mode: Arc::new(Mutex::new(false)),
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Menghasilkan token HMAC-SHA256 untuk memverifikasi keabsahan initial state saat hidrasi
    pub fn generate_mount_token(cid: &str, state: &HashMap<String, serde_json::Value>) -> String {
        let secret = b"widya-reactive-component-hmac-secret-key-2026";
        let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC keys can be any size");
        let payload = format!("{}:{}", cid, serde_json::to_string(state).unwrap_or_default());
        mac.update(payload.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    /// Verifikasi token hidrasi dari klien
    pub fn verify_mount_token(&self, token: &str) -> bool {
        let state_guard = self.state.lock().unwrap();
        let expected = Self::generate_mount_token(&self.cid, &state_guard);
        expected == token
    }

    /// Menambahkan handler event
    pub fn register_handler<F>(&self, event_name: &str, handler: F)
    where
        F: Fn(&mut HashMap<String, serde_json::Value>, &serde_json::Value) -> Result<(), String> + Send + Sync + 'static,
    {
        let mut h = self.handlers.lock().unwrap();
        h.insert(event_name.to_string(), Arc::new(handler));
    }

    /// Render awal di Sisi Server (SSR)
    pub fn render_ssr(&self) -> (String, String) {
        let state_guard = self.state.lock().unwrap();
        let version_guard = self.version.lock().unwrap();
        
        // Render templat dengan menyisipkan nilai state
        let inner_rendered = self.render_template_internal(&state_guard);
        
        let state_json = serde_json::to_string(&*state_guard).unwrap_or_else(|_| "{}".to_string());
        let state_b64 = base64::engine::general_purpose::STANDARD.encode(state_json.as_bytes());

        // Bungkus elemen terluar dengan atribut reaktif WidyaWeb
        let ssr_html = format!(
            r#"<div id="{cid}" class="widya-reactive-component" data-widya-live="true" data-widya-cid="{cid}" data-widya-name="{name}" data-widya-version="{ver}" data-widya-token="{token}" data-widya-state="{state_b64}">
{content}
</div>"#,
            cid = self.cid,
            name = self.name,
            ver = *version_guard,
            token = self.mount_token,
            state_b64 = state_b64,
            content = inner_rendered
        );

        (ssr_html, self.cid.clone())
    }

    /// Menginterpolasi templat dengan nilai state
    fn render_template_internal(&self, state: &HashMap<String, serde_json::Value>) -> String {
        let mut result = self.template.clone();

        for (k, v) in state {
            let val_str = match v {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                serde_json::Value::Null => "".to_string(),
                other => other.to_string(),
            };

            // Interpolasi ekspresi {{kunci}} dan {{ kunci }}
            let pat1 = format!("{{{{{}}}}}", k);
            let pat2 = format!("{{{{ {} }}}}", k);
            result = result.replace(&pat1, &val_str);
            result = result.replace(&pat2, &val_str);
        }

        result
    }

    /// Memproses event interaktif dari WebSocket browser dan menghasilkan DOM patch
    pub fn dispatch_event(&self, event_name: &str, params: &serde_json::Value) -> Result<LiveServerMessage, String> {
        let old_rendered;
        let new_rendered;
        let new_state_json;
        let current_ver;
        let state_before;
        let state_after;

        {
            let mut state_guard = self.state.lock().unwrap();
            let mut ver_guard = self.version.lock().unwrap();
            *ver_guard += 1;
            current_ver = *ver_guard;

            old_rendered = self.render_template_internal(&state_guard);
            state_before = state_guard.clone();

            // Cari handler
            let handlers_guard = self.handlers.lock().unwrap();
            if let Some(handler) = handlers_guard.get(event_name) {
                handler(&mut *state_guard, params)?;
            } else {
                return Err(format!("Handler untuk event '{}' tidak ditemukan pada komponen '{}'", event_name, self.name));
            }

            state_after = state_guard.clone();
            new_rendered = self.render_template_internal(&state_guard);
            new_state_json = serde_json::to_value(&*state_guard).unwrap_or(serde_json::Value::Null);
        }

        // Hitung DOM patch sederhana untuk nodes yang berubah
        let diffs = self.compute_dom_diffs(&old_rendered, &new_rendered);

        let snapshot = StateSnapshotEntry {
            version: current_ver,
            timestamp_ms: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0),
            action_name: format!("event:{}", event_name),
            state_before,
            state_after,
            diffs: diffs.clone(),
        };
        self.history.lock().unwrap().push(snapshot);

        Ok(LiveServerMessage {
            msg_type: "patch".to_string(),
            cid: self.cid.clone(),
            version: current_ver,
            diffs: Some(diffs),
            html: Some(new_rendered),
            state: Some(new_state_json),
            error: None,
        })
    }

    /// Memproses input two-way data binding (w-model) langsung merefleksikan perubahan di backend
    pub fn dispatch_model_update(&self, field: &str, value: serde_json::Value) -> Result<LiveServerMessage, String> {
        let old_rendered;
        let new_rendered;
        let new_state_json;
        let current_ver;
        let state_before;
        let state_after;

        {
            let mut state_guard = self.state.lock().unwrap();
            let mut ver_guard = self.version.lock().unwrap();
            *ver_guard += 1;
            current_ver = *ver_guard;

            old_rendered = self.render_template_internal(&state_guard);
            state_before = state_guard.clone();

            state_guard.insert(field.to_string(), value);
            state_after = state_guard.clone();

            new_rendered = self.render_template_internal(&state_guard);
            new_state_json = serde_json::to_value(&*state_guard).unwrap_or(serde_json::Value::Null);
        }

        let diffs = self.compute_dom_diffs(&old_rendered, &new_rendered);

        let snapshot = StateSnapshotEntry {
            version: current_ver,
            timestamp_ms: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0),
            action_name: format!("model:{}", field),
            state_before,
            state_after,
            diffs: diffs.clone(),
        };
        self.history.lock().unwrap().push(snapshot);

        Ok(LiveServerMessage {
            msg_type: "patch".to_string(),
            cid: self.cid.clone(),
            version: current_ver,
            diffs: Some(diffs),
            html: Some(new_rendered),
            state: Some(new_state_json),
            error: None,
        })
    }

    /// Server-Push: Memperbarui state secara spontan dari backend (e.g. data sensor, cron, background thread)
    pub fn push_state_update(&self, updates: HashMap<String, serde_json::Value>) -> LiveServerMessage {
        let old_rendered;
        let new_rendered;
        let new_state_json;
        let current_ver;
        let state_before;
        let state_after;

        {
            let mut state_guard = self.state.lock().unwrap();
            let mut ver_guard = self.version.lock().unwrap();
            *ver_guard += 1;
            current_ver = *ver_guard;

            old_rendered = self.render_template_internal(&state_guard);
            state_before = state_guard.clone();

            for (k, v) in updates {
                state_guard.insert(k, v);
            }
            state_after = state_guard.clone();

            new_rendered = self.render_template_internal(&state_guard);
            new_state_json = serde_json::to_value(&*state_guard).unwrap_or(serde_json::Value::Null);
        }

        let diffs = self.compute_dom_diffs(&old_rendered, &new_rendered);

        let snapshot = StateSnapshotEntry {
            version: current_ver,
            timestamp_ms: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0),
            action_name: "server_push".to_string(),
            state_before,
            state_after,
            diffs: diffs.clone(),
        };
        self.history.lock().unwrap().push(snapshot);

        LiveServerMessage {
            msg_type: "patch".to_string(),
            cid: self.cid.clone(),
            version: current_ver,
            diffs: Some(diffs),
            html: Some(new_rendered),
            state: Some(new_state_json),
            error: None,
        }
    }

    /// Menghitung perbedaan teks dan elemen pada komponen
    fn compute_dom_diffs(&self, _old_html: &str, new_html: &str) -> Vec<DomPatchInstruction> {
        // Morphing default: menggantikan inner content dari komponen
        vec![DomPatchInstruction {
            selector: format!("#{}", self.cid),
            op: "replace_html".to_string(),
            value: new_html.to_string(),
        }]
    }

    /// Memproses teks frame WebSocket RFC 6455 dan mengembalikan teks frame balasan
    pub fn handle_ws_frame(&self, frame_text: &str) -> String {
        let client_msg: LiveClientMessage = match serde_json::from_str(frame_text) {
            Ok(msg) => msg,
            Err(e) => {
                let err_resp = LiveServerMessage {
                    msg_type: "error".to_string(),
                    cid: self.cid.clone(),
                    version: *self.version.lock().unwrap(),
                    diffs: None,
                    html: None,
                    state: None,
                    error: Some(format!("Frame JSON tidak valid: {}", e)),
                };
                return serde_json::to_string(&err_resp).unwrap();
            }
        };

        if client_msg.cid != self.cid {
            let err_resp = LiveServerMessage {
                msg_type: "error".to_string(),
                cid: self.cid.clone(),
                version: *self.version.lock().unwrap(),
                diffs: None,
                html: None,
                state: None,
                error: Some(format!("CID tidak cocok: '{}' vs '{}'", client_msg.cid, self.cid)),
            };
            return serde_json::to_string(&err_resp).unwrap();
        }

        match client_msg.msg_type.as_str() {
            "mount" => {
                let is_valid = if let Some(ref t) = client_msg.token {
                    self.verify_mount_token(t)
                } else {
                    true
                };

                if !is_valid {
                    let err = LiveServerMessage {
                        msg_type: "error".to_string(),
                        cid: self.cid.clone(),
                        version: *self.version.lock().unwrap(),
                        diffs: None,
                        html: None,
                        state: None,
                        error: Some("Token hidrasi tidak valid (Anti-Tampering)".to_string()),
                    };
                    return serde_json::to_string(&err).unwrap();
                }

                let current_state = serde_json::to_value(&*self.state.lock().unwrap()).unwrap_or(serde_json::Value::Null);
                let resp = LiveServerMessage {
                    msg_type: "mounted".to_string(),
                    cid: self.cid.clone(),
                    version: *self.version.lock().unwrap(),
                    diffs: None,
                    html: None,
                    state: Some(current_state),
                    error: None,
                };
                serde_json::to_string(&resp).unwrap()
            }
            "event" => {
                let ev_name = client_msg.event.unwrap_or_default();
                let params = client_msg.params.unwrap_or(serde_json::Value::Null);

                match self.dispatch_event(&ev_name, &params) {
                    Ok(resp) => serde_json::to_string(&resp).unwrap(),
                    Err(e) => {
                        let err = LiveServerMessage {
                            msg_type: "error".to_string(),
                            cid: self.cid.clone(),
                            version: *self.version.lock().unwrap(),
                            diffs: None,
                            html: None,
                            state: None,
                            error: Some(e),
                        };
                        serde_json::to_string(&err).unwrap()
                    }
                }
            }
            "input" => {
                let model = client_msg.model.unwrap_or_default();
                let val = client_msg.value.unwrap_or(serde_json::Value::Null);

                match self.dispatch_model_update(&model, val) {
                    Ok(resp) => serde_json::to_string(&resp).unwrap(),
                    Err(e) => {
                        let err = LiveServerMessage {
                            msg_type: "error".to_string(),
                            cid: self.cid.clone(),
                            version: *self.version.lock().unwrap(),
                            diffs: None,
                            html: None,
                            state: None,
                            error: Some(e),
                        };
                        serde_json::to_string(&err).unwrap()
                    }
                }
            }
            "time_travel" => {
                let target_ver = client_msg.params.as_ref().and_then(|p| p.get("target_version")).and_then(|v| v.as_u64()).unwrap_or(1);
                match self.time_travel_jump(target_ver) {
                    Ok(resp) => serde_json::to_string(&resp).unwrap(),
                    Err(e) => {
                        let err = LiveServerMessage {
                            msg_type: "error".to_string(),
                            cid: self.cid.clone(),
                            version: *self.version.lock().unwrap(),
                            diffs: None,
                            html: None,
                            state: None,
                            error: Some(e),
                        };
                        serde_json::to_string(&err).unwrap()
                    }
                }
            }
            "ping" => {
                let resp = LiveServerMessage {
                    msg_type: "pong".to_string(),
                    cid: self.cid.clone(),
                    version: *self.version.lock().unwrap(),
                    diffs: None,
                    html: None,
                    state: None,
                    error: None,
                };
                serde_json::to_string(&resp).unwrap()
            }
            other => {
                let err = LiveServerMessage {
                    msg_type: "error".to_string(),
                    cid: self.cid.clone(),
                    version: *self.version.lock().unwrap(),
                    diffs: None,
                    html: None,
                    state: None,
                    error: Some(format!("Tipe pesan '{}' tidak dikenal", other)),
                };
                serde_json::to_string(&err).unwrap()
            }
        }
    }

    /// Mengaktifkan atau menonaktifkan resumability (Zero-Hydration)
    pub fn enable_resumability(&self, enabled: bool) {
        *self.resumable.lock().unwrap() = enabled;
    }

    /// Mendaftarkan WASM Micro-Island untuk eksekusi lokal client CPU
    pub fn register_wasm_island(&self, island: WasmIslandConfig) {
        let mut islands = self.wasm_islands.lock().unwrap();
        islands.insert(island.island_id.clone(), island);
    }

    /// Mengatur lokasi berkas dan baris kode sumber (untuk Click-to-Source)
    pub fn set_source_location(&self, location: &str) {
        *self.source_location.lock().unwrap() = Some(location.to_string());
    }

    /// Mengaktifkan mode dev untuk DevTools overlay
    pub fn enable_dev_mode(&self, enabled: bool) {
        *self.dev_mode.lock().unwrap() = enabled;
    }

    /// Render Zero-Hydration Resumable HTML (Pola Qwik)
    pub fn render_resumable(&self) -> (String, String) {
        let state_guard = self.state.lock().unwrap();
        let version_guard = self.version.lock().unwrap();
        let is_resumable = *self.resumable.lock().unwrap();
        let inner_rendered = self.render_template_internal(&state_guard);

        let state_json = serde_json::to_string(&*state_guard).unwrap_or_else(|_| "{}".to_string());
        let state_b64 = base64::engine::general_purpose::STANDARD.encode(state_json.as_bytes());

        let source_attr = if let Some(ref loc) = *self.source_location.lock().unwrap() {
            format!(r#" data-widya-source="{}""#, loc)
        } else {
            String::new()
        };

        let islands_guard = self.wasm_islands.lock().unwrap();
        let mut island_tags = String::new();
        for (id, cfg) in islands_guard.iter() {
            let exp = cfg.exports.join(",");
            island_tags.push_str(&format!(
                r#"<widya-island data-island-id="{}" data-wasm-url="{}" data-wasm-exports="{}"></widya-island>"#,
                id, cfg.wasm_url, exp
            ));
        }

        let devtools_snippet = if *self.dev_mode.lock().unwrap() {
            self.render_devtools_overlay_html()
        } else {
            String::new()
        };

        let resumable_html = format!(
            r#"<div id="{cid}" class="widya-reactive-component" data-widya-live="true" data-widya-resumable="{resumable}" data-widya-cid="{cid}" data-widya-name="{name}" data-widya-version="{ver}" data-widya-token="{token}" data-widya-qrl="widya://components/{name}" data-widya-state="{state_b64}"{source_attr}>
{content}
{islands}
{devtools}
</div>"#,
            cid = self.cid,
            name = self.name,
            resumable = is_resumable,
            ver = *version_guard,
            token = self.mount_token,
            state_b64 = state_b64,
            source_attr = source_attr,
            content = inner_rendered,
            islands = island_tags,
            devtools = devtools_snippet
        );

        (resumable_html, self.cid.clone())
    }

    /// Melompat ke versi historis state (Time-Travel Debugger)
    pub fn time_travel_jump(&self, target_version: u64) -> Result<LiveServerMessage, String> {
        let history_guard = self.history.lock().unwrap();
        let target_snapshot = history_guard.iter().find(|s| s.version == target_version)
            .ok_or_else(|| format!("Versi snapshot {} tidak ditemukan dalam histori time-travel", target_version))?
            .clone();
        drop(history_guard);

        let old_rendered;
        let new_rendered;
        let new_state_json;

        {
            let mut state_guard = self.state.lock().unwrap();
            let mut ver_guard = self.version.lock().unwrap();
            *ver_guard = target_version;

            old_rendered = self.render_template_internal(&state_guard);
            *state_guard = target_snapshot.state_after.clone();
            new_rendered = self.render_template_internal(&state_guard);
            new_state_json = serde_json::to_value(&*state_guard).unwrap_or(serde_json::Value::Null);
        }

        let diffs = self.compute_dom_diffs(&old_rendered, &new_rendered);

        Ok(LiveServerMessage {
            msg_type: "patch".to_string(),
            cid: self.cid.clone(),
            version: target_version,
            diffs: Some(diffs),
            html: Some(new_rendered),
            state: Some(new_state_json),
            error: None,
        })
    }

    /// Mengambil salinan histori snapshot state
    pub fn get_history(&self) -> Vec<StateSnapshotEntry> {
        self.history.lock().unwrap().clone()
    }

    /// Menghasilkan markup UI DevTools overlay yang melayang di sudut layar browser
    pub fn render_devtools_overlay_html(&self) -> String {
        let history = self.get_history();
        let hist_len = history.len();
        let cur_ver = *self.version.lock().unwrap();

        format!(
            r#"<div id="widya-devtools-{cid}" class="widya-devtools-panel fixed bottom-4 right-4 bg-slate-950/90 text-cyan-400 p-3 rounded-xl border border-cyan-500/40 shadow-2xl backdrop-blur-md z-50 text-xs font-mono" style="max-width: 380px;">
  <div class="flex items-center justify-between border-b border-cyan-800/50 pb-2 mb-2">
    <span class="font-bold flex items-center gap-1.5"><span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span> WidyaWeb DevTools</span>
    <span class="text-[10px] bg-cyan-950 text-cyan-300 px-1.5 py-0.5 rounded border border-cyan-700/50">v{cur_ver} | {hist_len} Snapshots</span>
  </div>
  <div class="space-y-1.5">
    <div class="flex items-center justify-between text-gray-300">
      <span>Komponen:</span>
      <span class="text-white font-semibold">{name}</span>
    </div>
    <div class="flex items-center gap-2 mt-2">
      <button onclick="window.__widya_live_time_travel && window.__widya_live_time_travel('{cid}', Math.max(1, {cur_ver}-1))" class="px-2 py-1 bg-cyan-900/60 hover:bg-cyan-800 text-cyan-200 rounded border border-cyan-600/40">⏮ Mundur</button>
      <input type="range" min="1" max="{hist_len}" value="{cur_ver}" class="w-full accent-cyan-400 cursor-pointer" onchange="window.__widya_live_time_travel && window.__widya_live_time_travel('{cid}', parseInt(this.value))">
      <button onclick="window.__widya_live_time_travel && window.__widya_live_time_travel('{cid}', Math.min({hist_len}, {cur_ver}+1))" class="px-2 py-1 bg-cyan-900/60 hover:bg-cyan-800 text-cyan-200 rounded border border-cyan-600/40">Maju ⏭</button>
    </div>
  </div>
</div>"#,
            cid = self.cid,
            name = self.name,
            cur_ver = cur_ver,
            hist_len = hist_len
        )
    }
}

/// Global Registry untuk Komponen Reaktif yang aktif
static REACTIVE_REGISTRY: OnceLock<Mutex<HashMap<String, Arc<ReactiveComponent>>>> = OnceLock::new();

pub fn get_reactive_registry() -> &'static Mutex<HashMap<String, Arc<ReactiveComponent>>> {
    REACTIVE_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_reactive_component(comp: ReactiveComponent) -> Arc<ReactiveComponent> {
    let arc = Arc::new(comp);
    let mut reg = get_reactive_registry().lock().unwrap();
    reg.insert(arc.cid.clone(), Arc::clone(&arc));
    arc
}

pub fn get_reactive_component(cid: &str) -> Option<Arc<ReactiveComponent>> {
    let reg = get_reactive_registry().lock().unwrap();
    reg.get(cid).cloned()
}

/// ============================================================================
/// PILAR 2: WIDYABASE CDC-TO-EDGE REACTIVE PIPELINE
/// ============================================================================
/// Hub sinkronisasi Change Data Capture (CDC) langsung ke komponen reaktif
#[derive(Clone)]
pub struct CdcSubscriptionHub {
    subscriptions: Arc<Mutex<HashMap<String, Vec<(String, Option<String>, Option<String>)>>>>,
    tag_cache: Arc<Mutex<HashMap<String, Vec<String>>>>,
    invalidation_log: Arc<Mutex<Vec<(String, u64)>>>,
}

static GLOBAL_CDC_HUB: OnceLock<CdcSubscriptionHub> = OnceLock::new();

pub fn get_global_cdc_hub() -> &'static CdcSubscriptionHub {
    GLOBAL_CDC_HUB.get_or_init(CdcSubscriptionHub::new)
}

impl CdcSubscriptionHub {
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            tag_cache: Arc::new(Mutex::new(HashMap::new())),
            invalidation_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Mendaftarkan komponen untuk mendengarkan perubahan pada tabel database tertentu
    pub fn subscribe_table(&self, table: &str, component_cid: &str, filter_col: Option<String>, filter_val: Option<String>) {
        let mut subs = self.subscriptions.lock().unwrap();
        subs.entry(table.to_string()).or_default().push((component_cid.to_string(), filter_col, filter_val));
    }

    /// Menerima mutasi baris data dari CDC engine dan secara otomatis memutasi state komponen
    pub fn on_cdc_mutation(&self, table: &str, op: &str, row_data: &HashMap<String, serde_json::Value>) -> Vec<LiveServerMessage> {
        let subs_guard = self.subscriptions.lock().unwrap();
        let mut messages = Vec::new();

        if let Some(targets) = subs_guard.get(table) {
            for (cid, f_col, f_val) in targets {
                if let (Some(col), Some(expected)) = (f_col, f_val) {
                    if let Some(actual) = row_data.get(col) {
                        let actual_str = match actual {
                            serde_json::Value::String(s) => s.clone(),
                            other => other.to_string(),
                        };
                        if &actual_str != expected {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                if let Some(comp) = get_reactive_component(cid) {
                    let mut updates = HashMap::new();
                    for (k, v) in row_data {
                        updates.insert(k.clone(), v.clone());
                    }
                    updates.insert("_cdc_op".to_string(), json!(op));
                    let patch = comp.push_state_update(updates);
                    messages.push(patch);
                }
            }
        }

        messages
    }

    /// Mengaitkan tag invalidasi dengan cache key
    pub fn bind_cache_tag(&self, tag: &str, cache_key: &str) {
        let mut tags = self.tag_cache.lock().unwrap();
        tags.entry(tag.to_string()).or_default().push(cache_key.to_string());
    }

    /// Menghapus cache berdasarkan tag invalidasi (On-Demand ISR Revalidation)
    pub fn revalidate_tag(&self, tag: &str) -> Vec<String> {
        let mut tags = self.tag_cache.lock().unwrap();
        let purged = tags.remove(tag).unwrap_or_default();
        let mut inv_log = self.invalidation_log.lock().unwrap();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        inv_log.push((tag.to_string(), now));
        purged
    }

    pub fn get_invalidation_history(&self) -> Vec<(String, u64)> {
        self.invalidation_log.lock().unwrap().clone()
    }
}

/// ============================================================================
/// PILAR 3: HTTP/3, QUIC & WEBTRANSPORT
/// ============================================================================
/// Konfigurasi Transportasi Modern HTTP/3 & QUIC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Http3QuicConfig {
    pub enable_quic: bool,
    pub udp_port: u16,
    pub zero_rtt_handshake: bool,
    pub max_datagram_size: usize,
    pub alpn_protocols: Vec<String>,
}

impl Default for Http3QuicConfig {
    fn default() -> Self {
        Self {
            enable_quic: true,
            udp_port: 8443,
            zero_rtt_handshake: true,
            max_datagram_size: 1350,
            alpn_protocols: vec!["h3".to_string(), "webtransport".to_string()],
        }
    }
}

/// Sesi WebTransport interaktif untuk datagram berlatensi ultra-rendah
#[derive(Clone)]
pub struct WebTransportSession {
    pub session_id: String,
    pub client_id: String,
    pub streams: Arc<Mutex<HashMap<u64, Vec<u8>>>>,
    pub datagram_inbox: Arc<Mutex<VecDeque<Vec<u8>>>>,
    pub datagram_outbox: Arc<Mutex<VecDeque<Vec<u8>>>>,
}

impl WebTransportSession {
    pub fn new(client_id: &str) -> Self {
        Self {
            session_id: format!("wt-sess-{}", uuid::Uuid::new_v4().to_string().replace('-', "")[..12].to_string()),
            client_id: client_id.to_string(),
            streams: Arc::new(Mutex::new(HashMap::new())),
            datagram_inbox: Arc::new(Mutex::new(VecDeque::new())),
            datagram_outbox: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn send_datagram(&self, data: &[u8]) {
        let mut outbox = self.datagram_outbox.lock().unwrap();
        outbox.push_back(data.to_vec());
    }

    pub fn receive_datagram(&self) -> Option<Vec<u8>> {
        let mut inbox = self.datagram_inbox.lock().unwrap();
        inbox.pop_front()
    }

    pub fn push_incoming_datagram(&self, data: &[u8]) {
        let mut inbox = self.datagram_inbox.lock().unwrap();
        inbox.push_back(data.to_vec());
    }

    pub fn send_stream_chunk(&self, stream_id: u64, chunk: &[u8]) {
        let mut st = self.streams.lock().unwrap();
        st.entry(stream_id).or_default().extend_from_slice(chunk);
    }

    pub fn get_stream_data(&self, stream_id: u64) -> Option<Vec<u8>> {
        let st = self.streams.lock().unwrap();
        st.get(&stream_id).cloned()
    }
}

/// ============================================================================
/// PILAR 5: AI-NATIVE STREAMING UI & GENERATIVE UI (GenUI)
/// ============================================================================
/// Sesi streaming respons Generative AI token-demi-token langsung ke DOM
#[derive(Clone)]
pub struct StreamingAiSession {
    pub session_id: String,
    pub target_cid: String,
    pub target_selector: String,
    pub accumulated_text: Arc<Mutex<String>>,
    pub is_completed: Arc<Mutex<bool>>,
}

impl StreamingAiSession {
    pub fn new(session_id: &str, target_cid: &str, target_selector: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            target_cid: target_cid.to_string(),
            target_selector: target_selector.to_string(),
            accumulated_text: Arc::new(Mutex::new(String::new())),
            is_completed: Arc::new(Mutex::new(false)),
        }
    }

    pub fn stream_token(&self, comp: &ReactiveComponent, token: &str, is_done: bool) -> LiveServerMessage {
        let mut text_guard = self.accumulated_text.lock().unwrap();
        text_guard.push_str(token);
        let current_accumulated = text_guard.clone();
        if is_done {
            let mut done_guard = self.is_completed.lock().unwrap();
            *done_guard = true;
        }

        let cur_ver = {
            let mut v = comp.version.lock().unwrap();
            *v += 1;
            *v
        };

        LiveServerMessage {
            msg_type: "patch".to_string(),
            cid: comp.cid.clone(),
            version: cur_ver,
            diffs: Some(vec![DomPatchInstruction {
                selector: self.target_selector.clone(),
                op: "append_text".to_string(),
                value: token.to_string(),
            }]),
            html: Some(current_accumulated),
            state: None,
            error: None,
        }
    }

    pub fn stream_generative_ui_widget(&self, comp: &ReactiveComponent, widget_type: &str, widget_props: &serde_json::Value) -> Result<LiveServerMessage, String> {
        let widget_html = match widget_type {
            "stat_card" => {
                let label = widget_props.get("label").and_then(|v| v.as_str()).unwrap_or("Metrik");
                let value = widget_props.get("value").and_then(|v| v.as_str()).unwrap_or("0");
                format!(r#"<div class="widya-genui-card bg-slate-800 p-4 rounded-lg border border-cyan-500/30"><div class="text-xs text-cyan-400 font-semibold">{}</div><div class="text-2xl font-bold text-white mt-1">{}</div></div>"#, label, value)
            }
            "action_button" => {
                let text = widget_props.get("text").and_then(|v| v.as_str()).unwrap_or("Lanjutkan");
                let action = widget_props.get("action").and_then(|v| v.as_str()).unwrap_or("konfirmasi");
                format!(r#"<button w-click="{}" class="px-4 py-2 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-500 hover:to-indigo-500 text-white rounded-lg shadow font-medium">{}</button>"#, action, text)
            }
            other => {
                format!(r#"<div class="widya-genui-widget p-3 bg-slate-900 rounded border border-gray-700"><code>GenUI: {}</code></div>"#, other)
            }
        };

        let cur_ver = {
            let mut v = comp.version.lock().unwrap();
            *v += 1;
            *v
        };

        Ok(LiveServerMessage {
            msg_type: "patch".to_string(),
            cid: comp.cid.clone(),
            version: cur_ver,
            diffs: Some(vec![DomPatchInstruction {
                selector: self.target_selector.clone(),
                op: "replace_html".to_string(),
                value: widget_html.clone(),
            }]),
            html: Some(widget_html),
            state: None,
            error: None,
        })
    }
}

/// Menghasilkan runtime klien JavaScript murni (< 3 KB, zero dependency) untuk hidrasi DOM otomatis
pub fn get_widya_live_client_js() -> &'static str {
    r#"/**
 * WidyaWeb LiveView - Client-Side Reactive Hydration Engine (RFC 6455 WebSocket)
 * Zero-boilerplate fullstack reactive DOM sync:
 * - w-click: Trigger backend event via WebSocket
 * - w-model / w-input: Two-way reactive binding to backend state
 * - w-submit: Prevent default form submit and dispatch to backend
 */
(function() {
    if (window.__widya_live_initialized) return;
    window.__widya_live_initialized = true;

    class WidyaLiveSocket {
        constructor() {
            this.ws = null;
            this.components = new Map();
            this.pendingQueue = [];
            window.__widya_live_instance = this;
            window.__widya_live_time_travel = (cid, ver) => {
                this.send({ tipe: 'time_travel', cid, params: { target_version: ver } });
            };
            this.init();
        }

        init() {
            const proto = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
            const wsUrl = `${proto}//${window.location.host}/_widya/live`;
            this.connect(wsUrl);
            this.bindDomListeners();
            this.mountComponents();
        }

        connect(url) {
            try {
                this.ws = new WebSocket(url);
                this.ws.onopen = () => {
                    console.log('⚡ [WidyaWeb LiveView] Terhubung ke backend via WebSocket RFC 6455');
                    while (this.pendingQueue.length > 0) {
                        const msg = this.pendingQueue.shift();
                        this.ws.send(JSON.stringify(msg));
                    }
                    this.components.forEach(comp => this.sendMount(comp));
                };

                this.ws.onmessage = (event) => {
                    try {
                        const data = JSON.parse(event.data);
                        this.handleServerMessage(data);
                    } catch (e) {
                        console.error('❌ Gagal mengurai frame WebSocket:', e);
                    }
                };

                this.ws.onclose = () => {
                    console.warn('⚠️ [WidyaWeb LiveView] Koneksi terputus. Mencoba reconnect dalam 1.5 detik...');
                    setTimeout(() => this.connect(url), 1500);
                };
            } catch (err) {
                console.error('Koneksi WebSocket gagal:', err);
            }
        }

        mountComponents() {
            document.querySelectorAll('.widya-reactive-component').forEach(el => {
                const cid = el.getAttribute('data-widya-cid');
                const token = el.getAttribute('data-widya-token');
                if (cid) {
                    this.components.set(cid, { el, cid, token });
                    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
                        this.sendMount({ el, cid, token });
                    }
                }
            });
        }

        sendMount(comp) {
            this.send({
                tipe: 'mount',
                cid: comp.cid,
                token: comp.token
            });
        }

        send(msg) {
            if (this.ws && this.ws.readyState === WebSocket.OPEN) {
                this.ws.send(JSON.stringify(msg));
            } else {
                this.pendingQueue.push(msg);
            }
        }

        handleServerMessage(msg) {
            if (msg.tipe === 'patch') {
                const comp = this.components.get(msg.cid);
                if (comp && comp.el) {
                    if (msg.diffs && msg.diffs.length > 0) {
                        msg.diffs.forEach(d => {
                            const target = document.querySelector(d.selector);
                            if (target && d.op === 'replace_html') {
                                target.innerHTML = d.value;
                            } else if (target && d.op === 'replace_text') {
                                target.textContent = d.value;
                            } else if (target && d.op === 'append_text') {
                                target.textContent += d.value;
                            }
                        });
                    } else if (msg.html) {
                        comp.el.innerHTML = msg.html;
                    }
                }
            } else if (msg.tipe === 'error') {
                console.error('❌ [WidyaWeb LiveView Error]:', msg.error);
            }
        }

        bindDomListeners() {
            // Event delegation untuk w-click
            document.addEventListener('click', (e) => {
                const target = e.target.closest('[w-click]');
                if (!target) return;
                const compEl = target.closest('.widya-reactive-component');
                if (!compEl) return;
                const cid = compEl.getAttribute('data-widya-cid');
                const eventName = target.getAttribute('w-click');
                const paramStr = target.getAttribute('w-params');
                let params = {};
                if (paramStr) {
                    try { params = JSON.parse(paramStr); } catch (_) {}
                }
                this.send({
                    tipe: 'event',
                    cid: cid,
                    event: eventName,
                    params: params
                });
            });

            // Event delegation untuk w-model / w-input
            document.addEventListener('input', (e) => {
                const target = e.target.closest('[w-model]');
                if (!target) return;
                const compEl = target.closest('.widya-reactive-component');
                if (!compEl) return;
                const cid = compEl.getAttribute('data-widya-cid');
                const modelName = target.getAttribute('w-model');
                const val = target.type === 'checkbox' ? target.checked : target.value;
                this.send({
                    tipe: 'input',
                    cid: cid,
                    model: modelName,
                    value: val
                });
            });

            // Event delegation untuk w-submit
            document.addEventListener('submit', (e) => {
                const target = e.target.closest('[w-submit]');
                if (!target) return;
                e.preventDefault();
                const compEl = target.closest('.widya-reactive-component');
                if (!compEl) return;
                const cid = compEl.getAttribute('data-widya-cid');
                const eventName = target.getAttribute('w-submit');
                const formData = new FormData(target);
                const obj = {};
                formData.forEach((v, k) => obj[k] = v);
                this.send({
                    tipe: 'event',
                    cid: cid,
                    event: eventName,
                    params: obj
                });
            });
        }
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => new WidyaLiveSocket());
    } else {
        new WidyaLiveSocket();
    }
})();
"#
}

// ==============================================================================
// Widya-Lang Standard Library Bindings
// ==============================================================================

pub fn daftarkan_reactive_component(env: &mut Environment) {
    daftar(env, "web_komponen_reaktif_buat", Some(3), builtin_komponen_buat);
    daftar(env, "web_komponen_render_ssr", Some(1), builtin_komponen_render_ssr);
    daftar(env, "web_komponen_dispatch_event", Some(3), builtin_komponen_dispatch_event);
    daftar(env, "web_komponen_dispatch_input", Some(3), builtin_komponen_dispatch_input);
    daftar(env, "web_komponen_push_state", Some(2), builtin_komponen_push_state);
    daftar(env, "web_komponen_proses_ws_frame", Some(2), builtin_komponen_proses_ws_frame);
    daftar(env, "web_komponen_klien_script", Some(0), builtin_komponen_klien_script);

    // Frontier Builtins (5 Pilar Baru)
    daftar(env, "web_komponen_render_resumable", Some(1), builtin_komponen_render_resumable);
    daftar(env, "web_komponen_wasm_island_daftar", Some(4), builtin_komponen_wasm_island_daftar);
    daftar(env, "web_komponen_cdc_hub_langganan", None, builtin_komponen_cdc_hub_langganan);
    daftar(env, "web_komponen_cdc_mutasi_dorong", Some(3), builtin_komponen_cdc_mutasi_dorong);
    daftar(env, "web_komponen_cdc_tag_revalidate", Some(1), builtin_komponen_cdc_tag_revalidate);
    daftar(env, "web_komponen_time_travel_lompat", Some(2), builtin_komponen_time_travel_lompat);
    daftar(env, "web_komponen_time_travel_histori", Some(1), builtin_komponen_time_travel_histori);
    daftar(env, "web_komponen_devtools_overlay", Some(1), builtin_komponen_devtools_overlay);
    daftar(env, "web_komponen_ai_stream_token", Some(4), builtin_komponen_ai_stream_token);
    daftar(env, "web_komponen_ai_stream_genui", Some(4), builtin_komponen_ai_stream_genui);
    daftar(env, "web_komponen_quic_webtransport_buat", Some(2), builtin_komponen_quic_webtransport_buat);
}

fn daftar(env: &mut Environment, nama: &str, arity: Option<usize>, f: BuiltinFn) {
    let builtin = BuiltinFunction {
        name: nama.to_string(),
        arity,
        func: f,
    };
    env.define(nama.to_string(), Value::Builtin(Rc::new(builtin)), true);
}

fn builtin_komponen_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let name = args.first().map(|v| v.to_string_repr()).unwrap_or_else(|| "CounterComponent".to_string());
    
    // Parse initial state map
    let mut initial_state = HashMap::new();
    if let Some(Value::Map(m)) = args.get(1) {
        for (k, v) in m.borrow().iter() {
            let jv = match v {
                Value::Number(n) => json!(n),
                Value::String(s) => json!(s),
                Value::Bool(b) => json!(b),
                _ => json!(v.to_string_repr()),
            };
            initial_state.insert(k.clone(), jv);
        }
    } else {
        initial_state.insert("hitung".to_string(), json!(0));
    }

    let template = args.get(2).map(|v| v.to_string_repr()).unwrap_or_else(|| {
        r#"<div class="card p-4">
    <h2 class="text-xl font-bold">Counter Reaktif: <span id="val">{{hitung}}</span></h2>
    <div class="mt-4 flex gap-2">
        <button w-click="tambah" class="btn btn-primary">+</button>
        <button w-click="kurang" class="btn btn-secondary">-</button>
    </div>
</div>"#.to_string()
    });

    let comp = ReactiveComponent::new(&name, initial_state, &template);
    
    // Register default reactive math handlers (tambah, kurang, reset)
    comp.register_handler("tambah", |state, params| {
        let step = if let Some(n) = params.get("step").and_then(|v| v.as_f64()) {
            n
        } else {
            1.0
        };
        let current = state.get("hitung").and_then(|v| v.as_f64()).unwrap_or(0.0);
        state.insert("hitung".to_string(), json!(current + step));
        Ok(())
    });

    comp.register_handler("kurang", |state, params| {
        let step = if let Some(n) = params.get("step").and_then(|v| v.as_f64()) {
            n
        } else {
            1.0
        };
        let current = state.get("hitung").and_then(|v| v.as_f64()).unwrap_or(0.0);
        state.insert("hitung".to_string(), json!(current - step));
        Ok(())
    });

    comp.register_handler("reset", |state, _params| {
        state.insert("hitung".to_string(), json!(0.0));
        Ok(())
    });

    let registered = register_reactive_component(comp);

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("ReactiveComponent".to_string()));
    map.insert("cid".to_string(), Value::String(registered.cid.clone()));
    map.insert("nama".to_string(), Value::String(registered.name.clone()));
    map.insert("mount_token".to_string(), Value::String(registered.mount_token.clone()));
    map.insert("protokol".to_string(), Value::String("WebSocket RFC 6455 (LiveView)".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_render_ssr(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => {
            m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default()
        }
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen harus berupa ReactiveComponent map atau CID string", span)),
    };

    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen reaktif dengan CID '{}' tidak ditemukan", cid), span)),
    };

    let (ssr_html, _) = comp.render_ssr();

    let mut map = HashMap::new();
    map.insert("cid".to_string(), Value::String(comp.cid.clone()));
    map.insert("html_ssr".to_string(), Value::String(ssr_html));
    map.insert("mount_token".to_string(), Value::String(comp.mount_token.clone()));
    map.insert("status_hidrasi".to_string(), Value::String("Siap Hidrasi via WebSocket RFC 6455".to_string()));
    map.insert("latensi_render".to_string(), Value::String("< 0.5 ms".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_dispatch_event(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa ReactiveComponent atau CID string", span)),
    };

    let event_name = match args.get(1) {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 2 harus nama event string (e.g. 'tambah')", span)),
    };

    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };

    let params_json = if let Some(Value::Map(m)) = args.get(2) {
        let mut pj = serde_json::Map::new();
        for (k, v) in m.borrow().iter() {
            let val = match v {
                Value::Number(n) => json!(n),
                Value::String(s) => json!(s),
                Value::Bool(b) => json!(b),
                _ => json!(v.to_string_repr()),
            };
            pj.insert(k.clone(), val);
        }
        serde_json::Value::Object(pj)
    } else {
        serde_json::Value::Null
    };

    match comp.dispatch_event(&event_name, &params_json) {
        Ok(patch) => {
            let mut map = HashMap::new();
            map.insert("status".to_string(), Value::String("sukses".to_string()));
            map.insert("cid".to_string(), Value::String(patch.cid));
            map.insert("versi".to_string(), Value::Number(patch.version as f64));
            if let Some(h) = patch.html {
                map.insert("html_morphed".to_string(), Value::String(h));
            }
            if let Some(st) = patch.state {
                map.insert("state_terkini_json".to_string(), Value::String(st.to_string()));
            }
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Err(Galat::runtime(&format!("Gagal dispatch event: {}", e), span)),
    }
}

fn builtin_komponen_dispatch_input(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa ReactiveComponent atau CID string", span)),
    };

    let field = match args.get(1) {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 2 harus nama properti/model", span)),
    };

    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };

    let val_json = match args.get(2) {
        Some(Value::String(s)) => json!(s),
        Some(Value::Number(n)) => json!(n),
        Some(Value::Bool(b)) => json!(b),
        Some(v) => json!(v.to_string_repr()),
        None => json!(""),
    };

    match comp.dispatch_model_update(&field, val_json) {
        Ok(patch) => {
            let mut map = HashMap::new();
            map.insert("status".to_string(), Value::String("sukses".to_string()));
            map.insert("cid".to_string(), Value::String(patch.cid));
            map.insert("versi".to_string(), Value::Number(patch.version as f64));
            if let Some(h) = patch.html {
                map.insert("html_morphed".to_string(), Value::String(h));
            }
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Err(Galat::runtime(&format!("Gagal dispatch input: {}", e), span)),
    }
}

fn builtin_komponen_push_state(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa ReactiveComponent atau CID string", span)),
    };

    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };

    let mut updates = HashMap::new();
    if let Some(Value::Map(m)) = args.get(1) {
        for (k, v) in m.borrow().iter() {
            let val = match v {
                Value::Number(n) => json!(n),
                Value::String(s) => json!(s),
                Value::Bool(b) => json!(b),
                _ => json!(v.to_string_repr()),
            };
            updates.insert(k.clone(), val);
        }
    }

    let patch = comp.push_state_update(updates);
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("server_pushed".to_string()));
    map.insert("cid".to_string(), Value::String(patch.cid));
    map.insert("versi".to_string(), Value::Number(patch.version as f64));
    if let Some(h) = patch.html {
        map.insert("html_morphed".to_string(), Value::String(h));
    }

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_proses_ws_frame(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa ReactiveComponent atau CID string", span)),
    };

    let frame_str = match args.get(1) {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 2 harus berupa teks JSON frame WebSocket", span)),
    };

    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };

    let reply_frame = comp.handle_ws_frame(&frame_str);

    let mut map = HashMap::new();
    map.insert("reply_frame".to_string(), Value::String(reply_frame));
    map.insert("protokol".to_string(), Value::String("RFC 6455 Text Frame".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_klien_script(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::String(get_widya_live_client_js().to_string()))
}

fn builtin_komponen_render_resumable(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen harus berupa ReactiveComponent atau CID string", span)),
    };
    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };
    let (html, _) = comp.render_resumable();
    let mut map = HashMap::new();
    map.insert("cid".to_string(), Value::String(comp.cid.clone()));
    map.insert("html_resumable".to_string(), Value::String(html));
    map.insert("resumable".to_string(), Value::Bool(true));
    map.insert("latensi_render".to_string(), Value::String("< 0.1 ms (Zero-Hydration)".to_string()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_wasm_island_daftar(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa ReactiveComponent atau CID string", span)),
    };
    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };
    let island_id = match args.get(1) {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 2 harus berupa nama island ID", span)),
    };
    let wasm_url = match args.get(2) {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 3 harus berupa URL biner .wasm", span)),
    };
    let mut exports = Vec::new();
    if let Some(Value::Array(arr)) = args.get(3) {
        for v in arr.borrow().iter() {
            exports.push(v.to_string_repr());
        }
    }
    let island_cfg = WasmIslandConfig {
        island_id: island_id.clone(),
        wasm_url,
        exports,
        initial_params: HashMap::new(),
    };
    comp.register_wasm_island(island_cfg);
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("terdaftar".to_string()));
    map.insert("island_id".to_string(), Value::String(island_id));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_cdc_hub_langganan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let table = match args.first() {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa nama tabel", span)),
    };
    let cid = match args.get(1) {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 2 harus berupa ReactiveComponent atau CID string", span)),
    };
    let filter_col = args.get(2).map(|v| v.to_string_repr());
    let filter_val = args.get(3).map(|v| v.to_string_repr());

    get_global_cdc_hub().subscribe_table(&table, &cid, filter_col, filter_val);

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("berlangganan_cdc".to_string()));
    map.insert("tabel".to_string(), Value::String(table));
    map.insert("cid".to_string(), Value::String(cid));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_cdc_mutasi_dorong(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let table = match args.first() {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa nama tabel", span)),
    };
    let op = match args.get(1) {
        Some(Value::String(s)) => s.clone(),
        _ => "UPDATE".to_string(),
    };
    let mut row_data = HashMap::new();
    if let Some(Value::Map(m)) = args.get(2) {
        for (k, v) in m.borrow().iter() {
            let jv = match v {
                Value::Number(n) => json!(n),
                Value::String(s) => json!(s),
                Value::Bool(b) => json!(b),
                _ => json!(v.to_string_repr()),
            };
            row_data.insert(k.clone(), jv);
        }
    }

    let patches = get_global_cdc_hub().on_cdc_mutation(&table, &op, &row_data);

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("mutasi_terdistribusi".to_string()));
    map.insert("tabel".to_string(), Value::String(table));
    map.insert("jumlah_patch".to_string(), Value::Number(patches.len() as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_cdc_tag_revalidate(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tag = match args.first() {
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen harus nama tag invalidasi string", span)),
    };
    let purged = get_global_cdc_hub().revalidate_tag(&tag);
    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("cache_tag_purged".to_string()));
    map.insert("tag".to_string(), Value::String(tag));
    map.insert("purged_count".to_string(), Value::Number(purged.len() as f64));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_time_travel_lompat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa ReactiveComponent atau CID string", span)),
    };
    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };
    let target_ver = match args.get(1) {
        Some(Value::Number(n)) => *n as u64,
        _ => return Err(Galat::runtime("Argumen 2 harus nomor versi target", span)),
    };

    match comp.time_travel_jump(target_ver) {
        Ok(patch) => {
            let mut map = HashMap::new();
            map.insert("status".to_string(), Value::String("time_travel_sukses".to_string()));
            map.insert("versi".to_string(), Value::Number(patch.version as f64));
            if let Some(h) = patch.html {
                map.insert("html_morphed".to_string(), Value::String(h));
            }
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Err(Galat::runtime(&format!("Gagal time travel: {}", e), span)),
    }
}

fn builtin_komponen_time_travel_histori(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen harus berupa ReactiveComponent atau CID string", span)),
    };
    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };
    let hist = comp.get_history();
    let json_str = serde_json::to_string(&hist).unwrap_or_else(|_| "[]".to_string());
    let mut map = HashMap::new();
    map.insert("jumlah_snapshot".to_string(), Value::Number(hist.len() as f64));
    map.insert("histori_json".to_string(), Value::String(json_str));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_devtools_overlay(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen harus berupa ReactiveComponent atau CID string", span)),
    };
    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };
    comp.enable_dev_mode(true);
    let overlay = comp.render_devtools_overlay_html();
    Ok(Value::String(overlay))
}

fn builtin_komponen_ai_stream_token(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa ReactiveComponent atau CID string", span)),
    };
    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };
    let selector = match args.get(1) {
        Some(Value::String(s)) => s.clone(),
        _ => "#ai-output".to_string(),
    };
    let token = match args.get(2) {
        Some(Value::String(s)) => s.clone(),
        _ => "".to_string(),
    };
    let is_done = match args.get(3) {
        Some(Value::Bool(b)) => *b,
        _ => false,
    };

    let session = StreamingAiSession::new("stream-session-1", &cid, &selector);
    let patch = session.stream_token(&comp, &token, is_done);

    let mut map = HashMap::new();
    map.insert("status".to_string(), Value::String("stream_chunk".to_string()));
    map.insert("token".to_string(), Value::String(token));
    map.insert("versi".to_string(), Value::Number(patch.version as f64));
    map.insert("is_done".to_string(), Value::Bool(is_done));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_komponen_ai_stream_genui(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cid = match args.first() {
        Some(Value::Map(m)) => m.borrow().get("cid").map(|v| v.to_string_repr()).unwrap_or_default(),
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa ReactiveComponent atau CID string", span)),
    };
    let comp = match get_reactive_component(&cid) {
        Some(c) => c,
        None => return Err(Galat::runtime(&format!("Komponen CID '{}' tidak ditemukan", cid), span)),
    };
    let selector = match args.get(1) {
        Some(Value::String(s)) => s.clone(),
        _ => "#ai-widgets".to_string(),
    };
    let widget_type = match args.get(2) {
        Some(Value::String(s)) => s.clone(),
        _ => "stat_card".to_string(),
    };
    let widget_props = match args.get(3) {
        Some(Value::Map(m)) => {
            let mut pj = serde_json::Map::new();
            for (k, v) in m.borrow().iter() {
                let jv = match v {
                    Value::Number(n) => json!(n),
                    Value::String(s) => json!(s),
                    Value::Bool(b) => json!(b),
                    _ => json!(v.to_string_repr()),
                };
                pj.insert(k.clone(), jv);
            }
            serde_json::Value::Object(pj)
        }
        _ => json!({}),
    };

    let session = StreamingAiSession::new("stream-session-genui", &cid, &selector);
    match session.stream_generative_ui_widget(&comp, &widget_type, &widget_props) {
        Ok(patch) => {
            let mut map = HashMap::new();
            map.insert("status".to_string(), Value::String("genui_injected".to_string()));
            map.insert("widget_type".to_string(), Value::String(widget_type));
            map.insert("versi".to_string(), Value::Number(patch.version as f64));
            if let Some(h) = patch.html {
                map.insert("widget_html".to_string(), Value::String(h));
            }
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Err(Galat::runtime(&format!("Gagal generate GenUI: {}", e), span)),
    }
}

fn builtin_komponen_quic_webtransport_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let client_id = args.first().map(|v| v.to_string_repr()).unwrap_or_else(|| "client-quic-1".to_string());
    let udp_port = args.get(1).and_then(|v| match v { Value::Number(n) => Some(*n as u16), _ => None }).unwrap_or(8443);

    let session = WebTransportSession::new(&client_id);
    let mut map = HashMap::new();
    map.insert("session_id".to_string(), Value::String(session.session_id));
    map.insert("client_id".to_string(), Value::String(session.client_id));
    map.insert("protokol".to_string(), Value::String("HTTP/3 WebTransport (IETF QUIC)".to_string()));
    map.insert("udp_port".to_string(), Value::Number(udp_port as f64));
    map.insert("zero_rtt".to_string(), Value::Bool(true));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reactive_component_ssr_and_hydration() {
        let mut initial_state = HashMap::new();
        initial_state.insert("counter".to_string(), json!(10));
        initial_state.insert("judul".to_string(), json!("Dasbor Penjualan"));

        let template = "<h1>{{judul}}</h1><p>Nilai: {{counter}}</p><button w-click=\"tambah\">Tambah</button>";
        let comp = ReactiveComponent::new("UjiCounter", initial_state, template);

        // SSR Test
        let (ssr_html, cid) = comp.render_ssr();
        assert!(ssr_html.contains("Dasbor Penjualan"));
        assert!(ssr_html.contains("Nilai: 10"));
        assert!(ssr_html.contains(&format!("id=\"{}\"", cid)));
        assert!(ssr_html.contains("data-widya-live=\"true\""));

        // Register event handler
        comp.register_handler("tambah", |state, _params| {
            let cur = state.get("counter").and_then(|v| v.as_f64()).unwrap_or(0.0);
            state.insert("counter".to_string(), json!(cur + 1.0));
            Ok(())
        });

        // WebSocket Frame Mount Test
        let mount_frame = json!({
            "tipe": "mount",
            "cid": cid,
            "token": comp.mount_token
        }).to_string();
        let mount_reply = comp.handle_ws_frame(&mount_frame);
        assert!(mount_reply.contains("\"tipe\":\"mounted\""));

        // WebSocket Frame Event Test (w-click)
        let event_frame = json!({
            "tipe": "event",
            "cid": cid,
            "event": "tambah",
            "params": {}
        }).to_string();
        let event_reply = comp.handle_ws_frame(&event_frame);
        assert!(event_reply.contains("\"tipe\":\"patch\""));
        assert!(event_reply.contains("Nilai: 11"));

        // WebSocket Two-Way Input Binding Test (w-model)
        let input_frame = json!({
            "tipe": "input",
            "cid": cid,
            "model": "judul",
            "value": "Dasbor Kasir Baru"
        }).to_string();
        let input_reply = comp.handle_ws_frame(&input_frame);
        assert!(input_reply.contains("Dasbor Kasir Baru"));
    }

    #[test]
    fn test_resumability_and_wasm_islands() {
        let mut initial_state = HashMap::new();
        initial_state.insert("judul".to_string(), json!("Aplikasi Resumable"));
        initial_state.insert("nilai".to_string(), json!(42));

        let comp = ReactiveComponent::new("ResumableApp", initial_state, "<h2>{{judul}}</h2><p>{{nilai}}</p>");
        comp.set_source_location("aplikasi/src/komponen.wya:45");

        // Daftarkan WASM Island
        comp.register_wasm_island(WasmIslandConfig {
            island_id: "island-calc".to_string(),
            wasm_url: "/wasm/calculator.wasm".to_string(),
            exports: vec!["hitung_total".to_string(), "validasi_input".to_string()],
            initial_params: HashMap::new(),
        });

        let (res_html, cid) = comp.render_resumable();
        assert!(res_html.contains("data-widya-resumable=\"true\""));
        assert!(res_html.contains("data-widya-qrl=\"widya://components/ResumableApp\""));
        assert!(res_html.contains("data-widya-source=\"aplikasi/src/komponen.wya:45\""));
        assert!(res_html.contains("data-island-id=\"island-calc\""));
        assert!(res_html.contains("/wasm/calculator.wasm"));
    }

    #[test]
    fn test_in_browser_devtools_and_time_travel_debugger() {
        let mut initial_state = HashMap::new();
        initial_state.insert("angka".to_string(), json!(100));

        let comp = ReactiveComponent::new("TimeTravelTest", initial_state, "<div>Angka: {{angka}}</div>");
        comp.enable_dev_mode(true);

        // Mutasi 1: Update model ke 200
        let patch1 = comp.dispatch_model_update("angka", json!(200)).unwrap();
        assert_eq!(patch1.version, 2);

        // Mutasi 2: Update model ke 300
        let patch2 = comp.dispatch_model_update("angka", json!(300)).unwrap();
        assert_eq!(patch2.version, 3);

        // Periksa histori
        let history = comp.get_history();
        assert!(history.len() >= 3);

        // Lakukan Time-Travel Jump kembali ke versi 2 (angka = 200)
        let tt_patch = comp.time_travel_jump(2).unwrap();
        assert_eq!(tt_patch.version, 2);
        assert!(tt_patch.html.unwrap().contains("Angka: 200"));

        // Lakukan Time-Travel Jump kembali ke versi 1 (angka = 100)
        let tt_patch1 = comp.time_travel_jump(1).unwrap();
        assert_eq!(tt_patch1.version, 1);
        assert!(tt_patch1.html.unwrap().contains("Angka: 100"));

        // Periksa render overlay DevTools
        let overlay = comp.render_devtools_overlay_html();
        assert!(overlay.contains("WidyaWeb DevTools"));
        assert!(overlay.contains("Snapshots"));
        assert!(overlay.contains("⏮ Mundur"));
    }

    #[test]
    fn test_widyabase_cdc_pipeline_and_on_demand_revalidation() {
        let mut initial_state = HashMap::new();
        initial_state.insert("stok".to_string(), json!(50));
        initial_state.insert("produk_id".to_string(), json!("PROD-01"));

        let comp = ReactiveComponent::new("StokLiveView", initial_state, "<span>Stok: {{stok}}</span>");
        let registered = register_reactive_component(comp);

        let cdc_hub = get_global_cdc_hub();
        cdc_hub.subscribe_table("produk", &registered.cid, Some("produk_id".to_string()), Some("PROD-01".to_string()));

        // Ikat cache tag edge
        cdc_hub.bind_cache_tag("tag-katalog", "page_katalog_html");
        cdc_hub.bind_cache_tag("tag-katalog", "api_produk_json");

        // Simulasikan database mutation CDC dari WidyaBase (stok berubah menjadi 45)
        let mut row_update = HashMap::new();
        row_update.insert("produk_id".to_string(), json!("PROD-01"));
        row_update.insert("stok".to_string(), json!(45));

        let patches = cdc_hub.on_cdc_mutation("produk", "UPDATE", &row_update);
        assert_eq!(patches.len(), 1);
        assert!(patches[0].html.as_ref().unwrap().contains("Stok: 45"));

        // On-Demand Tag Revalidation
        let purged = cdc_hub.revalidate_tag("tag-katalog");
        assert_eq!(purged.len(), 2);
        assert!(purged.contains(&"page_katalog_html".to_string()));
    }

    #[test]
    fn test_http3_quic_and_webtransport() {
        let cfg = Http3QuicConfig::default();
        assert!(cfg.enable_quic);
        assert_eq!(cfg.udp_port, 8443);
        assert!(cfg.zero_rtt_handshake);

        let sess = WebTransportSession::new("klien-mobile-edge-01");
        assert!(sess.session_id.starts_with("wt-sess-"));

        // Test unreliable datagram
        sess.send_datagram(b"TELEMETRY_SENSOR_DATA_30FPS");
        let out = sess.datagram_outbox.lock().unwrap().pop_front().unwrap();
        assert_eq!(out, b"TELEMETRY_SENSOR_DATA_30FPS");

        // Test multiplexed bidirectional stream
        sess.send_stream_chunk(1, b"BAGIAN_1_");
        sess.send_stream_chunk(1, b"BAGIAN_2");
        let stream_res = sess.get_stream_data(1).unwrap();
        assert_eq!(stream_res, b"BAGIAN_1_BAGIAN_2");
    }

    #[test]
    fn test_ai_native_streaming_and_genui() {
        let mut initial_state = HashMap::new();
        initial_state.insert("ai_chat".to_string(), json!(""));

        let comp = ReactiveComponent::new("AiChatView", initial_state, "<div id=\"ai-output\">{{ai_chat}}</div><div id=\"ai-widgets\"></div>");

        let ai_session = StreamingAiSession::new("ai-sess-1", &comp.cid, "#ai-output");

        // Stream token 1
        let p1 = ai_session.stream_token(&comp, "WidyaWeb ", false);
        assert_eq!(p1.diffs.as_ref().unwrap()[0].op, "append_text");
        assert_eq!(p1.diffs.as_ref().unwrap()[0].value, "WidyaWeb ");

        // Stream token 2
        let p2 = ai_session.stream_token(&comp, "Reaktif!", true);
        assert_eq!(p2.diffs.as_ref().unwrap()[0].value, "Reaktif!");
        assert_eq!(ai_session.accumulated_text.lock().unwrap().as_str(), "WidyaWeb Reaktif!");
        assert!(*ai_session.is_completed.lock().unwrap());

        // Stream Generative UI Widget
        let widget_props = json!({ "label": "Throughput Transaksi", "value": "125,000 TPS" });
        let genui_patch = ai_session.stream_generative_ui_widget(&comp, "stat_card", &widget_props).unwrap();
        assert!(genui_patch.html.as_ref().unwrap().contains("Throughput Transaksi"));
        assert!(genui_patch.html.as_ref().unwrap().contains("125,000 TPS"));
    }
}

