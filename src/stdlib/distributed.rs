use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::collections::invoke_closure_or_fn;
use crate::stdlib::common::*;
use crate::value::Value;
use crate::stdlib::concurrency::ValueSerial;
use sha2::{Digest, Sha256};
use std::sync::OnceLock;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub fn register_distributed(env: &mut Environment) {
    // WidyaActor (Actor Model Concurrency System)
    register_fn(env, "Aktor", Some(2), builtin_aktor_buat);
    register_fn(env, "kirim_ke_aktor", Some(2), builtin_aktor_kirim);
    register_fn(env, "tunggu_respon_aktor", Some(1), builtin_aktor_tunggu_respon);

    // WidyaRaft (Distributed Consensus Engine)
    register_fn(env, "NodeRaft", Some(2), builtin_raft_node);
    register_fn(env, "usulkan_log", Some(2), builtin_raft_usul);
    register_fn(env, "pilih_pemimpin", Some(1), builtin_raft_pilih);
    register_fn(env, "status_konsensus", Some(1), builtin_raft_status);

    // WidyaActorSystem (Supervised Actor Concurrency & Mailbox)
    register_fn(env, "SistemAktor", None, builtin_actor_system_buat);
    register_fn(env, "aktor_spawn", Some(3), builtin_actor_spawn);
    register_fn(env, "aktor_kirim", Some(2), builtin_actor_kirim);
    register_fn(env, "aktor_status", Some(1), builtin_actor_status);
    register_fn(env, "aktor_mailbox_panjang", Some(1), builtin_actor_mailbox_panjang);

    // WidyaRaftLite (Distributed Consensus Engine)
    register_fn(env, "KlasterRaft", Some(1), builtin_raft_buat);
    register_fn(env, "raft_usulkan_perintah", Some(2), builtin_raft_usulkan_perintah);
    register_fn(env, "raft_detak_pemilihan", Some(1), builtin_raft_detak_pemilihan);
    register_fn(env, "raft_dapatkan_leader", Some(1), builtin_raft_dapatkan_leader);
    register_fn(env, "raft_klaster_status", Some(1), builtin_raft_klaster_status);

    // WidyaGossip (Epidemic / P2P Dissemination Protocol)
    register_fn(env, "KlasterGossip", Some(1), builtin_gossip_buat);
    register_fn(env, "gossip_siarkan", Some(3), builtin_gossip_siarkan);
    register_fn(env, "gossip_detak", Some(1), builtin_gossip_detak);
    register_fn(env, "gossip_keadaan_node", Some(2), builtin_gossip_keadaan_node);

    // WidyaHLC (Hybrid Logical Clock & Causality Tracking)
    register_fn(env, "JamHLC", None, builtin_hlc_buat);
    register_fn(env, "hlc_detak", Some(1), builtin_hlc_detak);
    register_fn(env, "hlc_perbarui", Some(2), builtin_hlc_perbarui);
    register_fn(env, "hlc_apakah_sebelum", Some(2), builtin_hlc_apakah_sebelum);
    register_fn(env, "hlc_format", Some(1), builtin_hlc_format);

    // WidyaKademlia (K-Bucket Routing & XOR Metric P2P DHT)
    register_fn(env, "TabelKademlia", None, builtin_kademlia_buat);
    register_fn(env, "kademlia_tambah_node", None, builtin_kademlia_tambah_node);
    register_fn(env, "kademlia_node_terdekat", None, builtin_kademlia_node_terdekat);
    register_fn(env, "kademlia_simpan", Some(3), builtin_kademlia_simpan);
    register_fn(env, "kademlia_ambil", Some(2), builtin_kademlia_ambil);

    // WidyaBullyElection (Distributed Bully Leader Election Protocol)
    register_fn(env, "KlasterBully", Some(1), builtin_bully_buat);
    register_fn(env, "bully_mulai_pemilihan", Some(2), builtin_bully_mulai_pemilihan);
    register_fn(env, "bully_detak", Some(1), builtin_bully_detak);
    register_fn(env, "bully_matikan_node", Some(2), builtin_bully_matikan_node);
    register_fn(env, "bully_dapatkan_leader", Some(1), builtin_bully_dapatkan_leader);

    // WidyaPaxos (Classic Distributed Paxos Consensus Protocol)
    register_fn(env, "KlasterPaxos", Some(1), builtin_paxos_buat);
    register_fn(env, "paxos_siapkan", Some(3), builtin_paxos_siapkan);
    register_fn(env, "paxos_usulkan", Some(4), builtin_paxos_usulkan);
    register_fn(env, "paxos_putuskan", Some(2), builtin_paxos_putuskan);
    register_fn(env, "paxos_nilai_terpilih", Some(1), builtin_paxos_nilai_terpilih);

    // WidyaVectorClock (Multi-Node Vector Clocks for Distributed Event Causality)
    register_fn(env, "JamVektor", None, builtin_vclock_buat);
    register_fn(env, "vclock_detak", Some(1), builtin_vclock_detak);
    register_fn(env, "vclock_gabung", Some(2), builtin_vclock_gabung);
    register_fn(env, "vclock_apakah_konflik", Some(2), builtin_vclock_apakah_konflik);
    register_fn(env, "vclock_bandingkan", Some(2), builtin_vclock_bandingkan);
    register_fn(env, "vclock_ke_kamus", Some(1), builtin_vclock_ke_kamus);

    // WidyaKademliaRPC (Decentralized P2P RPC Message Protocol)
    register_fn(env, "kademlia_rpc_buat_pesan", Some(4), builtin_kademlia_rpc_buat_pesan);
    register_fn(env, "kademlia_rpc_uraikan_pesan", Some(1), builtin_kademlia_rpc_uraikan_pesan);
    register_fn(env, "kademlia_rpc_buat_respons", Some(2), builtin_kademlia_rpc_buat_respons);

}

// --- WidyaActor (Actor Model Concurrency System) Implementation ---

struct ActorInstance {
    mailbox_tx: mpsc::Sender<ValueSerial>,
    response_rx: mpsc::Receiver<ValueSerial>,
}

fn get_actor_pool() -> &'static Mutex<HashMap<String, ActorInstance>> {
    static ACTOR_POOL: OnceLock<Mutex<HashMap<String, ActorInstance>>> = OnceLock::new();
    ACTOR_POOL.get_or_init(|| Mutex::new(HashMap::new()))
}

