//! Pure Sovereign Zero-Dependency Automotive Diagnostics & EV BMS Engine (pure_automotive.rs)
//!
//! 100% In-House Implementation:
//! - ISO 15765-2 (ISO-TP / DoCAN) Transport Protocol Segmenter & Reassembler
//! - ISO 14229-1 (UDS - Unified Diagnostic Services) Server & Client Session State Machine
//! - Seed/Key Security Access Challenge-Response Engine (Service 0x27)
//! - Electric Vehicle (EV) Battery Management System (BMS) Telemetry & Safety Guard
//! - Coulomb Counting State-of-Charge (SoC) & Thermal Runaway Early Warning
//! - Interoperability Bridges with `CanFrame`, `CanFilter`, and `Obd2Telemetry`

use serde::{Deserialize, Serialize};

// ============================================================================
// 1. ISO 15765-2 (ISO-TP / DoCAN) TRANSPORT LAYER
// ============================================================================

/// Tipe Protokol Frame Jaringan Transport CAN (N_PCI Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsoTpFrameType {
    SingleFrame,       // SF (0x0): Panjang payload 1..7 byte
    FirstFrame,        // FF (0x1): Frame pembuka multi-frame (panjang 8..4095 byte)
    ConsecutiveFrame,  // CF (0x2): Frame pecahan payload berurutan
    FlowControl,       // FC (0x3): Pengendali aliran (CTS, Wait, Overflow)
}

/// Status Aliran Flow Control ISO-TP
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlowStatus {
    ClearToSend, // 0x0
    Wait,        // 0x1
    Overflow,    // 0x2
}

/// Segmenter & Reassembler ISO-TP
pub struct IsoTpTransport;

impl IsoTpTransport {
    /// Segmentasi paket data UDS diagnostik menjadi deretan frame CAN 8-byte
    pub fn segment_message(can_id: u32, payload: &[u8]) -> Vec<crate::automotive::CanFrame> {
        let mut frames = Vec::new();
        let len = payload.len();

        if len == 0 {
            return frames;
        }

        if len <= 7 {
            // Single Frame (SF)
            let mut data = [0u8; 8];
            data[0] = len as u8; // PCI: 0x00 | length
            data[1..1 + len].copy_from_slice(payload);
            frames.push(crate::automotive::CanFrame {
                id: can_id,
                is_extended: false,
                is_rtr: false,
                dlc: 8,
                data,
            });
        } else {
            // First Frame (FF)
            let mut data = [0u8; 8];
            let len_u16 = len.min(4095) as u16;
            data[0] = 0x10 | ((len_u16 >> 8) as u8 & 0x0F);
            data[1] = (len_u16 & 0xFF) as u8;
            data[2..8].copy_from_slice(&payload[0..6]);
            frames.push(crate::automotive::CanFrame {
                id: can_id,
                is_extended: false,
                is_rtr: false,
                dlc: 8,
                data,
            });

            // Consecutive Frames (CF)
            let mut offset = 6;
            let mut seq_idx = 1u8;

            while offset < len {
                let mut cf_data = [0u8; 8];
                cf_data[0] = 0x20 | (seq_idx & 0x0F);
                let chunk_len = (len - offset).min(7);
                cf_data[1..1 + chunk_len].copy_from_slice(&payload[offset..offset + chunk_len]);
                frames.push(crate::automotive::CanFrame {
                    id: can_id,
                    is_extended: false,
                    is_rtr: false,
                    dlc: 8,
                    data: cf_data,
                });
                offset += chunk_len;
                seq_idx = (seq_idx + 1) & 0x0F;
            }
        }

        frames
    }

