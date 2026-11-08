//! Sovereign Pure HTTP/1.1 Engine (Client & Server Murni)
//! 100% Bebas Ketergantungan Eksternal (Zero External Crates: Tanpa ureq, reqwest, hyper, tokio).
//! Mengimplementasikan RFC 7230/7231 (HTTP/1.1) menggunakan direct `std::net::TcpStream` & `TcpListener`.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub struct PureHttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl PureHttpRequest {
    pub fn new(method: &str, path: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), "Widya-Sovereign-HTTP/0.1.0".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());
        Self {
            method: method.to_uppercase(),
            path: path.to_string(),
            version: "HTTP/1.1".to_string(),
            headers,
            body: Vec::new(),
        }
    }

    pub fn header(mut self, key: &str, val: &str) -> Self {
        self.headers.insert(key.to_string(), val.to_string());
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.headers.insert("Content-Length".to_string(), body.len().to_string());
        self.body = body;
        self
    }

    pub fn body_str(self, body_text: &str) -> Self {
        self.body(body_text.as_bytes().to_vec())
    }

    pub fn body_as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.body)
    }

    pub fn to_bytes(&self, host: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        let _ = write!(buf, "{} {} {}\r\n", self.method, self.path, self.version);
        let _ = write!(buf, "Host: {}\r\n", host);
        for (k, v) in &self.headers {
            if k.eq_ignore_ascii_case("Host") {
                continue;
            }
            let _ = write!(buf, "{}: {}\r\n", k, v);
        }
        buf.extend_from_slice(b"\r\n");
        buf.extend_from_slice(&self.body);
        buf
    }

    pub fn from_stream<R: Read>(reader: &mut R) -> Result<Self, String> {
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();
        if buf_reader.read_line(&mut line).map_err(|e| e.to_string())? == 0 {
            return Err("Koneksi ditutup sebelum membaca request".to_string());
        }

        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() < 3 {
            return Err(format!("Format Request HTTP tidak valid: '{}'", line.trim()));
        }
        let method = parts[0].to_string();
        let path = parts[1].to_string();
        let version = parts[2].to_string();

        let mut headers = HashMap::new();
        let mut content_length = 0usize;

        loop {
            line.clear();
            let bytes_read = buf_reader.read_line(&mut line).map_err(|e| e.to_string())?;
            if bytes_read == 0 || line == "\r\n" || line == "\n" {
                break;
            }
            if let Some(colon) = line.find(':') {
                let key = line[..colon].trim().to_string();
                let val = line[colon + 1..].trim().to_string();
                if key.eq_ignore_ascii_case("Content-Length") {
                    content_length = val.parse::<usize>().unwrap_or(0);
                }
                headers.insert(key, val);
            }
        }

        let mut body = vec![0u8; content_length];
        if content_length > 0 {
            buf_reader.read_exact(&mut body).map_err(|e| e.to_string())?;
        }

        Ok(Self {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PureHttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl PureHttpResponse {
    pub fn new(status_code: u16, status_text: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Server".to_string(), "Widya-Sovereign-HTTP-Server/0.1.0".to_string());
        headers.insert("Connection".to_string(), "close".to_string());
        Self {
            status_code,
            status_text: status_text.to_string(),
            headers,
            body: Vec::new(),
        }
    }

    pub fn ok() -> Self {
        Self::new(200, "OK")
    }

    pub fn not_found() -> Self {
        Self::new(404, "Not Found").body_str("404 Halaman Tidak Ditemukan")
    }

    pub fn bad_request(msg: &str) -> Self {
        Self::new(400, "Bad Request").body_str(msg)
    }

    pub fn header(mut self, key: &str, val: &str) -> Self {
        self.headers.insert(key.to_string(), val.to_string());
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.headers.insert("Content-Length".to_string(), body.len().to_string());
        self.body = body;
        self
    }

    pub fn body_str(self, text: &str) -> Self {
        self.body(text.as_bytes().to_vec())
    }

    pub fn json(mut self, json_str: &str) -> Self {
        self.headers.insert("Content-Type".to_string(), "application/json; charset=utf-8".to_string());
        self.body(json_str.as_bytes().to_vec())
    }

    pub fn body_as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.body)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let _ = write!(buf, "HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        for (k, v) in &self.headers {
            let _ = write!(buf, "{}: {}\r\n", k, v);
        }
        buf.extend_from_slice(b"\r\n");
        buf.extend_from_slice(&self.body);
        buf
    }

    pub fn from_stream<R: Read>(reader: &mut R) -> Result<Self, String> {
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();
        if buf_reader.read_line(&mut line).map_err(|e| e.to_string())? == 0 {
            return Err("Koneksi terputus saat membaca respons".to_string());
        }

        let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();
        if parts.len() < 2 {
            return Err(format!("Header respons HTTP tidak valid: '{}'", line.trim()));
        }

        let status_code = parts[1].parse::<u16>().map_err(|_| "Kode status bukan integer".to_string())?;
        let status_text = if parts.len() >= 3 { parts[2].to_string() } else { "OK".to_string() };

        let mut headers = HashMap::new();
        let mut content_length = 0usize;

        loop {
            line.clear();
            let bytes_read = buf_reader.read_line(&mut line).map_err(|e| e.to_string())?;
            if bytes_read == 0 || line == "\r\n" || line == "\n" {
                break;
            }
            if let Some(colon) = line.find(':') {
                let key = line[..colon].trim().to_string();
                let val = line[colon + 1..].trim().to_string();
                if key.eq_ignore_ascii_case("Content-Length") {
                    content_length = val.parse::<usize>().unwrap_or(0);
                }
                headers.insert(key, val);
            }
        }

        let mut body = vec![0u8; content_length];
        if content_length > 0 {
            buf_reader.read_exact(&mut body).map_err(|e| e.to_string())?;
        }

        Ok(Self {
            status_code,
            status_text,
            headers,
            body,
        })
    }
}

pub struct PureHttpClient {
    pub timeout: Duration,
}

impl PureHttpClient {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(10),
        }
    }

    pub fn get(url: &str) -> Result<PureHttpResponse, String> {
        Self::new().send_request("GET", url, &[], None)
    }

    pub fn post(url: &str, body: &[u8], content_type: &str) -> Result<PureHttpResponse, String> {
        Self::new().send_request("POST", url, body, Some(content_type))
    }

    pub fn send_request(&self, method: &str, url: &str, body: &[u8], content_type: Option<&str>) -> Result<PureHttpResponse, String> {
        let (host, port, path) = Self::parse_url(url)?;
        let addr = format!("{}:{}", host, port);

        let mut stream = TcpStream::connect_timeout(
            &addr.to_socket_addrs().map_err(|e| e.to_string())?.next().ok_or("Alamat host tidak valid")?,
            self.timeout,
        ).map_err(|e| format!("Gagal menghubungkan ke {}: {}", addr, e))?;

        stream.set_read_timeout(Some(self.timeout)).map_err(|e| e.to_string())?;
        stream.set_write_timeout(Some(self.timeout)).map_err(|e| e.to_string())?;

        let mut req = PureHttpRequest::new(method, &path);
        if let Some(ct) = content_type {
            req = req.header("Content-Type", ct);
        }
        if !body.is_empty() {
            req = req.body(body.to_vec());
        }

        let bytes = req.to_bytes(&host);
        stream.write_all(&bytes).map_err(|e| format!("Gagal mengirim payload HTTP: {}", e))?;
        stream.flush().map_err(|e| e.to_string())?;

        PureHttpResponse::from_stream(&mut stream)
    }

    fn parse_url(url: &str) -> Result<(String, u16, String), String> {
        let stripped = if let Some(s) = url.strip_prefix("http://") {
            s
        } else if let Some(s) = url.strip_prefix("https://") {
            return Err("Protokol HTTPS memerlukan modul TLS pasca-kuantum (gunakan http:// untuk direct raw TCP)".to_string());
        } else {
            url
        };

        let (host_port, path) = match stripped.find('/') {
            Some(idx) => (&stripped[..idx], &stripped[idx..]),
            None => (stripped, "/"),
        };

        let (host, port) = match host_port.find(':') {
            Some(idx) => {
                let h = host_port[..idx].to_string();
                let p = host_port[idx + 1..].parse::<u16>().map_err(|_| "Port tidak valid".to_string())?;
                (h, p)
            }
            None => (host_port.to_string(), 80),
        };

        Ok((host, port, path.to_string()))
    }
}

