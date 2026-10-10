//! Pure Sovereign Zero-Dependency Industrial Automation Engine (pure_industrial.rs)
//!
//! 100% In-House Industrial Protocol Stack:
//! - CAN 2.0A (11-bit standard) & CAN 2.0B (29-bit extended) Frame Engine (SocketCAN ABI compliant)
//! - Modbus RTU Binary Protocol with CRC-16 Modbus (Polynomial 0xA001)
//! - In-Memory Modbus PLC Slave/Server State Machine (Coils, Holding/Input Registers)
//! - IEEE-754 Float32 & Int32 Industrial Multi-Register Word Conversion

use crate::iiot::modbus::calculate_crc16_modbus;
use serde::{Deserialize, Serialize};

// ============================================================================
// 1. CAN-BUS 2.0A / 2.0B SOCKETCAN COMPLIANT FRAME ENGINE
// ============================================================================

pub const CAN_EFF_FLAG: u32 = 0x80000000; // Bit 31: Extended Frame Format (29-bit)
pub const CAN_RTR_FLAG: u32 = 0x40000000; // Bit 30: Remote Transmission Request
pub const CAN_ERR_FLAG: u32 = 0x20000000; // Bit 29: Error Frame
pub const CAN_SFF_MASK: u32 = 0x000007FF; // 11-bit standard mask
pub const CAN_EFF_MASK: u32 = 0x1FFFFFFF; // 29-bit extended mask

/// Tipe Pengalamatan CAN-Bus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanId {
    Standard(u16), // 11-bit (0x000 .. 0x7FF)
    Extended(u32), // 29-bit (0x00000000 .. 0x1FFFFFFF)
}

/// Struktur Frame CAN-Bus 2.0 (16-byte Linux SocketCAN struct can_frame layout)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanFrame {
    pub id: CanId,
    pub is_rtr: bool,
    pub is_error: bool,
    pub dlc: u8,
    pub data: [u8; 8],
}

impl CanFrame {
    /// Buat frame CAN standar 11-bit
    pub fn new_standard(id: u16, data: &[u8]) -> Result<Self, String> {
        if id > 0x7FF {
            return Err(format!("Standard CAN ID 0x{:X} melebihi batas 11-bit (0x7FF)", id));
        }
        if data.len() > 8 {
            return Err("CAN 2.0 data payload maksimal 8 byte".to_string());
        }

        let mut buf = [0u8; 8];
        buf[..data.len()].copy_from_slice(data);

        Ok(Self {
            id: CanId::Standard(id),
            is_rtr: false,
            is_error: false,
            dlc: data.len() as u8,
            data: buf,
        })
    }

    /// Buat frame CAN extended 29-bit
    pub fn new_extended(id: u32, data: &[u8]) -> Result<Self, String> {
        if id > CAN_EFF_MASK {
            return Err(format!("Extended CAN ID 0x{:X} melebihi batas 29-bit (0x1FFFFFFF)", id));
        }
        if data.len() > 8 {
            return Err("CAN 2.0 data payload maksimal 8 byte".to_string());
        }

        let mut buf = [0u8; 8];
        buf[..data.len()].copy_from_slice(data);

        Ok(Self {
            id: CanId::Extended(id),
            is_rtr: false,
            is_error: false,
            dlc: data.len() as u8,
            data: buf,
        })
    }

    /// Buat RTR (Remote Transmission Request) frame
    pub fn new_rtr(id: CanId, requested_dlc: u8) -> Self {
        Self {
            id,
            is_rtr: true,
            is_error: false,
            dlc: requested_dlc.min(8),
            data: [0u8; 8],
        }
    }

    /// Dapatkan ID numerik 32-bit (raw ID value)
    pub fn raw_id(&self) -> u32 {
        match self.id {
            CanId::Standard(id) => (id as u32) & CAN_SFF_MASK,
            CanId::Extended(id) => id & CAN_EFF_MASK,
        }
    }

