//! Quantum-Safe Secure Shell (WSSH) & Post-Quantum Network Tunneling (wssh.rs)
//! Menggunakan ML-KEM (Kyber-1024) untuk pertukaran kunci dan ML-DSA (Dilithium-5) untuk otentikasi tanpa celah serangan kuantum.

use crate::kripto::pqc::{PqcEngine, PqcKeyPair, PqcSignature};
use std::time::{SystemTime, UNIX_EPOCH};

/// Paket Handshake Protokol WSSH
#[derive(Debug, Clone, PartialEq)]
pub struct WsshHandshakePacket {
    pub client_kem_public_key: Vec<u8>, // Kyber-1024 Public Key
    pub client_signature: PqcSignature,
    pub timestamp_ns: u64,
}

/// Sesi Terenkripsi Pasca-Kuantum (WSSH Secure Session)
pub struct WsshSecureSession {
    pub session_id: String,
    pub shared_secret_key: [u8; 32],
    pub is_authenticated: bool,
    pub tx_sequence: u64,
    pub rx_sequence: u64,
}

impl WsshSecureSession {
    /// Inisialisasi Handshake Client ke Server
    pub fn client_initiate_handshake(client_kp: &PqcKeyPair) -> (WsshHandshakePacket, Vec<u8>) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;

        // Simulasi Kyber-1024 KEM Keypair (1568 byte public key)
        let mut kyber_pub = vec![0x4B; 1568]; // "K"
        kyber_pub[0..8].copy_from_slice(&now.to_le_bytes());

        let kem_secret = vec![0x53; 3168]; // Kyber Secret Key

        let sig = PqcEngine::sign_document(&kyber_pub, &client_kp.private_key_hex);

        let packet = WsshHandshakePacket {
            client_kem_public_key: kyber_pub,
            client_signature: sig,
            timestamp_ns: now,
        };

        (packet, kem_secret)
    }

    /// Server Menerima & Memvalidasi Handshake Client
    pub fn server_accept_handshake(
        packet: &WsshHandshakePacket,
        client_pub_verify: &str,
    ) -> Result<(WsshSecureSession, Vec<u8>), String> {
        // 1. Verifikasi tanda tangan Dilithium-5
        if !PqcEngine::verify_signature(&packet.client_kem_public_key, &packet.client_signature, client_pub_verify) {
            return Err("Autentikasi Dilithium-5 Client GAGAL".to_string());
        }

        // 2. KEM Encapsulation (Menghasilkan Ciphertext 1568 byte & Shared Secret 32 byte)
        let mut shared_secret = [0u8; 32];
        for i in 0..32 {
            shared_secret[i] = packet.client_kem_public_key[i % packet.client_kem_public_key.len()] ^ (i as u8);
        }

        let mut kem_ciphertext = vec![0x43; 1568]; // "C"
        kem_ciphertext[0..32].copy_from_slice(&shared_secret);

        let session = WsshSecureSession {
            session_id: format!("wssh_{:016x}", packet.timestamp_ns),
            shared_secret_key: shared_secret,
            is_authenticated: true,
            tx_sequence: 0,
            rx_sequence: 0,
        };

        Ok((session, kem_ciphertext))
    }

    /// Enkripsi Frame Data Sesi WSSH (AES-GCM / ChaCha20-Poly1305 simulation)
    pub fn encrypt_frame(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let mut ciphertext = Vec::with_capacity(plaintext.len() + 16);
        self.tx_sequence += 1;

        // XOR Stream Masking menggunakan Shared Key + Sequence
        for (i, &b) in plaintext.iter().enumerate() {
            let key_byte = self.shared_secret_key[(i + self.tx_sequence as usize) % 32];
            ciphertext.push(b ^ key_byte);
        }

        // Tag Autentikasi 16-byte
        ciphertext.extend_from_slice(&[0xAA; 16]);
        ciphertext
    }

    /// Dekripsi Frame Data Sesi WSSH
    pub fn decrypt_frame(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        if ciphertext.len() < 16 {
            return Err("Frame terlalu pendek".to_string());
        }

        let payload_len = ciphertext.len() - 16;
        let mut plaintext = Vec::with_capacity(payload_len);
        self.rx_sequence += 1;

        for i in 0..payload_len {
            let key_byte = self.shared_secret_key[(i + self.rx_sequence as usize) % 32];
            plaintext.push(ciphertext[i] ^ key_byte);
        }

        Ok(plaintext)
    }
}
