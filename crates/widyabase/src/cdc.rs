//! ============================================================================
//! WIDYABASE REAL-TIME CHANGE DATA CAPTURE (CDC) STREAMING (cdc.rs)
//! ============================================================================
//! Arsitektur Aliran Perubahan Data Real-Time Skala Industri:
//! - Menangkap operasi DML (INSERT, UPDATE, DELETE, TRUNCATE) & Batas Transaksi
//! - Menjaga LSN (Log Sequence Number) dari WAL dan ID Transaksi (tx_id)
//! - Rekaman Before-Image dan After-Image untuk audit log dan sinkronisasi
//! - Format Envelope Debezium / Kafka Connect Kompatibel
//! - Streaming Pub-Sub Multi-Subscriber Berkinerja Tinggi (tokio broadcast)
//! - Historical Replay dari LSN tertentu (Catch-Up Replication & Slot Streaming)
//! ============================================================================

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;
use std::time::{Duration, Instant};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use crate::sql_parser::SqlValue;

/// Tipe Operasi Perubahan CDC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CdcOp {
    Insert,
    Update,
    Delete,
    Truncate,
    Begin,
    Commit,
    Rollback,
}

impl CdcOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            CdcOp::Insert => "INSERT",
            CdcOp::Update => "UPDATE",
            CdcOp::Delete => "DELETE",
            CdcOp::Truncate => "TRUNCATE",
            CdcOp::Begin => "BEGIN",
            CdcOp::Commit => "COMMIT",
            CdcOp::Rollback => "ROLLBACK",
        }
    }

    /// Kode operasi standar Debezium
    pub fn debezium_code(&self) -> &'static str {
        match self {
            CdcOp::Insert => "c",
            CdcOp::Update => "u",
            CdcOp::Delete => "d",
            CdcOp::Truncate => "t",
            CdcOp::Begin => "b",
            CdcOp::Commit => "m",
            CdcOp::Rollback => "r",
        }
    }
}

/// Rekod Perubahan Data Real-Time CDC
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CdcRecord {
    pub lsn: u64,
    pub tx_id: u64,
    pub timestamp: u64, // Epoch millisecond
    pub table: String,
    pub op: CdcOp,
    pub before: Option<HashMap<String, SqlValue>>,
    pub after: Option<HashMap<String, SqlValue>>,
}

/// Mengonversi SqlValue ke serde_json::Value
pub fn sql_value_to_json(val: &SqlValue) -> serde_json::Value {
    match val {
        SqlValue::Null => serde_json::Value::Null,
        SqlValue::Boolean(b) => serde_json::Value::Bool(*b),
        SqlValue::Integer(i) => serde_json::json!(*i),
        SqlValue::Float(f) => serde_json::json!(*f),
        SqlValue::Text(s) => serde_json::Value::String(s.clone()),
    }
}

/// Mengonversi HashMap kolom ke serde_json::Value Object
pub fn row_fields_to_json(fields: &HashMap<String, SqlValue>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (k, v) in fields {
        map.insert(k.clone(), sql_value_to_json(v));
    }
    serde_json::Value::Object(map)
}

impl CdcRecord {
    pub fn new(
        lsn: u64,
        tx_id: u64,
        table: impl Into<String>,
        op: CdcOp,
        before: Option<HashMap<String, SqlValue>>,
        after: Option<HashMap<String, SqlValue>>,
    ) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u64;
        Self {
            lsn,
            tx_id,
            timestamp,
            table: table.into().to_uppercase(),
            op,
            before,
            after,
        }
    }

    /// Serialisasi rekod CDC ke format JSON baku
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Serialisasi ke amplop format standar Debezium / Kafka Connect
    pub fn to_debezium_format(&self) -> serde_json::Value {
        let before_json = self.before.as_ref().map(row_fields_to_json);
        let after_json = self.after.as_ref().map(row_fields_to_json);

        serde_json::json!({
            "before": before_json,
            "after": after_json,
            "source": {
                "version": "1.0",
                "connector": "widyabase-cdc",
                "name": "widyabase",
                "ts_ms": self.timestamp,
                "snapshot": "false",
                "db": "widyabase",
                "table": self.table.to_lowercase(),
                "lsn": self.lsn,
                "tx_id": self.tx_id
            },
            "op": self.op.debezium_code(),
            "ts_ms": self.timestamp
        })
    }
}

/// Filter untuk langganan CDC (Subscription Filtering)
#[derive(Debug, Clone, Default)]
pub struct CdcFilter {
    pub tables: Option<HashSet<String>>,
    pub operations: Option<HashSet<CdcOp>>,
    pub min_lsn: Option<u64>,
}

impl CdcFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_table(mut self, table: &str) -> Self {
        let mut set = self.tables.unwrap_or_default();
        set.insert(table.to_uppercase());
        self.tables = Some(set);
        self
    }

    pub fn with_operation(mut self, op: CdcOp) -> Self {
        let mut set = self.operations.unwrap_or_default();
        set.insert(op);
        self.operations = Some(set);
        self
    }

    pub fn with_min_lsn(mut self, min_lsn: u64) -> Self {
        self.min_lsn = Some(min_lsn);
        self
    }

    pub fn matches(&self, record: &CdcRecord) -> bool {
        if let Some(min_lsn) = self.min_lsn {
            if record.lsn < min_lsn {
                return false;
            }
        }
        if let Some(tables) = &self.tables {
            if !tables.contains(&record.table.to_uppercase()) {
                return false;
            }
        }
        if let Some(ops) = &self.operations {
            if !ops.contains(&record.op) {
                return false;
            }
        }
        true
    }
}

/// Langganan CDC Aktif untuk Client / Consumer
pub struct CdcSubscription {
    receiver: broadcast::Receiver<CdcRecord>,
    filter: Option<CdcFilter>,
    replay_buffer: VecDeque<CdcRecord>,
}

