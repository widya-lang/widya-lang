//! Redb wrapper (Embedded pure-Rust key-value store, single file).
//! Mobile-ready: 100% Rust tanpa FFI, crash-safe copy-on-write B-Tree.
//!
//! Redb tidak punya SQL engine, jadi kita implementasikan MINI SQL command yang
//! dipakai ORM Widya. Placeholder = `?N` style SQLite.
//! Kontrak interface SAMA SqliteDb: new(url)/execute/query/close.

use crate::value::Value;
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

const REDB_TBL_PREFIX: &str = "widya_";
const REDB_META_TBL: TableDefinition<&str, u64> = TableDefinition::new(concat!(env!("CARGO_PKG_NAME"), "_meta"));

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ValueSer {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct RedbRowPacked {
    data: HashMap<String, ValueSer>,
}
fn v_to_vs(v: &Value) -> ValueSer {
    match v {
        Value::Nil => ValueSer::Nil,
        Value::Bool(b) => ValueSer::Bool(*b),
        Value::Number(n) => ValueSer::Number(*n),
        Value::String(s) => ValueSer::String(s.clone()),
        other => ValueSer::String(other.to_debug_repr()),
    }
}
fn vs_to_v(vs: &ValueSer) -> Value {
    match vs {
        ValueSer::Nil => Value::Nil,
        ValueSer::Bool(b) => Value::Bool(*b),
        ValueSer::Number(n) => Value::Number(*n),
        ValueSer::String(s) => Value::String(s.clone()),
    }
}

fn norm_sql(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ").to_lowercase()
}

fn clean_tn(s: &str) -> String {
    s.trim().trim_matches(|c: char| c == ';' || c == ',' || c == '`' || c == '"' || c == '(' || c == ')').to_string()
}

fn normalize_path(p: &str) -> String {
    if let Some(stripped) = p.strip_prefix("/./") {
        format!("./{}", stripped)
    } else if let Some(stripped) = p.strip_prefix('/') {
        stripped.to_string()
    } else {
        p.to_string()
    }
}

fn tbl_def_user(name: &str) -> TableDefinition<'static, u64, &'static [u8]> {
    let full = format!("{}{}", REDB_TBL_PREFIX, name);
    TableDefinition::new(Box::leak(full.into_boxed_str()))
}

fn try_extract_id(params: &[Value]) -> Option<u64> {
    for p in params {
        if let Value::Number(n) = p {
            if *n >= 1.0 && n.fract() == 0.0 { return Some(*n as u64); }
        }
    }
    None
}

pub struct RedbW {
    pub db: Arc<Database>,
    pub path: String,
    pub autoincs: std::sync::Mutex<HashMap<String, Arc<AtomicU64>>>,
}

impl RedbW {
    pub fn new(url: &str) -> Result<Self, String> {
        let path_eff_raw = if url.is_empty() || url == "mem" || url == ":memory:" {
            let tmp = std::env::temp_dir().join(format!("redb_widya_{}.db", std::process::id()));
            tmp.to_string_lossy().to_string()
        } else if let Some(p) = url.strip_prefix("redb://") {
            p.strip_prefix('/').unwrap_or(p).to_string()
        } else if let Some(p) = url.strip_prefix("redb:///") {
            p.to_string()
        } else {
            url.to_string()
        };
        let path_eff = normalize_path(&path_eff_raw);
        if let Some(parent) = Path::new(&path_eff).parent() {
            if !parent.as_os_str().is_empty() { let _ = std::fs::create_dir_all(parent); }
        }
        let db = Database::create(&path_eff).map_err(|e| format!("Redb create [{}]: {}", path_eff, e))?;
        Ok(Self {
            db: Arc::new(db),
            path: path_eff,
            autoincs: std::sync::Mutex::new(HashMap::new()),
        })
    }

    fn register_autoinc(&self, tbl: &str, start_from: u64) {
        let mut g = self.autoincs.lock().unwrap();
        let entry = g.entry(tbl.to_string()).or_insert_with(|| Arc::new(AtomicU64::new(start_from.max(1))));
        if entry.load(Ordering::SeqCst) < start_from {
            entry.store(start_from.max(1), Ordering::SeqCst);
        }
    }

    pub fn execute(&self, sql: &str, params: &[Value]) -> Result<(bool, i64), String> {
        let norm = norm_sql(sql);
        if norm.starts_with("create table ") {
            let rest = norm.strip_prefix("create table ").unwrap_or("");
            let tn_raw = rest.split_whitespace().next().unwrap_or("");
            let tn = clean_tn(tn_raw);
            if tn.is_empty() { return Err("Redb CREATE: nama tabel kosong".to_string()); }
            let tx = self.db.begin_write().map_err(|e| format!("Redb tx create: {}", e))?;
            {
                let _ = tx.open_table(tbl_def_user(&tn));
                let mut meta = tx.open_table(REDB_META_TBL).map_err(|e| format!("Redb open meta: {}", e))?;
                let _ = meta.insert(tn.as_str(), 1u64);
            }
            tx.commit().map_err(|e| format!("Redb commit create: {}", e))?;
            self.register_autoinc(&tn, 1);
            return Ok((true, 0));
        }
        if norm.starts_with("insert into ") {
            let rest = norm.strip_prefix("insert into ").unwrap_or("");
            let mut parts = rest.splitn(2, "(");
            let tn = clean_tn(parts.next().unwrap_or(""));
            let rest2 = parts.next().unwrap_or("");
            let mut col_parts = rest2.splitn(2, ") values (");
            let cols_raw = col_parts.next().unwrap_or("");
            let columns: Vec<String> = cols_raw.split(',').map(|s| s.trim().trim_matches(|c| c=='`'||c=='"').to_string()).collect();
            let mut data: HashMap<String, ValueSer> = HashMap::new();
            for (i, col) in columns.iter().enumerate() {
                if i < params.len() {
                    data.insert(col.clone(), v_to_vs(&params[i]));
                }
            }
            // Pastikan autoinc registry + start from = 1
            self.register_autoinc(&tn, 1);
            let new_id = {
                let g = self.autoincs.lock().unwrap();
                g.get(&tn).unwrap().fetch_add(1, Ordering::SeqCst)
            };
            if !data.contains_key("id") {
                data.insert("id".to_string(), ValueSer::Number(new_id as f64));
            }
            let packed = RedbRowPacked { data };
            let payload = serde_cbor::to_vec(&packed).map_err(|e| format!("Redb CBOR ser: {}", e))?;
            let tx = self.db.begin_write().map_err(|e| format!("Redb tx insert: {}", e))?;
            {
                let mut h = tx.open_table(tbl_def_user(&tn)).map_err(|e| format!("Redb open insert: {}", e))?;
                h.insert(new_id, payload.as_slice()).map_err(|e| format!("Redb insert: {}", e))?;
            }
            tx.commit().map_err(|e| format!("Redb commit insert: {}", e))?;
            return Ok((true, 1));
        }
        if norm.starts_with("update ") {
            let parts: Vec<&str> = norm.splitn(2, " set ").collect();
            if parts.len() != 2 { return Err("Redb UPDATE butuh SET clause".to_string()); }
            let tn = clean_tn(parts[0].strip_prefix("update ").unwrap_or(""));
            let rest2 = parts[1];
            let parts2: Vec<&str> = rest2.splitn(2, " where ").collect();
            let set_part = parts2[0];
            let set_items: Vec<(String, usize)> = set_part.split(',').map(|s| {
                let s = s.trim();
                let kv: Vec<&str> = s.splitn(2, '=').collect();
                let col = kv.get(0).unwrap_or(&"").trim().to_string();
                let ph = kv.get(1).unwrap_or(&"").trim().trim_start_matches('?').parse::<usize>().ok().unwrap_or(0);
                (col, ph)
            }).collect();
            let target_id = try_extract_id(params).ok_or_else(|| "Redb UPDATE butuh WHERE id = ? (numeric)".to_string())?;
            let tx = self.db.begin_write().map_err(|e| format!("Redb tx update: {}", e))?;
            // STEP 1: Extract old bytes (read-only) — scope close → drop semua borrow
            let maybe_old_row: Option<Result<RedbRowPacked, String>> = {
                let h1 = tx.open_table(tbl_def_user(&tn)).map_err(|e| format!("Redb open update r: {}", e))?;
                let g1 = h1.get(target_id).map_err(|e| format!("Redb update get: {}", e))?;
                g1.map(|guard| {
                    let bytes = guard.value().to_vec();
                    serde_cbor::from_slice::<RedbRowPacked>(&bytes)
                        .map_err(|e| format!("Redb deser update: {}", e))
                })
            };
            let maybe_payload: Option<Vec<u8>> = match maybe_old_row {
                Some(Ok(mut row)) => {
                    for (col, ph) in set_items {
                        if ph >= 1 && ph <= params.len() {
                            row.data.insert(col, v_to_vs(&params[ph-1]));
                        }
                    }
                    let payload = serde_cbor::to_vec(&row)
                        .map_err(|e| format!("Redb CBOR ser update: {}", e))?;
                    Some(payload)
                }
                Some(Err(e)) => return Err(e),
                None => None,
            };
            let affected: i64 = if let Some(payload) = maybe_payload {
                let mut h2 = tx.open_table(tbl_def_user(&tn)).map_err(|e| format!("Redb open update w: {}", e))?;
                h2.insert(target_id, payload.as_slice()).map_err(|e| format!("Redb update write: {}", e))?;
                1
            } else { 0 };
            tx.commit().map_err(|e| format!("Redb commit update: {}", e))?;
            return Ok((true, affected));
        }
        if norm.starts_with("delete from ") {
            let rest = norm.strip_prefix("delete from ").unwrap_or("");
            let parts: Vec<&str> = rest.splitn(2, " where ").collect();
            let tn = clean_tn(parts[0]);
            let target_id = try_extract_id(params).ok_or_else(|| "Redb DELETE butuh WHERE id = ? (numeric)".to_string())?;
            let tx = self.db.begin_write().map_err(|e| format!("Redb tx delete: {}", e))?;
            let affected: i64 = {
                let mut h = tx.open_table(tbl_def_user(&tn)).map_err(|e| format!("Redb open delete: {}", e))?;
                let existed = h.remove(target_id).map_err(|e| format!("Redb delete: {}", e))?;
                if existed.is_some() { 1 } else { 0 }
            };
            tx.commit().map_err(|e| format!("Redb commit delete: {}", e))?;
            return Ok((true, affected));
        }
        if norm.starts_with("drop table ") {
            let tn = clean_tn(norm.strip_prefix("drop table ").unwrap_or(""));
            let tx = self.db.begin_write().map_err(|e| format!("Redb tx drop: {}", e))?;
            {
                let _ = tx.delete_table(tbl_def_user(&tn));
                if let Ok(mut meta) = tx.open_table(REDB_META_TBL) {
                    let _ = meta.remove(tn.as_str());
                }
            }
            tx.commit().map_err(|e| format!("Redb commit drop: {}", e))?;
            { let mut g = self.autoincs.lock().unwrap(); g.remove(&tn); }
            return Ok((true, 0));
        }
        Ok((true, 0))
    }

    pub fn query(&self, sql: &str, params: &[Value]) -> Result<Vec<HashMap<String, Value>>, String> {
        let norm = norm_sql(sql);
        if norm == "show tables" || norm.contains("sqlite_master") || norm.contains("pg_tables") || norm.contains("information_schema") {
            let tx = self.db.begin_read().map_err(|e| format!("Redb tx list tables: {}", e))?;
            let mut out: Vec<HashMap<String, Value>> = Vec::new();
            match tx.open_table(REDB_META_TBL) {
                Ok(meta) => {
                    let iter = meta.iter().map_err(|e| format!("Redb meta iter: {}", e))?;
                    for item in iter.flatten() {
                        let (k_guard, _v) = item;
                        let tn = k_guard.value().to_string();
                        let mut map = HashMap::new();
                        map.insert("name".to_string(), Value::String(tn.clone()));
                        map.insert("tablename".to_string(), Value::String(tn));
                        out.push(map);
                    }
                }
                Err(_) => {}
            }
            return Ok(out);
        }
        if norm.starts_with("select count") && norm.contains(" from ") {
            let rest = norm.strip_prefix("select count").unwrap_or("");
            let from_idx = rest.find(" from ");
            if let Some(_) = from_idx {
                let after = rest.splitn(2, " from ").nth(1).unwrap_or("").trim();
                let tn = clean_tn(after.split_whitespace().next().unwrap_or(""));
                let tx = self.db.begin_read().map_err(|e| format!("Redb tx count: {}", e))?;
                let h = tx.open_table(tbl_def_user(&tn)).map_err(|e| format!("Redb count open: {}", e))?;
                let iter = h.iter().map_err(|e| format!("Redb count iter: {}", e))?;
                let n = iter.flatten().count() as u64;
                let mut map = HashMap::new();
                map.insert("count(*)".to_string(), Value::Number(n as f64));
                map.insert("hitung".to_string(), Value::Number(n as f64));
                return Ok(vec![map]);
            }
        }
        if norm.starts_with("select ") && norm.contains(" from ") {
            let rest = norm.strip_prefix("select ").unwrap_or("");
            let from_idx = rest.find(" from ").unwrap_or(rest.len());
            let after_from = rest[from_idx + " from ".len()..].to_string();
            let mut parts = after_from.split_whitespace();
            let tn_raw = parts.next().unwrap_or("");
            let tn = clean_tn(tn_raw);
            // LIMIT/OFFSET
            let toks: Vec<&str> = norm.split_whitespace().collect();
            let mut limit: Option<usize> = None;
            let mut offset: Option<usize> = None;
            let mut i = 0;
            while i < toks.len() {
                if toks[i] == "limit" && i+1 < toks.len() {
                    limit = toks[i+1].parse::<usize>().ok();
                    i += 2;
                } else if toks[i] == "offset" && i+1 < toks.len() {
                    offset = toks[i+1].parse::<usize>().ok();
                    i += 2;
                } else { i += 1; }
            }
            let target_id = try_extract_id(params);
            let tx = self.db.begin_read().map_err(|e| format!("Redb tx select: {}", e))?;
            let h = tx.open_table(tbl_def_user(&tn)).map_err(|e| format!("Redb open table select [{}]: {}", tn, e))?;
            let mut out: Vec<HashMap<String, Value>> = Vec::new();
            let off_n = offset.unwrap_or(0);
            let lim_n = limit.unwrap_or(usize::MAX);
            if let Some(tid) = target_id {
                if let Ok(Some(v)) = h.get(tid) {
                    let bytes = v.value();
                    let r: RedbRowPacked = serde_cbor::from_slice(bytes)
                        .map_err(|e| format!("Redb deser row {}: {}", tid, e))?;
                    let mut map = HashMap::new();
                    for (k, vs) in r.data { map.insert(k, vs_to_v(&vs)); }
                    out.push(map);
                }
            } else {
                let iter_result = h.iter().map_err(|e| format!("Redb iter: {}", e))?;
                let mut skipped = 0usize;
                for item in iter_result.flatten() {
                    let (_k, v_guard) = item;
                    if skipped < off_n { skipped += 1; continue; }
                    if out.len() >= lim_n { break; }
                    let bytes = v_guard.value();
                    let r_res: Result<RedbRowPacked, _> = serde_cbor::from_slice(bytes);
                    if let Ok(r) = r_res {
                        let mut map = HashMap::new();
                        for (k, vs) in r.data { map.insert(k, vs_to_v(&vs)); }
                        out.push(map);
                    }
                }
            }
            return Ok(out);
        }
        Ok(Vec::new())
    }

    pub fn close(&self) -> bool {
        true
    }
}
