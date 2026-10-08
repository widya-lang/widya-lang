// ============================================================================
// WIDYABASE REDIS-COMPATIBLE IN-MEMORY CACHE & DATA STRUCTURES (cache.rs)
// ============================================================================
// Menyediakan kapabilitas setara Redis dalam WidyaBase:
// - Strings / Key-Value dengan TTL Expiration (SET, GET, DEL, INCR, DECR, EXPIRE, TTL)
// - Hashes (HSET, HGET, HDEL, HGETALL, HKEYS, HVALS)
// - Lists / Deque (LPUSH, RPUSH, LPOP, RPOP, LRANGE, LLEN)
// - Sets / Unik (SADD, SREM, SMEMBERS, SISMEMBER, SCARD)
// - Sorted Sets / ZSets (ZADD, ZSCORE, ZRANGE, ZREVRANGE, ZCARD)
// - Pub/Sub Message Broker (PUBLISH, SUBSCRIBE, UNSUBSCRIBE)
// - RESP (REdis Serialization Protocol) Serializer & Parser
// ============================================================================

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Nilai data terstruktur dalam Cache Engine
#[derive(Debug, Clone, PartialEq)]
pub enum CacheValue {
    String(String),
    Integer(i64),
    Float(f64),
    Hash(HashMap<String, String>),
    List(VecDeque<String>),
    Set(HashSet<String>),
    SortedSet(BTreeMap<i64, Vec<String>>), // score -> [members]
}

/// Metadata item cache termasuk waktu kedaluwarsa (TTL)
#[derive(Debug, Clone)]
pub struct CacheItem {
    pub value: CacheValue,
    pub expires_at: Option<u64>, // Unix timestamp in seconds (None = persistent)
}

impl CacheItem {
    pub fn is_expired(&self, now: u64) -> bool {
        if let Some(exp) = self.expires_at {
            now >= exp
        } else {
            false
        }
    }
}

/// Internal simplified unbounded message sender channel
pub mod tokio_sync_mpsc {
    use std::sync::{Arc, Mutex};
    use std::collections::VecDeque;

    #[derive(Clone)]
    pub struct Sender<T> {
        queue: Arc<Mutex<VecDeque<T>>>,
    }

    pub struct Receiver<T> {
        queue: Arc<Mutex<VecDeque<T>>>,
    }

    pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        (Sender { queue: Arc::clone(&queue) }, Receiver { queue })
    }

    impl<T> Sender<T> {
        pub fn send(&self, val: T) {
            let mut q = self.queue.lock().unwrap();
            q.push_back(val);
        }
    }

    impl<T> Receiver<T> {
        pub fn try_recv(&self) -> Option<T> {
            let mut q = self.queue.lock().unwrap();
            q.pop_front()
        }
    }
}

/// Mesin In-Memory Cache & Key-Value Store
pub struct CacheEngine {
    store: Arc<RwLock<HashMap<String, CacheItem>>>,
    pubsub_channels: Arc<RwLock<HashMap<String, Vec<Arc<tokio_sync_mpsc::Sender<String>>>>>>,
}

