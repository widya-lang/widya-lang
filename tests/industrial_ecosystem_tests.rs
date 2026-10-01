use widya::web3::*;
use widya::healthtech::*;
use widya::iot::*;
use widya::ai::*;
use widya::automotive::*;

#[test]
fn test_web3_cryptography_and_address_checksum() {
    let hash = keccak256(b"widya-lang");
    assert_ne!(hash, [0u8; 32]);
    let hex_str = keccak256_hex(b"widya-lang");
    assert!(hex_str.starts_with("0x"));

    // Tes EIP-55 checksum address
    let raw_addr = [0x5a, 0xae, 0xb6, 0x05, 0x3f, 0x3e, 0x94, 0xc9, 0xb9, 0xa0, 0x9f, 0x33, 0x66, 0x94, 0x35, 0xe7, 0xef, 0x1b, 0xea, 0xed];
    let checksum = to_checksum_address(&raw_addr);
    assert_eq!(checksum.len(), 42);
    assert!(is_valid_ethereum_address(&checksum));
}

#[test]
fn test_web3_abi_encoder_and_decoder() {
    let selector = hitung_function_selector("transfer(address,uint256)");
    assert_eq!(selector, [0xa9, 0x05, 0x9c, 0xbb]);

    let event_topic = hitung_event_topic("Transfer(address,address,uint256)");
    assert_ne!(event_topic, [0u8; 32]);

    let calldata = AbiEncoder::encode_erc20_transfer("0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed", 1_000_000).unwrap();
    assert_eq!(calldata.len(), 4 + 32 + 32);

    let decoded_amount = AbiDecoder::decode_uint256(&calldata[36..68]).unwrap();
    assert_eq!(decoded_amount, 1_000_000);
}

#[test]
fn test_web3_evm_micro_runtime_execution() {
    let mut runtime = EvmRuntime::new(100_000);
    let ctx = EvmContext::default();

    // Bytecode: PUSH1 0x0A (10), PUSH1 0x14 (20), ADD, PUSH1 0x00, MSTORE, PUSH1 0x20, PUSH1 0x00, RETURN
    let bytecode = vec![
        0x60, 0x0a,
        0x60, 0x14,
        0x01,
        0x60, 0x00,
        0x52,
        0x60, 0x20,
        0x60, 0x00,
        0xf3,
    ];

    let result = runtime.execute(&bytecode, &ctx);
    assert_eq!(result.status, EvmExecutionStatus::Success);
    assert_eq!(result.return_data.len(), 32);
    let out_val = AbiDecoder::decode_uint256(&result.return_data).unwrap();
    assert_eq!(out_val, 30); // 10 + 20 = 30
}

#[test]
fn test_web3_rwa_tokenization_and_proof_of_reserve() {
    let mut gold_token = RwaToken::new(
        "Antam Gold Token",
        "W-GOLD",
        RwaAssetType::EmasFisik,
        1_350_000, // Rp 1.350.000 / gram
        "PT Pegadaian & Antam Persero",
    );

    let serial_numbers = vec!["ANTAM-AU-999-001", "ANTAM-AU-999-002", "ANTAM-AU-999-003"];
    let mint_res = gold_token.mint("0xUserWalletAddress1234567890", 100, &serial_numbers);
    assert!(mint_res.is_ok());
    assert_eq!(gold_token.balance_of("0xUserWalletAddress1234567890"), 100);
    assert!(gold_token.proof_of_reserve_root.starts_with("0x"));
    assert_eq!(gold_token.total_market_cap_idr(), 135_000_000);
}

#[test]
fn test_healthtech_fhir_satusehat_models() {
    let patient = FhirPatient::new_satusehat(
        "3273012345670001",
        "Budi Santoso",
        "male",
        "1990-05-15",
        "081234567890",
    );
    assert_eq!(patient.resource_type, "Patient");
    assert_eq!(patient.identifier[0].value, "3273012345670001");

    let obs = FhirObservation::sistolik(&patient.id, 120.0, "2026-09-30T09:00:00+07:00");
    assert_eq!(obs.resource_type, "Observation");
    assert_eq!(obs.value_quantity.unwrap().value, 120.0);
}

