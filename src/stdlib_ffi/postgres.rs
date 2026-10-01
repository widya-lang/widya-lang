//! Native PostgreSQL backend via `postgres` crate (sync pure-Rust driver, NoTls default).
//! Widya builtin exposed: `pg_buka`, `pg_jalan`, `pg_kueri`, `pg_tutup`.
//! Placeholder auto-conversion: Widya SQLite-style `?N` → PostgreSQL `$N`.

use crate::value::Value;
use postgres::{Client, NoTls, Row, types::ToSql, types::Type, Column};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct PgDb {
    pub conn: Rc<RefCell<Client>>,
    pub url: String,
}

fn value_to_pg(v: &Value) -> Box<dyn ToSql + Sync + '_> {
    match v {
        Value::Nil => Box::new(Option::<String>::None),
        Value::Bool(b) => Box::new(*b),
        Value::Number(n) if n.fract() == 0.0 && *n <= i64::MAX as f64 && *n >= i64::MIN as f64 => {
            let i = *n as i64;
            Box::new(i)
        },
        Value::Number(n) => Box::new(*n),
        Value::String(s) => Box::new(s.as_str()),
        other => {
            let s = other.to_debug_repr();
            Box::new(s)
        }
    }
}

fn pg_row_to_value(row: &Row, idx: usize) -> Value {
    let col_type = match row.columns().get(idx) {
        Some(c) => c.type_().clone(),
        None => return Value::Nil,
    };
    let name = match row.columns().get(idx) {
        Some(c) => c.name(),
        None => return Value::Nil,
    };
    // Try common types using pattern match names
    let tname = col_type.name();
    match tname {
        "int4" | "int2" | "int8" => {
            if let Ok(v) = row.try_get::<_, Option<i64>>(idx) {
                match v {
                    Some(i) => return Value::Number(i as f64),
                    None => return Value::Nil,
                }
            }
        },
        "float4" | "float8" | "numeric" => {
            if let Ok(v) = row.try_get::<_, Option<f64>>(idx) {
                match v {
                    Some(f) => return Value::Number(f),
                    None => return Value::Nil,
                }
            }
        },
        "bool" => {
            if let Ok(v) = row.try_get::<_, Option<bool>>(idx) {
                match v {
                    Some(b) => return Value::Bool(b),
                    None => return Value::Nil,
                }
            }
        },
        "text" | "varchar" | "bpchar" | "name" | "unknown" | "json" | "jsonb" | "uuid" | "bytea" | "date" | "timestamp" | "timestamptz" | "time" | "interval" | "inet" | "cidr" | "macaddr" | "macaddr8" => {
            if let Ok(v) = row.try_get::<_, Option<String>>(idx) {
                match v {
                    Some(s) => return Value::String(s),
                    None => return Value::Nil,
                }
            }
            if let Ok(v) = row.try_get::<_, Option<Vec<u8>>>(idx) {
                match v {
                    Some(b) => return Value::String(String::from_utf8_lossy(&b).to_string()),
                    None => return Value::Nil,
                }
            }
        },
        _ => {},
    }
    // Fallback: format via Debug
    if let Ok(v) = row.try_get::<_, Option<String>>(idx) {
        match v {
            Some(s) => Value::String(s),
            None => Value::Nil,
        }
    } else {
        Value::Nil
    }
}

fn rewrite_sql_placeholders_pg(sql: &str) -> String {
    // Rewrite ?<DIGITS> → $<DIGITS>
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

impl PgDb {
    pub fn new(url: &str) -> Result<Self, String> {
        let mut client = Client::connect(url, NoTls)
            .map_err(|e| format!("PostgreSQL Gagal Buka {}: {}", url, e))?;
        let _rows = client.query_one("SELECT 1::int4 AS n", &[])
            .map_err(|e| format!("PostgreSQL SELECT 1 Gagal: {}", e))?;
        let _ = client.simple_query("SET NAMES 'UTF8'");
        let _ = client.simple_query("SET client_min_messages = WARNING");
        Ok(Self {
            conn: Rc::new(RefCell::new(client)),
            url: url.to_string(),
        })
    }

    pub fn execute(&self, sql: &str, params: &[Value]) -> Result<(bool, i64), String> {
        let sql_rewritten = rewrite_sql_placeholders_pg(sql);
        let pg_params: Vec<Box<dyn ToSql + Sync>> = params.iter().map(value_to_pg).collect();
        let pg_param_refs: Vec<&(dyn ToSql + Sync)> = pg_params.iter().map(|bp| bp.as_ref()).collect();
        let mut conn = self.conn.borrow_mut();
        let rows_affected = conn.execute(&sql_rewritten, pg_param_refs.as_slice())
            .map_err(|e| format!("PostgreSQL Execute Error: {}", e))?;
        Ok((true, rows_affected as i64))
    }

    pub fn query(&self, sql: &str, params: &[Value]) -> Result<Vec<HashMap<String, Value>>, String> {
        let sql_rewritten = rewrite_sql_placeholders_pg(sql);
        let pg_params: Vec<Box<dyn ToSql + Sync>> = params.iter().map(value_to_pg).collect();
        let pg_param_refs: Vec<&(dyn ToSql + Sync)> = pg_params.iter().map(|bp| bp.as_ref()).collect();
        let mut conn = self.conn.borrow_mut();
        let rows = conn.query(&sql_rewritten, pg_param_refs.as_slice())
            .map_err(|e| format!("PostgreSQL Query Error: {}", e))?;
        let column_names: Vec<String> = if rows.is_empty() {
            Vec::new()
        } else {
            rows[0].columns().iter().map(|c: &Column| c.name().to_string()).collect()
        };
        let mut out: Vec<HashMap<String, Value>> = Vec::new();
        for row in rows.iter() {
            let mut map = HashMap::new();
            for (idx, col_name) in column_names.iter().enumerate() {
                let v = pg_row_to_value(row, idx);
                map.insert(col_name.clone(), v);
            }
            out.push(map);
        }
        Ok(out)
    }

    pub fn close(&self) -> bool {
        // PostgreSQL client auto-close on Drop; we return true (best effort)
        true
    }
}