fn builtin_aktor_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama = args[0].to_string_repr();
    let state_awal = ValueSerial::from_value(&args[1]);

    let (inbox_tx, inbox_rx) = mpsc::channel::<ValueSerial>();
    let (outbox_tx, outbox_rx) = mpsc::channel::<ValueSerial>();

    let mut pool = get_actor_pool().lock().unwrap();
    pool.insert(nama.clone(), ActorInstance {
        mailbox_tx: inbox_tx,
        response_rx: outbox_rx,
    });

    // Spawn isolated actor worker thread
    let actor_name = nama.clone();
    std::thread::spawn(move || {
        let mut internal_state = state_awal;
        while let Ok(msg) = inbox_rx.recv() {
            // Process message and return acknowledgment / next state
            let reply = match (&internal_state, &msg) {
                (ValueSerial::Number(s), ValueSerial::Number(m)) => ValueSerial::Number(s + m),
                _ => ValueSerial::String(format!("Aktor '{}' memproses pesan: {:?}", actor_name, msg)),
            };
            internal_state = reply.clone();
            let _ = outbox_tx.send(reply);
        }
    });

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("Aktor".to_string()));
    map.insert("nama".to_string(), Value::String(nama));
    map.insert("status".to_string(), Value::String("AKTIF".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_aktor_kirim(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama = match &args[0] {
        Value::Map(m) => m.borrow().get("nama").map(|v| v.to_string_repr()).unwrap_or_default(),
        _ => args[0].to_string_repr(),
    };
    let pesan = ValueSerial::from_value(&args[1]);

    let pool = get_actor_pool().lock().unwrap();
    if let Some(actor) = pool.get(&nama) {
        actor.mailbox_tx.send(pesan)
            .map_err(|e| Galat::runtime(format!("Gagal mengirim pesan ke aktor: {}", e), span))?;
        Ok(Value::Bool(true))
    } else {
        Err(Galat::runtime(format!("Aktor '{}' tidak ditemukan", nama), span))
    }
}

fn builtin_aktor_tunggu_respon(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nama = match &args[0] {
        Value::Map(m) => m.borrow().get("nama").map(|v| v.to_string_repr()).unwrap_or_default(),
        _ => args[0].to_string_repr(),
    };

    let pool = get_actor_pool().lock().unwrap();
    if let Some(actor) = pool.get(&nama) {
        match actor.response_rx.recv_timeout(Duration::from_millis(500)) {
            Ok(val) => Ok(val.to_value()),
            Err(e) => Err(Galat::runtime(format!("Batas waktu respon aktor terlampaui: {}", e), span)),
        }
    } else {
        Err(Galat::runtime(format!("Aktor '{}' tidak ditemukan", nama), span))
    }
}

// --- WidyaRaft (Distributed Consensus Engine) ---

fn builtin_raft_node(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let node_id = args[0].to_string_repr();
    let peers = match &args[1] {
        Value::Array(a) => a.borrow().clone(),
        _ => Vec::new(),
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("NodeRaft".to_string()));
    map.insert("id".to_string(), Value::String(node_id));
    map.insert("peran".to_string(), Value::String("PENGIKUT".to_string()));
    map.insert("istilah_saat_ini".to_string(), Value::Number(1.0));
    map.insert("log".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("peers".to_string(), Value::Array(Rc::new(RefCell::new(peers))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_raft_pilih(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            map.insert("peran".to_string(), Value::String("PEMIMPIN".to_string()));
            if let Some(Value::Number(t)) = map.get("istilah_saat_ini") {
                let next_t = *t + 1.0;
                map.insert("istilah_saat_ini".to_string(), Value::Number(next_t));
            }
            Ok(Value::String("PEMIMPIN".to_string()))
        }
        _ => Err(Galat::runtime("Diharapkan objek NodeRaft", span)),
    }
}

fn builtin_raft_usul(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let entri = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            if let Some(Value::Array(log_arr)) = map.get_mut("log") {
                log_arr.borrow_mut().push(Value::String(entri));
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("Log Raft tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek NodeRaft", span)),
    }
}

fn builtin_raft_status(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let peran = map.get("peran").map(|v| v.to_string_repr()).unwrap_or_default();
            let istilah = map.get("istilah_saat_ini").map(|v| v.to_string_repr()).unwrap_or_default();
            let log_len = match map.get("log") {
                Some(Value::Array(a)) => a.borrow().len(),
                _ => 0,
            };

            let mut res = HashMap::new();
            res.insert("peran".to_string(), Value::String(peran));
            res.insert("istilah".to_string(), Value::String(istilah));
            res.insert("jumlah_log".to_string(), Value::Number(log_len as f64));
            res.insert("konsensus".to_string(), Value::Bool(true));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Diharapkan objek NodeRaft", span)),
    }
}

// =========================================================================
// WIDYAACTORSYSTEM (SUPERVISED ACTOR CONCURRENCY & MAILBOX)
// =========================================================================

fn builtin_actor_system_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let name = args.first().map(|v| v.to_string_repr()).unwrap_or_else(|| "SistemAktorUtama".to_string());
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("SistemAktor".to_string()));
    map.insert("nama".to_string(), Value::String(name));
    map.insert("aktor".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_actor_spawn(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let system_val = &args[0];
    let actor_name = args[1].to_string_repr();
    let handler = args[2].clone();

    match system_val {
        Value::Map(sm) => {
            let sys_map = sm.borrow();
            if let Some(Value::Map(actors_map)) = sys_map.get("aktor") {
                let mut actor_obj = HashMap::new();
                actor_obj.insert("_tipe".to_string(), Value::String("AktorRef".to_string()));
                actor_obj.insert("nama".to_string(), Value::String(actor_name.clone()));
                actor_obj.insert("status".to_string(), Value::String("AKTIF".to_string()));
                actor_obj.insert("penangan".to_string(), handler);
                actor_obj.insert("mailbox".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
                actor_obj.insert("pesan_diproses".to_string(), Value::Number(0.0));

                let actor_val = Value::Map(Rc::new(RefCell::new(actor_obj)));
                actors_map.borrow_mut().insert(actor_name, actor_val.clone());
                return Ok(actor_val);
            }
            Err(Galat::runtime("Objek SistemAktor tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'aktor_spawn' memerlukan objek SistemAktor", span)),
    }
}

fn builtin_actor_kirim(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let actor_val = &args[0];
    let msg = args[1].clone();

    match actor_val {
        Value::Map(m) => {
            let (handler, actor_ref_clone) = {
                let mut actor_map = m.borrow_mut();
                if let Some(Value::Array(mb)) = actor_map.get("mailbox") {
                    mb.borrow_mut().push(msg.clone());
                }
                let count = match actor_map.get("pesan_diproses") {
                    Some(Value::Number(n)) => *n + 1.0,
                    _ => 1.0,
                };
                actor_map.insert("pesan_diproses".to_string(), Value::Number(count));
                (actor_map.get("penangan").cloned(), actor_val.clone())
            };

            if let Some(h) = handler {
                let _ = invoke_closure_or_fn(&h, &[actor_ref_clone, msg], span);
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'aktor_kirim' memerlukan referensi Aktor", span)),
    }
}

fn builtin_actor_status(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            Ok(map.get("status").cloned().unwrap_or(Value::String("TIDAK_DIKETAHUI".to_string())))
        }
        _ => Err(Galat::runtime("Fungsi 'aktor_status' memerlukan referensi Aktor", span)),
    }
}

fn builtin_actor_mailbox_panjang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(mb)) = map.get("mailbox") {
                Ok(Value::Number(mb.borrow().len() as f64))
            } else {
                Ok(Value::Number(0.0))
            }
        }
        _ => Err(Galat::runtime("Fungsi 'aktor_mailbox_panjang' memerlukan referensi Aktor", span)),
    }
}

