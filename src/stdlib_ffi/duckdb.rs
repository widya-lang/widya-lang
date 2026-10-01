//! DuckDB embedded OLAP (analytical) — single file, bundled C++ via feature "bundled".
//! Cocok untuk mobile app yang butuh query laporan/data besar (aggregation/parquet export).
//! Kontrak interface SAMA DENGAN SqliteDb: new(url)/execute/query/close.
//! Placeholder DuckDB = `$1, $2, ...` style PostgreSQL.

use crate::value::Value;
use duckdb::{params_from_iter, Connection};
use std::collections::HashMap;
use std::rc::Rc;

pub struct DuckDb {
    pub conn: Rc<Connection>,
    pub url: String,
}

fn duckdb_value_to_widya(v: &duckdb::types::Value) -> Value {
    use duckdb::types::Value as DV;
    match v {
        DV::Null => Value::Nil,
        DV::Boolean(b) => Value::Bool(*b),
        DV::TinyInt(i) => Value::Number(*i as f64),
        DV::SmallInt(i) => Value::Number(*i as f64),
        DV::Int(i) => Value::Number(*i as f64),
        DV::BigInt(i) => Value::Number(*i as f64),
        DV::HugeInt(_i) => Value::Number(0.0),
        DV::UTinyInt(u) => Value::Number(*u as f64),
        DV::USmallInt(u) => Value::Number(*u as f64),
        DV::UInt(u) => Value::Number(*u as f64),
        DV::UBigInt(u) => Value::Number(*u as f64),
        DV::Float(f) => Value::Number(*f as f64),
        DV::Double(d) => Value::Number(*d),
        DV::Decimal(_d) => Value::Number(f64::NAN),
        DV::Timestamp(_unit, t) => Value::Number(*t as f64),
        DV::Date32(days) => Value::String(format!("DuckDB_Date32({})", days)),
        DV::Time64(_unit, val) => Value::String(format!("DuckDB_Time64({})", val)),
        DV::Text(s) => Value::String(s.clone()),
        DV::Blob(b) => Value::String(String::from_utf8_lossy(b).to_string()),
        _ => Value::String(format!("{:?}", v)),
    }
}

fn widya_value_to_duckdb_ref<'a>(v: &'a Value) -> Box<dyn duckdb::ToSql + 'a> {
    match v {
        Value::Nil => Box::new(duckdb::types::Null),
        Value::Bool(b) => Box::new(*b),
        Value::Number(n) if n.fract() == 0.0 && *n <= i64::MAX as f64 && *n >= i64::MIN as f64 => {
            let i = *n as i64;
            Box::new(i)
        }
        Value::Number(n) => Box::new(*n),
        Value::String(s) => Box::new(s.as_str()),
        other => {
            let s = other.to_debug_repr();
            Box::new(s)
        }
    }
}

