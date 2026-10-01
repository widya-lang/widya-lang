use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 5G Network Slicing & SLA Enforcer (eMBB, URLLC, mMTC)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SliceType {
    Embb,   // Enhanced Mobile Broadband (High Throughput, e.g. 4K/8K, VR)
    Urllc,  // Ultra-Reliable Low-Latency Communication (Sub-1ms, e.g. Autonomous Vehicle, Surgery)
    Mmtc,   // Massive Machine Type Communications (High Density IoT)
    Custom(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceSlaSpec {
    pub slice_id: u32,
    pub slice_type: SliceType,
    pub guaranteed_bitrate_mbps: u32,
    pub max_latency_ms: f64,
    pub max_jitter_ms: f64,
    pub packet_loss_rate_max: f64,
    pub priority_weight: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliceTelemetry {
    pub slice_id: u32,
    pub current_bitrate_mbps: f64,
    pub current_latency_ms: f64,
    pub current_packet_loss: f64,
    pub is_sla_violated: bool,
    pub active_ue_count: usize,
}

pub struct NetworkSliceManager {
    pub slices: HashMap<u32, SliceSlaSpec>,
    pub telemetry: HashMap<u32, SliceTelemetry>,
}

impl NetworkSliceManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            slices: HashMap::new(),
            telemetry: HashMap::new(),
        };

        // Standard 5G 3GPP default slices (S-NSSAI SST 1, 2, 3)
        mgr.register_slice(1, SliceType::Embb, 1000, 15.0, 5.0, 0.001, 5);
        mgr.register_slice(2, SliceType::Urllc, 100, 1.0, 0.2, 0.00001, 10);
        mgr.register_slice(3, SliceType::Mmtc, 50, 50.0, 20.0, 0.01, 2);

        mgr
    }

    pub fn register_slice(&mut self, id: u32, st: SliceType, gbr: u32, max_lat: f64, max_jit: f64, plr: f64, prio: u8) {
        let spec = SliceSlaSpec {
            slice_id: id,
            slice_type: st,
            guaranteed_bitrate_mbps: gbr,
            max_latency_ms: max_lat,
            max_jitter_ms: max_jit,
            packet_loss_rate_max: plr,
            priority_weight: prio,
        };

        let telem = SliceTelemetry {
            slice_id: id,
            current_bitrate_mbps: 0.0,
            current_latency_ms: max_lat * 0.5,
            current_packet_loss: 0.0,
            is_sla_violated: false,
            active_ue_count: 0,
        };

        self.slices.insert(id, spec);
        self.telemetry.insert(id, telem);
    }

    /// Evaluasi metrik real-time terhadap SLA
    pub fn evaluate_sla(&mut self, slice_id: u32, measured_latency_ms: f64, measured_loss: f64, current_mbps: f64, ue_count: usize) -> bool {
        if let Some(spec) = self.slices.get(&slice_id) {
            let violated = measured_latency_ms > spec.max_latency_ms || measured_loss > spec.packet_loss_rate_max;
            if let Some(t) = self.telemetry.get_mut(&slice_id) {
                t.current_latency_ms = measured_latency_ms;
                t.current_packet_loss = measured_loss;
                t.current_bitrate_mbps = current_mbps;
                t.active_ue_count = ue_count;
                t.is_sla_violated = violated;
            }
            !violated
        } else {
            false
        }
    }
}