// =========================================================================
// WIDYARAFTLITE (DISTRIBUTED RAFT CONSENSUS ENGINE)
// =========================================================================

fn builtin_raft_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nodes = match &args[0] {
        Value::Array(a) => {
            let items = a.borrow();
            items.iter().map(|v| v.to_string_repr()).collect::<Vec<String>>()
        }
        _ => return Err(Galat::runtime("Fungsi 'KlasterRaft' memerlukan array ID node", span)),
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KlasterRaft".to_string()));
    map.insert("term".to_string(), Value::Number(1.0));
    let leader = nodes.first().cloned().unwrap_or_else(|| "node_1".to_string());
    map.insert("leader".to_string(), Value::String(leader));
    
    let mut node_states = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        let mut st = HashMap::new();
        st.insert("node_id".to_string(), Value::String(node.clone()));
        st.insert("peran".to_string(), Value::String(if i == 0 { "Leader".to_string() } else { "Follower".to_string() }));
        st.insert("term".to_string(), Value::Number(1.0));
        st.insert("commit_index".to_string(), Value::Number(0.0));
        st.insert("log".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
        node_states.insert(node.clone(), Value::Map(Rc::new(RefCell::new(st))));
    }
    map.insert("nodes".to_string(), Value::Map(Rc::new(RefCell::new(node_states))));
    map.insert("log_komitmen".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_raft_usulkan_perintah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cluster_val = &args[0];
    let cmd_val = &args[1];

    match cluster_val {
        Value::Map(m) => {
            let cluster = m.borrow_mut();
            let leader_id = cluster.get("leader").map(|v| v.to_string_repr()).unwrap_or_default();
            let term = cluster.get("term").cloned().unwrap_or(Value::Number(1.0));

            let mut log_entry = HashMap::new();
            log_entry.insert("term".to_string(), term);
            log_entry.insert("perintah".to_string(), cmd_val.clone());
            log_entry.insert("waktu".to_string(), Value::Number(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64()));

            let entry_val = Value::Map(Rc::new(RefCell::new(log_entry)));

            // Replicate across majority
            if let Some(Value::Map(nodes_map)) = cluster.get("nodes") {
                for (_, n_val) in nodes_map.borrow().iter() {
                    if let Value::Map(nm) = n_val {
                        if let Some(Value::Array(log_arr)) = nm.borrow().get("log") {
                            log_arr.borrow_mut().push(entry_val.clone());
                        }
                    }
                }
            }

            if let Some(Value::Array(commit_log)) = cluster.get("log_komitmen") {
                commit_log.borrow_mut().push(entry_val);
            }

            let mut res = HashMap::new();
            res.insert("sukses".to_string(), Value::Bool(true));
            res.insert("leader".to_string(), Value::String(leader_id));
            res.insert("index_komitmen".to_string(), Value::Number(
                cluster.get("log_komitmen").and_then(|v| match v {
                    Value::Array(a) => Some(a.borrow().len() as f64),
                    _ => None,
                }).unwrap_or(1.0)
            ));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'raft_usulkan_perintah' memerlukan objek KlasterRaft", span)),
    }
}

fn builtin_raft_detak_pemilihan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut curr_term = {
                let cluster = m.borrow();
                match cluster.get("term") {
                    Some(Value::Number(n)) => *n,
                    _ => 1.0,
                }
            };
            curr_term += 1.0;

            let (next_leader, nodes_opt) = {
                let cluster = m.borrow();
                let leader_choice = if let Some(Value::Map(nodes_map)) = cluster.get("nodes") {
                    let n_map = nodes_map.borrow();
                    let keys: Vec<String> = n_map.keys().cloned().collect();
                    if !keys.is_empty() {
                        let next_leader_idx = (curr_term as usize) % keys.len();
                        Some((keys[next_leader_idx].clone(), nodes_map.clone()))
                    } else {
                        None
                    }
                } else {
                    None
                };
                match leader_choice {
                    Some((l, nm)) => (Some(l), Some(nm)),
                    None => (None, None),
                }
            };

            if let Some(leader) = next_leader {
                m.borrow_mut().insert("term".to_string(), Value::Number(curr_term));
                m.borrow_mut().insert("leader".to_string(), Value::String(leader.clone()));

                if let Some(nodes_map) = nodes_opt {
                    let n_map = nodes_map.borrow();
                    for (k, v) in n_map.iter() {
                        if let Value::Map(node_cell) = v {
                            let mut node_st = node_cell.borrow_mut();
                            node_st.insert("term".to_string(), Value::Number(curr_term));
                            if k == &leader {
                                node_st.insert("peran".to_string(), Value::String("Leader".to_string()));
                            } else {
                                node_st.insert("peran".to_string(), Value::String("Follower".to_string()));
                            }
                        }
                    }
                }
                return Ok(Value::String(leader));
            }

            Ok(Value::Nil)
        }
        _ => Err(Galat::runtime("Fungsi 'raft_detak_pemilihan' memerlukan objek KlasterRaft", span)),
    }
}

fn builtin_raft_dapatkan_leader(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let cluster = m.borrow();
            Ok(cluster.get("leader").cloned().unwrap_or(Value::Nil))
        }
        _ => Err(Galat::runtime("Fungsi 'raft_dapatkan_leader' memerlukan objek KlasterRaft", span)),
    }
}

fn builtin_raft_klaster_status(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let cluster = m.borrow();
            let mut res = HashMap::new();
            res.insert("term".to_string(), cluster.get("term").cloned().unwrap_or(Value::Number(1.0)));
            res.insert("leader".to_string(), cluster.get("leader").cloned().unwrap_or(Value::Nil));
            res.insert("total_komitmen".to_string(), Value::Number(
                cluster.get("log_komitmen").and_then(|v| match v {
                    Value::Array(a) => Some(a.borrow().len() as f64),
                    _ => None,
                }).unwrap_or(0.0)
            ));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'raft_klaster_status' memerlukan objek KlasterRaft", span)),
    }
}

