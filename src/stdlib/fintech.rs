use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn register_fintech(env: &mut Environment) {
    // WidyaPay (Turnkey Fintech, QRIS, SNAP BI Open API, ISO 8583 & PCI-DSS)
    register_fn(env, "qris_buat_dinamis", Some(3), builtin_fintech_qris_dinamis);
    register_fn(env, "qris_validasi", Some(1), builtin_fintech_qris_validasi);
    register_fn(env, "qris_urai", Some(1), builtin_fintech_qris_urai);
    register_fn(env, "snap_simetris_signature", Some(5), builtin_fintech_snap_signature);
    register_fn(env, "snap_verifikasi_signature", Some(6), builtin_fintech_snap_verifikasi);
    register_fn(env, "snap_timestamp", Some(0), builtin_fintech_snap_timestamp);
    register_fn(env, "kartu_validasi_luhn", Some(1), builtin_fintech_kartu_luhn);
    register_fn(env, "kartu_deteksi_jaringan", Some(1), builtin_fintech_kartu_jaringan);
    register_fn(env, "kartu_masking", Some(1), builtin_fintech_kartu_masking);
    register_fn(env, "kartu_pin_block_iso9564", Some(2), builtin_fintech_kartu_pin_block);
    register_fn(env, "pembayaran_hitung_fee_split", Some(4), builtin_fintech_fee_split);
    register_fn(env, "pembayaran_evaluasi_fraud", Some(1), builtin_fintech_evaluasi_fraud);
    register_fn(env, "iso8583_kemas", Some(2), builtin_fintech_iso8583_kemas);
    register_fn(env, "iso8583_urai", Some(1), builtin_fintech_iso8583_urai);
}

// --- WidyaPay / Fintech Standard Library Implementations ---

fn builtin_fintech_qris_dinamis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let static_qris = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa string QRIS statis", span)),
    };
    let nominal = match &args[1] {
        Value::Number(n) => *n,
        _ => return Err(Galat::runtime("Argumen 2 harus berupa angka nominal pembayaran", span)),
    };
    let invoice_id = match &args[2] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Argumen 3 harus berupa string ID Invoice", span)),
    };

    match crate::fintech::konversi_statis_ke_dinamis(static_qris, nominal, invoice_id) {
        Ok(dyn_qris) => Ok(Value::String(dyn_qris)),
        Err(e) => Err(Galat::runtime(format!("Gagal konversi QRIS: {}", e), span)),
    }
}

fn builtin_fintech_qris_validasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let raw_qris = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa string QRIS", span)),
    };
    Ok(Value::Bool(crate::fintech::validasi_qris_crc16(raw_qris)))
}

fn builtin_fintech_qris_urai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let raw_qris = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Argumen 1 harus berupa string QRIS", span)),
    };

    match crate::fintech::parse_qris(raw_qris) {
        Ok(payload) => {
            let mut map = HashMap::new();
            map.insert("nama_merchant".to_string(), Value::String(payload.merchant_name));
            map.insert("kota_merchant".to_string(), Value::String(payload.merchant_city));
            map.insert("mcc".to_string(), Value::String(payload.merchant_category_code));
            map.insert("mata_uang".to_string(), Value::String(payload.currency_code));
            map.insert("tipe_qris".to_string(), Value::String(if payload.point_of_initiation == "12" { "DINAMIS".to_string() } else { "STATIS".to_string() }));
            map.insert("crc16".to_string(), Value::String(payload.crc16));
            if let Some(amt) = payload.transaction_amount {
                map.insert("nominal".to_string(), Value::Number(amt));
            }
            if let Some(nmid) = payload.national_merchant_id {
                map.insert("nmid".to_string(), Value::String(nmid));
            }
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Err(Galat::runtime(format!("Gagal mengurai QRIS: {}", e), span)),
    }
}

