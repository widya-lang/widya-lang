// ============================================================================
// WIDYABASE DATABASE TRIGGERS & AUDIT TRAIL ENGINE (trigger.rs)
// ============================================================================
// Menyediakan pemicu basis data asli (BEFORE/AFTER INSERT/UPDATE/DELETE),
// penegakan kendala integritas data deklaratif, mutasi kolom otomatis (timestamp),
// dan pencatatan jejak audit (Audit Trail) dengan rekod LAMA (OLD) & BARU (NEW).
// ============================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

/// Waktu Pemicu Dieksekusi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerTiming {
    Before,    // SEBELUM
    After,     // SETELAH
    InsteadOf, // SEBAGAI_GANTI
}

/// Kejadian Aksi yang Memicu Trigger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerEvent {
    Insert, // SISIP
    Update, // UBAH
    Delete, // HAPUS
}

/// Tingkat Granularitas Pemicu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerLevel {
    Row,       // SETIAP_BARIS
    Statement, // SETIAP_PERNYATAAN
}

/// Representasi baris data mentah untuk evaluasi OLD & NEW
pub type RowData = HashMap<String, String>;

/// Aksi yang Dijalankan oleh Pemicu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerAction {
    /// Validasi kendala kustom: jika kondisi salah, batalkan transaksi
    ValidateCondition {
        column_name: String,
        rule_description: String,
        min_numeric: Option<f64>,
        max_numeric: Option<f64>,
        not_empty: bool,
    },
    /// Otomatis isi/mutasi kolom saat penulisan (misal: updated_at, status_verifikasi)
    AutoFillColumn {
        target_column: String,
        value_pattern: String,
    },
    /// Pencatatan riwayat perubahan ke tabel audit trail
    LogAuditTrail {
        actor: String,
    },
}

/// Definisi Pemicu (Trigger Definition)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerDefinition {
    pub name: String,
    pub table_name: String,
    pub timing: TriggerTiming,
    pub event: TriggerEvent,
    pub level: TriggerLevel,
    pub action: TriggerAction,
    pub is_enabled: bool,
}

/// Rekod Catatan Jejak Audit Resmi (Audit Trail Record)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub audit_id: u64,
    pub table_name: String,
    pub operation: String,
    pub timestamp_epoch_ms: u64,
    pub old_values: Option<RowData>,
    pub new_values: Option<RowData>,
    pub changed_columns: Vec<String>,
    pub actor: String,
}

/// Pengelola Pemicu dan Jejak Audit Basis Data
pub struct TriggerManager {
    // table_name (uppercase) -> Vec<TriggerDefinition>
    triggers: RwLock<HashMap<String, Vec<TriggerDefinition>>>,
    audit_log: RwLock<Vec<AuditRecord>>,
    next_audit_id: AtomicU64,
}

impl TriggerManager {
    pub fn new() -> Self {
        Self {
            triggers: RwLock::new(HashMap::new()),
            audit_log: RwLock::new(Vec::new()),
            next_audit_id: AtomicU64::new(1),
        }
    }

    /// Mendaftarkan pemicu baru ke tabel target
    pub fn create_trigger(&self, def: TriggerDefinition) {
        let tbl = def.table_name.to_uppercase();
        let mut trigs = self.triggers.write().unwrap();
        trigs.entry(tbl).or_default().push(def);
    }

    /// Menghapus pemicu berdasarkan nama
    pub fn drop_trigger(&self, trigger_name: &str) -> bool {
        let mut trigs = self.triggers.write().unwrap();
        let mut found = false;
        for list in trigs.values_mut() {
            let before_len = list.len();
            list.retain(|t| t.name != trigger_name);
            if list.len() < before_len {
                found = true;
            }
        }
        found
    }

