use widya::compiler::decimal::Desimal;
use widya::fintech::iso20022::{Iso20022Pacs008, Iso20022Pacs002};
use widya::fintech::qris::hitung_qris_crc16;
use widya::healthtech::bpjs::BpjsBridge;
use widya::healthtech::fhir::FhirPatient;
use widya::iot::modbus::{ModbusFunction, ModbusTcpFrame};
use widya::smartcity::hitung_ispu_pm25;
use widya::kripto::pqc::PqcEngine;
use widya::kripto::uupdp::UuPdpShield;

#[test]
fn test_fintech_iso20022_and_qris() {
    let transfer = Iso20022Pacs008::new(
        "MSG-2026-001",
        "INS-9901",
        Desimal::dari_int(2500000),
        "Ahmad Fauzi",
        "1029384756",
        "Koperasi Desa",
        "9988776655",
    );
    let xml = transfer.to_xml();
    assert!(xml.contains("<MsgId>MSG-2026-001</MsgId>"));
    assert!(xml.contains("2500000.0000"));

    let crc = hitung_qris_crc16("000201010212265802ID5911KOPERASI6007JAKARTA6304");
    assert_eq!(crc.len(), 4);
}

#[test]
fn test_healthtech_satusehat_and_bpjs() {
    let patient = FhirPatient::new_satusehat(
        "3302011504900003",
        "Budi Santoso",
        "male",
        "1990-04-15",
        "081234567890",
    );
    assert_eq!(patient.resource_type, "Patient");
    assert_eq!(patient.identifier[0].value, "3302011504900003");

    let bpjs = BpjsBridge::new("12345", "secret_bpjs_key", "user_key_99");
    let headers = bpjs.generate_headers().unwrap();
    assert_eq!(headers.cons_id, "12345");
    assert!(!headers.signature.is_empty());
}

#[test]
fn test_iot_modbus_and_ispu() {
    let frame = ModbusTcpFrame::read_holding_registers(1, 1, 0x0001, 4);
    let bytes = frame.to_bytes();
    assert_eq!(bytes[7], ModbusFunction::ReadHoldingRegisters as u8);

    let ispu = hitung_ispu_pm25(38.4);
    assert!(ispu.nilai_ispu > 50);
    assert_eq!(ispu.kategori.label(), "Sedang");
}

#[test]
fn test_security_pqc_and_uupdp() {
    let keys = PqcEngine::generate_dilithium5_keypair("seed_sovereign_widya");
    let doc = b"TRANSAKSI OKE 100JT";
    let sig = PqcEngine::sign_document(doc, &keys.private_key_hex);
    assert!(PqcEngine::verify_signature(doc, &sig, &keys.public_key_hex));

    let masked_nik = UuPdpShield::mask_nik("3174092109880001");
    assert_eq!(masked_nik, "************0001");

    let masked_hp = UuPdpShield::mask_nomor_hp("081234567890");
    assert_eq!(masked_hp, "0812****7890");
}
