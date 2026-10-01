use serde::{Deserialize, Serialize};

/// Tipe Paket Pesan Protokol MQTT v5.0
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MqttPacketType {
    Connect,
    ConnAck,
    Publish,
    PubAck,
    Subscribe,
    SubAck,
    PingReq,
    PingResp,
    Disconnect,
}

/// Paket Pesan MQTT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttMessage {
    pub packet_type: MqttPacketType,
    pub topic: String,
    pub payload: Vec<u8>,
    pub qos: u8, // 0, 1, 2
    pub retain: bool,
}

impl MqttMessage {
    pub fn new_publish(topic: &str, payload: &[u8], qos: u8) -> Self {
        Self {
            packet_type: MqttPacketType::Publish,
            topic: topic.to_string(),
            payload: payload.to_vec(),
            qos,
            retain: false,
        }
    }
}

/// Pencocokan Topik MQTT dengan Dukungan Wildcard (+ dan #)
pub fn cocokkan_topik_mqtt(sub_filter: &str, publish_topic: &str) -> bool {
    if sub_filter == "#" {
        return true;
    }
    if sub_filter == publish_topic {
        return true;
    }

    let filter_parts: Vec<&str> = sub_filter.split('/').collect();
    let topic_parts: Vec<&str> = publish_topic.split('/').collect();

    for (i, filter) in filter_parts.iter().enumerate() {
        if *filter == "#" {
            return true;
        }
        if i >= topic_parts.len() {
            return false;
        }
        if *filter != "+" && *filter != topic_parts[i] {
            return false;
        }
    }

    filter_parts.len() == topic_parts.len()
}
