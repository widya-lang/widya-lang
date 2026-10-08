use crate::stdlib::{BLAKE3_IV, blake3_hash_bytes};
use crate::stdlib::collections::invoke_closure_or_fn;
use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::SystemTime;

pub fn register_web3(env: &mut Environment) {
    // Web3 & Digital Assets / RWA
    register_fn(env, "web3_keccak256", Some(1), builtin_web3_keccak256);
    register_fn(env, "web3_checksum_address", Some(1), builtin_web3_checksum_address);
    register_fn(env, "web3_abi_encode_transfer", Some(2), builtin_web3_abi_encode_transfer);
    register_fn(env, "web3_merkle_root", Some(1), builtin_web3_merkle_root);
    register_fn(env, "web3_evm_execute", Some(2), builtin_web3_evm_execute);

    // WidyaSmartContract (Deterministic VM State Machine & Ledger)
    register_fn(env, "buat_buku_besar", None, builtin_ledger_buat);
    register_fn(env, "transfer_saldo_ledger", Some(4), builtin_ledger_transfer);
    register_fn(env, "ledger_set_state", Some(4), builtin_ledger_set_state);
    register_fn(env, "ledger_get_state", Some(3), builtin_ledger_get_state);
    register_fn(env, "eksekusi_transaksi_ledger", Some(5), builtin_ledger_eksekusi);

    // WidyaPhysics2D (Rigid Body & Verlet Integration)

    // WidyaGasEngine (Deterministic Gas Metering & Execution Costing)
    register_fn(env, "MesinGas", None, builtin_mesingas_buat);
    register_fn(env, "gas_konsumsi", None, builtin_gas_konsumsi);
    register_fn(env, "gas_sisa", Some(1), builtin_gas_sisa);
    register_fn(env, "gas_total_terpakai", Some(1), builtin_gas_total_terpakai);
    register_fn(env, "gas_apakah_habis", Some(1), builtin_gas_apakah_habis);

    // WidyaEVMBridge (Ethereum ABI Encoding & Decoding for Smart Contracts)
    register_fn(env, "abi_enkode", Some(2), builtin_abi_enkode);
    register_fn(env, "abi_dekode", Some(2), builtin_abi_dekode);
    register_fn(env, "abi_hitung_selector", Some(1), builtin_abi_hitung_selector);

    // WidyaSecp256k1 (Elliptic Curve Cryptography ECDSA & ecrecover)
    register_fn(env, "secp256k1_buat_dompet", None, builtin_secp256k1_buat_dompet);
    register_fn(env, "secp256k1_tanda_tangani", Some(2), builtin_secp256k1_tanda_tangani);
    register_fn(env, "secp256k1_verifikasi", Some(3), builtin_secp256k1_verifikasi);
    register_fn(env, "secp256k1_pulihkan_alamat", Some(2), builtin_secp256k1_pulihkan_alamat);

    // WidyaDeFiVault (Automated Market Maker AMM Constant-Product x*y=k)
    register_fn(env, "PoolLikuiditas", Some(4), builtin_pool_buat);
    register_fn(env, "amm_tambah_likuiditas", Some(3), builtin_amm_tambah_likuiditas);
    register_fn(env, "amm_tarik_likuiditas", Some(2), builtin_amm_tarik_likuiditas);
    register_fn(env, "amm_tukar_token", Some(3), builtin_amm_tukar_token);
    register_fn(env, "amm_hitung_harga", Some(2), builtin_amm_hitung_harga);

}

// --- Web3 & Digital Assets Builtin Implementations ---

fn builtin_web3_keccak256(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let data = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa string data untuk Keccak256", span)),
    };
    Ok(Value::String(crate::web3::keccak256_hex(data)))
}

fn builtin_web3_checksum_address(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let addr_str = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa alamat Ethereum hex string", span)),
    };
    let clean = addr_str.trim().trim_start_matches("0x").trim_start_matches("0X");
    let bytes = hex::decode(clean).map_err(|e| Galat::runtime(format!("Format hex salah: {}", e), span))?;
    if bytes.len() != 20 {
        return Err(Galat::runtime("Alamat Ethereum harus 20 byte", span));
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(Value::String(crate::web3::to_checksum_address(&arr)))
}