    /// Mengeksekusi pemicu yang relevan sebelum/setelah aksi mutasi baris data
    pub fn fire_row_triggers(
        &self,
        table_name: &str,
        timing: TriggerTiming,
        event: TriggerEvent,
        old_row: Option<&RowData>,
        mut new_row: Option<&mut RowData>,
    ) -> Result<(), String> {
        let tbl = table_name.to_uppercase();
        let trigs = self.triggers.read().unwrap();
        let trigger_list = match trigs.get(&tbl) {
            Some(l) => l.clone(),
            None => return Ok(()),
        };
        drop(trigs);

        for trig in trigger_list {
            if !trig.is_enabled || trig.timing != timing || trig.event != event {
                continue;
            }

            match &trig.action {
                TriggerAction::ValidateCondition {
                    column_name,
                    rule_description,
                    min_numeric,
                    max_numeric,
                    not_empty,
                } => {
                    let target_data = new_row.as_deref().or(old_row);
                    if let Some(data) = target_data {
                        if let Some(val) = data.get(column_name) {
                            if *not_empty && val.trim().is_empty() {
                                return Err(format!(
                                    "Trigger '{}' menolak operasi: Kolom '{}' tidak boleh kosong ({})",
                                    trig.name, column_name, rule_description
                                ));
                            }
                            if let Ok(num) = val.parse::<f64>() {
                                if let Some(min) = min_numeric {
                                    if num < *min {
                                        return Err(format!(
                                            "Trigger '{}' menolak operasi: Kolom '{}' bernilai {} < batas minimum {} ({})",
                                            trig.name, column_name, num, min, rule_description
                                        ));
                                    }
                                }
                                if let Some(max) = max_numeric {
                                    if num > *max {
                                        return Err(format!(
                                            "Trigger '{}' menolak operasi: Kolom '{}' bernilai {} > batas maksimum {} ({})",
                                            trig.name, column_name, num, max, rule_description
                                        ));
                                    }
                                }
                            }
                        } else if *not_empty {
                            return Err(format!(
                                "Trigger '{}' menolak operasi: Kolom '{}' tidak ditemukan ({})",
                                trig.name, column_name, rule_description
                            ));
                        }
                    }
                }
                TriggerAction::AutoFillColumn {
                    target_column,
                    value_pattern,
                } => {
                    if let Some(data) = new_row.as_deref_mut() {
                        let fill_val = if value_pattern == "NOW()" || value_pattern == "SEKARANG()" {
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_millis().to_string())
                                .unwrap_or_else(|_| "0".to_string())
                        } else {
                            value_pattern.clone()
                        };
                        data.insert(target_column.clone(), fill_val);
                    }
                }
                TriggerAction::LogAuditTrail { actor } => {
                    let mut changed_cols = Vec::new();
                    if let (Some(o), Some(n)) = (old_row, new_row.as_deref()) {
                        for (k, v) in n.iter() {
                            if o.get(k) != Some(v) {
                                changed_cols.push(k.clone());
                            }
                        }
                    }

                    let op_str = match event {
                        TriggerEvent::Insert => "INSERT",
                        TriggerEvent::Update => "UPDATE",
                        TriggerEvent::Delete => "DELETE",
                    };

                    let record = AuditRecord {
                        audit_id: self.next_audit_id.fetch_add(1, Ordering::SeqCst),
                        table_name: tbl.clone(),
                        operation: op_str.to_string(),
                        timestamp_epoch_ms: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_millis() as u64)
                            .unwrap_or(0),
                        old_values: old_row.cloned(),
                        new_values: new_row.as_deref().cloned(),
                        changed_columns: changed_cols,
                        actor: actor.clone(),
                    };

                    self.audit_log.write().unwrap().push(record);
                }
            }
        }

        Ok(())
    }

    /// Mengambil catatan jejak audit untuk tabel tertentu atau seluruh database
    pub fn get_audit_trail(&self, table_name: Option<&str>) -> Vec<AuditRecord> {
        let logs = self.audit_log.read().unwrap();
        if let Some(tbl) = table_name {
            let u_tbl = tbl.to_uppercase();
            logs.iter().filter(|r| r.table_name == u_tbl).cloned().collect()
        } else {
            logs.clone()
        }
    }
}

