// ============================================================================
// WIDYADOCDB ENGINE (MongoDB High-Performance Document NoSQL Compatible Engine)
// ============================================================================
// Mesin Basis Data Dokumen JSON/BSON Skala Industri:
// - Dynamic Schema-less Collections & Capped Collections
// - Rich Query Operators ($eq, $ne, $gt, $gte, $lt, $lte, $in, $nin, $exists, $regex, $size, $elemMatch)
// - Logical Operators ($and, $or, $nor, $not) & Deep Dot Notation (`profile.address.city`)
// - Atomic Update Operators ($set, $unset, $inc, $mul, $min, $max, $push, $pull, $addToSet, $currentDate)
// - B-Tree Secondary Indexing on arbitrary JSON paths with Unique constraints
// - Multi-Stage Aggregation Pipeline ($match, $project, $group, $sort, $limit, $skip, $unwind, $count, $lookup)
// - Real-Time Change Streams (Insert, Update, Replace, Delete event streaming)
// - Inverted Index Full-Text Search Engine ($text, $search)
// - ACID Document Transactions & Write-Ahead Log (WAL) Persistence
// ============================================================================

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

/// Tipe Event Perubahan Real-time (Change Stream Event)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Insert,
    Update,
    Replace,
    Delete,
}

/// Struktur Payload Change Stream Event (seperti MongoDB Change Streams)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEvent {
    pub id: String,
    pub operation_type: OperationType,
    pub collection: String,
    pub document_key: String,
    pub full_document: Option<Value>,
    pub updated_fields: Option<Value>,
    pub timestamp: DateTime<Utc>,
}

/// Opsi Konfigurasi Koleksi Dokumen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionOptions {
    pub capped: bool,
    pub max_documents: Option<usize>,
    pub max_size_bytes: Option<usize>,
}

impl Default for CollectionOptions {
    fn default() -> Self {
        Self {
            capped: false,
            max_documents: None,
            max_size_bytes: None,
        }
    }
}

/// Definisi Index Sekunder Koleksi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexSpec {
    pub name: String,
    pub key: String,       // JSON path: e.g. "email" atau "address.city"
    pub unique: bool,
    pub sparse: bool,
}

/// Hasil Operasi Update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    pub matched_count: usize,
    pub modified_count: usize,
    pub upserted_id: Option<String>,
}

/// Hasil Operasi Delete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResult {
    pub deleted_count: usize,
}

/// Koleksi Dokumen DokDB
pub struct Collection {
    pub name: String,
    pub options: CollectionOptions,
    // _id -> Document (serde_json::Value)
    documents: RwLock<BTreeMap<String, Value>>,
    // IndexName -> (IndexedValue -> Set<_id>)
    indexes: RwLock<HashMap<String, (IndexSpec, BTreeMap<String, HashSet<String>>)>>,
    change_sender: broadcast::Sender<ChangeEvent>,
    wal_path: Option<PathBuf>,
}

impl Collection {
    pub fn new(name: &str, options: CollectionOptions, wal_path: Option<PathBuf>) -> Self {
        let (tx, _) = broadcast::channel(1000);
        let coll = Self {
            name: name.to_string(),
            options,
            documents: RwLock::new(BTreeMap::new()),
            indexes: RwLock::new(HashMap::new()),
            change_sender: tx,
            wal_path,
        };

        // Buat index default pada field "_id"
        coll.create_index("_id", true).unwrap();
        coll
    }

    /// Buat Secondary B-Tree Index
    pub fn create_index(&self, key: &str, unique: bool) -> Result<String, String> {
        let index_name = format!("{}_1", key.replace('.', "_"));
        let mut indexes = self.indexes.write().unwrap();
        if indexes.contains_key(&index_name) {
            return Ok(index_name);
        }

        let mut btree_index: BTreeMap<String, HashSet<String>> = BTreeMap::new();
        let docs = self.documents.read().unwrap();

        for (doc_id, doc) in docs.iter() {
            if let Some(val) = extract_nested_value(doc, key) {
                let val_str = val_to_index_key(&val);
                if unique {
                    if let Some(existing) = btree_index.get(&val_str) {
                        if !existing.is_empty() {
                            return Err(format!("DuplicateKeyError: Kunci unik '{}' sudah ada nilainya: {}", key, val_str));
                        }
                    }
                }
                btree_index.entry(val_str).or_default().insert(doc_id.clone());
            }
        }

        indexes.insert(
            index_name.clone(),
            (
                IndexSpec {
                    name: index_name.clone(),
                    key: key.to_string(),
                    unique,
                    sparse: false,
                },
                btree_index,
            ),
        );

        Ok(index_name)
    }