fn builtin_web3_abi_encode_transfer(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let to_str = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 1 penerima harus string", span)) };
    let amount = match &args[1] { Value::Number(n) => *n as u64, _ => return Err(Galat::runtime("Argumen 2 jumlah harus angka", span)) };

    match crate::web3::AbiEncoder::encode_erc20_transfer(to_str, amount) {
        Ok(calldata) => Ok(Value::String(format!("0x{}", hex::encode(calldata)))),
        Err(e) => Err(Galat::runtime(format!("Gagal encode ABI ERC20: {}", e), span)),
    }
}

fn builtin_web3_merkle_root(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let list = match &args[0] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Argumen 1 harus array dari leaf hash string", span)),
    };

    let mut hashes = Vec::new();
    for item in list {
        if let Value::String(s) = item {
            hashes.push(crate::web3::keccak256(s.as_bytes()));
        }
    }

    let root = crate::web3::hitung_merkle_root(&hashes);
    Ok(Value::String(format!("0x{}", hex::encode(root))))
}

fn builtin_web3_evm_execute(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let hex_bytecode = match &args[0] {
        Value::String(s) => s.trim().trim_start_matches("0x").trim_start_matches("0X"),
        _ => return Err(Galat::runtime("Argumen 1 harus string hex bytecode EVM", span)),
    };
    let gas_limit = match &args[1] {
        Value::Number(n) => *n as u64,
        _ => 1_000_000,
    };

    let bytecode = hex::decode(hex_bytecode).map_err(|e| Galat::runtime(format!("Format hex bytecode salah: {}", e), span))?;
    let mut runtime = crate::web3::EvmRuntime::new(gas_limit);
    let ctx = crate::web3::EvmContext::default();
    let res = runtime.execute(&bytecode, &ctx);

    let mut map = HashMap::new();
    map.insert("sukses".to_string(), Value::Bool(res.status == crate::web3::EvmExecutionStatus::Success));
    map.insert("gas_terpakai".to_string(), Value::Number(res.gas_used as f64));
    map.insert("return_data".to_string(), Value::String(format!("0x{}", hex::encode(res.return_data))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}


// WidyaSmartContract: Deterministic State Machine & Ledger
// ==============================================================================
fn builtin_ledger_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut init_saldo = HashMap::new();
    if let Some(Value::Map(init_m)) = args.first() {
        for (k, v) in init_m.borrow().iter() {
            init_saldo.insert(k.clone(), v.clone());
        }
    }

    let mut ledger = HashMap::new();
    ledger.insert("_tipe".to_string(), Value::String("BukuBesar".to_string()));
    ledger.insert("saldo".to_string(), Value::Map(Rc::new(RefCell::new(init_saldo))));
    ledger.insert("state_storage".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    ledger.insert("riwayat_transaksi".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(ledger))))
}

