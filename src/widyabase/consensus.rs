//! Konsensus Terdistribusi Raft untuk WidyaBase Cluster (consensus.rs)
//! Mengelola pemilihan Leader (Leader Election), Replikasi Log (Log Replication), dan Heartbeat Sinkronisasi.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaftRole {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub term: u64,
    pub index: u64,
    pub command: Vec<u8>,
}

pub struct RaftNode {
    pub node_id: u64,
    pub current_term: AtomicU64,
    pub voted_for: Mutex<Option<u64>>,
    pub role: Mutex<RaftRole>,
    pub log: Mutex<Vec<LogEntry>>,
    pub commit_index: AtomicU64,
}

impl RaftNode {
    pub fn new(node_id: u64) -> Self {
        Self {
            node_id,
            current_term: AtomicU64::new(0),
            voted_for: Mutex::new(None),
            role: Mutex::new(RaftRole::Follower),
            log: Mutex::new(Vec::new()),
            commit_index: AtomicU64::new(0),
        }
    }

    /// Mulai Pemilihan Pemimpin (Election Timeout Triggered)
    pub fn start_election(&self) -> (u64, u64) {
        let new_term = self.current_term.fetch_add(1, Ordering::SeqCst) + 1;
        *self.role.lock().unwrap() = RaftRole::Candidate;
        *self.voted_for.lock().unwrap() = Some(self.node_id);

        (self.node_id, new_term)
    }

    /// Menjadi Pemimpin setelah menerima suara mayoritas
    pub fn become_leader(&self) {
        *self.role.lock().unwrap() = RaftRole::Leader;
    }

    /// Append Log Entry oleh Leader
    pub fn append_entry(&self, command: Vec<u8>) -> u64 {
        let mut log = self.log.lock().unwrap();
        let index = log.len() as u64 + 1;
        let term = self.current_term.load(Ordering::SeqCst);

        log.push(LogEntry { term, index, command });
        index
    }

    /// Commit log entries sampai commit_index
    pub fn commit_to(&self, index: u64) {
        self.commit_index.store(index, Ordering::SeqCst);
    }

    pub fn get_role(&self) -> RaftRole {
        *self.role.lock().unwrap()
    }
}