    /// Serialisasi ke 16-byte biner SocketCAN ABI
    pub fn to_socketcan_bytes(&self) -> [u8; 16] {
        let mut out = [0u8; 16];

        let mut can_id_field = match self.id {
            CanId::Standard(id) => (id as u32) & CAN_SFF_MASK,
            CanId::Extended(id) => ((id as u32) & CAN_EFF_MASK) | CAN_EFF_FLAG,
        };

        if self.is_rtr {
            can_id_field |= CAN_RTR_FLAG;
        }
        if self.is_error {
            can_id_field |= CAN_ERR_FLAG;
        }

        let id_bytes = can_id_field.to_le_bytes();
        out[0..4].copy_from_slice(&id_bytes);
        out[4] = self.dlc;
        out[5] = 0; // padding
        out[6] = 0; // res0
        out[7] = 0; // res1
        out[8..16].copy_from_slice(&self.data);

        out
    }

    /// Deserialisasi dari 16-byte biner SocketCAN
    pub fn from_socketcan_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 16 {
            return Err("Ukuran buffer terlalu pendek untuk struct can_frame (harus 16 byte)".to_string());
        }

        let raw_can_id = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let is_extended = (raw_can_id & CAN_EFF_FLAG) != 0;
        let is_rtr = (raw_can_id & CAN_RTR_FLAG) != 0;
        let is_error = (raw_can_id & CAN_ERR_FLAG) != 0;

        let id = if is_extended {
            CanId::Extended(raw_can_id & CAN_EFF_MASK)
        } else {
            CanId::Standard((raw_can_id & CAN_SFF_MASK) as u16)
        };

        let dlc = bytes[4].min(8);
        let mut data = [0u8; 8];
        data.copy_from_slice(&bytes[8..16]);

        Ok(Self {
            id,
            is_rtr,
            is_error,
            dlc,
            data,
        })
    }

    /// Cek kecocokan filter CAN (ID & Mask)
    pub fn matches_filter(&self, filter_id: u32, filter_mask: u32) -> bool {
        (self.raw_id() & filter_mask) == (filter_id & filter_mask)
    }
}

impl From<crate::automotive::canbus::CanFrame> for CanFrame {
    fn from(f: crate::automotive::canbus::CanFrame) -> Self {
        let id = if f.is_extended {
            CanId::Extended(f.id)
        } else {
            CanId::Standard(f.id as u16)
        };
        Self {
            id,
            is_rtr: f.is_rtr,
            is_error: false,
            dlc: f.dlc,
            data: f.data,
        }
    }
}

impl From<CanFrame> for crate::automotive::canbus::CanFrame {
    fn from(f: CanFrame) -> Self {
        let (id, is_extended) = match f.id {
            CanId::Standard(s) => (s as u32, false),
            CanId::Extended(e) => (e, true),
        };
        Self {
            id,
            is_extended,
            is_rtr: f.is_rtr,
            dlc: f.dlc,
            data: f.data,
        }
    }
}

impl From<crate::rtos::CanFrame> for CanFrame {
    fn from(f: crate::rtos::CanFrame) -> Self {
        let id = if f.is_extended {
            CanId::Extended(f.id)
        } else {
            CanId::Standard(f.id as u16)
        };
        Self {
            id,
            is_rtr: false,
            is_error: false,
            dlc: f.dlc,
            data: f.data,
        }
    }
}

impl From<CanFrame> for crate::rtos::CanFrame {
    fn from(f: CanFrame) -> Self {
        let (id, is_extended) = match f.id {
            CanId::Standard(s) => (s as u32, false),
            CanId::Extended(e) => (e, true),
        };
        Self {
            id,
            is_extended,
            dlc: f.dlc,
            data: f.data,
        }
    }
}

// ============================================================================
// 2. MODBUS RTU SERIAL PROTOCOL STACK
// ============================================================================

/// Frame Modbus RTU: [Slave ID (1B), Function Code (1B), Payload (NB), CRC-16 (2B Little-Endian)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModbusRtuFrame {
    pub slave_id: u8,
    pub function_code: u8,
    pub payload: Vec<u8>,
}

impl ModbusRtuFrame {
    pub fn new(slave_id: u8, function_code: u8, payload: Vec<u8>) -> Self {
        Self {
            slave_id,
            function_code,
            payload,
        }
    }

