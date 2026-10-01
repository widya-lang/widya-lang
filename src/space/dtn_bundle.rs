use serde::{Deserialize, Serialize};

/// ============================================================================
/// RFC 9171 Delay-Tolerant Networking (DTN) Bundle Protocol Version 7
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointId {
    pub uri_scheme: String,  // e.g. "dtn" or "ipn"
    pub ssp: String,         // Scheme-Specific Part e.g. "//earth-station-1/telemetry"
}

impl EndpointId {
    pub fn new(uri: &str) -> Self {
        let parts: Vec<&str> = uri.splitn(2, ':').collect();
        if parts.len() == 2 {
            Self {
                uri_scheme: parts[0].to_string(),
                ssp: parts[1].to_string(),
            }
        } else {
            Self {
                uri_scheme: "dtn".to_string(),
                ssp: uri.to_string(),
            }
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.uri_scheme, self.ssp)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtnBundle {
    pub version: u8, // Always 7
    pub bundle_processing_flags: u64,
    pub crc_type: u8,
    pub destination_eid: EndpointId,
    pub source_node_eid: EndpointId,
    pub report_to_eid: EndpointId,
    pub creation_timestamp_ms: u64,
    pub sequence_number: u64,
    pub lifetime_ms: u64,
    pub payload_block: Vec<u8>,
}

impl DtnBundle {
    pub fn new(source: &str, destination: &str, payload: Vec<u8>, lifetime_sec: u64) -> Self {
        Self {
            version: 7,
            bundle_processing_flags: 0x0001, // Custody transfer requested
            crc_type: 1, // Standard CRC-16
            destination_eid: EndpointId::new(destination),
            source_node_eid: EndpointId::new(source),
            report_to_eid: EndpointId::new(source),
            creation_timestamp_ms: 1775000000000,
            sequence_number: 1,
            lifetime_ms: lifetime_sec * 1000,
            payload_block: payload,
        }
    }

    /// Evaluasi apakah bundle telah kedaluwarsa berdasarkan waktu saat ini
    pub fn is_expired(&self, current_time_ms: u64) -> bool {
        current_time_ms > (self.creation_timestamp_ms + self.lifetime_ms)
    }
}