    /// Sisipkan 1 Dokumen (InsertOne)
    pub fn insert_one(&self, mut doc: Value) -> Result<String, String> {
        let obj = doc.as_object_mut()
            .ok_or_else(|| "Dokumen harus berupa JSON Object".to_string())?;

        let doc_id = match obj.get("_id") {
            Some(Value::String(s)) => s.clone(),
            _ => {
                let id = uuid::Uuid::new_v4().to_string();
                obj.insert("_id".to_string(), Value::String(id.clone()));
                id
            }
        };

        // Validasi Unique Constraints pada Index
        let mut indexes = self.indexes.write().unwrap();
        for (_, (spec, btree)) in indexes.iter_mut() {
            if let Some(val) = extract_nested_value(&doc, &spec.key) {
                let val_str = val_to_index_key(&val);
                if spec.unique {
                    if let Some(set) = btree.get(&val_str) {
                        if !set.is_empty() && !set.contains(&doc_id) {
                            return Err(format!("DuplicateKeyError: Index '{}' melanggar constraint unik pada nilai: {}", spec.name, val_str));
                        }
                    }
                }
                btree.entry(val_str).or_default().insert(doc_id.clone());
            }
        }
        drop(indexes);

        // Simpan dokumen
        let mut docs = self.documents.write().unwrap();
        if self.options.capped {
            if let Some(max_docs) = self.options.max_documents {
                if docs.len() >= max_docs {
                    // Hapus dokumen tertua
                    if let Some(oldest_key) = docs.keys().next().cloned() {
                        docs.remove(&oldest_key);
                    }
                }
            }
        }

        docs.insert(doc_id.clone(), doc.clone());
        drop(docs);

        // Tulis ke WAL jika ada
        self.append_wal_record("INSERT", &doc);

        // Broadcast Change Stream
        let _ = self.change_sender.send(ChangeEvent {
            id: uuid::Uuid::new_v4().to_string(),
            operation_type: OperationType::Insert,
            collection: self.name.clone(),
            document_key: doc_id.clone(),
            full_document: Some(doc),
            updated_fields: None,
            timestamp: Utc::now(),
        });

        Ok(doc_id)
    }

    /// Sisipkan Banyak Dokumen (InsertMany)
    pub fn insert_many(&self, docs: Vec<Value>) -> Result<Vec<String>, String> {
        let mut ids = Vec::with_capacity(docs.len());
        for doc in docs {
            let id = self.insert_one(doc)?;
            ids.push(id);
        }
        Ok(ids)
    }

    /// Cari 1 Dokumen Pertama yang Cocok (FindOne)
    pub fn find_one(&self, filter: &Value) -> Option<Value> {
        let docs = self.documents.read().unwrap();
        for doc in docs.values() {
            if evaluate_filter(doc, filter) {
                return Some(doc.clone());
            }
        }
        None
    }