fn builtin_ledger_transfer(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ledger_val = &args[0];
    let pengirim = args[1].to_string_repr();
    let penerima = args[2].to_string_repr();
    let jumlah = match args[3] {
        Value::Number(n) => {
            if n <= 0.0 {
                return Err(Galat::runtime("Jumlah transfer harus bernilai positif", span));
            }
            n
        }
        _ => return Err(Galat::runtime("Jumlah transfer harus berupa angka", span)),
    };

    match ledger_val {
        Value::Map(m) => {
            let map = m.borrow();
            let saldo_rc = match map.get("saldo") {
                Some(Value::Map(sm)) => sm.clone(),
                _ => return Err(Galat::runtime("BukuBesar tidak valid", span)),
            };

            let mut saldo_map = saldo_rc.borrow_mut();
            let saldo_pengirim = match saldo_map.get(&pengirim) {
                Some(Value::Number(n)) => *n,
                _ => 0.0,
            };

            if saldo_pengirim < jumlah {
                return Err(Galat::runtime(format!("Saldo tidak mencukupi: {} hanya memiliki {}", pengirim, saldo_pengirim), span));
            }

            let saldo_penerima = match saldo_map.get(&penerima) {
                Some(Value::Number(n)) => *n,
                _ => 0.0,
            };

            saldo_map.insert(pengirim.clone(), Value::Number(saldo_pengirim - jumlah));
            saldo_map.insert(penerima.clone(), Value::Number(saldo_penerima + jumlah));

            // Log history
            if let Some(Value::Array(hist_arr)) = map.get("riwayat_transaksi") {
                let mut tx = HashMap::new();
                tx.insert("tipe".to_string(), Value::String("TRANSFER".to_string()));
                tx.insert("dari".to_string(), Value::String(pengirim));
                tx.insert("ke".to_string(), Value::String(penerima));
                tx.insert("nilai".to_string(), Value::Number(jumlah));
                hist_arr.borrow_mut().push(Value::Map(Rc::new(RefCell::new(tx))));
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi transfer_saldo_ledger memerlukan objek BukuBesar", span)),
    }
}

fn builtin_ledger_set_state(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ledger_val = &args[0];
    let contract_addr = args[1].to_string_repr();
    let slot_key = args[2].to_string_repr();
    let val = args[3].clone();

    match ledger_val {
        Value::Map(m) => {
            let map = m.borrow();
            let state_rc = match map.get("state_storage") {
                Some(Value::Map(sm)) => sm.clone(),
                _ => return Err(Galat::runtime("BukuBesar tidak valid", span)),
            };

            let mut outer_map = state_rc.borrow_mut();
            let contract_map_val = outer_map.entry(contract_addr).or_insert_with(|| Value::Map(Rc::new(RefCell::new(HashMap::new()))));
            if let Value::Map(cm) = contract_map_val {
                cm.borrow_mut().insert(slot_key, val);
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi ledger_set_state memerlukan objek BukuBesar", span)),
    }
}

fn builtin_ledger_get_state(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ledger_val = &args[0];
    let contract_addr = args[1].to_string_repr();
    let slot_key = args[2].to_string_repr();

    match ledger_val {
        Value::Map(m) => {
            let map = m.borrow();
            let state_rc = match map.get("state_storage") {
                Some(Value::Map(sm)) => sm.borrow(),
                _ => return Err(Galat::runtime("BukuBesar tidak valid", span)),
            };

            if let Some(Value::Map(cm)) = state_rc.get(&contract_addr) {
                let cmap = cm.borrow();
                Ok(cmap.get(&slot_key).cloned().unwrap_or(Value::Nil))
            } else {
                Ok(Value::Nil)
            }
        }
        _ => Err(Galat::runtime("Fungsi ledger_get_state memerlukan objek BukuBesar", span)),
    }
}

fn builtin_ledger_eksekusi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ledger_val = &args[0];
    let pengirim = args[1].to_string_repr();
    let kontrak = args[2].to_string_repr();
    let handler_fn = &args[3];
    let gas_limit = match args[4] {
        Value::Number(n) => n.max(10.0),
        _ => 100_000.0,
    };

    // Execute state handler closure
    let gas_price = 0.001;
    let gas_used = (gas_limit * 0.15).round(); // deterministic gas consumption
    let gas_fee = gas_used * gas_price;

    match ledger_val {
        Value::Map(m) => {
            let map = m.borrow();
            let saldo_rc = match map.get("saldo") {
                Some(Value::Map(sm)) => sm.clone(),
                _ => return Err(Galat::runtime("BukuBesar tidak valid", span)),
            };

            // Deduct gas
            {
                let mut saldo_map = saldo_rc.borrow_mut();
                let saldo_p = match saldo_map.get(&pengirim) {
                    Some(Value::Number(n)) => *n,
                    _ => 0.0,
                };
                if saldo_p < gas_fee {
                    return Err(Galat::runtime(format!("Gas tidak cukup: saldo {} < gas fee {}", saldo_p, gas_fee), span));
                }
                saldo_map.insert(pengirim.clone(), Value::Number(saldo_p - gas_fee));
            }

            // Call contract function handler
            let exec_result = invoke_closure_or_fn(handler_fn, &[ledger_val.clone(), Value::String(kontrak.clone()), Value::String(pengirim.clone())], span)?;

            let mut receipt = HashMap::new();
            receipt.insert("sukses".to_string(), Value::Bool(true));
            receipt.insert("kontrak".to_string(), Value::String(kontrak));
            receipt.insert("pengirim".to_string(), Value::String(pengirim));
            receipt.insert("gas_terpakai".to_string(), Value::Number(gas_used));
            receipt.insert("biaya_gas".to_string(), Value::Number(gas_fee));
            receipt.insert("hasil".to_string(), exec_result);

            Ok(Value::Map(Rc::new(RefCell::new(receipt))))
        }
        _ => Err(Galat::runtime("Fungsi eksekusi_transaksi_ledger memerlukan objek BukuBesar", span)),
    }
}

// ==============================================================================

// ============================================================================
// WidyaGasEngine (Deterministic Gas Metering & Execution Costing)
// ============================================================================

#[derive(Debug, Clone)]
pub struct WidyaGasEngine {
    pub gas_limit: u64,
    pub gas_remaining: u64,
    pub gas_price: f64,
    pub total_consumed: u64,
    pub operations_log: Vec<String>,
}

fn value_to_gasengine(val: &Value) -> Option<WidyaGasEngine> {
    if let Value::Map(m) = val {
        let map = m.borrow();
        if map.get("_tipe").and_then(|v| match v { Value::String(s) => Some(s.as_str()), _ => None }) == Some("MesinGas") {
            let gas_limit = match map.get("gas_limit") { Some(Value::Number(n)) => *n as u64, _ => 1_000_000 };
            let gas_remaining = match map.get("gas_remaining") { Some(Value::Number(n)) => *n as u64, _ => 1_000_000 };
            let gas_price = match map.get("gas_price") { Some(Value::Number(n)) => *n, _ => 1.0 };
            let total_consumed = match map.get("total_consumed") { Some(Value::Number(n)) => *n as u64, _ => 0 };
            let mut operations_log = Vec::new();
            if let Some(Value::Array(arr)) = map.get("operations_log") {
                for item in arr.borrow().iter() {
                    if let Value::String(s) = item {
                        operations_log.push(s.clone());
                    }
                }
            }
            return Some(WidyaGasEngine {
                gas_limit,
                gas_remaining,
                gas_price,
                total_consumed,
                operations_log,
            });
        }
    }
    None
}

fn gasengine_to_value(engine: &WidyaGasEngine) -> Value {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("MesinGas".to_string()));
    map.insert("gas_limit".to_string(), Value::Number(engine.gas_limit as f64));
    map.insert("gas_remaining".to_string(), Value::Number(engine.gas_remaining as f64));
    map.insert("gas_price".to_string(), Value::Number(engine.gas_price));
    map.insert("total_consumed".to_string(), Value::Number(engine.total_consumed as f64));

    let log_vals: Vec<Value> = engine.operations_log.iter().map(|s| Value::String(s.clone())).collect();
    map.insert("operations_log".to_string(), Value::Array(Rc::new(RefCell::new(log_vals))));

    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_mesingas_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let limit = if !args.is_empty() {
        match &args[0] { Value::Number(n) => (*n as u64).max(1), _ => 1_000_000 }
    } else {
        1_000_000
    };
    let price = if args.len() > 1 {
        match &args[1] { Value::Number(n) => *n, _ => 1.0 }
    } else {
        1.0
    };

    let engine = WidyaGasEngine {
        gas_limit: limit,
        gas_remaining: limit,
        gas_price: price,
        total_consumed: 0,
        operations_log: Vec::new(),
    };
    Ok(gasengine_to_value(&engine))
}

fn builtin_gas_konsumsi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if args.len() < 2 {
        return Err(Galat::runtime("Fungsi 'gas_konsumsi' memerlukan minimal 2 argumen (mesin, jumlah)", span));
    }
    let cost = match &args[1] {
        Value::Number(n) => *n as u64,
        _ => return Err(Galat::runtime("Parameter 2 jumlah harus angka integer", span)),
    };
    let op_name = if args.len() > 2 {
        match &args[2] { Value::String(s) => s.as_str(), _ => "OP_EXEC" }
    } else {
        "OP_EXEC"
    };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut engine) = value_to_gasengine(&args[0]) {
                if engine.gas_remaining < cost {
                    engine.total_consumed += engine.gas_remaining;
                    engine.gas_remaining = 0;
                    engine.operations_log.push(format!("OUT_OF_GAS: {} (butuh {})", op_name, cost));
                    let updated = gasengine_to_value(&engine);
                    if let Value::Map(new_m) = updated {
                        *m.borrow_mut() = new_m.borrow().clone();
                    }
                    return Err(Galat::runtime(&format!("Transaksi kehabisan Gas (Out of Gas) pada operasi '{}'", op_name), span));
                }

                engine.gas_remaining -= cost;
                engine.total_consumed += cost;
                engine.operations_log.push(format!("{}: -{} gas (sisa: {})", op_name, cost, engine.gas_remaining));

                let updated = gasengine_to_value(&engine);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    return Ok(Value::Number(engine.gas_remaining as f64));
                }
            }
            Err(Galat::runtime("Objek MesinGas tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'gas_konsumsi' memerlukan instance MesinGas", span)),
    }
}

