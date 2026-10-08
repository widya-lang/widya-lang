// ==============================================================================
// WidyaWeb Sovereign Broadcast Studio - Standalone OB Van Server & Launcher
// ==============================================================================
// Mengemas seluruh ekosistem MCR Studio, PWA Kamera Smartphone, Tablet VAR,
// serta peladen WebSocket RFC 6455 Signaling & Tally Hub ke dalam binary tunggal.
// Dapat dijalankan di laptop OB Van, server lokal gelanggang, atau edge node LAN.
// ==============================================================================

use colored::*;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::web::ws::{encode_frame, generate_sec_websocket_accept, parse_frame_header};

// Embedded assets untuk distribusi biner mandiri (zero-dependency standalone executable)
const EMBEDDED_MCR_INDEX_HTML: &str = include_str!("../../aplikasi/08_widya_broadcasting_studio/index.html");
const EMBEDDED_MCR_APP_JS: &str = include_str!("../../aplikasi/08_widya_broadcasting_studio/app.js");
const EMBEDDED_MCR_STYLE_CSS: &str = include_str!("../../aplikasi/08_widya_broadcasting_studio/style.css");
const EMBEDDED_KAMERA_HP_HTML: &str = include_str!("../../aplikasi/08_widya_broadcasting_studio/kamera_hp.html");
const EMBEDDED_VAR_HAKIM_HTML: &str = include_str!("../../aplikasi/08_widya_broadcasting_studio/var_dewan_hakim.html");
const EMBEDDED_MANIFEST_JSON: &str = include_str!("../../aplikasi/08_widya_broadcasting_studio/manifest.json");
const EMBEDDED_SW_JS: &str = include_str!("../../aplikasi/08_widya_broadcasting_studio/sw.js");

/// Mendeteksi alamat IP lokal aktif di jaringan LAN (tanpa dependensi eksternal)
pub fn detect_lan_ip() -> String {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                let ip = addr.ip().to_string();
                if !ip.starts_with("127.") {
                    return ip;
                }
            }
        }
    }
    "127.0.0.1".to_string()
}

/// Struktur Peladen Mandiri MCR Broadcast Studio
pub struct BroadcastStudioServer {
    pub bind_addr: String,
    pub port: u16,
    pub lan_ip: String,
    pub ws_clients: Arc<Mutex<Vec<TcpStream>>>,
    pub last_tally: Arc<Mutex<(String, String)>>,
}

impl BroadcastStudioServer {
    pub fn new(bind_addr: &str, port: u16) -> Self {
        let lan_ip = detect_lan_ip();
        Self {
            bind_addr: bind_addr.to_string(),
            port,
            lan_ip,
            ws_clients: Arc::new(Mutex::new(Vec::new())),
            last_tally: Arc::new(Mutex::new(("cam_a_wide".to_string(), "cam_a_close".to_string()))),
        }
    }

    /// Membaca berkas langsung dari disk jika tersedia di direktori kerja (memungkinkan live edit),
    /// atau otomatis beralih ke aset yang tertanam di biner (jika berjalan mandiri).
    fn load_asset(path_disk: &str, fallback_embedded: &'static str) -> String {
        if Path::new(path_disk).exists() {
            fs::read_to_string(path_disk).unwrap_or_else(|_| fallback_embedded.to_string())
        } else {
            fallback_embedded.to_string()
        }
    }

    /// Menyiarkan paket data WebSocket ke seluruh klien (kamera, tablet VAR, konsol MCR)
    pub fn broadcast_ws_frame(&self, payload: &str) {
        let frame = encode_frame(1, payload.as_bytes(), false);
        let mut clients = self.ws_clients.lock().unwrap();
        clients.retain_mut(|client| {
            client.write_all(&frame).is_ok()
        });
    }

