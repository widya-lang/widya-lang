use sha3::{Digest, Keccak256};

/// Menghitung Hash Keccak-256 standar Ethereum dari byte slice
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

/// Menghitung Hash Keccak-256 dan mengembalikan representasi string Hexadecimal dengan prefix 0x
pub fn keccak256_hex(data: &[u8]) -> String {
    format!("0x{}", hex::encode(keccak256(data)))
}

/// Menghasilkan Alamat Ethereum (20-byte address) dari 64-byte uncompressed public key (tanpa 0x04 prefix)
pub fn public_key_to_address(pubkey_64: &[u8]) -> [u8; 20] {
    let hash = keccak256(pubkey_64);
    let mut addr = [0u8; 20];
    addr.copy_from_slice(&hash[12..32]);
    addr
}

/// Format Alamat Ethereum dengan Standar Checksum EIP-55 (Mixed-case)
pub fn to_checksum_address(address_bytes: &[u8; 20]) -> String {
    let hex_addr = hex::encode(address_bytes).to_lowercase();
    let hash = hex::encode(keccak256(hex_addr.as_bytes()));

    let mut checksum_addr = String::with_capacity(42);
    checksum_addr.push_str("0x");

    for (i, c) in hex_addr.chars().enumerate() {
        if c.is_alphabetic() {
            let hash_nibble = u8::from_str_radix(&hash[i..i + 1], 16).unwrap_or(0);
            if hash_nibble >= 8 {
                checksum_addr.push(c.to_ascii_uppercase());
            } else {
                checksum_addr.push(c);
            }
        } else {
            checksum_addr.push(c);
        }
    }

    checksum_addr
}

/// Memvalidasi format alamat Ethereum (apakah panjang 42 karakter dan diawali 0x)
pub fn is_valid_ethereum_address(address_str: &str) -> bool {
    let clean = address_str.trim();
    if !clean.starts_with("0x") && !clean.starts_with("0X") {
        return false;
    }
    if clean.len() != 42 {
        return false;
    }
    clean[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Menghitung Root Hash Merkle Tree untuk Verifikasi Bukti Cadangan (Proof of Reserve / RWA)
pub fn hitung_merkle_root(leaf_hashes: &[[u8; 32]]) -> [u8; 32] {
    if leaf_hashes.is_empty() {
        return [0u8; 32];
    }
    if leaf_hashes.len() == 1 {
        return leaf_hashes[0];
    }

    let mut current_level = leaf_hashes.to_vec();
    while current_level.len() > 1 {
        let mut next_level = Vec::new();
        for chunk in current_level.chunks(2) {
            if chunk.len() == 2 {
                let mut combined = Vec::with_capacity(64);
                // Urutkan secara leksikografis untuk determinisme
                if chunk[0] <= chunk[1] {
                    combined.extend_from_slice(&chunk[0]);
                    combined.extend_from_slice(&chunk[1]);
                } else {
                    combined.extend_from_slice(&chunk[1]);
                    combined.extend_from_slice(&chunk[0]);
                }
                next_level.push(keccak256(&combined));
            } else {
                // Elemen ganjil diduplikasi
                let mut combined = Vec::with_capacity(64);
                combined.extend_from_slice(&chunk[0]);
                combined.extend_from_slice(&chunk[0]);
                next_level.push(keccak256(&combined));
            }
        }
        current_level = next_level;
    }

    current_level[0]
}
