use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::stdlib::common::*;
use crate::value::Value;
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn register_storage(env: &mut Environment) {
    // WidyaLSM (Log-Structured Merge Tree Storage Engine)
    register_fn(env, "BukaLSM", None, builtin_lsm_buka);
    register_fn(env, "lsm_tulis", Some(3), builtin_lsm_tulis);
    register_fn(env, "lsm_baca", Some(2), builtin_lsm_baca);
    register_fn(env, "lsm_flush", Some(1), builtin_lsm_flush);
    register_fn(env, "lsm_hapus", Some(2), builtin_lsm_hapus);

    // WidyaTar (Tarball Archival & Packing Engine)
    register_fn(env, "buat_arsip_tar", None, builtin_tar_buat);
    register_fn(env, "tar_tambah_berkas", Some(3), builtin_tar_tambah);
    register_fn(env, "tar_ekspor_biner", Some(1), builtin_tar_ekspor);
    register_fn(env, "tar_baca_entri", Some(1), builtin_tar_baca);

    // WidyaKV (In-Memory Key-Value Store with TTL & Prefix Scan)
    register_fn(env, "PenyimpananKV", None, builtin_kv_buat);
    register_fn(env, "kv_pasang", None, builtin_kv_pasang);
    register_fn(env, "kv_ambil", Some(2), builtin_kv_ambil);
    register_fn(env, "kv_hapus", Some(2), builtin_kv_hapus);
    register_fn(env, "kv_pindai_awalan", Some(2), builtin_kv_pindai);
    register_fn(env, "kv_inkremen", None, builtin_kv_inkremen);

    // WidyaLRUCache (Least Recently Used Cache Engine)
    register_fn(env, "CacheLRU", Some(1), builtin_lru_buat);
    register_fn(env, "lru_pasang", Some(3), builtin_lru_pasang);
    register_fn(env, "lru_ambil", Some(2), builtin_lru_ambil);
    register_fn(env, "lru_ukuran", Some(1), builtin_lru_ukuran);
    register_fn(env, "lru_bersihkan", Some(1), builtin_lru_bersihkan);

    // WidyaWAL (Write-Ahead Logging & Crash Recovery)
    register_fn(env, "LogWAL", None, builtin_wal_buat);
    register_fn(env, "wal_tulis", Some(4), builtin_wal_tulis);
    register_fn(env, "wal_checkpoint", Some(1), builtin_wal_checkpoint);
    register_fn(env, "wal_pulihkan", Some(1), builtin_wal_pulihkan);
    register_fn(env, "wal_panjang", Some(1), builtin_wal_panjang);

}

// =========================================================================
// WidyaLSM: Persistent Embedded Key-Value Storage Engine (LSM-Tree)
// =========================================================================