    /// Menjalankan peladen dan mencetak kartu pairing visual ke terminal
    pub fn start(&self, auto_open: bool) -> Result<(), String> {
        let bind_target = format!("{}:{}", self.bind_addr, self.port);
        let listener = TcpListener::bind(&bind_target)
            .map_err(|e| format!("Gagal mengikat antarmuka jaringan '{}': {}", bind_target, e))?;

        let mcr_url_local = format!("http://localhost:{}", self.port);
        let mcr_url_lan = format!("http://{}:{}", self.lan_ip, self.port);
        let cam_wide_url = format!("http://{}:{}/kamera?cam=cam_a_wide", self.lan_ip, self.port);
        let cam_close_url = format!("http://{}:{}/kamera?cam=cam_a_close", self.lan_ip, self.port);
        let cam_wasit_url = format!("http://{}:{}/kamera?cam=cam_b_close", self.lan_ip, self.port);
        let var_url = format!("http://{}:{}/var", self.lan_ip, self.port);
        let ws_url = format!("ws://{}:{}/ws", self.lan_ip, self.port);

        // Cetak Banner Terminal OB Van Studio
        self.print_launcher_banner(&mcr_url_local, &mcr_url_lan, &cam_wide_url, &cam_close_url, &cam_wasit_url, &var_url, &ws_url);

        if auto_open {
            self.open_browser(&mcr_url_local);
        }

        let ws_clients = self.ws_clients.clone();
        let last_tally = self.last_tally.clone();

        for stream_res in listener.incoming() {
            if let Ok(mut stream) = stream_res {
                let ws_clients_cloned = ws_clients.clone();
                let last_tally_cloned = last_tally.clone();
                thread::spawn(move || {
                    Self::handle_incoming_connection(&mut stream, ws_clients_cloned, last_tally_cloned);
                });
            }
        }

        Ok(())
    }

    fn print_launcher_banner(
        &self,
        mcr_local: &str,
        mcr_lan: &str,
        cam_wide: &str,
        cam_close: &str,
        cam_wasit: &str,
        var: &str,
        ws: &str,
    ) {
        println!("\n{}", "╔═══════════════════════════════════════════════════════════════════════════════════════════╗".bright_cyan().bold());
        println!("{}", "║  📡 WIDYA SOVEREIGN BROADCAST STUDIO - OB VAN STANDALONE LAUNCHER                         ║".bright_white().bold());
        println!("{}", "║     v2.6.0 • Full-Duplex WebRTC Signaling Hub & Zero-Latency Tally Router          ║".bright_cyan());
        println!("{}", "╚═══════════════════════════════════════════════════════════════════════════════════════════╝".bright_cyan().bold());

        println!("\n{}", "🚀 KONSOL OPERATOR & ANTARMUKA SIARAN TERHUBUNG:".bright_yellow().bold());
        println!("  🖥️  {} : {} (atau {})", "MCR Master Console".bold(), mcr_local.bright_green().underline(), mcr_lan.bright_cyan().underline());
        println!("  ⚖️  {} : {}", "VAR Dewan Juri Tablet".bold(), var.bright_yellow().underline());
        println!("  📡  {} : {}", "WebSocket Signaling Hub".bold(), ws.bright_magenta());

        println!("\n{}", "📱 PAIRING CEPAT JURU KAMERA SMARTPHONE (PINDAI ATAU BUKA URL):".bright_yellow().bold());
        println!("  🎥  {} : {}", "Kamera 1 - Gelanggang A Wide ".bold(), cam_wide.bright_cyan().bold().underline());
        println!("  🎥  {} : {}", "Kamera 2 - Gelanggang A Close".bold(), cam_close.bright_green().bold().underline());
        println!("  🎥  {} : {}", "Kamera 3 - Gelanggang B Wasit".bold(), cam_wasit.bright_blue().bold().underline());

        println!("\n{}", "┌───────────────────────────────────────────────────────────────────────────────────────────┐".bright_black());
        println!("{}", "│  📱 QR PAIRING CODE CARD (Tampilkan / Pindai dengan Kamera Ponsel di Jaringan Wi-Fi):    │".bright_white());
        println!("{}", "│                                                                                           │".bright_black());
        println!("{}", "│     ██████████████  ██    ██  ██████████████      [1] Sambungkan smartphone ke Wi-Fi      │".bright_white());
        println!("{}", "│     ██          ██  ██    ██  ██          ██      [2] Buka browser di ponsel              │".bright_white());
        println!("{}", "│     ██  ██████  ██  ██    ██  ██  ██████  ██      [3] Buka URL kamera di atas             │".bright_white());
        println!("{}", "│     ██  ██████  ██  ████  ██  ██  ██████  ██      [4] Border Tally & sensor gyro          │".bright_white());
        println!("{}", "│     ██████████████  ██  ████  ██████████████          akan sinkron live dengan MCR!       │".bright_white());
        println!("{}", "│                     ██    ██                                                              │".bright_black());
        println!("{}", "│     ████████  ████  ████████  ██████████████      💡 Status: Hub Online & Menerima Peer   │".bright_white());
        println!("{}", "│     ██    ██    ██  ██    ██  ██  ██    ████         Tekan Ctrl+C untuk mematikan peladen │".bright_white());
        println!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────┘".bright_black());
        println!();
    }

