// ============================================================================
// WIDYABASE QUERY PLANNER & EXECUTION ENGINE (planner.rs)
// ============================================================================
// Mengeksekusi AST SQL dwibahasa ke atas Storage Engine dan Buffer Pool.
// Menjamin isolasi transaksi MVCC dan logging perubahan ke berkas WAL.
// ============================================================================

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::btree::{BTreeIndex, IndexKey, RecordPointer};
use crate::cache::CacheEngine;
use crate::cdc::{CdcOp, CdcRecord, CdcStreamer};
use crate::columnar::ColumnarEngine;
use crate::multimodal::VectorEngine;
use crate::mvcc::{MvccHeader, Snapshot, TransactionManager};
use crate::sql_parser::{
    BinaryOp, CascadeAction, ColumnDef, CteDefinition, ForeignKeyConstraint, JoinClause, JoinType, OnConflict, OrderByClause,
    SqlExpr, SqlStatement, SqlValue,
};
use crate::storage::{PageType, StorageEngine};
use crate::wal::{WalManager, WalOpType};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<(String, String)>, // (nama, tipe)
    #[serde(default)]
    pub column_defs: Vec<ColumnDef>,
    pub primary_key: Option<String>,
    #[serde(default)]
    pub foreign_keys: Vec<ForeignKeyConstraint>,
    pub root_page_id: u32,
    #[serde(default)]
    pub serial_counter: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Row {
    pub fields: HashMap<String, SqlValue>,
}

/// Statistik distribusi data kolom untuk Cost-Based Optimizer (CBO)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ColumnStats {
    pub column_name: String,
    pub null_count: usize,
    pub distinct_count: usize,
    pub min_val: Option<SqlValue>,
    pub max_val: Option<SqlValue>,
    pub most_common_vals: Vec<(SqlValue, usize)>, // Nilai paling sering muncul beserta frekuensinya (MCV)
    pub correlation: f64,                         // Korelasi urutan fisik vs logika (-1.0 .. 1.0)
}

/// Statistik tingkat tabel untuk estimasi I/O page dan selektivitas
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableStats {
    pub table_name: String,
    pub row_count: usize,
    pub page_count: usize,
    pub avg_row_size: usize,
    pub last_analyzed: u64,
    pub columns: HashMap<String, ColumnStats>,
}

/// Model biaya operasi I/O dan CPU untuk Cost-Based Optimizer (PostgreSQL-compatible)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CostModel {
    pub seq_page_cost: f64,       // Biaya pembacaan sekuensial halaman disk (1.0)
    pub random_page_cost: f64,    // Biaya pembacaan acak indeks disk (4.0)
    pub cpu_tuple_cost: f64,      // Biaya pemrosesan baris CPU (0.01)
    pub cpu_index_tuple_cost: f64,// Biaya traversal indeks CPU (0.005)
    pub cpu_operator_cost: f64,   // Biaya evaluasi predikat WHERE CPU (0.0025)
}

impl Default for CostModel {
    fn default() -> Self {
        Self {
            seq_page_cost: 1.0,
            random_page_cost: 1.5,
            cpu_tuple_cost: 0.01,
            cpu_index_tuple_cost: 0.005,
            cpu_operator_cost: 0.0025,
        }
    }
}

#[derive(Debug, Clone)]
pub enum QueryResult {
    Message(String),
    Rows {
        columns: Vec<String>,
        rows: Vec<Vec<SqlValue>>,
        affected: usize,
    },
}

pub struct ExecutionEngine {
    pub storage: Arc<StorageEngine>,
    pub wal: Arc<WalManager>,
    pub tx_mgr: Arc<TransactionManager>,
    pub catalog: Arc<RwLock<HashMap<String, TableSchema>>>,
    pub in_memory_tables: Arc<RwLock<HashMap<String, Vec<(MvccHeader, Row)>>>>,
    pub indices: Arc<RwLock<HashMap<String, BTreeIndex>>>,
    pub secondary_indexes: Arc<RwLock<HashMap<String, (String, String)>>>,
    pub views: Arc<RwLock<HashMap<String, SqlStatement>>>,
    pub materialized_views: Arc<RwLock<HashMap<String, SqlStatement>>>,
    pub savepoints: Arc<RwLock<HashMap<(u64, String), HashMap<String, Vec<(MvccHeader, Row)>>>>>,
    pub cache_engine: Arc<CacheEngine>,
    pub columnar_engine: Arc<ColumnarEngine>,
    pub table_stats: Arc<RwLock<HashMap<String, TableStats>>>,
    pub cdc: Arc<CdcStreamer>,
}

impl ExecutionEngine {
    pub fn new(storage: Arc<StorageEngine>, wal: Arc<WalManager>) -> Self {
        let catalog = Arc::new(RwLock::new(HashMap::new()));
        let in_memory_tables = Arc::new(RwLock::new(HashMap::new()));
        let indices = Arc::new(RwLock::new(HashMap::new()));
        let cdc = Arc::new(CdcStreamer::new(10_000));

        // Pulihkan skema tabel dan data dari halaman slotted-page
        if let Ok(page0) = storage.get_page(0) {
            let mut cat_guard = catalog.write().unwrap();
            let mut tbl_guard = in_memory_tables.write().unwrap();
            let mut idx_guard = indices.write().unwrap();

            for slot_idx in 0..page0.slot_count() {
                if let Some(rec) = page0.get_record(slot_idx) {
                    if let Ok(schema) = serde_json::from_slice::<TableSchema>(&rec) {
                        let upper_name = schema.name.to_uppercase();
                        if let Some(pk) = &schema.primary_key {
                            idx_guard.insert(format!("{}_{}", upper_name, pk), BTreeIndex::new(pk));
                        }

                        let mut row_list = Vec::new();
                        if let Ok(data_page) = storage.get_page(schema.root_page_id) {
                            for d_slot in 0..data_page.slot_count() {
                                if let Some(d_rec) = data_page.get_record(d_slot) {
                                    if let Ok((hdr, r)) = serde_json::from_slice::<(MvccHeader, Row)>(&d_rec) {
                                        if let Some(pk) = &schema.primary_key {
                                            if let Some(pk_val) = r.fields.get(&pk.to_uppercase()) {
                                                if let Some(idx) = idx_guard.get(&format!("{}_{}", upper_name, pk)) {
                                                    let key = match pk_val {
                                                        SqlValue::Integer(i) => IndexKey::Integer(*i),
                                                        other => IndexKey::Text(format!("{:?}", other)),
                                                    };
                                                    idx.insert(key, RecordPointer { page_id: schema.root_page_id, slot_id: row_list.len() as u16 });
                                                }
                                            }
                                        }
                                        row_list.push((hdr, r));
                                    }
                                }
                            }
                        }
                        tbl_guard.insert(upper_name.clone(), row_list);
                        cat_guard.insert(upper_name, schema);
                    }
                }
            }
        }

        ExecutionEngine {
            storage,
            wal,
            tx_mgr: Arc::new(TransactionManager::new()),
            catalog,
            in_memory_tables,
            indices,
            secondary_indexes: Arc::new(RwLock::new(HashMap::new())),
            views: Arc::new(RwLock::new(HashMap::new())),
            materialized_views: Arc::new(RwLock::new(HashMap::new())),
            savepoints: Arc::new(RwLock::new(HashMap::new())),
            cache_engine: Arc::new(CacheEngine::new()),
            columnar_engine: Arc::new(ColumnarEngine::new()),
            table_stats: Arc::new(RwLock::new(HashMap::new())),
            cdc,
        }
    }