fn builtin_lsm_buka(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let nama_tabel = if !args.is_empty() { args[0].to_string_repr() } else { "lsm_default".to_string() };

    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("EngineLSM".to_string()));
    map.insert("nama_tabel".to_string(), Value::String(nama_tabel));
    map.insert("memtable".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("sstable".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    map.insert("wal_log".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_lsm_tulis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let db = &args[0];
    let key = args[1].to_string_repr();
    let val = args[2].clone();

    match db {
        Value::Map(m) => {
            let map = m.borrow();
            if let (Some(Value::Map(mem_rc)), Some(Value::Array(wal_rc))) = (map.get("memtable"), map.get("wal_log")) {
                mem_rc.borrow_mut().insert(key.clone(), val.clone());
                wal_rc.borrow_mut().push(Value::String(format!("PUT:{}={}", key, val.to_string_repr())));
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("Struktur LSM Engine tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek EngineLSM", span)),
    }
}

fn builtin_lsm_baca(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let db = &args[0];
    let key = args[1].to_string_repr();

    match db {
        Value::Map(m) => {
            let map = m.borrow();
            // 1. Cek MemTable
            if let Some(Value::Map(mem_rc)) = map.get("memtable") {
                if let Some(v) = mem_rc.borrow().get(&key) {
                    if let Value::String(s) = v {
                        if s == "__TOMBSTONE__" {
                            return Ok(Value::Nil);
                        }
                    }
                    return Ok(v.clone());
                }
            }
            // 2. Cek SSTable
            if let Some(Value::Map(sst_rc)) = map.get("sstable") {
                if let Some(v) = sst_rc.borrow().get(&key) {
                    if let Value::String(s) = v {
                        if s == "__TOMBSTONE__" {
                            return Ok(Value::Nil);
                        }
                    }
                    return Ok(v.clone());
                }
            }
            Ok(Value::Nil)
        }
        _ => Err(Galat::runtime("Diharapkan objek EngineLSM", span)),
    }
}

fn builtin_lsm_flush(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let db = &args[0];

    match db {
        Value::Map(m) => {
            let map = m.borrow();
            if let (Some(Value::Map(mem_rc)), Some(Value::Map(sst_rc)), Some(Value::Array(wal_rc))) =
                (map.get("memtable"), map.get("sstable"), map.get("wal_log"))
            {
                let mut mem = mem_rc.borrow_mut();
                let mut sst = sst_rc.borrow_mut();

                for (k, v) in mem.drain() {
                    sst.insert(k, v);
                }
                wal_rc.borrow_mut().clear();
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("Struktur LSM Engine tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek EngineLSM", span)),
    }
}

fn builtin_lsm_hapus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let db = &args[0];
    let key = args[1].to_string_repr();

    match db {
        Value::Map(m) => {
            let map = m.borrow();
            if let (Some(Value::Map(mem_rc)), Some(Value::Array(wal_rc))) = (map.get("memtable"), map.get("wal_log")) {
                mem_rc.borrow_mut().insert(key.clone(), Value::String("__TOMBSTONE__".to_string()));
                wal_rc.borrow_mut().push(Value::String(format!("DELETE:{}", key)));
                Ok(Value::Bool(true))
            } else {
                Err(Galat::runtime("Struktur LSM Engine tidak valid", span))
            }
        }
        _ => Err(Galat::runtime("Diharapkan objek EngineLSM", span)),
    }
}

// ==============================================================================
// WidyaTar: Tarball Archival & In-Memory Packing Engine
// ==============================================================================
fn builtin_tar_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut obj = HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("ArsipTar".to_string()));
    obj.insert("entri".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));

    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_tar_tambah(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tar_val = &args[0];
    let path = args[1].to_string_repr();
    let content = match &args[2] {
        Value::String(s) => s.clone(),
        Value::Array(a) => {
            let bytes: Vec<u8> = a.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect();
            String::from_utf8_lossy(&bytes).to_string()
        }
        _ => return Err(Galat::runtime("Konten arsip harus berupa teks atau array byte", span)),
    };

    match tar_val {
        Value::Map(m) => {
            let map = m.borrow();
            let entri_rc = match map.get("entri") {
                Some(Value::Map(em)) => em.clone(),
                _ => return Err(Galat::runtime("Objek ArsipTar tidak valid", span)),
            };

            let mut entry = HashMap::new();
            entry.insert("jalur".to_string(), Value::String(path.clone()));
            entry.insert("ukuran".to_string(), Value::Number(content.len() as f64));
            entry.insert("konten".to_string(), Value::String(content));

            entri_rc.borrow_mut().insert(path, Value::Map(Rc::new(RefCell::new(entry))));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi tar_tambah_berkas memerlukan objek ArsipTar", span)),
    }
}

fn write_octal(slice: &mut [u8], value: usize) {
    let s = format!("{:011o} ", value);
    let bytes = s.as_bytes();
    let len = bytes.len().min(slice.len());
    slice[..len].copy_from_slice(&bytes[..len]);
}

fn builtin_tar_ekspor(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tar_val = &args[0];
    match tar_val {
        Value::Map(m) => {
            let map = m.borrow();
            let entri_rc = match map.get("entri") {
                Some(Value::Map(em)) => em.borrow(),
                _ => return Err(Galat::runtime("Objek ArsipTar tidak valid", span)),
            };

            let mut tar_bytes: Vec<u8> = Vec::new();

            for (path, entry_val) in entri_rc.iter() {
                if let Value::Map(em) = entry_val {
                    let emap = em.borrow();
                    let content = emap.get("konten").map(|v| v.to_string_repr()).unwrap_or_default();
                    let content_bytes = content.as_bytes();

                    let mut header = [0u8; 512];
                    // 0..100: file name
                    let path_bytes = path.as_bytes();
                    let name_len = path_bytes.len().min(100);
                    header[..name_len].copy_from_slice(&path_bytes[..name_len]);

                    // 100..108: mode (0644)
                    header[100..108].copy_from_slice(b"0000644\0");
                    // 108..116: uid (0)
                    header[108..116].copy_from_slice(b"0000000\0");
                    // 116..124: gid (0)
                    header[116..124].copy_from_slice(b"0000000\0");

                    // 124..136: size in octal
                    write_octal(&mut header[124..136], content_bytes.len());

                    // 136..148: mtime (0)
                    header[136..148].copy_from_slice(b"00000000000\0");

                    // 156: typeflag ('0' = normal file)
                    header[156] = b'0';

                    // 257..263: magic ustar
                    header[257..263].copy_from_slice(b"ustar\0");
                    // 263..265: version
                    header[263..265].copy_from_slice(b"00");

                    // Compute checksum (148..156 initially spaces)
                    header[148..156].copy_from_slice(b"        ");
                    let mut cksum: u32 = 0;
                    for b in &header {
                        cksum += *b as u32;
                    }
                    let cksum_str = format!("{:06o}\0 ", cksum);
                    header[148..156].copy_from_slice(&cksum_str.as_bytes()[..8]);

                    tar_bytes.extend_from_slice(&header);
                    tar_bytes.extend_from_slice(content_bytes);

                    // Pad content to 512-byte boundary
                    let pad_len = (512 - (content_bytes.len() % 512)) % 512;
                    if pad_len > 0 {
                        tar_bytes.extend(vec![0u8; pad_len]);
                    }
                }
            }

            // Two 512-byte zero blocks at the end of tar
            tar_bytes.extend(vec![0u8; 1024]);

            let val_bytes: Vec<Value> = tar_bytes.into_iter().map(|b| Value::Number(b as f64)).collect();
            Ok(Value::Array(Rc::new(RefCell::new(val_bytes))))
        }
        _ => Err(Galat::runtime("Fungsi tar_ekspor_biner memerlukan objek ArsipTar", span)),
    }
}

fn builtin_tar_baca(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let tar_bytes: Vec<u8> = match &args[0] {
        Value::Array(a) => a.borrow().iter().map(|v| match v { Value::Number(n) => *n as u8, _ => 0 }).collect(),
        _ => return Err(Galat::runtime("Fungsi tar_baca_entri memerlukan array byte tar", span)),
    };

    let mut entries_list = Vec::new();
    let mut offset = 0;

    while offset + 512 <= tar_bytes.len() {
        let header = &tar_bytes[offset..offset + 512];
        if header.iter().all(|&b| b == 0) {
            break; // End of archive
        }

        // File name (0..100)
        let name_end = header[..100].iter().position(|&b| b == 0).unwrap_or(100);
        let path = String::from_utf8_lossy(&header[..name_end]).to_string();

        // Size (124..136) in octal
        let size_str = String::from_utf8_lossy(&header[124..136]);
        let size = usize::from_str_radix(size_str.trim().trim_matches('\0'), 8).unwrap_or(0);

        offset += 512;
        if offset + size > tar_bytes.len() {
            break;
        }

        let content_bytes = &tar_bytes[offset..offset + size];
        let content_str = String::from_utf8_lossy(content_bytes).to_string();

        let mut ent_obj = HashMap::new();
        ent_obj.insert("jalur".to_string(), Value::String(path));
        ent_obj.insert("ukuran".to_string(), Value::Number(size as f64));
        ent_obj.insert("konten".to_string(), Value::String(content_str));
        entries_list.push(Value::Map(Rc::new(RefCell::new(ent_obj))));

        let pad_len = (512 - (size % 512)) % 512;
        offset += size + pad_len;
    }

    Ok(Value::Array(Rc::new(RefCell::new(entries_list))))
}

// ==============================================================================
// WidyaKV: In-Memory Key-Value Store with TTL & Prefix Scan
// ==============================================================================
pub fn current_epoch_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}

fn builtin_kv_buat(_args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut obj = std::collections::HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("PenyimpananKV".to_string()));
    // store holds map of key -> (value, expire_at_ms)
    obj.insert("data".to_string(), Value::Map(Rc::new(RefCell::new(std::collections::HashMap::new()))));
    obj.insert("ekspirasi".to_string(), Value::Map(Rc::new(RefCell::new(std::collections::HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_kv_pasang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let kv_val = &args[0];
    let key = args[1].to_string_repr();
    let val = args[2].clone();
    let ttl_ms = args.get(3).and_then(|v| if let Value::Number(n) = v { Some(*n as u64) } else { None });

    match kv_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") {
                Some(Value::Map(dm)) => dm.clone(),
                _ => return Err(Galat::runtime("Objek PenyimpananKV tidak valid", span)),
            };
            let exp_rc = match map.get("ekspirasi") {
                Some(Value::Map(em)) => em.clone(),
                _ => return Err(Galat::runtime("Objek PenyimpananKV tidak valid", span)),
            };

            data_rc.borrow_mut().insert(key.clone(), val);
            if let Some(ttl) = ttl_ms {
                let expire_time = current_epoch_ms() + ttl;
                exp_rc.borrow_mut().insert(key, Value::Number(expire_time as f64));
            } else {
                exp_rc.borrow_mut().remove(&key);
            }
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi kv_pasang memerlukan objek PenyimpananKV", span)),
    }
}

fn builtin_kv_ambil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let kv_val = &args[0];
    let key = args[1].to_string_repr();

    match kv_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") {
                Some(Value::Map(dm)) => dm.clone(),
                _ => return Err(Galat::runtime("Objek PenyimpananKV tidak valid", span)),
            };
            let exp_rc = match map.get("ekspirasi") {
                Some(Value::Map(em)) => em.clone(),
                _ => return Err(Galat::runtime("Objek PenyimpananKV tidak valid", span)),
            };

            let now = current_epoch_ms();
            if let Some(Value::Number(exp_time)) = exp_rc.borrow().get(&key) {
                if now >= *exp_time as u64 {
                    // Expired: cleanup lazily
                    data_rc.borrow_mut().remove(&key);
                    exp_rc.borrow_mut().remove(&key);
                    return Ok(Value::Nil);
                }
            }

            match data_rc.borrow().get(&key) {
                Some(v) => Ok(v.clone()),
                None => Ok(Value::Nil),
            }
        }
        _ => Err(Galat::runtime("Fungsi kv_ambil memerlukan objek PenyimpananKV", span)),
    }
}