impl Default for TriggerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_validation_and_rejection() {
        let tm = TriggerManager::new();

        // Daftarkan trigger pencegah skor/poin negatif pada tabel atlet
        tm.create_trigger(TriggerDefinition {
            name: "cek_poin_positif".to_string(),
            table_name: "atlet_skor".to_string(),
            timing: TriggerTiming::Before,
            event: TriggerEvent::Insert,
            level: TriggerLevel::Row,
            action: TriggerAction::ValidateCondition {
                column_name: "poin".to_string(),
                rule_description: "Poin pesilat tidak boleh bernilai negatif".to_string(),
                min_numeric: Some(0.0),
                max_numeric: Some(100.0),
                not_empty: true,
            },
            is_enabled: true,
        });

        // 1. Uji masukan sah
        let mut valid_row: RowData = HashMap::new();
        valid_row.insert("poin".to_string(), "15".to_string());
        let res_valid = tm.fire_row_triggers(
            "atlet_skor",
            TriggerTiming::Before,
            TriggerEvent::Insert,
            None,
            Some(&mut valid_row),
        );
        assert!(res_valid.is_ok());

        // 2. Uji masukan tidak sah (poin negatif -5) -> harus ditolak
        let mut invalid_row: RowData = HashMap::new();
        invalid_row.insert("poin".to_string(), "-5".to_string());
        let res_invalid = tm.fire_row_triggers(
            "atlet_skor",
            TriggerTiming::Before,
            TriggerEvent::Insert,
            None,
            Some(&mut invalid_row),
        );
        assert!(res_invalid.is_err());
        assert!(res_invalid.unwrap_err().contains("tidak boleh bernilai negatif"));
    }

    #[test]
    fn test_trigger_autofill_and_audit_trail() {
        let tm = TriggerManager::new();

        // Trigger auto-fill timestamp
        tm.create_trigger(TriggerDefinition {
            name: "set_updated_at".to_string(),
            table_name: "produk".to_string(),
            timing: TriggerTiming::Before,
            event: TriggerEvent::Update,
            level: TriggerLevel::Row,
            action: TriggerAction::AutoFillColumn {
                target_column: "updated_at".to_string(),
                value_pattern: "NOW()".to_string(),
            },
            is_enabled: true,
        });

        // Trigger audit trail
        tm.create_trigger(TriggerDefinition {
            name: "audit_produk".to_string(),
            table_name: "produk".to_string(),
            timing: TriggerTiming::After,
            event: TriggerEvent::Update,
            level: TriggerLevel::Row,
            action: TriggerAction::LogAuditTrail {
                actor: "operator_gudang_01".to_string(),
            },
            is_enabled: true,
        });

        let mut old_row: RowData = HashMap::new();
        old_row.insert("nama".to_string(), "Sabuk Silat".to_string());
        old_row.insert("harga".to_string(), "50000".to_string());

        let mut new_row: RowData = HashMap::new();
        new_row.insert("nama".to_string(), "Sabuk Silat Merah".to_string());
        new_row.insert("harga".to_string(), "65000".to_string());

        // Jalankan BEFORE UPDATE (autofill)
        tm.fire_row_triggers(
            "produk",
            TriggerTiming::Before,
            TriggerEvent::Update,
            Some(&old_row),
            Some(&mut new_row),
        ).unwrap();

        assert!(new_row.contains_key("updated_at"));

        // Jalankan AFTER UPDATE (audit log)
        tm.fire_row_triggers(
            "produk",
            TriggerTiming::After,
            TriggerEvent::Update,
            Some(&old_row),
            Some(&mut new_row),
        ).unwrap();

        let audits = tm.get_audit_trail(Some("produk"));
        assert_eq!(audits.len(), 1);
        assert_eq!(audits[0].operation, "UPDATE");
        assert_eq!(audits[0].actor, "operator_gudang_01");
        assert!(audits[0].changed_columns.contains(&"nama".to_string()));
        assert!(audits[0].changed_columns.contains(&"harga".to_string()));
    }
}
