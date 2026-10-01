use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};

type HmacSha512 = Hmac<Sha512>;

/// Header Standar SNAP Bank Indonesia (Standar Nasional Open API Pembayaran)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapHeaders {
    pub x_timestamp: String,      // Format ISO 8601 (e.g. 2026-09-30T10:15:30+07:00)
    pub x_signature: String,      // Base64 / Hex signature
    pub x_partner_id: String,     // ID Merchant / Partner
    pub x_external_id: String,    // ID Unik Permintaan Transaksi (Trace Number)
    pub channel_id: Option<String>,
    pub authorization: Option<String>, // Bearer <Token>
}

/// Struktur Permintaan Pembuatan Virtual Account (SNAP VA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapCreateVaRequest {
    #[serde(rename = "partnerServiceId")]
    pub partner_service_id: String, // 8 digit prefix bank
    #[serde(rename = "customerNo")]
    pub customer_no: String,
    #[serde(rename = "virtualAccountNo")]
    pub virtual_account_no: String,
    #[serde(rename = "virtualAccountName")]
    pub virtual_account_name: String,
    #[serde(rename = "trxId")]
    pub trx_id: String,
    #[serde(rename = "totalAmount")]
    pub total_amount: SnapAmount,
    #[serde(rename = "expiredDate")]
    pub expired_date: String,
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapAmount {
    pub value: String, // Format "150000.00"
    pub currency: String, // "IDR"
}

/// Struktur Respon Standar SNAP BI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapResponse<T> {
    #[serde(rename = "responseCode")]
    pub response_code: String, // e.g. "2002700" (Berhasil), "4012700" (Unauthorized)
    #[serde(rename = "responseMessage")]
    pub response_message: String,
    #[serde(rename = "virtualAccountData", skip_serializing_if = "Option::is_none")]
    pub virtual_account_data: Option<T>,
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<serde_json::Value>,
}

/// Menghitung SHA-256 Digest dari Request Body (Minified JSON) dalam format Hex Lowercase
pub fn hitung_snap_body_hash(body: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    hex::encode(hasher.finalize()).to_lowercase()
}

/// Membentuk String-to-Sign Standar SNAP BI Simetris (Transaksional / B2B2C):
/// Format: `HTTPMethod + ":" + EndpointUrl + ":" + AccessToken + ":" + Lowercase(Hex(SHA256(MinifiedBody))) + ":" + Timestamp`
pub fn bentuk_snap_string_to_sign(
    http_method: &str,
    endpoint_url: &str,
    access_token: &str,
    body: &str,
    timestamp: &str,
) -> String {
    let body_hash = hitung_snap_body_hash(body);
    let clean_method = http_method.to_uppercase();
    format!("{}:{}:{}:{}:{}", clean_method, endpoint_url, access_token, body_hash, timestamp)
}

/// Menghasilkan Tanda Tangan Simetris HMAC-SHA512 SNAP BI (Base64 Encoded)
pub fn buat_snap_simetris_signature(
    string_to_sign: &str,
    client_secret: &str,
) -> Result<String, String> {
    use base64::Engine;
    let mut mac = HmacSha512::new_from_slice(client_secret.as_bytes())
        .map_err(|e| format!("Gagal inisialisasi HMAC-SHA512: {}", e))?;
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize();
    Ok(base64::engine::general_purpose::STANDARD.encode(result.into_bytes()))
}

/// Verifikasi Tanda Tangan Simetris HMAC-SHA512 SNAP BI
pub fn verifikasi_snap_simetris_signature(
    string_to_sign: &str,
    client_secret: &str,
    signature_base64: &str,
) -> bool {
    if let Ok(expected) = buat_snap_simetris_signature(string_to_sign, client_secret) {
        expected == signature_base64
    } else {
        false
    }
}

/// Membentuk String-to-Sign Standar SNAP BI Asimetris (Token B2B):
/// Format: `client_id + "|" + timestamp`
pub fn bentuk_snap_asimetris_string_to_sign(client_id: &str, timestamp: &str) -> String {
    format!("{}|{}", client_id, timestamp)
}

/// Menghasilkan Timestamp ISO 8601 Resmi dengan Offset Waktu
pub fn generate_snap_timestamp() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// Kode Respon Standar SNAP BI
pub struct SnapResponseCodes;
impl SnapResponseCodes {
    pub const SUCCESS: &'static str = "2000000";
    pub const SUCCESS_VA_CREATE: &'static str = "2002700";
    pub const SUCCESS_VA_INQUIRY: &'static str = "2002400";
    pub const SUCCESS_VA_PAYMENT: &'static str = "2002500";
    pub const SUCCESS_QRIS_GENERATE: &'static str = "2004700";
    pub const SUCCESS_TRANSFER: &'static str = "2001800";
    
    pub const BAD_REQUEST: &'static str = "4000000";
    pub const INVALID_SIGNATURE: &'static str = "4010000";
    pub const UNAUTHORIZED: &'static str = "4017300";
    pub const BILL_EXPIRED: &'static str = "4042419";
    pub const BILL_ALREADY_PAID: &'static str = "4042512";
    pub const INSUFFICIENT_FUNDS: &'static str = "4047008";
    pub const INTERNAL_SERVER_ERROR: &'static str = "5000000";
}
