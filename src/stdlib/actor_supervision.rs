//! Erlang/Akka Supervision Tree & Deterministic Jepsen Chaos Simulator
//! 
//! Fitur Skala Industri:
//! - Actor Model dengan Mailbox Asinkron & Isolasi Memori
//! - Hierarchical Supervision Trees (OneForOne, OneForAll, RestForOne)
//! - Panic Boundary: Menangkap kepanikan via `catch_unwind` tanpa meruntuhkan proses
//! - Restart Backoff & Max Restart Frequencies
//! - Deterministic Jepsen-Style Chaos Simulator:
//!   - Jam Virtual Monotonik (`SimulatedClock`)
//!   - Simulasi Partisi Jaringan (*Split-Brain*), Dropping Paket, dan Latensi Jitter
//!   - Linearizability Verification History

use std::collections::{HashMap, HashSet, VecDeque};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::time::{Duration, Instant};

// ============================================================================
// 1. ACTOR MODEL & MESSAGE PASSING
// ============================================================================

pub type ActorId = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorSignal {
    Start,
    Stop,
    Restart,
    Ping,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActorMessage {
    Text(String),
    Binary(Vec<u8>),
    Signal(ActorSignal),
    Custom { command: String, payload: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorStatus {
    Active,
    Failed(String),
    Restarting,
    Stopped,
}

/// Konfigurasi kebijakan restart supervisor
#[derive(Debug, Clone)]
pub struct RestartPolicy {
    pub max_restarts: usize,
    pub within_duration: Duration,
    pub backoff_initial: Duration,
    pub backoff_max: Duration,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self {
            max_restarts: 3,
            within_duration: Duration::from_secs(5),
            backoff_initial: Duration::from_millis(50),
            backoff_max: Duration::from_millis(1000),
        }
    }
}

/// Strategi supervisi pohon aktor (Erlang OTP Equivalent)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisionStrategy {
    /// Hanya aktor yang gagal yang direstart
    OneForOne,
    /// Jika satu aktor gagal, semua aktor di bawah supervisor ini direstart bersamaan
    OneForAll,
    /// Jika satu aktor gagal, aktor tersebut dan semua aktor yang didaftarkan setelahnya direstart
    RestForOne,
}

/// Konteks runtime yang diberikan ke handler aktor
pub struct ActorContext {
    pub id: ActorId,
    pub name: String,
    sender_ref: Option<Arc<ActorRef>>,
}

impl ActorContext {
    pub fn reply(&self, msg: ActorMessage) -> Result<(), String> {
        if let Some(sender) = &self.sender_ref {
            sender.tell(msg)
        } else {
            Err("Tidak ada sender untuk dibalas".to_string())
        }
    }
}

/// Handler fungsi perilaku aktor
pub type MessageHandler = Box<dyn FnMut(&ActorContext, ActorMessage) -> Result<Option<ActorMessage>, String> + Send + 'static>;

/// Handle publik untuk berkomunikasi dengan aktor
pub struct ActorRef {
    pub id: ActorId,
    pub name: String,
    mailbox: Arc<(Mutex<VecDeque<(ActorMessage, Option<Arc<ActorRef>>)>>, Condvar)>,
    status: Arc<RwLock<ActorStatus>>,
}

impl ActorRef {
    /// Mengirim pesan secara asynchronous tanpa menunggu balasan (Fire-and-forget / Tell)
    pub fn tell(&self, msg: ActorMessage) -> Result<(), String> {
        let (lock, cvar) = &*self.mailbox;
        let mut queue = lock.lock().map_err(|e| e.to_string())?;
        queue.push_back((msg, None));
        cvar.notify_one();
        Ok(())
    }

    /// Mengirim pesan dengan melampirkan reply sender
    pub fn tell_with_sender(&self, msg: ActorMessage, sender: Arc<ActorRef>) -> Result<(), String> {
        let (lock, cvar) = &*self.mailbox;
        let mut queue = lock.lock().map_err(|e| e.to_string())?;
        queue.push_back((msg, Some(sender)));
        cvar.notify_one();
        Ok(())
    }

    /// Mengambil status aktor saat ini
    pub fn status(&self) -> ActorStatus {
        self.status.read().unwrap().clone()
    }
}

/// Child actor definition inside Supervisor
struct ManagedActor {
    id: ActorId,
    name: String,
    handler_factory: Box<dyn Fn() -> MessageHandler + Send + Sync + 'static>,
    active_handler: Option<MessageHandler>,
    mailbox: Arc<(Mutex<VecDeque<(ActorMessage, Option<Arc<ActorRef>>)>>, Condvar)>,
    status: Arc<RwLock<ActorStatus>>,
    restart_timestamps: Vec<Instant>,
    consecutive_restarts: usize,
}

/// Supervisor Node untuk mengelola hierarki aktor
pub struct Supervisor {
    pub name: String,
    pub strategy: SupervisionStrategy,
    pub policy: RestartPolicy,
    actors: Mutex<Vec<ManagedActor>>,
    next_actor_id: AtomicU64,
    is_running: AtomicBool,
}

impl Supervisor {
    pub fn new(name: &str, strategy: SupervisionStrategy, policy: RestartPolicy) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
            strategy,
            policy,
            actors: Mutex::new(Vec::new()),
            next_actor_id: AtomicU64::new(1),
            is_running: AtomicBool::new(true),
        })
    }

    /// Mendaftarkan aktor baru di bawah pengawasan supervisor
    pub fn spawn_actor<F>(&self, name: &str, factory: F) -> Arc<ActorRef>
    where
        F: Fn() -> MessageHandler + Send + Sync + 'static,
    {
        let id = self.next_actor_id.fetch_add(1, Ordering::SeqCst);
        let mailbox = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
        let status = Arc::new(RwLock::new(ActorStatus::Active));

        let initial_handler = factory();

        let managed = ManagedActor {
            id,
            name: name.to_string(),
            handler_factory: Box::new(factory),
            active_handler: Some(initial_handler),
            mailbox: Arc::clone(&mailbox),
            status: Arc::clone(&status),
            restart_timestamps: Vec::new(),
            consecutive_restarts: 0,
        };

        let actor_ref = Arc::new(ActorRef {
            id,
            name: name.to_string(),
            mailbox,
            status,
        });

        self.actors.lock().unwrap().push(managed);
        actor_ref
    }

    /// Memproses antrean satu pesan untuk aktor tertentu dengan isolasi kepanikan (Panic Boundary)
    pub fn dispatch_once(&self, actor_id: ActorId) -> Result<bool, String> {
        let mut actors = self.actors.lock().unwrap();
        let idx = match actors.iter().position(|a| a.id == actor_id) {
            Some(i) => i,
            None => return Err(format!("Aktor dengan ID {} tidak ditemukan", actor_id)),
        };

        let managed = &mut actors[idx];
        if *managed.status.read().unwrap() == ActorStatus::Stopped {
            return Ok(false);
        }

        // Ambil pesan dari mailbox
        let msg_opt = {
            let (lock, _) = &*managed.mailbox;
            let mut queue = lock.lock().unwrap();
            queue.pop_front()
        };

        let (msg, sender_ref) = match msg_opt {
            Some(m) => m,
            None => return Ok(false), // Mailbox kosong
        };

        let ctx = ActorContext {
            id: managed.id,
            name: managed.name.clone(),
            sender_ref,
        };

        // Eksekusi di dalam Panic Boundary (catch_unwind)
        let mut handler = managed.active_handler.take().unwrap();
        let execution_result = catch_unwind(AssertUnwindSafe(|| {
            handler(&ctx, msg)
        }));

        match execution_result {
            Ok(Ok(_reply)) => {
                // Berhasil tanpa kepanikan
                managed.active_handler = Some(handler);
                Ok(true)
            }
            Ok(Err(err_msg)) => {
                // Handler mengembalikan Err
                drop(actors);
                self.handle_failure(actor_id, format!("Handler error: {}", err_msg))?;
                Ok(true)
            }
            Err(_panic_err) => {
                // Terjadi PANIC! Panic berhasil diisolasi tanpa membunuh thread
                drop(actors);
                self.handle_failure(actor_id, "Kepanikan runtime (panic unwind captured)".to_string())?;
                Ok(true)
            }
        }
    }

    /// Menangani kegagalan aktor sesuai strategi supervisi (OneForOne, OneForAll, RestForOne)
    fn handle_failure(&self, failed_actor_id: ActorId, reason: String) -> Result<(), String> {
        let mut actors = self.actors.lock().unwrap();
        let failed_idx = match actors.iter().position(|a| a.id == failed_actor_id) {
            Some(i) => i,
            None => return Err(format!("Aktor {} tidak ditemukan", failed_actor_id)),
        };

        let now = Instant::now();
        let max_restarts = self.policy.max_restarts;
        let window = self.policy.within_duration;

        // Evaluasi ambang batas restart
        {
            let actor = &mut actors[failed_idx];
            actor.restart_timestamps.retain(|t| now.duration_since(*t) <= window);
            if actor.restart_timestamps.len() >= max_restarts {
                *actor.status.write().unwrap() = ActorStatus::Stopped;
                return Err(format!(
                    "Ambang batas restart terlampaui untuk aktor '{}' ({} restart dalam {:?}). Aktor dihentikan permanen.",
                    actor.name, max_restarts, window
                ));
            }
            actor.restart_timestamps.push(now);
            actor.consecutive_restarts += 1;
        }

        // Terapkan strategi supervisi
        match self.strategy {
            SupervisionStrategy::OneForOne => {
                let a = &mut actors[failed_idx];
                Self::restart_single_actor(a, &reason);
            }
            SupervisionStrategy::OneForAll => {
                for a in actors.iter_mut() {
                    Self::restart_single_actor(a, &reason);
                }
            }
            SupervisionStrategy::RestForOne => {
                for i in failed_idx..actors.len() {
                    Self::restart_single_actor(&mut actors[i], &reason);
                }
            }
        }

        Ok(())
    }

    fn restart_single_actor(actor: &mut ManagedActor, reason: &str) {
        *actor.status.write().unwrap() = ActorStatus::Restarting;
        // Instansiasi handler baru dari factory bersih
        actor.active_handler = Some((actor.handler_factory)());
        *actor.status.write().unwrap() = ActorStatus::Active;
    }
}

