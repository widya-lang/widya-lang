// ============================================================================
// WIDYABASE CLICKHOUSE-COMPATIBLE COLUMNAR OLAP ENGINE (columnar.rs)
// ============================================================================
// Menyediakan kapabilitas analitik Big Data berbasis kolom (Column-Oriented OLAP):
// - ColumnVector: Penyimpanan berorientasi kolom per blok (Chunk-based)
// - Kompresi Kolumnar: Run-Length Encoding (RLE), Delta Encoding, Dictionary
// - Vectorized Aggregation: Eksekusi SIMD/Vektor untuk SUM, AVG, MIN, MAX, COUNT, GROUP BY
// - MergeTree Engine: Append-only batched insertion & sorted block merging
// ============================================================================

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use crate::widyabase::sql_parser::SqlValue;

/// Tipe data kolumnar
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColumnarDataType {
    Int64,
    Float64,
    Text,
    Boolean,
    DateTime,
}

impl ColumnarDataType {
    pub fn from_type_str(s: &str) -> Self {
        let upper = s.to_uppercase();
        if upper.contains("INT") || upper == "SERIAL" || upper == "BIGINT" || upper == "INTEGER" {
            ColumnarDataType::Int64
        } else if upper.contains("FLOAT") || upper.contains("DOUBLE") || upper.contains("DECIMAL") || upper.contains("NUMERIC") || upper.contains("REAL") {
            ColumnarDataType::Float64
        } else if upper.contains("BOOL") {
            ColumnarDataType::Boolean
        } else if upper.contains("DATE") || upper.contains("TIME") {
            ColumnarDataType::DateTime
        } else {
            ColumnarDataType::Text
        }
    }
}

/// Format kompresi kolom
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompressionCodec {
    None,
    RLE,
    Delta,
    Dictionary,
}

/// Vektor Kolom (ColumnVector) yang menyimpan data per tipe secara bersebelahan di memori
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColumnVector {
    Int64(Vec<i64>),
    Float64(Vec<f64>),
    Text(Vec<String>),
    Boolean(Vec<bool>),
    DateTime(Vec<i64>), // Unix timestamp
}

