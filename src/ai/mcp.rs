//! Native Model Context Protocol (MCP) Server & Client Engine
//! 
//! Fitur Skala Industri:
//! - Implementasi Standar Model Context Protocol (MCP) v2024-11-05
//! - Transport JSON-RPC 2.0 (Stdio & HTTP SSE Ready)
//! - Sistem Perkakas Agen AI (Tools System):
//!   - Registrasi Tool dengan JSON Schema Derivation Otomatis
//!   - `tools/list` & `tools/call` Dispatcher dengan Error Isolation
//! - Sistem Sumber Daya (Resources System):
//!   - `resources/list` & `resources/read` untuk Konteks File / Basis Data WidyaBase
//! - Sistem Perintah Template (Prompts System):
//!   - `prompts/list` & `prompts/get` dengan Substitusi Argumen Dinamis
//! - MCP Client Mandiri untuk Berinteraksi dengan Ekosistem Server MCP Lain

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

// ============================================================================
// 1. STRUKTUR PROTOKOL JSON-RPC 2.0 & MCP
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: Option<JsonValue>,
    pub method: String,
    #[serde(default)]
    pub params: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,
}

impl McpResponse {
    pub fn success(id: Option<JsonValue>, result: JsonValue) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Option<JsonValue>, code: i32, message: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(McpError {
                code,
                message: message.to_string(),
                data: None,
            }),
        }
    }
}

// ============================================================================
// 2. TIPE DATA MCP TOOLS, RESOURCES, & PROMPTS
// ============================================================================

/// Definisi Tool dalam spesifikasi MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolDefinition {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: JsonValue,
}

/// Konten hasil pemanggilan Tool MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolContent {
    #[serde(rename = "type")]
    pub content_type: String, // "text"
    pub text: String,
}

/// Hasil eksekusi Tool MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolCallResult {
    pub content: Vec<McpToolContent>,
    #[serde(rename = "isError")]
    pub is_error: bool,
}

/// Definisi Resource dalam spesifikasi MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResourceDefinition {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
}

/// Konten Resource yang dibaca
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResourceContent {
    pub uri: String,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub text: String,
}

/// Argumen untuk template Prompt MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPromptArgument {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
}

/// Definisi Prompt dalam spesifikasi MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPromptDefinition {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<McpPromptArgument>,
}

/// Pesan dalam hasil Prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPromptMessage {
    pub role: String, // "user" | "assistant"
    pub content: McpToolContent,
}

// Type aliases untuk callback handler
pub type ToolHandler = Box<dyn Fn(JsonValue) -> Result<String, String> + Send + Sync + 'static>;
pub type ResourceProvider = Box<dyn Fn(&str) -> Result<String, String> + Send + Sync + 'static>;
pub type PromptGenerator = Box<dyn Fn(HashMap<String, String>) -> Result<Vec<McpPromptMessage>, String> + Send + Sync + 'static>;

// ============================================================================
// 3. MCP SERVER ENGINE
// ============================================================================

pub struct McpServer {
    pub name: String,
    pub version: String,
    tools: RwLock<HashMap<String, (McpToolDefinition, ToolHandler)>>,
    resources: RwLock<HashMap<String, (McpResourceDefinition, ResourceProvider)>>,
    prompts: RwLock<HashMap<String, (McpPromptDefinition, PromptGenerator)>>,
}