fn builtin_fintech_snap_signature(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let method = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 1 harus berupa metode HTTP", span)) };
    let endpoint = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 2 harus berupa endpoint URL", span)) };
    let token = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 3 harus berupa access token", span)) };
    let body = match &args[3] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 4 harus berupa string JSON body", span)) };
    let secret = match &args[4] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 5 harus berupa client secret", span)) };

    let timestamp = crate::fintech::generate_snap_timestamp();
    let string_to_sign = crate::fintech::bentuk_snap_string_to_sign(method, endpoint, token, body, &timestamp);
    match crate::fintech::buat_snap_simetris_signature(&string_to_sign, secret) {
        Ok(sig) => {
            let mut res = HashMap::new();
            res.insert("signature".to_string(), Value::String(sig));
            res.insert("timestamp".to_string(), Value::String(timestamp));
            res.insert("string_to_sign".to_string(), Value::String(string_to_sign));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        Err(e) => Err(Galat::runtime(format!("Gagal membuat tanda tangan SNAP: {}", e), span)),
    }
}

fn builtin_fintech_snap_verifikasi(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let method = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 1 harus berupa metode HTTP", span)) };
    let endpoint = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 2 harus berupa endpoint URL", span)) };
    let token = match &args[2] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 3 harus berupa access token", span)) };
    let body = match &args[3] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 4 harus berupa string JSON body", span)) };
    let timestamp = match &args[4] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 5 harus berupa timestamp", span)) };
    let sig = match &args[5] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 6 harus berupa signature", span)) };
    let secret = match args.get(6) {
        Some(Value::String(s)) => s.as_str(),
        _ => return Err(Galat::runtime("Argumen 7 harus berupa client secret", span)),
    };

    let string_to_sign = crate::fintech::bentuk_snap_string_to_sign(method, endpoint, token, body, timestamp);
    let valid = crate::fintech::verifikasi_snap_simetris_signature(&string_to_sign, secret, sig);
    Ok(Value::Bool(valid))
}

fn builtin_fintech_snap_timestamp(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    Ok(Value::String(crate::fintech::generate_snap_timestamp()))
}

fn builtin_fintech_kartu_luhn(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pan = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen harus nomor kartu PAN", span)) };
    Ok(Value::Bool(crate::fintech::validasi_luhn(pan)))
}

fn builtin_fintech_kartu_jaringan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pan = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen harus nomor kartu PAN", span)) };
    let net = match crate::fintech::deteksi_jaringan_kartu(pan) {
        crate::fintech::CardNetwork::Visa => "VISA",
        crate::fintech::CardNetwork::MasterCard => "MASTERCARD",
        crate::fintech::CardNetwork::Jcb => "JCB",
        crate::fintech::CardNetwork::AmericanExpress => "AMEX",
        crate::fintech::CardNetwork::Gpn => "GPN",
        crate::fintech::CardNetwork::Unknown => "TIDAK_DIKETAHUI",
    };
    Ok(Value::String(net.to_string()))
}

fn builtin_fintech_kartu_masking(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pan = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen harus nomor kartu PAN", span)) };
    Ok(Value::String(crate::fintech::mask_pan(pan)))
}

fn builtin_fintech_kartu_pin_block(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let pin = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 1 harus PIN", span)) };
    let pan = match &args[1] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 2 harus PAN", span)) };
    match crate::fintech::generate_iso9564_format0_pin_block(pin, pan) {
        Ok(pb) => Ok(Value::String(pb)),
        Err(e) => Err(Galat::runtime(format!("Gagal membuat PIN block: {}", e), span)),
    }
}