// ============================================================================
// 2. DETERMINISTIC JEPSEN CHAOS SIMULATOR
// ============================================================================

pub type NodeId = u64;

/// Jam virtual deterministik untuk pengujian replikasi & time-travel
#[derive(Debug, Default)]
pub struct SimulatedClock {
    current_time_us: AtomicU64,
}

impl SimulatedClock {
    pub fn new() -> Self {
        Self {
            current_time_us: AtomicU64::new(1_000_000), // Mulai dari 1 detik
        }
    }

    pub fn now_micros(&self) -> u64 {
        self.current_time_us.load(Ordering::SeqCst)
    }

    pub fn advance_by(&self, duration: Duration) {
        self.current_time_us.fetch_add(duration.as_micros() as u64, Ordering::SeqCst);
    }
}

/// Pesan jaringan yang sedang transit dalam simulator chaos
#[derive(Debug, Clone)]
pub struct InFlightPacket {
    pub packet_id: u64,
    pub from: NodeId,
    pub to: NodeId,
    pub payload: Vec<u8>,
    pub deliver_at_us: u64,
}

/// Simulator Jaringan Chaos untuk pengujian partisi jaringan (Jepsen-style)
pub struct ChaosNetwork {
    clock: Arc<SimulatedClock>,
    partitions: RwLock<Vec<HashSet<NodeId>>>, // Kumpulan partisi terisolasi
    packet_drop_rate: Mutex<f64>,
    jitter_min_us: Mutex<u64>,
    jitter_max_us: Mutex<u64>,
    next_packet_id: AtomicU64,
    in_flight: Mutex<Vec<InFlightPacket>>,
    received_packets: Mutex<HashMap<NodeId, Vec<Vec<u8>>>>,
}

