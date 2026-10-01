//! Modul Protokol Modbus TCP/RTU untuk Sensor Industri, PLC, dan Telemetri Pabrik
//! Mendukung Function Code 0x03 (Read Holding Registers), 0x04 (Read Input Registers), dan 0x06 (Write Single Register).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModbusFunction {
    ReadCoils = 0x01,
    ReadDiscreteInputs = 0x02,
    ReadHoldingRegisters = 0x03,
    ReadInputRegisters = 0x04,
    WriteSingleCoil = 0x05,
    WriteSingleRegister = 0x06,
    WriteMultipleRegisters = 0x10,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModbusTcpFrame {
    pub transaction_id: u16,
    pub protocol_id: u16, // Selalu 0 untuk Modbus TCP
    pub length: u16,
    pub unit_id: u8,
    pub function_code: u8,
    pub data: Vec<u8>,
}

impl ModbusTcpFrame {
    /// Membuat frame permintaan Read Holding Registers (Function Code 0x03)
    pub fn read_holding_registers(tx_id: u16, unit_id: u8, start_address: u16, quantity: u16) -> Self {
        let mut data = Vec::new();
        data.extend_from_slice(&start_address.to_be_bytes());
        data.extend_from_slice(&quantity.to_be_bytes());

        Self {
            transaction_id: tx_id,
            protocol_id: 0,
            length: 6, // Unit ID (1) + Function Code (1) + Address (2) + Quantity (2)
            unit_id,
            function_code: ModbusFunction::ReadHoldingRegisters as u8,
            data,
        }
    }

    /// Serialisasi frame ke byte array network wire format
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.transaction_id.to_be_bytes());
        bytes.extend_from_slice(&self.protocol_id.to_be_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.push(self.unit_id);
        bytes.push(self.function_code);
        bytes.extend_from_slice(&self.data);
        bytes
    }

    /// Parse respons byte array dari perangkat PLC
    pub fn parse_response(bytes: &[u8]) -> Result<Vec<u16>, String> {
        if bytes.len() < 9 {
            return Err("Frame Modbus TCP tidak lengkap (terlalu pendek)".into());
        }

        let function_code = bytes[7];
        if function_code & 0x80 != 0 {
            let error_code = bytes[8];
            return Err(format!("Modbus Exception Response Code: 0x{:02X}", error_code));
        }

        let byte_count = bytes[8] as usize;
        let mut registers = Vec::new();

        let mut offset = 9;
        while offset + 1 < 9 + byte_count && offset + 1 < bytes.len() {
            let val = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
            registers.push(val);
            offset += 2;
        }

        Ok(registers)
    }
}
