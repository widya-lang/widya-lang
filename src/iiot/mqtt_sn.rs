use serde::{Deserialize, Serialize};

/// ============================================================================
/// MQTT-SN (MQTT for Sensor Networks) Lightweight Packet Codec
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MqttSnMsgType {
    Advertise = 0x00,
    SearchGw = 0x01,
    GwInfo = 0x02,
    Connect = 0x04,
    ConnAck = 0x05,
    Register = 0x0A,
    RegAck = 0x0B,
    Publish = 0x0C,
    PubAck = 0x0D,
    Subscribe = 0x12,
    SubAck = 0x13,
    PingReq = 0x16,
    PingResp = 0x17,
    Disconnect = 0x18,
}

impl MqttSnMsgType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0x00 => Some(Self::Advertise),
            0x01 => Some(Self::SearchGw),
            0x02 => Some(Self::GwInfo),
            0x04 => Some(Self::Connect),
            0x05 => Some(Self::ConnAck),
            0x0A => Some(Self::Register),
            0x0B => Some(Self::RegAck),
            0x0C => Some(Self::Publish),
            0x0D => Some(Self::PubAck),
            0x12 => Some(Self::Subscribe),
            0x13 => Some(Self::SubAck),
            0x16 => Some(Self::PingReq),
            0x17 => Some(Self::PingResp),
            0x18 => Some(Self::Disconnect),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttSnPublish {
    pub topic_id: u16,
    pub msg_id: u16,
    pub qos: i8,
    pub payload: Vec<u8>,
}

impl MqttSnPublish {
    pub fn encode(&self) -> Vec<u8> {
        let len = 7 + self.payload.len();
        let mut buf = Vec::with_capacity(len);
        buf.push(len as u8);
        buf.push(MqttSnMsgType::Publish as u8);
        let flags: u8 = if self.qos == 1 { 0x20 } else if self.qos == 2 { 0x40 } else { 0x00 };
        buf.push(flags);
        buf.extend_from_slice(&self.topic_id.to_be_bytes());
        buf.extend_from_slice(&self.msg_id.to_be_bytes());
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn decode(data: &[u8]) -> Result<Self, String> {
        if data.len() < 7 {
            return Err("Data MQTT-SN Publish terlalu pendek (< 7 bytes)".to_string());
        }
        if data[1] != (MqttSnMsgType::Publish as u8) {
            return Err("Bukan pesan MQTT-SN Publish".to_string());
        }

        let flags = data[2];
        let qos = if (flags & 0x40) != 0 { 2 } else if (flags & 0x20) != 0 { 1 } else { 0 };
        let topic_id = u16::from_be_bytes([data[3], data[4]]);
        let msg_id = u16::from_be_bytes([data[5], data[6]]);
        let payload = data[7..].to_vec();

        Ok(Self {
            topic_id,
            msg_id,
            qos,
            payload,
        })
    }
}