fn builtin_fintech_fee_split(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let gross = match &args[0] { Value::Number(n) => *n as i64, _ => return Err(Galat::runtime("Argumen 1 harus gross amount (angka)", span)) };
    let mdr = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("Argumen 2 harus MDR percent rate (angka)", span)) };
    let fixed = match &args[2] { Value::Number(n) => *n as i64, _ => return Err(Galat::runtime("Argumen 3 harus fixed fee (angka)", span)) };
    let bank = match &args[3] { Value::Number(n) => *n as i64, _ => return Err(Galat::runtime("Argumen 4 harus bank switching fee (angka)", span)) };

    let split = crate::fintech::hitung_fee_split(gross, mdr, fixed, bank);
    let mut map = HashMap::new();
    map.insert("gross_amount".to_string(), Value::Number(split.gross_amount as f64));
    map.insert("total_fee".to_string(), Value::Number(split.total_fee as f64));
    map.insert("ppn_tax".to_string(), Value::Number(split.vat_tax as f64));
    map.insert("platform_net_revenue".to_string(), Value::Number(split.platform_net_revenue as f64));
    map.insert("bank_switching_fee".to_string(), Value::Number(split.bank_switching_fee as f64));
    map.insert("merchant_net_payout".to_string(), Value::Number(split.merchant_net_payout as f64));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_fintech_evaluasi_fraud(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let map = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => return Err(Galat::runtime("Argumen harus berupa Map konteks transaksi", span)),
    };

    let amount = match map.get("nominal") { Some(Value::Number(n)) => *n as i64, _ => 0 };
    let ip = match map.get("ip") { Some(Value::String(s)) => s.clone(), _ => "127.0.0.1".to_string() };
    let velocity = match map.get("frekuensi_10m") { Some(Value::Number(n)) => *n as u32, _ => 1 };
    let failed = match map.get("gagal_1jam") { Some(Value::Number(n)) => *n as u32, _ => 0 };
    let is_vpn = match map.get("apakah_vpn") { Some(Value::Bool(b)) => *b, _ => false };
    let card_cc = match map.get("negara_kartu") { Some(Value::String(s)) => Some(s.clone()), _ => None };
    let merchant_cc = match map.get("negara_merchant") { Some(Value::String(s)) => s.clone(), _ => "ID".to_string() };

    let ctx = crate::fintech::TransactionRiskContext {
        pan: None,
        amount_idr: amount,
        ip_address: ip,
        user_agent: "WidyaBrowser/1.0".to_string(),
        velocity_tx_count_last_10m: velocity,
        failed_attempts_last_hour: failed,
        is_vpn_or_proxy: is_vpn,
        card_country: card_cc,
        merchant_country: merchant_cc,
    };

    let res = crate::fintech::evaluasi_risiko_transaksi(&ctx);
    let mut out = HashMap::new();
    out.insert("skor_risiko".to_string(), Value::Number(res.total_risk_score as f64));
    let dec_str = match res.decision {
        crate::fintech::FdsDecision::Approve => "TERIMA",
        crate::fintech::FdsDecision::Challenge => "TANTANGAN_OTP",
        crate::fintech::FdsDecision::Reject => "TOLAK",
    };
    out.insert("keputusan".to_string(), Value::String(dec_str.to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(out))))
}

fn builtin_fintech_iso8583_kemas(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let mti = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 1 harus string MTI", span)) };
    let map = match &args[1] {
        Value::Map(m) => m.borrow().clone(),
        _ => return Err(Galat::runtime("Argumen 2 harus Map field ISO 8583", span)),
    };

    let mut msg = crate::fintech::Iso8583Message::new(mti);
    for (k, v) in map {
        if let Ok(field_num) = k.parse::<u32>() {
            msg.set_field(field_num, &v.to_string_repr());
        }
    }

    match msg.pack() {
        Ok(raw) => Ok(Value::String(raw)),
        Err(e) => Err(Galat::runtime(format!("Gagal mengemas ISO 8583: {}", e), span)),
    }
}

fn builtin_fintech_iso8583_urai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let raw = match &args[0] { Value::String(s) => s.as_str(), _ => return Err(Galat::runtime("Argumen 1 harus string raw ISO 8583", span)) };

    match crate::fintech::Iso8583Message::unpack(raw) {
        Ok(msg) => {
            let mut map = HashMap::new();
            map.insert("mti".to_string(), Value::String(msg.mti));
            let mut fields_map = HashMap::new();
            for (k, v) in msg.fields {
                fields_map.insert(k.to_string(), Value::String(v));
            }
            map.insert("fields".to_string(), Value::Map(Rc::new(RefCell::new(fields_map))));
            Ok(Value::Map(Rc::new(RefCell::new(map))))
        }
        Err(e) => Err(Galat::runtime(format!("Gagal mengurai ISO 8583: {}", e), span)),
    }
}