// =========================================================================
// WIDYAGOSSIP (EPIDEMIC / P2P GOSSIP DISSEMINATION PROTOCOL)
// =========================================================================

fn builtin_gossip_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let nodes = match &args[0] {
        Value::Array(a) => {
            let items = a.borrow();
            items.iter().map(|v| v.to_string_repr()).collect::<Vec<String>>()
        }
        _ => return Err(Galat::runtime("Fungsi 'KlasterGossip' memerlukan array ID node", span)),
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KlasterGossip".to_string()));
    
    let mut node_states = HashMap::new();
    for node in nodes {
        let mut n_map = HashMap::new();
        n_map.insert("node_id".to_string(), Value::String(node.clone()));
        n_map.insert("heartbeat".to_string(), Value::Number(1.0));
        n_map.insert("pesan_diterima".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
        n_map.insert("status".to_string(), Value::String("HIDUP".to_string()));
        node_states.insert(node, Value::Map(Rc::new(RefCell::new(n_map))));
    }
    map.insert("nodes".to_string(), Value::Map(Rc::new(RefCell::new(node_states))));
    map.insert("antrian_gossip".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_gossip_siarkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cluster_val = &args[0];
    let source_node = args[1].to_string_repr();
    let message_val = &args[2];

    match cluster_val {
        Value::Map(m) => {
            let cluster = m.borrow();
            let mut payload = HashMap::new();
            payload.insert("asal".to_string(), Value::String(source_node.clone()));
            payload.insert("konten".to_string(), message_val.clone());
            payload.insert("waktu".to_string(), Value::Number(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64()));
            let payload_val = Value::Map(Rc::new(RefCell::new(payload)));

            if let Some(Value::Array(queue)) = cluster.get("antrian_gossip") {
                queue.borrow_mut().push(payload_val.clone());
            }

            // Immediately deliver to source node
            if let Some(Value::Map(nodes_map)) = cluster.get("nodes") {
                if let Some(Value::Map(source_node_map)) = nodes_map.borrow().get(&source_node) {
                    if let Some(Value::Array(msg_arr)) = source_node_map.borrow().get("pesan_diterima") {
                        msg_arr.borrow_mut().push(payload_val);
                    }
                }
            }

            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi 'gossip_siarkan' memerlukan objek KlasterGossip", span)),
    }
}

fn builtin_gossip_detak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let cluster = m.borrow();
            let mut delivered = 0;

            if let (Some(Value::Array(queue)), Some(Value::Map(nodes_map))) = (cluster.get("antrian_gossip"), cluster.get("nodes")) {
                let mut q = queue.borrow_mut();
                let n_map = nodes_map.borrow();

                for item in q.drain(..) {
                    for (_, node_cell) in n_map.iter() {
                        if let Value::Map(nm) = node_cell {
                            let mut node_st = nm.borrow_mut();
                            let hb = match node_st.get("heartbeat") {
                                Some(Value::Number(n)) => *n + 1.0,
                                _ => 1.0,
                            };
                            node_st.insert("heartbeat".to_string(), Value::Number(hb));

                            if let Some(Value::Array(msgs)) = node_st.get("pesan_diterima") {
                                msgs.borrow_mut().push(item.clone());
                                delivered += 1;
                            }
                        }
                    }
                }
            }

            Ok(Value::Number(delivered as f64))
        }
        _ => Err(Galat::runtime("Fungsi 'gossip_detak' memerlukan objek KlasterGossip", span)),
    }
}

fn builtin_gossip_keadaan_node(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let cluster_val = &args[0];
    let node_id = args[1].to_string_repr();

    match cluster_val {
        Value::Map(m) => {
            let cluster = m.borrow();
            if let Some(Value::Map(nodes_map)) = cluster.get("nodes") {
                if let Some(node_val) = nodes_map.borrow().get(&node_id) {
                    return Ok(node_val.clone());
                }
            }
            Ok(Value::Nil)
        }
        _ => Err(Galat::runtime("Fungsi 'gossip_keadaan_node' memerlukan objek KlasterGossip", span)),
    }
}

// =========================================================================
// WIDYA_HLC (HYBRID LOGICAL CLOCK & CAUSALITY TRACKING)
// =========================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
struct HlcTimestamp {
    l: u64, // physical logical time
    c: u64, // counter / logical progression
    node_id: String,
}

impl PartialOrd for HlcTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HlcTimestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.l.cmp(&other.l) {
            std::cmp::Ordering::Equal => match self.c.cmp(&other.c) {
                std::cmp::Ordering::Equal => self.node_id.cmp(&other.node_id),
                other_ord => other_ord,
            },
            other_ord => other_ord,
        }
    }
}

fn get_physical_now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_millis(0))
        .as_millis() as u64
}

fn hlc_to_dict(ts: &HlcTimestamp) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    map.insert("l".to_string(), Value::Number(ts.l as f64));
    map.insert("c".to_string(), Value::Number(ts.c as f64));
    map.insert("node_id".to_string(), Value::String(ts.node_id.clone()));
    map
}

fn dict_to_hlc(val: &Value, span: &Span) -> Result<HlcTimestamp, Galat> {
    match val {
        Value::Map(dict) => {
            let map = dict.borrow();
            let l = match map.get("l") {
                Some(Value::Number(n)) => *n as u64,
                _ => return Err(Galat::runtime("Timestamp HLC memerlukan atribut 'l' berupa angka", span)),
            };
            let c = match map.get("c") {
                Some(Value::Number(n)) => *n as u64,
                _ => return Err(Galat::runtime("Timestamp HLC memerlukan atribut 'c' berupa angka", span)),
            };
            let node_id = match map.get("node_id") {
                Some(Value::String(s)) => s.clone(),
                _ => "node".to_string(),
            };
            Ok(HlcTimestamp { l, c, node_id })
        }
        _ => Err(Galat::runtime("Argumen harus berupa kamus Timestamp HLC", span)),
    }
}