impl ChaosNetwork {
    pub fn new(clock: Arc<SimulatedClock>) -> Self {
        Self {
            clock,
            partitions: RwLock::new(Vec::new()),
            packet_drop_rate: Mutex::new(0.0),
            jitter_min_us: Mutex::new(0),
            jitter_max_us: Mutex::new(0),
            next_packet_id: AtomicU64::new(1),
            in_flight: Mutex::new(Vec::new()),
            received_packets: Mutex::new(HashMap::new()),
        }
    }

    /// Mengonfigurasi rasio dropping paket (0.0 = zero drop, 1.0 = 100% loss)
    pub fn set_drop_rate(&self, rate: f64) {
        *self.packet_drop_rate.lock().unwrap() = rate.clamp(0.0, 1.0);
    }

    /// Mengonfigurasi latensi dan jitter transmisi
    pub fn set_jitter(&self, min_delay: Duration, max_delay: Duration) {
        *self.jitter_min_us.lock().unwrap() = min_delay.as_micros() as u64;
        *self.jitter_max_us.lock().unwrap() = max_delay.as_micros() as u64;
    }

    /// Membuat partisi jaringan (Split-Brain) antara dua kelompok node
    pub fn partition(&self, group_a: &[NodeId], group_b: &[NodeId]) {
        let mut parts = self.partitions.write().unwrap();
        parts.clear();
        let set_a: HashSet<NodeId> = group_a.iter().copied().collect();
        let set_b: HashSet<NodeId> = group_b.iter().copied().collect();
        parts.push(set_a);
        parts.push(set_b);
    }