impl CacheEngine {
    pub fn new() -> Self {
        CacheEngine {
            store: Arc::new(RwLock::new(HashMap::new())),
            pubsub_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    // ========================================================================
    // 1. STRINGS & KEY-VALUE DENGAN TTL
    // ========================================================================

    pub fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) {
        let now = Self::current_timestamp();
        let expires_at = ttl_seconds.map(|ttl| now + ttl);
        let mut store = self.store.write().unwrap();
        store.insert(key.to_string(), CacheItem {
            value: CacheValue::String(value.to_string()),
            expires_at,
        });
    }

    pub fn set_int(&self, key: &str, value: i64, ttl_seconds: Option<u64>) {
        let now = Self::current_timestamp();
        let expires_at = ttl_seconds.map(|ttl| now + ttl);
        let mut store = self.store.write().unwrap();
        store.insert(key.to_string(), CacheItem {
            value: CacheValue::Integer(value),
            expires_at,
        });
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get(key) {
            if item.is_expired(now) {
                store.remove(key);
                return None;
            }
            match &item.value {
                CacheValue::String(s) => Some(s.clone()),
                CacheValue::Integer(i) => Some(i.to_string()),
                CacheValue::Float(f) => Some(f.to_string()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn del(&self, key: &str) -> bool {
        let mut store = self.store.write().unwrap();
        store.remove(key).is_some()
    }

    pub fn exists(&self, key: &str) -> bool {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get(key) {
            if item.is_expired(now) {
                store.remove(key);
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn expire(&self, key: &str, ttl_seconds: u64) -> bool {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get_mut(key) {
            if item.is_expired(now) {
                store.remove(key);
                false
            } else {
                item.expires_at = Some(now + ttl_seconds);
                true
            }
        } else {
            false
        }
    }

    pub fn ttl(&self, key: &str) -> i64 {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get(key) {
            if item.is_expired(now) {
                store.remove(key);
                -2 // Key does not exist
            } else if let Some(exp) = item.expires_at {
                (exp as i64) - (now as i64)
            } else {
                -1 // Key exists but has no expiration
            }
        } else {
            -2 // Key does not exist
        }
    }

    pub fn incr_by(&self, key: &str, delta: i64) -> Result<i64, String> {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get_mut(key) {
            if item.is_expired(now) {
                item.value = CacheValue::Integer(delta);
                item.expires_at = None;
                Ok(delta)
            } else {
                match &mut item.value {
                    CacheValue::Integer(i) => {
                        *i += delta;
                        Ok(*i)
                    }
                    CacheValue::String(s) => {
                        let parsed = s.parse::<i64>().map_err(|_| "Nilai bukan integer".to_string())?;
                        let new_val = parsed + delta;
                        item.value = CacheValue::Integer(new_val);
                        Ok(new_val)
                    }
                    _ => Err("Tipe nilai tidak dapat di-increment".to_string()),
                }
            }
        } else {
            store.insert(key.to_string(), CacheItem {
                value: CacheValue::Integer(delta),
                expires_at: None,
            });
            Ok(delta)
        }
    }

    // ========================================================================
    // 2. HASHES (HSET, HGET, HDEL, HGETALL)
    // ========================================================================

    pub fn hset(&self, key: &str, field: &str, val: &str) -> bool {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        let item = store.entry(key.to_string()).or_insert_with(|| CacheItem {
            value: CacheValue::Hash(HashMap::new()),
            expires_at: None,
        });
        if item.is_expired(now) {
            item.value = CacheValue::Hash(HashMap::new());
            item.expires_at = None;
        }
        if let CacheValue::Hash(ref mut h) = item.value {
            let is_new = !h.contains_key(field);
            h.insert(field.to_string(), val.to_string());
            is_new
        } else {
            false
        }
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<String> {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get(key) {
            if item.is_expired(now) {
                store.remove(key);
                None
            } else if let CacheValue::Hash(h) = &item.value {
                h.get(field).cloned()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn hgetall(&self, key: &str) -> HashMap<String, String> {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get(key) {
            if item.is_expired(now) {
                store.remove(key);
                HashMap::new()
            } else if let CacheValue::Hash(h) = &item.value {
                h.clone()
            } else {
                HashMap::new()
            }
        } else {
            HashMap::new()
        }
    }

    // ========================================================================
    // 3. LISTS / ANTREAN (LPUSH, RPUSH, LPOP, RPOP, LRANGE, LLEN)
    // ========================================================================

    pub fn lpush(&self, key: &str, val: &str) -> usize {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        let item = store.entry(key.to_string()).or_insert_with(|| CacheItem {
            value: CacheValue::List(VecDeque::new()),
            expires_at: None,
        });
        if item.is_expired(now) {
            item.value = CacheValue::List(VecDeque::new());
            item.expires_at = None;
        }
        if let CacheValue::List(ref mut list) = item.value {
            list.push_front(val.to_string());
            list.len()
        } else {
            0
        }
    }

    pub fn rpush(&self, key: &str, val: &str) -> usize {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        let item = store.entry(key.to_string()).or_insert_with(|| CacheItem {
            value: CacheValue::List(VecDeque::new()),
            expires_at: None,
        });
        if item.is_expired(now) {
            item.value = CacheValue::List(VecDeque::new());
            item.expires_at = None;
        }
        if let CacheValue::List(ref mut list) = item.value {
            list.push_back(val.to_string());
            list.len()
        } else {
            0
        }
    }

    pub fn lpop(&self, key: &str) -> Option<String> {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get_mut(key) {
            if item.is_expired(now) {
                store.remove(key);
                None
            } else if let CacheValue::List(ref mut list) = item.value {
                list.pop_front()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn rpop(&self, key: &str) -> Option<String> {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get_mut(key) {
            if item.is_expired(now) {
                store.remove(key);
                None
            } else if let CacheValue::List(ref mut list) = item.value {
                list.pop_back()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn lrange(&self, key: &str, start: isize, stop: isize) -> Vec<String> {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get(key) {
            if item.is_expired(now) {
                store.remove(key);
                Vec::new()
            } else if let CacheValue::List(list) = &item.value {
                let len = list.len() as isize;
                if len == 0 { return Vec::new(); }
                let s = if start < 0 { (len + start).max(0) as usize } else { start.min(len - 1) as usize };
                let e = if stop < 0 { (len + stop).max(0) as usize } else { stop.min(len - 1) as usize };
                if s > e || s >= list.len() {
                    Vec::new()
                } else {
                    list.iter().skip(s).take(e - s + 1).cloned().collect()
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    // ========================================================================
    // 4. SETS / HIMPUNAN UNIK (SADD, SMEMBERS, SREM, SISMEMBER)
    // ========================================================================

    pub fn sadd(&self, key: &str, member: &str) -> bool {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        let item = store.entry(key.to_string()).or_insert_with(|| CacheItem {
            value: CacheValue::Set(HashSet::new()),
            expires_at: None,
        });
        if item.is_expired(now) {
            item.value = CacheValue::Set(HashSet::new());
            item.expires_at = None;
        }
        if let CacheValue::Set(ref mut set) = item.value {
            set.insert(member.to_string())
        } else {
            false
        }
    }

    pub fn smembers(&self, key: &str) -> Vec<String> {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get(key) {
            if item.is_expired(now) {
                store.remove(key);
                Vec::new()
            } else if let CacheValue::Set(set) = &item.value {
                set.iter().cloned().collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    pub fn sismember(&self, key: &str, member: &str) -> bool {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get(key) {
            if item.is_expired(now) {
                store.remove(key);
                false
            } else if let CacheValue::Set(set) = &item.value {
                set.contains(member)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn srem(&self, key: &str, member: &str) -> bool {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get_mut(key) {
            if item.is_expired(now) {
                false
            } else if let CacheValue::Set(ref mut set) = item.value {
                set.remove(member)
            } else {
                false
            }
        } else {
            false
        }
    }

    // ========================================================================
    // 5. SORTED SETS / ZSETS (ZADD, ZRANGE, ZREVRANGE)
    // ========================================================================

    pub fn zadd(&self, key: &str, score: i64, member: &str) -> bool {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        let item = store.entry(key.to_string()).or_insert_with(|| CacheItem {
            value: CacheValue::SortedSet(BTreeMap::new()),
            expires_at: None,
        });
        if item.is_expired(now) {
            item.value = CacheValue::SortedSet(BTreeMap::new());
            item.expires_at = None;
        }
        if let CacheValue::SortedSet(ref mut zset) = item.value {
            let members = zset.entry(score).or_insert_with(Vec::new);
            if !members.contains(&member.to_string()) {
                members.push(member.to_string());
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn zrange(&self, key: &str, start: usize, stop: usize) -> Vec<(String, i64)> {
        let now = Self::current_timestamp();
        let mut store = self.store.write().unwrap();
        if let Some(item) = store.get(key) {
            if item.is_expired(now) {
                store.remove(key);
                Vec::new()
            } else if let CacheValue::SortedSet(zset) = &item.value {
                let mut flat = Vec::new();
                for (score, members) in zset.iter() {
                    for m in members {
                        flat.push((m.clone(), *score));
                    }
                }
                let count = stop.saturating_sub(start).saturating_add(1);
                flat.into_iter().skip(start).take(count).collect()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    // ========================================================================
    // 6. PUB/SUB REALTIME BROKER
    // ========================================================================

    pub fn subscribe(&self, channel: &str) -> tokio_sync_mpsc::Receiver<String> {
        let (tx, rx) = tokio_sync_mpsc::channel();
        let mut channels = self.pubsub_channels.write().unwrap();
        let list = channels.entry(channel.to_string()).or_insert_with(Vec::new);
        list.push(Arc::new(tx));
        rx
    }

    pub fn publish(&self, channel: &str, message: &str) -> usize {
        let channels = self.pubsub_channels.read().unwrap();
        if let Some(subs) = channels.get(channel) {
            let count = subs.len();
            for sub in subs {
                sub.send(message.to_string());
            }
            count
        } else {
            0
        }
    }

    // ========================================================================
    // 7. RESP (REDIS SERIALIZATION PROTOCOL) PARSER & FORMATTER
    // ========================================================================

    pub fn format_resp_simple_string(msg: &str) -> String {
        format!("+{}\r\n", msg)
    }

    pub fn format_resp_error(err: &str) -> String {
        format!("-{}\r\n", err)
    }

    pub fn format_resp_integer(num: i64) -> String {
        format!(":{}\r\n", num)
    }

    pub fn format_resp_bulk_string(val: Option<&str>) -> String {
        match val {
            Some(s) => format!("${}\r\n{}\r\n", s.len(), s),
            None => "$-1\r\n".to_string(),
        }
    }

    pub fn format_resp_array(items: &[String]) -> String {
        let mut out = format!("*{}\r\n", items.len());
        for item in items {
            out.push_str(&Self::format_resp_bulk_string(Some(item)));
        }
        out
    }

    /// Menjalankan perintah bergaya Redis Command String
    pub fn execute_redis_command(&self, cmd_line: &str) -> Result<String, String> {
        let parts: Vec<&str> = cmd_line.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(Self::format_resp_error("ERR empty command"));
        }

        let cmd = parts[0].to_uppercase();
        match cmd.as_str() {
            "PING" | "TES" => Ok(Self::format_resp_simple_string("PONG")),
            "SET" | "SIMPAN_KUNCI" => {
                if parts.len() < 3 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'set'"));
                }
                let key = parts[1];
                let val = parts[2];
                let mut ttl = None;
                if parts.len() >= 5 && (parts[3].eq_ignore_ascii_case("EX") || parts[3].eq_ignore_ascii_case("KEDALUWARSA")) {
                    ttl = parts[4].parse::<u64>().ok();
                }
                self.set(key, val, ttl);
                Ok(Self::format_resp_simple_string("OK"))
            }
            "GET" | "AMBIL_KUNCI" => {
                if parts.len() < 2 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'get'"));
                }
                let res = self.get(parts[1]);
                Ok(Self::format_resp_bulk_string(res.as_deref()))
            }
            "DEL" | "HAPUS_KUNCI" => {
                if parts.len() < 2 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'del'"));
                }
                let deleted = self.del(parts[1]);
                Ok(Self::format_resp_integer(if deleted { 1 } else { 0 }))
            }
            "EXPIRE" | "KEDALUWARSA" => {
                if parts.len() < 3 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'expire'"));
                }
                let ttl = parts[2].parse::<u64>().unwrap_or(0);
                let res = self.expire(parts[1], ttl);
                Ok(Self::format_resp_integer(if res { 1 } else { 0 }))
            }
            "TTL" | "SISA_WAKTU" => {
                if parts.len() < 2 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'ttl'"));
                }
                let t = self.ttl(parts[1]);
                Ok(Self::format_resp_integer(t))
            }
            "INCR" | "TAMBAH_SATU" => {
                if parts.len() < 2 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'incr'"));
                }
                match self.incr_by(parts[1], 1) {
                    Ok(val) => Ok(Self::format_resp_integer(val)),
                    Err(e) => Ok(Self::format_resp_error(&e)),
                }
            }
            "DECR" | "KURANG_SATU" => {
                if parts.len() < 2 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'decr'"));
                }
                match self.incr_by(parts[1], -1) {
                    Ok(val) => Ok(Self::format_resp_integer(val)),
                    Err(e) => Ok(Self::format_resp_error(&e)),
                }
            }
            "HSET" => {
                if parts.len() < 4 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'hset'"));
                }
                let is_new = self.hset(parts[1], parts[2], parts[3]);
                Ok(Self::format_resp_integer(if is_new { 1 } else { 0 }))
            }
            "HGET" => {
                if parts.len() < 3 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'hget'"));
                }
                let val = self.hget(parts[1], parts[2]);
                Ok(Self::format_resp_bulk_string(val.as_deref()))
            }
            "LPUSH" => {
                if parts.len() < 3 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'lpush'"));
                }
                let len = self.lpush(parts[1], parts[2]);
                Ok(Self::format_resp_integer(len as i64))
            }
            "RPOP" => {
                if parts.len() < 2 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'rpop'"));
                }
                let val = self.rpop(parts[1]);
                Ok(Self::format_resp_bulk_string(val.as_deref()))
            }
            "LRANGE" => {
                if parts.len() < 4 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'lrange'"));
                }
                let s = parts[2].parse::<isize>().unwrap_or(0);
                let e = parts[3].parse::<isize>().unwrap_or(-1);
                let list = self.lrange(parts[1], s, e);
                Ok(Self::format_resp_array(&list))
            }
            "SADD" => {
                if parts.len() < 3 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'sadd'"));
                }
                let added = self.sadd(parts[1], parts[2]);
                Ok(Self::format_resp_integer(if added { 1 } else { 0 }))
            }
            "SMEMBERS" => {
                if parts.len() < 2 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'smembers'"));
                }
                let members = self.smembers(parts[1]);
                Ok(Self::format_resp_array(&members))
            }
            "PUBLISH" | "TERBITKAN" => {
                if parts.len() < 3 {
                    return Ok(Self::format_resp_error("ERR wrong number of arguments for 'publish'"));
                }
                let count = self.publish(parts[1], parts[2]);
                Ok(Self::format_resp_integer(count as i64))
            }
            _ => Ok(Self::format_resp_error(&format!("ERR unknown command '{}'", parts[0]))),
        }
    }
}