fn builtin_kv_hapus(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let kv_val = &args[0];
    let key = args[1].to_string_repr();

    match kv_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") {
                Some(Value::Map(dm)) => dm.clone(),
                _ => return Err(Galat::runtime("Objek PenyimpananKV tidak valid", span)),
            };
            let exp_rc = match map.get("ekspirasi") {
                Some(Value::Map(em)) => em.clone(),
                _ => return Err(Galat::runtime("Objek PenyimpananKV tidak valid", span)),
            };

            exp_rc.borrow_mut().remove(&key);
            let existed = data_rc.borrow_mut().remove(&key).is_some();
            Ok(Value::Bool(existed))
        }
        _ => Err(Galat::runtime("Fungsi kv_hapus memerlukan objek PenyimpananKV", span)),
    }
}

fn builtin_kv_pindai(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let kv_val = &args[0];
    let prefix = args[1].to_string_repr();

    match kv_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") {
                Some(Value::Map(dm)) => dm.clone(),
                _ => return Err(Galat::runtime("Objek PenyimpananKV tidak valid", span)),
            };
            let exp_rc = match map.get("ekspirasi") {
                Some(Value::Map(em)) => em.clone(),
                _ => return Err(Galat::runtime("Objek PenyimpananKV tidak valid", span)),
            };

            let now = current_epoch_ms();
            let mut results = Vec::new();
            let keys: Vec<String> = data_rc.borrow().keys().cloned().collect();

            for k in keys {
                if k.starts_with(&prefix) {
                    let mut expired = false;
                    if let Some(Value::Number(exp_time)) = exp_rc.borrow().get(&k) {
                        if now >= *exp_time as u64 {
                            expired = true;
                        }
                    }

                    if expired {
                        data_rc.borrow_mut().remove(&k);
                        exp_rc.borrow_mut().remove(&k);
                    } else if let Some(v) = data_rc.borrow().get(&k) {
                        let mut pair = std::collections::HashMap::new();
                        pair.insert("kunci".to_string(), Value::String(k.clone()));
                        pair.insert("nilai".to_string(), v.clone());
                        results.push(Value::Map(Rc::new(RefCell::new(pair))));
                    }
                }
            }

            Ok(Value::Array(Rc::new(RefCell::new(results))))
        }
        _ => Err(Galat::runtime("Fungsi kv_pindai_awalan memerlukan objek PenyimpananKV", span)),
    }
}

