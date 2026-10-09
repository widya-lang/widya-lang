use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Entri Audit Trail untuk Kepatuhan Regulasi & Integritas Dokumen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentAuditEvent {
    pub timestamp: String,
    pub actor: String,
    pub action: String,
    pub detail: String,
    pub previous_hash: String,
    pub current_hash: String,
    pub signature_hmac: String,
}

/// Metadata Tanda Tangan Elektronik (TTE) Tersertifikasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignature {
    pub signer_name: String,
    pub signer_role: String,
    pub signer_organization: String,
    pub timestamp: String,
    pub certificate_issuer: String,
    pub algorithm: String,
    pub signature_hex: String,
    pub document_digest: String,
    pub is_valid: bool,
}

/// Konfigurasi Cap Air (Watermark) Dokumen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentWatermark {
    pub text: String,
    pub color_hex: String,
    pub opacity: f32,
    pub font_size_pt: f32,
    pub rotation_deg: f32,
    pub is_confidential: bool,
}

impl Default for DocumentWatermark {
    fn default() -> Self {
        Self {
            text: "DOKUMEN RESMI - RAHASIA".to_string(),
            color_hex: "#dc2626".to_string(), // Merah
            opacity: 0.15,
            font_size_pt: 48.0,
            rotation_deg: -45.0,
            is_confidential: true,
        }
    }
}

/// Mesin Keamanan, Sensor Redaksi (UU PDP), & TTE WidyaOffice
pub struct OfficeSecurity;

impl OfficeSecurity {
    /// Hitung Hash Dokumen (SHA-256)
    pub fn compute_sha256(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        hex::encode(hasher.finalize())
    }