fn builtin_hlc_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let node_id = if !args.is_empty() {
        match &args[0] {
            Value::String(s) => s.clone(),
            _ => "node-1".to_string(),
        }
    } else {
        "node-1".to_string()
    };

    let now = get_physical_now_ms();
    let init_ts = HlcTimestamp {
        l: now,
        c: 0,
        node_id: node_id.clone(),
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaHLC".to_string()));
    map.insert("node_id".to_string(), Value::String(node_id));
    map.insert("l".to_string(), Value::Number(init_ts.l as f64));
    map.insert("c".to_string(), Value::Number(init_ts.c as f64));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_hlc_detak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(dict) => {
            let mut map = dict.borrow_mut();
            let node_id = match map.get("node_id") {
                Some(Value::String(s)) => s.clone(),
                _ => "node".to_string(),
            };
            let l_old = match map.get("l") {
                Some(Value::Number(n)) => *n as u64,
                _ => 0,
            };
            let c_old = match map.get("c") {
                Some(Value::Number(n)) => *n as u64,
                _ => 0,
            };

            let pt = get_physical_now_ms();
            let l_new = l_old.max(pt);
            let c_new = if l_new == l_old { c_old + 1 } else { 0 };

            map.insert("l".to_string(), Value::Number(l_new as f64));
            map.insert("c".to_string(), Value::Number(c_new as f64));

            let ts = HlcTimestamp {
                l: l_new,
                c: c_new,
                node_id,
            };
            Ok(Value::Map(Rc::new(RefCell::new(hlc_to_dict(&ts)))))
        }
        _ => Err(Galat::runtime("Fungsi 'hlc_detak' memerlukan instance JamHLC", span)),
    }
}

fn builtin_hlc_perbarui(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let msg_ts = dict_to_hlc(&args[1], span)?;
    match &args[0] {
        Value::Map(dict) => {
            let mut map = dict.borrow_mut();
            let node_id = match map.get("node_id") {
                Some(Value::String(s)) => s.clone(),
                _ => "node".to_string(),
            };
            let l_old = match map.get("l") {
                Some(Value::Number(n)) => *n as u64,
                _ => 0,
            };
            let c_old = match map.get("c") {
                Some(Value::Number(n)) => *n as u64,
                _ => 0,
            };

            let pt = get_physical_now_ms();
            let l_new = l_old.max(msg_ts.l).max(pt);
            let c_new = if l_new == l_old && l_new == msg_ts.l {
                c_old.max(msg_ts.c) + 1
            } else if l_new == l_old {
                c_old + 1
            } else if l_new == msg_ts.l {
                msg_ts.c + 1
            } else {
                0
            };

            map.insert("l".to_string(), Value::Number(l_new as f64));
            map.insert("c".to_string(), Value::Number(c_new as f64));

            let ts = HlcTimestamp {
                l: l_new,
                c: c_new,
                node_id,
            };
            Ok(Value::Map(Rc::new(RefCell::new(hlc_to_dict(&ts)))))
        }
        _ => Err(Galat::runtime("Fungsi 'hlc_perbarui' memerlukan instance JamHLC", span)),
    }
}

fn builtin_hlc_apakah_sebelum(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ts1 = dict_to_hlc(&args[0], span)?;
    let ts2 = dict_to_hlc(&args[1], span)?;
    Ok(Value::Bool(ts1 < ts2))
}

fn builtin_hlc_format(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let ts = dict_to_hlc(&args[0], span)?;
    Ok(Value::String(format!("(l:{}, c:{}, node:{})", ts.l, ts.c, ts.node_id)))
}

// =========================================================================
// WIDYA_KADEMLIA (K-BUCKET ROUTING & XOR METRIC P2P DHT)
// =========================================================================

fn kademlia_hash_id(s: &str) -> [u8; 20] {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    let res = hasher.finalize();
    let mut out = [0u8; 20];
    out.copy_from_slice(&res[..20]);
    out
}

fn kademlia_xor_distance(a: &[u8; 20], b: &[u8; 20]) -> [u8; 20] {
    let mut dist = [0u8; 20];
    for i in 0..20 {
        dist[i] = a[i] ^ b[i];
    }
    dist
}

fn builtin_kademlia_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let local_id_str = if !args.is_empty() {
        args[0].to_string_repr()
    } else {
        "node-local-01".to_string()
    };

    let k_bucket_size = if args.len() > 1 {
        match &args[1] {
            Value::Number(n) => (*n as usize).max(1),
            _ => 20,
        }
    } else {
        20
    };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaKademlia".to_string()));
    map.insert("local_id".to_string(), Value::String(local_id_str));
    map.insert("k".to_string(), Value::Number(k_bucket_size as f64));
    map.insert("nodes".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("storage".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_kademlia_tambah_node(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let node_id_str = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(nodes_arr)) = map.get("nodes") {
                let mut nodes = nodes_arr.borrow_mut();
                // Check if already exists
                let mut found = false;
                for item in nodes.iter_mut() {
                    if let Value::Map(im) = item {
                        let im_map = im.borrow();
                        if let Some(Value::String(id)) = im_map.get("id") {
                            if id == &node_id_str {
                                found = true;
                                break;
                            }
                        }
                    }
                }
                if !found {
                    let mut node_obj = HashMap::new();
                    node_obj.insert("id".to_string(), Value::String(node_id_str.clone()));
                    node_obj.insert("address".to_string(), args.get(2).cloned().unwrap_or(Value::String("127.0.0.1".to_string())));
                    nodes.push(Value::Map(Rc::new(RefCell::new(node_obj))));
                }
                return Ok(Value::Bool(true));
            }
            Err(Galat::runtime("Objek Kademlia tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'kademlia_tambah_node' memerlukan instance TabelKademlia", span)),
    }
}

fn builtin_kademlia_node_terdekat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let target_key = args[1].to_string_repr();
    let count = if args.len() > 2 {
        match &args[2] { Value::Number(n) => *n as usize, _ => 5 }
    } else {
        5
    };

    let target_hash = kademlia_hash_id(&target_key);

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Array(nodes_arr)) = map.get("nodes") {
                let nodes = nodes_arr.borrow();
                let mut scored: Vec<([u8; 20], Value)> = Vec::new();

                for item in nodes.iter() {
                    if let Value::Map(im) = item {
                        let im_map = im.borrow();
                        if let Some(Value::String(id)) = im_map.get("id") {
                            let nh = kademlia_hash_id(id);
                            let dist = kademlia_xor_distance(&nh, &target_hash);
                            scored.push((dist, item.clone()));
                        }
                    }
                }

                scored.sort_by(|a, b| a.0.cmp(&b.0));
                let res: Vec<Value> = scored.into_iter().take(count).map(|(_, v)| v).collect();
                return Ok(Value::Array(Rc::new(RefCell::new(res))));
            }
            Err(Galat::runtime("Objek Kademlia tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'kademlia_node_terdekat' memerlukan instance TabelKademlia", span)),
    }
}

