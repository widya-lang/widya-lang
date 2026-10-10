//! Pure Sovereign Zero-Dependency 5G Core UPF & GTP-U / PFCP Engine (pure_5g.rs)
//!
//! 100% In-House Implementation:
//! - 3GPP TS 29.281 GTP-U (GPRS Tunnelling Protocol User Plane) Wire Engine
//! - 3GPP TS 38.415 5G PDU Session Container Extension with QFI (QoS Flow Identifier)
//! - 3GPP TS 29.244 PFCP (Packet Forwarding Control Protocol) Wire Engine
//! - Sovereign High-Performance Line-Rate UPF Tunnel Router (Uplink / Downlink)

use std::collections::HashMap;

// ============================================================================
// 1. GTP-U (3GPP TS 29.281) & 5G PDU SESSION CONTAINER (3GPP TS 38.415)
// ============================================================================

pub const GTPU_PORT: u16 = 2152;
pub const GTPU_MSG_ECHO_REQ: u8  = 1;
pub const GTPU_MSG_ECHO_RESP: u8 = 2;
pub const GTPU_MSG_ERROR_IND: u8 = 26;
pub const GTPU_MSG_END_MARKER: u8 = 254;
pub const GTPU_MSG_TPDU: u8      = 255; // User payload

pub const GTPU_EXT_PDU_SESSION_CONTAINER: u8 = 0x85; // 5G PDU Session Container extension

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PureGtpuHeader {
    pub version: u8,         // 3 bits (always 1)
    pub protocol_type: u8,   // 1 bit (1 for GTP)
    pub has_extension: bool, // 1 bit
    pub has_sequence: bool,  // 1 bit
    pub has_npdu: bool,      // 1 bit
    pub message_type: u8,    // 1 byte (255 for T-PDU)
    pub length: u16,         // 2 bytes
    pub teid: u32,           // 4 bytes Tunnel Endpoint Identifier
    pub sequence_number: Option<u16>,
    pub npdu_number: Option<u8>,
    pub qfi: Option<u8>,     // 6 bits (0..63) 5G QoS Flow Identifier
}

pub struct PureGtpuCodec;

impl PureGtpuCodec {
    /// Enkapsulasi paket IP ke dalam format biner GTP-U wire frame dengan opsional 5G QFI
    pub fn encapsulate(teid: u32, qfi: Option<u8>, inner_payload: &[u8]) -> Vec<u8> {
        let has_ext = qfi.is_some();
        let ext_header_len = if has_ext { 4 + 4 } else { 0 }; // 4 bytes optional fields + 4 bytes extension header
        let total_length = (inner_payload.len() + ext_header_len) as u16;

        let mut out = Vec::with_capacity(8 + ext_header_len + inner_payload.len());

        // Flags byte: Version (3 bits = 1) | PT (1 bit = 1) | Spare (1 bit = 0) | E (1 bit) | S (0) | PN (0)
        let flags: u8 = (1 << 5) | (1 << 4) | if has_ext { 0x04 } else { 0x00 };
        out.push(flags);
        out.push(GTPU_MSG_TPDU);
        out.extend_from_slice(&total_length.to_be_bytes());
        out.extend_from_slice(&teid.to_be_bytes());

        if has_ext {
            // Sequence Number (2B = 0), N-PDU (1B = 0), Next Ext Type (1B = 0x85)
            out.extend_from_slice(&0u16.to_be_bytes());
            out.push(0);
            out.push(GTPU_EXT_PDU_SESSION_CONTAINER);

            // Extension Header 3GPP TS 38.415:
            // Length in 4-octet units: 1 unit (4 bytes)
            out.push(1);
            // PDU Type = 0 (DL) or 1 (UL), Spare = 0
            // Byte 1: PDU Type (4 bits: 0 = DL) | Spare
            out.push(0x00);
            // Byte 2: Spare (1 bit) | RQI (1 bit = 0) | QFI (6 bits)
            let qfi_val = qfi.unwrap_or(1) & 0x3F;
            out.push(qfi_val);
            // Next Extension Header: 0 (No more extensions)
            out.push(0x00);
        }

        out.extend_from_slice(inner_payload);
        out
    }

