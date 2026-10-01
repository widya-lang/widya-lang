use serde::{Deserialize, Serialize};

/// ============================================================================
/// Modbus TCP & RTU Protocol Industrial Automation Engine
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModbusFunctionCode {
    ReadCoils = 0x01,
    ReadDiscreteInputs = 0x02,
    ReadHoldingRegisters = 0x03,
    ReadInputRegisters = 0x04,
    WriteSingleCoil = 0x05,
    WriteSingleRegister = 0x06,
    WriteMultipleRegisters = 0x10,
}

impl ModbusFunctionCode {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0x01 => Some(Self::ReadCoils),
            0x02 => Some(Self::ReadDiscreteInputs),
            0x03 => Some(Self::ReadHoldingRegisters),
            0x04 => Some(Self::ReadInputRegisters),
            0x05 => Some(Self::WriteSingleCoil),
            0x06 => Some(Self::WriteSingleRegister),
            0x10 => Some(Self::WriteMultipleRegisters),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModbusTcpFrame {
    pub transaction_id: u16,
    pub protocol_id: u16, // Always 0 for Modbus TCP
    pub length: u16,
    pub unit_id: u8,
    pub function_code: ModbusFunctionCode,
    pub address: u16,
    pub count_or_value: u16,
    pub data_payload: Vec<u8>,
}

pub fn calculate_crc16_modbus(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if (crc & 0x0001) != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

impl ModbusTcpFrame {
    pub fn new_read_holding_registers(tid: u16, unit: u8, addr: u16, count: u16) -> Self {
        Self {
            transaction_id: tid,
            protocol_id: 0,
            length: 6,
            unit_id: unit,
            function_code: ModbusFunctionCode::ReadHoldingRegisters,
            address: addr,
            count_or_value: count,
            data_payload: Vec::new(),
        }
    }

    pub fn new_write_single_register(tid: u16, unit: u8, addr: u16, val: u16) -> Self {
        Self {
            transaction_id: tid,
            protocol_id: 0,
            length: 6,
            unit_id: unit,
            function_code: ModbusFunctionCode::WriteSingleRegister,
            address: addr,
            count_or_value: val,
            data_payload: Vec::new(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12 + self.data_payload.len());
        bytes.extend_from_slice(&self.transaction_id.to_be_bytes());
        bytes.extend_from_slice(&self.protocol_id.to_be_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.push(self.unit_id);
        bytes.push(self.function_code as u8);
        bytes.extend_from_slice(&self.address.to_be_bytes());
        bytes.extend_from_slice(&self.count_or_value.to_be_bytes());
        bytes.extend_from_slice(&self.data_payload);
        bytes
    }

    pub fn parse_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() < 8 {
            return Err("Data terlalu pendek untuk frame Modbus TCP (< 8 bytes)".to_string());
        }

        let tid = u16::from_be_bytes([data[0], data[1]]);
        let pid = u16::from_be_bytes([data[2], data[3]]);
        let len = u16::from_be_bytes([data[4], data[5]]);
        let unit = data[6];
        let fc = ModbusFunctionCode::from_u8(data[7])
            .ok_or_else(|| format!("Function Code 0x{:02X} tidak valid", data[7]))?;

        let (addr, count_val) = if data.len() >= 12 {
            (
                u16::from_be_bytes([data[8], data[9]]),
                u16::from_be_bytes([data[10], data[11]]),
            )
        } else {
            (0, 0)
        };

        let payload = if data.len() > 12 {
            data[12..].to_vec()
        } else {
            Vec::new()
        };

        Ok(Self {
            transaction_id: tid,
            protocol_id: pid,
            length: len,
            unit_id: unit,
            function_code: fc,
            address: addr,
            count_or_value: count_val,
            data_payload: payload,
        })
    }
}