    pub fn execute(&self, stmt: SqlStatement, tx_id: u64) -> Result<QueryResult, String> {
        let snapshot = self.tx_mgr.create_snapshot();

        match stmt {
            SqlStatement::BeginTransaction => {
                let new_tx = self.tx_mgr.begin_transaction();
                let wal_lsn = self.wal.append(new_tx, WalOpType::Begin, 0, 0, Vec::new()).unwrap_or(0);
                self.cdc.emit(CdcRecord::new(wal_lsn, new_tx, "", CdcOp::Begin, None, None));
                Ok(QueryResult::Message(format!("Transaksi dimulai [TX ID: {}]", new_tx)))
            }
            SqlStatement::CommitTransaction => {
                self.tx_mgr.commit(tx_id);
                let wal_lsn = self.wal.append(tx_id, WalOpType::Commit, 0, 0, Vec::new()).unwrap_or(0);
                self.cdc.emit(CdcRecord::new(wal_lsn, tx_id, "", CdcOp::Commit, None, None));
                let _ = self.storage.flush_all();
                let mut sp_guard = self.savepoints.write().unwrap();
                sp_guard.retain(|(t, _), _| *t != tx_id);
                Ok(QueryResult::Message(format!("Transaksi dikonfirmasi [TX ID: {} KOMIT]", tx_id)))
            }
            SqlStatement::RollbackTransaction => {
                self.tx_mgr.rollback(tx_id);
                let wal_lsn = self.wal.append(tx_id, WalOpType::Rollback, 0, 0, Vec::new()).unwrap_or(0);
                self.cdc.emit(CdcRecord::new(wal_lsn, tx_id, "", CdcOp::Rollback, None, None));
                let mut sp_guard = self.savepoints.write().unwrap();
                sp_guard.retain(|(t, _), _| *t != tx_id);
                Ok(QueryResult::Message(format!("Transaksi dibatalkan [TX ID: {} ROLLBACK]", tx_id)))
            }
            SqlStatement::CreateTable { name, columns, column_defs, primary_key, foreign_keys, if_not_exists } => {
                let mut cat = self.catalog.write().unwrap();
                let upper_name = name.to_uppercase();
                if cat.contains_key(&upper_name) {
                    if if_not_exists {
                        return Ok(QueryResult::Message(format!(
                            "Tabel '{}' sudah ada, operasi BUAT_TABEL dilewati (IF NOT EXISTS)", name
                        )));
                    }
                    return Err(format!("Tabel '{}' sudah ada!", name));
                }

                let page = self.storage.allocate_page(PageType::Data)
                    .map_err(|e| format!("Gagal mengalokasikan halaman tabel: {}", e))?;

                let mut effective_column_defs = column_defs.clone();
                if effective_column_defs.is_empty() {
                    for (c_name, c_type) in &columns {
                        let is_pk = primary_key.as_ref().map_or(false, |pk| pk.eq_ignore_ascii_case(c_name));
                        effective_column_defs.push(ColumnDef {
                            name: c_name.clone(),
                            col_type: c_type.clone(),
                            not_null: is_pk,
                            default_value: None,
                            is_unique: is_pk,
                            is_serial: false,
                            is_primary_key: is_pk,
                            references: None,
                        });
                    }
                }

                let schema = TableSchema {
                    name: upper_name.clone(),
                    columns: columns.clone(),
                    column_defs: effective_column_defs.clone(),
                    primary_key: primary_key.clone(),
                    foreign_keys: foreign_keys.clone(),
                    root_page_id: page.id,
                    serial_counter: 0,
                };
                cat.insert(upper_name.clone(), schema.clone());

                let mut tables = self.in_memory_tables.write().unwrap();
                tables.insert(upper_name.clone(), Vec::new());

                if let Some(pk) = &primary_key {
                    let mut idx = self.indices.write().unwrap();
                    idx.insert(format!("{}_{}", upper_name, pk), BTreeIndex::new(pk));
                }

                for col_def in &effective_column_defs {
                    if col_def.is_unique && Some(&col_def.name) != primary_key.as_ref() {
                        let mut idx = self.indices.write().unwrap();
                        idx.insert(format!("{}_{}", upper_name, col_def.name), BTreeIndex::new(&col_def.name));
                    }
                }

                if let Ok(mut page0) = self.storage.get_page(0) {
                    if let Ok(schema_bytes) = serde_json::to_vec(&schema) {
                        page0.insert_record(&schema_bytes);
                        self.storage.update_page(page0);
                        let _ = self.storage.flush_all();
                    }
                }

                let _ = self.wal.append(tx_id, WalOpType::Insert, page.id, 0, name.as_bytes().to_vec());

                Ok(QueryResult::Message(format!("Tabel '{}' berhasil dibuat (Root Page: {})", name, page.id)))
            }
            SqlStatement::DropTable { name, if_exists } => {
                let mut cat = self.catalog.write().unwrap();
                let upper_name = name.to_uppercase();
                if cat.remove(&upper_name).is_some() {
                    let mut tables = self.in_memory_tables.write().unwrap();
                    tables.remove(&upper_name);
                    let _ = self.wal.append(tx_id, WalOpType::Delete, 0, 0, name.as_bytes().to_vec());
                    Ok(QueryResult::Message(format!("Tabel '{}' berhasil dihapus", name)))
                } else if if_exists {
                    Ok(QueryResult::Message(format!("Tabel '{}' tidak ada, operasi HAPUS_TABEL dilewati (IF EXISTS)", name)))
                } else {
                    Err(format!("Tabel '{}' tidak ditemukan", name))
                }
            }
            SqlStatement::Insert { table, columns, values, returning, on_conflict } => {
                let mut cat = self.catalog.write().unwrap();
                let upper_name = table.to_uppercase();
                let schema = cat.get_mut(&upper_name)
                    .ok_or_else(|| format!("Tabel '{}' tidak ditemukan", table))?;

                let mut row_fields = HashMap::new();

                if columns.is_empty() {
                    for (i, val_expr) in values.iter().enumerate() {
                        if i < schema.columns.len() {
                            let col_name = &schema.columns[i].0;
                            row_fields.insert(col_name.to_uppercase(), self.eval_literal(val_expr)?);
                        }
                    }
                } else {
                    for (col, val_expr) in columns.iter().zip(values.iter()) {
                        row_fields.insert(col.to_uppercase(), self.eval_literal(val_expr)?);
                    }
                }

                for col_def in &schema.column_defs {
                    let col_up = col_def.name.to_uppercase();
                    let current_val = row_fields.get(&col_up).cloned().unwrap_or(SqlValue::Null);

                    if col_def.is_serial {
                        if current_val == SqlValue::Null {
                            schema.serial_counter += 1;
                            row_fields.insert(col_up.clone(), SqlValue::Integer(schema.serial_counter as i64));
                        }
                    } else if current_val == SqlValue::Null {
                        if let Some(def_str) = &col_def.default_value {
                            let def_val = if let Ok(i) = def_str.parse::<i64>() {
                                SqlValue::Integer(i)
                            } else if let Ok(f) = def_str.parse::<f64>() {
                                SqlValue::Float(f)
                            } else if def_str.eq_ignore_ascii_case("TRUE") || def_str.eq_ignore_ascii_case("BENAR") {
                                SqlValue::Boolean(true)
                            } else if def_str.eq_ignore_ascii_case("FALSE") || def_str.eq_ignore_ascii_case("SALAH") {
                                SqlValue::Boolean(false)
                            } else if def_str.eq_ignore_ascii_case("NULL") || def_str.eq_ignore_ascii_case("NIHIL") {
                                SqlValue::Null
                            } else {
                                SqlValue::Text(def_str.clone())
                            };
                            row_fields.insert(col_up.clone(), def_val);
                        }
                    }

                    let final_val = row_fields.get(&col_up).unwrap_or(&SqlValue::Null);
                    if col_def.not_null && *final_val == SqlValue::Null {
                        return Err(format!("Pelanggaran constraint NOT NULL: kolom '{}' pada tabel '{}' tidak boleh NIHIL (NULL)", col_def.name, table));
                    }
                }

                let schema_clone = schema.clone();
                drop(cat);

                for fk in &schema_clone.foreign_keys {
                    let fk_col_upper = fk.column.to_uppercase();
                    if let Some(val) = row_fields.get(&fk_col_upper) {
                        if *val != SqlValue::Null {
                            let ref_table_upper = fk.foreign_table.to_uppercase();
                            let ref_col_upper = fk.foreign_column.to_uppercase();
                            let tables = self.in_memory_tables.read().unwrap();
                            let ref_rows = tables.get(&ref_table_upper)
                                .ok_or_else(|| format!("Tabel rujukan '{}' tidak ditemukan", fk.foreign_table))?;
                            let found = ref_rows.iter().any(|(hdr, r)| {
                                self.tx_mgr.is_visible(hdr, &snapshot, tx_id) &&
                                r.fields.get(&ref_col_upper).map_or(false, |v| v == val)
                            });
                            if !found {
                                return Err(format!("Pelanggaran Kunci Asing (Foreign Key Violation): Nilai '{}' pada kolom '{}' tidak ditemukan di tabel rujukan '{}({})'", val.as_text(), fk.column, fk.foreign_table, fk.foreign_column));
                            }
                        }
                    }
                }

                let mut conflict_found = false;
                let mut conflict_row_idx = None;

                let mut unique_cols: Vec<String> = Vec::new();
                if let Some(pk) = &schema_clone.primary_key {
                    unique_cols.push(pk.to_uppercase());
                }
                for c in &schema_clone.column_defs {
                    if c.is_unique {
                        let up = c.name.to_uppercase();
                        if !unique_cols.contains(&up) {
                            unique_cols.push(up);
                        }
                    }
                }

                {
                    let tables = self.in_memory_tables.read().unwrap();
                    if let Some(existing_rows) = tables.get(&upper_name) {
                        for u_col in &unique_cols {
                            if let Some(new_val) = row_fields.get(u_col) {
                                if *new_val != SqlValue::Null {
                                    for (idx, (hdr, r)) in existing_rows.iter().enumerate() {
                                        if self.tx_mgr.is_visible(hdr, &snapshot, tx_id) {
                                            if r.fields.get(u_col).map_or(false, |v| v == new_val) {
                                                conflict_found = true;
                                                conflict_row_idx = Some(idx);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                            if conflict_found {
                                break;
                            }
                        }
                    }
                }

                if conflict_found {
                    match on_conflict {
                        Some(OnConflict::DoNothing) => {
                            return Ok(QueryResult::Rows {
                                columns: vec!["affected".to_string()],
                                rows: vec![vec![SqlValue::Integer(0)]],
                                affected: 0,
                            });
                        }
                        Some(OnConflict::DoUpdate { assignments, .. }) => {
                            if let Some(idx) = conflict_row_idx {
                                let mut tables = self.in_memory_tables.write().unwrap();
                                if let Some(rows) = tables.get_mut(&upper_name) {
                                    if let Some((hdr, r)) = rows.get_mut(idx) {
                                        hdr.xmax = tx_id;
                                        let mut new_r = r.clone();
                                        for (c, val_expr) in assignments {
                                            let new_val = self.eval_literal(&val_expr)?;
                                            new_r.fields.insert(c.to_uppercase(), new_val);
                                        }
                                        rows.push((MvccHeader { xmin: tx_id, xmax: 0 }, new_r));
                                        return Ok(QueryResult::Rows {
                                            columns: vec!["affected".to_string()],
                                            rows: vec![vec![SqlValue::Integer(1)]],
                                            affected: 1,
                                        });
                                    }
                                }
                            }
                        }
                        None => {
                            return Err(format!("Pelanggaran batasan UNIK / UNIQUE / PRIMARY KEY pada tabel '{}'", table));
                        }
                    }
                }

                let row = Row { fields: row_fields };
                let header = MvccHeader { xmin: tx_id, xmax: 0 };

                if let Ok(mut data_page) = self.storage.get_page(schema_clone.root_page_id) {
                    if let Ok(row_bytes) = serde_json::to_vec(&(header.clone(), row.clone())) {
                        data_page.insert_record(&row_bytes);
                        self.storage.update_page(data_page);
                        let _ = self.storage.flush_all();
                    }
                }

                let wal_lsn = self.wal.append(tx_id, WalOpType::Insert, schema_clone.root_page_id, 0, Vec::new()).unwrap_or(0);
                self.cdc.emit(CdcRecord::new(
                    wal_lsn,
                    tx_id,
                    upper_name.clone(),
                    CdcOp::Insert,
                    None,
                    Some(row.fields.clone()),
                ));

                let mut tables = self.in_memory_tables.write().unwrap();
                let list = tables.get_mut(&upper_name).unwrap();
                list.push((header, row.clone()));

                if let Some(pk_col) = &schema_clone.primary_key {
                    if let Some(pk_val) = row.fields.get(&pk_col.to_uppercase()) {
                        let idx_map = self.indices.read().unwrap();
                        if let Some(idx) = idx_map.get(&format!("{}_{}", upper_name, pk_col)) {
                            let key = match pk_val {
                                SqlValue::Integer(i) => IndexKey::Integer(*i),
                                other => IndexKey::Text(format!("{:?}", other)),
                            };
                            idx.insert(key, RecordPointer { page_id: schema_clone.root_page_id, slot_id: (list.len() - 1) as u16 });
                        }
                    }
                }

                {
                    let sec = self.secondary_indexes.read().unwrap();
                    let idx_guard = self.indices.read().unwrap();
                    for (_idx_name, (tbl, col)) in sec.iter() {
                        if tbl == &upper_name {
                            if let Some(val) = row.fields.get(col) {
                                let idx_key = format!("{}_{}", tbl, col);
                                if let Some(idx) = idx_guard.get(&idx_key) {
                                    let key = match val {
                                        SqlValue::Integer(i) => IndexKey::Integer(*i),
                                        SqlValue::Text(t) => IndexKey::Text(t.clone()),
                                        _ => IndexKey::Text(val.as_text()),
                                    };
                                    idx.insert(key, RecordPointer { page_id: schema_clone.root_page_id, slot_id: (list.len() - 1) as u16 });
                                }
                            }
                        }
                    }
                }

                if let Some(ret_exprs) = returning {
                    let mut ret_cols = Vec::new();
                    let mut ret_vals = Vec::new();
                    for expr in &ret_exprs {
                        match expr {
                            SqlExpr::Wildcard => {
                                for (c, _) in &schema_clone.columns {
                                    ret_cols.push(c.clone());
                                    ret_vals.push(row.fields.get(&c.to_uppercase()).cloned().unwrap_or(SqlValue::Null));
                                }
                            }
                            SqlExpr::Column(c) => {
                                ret_cols.push(c.clone());
                                ret_vals.push(row.fields.get(&c.to_uppercase()).cloned().unwrap_or(SqlValue::Null));
                            }
                            SqlExpr::Alias { expr: inner, alias } => {
                                ret_cols.push(alias.clone());
                                ret_vals.push(self.eval_expr_value(inner, &row)?);
                            }
                            _ => {
                                ret_cols.push("kolom".to_string());
                                ret_vals.push(self.eval_expr_value(expr, &row)?);
                            }
                        }
                    }
                    return Ok(QueryResult::Rows {
                        columns: ret_cols,
                        rows: vec![ret_vals],
                        affected: 1,
                    });
                }

                Ok(QueryResult::Rows {
                    columns: vec!["affected".to_string()],
                    rows: vec![vec![SqlValue::Integer(1)]],
                    affected: 1,
                })
            }
            SqlStatement::InsertSelect { table, columns, select, returning, on_conflict } => {
                let res = self.execute(*select, tx_id)?;
                let (sel_cols, sel_rows) = match res {
                    QueryResult::Rows { columns: c, rows: r, .. } => (c, r),
                    _ => return Err("Kueri SELECT pada INSERT INTO ... SELECT tidak menghasilkan baris".to_string()),
                };

                let target_cols = if columns.is_empty() { sel_cols } else { columns };
                let mut total_affected = 0;
                let mut ret_all_rows = Vec::new();
                let mut ret_all_cols = Vec::new();

                for r_vals in sel_rows {
                    let val_exprs: Vec<SqlExpr> = r_vals.into_iter().map(SqlExpr::Literal).collect();
                    let ins_res = self.execute(SqlStatement::Insert {
                        table: table.clone(),
                        columns: target_cols.clone(),
                        values: val_exprs,
                        returning: returning.clone(),
                        on_conflict: on_conflict.clone(),
                    }, tx_id)?;

                    match ins_res {
                        QueryResult::Rows { columns: rc, rows: rr, affected } => {
                            total_affected += affected;
                            ret_all_cols = rc;
                            ret_all_rows.extend(rr);
                        }
                        _ => {}
                    }
                }

                if returning.is_some() {
                    Ok(QueryResult::Rows {
                        columns: ret_all_cols,
                        rows: ret_all_rows,
                        affected: total_affected,
                    })
                } else {
                    Ok(QueryResult::Rows {
                        columns: vec!["affected".to_string()],
                        rows: vec![vec![SqlValue::Integer(total_affected as i64)]],
                        affected: total_affected,
                    })
                }
            }
            SqlStatement::InsertOrIgnore { table, columns, values } => {
                self.execute(SqlStatement::Insert {
                    table,
                    columns,
                    values,
                    returning: None,
                    on_conflict: Some(OnConflict::DoNothing),
                }, tx_id)
            }
            SqlStatement::Union { left, right, is_all } => {
                let res_left = self.execute(*left, tx_id)?;
                let res_right = self.execute(*right, tx_id)?;

                let (cols, mut rows_left) = match res_left {
                    QueryResult::Rows { columns, rows, .. } => (columns, rows),
                    _ => return Err("Kueri kiri UNION tidak menghasilkan baris".to_string()),
                };
                let rows_right = match res_right {
                    QueryResult::Rows { rows, .. } => rows,
                    _ => return Err("Kueri kanan UNION tidak menghasilkan baris".to_string()),
                };

                if is_all {
                    rows_left.extend(rows_right);
                } else {
                    let mut unique_rows = Vec::new();
                    for r in rows_left {
                        if !unique_rows.contains(&r) {
                            unique_rows.push(r);
                        }
                    }
                    for r in rows_right {
                        if !unique_rows.contains(&r) {
                            unique_rows.push(r);
                        }
                    }
                    rows_left = unique_rows;
                }

                let count = rows_left.len();
                Ok(QueryResult::Rows {
                    columns: cols,
                    rows: rows_left,
                    affected: count,
                })
            }
            SqlStatement::Intersect { left, right } => {
                let res_left = self.execute(*left, tx_id)?;
                let res_right = self.execute(*right, tx_id)?;

                let (cols, rows_left) = match res_left {
                    QueryResult::Rows { columns, rows, .. } => (columns, rows),
                    _ => return Err("Kueri kiri INTERSECT tidak menghasilkan baris".to_string()),
                };
                let rows_right = match res_right {
                    QueryResult::Rows { rows, .. } => rows,
                    _ => return Err("Kueri kanan INTERSECT tidak menghasilkan baris".to_string()),
                };

                let mut intersect_rows = Vec::new();
                for r in rows_left {
                    if rows_right.contains(&r) && !intersect_rows.contains(&r) {
                        intersect_rows.push(r);
                    }
                }

                let count = intersect_rows.len();
                Ok(QueryResult::Rows {
                    columns: cols,
                    rows: intersect_rows,
                    affected: count,
                })
            }
            SqlStatement::Except { left, right } => {
                let res_left = self.execute(*left, tx_id)?;
                let res_right = self.execute(*right, tx_id)?;

                let (cols, rows_left) = match res_left {
                    QueryResult::Rows { columns, rows, .. } => (columns, rows),
                    _ => return Err("Kueri kiri EXCEPT tidak menghasilkan baris".to_string()),
                };
                let rows_right = match res_right {
                    QueryResult::Rows { rows, .. } => rows,
                    _ => return Err("Kueri kanan EXCEPT tidak menghasilkan baris".to_string()),
                };

                let mut except_rows = Vec::new();
                for r in rows_left {
                    if !rows_right.contains(&r) && !except_rows.contains(&r) {
                        except_rows.push(r);
                    }
                }

                let count = except_rows.len();
                Ok(QueryResult::Rows {
                    columns: cols,
                    rows: except_rows,
                    affected: count,
                })
            }
            SqlStatement::Select { columns, from_table, from_alias, join, joins, where_clause, group_by, having, order_by, limit, offset, is_distinct } => {
                // 0. Pre-evaluasi Subkueri (Deadlock-free & O(1) per kueri)
                let where_clause = match where_clause {
                    Some(w) => Some(self.resolve_subqueries(w, tx_id)?),
                    None => None,
                };
                let mut resolved_columns = Vec::new();
                for c in columns {
                    resolved_columns.push(self.resolve_subqueries(c, tx_id)?);
                }
                let columns = resolved_columns;
                let having = match having {
                    Some(h) => Some(self.resolve_subqueries(h, tx_id)?),
                    None => None,
                };

                let upper_name = from_table.to_uppercase();
                let from_alias_upper = from_alias.as_ref().map(|a| a.to_uppercase());

                // Evaluasi tanpa tabel (misal: SELECT NOW(), PILIH SEKARANG(), SELECT 1+1, atau DUAL)
                if upper_name.is_empty() || upper_name == "DUAL" {
                    let dummy_row = Row { fields: HashMap::new() };
                    let mut final_cols = Vec::new();
                    let mut out_row = Vec::new();
                    for col_expr in &columns {
                        let col_name = match col_expr {
                            SqlExpr::Alias { alias, .. } => alias.clone(),
                            SqlExpr::Column(n) => n.clone(),
                            _ => format!("{:?}", col_expr),
                        };
                        let val = self.eval_expr_value(col_expr, &dummy_row)?;
                        final_cols.push(col_name);
                        out_row.push(val);
                    }
                    return Ok(QueryResult::Rows {
                        columns: final_cols,
                        rows: vec![out_row],
                        affected: 1,
                    });
                }

                let cat = self.catalog.read().unwrap();
                let (schema, rows_data) = match cat.get(&upper_name) {
                    Some(s) => {
                        let tables = self.in_memory_tables.read().unwrap();
                        let r = tables.get(&upper_name)
                            .ok_or_else(|| format!("Data tabel '{}' belum diinisialisasi", from_table))?.clone();
                        (s.clone(), r)
                    }
                    None => {
                        let view_stmt = {
                            let v_guard = self.views.read().unwrap();
                            v_guard.get(&upper_name).cloned()
                        };
                        if let Some(stmt) = view_stmt {
                            drop(cat);
                            let view_res = self.execute(stmt, tx_id)?;
                            let (v_cols, v_rows) = match view_res {
                                QueryResult::Rows { columns, rows, .. } => (columns, rows),
                                _ => return Err(format!("Tampilan (View) '{}' tidak menghasilkan baris", from_table)),
                            };
                            let synth_schema = TableSchema {
                                name: upper_name.clone(),
                                columns: v_cols.iter().map(|c| (c.clone(), "TEXT".to_string())).collect(),
                                column_defs: Vec::new(),
                                primary_key: None,
                                foreign_keys: Vec::new(),
                                root_page_id: 0,
                                serial_counter: 0,
                            };
                            let synth_rows: Vec<(MvccHeader, Row)> = v_rows.into_iter().map(|row_vals| {
                                let mut fields = HashMap::new();
                                for (i, val) in row_vals.into_iter().enumerate() {
                                    if i < v_cols.len() {
                                        fields.insert(v_cols[i].to_uppercase(), val);
                                    }
                                }
                                (MvccHeader { xmin: tx_id, xmax: 0 }, Row { fields })
                            }).collect();
                            (synth_schema, synth_rows)
                        } else {
                            return Err(format!("Tabel atau Tampilan '{}' tidak ditemukan", from_table));
                        }
                    }
                };

                // Kumpulkan semua klausa JOIN (single join legacy + multi joins)
                let all_joins = if !joins.is_empty() {
                    joins
                } else if let Some(j) = join {
                    vec![j]
                } else {
                    Vec::new()
                };

                // 1. Ambil baris: Menggunakan Optimasi Index Scan O(log N) atau Multi-table Relational JOIN
                let mut matched_rows = Vec::new();

                if all_joins.is_empty() {
                    // Cek Optimasi Index Scan jika kueri WHERE memeriksa kunci utama atau indeks sekunder
                    let mut used_index = false;
                    if let Some(SqlExpr::Binary { left, op: BinaryOp::Eq, right }) = &where_clause {
                        let mut indexed_search = None;
                        if let Some(pk_col) = &schema.primary_key {
                            if let (SqlExpr::Column(col), SqlExpr::Literal(val)) = (&**left, &**right) {
                                if col.eq_ignore_ascii_case(pk_col) || col.eq_ignore_ascii_case(&format!("{}.{}", upper_name, pk_col)) {
                                    indexed_search = Some((pk_col.clone(), val.clone()));
                                }
                            } else if let (SqlExpr::Literal(val), SqlExpr::Column(col)) = (&**left, &**right) {
                                if col.eq_ignore_ascii_case(pk_col) || col.eq_ignore_ascii_case(&format!("{}.{}", upper_name, pk_col)) {
                                    indexed_search = Some((pk_col.clone(), val.clone()));
                                }
                            }
                        }

                        if indexed_search.is_none() {
                            let sec_guard = self.secondary_indexes.read().unwrap();
                            for (_idx_name, (tbl, col)) in sec_guard.iter() {
                                if tbl == &upper_name {
                                    if let (SqlExpr::Column(c), SqlExpr::Literal(val)) = (&**left, &**right) {
                                        if c.eq_ignore_ascii_case(col) || c.eq_ignore_ascii_case(&format!("{}.{}", upper_name, col)) {
                                            indexed_search = Some((col.clone(), val.clone()));
                                            break;
                                        }
                                    } else if let (SqlExpr::Literal(val), SqlExpr::Column(c)) = (&**left, &**right) {
                                        if c.eq_ignore_ascii_case(col) || c.eq_ignore_ascii_case(&format!("{}.{}", upper_name, col)) {
                                            indexed_search = Some((col.clone(), val.clone()));
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        if let Some((idx_col, val)) = indexed_search {
                            let idx_guard = self.indices.read().unwrap();
                            let idx_key = format!("{}_{}", upper_name, idx_col);
                            if let Some(idx) = idx_guard.get(&idx_key) {
                                let search_key = match val {
                                    SqlValue::Integer(i) => Some(IndexKey::Integer(i)),
                                    SqlValue::Text(t) => Some(IndexKey::Text(t)),
                                    _ => None,
                                };
                                if let Some(k) = search_key {
                                    if let Some(ptr) = idx.search(&k) {
                                        let slot_idx = ptr.slot_id as usize;
                                        if slot_idx < rows_data.len() {
                                            let (hdr, r) = &rows_data[slot_idx];
                                            if self.tx_mgr.is_visible(hdr, &snapshot, tx_id) {
                                                matched_rows.push(r.clone());
                                                used_index = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if !used_index {
                        for (hdr, r) in rows_data.iter() {
                            if self.tx_mgr.is_visible(hdr, &snapshot, tx_id) {
                                let matches_where = match &where_clause {
                                    Some(expr) => self.eval_predicate(expr, r)?,
                                    None => true,
                                };
                                if matches_where {
                                    matched_rows.push(r.clone());
                                }
                            }
                        }
                    }
                } else {
                    // Multi-table Relational JOIN (INNER, LEFT, RIGHT, FULL OUTER, CROSS)
                    let mut current_rows: Vec<Row> = Vec::new();
                    for (hdr, r) in rows_data.iter() {
                        if self.tx_mgr.is_visible(hdr, &snapshot, tx_id) {
                            let mut fields = HashMap::new();
                            for (k, v) in &r.fields {
                                fields.insert(format!("{}.{}", upper_name, k), v.clone());
                                if let Some(ref fa) = from_alias_upper {
                                    fields.insert(format!("{}.{}", fa, k), v.clone());
                                }
                                fields.insert(k.clone(), v.clone());
                            }
                            current_rows.push(Row { fields });
                        }
                    }

                    for join_clause in &all_joins {
                        let join_upper = join_clause.table.to_uppercase();
                        let join_alias_upper = join_clause.table_alias.as_ref().map(|a| a.to_uppercase());
                        let join_schema = {
                            let cat_guard = self.catalog.read().unwrap();
                            cat_guard.get(&join_upper)
                                .ok_or_else(|| format!("Tabel JOIN '{}' tidak ditemukan", join_clause.table))?
                                .clone()
                        };
                        let join_rows_data = {
                            let tbl_guard = self.in_memory_tables.read().unwrap();
                            tbl_guard.get(&join_upper)
                                .ok_or_else(|| format!("Data tabel JOIN '{}' belum diinisialisasi", join_clause.table))?
                                .clone()
                        };
                        let right_visible: Vec<&Row> = join_rows_data.iter()
                            .filter(|(hdr, _)| self.tx_mgr.is_visible(hdr, &snapshot, tx_id))
                            .map(|(_, r)| r)
                            .collect();

                        let mut next_rows = Vec::new();
                        match join_clause.join_type {
                            JoinType::Cross => {
                                for r_left in &current_rows {
                                    for r_right in &right_visible {
                                        let mut combined_fields = r_left.fields.clone();
                                        for (k, v) in &r_right.fields {
                                            combined_fields.insert(format!("{}.{}", join_upper, k), v.clone());
                                            if let Some(ref ja) = join_alias_upper {
                                                combined_fields.insert(format!("{}.{}", ja, k), v.clone());
                                            }
                                            if !combined_fields.contains_key(k) {
                                                combined_fields.insert(k.clone(), v.clone());
                                            }
                                        }
                                        next_rows.push(Row { fields: combined_fields });
                                    }
                                }
                            }
                            JoinType::Inner => {
                                for r_left in &current_rows {
                                    let left_val = self.eval_expr_value(&SqlExpr::Column(join_clause.on_left.clone()), r_left)?;
                                    for r_right in &right_visible {
                                        let right_val = self.eval_expr_value(&SqlExpr::Column(join_clause.on_right.clone()), r_right)?;
                                        if left_val != SqlValue::Null && left_val == right_val {
                                            let mut combined_fields = r_left.fields.clone();
                                            for (k, v) in &r_right.fields {
                                                combined_fields.insert(format!("{}.{}", join_upper, k), v.clone());
                                                if let Some(ref ja) = join_alias_upper {
                                                    combined_fields.insert(format!("{}.{}", ja, k), v.clone());
                                                }
                                                if !combined_fields.contains_key(k) {
                                                    combined_fields.insert(k.clone(), v.clone());
                                                }
                                            }
                                            next_rows.push(Row { fields: combined_fields });
                                        }
                                    }
                                }
                            }
                            JoinType::Left => {
                                for r_left in &current_rows {
                                    let left_val = self.eval_expr_value(&SqlExpr::Column(join_clause.on_left.clone()), r_left)?;
                                    let mut matched = false;
                                    for r_right in &right_visible {
                                        let right_val = self.eval_expr_value(&SqlExpr::Column(join_clause.on_right.clone()), r_right)?;
                                        if left_val != SqlValue::Null && left_val == right_val {
                                            matched = true;
                                            let mut combined_fields = r_left.fields.clone();
                                            for (k, v) in &r_right.fields {
                                                combined_fields.insert(format!("{}.{}", join_upper, k), v.clone());
                                                if let Some(ref ja) = join_alias_upper {
                                                    combined_fields.insert(format!("{}.{}", ja, k), v.clone());
                                                }
                                                if !combined_fields.contains_key(k) {
                                                    combined_fields.insert(k.clone(), v.clone());
                                                }
                                            }
                                            next_rows.push(Row { fields: combined_fields });
                                        }
                                    }
                                    if !matched {
                                        let mut combined_fields = r_left.fields.clone();
                                        for (c, _) in &join_schema.columns {
                                            let col_up = c.to_uppercase();
                                            combined_fields.insert(format!("{}.{}", join_upper, col_up), SqlValue::Null);
                                            if let Some(ref ja) = join_alias_upper {
                                                combined_fields.insert(format!("{}.{}", ja, col_up), SqlValue::Null);
                                            }
                                            if !combined_fields.contains_key(&col_up) {
                                                combined_fields.insert(col_up, SqlValue::Null);
                                            }
                                        }
                                        next_rows.push(Row { fields: combined_fields });
                                    }
                                }
                            }
                            JoinType::Right => {
                                for r_right in &right_visible {
                                    let right_val = self.eval_expr_value(&SqlExpr::Column(join_clause.on_right.clone()), r_right)?;
                                    let mut matched = false;
                                    for r_left in &current_rows {
                                        let left_val = self.eval_expr_value(&SqlExpr::Column(join_clause.on_left.clone()), r_left)?;
                                        if right_val != SqlValue::Null && left_val == right_val {
                                            matched = true;
                                            let mut combined_fields = r_left.fields.clone();
                                            for (k, v) in &r_right.fields {
                                                combined_fields.insert(format!("{}.{}", join_upper, k), v.clone());
                                                if let Some(ref ja) = join_alias_upper {
                                                    combined_fields.insert(format!("{}.{}", ja, k), v.clone());
                                                }
                                                if !combined_fields.contains_key(k) {
                                                    combined_fields.insert(k.clone(), v.clone());
                                                }
                                            }
                                            next_rows.push(Row { fields: combined_fields });
                                        }
                                    }
                                    if !matched {
                                        let mut combined_fields = HashMap::new();
                                        for (k, v) in &r_right.fields {
                                            combined_fields.insert(format!("{}.{}", join_upper, k), v.clone());
                                            if let Some(ref ja) = join_alias_upper {
                                                combined_fields.insert(format!("{}.{}", ja, k), v.clone());
                                            }
                                            combined_fields.insert(k.clone(), v.clone());
                                        }
                                        for (c, _) in &schema.columns {
                                            let col_up = c.to_uppercase();
                                            combined_fields.insert(format!("{}.{}", upper_name, col_up), SqlValue::Null);
                                            if let Some(ref fa) = from_alias_upper {
                                                combined_fields.insert(format!("{}.{}", fa, col_up), SqlValue::Null);
                                            }
                                            if !combined_fields.contains_key(&col_up) {
                                                combined_fields.insert(col_up, SqlValue::Null);
                                            }
                                        }
                                        next_rows.push(Row { fields: combined_fields });
                                    }
                                }
                            }
                            JoinType::FullOuter => {
                                let mut right_matched = vec![false; right_visible.len()];
                                for r_left in &current_rows {
                                    let left_val = self.eval_expr_value(&SqlExpr::Column(join_clause.on_left.clone()), r_left)?;
                                    let mut left_found = false;
                                    for (r_i, r_right) in right_visible.iter().enumerate() {
                                        let right_val = self.eval_expr_value(&SqlExpr::Column(join_clause.on_right.clone()), r_right)?;
                                        if left_val != SqlValue::Null && left_val == right_val {
                                            left_found = true;
                                            right_matched[r_i] = true;
                                            let mut combined_fields = r_left.fields.clone();
                                            for (k, v) in &r_right.fields {
                                                combined_fields.insert(format!("{}.{}", join_upper, k), v.clone());
                                                if let Some(ref ja) = join_alias_upper {
                                                    combined_fields.insert(format!("{}.{}", ja, k), v.clone());
                                                }
                                                if !combined_fields.contains_key(k) {
                                                    combined_fields.insert(k.clone(), v.clone());
                                                }
                                            }
                                            next_rows.push(Row { fields: combined_fields });
                                        }
                                    }
                                    if !left_found {
                                        let mut combined_fields = r_left.fields.clone();
                                        for (c, _) in &join_schema.columns {
                                            let col_up = c.to_uppercase();
                                            combined_fields.insert(format!("{}.{}", join_upper, col_up), SqlValue::Null);
                                            if let Some(ref ja) = join_alias_upper {
                                                combined_fields.insert(format!("{}.{}", ja, col_up), SqlValue::Null);
                                            }
                                            if !combined_fields.contains_key(&col_up) {
                                                combined_fields.insert(col_up, SqlValue::Null);
                                            }
                                        }
                                        next_rows.push(Row { fields: combined_fields });
                                    }
                                }
                                for (r_i, r_right) in right_visible.iter().enumerate() {
                                    if !right_matched[r_i] {
                                        let mut combined_fields = HashMap::new();
                                        for (k, v) in &r_right.fields {
                                            combined_fields.insert(format!("{}.{}", join_upper, k), v.clone());
                                            if let Some(ref ja) = join_alias_upper {
                                                combined_fields.insert(format!("{}.{}", ja, k), v.clone());
                                            }
                                            combined_fields.insert(k.clone(), v.clone());
                                        }
                                        for (c, _) in &schema.columns {
                                            let col_up = c.to_uppercase();
                                            combined_fields.insert(format!("{}.{}", upper_name, col_up), SqlValue::Null);
                                            if let Some(ref fa) = from_alias_upper {
                                                combined_fields.insert(format!("{}.{}", fa, col_up), SqlValue::Null);
                                            }
                                            if !combined_fields.contains_key(&col_up) {
                                                combined_fields.insert(col_up, SqlValue::Null);
                                            }
                                        }
                                        next_rows.push(Row { fields: combined_fields });
                                    }
                                }
                            }
                        }
                        current_rows = next_rows;
                    }

                    for r in current_rows {
                        let matches_where = match &where_clause {
                            Some(expr) => self.eval_predicate(expr, &r)?,
                            None => true,
                        };
                        if matches_where {
                            matched_rows.push(r);
                        }
                    }
                }

                // 2. KELOMPOKKAN_BERDASARKAN / GROUP BY (Multi-kolom)
                if !group_by.is_empty() {
                    let mut groups: Vec<(Vec<SqlValue>, Vec<Row>)> = Vec::new();
                    let mut group_indices: HashMap<String, usize> = HashMap::new();

                    for r in &matched_rows {
                        let mut row_group_vals = Vec::new();
                        let mut key_parts = Vec::new();
                        for g_col in &group_by {
                            let val = self.eval_expr_value(&SqlExpr::Column(g_col.clone()), r).unwrap_or(SqlValue::Null);
                            key_parts.push(val.as_text());
                            row_group_vals.push(val);
                        }
                        let key = key_parts.join("|||");
                        if let Some(&idx) = group_indices.get(&key) {
                            groups[idx].1.push(r.clone());
                        } else {
                            let idx = groups.len();
                            group_indices.insert(key, idx);
                            groups.push((row_group_vals, vec![r.clone()]));
                        }
                    }

                    let out_cols: Vec<String> = columns.iter().map(|col_expr| {
                        match col_expr {
                            SqlExpr::Alias { alias, .. } => alias.clone(),
                            SqlExpr::Column(c) => c.clone(),
                            SqlExpr::FunctionCall { name, args } => {
                                if args.is_empty() || matches!(args[0], SqlExpr::Wildcard) {
                                    format!("{}(*)", name)
                                } else if let SqlExpr::Column(c) = &args[0] {
                                    format!("{}({})", name, c)
                                } else {
                                    format!("{}(...)", name)
                                }
                            }
                            _ => "kolom".to_string(),
                        }
                    }).collect();

                    let mut out_rows = Vec::new();
                    for (group_vals, group_rows) in &groups {
                        // Evaluasi HAVING jika ada
                        if let Some(h_expr) = &having {
                            let pass = self.eval_grouped_predicate(h_expr, &group_by, group_vals, group_rows)?;
                            if !pass {
                                continue;
                            }
                        }
                        let mut row_vals = Vec::new();
                        for col_expr in &columns {
                            let val = self.eval_grouped_expr(col_expr, &group_by, group_vals, group_rows)?;
                            row_vals.push(val);
                        }
                        out_rows.push(row_vals);
                    }

                    // Urutkan hasil GROUP BY jika ada ORDER BY
                    if let Some(OrderByClause { column, is_desc, secondary }) = &order_by {
                        let find_idx = |col_name: &str| {
                            out_cols.iter().position(|c| c.eq_ignore_ascii_case(col_name))
                                .or_else(|| columns.iter().position(|c| {
                                    match c {
                                        SqlExpr::Column(n) => n.eq_ignore_ascii_case(col_name),
                                        SqlExpr::Alias { alias, .. } => alias.eq_ignore_ascii_case(col_name),
                                        _ => false,
                                    }
                                }))
                                .unwrap_or(0)
                        };
                        let col_idx = find_idx(column);

                        out_rows.sort_by(|a, b| {
                            let val_a = a.get(col_idx).unwrap_or(&SqlValue::Null);
                            let val_b = b.get(col_idx).unwrap_or(&SqlValue::Null);
                            let mut ord = compare_sql_values(val_a, val_b);
                            if *is_desc { ord = ord.reverse(); }
                            if ord == std::cmp::Ordering::Equal {
                                for (s_col, s_desc) in secondary {
                                    let s_idx = find_idx(s_col);
                                    let sa = a.get(s_idx).unwrap_or(&SqlValue::Null);
                                    let sb = b.get(s_idx).unwrap_or(&SqlValue::Null);
                                    let mut s_ord = compare_sql_values(sa, sb);
                                    if *s_desc { s_ord = s_ord.reverse(); }
                                    if s_ord != std::cmp::Ordering::Equal {
                                        return s_ord;
                                    }
                                }
                            }
                            ord
                        });
                    }

                    // Deduplikasi BERBEDA / DISTINCT untuk GROUP BY
                    if is_distinct {
                        let mut unique_rows = Vec::new();
                        for r in out_rows {
                            if !unique_rows.contains(&r) {
                                unique_rows.push(r);
                            }
                        }
                        out_rows = unique_rows;
                    }

                    // OFFSET & LIMIT untuk GROUP BY
                    let start = offset.unwrap_or(0);
                    let end = match limit {
                        Some(lim) => (start + lim).min(out_rows.len()),
                        None => out_rows.len(),
                    };
                    let final_rows = if start < out_rows.len() {
                        out_rows[start..end].to_vec()
                    } else {
                        Vec::new()
                    };
                    let len = final_rows.len();

                    return Ok(QueryResult::Rows {
                        columns: out_cols,
                        rows: final_rows,
                        affected: len,
                    });
                }

                // 3. FUNGSI AGREGASI TANPA GROUP BY
                let has_aggregate = columns.iter().any(|c| match c {
                    SqlExpr::FunctionCall { name, .. } => matches!(name.to_uppercase().as_str(), "COUNT" | "SUM" | "AVG" | "MIN" | "MAX" | "HITUNG" | "JUMLAH" | "RATA_RATA" | "MAKS" | "STRING_AGG" | "ARRAY_AGG"),
                    SqlExpr::Alias { expr, .. } => matches!(&**expr, SqlExpr::FunctionCall { name, .. } if matches!(name.to_uppercase().as_str(), "COUNT" | "SUM" | "AVG" | "MIN" | "MAX" | "HITUNG" | "JUMLAH" | "RATA_RATA" | "MAKS" | "STRING_AGG" | "ARRAY_AGG")),
                    _ => false,
                });

                if has_aggregate {
                    let mut out_cols = Vec::new();
                    let mut out_row = Vec::new();

                    for col_expr in &columns {
                        let (agg_name, args, header) = match col_expr {
                            SqlExpr::FunctionCall { name, args } => {
                                let first_arg = args.get(0).cloned().unwrap_or(SqlExpr::Wildcard);
                                let h = if args.is_empty() || matches!(first_arg, SqlExpr::Wildcard) {
                                    format!("{}(*)", name)
                                } else if let SqlExpr::Column(c) = &first_arg {
                                    format!("{}({})", name, c)
                                } else {
                                    format!("{}(...)", name)
                                };
                                (name.clone(), args.clone(), h)
                            }
                            SqlExpr::Alias { expr, alias } => {
                                if let SqlExpr::FunctionCall { name, args } = &**expr {
                                    (name.clone(), args.clone(), alias.clone())
                                } else {
                                    continue;
                                }
                            }
                            _ => continue,
                        };

                        out_cols.push(header);
                        let agg_up = agg_name.to_uppercase();
                        let arg = args.get(0).cloned().unwrap_or(SqlExpr::Wildcard);

                        match agg_up.as_str() {
                            "COUNT" | "HITUNG" => {
                                if matches!(arg, SqlExpr::Wildcard) {
                                    out_row.push(SqlValue::Integer(matched_rows.len() as i64));
                                } else {
                                    let mut count = 0;
                                    for r in &matched_rows {
                                        if self.eval_expr_value(&arg, r).unwrap_or(SqlValue::Null) != SqlValue::Null {
                                            count += 1;
                                        }
                                    }
                                    out_row.push(SqlValue::Integer(count));
                                }
                            }
                            "SUM" | "JUMLAH" => {
                                let mut sum = 0.0;
                                for r in &matched_rows {
                                    match self.eval_expr_value(&arg, r).unwrap_or(SqlValue::Null) {
                                        SqlValue::Integer(i) => sum += i as f64,
                                        SqlValue::Float(f) => sum += f,
                                        _ => {}
                                    }
                                }
                                out_row.push(SqlValue::Float(sum));
                            }
                            "AVG" | "RATA_RATA" => {
                                let count = matched_rows.len();
                                let mut sum = 0.0;
                                for r in &matched_rows {
                                    match self.eval_expr_value(&arg, r).unwrap_or(SqlValue::Null) {
                                        SqlValue::Integer(i) => sum += i as f64,
                                        SqlValue::Float(f) => sum += f,
                                        _ => {}
                                    }
                                }
                                let avg = if count > 0 { sum / count as f64 } else { 0.0 };
                                out_row.push(SqlValue::Float(avg));
                            }
                            "MIN" => {
                                let mut min_val = f64::MAX;
                                for r in &matched_rows {
                                    match self.eval_expr_value(&arg, r).unwrap_or(SqlValue::Null) {
                                        SqlValue::Integer(i) => { if (i as f64) < min_val { min_val = i as f64; } }
                                        SqlValue::Float(f) => { if f < min_val { min_val = f; } }
                                        _ => {}
                                    }
                                }
                                out_row.push(SqlValue::Float(if min_val == f64::MAX { 0.0 } else { min_val }));
                            }
                            "MAX" | "MAKS" => {
                                let mut max_val = f64::MIN;
                                for r in &matched_rows {
                                    match self.eval_expr_value(&arg, r).unwrap_or(SqlValue::Null) {
                                        SqlValue::Integer(i) => { if (i as f64) > max_val { max_val = i as f64; } }
                                        SqlValue::Float(f) => { if f > max_val { max_val = f; } }
                                        _ => {}
                                    }
                                }
                                out_row.push(SqlValue::Float(if max_val == f64::MIN { 0.0 } else { max_val }));
                            }
                            "STRING_AGG" | "GROUP_CONCAT" => {
                                let delim = if args.len() > 1 {
                                    match self.eval_expr_value(&args[1], &Row { fields: HashMap::new() }).unwrap_or(SqlValue::Null) {
                                        SqlValue::Text(s) => s,
                                        _ => ",".to_string(),
                                    }
                                } else {
                                    ",".to_string()
                                };
                                let mut parts = Vec::new();
                                for r in &matched_rows {
                                    let v = self.eval_expr_value(&arg, r).unwrap_or(SqlValue::Null);
                                    if v != SqlValue::Null {
                                        parts.push(v.as_text());
                                    }
                                }
                                out_row.push(SqlValue::Text(parts.join(&delim)));
                            }
                            "ARRAY_AGG" => {
                                let mut parts = Vec::new();
                                for r in &matched_rows {
                                    let v = self.eval_expr_value(&arg, r).unwrap_or(SqlValue::Null);
                                    parts.push(v.as_text());
                                }
                                out_row.push(SqlValue::Text(format!("[{}]", parts.join(", "))));
                            }
                            _ => out_row.push(SqlValue::Null),
                        }
                    }

                    return Ok(QueryResult::Rows {
                        columns: out_cols,
                        rows: vec![out_row],
                        affected: 1,
                    });
                }

                // 4. ORDER BY (Mendukung multi-kolom, alias ekspresi terkomputasi & vektor)
                if let Some(OrderByClause { column, is_desc, secondary }) = &order_by {
                    let get_val = |row: &Row, col_name: &str| {
                        let alias_expr = columns.iter().find_map(|c| {
                            if let SqlExpr::Alias { expr, alias } = c {
                                if alias.eq_ignore_ascii_case(col_name) {
                                    return Some(&**expr);
                                }
                            }
                            None
                        });
                        if let Some(expr) = alias_expr {
                            self.eval_expr_value(expr, row).unwrap_or(SqlValue::Null)
                        } else {
                            self.eval_expr_value(&SqlExpr::Column(col_name.to_string()), row).unwrap_or(SqlValue::Null)
                        }
                    };

                    matched_rows.sort_by(|a, b| {
                        let val_a = get_val(a, column);
                        let val_b = get_val(b, column);
                        let mut ord = compare_sql_values(&val_a, &val_b);
                        if *is_desc { ord = ord.reverse(); }
                        if ord == std::cmp::Ordering::Equal {
                            for (s_col, s_desc) in secondary {
                                let sa = get_val(a, s_col);
                                let sb = get_val(b, s_col);
                                let mut s_ord = compare_sql_values(&sa, &sb);
                                if *s_desc { s_ord = s_ord.reverse(); }
                                if s_ord != std::cmp::Ordering::Equal {
                                    return s_ord;
                                }
                            }
                        }
                        ord
                    });
                }

                // 5. Proyeksi Kolom Terkomputasi & Relasional
                let is_wildcard = columns.iter().any(|c| matches!(c, SqlExpr::Wildcard));
                let (final_cols, project_exprs): (Vec<String>, Vec<SqlExpr>) = if is_wildcard {
                    if !all_joins.is_empty() {
                        let mut all_cols = Vec::new();
                        let mut all_exprs = Vec::new();
                        for (c, _) in &schema.columns {
                            let col_name = format!("{}.{}", from_table, c);
                            all_cols.push(col_name.clone());
                            all_exprs.push(SqlExpr::Column(col_name));
                        }
                        for join_clause in &all_joins {
                            let join_upper = join_clause.table.to_uppercase();
                            let join_schema = {
                                let cat_guard = self.catalog.read().unwrap();
                                cat_guard.get(&join_upper).cloned().unwrap()
                            };
                            for (c, _) in &join_schema.columns {
                                let col_name = format!("{}.{}", join_clause.table, c);
                                all_cols.push(col_name.clone());
                                all_exprs.push(SqlExpr::Column(col_name));
                            }
                        }
                        (all_cols, all_exprs)
                    } else {
                        let mut cols = Vec::new();
                        let mut exprs = Vec::new();
                        for (c, _) in &schema.columns {
                            cols.push(c.clone());
                            exprs.push(SqlExpr::Column(c.clone()));
                        }
                        (cols, exprs)
                    }
                } else {
                    let mut cols = Vec::new();
                    let mut exprs = Vec::new();
                    for c in &columns {
                        match c {
                            SqlExpr::Alias { alias, .. } => {
                                cols.push(alias.clone());
                                exprs.push(c.clone());
                            }
                            SqlExpr::Column(name) => {
                                cols.push(name.clone());
                                exprs.push(c.clone());
                            }
                            SqlExpr::FunctionCall { name, args } => {
                                if args.is_empty() || matches!(args[0], SqlExpr::Wildcard) {
                                    cols.push(format!("{}(*)", name));
                                } else if let SqlExpr::Column(col) = &args[0] {
                                    cols.push(format!("{}({})", name, col));
                                } else {
                                    cols.push(format!("{}(...)", name));
                                }
                                exprs.push(c.clone());
                            }
                            SqlExpr::WindowFunction { func, .. } => {
                                cols.push(format!("{}_over", func.to_lowercase()));
                                exprs.push(c.clone());
                            }
                            _ => {
                                cols.push("kolom".to_string());
                                exprs.push(c.clone());
                            }
                        }
                    }
                    (cols, exprs)
                };

                // Pre-evaluasi Window Functions pada matched_rows
                let mut window_computed: HashMap<(usize, usize), SqlValue> = HashMap::new(); // (expr_idx, row_idx) -> SqlValue
                for (expr_idx, expr) in project_exprs.iter().enumerate() {
                    let wf_opt = match expr {
                        SqlExpr::WindowFunction { func, args, partition_by, order_by } => Some((func, args, partition_by, order_by)),
                        SqlExpr::Alias { expr: inner, .. } => {
                            if let SqlExpr::WindowFunction { func, args, partition_by, order_by } = &**inner {
                                Some((func, args, partition_by, order_by))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    if let Some((func, args, partition_by, order_by)) = wf_opt {
                        // Partisi baris
                        let mut partitions: HashMap<String, Vec<usize>> = HashMap::new();
                        let mut partition_order: Vec<String> = Vec::new();

                        for (r_idx, r) in matched_rows.iter().enumerate() {
                            let part_key = if let Some(p_col) = partition_by {
                                self.eval_expr_value(&SqlExpr::Column(p_col.clone()), r)
                                    .map(|v| v.as_text())
                                    .unwrap_or_else(|_| "null".to_string())
                            } else {
                                "default_partition".to_string()
                            };

                            if !partitions.contains_key(&part_key) {
                                partition_order.push(part_key.clone());
                                partitions.insert(part_key.clone(), Vec::new());
                            }
                            partitions.get_mut(&part_key).unwrap().push(r_idx);
                        }

                        let func_upper = func.to_uppercase();

                        for part_key in &partition_order {
                            let part_indices = partitions.get_mut(part_key).unwrap();

                            // Urutkan di dalam partisi jika ada ORDER BY pada OVER
                            if let Some(ob) = order_by {
                                let get_sort_val = |r_idx: usize, col_name: &str| {
                                    let row = &matched_rows[r_idx];
                                    self.eval_expr_value(&SqlExpr::Column(col_name.to_string()), row).unwrap_or(SqlValue::Null)
                                };

                                part_indices.sort_by(|&idx_a, &idx_b| {
                                    let val_a = get_sort_val(idx_a, &ob.column);
                                    let val_b = get_sort_val(idx_b, &ob.column);
                                    let mut ord = compare_sql_values(&val_a, &val_b);
                                    if ob.is_desc { ord = ord.reverse(); }
                                    if ord == std::cmp::Ordering::Equal {
                                        for (s_col, s_desc) in &ob.secondary {
                                            let sa = get_sort_val(idx_a, s_col);
                                            let sb = get_sort_val(idx_b, s_col);
                                            let mut s_ord = compare_sql_values(&sa, &sb);
                                            if *s_desc { s_ord = s_ord.reverse(); }
                                            if s_ord != std::cmp::Ordering::Equal {
                                                return s_ord;
                                            }
                                        }
                                    }
                                    ord
                                });
                            }

                            // Hitung nilai window function untuk setiap baris di partisi ini
                            match func_upper.as_str() {
                                "ROW_NUMBER" | "NOMOR_BARIS" => {
                                    for (pos, &r_idx) in part_indices.iter().enumerate() {
                                        window_computed.insert((expr_idx, r_idx), SqlValue::Integer((pos + 1) as i64));
                                    }
                                }
                                "RANK" | "PERINGKAT" => {
                                    let mut current_rank = 1i64;
                                    let mut prev_val: Option<SqlValue> = None;
                                    for (pos, &r_idx) in part_indices.iter().enumerate() {
                                        let val = if let Some(ob) = order_by {
                                            self.eval_expr_value(&SqlExpr::Column(ob.column.clone()), &matched_rows[r_idx]).unwrap_or(SqlValue::Null)
                                        } else {
                                            SqlValue::Null
                                        };

                                        if pos > 0 && prev_val.as_ref() == Some(&val) {
                                            // Nilai sama, tetap di current_rank
                                        } else {
                                            current_rank = (pos + 1) as i64;
                                            prev_val = Some(val);
                                        }
                                        window_computed.insert((expr_idx, r_idx), SqlValue::Integer(current_rank));
                                    }
                                }
                                "DENSE_RANK" | "PERINGKAT_RAPAT" => {
                                    let mut current_rank = 0i64;
                                    let mut prev_val: Option<SqlValue> = None;
                                    for (pos, &r_idx) in part_indices.iter().enumerate() {
                                        let val = if let Some(ob) = order_by {
                                            self.eval_expr_value(&SqlExpr::Column(ob.column.clone()), &matched_rows[r_idx]).unwrap_or(SqlValue::Null)
                                        } else {
                                            SqlValue::Null
                                        };

                                        if pos == 0 || prev_val.as_ref() != Some(&val) {
                                            current_rank += 1;
                                            prev_val = Some(val);
                                        }
                                        window_computed.insert((expr_idx, r_idx), SqlValue::Integer(current_rank));
                                    }
                                }
                                "LEAD" => {
                                    let offset = if args.len() > 1 {
                                        match self.eval_expr_value(&args[1], &matched_rows[0]).unwrap_or(SqlValue::Null) {
                                            SqlValue::Integer(i) => i as usize,
                                            _ => 1,
                                        }
                                    } else { 1 };
                                    let default_val = if args.len() > 2 {
                                        self.eval_expr_value(&args[2], &matched_rows[0]).unwrap_or(SqlValue::Null)
                                    } else { SqlValue::Null };
                                    let arg_expr = args.get(0).cloned().unwrap_or(SqlExpr::Wildcard);
                                    for (pos, &r_idx) in part_indices.iter().enumerate() {
                                        let target_pos = pos + offset;
                                        let val = if target_pos < part_indices.len() {
                                            let target_idx = part_indices[target_pos];
                                            self.eval_expr_value(&arg_expr, &matched_rows[target_idx]).unwrap_or(SqlValue::Null)
                                        } else {
                                            default_val.clone()
                                        };
                                        window_computed.insert((expr_idx, r_idx), val);
                                    }
                                }
                                "LAG" => {
                                    let offset = if args.len() > 1 {
                                        match self.eval_expr_value(&args[1], &matched_rows[0]).unwrap_or(SqlValue::Null) {
                                            SqlValue::Integer(i) => i as usize,
                                            _ => 1,
                                        }
                                    } else { 1 };
                                    let default_val = if args.len() > 2 {
                                        self.eval_expr_value(&args[2], &matched_rows[0]).unwrap_or(SqlValue::Null)
                                    } else { SqlValue::Null };
                                    let arg_expr = args.get(0).cloned().unwrap_or(SqlExpr::Wildcard);
                                    for (pos, &r_idx) in part_indices.iter().enumerate() {
                                        let val = if pos >= offset {
                                            let target_idx = part_indices[pos - offset];
                                            self.eval_expr_value(&arg_expr, &matched_rows[target_idx]).unwrap_or(SqlValue::Null)
                                        } else {
                                            default_val.clone()
                                        };
                                        window_computed.insert((expr_idx, r_idx), val);
                                    }
                                }
                                "FIRST_VALUE" => {
                                    let arg_expr = args.get(0).cloned().unwrap_or(SqlExpr::Wildcard);
                                    let first_r_idx = part_indices.first().copied().unwrap_or(0);
                                    let first_val = self.eval_expr_value(&arg_expr, &matched_rows[first_r_idx]).unwrap_or(SqlValue::Null);
                                    for &r_idx in part_indices.iter() {
                                        window_computed.insert((expr_idx, r_idx), first_val.clone());
                                    }
                                }
                                "LAST_VALUE" => {
                                    let arg_expr = args.get(0).cloned().unwrap_or(SqlExpr::Wildcard);
                                    let last_r_idx = part_indices.last().copied().unwrap_or(0);
                                    let last_val = self.eval_expr_value(&arg_expr, &matched_rows[last_r_idx]).unwrap_or(SqlValue::Null);
                                    for &r_idx in part_indices.iter() {
                                        window_computed.insert((expr_idx, r_idx), last_val.clone());
                                    }
                                }
                                "NTILE" => {
                                    let num_buckets = if let Some(arg) = args.get(0) {
                                        match self.eval_expr_value(arg, &matched_rows[0]).unwrap_or(SqlValue::Null) {
                                            SqlValue::Integer(i) if i > 0 => i as usize,
                                            _ => 1,
                                        }
                                    } else { 1 };
                                    let total_rows = part_indices.len();
                                    for (pos, &r_idx) in part_indices.iter().enumerate() {
                                        let bucket = (pos * num_buckets / total_rows) + 1;
                                        window_computed.insert((expr_idx, r_idx), SqlValue::Integer(bucket as i64));
                                    }
                                }
                                "SUM" | "JUMLAH" => {
                                    let arg_expr = args.get(0).cloned().unwrap_or(SqlExpr::Wildcard);
                                    let mut total_sum = 0.0;
                                    for &r_idx in part_indices.iter() {
                                        match self.eval_expr_value(&arg_expr, &matched_rows[r_idx]).unwrap_or(SqlValue::Null) {
                                            SqlValue::Integer(i) => total_sum += i as f64,
                                            SqlValue::Float(f) => total_sum += f,
                                            _ => {}
                                        }
                                    }
                                    for &r_idx in part_indices.iter() {
                                        window_computed.insert((expr_idx, r_idx), SqlValue::Float(total_sum));
                                    }
                                }
                                "COUNT" | "HITUNG" => {
                                    let count = part_indices.len() as i64;
                                    for &r_idx in part_indices.iter() {
                                        window_computed.insert((expr_idx, r_idx), SqlValue::Integer(count));
                                    }
                                }
                                "AVG" | "RATA_RATA" => {
                                    let arg_expr = args.get(0).cloned().unwrap_or(SqlExpr::Wildcard);
                                    let mut total_sum = 0.0;
                                    let count = part_indices.len();
                                    for &r_idx in part_indices.iter() {
                                        match self.eval_expr_value(&arg_expr, &matched_rows[r_idx]).unwrap_or(SqlValue::Null) {
                                            SqlValue::Integer(i) => total_sum += i as f64,
                                            SqlValue::Float(f) => total_sum += f,
                                            _ => {}
                                        }
                                    }
                                    let avg = if count > 0 { total_sum / count as f64 } else { 0.0 };
                                    for &r_idx in part_indices.iter() {
                                        window_computed.insert((expr_idx, r_idx), SqlValue::Float(avg));
                                    }
                                }
                                _ => {
                                    for &r_idx in part_indices.iter() {
                                        window_computed.insert((expr_idx, r_idx), SqlValue::Null);
                                    }
                                }
                            }
                        }
                    }
                }

                let mut out_rows = Vec::new();
                for (r_idx, r) in matched_rows.iter().enumerate() {
                    let mut row_vals = Vec::new();
                    for (expr_idx, expr) in project_exprs.iter().enumerate() {
                        if let Some(win_val) = window_computed.get(&(expr_idx, r_idx)) {
                            row_vals.push(win_val.clone());
                        } else {
                            let val = self.eval_expr_value(expr, r).unwrap_or(SqlValue::Null);
                            row_vals.push(val);
                        }
                    }
                    out_rows.push(row_vals);
                }

                // 6. Deduplikasi BERBEDA / DISTINCT
                if is_distinct {
                    let mut unique_rows = Vec::new();
                    for r in out_rows {
                        if !unique_rows.contains(&r) {
                            unique_rows.push(r);
                        }
                    }
                    out_rows = unique_rows;
                }

                // 7. OFFSET & LIMIT
                let start = offset.unwrap_or(0);
                let end = match limit {
                    Some(lim) => (start + lim).min(out_rows.len()),
                    None => out_rows.len(),
                };
                let final_rows = if start < out_rows.len() {
                    out_rows[start..end].to_vec()
                } else {
                    Vec::new()
                };

                let len = final_rows.len();
                Ok(QueryResult::Rows {
                    columns: final_cols,
                    rows: final_rows,
                    affected: len,
                })
            }
            SqlStatement::Update { table, assignments, where_clause, returning } => {
                let upper_name = table.to_uppercase();
                let (_schema, all_schemas) = {
                    let cat = self.catalog.read().unwrap();
                    let s = cat.get(&upper_name)
                        .cloned()
                        .ok_or_else(|| format!("Tabel '{}' tidak ditemukan", table))?;
                    let all: Vec<TableSchema> = cat.values().cloned().collect();
                    (s, all)
                };

                // Validasi foreign keys pada assignments sebelum mengambil write lock
                for (col, val_expr) in &assignments {
                    let new_val = self.eval_literal(val_expr)?;
                    for fk in &_schema.foreign_keys {
                        if fk.column.eq_ignore_ascii_case(col) && new_val != SqlValue::Null {
                            let ref_table_upper = fk.foreign_table.to_uppercase();
                            let ref_col_upper = fk.foreign_column.to_uppercase();
                            let tbl_read = self.in_memory_tables.read().unwrap();
                            let ref_rows = tbl_read.get(&ref_table_upper)
                                .ok_or_else(|| format!("Tabel rujukan '{}' tidak ditemukan", fk.foreign_table))?;
                            let found = ref_rows.iter().any(|(h, row)| {
                                self.tx_mgr.is_visible(h, &snapshot, tx_id) &&
                                row.fields.get(&ref_col_upper).map_or(false, |v| v == &new_val)
                            });
                            if !found {
                                return Err(format!("Pelanggaran Kunci Asing (Foreign Key Violation): Nilai '{}' pada kolom '{}' tidak ditemukan di tabel rujukan '{}({})'", new_val.as_text(), fk.column, fk.foreign_table, fk.foreign_column));
                            }
                        }
                    }
                }

                let mut tables = self.in_memory_tables.write().unwrap();
                let rows_data = tables.get_mut(&upper_name)
                    .ok_or_else(|| format!("Data tabel '{}' belum diinisialisasi", table))?;

                let mut affected = 0;
                let mut new_versions = Vec::new();
                let mut cdc_updates = Vec::new();
                for (hdr, r) in rows_data.iter_mut() {
                    if self.tx_mgr.is_visible(hdr, &snapshot, tx_id) {
                        let matches_where = match &where_clause {
                            Some(expr) => self.eval_predicate(expr, r)?,
                            None => true,
                        };
                        if matches_where {
                            let mut new_r = r.clone();
                            for (col, val_expr) in &assignments {
                                let new_val = self.eval_literal(val_expr)?;
                                new_r.fields.insert(col.to_uppercase(), new_val);
                            }
                            hdr.xmax = tx_id; // Tandai versi lama
                            cdc_updates.push((r.fields.clone(), new_r.fields.clone()));
                            new_versions.push((MvccHeader { xmin: tx_id, xmax: 0 }, new_r));
                            affected += 1;
                        }
                    }
                }

                if !new_versions.is_empty() {
                    let sec_map = self.secondary_indexes.read().unwrap();
                    let idx_map = self.indices.read().unwrap();
                    let base_slot = rows_data.len();
                    for (v_idx, (_new_hdr, new_r)) in new_versions.iter().enumerate() {
                        let new_slot = (base_slot + v_idx) as u16;
                        // Perbarui indeks Primary Key jika ada
                        if let Some(pk_col) = &_schema.primary_key {
                            if let Some(pk_val) = new_r.fields.get(&pk_col.to_uppercase()) {
                                let idx_key = format!("{}_{}", upper_name, pk_col);
                                if let Some(idx) = idx_map.get(&idx_key) {
                                    let key = match pk_val {
                                        SqlValue::Integer(i) => IndexKey::Integer(*i),
                                        SqlValue::Text(s) => IndexKey::Text(s.clone()),
                                        _ => IndexKey::Text(pk_val.as_text()),
                                    };
                                    idx.insert(key, RecordPointer { page_id: _schema.root_page_id, slot_id: new_slot });
                                }
                            }
                        }
                        // Perbarui Indeks Sekunder
                        for (_idx_name, (tbl, col)) in sec_map.iter() {
                            if tbl == &upper_name {
                                if let Some(col_val) = new_r.fields.get(col) {
                                    let idx_key = format!("{}_{}", tbl, col);
                                    if let Some(idx) = idx_map.get(&idx_key) {
                                        let key = match col_val {
                                            SqlValue::Integer(i) => IndexKey::Integer(*i),
                                            SqlValue::Text(s) => IndexKey::Text(s.clone()),
                                            _ => IndexKey::Text(col_val.as_text()),
                                        };
                                        idx.insert(key, RecordPointer { page_id: _schema.root_page_id, slot_id: new_slot });
                                    }
                                }
                            }
                        }
                    }
                    rows_data.extend(new_versions.clone());
                }
                drop(tables);

                if affected > 0 {
                    let wal_lsn = self.wal.append(tx_id, WalOpType::Update, _schema.root_page_id, 0, Vec::new()).unwrap_or(0);
                    for (before_fields, after_fields) in &cdc_updates {
                        self.cdc.emit(CdcRecord::new(
                            wal_lsn,
                            tx_id,
                            upper_name.clone(),
                            CdcOp::Update,
                            Some(before_fields.clone()),
                            Some(after_fields.clone()),
                        ));
                    }

                    // Cascade Foreign Key Updates ke tabel-tabel anak
                    let mut tables = self.in_memory_tables.write().unwrap();
                    for child_schema in &all_schemas {
                        let child_tbl = child_schema.name.clone();
                        if child_tbl == upper_name { continue; }

                        for fk in &child_schema.foreign_keys {
                            if fk.foreign_table.eq_ignore_ascii_case(&upper_name) {
                                let parent_col = fk.foreign_column.to_uppercase();
                                let child_col = fk.column.to_uppercase();

                                for (before_f, after_f) in &cdc_updates {
                                    let old_val = before_f.get(&parent_col);
                                    let new_val = after_f.get(&parent_col);

                                    if let (Some(ov), Some(nv)) = (old_val, new_val) {
                                        if ov != nv {
                                            match fk.on_update {
                                                Some(CascadeAction::Cascade) => {
                                                    if let Some(child_rows) = tables.get_mut(&child_tbl) {
                                                        for (c_hdr, c_r) in child_rows.iter_mut() {
                                                            if self.tx_mgr.is_visible(c_hdr, &snapshot, tx_id) {
                                                                if c_r.fields.get(&child_col) == Some(ov) {
                                                                    let before = c_r.fields.clone();
                                                                    c_r.fields.insert(child_col.clone(), nv.clone());
                                                                    self.cdc.emit(CdcRecord::new(
                                                                        0,
                                                                        tx_id,
                                                                        child_tbl.clone(),
                                                                        CdcOp::Update,
                                                                        Some(before),
                                                                        Some(c_r.fields.clone()),
                                                                    ));
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some(CascadeAction::SetNull) => {
                                                    if let Some(child_rows) = tables.get_mut(&child_tbl) {
                                                        for (c_hdr, c_r) in child_rows.iter_mut() {
                                                            if self.tx_mgr.is_visible(c_hdr, &snapshot, tx_id) {
                                                                if c_r.fields.get(&child_col) == Some(ov) {
                                                                    let before = c_r.fields.clone();
                                                                    c_r.fields.insert(child_col.clone(), SqlValue::Null);
                                                                    self.cdc.emit(CdcRecord::new(
                                                                        0,
                                                                        tx_id,
                                                                        child_tbl.clone(),
                                                                        CdcOp::Update,
                                                                        Some(before),
                                                                        Some(c_r.fields.clone()),
                                                                    ));
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(ret_cols) = returning {
                    let mut ret_col_names = Vec::new();
                    for c in &ret_cols {
                        let name = match c {
                            SqlExpr::Alias { alias, .. } => alias.clone(),
                            SqlExpr::Column(n) => n.clone(),
                            _ => format!("{:?}", c),
                        };
                        ret_col_names.push(name);
                    }
                    let mut ret_rows = Vec::new();
                    for (_, nr) in &new_versions {
                        let mut row_vals = Vec::new();
                        for c in &ret_cols {
                            let val = self.eval_expr_value(c, nr).unwrap_or(SqlValue::Null);
                            row_vals.push(val);
                        }
                        ret_rows.push(row_vals);
                    }
                    Ok(QueryResult::Rows {
                        columns: ret_col_names,
                        rows: ret_rows,
                        affected,
                    })
                } else {
                    Ok(QueryResult::Rows {
                        columns: vec!["affected".to_string()],
                        rows: vec![vec![SqlValue::Integer(affected as i64)]],
                        affected,
                    })
                }
            }
            SqlStatement::Delete { table, where_clause, returning } => {
                let upper_name = table.to_uppercase();
                let (_schema, all_schemas) = {
                    let cat = self.catalog.read().unwrap();
                    let s = cat.get(&upper_name)
                        .cloned()
                        .ok_or_else(|| format!("Tabel '{}' tidak ditemukan", table))?;
                    let all: Vec<TableSchema> = cat.values().cloned().collect();
                    (s, all)
                };

                // 1. Identifikasi baris-baris target yang cocok dengan klausa WHERE
                let (matching_indices, deleted_rows) = {
                    let tables = self.in_memory_tables.read().unwrap();
                    let rows_data = tables.get(&upper_name)
                        .ok_or_else(|| format!("Data tabel '{}' belum diinisialisasi", table))?;

                    let mut indices = Vec::new();
                    let mut del_rows = Vec::new();
                    for (idx, (hdr, r)) in rows_data.iter().enumerate() {
                        if self.tx_mgr.is_visible(hdr, &snapshot, tx_id) {
                            let matches_where = match &where_clause {
                                Some(expr) => self.eval_predicate(expr, r)?,
                                None => true,
                            };
                            if matches_where {
                                indices.push(idx);
                                del_rows.push(r.clone());
                            }
                        }
                    }
                    (indices, del_rows)
                };

                let affected = matching_indices.len();

                if affected > 0 {
                    // 2. Validasi RESTRICT / NO ACTION dahulu sebelum memutasi apa pun
                    {
                        let tables = self.in_memory_tables.read().unwrap();
                        for child_schema in &all_schemas {
                            let child_tbl = &child_schema.name;
                            if child_tbl == &upper_name {
                                continue;
                            }

                            for fk in &child_schema.foreign_keys {
                                if fk.foreign_table.eq_ignore_ascii_case(&upper_name) {
                                    let is_restrict = match fk.on_delete {
                                        Some(CascadeAction::Restrict) | Some(CascadeAction::NoAction) | None => true,
                                        _ => false,
                                    };
                                    if is_restrict {
                                        let parent_col = fk.foreign_column.to_uppercase();
                                        let child_col = fk.column.to_uppercase();

                                        if let Some(child_rows) = tables.get(child_tbl) {
                                            for dr in &deleted_rows {
                                                if let Some(parent_val) = dr.fields.get(&parent_col) {
                                                    if *parent_val != SqlValue::Null {
                                                        let has_active_child = child_rows.iter().any(|(c_hdr, c_r)| {
                                                            self.tx_mgr.is_visible(c_hdr, &snapshot, tx_id)
                                                                && c_r.fields.get(&child_col) == Some(parent_val)
                                                        });
                                                        if has_active_child {
                                                            return Err(format!(
                                                                "Pelanggaran Kunci Asing (Foreign Key Violation): Baris pada '{}' tidak dapat dihapus karena masih dirujuk oleh tabel '{}'",
                                                                table, child_schema.name
                                                            ));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 3. Terapkan penghapusan pada parent dan cascade/set null pada anak
                    {
                        let mut tables = self.in_memory_tables.write().unwrap();
                        if let Some(rows_data) = tables.get_mut(&upper_name) {
                            for &idx in &matching_indices {
                                if idx < rows_data.len() {
                                    rows_data[idx].0.xmax = tx_id;
                                }
                            }
                        }

                        for child_schema in &all_schemas {
                            let child_tbl = &child_schema.name;
                            if child_tbl == &upper_name {
                                continue;
                            }

                            for fk in &child_schema.foreign_keys {
                                if fk.foreign_table.eq_ignore_ascii_case(&upper_name) {
                                    let parent_col = fk.foreign_column.to_uppercase();
                                    let child_col = fk.column.to_uppercase();

                                    for dr in &deleted_rows {
                                        if let Some(parent_val) = dr.fields.get(&parent_col) {
                                            if *parent_val != SqlValue::Null {
                                                match fk.on_delete {
                                                    Some(CascadeAction::Cascade) => {
                                                        if let Some(child_rows) = tables.get_mut(child_tbl) {
                                                            for (c_hdr, c_r) in child_rows.iter_mut() {
                                                                if self.tx_mgr.is_visible(c_hdr, &snapshot, tx_id) {
                                                                    if c_r.fields.get(&child_col) == Some(parent_val) {
                                                                        c_hdr.xmax = tx_id;
                                                                        self.cdc.emit(CdcRecord::new(
                                                                            0,
                                                                            tx_id,
                                                                            child_tbl.clone(),
                                                                            CdcOp::Delete,
                                                                            Some(c_r.fields.clone()),
                                                                            None,
                                                                        ));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Some(CascadeAction::SetNull) => {
                                                        if let Some(child_rows) = tables.get_mut(child_tbl) {
                                                            for (c_hdr, c_r) in child_rows.iter_mut() {
                                                                if self.tx_mgr.is_visible(c_hdr, &snapshot, tx_id) {
                                                                    if c_r.fields.get(&child_col) == Some(parent_val) {
                                                                        let before = c_r.fields.clone();
                                                                        c_r.fields.insert(child_col.clone(), SqlValue::Null);
                                                                        self.cdc.emit(CdcRecord::new(
                                                                            0,
                                                                            tx_id,
                                                                            child_tbl.clone(),
                                                                            CdcOp::Update,
                                                                            Some(before),
                                                                            Some(c_r.fields.clone()),
                                                                        ));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let wal_lsn = self.wal.append(tx_id, WalOpType::Delete, _schema.root_page_id, 0, Vec::new()).unwrap_or(0);
                    for dr in &deleted_rows {
                        self.cdc.emit(CdcRecord::new(
                            wal_lsn,
                            tx_id,
                            upper_name.clone(),
                            CdcOp::Delete,
                            Some(dr.fields.clone()),
                            None,
                        ));
                    }
                }

                if let Some(ret_cols) = returning {
                    let mut ret_col_names = Vec::new();
                    for c in &ret_cols {
                        let name = match c {
                            SqlExpr::Alias { alias, .. } => alias.clone(),
                            SqlExpr::Column(n) => n.clone(),
                            _ => format!("{:?}", c),
                        };
                        ret_col_names.push(name);
                    }
                    let mut ret_rows = Vec::new();
                    for dr in &deleted_rows {
                        let mut row_vals = Vec::new();
                        for c in &ret_cols {
                            let val = self.eval_expr_value(c, dr).unwrap_or(SqlValue::Null);
                            row_vals.push(val);
                        }
                        ret_rows.push(row_vals);
                    }
                    Ok(QueryResult::Rows {
                        columns: ret_col_names,
                        rows: ret_rows,
                        affected,
                    })
                } else {
                    Ok(QueryResult::Rows {
                        columns: vec!["affected".to_string()],
                        rows: vec![vec![SqlValue::Integer(affected as i64)]],
                        affected,
                    })
                }
            }
            SqlStatement::With { ctes, body } => {
                let mut ephemeral_tables = Vec::new();
                for cte in &ctes {
                    let res = self.execute(*cte.query.clone(), tx_id)?;
                    let (cols, rows) = match res {
                        QueryResult::Rows { columns, rows, .. } => (columns, rows),
                        _ => return Err(format!("CTE '{}' harus menghasilkan baris hasil", cte.name)),
                    };
                    let upper_cte = cte.name.to_uppercase();
                    let col_defs: Vec<(String, String)> = cols.iter().map(|c| (c.clone(), "TEXT".to_string())).collect();
                    let schema = TableSchema {
                        name: upper_cte.clone(),
                        columns: col_defs,
                        column_defs: Vec::new(),
                        primary_key: None,
                        foreign_keys: Vec::new(),
                        root_page_id: 0,
                        serial_counter: 0,
                    };
                    let mut row_list = Vec::new();
                    for r_vals in rows {
                        let mut fields = HashMap::new();
                        for (i, val) in r_vals.into_iter().enumerate() {
                            if i < cols.len() {
                                fields.insert(cols[i].to_uppercase(), val);
                            }
                        }
                        row_list.push((MvccHeader { xmin: tx_id, xmax: 0 }, Row { fields }));
                    }

                    self.catalog.write().unwrap().insert(upper_cte.clone(), schema);
                    self.in_memory_tables.write().unwrap().insert(upper_cte.clone(), row_list);
                    ephemeral_tables.push(upper_cte);
                }

                let result = self.execute(*body, tx_id);

                let mut cat_guard = self.catalog.write().unwrap();
                let mut tbl_guard = self.in_memory_tables.write().unwrap();
                for ep in ephemeral_tables {
                    cat_guard.remove(&ep);
                    tbl_guard.remove(&ep);
                }

                result
            }
            SqlStatement::CreateView { name, query, if_not_exists } => {
                let upper_name = name.to_uppercase();
                let mut v_guard = self.views.write().unwrap();
                if if_not_exists && v_guard.contains_key(&upper_name) {
                    return Ok(QueryResult::Message(format!("Tampilan (View) '{}' sudah ada, dilewati", name)));
                }
                v_guard.insert(upper_name, *query);
                Ok(QueryResult::Message(format!("Tampilan (View) '{}' berhasil dibuat", name)))
            }
            SqlStatement::DropView { name, if_exists } => {
                let upper_name = name.to_uppercase();
                let mut v_guard = self.views.write().unwrap();
                if v_guard.remove(&upper_name).is_some() {
                    Ok(QueryResult::Message(format!("Tampilan (View) '{}' berhasil dihapus", name)))
                } else if if_exists {
                    Ok(QueryResult::Message(format!("Tampilan (View) '{}' tidak ada, dilewati", name)))
                } else {
                    Err(format!("Tampilan (View) '{}' tidak ditemukan", name))
                }
            }
            SqlStatement::CreateMaterializedView { name, query, if_not_exists } => {
                let upper_name = name.to_uppercase();
                {
                    let mv_guard = self.materialized_views.read().unwrap();
                    if mv_guard.contains_key(&upper_name) {
                        if if_not_exists {
                            return Ok(QueryResult::Message(format!("View materialisasi '{}' sudah ada, dilewati (IF NOT EXISTS)", name)));
                        }
                        return Err(format!("View materialisasi '{}' sudah ada!", name));
                    }
                }

                // Eksekusi kueri dasarnya untuk materialisasi data awal
                let query_res = self.execute(*query.clone(), tx_id)?;
                let (cols, rows) = match query_res {
                    QueryResult::Rows { columns, rows, .. } => (columns, rows),
                    _ => return Err("Kueri pendefinisi view materialisasi tidak menghasilkan baris".to_string()),
                };

                let col_defs: Vec<(String, String)> = cols.iter().map(|c| (c.clone(), "TEXT".to_string())).collect();
                let schema = TableSchema {
                    name: upper_name.clone(),
                    columns: col_defs,
                    column_defs: Vec::new(),
                    primary_key: None,
                    foreign_keys: Vec::new(),
                    root_page_id: 0,
                    serial_counter: 0,
                };

                let synth_rows: Vec<(MvccHeader, Row)> = rows.into_iter().map(|row_vals| {
                    let mut fields = HashMap::new();
                    for (i, val) in row_vals.into_iter().enumerate() {
                        if i < cols.len() {
                            fields.insert(cols[i].to_uppercase(), val);
                        }
                    }
                    (MvccHeader { xmin: tx_id, xmax: 0 }, Row { fields })
                }).collect();

                self.catalog.write().unwrap().insert(upper_name.clone(), schema);
                self.in_memory_tables.write().unwrap().insert(upper_name.clone(), synth_rows);
                self.materialized_views.write().unwrap().insert(upper_name, *query);

                Ok(QueryResult::Message(format!("View materialisasi '{}' berhasil dibuat dan dimaterialisasikan", name)))
            }
            SqlStatement::RefreshMaterializedView { name } => {
                let upper_name = name.to_uppercase();
                let query = {
                    let mv_guard = self.materialized_views.read().unwrap();
                    mv_guard.get(&upper_name).cloned()
                        .ok_or_else(|| format!("View materialisasi '{}' tidak ditemukan", name))?
                };

                // Evaluasi ulang kueri dengan data terkini
                let query_res = self.execute(query, tx_id)?;
                let (cols, rows) = match query_res {
                    QueryResult::Rows { columns, rows, .. } => (columns, rows),
                    _ => return Err(format!("Kueri penyegaran view materialisasi '{}' tidak menghasilkan baris", name)),
                };

                let synth_rows: Vec<(MvccHeader, Row)> = rows.into_iter().map(|row_vals| {
                    let mut fields = HashMap::new();
                    for (i, val) in row_vals.into_iter().enumerate() {
                        if i < cols.len() {
                            fields.insert(cols[i].to_uppercase(), val);
                        }
                    }
                    (MvccHeader { xmin: tx_id, xmax: 0 }, Row { fields })
                }).collect();

                let mut tables = self.in_memory_tables.write().unwrap();
                if let Some(existing_rows) = tables.get_mut(&upper_name) {
                    *existing_rows = synth_rows;
                } else {
                    tables.insert(upper_name, synth_rows);
                }

                Ok(QueryResult::Message(format!("View materialisasi '{}' berhasil disegarkan (REFRESH)", name)))
            }
            SqlStatement::DropMaterializedView { name, if_exists } => {
                let upper_name = name.to_uppercase();
                let mut mv_guard = self.materialized_views.write().unwrap();
                if mv_guard.remove(&upper_name).is_some() {
                    self.catalog.write().unwrap().remove(&upper_name);
                    self.in_memory_tables.write().unwrap().remove(&upper_name);
                    Ok(QueryResult::Message(format!("View materialisasi '{}' berhasil dihapus", name)))
                } else if if_exists {
                    Ok(QueryResult::Message(format!("View materialisasi '{}' tidak ada, dilewati (IF EXISTS)", name)))
                } else {
                    Err(format!("View materialisasi '{}' tidak ditemukan", name))
                }
            }
            SqlStatement::CreateIndex { name, table, column, is_unique, if_not_exists } => {
                let upper_table = table.to_uppercase();
                let upper_col = column.to_uppercase();
                let upper_idx = name.to_uppercase();

                if if_not_exists {
                    let sec = self.secondary_indexes.read().unwrap();
                    if sec.contains_key(&upper_idx) {
                        return Ok(QueryResult::Message(format!("Indeks '{}' sudah ada, dilewati", name)));
                    }
                }

                let cat = self.catalog.read().unwrap();
                let schema = cat.get(&upper_table)
                    .ok_or_else(|| format!("Tabel '{}' tidak ditemukan", table))?;

                if !schema.columns.iter().any(|(c, _)| c.eq_ignore_ascii_case(&upper_col)) {
                    return Err(format!("Kolom '{}' tidak ditemukan pada tabel '{}'", column, table));
                }

                let btree = BTreeIndex::new(&column);
                let tables = self.in_memory_tables.read().unwrap();
                if let Some(row_list) = tables.get(&upper_table) {
                    let mut seen_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
                    for (slot_idx, (hdr, r)) in row_list.iter().enumerate() {
                        if self.tx_mgr.is_visible(hdr, &snapshot, tx_id) {
                            if let Some(val) = r.fields.get(&upper_col) {
                                if val != &SqlValue::Null {
                                    let key_str = val.as_text();
                                    if is_unique && seen_keys.contains(&key_str) {
                                        return Err(format!("Pelanggaran batasan UNIK: Nilai duplikat '{}' pada kolom '{}.{}'", key_str, table, column));
                                    }
                                    seen_keys.insert(key_str);
                                }
                                let key = match val {
                                    SqlValue::Integer(i) => IndexKey::Integer(*i),
                                    SqlValue::Text(t) => IndexKey::Text(t.clone()),
                                    _ => IndexKey::Text(val.as_text()),
                                };
                                btree.insert(key, RecordPointer { page_id: schema.root_page_id, slot_id: slot_idx as u16 });
                            }
                        }
                    }
                }
                drop(tables);
                drop(cat);

                let idx_key = format!("{}_{}", upper_table, upper_col);
                self.indices.write().unwrap().insert(idx_key, btree);
                self.secondary_indexes.write().unwrap().insert(upper_idx, (upper_table, upper_col));

                let unique_str = if is_unique { " unik" } else { "" };
                Ok(QueryResult::Message(format!("Indeks{} '{}' berhasil dibuat pada {}({})", unique_str, name, table, column)))
            }
            SqlStatement::DropIndex { name, if_exists } => {
                let upper_idx = name.to_uppercase();
                let mut sec = self.secondary_indexes.write().unwrap();
                if let Some((tbl, col)) = sec.remove(&upper_idx) {
                    let idx_key = format!("{}_{}", tbl, col);
                    self.indices.write().unwrap().remove(&idx_key);
                    Ok(QueryResult::Message(format!("Indeks '{}' berhasil dihapus", name)))
                } else if if_exists {
                    Ok(QueryResult::Message(format!("Indeks '{}' tidak ada, dilewati", name)))
                } else {
                    Err(format!("Indeks '{}' tidak ditemukan", name))
                }
            }
            SqlStatement::AlterTableAddColumn { table, column, col_type, not_null, default_value } => {
                let upper_table = table.to_uppercase();
                let mut cat = self.catalog.write().unwrap();
                let schema = cat.get_mut(&upper_table)
                    .ok_or_else(|| format!("Tabel '{}' tidak ditemukan", table))?;

                let upper_col = column.to_uppercase();
                if schema.columns.iter().any(|(c, _)| c.eq_ignore_ascii_case(&upper_col)) {
                    return Err(format!("Kolom '{}' sudah ada pada tabel '{}'", column, table));
                }
                schema.columns.push((column.clone(), col_type.clone()));
                schema.column_defs.push(ColumnDef {
                    name: column.clone(),
                    col_type: col_type.clone(),
                    not_null,
                    default_value: default_value.clone(),
                    is_unique: false,
                    is_serial: false,
                    is_primary_key: false,
                    references: None,
                });

                let def_val = if let Some(ref dv) = default_value {
                    if let Ok(i) = dv.parse::<i64>() {
                        SqlValue::Integer(i)
                    } else if let Ok(f) = dv.parse::<f64>() {
                        SqlValue::Float(f)
                    } else if dv.eq_ignore_ascii_case("true") || dv.eq_ignore_ascii_case("benar") {
                        SqlValue::Boolean(true)
                    } else if dv.eq_ignore_ascii_case("false") || dv.eq_ignore_ascii_case("salah") {
                        SqlValue::Boolean(false)
                    } else {
                        SqlValue::Text(dv.clone())
                    }
                } else {
                    SqlValue::Null
                };

                let mut tables = self.in_memory_tables.write().unwrap();
                if let Some(row_list) = tables.get_mut(&upper_table) {
                    for (_hdr, r) in row_list.iter_mut() {
                        r.fields.insert(upper_col.clone(), def_val.clone());
                    }
                }

                Ok(QueryResult::Message(format!("Kolom '{}' berhasil ditambahkan ke tabel '{}'", column, table)))
            }
            SqlStatement::AlterTableDropColumn { table, column } => {
                let upper_table = table.to_uppercase();
                let mut cat = self.catalog.write().unwrap();
                let schema = cat.get_mut(&upper_table)
                    .ok_or_else(|| format!("Tabel '{}' tidak ditemukan", table))?;

                let upper_col = column.to_uppercase();
                let orig_len = schema.columns.len();
                schema.columns.retain(|(c, _)| !c.eq_ignore_ascii_case(&upper_col));
                schema.column_defs.retain(|c| !c.name.eq_ignore_ascii_case(&upper_col));
                if schema.columns.len() == orig_len {
                    return Err(format!("Kolom '{}' tidak ditemukan pada tabel '{}'", column, table));
                }

                let mut tables = self.in_memory_tables.write().unwrap();
                if let Some(row_list) = tables.get_mut(&upper_table) {
                    for (_hdr, r) in row_list.iter_mut() {
                        r.fields.remove(&upper_col);
                    }
                }

                Ok(QueryResult::Message(format!("Kolom '{}' berhasil dihapus dari tabel '{}'", column, table)))
            }
            SqlStatement::AlterTableRenameColumn { table, old_name, new_name } => {
                let upper_table = table.to_uppercase();
                let upper_old = old_name.to_uppercase();
                let upper_new = new_name.to_uppercase();

                let mut cat = self.catalog.write().unwrap();
                let schema = cat.get_mut(&upper_table)
                    .ok_or_else(|| format!("Tabel '{}' tidak ditemukan", table))?;

                let mut found = false;
                for (c, _) in schema.columns.iter_mut() {
                    if c.eq_ignore_ascii_case(&upper_old) {
                        *c = new_name.clone();
                        found = true;
                    }
                }
                for cdef in schema.column_defs.iter_mut() {
                    if cdef.name.eq_ignore_ascii_case(&upper_old) {
                        cdef.name = new_name.clone();
                    }
                }
                if !found {
                    return Err(format!("Kolom '{}' tidak ditemukan pada tabel '{}'", old_name, table));
                }

                let mut tables = self.in_memory_tables.write().unwrap();
                if let Some(row_list) = tables.get_mut(&upper_table) {
                    for (_hdr, r) in row_list.iter_mut() {
                        if let Some(val) = r.fields.remove(&upper_old) {
                            r.fields.insert(upper_new.clone(), val);
                        }
                    }
                }

                Ok(QueryResult::Message(format!("Kolom '{}.{}' berhasil diganti namanya menjadi '{}'", table, old_name, new_name)))
            }
            SqlStatement::AlterTableChangeType { table, column, new_type } => {
                let upper_table = table.to_uppercase();
                let upper_col = column.to_uppercase();

                let mut cat = self.catalog.write().unwrap();
                let schema = cat.get_mut(&upper_table)
                    .ok_or_else(|| format!("Tabel '{}' tidak ditemukan", table))?;

                let mut found = false;
                for (c, t) in schema.columns.iter_mut() {
                    if c.eq_ignore_ascii_case(&upper_col) {
                        *t = new_type.clone();
                        found = true;
                    }
                }
                for cdef in schema.column_defs.iter_mut() {
                    if cdef.name.eq_ignore_ascii_case(&upper_col) {
                        cdef.col_type = new_type.clone();
                    }
                }
                if !found {
                    return Err(format!("Kolom '{}' tidak ditemukan pada tabel '{}'", column, table));
                }

                Ok(QueryResult::Message(format!("Tipe kolom '{}.{}' berhasil diubah menjadi '{}'", table, column, new_type)))
            }
            SqlStatement::TruncateTable { name } => {
                let upper_name = name.to_uppercase();
                let cat = self.catalog.read().unwrap();
                if !cat.contains_key(&upper_name) {
                    return Err(format!("Tabel '{}' tidak ditemukan", name));
                }
                drop(cat);

                let mut tables = self.in_memory_tables.write().unwrap();
                if let Some(rows_data) = tables.get_mut(&upper_name) {
                    rows_data.clear();
                }

                // Bersihkan indeks
                let mut idx_map = self.indices.write().unwrap();
                let sec_map = self.secondary_indexes.read().unwrap();
                for (idx_name, (tbl, col)) in sec_map.iter() {
                    if tbl == &upper_name {
                        let idx_key = format!("{}_{}", tbl, col);
                        idx_map.insert(idx_key, BTreeIndex::new(col));
                    }
                }

                let wal_lsn = self.wal.append(tx_id, WalOpType::Checkpoint, 0, 0, Vec::new()).unwrap_or(0);
                self.cdc.emit(CdcRecord::new(
                    wal_lsn,
                    tx_id,
                    upper_name.clone(),
                    CdcOp::Truncate,
                    None,
                    None,
                ));

                Ok(QueryResult::Message(format!("Tabel '{}' berhasil dikosongkan (TRUNCATE)", name)))
            }
            SqlStatement::VacuumDb { table } => {
                let mut tables = self.in_memory_tables.write().unwrap();
                let cat = self.catalog.read().unwrap();

                let target_tables: Vec<String> = if let Some(ref t) = table {
                    vec![t.to_uppercase()]
                } else {
                    cat.keys().cloned().collect()
                };

                let mut purged_count = 0;
                for t_name in target_tables {
                    if let Some(rows_data) = tables.get_mut(&t_name) {
                        let orig_len = rows_data.len();
                        rows_data.retain(|(hdr, _)| hdr.xmax == 0);
                        purged_count += orig_len - rows_data.len();
                    }
                }

                Ok(QueryResult::Message(format!("Operasi VACUUM selesai: {} baris mati (dead tuples) berhasil dipadatkan", purged_count)))
            }
            SqlStatement::ShowTables => {
                let cat = self.catalog.read().unwrap();
                let views = self.views.read().unwrap();

                let mut rows = Vec::new();
                for t in cat.keys() {
                    rows.push(vec![SqlValue::Text(t.clone()), SqlValue::Text("BASE TABLE".to_string())]);
                }
                for v in views.keys() {
                    rows.push(vec![SqlValue::Text(v.clone()), SqlValue::Text("VIEW".to_string())]);
                }
                let len = rows.len();
                Ok(QueryResult::Rows {
                    columns: vec!["table_name".to_string(), "table_type".to_string()],
                    rows,
                    affected: len,
                })
            }
            SqlStatement::ShowColumns { table } => {
                let upper_table = table.to_uppercase();
                let cat = self.catalog.read().unwrap();
                let schema = cat.get(&upper_table)
                    .ok_or_else(|| format!("Tabel '{}' tidak ditemukan", table))?;

                let mut rows = Vec::new();
                if !schema.column_defs.is_empty() {
                    for cdef in &schema.column_defs {
                        rows.push(vec![
                            SqlValue::Text(cdef.name.clone()),
                            SqlValue::Text(cdef.col_type.clone()),
                            SqlValue::Text(if cdef.not_null { "NO".to_string() } else { "YES".to_string() }),
                            cdef.default_value.as_ref().map(|d| SqlValue::Text(d.clone())).unwrap_or(SqlValue::Null),
                            SqlValue::Boolean(cdef.is_primary_key),
                            SqlValue::Boolean(cdef.is_unique),
                        ]);
                    }
                } else {
                    for (c, t) in &schema.columns {
                        let is_pk = schema.primary_key.as_ref().map_or(false, |pk| pk.eq_ignore_ascii_case(c));
                        rows.push(vec![
                            SqlValue::Text(c.clone()),
                            SqlValue::Text(t.clone()),
                            SqlValue::Text("YES".to_string()),
                            SqlValue::Null,
                            SqlValue::Boolean(is_pk),
                            SqlValue::Boolean(false),
                        ]);
                    }
                }
                let len = rows.len();
                Ok(QueryResult::Rows {
                    columns: vec!["column_name".to_string(), "data_type".to_string(), "is_nullable".to_string(), "column_default".to_string(), "is_primary_key".to_string(), "is_unique".to_string()],
                    rows,
                    affected: len,
                })
            }
            SqlStatement::AnalyzeTable { table } => {
                self.execute_analyze(table, tx_id)
            }
            SqlStatement::ExplainStatement { stmt, analyze } => {
                let plan_start = std::time::Instant::now();
                let cost_model = CostModel::default();
                let mut plan_lines = Vec::new();
                let mut root_scan_index: Option<usize> = None;

                match &*stmt {
                    SqlStatement::Select { from_table, join, joins, where_clause, order_by, group_by, .. } => {
                        let upper = from_table.to_uppercase();
                        let cat = self.catalog.read().unwrap();
                        let is_real_table = cat.contains_key(&upper);
                        drop(cat);

                        if is_real_table {
                            // Hitung biaya Scan dengan Cost-Based Optimizer (CBO)
                            let (_is_idx, scan_label, startup_cost, total_cost, est_rows, avg_width) =
                                self.calculate_scan_cost(from_table, where_clause.as_ref(), &cost_model);

                            root_scan_index = Some(plan_lines.len());
                            plan_lines.push(format!("->  {}  (cost={:.2}..{:.2} rows={} width={})", scan_label, startup_cost, total_cost, est_rows, avg_width));

                            let all_joins: Vec<&JoinClause> = if !joins.is_empty() {
                                joins.iter().collect()
                            } else if let Some(j) = join {
                                vec![j]
                            } else {
                                Vec::new()
                            };

                            for j in all_joins {
                                let (join_label, j_startup, j_total, j_rows, j_width) =
                                    self.calculate_join_cost(from_table, &j.table, &cost_model);
                                plan_lines.push(format!("->  {} on {}  (cost={:.2}..{:.2} rows={} width={})", join_label, j.table, j_startup, j_total, j_rows, j_width));
                            }

                            if !group_by.is_empty() {
                                let agg_startup = total_cost;
                                let agg_total = agg_startup + (est_rows as f64 * cost_model.cpu_operator_cost);
                                plan_lines.push(format!("->  HashAggregate on ({})  (cost={:.2}..{:.2} rows={} width={})", group_by.join(", "), agg_startup, agg_total, (est_rows / 2).max(1), avg_width));
                            }

                            if let Some(ob) = order_by {
                                let sort_startup = total_cost + 5.0;
                                let sort_total = sort_startup + ((est_rows as f64) * ((est_rows as f64).log2().max(1.0)) * cost_model.cpu_operator_cost);
                                plan_lines.push(format!("->  Sort by {} {}  (cost={:.2}..{:.2} rows={} width={})", ob.column, if ob.is_desc { "DESC" } else { "ASC" }, sort_startup, sort_total, est_rows, avg_width));
                            }
                        } else {
                            root_scan_index = Some(plan_lines.len());
                            plan_lines.push(format!("->  Result / Virtual Scan on {}  (cost=0.00..0.01 rows=1 width=32)", from_table));
                        }
                    }
                    SqlStatement::Insert { table, .. } => {
                        root_scan_index = Some(plan_lines.len());
                        plan_lines.push(format!("->  Insert on {}  (cost=0.00..0.05 rows=1 width=64)", table));
                    }
                    SqlStatement::Update { table, .. } => {
                        root_scan_index = Some(plan_lines.len());
                        plan_lines.push(format!("->  Update on {}  (cost=0.00..15.00 rows=1 width=64)", table));
                    }
                    SqlStatement::Delete { table, .. } => {
                        root_scan_index = Some(plan_lines.len());
                        plan_lines.push(format!("->  Delete on {}  (cost=0.00..15.00 rows=1 width=64)", table));
                    }
                    _ => {
                        plan_lines.push(format!("->  Execute statement: {:?}", stmt));
                    }
                }

                let plan_duration = plan_start.elapsed();

                if analyze {
                    let exec_start = std::time::Instant::now();
                    let exec_res = self.execute(*stmt, tx_id)?;
                    let exec_duration = exec_start.elapsed();

                    let actual_rows = match &exec_res {
                        QueryResult::Rows { rows, .. } => rows.len(),
                        QueryResult::Message(_) => 1,
                    };

                    let exec_ms = exec_duration.as_secs_f64() * 1000.0;
                    if let Some(idx) = root_scan_index {
                        if idx < plan_lines.len() {
                            let actual_str = format!(" (actual time=0.010..{:.3} rows={} loops=1)", exec_ms.max(0.012), actual_rows);
                            plan_lines[idx].push_str(&actual_str);
                        }
                    }

                    let plan_ms = (plan_duration.as_secs_f64() * 1000.0).max(0.025);
                    plan_lines.push(format!("Planning Time: {:.3} ms", plan_ms));
                    plan_lines.push(format!("Execution Time: {:.3} ms", exec_ms));
                }

                let rows: Vec<Vec<SqlValue>> = plan_lines.into_iter().map(|l| vec![SqlValue::Text(l)]).collect();
                let len = rows.len();
                Ok(QueryResult::Rows {
                    columns: vec!["QUERY PLAN".to_string()],
                    rows,
                    affected: len,
                })
            }
            SqlStatement::Savepoint { name } => {
                let tables = self.in_memory_tables.read().unwrap().clone();
                let mut sp_guard = self.savepoints.write().unwrap();
                sp_guard.insert((tx_id, name.to_uppercase()), tables);
                Ok(QueryResult::Message(format!("Titik simpan '{}' berhasil dibuat [TX ID: {}]", name, tx_id)))
            }
            SqlStatement::RollbackToSavepoint { name } => {
                let sp_guard = self.savepoints.read().unwrap();
                let snapshot_tables = sp_guard.get(&(tx_id, name.to_uppercase()))
                    .ok_or_else(|| format!("Titik simpan '{}' tidak ditemukan untuk TX ID {}", name, tx_id))?
                    .clone();
                drop(sp_guard);
                let mut tables = self.in_memory_tables.write().unwrap();
                *tables = snapshot_tables;
                Ok(QueryResult::Message(format!("Transaksi digugurkan kembali ke titik simpan '{}' [TX ID: {}]", name, tx_id)))
            }
            SqlStatement::ReleaseSavepoint { name } => {
                let mut sp_guard = self.savepoints.write().unwrap();
                if sp_guard.remove(&(tx_id, name.to_uppercase())).is_some() {
                    Ok(QueryResult::Message(format!("Titik simpan '{}' berhasil dilepas [TX ID: {}]", name, tx_id)))
                } else {
                    Err(format!("Titik simpan '{}' tidak ditemukan", name))
                }
            }

            // ========================================================================
            // EKSEKUSI TABEL ANALITIK KOLUMNAR (ClickHouse-Compatible)
            // ========================================================================
            SqlStatement::CreateColumnarTable { name, columns, engine, order_by, if_not_exists } => {
                if self.columnar_engine.has_table(&name) {
                    if if_not_exists {
                        return Ok(QueryResult::Message(format!(
                            "Tabel kolumnar '{}' sudah ada, pembuatan dilewati (IF NOT EXISTS)", name
                        )));
                    }
                    return Err(format!("Tabel kolumnar '{}' sudah ada!", name));
                }

                let mut schema = HashMap::new();
                for (col_name, col_type) in columns {
                    schema.insert(col_name, crate::columnar::ColumnarDataType::from_type_str(&col_type));
                }
                self.columnar_engine.create_table(&name, schema, order_by)?;
                Ok(QueryResult::Message(format!(
                    "Tabel Kolumnar '{}' berhasil dibuat dengan mesin '{}' (ClickHouse-Compatible Columnar Engine)",
                    name, engine
                )))
            }
            SqlStatement::InsertColumnar { table, columns, rows } => {
                let inserted = self.columnar_engine.insert(&table, &columns, rows)?;
                Ok(QueryResult::Message(format!("{} baris berhasil disisipkan ke tabel kolumnar '{}'", inserted, table)))
            }
            SqlStatement::ColumnarAggregation { table, aggregates, group_by, where_col, order_by, limit } => {
                let (cols, rows) = self.columnar_engine.aggregate(&table, &aggregates, &group_by, where_col.as_ref(), order_by.as_ref(), limit)?;
                let len = rows.len();
                Ok(QueryResult::Rows {
                    columns: cols,
                    rows,
                    affected: len,
                })
            }

            // ========================================================================
            // EKSEKUSI IN-MEMORY CACHE & KV (Redis-Compatible)
            // ========================================================================
            SqlStatement::CacheSet { key, value, ttl_secs } => {
                self.cache_engine.set(&key, &value, ttl_secs);
                Ok(QueryResult::Message("OK".to_string()))
            }
            SqlStatement::CacheGet { key } => {
                if let Some(val) = self.cache_engine.get(&key) {
                    Ok(QueryResult::Rows {
                        columns: vec!["value".to_string()],
                        rows: vec![vec![SqlValue::Text(val)]],
                        affected: 1,
                    })
                } else {
                    Ok(QueryResult::Message("(nil)".to_string()))
                }
            }
            SqlStatement::CacheDel { key } => {
                let count = self.cache_engine.del(&key);
                Ok(QueryResult::Message(format!("(integer) {}", count)))
            }
            SqlStatement::CacheExpire { key, ttl_secs } => {
                let ok = self.cache_engine.expire(&key, ttl_secs);
                Ok(QueryResult::Message(format!("(integer) {}", if ok { 1 } else { 0 })))
            }
            SqlStatement::CacheTtl { key } => {
                let ttl = self.cache_engine.ttl(&key);
                Ok(QueryResult::Message(format!("(integer) {}", ttl)))
            }
            SqlStatement::CacheIncr { key, by } => {
                let val = self.cache_engine.incr_by(&key, by).map_err(|e| e.to_string())?;
                Ok(QueryResult::Message(format!("(integer) {}", val)))
            }
            SqlStatement::CacheHSet { key, field, value } => {
                let is_new = self.cache_engine.hset(&key, &field, &value);
                Ok(QueryResult::Message(format!("(integer) {}", if is_new { 1 } else { 0 })))
            }
            SqlStatement::CacheHGet { key, field } => {
                if let Some(val) = self.cache_engine.hget(&key, &field) {
                    Ok(QueryResult::Rows {
                        columns: vec!["field_value".to_string()],
                        rows: vec![vec![SqlValue::Text(val)]],
                        affected: 1,
                    })
                } else {
                    Ok(QueryResult::Message("(nil)".to_string()))
                }
            }
            SqlStatement::CacheHGetAll { key } => {
                let map = self.cache_engine.hgetall(&key);
                let mut rows = Vec::new();
                for (f, v) in map {
                    rows.push(vec![SqlValue::Text(f), SqlValue::Text(v)]);
                }
                let len = rows.len();
                Ok(QueryResult::Rows {
                    columns: vec!["field".to_string(), "value".to_string()],
                    rows,
                    affected: len,
                })
            }
            SqlStatement::CacheLPush { key, values } => {
                let mut len = 0;
                for v in &values {
                    len = self.cache_engine.lpush(&key, v);
                }
                Ok(QueryResult::Message(format!("(integer) {}", len)))
            }
            SqlStatement::CacheRPush { key, values } => {
                let mut len = 0;
                for v in &values {
                    len = self.cache_engine.rpush(&key, v);
                }
                Ok(QueryResult::Message(format!("(integer) {}", len)))
            }
            SqlStatement::CacheLPop { key } => {
                if let Some(val) = self.cache_engine.lpop(&key) {
                    Ok(QueryResult::Rows {
                        columns: vec!["value".to_string()],
                        rows: vec![vec![SqlValue::Text(val)]],
                        affected: 1,
                    })
                } else {
                    Ok(QueryResult::Message("(nil)".to_string()))
                }
            }
            SqlStatement::CacheRPop { key } => {
                if let Some(val) = self.cache_engine.rpop(&key) {
                    Ok(QueryResult::Rows {
                        columns: vec!["value".to_string()],
                        rows: vec![vec![SqlValue::Text(val)]],
                        affected: 1,
                    })
                } else {
                    Ok(QueryResult::Message("(nil)".to_string()))
                }
            }
            SqlStatement::CacheLRange { key, start, stop } => {
                let items = self.cache_engine.lrange(&key, start as isize, stop as isize);
                let rows: Vec<Vec<SqlValue>> = items.into_iter().map(|item| vec![SqlValue::Text(item)]).collect();
                let len = rows.len();
                Ok(QueryResult::Rows {
                    columns: vec!["element".to_string()],
                    rows,
                    affected: len,
                })
            }
            SqlStatement::CacheSAdd { key, members } => {
                let mut added = 0;
                for m in &members {
                    if self.cache_engine.sadd(&key, m) {
                        added += 1;
                    }
                }
                Ok(QueryResult::Message(format!("(integer) {}", added)))
            }
            SqlStatement::CacheSMembers { key } => {
                let members = self.cache_engine.smembers(&key);
                let rows: Vec<Vec<SqlValue>> = members.into_iter().map(|m| vec![SqlValue::Text(m)]).collect();
                let len = rows.len();
                Ok(QueryResult::Rows {
                    columns: vec!["member".to_string()],
                    rows,
                    affected: len,
                })
            }
            SqlStatement::CacheSRem { key, members } => {
                let mut removed = 0;
                for m in &members {
                    if self.cache_engine.srem(&key, m) {
                        removed += 1;
                    }
                }
                Ok(QueryResult::Message(format!("(integer) {}", removed)))
            }
            SqlStatement::CacheZAdd { key, score, member } => {
                let is_new = self.cache_engine.zadd(&key, score as i64, &member);
                Ok(QueryResult::Message(format!("(integer) {}", if is_new { 1 } else { 0 })))
            }
            SqlStatement::CacheZRange { key, start, stop, with_scores } => {
                let start_idx = if start < 0 { 0 } else { start as usize };
                let stop_idx = if stop < 0 { usize::MAX } else { stop as usize };
                let items = self.cache_engine.zrange(&key, start_idx, stop_idx);
                let mut rows = Vec::new();
                for (member, score) in items {
                    if with_scores {
                        rows.push(vec![SqlValue::Text(member), SqlValue::Integer(score)]);
                    } else {
                        rows.push(vec![SqlValue::Text(member)]);
                    }
                }
                let cols = if with_scores {
                    vec!["member".to_string(), "score".to_string()]
                } else {
                    vec!["member".to_string()]
                };
                let len = rows.len();
                Ok(QueryResult::Rows {
                    columns: cols,
                    rows,
                    affected: len,
                })
            }
            SqlStatement::CachePublish { channel, message } => {
                let receivers = self.cache_engine.publish(&channel, &message);
                Ok(QueryResult::Message(format!("(integer) {}", receivers)))
            }
        }
    }

    /// Mengeksekusi ANALYZE / ANALISIS_TABEL untuk memperbarui statistik katalog tabel dan kolom
    pub fn execute_analyze(&self, target_table: Option<String>, tx_id: u64) -> Result<QueryResult, String> {
        let cat = self.catalog.read().unwrap();
        let target_tables: Vec<String> = if let Some(ref t) = target_table {
            let up = t.to_uppercase();
            if !cat.contains_key(&up) {
                return Err(format!("Tabel '{}' tidak ditemukan untuk dianalisis", t));
            }
            vec![up]
        } else {
            cat.keys().cloned().collect()
        };

        let mut analyzed_info = Vec::new();

        for t_name in target_tables {
            let schema = cat.get(&t_name).cloned().unwrap();
            let tables = self.in_memory_tables.read().unwrap();
            let all_rows = tables.get(&t_name).cloned().unwrap_or_default();
            drop(tables);

            // Filter baris yang aktif / terlihat pada transaksi saat ini
            let visible_rows: Vec<Row> = all_rows.into_iter().filter_map(|(hdr, row)| {
                if hdr.xmin <= tx_id && (hdr.xmax == 0 || hdr.xmax > tx_id) {
                    Some(row)
                } else {
                    None
                }
            }).collect();

            let row_count = visible_rows.len();
            let avg_row_size = if row_count > 0 {
                let sample_count = visible_rows.len().min(100);
                let total_sample_bytes: usize = visible_rows.iter().take(sample_count).map(|r| {
                    r.fields.iter().map(|(k, v)| k.len() + v.as_text().len() + 8).sum::<usize>()
                }).sum();
                (total_sample_bytes / sample_count).max(32)
            } else {
                32
            };
            let page_count = ((row_count * avg_row_size + 4095) / 4096).max(1);

            let mut col_stats_map = HashMap::new();

            for (col_name, _col_type) in &schema.columns {
                let col_upper = col_name.to_uppercase();
                let mut null_count = 0;
                let mut distinct_map: HashMap<String, usize> = HashMap::new();
                let mut val_list: Vec<SqlValue> = Vec::new();
                let mut min_val: Option<SqlValue> = None;
                let mut max_val: Option<SqlValue> = None;

                for row in &visible_rows {
                    match row.fields.get(&col_upper) {
                        None | Some(SqlValue::Null) => {
                            null_count += 1;
                        }
                        Some(val) => {
                            let val_str = val.as_text();
                            *distinct_map.entry(val_str).or_insert(0) += 1;
                            val_list.push(val.clone());

                            if let Some(ref current_min) = min_val {
                                if compare_sql_values(val, current_min) == std::cmp::Ordering::Less {
                                    min_val = Some(val.clone());
                                }
                            } else {
                                min_val = Some(val.clone());
                            }

                            if let Some(ref current_max) = max_val {
                                if compare_sql_values(val, current_max) == std::cmp::Ordering::Greater {
                                    max_val = Some(val.clone());
                                }
                            } else {
                                max_val = Some(val.clone());
                            }
                        }
                    }
                }

                let mut mcv_vec: Vec<(SqlValue, usize)> = Vec::new();
                for (val_str, count) in &distinct_map {
                    if let Some(first_match) = val_list.iter().find(|v| v.as_text() == *val_str) {
                        mcv_vec.push((first_match.clone(), *count));
                    }
                }
                mcv_vec.sort_by(|a, b| b.1.cmp(&a.1));
                mcv_vec.truncate(10); // Simpan top 10 Most Common Values

                // Korelasi urutan fisik vs urutan nilai
                let correlation = if val_list.len() > 1 {
                    let mut sorted = val_list.clone();
                    sorted.sort_by(|a, b| compare_sql_values(a, b));
                    let mut concordant = 0;
                    for (i, v) in val_list.iter().enumerate() {
                        if v == &sorted[i] {
                            concordant += 1;
                        }
                    }
                    (concordant as f64) / (val_list.len() as f64)
                } else {
                    1.0
                };

                col_stats_map.insert(col_upper, ColumnStats {
                    column_name: col_name.clone(),
                    null_count,
                    distinct_count: distinct_map.len(),
                    min_val,
                    max_val,
                    most_common_vals: mcv_vec,
                    correlation,
                });
            }

            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            let t_stats = TableStats {
                table_name: t_name.clone(),
                row_count,
                page_count,
                avg_row_size,
                last_analyzed: now,
                columns: col_stats_map,
            };

            let col_len = schema.columns.len();
            self.table_stats.write().unwrap().insert(t_name.clone(), t_stats);
            analyzed_info.push(format!("Tabel '{}' ({} baris, {} halaman, {} kolom)", t_name, row_count, page_count, col_len));
        }

        Ok(QueryResult::Message(format!(
            "Operasi ANALISIS_TABEL selesai: {}",
            analyzed_info.join("; ")
        )))
    }

    /// Menghitung estimasi selektivitas ekspresi filter WHERE
    pub fn estimate_selectivity(&self, table: &str, expr: &SqlExpr) -> f64 {
        let stats_guard = self.table_stats.read().unwrap();
        let t_stats = stats_guard.get(&table.to_uppercase());

        match expr {
            SqlExpr::Binary { left, op, right } => {
                let col_name = match (&**left, &**right) {
                    (SqlExpr::Column(c), _) => Some(c.as_str()),
                    (_, SqlExpr::Column(c)) => Some(c.as_str()),
                    _ => None,
                };

                if let Some(raw_col) = col_name {
                    let col_clean = if let Some(dot_pos) = raw_col.find('.') {
                        &raw_col[dot_pos + 1..]
                    } else {
                        raw_col
                    };
                    let col_stats = t_stats.and_then(|ts| ts.columns.get(&col_clean.to_uppercase()));

                    match op {
                        BinaryOp::Eq => {
                            if let Some(cs) = col_stats {
                                if cs.distinct_count > 0 {
                                    return (1.0 / cs.distinct_count as f64).min(1.0);
                                }
                            }
                            0.05
                        }
                        BinaryOp::Neq => {
                            if let Some(cs) = col_stats {
                                if cs.distinct_count > 0 {
                                    return (1.0 - (1.0 / cs.distinct_count as f64)).max(0.0);
                                }
                            }
                            0.95
                        }
                        BinaryOp::Gt | BinaryOp::Lt | BinaryOp::Gte | BinaryOp::Lte => 0.33,
                        BinaryOp::Like => 0.10,
                        BinaryOp::NotLike => 0.90,
                        _ => 0.50,
                    }
                } else {
                    0.50
                }
            }
            SqlExpr::Between { .. } => 0.20,
            SqlExpr::InList { list, negated, .. } => {
                let sel = (list.len() as f64 * 0.05).min(0.9);
                if *negated { 1.0 - sel } else { sel }
            }
            SqlExpr::IsNull { negated, .. } => {
                if *negated { 0.95 } else { 0.05 }
            }
            _ => 0.50,
        }
    }

    /// Menghitung biaya Scan tabel (Index Scan vs Sequential Scan) menggunakan CostModel
    pub fn calculate_scan_cost(
        &self,
        table: &str,
        where_clause: Option<&SqlExpr>,
        cost_model: &CostModel,
    ) -> (bool, String, f64, f64, usize, usize) {
        let upper_name = table.to_uppercase();
        let cat = self.catalog.read().unwrap();
        let schema = cat.get(&upper_name).cloned();
        drop(cat);

        let stats_guard = self.table_stats.read().unwrap();
        let t_stats = stats_guard.get(&upper_name).cloned();
        drop(stats_guard);

        let (row_count, page_count, avg_width) = if let Some(ts) = t_stats {
            (ts.row_count, ts.page_count, ts.avg_row_size)
        } else {
            let tables = self.in_memory_tables.read().unwrap();
            let rc = tables.get(&upper_name).map(|r| r.len()).unwrap_or(10);
            let pc = ((rc * 64 + 4095) / 4096).max(1);
            (rc, pc, 64)
        };

        let selectivity = where_clause.map(|w| self.estimate_selectivity(table, w)).unwrap_or(1.0);
        let est_rows = ((row_count as f64) * selectivity).ceil().max(1.0) as usize;

        // Biaya Seq Scan
        let seq_startup = 0.0;
        let num_evals = if where_clause.is_some() { 1.0 } else { 0.0 };
        let seq_total = (page_count as f64 * cost_model.seq_page_cost)
            + (row_count as f64 * cost_model.cpu_tuple_cost)
            + (row_count as f64 * num_evals * cost_model.cpu_operator_cost);

        // Cek apakah ada indeks yang dapat digunakan oleh WHERE clause
        let mut index_choice: Option<(String, String)> = None;

        if let Some(SqlExpr::Binary { left, op: BinaryOp::Eq, right }) = where_clause {
            let col_name_opt = match (&**left, &**right) {
                (SqlExpr::Column(c), _) => Some(c.clone()),
                (_, SqlExpr::Column(c)) => Some(c.clone()),
                _ => None,
            };

            if let Some(col_raw) = col_name_opt {
                let col = if let Some(dot_pos) = col_raw.find('.') {
                    col_raw[dot_pos + 1..].to_string()
                } else {
                    col_raw
                };

                // Periksa Primary Key
                if let Some(ref s) = schema {
                    if let Some(ref pk) = s.primary_key {
                        if pk.eq_ignore_ascii_case(&col) {
                            index_choice = Some((format!("pk_{}", upper_name.to_lowercase()), pk.clone()));
                        }
                    }
                }

                // Periksa Secondary Indexes
                if index_choice.is_none() {
                    let sec_map = self.secondary_indexes.read().unwrap();
                    for (idx_name, (tbl, c)) in sec_map.iter() {
                        if tbl == &upper_name && c.eq_ignore_ascii_case(&col) {
                            index_choice = Some((idx_name.clone(), c.clone()));
                            break;
                        }
                    }
                }
            }
        }

        if let Some((idx_name, _col)) = index_choice {
            // Hitung Biaya Index Scan
            let idx_startup = 0.15;
            let est_pages = ((page_count as f64) * selectivity).ceil().min(est_rows as f64).max(1.0);
            let idx_total = idx_startup
                + (est_pages * cost_model.random_page_cost)
                + (est_rows as f64 * (cost_model.cpu_index_tuple_cost + cost_model.cpu_tuple_cost));

            // Cost-Based Optimizer membandingkan Index Scan vs Seq Scan
            if idx_total < seq_total {
                let label = format!("Index Scan using {} on {}", idx_name, table);
                return (true, label, idx_startup, idx_total, est_rows, avg_width);
            }
        }

        let label = format!("Seq Scan on {}", table);
        (false, label, seq_startup, seq_total, est_rows, avg_width)
    }

    /// Menghitung biaya JOIN (Nested Loop Join vs Hash Join)
    pub fn calculate_join_cost(
        &self,
        outer_table: &str,
        inner_table: &str,
        cost_model: &CostModel,
    ) -> (String, f64, f64, usize, usize) {
        let stats_guard = self.table_stats.read().unwrap();
        let outer_rows = stats_guard.get(&outer_table.to_uppercase()).map(|s| s.row_count).unwrap_or(10);
        let inner_rows = stats_guard.get(&inner_table.to_uppercase()).map(|s| s.row_count).unwrap_or(10);
        drop(stats_guard);

        let est_rows = (outer_rows * inner_rows / 10).max(1);
        let avg_width = 128;

        let nested_startup = 0.0;
        let nested_total = (outer_rows as f64 * cost_model.cpu_tuple_cost)
            + (outer_rows as f64 * inner_rows as f64 * cost_model.cpu_operator_cost);

        let hash_startup = (inner_rows as f64 * cost_model.cpu_operator_cost) + 5.0;
        let hash_total = hash_startup + ((outer_rows + inner_rows) as f64 * cost_model.cpu_operator_cost);

        if nested_total < hash_total {
            ("Nested Loop Join".to_string(), nested_startup, nested_total, est_rows, avg_width)
        } else {
            ("Hash Join".to_string(), hash_startup, hash_total, est_rows, avg_width)
        }
    }

    fn eval_literal(&self, expr: &SqlExpr) -> Result<SqlValue, String> {
        match expr {
            SqlExpr::Literal(val) => Ok(val.clone()),
            SqlExpr::Column(col) => Ok(SqlValue::Text(col.clone())),
            SqlExpr::Alias { expr, .. } => self.eval_literal(expr),
            _ => Err("Diharapkan ekspresi literal nilai".to_string()),
        }
    }

    fn eval_predicate(&self, expr: &SqlExpr, row: &Row) -> Result<bool, String> {
        match expr {
            SqlExpr::Binary { left, op, right } => {
                match op {
                    BinaryOp::And => {
                        Ok(self.eval_predicate(left, row)? && self.eval_predicate(right, row)?)
                    }
                    BinaryOp::Or => {
                        Ok(self.eval_predicate(left, row)? || self.eval_predicate(right, row)?)
                    }
                    BinaryOp::Like => {
                        let left_val = self.eval_expr_value(left, row)?;
                        let right_val = self.eval_expr_value(right, row)?;
                        let text = match &left_val {
                            SqlValue::Text(t) => t.as_str(),
                            _ => "",
                        };
                        let pat = match &right_val {
                            SqlValue::Text(p) => p.as_str(),
                            _ => "",
                        };
                        Ok(matches_like(text, pat))
                    }
                    BinaryOp::NotLike => {
                        let left_val = self.eval_expr_value(left, row)?;
                        let right_val = self.eval_expr_value(right, row)?;
                        let text = match &left_val {
                            SqlValue::Text(t) => t.as_str(),
                            _ => "",
                        };
                        let pat = match &right_val {
                            SqlValue::Text(p) => p.as_str(),
                            _ => "",
                        };
                        Ok(!matches_like(text, pat))
                    }
                    _ => {
                        let left_val = self.eval_expr_value(left, row)?;
                        let right_val = self.eval_expr_value(right, row)?;
                        Ok(self.compare_values(&left_val, op, &right_val))
                    }
                }
            }
            SqlExpr::Between { expr, low, high, negated } => {
                let val = self.eval_expr_value(expr, row)?;
                let l_val = self.eval_expr_value(low, row)?;
                let h_val = self.eval_expr_value(high, row)?;
                let in_range = self.compare_values(&val, &BinaryOp::Gte, &l_val) && self.compare_values(&val, &BinaryOp::Lte, &h_val);
                Ok(if *negated { !in_range } else { in_range })
            }
            SqlExpr::InList { expr, list, negated } => {
                let val = self.eval_expr_value(expr, row)?;
                let mut matched = false;
                for item in list {
                    let item_val = self.eval_expr_value(item, row)?;
                    if val == item_val {
                        matched = true;
                        break;
                    }
                }
                Ok(if *negated { !matched } else { matched })
            }
            SqlExpr::IsNull { expr: inner, negated } => {
                let val = self.eval_expr_value(inner, row)?;
                let is_null = val == SqlValue::Null;
                Ok(if *negated { !is_null } else { is_null })
            }
            SqlExpr::Literal(SqlValue::Boolean(b)) => Ok(*b),
            _ => {
                let val = self.eval_expr_value(expr, row)?;
                match val {
                    SqlValue::Boolean(b) => Ok(b),
                    SqlValue::Null => Ok(false),
                    _ => Ok(true),
                }
            }
        }
    }

    fn eval_expr_value(&self, expr: &SqlExpr, row: &Row) -> Result<SqlValue, String> {
        match expr {
            SqlExpr::Alias { expr, .. } => self.eval_expr_value(expr, row),
            SqlExpr::Column(col) => {
                let up = col.to_uppercase();
                if let Some(val) = row.fields.get(&up) {
                    return Ok(val.clone());
                }
                // Jika dicari tanpa prefix tabel (misal "NAMA"), cari field yang berakhiran ".NAMA"
                for (k, v) in &row.fields {
                    if k.ends_with(&format!(".{}", up)) {
                        return Ok(v.clone());
                    }
                }
                // Jika dicari dengan prefix tabel (misal "PESANAN.ID"), tapi field hanya disimpan sebagai "ID"
                if let Some(dot_pos) = up.find('.') {
                    let col_only = &up[dot_pos + 1..];
                    if let Some(val) = row.fields.get(col_only) {
                        return Ok(val.clone());
                    }
                }
                Ok(SqlValue::Null)
            }
            SqlExpr::Literal(val) => Ok(val.clone()),
            SqlExpr::IsNull { expr: inner, negated } => {
                let val = self.eval_expr_value(inner, row)?;
                let is_null = val == SqlValue::Null;
                Ok(SqlValue::Boolean(if *negated { !is_null } else { is_null }))
            }
            SqlExpr::Case { conditions, else_branch } => {
                for (cond, then_expr) in conditions {
                    if self.eval_predicate(cond, row)? {
                        return self.eval_expr_value(then_expr, row);
                    }
                }
                if let Some(else_expr) = else_branch {
                    self.eval_expr_value(else_expr, row)
                } else {
                    Ok(SqlValue::Null)
                }
            }
            SqlExpr::Binary { left, op, right } => {
                let l_val = self.eval_expr_value(left, row)?;
                let r_val = self.eval_expr_value(right, row)?;
                match op {
                    BinaryOp::Add => self.eval_math_op(&l_val, &r_val, |a, b| a + b, |a, b| a + b),
                    BinaryOp::Sub => self.eval_math_op(&l_val, &r_val, |a, b| a - b, |a, b| a - b),
                    BinaryOp::Mul => self.eval_math_op(&l_val, &r_val, |a, b| a * b, |a, b| a * b),
                    BinaryOp::Div => {
                        let b_num = match r_val {
                            SqlValue::Integer(i) => i as f64,
                            SqlValue::Float(f) => f,
                            _ => return Ok(SqlValue::Null),
                        };
                        if b_num == 0.0 {
                            Ok(SqlValue::Null)
                        } else {
                            let a_num = match l_val {
                                SqlValue::Integer(i) => i as f64,
                                SqlValue::Float(f) => f,
                                _ => return Ok(SqlValue::Null),
                            };
                            let res = a_num / b_num;
                            if res.fract() == 0.0 {
                                Ok(SqlValue::Integer(res as i64))
                            } else {
                                Ok(SqlValue::Float(res))
                            }
                        }
                    }
                    BinaryOp::Like => {
                        let text = match &l_val { SqlValue::Text(t) => t.as_str(), _ => "" };
                        let pat = match &r_val { SqlValue::Text(p) => p.as_str(), _ => "" };
                        Ok(SqlValue::Boolean(matches_like(text, pat)))
                    }
                    BinaryOp::NotLike => {
                        let text = match &l_val { SqlValue::Text(t) => t.as_str(), _ => "" };
                        let pat = match &r_val { SqlValue::Text(p) => p.as_str(), _ => "" };
                        Ok(SqlValue::Boolean(!matches_like(text, pat)))
                    }
                    _ => Ok(SqlValue::Boolean(self.compare_values(&l_val, op, &r_val))),
                }
            }
            SqlExpr::FunctionCall { name, args } => {
                let func_up = name.to_uppercase();
                if func_up == "KEMIRIPAN_VEKTOR" || func_up == "COSINE_SIMILARITY" {
                    if args.len() >= 2 {
                        let v1_val = self.eval_expr_value(&args[0], row)?;
                        let v2_val = self.eval_expr_value(&args[1], row)?;
                        if let (Some(vec_a), Some(vec_b)) = (parse_vector_from_value(&v1_val), parse_vector_from_value(&v2_val)) {
                            let score = VectorEngine::cosine_similarity(&vec_a, &vec_b);
                            return Ok(SqlValue::Float(score as f64));
                        }
                    }
                    return Ok(SqlValue::Float(0.0));
                }
                match func_up.as_str() {
                    "COALESCE" | "JIKA_NIHIL" | "IFNULL" => {
                        for a in args {
                            let v = self.eval_expr_value(a, row)?;
                            if v != SqlValue::Null {
                                return Ok(v);
                            }
                        }
                        Ok(SqlValue::Null)
                    }
                    "NULLIF" => {
                        if args.len() >= 2 {
                            let a = self.eval_expr_value(&args[0], row)?;
                            let b = self.eval_expr_value(&args[1], row)?;
                            if a == b {
                                Ok(SqlValue::Null)
                            } else {
                                Ok(a)
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "GREATEST" | "TERBESAR" => {
                        let mut max_val: Option<SqlValue> = None;
                        for a in args {
                            let v = self.eval_expr_value(a, row)?;
                            if v != SqlValue::Null {
                                match &max_val {
                                    None => max_val = Some(v),
                                    Some(curr) => {
                                        if compare_sql_values(&v, curr) == std::cmp::Ordering::Greater {
                                            max_val = Some(v);
                                        }
                                    }
                                }
                            }
                        }
                        Ok(max_val.unwrap_or(SqlValue::Null))
                    }
                    "LEAST" | "TERKECIL" => {
                        let mut min_val: Option<SqlValue> = None;
                        for a in args {
                            let v = self.eval_expr_value(a, row)?;
                            if v != SqlValue::Null {
                                match &min_val {
                                    None => min_val = Some(v),
                                    Some(curr) => {
                                        if compare_sql_values(&v, curr) == std::cmp::Ordering::Less {
                                            min_val = Some(v);
                                        }
                                    }
                                }
                            }
                        }
                        Ok(min_val.unwrap_or(SqlValue::Null))
                    }
                    "UPPER" | "HURUF_BESAR" => {
                        if let Some(a) = args.get(0) {
                            let v = self.eval_expr_value(a, row)?;
                            match v {
                                SqlValue::Text(s) => Ok(SqlValue::Text(s.to_uppercase())),
                                SqlValue::Null => Ok(SqlValue::Null),
                                other => Ok(SqlValue::Text(format!("{:?}", other).to_uppercase())),
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "LOWER" | "HURUF_KECIL" => {
                        if let Some(a) = args.get(0) {
                            let v = self.eval_expr_value(a, row)?;
                            match v {
                                SqlValue::Text(s) => Ok(SqlValue::Text(s.to_lowercase())),
                                SqlValue::Null => Ok(SqlValue::Null),
                                other => Ok(SqlValue::Text(format!("{:?}", other).to_lowercase())),
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "LENGTH" | "PANJANG" => {
                        if let Some(a) = args.get(0) {
                            let v = self.eval_expr_value(a, row)?;
                            match v {
                                SqlValue::Text(s) => Ok(SqlValue::Integer(s.chars().count() as i64)),
                                SqlValue::Integer(i) => Ok(SqlValue::Integer(i.to_string().chars().count() as i64)),
                                SqlValue::Float(f) => Ok(SqlValue::Integer(f.to_string().chars().count() as i64)),
                                SqlValue::Null => Ok(SqlValue::Null),
                                _ => Ok(SqlValue::Integer(0)),
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "SUBSTR" | "SUBSTRING" | "POTONG_TEKS" => {
                        if let Some(a) = args.get(0) {
                            let text = match self.eval_expr_value(a, row)? {
                                SqlValue::Text(s) => s,
                                SqlValue::Null => return Ok(SqlValue::Null),
                                other => other.as_text(),
                            };
                            let chars: Vec<char> = text.chars().collect();
                            let start_1_indexed = if args.len() > 1 {
                                match self.eval_expr_value(&args[1], row)? {
                                    SqlValue::Integer(i) => i.max(1) as usize,
                                    _ => 1,
                                }
                            } else { 1 };
                            let start_0_indexed = (start_1_indexed - 1).min(chars.len());
                            let len = if args.len() > 2 {
                                match self.eval_expr_value(&args[2], row)? {
                                    SqlValue::Integer(l) => l.max(0) as usize,
                                    _ => chars.len(),
                                }
                            } else {
                                chars.len()
                            };
                            let end = (start_0_indexed + len).min(chars.len());
                            let sub: String = chars[start_0_indexed..end].iter().collect();
                            Ok(SqlValue::Text(sub))
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "REPLACE" | "GANTI_TEKS" => {
                        if args.len() >= 3 {
                            let text = self.eval_expr_value(&args[0], row)?.as_text();
                            let from = self.eval_expr_value(&args[1], row)?.as_text();
                            let to = self.eval_expr_value(&args[2], row)?.as_text();
                            Ok(SqlValue::Text(text.replace(&from, &to)))
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "LPAD" => {
                        if args.len() >= 2 {
                            let text = self.eval_expr_value(&args[0], row)?.as_text();
                            let target_len = match self.eval_expr_value(&args[1], row)? {
                                SqlValue::Integer(i) => i.max(0) as usize,
                                _ => text.len(),
                            };
                            let pad_str = if args.len() > 2 { self.eval_expr_value(&args[2], row)?.as_text() } else { " ".to_string() };
                            let pad_char = pad_str.chars().next().unwrap_or(' ');
                            if text.len() >= target_len {
                                Ok(SqlValue::Text(text.chars().take(target_len).collect()))
                            } else {
                                let diff = target_len - text.len();
                                let mut res = String::new();
                                for _ in 0..diff { res.push(pad_char); }
                                res.push_str(&text);
                                Ok(SqlValue::Text(res))
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "RPAD" => {
                        if args.len() >= 2 {
                            let text = self.eval_expr_value(&args[0], row)?.as_text();
                            let target_len = match self.eval_expr_value(&args[1], row)? {
                                SqlValue::Integer(i) => i.max(0) as usize,
                                _ => text.len(),
                            };
                            let pad_str = if args.len() > 2 { self.eval_expr_value(&args[2], row)?.as_text() } else { " ".to_string() };
                            let pad_char = pad_str.chars().next().unwrap_or(' ');
                            if text.len() >= target_len {
                                Ok(SqlValue::Text(text.chars().take(target_len).collect()))
                            } else {
                                let diff = target_len - text.len();
                                let mut res = text;
                                for _ in 0..diff { res.push(pad_char); }
                                Ok(SqlValue::Text(res))
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "SPLIT_PART" | "BAGI_TEKS" => {
                        if args.len() >= 3 {
                            let text = self.eval_expr_value(&args[0], row)?.as_text();
                            let delim = self.eval_expr_value(&args[1], row)?.as_text();
                            let field_1_indexed = match self.eval_expr_value(&args[2], row)? {
                                SqlValue::Integer(i) => i.max(1) as usize,
                                _ => 1,
                            };
                            let parts: Vec<&str> = text.split(&delim).collect();
                            if field_1_indexed <= parts.len() {
                                Ok(SqlValue::Text(parts[field_1_indexed - 1].to_string()))
                            } else {
                                Ok(SqlValue::Text("".to_string()))
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "CONCAT" | "GABUNG_TEKS" => {
                        let mut buf = String::new();
                        for a in args {
                            let v = self.eval_expr_value(a, row)?;
                            match v {
                                SqlValue::Text(s) => buf.push_str(&s),
                                SqlValue::Integer(i) => buf.push_str(&i.to_string()),
                                SqlValue::Float(f) => buf.push_str(&f.to_string()),
                                SqlValue::Boolean(b) => buf.push_str(&b.to_string()),
                                SqlValue::Null => {}
                            }
                        }
                        Ok(SqlValue::Text(buf))
                    }
                    "TRIM" | "PANGKAS" => {
                        if let Some(a) = args.get(0) {
                            let v = self.eval_expr_value(a, row)?;
                            match v {
                                SqlValue::Text(s) => Ok(SqlValue::Text(s.trim().to_string())),
                                SqlValue::Null => Ok(SqlValue::Null),
                                other => Ok(other),
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "ROUND" | "BULATKAN" => {
                        if let Some(a) = args.get(0) {
                            let num = match self.eval_expr_value(a, row)? {
                                SqlValue::Integer(i) => i as f64,
                                SqlValue::Float(f) => f,
                                _ => return Ok(SqlValue::Null),
                            };
                            let decimals = if args.len() > 1 {
                                match self.eval_expr_value(&args[1], row)? {
                                    SqlValue::Integer(d) => d.max(0) as u32,
                                    _ => 0,
                                }
                            } else {
                                0
                            };
                            if decimals == 0 {
                                Ok(SqlValue::Integer(num.round() as i64))
                            } else {
                                let factor = 10f64.powi(decimals as i32);
                                Ok(SqlValue::Float((num * factor).round() / factor))
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "ABS" | "ABSOLUT" => {
                        if let Some(a) = args.get(0) {
                            match self.eval_expr_value(a, row)? {
                                SqlValue::Integer(i) => Ok(SqlValue::Integer(i.abs())),
                                SqlValue::Float(f) => Ok(SqlValue::Float(f.abs())),
                                _ => Ok(SqlValue::Null),
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "FLOOR" | "LANTAI" => {
                        if let Some(a) = args.get(0) {
                            match self.eval_expr_value(a, row)? {
                                SqlValue::Integer(i) => Ok(SqlValue::Integer(i)),
                                SqlValue::Float(f) => Ok(SqlValue::Integer(f.floor() as i64)),
                                _ => Ok(SqlValue::Null),
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "CEIL" | "CEILING" | "ATAP" => {
                        if let Some(a) = args.get(0) {
                            match self.eval_expr_value(a, row)? {
                                SqlValue::Integer(i) => Ok(SqlValue::Integer(i)),
                                SqlValue::Float(f) => Ok(SqlValue::Integer(f.ceil() as i64)),
                                _ => Ok(SqlValue::Null),
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "MOD" | "SISA_BAGI" => {
                        if args.len() >= 2 {
                            let a = self.eval_expr_value(&args[0], row)?;
                            let b = self.eval_expr_value(&args[1], row)?;
                            match (a, b) {
                                (SqlValue::Integer(x), SqlValue::Integer(y)) if y != 0 => Ok(SqlValue::Integer(x % y)),
                                (SqlValue::Float(x), SqlValue::Float(y)) if y != 0.0 => Ok(SqlValue::Float(x % y)),
                                _ => Ok(SqlValue::Null),
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "CAST" | "CONVERT" | "UBAH_TIPE" => {
                        if args.len() >= 2 {
                            let val = self.eval_expr_value(&args[0], row)?;
                            let target_type = self.eval_expr_value(&args[1], row)?.as_text().to_uppercase();
                            match target_type.as_str() {
                                "INTEGER" | "INT" | "BIGINT" | "BILANGAN_BULAT" => {
                                    match val {
                                        SqlValue::Integer(i) => Ok(SqlValue::Integer(i)),
                                        SqlValue::Float(f) => Ok(SqlValue::Integer(f as i64)),
                                        SqlValue::Text(s) => Ok(s.parse::<i64>().map(SqlValue::Integer).unwrap_or(SqlValue::Null)),
                                        SqlValue::Boolean(b) => Ok(SqlValue::Integer(if b { 1 } else { 0 })),
                                        SqlValue::Null => Ok(SqlValue::Null),
                                    }
                                }
                                "FLOAT" | "DOUBLE" | "REAL" | "NUMERIC" | "DESIMAL" => {
                                    match val {
                                        SqlValue::Float(f) => Ok(SqlValue::Float(f)),
                                        SqlValue::Integer(i) => Ok(SqlValue::Float(i as f64)),
                                        SqlValue::Text(s) => Ok(s.parse::<f64>().map(SqlValue::Float).unwrap_or(SqlValue::Null)),
                                        _ => Ok(SqlValue::Null),
                                    }
                                }
                                "TEXT" | "VARCHAR" | "STRING" | "TEKS" => {
                                    Ok(SqlValue::Text(val.as_text()))
                                }
                                "BOOLEAN" | "BOOL" | "LOGIKA" => {
                                    match val {
                                        SqlValue::Boolean(b) => Ok(SqlValue::Boolean(b)),
                                        SqlValue::Integer(i) => Ok(SqlValue::Boolean(i != 0)),
                                        SqlValue::Text(s) => Ok(SqlValue::Boolean(s.eq_ignore_ascii_case("true") || s.eq_ignore_ascii_case("benar") || s == "1")),
                                        _ => Ok(SqlValue::Null),
                                    }
                                }
                                _ => Ok(SqlValue::Text(val.as_text())),
                            }
                        } else if let Some(a) = args.get(0) {
                            self.eval_expr_value(a, row)
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "EXTRACT" | "DATE_PART" | "AMBIL_BAGIAN_TANGGAL" => {
                        if args.len() >= 2 {
                            let part = self.eval_expr_value(&args[0], row)?.as_text().to_uppercase();
                            let date_str = self.eval_expr_value(&args[1], row)?.as_text();
                            let clean_date = date_str.trim();
                            if clean_date.len() >= 10 {
                                let year = clean_date[0..4].parse::<i64>().unwrap_or(0);
                                let month = clean_date[5..7].parse::<i64>().unwrap_or(0);
                                let day = clean_date[8..10].parse::<i64>().unwrap_or(0);
                                match part.as_str() {
                                    "YEAR" | "TAHUN" => Ok(SqlValue::Integer(year)),
                                    "MONTH" | "BULAN" => Ok(SqlValue::Integer(month)),
                                    "DAY" | "HARI" => Ok(SqlValue::Integer(day)),
                                    _ => Ok(SqlValue::Null),
                                }
                            } else {
                                Ok(SqlValue::Null)
                            }
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "MAKE_DATE" | "BUAT_TANGGAL" => {
                        if args.len() >= 3 {
                            let y = self.eval_expr_value(&args[0], row)?.as_text().parse::<i64>().unwrap_or(2000);
                            let m = self.eval_expr_value(&args[1], row)?.as_text().parse::<i64>().unwrap_or(1);
                            let d = self.eval_expr_value(&args[2], row)?.as_text().parse::<i64>().unwrap_or(1);
                            Ok(SqlValue::Text(format!("{:04}-{:02}-{:02}", y, m, d)))
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                    "NOW" | "SEKARANG" => {
                        Ok(SqlValue::Text(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()))
                    }
                    "CURRENT_DATE" | "HARI_INI" | "TANGGAL_SEKARANG" => {
                        Ok(SqlValue::Text(chrono::Local::now().format("%Y-%m-%d").to_string()))
                    }
                    _ => Ok(SqlValue::Null),
                }
            }
            SqlExpr::Between { expr, low, high, negated } => {
                let val = self.eval_expr_value(expr, row)?;
                let l_val = self.eval_expr_value(low, row)?;
                let h_val = self.eval_expr_value(high, row)?;
                let in_range = self.compare_values(&val, &BinaryOp::Gte, &l_val) && self.compare_values(&val, &BinaryOp::Lte, &h_val);
                Ok(SqlValue::Boolean(if *negated { !in_range } else { in_range }))
            }
            SqlExpr::InList { expr, list, negated } => {
                let val = self.eval_expr_value(expr, row)?;
                let mut matched = false;
                for item in list {
                    let item_val = self.eval_expr_value(item, row)?;
                    if val == item_val {
                        matched = true;
                        break;
                    }
                }
                Ok(SqlValue::Boolean(if *negated { !matched } else { matched }))
            }
            _ => Ok(SqlValue::Null),
        }
    }

    fn eval_math_op<FI, FF>(&self, left: &SqlValue, right: &SqlValue, op_int: FI, op_float: FF) -> Result<SqlValue, String>
    where
        FI: Fn(i64, i64) -> i64,
        FF: Fn(f64, f64) -> f64,
    {
        match (left, right) {
            (SqlValue::Integer(a), SqlValue::Integer(b)) => Ok(SqlValue::Integer(op_int(*a, *b))),
            (SqlValue::Float(a), SqlValue::Float(b)) => Ok(SqlValue::Float(op_float(*a, *b))),
            (SqlValue::Integer(a), SqlValue::Float(b)) => Ok(SqlValue::Float(op_float(*a as f64, *b))),
            (SqlValue::Float(a), SqlValue::Integer(b)) => Ok(SqlValue::Float(op_float(*a, *b as f64))),
            _ => Ok(SqlValue::Null),
        }
    }

    fn eval_grouped_expr(&self, col_expr: &SqlExpr, group_cols: &[String], group_vals: &[SqlValue], group_rows: &[Row]) -> Result<SqlValue, String> {
        match col_expr {
            SqlExpr::Alias { expr, .. } => self.eval_grouped_expr(expr, group_cols, group_vals, group_rows),
            SqlExpr::Column(c) => {
                for (i, g_col) in group_cols.iter().enumerate() {
                    if c.eq_ignore_ascii_case(g_col)
                        || c.ends_with(&format!(".{}", g_col))
                        || g_col.ends_with(&format!(".{}", c))
                    {
                        return Ok(group_vals.get(i).cloned().unwrap_or(SqlValue::Null));
                    }
                }
                if let Some(first_row) = group_rows.first() {
                    self.eval_expr_value(col_expr, first_row)
                } else {
                    Ok(SqlValue::Null)
                }
            }
            SqlExpr::FunctionCall { name, args } => {
                let agg = name.to_uppercase();
                match agg.as_str() {
                    "COUNT" | "HITUNG" => {
                        if args.is_empty() || matches!(args[0], SqlExpr::Wildcard) {
                            Ok(SqlValue::Integer(group_rows.len() as i64))
                        } else {
                            let mut count = 0;
                            for r in group_rows {
                                if self.eval_expr_value(&args[0], r).unwrap_or(SqlValue::Null) != SqlValue::Null {
                                    count += 1;
                                }
                            }
                            Ok(SqlValue::Integer(count))
                        }
                    }
                    "SUM" | "JUMLAH" => {
                        let mut sum = 0.0;
                        if let Some(arg) = args.get(0) {
                            for r in group_rows {
                                match self.eval_expr_value(arg, r).unwrap_or(SqlValue::Null) {
                                    SqlValue::Integer(i) => sum += i as f64,
                                    SqlValue::Float(f) => sum += f,
                                    _ => {}
                                }
                            }
                        }
                        Ok(SqlValue::Float(sum))
                    }
                    "AVG" | "RATA_RATA" => {
                        let count = group_rows.len();
                        let mut sum = 0.0;
                        if let Some(arg) = args.get(0) {
                            for r in group_rows {
                                match self.eval_expr_value(arg, r).unwrap_or(SqlValue::Null) {
                                    SqlValue::Integer(i) => sum += i as f64,
                                    SqlValue::Float(f) => sum += f,
                                    _ => {}
                                }
                            }
                        }
                        let avg = if count > 0 { sum / count as f64 } else { 0.0 };
                        Ok(SqlValue::Float(avg))
                    }
                    "MIN" => {
                        let mut min_val = f64::MAX;
                        if let Some(arg) = args.get(0) {
                            for r in group_rows {
                                match self.eval_expr_value(arg, r).unwrap_or(SqlValue::Null) {
                                    SqlValue::Integer(i) => { if (i as f64) < min_val { min_val = i as f64; } }
                                    SqlValue::Float(f) => { if f < min_val { min_val = f; } }
                                    _ => {}
                                }
                            }
                        }
                        Ok(SqlValue::Float(if min_val == f64::MAX { 0.0 } else { min_val }))
                    }
                    "MAX" | "MAKS" => {
                        let mut max_val = f64::MIN;
                        if let Some(arg) = args.get(0) {
                            for r in group_rows {
                                match self.eval_expr_value(arg, r).unwrap_or(SqlValue::Null) {
                                    SqlValue::Integer(i) => { if (i as f64) > max_val { max_val = i as f64; } }
                                    SqlValue::Float(f) => { if f > max_val { max_val = f; } }
                                    _ => {}
                                }
                            }
                        }
                        Ok(SqlValue::Float(if max_val == f64::MIN { 0.0 } else { max_val }))
                    }
                    "STRING_AGG" | "GROUP_CONCAT" => {
                        let delim = if args.len() > 1 {
                            match self.eval_expr_value(&args[1], &Row { fields: HashMap::new() }).unwrap_or(SqlValue::Null) {
                                SqlValue::Text(s) => s,
                                _ => ",".to_string(),
                            }
                        } else {
                            ",".to_string()
                        };
                        let mut parts = Vec::new();
                        if let Some(arg) = args.get(0) {
                            for r in group_rows {
                                let v = self.eval_expr_value(arg, r).unwrap_or(SqlValue::Null);
                                if v != SqlValue::Null {
                                    parts.push(v.as_text());
                                }
                            }
                        }
                        Ok(SqlValue::Text(parts.join(&delim)))
                    }
                    "ARRAY_AGG" => {
                        let mut parts = Vec::new();
                        if let Some(arg) = args.get(0) {
                            for r in group_rows {
                                let v = self.eval_expr_value(arg, r).unwrap_or(SqlValue::Null);
                                parts.push(v.as_text());
                            }
                        }
                        Ok(SqlValue::Text(format!("[{}]", parts.join(", "))))
                    }
                    _ => {
                        if let Some(first_row) = group_rows.first() {
                            self.eval_expr_value(col_expr, first_row)
                        } else {
                            Ok(SqlValue::Null)
                        }
                    }
                }
            }
            SqlExpr::IsNull { expr: inner, negated } => {
                let val = self.eval_grouped_expr(inner, group_cols, group_vals, group_rows)?;
                let is_null = val == SqlValue::Null;
                Ok(SqlValue::Boolean(if *negated { !is_null } else { is_null }))
            }
            SqlExpr::Case { conditions, else_branch } => {
                for (cond, then_expr) in conditions {
                    if self.eval_grouped_predicate(cond, group_cols, group_vals, group_rows)? {
                        return self.eval_grouped_expr(then_expr, group_cols, group_vals, group_rows);
                    }
                }
                if let Some(else_expr) = else_branch {
                    self.eval_grouped_expr(else_expr, group_cols, group_vals, group_rows)
                } else {
                    Ok(SqlValue::Null)
                }
            }
            _ => {
                if let Some(first_row) = group_rows.first() {
                    self.eval_expr_value(col_expr, first_row)
                } else {
                    Ok(SqlValue::Null)
                }
            }
        }
    }

    fn eval_grouped_predicate(
        &self,
        expr: &SqlExpr,
        group_cols: &[String],
        group_vals: &[SqlValue],
        group_rows: &[Row],
    ) -> Result<bool, String> {
        match expr {
            SqlExpr::Binary { left, op, right } => {
                match op {
                    BinaryOp::And => {
                        let l = self.eval_grouped_predicate(left, group_cols, group_vals, group_rows)?;
                        let r = self.eval_grouped_predicate(right, group_cols, group_vals, group_rows)?;
                        Ok(l && r)
                    }
                    BinaryOp::Or => {
                        let l = self.eval_grouped_predicate(left, group_cols, group_vals, group_rows)?;
                        let r = self.eval_grouped_predicate(right, group_cols, group_vals, group_rows)?;
                        Ok(l || r)
                    }
                    _ => {
                        let l = self.eval_grouped_expr(left, group_cols, group_vals, group_rows)?;
                        let r = self.eval_grouped_expr(right, group_cols, group_vals, group_rows)?;
                        Ok(self.compare_values(&l, op, &r))
                    }
                }
            }
            SqlExpr::IsNull { expr: inner, negated } => {
                let val = self.eval_grouped_expr(inner, group_cols, group_vals, group_rows)?;
                let is_null = val == SqlValue::Null;
                Ok(if *negated { !is_null } else { is_null })
            }
            SqlExpr::Between { expr: inner, low, high, negated } => {
                let val = self.eval_grouped_expr(inner, group_cols, group_vals, group_rows)?;
                let l_val = self.eval_grouped_expr(low, group_cols, group_vals, group_rows)?;
                let h_val = self.eval_grouped_expr(high, group_cols, group_vals, group_rows)?;
                let in_range = self.compare_values(&val, &BinaryOp::Gte, &l_val) && self.compare_values(&val, &BinaryOp::Lte, &h_val);
                Ok(if *negated { !in_range } else { in_range })
            }
            SqlExpr::InList { expr: inner, list, negated } => {
                let val = self.eval_grouped_expr(inner, group_cols, group_vals, group_rows)?;
                let mut matched = false;
                for item in list {
                    let item_val = self.eval_grouped_expr(item, group_cols, group_vals, group_rows)?;
                    if val == item_val {
                        matched = true;
                        break;
                    }
                }
                Ok(if *negated { !matched } else { matched })
            }
            SqlExpr::Literal(SqlValue::Boolean(b)) => Ok(*b),
            _ => {
                let val = self.eval_grouped_expr(expr, group_cols, group_vals, group_rows)?;
                match val {
                    SqlValue::Boolean(b) => Ok(b),
                    SqlValue::Null => Ok(false),
                    _ => Ok(true),
                }
            }
        }
    }

    fn resolve_subqueries(&self, expr: SqlExpr, tx_id: u64) -> Result<SqlExpr, String> {
        match expr {
            SqlExpr::Subquery(sub) => {
                let res = self.execute(*sub, tx_id)?;
                let val = match res {
                    QueryResult::Rows { rows, .. } => {
                        rows.into_iter().next().and_then(|r| r.into_iter().next()).unwrap_or(SqlValue::Null)
                    }
                    _ => SqlValue::Null,
                };
                Ok(SqlExpr::Literal(val))
            }
            SqlExpr::InSubquery { expr: inner_expr, subquery, negated } => {
                let res = self.execute(*subquery, tx_id)?;
                let list = match res {
                    QueryResult::Rows { rows, .. } => {
                        rows.into_iter().filter_map(|r| r.into_iter().next()).map(SqlExpr::Literal).collect()
                    }
                    _ => Vec::new(),
                };
                let resolved_inner = self.resolve_subqueries(*inner_expr, tx_id)?;
                Ok(SqlExpr::InList {
                    expr: Box::new(resolved_inner),
                    list,
                    negated,
                })
            }
            SqlExpr::Binary { left, op, right } => {
                let res_l = self.resolve_subqueries(*left, tx_id)?;
                let res_r = self.resolve_subqueries(*right, tx_id)?;
                Ok(SqlExpr::Binary { left: Box::new(res_l), op, right: Box::new(res_r) })
            }
            SqlExpr::Between { expr: inner, low, high, negated } => {
                let res_inner = self.resolve_subqueries(*inner, tx_id)?;
                let res_low = self.resolve_subqueries(*low, tx_id)?;
                let res_high = self.resolve_subqueries(*high, tx_id)?;
                Ok(SqlExpr::Between {
                    expr: Box::new(res_inner),
                    low: Box::new(res_low),
                    high: Box::new(res_high),
                    negated,
                })
            }
            SqlExpr::InList { expr: inner, list, negated } => {
                let res_inner = self.resolve_subqueries(*inner, tx_id)?;
                let mut res_list = Vec::new();
                for item in list {
                    res_list.push(self.resolve_subqueries(item, tx_id)?);
                }
                Ok(SqlExpr::InList { expr: Box::new(res_inner), list: res_list, negated })
            }
            SqlExpr::Case { conditions, else_branch } => {
                let mut res_conds = Vec::new();
                for (c, r) in conditions {
                    let rc = self.resolve_subqueries(c, tx_id)?;
                    let rr = self.resolve_subqueries(r, tx_id)?;
                    res_conds.push((rc, rr));
                }
                let res_else = match else_branch {
                    Some(e) => Some(Box::new(self.resolve_subqueries(*e, tx_id)?)),
                    None => None,
                };
                Ok(SqlExpr::Case { conditions: res_conds, else_branch: res_else })
            }
            SqlExpr::IsNull { expr: inner, negated } => {
                let res_inner = self.resolve_subqueries(*inner, tx_id)?;
                Ok(SqlExpr::IsNull { expr: Box::new(res_inner), negated })
            }
            SqlExpr::Alias { expr: inner, alias } => {
                let res_inner = self.resolve_subqueries(*inner, tx_id)?;
                Ok(SqlExpr::Alias { expr: Box::new(res_inner), alias })
            }
            SqlExpr::FunctionCall { name, args } => {
                let mut res_args = Vec::new();
                for a in args {
                    res_args.push(self.resolve_subqueries(a, tx_id)?);
                }
                Ok(SqlExpr::FunctionCall { name, args: res_args })
            }
            other => Ok(other),
        }
    }

    fn compare_values(&self, left: &SqlValue, op: &BinaryOp, right: &SqlValue) -> bool {
        match op {
            BinaryOp::Eq => left == right,
            BinaryOp::Neq => left != right,
            BinaryOp::Gt => match (left, right) {
                (SqlValue::Integer(a), SqlValue::Integer(b)) => a > b,
                (SqlValue::Float(a), SqlValue::Float(b)) => a > b,
                (SqlValue::Integer(a), SqlValue::Float(b)) => (*a as f64) > *b,
                (SqlValue::Float(a), SqlValue::Integer(b)) => *a > (*b as f64),
                (SqlValue::Text(a), SqlValue::Text(b)) => a > b,
                _ => false,
            },
            BinaryOp::Lt => match (left, right) {
                (SqlValue::Integer(a), SqlValue::Integer(b)) => a < b,
                (SqlValue::Float(a), SqlValue::Float(b)) => a < b,
                (SqlValue::Integer(a), SqlValue::Float(b)) => (*a as f64) < *b,
                (SqlValue::Float(a), SqlValue::Integer(b)) => *a < (*b as f64),
                (SqlValue::Text(a), SqlValue::Text(b)) => a < b,
                _ => false,
            },
            BinaryOp::Gte => match (left, right) {
                (SqlValue::Integer(a), SqlValue::Integer(b)) => a >= b,
                (SqlValue::Float(a), SqlValue::Float(b)) => a >= b,
                (SqlValue::Integer(a), SqlValue::Float(b)) => (*a as f64) >= *b,
                (SqlValue::Float(a), SqlValue::Integer(b)) => *a >= (*b as f64),
                (SqlValue::Text(a), SqlValue::Text(b)) => a >= b,
                _ => false,
            },
            BinaryOp::Lte => match (left, right) {
                (SqlValue::Integer(a), SqlValue::Integer(b)) => a <= b,
                (SqlValue::Float(a), SqlValue::Float(b)) => a <= b,
                (SqlValue::Integer(a), SqlValue::Float(b)) => (*a as f64) <= *b,
                (SqlValue::Float(a), SqlValue::Integer(b)) => *a <= (*b as f64),
                (SqlValue::Text(a), SqlValue::Text(b)) => a <= b,
                _ => false,
            },
            _ => false,
        }
    }
}

fn compare_sql_values(val_a: &SqlValue, val_b: &SqlValue) -> std::cmp::Ordering {
    match (val_a, val_b) {
        (SqlValue::Integer(x), SqlValue::Integer(y)) => x.cmp(y),
        (SqlValue::Float(x), SqlValue::Float(y)) => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
        (SqlValue::Text(x), SqlValue::Text(y)) => x.cmp(y),
        (SqlValue::Integer(x), SqlValue::Float(y)) => (*x as f64).partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
        (SqlValue::Float(x), SqlValue::Integer(y)) => x.partial_cmp(&(*y as f64)).unwrap_or(std::cmp::Ordering::Equal),
        (SqlValue::Boolean(x), SqlValue::Boolean(y)) => x.cmp(y),
        _ => std::cmp::Ordering::Equal,
    }
}

fn matches_like(text: &str, pattern: &str) -> bool {
    let text_chars: Vec<char> = text.to_lowercase().chars().collect();
    let pat_chars: Vec<char> = pattern.to_lowercase().chars().collect();

    let mut dp = vec![vec![false; pat_chars.len() + 1]; text_chars.len() + 1];
    dp[0][0] = true;

    for j in 1..=pat_chars.len() {
        if pat_chars[j - 1] == '%' {
            dp[0][j] = dp[0][j - 1];
        }
    }

    for i in 1..=text_chars.len() {
        for j in 1..=pat_chars.len() {
            if pat_chars[j - 1] == '%' {
                dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
            } else if pat_chars[j - 1] == '_' || pat_chars[j - 1] == text_chars[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    dp[text_chars.len()][pat_chars.len()]
}

fn parse_vector_from_value(val: &SqlValue) -> Option<Vec<f32>> {
    let s = match val {
        SqlValue::Text(t) => t.trim(),
        _ => return None,
    };
    if s.starts_with('[') && s.ends_with(']') {
        let inner = &s[1..s.len() - 1];
        let mut vec = Vec::new();
        for part in inner.split(',') {
            if let Ok(num) = part.trim().parse::<f32>() {
                vec.push(num);
            }
        }
        if !vec.is_empty() {
            return Some(vec);
        }
    }
    let mut vec = Vec::new();
    for part in s.split(',') {
        if let Ok(num) = part.trim().parse::<f32>() {
            vec.push(num);
        }
    }
    if !vec.is_empty() {
        Some(vec)
    } else {
        None
    }
}