fn builtin_kademlia_simpan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = args[1].to_string_repr();
    let val = args[2].clone();

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(sm)) = map.get("storage") {
                sm.borrow_mut().insert(key, val);
                return Ok(Value::Bool(true));
            }
            Err(Galat::runtime("Objek Kademlia tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'kademlia_simpan' memerlukan instance TabelKademlia", span)),
    }
}

fn builtin_kademlia_ambil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let key = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(sm)) = map.get("storage") {
                let s_borrow = sm.borrow();
                if let Some(v) = s_borrow.get(&key) {
                    return Ok(v.clone());
                }
                return Ok(Value::Nil);
            }
            Err(Galat::runtime("Objek Kademlia tidak valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'kademlia_ambil' memerlukan instance TabelKademlia", span)),
    }
}

// =========================================================================
// WIDYA_BULLY_ELECTION (DISTRIBUTED BULLY LEADER ELECTION PROTOCOL)
// =========================================================================

fn builtin_bully_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let node_ids = match &args[0] {
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().map(|v| v.to_string_repr()).collect::<Vec<String>>()
        }
        _ => return Err(Galat::runtime("Argumen pertama harus berupa daftar ID node", span)),
    };

    let mut nodes_map = HashMap::new();
    let mut highest_id = 0i64;
    let mut initial_leader = "".to_string();

    for (idx, id) in node_ids.iter().enumerate() {
        let rank = (idx + 1) as i64;
        let mut n_info = HashMap::new();
        n_info.insert("id".to_string(), Value::String(id.clone()));
        n_info.insert("rank".to_string(), Value::Number(rank as f64));
        n_info.insert("alive".to_string(), Value::Bool(true));
        nodes_map.insert(id.clone(), Value::Map(Rc::new(RefCell::new(n_info))));

        if rank > highest_id {
            highest_id = rank;
            initial_leader = id.clone();
        }
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaBullyElection".to_string()));
    map.insert("leader".to_string(), Value::String(initial_leader));
    map.insert("nodes".to_string(), Value::Map(Rc::new(RefCell::new(nodes_map))));
    map.insert("election_in_progress".to_string(), Value::Bool(false));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_bully_mulai_pemilihan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let initiator_id = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let mut highest_alive_rank = 0i64;
            let mut new_leader = initiator_id.clone();

            if let Some(Value::Map(nm)) = map.get("nodes") {
                let nodes = nm.borrow();
                for (id, n_val) in nodes.iter() {
                    if let Value::Map(info) = n_val {
                        let i_map = info.borrow();
                        let alive = match i_map.get("alive") { Some(Value::Bool(b)) => *b, _ => false };
                        let rank = match i_map.get("rank") { Some(Value::Number(n)) => *n as i64, _ => 0 };
                        if alive && rank > highest_alive_rank {
                            highest_alive_rank = rank;
                            new_leader = id.clone();
                        }
                    }
                }
            }

            map.insert("leader".to_string(), Value::String(new_leader.clone()));
            map.insert("election_in_progress".to_string(), Value::Bool(false));
            Ok(Value::String(new_leader))
        }
        _ => Err(Galat::runtime("Fungsi 'bully_mulai_pemilihan' memerlukan instance KlasterBully", span)),
    }
}

fn builtin_bully_detak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            let current_leader = map.get("leader").map(|v| v.to_string_repr()).unwrap_or_default();
            let mut leader_alive = false;

            if let Some(Value::Map(nm)) = map.get("nodes") {
                let nodes = nm.borrow();
                if let Some(Value::Map(info)) = nodes.get(&current_leader) {
                    if let Some(Value::Bool(true)) = info.borrow().get("alive") {
                        leader_alive = true;
                    }
                }
            }

            if !leader_alive {
                // Trigger auto-election from highest remaining
                let mut highest_rank = 0i64;
                let mut elected = "".to_string();
                if let Some(Value::Map(nm)) = map.get("nodes") {
                    let nodes = nm.borrow();
                    for (id, n_val) in nodes.iter() {
                        if let Value::Map(info) = n_val {
                            let i_map = info.borrow();
                            let alive = match i_map.get("alive") { Some(Value::Bool(b)) => *b, _ => false };
                            let rank = match i_map.get("rank") { Some(Value::Number(n)) => *n as i64, _ => 0 };
                            if alive && rank > highest_rank {
                                highest_rank = rank;
                                elected = id.clone();
                            }
                        }
                    }
                }
                map.insert("leader".to_string(), Value::String(elected.clone()));
                return Ok(Value::String(elected));
            }

            Ok(Value::String(current_leader))
        }
        _ => Err(Galat::runtime("Fungsi 'bully_detak' memerlukan instance KlasterBully", span)),
    }
}

fn builtin_bully_matikan_node(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let target_id = args[1].to_string_repr();
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(Value::Map(nm)) = map.get("nodes") {
                let nodes = nm.borrow();
                if let Some(Value::Map(info)) = nodes.get(&target_id) {
                    info.borrow_mut().insert("alive".to_string(), Value::Bool(false));
                    return Ok(Value::Bool(true));
                }
            }
            Err(Galat::runtime("Node tidak ditemukan pada klaster", span))
        }
        _ => Err(Galat::runtime("Fungsi 'bully_matikan_node' memerlukan instance KlasterBully", span)),
    }
}

fn builtin_bully_dapatkan_leader(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(l) = map.get("leader") {
                return Ok(l.clone());
            }
            Ok(Value::Nil)
        }
        _ => Err(Galat::runtime("Fungsi 'bully_dapatkan_leader' memerlukan instance KlasterBully", span)),
    }
}

// =========================================================================
// WIDYA_PAXOS (CLASSIC DISTRIBUTED PAXOS CONSENSUS PROTOCOL)
// =========================================================================