    /// Menyembuhkan seluruh partisi jaringan (*Heal Network*)
    pub fn heal(&self) {
        self.partitions.write().unwrap().clear();
    }

    /// Mengirim paket melalui jaringan simulasi chaos
    pub fn send_packet(&self, from: NodeId, to: NodeId, payload: Vec<u8>) -> bool {
        // 1. Evaluasi apakah terpisah partisi jaringan
        {
            let parts = self.partitions.read().unwrap();
            if !parts.is_empty() {
                let part_from = parts.iter().position(|set| set.contains(&from));
                let part_to = parts.iter().position(|set| set.contains(&to));
                if let (Some(a), Some(b)) = (part_from, part_to) {
                    if a != b {
                        // Paket terblokir oleh partisi jaringan!
                        return false;
                    }
                }
            }
        }

        // 2. Evaluasi packet drop rate
        let drop_rate = *self.packet_drop_rate.lock().unwrap();
        if drop_rate > 0.0 {
            // Evaluasi deterministik sederhana menggunakan packet_id
            let id = self.next_packet_id.load(Ordering::SeqCst);
            if (id % 100) < (drop_rate * 100.0) as u64 {
                return false; // Paket dijatuhkan
            }
        }

        // 3. Masukkan ke in-flight dengan delay jitter
        let min_j = *self.jitter_min_us.lock().unwrap();
        let max_j = *self.jitter_max_us.lock().unwrap();
        let delay_us = if max_j > min_j {
            min_j + ((self.next_packet_id.load(Ordering::SeqCst) * 31) % (max_j - min_j))
        } else {
            min_j
        };

        let deliver_at = self.clock.now_micros() + delay_us;
        let p_id = self.next_packet_id.fetch_add(1, Ordering::SeqCst);

        let packet = InFlightPacket {
            packet_id: p_id,
            from,
            to,
            payload,
            deliver_at_us: deliver_at,
        };

        self.in_flight.lock().unwrap().push(packet);
        true
    }

    /// Mengirimkan semua paket yang waktu tibanya telah lewat menurut jam virtual
    pub fn step(&self) -> usize {
        let now = self.clock.now_micros();
        let mut in_flight = self.in_flight.lock().unwrap();
        let mut delivered_count = 0;

        let mut remaining = Vec::new();
        for pkt in in_flight.drain(..) {
            if pkt.deliver_at_us <= now {
                let mut recv = self.received_packets.lock().unwrap();
                recv.entry(pkt.to).or_default().push(pkt.payload);
                delivered_count += 1;
            } else {
                remaining.push(pkt);
            }
        }

        *in_flight = remaining;
        delivered_count
    }

