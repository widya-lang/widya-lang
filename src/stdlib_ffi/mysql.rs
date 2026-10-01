//! Native MySQL backend via `mysql` crate (sync pure-Rust driver).
//! Widya builtin exposed: `mysql_buka`, `mysql_jalan`, `mysql_kueri`, `mysql_tutup`.
//! Placeholder auto-conversion: Widya SQLite-style `?N` → native MySQL `?` (positional).

use crate::value::Value;
use mysql::prelude::*;
use mysql::{Pool, Value as MyVal};
use std::collections::HashMap;

pub struct MySqlDb {
    pub conn: Pool,
    pub url: String,
}

fn value_to_mysql(v: Value) -> MyVal {
    match v {
        Value::Nil => MyVal::NULL,
        Value::Bool(b) => if b { MyVal::Int(1) } else { MyVal::Int(0) },
        Value::Number(n) if n.fract() == 0.0 && n <= i64::MAX as f64 && n >= i64::MIN as f64 => {
            MyVal::Int(n as i64)
        },
        Value::Number(n) => MyVal::Double(n),
        Value::String(s) => MyVal::Bytes(s.into_bytes()),
        other => MyVal::Bytes(other.to_debug_repr().into_bytes()),
    }
}

fn mysql_row_to_map(columns: &[String], row: Vec<MyVal>) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    for (idx, col_name) in columns.iter().enumerate() {
        let v = match row.get(idx) {
            Some(MyVal::NULL) => Value::Nil,
            Some(MyVal::Bytes(b)) => {
                match String::from_utf8(b.clone()) {
                    Ok(s) => Value::String(s),
                    Err(_) => Value::String(String::from_utf8_lossy(b).to_string()),
                }
            },
            Some(MyVal::Int(i)) => Value::Number(*i as f64),
            Some(MyVal::UInt(u)) => Value::Number(*u as f64),
            Some(MyVal::Double(f)) => Value::Number(*f),
            Some(MyVal::Date(..)) | Some(MyVal::Time(..)) => {
                if let Some(mv) = row.get(idx) {
                    Value::String(format!("{:?}", mv))
                } else {
                    Value::Nil
                }
            },
            None => Value::Nil,
            _ => Value::Nil,
        };
        map.insert(col_name.clone(), v);
    }
    map
}

/// Widya → MySQL placeholder rewrite: `?12` → `?` (positional, sama order params array)
fn rewrite_sql_placeholders_mysql(sql: &str) -> String {
    let bytes = sql.as_bytes();
    let mut out = String::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'?' {
            out.push('?');
            i += 1;
            // Skip any trailing digits (e.g. ?123)
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
        } else {
            out.push(c as char);
            i += 1;
        }
    }
    out
}

impl MySqlDb {
    pub fn new(url: &str) -> Result<Self, String> {
        let pool = Pool::new(url)
            .map_err(|e| format!("MySQL Gagal Buka {}: {}", url, e))?;
        let mut conn = pool.get_conn()
            .map_err(|e| format!("MySQL Test Koneksi Gagal: {}", e))?;
        conn.query_drop("SELECT 1").map_err(|e| format!("MySQL SELECT 1 Gagal: {}", e))?;
        let _ = conn.query_drop("SET NAMES utf8mb4");
        let _ = conn.query_drop("SET SESSION sql_mode = 'STRICT_TRANS_TABLES,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO'");
        Ok(Self {
            conn: pool,
            url: url.to_string(),
        })
    }

    pub fn execute(&self, sql: &str, params: &[Value]) -> Result<(bool, i64), String> {
        let sql_rewritten = rewrite_sql_placeholders_mysql(sql);
        let converted: Vec<MyVal> = params.iter().cloned().map(value_to_mysql).collect();
        let converted_ref: Vec<&dyn mysql::prelude::ToValue> = converted.iter().map(|mv| mv as _).collect();
        let mut conn = self.conn.get_conn()
            .map_err(|e| format!("MySQL GetConn Execute Error: {}", e))?;
        let stmt = conn.prep(&sql_rewritten)
            .map_err(|e| format!("MySQL Prepare Execute Error: {}", e))?;
        let rows_affected = conn.exec_drop(&stmt, converted_ref.as_slice())
            .map(|()| conn.affected_rows())
            .map_err(|e| format!("MySQL Execute Error: {}", e))?;
        let last_id = conn.last_insert_id();
        Ok((true, if rows_affected > 0 { rows_affected as i64 } else { last_id as i64 }))
    }

    pub fn query(&self, sql: &str, params: &[Value]) -> Result<Vec<HashMap<String, Value>>, String> {
        let sql_rewritten = rewrite_sql_placeholders_mysql(sql);
        let converted: Vec<MyVal> = params.iter().cloned().map(value_to_mysql).collect();
        let converted_ref: Vec<&dyn mysql::prelude::ToValue> = converted.iter().map(|mv| mv as _).collect();
        let mut conn = self.conn.get_conn()
            .map_err(|e| format!("MySQL GetConn Query Error: {}", e))?;
        let stmt = conn.prep(&sql_rewritten)
            .map_err(|e| format!("MySQL Prepare Query Error: {}", e))?;
        let columns: Vec<String> = stmt.columns().iter().map(|c| c.name_str().to_string()).collect();
        let query_res = conn.exec_iter(&stmt, converted_ref.as_slice())
            .map_err(|e| format!("MySQL ExecIter Error: {}", e))?;
        let mut out: Vec<HashMap<String, Value>> = Vec::new();
        for row_result in query_res {
            match row_result {
                Ok(row) => {
                    let cells: Vec<MyVal> = row.unwrap();
                    out.push(mysql_row_to_map(&columns, cells));
                },
                Err(e) => return Err(format!("MySQL Row Query Error: {}", e)),
            }
        }
        Ok(out)
    }

    pub fn close(&self) -> bool {
        // Drop pool; best-effort
        true
    }
}