fn builtin_paxos_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let node_ids = match &args[0] {
        Value::Array(arr) => {
            let items = arr.borrow();
            items.iter().map(|v| v.to_string_repr()).collect::<Vec<String>>()
        }
        _ => return Err(Galat::runtime("Argumen pertama harus berupa daftar ID node", span)),
    };

    let mut acceptors = HashMap::new();
    for id in &node_ids {
        let mut acc = HashMap::new();
        acc.insert("promised_n".to_string(), Value::Number(0.0));
        acc.insert("accepted_n".to_string(), Value::Number(0.0));
        acc.insert("accepted_val".to_string(), Value::Nil);
        acceptors.insert(id.clone(), Value::Map(Rc::new(RefCell::new(acc))));
    }

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("WidyaPaxos".to_string()));
    map.insert("acceptors".to_string(), Value::Map(Rc::new(RefCell::new(acceptors))));
    map.insert("decided_value".to_string(), Value::Nil);
    map.insert("total_nodes".to_string(), Value::Number(node_ids.len() as f64));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_paxos_siapkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let proposal_n = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("proposal_n harus angka", span)) };
    let proposer_id = args[2].to_string_repr();

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let mut promises_count = 0usize;
            let mut highest_accepted_n = 0.0;
            let mut highest_accepted_val = Value::Nil;

            if let Some(Value::Map(acc_map)) = map.get("acceptors") {
                let accs = acc_map.borrow();
                for (_id, acc_val) in accs.iter() {
                    if let Value::Map(im) = acc_val {
                        let mut acc_info = im.borrow_mut();
                        let promised_n = match acc_info.get("promised_n") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        if proposal_n > promised_n {
                            acc_info.insert("promised_n".to_string(), Value::Number(proposal_n));
                            promises_count += 1;
                            let accepted_n = match acc_info.get("accepted_n") { Some(Value::Number(n)) => *n, _ => 0.0 };
                            if accepted_n > highest_accepted_n {
                                highest_accepted_n = accepted_n;
                                highest_accepted_val = acc_info.get("accepted_val").cloned().unwrap_or(Value::Nil);
                            }
                        }
                    }
                }
            }

            let total_nodes = match map.get("total_nodes") { Some(Value::Number(n)) => *n as usize, _ => 1 };
            let quorum = (total_nodes / 2) + 1;
            let ok = promises_count >= quorum;

            let mut res = HashMap::new();
            res.insert("ok".to_string(), Value::Bool(ok));
            res.insert("promises".to_string(), Value::Number(promises_count as f64));
            res.insert("quorum".to_string(), Value::Number(quorum as f64));
            res.insert("highest_val".to_string(), highest_accepted_val);
            res.insert("proposer".to_string(), Value::String(proposer_id));

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'paxos_siapkan' memerlukan instance KlasterPaxos", span)),
    }
}

fn builtin_paxos_usulkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let proposal_n = match &args[1] { Value::Number(n) => *n, _ => return Err(Galat::runtime("proposal_n harus angka", span)) };
    let proposed_val = args[2].clone();
    let _proposer_id = args[3].to_string_repr();

    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            let mut accepts_count = 0usize;

            if let Some(Value::Map(acc_map)) = map.get("acceptors") {
                let accs = acc_map.borrow();
                for (_id, acc_val) in accs.iter() {
                    if let Value::Map(im) = acc_val {
                        let mut acc_info = im.borrow_mut();
                        let promised_n = match acc_info.get("promised_n") { Some(Value::Number(n)) => *n, _ => 0.0 };
                        if proposal_n >= promised_n {
                            acc_info.insert("promised_n".to_string(), Value::Number(proposal_n));
                            acc_info.insert("accepted_n".to_string(), Value::Number(proposal_n));
                            acc_info.insert("accepted_val".to_string(), proposed_val.clone());
                            accepts_count += 1;
                        }
                    }
                }
            }

            let total_nodes = match map.get("total_nodes") { Some(Value::Number(n)) => *n as usize, _ => 1 };
            let quorum = (total_nodes / 2) + 1;
            let ok = accepts_count >= quorum;

            let mut res = HashMap::new();
            res.insert("ok".to_string(), Value::Bool(ok));
            res.insert("accepts".to_string(), Value::Number(accepts_count as f64));
            res.insert("quorum".to_string(), Value::Number(quorum as f64));
            res.insert("value".to_string(), proposed_val);

            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'paxos_usulkan' memerlukan instance KlasterPaxos", span)),
    }
}

fn builtin_paxos_putuskan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let decided_val = args[1].clone();
    match &args[0] {
        Value::Map(m) => {
            let mut map = m.borrow_mut();
            map.insert("decided_value".to_string(), decided_val.clone());
            Ok(decided_val)
        }
        _ => Err(Galat::runtime("Fungsi 'paxos_putuskan' memerlukan instance KlasterPaxos", span)),
    }
}

fn builtin_paxos_nilai_terpilih(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if let Some(v) = map.get("decided_value") {
                return Ok(v.clone());
            }
            Ok(Value::Nil)
        }
        _ => Err(Galat::runtime("Fungsi 'paxos_nilai_terpilih' memerlukan instance KlasterPaxos", span)),
    }
}

// ============================================================================
// WidyaVectorClock (Multi-Node Vector Clocks for Distributed Event Causality)
// ============================================================================

#[derive(Debug, Clone)]
pub struct WidyaVectorClock {
    pub node_id: String,
    pub clock: std::collections::BTreeMap<String, u64>,
}

fn value_to_vclock(val: &Value) -> Option<WidyaVectorClock> {
    if let Value::Map(obj) = val {
        let borrowed = obj.borrow();
        if borrowed.get("_tipe").and_then(|v| match v { Value::String(s) => Some(s.as_str()), _ => None }) == Some("JamVektor") {
            let node_id = match borrowed.get("node_id") {
                Some(Value::String(s)) => s.clone(),
                _ => "".to_string(),
            };
            let mut clock = std::collections::BTreeMap::new();
            if let Some(Value::Map(c_obj)) = borrowed.get("clock") {
                for (k, v) in c_obj.borrow().iter() {
                    if let Value::Number(n) = v {
                        clock.insert(k.clone(), *n as u64);
                    }
                }
            }
            return Some(WidyaVectorClock { node_id, clock });
        }
    }
    None
}

fn vclock_to_value(vc: &WidyaVectorClock) -> Value {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("JamVektor".to_string()));
    map.insert("node_id".to_string(), Value::String(vc.node_id.clone()));
    
    let mut c_map = HashMap::new();
    for (k, v) in &vc.clock {
        c_map.insert(k.clone(), Value::Number(*v as f64));
    }
    map.insert("clock".to_string(), Value::Map(Rc::new(RefCell::new(c_map))));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn builtin_vclock_buat(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let node_id = if !args.is_empty() {
        match &args[0] {
            Value::String(s) => s.clone(),
            _ => return Err(Galat::runtime("Parameter 1 node_id harus string", span)),
        }
    } else {
        "node_1".to_string()
    };

    let mut clock = std::collections::BTreeMap::new();
    clock.insert(node_id.clone(), 0);

    if args.len() > 1 {
        if let Value::Array(nodes) = &args[1] {
            for n in nodes.borrow().iter() {
                if let Value::String(name) = n {
                    clock.insert(name.clone(), 0);
                }
            }
        }
    }

    Ok(vclock_to_value(&WidyaVectorClock { node_id, clock }))
}

