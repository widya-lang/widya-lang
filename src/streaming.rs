//! WidyaStream — Sovereign Distributed Event Streaming & Partitioned Commit Log Engine (Kafka Equivalent)
//! Provides high-throughput partitioned topics, append-only commit logs, consumer group offset tracking,
//! and real-time event streaming without external broker dependencies.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordHeader {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct StreamRecord {
    pub offset: u64,
    pub timestamp: u64,
    pub partition: usize,
    pub key: Option<String>,
    pub payload: Vec<u8>,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TopicConfig {
    pub name: String,
    pub partitions_count: usize,
    pub retention_ms: u64,
    pub max_message_size: usize,
}

impl Default for TopicConfig {
    fn default() -> Self {
        Self {
            name: "default_topic".to_string(),
            partitions_count: 3,
            retention_ms: 7 * 24 * 3600 * 1000, // 7 days
            max_message_size: 10 * 1024 * 1024,  // 10MB
        }
    }
}

pub struct PartitionLog {
    pub partition_id: usize,
    records: Vec<StreamRecord>,
    high_watermark: u64,
}

impl PartitionLog {
    pub fn new(partition_id: usize) -> Self {
        Self {
            partition_id,
            records: Vec::new(),
            high_watermark: 0,
        }
    }

    pub fn append(&mut self, key: Option<String>, payload: Vec<u8>, headers: HashMap<String, String>) -> u64 {
        let offset = self.high_watermark;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let record = StreamRecord {
            offset,
            timestamp: now,
            partition: self.partition_id,
            key,
            payload,
            headers,
        };

        self.records.push(record);
        self.high_watermark += 1;
        offset
    }

    pub fn read_from(&self, start_offset: u64, max_records: usize) -> Vec<StreamRecord> {
        self.records
            .iter()
            .filter(|r| r.offset >= start_offset)
            .take(max_records)
            .cloned()
            .collect()
    }

    pub fn latest_offset(&self) -> u64 {
        self.high_watermark
    }
}

pub struct Topic {
    pub config: TopicConfig,
    partitions: Vec<Arc<RwLock<PartitionLog>>>,
}

impl Topic {
    pub fn new(config: TopicConfig) -> Self {
        let mut partitions = Vec::with_capacity(config.partitions_count);
        for i in 0..config.partitions_count {
            partitions.push(Arc::new(RwLock::new(PartitionLog::new(i))));
        }
        Self { config, partitions }
    }

    pub fn produce(&self, key: Option<String>, payload: Vec<u8>, headers: HashMap<String, String>) -> (usize, u64) {
        let partition_id = match &key {
            Some(k) => {
                let mut hash: usize = 0;
                for b in k.bytes() {
                    hash = hash.wrapping_mul(31).wrapping_add(b as usize);
                }
                hash % self.partitions.len()
            }
            None => {
                // Round-robin or random partition
                (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as usize) % self.partitions.len()
            }
        };

        let mut part = self.partitions[partition_id].write().unwrap();
        let offset = part.append(key, payload, headers);
        (partition_id, offset)
    }

    pub fn consume(&self, partition_id: usize, start_offset: u64, limit: usize) -> Vec<StreamRecord> {
        if partition_id >= self.partitions.len() {
            return Vec::new();
        }
        let part = self.partitions[partition_id].read().unwrap();
        part.read_from(start_offset, limit)
    }
}

pub struct ConsumerGroup {
    pub group_id: String,
    committed_offsets: Arc<RwLock<HashMap<(String, usize), u64>>>, // (topic, partition) -> offset
}

impl ConsumerGroup {
    pub fn new(group_id: &str) -> Self {
        Self {
            group_id: group_id.to_string(),
            committed_offsets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn commit(&self, topic: &str, partition: usize, offset: u64) {
        let mut offsets = self.committed_offsets.write().unwrap();
        offsets.insert((topic.to_string(), partition), offset);
    }

    pub fn get_offset(&self, topic: &str, partition: usize) -> u64 {
        let offsets = self.committed_offsets.read().unwrap();
        *offsets.get(&(topic.to_string(), partition)).unwrap_or(&0)
    }
}

pub struct WidyaStreamBroker {
    topics: Arc<RwLock<HashMap<String, Arc<Topic>>>>,
    groups: Arc<RwLock<HashMap<String, Arc<ConsumerGroup>>>>,
}

impl WidyaStreamBroker {
    pub fn new() -> Self {
        Self {
            topics: Arc::new(RwLock::new(HashMap::new())),
            groups: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_topic(&self, config: TopicConfig) -> bool {
        let mut topics = self.topics.write().unwrap();
        if topics.contains_key(&config.name) {
            return false;
        }
        let name = config.name.clone();
        topics.insert(name, Arc::new(Topic::new(config)));
        true
    }

    pub fn list_topics(&self) -> Vec<String> {
        let topics = self.topics.read().unwrap();
        topics.keys().cloned().collect()
    }

    pub fn produce(&self, topic_name: &str, key: Option<String>, payload: Vec<u8>, headers: HashMap<String, String>) -> Result<(usize, u64), String> {
        let topics = self.topics.read().unwrap();
        if let Some(topic) = topics.get(topic_name) {
            Ok(topic.produce(key, payload, headers))
        } else {
            Err(format!("Topik '{}' tidak ditemukan", topic_name))
        }
    }

    pub fn consume(&self, topic_name: &str, partition: usize, offset: u64, limit: usize) -> Result<Vec<StreamRecord>, String> {
        let topics = self.topics.read().unwrap();
        if let Some(topic) = topics.get(topic_name) {
            Ok(topic.consume(partition, offset, limit))
        } else {
            Err(format!("Topik '{}' tidak ditemukan", topic_name))
        }
    }

    pub fn get_or_create_group(&self, group_id: &str) -> Arc<ConsumerGroup> {
        let mut groups = self.groups.write().unwrap();
        groups
            .entry(group_id.to_string())
            .or_insert_with(|| Arc::new(ConsumerGroup::new(group_id)))
            .clone()
    }

    pub fn consume_group(&self, group_id: &str, topic_name: &str, max_per_partition: usize) -> Result<Vec<StreamRecord>, String> {
        let group = self.get_or_create_group(group_id);
        let topics = self.topics.read().unwrap();
        let topic = topics.get(topic_name).ok_or_else(|| format!("Topik '{}' tidak ditemukan", topic_name))?;

        let mut results = Vec::new();
        for partition_id in 0..topic.partitions.len() {
            let current_offset = group.get_offset(topic_name, partition_id);
            let records = topic.consume(partition_id, current_offset, max_per_partition);
            if let Some(last) = records.last() {
                group.commit(topic_name, partition_id, last.offset + 1);
            }
            results.extend(records);
        }
        Ok(results)
    }
}