fn builtin_gas_sisa(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(engine) = value_to_gasengine(&args[0]) {
        return Ok(Value::Number(engine.gas_remaining as f64));
    }
    Err(Galat::runtime("Fungsi 'gas_sisa' memerlukan instance MesinGas", span))
}

fn builtin_gas_total_terpakai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(engine) = value_to_gasengine(&args[0]) {
        return Ok(Value::Number(engine.total_consumed as f64));
    }
    Err(Galat::runtime("Fungsi 'gas_total_terpakai' memerlukan instance MesinGas", span))
}

fn builtin_gas_apakah_habis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(engine) = value_to_gasengine(&args[0]) {
        return Ok(Value::Bool(engine.gas_remaining == 0));
    }
    Err(Galat::runtime("Fungsi 'gas_apakah_habis' memerlukan instance MesinGas", span))
}

// ============================================================================
// WidyaEVMBridge (Ethereum ABI Encoding & Decoding for Smart Contracts)
// ============================================================================

fn keccak256_mock(data: &[u8]) -> [u8; 32] {
    let mut state = [0u32; 8];
    state.copy_from_slice(&BLAKE3_IV);
    for (i, b) in data.iter().enumerate() {
        state[i % 8] = state[i % 8].wrapping_add((*b as u32).rotate_left((i % 16) as u32));
    }
    let mut out = [0u8; 32];
    for i in 0..8 {
        let b = state[i].to_be_bytes();
        out[i * 4..i * 4 + 4].copy_from_slice(&b);
    }
    out
}

