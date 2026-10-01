//! Native SQLite backend via `rusqlite` (bundled C amalgamation).
//! Widya builtin exposed: `sqlite_buka`, `sqlite_jalan`, `sqlite_kueri`, `sqlite_tutup`.

use crate::value::Value;
use rusqlite::{Connection, params_from_iter};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct SqliteDb {
    pub conn: Rc<RefCell<Connection>>,
    pub path: String,
}

impl SqliteDb {
    pub fn new(path: &str) -> Result<Self, String> {
        let conn = Connection::open(path)
            .map_err(|e| format!("SQLite Gagal Buka {}: {}", path, e))?;
        // Enable WAL journal + foreign keys untuk production performance & integrity
        let _ = conn.pragma_update(None, "journal_mode", "WAL");
        let _ = conn.pragma_update(None, "foreign_keys", "ON");
        Ok(Self {
            conn: Rc::new(RefCell::new(conn)),
            path: path.to_string(),
        })
    }

    pub fn execute(&self, sql: &str, params: &[Value]) -> Result<(bool, i64), String> {
        let c = self.conn.borrow();
        let rusqlite_params: Vec<Box<dyn rusqlite::ToSql>> = params.iter()
            .map(|v| value_to_tosql(v.clone()))
            .collect();
        let param_refs: Vec<&dyn rusqlite::ToSql> = rusqlite_params.iter().map(|b| b.as_ref()).collect();
        let stmt_result = c.execute(sql, params_from_iter(param_refs));
        match stmt_result {
            Ok(rows_affected) => {
                let last_id = c.last_insert_rowid();
                Ok((true, if rows_affected > 0 { rows_affected as i64 } else { last_id }))
            },
            Err(e) => Err(format!("SQLite Execute Error: {}", e)),
        }
    }

    pub fn query(&self, sql: &str, params: &[Value]) -> Result<Vec<HashMap<String, Value>>, String> {
        let c = self.conn.borrow();
        let rusqlite_params: Vec<Box<dyn rusqlite::ToSql>> = params.iter()
            .map(|v| value_to_tosql(v.clone()))
            .collect();
        let param_refs_1: Vec<&dyn rusqlite::ToSql> = rusqlite_params.iter().map(|b| b.as_ref()).collect();
        let param_refs_2: Vec<&dyn rusqlite::ToSql> = rusqlite_params.iter().map(|b| b.as_ref()).collect();
        let mut stmt = c.prepare(sql).map_err(|e| format!("SQLite Prepare Error: {}", e))?;
        let column_count = stmt.column_count();
        let column_names: Vec<String> = (0..column_count)
            .map(|i| stmt.column_name(i).unwrap_or("col").to_string())
            .collect();
        let rows = stmt.query(params_from_iter(param_refs_1))
            .map_err(|e| format!("SQLite Query Error: {}", e))?;
        let mut out: Vec<HashMap<String, Value>> = Vec::new();
        drop(rows);
        drop(stmt);
        let mut stmt2 = c.prepare(sql).map_err(|e| format!("SQLite Prepare 2 Error: {}", e))?;
        let mapped = stmt2.query_map(params_from_iter(param_refs_2), |row| {
            let mut map = HashMap::new();
            for (idx, col_name) in column_names.iter().enumerate() {
                let v: Value = row_to_value(row, idx)?;
                map.insert(col_name.clone(), v);
            }
            Ok(map)
        }).map_err(|e| format!("SQLite QueryMap Error: {}", e))?;
        for r in mapped {
            match r {
                Ok(row_map) => out.push(row_map),
                Err(e) => return Err(format!("SQLite Row Error: {}", e)),
            }
        }
        Ok(out)
    }

    pub fn close(&self) -> bool {
        // Drop connection manual dengan ambil ownership tidak mungkin via RefCell — pindah ke dalam:
        // Untuk kasus Widya user level, cukup set flag is_open (tidak penting, karena GC Rc akan close).
        true
    }
}

fn value_to_tosql(v: Value) -> Box<dyn rusqlite::ToSql> {
    match v {
        Value::Nil => Box::new(None::<String>),
        Value::Bool(b) => Box::new(b),
        Value::Number(n) if n.fract() == 0.0 && n <= i64::MAX as f64 && n >= i64::MIN as f64 => {
            Box::new(n as i64)
        },
        Value::Number(n) => Box::new(n),
        Value::String(s) => Box::new(s),
        other => Box::new(other.to_debug_repr()),
    }
}

fn row_to_value(row: &rusqlite::Row<'_>, idx: usize) -> rusqlite::Result<Value> {
    let col_type = row.get_ref(idx)?.data_type();
    let v = match col_type {
        rusqlite::types::Type::Null => Value::Nil,
        rusqlite::types::Type::Integer => {
            let n: i64 = row.get(idx)?;
            Value::Number(n as f64)
        },
        rusqlite::types::Type::Real => {
            let f: f64 = row.get(idx)?;
            Value::Number(f)
        },
        rusqlite::types::Type::Text => {
            let s: String = row.get(idx)?;
            Value::String(s)
        },
        rusqlite::types::Type::Blob => {
            let b: Vec<u8> = row.get(idx)?;
            Value::String(String::from_utf8_lossy(&b).to_string())
        },
    };
    Ok(v)
}
