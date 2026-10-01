use widya::fintech::*;
use widya::value::Value;

#[test]
fn test_qris_generation_parsing_and_crc16() {
    let builder = QrisBuilder::new()
        .dynamic(true)
        .merchant("TOKO MAJU JAYA", "JAKARTA", Some("12950"))
        .amount(75000.0)
        .invoice("INV-2026-9901")
        .terminal("T01");

    let raw_qris = builder.build();
    assert!(validasi_qris_crc16(&raw_qris), "QRIS CRC16 harus valid");
    assert!(raw_qris.contains("TOKO MAJU JAYA"));
    assert!(raw_qris.contains("540575000")); // Tag 54 nominal Rp 75.000

    let parsed = parse_qris(&raw_qris).expect("Parsing QRIS harus berhasil");
    assert_eq!(parsed.merchant_name, "TOKO MAJU JAYA");
    assert_eq!(parsed.merchant_city, "JAKARTA");
    assert_eq!(parsed.point_of_initiation, "12"); // Dinamis
    assert_eq!(parsed.transaction_amount, Some(75000.0));
    assert!(parsed.additional_data.is_some());
    assert_eq!(parsed.additional_data.unwrap().bill_number, Some("INV-2026-9901".to_string()));
}

#[test]
fn test_qris_static_to_dynamic_conversion() {
    let static_builder = QrisBuilder::new()
        .dynamic(false)
        .merchant("KAFE KOPI WIDYA", "BANDUNG", Some("40115"));
    let static_raw = static_builder.build();

    let dynamic_raw = konversi_statis_ke_dinamis(&static_raw, 35000.0, "ORDER-882")
        .expect("Konversi statis ke dinamis harus berhasil");

    assert!(validasi_qris_crc16(&dynamic_raw));
    let parsed_dyn = parse_qris(&dynamic_raw).unwrap();
    assert_eq!(parsed_dyn.point_of_initiation, "12");
    assert_eq!(parsed_dyn.transaction_amount, Some(35000.0));
    assert_eq!(parsed_dyn.merchant_name, "KAFE KOPI WIDYA");
}

#[test]
fn test_snap_bi_signature_and_hash() {
    let http_method = "POST";
    let endpoint = "/v1.0/transfer-va/payment";
    let access_token = "Bearer test_token_12345";
    let body = r#"{"partnerServiceId":"08888","customerNo":"000123","virtualAccountNo":"08888000123","trxId":"TRX-001"}"#;
    let timestamp = "2026-09-30T10:00:00+07:00";
    let client_secret = "rahasia_merchant_super_aman_123";

    let string_to_sign = bentuk_snap_string_to_sign(http_method, endpoint, access_token, body, timestamp);
    assert!(string_to_sign.starts_with("POST:/v1.0/transfer-va/payment:"));

    let signature = buat_snap_simetris_signature(&string_to_sign, client_secret)
        .expect("Pembuatan tanda tangan SNAP harus berhasil");
    assert!(!signature.is_empty());

    let is_valid = verifikasi_snap_simetris_signature(&string_to_sign, client_secret, &signature);
    assert!(is_valid, "Verifikasi tanda tangan SNAP harus sah");

    let invalid = verifikasi_snap_simetris_signature(&string_to_sign, "kunci_palsu", &signature);
    assert!(!invalid, "Verifikasi dengan secret salah harus gagal");
}

#[test]
fn test_iso8583_message_pack_and_unpack() {
    let mut msg = Iso8583Message::new("0200");
    msg.set_field(2, "4111111111111111"); // PAN (LLVAR)
    msg.set_field(3, "000000");           // Purchase Processing Code
    msg.set_field(4, "000000050000");     // Rp 50.000,00
    msg.set_field(11, "123456");          // STAN
    msg.set_field(41, "TERM0001");        // Terminal ID (8 char)
    msg.set_field(49, "360");             // IDR Currency

    let packed = msg.pack().expect("ISO 8583 packing harus berhasil");
    assert!(packed.starts_with("0200"), "Harus diawali MTI 0200");

    let unpacked = Iso8583Message::unpack(&packed).expect("ISO 8583 unpacking harus berhasil");
    assert_eq!(unpacked.mti, "0200");
    assert_eq!(unpacked.get_field(2).map(|s| s.as_str()), Some("4111111111111111"));
    assert_eq!(unpacked.get_field(3).map(|s| s.as_str()), Some("000000"));
    assert_eq!(unpacked.get_field(4).map(|s| s.as_str()), Some("000000050000"));
    assert_eq!(unpacked.get_field(11).map(|s| s.as_str()), Some("123456"));
    assert_eq!(unpacked.get_field(41).map(|s| s.as_str()), Some("TERM0001"));
    assert_eq!(unpacked.get_field(49).map(|s| s.as_str()), Some("360"));
}

#[test]
fn test_pci_luhn_card_detection_and_masking() {
    // Valid Visa
    let valid_visa = "4111111111111111";
    assert!(validasi_luhn(valid_visa));
    assert_eq!(deteksi_jaringan_kartu(valid_visa), CardNetwork::Visa);

    // Invalid Card PAN
    let invalid_card = "4111111111111112";
    assert!(!validasi_luhn(invalid_card));

    // Masking
    let masked = mask_pan(valid_visa);
    assert_eq!(masked, "4111-11XX-XXXX-1111");

    // Mastercard Detection
    let mastercard_pan = "5500000000000004";
    assert!(validasi_luhn(mastercard_pan));
    assert_eq!(deteksi_jaringan_kartu(mastercard_pan), CardNetwork::MasterCard);
}