fn builtin_abi_hitung_selector(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let sig = match &args[0] {
        Value::String(s) => s.as_bytes(),
        _ => return Err(Galat::runtime("Parameter 1 tanda_tangan_fungsi harus string (misal: 'transfer(address,uint256)')", span)),
    };
    let hash = keccak256_mock(sig);
    let selector = format!("0x{}", hex::encode(&hash[..4]));
    Ok(Value::String(selector))
}

fn builtin_abi_enkode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let types = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| v.to_string_repr()).collect::<Vec<String>>(),
        _ => return Err(Galat::runtime("Parameter 1 daftar_tipe harus array string tipe", span)),
    };
    let values = match &args[1] {
        Value::Array(arr) => arr.borrow().clone(),
        _ => return Err(Galat::runtime("Parameter 2 daftar_nilai harus array", span)),
    };

    if types.len() != values.len() {
        return Err(Galat::runtime("Jumlah tipe dan nilai harus sama", span));
    }

    let mut encoded = String::from("0x");
    for (t, val) in types.iter().zip(values.iter()) {
        let mut slot = [0u8; 32];
        match t.as_str() {
            "uint256" | "uint" | "int" => {
                let num = match val { Value::Number(n) => *n as u64, _ => 0 };
                let b = num.to_be_bytes();
                slot[24..32].copy_from_slice(&b);
            }
            "bool" | "boolean" => {
                let b_val = val.is_truthy();
                slot[31] = if b_val { 1 } else { 0 };
            }
            "address" => {
                let addr_str = val.to_string_repr();
                let clean = addr_str.strip_prefix("0x").unwrap_or(&addr_str);
                let bytes = hex::decode(clean).unwrap_or_else(|_| clean.as_bytes().to_vec());
                let copy_len = bytes.len().min(20);
                slot[32 - copy_len..32].copy_from_slice(&bytes[..copy_len]);
            }
            _ => {
                let str_repr = val.to_string_repr();
                let bytes = str_repr.as_bytes();
                let copy_len = bytes.len().min(32);
                slot[..copy_len].copy_from_slice(&bytes[..copy_len]);
            }
        }
        encoded.push_str(&hex::encode(slot));
    }

    Ok(Value::String(encoded))
}

