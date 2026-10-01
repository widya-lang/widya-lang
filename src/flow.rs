//! WidyaFlow — Visual Low-Code Workflow & ETL Transpiler Engine (n8n / Airflow Equivalent)
//! Enables visual drag-and-drop workflow graph definition and automatically transpiles
//! node graphs into native, high-performance Widya-Lang (.wya) executable scripts.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlowNodeType {
    WebhookTrigger,
    TransformJson,
    DatabaseQuery,
    FilterCondition,
    ProduceStreamEvent,
    SendNotification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowNode {
    pub id: String,
    pub name: String,
    pub node_type: FlowNodeType,
    pub config: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowConnection {
    pub from_node: String,
    pub to_node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualFlowGraph {
    pub flow_id: String,
    pub flow_name: String,
    pub nodes: Vec<FlowNode>,
    pub connections: Vec<FlowConnection>,
}

pub struct FlowTranspiler;

impl FlowTranspiler {
    /// Transpiles visual node graph into native executable .wya source code
    pub fn transpile_to_widya_code(graph: &VisualFlowGraph) -> String {
        let mut code = String::new();
        code.push_str(&format!("// ====================================================\n"));
        code.push_str(&format!("// ALUR OTOMASI BERDAULAT: {}\n", graph.flow_name));
        code.push_str(&format!("// Dihasilkan secara otomatis oleh WidyaFlow Engine\n"));
        code.push_str(&format!("// ====================================================\n\n"));
        code.push_str("fungsi utama() {\n");
        code.push_str("    tulis(\"🚀 Memulai alur kerja otomatisasi...\");\n\n");

        for node in &graph.nodes {
            code.push_str(&format!("    // Langkah: {}\n", node.name));
            match node.node_type {
                FlowNodeType::WebhookTrigger => {
                    let path = node.config.get("path").cloned().unwrap_or("/webhook".to_string());
                    code.push_str(&format!("    let payload_masuk = web::tangkap_webhook(\"{}\");\n\n", path));
                }
                FlowNodeType::TransformJson => {
                    code.push_str("    let data_terformat = json::transformasi(payload_masuk);\n\n");
                }
                FlowNodeType::DatabaseQuery => {
                    let query = node.config.get("sql").cloned().unwrap_or("SELECT 1".to_string());
                    code.push_str(&format!("    let hasil_db = widyabase::eksekusi_sql(\"{}\");\n\n", query));
                }
                FlowNodeType::FilterCondition => {
                    code.push_str("    jika (data_terformat.status == \"VALID\") {\n");
                    code.push_str("        tulis(\"✅ Data valid, melanjutkan...\");\n");
                    code.push_str("    }\n\n");
                }
                FlowNodeType::ProduceStreamEvent => {
                    let topic = node.config.get("topic").cloned().unwrap_or("transaksi".to_string());
                    code.push_str(&format!("    stream::kirim_event(\"{}\", data_terformat);\n\n", topic));
                }
                FlowNodeType::SendNotification => {
                    code.push_str("    notifikasi::kirim_telegram(\"Transaksi Berhasil Diproses\");\n\n");
                }
            }
        }

        code.push_str("    tulis(\"✨ Alur kerja selesai dengan sukses.\");\n");
        code.push_str("}\n");
        code
    }
}