fn builtin_vclock_detak(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(mut vc) = value_to_vclock(&args[0]) {
        let current = vc.clock.entry(vc.node_id.clone()).or_insert(0);
        *current += 1;
        return Ok(vclock_to_value(&vc));
    }
    Err(Galat::runtime("Fungsi 'vclock_detak' memerlukan instance JamVektor", span))
}

fn builtin_vclock_gabung(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let vca = match value_to_vclock(&args[0]) {
        Some(v) => v,
        None => return Err(Galat::runtime("Parameter 1 harus instance JamVektor", span)),
    };
    let vcb = match value_to_vclock(&args[1]) {
        Some(v) => v,
        None => return Err(Galat::runtime("Parameter 2 harus instance JamVektor", span)),
    };

    let mut merged = vca.clock.clone();
    for (k, v) in &vcb.clock {
        let entry = merged.entry(k.clone()).or_insert(0);
        *entry = (*entry).max(*v);
    }

    Ok(vclock_to_value(&WidyaVectorClock {
        node_id: vca.node_id,
        clock: merged,
    }))
}

// Bandingkan causal order: "SEBELUM" (<), "SESUDAH" (>), "SAMA" (==), "KONFLIK" (|| concurrency)
fn compare_vclocks(vca: &WidyaVectorClock, vcb: &WidyaVectorClock) -> &'static str {
    let mut all_keys: std::collections::BTreeSet<String> = vca.clock.keys().cloned().collect();
    for k in vcb.clock.keys() {
        all_keys.insert(k.clone());
    }

    let mut a_less = false;
    let mut b_less = false;

    for k in all_keys {
        let val_a = vca.clock.get(&k).copied().unwrap_or(0);
        let val_b = vcb.clock.get(&k).copied().unwrap_or(0);
        if val_a < val_b {
            a_less = true;
        } else if val_a > val_b {
            b_less = true;
        }
    }

    if !a_less && !b_less {
        "SAMA"
    } else if a_less && !b_less {
        "SEBELUM"
    } else if !a_less && b_less {
        "SESUDAH"
    } else {
        "KONFLIK"
    }
}

fn builtin_vclock_apakah_konflik(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let vca = match value_to_vclock(&args[0]) {
        Some(v) => v,
        None => return Err(Galat::runtime("Parameter 1 harus instance JamVektor", span)),
    };
    let vcb = match value_to_vclock(&args[1]) {
        Some(v) => v,
        None => return Err(Galat::runtime("Parameter 2 harus instance JamVektor", span)),
    };

    Ok(Value::Bool(compare_vclocks(&vca, &vcb) == "KONFLIK"))
}

fn builtin_vclock_bandingkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let vca = match value_to_vclock(&args[0]) {
        Some(v) => v,
        None => return Err(Galat::runtime("Parameter 1 harus instance JamVektor", span)),
    };
    let vcb = match value_to_vclock(&args[1]) {
        Some(v) => v,
        None => return Err(Galat::runtime("Parameter 2 harus instance JamVektor", span)),
    };

    Ok(Value::String(compare_vclocks(&vca, &vcb).to_string()))
}

fn builtin_vclock_ke_kamus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    if let Some(vc) = value_to_vclock(&args[0]) {
        let mut map = HashMap::new();
        for (k, v) in vc.clock {
            map.insert(k, Value::Number(v as f64));
        }
        return Ok(Value::Map(Rc::new(RefCell::new(map))));
    }
    Err(Galat::runtime("Fungsi 'vclock_ke_kamus' memerlukan instance JamVektor", span))
}

// ============================================================================
// WidyaKademliaRPC (Decentralized P2P RPC Message Layer)
// ============================================================================

fn builtin_kademlia_rpc_buat_pesan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let rpc_type = match &args[0] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 1 tipe_rpc harus string (PING, STORE, FIND_NODE, FIND_VALUE)", span)),
    };
    let sender_id = match &args[1] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 2 pengirim_id harus string", span)),
    };
    let target_id = match &args[2] {
        Value::String(s) => s.as_str(),
        _ => return Err(Galat::runtime("Parameter 3 id_target harus string", span)),
    };
    let payload = args[3].clone();

    let tx_id = format!("tx_{:08x}", (sender_id.len() * 31 + target_id.len() * 17 + rpc_type.len() * 7) as u32);

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KademliaRPCMessage".to_string()));
    map.insert("tipe_rpc".to_string(), Value::String(rpc_type.to_string()));
    map.insert("pengirim_id".to_string(), Value::String(sender_id.to_string()));
    map.insert("target_id".to_string(), Value::String(target_id.to_string()));
    map.insert("tx_id".to_string(), Value::String(tx_id));
    map.insert("payload".to_string(), payload);
    map.insert("status".to_string(), Value::String("REQUEST".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_kademlia_rpc_uraikan_pesan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    match &args[0] {
        Value::Map(m) => {
            let map = m.borrow();
            if map.get("_tipe").and_then(|v| match v { Value::String(s) => Some(s.as_str()), _ => None }) == Some("KademliaRPCMessage") {
                return Ok(Value::Map(Rc::new(RefCell::new(map.clone()))));
            }
            Err(Galat::runtime("Objek bukan merupakan pesan KademliaRPC valid", span))
        }
        _ => Err(Galat::runtime("Fungsi 'kademlia_rpc_uraikan_pesan' memerlukan instance pesan KademliaRPC", span)),
    }
}

fn builtin_kademlia_rpc_buat_respons(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let req = match &args[0] {
        Value::Map(m) => m.borrow().clone(),
        _ => return Err(Galat::runtime("Parameter 1 harus instance pesan KademliaRPC", span)),
    };
    let payload_resp = args[1].clone();

    let orig_sender = req.get("pengirim_id").and_then(|v| match v { Value::String(s) => Some(s.clone()), _ => None }).unwrap_or_default();
    let orig_target = req.get("target_id").and_then(|v| match v { Value::String(s) => Some(s.clone()), _ => None }).unwrap_or_default();
    let tx_id = req.get("tx_id").and_then(|v| match v { Value::String(s) => Some(s.clone()), _ => None }).unwrap_or_default();
    let rpc_type = req.get("tipe_rpc").and_then(|v| match v { Value::String(s) => Some(s.clone()), _ => None }).unwrap_or_default();

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("KademliaRPCMessage".to_string()));
    map.insert("tipe_rpc".to_string(), Value::String(format!("{}_RESPONSE", rpc_type)));
    map.insert("pengirim_id".to_string(), Value::String(orig_target));
    map.insert("target_id".to_string(), Value::String(orig_sender));
    map.insert("tx_id".to_string(), Value::String(tx_id));
    map.insert("payload".to_string(), payload_resp);
    map.insert("status".to_string(), Value::String("RESPONSE".to_string()));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