    /// Dekapsulasi paket biner GTP-U frame
    pub fn decapsulate(bytes: &[u8]) -> Result<(PureGtpuHeader, Vec<u8>), String> {
        if bytes.len() < 8 {
            return Err("Ukuran paket GTP-U terlalu pendek (< 8 bytes)".to_string());
        }

        let flags = bytes[0];
        let version = (flags >> 5) & 0x07;
        let protocol_type = (flags >> 4) & 0x01;
        let has_ext = (flags & 0x04) != 0;
        let has_seq = (flags & 0x02) != 0;
        let has_npdu = (flags & 0x01) != 0;

        if version != 1 {
            return Err(format!("GTP Version {} tidak didukung (harus 1)", version));
        }

        let msg_type = bytes[1];
        let length = u16::from_be_bytes([bytes[2], bytes[3]]);
        let teid = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

        let mut offset = 8;
        let mut seq_num = None;
        let mut npdu_num = None;
        let mut qfi = None;

        if has_ext || has_seq || has_npdu {
            if bytes.len() < offset + 4 {
                return Err("GTP-U optional header fields terpotong".to_string());
            }
            if has_seq {
                seq_num = Some(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
            }
            if has_npdu {
                npdu_num = Some(bytes[offset + 2]);
            }
            let mut next_ext = bytes[offset + 3];
            offset += 4;

            // Baca chain extension headers
            while next_ext != 0 && offset < bytes.len() {
                let ext_len_units = bytes[offset] as usize;
                let ext_len_bytes = ext_len_units * 4;
                if offset + ext_len_bytes > bytes.len() {
                    return Err("Extension header length melebihi batas paket".to_string());
                }

                if next_ext == GTPU_EXT_PDU_SESSION_CONTAINER && ext_len_bytes >= 4 {
                    // QFI ada di byte index offset + 2 (bits 0..5)
                    let qfi_byte = bytes[offset + 2];
                    qfi = Some(qfi_byte & 0x3F);
                }

                next_ext = bytes[offset + ext_len_bytes - 1];
                offset += ext_len_bytes;
            }
        }

        if offset > bytes.len() {
            return Err("Payload GTP-U tidak valid".to_string());
        }

        let payload = bytes[offset..].to_vec();

        let header = PureGtpuHeader {
            version,
            protocol_type,
            has_extension: has_ext,
            has_sequence: has_seq,
            has_npdu,
            message_type: msg_type,
            length,
            teid,
            sequence_number: seq_num,
            npdu_number: npdu_num,
            qfi,
        };

        Ok((header, payload))
    }
}

// ============================================================================
// 2. PFCP (3GPP TS 29.244) PROTOCOL ENGINE
// ============================================================================

pub const PFCP_PORT: u16 = 8805;
pub const PFCP_MSG_HEARTBEAT_REQ: u8       = 1;
pub const PFCP_MSG_HEARTBEAT_RESP: u8      = 2;
pub const PFCP_MSG_ASSOC_SETUP_REQ: u8     = 5;
pub const PFCP_MSG_ASSOC_SETUP_RESP: u8    = 6;
pub const PFCP_MSG_SESSION_ESTABLISH_REQ: u8 = 50;
pub const PFCP_MSG_SESSION_ESTABLISH_RESP: u8 = 51;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurePfcpHeader {
    pub version: u8,
    pub has_seid: bool,
    pub message_type: u8,
    pub length: u16,
    pub seid: Option<u64>,
    pub sequence_number: u32,
}

pub struct PurePfcpCodec;

impl PurePfcpCodec {
    /// Buat pesan PFCP Heartbeat Request
    pub fn build_heartbeat_req(seq: u32) -> Vec<u8> {
        let mut out = Vec::with_capacity(16);
        // Flags: Version (1) | MP (0) | S (0) -> 0x20
        out.push(0x20);
        out.push(PFCP_MSG_HEARTBEAT_REQ);
        out.extend_from_slice(&4u16.to_be_bytes()); // Length of remaining header (4 bytes seq + spare)
        // Sequence Number 24-bit + 1 byte spare
        let seq_bytes = seq.to_be_bytes();
        out.push(seq_bytes[1]);
        out.push(seq_bytes[2]);
        out.push(seq_bytes[3]);
        out.push(0); // Spare
        out
    }

