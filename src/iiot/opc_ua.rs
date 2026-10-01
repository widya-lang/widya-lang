use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// OPC-UA (IEC 62541) Client & Server Model
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OpcUaDataType {
    Boolean,
    Int32,
    Int64,
    Float,
    Double,
    String,
    DateTime,
    ByteString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpcUaNodeId {
    pub namespace_index: u16,
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpcUaVariableNode {
    pub node_id: OpcUaNodeId,
    pub browse_name: String,
    pub data_type: OpcUaDataType,
    pub value_string: String,
    pub status_code: u32, // 0x00000000 = Good
    pub timestamp_epoch_ms: u64,
}

pub struct OpcUaServer {
    pub endpoint_url: String,
    pub server_name: String,
    pub address_space: HashMap<String, OpcUaVariableNode>,
}

impl OpcUaServer {
    pub fn new(endpoint_url: &str, server_name: &str) -> Self {
        Self {
            endpoint_url: endpoint_url.to_string(),
            server_name: server_name.to_string(),
            address_space: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, ns: u16, id: &str, name: &str, data_type: OpcUaDataType, initial_val: &str) {
        let node_id = OpcUaNodeId {
            namespace_index: ns,
            identifier: id.to_string(),
        };

        let node = OpcUaVariableNode {
            node_id,
            browse_name: name.to_string(),
            data_type,
            value_string: initial_val.to_string(),
            status_code: 0x00000000,
            timestamp_epoch_ms: 1775000000000,
        };

        self.address_space.insert(format!("ns={};s={}", ns, id), node);
    }

    pub fn read_variable(&self, ns: u16, id: &str) -> Option<&OpcUaVariableNode> {
        let key = format!("ns={};s={}", ns, id);
        self.address_space.get(&key)
    }

    pub fn write_variable(&mut self, ns: u16, id: &str, new_val: &str) -> Result<bool, String> {
        let key = format!("ns={};s={}", ns, id);
        if let Some(node) = self.address_space.get_mut(&key) {
            node.value_string = new_val.to_string();
            node.status_code = 0x00000000;
            Ok(true)
        } else {
            Err(format!("Node {} tidak ditemukan di OPC UA address space", key))
        }
    }
}