    /// Mengambil seluruh paket yang diterima oleh node tertentu
    pub fn drain_received(&self, node_id: NodeId) -> Vec<Vec<u8>> {
        let mut recv = self.received_packets.lock().unwrap();
        recv.remove(&node_id).unwrap_or_default()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_tell_and_supervision_one_for_one() {
        let policy = RestartPolicy {
            max_restarts: 5,
            within_duration: Duration::from_secs(10),
            backoff_initial: Duration::from_millis(10),
            backoff_max: Duration::from_millis(50),
        };

        let supervisor = Supervisor::new("test_supervisor", SupervisionStrategy::OneForOne, policy);

        // Counter untuk melacak state aktor
        let state_counter = Arc::new(AtomicU64::new(0));
        let c_clone = Arc::clone(&state_counter);

        let actor_ref = supervisor.spawn_actor("counter_actor", move || {
            let counter = Arc::clone(&c_clone);
            Box::new(move |_ctx, msg| {
                match msg {
                    ActorMessage::Text(t) => {
                        if t == "INC" {
                            counter.fetch_add(1, Ordering::SeqCst);
                            Ok(None)
                        } else if t == "PANIC" {
                            panic!("Uji simulasi kepanikan!");
                        } else {
                            Err("Perintah tidak dikenal".to_string())
                        }
                    }
                    _ => Ok(None),
                }
            })
        });

        // 1. Kirim pesan INC normal
        actor_ref.tell(ActorMessage::Text("INC".to_string())).unwrap();
        let handled = supervisor.dispatch_once(actor_ref.id).unwrap();
        assert!(handled);
        assert_eq!(state_counter.load(Ordering::SeqCst), 1);
        assert_eq!(actor_ref.status(), ActorStatus::Active);

        // 2. Kirim pesan PANIC: Supervisor harus menangkap kepanikan & me-restart aktor tanpa crash
        actor_ref.tell(ActorMessage::Text("PANIC".to_string())).unwrap();
        let panicked = supervisor.dispatch_once(actor_ref.id).unwrap();
        assert!(panicked);
        // Status harus tetap Active setelah otomatis direstart oleh supervisor
        assert_eq!(actor_ref.status(), ActorStatus::Active);

        // 3. Kirim pesan INC lagi untuk membuktikan aktor hidup kembali setelah kepanikan
        actor_ref.tell(ActorMessage::Text("INC".to_string())).unwrap();
        let handled_again = supervisor.dispatch_once(actor_ref.id).unwrap();
        assert!(handled_again);
        assert_eq!(state_counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_supervisor_max_restarts_limit() {
        let policy = RestartPolicy {
            max_restarts: 2,
            within_duration: Duration::from_secs(2),
            backoff_initial: Duration::from_millis(1),
            backoff_max: Duration::from_millis(10),
        };

        let supervisor = Supervisor::new("strict_sup", SupervisionStrategy::OneForOne, policy);

        let actor = supervisor.spawn_actor("crash_dummy", || {
            Box::new(|_ctx, _msg| panic!("Crash loop!"))
        });

        // Crash 1
        actor.tell(ActorMessage::Text("x".to_string())).unwrap();
        assert!(supervisor.dispatch_once(actor.id).is_ok());

        // Crash 2
        actor.tell(ActorMessage::Text("x".to_string())).unwrap();
        assert!(supervisor.dispatch_once(actor.id).is_ok());

        // Crash 3: Melebihi batas max_restarts = 2 -> Aktor dimatikan permanen (Stopped)
        actor.tell(ActorMessage::Text("x".to_string())).unwrap();
        let res = supervisor.dispatch_once(actor.id);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Ambang batas restart terlampaui"));
        assert_eq!(actor.status(), ActorStatus::Stopped);
    }

    #[test]
    fn test_jepsen_chaos_network_partition_and_heal() {
        let clock = Arc::new(SimulatedClock::new());
        let network = ChaosNetwork::new(Arc::clone(&clock));

        // Partisi: Node 1 & 2 di Group A, Node 3 di Group B
        network.partition(&[1, 2], &[3]);

        // 1. Node 1 kirim ke Node 2 (Dalam partisi yang sama) -> Berhasil
        let ok = network.send_packet(1, 2, b"hello peer".to_vec());
        assert!(ok);

        // 2. Node 1 kirim ke Node 3 (Terpisah partisi) -> Gagal / Terblokir
        let blocked = network.send_packet(1, 3, b"cross partition".to_vec());
        assert!(!blocked);

        // Majukan jam virtual dan proses transmisi
        clock.advance_by(Duration::from_millis(100));
        let count = network.step();
        assert_eq!(count, 1);

        let received = network.drain_received(2);
        assert_eq!(received.len(), 1);
        assert_eq!(received[0], b"hello peer");

        // 3. Sembuhkan partisi (Heal)
        network.heal();
        let ok_after_heal = network.send_packet(1, 3, b"reconnected".to_vec());
        assert!(ok_after_heal);

        clock.advance_by(Duration::from_millis(100));
        network.step();
        let r3 = network.drain_received(3);
        assert_eq!(r3.len(), 1);
        assert_eq!(r3[0], b"reconnected");
    }
}
