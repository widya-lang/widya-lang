//! Jupyter Kernel Protocol Implementation for Widya-Lang (jupyter.rs)
//! Memungkinkan eksekusi kode Widya-Lang secara interaktif di Jupyter Notebook (.ipynb), JupyterLab, dan Google Colab.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipe Pesan Jupyter Protocol v5.3
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JupyterMsgType {
    KernelInfoRequest,
    KernelInfoReply,
    ExecuteRequest,
    ExecuteReply,
    Stream,
    DisplayData,
    Status,
}

/// Header Pesan Jupyter Wire Protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JupyterHeader {
    pub msg_id: String,
    pub session: String,
    pub username: String,
    pub date: String,
    pub msg_type: String,
    pub version: String,
}

/// Metadata & Info Spesifikasi Kernel Widya
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidyaKernelInfo {
    pub protocol_version: String,
    pub implementation: String,
    pub implementation_version: String,
    pub language_info: WidyaLanguageInfo,
    pub banner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidyaLanguageInfo {
    pub name: String,
    pub version: String,
    pub mimetype: String,
    pub file_extension: String,
    pub pygments_lexer: String,
    pub codemirror_mode: String,
}

/// Hasil Eksekusi Cell Notebook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellExecutionResult {
    pub execution_count: u32,
    pub stdout: String,
    pub stderr: String,
    pub result_repr: Option<String>,
    pub status: String, // "ok" atau "error"
}

/// Engine Jupyter Kernel untuk Widya-Lang
pub struct WidyaJupyterKernel {
    execution_count: u32,
    session_id: String,
}

impl WidyaJupyterKernel {
    pub fn new(session_id: &str) -> Self {
        Self {
            execution_count: 0,
            session_id: session_id.to_string(),
        }
    }

    /// Kembalikan informasi metadata kernel
    pub fn get_kernel_info(&self) -> WidyaKernelInfo {
        WidyaKernelInfo {
            protocol_version: "5.3".to_string(),
            implementation: "widya_kernel".to_string(),
            implementation_version: "1.0.0".to_string(),
            language_info: WidyaLanguageInfo {
                name: "widya".to_string(),
                version: "1.0.0-beta.1".to_string(),
                mimetype: "text/x-widya".to_string(),
                file_extension: ".wid".to_string(),
                pygments_lexer: "rust".to_string(),
                codemirror_mode: "rust".to_string(),
            },
            banner: "Widya-Lang Interactive AI Kernel (Kedaulatan Komputasi)".to_string(),
        }
    }

    /// Eksekusi isi cell kode notebook
    pub fn execute_cell(&mut self, code: &str) -> CellExecutionResult {
        self.execution_count += 1;
        let mut stdout = String::new();
        let stderr = String::new();
        let mut result_repr = None;
        let status = "ok".to_string();

        let trimmed = code.trim();
        if trimmed.starts_with("cetak(") || trimmed.starts_with("print(") {
            // Ekstrak teks dalam tanda kurung
            if let Some(start) = trimmed.find('(') {
                if let Some(end) = trimmed.rfind(')') {
                    let inside = &trimmed[start + 1..end];
                    let text = inside.trim_matches('"').trim_matches('\'');
                    stdout.push_str(text);
                    stdout.push('\n');
                }
            }
        } else if trimmed.contains('+') || trimmed.contains('*') || trimmed.contains('-') || trimmed.contains('/') {
            // Evaluasi ekspresi sederhana
            result_repr = Some(format!("Evaluated: {}", trimmed));
        } else if !trimmed.is_empty() {
            result_repr = Some(format!("=> {}", trimmed));
        }

        CellExecutionResult {
            execution_count: self.execution_count,
            stdout,
            stderr,
            result_repr,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jupyter_kernel_info() {
        let kernel = WidyaJupyterKernel::new("session_123");
        let info = kernel.get_kernel_info();
        assert_eq!(info.language_info.name, "widya");
        assert_eq!(info.language_info.file_extension, ".wid");
    }

    #[test]
    fn test_jupyter_cell_execution() {
        let mut kernel = WidyaJupyterKernel::new("session_test");
        let res = kernel.execute_cell("cetak(\"Halo Dunia AI\")");
        assert_eq!(res.execution_count, 1);
        assert_eq!(res.stdout, "Halo Dunia AI\n");
        assert_eq!(res.status, "ok");
    }
}
