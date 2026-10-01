//! Modul Bridging BPJS Kesehatan V-Claim & Antrean Online
//! Pembangkit header otentikasi X-Cons-ID, X-Timestamp, dan X-Signature HMAC-SHA256 standar BPJS.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct BpjsAuthHeaders {
    pub cons_id: String,
    pub timestamp: String,
    pub signature: String,
    pub user_key: String,
}

pub struct BpjsBridge {
    cons_id: String,
    secret_key: String,
    user_key: String,
}

impl BpjsBridge {
    pub fn new(cons_id: &str, secret_key: &str, user_key: &str) -> Self {
        Self {
            cons_id: cons_id.to_string(),
            secret_key: secret_key.to_string(),
            user_key: user_key.to_string(),
        }
    }

    /// Menghasilkan header otentikasi resmi untuk request ke REST API BPJS Kesehatan
    pub fn generate_headers(&self) -> Result<BpjsAuthHeaders, String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs();
        let timestamp = now.to_string();

        let data_to_sign = format!("{}&{}", self.cons_id, timestamp);
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .map_err(|e| format!("Galat HMAC key: {}", e))?;
        mac.update(data_to_sign.as_bytes());
        let result = mac.finalize();
        use base64::Engine;
        let signature = base64::engine::general_purpose::STANDARD.encode(result.into_bytes());

        Ok(BpjsAuthHeaders {
            cons_id: self.cons_id.clone(),
            timestamp,
            signature,
            user_key: self.user_key.clone(),
        })
    }
}
