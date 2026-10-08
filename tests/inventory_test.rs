// ============================================================================
// UJI INTEGRASI MODUL INVENTARIS: BARCODE & RFID GUDANG (tests/inventory_test.rs)
// ============================================================================

use widya::inventory::{
    Ean13Barcode, EpcFilterValue, EpcSgtin96, GateDirection, Gs1Barcode, RfidGateEvent,
    WarehouseLocation, WarehouseManager,
};

#[test]
fn test_ean13_checksum_and_validation() {
    // 899 adalah kode negara Indonesia (GS1 Indonesia)
    let base12 = "899123456789";
    let check = Ean13Barcode::calculate_checksum(base12).expect("Kalkulasi checksum gagal");
    assert_eq!(check, 1);

    let barcode = Ean13Barcode::from_12_digits(base12).expect("Pembuatan EAN-13 gagal");
    assert_eq!(barcode.raw, "8991234567891");
    assert_eq!(barcode.country_code, "899");
    assert!(barcode.is_valid);

    // Verifikasi penolakan barcode yang checksum-nya salah
    let invalid = Ean13Barcode::parse("8991234567899");
    assert!(invalid.is_err());
}

#[test]
fn test_gs1_128_application_identifiers() {
    let raw = "(01)08991234567891(10)LOT-2026-X(17)261231(21)SN-998877";
    let gs1 = Gs1Barcode::parse(raw);

    assert_eq!(gs1.gtin.as_deref(), Some("08991234567891"));
    assert_eq!(gs1.batch_lot.as_deref(), Some("LOT-2026-X"));
    assert_eq!(gs1.expiry_date.as_deref(), Some("261231"));
    assert_eq!(gs1.serial_number.as_deref(), Some("SN-998877"));
}

#[test]
fn test_rfid_epc_sgtin96_encode_decode() {
    let company_prefix = 8991234u64;
    let item_ref = 42u32;
    let serial = 98765u64;

    let epc = EpcSgtin96::new(EpcFilterValue::FullCase, company_prefix, item_ref, serial);
    assert_eq!(epc.header, 0x30);
    assert_eq!(epc.raw_hex.len(), 24);

    let decoded = EpcSgtin96::decode(&epc.raw_hex).expect("Dekode EPC SGTIN-96 gagal");
    assert_eq!(decoded.header, 0x30);
    assert_eq!(decoded.filter, EpcFilterValue::FullCase);
    assert_eq!(decoded.company_prefix, company_prefix);
    assert_eq!(decoded.item_reference, item_ref);
    assert_eq!(decoded.serial_number, serial);
}

#[test]
fn test_warehouse_movement_and_opname() {
    let mut wm = WarehouseManager::new();

    // 1. Daftarkan lokasi gudang
    let loc = WarehouseLocation {
        location_id: "LOC-A-01".to_string(),
        zone: "ZONA-UTAMA".to_string(),
        aisle: "LORONG-1".to_string(),
        rack: "RAK-01".to_string(),
        bin: "BIN-A".to_string(),
        max_capacity_units: 500,
        current_units: 0,
    };
    wm.register_location(loc);

    // 2. Daftarkan item barang
    let barcode = "8991234567891";
    let item = wm
        .register_item(
            barcode,
            "Beras Organik Nusantara 5kg",
            "Pangan",
            "Sak",
            50, // stok awal
            10, // min threshold
            75000,
            "LOC-A-01",
        )
        .expect("Pendaftaran barang gagal");

    let epc_hex = item.rfid_epc_hex.clone();

    // 3. Simulasikan event Gate-In (barang masuk bertambah 1)
    let gate_in = RfidGateEvent {
        gate_id: "PORTAL-GATE-01".to_string(),
        tag_epc_hex: epc_hex.clone(),
        antenna_port: 1,
        rssi_dbm: -42.5,
        read_count: 5,
        timestamp_ms: 1760000000000,
        direction: GateDirection::MasukGudang,
    };
    let mov_in = wm.process_gate_event(gate_in).expect("Gate-in gagal");
    assert_eq!(mov_in.direction, GateDirection::MasukGudang);

    // 4. Simulasikan event Gate-Out (barang keluar berkurang 1)
    let gate_out = RfidGateEvent {
        gate_id: "PORTAL-GATE-DOCK-B".to_string(),
        tag_epc_hex: epc_hex.clone(),
        antenna_port: 2,
        rssi_dbm: -39.0,
        read_count: 8,
        timestamp_ms: 1760000001000,
        direction: GateDirection::KeluarGudang,
    };
    let mov_out = wm.process_gate_event(gate_out).expect("Gate-out gagal");
    assert_eq!(mov_out.direction, GateDirection::KeluarGudang);

    // 5. Uji Stok Opname Otomatis
    let scanned_tags = vec![epc_hex];
    let report = wm.reconcile_stock_opname(&scanned_tags);
    assert_eq!(report.matched_tags, 1);
    assert!(report.missing_tags.is_empty());
}