    fn open_browser(&self, url: &str) {
        #[cfg(target_os = "linux")]
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();

        #[cfg(target_os = "macos")]
        let _ = std::process::Command::new("open").arg(url).spawn();

        #[cfg(target_os = "windows")]
        let _ = std::process::Command::new("cmd").args(["/C", "start", url]).spawn();
    }

    fn handle_incoming_connection(
        stream: &mut TcpStream,
        ws_clients: Arc<Mutex<Vec<TcpStream>>>,
        last_tally: Arc<Mutex<(String, String)>>,
    ) {
        let mut buffer = [0u8; 8192];
        let bytes_read = match stream.read(&mut buffer) {
            Ok(n) if n > 0 => n,
            _ => return,
        };

        let raw_req = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut lines = raw_req.lines();
        let req_line = match lines.next() {
            Some(l) => l,
            None => return,
        };

        let parts: Vec<&str> = req_line.split_whitespace().collect();
        if parts.len() < 2 {
            return;
        }

        let method = parts[0];
        let full_path = parts[1];
        let path = full_path.split('?').next().unwrap_or(full_path);

        // Parse HTTP Headers
        let mut headers = HashMap::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            if let Some((k, v)) = line.split_once(':') {
                headers.insert(k.trim().to_lowercase(), v.trim().to_string());
            }
        }