    /// Buat pesan PFCP Session Establishment Response
    pub fn build_session_establish_resp(seid: u64, seq: u32, cause_accepted: bool) -> Vec<u8> {
        // IE Cause: Type (19), Length (1), Value (1 = Request Accepted, 73 = Request Rejected)
        let cause_val: u8 = if cause_accepted { 1 } else { 73 };
        let ie_len = 4 + 1; // 2B type + 2B len + 1B val
        let remaining_len = (12 + ie_len) as u16; // 8B SEID + 4B Seq/Spare + IE

        let mut out = Vec::with_capacity(16 + ie_len);
        // Flags: Version (1) | MP (0) | S (1) -> 0x21
        out.push(0x21);
        out.push(PFCP_MSG_SESSION_ESTABLISH_RESP);
        out.extend_from_slice(&remaining_len.to_be_bytes());
        out.extend_from_slice(&seid.to_be_bytes());

        let seq_bytes = seq.to_be_bytes();
        out.push(seq_bytes[1]);
        out.push(seq_bytes[2]);
        out.push(seq_bytes[3]);
        out.push(0); // Spare

        // IE Cause (Type 19)
        out.extend_from_slice(&19u16.to_be_bytes());
        out.extend_from_slice(&1u16.to_be_bytes());
        out.push(cause_val);

        out
    }

    /// Parse header PFCP
    pub fn parse_header(bytes: &[u8]) -> Result<PurePfcpHeader, String> {
        if bytes.len() < 8 {
            return Err("Ukuran header PFCP terlalu pendek (< 8 bytes)".to_string());
        }

        let flags = bytes[0];
        let version = (flags >> 5) & 0x07;
        let has_seid = (flags & 0x01) != 0;
        let msg_type = bytes[1];
        let length = u16::from_be_bytes([bytes[2], bytes[3]]);

        if has_seid {
            if bytes.len() < 16 {
                return Err("Header PFCP dengan SEID terlalu pendek (< 16 bytes)".to_string());
            }
            let seid = u64::from_be_bytes([
                bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11],
            ]);
            let seq = ((bytes[12] as u32) << 16) | ((bytes[13] as u32) << 8) | (bytes[14] as u32);
            Ok(PurePfcpHeader {
                version,
                has_seid: true,
                message_type: msg_type,
                length,
                seid: Some(seid),
                sequence_number: seq,
            })
        } else {
            let seq = ((bytes[4] as u32) << 16) | ((bytes[5] as u32) << 8) | (bytes[6] as u32);
            Ok(PurePfcpHeader {
                version,
                has_seid: false,
                message_type: msg_type,
                length,
                seid: None,
                sequence_number: seq,
            })
        }
    }
}

// ============================================================================
// 3. SOVEREIGN 5G LINE-RATE UPF TUNNEL ROUTER
// ============================================================================

#[derive(Debug, Clone)]
pub struct Pure5gSession {
    pub seid: u64,
    pub teid_uplink: u32,
    pub teid_downlink: u32,
    pub ue_ip: [u8; 4],
    pub qfi: u8,
}

pub struct Pure5gUpfRouter {
    pub uplink_map: HashMap<u32, Pure5gSession>,   // Key: teid_uplink
    pub downlink_map: HashMap<[u8; 4], Pure5gSession>, // Key: ue_ip
}

impl Pure5gUpfRouter {
    pub fn new() -> Self {
        Self {
            uplink_map: HashMap::new(),
            downlink_map: HashMap::new(),
        }
    }

    /// Daftarkan PDU Session aktif
    pub fn register_session(&mut self, session: Pure5gSession) {
        self.downlink_map.insert(session.ue_ip, session.clone());
        self.uplink_map.insert(session.teid_uplink, session);
    }

    /// Rute Uplink: Dari gNodeB via GTP-U -> Dekapsulasi -> Paket IP asli menuju Internet (N6)
    pub fn route_uplink(&self, raw_gtpu_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let (header, ip_payload) = PureGtpuCodec::decapsulate(raw_gtpu_bytes)?;

        // Validasi keberadaan sesi berdasarkan Uplink TEID
        let _session = self.uplink_map.get(&header.teid)
            .ok_or_else(|| format!("Uplink TEID 0x{:08X} tidak terdaftar pada UPF", header.teid))?;

        // Paket IP berhasil didekapsulasi menuju jaringan internet eksternal (N6)
        Ok(ip_payload)
    }

    /// Rute Downlink: Dari Internet (N6) -> Enkapsulasi ke dalam GTP-U menuju gNodeB (N3)
    pub fn route_downlink(&self, target_ue_ip: [u8; 4], raw_ip_payload: &[u8]) -> Result<Vec<u8>, String> {
        let session = self.downlink_map.get(&target_ue_ip)
            .ok_or_else(|| format!("UE IP {:?} tidak ditemukan dalam tabel rute downlink UPF", target_ue_ip))?;

        let gtpu_packet = PureGtpuCodec::encapsulate(session.teid_downlink, Some(session.qfi), raw_ip_payload);
        Ok(gtpu_packet)
    }
}