/// DuckDB placeholder: `?12` → `$12` (same style PG numbered)
fn rewrite_placeholders_duckdb(sql: &str) -> String {
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

fn normalize_path(p: &str) -> String {
    if let Some(stripped) = p.strip_prefix("/./") {
        format!("./{}", stripped)
    } else if let Some(stripped) = p.strip_prefix('/') {
        stripped.to_string()
    } else {
        p.to_string()
    }
}

impl DuckDb {
    pub fn new(url: &str) -> Result<Self, String> {
        let conn_path_raw = if url.is_empty() || url == ":memory:" {
            ":memory:".to_string()
        } else if url == "duckdb:///:memory:" {
            ":memory:".to_string()
        } else if let Some(p) = url.strip_prefix("duckdb://") {
            p.strip_prefix('/').unwrap_or(p).to_string()
        } else if let Some(p) = url.strip_prefix("duckdb:///") {
            p.to_string()
        } else {
            url.to_string()
        };
        let conn_path = if conn_path_raw == ":memory:" { conn_path_raw } else { normalize_path(&conn_path_raw) };
        if conn_path != ":memory:" {
            if let Some(parent) = std::path::Path::new(&conn_path).parent() {
                if !parent.as_os_str().is_empty() { let _ = std::fs::create_dir_all(parent); }
            }
        }
        let conn = if conn_path == ":memory:" {
            Connection::open_in_memory()
        } else {
            Connection::open(&conn_path)
        }.map_err(|e| format!("DuckDB Gagal Buka {}: {}", conn_path, e))?;
        let _ = conn.execute_batch("SET memory_limit='512MB';");
        Ok(Self {
            conn: Rc::new(conn),
            url: conn_path,
        })
    }

    pub fn execute(&self, sql: &str, params: &[Value]) -> Result<(bool, i64), String> {
        let sql_rewritten = rewrite_placeholders_duckdb(sql);
        let rows_affected: usize = if params.is_empty() {
            self.conn.execute_batch(&sql_rewritten)
                .map_err(|e| format!("DuckDB ExecuteBatch (tanpa params) Error: {}", e))?;
            0
        } else {
            let owned_params: Vec<Box<dyn duckdb::ToSql>> = params.iter()
                .map(widya_value_to_duckdb_ref).collect();
            let refs: Vec<&dyn duckdb::ToSql> = owned_params.iter().map(|b| b.as_ref()).collect();
            let mut stmt = self.conn.prepare(&sql_rewritten)
                .map_err(|e| format!("DuckDB Prepare Execute Error: {}", e))?;
            stmt.execute(params_from_iter(refs.iter().copied()))
                .map_err(|e| format!("DuckDB Execute Error: {}", e))?
        };
        Ok((true, rows_affected as i64))
    }

    pub fn query(&self, sql: &str, params: &[Value]) -> Result<Vec<HashMap<String, Value>>, String> {
        let sql_rewritten = rewrite_placeholders_duckdb(sql);
        let owned_params: Vec<Box<dyn duckdb::ToSql>> = params.iter()
            .map(widya_value_to_duckdb_ref).collect();
        let refs: Vec<&dyn duckdb::ToSql> = owned_params.iter().map(|b| b.as_ref()).collect();

        let sql_trim = sql_rewritten.trim_end();
        let (sql_core, trailing_semi) = if sql_trim.ends_with(';') {
            (&sql_rewritten[..sql_trim.len() - 1], true)
        } else {
            (sql_rewritten.as_str(), false)
        };
        let sql_lim0 = format!("{} LIMIT 0{}", sql_core, if trailing_semi { ";" } else { "" });
        let column_names: Vec<String> = {
            let mut stmt0 = self.conn.prepare(&sql_lim0)
                .map_err(|e| format!("DuckDB Prepare Lim0 Error: {}", e))?;
            let _ = if params.is_empty() {
                stmt0.query(())
            } else {
                stmt0.query(params_from_iter(refs.iter().copied()))
            }.map_err(|e| format!("DuckDB Execute Lim0 Error: {}", e))?;
            stmt0.column_names()
        };
        let n_cols_pre = column_names.len();

        let mut stmt = self.conn.prepare(&sql_rewritten)
            .map_err(|e| format!("DuckDB Prepare Query Error: {}", e))?;
        let mut out: Vec<HashMap<String, Value>> = Vec::new();
        let mut result = if params.is_empty() {
            stmt.query(()).map_err(|e| format!("DuckDB Query (unit) Error: {}", e))?
        } else {
            stmt.query(params_from_iter(refs.iter().copied()))
                .map_err(|e| format!("DuckDB Query Error: {}", e))?
        };
        while let Some(row) = result.next().map_err(|e| format!("DuckDB Row Iter Error: {}", e))? {
            let mut map = HashMap::with_capacity(n_cols_pre);
            for idx in 0..n_cols_pre {
                let v: duckdb::types::Value = row.get(idx)
                    .unwrap_or(duckdb::types::Value::Null);
                let col_name = column_names.get(idx).cloned().unwrap_or_else(|| format!("col{}", idx));
                map.insert(col_name, duckdb_value_to_widya(&v));
            }
            out.push(map);
        }
        Ok(out)
    }

    pub fn close(&self) -> bool {
        true
    }
}
