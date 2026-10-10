use serde::{Deserialize, Serialize};

/// ============================================================================
/// GTP-U (GPRS Tunneling Protocol User Plane - 3GPP TS 29.281)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GtpMessageType {
    EchoRequest,
    EchoResponse,
    ErrorIndication,
    EndMarker,
    TPdu, // User Data Packet
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GtpHeader {
    pub version: u8,             // Typically 1
    pub protocol_type: u8,       // 1 for GTP
    pub message_type: GtpMessageType,
    pub length: u16,
    pub teid: u32,               // Tunnel Endpoint Identifier
    pub sequence_number: Option<u16>,
    pub qfi_extension: Option<u8>, // 5G QoS Flow Identifier Extension
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GtpPacket {
    pub header: GtpHeader,
    pub payload: Vec<u8>,
}

impl GtpPacket {
    /// Membuat paket GTP-U T-PDU baru
    pub fn new_t_pdu(teid: u32, qfi: u8, payload: Vec<u8>) -> Self {
        let length = payload.len() as u16 + if qfi > 0 { 4 } else { 0 };
        Self {
            header: GtpHeader {
                version: 1,
                protocol_type: 1,
                message_type: GtpMessageType::TPdu,
                length,
                teid,
                sequence_number: None,
                qfi_extension: if qfi > 0 { Some(qfi) } else { None },
            },
            payload,
        }
    }

    /// Serialisasi biner GTP-U frame
    pub fn serialize_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        // Byte 0: Version (3 bits) | PT (1 bit) | Reserved (1 bit) | E (1 bit) | S (1 bit) | PN (1 bit)
        let flags: u8 = (1 << 5) | (1 << 4) | if self.header.qfi_extension.is_some() { 0x04 } else { 0x00 };
        buf.push(flags);

        // Byte 1: Message Type
        let msg_type_byte = match self.header.message_type {
            GtpMessageType::EchoRequest => 1,
            GtpMessageType::EchoResponse => 2,
            GtpMessageType::ErrorIndication => 26,
            GtpMessageType::EndMarker => 254,
            GtpMessageType::TPdu => 255,
        };
        buf.push(msg_type_byte);

        // Byte 2-3: Length
        buf.extend_from_slice(&self.header.length.to_be_bytes());

        // Byte 4-7: TEID
        buf.extend_from_slice(&self.header.teid.to_be_bytes());

        // Extension header jika ada QFI
        if let Some(qfi) = self.header.qfi_extension {
            buf.push(0x00); // Next extension type
            buf.push(0x01); // Ext Length
            buf.push(qfi & 0x3F); // PDU Session Container QFI
            buf.push(0x00); // Pad
        }

        buf.extend_from_slice(&self.payload);
        buf
    }

    /// Deserialisasi biner GTP-U frame
    pub fn parse_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() < 8 {
            return Err("Paket GTP-U terlalu pendek (< 8 bytes)".to_string());
        }

        let flags = data[0];
        let version = (flags >> 5) & 0x07;
        if version != 1 {
            return Err(format!("GTP-U versi {} tidak didukung, harus versi 1", version));
        }

        let msg_type = match data[1] {
            1 => GtpMessageType::EchoRequest,
            2 => GtpMessageType::EchoResponse,
            26 => GtpMessageType::ErrorIndication,
            254 => GtpMessageType::EndMarker,
            255 => GtpMessageType::TPdu,
            other => return Err(format!("GTP Message Type {} tidak dikenali", other)),
        };

        let length = u16::from_be_bytes([data[2], data[3]]);
        let teid = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);

        let has_ext = (flags & 0x04) != 0;
        let mut offset = 8;
        let mut qfi_ext = None;

        if has_ext && data.len() >= 12 {
            // Next Extension Header
            qfi_ext = Some(data[10] & 0x3F);
            offset = 12;
        }

        let payload = if data.len() > offset {
            data[offset..].to_vec()
        } else {
            Vec::new()
        };

        Ok(Self {
            header: GtpHeader {
                version: 1,
                protocol_type: 1,
                message_type: msg_type,
                length,
                teid,
                sequence_number: None,
                qfi_extension: qfi_ext,
            },
            payload,
        })
    }
}

impl From<GtpPacket> for (crate::telecom::pure_5g::PureGtpuHeader, Vec<u8>) {
    fn from(p: GtpPacket) -> Self {
        let msg_type = match p.header.message_type {
            GtpMessageType::EchoRequest => 1,
            GtpMessageType::EchoResponse => 2,
            GtpMessageType::ErrorIndication => 26,
            GtpMessageType::EndMarker => 254,
            GtpMessageType::TPdu => 255,
        };
        let header = crate::telecom::pure_5g::PureGtpuHeader {
            version: p.header.version,
            protocol_type: p.header.protocol_type,
            has_extension: p.header.qfi_extension.is_some(),
            has_sequence: p.header.sequence_number.is_some(),
            has_npdu: false,
            message_type: msg_type,
            length: p.header.length,
            teid: p.header.teid,
            sequence_number: p.header.sequence_number,
            npdu_number: None,
            qfi: p.header.qfi_extension,
        };
        (header, p.payload)
    }
}

impl From<(crate::telecom::pure_5g::PureGtpuHeader, Vec<u8>)> for GtpPacket {
    fn from((h, payload): (crate::telecom::pure_5g::PureGtpuHeader, Vec<u8>)) -> Self {
        let msg_type = match h.message_type {
            1 => GtpMessageType::EchoRequest,
            2 => GtpMessageType::EchoResponse,
            26 => GtpMessageType::ErrorIndication,
            254 => GtpMessageType::EndMarker,
            _ => GtpMessageType::TPdu,
        };
        Self {
            header: GtpHeader {
                version: h.version,
                protocol_type: h.protocol_type,
                message_type: msg_type,
                length: h.length,
                teid: h.teid,
                sequence_number: h.sequence_number,
                qfi_extension: h.qfi,
            },
            payload,
        }
    }
}