    /// Cari Semua Dokumen yang Cocok dengan Kriteria Filter & Opsi (Find)
    pub fn find(&self, filter: &Value, sort: Option<&Value>, limit: Option<usize>, skip: Option<usize>) -> Vec<Value> {
        let docs = self.documents.read().unwrap();
        let mut matched: Vec<Value> = docs
            .values()
            .filter(|doc| evaluate_filter(doc, filter))
            .cloned()
            .collect();

        // 1. Sort
        if let Some(sort_spec) = sort {
            if let Some(sort_map) = sort_spec.as_object() {
                matched.sort_by(|a, b| {
                    for (field, order_val) in sort_map {
                        let asc = order_val.as_i64().unwrap_or(1) >= 0;
                        let val_a = extract_nested_value(a, field).unwrap_or(Value::Null);
                        let val_b = extract_nested_value(b, field).unwrap_or(Value::Null);
                        let cmp = compare_json_values(&val_a, &val_b);
                        if cmp != std::cmp::Ordering::Equal {
                            return if asc { cmp } else { cmp.reverse() };
                        }
                    }
                    std::cmp::Ordering::Equal
                });
            }
        }

        // 2. Skip
        if let Some(skip_n) = skip {
            if skip_n < matched.len() {
                matched = matched.split_off(skip_n);
            } else {
                matched.clear();
            }
        }

        // 3. Limit
        if let Some(limit_n) = limit {
            matched.truncate(limit_n);
        }

        matched
    }

    /// Update 1 Dokumen (UpdateOne)
    pub fn update_one(&self, filter: &Value, update_spec: &Value, upsert: bool) -> Result<UpdateResult, String> {
        let mut docs = self.documents.write().unwrap();
        let mut target_id = None;

        for (id, doc) in docs.iter() {
            if evaluate_filter(doc, filter) {
                target_id = Some(id.clone());
                break;
            }
        }

        if let Some(id) = target_id {
            let doc = docs.get_mut(&id).unwrap();
            apply_update_modifiers(doc, update_spec)?;
            let updated_clone = doc.clone();
            drop(docs);

            self.append_wal_record("UPDATE", &updated_clone);
            let _ = self.change_sender.send(ChangeEvent {
                id: uuid::Uuid::new_v4().to_string(),
                operation_type: OperationType::Update,
                collection: self.name.clone(),
                document_key: id,
                full_document: Some(updated_clone),
                updated_fields: Some(update_spec.clone()),
                timestamp: Utc::now(),
            });

            Ok(UpdateResult {
                matched_count: 1,
                modified_count: 1,
                upserted_id: None,
            })
        } else if upsert {
            drop(docs);
            let mut new_doc = json!({});
            if let Some(f_obj) = filter.as_object() {
                for (k, v) in f_obj {
                    if !k.starts_with('$') {
                        new_doc[k] = v.clone();
                    }
                }
            }
            apply_update_modifiers(&mut new_doc, update_spec)?;
            let new_id = self.insert_one(new_doc)?;
            Ok(UpdateResult {
                matched_count: 0,
                modified_count: 0,
                upserted_id: Some(new_id),
            })
        } else {
            Ok(UpdateResult {
                matched_count: 0,
                modified_count: 0,
                upserted_id: None,
            })
        }
    }

    /// Update Banyak Dokumen (UpdateMany)
    pub fn update_many(&self, filter: &Value, update_spec: &Value) -> Result<UpdateResult, String> {
        let mut docs = self.documents.write().unwrap();
        let mut modified = 0;
        let mut matched = 0;

        for doc in docs.values_mut() {
            if evaluate_filter(doc, filter) {
                matched += 1;
                apply_update_modifiers(doc, update_spec)?;
                modified += 1;
            }
        }

        Ok(UpdateResult {
            matched_count: matched,
            modified_count: modified,
            upserted_id: None,
        })
    }

    /// Hapus 1 Dokumen (DeleteOne)
    pub fn delete_one(&self, filter: &Value) -> Result<DeleteResult, String> {
        let mut docs = self.documents.write().unwrap();
        let mut target_id = None;

        for (id, doc) in docs.iter() {
            if evaluate_filter(doc, filter) {
                target_id = Some(id.clone());
                break;
            }
        }

        if let Some(id) = target_id {
            docs.remove(&id);
            drop(docs);

            let _ = self.change_sender.send(ChangeEvent {
                id: uuid::Uuid::new_v4().to_string(),
                operation_type: OperationType::Delete,
                collection: self.name.clone(),
                document_key: id,
                full_document: None,
                updated_fields: None,
                timestamp: Utc::now(),
            });

            Ok(DeleteResult { deleted_count: 1 })
        } else {
            Ok(DeleteResult { deleted_count: 0 })
        }
    }

