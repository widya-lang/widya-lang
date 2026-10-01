use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// IEC 61850 Substation Automation System & SCL Model
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogicalNodeClass {
    XCBR, // Circuit Breaker
    XSWI, // Circuit Switch
    MMXU, // Measurement Unit (Voltage, Current, Power)
    PTOC, // Time Overcurrent Protection
    PDIS, // Distance Protection
    CSWI, // Switch Controller
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAttributeValue {
    pub name: String,
    pub val_float: Option<f64>,
    pub val_bool: Option<bool>,
    pub val_str: Option<String>,
    pub quality: u32, // 0 = Good
    pub timestamp_epoch_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalNode {
    pub name: String,
    pub class: LogicalNodeClass,
    pub attributes: HashMap<String, DataAttributeValue>,
}

pub struct IedModel {
    pub ied_name: String,
    pub ip_address: String,
    pub logical_nodes: HashMap<String, LogicalNode>,
}

impl IedModel {
    pub fn new(name: &str, ip: &str) -> Self {
        Self {
            ied_name: name.to_string(),
            ip_address: ip.to_string(),
            logical_nodes: HashMap::new(),
        }
    }

    pub fn add_logical_node(&mut self, name: &str, class: LogicalNodeClass) {
        let ln = LogicalNode {
            name: name.to_string(),
            class,
            attributes: HashMap::new(),
        };
        self.logical_nodes.insert(name.to_string(), ln);
    }

    pub fn update_float_attribute(&mut self, ln_name: &str, attr_name: &str, val: f64) -> Result<(), String> {
        if let Some(ln) = self.logical_nodes.get_mut(ln_name) {
            let attr = DataAttributeValue {
                name: attr_name.to_string(),
                val_float: Some(val),
                val_bool: None,
                val_str: None,
                quality: 0,
                timestamp_epoch_ms: 1775000000000,
            };
            ln.attributes.insert(attr_name.to_string(), attr);
            Ok(())
        } else {
            Err(format!("Logical node {} tidak ditemukan di IED {}", ln_name, self.ied_name))
        }
    }
}