pub struct PureHttpServer;

impl PureHttpServer {
    pub fn serve<F>(addr: &str, handler: F) -> Result<(), String>
    where
        F: Fn(PureHttpRequest) -> PureHttpResponse + Send + Sync + 'static,
    {
        let listener = TcpListener::bind(addr).map_err(|e| format!("Gagal mengikat port {}: {}", addr, e))?;
        let handler = Arc::new(handler);

        for stream_res in listener.incoming() {
            match stream_res {
                Ok(mut stream) => {
                    let h = Arc::clone(&handler);
                    thread::spawn(move || {
                        if let Ok(req) = PureHttpRequest::from_stream(&mut stream) {
                            let resp = h(req);
                            let _ = stream.write_all(&resp.to_bytes());
                            let _ = stream.flush();
                        }
                    });
                }
                Err(_) => break,
            }
        }
        Ok(())
    }
}

// ============================================================================
// UNIT TESTS UNTUK PURE HTTP CLIENT & SERVER
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_http_request_serialization() {
        let req = PureHttpRequest::new("POST", "/api/v1/transaksi")
            .header("X-Widya-Auth", "Berdaulat-2026")
            .body_str(r#"{"status":"BERHASIL"}"#);

        let bytes = req.to_bytes("localhost:8080");
        let text = std::str::from_utf8(&bytes).unwrap();
        assert!(text.starts_with("POST /api/v1/transaksi HTTP/1.1\r\n"));
        assert!(text.contains("Host: localhost:8080\r\n"));
        assert!(text.contains("Content-Length: 21\r\n"));
        assert!(text.ends_with(r#"{"status":"BERHASIL"}"#));
    }

    #[test]
    fn test_pure_http_response_serialization() {
        let resp = PureHttpResponse::ok()
            .header("X-Powered-By", "WidyaBase Native")
            .json(r#"{"pesan":"Kedaulatan Sistem Terpenuhi"}"#);

        let bytes = resp.to_bytes();
        let text = std::str::from_utf8(&bytes).unwrap();
        assert!(text.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(text.contains("Content-Type: application/json; charset=utf-8\r\n"));
        assert!(text.contains("Kedaulatan Sistem Terpenuhi"));
    }

    #[test]
    fn test_pure_http_loopback_roundtrip() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        // Server thread
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let req = PureHttpRequest::from_stream(&mut stream).unwrap();
                assert_eq!(req.method, "GET");
                assert_eq!(req.path, "/status");

                let resp = PureHttpResponse::ok().body_str("Pilar II Mandiri!");
                let _ = stream.write_all(&resp.to_bytes());
                let _ = stream.flush();
            }
        });

        // Client request
        let url = format!("http://{}", addr);
        let resp = PureHttpClient::get(&format!("{}/status", url)).expect("Gagal memanggil pure HTTP server");
        assert_eq!(resp.status_code, 200);
        assert_eq!(resp.body_as_str().unwrap(), "Pilar II Mandiri!");
    }
}