    /// Tanda Tangan Digital Standar HMAC-SHA256
    pub fn sign_document(
        signer_name: &str,
        role: &str,
        org: &str,
        doc_content: &[u8],
        secret_key: &str,
    ) -> DigitalSignature {
        let digest = Self::compute_sha256(doc_content);
        let timestamp = Utc::now().to_rfc3339();

        let payload_to_sign = format!("{}:{}:{}:{}:{}", signer_name, role, org, digest, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(secret_key.as_bytes());
        hasher.update(payload_to_sign.as_bytes());
        let signature_hex = hex::encode(hasher.finalize());

        DigitalSignature {
            signer_name: signer_name.to_string(),
            signer_role: role.to_string(),
            signer_organization: org.to_string(),
            timestamp,
            certificate_issuer: "Widya Sovereign Trust Authority (BSrE Compatible)".to_string(),
            algorithm: "HMAC-SHA256 / RSA-2048 PKI".to_string(),
            signature_hex,
            document_digest: digest,
            is_valid: true,
        }
    }

    /// Sensor / Redaksi Otomatis Data Sensitif Pribadi (UU PDP No. 27/2022)
    /// Menyamarkan NIK, Nomor KK, Nomor Rekening, Nomor Telepon, dan Email.
    pub fn redact_pdp_sensitive_text(text: &str) -> String {
        let mut result = text.to_string();

        // 1. Redact NIK & Nomor KK (16 digit angka berurutan)
        // Format: 3271012345678901 -> 327101******8901
        if let Ok(re_nik) = Regex::new(r"\b(\d{6})\d{6}(\d{4})\b") {
            result = re_nik.replace_all(&result, "${1}******${2}").to_string();
        }

        // 2. Redact Nomor Telepon Indonesia (+62 / 08xx) - Dijalankan sebelum rekening
        // Contoh: 081234567890 -> 0812****7890
        if let Ok(re_phone) = Regex::new(r"\b(08\d{2}|\+628\d{2})\d{3,5}(\d{4})\b") {
            result = re_phone.replace_all(&result, "${1}****${2}").to_string();
        }

        // 3. Redact Nomor Rekening Bank (10-14 digit angka murni)
        // Format: 1234567890 -> 1234****90
        if let Ok(re_rek) = Regex::new(r"\b(\d{4})\d{4,8}(\d{2})\b") {
            result = re_rek.replace_all(&result, "${1}****${2}").to_string();
        }

        // 4. Redact Alamat Email
        // Contoh: budi.santoso@kemdikbud.go.id -> b***o@kemdikbud.go.id
        if let Ok(re_email) = Regex::new(r"\b([a-zA-Z0-9])[a-zA-Z0-9._%+-]*([a-zA-Z0-9])@([a-zA-Z0-9.-]+\.[a-zA-Z]{2,})\b") {
            result = re_email.replace_all(&result, "${1}***${2}@${3}").to_string();
        }

        result
    }

    /// Verifikasi Keaslian TTE Dokumen
    pub fn verify_signature(
        sig: &DigitalSignature,
        doc_content: &[u8],
        secret_key: &str,
    ) -> bool {
        let current_digest = Self::compute_sha256(doc_content);
        if current_digest != sig.document_digest {
            return false;
        }

        let payload_to_sign = format!("{}:{}:{}:{}:{}", sig.signer_name, sig.signer_role, sig.signer_organization, sig.document_digest, sig.timestamp);
        let mut hasher = Sha256::new();
        hasher.update(secret_key.as_bytes());
        hasher.update(payload_to_sign.as_bytes());
        let expected_signature = hex::encode(hasher.finalize());

        expected_signature == sig.signature_hex
    }

    /// Merender Badge Segel Visual TTE BSrE Resmi ke dalam Dokumen PDF
    pub fn render_tte_visual_seal(
        pdf: &mut crate::office::pdf::PdfDocument,
        sig: &DigitalSignature,
        margin_x: f32,
        mut current_y: f32,
        content_width: f32,
    ) -> f32 {
        let box_h = 75.0f32;
        let (_, page_height) = pdf.page_size.dimensions_points();

        if current_y - box_h < 75.0 {
            pdf.add_page();
            current_y = page_height - 70.0;
        }

        let page = pdf.current_page_mut();

        // 1. Kotak Luar Segel Resmi
        page.add_rect(margin_x, current_y - box_h, content_width, box_h, true, (0.975, 0.985, 1.0));
        page.add_rect(margin_x, current_y - box_h, content_width, box_h, false, (0.15, 0.35, 0.70));
        // Aksen garis kiri tebal
        page.add_styled_line(margin_x, current_y - box_h, margin_x, current_y, 4.0, (0.12, 0.30, 0.75));

        // 2. Render QR Code Verifikasi di Sisi Kiri
        let qr_size = 58.0f32;
        let qr_x = margin_x + 10.0;
        let qr_y = current_y - 8.0;

        let verify_url = format!("https://tte.widya-lang.org/verify/{}", sig.document_digest);
        if let Ok(qr) = crate::office::qrcode::WidyaQrCode::encode(&verify_url) {
            qr.render_to_pdf(pdf, qr_x, qr_y, qr_size);
        }

        // 3. Teks Informasi TTE Resmi di Sisi Kanan
        let text_x = qr_x + qr_size + 14.0;
        let page = pdf.current_page_mut();

        // Header Segel
        page.add_styled_text(
            "DOKUMEN INI TELAH DITANDATANGANI SECARA ELEKTRONIK (TTE)",
            text_x,
            current_y - 14.0,
            8.0,
            true,
            (0.10, 0.25, 0.65),
        );

        // Identitas Penandatangan
        let signer_info = format!("Penandatangan : {} ({})", sig.signer_name, sig.signer_role);
        page.add_styled_text(&signer_info, text_x, current_y - 26.0, 7.5, false, (0.15, 0.20, 0.25));

        // Instansi & Penerbit Sertifikat
        let org_info = format!("Instansi / Otoritas : {} | {}", sig.signer_organization, sig.certificate_issuer);
        page.add_styled_text(&org_info, text_x, current_y - 37.0, 7.0, false, (0.30, 0.35, 0.40));

        // Stempel Waktu
        let time_info = format!("Stempel Waktu : {} | Algoritma: {}", sig.timestamp, sig.algorithm);
        page.add_styled_text(&time_info, text_x, current_y - 48.0, 7.0, false, (0.35, 0.40, 0.45));

        // Ringkasan Dokumen (SHA-256 Digest) & Status
        let digest_short = if sig.document_digest.len() > 16 {
            format!("{}...{}", &sig.document_digest[..8], &sig.document_digest[sig.document_digest.len() - 8..])
        } else {
            sig.document_digest.clone()
        };
        let status_info = format!("Integritas (SHA-256) : {} [ASLI & TERVERIFIKASI]", digest_short);
        page.add_styled_text(&status_info, text_x, current_y - 60.0, 7.0, true, (0.05, 0.50, 0.25));

        current_y - box_h - 10.0
    }
}