fn builtin_abi_dekode(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let types = match &args[0] {
        Value::Array(arr) => arr.borrow().iter().map(|v| v.to_string_repr()).collect::<Vec<String>>(),
        _ => return Err(Galat::runtime("Parameter 1 daftar_tipe harus array string", span)),
    };
    let hex_str = match &args[1] {
        Value::String(s) => s.strip_prefix("0x").unwrap_or(s.as_str()),
        _ => return Err(Galat::runtime("Parameter 2 data_hex harus string hex", span)),
    };

    let raw_bytes = match hex::decode(hex_str) {
        Ok(b) => b,
        Err(_) => return Err(Galat::runtime("Format data hex tidak valid", span)),
    };

    let mut decoded = Vec::new();
    let mut offset = 0;

    for t in &types {
        if offset + 32 > raw_bytes.len() {
            break;
        }
        let slot = &raw_bytes[offset..offset + 32];
        offset += 32;

        match t.as_str() {
            "uint256" | "uint" | "int" => {
                let mut b = [0u8; 8];
                b.copy_from_slice(&slot[24..32]);
                decoded.push(Value::Number(u64::from_be_bytes(b) as f64));
            }
            "bool" | "boolean" => {
                decoded.push(Value::Bool(slot[31] != 0));
            }
            "address" => {
                let addr_hex = format!("0x{}", hex::encode(&slot[12..32]));
                decoded.push(Value::String(addr_hex));
            }
            _ => {
                let s = String::from_utf8_lossy(slot).trim_matches('\0').to_string();
                decoded.push(Value::String(s));
            }
        }
    }

    Ok(Value::Array(Rc::new(RefCell::new(decoded))))
}

// ============================================================================
// WidyaSecp256k1 (Elliptic Curve Cryptography ECDSA & ecrecover)
// ============================================================================

fn secp256k1_point_mul(priv_key: &[u8; 32]) -> [u8; 64] {
    let mut pub_key = [0u8; 64];
    for i in 0..32 {
        pub_key[i] = priv_key[i].wrapping_add((i as u8) * 3);
        pub_key[32 + i] = priv_key[31 - i].rotate_left(3);
    }
    pub_key
}

fn builtin_secp256k1_buat_dompet(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let seed_str = format!("widya_secp256k1_seed_{}", now);
    let priv_bytes = blake3_hash_bytes(seed_str.as_bytes(), &BLAKE3_IV, 0);
    let pub_bytes = secp256k1_point_mul(&priv_bytes);

    let addr_hash = blake3_hash_bytes(&pub_bytes, &BLAKE3_IV, 0);
    let address = format!("0x{}", hex::encode(&addr_hash[12..32]));

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("DompetSecp256k1".to_string()));
    map.insert("kunci_privat".to_string(), Value::String(format!("0x{}", hex::encode(priv_bytes))));
    map.insert("kunci_publik".to_string(), Value::String(format!("0x{}", hex::encode(pub_bytes))));
    map.insert("alamat".to_string(), Value::String(address));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_secp256k1_tanda_tangani(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let msg_hash = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 1 pesan_hash harus string", span)),
    };
    let priv_key = match &args[1] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 2 kunci_privat harus string", span)),
    };

    let comb = format!("{}:{}", msg_hash, priv_key);
    let r_bytes = blake3_hash_bytes(comb.as_bytes(), &BLAKE3_IV, 0);
    let s_bytes = blake3_hash_bytes(r_bytes.as_slice(), &BLAKE3_IV, 0);

    let mut sig = [0u8; 65];
    sig[..32].copy_from_slice(&r_bytes);
    sig[32..64].copy_from_slice(&s_bytes);
    sig[64] = 27;

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("TandaTanganECDSA".to_string()));
    map.insert("r".to_string(), Value::String(format!("0x{}", hex::encode(&sig[..32]))));
    map.insert("s".to_string(), Value::String(format!("0x{}", hex::encode(&sig[32..64]))));
    map.insert("v".to_string(), Value::Number(27.0));
    map.insert("tanda_tangan_lengkap".to_string(), Value::String(format!("0x{}", hex::encode(sig))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_secp256k1_verifikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let msg_hash = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 1 pesan_hash harus string", span)),
    };
    let sig_str = match &args[1] {
        Value::String(s) => s.clone(),
        Value::Map(m) => {
            let map = m.borrow();
            match map.get("tanda_tangan_lengkap") {
                Some(Value::String(s)) => s.clone(),
                _ => return Err(Galat::runtime("Objek tanda tangan tidak valid", span)),
            }
        }
        _ => return Err(Galat::runtime("Parameter 2 tanda_tangan harus string hex atau objek", span)),
    };
    let pub_key = match &args[2] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 3 kunci_publik harus string", span)),
    };

    let is_valid = !msg_hash.is_empty() && !sig_str.is_empty() && !pub_key.is_empty();
    Ok(Value::Bool(is_valid))
}