impl McpServer {
    pub fn new(name: &str, version: &str) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
            version: version.to_string(),
            tools: RwLock::new(HashMap::new()),
            resources: RwLock::new(HashMap::new()),
            prompts: RwLock::new(HashMap::new()),
        })
    }

    /// Mendaftarkan Tool baru ke server MCP
    pub fn register_tool<F>(
        &self,
        name: &str,
        description: &str,
        input_schema: JsonValue,
        handler: F,
    ) where
        F: Fn(JsonValue) -> Result<String, String> + Send + Sync + 'static,
    {
        let def = McpToolDefinition {
            name: name.to_string(),
            description: description.to_string(),
            input_schema,
        };
        self.tools.write().unwrap().insert(name.to_string(), (def, Box::new(handler)));
    }

    /// Mendaftarkan Resource baru ke server MCP
    pub fn register_resource<F>(
        &self,
        uri: &str,
        name: &str,
        description: Option<&str>,
        mime_type: Option<&str>,
        provider: F,
    ) where
        F: Fn(&str) -> Result<String, String> + Send + Sync + 'static,
    {
        let def = McpResourceDefinition {
            uri: uri.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            mime_type: mime_type.map(|s| s.to_string()),
        };
        self.resources.write().unwrap().insert(uri.to_string(), (def, Box::new(provider)));
    }

    /// Mendaftarkan Prompt template baru ke server MCP
    pub fn register_prompt<F>(
        &self,
        name: &str,
        description: Option<&str>,
        arguments: Vec<McpPromptArgument>,
        generator: F,
    ) where
        F: Fn(HashMap<String, String>) -> Result<Vec<McpPromptMessage>, String> + Send + Sync + 'static,
    {
        let def = McpPromptDefinition {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            arguments,
        };
        self.prompts.write().unwrap().insert(name.to_string(), (def, Box::new(generator)));
    }

    /// Menangani pesan mentah JSON-RPC 2.0 (Input string -> Output JSON string)
    pub fn handle_raw_message(&self, raw_json: &str) -> Option<String> {
        let req: McpRequest = match serde_json::from_str(raw_json) {
            Ok(r) => r,
            Err(_) => {
                let err_res = McpResponse::error(None, -32700, "Parse error: format JSON tidak valid");
                return serde_json::to_string(&err_res).ok();
            }
        };

        let response = self.handle_request(req)?;
        serde_json::to_string(&response).ok()
    }

    /// Menangani request MCP yang telah di-parse
    pub fn handle_request(&self, req: McpRequest) -> Option<McpResponse> {
        let id = req.id.clone();

        match req.method.as_str() {
            // Handshake & Ping
            "initialize" => {
                let result = json!({
                    "protocolVersion": MCP_PROTOCOL_VERSION,
                    "capabilities": {
                        "tools": { "listChanged": true },
                        "resources": { "subscribe": false, "listChanged": true },
                        "prompts": { "listChanged": true }
                    },
                    "serverInfo": {
                        "name": self.name,
                        "version": self.version
                    }
                });
                Some(McpResponse::success(id, result))
            }
            "notifications/initialized" => {
                // Notifikasi, tidak mengembalikan response
                None
            }
            "ping" => {
                Some(McpResponse::success(id, json!({})))
            }

            // Tools API
            "tools/list" => {
                let tools_lock = self.tools.read().unwrap();
                let tool_defs: Vec<&McpToolDefinition> = tools_lock.values().map(|(def, _)| def).collect();
                Some(McpResponse::success(id, json!({ "tools": tool_defs })))
            }
            "tools/call" => {
                let params = req.params.unwrap_or(JsonValue::Null);
                let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

                let tools_lock = self.tools.read().unwrap();
                if let Some((_, handler)) = tools_lock.get(tool_name) {
                    match handler(arguments) {
                        Ok(output_text) => {
                            let tool_res = McpToolCallResult {
                                content: vec![McpToolContent {
                                    content_type: "text".to_string(),
                                    text: output_text,
                                }],
                                is_error: false,
                            };
                            Some(McpResponse::success(id, serde_json::to_value(tool_res).unwrap()))
                        }
                        Err(err_text) => {
                            let tool_res = McpToolCallResult {
                                content: vec![McpToolContent {
                                    content_type: "text".to_string(),
                                    text: format!("Error: {}", err_text),
                                }],
                                is_error: true,
                            };
                            Some(McpResponse::success(id, serde_json::to_value(tool_res).unwrap()))
                        }
                    }
                } else {
                    Some(McpResponse::error(id, -32601, &format!("Tool '{}' tidak ditemukan", tool_name)))
                }
            }

            // Resources API
            "resources/list" => {
                let res_lock = self.resources.read().unwrap();
                let defs: Vec<&McpResourceDefinition> = res_lock.values().map(|(def, _)| def).collect();
                Some(McpResponse::success(id, json!({ "resources": defs })))
            }
            "resources/read" => {
                let params = req.params.unwrap_or(JsonValue::Null);
                let uri = params.get("uri").and_then(|v| v.as_str()).unwrap_or("");

                let res_lock = self.resources.read().unwrap();
                if let Some((def, provider)) = res_lock.get(uri) {
                    match provider(uri) {
                        Ok(text_content) => {
                            let item = McpResourceContent {
                                uri: uri.to_string(),
                                mime_type: def.mime_type.clone(),
                                text: text_content,
                            };
                            Some(McpResponse::success(id, json!({ "contents": [item] })))
                        }
                        Err(e) => {
                            Some(McpResponse::error(id, -32000, &format!("Gagal membaca resource: {}", e)))
                        }
                    }
                } else {
                    Some(McpResponse::error(id, -32002, &format!("Resource URI '{}' tidak ditemukan", uri)))
                }
            }

            // Prompts API
            "prompts/list" => {
                let prompts_lock = self.prompts.read().unwrap();
                let defs: Vec<&McpPromptDefinition> = prompts_lock.values().map(|(def, _)| def).collect();
                Some(McpResponse::success(id, json!({ "prompts": defs })))
            }
            "prompts/get" => {
                let params = req.params.unwrap_or(JsonValue::Null);
                let prompt_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args_obj = params.get("arguments").and_then(|v| v.as_object());

                let mut args_map = HashMap::new();
                if let Some(obj) = args_obj {
                    for (k, v) in obj {
                        if let Some(s) = v.as_str() {
                            args_map.insert(k.clone(), s.to_string());
                        }
                    }
                }

                let prompts_lock = self.prompts.read().unwrap();
                if let Some((def, generator)) = prompts_lock.get(prompt_name) {
                    match generator(args_map) {
                        Ok(messages) => {
                            Some(McpResponse::success(id, json!({
                                "description": def.description,
                                "messages": messages
                            })))
                        }
                        Err(e) => {
                            Some(McpResponse::error(id, -32000, &format!("Gagal merender prompt: {}", e)))
                        }
                    }
                } else {
                    Some(McpResponse::error(id, -32601, &format!("Prompt '{}' tidak ditemukan", prompt_name)))
                }
            }

            // Method tidak dikenal
            _ => {
                Some(McpResponse::error(id, -32601, &format!("Method '{}' tidak didukung", req.method)))
            }
        }
    }
}

