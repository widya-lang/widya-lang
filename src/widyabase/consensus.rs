//! Konsensus Terdistribusi Raft & Koordinasi Transaksi 2PC (Two-Phase Commit)
//! untuk WidyaBase Cluster (consensus.rs)
//!
//! Modul ini menyediakan:
//! 1. Protokol Konsensus Raft:
//!    - Pemilihan Pemimpin (Leader Election) dengan Quorum Majority
//!    - Replikasi Log (AppendEntries RPC) & Sinkronisasi Commit Index
//!    - Heartbeat & Deteksi Kegagalan Jaringan / Partisi Node
//! 2. Distributed Two-Phase Commit (2PC):
//!    - Fase 1: Prepare (Penyelarasan & Penguncian Transaksi Terdistribusi)
//!    - Fase 2: Global Commit / Global Abort dengan Ketahanan Atomik

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RaftRole {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LogEntry {
    pub term: u64,
    pub index: u64,
    pub command: Vec<u8>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestVoteArgs {
    pub term: u64,
    pub candidate_id: u64,
    pub last_log_index: u64,
    pub last_log_term: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestVoteReply {
    pub term: u64,
    pub vote_granted: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppendEntriesArgs {
    pub term: u64,
    pub leader_id: u64,
    pub prev_log_index: u64,
    pub prev_log_term: u64,
    pub entries: Vec<LogEntry>,
    pub leader_commit: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppendEntriesReply {
    pub term: u64,
    pub success: bool,
    pub match_index: u64,
}

pub struct RaftNode {
    pub node_id: u64,
    pub current_term: AtomicU64,
    pub voted_for: Mutex<Option<u64>>,
    pub role: Mutex<RaftRole>,
    pub log: Mutex<Vec<LogEntry>>,
    pub commit_index: AtomicU64,
    pub last_applied: AtomicU64,
    pub match_index: Mutex<HashMap<u64, u64>>,
    pub next_index: Mutex<HashMap<u64, u64>>,
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
            last_applied: AtomicU64::new(0),
            match_index: Mutex::new(HashMap::new()),
            next_index: Mutex::new(HashMap::new()),
        }
    }

    /// Mulai Pemilihan Pemimpin (Election Timeout Triggered)
    pub fn start_election(&self) -> (u64, u64) {
        let new_term = self.current_term.fetch_add(1, Ordering::SeqCst) + 1;
        *self.role.lock().unwrap() = RaftRole::Candidate;
        *self.voted_for.lock().unwrap() = Some(self.node_id);

        (self.node_id, new_term)
    }

    /// Menjadi Pemimpin setelah menerima suara mayoritas (Quorum)
    pub fn become_leader(&self) {
        *self.role.lock().unwrap() = RaftRole::Leader;
    }

    /// Menjadi Follower (misal saat menemukan term yang lebih tinggi)
    pub fn become_follower(&self, term: u64) {
        self.current_term.store(term, Ordering::SeqCst);
        *self.role.lock().unwrap() = RaftRole::Follower;
        *self.voted_for.lock().unwrap() = None;
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

    pub fn get_last_log_info(&self) -> (u64, u64) {
        let log = self.log.lock().unwrap();
        if let Some(entry) = log.last() {
            (entry.index, entry.term)
        } else {
            (0, 0)
        }
    }

    /// Menangani RPC RequestVote dari Kandidat
    pub fn handle_request_vote(&self, args: RequestVoteArgs) -> RequestVoteReply {
        let mut current_term = self.current_term.load(Ordering::SeqCst);

        // Jika term kandidat lebih tinggi, perbarui term sendiri dan beralih ke Follower
        if args.term > current_term {
            self.become_follower(args.term);
            current_term = args.term;
        }

        if args.term < current_term {
            return RequestVoteReply {
                term: current_term,
                vote_granted: false,
            };
        }

        let mut voted_for = self.voted_for.lock().unwrap();
        let can_vote = voted_for.is_none() || *voted_for == Some(args.candidate_id);

        if can_vote {
            // Periksa apakah log kandidat setidaknya sama terkininya dengan log pemilih
            let (my_last_index, my_last_term) = self.get_last_log_info();
            let is_up_to_date = args.last_log_term > my_last_term
                || (args.last_log_term == my_last_term && args.last_log_index >= my_last_index);

            if is_up_to_date {
                *voted_for = Some(args.candidate_id);
                return RequestVoteReply {
                    term: current_term,
                    vote_granted: true,
                };
            }
        }

        RequestVoteReply {
            term: current_term,
            vote_granted: false,
        }
    }

    /// Menangani RPC AppendEntries (Replikasi Log & Heartbeat) dari Leader
    pub fn handle_append_entries(&self, args: AppendEntriesArgs) -> AppendEntriesReply {
        let mut current_term = self.current_term.load(Ordering::SeqCst);

        if args.term > current_term {
            self.become_follower(args.term);
            current_term = args.term;
        }

        if args.term < current_term {
            return AppendEntriesReply {
                term: current_term,
                success: false,
                match_index: self.log.lock().unwrap().len() as u64,
            };
        }

        // Jika saat ini candidate dan menerima heartbeat dari leader sah, beralih ke follower
        let mut role = self.role.lock().unwrap();
        if *role == RaftRole::Candidate {
            *role = RaftRole::Follower;
        }
        drop(role);

        let mut log = self.log.lock().unwrap();

        // Validasi prev_log_index dan prev_log_term
        if args.prev_log_index > 0 {
            let prev_idx = (args.prev_log_index - 1) as usize;
            if prev_idx >= log.len() || log[prev_idx].term != args.prev_log_term {
                return AppendEntriesReply {
                    term: current_term,
                    success: false,
                    match_index: log.len() as u64,
                };
            }
        }

        // Pangkas entri log yang konflik dan tambahkan entri baru
        let mut insert_pos = args.prev_log_index as usize;
        for entry in args.entries {
            if insert_pos < log.len() {
                if log[insert_pos].term != entry.term {
                    log.truncate(insert_pos);
                    log.push(entry);
                }
            } else {
                log.push(entry);
            }
            insert_pos += 1;
        }

        // Perbarui commit_index sesuai instruksi leader
        if args.leader_commit > self.commit_index.load(Ordering::SeqCst) {
            let new_commit = args.leader_commit.min(log.len() as u64);
            self.commit_index.store(new_commit, Ordering::SeqCst);
        }

        let match_index = log.len() as u64;

        AppendEntriesReply {
            term: current_term,
            success: true,
            match_index,
        }
    }
}

/// Koordinator Cluster Raft Multi-Node (In-Process & Network Simulation)
pub struct RaftCluster {
    pub nodes: RwLock<HashMap<u64, Arc<RaftNode>>>,
    pub isolated_nodes: Mutex<HashSet<u64>>, // Untuk simulasi partisi jaringan / failover
}

impl RaftCluster {
    pub fn new(node_ids: Vec<u64>) -> Self {
        let mut map = HashMap::new();
        for id in node_ids {
            map.insert(id, Arc::new(RaftNode::new(id)));
        }
        Self {
            nodes: RwLock::new(map),
            isolated_nodes: Mutex::new(HashSet::new()),
        }
    }

    pub fn add_node(&self, node: Arc<RaftNode>) {
        self.nodes.write().unwrap().insert(node.node_id, node);
    }

    pub fn get_node(&self, node_id: u64) -> Option<Arc<RaftNode>> {
        self.nodes.read().unwrap().get(&node_id).cloned()
    }

    pub fn isolate_node(&self, node_id: u64) {
        self.isolated_nodes.lock().unwrap().insert(node_id);
    }

    pub fn reconnect_node(&self, node_id: u64) {
        self.isolated_nodes.lock().unwrap().remove(&node_id);
    }

    pub fn is_isolated(&self, node_id: u64) -> bool {
        self.isolated_nodes.lock().unwrap().contains(&node_id)
    }

    pub fn get_leader(&self) -> Option<u64> {
        let nodes = self.nodes.read().unwrap();
        let isolated = self.isolated_nodes.lock().unwrap();
        for (id, node) in nodes.iter() {
            if !isolated.contains(id) && node.get_role() == RaftRole::Leader {
                return Some(*id);
            }
        }
        None
    }

    /// Memulai pemilihan untuk kandidat tertentu dengan meminta suara ke semua peer aktif
    pub fn start_election_for(&self, candidate_id: u64) -> bool {
        if self.is_isolated(candidate_id) {
            return false;
        }

        let nodes = self.nodes.read().unwrap();
        let candidate = match nodes.get(&candidate_id) {
            Some(n) => Arc::clone(n),
            None => return false,
        };

        let (_id, term) = candidate.start_election();
        let (last_log_index, last_log_term) = candidate.get_last_log_info();

        let vote_args = RequestVoteArgs {
            term,
            candidate_id,
            last_log_index,
            last_log_term,
        };

        let mut votes = 1; // Suara sendiri
        let total_nodes = nodes.len();
        let quorum = (total_nodes / 2) + 1;

        for (peer_id, peer) in nodes.iter() {
            if *peer_id == candidate_id || self.is_isolated(*peer_id) {
                continue;
            }

            let reply = peer.handle_request_vote(vote_args.clone());
            if reply.vote_granted {
                votes += 1;
            }
        }

        if votes >= quorum {
            candidate.become_leader();

            // Inisialisasi next_index dan match_index untuk semua follower
            let last_index = candidate.log.lock().unwrap().len() as u64;
            let mut next_map = candidate.next_index.lock().unwrap();
            let mut match_map = candidate.match_index.lock().unwrap();
            for peer_id in nodes.keys() {
                if *peer_id != candidate_id {
                    next_map.insert(*peer_id, last_index + 1);
                    match_map.insert(*peer_id, 0);
                }
            }

            true
        } else {
            false
        }
    }

    /// Leader menyebarkan log entry ke seluruh follower dan melakukan commit jika kuorum tercapai
    pub fn replicate(&self, leader_id: u64, command: Vec<u8>) -> Result<(u64, usize), String> {
        if self.is_isolated(leader_id) {
            return Err(format!("Node {} terisolasi dari cluster", leader_id));
        }

        let nodes = self.nodes.read().unwrap();
        let leader = match nodes.get(&leader_id) {
            Some(n) => Arc::clone(n),
            None => return Err(format!("Leader {} tidak ditemukan", leader_id)),
        };

        if leader.get_role() != RaftRole::Leader {
            return Err(format!("Node {} bukan merupakan Leader", leader_id));
        }

        let entry_index = leader.append_entry(command.clone());
        let term = leader.current_term.load(Ordering::SeqCst);

        let total_nodes = nodes.len();
        let quorum = (total_nodes / 2) + 1;
        let mut quorum_count = 1; // Suara leader sendiri

        for (peer_id, peer) in nodes.iter() {
            if *peer_id == leader_id || self.is_isolated(*peer_id) {
                continue;
            }

            let (prev_index, prev_term) = {
                let log = leader.log.lock().unwrap();
                if entry_index > 1 && (entry_index as usize - 2) < log.len() {
                    let prev_entry = &log[entry_index as usize - 2];
                    (prev_entry.index, prev_entry.term)
                } else {
                    (0, 0)
                }
            };

            let args = AppendEntriesArgs {
                term,
                leader_id,
                prev_log_index: prev_index,
                prev_log_term: prev_term,
                entries: vec![LogEntry {
                    term,
                    index: entry_index,
                    command: command.clone(),
                }],
                leader_commit: leader.commit_index.load(Ordering::SeqCst),
            };

            let reply = peer.handle_append_entries(args);
            if reply.success {
                leader.match_index.lock().unwrap().insert(*peer_id, reply.match_index);
                quorum_count += 1;
            }
        }

        if quorum_count >= quorum {
            leader.commit_to(entry_index);

            // Perbarui commit_index pada follower yang tersambung
            for (peer_id, peer) in nodes.iter() {
                if *peer_id != leader_id && !self.is_isolated(*peer_id) {
                    peer.commit_to(entry_index);
                }
            }

            Ok((entry_index, quorum_count))
        } else {
            Err(format!(
                "Gagal mencapai kuorum replikasi Raft: hanya {}/{} node yang mengonfirmasi",
                quorum_count, total_nodes
            ))
        }
    }

    /// Mengirimkan heartbeat kosong ke seluruh follower
    pub fn broadcast_heartbeat(&self, leader_id: u64) {
        if self.is_isolated(leader_id) {
            return;
        }

        let nodes = self.nodes.read().unwrap();
        let leader = match nodes.get(&leader_id) {
            Some(n) => Arc::clone(n),
            None => return,
        };

        if leader.get_role() != RaftRole::Leader {
            return;
        }

        let term = leader.current_term.load(Ordering::SeqCst);
        let commit = leader.commit_index.load(Ordering::SeqCst);
        let (last_idx, last_term) = leader.get_last_log_info();

        for (peer_id, peer) in nodes.iter() {
            if *peer_id != leader_id && !self.is_isolated(*peer_id) {
                let args = AppendEntriesArgs {
                    term,
                    leader_id,
                    prev_log_index: last_idx,
                    prev_log_term: last_term,
                    entries: Vec::new(),
                    leader_commit: commit,
                };
                peer.handle_append_entries(args);
            }
        }
    }

    /// Automated Failover: Deteksi kegagalan leader dan lakukan pemilihan otomatis pada node kuorum yang bertahan
    pub fn trigger_automated_failover(&self) -> Result<u64, String> {
        let nodes = self.nodes.read().unwrap();
        let isolated = self.isolated_nodes.lock().unwrap();

        // Cari node yang tidak terisolasi dan memiliki index log tertinggi
        let mut candidates: Vec<u64> = nodes
            .keys()
            .copied()
            .filter(|id| !isolated.contains(id))
            .collect();

        if candidates.is_empty() {
            return Err("Tidak ada node yang tersisa untuk failover (seluruh cluster terisolasi)".to_string());
        }

        // Urutkan kandidat berdasarkan last log term & index
        candidates.sort_by(|&a, &b| {
            let info_a = nodes.get(&a).map(|n| n.get_last_log_info()).unwrap_or((0, 0));
            let info_b = nodes.get(&b).map(|n| n.get_last_log_info()).unwrap_or((0, 0));
            // Prioritaskan term lebih tinggi, lalu index lebih panjang
            (info_b.1, info_b.0).cmp(&(info_a.1, info_a.0))
        });

        drop(nodes);
        drop(isolated);

        for cand_id in candidates {
            if self.start_election_for(cand_id) {
                // Berhasil terpilih menjadi Leader baru!
                self.broadcast_heartbeat(cand_id);
                return Ok(cand_id);
            }
        }

        Err("Gagal mencapai kuorum suara mayoritas dalam pemilihan failover otomatis".to_string())
    }

    /// Memeriksa kesehatan cluster dan memicu failover otomatis jika leader tidak aktif
    pub fn check_and_recover_cluster(&self) -> Option<u64> {
        match self.get_leader() {
            Some(leader) => Some(leader),
            None => self.trigger_automated_failover().ok(),
        }
    }

    /// Sinkronisasi & Replikasi Rekod WAL (Write-Ahead Logging) melalui Raft Consensus
    pub fn replicate_wal_record(
        &self,
        leader_id: u64,
        record: &crate::widyabase::wal::WalRecord,
    ) -> Result<(u64, usize), String> {
        let bytes = record.to_bytes();
        self.replicate(leader_id, bytes)
    }

    /// Menjalankan Replikasi Aliran WAL (Streaming WAL Replication) untuk sekumpulan rekod
    pub fn replicate_wal_stream(
        &self,
        leader_id: u64,
        records: &[crate::widyabase::wal::WalRecord],
    ) -> Result<u64, String> {
        let mut last_index = 0;
        for rec in records {
            let (idx, _) = self.replicate_wal_record(leader_id, rec)?;
            last_index = idx;
        }
        Ok(last_index)
    }

    /// Sinkronisasi Susulan (Catch-Up Replication) untuk node follower yang tertinggal
    pub fn catch_up_follower_wal(
        &self,
        leader_id: u64,
        follower_id: u64,
        from_index: u64,
    ) -> Result<usize, String> {
        if self.is_isolated(leader_id) || self.is_isolated(follower_id) {
            return Err("Salah satu node terisolasi dari jaringan".to_string());
        }

        let nodes = self.nodes.read().unwrap();
        let leader = nodes.get(&leader_id).ok_or("Leader tidak ditemukan")?;
        let follower = nodes.get(&follower_id).ok_or("Follower tidak ditemukan")?;

        let leader_log = leader.log.lock().unwrap();
        let start_pos = if from_index > 0 { (from_index - 1) as usize } else { 0 };

        if start_pos >= leader_log.len() {
            return Ok(0); // Sudah sinkron
        }

        let missing_entries: Vec<LogEntry> = leader_log[start_pos..].to_vec();
        let count = missing_entries.len();

        let term = leader.current_term.load(Ordering::SeqCst);
        let commit = leader.commit_index.load(Ordering::SeqCst);

        let (prev_index, prev_term) = if start_pos > 0 {
            (leader_log[start_pos - 1].index, leader_log[start_pos - 1].term)
        } else {
            (0, 0)
        };

        drop(leader_log);

        let args = AppendEntriesArgs {
            term,
            leader_id,
            prev_log_index: prev_index,
            prev_log_term: prev_term,
            entries: missing_entries,
            leader_commit: commit,
        };

        let reply = follower.handle_append_entries(args);
        if reply.success {
            leader.match_index.lock().unwrap().insert(follower_id, reply.match_index);
            Ok(count)
        } else {
            Err("Follower menolak sinkronisasi susulan WAL".to_string())
        }
    }
}

/// Mesin Replikasi Basis Data Terdistribusi Aktif-Aktif & Failover Otomatis
pub struct DistributedReplicationEngine {
    pub node_id: u64,
    pub cluster: Arc<RaftCluster>,
    pub local_lsn: AtomicU64,
}

impl DistributedReplicationEngine {
    pub fn new(node_id: u64, cluster: Arc<RaftCluster>) -> Self {
        Self {
            node_id,
            cluster,
            local_lsn: AtomicU64::new(1),
        }
    }

    /// Memeriksa apakah node saat ini bertindak sebagai Leader
    pub fn is_leader(&self) -> bool {
        self.cluster
            .get_node(self.node_id)
            .map(|n| n.get_role() == RaftRole::Leader)
            .unwrap_or(false)
    }

    /// Mendapatkan ID pemimpin aktif cluster saat ini
    pub fn current_leader(&self) -> Option<u64> {
        self.cluster.get_leader()
    }

    /// Menjalankan transaksi tulis terdistribusi:
    /// Menjamin ketahanan kuorum konsensus sebelum status committed dikembalikan.
    pub fn execute_write_transaction(
        &self,
        record: &crate::widyabase::wal::WalRecord,
    ) -> Result<(u64, usize), String> {
        let leader_id = match self.cluster.get_leader() {
            Some(id) => id,
            None => {
                // Picu failover otomatis jika leader hilang
                self.cluster.trigger_automated_failover()?
            }
        };

        let res = self.cluster.replicate_wal_record(leader_id, record)?;
        self.local_lsn.store(record.lsn, Ordering::SeqCst);
        Ok(res)
    }

    /// Memicu automated failover saat node mendeteksi kehilangan heartbeat
    pub fn trigger_failover(&self) -> Result<u64, String> {
        self.cluster.trigger_automated_failover()
    }
}

// ============================================================================
// DISTRIBUTED TWO-PHASE COMMIT (2PC) COORDINATOR & PARTICIPANT
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TwoPhaseStatus {
    Active,
    Preparing,
    Prepared,
    Committed,
    Aborted,
}

pub struct TwoPhaseParticipant {
    pub node_id: u64,
    pub tx_states: Mutex<HashMap<u64, (TwoPhaseStatus, Vec<u8>)>>,
    pub forced_aborts: Mutex<HashSet<u64>>, // Untuk simulasi kegagalan pemungutan suara
}

impl TwoPhaseParticipant {
    pub fn new(node_id: u64) -> Self {
        Self {
            node_id,
            tx_states: Mutex::new(HashMap::new()),
            forced_aborts: Mutex::new(HashSet::new()),
        }
    }

    pub fn set_forced_abort(&self, tx_id: u64, should_abort: bool) {
        let mut set = self.forced_aborts.lock().unwrap();
        if should_abort {
            set.insert(tx_id);
        } else {
            set.remove(&tx_id);
        }
    }

    /// Fase 1: Prepare (Simpan ke WAL log prepare dan kunci resource)
    pub fn prepare(&self, tx_id: u64, data: Vec<u8>) -> Result<bool, String> {
        let should_abort = self.forced_aborts.lock().unwrap().contains(&tx_id);
        let mut states = self.tx_states.lock().unwrap();

        if should_abort {
            states.insert(tx_id, (TwoPhaseStatus::Aborted, data));
            return Ok(false); // Vote: ABORT
        }

        states.insert(tx_id, (TwoPhaseStatus::Prepared, data));
        Ok(true) // Vote: COMMIT
    }

    /// Fase 2: Global Commit
    pub fn commit(&self, tx_id: u64) -> Result<(), String> {
        let mut states = self.tx_states.lock().unwrap();
        if let Some((status, _)) = states.get_mut(&tx_id) {
            *status = TwoPhaseStatus::Committed;
            Ok(())
        } else {
            Err(format!("Transaksi {} tidak ditemukan pada node {}", tx_id, self.node_id))
        }
    }

    /// Fase 2: Global Abort
    pub fn abort(&self, tx_id: u64) -> Result<(), String> {
        let mut states = self.tx_states.lock().unwrap();
        if let Some((status, _)) = states.get_mut(&tx_id) {
            *status = TwoPhaseStatus::Aborted;
            Ok(())
        } else {
            // Bila belum disiapkan, tetap catat status Aborted
            states.insert(tx_id, (TwoPhaseStatus::Aborted, Vec::new()));
            Ok(())
        }
    }

    pub fn get_status(&self, tx_id: u64) -> Option<TwoPhaseStatus> {
        self.tx_states.lock().unwrap().get(&tx_id).map(|(s, _)| *s)
    }
}

pub struct TwoPhaseCoordinator {
    pub coordinator_id: u64,
    pub participants: RwLock<HashMap<u64, Arc<TwoPhaseParticipant>>>,
    pub global_tx_log: Mutex<HashMap<u64, TwoPhaseStatus>>,
}

impl TwoPhaseCoordinator {
    pub fn new(coordinator_id: u64) -> Self {
        Self {
            coordinator_id,
            participants: RwLock::new(HashMap::new()),
            global_tx_log: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_participant(&self, participant: Arc<TwoPhaseParticipant>) {
        self.participants.write().unwrap().insert(participant.node_id, participant);
    }

    /// Mengeksekusi transaksi 2PC terdistribusi penuh
    pub fn execute_2pc(
        &self,
        tx_id: u64,
        participant_ids: &[u64],
        data: Vec<u8>,
    ) -> Result<TwoPhaseStatus, String> {
        let parts_map = self.participants.read().unwrap();
        self.global_tx_log.lock().unwrap().insert(tx_id, TwoPhaseStatus::Preparing);

        // --- FASE 1: PREPARE ---
        let mut all_voted_commit = true;
        let mut available_participants = Vec::new();

        for &p_id in participant_ids {
            if let Some(p) = parts_map.get(&p_id) {
                available_participants.push(Arc::clone(p));
                match p.prepare(tx_id, data.clone()) {
                    Ok(true) => {} // Vote COMMIT
                    _ => {
                        all_voted_commit = false; // Vote ABORT atau error
                    }
                }
            } else {
                all_voted_commit = false;
            }
        }

        // --- FASE 2: COMMIT ATAU ABORT ---
        if all_voted_commit {
            // Catat Global Commit pada WAL koordinator
            self.global_tx_log.lock().unwrap().insert(tx_id, TwoPhaseStatus::Committed);

            // Kirim Global Commit ke seluruh partisipan
            for p in &available_participants {
                let _ = p.commit(tx_id);
            }

            Ok(TwoPhaseStatus::Committed)
        } else {
            // Catat Global Abort pada WAL koordinator
            self.global_tx_log.lock().unwrap().insert(tx_id, TwoPhaseStatus::Aborted);

            // Kirim Global Abort ke seluruh partisipan
            for p in &available_participants {
                let _ = p.abort(tx_id);
            }

            Ok(TwoPhaseStatus::Aborted)
        }
    }

    pub fn get_transaction_status(&self, tx_id: u64) -> Option<TwoPhaseStatus> {
        self.global_tx_log.lock().unwrap().get(&tx_id).copied()
    }
}