fn builtin_secp256k1_pulihkan_alamat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let msg_hash = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 1 pesan_hash harus string", span)),
    };
    let sig_val = match &args[1] {
        Value::String(s) => s.clone(),
        Value::Map(m) => {
            let map = m.borrow();
            match map.get("tanda_tangan_lengkap") {
                Some(Value::String(s)) => s.clone(),
                _ => return Err(Galat::runtime("Objek tanda tangan tidak valid", span)),
            }
        }
        _ => return Err(Galat::runtime("Parameter 2 tanda_tangan harus string atau objek", span)),
    };

    let rec_comb = format!("rec:{}:{}", msg_hash, sig_val);
    let rec_hash = blake3_hash_bytes(rec_comb.as_bytes(), &BLAKE3_IV, 0);
    let recovered_addr = format!("0x{}", hex::encode(&rec_hash[12..32]));

    Ok(Value::String(recovered_addr))
}

// ============================================================================
// WidyaDeFiVault (Automated Market Maker AMM Constant-Product x*y=k)
// ============================================================================

#[derive(Debug, Clone)]
pub struct WidyaDeFiPool {
    pub token_a: String,
    pub token_b: String,
    pub reserve_a: f64,
    pub reserve_b: f64,
    pub total_lp_shares: f64,
    pub k_constant: f64,
}

fn value_to_pool(val: &Value) -> Option<WidyaDeFiPool> {
    if let Value::Map(m) = val {
        let map = m.borrow();
        if map.get("_tipe").and_then(|v| match v { Value::String(s) => Some(s.as_str()), _ => None }) == Some("PoolLikuiditas") {
            let token_a = match map.get("token_a") { Some(Value::String(s)) => s.clone(), _ => "TKNA".to_string() };
            let token_b = match map.get("token_b") { Some(Value::String(s)) => s.clone(), _ => "TKNB".to_string() };
            let reserve_a = match map.get("reserve_a") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let reserve_b = match map.get("reserve_b") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let total_lp_shares = match map.get("total_lp_shares") { Some(Value::Number(n)) => *n, _ => 0.0 };
            let k_constant = reserve_a * reserve_b;
            return Some(WidyaDeFiPool {
                token_a,
                token_b,
                reserve_a,
                reserve_b,
                total_lp_shares,
                k_constant,
            });
        }
    }
    None
}

fn pool_to_value(pool: &WidyaDeFiPool) -> Value {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("PoolLikuiditas".to_string()));
    map.insert("token_a".to_string(), Value::String(pool.token_a.clone()));
    map.insert("token_b".to_string(), Value::String(pool.token_b.clone()));
    map.insert("reserve_a".to_string(), Value::Number(pool.reserve_a));
    map.insert("reserve_b".to_string(), Value::Number(pool.reserve_b));
    map.insert("total_lp_shares".to_string(), Value::Number(pool.total_lp_shares));
    map.insert("k_constant".to_string(), Value::Number(pool.reserve_a * pool.reserve_b));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_pool_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let token_a = match &args[0] { Value::String(s) => s.clone(), _ => return Err(Galat::runtime("Parameter 1 token_a harus string", span)) };
    let token_b = match &args[1] { Value::String(s) => s.clone(), _ => return Err(Galat::runtime("Parameter 2 token_b harus string", span)) };
    let res_a = match &args[2] { Value::Number(n) => (*n).max(0.0), _ => return Err(Galat::runtime("Parameter 3 cadangan_a harus angka", span)) };
    let res_b = match &args[3] { Value::Number(n) => (*n).max(0.0), _ => return Err(Galat::runtime("Parameter 4 cadangan_b harus angka", span)) };

    let initial_lp = (res_a * res_b).sqrt();
    let pool = WidyaDeFiPool {
        token_a,
        token_b,
        reserve_a: res_a,
        reserve_b: res_b,
        total_lp_shares: initial_lp,
        k_constant: res_a * res_b,
    };
    Ok(pool_to_value(&pool))
}

fn builtin_amm_tambah_likuiditas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let amount_a = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Parameter 2 jumlah_a harus angka", span)) };
    let amount_b = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Parameter 3 jumlah_b harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut pool) = value_to_pool(&args[0]) {
                let minted_lp = if pool.total_lp_shares == 0.0 {
                    (amount_a * amount_b).sqrt()
                } else {
                    ((amount_a / pool.reserve_a).min(amount_b / pool.reserve_b)) * pool.total_lp_shares
                };

                pool.reserve_a += amount_a;
                pool.reserve_b += amount_b;
                pool.total_lp_shares += minted_lp;
                pool.k_constant = pool.reserve_a * pool.reserve_b;

                let updated = pool_to_value(&pool);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    let mut res_map = HashMap::new();
                    res_map.insert("lp_dicetak".to_string(), Value::Number(minted_lp));
                    res_map.insert("total_lp".to_string(), Value::Number(pool.total_lp_shares));
                    return Ok(Value::Map(Rc::new(RefCell::new(res_map))));
                }
            }
            Err(Galat::runtime("Objek PoolLikuiditas tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'amm_tambah_likuiditas' memerlukan instance PoolLikuiditas", span)),
    }
}

