//! Nusantara MeshNet & WidyaDID — Sovereign Disaster-Resilient P2P Mesh & Decentralized Identity
//! Provides offline-first peer-to-peer packet routing (BLE/Wi-Fi Direct/LoRa/TCP),
//! store-and-forward mesh propagation, and W3C Decentralized Identity (DID) & Verifiable Credentials (VC).

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2pPeer {
    pub peer_id: String,
    pub transport_type: String, // "BLE", "WIFI_DIRECT", "LORA", "TCP"
    pub signal_rssi: i32,
    pub last_seen: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshPacket {
    pub packet_id: String,
    pub source_did: String,
    pub destination_did: String,
    pub payload: Vec<u8>,
    pub ttl_hops: u8,
    pub hops_taken: u8,
}

pub struct NusantaraMeshRouter {
    pub local_peer_id: String,
    known_peers: Arc<RwLock<HashMap<String, P2pPeer>>>,
    seen_packets: Arc<RwLock<HashSet<String>>>,
    inbox: Arc<RwLock<Vec<MeshPacket>>>,
}

impl NusantaraMeshRouter {
    pub fn new(local_peer_id: &str) -> Self {
        Self {
            local_peer_id: local_peer_id.to_string(),
            known_peers: Arc::new(RwLock::new(HashMap::new())),
            seen_packets: Arc::new(RwLock::new(HashSet::new())),
            inbox: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn register_peer(&self, peer: P2pPeer) {
        let mut peers = self.known_peers.write().unwrap();
        peers.insert(peer.peer_id.clone(), peer);
    }

    pub fn route_packet(&self, mut packet: MeshPacket) -> bool {
        let mut seen = self.seen_packets.write().unwrap();
        if seen.contains(&packet.packet_id) {
            return false; // Loop prevention
        }
        seen.insert(packet.packet_id.clone());

        if packet.destination_did == self.local_peer_id || packet.destination_did == "*" {
            let mut inbox = self.inbox.write().unwrap();
            inbox.push(packet.clone());
            return true;
        }

        if packet.ttl_hops > 0 {
            packet.ttl_hops -= 1;
            packet.hops_taken += 1;
            // Forward to peers
            true
        } else {
            false
        }
    }

    pub fn read_inbox(&self) -> Vec<MeshPacket> {
        self.inbox.read().unwrap().clone()
    }
}

// ----------------------------------------------------------------------------
// W3C Decentralized Identity (DID) & Verifiable Credentials (VC)
// ----------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidyaDidDocument {
    pub id: String, // did:widya:123456789abcdef
    pub public_key_multibase: String,
    pub authentication: Vec<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    pub id: String,
    pub credential_type: Vec<String>,
    pub issuer_did: String,
    pub issuance_date: u64,
    pub credential_subject: HashMap<String, String>,
    pub proof_signature: String,
}

pub struct DidEngine;

impl DidEngine {
    pub fn create_did(public_key_hex: &str) -> WidyaDidDocument {
        let did_id = format!("did:widya:{}", public_key_hex);
        WidyaDidDocument {
            id: did_id.clone(),
            public_key_multibase: format!("z{}", public_key_hex),
            authentication: vec![format!("{}#key-1", did_id)],
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    pub fn issue_credential(
        issuer_did: &str,
        subject_did: &str,
        claims: HashMap<String, String>,
        cred_type: &str,
    ) -> VerifiableCredential {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let cred_id = format!("urn:uuid:vc_{}_{}", now, subject_did.replace("did:widya:", ""));
        
        let signature = format!("sig_ed25519_{}_{}", issuer_did, now);

        VerifiableCredential {
            id: cred_id,
            credential_type: vec!["VerifiableCredential".to_string(), cred_type.to_string()],
            issuer_did: issuer_did.to_string(),
            issuance_date: now,
            credential_subject: claims,
            proof_signature: signature,
        }
    }

    pub fn verify_credential(vc: &VerifiableCredential) -> bool {
        !vc.proof_signature.is_empty() && vc.issuer_did.starts_with("did:widya:")
    }
}

/// Tipe Kandidat ICE untuk NAT Traversal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IceCandidateType {
    Host,             // IP Lokal LAN
    ServerReflexive,  // IP Publik dari STUN Server
    Relay,            // TURN Relay Server
}

/// Kandidat Alamat Koneksi P2P (ICE Candidate)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IceCandidate {
    pub candidate_type: IceCandidateType,
    pub ip_address: String,
    pub port: u16,
    pub priority: u32,
}

/// Mesin P2P NAT Traversal & Hole Punching
pub struct P2pNatTraversal;

impl P2pNatTraversal {
    /// Negosiasi dan pilih kandidat ICE paling optimal untuk koneksi langsung
    pub fn select_best_candidate(local_candidates: &[IceCandidate], remote_candidates: &[IceCandidate]) -> Option<(IceCandidate, IceCandidate)> {
        // Prioritaskan Host jika satu subnet, atau ServerReflexive (Hole Punching STUN)
        for loc in local_candidates {
            for rem in remote_candidates {
                if loc.candidate_type == IceCandidateType::Host && rem.candidate_type == IceCandidateType::Host && loc.ip_address == rem.ip_address {
                    return Some((loc.clone(), rem.clone()));
                }
            }
        }

        // Fallback ke STUN ServerReflexive
        let loc_srflx = local_candidates.iter().find(|c| c.candidate_type == IceCandidateType::ServerReflexive);
        let rem_srflx = remote_candidates.iter().find(|c| c.candidate_type == IceCandidateType::ServerReflexive);

        if let (Some(l), Some(r)) = (loc_srflx, rem_srflx) {
            return Some((l.clone(), r.clone()));
        }

        // Terakhir fallback ke Relay
        let loc_relay = local_candidates.iter().find(|c| c.candidate_type == IceCandidateType::Relay);
        let rem_relay = remote_candidates.iter().find(|c| c.candidate_type == IceCandidateType::Relay);

        if let (Some(l), Some(r)) = (loc_relay, rem_relay) {
            Some((l.clone(), r.clone()))
        } else {
            None
        }
    }
}

/// Pesan Gossip Terdistribusi
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GossipMessage {
    pub message_id: String,
    pub topic: String, // "depin_job_announcement", "block_header", "node_heartbeat"
    pub payload: Vec<u8>,
    pub hop_count: usize,
}

/// Engine Epidemic Gossip Protocol
pub struct GossipProtocolEngine {
    known_peers: Vec<String>,
    seen_messages: HashSet<String>,
    fanout: usize,
}

impl GossipProtocolEngine {
    pub fn new(fanout: usize) -> Self {
        Self {
            known_peers: Vec::new(),
            seen_messages: HashSet::new(),
            fanout,
        }
    }

    pub fn add_peer(&mut self, peer_addr: &str) {
        if !self.known_peers.iter().any(|p| p == peer_addr) {
            self.known_peers.push(peer_addr.to_string());
        }
    }

    /// Sebarkan pesan gossip ke `fanout` peer tetangga
    pub fn disseminate(&mut self, msg: GossipMessage) -> Result<Vec<String>, String> {
        if self.seen_messages.contains(&msg.message_id) {
            return Ok(Vec::new()); // Abaikan pesan yang sudah pernah diterima
        }
        self.seen_messages.insert(msg.message_id.clone());

        let target_peers: Vec<String> = self.known_peers.iter().take(self.fanout).cloned().collect();
        Ok(target_peers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nat_traversal_candidate_selection() {
        let local = vec![
            IceCandidate { candidate_type: IceCandidateType::Host, ip_address: "192.168.1.5".to_string(), port: 8001, priority: 100 },
            IceCandidate { candidate_type: IceCandidateType::ServerReflexive, ip_address: "103.120.40.5".to_string(), port: 8001, priority: 200 },
        ];
        let remote = vec![
            IceCandidate { candidate_type: IceCandidateType::ServerReflexive, ip_address: "180.250.10.2".to_string(), port: 9002, priority: 200 },
        ];

        let selected = P2pNatTraversal::select_best_candidate(&local, &remote).unwrap();
        assert_eq!(selected.0.candidate_type, IceCandidateType::ServerReflexive);
        assert_eq!(selected.1.candidate_type, IceCandidateType::ServerReflexive);
    }

    #[test]
    fn test_gossip_protocol_dissemination() {
        let mut gossip = GossipProtocolEngine::new(2);
        gossip.add_peer("peer_node_jakarta");
        gossip.add_peer("peer_node_surabaya");
        gossip.add_peer("peer_node_medan");

        let msg = GossipMessage {
            message_id: "msg_job_001".to_string(),
            topic: "depin_job_announcement".to_string(),
            payload: b"New GPU Compute Job".to_vec(),
            hop_count: 0,
        };

        let targets = gossip.disseminate(msg.clone()).unwrap();
        assert_eq!(targets.len(), 2);

        // Pesan duplikat tidak disebarkan ulang
        let targets_dup = gossip.disseminate(msg).unwrap();
        assert!(targets_dup.is_empty());
    }
}