    /// Query Baca Holding Registers (FC 0x03)
    pub fn read_holding_registers(slave_id: u8, start_addr: u16, count: u16) -> Self {
        let mut p = Vec::with_capacity(4);
        p.extend_from_slice(&start_addr.to_be_bytes());
        p.extend_from_slice(&count.to_be_bytes());
        Self::new(slave_id, 0x03, p)
    }

    /// Query Tulis Single Register (FC 0x06)
    pub fn write_single_register(slave_id: u8, addr: u16, value: u16) -> Self {
        let mut p = Vec::with_capacity(4);
        p.extend_from_slice(&addr.to_be_bytes());
        p.extend_from_slice(&value.to_be_bytes());
        Self::new(slave_id, 0x06, p)
    }

    /// Query Tulis Multiple Registers (FC 0x10)
    pub fn write_multiple_registers(slave_id: u8, start_addr: u16, values: &[u16]) -> Self {
        let byte_count = (values.len() * 2) as u8;
        let mut p = Vec::with_capacity(5 + values.len() * 2);
        p.extend_from_slice(&start_addr.to_be_bytes());
        p.extend_from_slice(&(values.len() as u16).to_be_bytes());
        p.push(byte_count);
        for &val in values {
            p.extend_from_slice(&val.to_be_bytes());
        }
        Self::new(slave_id, 0x10, p)
    }

    /// Serialisasi frame menjadi stream byte lengkap dengan CRC-16 Modbus (Little-Endian)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut frame = Vec::with_capacity(2 + self.payload.len() + 2);
        frame.push(self.slave_id);
        frame.push(self.function_code);
        frame.extend_from_slice(&self.payload);

        let crc = calculate_crc16_modbus(&frame);
        // Modbus RTU CRC16 disimpan dalam format Little-Endian (Low byte duluan)
        frame.push((crc & 0xFF) as u8);
        frame.push((crc >> 8) as u8);

        frame
    }

    /// Deserialisasi stream byte Modbus RTU dan validasi CRC
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 4 {
            return Err("Panjang frame Modbus RTU minimal 4 byte (Slave + FC + CRC)".to_string());
        }

        let data_len = bytes.len() - 2;
        let frame_data = &bytes[..data_len];
        let expected_crc = calculate_crc16_modbus(frame_data);
        let actual_crc = (bytes[data_len] as u16) | ((bytes[data_len + 1] as u16) << 8);

        if expected_crc != actual_crc {
            return Err(format!(
                "CRC-16 Modbus tidak cocok: dihitung 0x{:04X}, diterima 0x{:04X}",
                expected_crc, actual_crc
            ));
        }

        let slave_id = bytes[0];
        let function_code = bytes[1];
        let payload = bytes[2..data_len].to_vec();

        Ok(Self {
            slave_id,
            function_code,
            payload,
        })
    }
}

// ============================================================================
// 3. IN-MEMORY INDUSTRIAL REGISTER MAP & PLC STATE MACHINE
// ============================================================================

pub const MODBUS_EX_ILLEGAL_FUNCTION: u8 = 0x01;
pub const MODBUS_EX_ILLEGAL_DATA_ADDR: u8 = 0x02;
pub const MODBUS_EX_ILLEGAL_DATA_VAL: u8  = 0x03;

/// In-Memory Slave PLC State Simulator (Mendukung Coils, Discrete Inputs, Holding, Input Registers)
#[derive(Debug, Clone)]
pub struct IndustrialRegisterMap {
    pub slave_id: u8,
    pub coils: Vec<bool>,
    pub discrete_inputs: Vec<bool>,
    pub holding_registers: Vec<u16>,
    pub input_registers: Vec<u16>,
}

impl IndustrialRegisterMap {
    pub fn new(slave_id: u8, num_registers: usize) -> Self {
        Self {
            slave_id,
            coils: vec![false; num_registers],
            discrete_inputs: vec![false; num_registers],
            holding_registers: vec![0u16; num_registers],
            input_registers: vec![0u16; num_registers],
        }
    }