fn builtin_kv_inkremen(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let kv_val = &args[0];
    let key = args[1].to_string_repr();
    let delta = args.get(2).and_then(|v| if let Value::Number(n) = v { Some(*n) } else { None }).unwrap_or(1.0);

    match kv_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") {
                Some(Value::Map(dm)) => dm.clone(),
                _ => return Err(Galat::runtime("Objek PenyimpananKV tidak valid", span)),
            };

            let mut dm = data_rc.borrow_mut();
            let current = match dm.get(&key) {
                Some(Value::Number(n)) => *n,
                _ => 0.0,
            };
            let next = current + delta;
            dm.insert(key, Value::Number(next));
            Ok(Value::Number(next))
        }
        _ => Err(Galat::runtime("Fungsi kv_inkremen memerlukan objek PenyimpananKV", span)),
    }
}

// ==============================================================================
// WidyaLRUCache: Least Recently Used In-Memory Cache
// ==============================================================================
fn builtin_lru_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let capacity = match args.first() {
        Some(Value::Number(n)) => (*n as usize).max(1),
        _ => 10,
    };
    let mut obj = std::collections::HashMap::new();
    obj.insert("_tipe".to_string(), Value::String("CacheLRU".to_string()));
    obj.insert("kapasitas".to_string(), Value::Number(capacity as f64));
    obj.insert("data".to_string(), Value::Map(Rc::new(RefCell::new(std::collections::HashMap::new()))));
    // Order list stores keys from oldest (front) to newest (back)
    obj.insert("urutan".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(obj))))
}