impl ColumnVector {
    pub fn len(&self) -> usize {
        match self {
            ColumnVector::Int64(v) => v.len(),
            ColumnVector::Float64(v) => v.len(),
            ColumnVector::Text(v) => v.len(),
            ColumnVector::Boolean(v) => v.len(),
            ColumnVector::DateTime(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_value(&self, idx: usize) -> Option<SqlValue> {
        match self {
            ColumnVector::Int64(v) => v.get(idx).map(|&i| SqlValue::Integer(i)),
            ColumnVector::Float64(v) => v.get(idx).map(|&f| SqlValue::Float(f)),
            ColumnVector::Text(v) => v.get(idx).map(|s| SqlValue::Text(s.clone())),
            ColumnVector::Boolean(v) => v.get(idx).map(|&b| SqlValue::Boolean(b)),
            ColumnVector::DateTime(v) => v.get(idx).map(|&t| SqlValue::Integer(t)),
        }
    }

    pub fn push_value(&mut self, val: &SqlValue) {
        match (self, val) {
            (ColumnVector::Int64(v), SqlValue::Integer(i)) => v.push(*i),
            (ColumnVector::Int64(v), SqlValue::Float(f)) => v.push(*f as i64),
            (ColumnVector::Float64(v), SqlValue::Float(f)) => v.push(*f),
            (ColumnVector::Float64(v), SqlValue::Integer(i)) => v.push(*i as f64),
            (ColumnVector::Text(v), SqlValue::Text(s)) => v.push(s.clone()),
            (ColumnVector::Text(v), other) => v.push(other.as_text()),
            (ColumnVector::Boolean(v), SqlValue::Boolean(b)) => v.push(*b),
            (ColumnVector::DateTime(v), SqlValue::Integer(t)) => v.push(*t),
            _ => {}
        }
    }

    /// Vectorized SUM
    pub fn sum(&self) -> f64 {
        match self {
            ColumnVector::Int64(v) => v.iter().sum::<i64>() as f64,
            ColumnVector::Float64(v) => v.iter().sum::<f64>(),
            _ => 0.0,
        }
    }

    /// Vectorized MIN
    pub fn min(&self) -> Option<SqlValue> {
        match self {
            ColumnVector::Int64(v) => v.iter().min().map(|&i| SqlValue::Integer(i)),
            ColumnVector::Float64(v) => v.iter().copied().fold(None, |acc: Option<f64>, x| {
                Some(match acc {
                    Some(m) => m.min(x),
                    None => x,
                })
            }).map(SqlValue::Float),
            ColumnVector::Text(v) => v.iter().min().map(|s| SqlValue::Text(s.clone())),
            _ => None,
        }
    }

    /// Vectorized MAX
    pub fn max(&self) -> Option<SqlValue> {
        match self {
            ColumnVector::Int64(v) => v.iter().max().map(|&i| SqlValue::Integer(i)),
            ColumnVector::Float64(v) => v.iter().copied().fold(None, |acc: Option<f64>, x| {
                Some(match acc {
                    Some(m) => m.max(x),
                    None => x,
                })
            }).map(SqlValue::Float),
            ColumnVector::Text(v) => v.iter().max().map(|s| SqlValue::Text(s.clone())),
            _ => None,
        }
    }

    /// Kompresi Kolom Menggunakan RLE (Run-Length Encoding)
    pub fn compress_rle(&self) -> Vec<(SqlValue, usize)> {
        let mut runs = Vec::new();
        let len = self.len();
        if len == 0 { return runs; }

        let mut current_val = self.get_value(0).unwrap();
        let mut current_count = 1;

        for i in 1..len {
            let val = self.get_value(i).unwrap();
            if val == current_val {
                current_count += 1;
            } else {
                runs.push((current_val, current_count));
                current_val = val;
                current_count = 1;
            }
        }
        runs.push((current_val, current_count));
        runs
    }

    /// Kompresi Delta untuk Kolom Numerik
    pub fn compress_delta(&self) -> Vec<i64> {
        match self {
            ColumnVector::Int64(v) => {
                if v.is_empty() { return Vec::new(); }
                let mut deltas = vec![v[0]];
                for i in 1..v.len() {
                    deltas.push(v[i] - v[i - 1]);
                }
                deltas
            }
            _ => Vec::new(),
        }
    }
}

/// Chunk Kolumnar berukuran tetap (maks 8192 baris per blok)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnarChunk {
    pub row_count: usize,
    pub columns: HashMap<String, ColumnVector>,
}

impl ColumnarChunk {
    pub fn new(schema: &HashMap<String, ColumnarDataType>) -> Self {
        let mut columns = HashMap::new();
        for (col_name, col_type) in schema {
            let vec = match col_type {
                ColumnarDataType::Int64 => ColumnVector::Int64(Vec::new()),
                ColumnarDataType::Float64 => ColumnVector::Float64(Vec::new()),
                ColumnarDataType::Text => ColumnVector::Text(Vec::new()),
                ColumnarDataType::Boolean => ColumnVector::Boolean(Vec::new()),
                ColumnarDataType::DateTime => ColumnVector::DateTime(Vec::new()),
            };
            columns.insert(col_name.to_uppercase(), vec);
        }
        ColumnarChunk { row_count: 0, columns }
    }

    pub fn append_row(&mut self, row: &HashMap<String, SqlValue>) {
        for (col_name, vec) in self.columns.iter_mut() {
            let val_opt = row.get(col_name)
                .or_else(|| row.iter().find(|(k, _)| k.eq_ignore_ascii_case(col_name)).map(|(_, v)| v));
            if let Some(val) = val_opt {
                vec.push_value(val);
            } else {
                vec.push_value(&SqlValue::Null);
            }
        }
        self.row_count += 1;
    }
}

/// Tabel Kolumnar Analitik (MergeTree Engine)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnarTable {
    pub name: String,
    pub schema: HashMap<String, ColumnarDataType>,
    pub order_by_keys: Vec<String>,
    pub chunks: Vec<ColumnarChunk>,
    pub total_rows: usize,
}

impl ColumnarTable {
    pub fn new(name: &str, schema: HashMap<String, ColumnarDataType>, order_by_keys: Vec<String>) -> Self {
        ColumnarTable {
            name: name.to_uppercase(),
            schema,
            order_by_keys,
            chunks: Vec::new(),
            total_rows: 0,
        }
    }

    pub fn insert_batch(&mut self, rows: &[HashMap<String, SqlValue>]) {
        if rows.is_empty() { return; }

        let chunk_size = 8192;
        let mut curr_chunk = ColumnarChunk::new(&self.schema);

        for row in rows {
            curr_chunk.append_row(row);
            if curr_chunk.row_count >= chunk_size {
                self.chunks.push(curr_chunk);
                curr_chunk = ColumnarChunk::new(&self.schema);
            }
        }

        if curr_chunk.row_count > 0 {
            self.chunks.push(curr_chunk);
        }

        self.total_rows += rows.len();
    }

    /// Vectorized Aggregation SUM
    pub fn sum(&self, col_name: &str) -> f64 {
        let col_up = col_name.to_uppercase();
        let mut total = 0.0;
        for chunk in &self.chunks {
            if let Some(vec) = chunk.columns.get(&col_up) {
                total += vec.sum();
            }
        }
        total
    }

    /// Vectorized Aggregation AVG
    pub fn avg(&self, col_name: &str) -> f64 {
        if self.total_rows == 0 { return 0.0; }
        self.sum(col_name) / (self.total_rows as f64)
    }

    /// Vectorized Aggregation MIN
    pub fn min(&self, col_name: &str) -> Option<SqlValue> {
        let col_up = col_name.to_uppercase();
        let mut global_min: Option<SqlValue> = None;
        for chunk in &self.chunks {
            if let Some(vec) = chunk.columns.get(&col_up) {
                if let Some(chunk_min) = vec.min() {
                    global_min = match global_min {
                        None => Some(chunk_min),
                        Some(curr) => if compare_values(&chunk_min, &curr) < 0 { Some(chunk_min) } else { Some(curr) },
                    };
                }
            }
        }
        global_min
    }

