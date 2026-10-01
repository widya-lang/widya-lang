use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardNetwork {
    Visa,
    MasterCard,
    Jcb,
    AmericanExpress,
    Gpn,
    Unknown,
}

/// Validasi Nomor Kartu menggunakan Algoritma Luhn (Mod 10)
pub fn validasi_luhn(pan: &str) -> bool {
    let clean_pan: String = pan.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean_pan.len() < 12 || clean_pan.len() > 19 {
        return false;
    }

    let mut sum = 0;
    let mut double = false;

    for c in clean_pan.chars().rev() {
        let mut digit = c.to_digit(10).unwrap();
        if double {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        sum += digit;
        double = !double;
    }

    sum % 10 == 0
}

/// Mendeteksi Jaringan Kartu Pembayaran berdasarkan BIN / IIN
pub fn deteksi_jaringan_kartu(pan: &str) -> CardNetwork {
    let clean_pan: String = pan.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean_pan.is_empty() {
        return CardNetwork::Unknown;
    }

    if clean_pan.starts_with('4') {
        CardNetwork::Visa
    } else if clean_pan.starts_with("34") || clean_pan.starts_with("37") {
        CardNetwork::AmericanExpress
    } else if clean_pan.starts_with("35") {
        CardNetwork::Jcb
    } else if clean_pan.starts_with("001") || clean_pan.starts_with("002") || clean_pan.starts_with("003") {
        CardNetwork::Gpn
    } else if let Ok(first2) = clean_pan[0..2.min(clean_pan.len())].parse::<u32>() {
        if (51..=55).contains(&first2) {
            return CardNetwork::MasterCard;
        }
        if clean_pan.len() >= 4 {
            if let Ok(first4) = clean_pan[0..4].parse::<u32>() {
                if (2221..=2720).contains(&first4) {
                    return CardNetwork::MasterCard;
                }
            }
        }
        CardNetwork::Unknown
    } else {
        CardNetwork::Unknown
    }
}

/// Melakukan Masking nomor PAN sesuai standar PCI-DSS (misal: 4111-22XX-XXXX-3344)
pub fn mask_pan(pan: &str) -> String {
    let clean_pan: String = pan.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean_pan.len() < 10 {
        return "*".repeat(clean_pan.len());
    }

    let first6 = &clean_pan[0..6];
    let last4 = &clean_pan[clean_pan.len() - 4..];
    let masked_middle = "X".repeat(clean_pan.len() - 10);

    let full_masked = format!("{}{}{}", first6, masked_middle, last4);
    
    // Format ke kelompok 4 digit
    full_masked
        .as_bytes()
        .chunks(4)
        .map(|chunk| std::str::from_utf8(chunk).unwrap_or(""))
        .collect::<Vec<&str>>()
        .join("-")
}

/// Tokenizer Vault untuk penyimpanan kartu PCI-DSS secara aman (AES-256-GCM)
pub struct CardTokenizer {
    vault_key: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCardVault {
    pub token: String,
    pub masked_pan: String,
    pub network: CardNetwork,
    pub expiry: String, // MM/YY
    pub encrypted_payload_hex: String,
    pub nonce_hex: String,
}

impl CardTokenizer {
    pub fn new(vault_key_hex: &str) -> Result<Self, String> {
        let key_bytes = hex::decode(vault_key_hex).map_err(|e| format!("Kunci Vault Hex tidak sah: {}", e))?;
        if key_bytes.len() != 32 {
            return Err("Kunci Vault harus tepat 32 byte (256-bit)".to_string());
        }
        let mut key = [0u8; 32];
        key.copy_from_slice(&key_bytes);
        Ok(Self { vault_key: key })
    }

    /// Menghasilkan token deterministik dan mengenkripsi payload kartu mentah
    pub fn tokenize(
        &self,
        pan: &str,
        expiry: &str,
        cvv: &str,
    ) -> Result<EncryptedCardVault, String> {
        if !validasi_luhn(pan) {
            return Err("Nomor kartu gagal validasi algoritma Luhn (Mod 10)".to_string());
        }

        let clean_pan: String = pan.chars().filter(|c| c.is_ascii_digit()).collect();
        let network = deteksi_jaringan_kartu(&clean_pan);
        let masked = mask_pan(&clean_pan);
        let token = format!("tok_card_{}", uuid::Uuid::new_v4().simple());

        let raw_payload = serde_json::json!({
            "pan": clean_pan,
            "expiry": expiry,
            "cvv": cvv,
            "token": token
        }).to_string();

        let cipher = Aes256Gcm::new_from_slice(&self.vault_key)
            .map_err(|e| format!("Gagal inisialisasi AES-GCM: {}", e))?;

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, raw_payload.as_bytes())
            .map_err(|e| format!("Gagal enkripsi payload vault: {}", e))?;

        Ok(EncryptedCardVault {
            token,
            masked_pan: masked,
            network,
            expiry: expiry.to_string(),
            encrypted_payload_hex: hex::encode(ciphertext),
            nonce_hex: hex::encode(nonce_bytes),
        })
    }

    /// Melakukan dekripsi vault kartu (hanya untuk charge perbankan)
    pub fn detokenize(&self, vault: &EncryptedCardVault) -> Result<serde_json::Value, String> {
        let ciphertext = hex::decode(&vault.encrypted_payload_hex)
            .map_err(|e| format!("Ciphertext Hex tidak valid: {}", e))?;
        let nonce_bytes = hex::decode(&vault.nonce_hex)
            .map_err(|e| format!("Nonce Hex tidak valid: {}", e))?;

        let cipher = Aes256Gcm::new_from_slice(&self.vault_key)
            .map_err(|e| format!("Gagal inisialisasi AES-GCM: {}", e))?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        let plaintext_bytes = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| format!("Gagal dekripsi token vault: {}", e))?;

        let val: serde_json::Value = serde_json::from_slice(&plaintext_bytes)
            .map_err(|e| format!("Format JSON payload vault rusak: {}", e))?;

        Ok(val)
    }
}

/// Menghasilkan ISO 9564 Format 0 PIN Block (XOR PIN Block Plaintext dan PAN Block)
pub fn generate_iso9564_format0_pin_block(pin: &str, pan: &str) -> Result<String, String> {
    if pin.len() < 4 || pin.len() > 12 {
        return Err("Panjang PIN harus antara 4 hingga 12 digit".to_string());
    }
    let clean_pan: String = pan.chars().filter(|c| c.is_ascii_digit()).collect();
    if clean_pan.len() < 13 {
        return Err("Panjang PAN tidak mencukupi untuk PIN Block".to_string());
    }

    // PIN Field Block: 0 + L + PIN + F's padding to 16 hex chars
    let mut pin_block_str = format!("0{:X}{}", pin.len(), pin);
    while pin_block_str.len() < 16 {
        pin_block_str.push('F');
    }

    // PAN Field Block: 0000 + 12 digit PAN (tidak termasuk check digit terakhir)
    let pan_12_digits = &clean_pan[clean_pan.len() - 13..clean_pan.len() - 1];
    let pan_block_str = format!("0000{}", pan_12_digits);

    let pin_bytes = hex::decode(&pin_block_str).map_err(|e| format!("PIN hex error: {}", e))?;
    let pan_bytes = hex::decode(&pan_block_str).map_err(|e| format!("PAN hex error: {}", e))?;

    let mut result_bytes = [0u8; 8];
    for i in 0..8 {
        result_bytes[i] = pin_bytes[i] ^ pan_bytes[i];
    }

    Ok(hex::encode(result_bytes).to_uppercase())
}
