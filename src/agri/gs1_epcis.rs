use serde::{Deserialize, Serialize};

/// ============================================================================
/// GS1 EPCIS 2.0 Farm-to-Fork Supply Chain Traceability
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EpcisEventType {
    ObjectEvent,
    AggregationEvent,
    TransformationEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpcisTraceabilityRecord {
    pub event_type: EpcisEventType,
    pub event_time_iso: String,
    pub epc_list: Vec<String>,       // e.g. ["urn:epc:id:sgtin:8991234.012345.1001"]
    pub action: String,              // "ADD", "OBSERVE", "DELETE"
    pub biz_step: String,            // e.g. "urn:epcglobal:cbv:bizstep:harvesting"
    pub disposition: String,         // e.g. "urn:epcglobal:cbv:disp:in_progress"
    pub read_point_gln: String,      // Global Location Number
    pub certification_uri: String,   // Organic / Halal / RSPO cert link
}

pub struct EpcisEventStore {
    pub events: Vec<EpcisTraceabilityRecord>,
}

impl EpcisEventStore {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn record_harvest(
        &mut self,
        epc: &str,
        gln_farm: &str,
        cert_uri: &str,
    ) {
        self.events.push(EpcisTraceabilityRecord {
            event_type: EpcisEventType::ObjectEvent,
            event_time_iso: "2026-10-01T10:00:00Z".to_string(),
            epc_list: vec![epc.to_string()],
            action: "ADD".to_string(),
            biz_step: "urn:epcglobal:cbv:bizstep:harvesting".to_string(),
            disposition: "urn:epcglobal:cbv:disp:active".to_string(),
            read_point_gln: gln_farm.to_string(),
            certification_uri: cert_uri.to_string(),
        });
    }
}