    /// Proses request Modbus RTU dan hasilkan response byte stream
    pub fn process_rtu_request(&mut self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = ModbusRtuFrame::from_bytes(request_bytes)?;
        if req.slave_id != self.slave_id && req.slave_id != 0 {
            // Abaikan frame jika bukan untuk slave ID ini (dan bukan broadcast 0)
            return Err("Slave ID tidak ditujukan untuk perangkat ini".to_string());
        }

        let resp = self.dispatch_pdu(req.slave_id, req.function_code, &req.payload);
        Ok(resp.to_bytes())
    }

    fn dispatch_pdu(&mut self, slave: u8, fc: u8, payload: &[u8]) -> ModbusRtuFrame {
        match fc {
            0x01 => self.handle_read_coils(slave, payload),
            0x03 => self.handle_read_holding_registers(slave, payload),
            0x04 => self.handle_read_input_registers(slave, payload),
            0x06 => self.handle_write_single_register(slave, payload),
            0x10 => self.handle_write_multiple_registers(slave, payload),
            _ => {
                // Function code error (bit 7 set)
                ModbusRtuFrame::new(slave, fc | 0x80, vec![MODBUS_EX_ILLEGAL_FUNCTION])
            }
        }
    }

    fn handle_read_coils(&self, slave: u8, payload: &[u8]) -> ModbusRtuFrame {
        if payload.len() < 4 {
            return ModbusRtuFrame::new(slave, 0x81, vec![MODBUS_EX_ILLEGAL_DATA_VAL]);
        }
        let addr = u16::from_be_bytes([payload[0], payload[1]]) as usize;
        let count = u16::from_be_bytes([payload[2], payload[3]]) as usize;

        if addr + count > self.coils.len() {
            return ModbusRtuFrame::new(slave, 0x81, vec![MODBUS_EX_ILLEGAL_DATA_ADDR]);
        }

        let byte_count = (count + 7) / 8;
        let mut data = vec![0u8; 1 + byte_count];
        data[0] = byte_count as u8;

        for i in 0..count {
            if self.coils[addr + i] {
                let byte_idx = 1 + (i / 8);
                let bit_idx = i % 8;
                data[byte_idx] |= 1 << bit_idx;
            }
        }

        ModbusRtuFrame::new(slave, 0x01, data)
    }

    fn handle_read_holding_registers(&self, slave: u8, payload: &[u8]) -> ModbusRtuFrame {
        if payload.len() < 4 {
            return ModbusRtuFrame::new(slave, 0x83, vec![MODBUS_EX_ILLEGAL_DATA_VAL]);
        }
        let addr = u16::from_be_bytes([payload[0], payload[1]]) as usize;
        let count = u16::from_be_bytes([payload[2], payload[3]]) as usize;

        if addr + count > self.holding_registers.len() {
            return ModbusRtuFrame::new(slave, 0x83, vec![MODBUS_EX_ILLEGAL_DATA_ADDR]);
        }

        let byte_count = (count * 2) as u8;
        let mut data = Vec::with_capacity(1 + (count * 2));
        data.push(byte_count);

        for i in 0..count {
            let val = self.holding_registers[addr + i];
            data.extend_from_slice(&val.to_be_bytes());
        }

        ModbusRtuFrame::new(slave, 0x03, data)
    }

    fn handle_read_input_registers(&self, slave: u8, payload: &[u8]) -> ModbusRtuFrame {
        if payload.len() < 4 {
            return ModbusRtuFrame::new(slave, 0x84, vec![MODBUS_EX_ILLEGAL_DATA_VAL]);
        }
        let addr = u16::from_be_bytes([payload[0], payload[1]]) as usize;
        let count = u16::from_be_bytes([payload[2], payload[3]]) as usize;

        if addr + count > self.input_registers.len() {
            return ModbusRtuFrame::new(slave, 0x84, vec![MODBUS_EX_ILLEGAL_DATA_ADDR]);
        }

        let byte_count = (count * 2) as u8;
        let mut data = Vec::with_capacity(1 + (count * 2));
        data.push(byte_count);

        for i in 0..count {
            let val = self.input_registers[addr + i];
            data.extend_from_slice(&val.to_be_bytes());
        }

        ModbusRtuFrame::new(slave, 0x04, data)
    }

