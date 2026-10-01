use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEventType {
    #[serde(rename = "payment.pending")]
    PaymentPending,
    #[serde(rename = "payment.success")]
    PaymentSuccess,
    #[serde(rename = "payment.expired")]
    PaymentExpired,
    #[serde(rename = "payment.failed")]
    PaymentFailed,
    #[serde(rename = "refund.success")]
    RefundSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub id: String,
    pub event: WebhookEventType,
    pub created_at: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDeliveryAttempt {
    pub attempt_number: u32,
    pub status_code: Option<u16>,
    pub response_body: Option<String>,
    pub next_retry_timestamp: Option<String>,
    pub success: bool,
}

/// Menghasilkan Tanda Tangan Webhook HMAC-SHA256 (Hex Format)
pub fn buat_webhook_signature(payload_json: &str, webhook_secret: &str) -> Result<String, String> {
    let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())
        .map_err(|e| format!("Inisialisasi HMAC error: {}", e))?;
    mac.update(payload_json.as_bytes());
    let result = mac.finalize();
    Ok(hex::encode(result.into_bytes()))
}

/// Memvalidasi Tanda Tangan Webhook
pub fn verifikasi_webhook_signature(
    payload_json: &str,
    signature_header: &str,
    webhook_secret: &str,
) -> bool {
    if let Ok(expected) = buat_webhook_signature(payload_json, webhook_secret) {
        expected.eq_ignore_ascii_case(signature_header)
    } else {
        false
    }
}

/// Menghitung Jadwal Retry Exponential Backoff dalam detik
/// Percobaan 1: 1s, Percobaan 2: 5s, Percobaan 3: 30s, Percobaan 4: 300s (5m), Percobaan 5: 3600s (1h)
pub fn hitung_delay_retry_webhook(attempt: u32) -> Option<u64> {
    match attempt {
        1 => Some(1),
        2 => Some(5),
        3 => Some(30),
        4 => Some(300),
        5 => Some(3600),
        _ => None, // Maksimal 5 percobaan
    }
}

/// Membentuk Payload Notifikasi Standar Payment Gateway
pub fn buat_webhook_event(
    event_type: WebhookEventType,
    invoice_id: &str,
    amount: i64,
    status: &str,
    payment_method: &str,
) -> WebhookPayload {
    WebhookPayload {
        id: format!("evt_{}", uuid::Uuid::new_v4().simple()),
        event: event_type,
        created_at: Utc::now().to_rfc3339(),
        data: serde_json::json!({
            "invoice_id": invoice_id,
            "amount": amount,
            "currency": "IDR",
            "status": status,
            "payment_method": payment_method,
            "paid_at": Utc::now().to_rfc3339()
        }),
    }
}