fn builtin_lru_pasang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lru_val = &args[0];
    let key = args[1].to_string_repr();
    let val = args[2].clone();

    match lru_val {
        Value::Map(m) => {
            let map = m.borrow();
            let capacity = match map.get("kapasitas") { Some(Value::Number(n)) => *n as usize, _ => 10 };
            let data_rc = match map.get("data") { Some(Value::Map(dm)) => dm.clone(), _ => return Err(Galat::runtime("Objek CacheLRU tidak valid", span)) };
            let order_rc = match map.get("urutan") { Some(Value::Array(oa)) => oa.clone(), _ => return Err(Galat::runtime("Objek CacheLRU tidak valid", span)) };

            let mut dm = data_rc.borrow_mut();
            let mut oa = order_rc.borrow_mut();

            // If key already exists, remove from order
            oa.retain(|k_val| k_val.to_string_repr() != key);

            // If at capacity and key is new, evict oldest (first element)
            if !dm.contains_key(&key) && dm.len() >= capacity && !oa.is_empty() {
                let oldest_key = oa.remove(0).to_string_repr();
                dm.remove(&oldest_key);
            }

            dm.insert(key.clone(), val);
            oa.push(Value::String(key));
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi lru_pasang memerlukan objek CacheLRU", span)),
    }
}