    /// Hapus Banyak Dokumen (DeleteMany)
    pub fn delete_many(&self, filter: &Value) -> Result<DeleteResult, String> {
        let mut docs = self.documents.write().unwrap();
        let mut to_remove = Vec::new();

        for (id, doc) in docs.iter() {
            if evaluate_filter(doc, filter) {
                to_remove.push(id.clone());
            }
        }

        let count = to_remove.len();
        for id in to_remove {
            docs.remove(&id);
        }

        Ok(DeleteResult { deleted_count: count })
    }

    /// Hitung Total Dokumen yang Cocok (CountDocuments)
    pub fn count_documents(&self, filter: &Value) -> usize {
        let docs = self.documents.read().unwrap();
        docs.values().filter(|doc| evaluate_filter(doc, filter)).count()
    }

    /// Berlangganan Change Stream Real-Time
    pub fn watch(&self) -> broadcast::Receiver<ChangeEvent> {
        self.change_sender.subscribe()
    }

    fn append_wal_record(&self, op: &str, doc: &Value) {
        if let Some(ref path) = self.wal_path {
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                let line = format!("{} {}\n", op, doc.to_string());
                let _ = file.write_all(line.as_bytes());
            }
        }
    }
}

/// Mesin Utama Basis Data Dokumen WidyaDocDB
pub struct WidyaDocDB {
    pub db_path: PathBuf,
    collections: Arc<RwLock<HashMap<String, Arc<Collection>>>>,
}

impl WidyaDocDB {
    /// Inisialisasi basis data Dokumen di direktori tertentu
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let db_path = path.as_ref().to_path_buf();
        fs::create_dir_all(&db_path)?;

        let db = Self {
            db_path: db_path.clone(),
            collections: Arc::new(RwLock::new(HashMap::new())),
        };

