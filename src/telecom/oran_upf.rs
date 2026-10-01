use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// 5G Open-RAN User Plane Function (UPF) Engine
/// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SessionState {
    Active,
    Idle,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PduSession {
    pub session_id: u32,
    pub supi: String,            // Subscription Permanent Identifier (IMSI 5G)
    pub dnn: String,             // Data Network Name (e.g., "internet", "ims", "private-cloud")
    pub teid_uplink: u32,        // Tunnel Endpoint Identifier Uplink
    pub teid_downlink: u32,      // Tunnel Endpoint Identifier Downlink
    pub qfi: u8,                 // QoS Flow Identifier (1-64)
    pub max_bitrate_mbps: u32,
    pub state: SessionState,
    pub bytes_transmitted: u64,
    pub packets_processed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpfThroughputReport {
    pub total_active_sessions: usize,
    pub aggregate_bandwidth_gbps: f64,
    pub total_packets_forwarded: u64,
    pub dropped_packets_qos: u64,
    pub avg_latency_us: f64,
}

pub struct OpenRanUpf {
    pub node_id: String,
    pub ip_address: String,
    pub sessions: HashMap<u32, PduSession>,
    pub total_rx_packets: u64,
    pub total_tx_packets: u64,
    pub dropped_packets: u64,
}

impl OpenRanUpf {
    pub fn new(node_id: &str, ip_address: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            ip_address: ip_address.to_string(),
            sessions: HashMap::new(),
            total_rx_packets: 0,
            total_tx_packets: 0,
            dropped_packets: 0,
        }
    }

    /// Membuat atau mengikat PDU session baru (N4 interface session establishment)
    pub fn create_session(&mut self, supi: &str, dnn: &str, qfi: u8, max_bitrate_mbps: u32) -> u32 {
        let session_id = (self.sessions.len() as u32) + 1000;
        let teid_up = session_id * 2 + 1;
        let teid_down = session_id * 2 + 2;

        let session = PduSession {
            session_id,
            supi: supi.to_string(),
            dnn: dnn.to_string(),
            teid_uplink: teid_up,
            teid_downlink: teid_down,
            qfi,
            max_bitrate_mbps,
            state: SessionState::Active,
            bytes_transmitted: 0,
            packets_processed: 0,
        };

        self.sessions.insert(session_id, session);
        session_id
    }

    /// Meneruskan paket IP data plane (N3 / N6 forwarding)
    pub fn forward_packet(&mut self, session_id: u32, packet_size_bytes: usize) -> Result<bool, String> {
        self.total_rx_packets += 1;
        if let Some(session) = self.sessions.get_mut(&session_id) {
            if session.state != SessionState::Active {
                self.dropped_packets += 1;
                return Err("PDU session tidak aktif".to_string());
            }

            session.bytes_transmitted += packet_size_bytes as u64;
            session.packets_processed += 1;
            self.total_tx_packets += 1;
            Ok(true)
        } else {
            self.dropped_packets += 1;
            Err(format!("PDU Session ID {} tidak ditemukan di UPF {}", session_id, self.node_id))
        }
    }

    /// Laporan performa throughput UPF
    pub fn generate_throughput_report(&self) -> UpfThroughputReport {
        let active = self.sessions.values().filter(|s| s.state == SessionState::Active).count();
        let total_bytes: u64 = self.sessions.values().map(|s| s.bytes_transmitted).sum();
        let total_gb = (total_bytes as f64) * 8.0 / 1_000_000_000.0;

        UpfThroughputReport {
            total_active_sessions: active,
            aggregate_bandwidth_gbps: total_gb,
            total_packets_forwarded: self.total_tx_packets,
            dropped_packets_qos: self.dropped_packets,
            avg_latency_us: 12.5, // 12.5 us ultra-low latency DPDK baseline
        }
    }
}