    /// Rekonstruksi paket diagnostik lengkap dari deretan frame CAN
    pub fn reassemble_frames(frames: &[crate::automotive::CanFrame]) -> Result<Vec<u8>, String> {
        if frames.is_empty() {
            return Err("Daftar frame CAN kosong".to_string());
        }

        let first = &frames[0];
        let pci_type = (first.data[0] >> 4) & 0x0F;

        if pci_type == 0 {
            // Single Frame
            let sf_len = (first.data[0] & 0x0F) as usize;
            if sf_len > 7 {
                return Err("Panjang Single Frame tidak valid (> 7)".to_string());
            }
            return Ok(first.data[1..1 + sf_len].to_vec());
        }

        if pci_type != 1 {
            return Err("Frame awal harus bertipe First Frame (0x1) atau Single Frame (0x0)".to_string());
        }

        // First Frame
        let total_len = (((first.data[0] & 0x0F) as usize) << 8) | (first.data[1] as usize);
        let mut assembled = Vec::with_capacity(total_len);
        assembled.extend_from_slice(&first.data[2..8]);

        let mut expected_sn = 1u8;
        for frame in &frames[1..] {
            if assembled.len() >= total_len {
                break;
            }
            let cf_pci = (frame.data[0] >> 4) & 0x0F;
            if cf_pci != 2 {
                continue; // Lewati Flow Control atau frame non-CF
            }
            let sn = frame.data[0] & 0x0F;
            if sn != expected_sn {
                return Err(format!("Urutan Consecutive Frame rusak: diharapkan {}, ditemukan {}", expected_sn, sn));
            }
            expected_sn = (expected_sn + 1) & 0x0F;

            let remaining = total_len - assembled.len();
            let chunk_len = remaining.min(7);
            assembled.extend_from_slice(&frame.data[1..1 + chunk_len]);
        }

        if assembled.len() < total_len {
            return Err(format!("Payload belum lengkap (diterima {} / {} byte)", assembled.len(), total_len));
        }

        Ok(assembled)
    }
}

// ============================================================================
// 2. ISO 14229-1 (UDS - UNIFIED DIAGNOSTIC SERVICES) ENGINE
// ============================================================================

/// Identifikasi Layanan UDS (Service Identifier - SID)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UdsServiceId {
    DiagnosticSessionControl, // 0x10
    EcuReset,                 // 0x11
    SecurityAccess,           // 0x27
    TesterPresent,            // 0x3E
    ReadDataByIdentifier,     // 0x22
    WriteDataByIdentifier,    // 0x2E
    RoutineControl,           // 0x31
    NegativeResponse,         // 0x7F
}

/// Sesi Diagnostik ECU Otomotif
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSessionType {
    DefaultSession,            // 0x01
    ProgrammingSession,        // 0x02
    ExtendedDiagnosticSession, // 0x03
    SafetySystemDiagnostic,    // 0x04
}

/// Kode Respons Negatif UDS (NRC - Negative Response Code)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UdsNrc {
    ServiceNotSupported,            // 0x11
    SubFunctionNotSupported,        // 0x12
    IncorrectMessageLengthOrFormat, // 0x13
    ConditionsNotCorrect,           // 0x22
    RequestSequenceError,           // 0x24
    RequestOutOfRange,              // 0x31
    SecurityAccessDenied,           // 0x33
    InvalidKey,                     // 0x35
    ExceedNumberOfAttempts,         // 0x36
}

impl UdsNrc {
    pub fn to_byte(self) -> u8 {
        match self {
            UdsNrc::ServiceNotSupported => 0x11,
            UdsNrc::SubFunctionNotSupported => 0x12,
            UdsNrc::IncorrectMessageLengthOrFormat => 0x13,
            UdsNrc::ConditionsNotCorrect => 0x22,
            UdsNrc::RequestSequenceError => 0x24,
            UdsNrc::RequestOutOfRange => 0x31,
            UdsNrc::SecurityAccessDenied => 0x33,
            UdsNrc::InvalidKey => 0x35,
            UdsNrc::ExceedNumberOfAttempts => 0x36,
        }
    }
}

/// State Machine Server Diagnostik ECU Otomotif
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcuDiagnosticServer {
    pub session: DiagnosticSessionType,
    pub is_security_unlocked: bool,
    pub current_seed: u32,
    pub vin: String,
    pub battery_health_pct: f32,
}

impl EcuDiagnosticServer {
    pub fn new(vin: &str) -> Self {
        Self {
            session: DiagnosticSessionType::DefaultSession,
            is_security_unlocked: false,
            current_seed: 0,
            vin: vin.to_string(),
            battery_health_pct: 98.5,
        }
    }