    fn handle_write_single_register(&mut self, slave: u8, payload: &[u8]) -> ModbusRtuFrame {
        if payload.len() < 4 {
            return ModbusRtuFrame::new(slave, 0x86, vec![MODBUS_EX_ILLEGAL_DATA_VAL]);
        }
        let addr = u16::from_be_bytes([payload[0], payload[1]]) as usize;
        let val = u16::from_be_bytes([payload[2], payload[3]]);

        if addr >= self.holding_registers.len() {
            return ModbusRtuFrame::new(slave, 0x86, vec![MODBUS_EX_ILLEGAL_DATA_ADDR]);
        }

        self.holding_registers[addr] = val;
        // Echo kembali request payload
        ModbusRtuFrame::new(slave, 0x06, payload.to_vec())
    }

    fn handle_write_multiple_registers(&mut self, slave: u8, payload: &[u8]) -> ModbusRtuFrame {
        if payload.len() < 5 {
            return ModbusRtuFrame::new(slave, 0x90, vec![MODBUS_EX_ILLEGAL_DATA_VAL]);
        }
        let addr = u16::from_be_bytes([payload[0], payload[1]]) as usize;
        let count = u16::from_be_bytes([payload[2], payload[3]]) as usize;
        let byte_count = payload[4] as usize;

        if byte_count != count * 2 || payload.len() < 5 + byte_count {
            return ModbusRtuFrame::new(slave, 0x90, vec![MODBUS_EX_ILLEGAL_DATA_VAL]);
        }
        if addr + count > self.holding_registers.len() {
            return ModbusRtuFrame::new(slave, 0x90, vec![MODBUS_EX_ILLEGAL_DATA_ADDR]);
        }

        for i in 0..count {
            let offset = 5 + i * 2;
            let val = u16::from_be_bytes([payload[offset], payload[offset + 1]]);
            self.holding_registers[addr + i] = val;
        }

        // Response FC 0x10 mengembalikan addr dan count
        let mut resp_payload = Vec::with_capacity(4);
        resp_payload.extend_from_slice(&(addr as u16).to_be_bytes());
        resp_payload.extend_from_slice(&(count as u16).to_be_bytes());
        ModbusRtuFrame::new(slave, 0x10, resp_payload)
    }
}

// ============================================================================
// 4. INDUSTRIAL WORD CONVERSION (FLOAT32 & INT32 MULTI-REGISTER)
// ============================================================================

pub struct IndustrialWordConverter;

impl IndustrialWordConverter {
    /// Konversi Float32 ke sepasang Register 16-bit Big-Endian (ABCD)
    pub fn f32_to_be_registers(val: f32) -> (u16, u16) {
        let bits = val.to_bits();
        let high = (bits >> 16) as u16;
        let low = (bits & 0xFFFF) as u16;
        (high, low)
    }

    /// Konversi sepasang Register 16-bit Big-Endian (ABCD) ke Float32
    pub fn be_registers_to_f32(high: u16, low: u16) -> f32 {
        let bits = ((high as u32) << 16) | (low as u32);
        f32::from_bits(bits)
    }

    /// Konversi Float32 ke sepasang Register 16-bit Word-Swapped (CDAB)
    pub fn f32_to_word_swapped_registers(val: f32) -> (u16, u16) {
        let (high, low) = Self::f32_to_be_registers(val);
        (low, high)
    }

    /// Konversi sepasang Register 16-bit Word-Swapped (CDAB) ke Float32
    pub fn word_swapped_registers_to_f32(first: u16, second: u16) -> f32 {
        Self::be_registers_to_f32(second, first)
    }

    /// Konversi Int32 ke sepasang Register 16-bit
    pub fn i32_to_registers(val: i32) -> (u16, u16) {
        let u = val as u32;
        ((u >> 16) as u16, (u & 0xFFFF) as u16)
    }

    /// Konversi sepasang Register 16-bit ke Int32
    pub fn registers_to_i32(high: u16, low: u16) -> i32 {
        (((high as u32) << 16) | (low as u32)) as i32
    }
}