fn builtin_lru_ambil(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lru_val = &args[0];
    let key = args[1].to_string_repr();

    match lru_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") { Some(Value::Map(dm)) => dm.clone(), _ => return Err(Galat::runtime("Objek CacheLRU tidak valid", span)) };
            let order_rc = match map.get("urutan") { Some(Value::Array(oa)) => oa.clone(), _ => return Err(Galat::runtime("Objek CacheLRU tidak valid", span)) };

            let dm = data_rc.borrow();
            if let Some(v) = dm.get(&key) {
                let mut oa = order_rc.borrow_mut();
                oa.retain(|k_val| k_val.to_string_repr() != key);
                oa.push(Value::String(key));
                Ok(v.clone())
            } else {
                Ok(Value::Nil)
            }
        }
        _ => Err(Galat::runtime("Fungsi lru_ambil memerlukan objek CacheLRU", span)),
    }
}

fn builtin_lru_ukuran(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lru_val = &args[0];
    match lru_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") { Some(Value::Map(dm)) => dm.clone(), _ => return Err(Galat::runtime("Objek CacheLRU tidak valid", span)) };
            Ok(Value::Number(data_rc.borrow().len() as f64))
        }
        _ => Err(Galat::runtime("Fungsi lru_ukuran memerlukan objek CacheLRU", span)),
    }
}

fn builtin_lru_bersihkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let lru_val = &args[0];
    match lru_val {
        Value::Map(m) => {
            let map = m.borrow();
            let data_rc = match map.get("data") { Some(Value::Map(dm)) => dm.clone(), _ => return Err(Galat::runtime("Objek CacheLRU tidak valid", span)) };
            let order_rc = match map.get("urutan") { Some(Value::Array(oa)) => oa.clone(), _ => return Err(Galat::runtime("Objek CacheLRU tidak valid", span)) };
            data_rc.borrow_mut().clear();
            order_rc.borrow_mut().clear();
            Ok(Value::Bool(true))
        }
        _ => Err(Galat::runtime("Fungsi lru_bersihkan memerlukan objek CacheLRU", span)),
    }
}

// =========================================================================
// WIDYAWAL (WRITE-AHEAD LOGGING & CRASH RECOVERY ENGINE)
// =========================================================================