    /// Pemroses Permintaan UDS Murni
    pub fn handle_request(&mut self, request: &[u8]) -> Vec<u8> {
        if request.is_empty() {
            return vec![0x7F, 0x00, UdsNrc::IncorrectMessageLengthOrFormat.to_byte()];
        }

        let sid = request[0];
        match sid {
            0x10 => self.handle_session_control(request),
            0x27 => self.handle_security_access(request),
            0x22 => self.handle_read_did(request),
            0x3E => self.handle_tester_present(request),
            _ => vec![0x7F, sid, UdsNrc::ServiceNotSupported.to_byte()],
        }
    }

    fn handle_session_control(&mut self, req: &[u8]) -> Vec<u8> {
        if req.len() < 2 {
            return vec![0x7F, 0x10, UdsNrc::IncorrectMessageLengthOrFormat.to_byte()];
        }
        let sub_func = req[1] & 0x7F;
        match sub_func {
            0x01 => {
                self.session = DiagnosticSessionType::DefaultSession;
                self.is_security_unlocked = false;
                vec![0x50, 0x01, 0x00, 0x32, 0x01, 0xF4] // P2 server max timing
            }
            0x02 => {
                if !self.is_security_unlocked {
                    return vec![0x7F, 0x10, UdsNrc::SecurityAccessDenied.to_byte()];
                }
                self.session = DiagnosticSessionType::ProgrammingSession;
                vec![0x50, 0x02, 0x00, 0x32, 0x01, 0xF4]
            }
            0x03 => {
                self.session = DiagnosticSessionType::ExtendedDiagnosticSession;
                vec![0x50, 0x03, 0x00, 0x32, 0x01, 0xF4]
            }
            _ => vec![0x7F, 0x10, UdsNrc::SubFunctionNotSupported.to_byte()],
        }
    }

    fn handle_security_access(&mut self, req: &[u8]) -> Vec<u8> {
        if req.len() < 2 {
            return vec![0x7F, 0x27, UdsNrc::IncorrectMessageLengthOrFormat.to_byte()];
        }
        let sub_func = req[1];

        if sub_func == 0x01 {
            // Request Seed Level 1
            self.current_seed = 0x5A7C319E; // Seed pseudorandom deterministik
            let s_bytes = self.current_seed.to_be_bytes();
            vec![0x67, 0x01, s_bytes[0], s_bytes[1], s_bytes[2], s_bytes[3]]
        } else if sub_func == 0x02 {
            // Send Key Level 1
            if req.len() < 6 {
                return vec![0x7F, 0x27, UdsNrc::IncorrectMessageLengthOrFormat.to_byte()];
            }
            let key = u32::from_be_bytes([req[2], req[3], req[4], req[5]]);
            // Algoritma verifikasi Kunci: Key = Seed ^ 0xA5A5A5A5 + 0x1234
            let expected_key = (self.current_seed ^ 0xA5A5A5A5).wrapping_add(0x1234);
            if key == expected_key {
                self.is_security_unlocked = true;
                vec![0x67, 0x02] // Positive response
            } else {
                vec![0x7F, 0x27, UdsNrc::InvalidKey.to_byte()]
            }
        } else {
            vec![0x7F, 0x27, UdsNrc::SubFunctionNotSupported.to_byte()]
        }
    }

    fn handle_read_did(&self, req: &[u8]) -> Vec<u8> {
        if req.len() < 3 {
            return vec![0x7F, 0x22, UdsNrc::IncorrectMessageLengthOrFormat.to_byte()];
        }
        let did = u16::from_be_bytes([req[1], req[2]]);
        match did {
            0xF190 => {
                // VIN (Vehicle Identification Number)
                let mut resp = vec![0x62, 0xF1, 0x90];
                resp.extend_from_slice(self.vin.as_bytes());
                resp
            }
            0x0100 => {
                // Battery Health State of Health (SoH)
                let mut resp = vec![0x62, 0x01, 0x00];
                let val = (self.battery_health_pct * 10.0) as u16;
                resp.extend_from_slice(&val.to_be_bytes());
                resp
            }
            _ => vec![0x7F, 0x22, UdsNrc::RequestOutOfRange.to_byte()],
        }
    }

