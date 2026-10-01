use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ============================================================================
/// FIX Protocol v4.4 / v5.0 SP2 Gateway (Financial Information eXchange)
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixSide {
    Buy = 1,
    Sell = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixOrderType {
    Market = 1,
    Limit = 2,
    Stop = 3,
    StopLimit = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixMessage {
    pub msg_type: String, // Tag 35 (e.g. "D" for NewOrderSingle, "8" for ExecutionReport)
    pub sender_comp_id: String, // Tag 49
    pub target_comp_id: String, // Tag 56
    pub msg_seq_num: u64, // Tag 34
    pub sending_time: String, // Tag 52
    pub tags: HashMap<u32, String>,
}

impl FixMessage {
    pub const SOH: char = '\x01';

    pub fn new(msg_type: &str, sender: &str, target: &str, seq_num: u64) -> Self {
        Self {
            msg_type: msg_type.to_string(),
            sender_comp_id: sender.to_string(),
            target_comp_id: target.to_string(),
            msg_seq_num: seq_num,
            sending_time: "2026-10-01T10:00:00.000Z".to_string(),
            tags: HashMap::new(),
        }
    }

    pub fn set_tag(&mut self, tag: u32, value: &str) {
        self.tags.insert(tag, value.to_string());
    }

    pub fn get_tag(&self, tag: u32) -> Option<&String> {
        self.tags.get(&tag)
    }

    /// Serialisasi FIX string dengan pemisah SOH (\x01) dan perhitungan Checksum (Tag 10)
    pub fn serialize_to_fix_str(&self) -> String {
        let mut body = String::new();
        body.push_str(&format!("35={}{}", self.msg_type, Self::SOH));
        body.push_str(&format!("49={}{}", self.sender_comp_id, Self::SOH));
        body.push_str(&format!("56={}{}", self.target_comp_id, Self::SOH));
        body.push_str(&format!("34={}{}", self.msg_seq_num, Self::SOH));
        body.push_str(&format!("52={}{}", self.sending_time, Self::SOH));

        for (tag, val) in &self.tags {
            body.push_str(&format!("{}={}{}", tag, val, Self::SOH));
        }

        let body_len = body.len();
        let mut header = format!("8=FIX.4.4{}9={}{}", Self::SOH, body_len, Self::SOH);
        header.push_str(&body);

        // Calculate Checksum (sum of ASCII bytes % 256)
        let checksum: u32 = header.bytes().map(|b| b as u32).sum::<u32>() % 256;
        format!("{}10={:03}{}", header, checksum, Self::SOH)
    }

    /// Parsing pesan FIX Protocol mentah
    pub fn parse_fix_str(raw: &str) -> Result<Self, String> {
        let delimiter = if raw.contains('\x01') { '\x01' } else { '|' };
        let parts: Vec<&str> = raw.split(delimiter).filter(|s| !s.is_empty()).collect();

        let mut msg_type = String::new();
        let mut sender = String::new();
        let mut target = String::new();
        let mut seq_num = 0;
        let mut sending_time = String::new();
        let mut tags = HashMap::new();

        for part in parts {
            let kv: Vec<&str> = part.splitn(2, '=').collect();
            if kv.len() == 2 {
                let tag_num: u32 = kv[0].parse().unwrap_or(0);
                let val = kv[1].to_string();

                match tag_num {
                    35 => msg_type = val,
                    49 => sender = val,
                    56 => target = val,
                    34 => seq_num = val.parse().unwrap_or(0),
                    52 => sending_time = val,
                    _ => {
                        tags.insert(tag_num, val);
                    }
                }
            }
        }

        if msg_type.is_empty() {
            return Err("Tag 35 (MsgType) tidak ditemukan pada paket FIX".to_string());
        }

        Ok(Self {
            msg_type,
            sender_comp_id: sender,
            target_comp_id: target,
            msg_seq_num: seq_num,
            sending_time,
            tags,
        })
    }
}
