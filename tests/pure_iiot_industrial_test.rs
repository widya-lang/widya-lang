use widya::iiot::pure_industrial::{
    CanFrame, CanId, IndustrialRegisterMap, IndustrialWordConverter, ModbusRtuFrame,
    MODBUS_EX_ILLEGAL_FUNCTION,
};

#[test]
fn test_can_socketcan_standard_and_extended_roundtrip() {
    // 1. Standard 11-bit Frame
    let std_payload = [0x11, 0x22, 0x33, 0x44, 0x55];
    let std_frame = CanFrame::new_standard(0x123, &std_payload).expect("Standard CAN failed");
    assert_eq!(std_frame.id, CanId::Standard(0x123));
    assert_eq!(std_frame.dlc, 5);
    assert_eq!(std_frame.raw_id(), 0x123);

    let wire_bytes = std_frame.to_socketcan_bytes();
    assert_eq!(wire_bytes.len(), 16);

    let decoded_std = CanFrame::from_socketcan_bytes(&wire_bytes).expect("Decode standard failed");
    assert_eq!(decoded_std.id, CanId::Standard(0x123));
    assert_eq!(decoded_std.dlc, 5);
    assert_eq!(&decoded_std.data[..5], &std_payload);
    assert!(!decoded_std.is_rtr);
    assert!(!decoded_std.is_error);

    // 2. Extended 29-bit Frame
    let ext_payload = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11];
    let ext_frame = CanFrame::new_extended(0x18DAF110, &ext_payload).expect("Extended CAN failed");
    assert_eq!(ext_frame.id, CanId::Extended(0x18DAF110));
    assert_eq!(ext_frame.dlc, 8);
    assert_eq!(ext_frame.raw_id(), 0x18DAF110);

    let ext_wire = ext_frame.to_socketcan_bytes();
    let decoded_ext = CanFrame::from_socketcan_bytes(&ext_wire).expect("Decode extended failed");
    assert_eq!(decoded_ext.id, CanId::Extended(0x18DAF110));
    assert_eq!(decoded_ext.dlc, 8);
    assert_eq!(&decoded_ext.data, &ext_payload);

    // 3. CAN Filter Matching
    assert!(std_frame.matches_filter(0x120, 0x7F0)); // Mask 0x7F0 matches 0x12X
    assert!(!std_frame.matches_filter(0x130, 0x7F0));
}

#[test]
fn test_modbus_rtu_frame_crc16_and_roundtrip() {
    // Read holding registers: Slave 1, Start 100 (0x0064), Count 2 (0x0002)
    let req = ModbusRtuFrame::read_holding_registers(1, 100, 2);
    let bytes = req.to_bytes();

    // Verify minimum length (Slave 1 + FC 1 + Payload 4 + CRC 2 = 8 bytes)
    assert_eq!(bytes.len(), 8);
    assert_eq!(bytes[0], 1);
    assert_eq!(bytes[1], 0x03);

    // Parse bytes and verify CRC-16 check passes
    let parsed = ModbusRtuFrame::from_bytes(&bytes).expect("CRC parse failed");
    assert_eq!(parsed.slave_id, 1);
    assert_eq!(parsed.function_code, 0x03);
    assert_eq!(parsed.payload, vec![0x00, 0x64, 0x00, 0x02]);

    // Corrupt one byte and assert CRC failure
    let mut corrupted = bytes.clone();
    corrupted[3] ^= 0xFF;
    assert!(ModbusRtuFrame::from_bytes(&corrupted).is_err());
}

#[test]
fn test_in_memory_plc_state_machine() {
    let mut plc = IndustrialRegisterMap::new(1, 256);

    // 1. Initial State: register 10 is 0
    let read_req = ModbusRtuFrame::read_holding_registers(1, 10, 1).to_bytes();
    let resp_bytes = plc.process_rtu_request(&read_req).expect("Process read failed");
    let resp = ModbusRtuFrame::from_bytes(&resp_bytes).unwrap();
    assert_eq!(resp.function_code, 0x03);
    assert_eq!(resp.payload, vec![0x02, 0x00, 0x00]); // 2 bytes, value 0

    // 2. Write Single Register: Set register 10 to 1234 (0x04D2)
    let write_req = ModbusRtuFrame::write_single_register(1, 10, 1234).to_bytes();
    let write_resp = plc.process_rtu_request(&write_req).expect("Process write failed");
    let resp_w = ModbusRtuFrame::from_bytes(&write_resp).unwrap();
    assert_eq!(resp_w.function_code, 0x06);
    assert_eq!(plc.holding_registers[10], 1234);

    // 3. Read again -> should now return 1234
    let resp2_bytes = plc.process_rtu_request(&read_req).unwrap();
    let resp2 = ModbusRtuFrame::from_bytes(&resp2_bytes).unwrap();
    assert_eq!(resp2.payload, vec![0x02, 0x04, 0xD2]);

    // 4. Write Multiple Registers: Registers 20, 21, 22
    let multi_req = ModbusRtuFrame::write_multiple_registers(1, 20, &[100, 200, 300]).to_bytes();
    plc.process_rtu_request(&multi_req).expect("Write multi failed");
    assert_eq!(plc.holding_registers[20], 100);
    assert_eq!(plc.holding_registers[21], 200);
    assert_eq!(plc.holding_registers[22], 300);

    // 5. Illegal Function Code (e.g. 0x22)
    let bad_req = ModbusRtuFrame::new(1, 0x22, vec![]).to_bytes();
    let err_resp = plc.process_rtu_request(&bad_req).unwrap();
    let resp_err = ModbusRtuFrame::from_bytes(&err_resp).unwrap();
    assert_eq!(resp_err.function_code, 0xA2); // 0x22 | 0x80
    assert_eq!(resp_err.payload, vec![MODBUS_EX_ILLEGAL_FUNCTION]);
}

#[test]
fn test_industrial_word_conversions() {
    let original_val = 3.1415927f32;

    // Big-Endian ABCD
    let (h, l) = IndustrialWordConverter::f32_to_be_registers(original_val);
    let reconstructed = IndustrialWordConverter::be_registers_to_f32(h, l);
    assert!((reconstructed - original_val).abs() < 1e-6);

    // Word-Swapped CDAB
    let (first, second) = IndustrialWordConverter::f32_to_word_swapped_registers(original_val);
    assert_eq!(first, l);
    assert_eq!(second, h);
    let reconstructed_swapped = IndustrialWordConverter::word_swapped_registers_to_f32(first, second);
    assert!((reconstructed_swapped - original_val).abs() < 1e-6);

    // Int32 conversion
    let i_val = -987654321i32;
    let (ih, il) = IndustrialWordConverter::i32_to_registers(i_val);
    let reconstructed_i = IndustrialWordConverter::registers_to_i32(ih, il);
    assert_eq!(reconstructed_i, i_val);
}
