use serde::{Deserialize, Serialize};

/// ============================================================================
/// ISO 11783 (ISOBUS) Virtual Terminal (VT) & Implement Management
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsoObjectType {
    WorkingSet = 0,
    DataMask = 1,
    Container = 3,
    SoftKeyMask = 4,
    Button = 6,
    InputNumber = 9,
    OutputString = 11,
    Meter = 17,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsobusObject {
    pub object_id: u16,
    pub object_type: IsoObjectType,
    pub background_color: u8,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementTelemetry {
    pub implement_name: String,
    pub isobus_address: u8,
    pub boom_section_count: u8,
    pub active_sections_bitmask: u32,
    pub current_speed_kmh: f64,
    pub target_rate_liters_per_hectare: f64,
    pub actual_rate_liters_per_hectare: f64,
}

pub struct VirtualTerminalClient {
    pub client_name: String,
    pub object_pool: Vec<IsobusObject>,
    pub telemetry: ImplementTelemetry,
}

impl VirtualTerminalClient {
    pub fn new(implement_name: &str, isobus_addr: u8, sections: u8) -> Self {
        Self {
            client_name: implement_name.to_string(),
            object_pool: Vec::new(),
            telemetry: ImplementTelemetry {
                implement_name: implement_name.to_string(),
                isobus_address: isobus_addr,
                boom_section_count: sections,
                active_sections_bitmask: (1 << sections) - 1,
                current_speed_kmh: 0.0,
                target_rate_liters_per_hectare: 150.0,
                actual_rate_liters_per_hectare: 150.0,
            },
        }
    }

    pub fn register_object(&mut self, id: u16, obj_type: IsoObjectType, w: u16, h: u16) {
        self.object_pool.push(IsobusObject {
            object_id: id,
            object_type: obj_type,
            background_color: 0xFF,
            width: w,
            height: h,
        });
    }

    pub fn set_speed(&mut self, speed_kmh: f64) {
        self.telemetry.current_speed_kmh = speed_kmh;
    }
}