#[test]
fn test_pci_card_tokenization_vault_aes256gcm() {
    let vault_key_hex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let tokenizer = CardTokenizer::new(vault_key_hex).unwrap();

    let pan = "4111111111111111";
    let vault = tokenizer.tokenize(pan, "12/28", "123").expect("Tokenisasi kartu harus berhasil");

    assert!(vault.token.starts_with("tok_card_"));
    assert_eq!(vault.masked_pan, "4111-11XX-XXXX-1111");

    let decrypted = tokenizer.detokenize(&vault).expect("Detokenisasi kartu harus berhasil");
    assert_eq!(decrypted["pan"], "4111111111111111");
    assert_eq!(decrypted["cvv"], "123");
    assert_eq!(decrypted["expiry"], "12/28");
}

#[test]
fn test_iso9564_pin_block_format0() {
    let pin = "1234";
    let pan = "4111111111111111";
    let pin_block = generate_iso9564_format0_pin_block(pin, pan).expect("Pembuatan PIN block harus berhasil");
    assert_eq!(pin_block.len(), 16); // 16 Hex characters = 8 bytes
}

#[test]
fn test_double_entry_ledger_and_fee_split() {
    let gross_amount = 100_000i64; // Rp 100.000
    let mdr_rate = 0.007;          // 0.7% QRIS MDR
    let fixed_fee = 0i64;
    let bank_switching_fee = 300i64;

    let fee_split = hitung_fee_split(gross_amount, mdr_rate, fixed_fee, bank_switching_fee);
    assert_eq!(fee_split.gross_amount, 100_000);
    assert_eq!(fee_split.total_fee, 700); // 0.7% dari 100.000 = 700
    assert_eq!(fee_split.merchant_net_payout, 99_300);

    let journal = buat_jurnal_pembayaran_sukses("TX-001", "INV-100", "M-99", &fee_split)
        .expect("Jurnal akuntansi ganda harus seimbang");

    // Pastikan lolos validasi buku besar (Sum Debit == Sum Credit)
    assert!(journal.validate_balance().is_ok());
}

#[test]
fn test_webhook_signatures_and_retry_schedule() {
    let payload = buat_webhook_event(
        WebhookEventType::PaymentSuccess,
        "INV-2026-001",
        250_000,
        "PAID",
        "QRIS",
    );
    let payload_json = serde_json::to_string(&payload).unwrap();
    let secret = "webhook_secret_key_abc";

    let signature = buat_webhook_signature(&payload_json, secret).unwrap();
    assert!(verifikasi_webhook_signature(&payload_json, &signature, secret));
    assert!(!verifikasi_webhook_signature(&payload_json, "salah", secret));

    // Exponential Backoff Tests
    assert_eq!(hitung_delay_retry_webhook(1), Some(1));
    assert_eq!(hitung_delay_retry_webhook(2), Some(5));
    assert_eq!(hitung_delay_retry_webhook(3), Some(30));
    assert_eq!(hitung_delay_retry_webhook(4), Some(300));
    assert_eq!(hitung_delay_retry_webhook(5), Some(3600));
    assert_eq!(hitung_delay_retry_webhook(6), None);
}

#[test]
fn test_fraud_detection_system_rules() {
    // Normal Transaction
    let normal_ctx = TransactionRiskContext {
        pan: None,
        amount_idr: 50_000,
        ip_address: "182.1.2.3".to_string(),
        user_agent: "WidyaApp/1.0".to_string(),
        velocity_tx_count_last_10m: 1,
        failed_attempts_last_hour: 0,
        is_vpn_or_proxy: false,
        card_country: Some("ID".to_string()),
        merchant_country: "ID".to_string(),
    };
    let assessment = evaluasi_risiko_transaksi(&normal_ctx);
    assert_eq!(assessment.decision, FdsDecision::Approve);
    assert_eq!(assessment.total_risk_score, 0);

    // High Risk Spurt (Velocity + Brute Force + VPN)
    let risky_ctx = TransactionRiskContext {
        pan: None,
        amount_idr: 60_000_000,
        ip_address: "198.51.100.4".to_string(),
        user_agent: "WidyaApp/1.0".to_string(),
        velocity_tx_count_last_10m: 12,
        failed_attempts_last_hour: 4,
        is_vpn_or_proxy: true,
        card_country: Some("RU".to_string()),
        merchant_country: "ID".to_string(),
    };
    let risky_assessment = evaluasi_risiko_transaksi(&risky_ctx);
    assert_eq!(risky_assessment.decision, FdsDecision::Reject);
    assert!(risky_assessment.total_risk_score >= 65);
}

#[test]
fn test_widya_script_pembayaran_execution() {
    let script = r#"
        biar luhn_valid = kartu_validasi_luhn("4111111111111111")
        biar masked = kartu_masking("4111111111111111")
        biar jaringan = kartu_deteksi_jaringan("4111111111111111")
        biar fee = pembayaran_hitung_fee_split(100000, 0.007, 0, 300)
        
        pastikan_benar(luhn_valid)
        pastikan_sama(masked, "4111-11XX-XXXX-1111")
        pastikan_sama(jaringan, "VISA")
        pastikan_sama(fee["gross_amount"], 100000)
        pastikan_sama(fee["merchant_net_payout"], 99300)
    "#;

    let res = widya::jalankan(script);
    assert!(res.is_ok(), "Eksekusi skrip pembayaran gagal: {:?}", res.err());
}