fn builtin_wal_buat(args: &[Value], _span: &Span) -> Result<Value, Galat> {
    let mut map = HashMap::new();
    map.insert("_tipe".to_string(), Value::String("LogWAL".to_string()));
    let nama_tabel = if let Some(v) = args.first() {
        v.to_string_repr()
    } else {
        "utama".to_string()
    };
    map.insert("tabel".to_string(), Value::String(nama_tabel));
    map.insert("lsn".to_string(), Value::Number(0.0));
    map.insert("log".to_string(), Value::Array(Rc::new(RefCell::new(Vec::new()))));
    map.insert("status_terakhir".to_string(), Value::Map(Rc::new(RefCell::new(HashMap::new()))));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn builtin_wal_tulis(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let wal_val = &args[0];
    let operasi = args[1].to_string_repr().to_uppercase();
    let kunci = args[2].to_string_repr();
    let nilai = args[3].clone();

    match wal_val {
        Value::Map(m) => {
            let mut wal_map = m.borrow_mut();
            let lsn = match wal_map.get("lsn") {
                Some(Value::Number(n)) => *n + 1.0,
                _ => 1.0,
            };
            wal_map.insert("lsn".to_string(), Value::Number(lsn));

            let mut entri = HashMap::new();
            entri.insert("lsn".to_string(), Value::Number(lsn));
            entri.insert("op".to_string(), Value::String(operasi.clone()));
            entri.insert("kunci".to_string(), Value::String(kunci.clone()));
            entri.insert("nilai".to_string(), nilai.clone());

            let mut hasher = Sha256::new();
            hasher.update(format!("{}:{}:{}:{}", lsn, operasi, kunci, nilai.to_string_repr()).as_bytes());
            let hash_str = hex::encode(hasher.finalize());
            entri.insert("checksum".to_string(), Value::String(hash_str));

            if let Some(Value::Array(log_arr)) = wal_map.get("log") {
                log_arr.borrow_mut().push(Value::Map(Rc::new(RefCell::new(entri))));
            }

            if let Some(Value::Map(state_map)) = wal_map.get("status_terakhir") {
                if operasi == "DELETE" || operasi == "HAPUS" {
                    state_map.borrow_mut().remove(&kunci);
                } else {
                    state_map.borrow_mut().insert(kunci, nilai);
                }
            }

            Ok(Value::Number(lsn))
        }
        _ => Err(Galat::runtime("Fungsi 'wal_tulis' memerlukan objek LogWAL", span)),
    }
}

fn builtin_wal_checkpoint(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let wal_val = &args[0];
    match wal_val {
        Value::Map(m) => {
            let wal_map = m.borrow();
            let lsn = wal_map.get("lsn").cloned().unwrap_or(Value::Number(0.0));
            let state = wal_map.get("status_terakhir").cloned().unwrap_or_else(|| Value::Map(Rc::new(RefCell::new(HashMap::new()))));
            
            let mut res = HashMap::new();
            res.insert("checkpoint_lsn".to_string(), lsn);
            res.insert("data".to_string(), state);
            res.insert("waktu".to_string(), Value::Number(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs_f64()));
            Ok(Value::Map(Rc::new(RefCell::new(res))))
        }
        _ => Err(Galat::runtime("Fungsi 'wal_checkpoint' memerlukan objek LogWAL", span)),
    }
}

fn builtin_wal_pulihkan(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let wal_val = &args[0];
    match wal_val {
        Value::Map(m) => {
            let wal_map = m.borrow();
            let recovered_map = Rc::new(RefCell::new(HashMap::new()));

            if let Some(Value::Array(log_arr)) = wal_map.get("log") {
                let logs = log_arr.borrow();
                for entry in logs.iter() {
                    if let Value::Map(em) = entry {
                        let e = em.borrow();
                        let op = e.get("op").map(|v| v.to_string_repr()).unwrap_or_default();
                        let kunci = e.get("kunci").map(|v| v.to_string_repr()).unwrap_or_default();
                        let nilai = e.get("nilai").cloned().unwrap_or(Value::Nil);

                        if op == "DELETE" || op == "HAPUS" {
                            recovered_map.borrow_mut().remove(&kunci);
                        } else {
                            recovered_map.borrow_mut().insert(kunci, nilai);
                        }
                    }
                }
            }

            Ok(Value::Map(recovered_map))
        }
        _ => Err(Galat::runtime("Fungsi 'wal_pulihkan' memerlukan objek LogWAL", span)),
    }
}

fn builtin_wal_panjang(args: &[Value], span: &Span) -> Result<Value, Galat> {
    let wal_val = &args[0];
    match wal_val {
        Value::Map(m) => {
            let wal_map = m.borrow();
            if let Some(Value::Array(log_arr)) = wal_map.get("log") {
                Ok(Value::Number(log_arr.borrow().len() as f64))
            } else {
                Ok(Value::Number(0.0))
            }
        }
        _ => Err(Galat::runtime("Fungsi 'wal_panjang' memerlukan objek LogWAL", span)),
    }
}