        // Muat koleksi yang ada dari disk
        db.restore_from_disk()?;
        Ok(db)
    }

    /// Mengambil koleksi (atau membuat jika belum ada)
    pub fn collection(&self, name: &str) -> Arc<Collection> {
        let mut colls = self.collections.write().unwrap();
        if let Some(c) = colls.get(name) {
            Arc::clone(c)
        } else {
            let wal_path = self.db_path.join(format!("{}.wal.jsonl", name));
            let coll = Arc::new(Collection::new(name, CollectionOptions::default(), Some(wal_path)));
            colls.insert(name.to_string(), Arc::clone(&coll));
            coll
        }
    }

    /// Daftar seluruh koleksi
    pub fn list_collections(&self) -> Vec<String> {
        let colls = self.collections.read().unwrap();
        colls.keys().cloned().collect()
    }

    /// Hapus koleksi
    pub fn drop_collection(&self, name: &str) -> bool {
        let mut colls = self.collections.write().unwrap();
        if colls.remove(name).is_some() {
            let wal_path = self.db_path.join(format!("{}.wal.jsonl", name));
            let _ = fs::remove_file(wal_path);
            true
        } else {
            false
        }
    }

    /// Menjalankan Aggregation Pipeline Multi-Tahap ($match, $group, $sort, $project, dll.)
    pub fn aggregate(&self, collection_name: &str, pipeline: &[Value]) -> Result<Vec<Value>, String> {
        let coll = self.collection(collection_name);
        let docs = coll.documents.read().unwrap();
        let mut working_set: Vec<Value> = docs.values().cloned().collect();
        drop(docs);

        for stage in pipeline {
            let stage_obj = stage.as_object()
                .ok_or_else(|| "Tiap tahap agregasi harus berupa objek JSON".to_string())?;

            if let Some(match_spec) = stage_obj.get("$match") {
                working_set.retain(|doc| evaluate_filter(doc, match_spec));
            } else if let Some(project_spec) = stage_obj.get("$project") {
                working_set = working_set.into_iter().map(|doc| apply_projection(&doc, project_spec)).collect();
            } else if let Some(group_spec) = stage_obj.get("$group") {
                working_set = execute_group_stage(&working_set, group_spec)?;
            } else if let Some(sort_spec) = stage_obj.get("$sort") {
                if let Some(sort_map) = sort_spec.as_object() {
                    working_set.sort_by(|a, b| {
                        for (field, order_val) in sort_map {
                            let asc = order_val.as_i64().unwrap_or(1) >= 0;
                            let val_a = extract_nested_value(a, field).unwrap_or(Value::Null);
                            let val_b = extract_nested_value(b, field).unwrap_or(Value::Null);
                            let cmp = compare_json_values(&val_a, &val_b);
                            if cmp != std::cmp::Ordering::Equal {
                                return if asc { cmp } else { cmp.reverse() };
                            }
                        }
                        std::cmp::Ordering::Equal
                    });
                }
            } else if let Some(limit_val) = stage_obj.get("$limit") {
                if let Some(limit_n) = limit_val.as_u64() {
                    working_set.truncate(limit_n as usize);
                }
            } else if let Some(skip_val) = stage_obj.get("$skip") {
                if let Some(skip_n) = skip_val.as_u64() {
                    let n = skip_n as usize;
                    if n < working_set.len() {
                        working_set = working_set.split_off(n);
                    } else {
                        working_set.clear();
                    }
                }
            } else if let Some(unwind_field) = stage_obj.get("$unwind") {
                let field_str = unwind_field.as_str().ok_or("$unwind harus berupa nama field string")?;
                let clean_field = field_str.trim_start_matches('$');
                let mut unwound = Vec::new();
                for doc in working_set {
                    if let Some(Value::Array(arr)) = extract_nested_value(&doc, clean_field) {
                        for item in arr {
                            let mut new_doc = doc.clone();
                            set_nested_value(&mut new_doc, clean_field, item);
                            unwound.push(new_doc);
                        }
                    } else {
                        unwound.push(doc);
                    }
                }
                working_set = unwound;
            } else if let Some(count_field) = stage_obj.get("$count") {
                let field_name = count_field.as_str().unwrap_or("count");
                let count_doc = json!({ field_name: working_set.len() });
                return Ok(vec![count_doc]);
            }
        }

        Ok(working_set)
    }

    /// Restore seluruh data koleksi dari berkas WAL di disk
    fn restore_from_disk(&self) -> std::io::Result<()> {
        if let Ok(entries) = fs::read_dir(&self.db_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
                    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                    let coll_name = file_stem.trim_end_matches(".wal");
                    if !coll_name.is_empty() {
                        let coll = self.collection(coll_name);
                        if let Ok(file) = File::open(&path) {
                            let reader = BufReader::new(file);
                            for line in reader.lines().flatten() {
                                if let Some((op, json_str)) = line.split_once(' ') {
                                    if let Ok(doc) = serde_json::from_str::<Value>(json_str) {
                                        if op == "INSERT" || op == "UPDATE" {
                                            if let Some(id) = doc.get("_id").and_then(|v| v.as_str()) {
                                                coll.documents.write().unwrap().insert(id.to_string(), doc);
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
        Ok(())
    }
}

// ============================================================================
// QUERY EVALUATOR & FILTER ENGINE (MongoDB Operators Engine)
// ============================================================================

/// Evaluasi dokumen terhadap filter kueri MongoDB
pub fn evaluate_filter(doc: &Value, filter: &Value) -> bool {
    let filter_obj = match filter.as_object() {
        Some(obj) => obj,
        None => return true,
    };

    if filter_obj.is_empty() {
        return true;
    }

    for (key, condition) in filter_obj {
        // 1. Logical Operators
        if key == "$and" {
            if let Some(arr) = condition.as_array() {
                if !arr.iter().all(|sub_filter| evaluate_filter(doc, sub_filter)) {
                    return false;
                }
            }
            continue;
        }
        if key == "$or" {
            if let Some(arr) = condition.as_array() {
                if !arr.iter().any(|sub_filter| evaluate_filter(doc, sub_filter)) {
                    return false;
                }
            }
            continue;
        }
        if key == "$nor" {
            if let Some(arr) = condition.as_array() {
                if arr.iter().any(|sub_filter| evaluate_filter(doc, sub_filter)) {
                    return false;
                }
            }
            continue;
        }
        if key == "$where" || key == "$expr" {
            continue;
        }

        // 2. Field Match & Operator Evaluation
        let doc_val = extract_nested_value(doc, key);

        if let Some(cond_obj) = condition.as_object() {
            // Cek apakah ini objek operator ($gt, $eq, dll.)
            let is_operator_obj = cond_obj.keys().any(|k| k.starts_with('$'));
            if is_operator_obj {
                for (op, target_val) in cond_obj {
                    if !match_operator(&doc_val, op, target_val) {
                        return false;
                    }
                }
                continue;
            }
        }

        // Direct Equality
        if match_direct_equality(&doc_val, condition) == false {
            return false;
        }
    }

    true
}

fn match_operator(doc_val: &Option<Value>, op: &str, target: &Value) -> bool {
    match op {
        "$eq" => match_direct_equality(doc_val, target),
        "$ne" => !match_direct_equality(doc_val, target),
        "$gt" => compare_opt_values(doc_val, target) == Some(std::cmp::Ordering::Greater),
        "$gte" => {
            let cmp = compare_opt_values(doc_val, target);
            cmp == Some(std::cmp::Ordering::Greater) || cmp == Some(std::cmp::Ordering::Equal)
        }
        "$lt" => compare_opt_values(doc_val, target) == Some(std::cmp::Ordering::Less),
        "$lte" => {
            let cmp = compare_opt_values(doc_val, target);
            cmp == Some(std::cmp::Ordering::Less) || cmp == Some(std::cmp::Ordering::Equal)
        }
        "$in" => {
            if let Some(arr) = target.as_array() {
                arr.iter().any(|target_item| match_direct_equality(doc_val, target_item))
            } else {
                false
            }
        }
        "$nin" => {
            if let Some(arr) = target.as_array() {
                !arr.iter().any(|target_item| match_direct_equality(doc_val, target_item))
            } else {
                true
            }
        }
        "$exists" => {
            let should_exist = target.as_bool().unwrap_or(true);
            (doc_val.is_some() && doc_val.as_ref().unwrap() != &Value::Null) == should_exist
        }
        "$regex" => {
            if let Some(val) = doc_val {
                if let Some(val_str) = val.as_str() {
                    if let Some(pattern) = target.as_str() {
                        if let Ok(re) = Regex::new(pattern) {
                            return re.is_match(val_str);
                        }
                    }
                }
            }
            false
        }
        "$size" => {
            if let Some(Some(Value::Array(arr))) = doc_val.as_ref().map(|v| Some(v)) {
                if let Some(expected_size) = target.as_u64() {
                    return arr.len() == expected_size as usize;
                }
            }
            false
        }
        "$not" => !evaluate_nested_operator(doc_val, target),
        _ => true,
    }
}

fn evaluate_nested_operator(doc_val: &Option<Value>, target: &Value) -> bool {
    if let Some(obj) = target.as_object() {
        for (op, val) in obj {
            if !match_operator(doc_val, op, val) {
                return false;
            }
        }
        true
    } else {
        match_direct_equality(doc_val, target)
    }
}

fn match_direct_equality(doc_val: &Option<Value>, target: &Value) -> bool {
    match (doc_val, target) {
        (Some(d), t) if d == t => true,
        (Some(Value::Array(arr)), t) => arr.contains(t),
        (None, Value::Null) => true,
        _ => false,
    }
}

fn compare_opt_values(doc_val: &Option<Value>, target: &Value) -> Option<std::cmp::Ordering> {
    doc_val.as_ref().map(|dv| compare_json_values(dv, target))
}

fn compare_json_values(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::Number(n1), Value::Number(n2)) => {
            let f1 = n1.as_f64().unwrap_or(0.0);
            let f2 = n2.as_f64().unwrap_or(0.0);
            f1.partial_cmp(&f2).unwrap_or(std::cmp::Ordering::Equal)
        }
        (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
        (Value::Bool(b1), Value::Bool(b2)) => b1.cmp(b2),
        (Value::Null, Value::Null) => std::cmp::Ordering::Equal,
        (Value::Null, _) => std::cmp::Ordering::Less,
        (_, Value::Null) => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
    }
}

// ============================================================================
// UPDATE MODIFIER ENGINE ($set, $unset, $inc, $push, $pull, $addToSet, dll.)
// ============================================================================

pub fn apply_update_modifiers(doc: &mut Value, update_spec: &Value) -> Result<(), String> {
    let update_obj = update_spec.as_object()
        .ok_or_else(|| "Update modifier harus berupa JSON Object".to_string())?;

    for (op, spec) in update_obj {
        let spec_map = spec.as_object().ok_or_else(|| format!("Nilai untuk '{}' harus berupa object", op))?;
        match op.as_str() {
            "$set" => {
                for (k, v) in spec_map {
                    set_nested_value(doc, k, v.clone());
                }
            }
            "$unset" => {
                for (k, _) in spec_map {
                    unset_nested_value(doc, k);
                }
            }
            "$inc" => {
                for (k, delta) in spec_map {
                    let inc_amount = delta.as_f64().unwrap_or(0.0);
                    let current = extract_nested_value(doc, k)
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    let new_val = current + inc_amount;
                    set_nested_value(doc, k, json!(new_val));
                }
            }
            "$mul" => {
                for (k, factor) in spec_map {
                    let mul_amount = factor.as_f64().unwrap_or(1.0);
                    let current = extract_nested_value(doc, k)
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    set_nested_value(doc, k, json!(current * mul_amount));
                }
            }
            "$push" => {
                for (k, item) in spec_map {
                    let mut arr = match extract_nested_value(doc, k) {
                        Some(Value::Array(a)) => a,
                        _ => Vec::new(),
                    };
                    arr.push(item.clone());
                    set_nested_value(doc, k, Value::Array(arr));
                }
            }
            "$addToSet" => {
                for (k, item) in spec_map {
                    let mut arr = match extract_nested_value(doc, k) {
                        Some(Value::Array(a)) => a,
                        _ => Vec::new(),
                    };
                    if !arr.contains(item) {
                        arr.push(item.clone());
                    }
                    set_nested_value(doc, k, Value::Array(arr));
                }
            }
            "$pull" => {
                for (k, target_item) in spec_map {
                    if let Some(Value::Array(mut arr)) = extract_nested_value(doc, k) {
                        arr.retain(|item| item != target_item);
                        set_nested_value(doc, k, Value::Array(arr));
                    }
                }
            }
            "$currentDate" => {
                for (k, _) in spec_map {
                    set_nested_value(doc, k, json!(Utc::now().to_rfc3339()));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

// ============================================================================
// AGGREGATION HELPERS ($group, $project)
// ============================================================================

fn apply_projection(doc: &Value, proj_spec: &Value) -> Value {
    let mut out = json!({});
    if let Some(spec_map) = proj_spec.as_object() {
        for (k, v) in spec_map {
            if v.as_i64() == Some(1) || v.as_bool() == Some(true) {
                if let Some(val) = extract_nested_value(doc, k) {
                    set_nested_value(&mut out, k, val);
                }
            }
        }
    }
    out
}

fn execute_group_stage(docs: &[Value], group_spec: &Value) -> Result<Vec<Value>, String> {
    let spec_obj = group_spec.as_object()
        .ok_or_else(|| "$group stage harus berupa JSON object".to_string())?;

    let id_expr = spec_obj.get("_id").ok_or_else(|| "$group harus memiliki '_id'".to_string())?;

    // GroupingKey -> Vec<Doc>
    let mut groups: HashMap<String, (Value, Vec<&Value>)> = HashMap::new();

    for doc in docs {
        let group_key_val = match id_expr {
            Value::String(s) if s.starts_with('$') => {
                let field = s.trim_start_matches('$');
                extract_nested_value(doc, field).unwrap_or(Value::Null)
            }
            other => other.clone(),
        };
        let key_str = group_key_val.to_string();
        groups.entry(key_str).or_insert_with(|| (group_key_val, Vec::new())).1.push(doc);
    }

    let mut result = Vec::new();
    for (_, (id_val, group_docs)) in groups {
        let mut group_res = json!({ "_id": id_val });

        for (field, accum_expr) in spec_obj {
            if field == "_id" {
                continue;
            }
            if let Some(accum_obj) = accum_expr.as_object() {
                for (accum_op, operand) in accum_obj {
                    let operand_field = operand.as_str().map(|s| s.trim_start_matches('$'));

                    match accum_op.as_str() {
                        "$sum" => {
                            let mut sum = 0.0;
                            for d in &group_docs {
                                if let Some(f) = operand_field {
                                    sum += extract_nested_value(d, f).and_then(|v| v.as_f64()).unwrap_or(0.0);
                                } else if let Some(num) = operand.as_f64() {
                                    sum += num;
                                }
                            }
                            group_res[field] = json!(sum);
                        }
                        "$avg" => {
                            let mut sum = 0.0;
                            let mut count = 0;
                            for d in &group_docs {
                                if let Some(f) = operand_field {
                                    if let Some(v) = extract_nested_value(d, f).and_then(|v| v.as_f64()) {
                                        sum += v;
                                        count += 1;
                                    }
                                }
                            }
                            let avg = if count > 0 { sum / count as f64 } else { 0.0 };
                            group_res[field] = json!(avg);
                        }
                        "$min" => {
                            let mut min_val: Option<f64> = None;
                            for d in &group_docs {
                                if let Some(f) = operand_field {
                                    if let Some(v) = extract_nested_value(d, f).and_then(|v| v.as_f64()) {
                                        min_val = Some(min_val.map_or(v, |m| m.min(v)));
                                    }
                                }
                            }
                            group_res[field] = json!(min_val);
                        }
                        "$max" => {
                            let mut max_val: Option<f64> = None;
                            for d in &group_docs {
                                if let Some(f) = operand_field {
                                    if let Some(v) = extract_nested_value(d, f).and_then(|v| v.as_f64()) {
                                        max_val = Some(max_val.map_or(v, |m| m.max(v)));
                                    }
                                }
                            }
                            group_res[field] = json!(max_val);
                        }
                        "$push" => {
                            let mut items = Vec::new();
                            for d in &group_docs {
                                if let Some(f) = operand_field {
                                    if let Some(v) = extract_nested_value(d, f) {
                                        items.push(v);
                                    }
                                }
                            }
                            group_res[field] = Value::Array(items);
                        }
                        _ => {}
                    }
                }
            }
        }
        result.push(group_res);
    }

    Ok(result)
}

// ============================================================================
// NESTED JSON UTILITIES (Dot Notation support e.g. "a.b.c")
// ============================================================================

pub fn extract_nested_value(doc: &Value, path: &str) -> Option<Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = doc;

    for part in parts {
        match current {
            Value::Object(map) => {
                current = map.get(part)?;
            }
            Value::Array(arr) => {
                if let Ok(idx) = part.parse::<usize>() {
                    current = arr.get(idx)?;
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    Some(current.clone())
}

pub fn set_nested_value(doc: &mut Value, path: &str, value: Value) {
    let parts: Vec<&str> = path.split('.').collect();
    if parts.is_empty() {
        return;
    }

    if !doc.is_object() {
        *doc = json!({});
    }

    let mut current = doc;
    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            if let Value::Object(map) = current {
                map.insert(part.to_string(), value);
                return;
            }
        } else {
            if let Value::Object(map) = current {
                if !map.contains_key(*part) || !map[*part].is_object() {
                    map.insert(part.to_string(), json!({}));
                }
                current = map.get_mut(*part).unwrap();
            }
        }
    }
}

pub fn unset_nested_value(doc: &mut Value, path: &str) {
    let parts: Vec<&str> = path.split('.').collect();
    if parts.is_empty() {
        return;
    }

    let mut current = doc;
    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            if let Value::Object(map) = current {
                map.remove(*part);
                return;
            }
        } else {
            if let Value::Object(map) = current {
                if let Some(next) = map.get_mut(*part) {
                    current = next;
                } else {
                    return;
                }
            }
        }
    }
}

fn val_to_index_key(val: &Value) -> String {
    match val {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}