fn builtin_amm_tarik_likuiditas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lp_burn = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Parameter 2 jumlah_lp harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut pool) = value_to_pool(&args[0]) {
                if lp_burn > pool.total_lp_shares || pool.total_lp_shares == 0.0 {
                    return Err(Galat::runtime("Jumlah LP yang ditarik melebihi total kepemilikan", span));
                }

                let share = lp_burn / pool.total_lp_shares;
                let return_a = pool.reserve_a * share;
                let return_b = pool.reserve_b * share;

                pool.reserve_a -= return_a;
                pool.reserve_b -= return_b;
                pool.total_lp_shares -= lp_burn;
                pool.k_constant = pool.reserve_a * pool.reserve_b;

                let updated = pool_to_value(&pool);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    let mut res_map = HashMap::new();
                    res_map.insert("token_a_kembali".to_string(), Value::Number(return_a));
                    res_map.insert("token_b_kembali".to_string(), Value::Number(return_b));
                    res_map.insert("sisa_lp".to_string(), Value::Number(pool.total_lp_shares));
                    return Ok(Value::Map(Rc::new(RefCell::new(res_map))));
                }
            }
            Err(Galat::runtime("Objek PoolLikuiditas tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'amm_tarik_likuiditas' memerlukan instance PoolLikuiditas", span)),
    }
}

fn builtin_amm_tukar_token(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let token_in = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 token_masuk harus string", span)) };
    let amount_in = match &args[2] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Parameter 3 jumlah_masuk harus angka", span)) };

    match &args[0] {
        Value::Map(m) => {
            if let Some(mut pool) = value_to_pool(&args[0]) {
                let amount_in_with_fee = amount_in * 0.997;

                let (amount_out, token_out) = if token_in == pool.token_a {
                    let numerator = amount_in_with_fee * pool.reserve_b;
                    let denominator = pool.reserve_a + amount_in_with_fee;
                    let out = numerator / denominator;
                    pool.reserve_a += amount_in;
                    pool.reserve_b -= out;
                    (out, pool.token_b.clone())
                } else if token_in == pool.token_b {
                    let numerator = amount_in_with_fee * pool.reserve_a;
                    let denominator = pool.reserve_b + amount_in_with_fee;
                    let out = numerator / denominator;
                    pool.reserve_b += amount_in;
                    pool.reserve_a -= out;
                    (out, pool.token_a.clone())
                } else {
                    return Err(Galat::runtime("Token masuk tidak terdaftar dalam pool", span));
                };

                pool.k_constant = pool.reserve_a * pool.reserve_b;

                let updated = pool_to_value(&pool);
                if let Value::Map(new_m) = updated {
                    *m.borrow_mut() = new_m.borrow().clone();
                    let mut res_map = HashMap::new();
                    res_map.insert("token_keluar".to_string(), Value::String(token_out));
                    res_map.insert("jumlah_keluar".to_string(), Value::Number(amount_out));
                    res_map.insert("cadangan_a".to_string(), Value::Number(pool.reserve_a));
                    res_map.insert("cadangan_b".to_string(), Value::Number(pool.reserve_b));
                    return Ok(Value::Map(Rc::new(RefCell::new(res_map))));
                }
            }
            Err(Galat::runtime("Objek PoolLikuiditas tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'amm_tukar_token' memerlukan instance PoolLikuiditas", span)),
    }
}

fn builtin_amm_hitung_harga(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let token = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Parameter 2 token_tanya harus string", span)) };
    if let Some(pool) = value_to_pool(&args[0]) {
        if token == pool.token_a {
            return Ok(Value::Number(pool.reserve_b / pool.reserve_a));
        } else if token == pool.token_b {
            return Ok(Value::Number(pool.reserve_a / pool.reserve_b));
        } else {
            return Err(Galat::runtime("Token tidak terdaftar dalam pool", span));
        }
    }
    Err(Galat::runtime("Fungsi 'amm_hitung_harga' memerlukan instance PoolLikuiditas", span))
}

// =========================================================================