    fn handle_tester_present(&self, req: &[u8]) -> Vec<u8> {
        let sub = if req.len() > 1 { req[1] } else { 0x00 };
        vec![0x7E, sub]
    }
}

// ============================================================================
// 3. ELECTRIC VEHICLE (EV) BATTERY MANAGEMENT SYSTEM (BMS)
// ============================================================================

/// Status Keamanan & Peringatan Dini Baterai Traksi EV
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatterySafetyStatus {
    Normal,
    CellImbalanceWarning,
    HighTemperatureThrottling,
    ThermalRunawayHazard,
    UnderVoltageCritical,
}

/// Telemetri Paket Baterai Kendaraan Listrik (EV Traction Battery Pack)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvBatteryTelemetry {
    pub pack_voltage_v: f64,
    pub current_draw_a: f64, // Negatif = Charging, Positif = Discharging
    pub state_of_charge_pct: f64,
    pub cell_voltages_mv: Vec<u16>,
    pub temperature_celsius: f64,
    pub safety_status: BatterySafetyStatus,
}

pub struct EvBmsEngine {
    pub nominal_capacity_ah: f64,
    pub current_soc_pct: f64,
}

impl EvBmsEngine {
    pub fn new(capacity_ah: f64, initial_soc: f64) -> Self {
        Self {
            nominal_capacity_ah: capacity_ah,
            current_soc_pct: initial_soc.clamp(0.0, 100.0),
        }
    }

    /// Update State of Charge (SoC) dengan Algoritma Coulomb Counting
    /// delta_soc = (I * dt_sec / (capacity_ah * 3600)) * 100%
    pub fn update_coulomb_counting(&mut self, current_draw_a: f64, dt_seconds: f64) -> f64 {
        let delta_ah = (current_draw_a * dt_seconds) / 3600.0;
        let delta_soc_pct = (delta_ah / self.nominal_capacity_ah) * 100.0;
        self.current_soc_pct = (self.current_soc_pct - delta_soc_pct).clamp(0.0, 100.0);
        self.current_soc_pct
    }

    /// Evaluasi Keselamatan Sel & Indikasi Thermal Runaway
    pub fn evaluate_safety(&self, cell_voltages_mv: &[u16], temp_c: f64, temp_rate_deg_per_sec: f64) -> BatterySafetyStatus {
        if temp_c > 65.0 || temp_rate_deg_per_sec > 1.2 {
            return BatterySafetyStatus::ThermalRunawayHazard;
        }

        if temp_c > 50.0 {
            return BatterySafetyStatus::HighTemperatureThrottling;
        }

        if let (Some(&min_v), Some(&max_v)) = (cell_voltages_mv.iter().min(), cell_voltages_mv.iter().max()) {
            if min_v < 2800 {
                return BatterySafetyStatus::UnderVoltageCritical;
            }
            if (max_v - min_v) > 250 {
                // Selisih tegangan antar sel > 250 mV menandakan imbalance berat
                return BatterySafetyStatus::CellImbalanceWarning;
            }
        }

        BatterySafetyStatus::Normal
    }
}

// ============================================================================
// 4. BRIDGES DENGAN SUBSISTEM AUTOMOTIVE / OBD2 LEGACY
// ============================================================================

impl From<&EvBatteryTelemetry> for crate::automotive::obd2::Obd2Telemetry {
    fn from(bms: &EvBatteryTelemetry) -> Self {
        crate::automotive::obd2::Obd2Telemetry {
            engine_rpm: 0.0, // EV motor RPM jika dihubungkan
            vehicle_speed_kmh: 0,
            coolant_temp_celsius: bms.temperature_celsius as i16,
            throttle_position_pct: 0.0,
            fuel_level_pct: bms.state_of_charge_pct,
            ev_battery_soc_pct: bms.state_of_charge_pct,
        }
    }
}