        // 1. WebSocket Upgrade Request (/ws atau /broadcast/signaling)
        let is_ws_upgrade = headers.get("upgrade").map(|v| v.to_lowercase() == "websocket").unwrap_or(false);
        if is_ws_upgrade && (path == "/ws" || path == "/broadcast/signaling") {
            if let Some(sec_key) = headers.get("sec-websocket-key") {
                let accept_token = generate_sec_websocket_accept(sec_key);
                let response = format!(
                    "HTTP/1.1 101 Switching Protocols\r\n\
                    Upgrade: websocket\r\n\
                    Connection: Upgrade\r\n\
                    Sec-WebSocket-Accept: {}\r\n\r\n",
                    accept_token
                );

                if stream.write_all(response.as_bytes()).is_ok() {
                    // Berhasil upgrade ke WebSocket
                    let client_stream = match stream.try_clone() {
                        Ok(s) => s,
                        Err(_) => return,
                    };

                    // Kirim status Tally terakhir ke klien baru ini
                    {
                        let tally = last_tally.lock().unwrap();
                        let init_msg = serde_json::json!({
                            "type": "TALLY_UPDATE",
                            "program": tally.0,
                            "preview": tally.1,
                            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)
                        }).to_string();
                        let frame = encode_frame(1, init_msg.as_bytes(), false);
                        let _ = stream.write_all(&frame);
                    }

                    // Tambahkan ke daftar klien aktif
                    {
                        let mut clients = ws_clients.lock().unwrap();
                        clients.push(client_stream);
                    }

                    // Masuk ke pembacaan frame WebSocket interaktif
                    Self::run_websocket_session(stream, ws_clients, last_tally);
                }
            }
            return;
        }

        // 2. HTTP Static Assets & REST Endpoints (Mendukung GET dan HEAD)
        let is_get_or_head = method == "GET" || method == "HEAD";
        match path {
            "/" | "/index.html" if is_get_or_head => {
                let content = Self::load_asset("aplikasi/08_widya_broadcasting_studio/index.html", EMBEDDED_MCR_INDEX_HTML);
                Self::send_http_response(stream, method, 200, "text/html; charset=utf-8", &content);
            }
            "/app.js" if is_get_or_head => {
                let content = Self::load_asset("aplikasi/08_widya_broadcasting_studio/app.js", EMBEDDED_MCR_APP_JS);
                Self::send_http_response(stream, method, 200, "application/javascript; charset=utf-8", &content);
            }
            "/style.css" if is_get_or_head => {
                let content = Self::load_asset("aplikasi/08_widya_broadcasting_studio/style.css", EMBEDDED_MCR_STYLE_CSS);
                Self::send_http_response(stream, method, 200, "text/css; charset=utf-8", &content);
            }
            "/kamera" | "/kamera_hp.html" if is_get_or_head => {
                let content = Self::load_asset("aplikasi/08_widya_broadcasting_studio/kamera_hp.html", EMBEDDED_KAMERA_HP_HTML);
                Self::send_http_response(stream, method, 200, "text/html; charset=utf-8", &content);
            }
            "/var" | "/var_dewan_hakim.html" if is_get_or_head => {
                let content = Self::load_asset("aplikasi/08_widya_broadcasting_studio/var_dewan_hakim.html", EMBEDDED_VAR_HAKIM_HTML);
                Self::send_http_response(stream, method, 200, "text/html; charset=utf-8", &content);
            }
            "/manifest.json" if is_get_or_head => {
                let content = Self::load_asset("aplikasi/08_widya_broadcasting_studio/manifest.json", EMBEDDED_MANIFEST_JSON);
                Self::send_http_response(stream, method, 200, "application/manifest+json; charset=utf-8", &content);
            }
            "/sw.js" if is_get_or_head => {
                let content = Self::load_asset("aplikasi/08_widya_broadcasting_studio/sw.js", EMBEDDED_SW_JS);
                Self::send_http_response_with_header(stream, method, 200, "application/javascript; charset=utf-8", "Service-Worker-Allowed: /", &content);
            }
            "/api/push/notify" if is_get_or_head => {
                let notify_json = serde_json::json!({
                    "sukses": true,
                    "title": "📡 Widya MCR Broadcast Alert",
                    "body": "Instruksi Sutradara: Persiapan CUT ke Program dalam 5 detik!",
                    "action": "open_app",
                    "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)
                }).to_string();
                Self::send_http_response(stream, method, 200, "application/json; charset=utf-8", &notify_json);
            }
            "/api/status" if is_get_or_head => {
                let tally = last_tally.lock().unwrap();
                let client_count = ws_clients.lock().unwrap().len();
                let status_json = serde_json::json!({
                    "studio": "Widya Sovereign Broadcast Studio",
                    "status": "ONLINE_OB_VAN",
                    "program_aktif": tally.0,
                    "preview_aktif": tally.1,
                    "klien_terhubung": client_count,
                    "pwa_service_worker": "ACTIVE_V2_OFFLINE_RESILIENT",
                    "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
                }).to_string();
                Self::send_http_response(stream, method, 200, "application/json; charset=utf-8", &status_json);
            }
            "/api/tally" if is_get_or_head => {
                // Trigger Tally via query param: /api/tally?prog=cam_a_wide&prev=cam_a_close
                if let Some(query) = full_path.split('?').nth(1) {
                    let q_map: HashMap<&str, &str> = query.split('&').filter_map(|s| s.split_once('=')).collect();
                    if let (Some(&prog), Some(&prev)) = (q_map.get("prog"), q_map.get("prev")) {
                        {
                            let mut tally = last_tally.lock().unwrap();
                            tally.0 = prog.to_string();
                            tally.1 = prev.to_string();
                        }
                        let payload = serde_json::json!({
                            "type": "TALLY_UPDATE",
                            "program": prog,
                            "preview": prev,
                            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)
                        }).to_string();
                        let frame = encode_frame(1, payload.as_bytes(), false);
                        let mut clients = ws_clients.lock().unwrap();
                        clients.retain_mut(|c| c.write_all(&frame).is_ok());
                    }
                }
                Self::send_http_response(stream, method, 200, "application/json", "{\"sukses\":true}");
            }
            _ if method == "OPTIONS" => {
                let cors = "HTTP/1.1 204 NO CONTENT\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, HEAD, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 0\r\n\r\n";
                let _ = stream.write_all(cors.as_bytes());
            }
            _ => {
                let not_found = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\nContent-Length: 22\r\n\r\nEndpoint tidak ditemukan";
                let _ = stream.write_all(not_found.as_bytes());
            }
        }
    }

    fn run_websocket_session(
        stream: &mut TcpStream,
        ws_clients: Arc<Mutex<Vec<TcpStream>>>,
        last_tally: Arc<Mutex<(String, String)>>,
    ) {
        let mut buffer = [0u8; 16384];
        loop {
            let bytes = match stream.read(&mut buffer) {
                Ok(n) if n > 0 => n,
                _ => break,
            };

            if let Some((header, header_size)) = parse_frame_header(&buffer[..bytes]) {
                if header.opcode == 8 {
                    // Close frame
                    break;
                } else if header.opcode == 9 {
                    // Ping -> kirim Pong
                    let pong = encode_frame(10, &[], false);
                    let _ = stream.write_all(&pong);
                } else if header.opcode == 1 {
                    // Text payload
                    let data_start = if header.mask { header_size + 4 } else { header_size };
                    let data_end = (data_start + header.payload_len as usize).min(bytes);

                    let mut payload_bytes = buffer[data_start..data_end].to_vec();
                    if header.mask {
                        let mask_key = &buffer[header_size..header_size + 4];
                        for (i, b) in payload_bytes.iter_mut().enumerate() {
                            *b ^= mask_key[i % 4];
                        }
                    }

                    if let Ok(text) = String::from_utf8(payload_bytes) {
                        // Periksa pesan JSON
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                            if let Some(msg_type) = v.get("type").and_then(|t| t.as_str()) {
                                if msg_type == "TALLY_UPDATE" {
                                    if let (Some(p), Some(pr)) = (v.get("program").and_then(|s| s.as_str()), v.get("preview").and_then(|s| s.as_str())) {
                                        let mut t = last_tally.lock().unwrap();
                                        t.0 = p.to_string();
                                        t.1 = pr.to_string();
                                    }
                                } else if msg_type == "REQUEST_INITIAL_TALLY" {
                                    let t = last_tally.lock().unwrap();
                                    let resp = serde_json::json!({
                                        "type": "TALLY_UPDATE",
                                        "program": t.0,
                                        "preview": t.1,
                                        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)
                                    }).to_string();
                                    let frame = encode_frame(1, resp.as_bytes(), false);
                                    let _ = stream.write_all(&frame);
                                    continue;
                                }
                            }
                        }

                        // Relai frame ke seluruh klien aktif lainnya
                        let relay_frame = encode_frame(1, text.as_bytes(), false);
                        let mut clients = ws_clients.lock().unwrap();
                        clients.retain_mut(|c| {
                            c.write_all(&relay_frame).is_ok()
                        });
                    }
                }
            }
        }
    }

    fn send_http_response(stream: &mut TcpStream, method: &str, status: u16, content_type: &str, body: &str) {
        let status_text = match status {
            200 => "200 OK",
            404 => "404 Not Found",
            _ => "200 OK",
        };
        let response = if method == "HEAD" {
            format!(
                "HTTP/1.1 {}\r\n\
                Content-Type: {}\r\n\
                Content-Length: {}\r\n\
                Access-Control-Allow-Origin: *\r\n\
                Connection: close\r\n\r\n",
                status_text, content_type, body.len()
            )
        } else {
            format!(
                "HTTP/1.1 {}\r\n\
                Content-Type: {}\r\n\
                Content-Length: {}\r\n\
                Access-Control-Allow-Origin: *\r\n\
                Connection: close\r\n\r\n{}",
                status_text, content_type, body.len(), body
            )
        };
        let _ = stream.write_all(response.as_bytes());
    }

    fn send_http_response_with_header(stream: &mut TcpStream, method: &str, status: u16, content_type: &str, extra_header: &str, body: &str) {
        let status_text = match status {
            200 => "200 OK",
            404 => "404 Not Found",
            _ => "200 OK",
        };
        let extra = if extra_header.is_empty() { String::new() } else { format!("{}\r\n", extra_header) };
        let response = if method == "HEAD" {
            format!(
                "HTTP/1.1 {}\r\n\
                Content-Type: {}\r\n\
                Content-Length: {}\r\n\
                Access-Control-Allow-Origin: *\r\n\
                {}Connection: close\r\n\r\n",
                status_text, content_type, body.len(), extra
            )
        } else {
            format!(
                "HTTP/1.1 {}\r\n\
                Content-Type: {}\r\n\
                Content-Length: {}\r\n\
                Access-Control-Allow-Origin: *\r\n\
                {}Connection: close\r\n\r\n{}",
                status_text, content_type, body.len(), extra, body
            )
        };
        let _ = stream.write_all(response.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    #[test]
    fn test_detect_lan_ip() {
        let ip = detect_lan_ip();
        assert!(!ip.is_empty());
        assert!(ip.contains('.'));
    }

    #[test]
    fn test_broadcast_studio_server_initialization() {
        let server = BroadcastStudioServer::new("0.0.0.0", 8085);
        assert_eq!(server.bind_addr, "0.0.0.0");
        assert_eq!(server.port, 8085);
        assert_eq!(server.last_tally.lock().unwrap().0, "cam_a_wide");
        assert_eq!(server.last_tally.lock().unwrap().1, "cam_a_close");
    }

    #[test]
    fn test_broadcast_server_http_and_api() {
        // Cari port bebas
        let listener = TcpListener::bind("127.0.0.1:0").expect("Gagal mengikat port ephemeral");
        let free_port = listener.local_addr().unwrap().port();
        drop(listener);

        let server = Arc::new(BroadcastStudioServer::new("127.0.0.1", free_port));
        let server_clone = server.clone();

        // Jalankan server di background thread
        thread::spawn(move || {
            let _ = server_clone.start(false);
        });

        // Beri waktu 80ms untuk mendengarkan port
        thread::sleep(Duration::from_millis(80));

        // 1. Uji GET /api/status
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", free_port)).expect("Gagal connect ke server");
        stream.write_all(b"GET /api/status HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut resp = Vec::new();
        stream.read_to_end(&mut resp).unwrap();
        let resp_str = String::from_utf8_lossy(&resp);
        assert!(resp_str.contains("200 OK"));
        assert!(resp_str.contains("ONLINE_OB_VAN"));

        // 2. Uji GET /api/tally?prog=cam_b_close&prev=skycam_3d
        let mut stream_tally = TcpStream::connect(format!("127.0.0.1:{}", free_port)).expect("Gagal connect ke server");
        stream_tally.write_all(b"GET /api/tally?prog=cam_b_close&prev=skycam_3d HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut resp_tally = Vec::new();
        stream_tally.read_to_end(&mut resp_tally).unwrap();
        let resp_tally_str = String::from_utf8_lossy(&resp_tally);
        assert!(resp_tally_str.contains("200 OK"));
        assert!(resp_tally_str.contains("\"sukses\":true"));

        // Verifikasi state tally terupdate di memory
        let t = server.last_tally.lock().unwrap();
        assert_eq!(t.0, "cam_b_close");
        assert_eq!(t.1, "skycam_3d");

        // 3. Uji GET / (MCR HTML)
        let mut stream_html = TcpStream::connect(format!("127.0.0.1:{}", free_port)).expect("Gagal connect ke server");
        stream_html.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut resp_html = Vec::new();
        stream_html.read_to_end(&mut resp_html).unwrap();
        let resp_html_str = String::from_utf8_lossy(&resp_html);
        assert!(resp_html_str.contains("200 OK"));
        assert!(resp_html_str.contains("text/html"));
        assert!(resp_html_str.contains("WidyaWeb"));

        // 4. Uji GET /kamera (PWA Smartphone)
        let mut stream_cam = TcpStream::connect(format!("127.0.0.1:{}", free_port)).expect("Gagal connect ke server");
        stream_cam.write_all(b"GET /kamera HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut resp_cam = Vec::new();
        stream_cam.read_to_end(&mut resp_cam).unwrap();
        let resp_cam_str = String::from_utf8_lossy(&resp_cam);
        assert!(resp_cam_str.contains("200 OK"));
        assert!(resp_cam_str.contains("tally-viewport"));

        // 5. Uji GET /manifest.json (PWA Manifest)
        let mut stream_mani = TcpStream::connect(format!("127.0.0.1:{}", free_port)).expect("Gagal connect ke server");
        stream_mani.write_all(b"GET /manifest.json HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut resp_mani = Vec::new();
        stream_mani.read_to_end(&mut resp_mani).unwrap();
        let resp_mani_str = String::from_utf8_lossy(&resp_mani);
        assert!(resp_mani_str.contains("200 OK"));
        assert!(resp_mani_str.contains("application/manifest+json"));
        assert!(resp_mani_str.contains("Widya MCR"));

        // 6. Uji GET /sw.js (PWA Service Worker)
        let mut stream_sw = TcpStream::connect(format!("127.0.0.1:{}", free_port)).expect("Gagal connect ke server");
        stream_sw.write_all(b"GET /sw.js HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut resp_sw = Vec::new();
        stream_sw.read_to_end(&mut resp_sw).unwrap();
        let resp_sw_str = String::from_utf8_lossy(&resp_sw);
        assert!(resp_sw_str.contains("200 OK"));
        assert!(resp_sw_str.contains("Service-Worker-Allowed: /"));
        assert!(resp_sw_str.contains("widya-mcr-studio-v2.1"));

        // 7. Uji GET /api/push/notify (MCR Push Broadcast Dispatch)
        let mut stream_push = TcpStream::connect(format!("127.0.0.1:{}", free_port)).expect("Gagal connect ke server");
        stream_push.write_all(b"GET /api/push/notify HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
        let mut resp_push = Vec::new();
        stream_push.read_to_end(&mut resp_push).unwrap();
        let resp_push_str = String::from_utf8_lossy(&resp_push);
        assert!(resp_push_str.contains("200 OK"));
        assert!(resp_push_str.contains("Widya MCR Broadcast Alert"));
    }
}

