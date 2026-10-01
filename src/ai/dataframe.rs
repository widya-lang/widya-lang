//! High-Performance Columnar DataFrame Engine (dataframe.rs)
//! Mengimplementasikan arsitektur memori columnar (seperti Apache Arrow / Polars) untuk manipulasi data tabular berkecepatan tinggi dalam AI & Data Science.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipe Data Kolom DataFrame
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColumnType {
    Int64,
    Float64,
    String,
    Boolean,
}

/// Kolom Data Berbasis Array Homogen
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataColumn {
    Int64(Vec<i64>),
    Float64(Vec<f64>),
    String(Vec<String>),
    Boolean(Vec<bool>),
}

impl DataColumn {
    pub fn len(&self) -> usize {
        match self {
            DataColumn::Int64(v) => v.len(),
            DataColumn::Float64(v) => v.len(),
            DataColumn::String(v) => v.len(),
            DataColumn::Boolean(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn col_type(&self) -> ColumnType {
        match self {
            DataColumn::Int64(_) => ColumnType::Int64,
            DataColumn::Float64(_) => ColumnType::Float64,
            DataColumn::String(_) => ColumnType::String,
            DataColumn::Boolean(_) => ColumnType::Boolean,
        }
    }
}

/// Struktur Utama Columnar DataFrame
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataFrame {
    columns: HashMap<String, DataColumn>,
    column_order: Vec<String>,
    num_rows: usize,
}

impl DataFrame {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            column_order: Vec::new(),
            num_rows: 0,
        }
    }

    /// Menambahkan kolom baru ke DataFrame
    pub fn with_column(mut self, name: &str, column: DataColumn) -> Result<Self, String> {
        let col_len = column.len();
        if self.column_order.is_empty() {
            self.num_rows = col_len;
        } else if col_len != self.num_rows {
            return Err(format!(
                "Panjang kolom '{}' ({}) tidak cocok dengan jumlah baris dataframe ({})",
                name, col_len, self.num_rows
            ));
        }

        self.columns.insert(name.to_string(), column);
        if !self.column_order.iter().any(|c| c == name) {
            self.column_order.push(name.to_string());
        }
        Ok(self)
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_cols(&self) -> usize {
        self.columns.len()
    }

    pub fn column_names(&self) -> &[String] {
        &self.column_order
    }

    pub fn get_column(&self, name: &str) -> Option<&DataColumn> {
        self.columns.get(name)
    }

    /// Filter baris berdasarkan predikat kolom numerik (misal: col > threshold)
    pub fn filter_f64<F>(&self, col_name: &str, predicate: F) -> Result<DataFrame, String>
    where
        F: Fn(f64) -> bool,
    {
        let col = self.columns.get(col_name)
            .ok_or_else(|| format!("Kolom '{}' tidak ditemukan", col_name))?;

        let f64_vals = match col {
            DataColumn::Float64(vals) => vals,
            _ => return Err(format!("Kolom '{}' bukan bertipe Float64", col_name)),
        };

        let mask: Vec<bool> = f64_vals.iter().map(|&v| predicate(v)).collect();
        let new_rows = mask.iter().filter(|&&m| m).count();

        let mut new_df = DataFrame {
            columns: HashMap::new(),
            column_order: self.column_order.clone(),
            num_rows: new_rows,
        };

        for (name, c) in &self.columns {
            let filtered_col = match c {
                DataColumn::Int64(v) => {
                    let filtered: Vec<i64> = v.iter().zip(&mask).filter_map(|(&val, &m)| if m { Some(val) } else { None }).collect();
                    DataColumn::Int64(filtered)
                }
                DataColumn::Float64(v) => {
                    let filtered: Vec<f64> = v.iter().zip(&mask).filter_map(|(&val, &m)| if m { Some(val) } else { None }).collect();
                    DataColumn::Float64(filtered)
                }
                DataColumn::String(v) => {
                    let filtered: Vec<String> = v.iter().zip(&mask).filter_map(|(val, &m)| if m { Some(val.clone()) } else { None }).collect();
                    DataColumn::String(filtered)
                }
                DataColumn::Boolean(v) => {
                    let filtered: Vec<bool> = v.iter().zip(&mask).filter_map(|(&val, &m)| if m { Some(val) } else { None }).collect();
                    DataColumn::Boolean(filtered)
                }
            };
            new_df.columns.insert(name.clone(), filtered_col);
        }

        Ok(new_df)
    }

    /// Menghitung mean/rata-rata kolom numerik
    pub fn mean(&self, col_name: &str) -> Result<f64, String> {
        let col = self.columns.get(col_name)
            .ok_or_else(|| format!("Kolom '{}' tidak ditemukan", col_name))?;

        match col {
            DataColumn::Float64(v) => {
                if v.is_empty() { return Ok(0.0); }
                let sum: f64 = v.iter().sum();
                Ok(sum / (v.len() as f64))
            }
            DataColumn::Int64(v) => {
                if v.is_empty() { return Ok(0.0); }
                let sum: i64 = v.iter().sum();
                Ok((sum as f64) / (v.len() as f64))
            }
            _ => Err("Mean hanya dapat dihitung pada kolom numerik".to_string()),
        }
    }

    /// Parsing CSV sederhana ke DataFrame
    pub fn from_csv_str(csv: &str, has_header: bool) -> Result<DataFrame, String> {
        let lines: Vec<&str> = csv.lines().filter(|l| !l.trim().is_empty()).collect();
        if lines.is_empty() {
            return Ok(DataFrame::new());
        }

        let headers: Vec<String> = if has_header {
            lines[0].split(',').map(|s| s.trim().to_string()).collect()
        } else {
            let col_count = lines[0].split(',').count();
            (0..col_count).map(|i| format!("col_{}", i)).collect()
        };

        let start_row = if has_header { 1 } else { 0 };
        let mut raw_cols: Vec<Vec<String>> = vec![Vec::new(); headers.len()];

        for line in &lines[start_row..] {
            let cells: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            for (i, cell) in cells.iter().enumerate() {
                if i < raw_cols.len() {
                    raw_cols[i].push(cell.to_string());
                }
            }
        }

        let mut df = DataFrame::new();
        for (i, col_data) in raw_cols.into_iter().enumerate() {
            let col_name = &headers[i];
            // Coba parse ke Float64 atau fallback ke String
            let is_f64 = col_data.iter().all(|s| s.parse::<f64>().is_ok());
            if is_f64 {
                let nums: Vec<f64> = col_data.iter().map(|s| s.parse::<f64>().unwrap()).collect();
                df = df.with_column(col_name, DataColumn::Float64(nums))?;
            } else {
                df = df.with_column(col_name, DataColumn::String(col_data))?;
            }
        }

        Ok(df)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataframe_creation_and_filter() {
        let df = DataFrame::new()
            .with_column("nama", DataColumn::String(vec!["Ahmad".to_string(), "Budi".to_string(), "Citra".to_string()]))
            .unwrap()
            .with_column("usia", DataColumn::Float64(vec![25.0, 30.0, 35.0]))
            .unwrap();

        assert_eq!(df.num_rows(), 3);
        assert_eq!(df.num_cols(), 2);

        let filtered = df.filter_f64("usia", |v| v > 28.0).unwrap();
        assert_eq!(filtered.num_rows(), 2);
        assert_eq!(filtered.mean("usia").unwrap(), 32.5);
    }

    #[test]
    fn test_dataframe_from_csv() {
        let csv = "fitur_a,fitur_b\n1.5,10.0\n2.5,20.0\n3.5,30.0";
        let df = DataFrame::from_csv_str(csv, true).unwrap();

        assert_eq!(df.num_rows(), 3);
        assert_eq!(df.mean("fitur_a").unwrap(), 2.5);
        assert_eq!(df.mean("fitur_b").unwrap(), 20.0);
    }
}