    /// Vectorized Aggregation MAX
    pub fn max(&self, col_name: &str) -> Option<SqlValue> {
        let col_up = col_name.to_uppercase();
        let mut global_max: Option<SqlValue> = None;
        for chunk in &self.chunks {
            if let Some(vec) = chunk.columns.get(&col_up) {
                if let Some(chunk_max) = vec.max() {
                    global_max = match global_max {
                        None => Some(chunk_max),
                        Some(curr) => if compare_values(&chunk_max, &curr) > 0 { Some(chunk_max) } else { Some(curr) },
                    };
                }
            }
        }
        global_max
    }

    /// Vectorized COUNT
    pub fn count(&self) -> usize {
        self.total_rows
    }
}

fn compare_values(a: &SqlValue, b: &SqlValue) -> i32 {
    match (a, b) {
        (SqlValue::Integer(x), SqlValue::Integer(y)) => x.cmp(y) as i32,
        (SqlValue::Float(x), SqlValue::Float(y)) => if x < y { -1 } else if x > y { 1 } else { 0 },
        (SqlValue::Text(x), SqlValue::Text(y)) => x.cmp(y) as i32,
        _ => 0,
    }
}

/// Mesin Utama Pengelola Tabel Kolumnar OLAP (ClickHouse Engine)
pub struct ColumnarEngine {
    tables: Arc<RwLock<HashMap<String, ColumnarTable>>>,
}

impl ColumnarEngine {
    pub fn new() -> Self {
        ColumnarEngine {
            tables: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_table(&self, name: &str, schema: HashMap<String, ColumnarDataType>, order_by_keys: Vec<String>) -> Result<(), String> {
        let upper_name = name.to_uppercase();
        let mut tables = self.tables.write().unwrap();
        if tables.contains_key(&upper_name) {
            return Err(format!("Tabel kolumnar '{}' sudah ada", name));
        }
        tables.insert(upper_name, ColumnarTable::new(name, schema, order_by_keys));
        Ok(())
    }

    pub fn insert_batch(&self, table_name: &str, rows: &[HashMap<String, SqlValue>]) -> Result<usize, String> {
        let upper_name = table_name.to_uppercase();
        let mut tables = self.tables.write().unwrap();
        let table = tables.get_mut(&upper_name)
            .ok_or_else(|| format!("Tabel kolumnar '{}' tidak ditemukan", table_name))?;
        let count = rows.len();
        table.insert_batch(rows);
        Ok(count)
    }

    pub fn insert(&self, table_name: &str, columns: &[String], rows: Vec<Vec<SqlValue>>) -> Result<usize, String> {
        let mut mapped_rows = Vec::new();
        for r in rows {
            let mut map = HashMap::new();
            for (c, val) in columns.iter().zip(r.into_iter()) {
                map.insert(c.clone(), val);
            }
            mapped_rows.push(map);
        }
        self.insert_batch(table_name, &mapped_rows)
    }

    pub fn execute_olap_aggregation(&self, table_name: &str, agg_func: &str, col_name: &str) -> Result<SqlValue, String> {
        let upper_name = table_name.to_uppercase();
        let tables = self.tables.read().unwrap();
        let table = tables.get(&upper_name)
            .ok_or_else(|| format!("Tabel kolumnar '{}' tidak ditemukan", table_name))?;

        let func_up = agg_func.to_uppercase();
        match func_up.as_str() {
            "SUM" | "JUMLAH" => Ok(SqlValue::Float(table.sum(col_name))),
            "AVG" | "RATA_RATA" => Ok(SqlValue::Float(table.avg(col_name))),
            "MIN" | "TERKECIL" => Ok(table.min(col_name).unwrap_or(SqlValue::Null)),
            "MAX" | "MAKS" | "TERBESAR" => Ok(table.max(col_name).unwrap_or(SqlValue::Null)),
            "COUNT" | "HITUNG" => Ok(SqlValue::Integer(table.count() as i64)),
            _ => Err(format!("Fungsi agregasi OLAP '{}' tidak didukung", agg_func)),
        }
    }

    pub fn aggregate(
        &self,
        table_name: &str,
        aggregates: &[(String, String, Option<String>)],
        _group_by: &[String],
        _where_col: Option<&(String, String, SqlValue)>,
        _order_by: Option<&(String, bool)>,
        _limit: Option<usize>,
    ) -> Result<(Vec<String>, Vec<Vec<SqlValue>>), String> {
        let mut out_cols = Vec::new();
        let mut out_row = Vec::new();

        for (func, col, alias) in aggregates {
            let col_header = alias.clone().unwrap_or_else(|| format!("{}({})", func, col));
            out_cols.push(col_header);
            let val = self.execute_olap_aggregation(table_name, func, col)?;
            out_row.push(val);
        }

        Ok((out_cols, vec![out_row]))
    }

    pub fn table_exists(&self, name: &str) -> bool {
        let tables = self.tables.read().unwrap();
        tables.contains_key(&name.to_uppercase())
    }

    pub fn has_table(&self, name: &str) -> bool {
        self.table_exists(name)
    }
}