#[test]
fn test_healthtech_dicom_parser_and_icd10() {
    let mut mock_dicom = vec![0u8; 140];
    mock_dicom[128..132].copy_from_slice(b"DICM"); // Magic bytes
    let dicom = parse_dicom_bytes(&mock_dicom).expect("DICOM valid harus berhasil diurai");
    assert!(dicom.is_valid_dicom);

    let search = cari_diagnosis_icd10("Hipertensi");
    assert!(!search.is_empty());
    assert_eq!(search[0].icd10_code, "I10");
}

#[test]
fn test_iot_mqtt_wildcard_and_geofence() {
    assert!(cocokkan_topik_mqtt("kebun/sawit/+/suhu", "kebun/sawit/blokA/suhu"));
    assert!(cocokkan_topik_mqtt("kebun/#", "kebun/irigasi/pompa1/status"));
    assert!(!cocokkan_topik_mqtt("kebun/sawit/suhu", "kebun/padi/suhu"));

    // Monas Jakarta (lat: -6.1754, lon: 106.8272)
    let in_monas = periksa_geofence(-6.1753, 106.8271, -6.1754, 106.8272, 100.0);
    assert!(in_monas);
    let outside_monas = periksa_geofence(-6.2000, 106.8000, -6.1754, 106.8272, 100.0);
    assert!(!outside_monas);
}

#[test]
fn test_ai_tensor_matmul_and_cosine_similarity() {
    let t1 = Tensor::new(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]);
    let t2 = Tensor::new(vec![2, 2], vec![2.0, 0.0, 1.0, 2.0]);
    let res = t1.matmul(&t2).unwrap();
    // [1*2 + 2*1, 1*0 + 2*2] = [4, 4]
    // [3*2 + 4*1, 3*0 + 4*2] = [10, 8]
    assert_eq!(res.data, vec![4.0, 4.0, 10.0, 8.0]);

    let emb1 = Tensor::new(vec![3], vec![1.0, 0.0, 0.0]);
    let emb2 = Tensor::new(vec![3], vec![1.0, 0.0, 0.0]);
    assert!((emb1.cosine_similarity(&emb2) - 1.0).abs() < 1e-5);
}

#[test]
fn test_automotive_obd2_frame_decoding() {
    // Mode 01 PID 0x0C (Engine RPM): DataA = 0x1F, DataB = 0x40 -> (31*256 + 64)/4 = 8000/4 = 2000 RPM
    let can_payload = [0x04, 0x41, 0x0C, 0x1F, 0x40, 0x00, 0x00, 0x00];
    let frame = CanFrame::new_standard(0x7E8, &can_payload);
    let decoded = dekode_obd2_can_frame(&frame);
    assert!(decoded.is_some());
    let (pid, val) = decoded.unwrap();
    assert_eq!(pid, "ENGINE_RPM");
    assert_eq!(val, 2000.0);
}

#[test]
fn test_widya_script_multi_industry_execution() {
    let script = r##"
        // 1. Web3 Keccak & EIP-55
        biar hash = web3_keccak256("WidyaSovereignWeb3")
        pastikan_benar(panjang(hash) > 10)

        // 2. SatuSehat FHIR
        biar pasien = kesehatan_satusehat_pasien("3273010101900001", "Ahmad", "male", "1990-01-01", "0811111111")
        pastikan_sama(pasien["resourceType"], "Patient")

        // 3. IoT MQTT
        biar hasil_cocok = iot_cocokkan_topik_mqtt("iot/sensor/+/suhu", "iot/sensor/ruang1/suhu")
        pastikan_benar(hasil_cocok)

        // 4. AI Cosine Similarity
        biar sim = ai_tensor_cosine_similarity([1.0, 0.0], [1.0, 0.0])
        pastikan_sama(sim, 1.0)
    "##;

    let res = widya::jalankan(script);
    assert!(res.is_ok(), "Eksekusi skrip multi-industri gagal: {:?}", res.err());
}