impl CdcSubscription {
    pub fn new(
        receiver: broadcast::Receiver<CdcRecord>,
        filter: Option<CdcFilter>,
        historical_records: Vec<CdcRecord>,
    ) -> Self {
        let mut replay_buffer = VecDeque::new();
        for rec in historical_records {
            if let Some(f) = &filter {
                if !f.matches(&rec) {
                    continue;
                }
            }
            replay_buffer.push_back(rec);
        }

        Self {
            receiver,
            filter,
            replay_buffer,
        }
    }

    /// Mengambil rekod berikutnya secara non-blocking
    pub fn try_recv(&mut self) -> Result<Option<CdcRecord>, String> {
        // Cek replay buffer dahulu
        while let Some(rec) = self.replay_buffer.pop_front() {
            if let Some(f) = &self.filter {
                if f.matches(&rec) {
                    return Ok(Some(rec));
                }
            } else {
                return Ok(Some(rec));
            }
        }

        // Cek live stream
        loop {
            match self.receiver.try_recv() {
                Ok(rec) => {
                    if let Some(f) = &self.filter {
                        if f.matches(&rec) {
                            return Ok(Some(rec));
                        }
                    } else {
                        return Ok(Some(rec));
                    }
                }
                Err(broadcast::error::TryRecvError::Empty) => return Ok(None),
                Err(broadcast::error::TryRecvError::Lagged(n)) => {
                    // Receiver tertinggal n pesan; lanjutkan membaca pesan berikutnya
                    continue;
                }
                Err(broadcast::error::TryRecvError::Closed) => return Err("Kanal CDC ditutup".to_string()),
            }
        }
    }

    /// Mengambil rekod berikutnya dengan batas waktu (timeout) sinkron
    pub fn recv_timeout(&mut self, timeout: Duration) -> Result<Option<CdcRecord>, String> {
        let start = Instant::now();
        loop {
            if let Some(rec) = self.try_recv()? {
                return Ok(Some(rec));
            }

            if start.elapsed() >= timeout {
                return Ok(None);
            }

            std::thread::sleep(Duration::from_millis(2));
        }
    }

    /// Mengambil rekod berikutnya secara asinkron
    pub async fn recv_async(&mut self) -> Result<CdcRecord, String> {
        while let Some(rec) = self.replay_buffer.pop_front() {
            if let Some(f) = &self.filter {
                if f.matches(&rec) {
                    return Ok(rec);
                }
            } else {
                return Ok(rec);
            }
        }

        loop {
            match self.receiver.recv().await {
                Ok(rec) => {
                    if let Some(f) = &self.filter {
                        if f.matches(&rec) {
                            return Ok(rec);
                        }
                    } else {
                        return Ok(rec);
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => return Err("Kanal CDC ditutup".to_string()),
            }
        }
    }
}

/// Pengelola Aliran CDC (Change Data Capture Streamer)
pub struct CdcStreamer {
    sender: broadcast::Sender<CdcRecord>,
    history: RwLock<Vec<CdcRecord>>,
    max_history: usize,
    total_emitted: AtomicUsize,
}

impl CdcStreamer {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity.max(1024));
        Self {
            sender,
            history: RwLock::new(Vec::new()),
            max_history: capacity,
            total_emitted: AtomicUsize::new(0),
        }
    }

    /// Memancarkan rekod perubahan ke seluruh subscriber aktif dan menyimpannya di riwayat
    pub fn emit(&self, record: CdcRecord) {
        self.total_emitted.fetch_add(1, Ordering::SeqCst);

        // Simpan ke ring buffer riwayat
        {
            let mut hist = self.history.write().unwrap();
            if hist.len() >= self.max_history {
                hist.remove(0);
            }
            hist.push(record.clone());
        }

        // Pancarkan ke subscriber (abaikan jika belum ada listener aktif)
        let _ = self.sender.send(record);
    }

    /// Berlangganan aliran CDC dengan opsi filter dan pemutaran riwayat dari LSN tertentu
    pub fn subscribe(&self, filter: Option<CdcFilter>, from_lsn: Option<u64>) -> CdcSubscription {
        let rx = self.sender.subscribe();

        let historical_records = if let Some(min_lsn) = from_lsn {
            let hist = self.history.read().unwrap();
            hist.iter()
                .filter(|r| r.lsn >= min_lsn)
                .cloned()
                .collect()
        } else {
            Vec::new()
        };

        CdcSubscription::new(rx, filter, historical_records)
    }

    /// Mengambil rekod historis mulai dari LSN tertentu
    pub fn get_history(&self, from_lsn: Option<u64>, limit: Option<usize>) -> Vec<CdcRecord> {
        let hist = self.history.read().unwrap();
        let iter = hist.iter().filter(|r| {
            if let Some(min_lsn) = from_lsn {
                r.lsn >= min_lsn
            } else {
                true
            }
        });

        if let Some(lim) = limit {
            iter.take(lim).cloned().collect()
        } else {
            iter.cloned().collect()
        }
    }

    /// Mendapatkan LSN tertinggi yang pernah dipancarkan
    pub fn latest_lsn(&self) -> u64 {
        let hist = self.history.read().unwrap();
        hist.last().map(|r| r.lsn).unwrap_or(0)
    }

    /// Mendapatkan jumlah total event yang telah dipancarkan sejak startup
    pub fn total_emitted(&self) -> usize {
        self.total_emitted.load(Ordering::SeqCst)
    }

    /// Mengosongkan riwayat rekod dalam memori
    pub fn clear_history(&self) {
        self.history.write().unwrap().clear();
    }
}

impl Default for CdcStreamer {
    fn default() -> Self {
        Self::new(10_000)
    }
}
