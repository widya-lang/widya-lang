use serde::{Deserialize, Serialize};

/// ============================================================================
/// CCSDS Space Packet Protocol (CCSDS 133.0-B-2) for NASA / ESA / JAXA Satellite
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CcsdsPacketType {
    Telemetry = 0,
    Telecommand = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SequenceFlag {
    ContinuationSegment = 0b00,
    FirstSegment = 0b01,
    LastSegment = 0b10,
    Unsegmented = 0b11,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcsdsPrimaryHeader {
    pub version: u8,               // 3 bits (always 000)
    pub packet_type: CcsdsPacketType, // 1 bit (0 = TM, 1 = TC)
    pub sec_hdr_flag: bool,        // 1 bit (Secondary Header Flag)
    pub apid: u16,                 // 11 bits (Application Process Identifier 0-2047)
    pub sequence_flags: SequenceFlag, // 2 bits
    pub packet_sequence_count: u16, // 14 bits (0-16383)
    pub packet_data_length: u16,   // 16 bits (Total Octets in Packet Data Field - 1)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CcsdsSpacePacket {
    pub header: CcsdsPrimaryHeader,
    pub user_data: Vec<u8>,
}

impl CcsdsSpacePacket {
    pub fn new_telemetry(apid: u16, seq_count: u16, data: Vec<u8>) -> Self {
        let length = if data.is_empty() { 0 } else { (data.len() - 1) as u16 };
        Self {
            header: CcsdsPrimaryHeader {
                version: 0,
                packet_type: CcsdsPacketType::Telemetry,
                sec_hdr_flag: false,
                apid: apid & 0x07FF,
                sequence_flags: SequenceFlag::Unsegmented,
                packet_sequence_count: seq_count & 0x3FFF,
                packet_data_length: length,
            },
            user_data: data,
        }
    }

    pub fn new_telecommand(apid: u16, seq_count: u16, cmd_payload: Vec<u8>) -> Self {
        let length = if cmd_payload.is_empty() { 0 } else { (cmd_payload.len() - 1) as u16 };
        Self {
            header: CcsdsPrimaryHeader {
                version: 0,
                packet_type: CcsdsPacketType::Telecommand,
                sec_hdr_flag: false,
                apid: apid & 0x07FF,
                sequence_flags: SequenceFlag::Unsegmented,
                packet_sequence_count: seq_count & 0x3FFF,
                packet_data_length: length,
            },
            user_data: cmd_payload,
        }
    }

    /// Serialisasi biner paket CCSDS (6 bytes header + user data)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(6 + self.user_data.len());

        // Octet 0-1: Version (3), Type (1), SecHdr (1), APID (11)
        let type_bit = if self.header.packet_type == CcsdsPacketType::Telecommand { 1 << 12 } else { 0 };
        let sec_bit = if self.header.sec_hdr_flag { 1 << 11 } else { 0 };
        let word0: u16 = ((self.header.version as u16 & 0x07) << 13)
            | type_bit
            | sec_bit
            | (self.header.apid & 0x07FF);
        bytes.extend_from_slice(&word0.to_be_bytes());

        // Octet 2-3: SeqFlags (2), SeqCount (14)
        let seq_bits = match self.header.sequence_flags {
            SequenceFlag::ContinuationSegment => 0b00 << 14,
            SequenceFlag::FirstSegment => 0b01 << 14,
            SequenceFlag::LastSegment => 0b10 << 14,
            SequenceFlag::Unsegmented => 0b11 << 14,
        };
        let word1: u16 = seq_bits | (self.header.packet_sequence_count & 0x3FFF);
        bytes.extend_from_slice(&word1.to_be_bytes());

        // Octet 4-5: Packet Data Length (16)
        bytes.extend_from_slice(&self.header.packet_data_length.to_be_bytes());

        // Packet Data Field
        bytes.extend_from_slice(&self.user_data);
        bytes
    }

    /// Parsing biner paket CCSDS
    pub fn parse_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() < 6 {
            return Err("Paket CCSDS terlalu pendek (< 6 bytes header)".to_string());
        }

        let word0 = u16::from_be_bytes([data[0], data[1]]);
        let version = ((word0 >> 13) & 0x07) as u8;
        let pkt_type = if (word0 & (1 << 12)) != 0 {
            CcsdsPacketType::Telecommand
        } else {
            CcsdsPacketType::Telemetry
        };
        let sec_hdr = (word0 & (1 << 11)) != 0;
        let apid = word0 & 0x07FF;

        let word1 = u16::from_be_bytes([data[2], data[3]]);
        let seq_flags = match (word1 >> 14) & 0x03 {
            0b00 => SequenceFlag::ContinuationSegment,
            0b01 => SequenceFlag::FirstSegment,
            0b10 => SequenceFlag::LastSegment,
            _ => SequenceFlag::Unsegmented,
        };
        let seq_count = word1 & 0x3FFF;
        let data_len = u16::from_be_bytes([data[4], data[5]]);

        let user_data = if data.len() > 6 {
            data[6..].to_vec()
        } else {
            Vec::new()
        };

        Ok(Self {
            header: CcsdsPrimaryHeader {
                version,
                packet_type: pkt_type,
                sec_hdr_flag: sec_hdr,
                apid,
                sequence_flags: seq_flags,
                packet_sequence_count: seq_count,
                packet_data_length: data_len,
            },
            user_data,
        })
    }
}
