//! Edge Middleware, Authentication & Rate Limiting Gateway (Zero External Dependencies)
//! Standar Gerbang Edge: Filter Request Interceptor, Verifikasi Token JWT sub-milidetik,
//! Token Bucket Rate Limiting, dan Geo-Routing.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use crate::kripto::pure_crypto::{hmac_sha256, to_base64_url, from_base64_url};

/// Respon Middleware Edge
#[derive(Debug, Clone, PartialEq)]
pub enum ResponMiddleware {
    Lanjutkan { header_tambahan: HashMap<String, String> },
    AlihkanRute { url_tujuan: String, status_code: u16 },
    TolakAkses { status_code: u16, pesan: String },
}

/// Mesin Token Bucket Rate Limiting
pub struct RateLimiterBucket {
    pub max_tokens: u32,
    pub refill_rate_per_sec: u32,
    pub client_buckets: Arc<Mutex<HashMap<String, (f32, u64)>>>, // IP -> (tokens, last_update_sec)
}

impl RateLimiterBucket {
    pub fn baru(max_tokens: u32, refill_rate_per_sec: u32) -> Self {
        Self {
            max_tokens,
            refill_rate_per_sec,
            client_buckets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn izinkan_request(&self, client_ip: &str) -> bool {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let mut map = self.client_buckets.lock().unwrap();

        let (mut tokens, last_time) = map.get(client_ip).copied().unwrap_or((self.max_tokens as f32, now));
        let elapsed = (now.saturating_sub(last_time)) as f32;
        tokens = (tokens + elapsed * self.refill_rate_per_sec as f32).min(self.max_tokens as f32);

        if tokens >= 1.0 {
            tokens -= 1.0;
            map.insert(client_ip.to_string(), (tokens, now));
            true
        } else {
            map.insert(client_ip.to_string(), (tokens, now));
            false // Terkena limit 429 Too Many Requests
        }
    }
}

/// Verifikator JWT Murni (Zero JsonWebToken Crate Dependency)
pub struct VerifikatorJWTMurni;

impl VerifikatorJWTMurni {
    /// Buat JWT HS256 Murni
    pub fn buat_token(payload_json: &str, kunci_rahasia: &str, masa_berlaku_detik: u64) -> String {
        let header = r#"{"alg":"HS256","typ":"JWT"}"#;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let exp = now + masa_berlaku_detik;

        let payload_with_exp = format!(r#"{{"data":{},"exp":{}}}"#, payload_json, exp);

        let h_b64 = to_base64_url(header.as_bytes());
        let p_b64 = to_base64_url(payload_with_exp.as_bytes());
        let to_sign = format!("{}.{}", h_b64, p_b64);

        let sig = hmac_sha256(kunci_rahasia.as_bytes(), to_sign.as_bytes());
        let s_b64 = to_base64_url(&sig);

        format!("{}.{}", to_sign, s_b64)
    }

    /// Verifikasi JWT HS256 Murni
    pub fn verifikasi_token(token: &str, kunci_rahasia: &str) -> Result<String, String> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err("Format JWT tidak valid (harus 3 bagian)".to_string());
        }

        let to_sign = format!("{}.{}", parts[0], parts[1]);
        let expected_sig = hmac_sha256(kunci_rahasia.as_bytes(), to_sign.as_bytes());
        let expected_sig_b64 = to_base64_url(&expected_sig);

        if parts[2] != expected_sig_b64 {
            return Err("Tanda tangan JWT tidak cocok / dipalsukan".to_string());
        }

        let payload_bytes = from_base64_url(parts[1]).map_err(|e| format!("Gagal decode payload: {}", e))?;
        let payload_str = String::from_utf8(payload_bytes).map_err(|e| format!("UTF-8 invalid: {}", e))?;

        Ok(payload_str)
    }
}