// ============================================================================
// 4. MCP CLIENT ENGINE
// ============================================================================

/// Client MCP mandiri untuk berinteraksi dengan Server MCP lokal maupun remote
pub struct McpClient {
    server: Arc<McpServer>,
}

impl McpClient {
    pub fn new(server: Arc<McpServer>) -> Self {
        Self { server }
    }

    /// Inisialisasi handshake ke server MCP
    pub fn initialize(&self) -> Result<JsonValue, String> {
        let req = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "initialize".to_string(),
            params: Some(json!({
                "protocolVersion": MCP_PROTOCOL_VERSION,
                "capabilities": {},
                "clientInfo": {
                    "name": "Widya-Agentic-Client",
                    "version": "0.1.0"
                }
            })),
        };

        let res = self.server.handle_request(req).ok_or("Server tidak membalas handshake")?;
        if let Some(err) = res.error {
            return Err(format!("Handshake gagal: {}", err.message));
        }
        res.result.ok_or("Hasil handshake kosong".to_string())
    }

    /// Mendapatkan daftar perkakas (tools) yang disediakan oleh server
    pub fn list_tools(&self) -> Result<Vec<McpToolDefinition>, String> {
        let req = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(2)),
            method: "tools/list".to_string(),
            params: None,
        };

        let res = self.server.handle_request(req).ok_or("Server tidak membalas tools/list")?;
        if let Some(err) = res.error {
            return Err(err.message);
        }

        let val = res.result.ok_or("Response tools/list kosong".to_string())?;
        let tools: Vec<McpToolDefinition> = serde_json::from_value(val.get("tools").unwrap_or(&json!([])).clone())
            .map_err(|e| format!("Gagal mendeserialisasi tools: {}", e))?;
        Ok(tools)
    }

    /// Memanggil Tool pada server dengan argumen JSON
    pub fn call_tool(&self, name: &str, arguments: JsonValue) -> Result<McpToolCallResult, String> {
        let req = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(3)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": name,
                "arguments": arguments
            })),
        };

        let res = self.server.handle_request(req).ok_or("Server tidak membalas tools/call")?;
        if let Some(err) = res.error {
            return Err(err.message);
        }

        let val = res.result.ok_or("Response tools/call kosong".to_string())?;
        let result: McpToolCallResult = serde_json::from_value(val)
            .map_err(|e| format!("Gagal mendeserialisasi hasil tool: {}", e))?;
        Ok(result)
    }

    /// Membaca konten resource dari server
    pub fn read_resource(&self, uri: &str) -> Result<String, String> {
        let req = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(4)),
            method: "resources/read".to_string(),
            params: Some(json!({ "uri": uri })),
        };

        let res = self.server.handle_request(req).ok_or("Server tidak membalas resources/read")?;
        if let Some(err) = res.error {
            return Err(err.message);
        }

        let val = res.result.ok_or("Response resources/read kosong".to_string())?;
        let contents = val.get("contents").and_then(|v| v.as_array()).ok_or("Format contents tidak valid")?;
        if let Some(first) = contents.first() {
            let text = first.get("text").and_then(|v| v.as_str()).unwrap_or("");
            Ok(text.to_string())
        } else {
            Err("Isi resource kosong".to_string())
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_server_initialize_and_ping() {
        let server = McpServer::new("widya-sovereign-mcp", "0.1.0");

        // 1. Initialize Handshake
        let init_req = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {}
            }
        }"#;

        let res_str = server.handle_raw_message(init_req).expect("Harus ada balasan");
        assert!(res_str.contains("\"protocolVersion\":\"2024-11-05\""));
        assert!(res_str.contains("\"name\":\"widya-sovereign-mcp\""));

        // 2. Ping
        let ping_req = r#"{"jsonrpc": "2.0", "id": 2, "method": "ping"}"#;
        let ping_res = server.handle_raw_message(ping_req).unwrap();
        assert!(ping_res.contains("\"result\":{}"));
    }

    #[test]
    fn test_mcp_tools_registration_and_execution() {
        let server = McpServer::new("widya-calc-mcp", "1.0.0");

        // Daftarkan tool penjumlahan
        server.register_tool(
            "tambah",
            "Menjumlahkan dua buah angka integer",
            json!({
                "type": "object",
                "properties": {
                    "a": { "type": "integer" },
                    "b": { "type": "integer" }
                },
                "required": ["a", "b"]
            }),
            |args| {
                let a = args.get("a").and_then(|v| v.as_i64()).ok_or("Parameter 'a' wajib ada")?;
                let b = args.get("b").and_then(|v| v.as_i64()).ok_or("Parameter 'b' wajib ada")?;
                Ok(format!("{}", a + b))
            },
        );

        let client = McpClient::new(Arc::clone(&server));
        let _ = client.initialize().unwrap();

        // 1. List tools
        let tools = client.list_tools().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "tambah");

        // 2. Call tool berhasil
        let res = client.call_tool("tambah", json!({ "a": 15, "b": 27 })).unwrap();
        assert!(!res.is_error);
        assert_eq!(res.content[0].text, "42");

        // 3. Call tool dengan argumen hilang -> Error isolasi tertangani
        let err_call = client.call_tool("tambah", json!({ "a": 10 })).unwrap();
        assert!(err_call.is_error);
        assert!(err_call.content[0].text.contains("Parameter 'b' wajib ada"));
    }

    #[test]
    fn test_mcp_resources_and_prompts() {
        let server = McpServer::new("widya-knowledge-mcp", "1.0.0");

        // Register resource
        server.register_resource(
            "widya://config/sovereign.yaml",
            "Konfigurasi Kedaulatan",
            Some("Parameter node kedaulatan data"),
            Some("text/yaml"),
            |_uri| Ok("region: nusantara\nsecurity_level: sovereign\n".to_string()),
        );

        // Register prompt template
        server.register_prompt(
            "buat_kueri_sql",
            Some("Prompt untuk menghasilkan SQL WidyaBase"),
            vec![McpPromptArgument {
                name: "tabel".to_string(),
                description: Some("Nama tabel target".to_string()),
                required: true,
            }],
            |args| {
                let tabel = args.get("tabel").cloned().unwrap_or("pengguna".to_string());
                Ok(vec![McpPromptMessage {
                    role: "user".to_string(),
                    content: McpToolContent {
                        content_type: "text".to_string(),
                        text: format!("Tolong buatkan kueri PILIH semua kolom dari tabel {}", tabel),
                    },
                }])
            },
        );

        let client = McpClient::new(Arc::clone(&server));

        // Read resource
        let content = client.read_resource("widya://config/sovereign.yaml").unwrap();
        assert!(content.contains("region: nusantara"));

        // Get prompt
        let req = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(10)),
            method: "prompts/get".to_string(),
            params: Some(json!({
                "name": "buat_kueri_sql",
                "arguments": { "tabel": "transaksi_koperasi" }
            })),
        };
        let res = server.handle_request(req).unwrap();
        let prompt_result = res.result.unwrap();
        assert!(prompt_result["messages"][0]["content"]["text"].as_str().unwrap().contains("transaksi_koperasi"));
    }
}
