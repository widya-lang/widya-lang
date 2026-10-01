use crate::web3::crypto::keccak256;

/// Menghitung 4-byte Function Selector dari signature fungsi Solidity
/// Contoh: "transfer(address,uint256)" -> [0xa9, 0x05, 0x9c, 0xbb]
pub fn hitung_function_selector(signature: &str) -> [u8; 4] {
    let hash = keccak256(signature.as_bytes());
    let mut selector = [0u8; 4];
    selector.copy_from_slice(&hash[0..4]);
    selector
}

/// Menghitung 32-byte Event Topic Hash dari signature event Solidity
/// Contoh: "Transfer(address,address,uint256)" -> 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
pub fn hitung_event_topic(event_signature: &str) -> [u8; 32] {
    keccak256(event_signature.as_bytes())
}

/// Enkoder Tipe Data Ethereum ABI (32-byte Word Alignment)
pub struct AbiEncoder;

impl AbiEncoder {
    /// Enkode uint256 / u64 ke dalam 32 byte big-endian
    pub fn encode_uint256(val: u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        let bytes = val.to_be_bytes();
        out[24..32].copy_from_slice(&bytes);
        out
    }

    /// Enkode address (20 byte) menjadi 32 byte dengan 12 byte padding nol di depan
    pub fn encode_address(addr: &[u8; 20]) -> [u8; 32] {
        let mut out = [0u8; 32];
        out[12..32].copy_from_slice(addr);
        out
    }

    /// Enkode address dari string hex 0x...
    pub fn encode_address_str(addr_str: &str) -> Result<[u8; 32], String> {
        let clean = addr_str.trim().trim_start_matches("0x").trim_start_matches("0X");
        let decoded = hex::decode(clean).map_err(|e| format!("Format hex address salah: {}", e))?;
        if decoded.len() != 20 {
            return Err("Address harus sepanjang 20 byte (40 karakter hex)".to_string());
        }
        let mut addr_20 = [0u8; 20];
        addr_20.copy_from_slice(&decoded);
        Ok(Self::encode_address(&addr_20))
    }

    /// Enkode boolean ke 32 byte
    pub fn encode_bool(b: bool) -> [u8; 32] {
        let mut out = [0u8; 32];
        if b {
            out[31] = 1;
        }
        out
    }

    /// Membentuk Calldata Panggilan Kontrak ERC-20 `transfer(to, amount)`
    pub fn encode_erc20_transfer(to_address_str: &str, amount: u64) -> Result<Vec<u8>, String> {
        let selector = hitung_function_selector("transfer(address,uint256)");
        let encoded_to = Self::encode_address_str(to_address_str)?;
        let encoded_amount = Self::encode_uint256(amount);

        let mut calldata = Vec::with_capacity(4 + 32 + 32);
        calldata.extend_from_slice(&selector);
        calldata.extend_from_slice(&encoded_to);
        calldata.extend_from_slice(&encoded_amount);

        Ok(calldata)
    }

    /// Membentuk Calldata Panggilan Kontrak ERC-20 `balanceOf(owner)`
    pub fn encode_erc20_balance_of(owner_address_str: &str) -> Result<Vec<u8>, String> {
        let selector = hitung_function_selector("balanceOf(address)");
        let encoded_owner = Self::encode_address_str(owner_address_str)?;

        let mut calldata = Vec::with_capacity(4 + 32);
        calldata.extend_from_slice(&selector);
        calldata.extend_from_slice(&encoded_owner);

        Ok(calldata)
    }
}

/// Dekoder Tipe Data Ethereum ABI
pub struct AbiDecoder;

impl AbiDecoder {
    /// Dekode uint256 dari 32 byte data keluaran
    pub fn decode_uint256(data: &[u8]) -> Result<u64, String> {
        if data.len() < 32 {
            return Err("Data ABI terlalu pendek (kurang dari 32 byte)".to_string());
        }
        let mut num_bytes = [0u8; 8];
        num_bytes.copy_from_slice(&data[24..32]);
        Ok(u64::from_be_bytes(num_bytes))
    }

    /// Dekode address dari 32 byte data keluaran
    pub fn decode_address(data: &[u8]) -> Result<String, String> {
        if data.len() < 32 {
            return Err("Data ABI terlalu pendek untuk address".to_string());
        }
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&data[12..32]);
        Ok(crate::web3::crypto::to_checksum_address(&addr))
    }

    /// Dekode boolean dari 32 byte
    pub fn decode_bool(data: &[u8]) -> Result<bool, String> {
        if data.len() < 32 {
            return Err("Data ABI terlalu pendek untuk bool".to_string());
        }
        Ok(data[31] != 0)
    }
}
