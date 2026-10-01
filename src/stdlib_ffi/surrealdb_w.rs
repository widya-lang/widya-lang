//! SurrealDB-compatible wrapper (Embedded Single-file / In-memory).
//! Mobile-ready: 100% pure Rust — saat ini pakai in-memory HashMap dengan
//! persistence ke JSON file (lebih stabil untuk mobile cross-compile).
//! Kontrak interface SAMA SqliteDb: new(url)/execute/query/close.
//! Placeholder = `$N` (numbered dollar style).

use crate::value::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ValueSer {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RowData {
    data: HashMap<String, ValueSer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct SurrealStore {
    tables: HashMap<String, Vec<RowData>>,
    autoinc: HashMap<String, u64>,
}

fn vs_to_v(vs: &ValueSer) -> Value {
    match vs {
        ValueSer::Nil => Value::Nil,
        ValueSer::Bool(b) => Value::Bool(*b),
        ValueSer::Number(n) => Value::Number(*n),
        ValueSer::String(s) => Value::String(s.clone()),
    }
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

/// Placeholder rewrite: `?N` → `$N`
fn rewrite_ph(sql: &str) -> String {
    let bytes = sql.as_bytes();
    let mut out = String::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'?' && i + 1 < bytes.len() && bytes[i+1].is_ascii_digit() {
            out.push('$');
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                out.push(bytes[i] as char);
                i += 1;
            }
        } else {
            out.push(c as char);
            i += 1;
        }
    }
    out
}

fn norm(sql: &str) -> String {
    sql.split_whitespace().collect::<Vec<&str>>().join(" ").to_lowercase()
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

fn try_extract_where_id(params: &[Value]) -> Option<u64> {
    for p in params {
        if let Value::Number(n) = p {
            if *n >= 1.0 && n.fract() == 0.0 { return Some(*n as u64); }
        }
    }
    None
}

pub struct SurrealDbW {
    pub(crate) store: Arc<Mutex<SurrealStore>>,
    path: Option<String>,
}

impl SurrealDbW {
    fn persist(&self) {
        if let Some(p) = &self.path {
            if let Ok(guard) = self.store.lock() {
                let data = serde_json::to_vec(&*guard).unwrap_or_default();
                let _ = std::fs::write(p, data);
            }
        }
    }

    pub fn new(url: &str) -> Result<Self, String> {
        let (path, init_store) = if url.is_empty() || url == "mem" || url == "memory" || url == "kv-mem" || url == "surrealdb:///" {
            (None, SurrealStore { tables: HashMap::new(), autoinc: HashMap::new() })
        } else {
            let cleaned_raw = if let Some(p) = url.strip_prefix("surrealdb://") {
                p.strip_prefix('/').unwrap_or(p).to_string()
            } else if let Some(p) = url.strip_prefix("surrealdb:///") {
                p.to_string()
            } else {
                url.to_string()
            };
            let cleaned = normalize_path(&cleaned_raw);
            if let Some(parent) = Path::new(&cleaned).parent() {
                if !parent.as_os_str().is_empty() { let _ = std::fs::create_dir_all(parent); }
            }
            let loaded = if Path::new(&cleaned).exists() {
                match std::fs::read(&cleaned) {
                    Ok(bytes) => {
                        serde_json::from_slice::<SurrealStore>(&bytes)
                            .unwrap_or_else(|_| SurrealStore { tables: HashMap::new(), autoinc: HashMap::new() })
                    }
                    Err(_) => SurrealStore { tables: HashMap::new(), autoinc: HashMap::new() },
                }
            } else {
                SurrealStore { tables: HashMap::new(), autoinc: HashMap::new() }
            };
            (Some(cleaned), loaded)
        };
        Ok(Self {
            store: Arc::new(Mutex::new(init_store)),
            path,
        })
    }

    pub fn execute(&self, sql: &str, params: &[Value]) -> Result<(bool, i64), String> {
        let norm_str = norm(sql);
        let _rewritten = rewrite_ph(sql);
        let mut store = self.store.lock().map_err(|e| format!("SurrealDB lock poison: {}", e))?;
        if norm_str.starts_with("create table ") {
            let rest = norm_str.strip_prefix("create table ").unwrap_or("");
            let tn_raw = rest.split_whitespace().next().unwrap_or("");
            let table_name = clean_tn(tn_raw);
            store.tables.entry(table_name.clone()).or_insert_with(Vec::new);
            store.autoinc.entry(table_name).or_insert(1);
            drop(store);
            self.persist();
            return Ok((true, 0));
        }
        if norm_str.starts_with("insert into ") {
            let rest = norm_str.strip_prefix("insert into ").unwrap_or("");
            let mut parts = rest.splitn(2, "(");
            let table_name = clean_tn(parts.next().unwrap_or(""));
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
            let auto = store.autoinc.entry(table_name.to_string()).or_insert(1);
            let new_id = *auto;
            *auto += 1;
            if !data.contains_key("id") {
                data.insert("id".to_string(), ValueSer::Number(new_id as f64));
            }
            let tb = store.tables.entry(table_name.to_string()).or_insert_with(Vec::new);
            tb.push(RowData { data });
            drop(store);
            self.persist();
            return Ok((true, 1));
        }
        if norm_str.starts_with("update ") {
            let parts: Vec<&str> = norm_str.splitn(2, " set ").collect();
            if parts.len() != 2 { return Err("SurrealDB UPDATE perlu SET".to_string()); }
            let table_name = clean_tn(parts[0].strip_prefix("update ").unwrap_or(""));
            let rest2 = parts[1];
            let parts2: Vec<&str> = rest2.splitn(2, " where ").collect();
            let set_part = parts2[0];
            let _where = parts2.get(1).unwrap_or(&"");
            let set_items: Vec<(String, usize)> = set_part.split(',').map(|s| {
                let s = s.trim();
                let kv: Vec<&str> = s.splitn(2, '=').collect();
                let col = kv.get(0).unwrap_or(&"").trim().to_string();
                let ph = kv.get(1).unwrap_or(&"").trim().trim_start_matches('$').trim_start_matches('?').parse::<usize>().ok().unwrap_or(0);
                (col, ph)
            }).collect();
            let target_id = try_extract_where_id(params).unwrap_or(0);
            let mut affected = 0;
            if let Some(tb) = store.tables.get_mut(&table_name) {
                for row in tb.iter_mut() {
                    if target_id > 0 {
                        let matched = if let Some(ValueSer::Number(n)) = row.data.get("id") {
                            *n as u64 == target_id
                        } else { false };
                        if !matched { continue; }
                    }
                    for (col, ph) in &set_items {
                        if *ph >= 1 && *ph <= params.len() {
                            row.data.insert(col.clone(), v_to_vs(&params[ph-1]));
                        }
                    }
                    affected += 1;
                    if target_id > 0 { break; }
                }
            }
            drop(store);
            self.persist();
            return Ok((true, affected));
        }
        if norm_str.starts_with("delete from ") {
            let rest = norm_str.strip_prefix("delete from ").unwrap_or("");
            let parts: Vec<&str> = rest.splitn(2, " where ").collect();
            let table_name = clean_tn(parts[0]);
            let target_id = try_extract_where_id(params);
            let affected = if let Some(tb) = store.tables.get_mut(&table_name) {
                if let Some(tid) = target_id {
                    let before = tb.len();
                    tb.retain(|r| {
                        if let Some(ValueSer::Number(n)) = r.data.get("id") {
                            (*n as u64) != tid
                        } else { true }
                    });
                    (before - tb.len()) as i64
                } else {
                    let n = tb.len() as i64;
                    tb.clear();
                    n
                }
            } else { 0 };
            drop(store);
            self.persist();
            return Ok((true, affected));
        }
        if norm_str.starts_with("drop table ") {
            let tn = clean_tn(norm_str.strip_prefix("drop table ").unwrap_or(""));
            store.tables.remove(&tn);
            store.autoinc.remove(&tn);
            drop(store);
            self.persist();
            return Ok((true, 0));
        }
        Ok((true, 0))
    }

    pub fn query(&self, sql: &str, params: &[Value]) -> Result<Vec<HashMap<String, Value>>, String> {
        let norm_str = norm(sql);
        let store = self.store.lock().map_err(|e| format!("SurrealDB lock poison query: {}", e))?;
        if norm_str == "show tables" || norm_str.contains("sqlite_master") || norm_str.contains("pg_tables") || norm_str.contains("information_schema.tables") {
            let mut out: Vec<HashMap<String, Value>> = Vec::new();
            for tn in store.tables.keys() {
                let mut map = HashMap::new();
                map.insert("name".to_string(), Value::String(tn.clone()));
                map.insert("tablename".to_string(), Value::String(tn.clone()));
                out.push(map);
            }
            return Ok(out);
        }
        if norm_str.starts_with("select count") && norm_str.contains(" from ") {
            let after = norm_str.find(" from ").map(|i| &norm_str[i + " from ".len()..]).unwrap_or("");
            let table_name = clean_tn(after.split_whitespace().next().unwrap_or(""));
            let n = store.tables.get(&table_name).map(|t| t.len() as f64).unwrap_or(0.0);
            let mut map = HashMap::new();
            map.insert("count(*)".to_string(), Value::Number(n));
            map.insert("hitung".to_string(), Value::Number(n));
            return Ok(vec![map]);
        }
        if norm_str.starts_with("select ") && norm_str.contains(" from ") {
            let rest = norm_str.strip_prefix("select ").unwrap_or("");
            let from_idx = rest.find(" from ").unwrap_or(rest.len());
            let after_from = rest[from_idx + " from ".len()..].to_string();
            let mut parts = after_from.split_whitespace();
            let tn_raw = parts.next().unwrap_or("");
            let table_name = clean_tn(tn_raw);
            let tokens: Vec<&str> = norm_str.split_whitespace().collect();
            let mut limit: Option<usize> = None;
            let mut offset: Option<usize> = None;
            let mut i = 0;
            while i < tokens.len() {
                if tokens[i] == "limit" && i+1 < tokens.len() {
                    limit = tokens[i+1].parse::<usize>().ok();
                    i += 2;
                } else if tokens[i] == "offset" && i+1 < tokens.len() {
                    offset = tokens[i+1].parse::<usize>().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            let target_id = try_extract_where_id(params);
            let empty_vec = Vec::new();
            let tb = store.tables.get(&table_name).unwrap_or(&empty_vec);
            let mut out: Vec<HashMap<String, Value>> = Vec::new();
            let lim = limit.unwrap_or(usize::MAX);
            let off = offset.unwrap_or(0);
            let mut skipped = 0;
            for row in tb.iter() {
                if let Some(tid) = target_id {
                    let matched = row.data.get("id").and_then(|v| if let ValueSer::Number(n) = v { Some(*n as u64 == tid) } else { None }).unwrap_or(false);
                    if !matched { continue; }
                }
                if skipped < off { skipped += 1; continue; }
                if out.len() >= lim { break; }
                let mut map = HashMap::new();
                for (k, vs) in &row.data { map.insert(k.clone(), vs_to_v(vs)); }
                out.push(map);
            }
            return Ok(out);
        }
        Ok(Vec::new())
    }

    pub fn close(&self) -> bool {
        self.persist();
        true
    }
}
