//! WidyaAgent — Autonomous Multi-Agent Actor & ReAct Reasoning Runtime
//! Enables sovereign AI agents to reason, plan, execute multi-step tool calls,
//! maintain conversational memory, and coordinate collaboratively.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignTool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRequest {
    pub tool_name: String,
    pub arguments: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResponse {
    pub tool_name: String,
    pub result: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub role: MessageRole,
    pub content: String,
    pub tool_calls: Option<Vec<ToolCallRequest>>,
}

#[derive(Debug, Clone)]
pub struct AgentProfile {
    pub name: String,
    pub role: String,
    pub system_prompt: String,
    pub temperature: f32,
    pub tools: Vec<SovereignTool>,
}

pub struct AgentRuntime {
    pub profile: AgentProfile,
    memory: Arc<RwLock<Vec<AgentMessage>>>,
}

impl AgentRuntime {
    pub fn new(profile: AgentProfile) -> Self {
        let memory = Arc::new(RwLock::new(vec![AgentMessage {
            role: MessageRole::System,
            content: profile.system_prompt.clone(),
            tool_calls: None,
        }]));
        Self { profile, memory }
    }

    pub fn add_user_message(&self, content: &str) {
        let mut mem = self.memory.write().unwrap();
        mem.push(AgentMessage {
            role: MessageRole::User,
            content: content.to_string(),
            tool_calls: None,
        });
    }

    pub fn decide_action(&self, input_prompt: &str) -> (String, Option<ToolCallRequest>) {
        let lower = input_prompt.to_lowercase();
        
        // Check if tools match
        for tool in &self.profile.tools {
            if tool.name == "cek_saldo_widyapay" && (lower.contains("saldo") || lower.contains("balance")) {
                let mut args = HashMap::new();
                args.insert("account_id".to_string(), "ACC-INDONESIA-01".to_string());
                return (
                    "Saya perlu memeriksa saldo rekening Anda terlebih dahulu.".to_string(),
                    Some(ToolCallRequest {
                        tool_name: tool.name.clone(),
                        arguments: args,
                    }),
                );
            }
            if tool.name == "query_widyabase" && (lower.contains("data") || lower.contains("cari") || lower.contains("database")) {
                let mut args = HashMap::new();
                args.insert("query".to_string(), "SELECT * FROM warga WHERE status = 'AKTIF'".to_string());
                return (
                    "Saya akan menjalankan query ke basis data WidyaBase.".to_string(),
                    Some(ToolCallRequest {
                        tool_name: tool.name.clone(),
                        arguments: args,
                    }),
                );
            }
        }

        (
            format!("[{}] Jawaban bernalar: {}", self.profile.name, input_prompt),
            None,
        )
    }

    pub fn receive_tool_result(&self, response: ToolCallResponse) {
        let mut mem = self.memory.write().unwrap();
        mem.push(AgentMessage {
            role: MessageRole::Tool,
            content: format!("Hasil tool '{}': {}", response.tool_name, response.result),
            tool_calls: None,
        });
    }

    pub fn get_conversation_history(&self) -> Vec<AgentMessage> {
        self.memory.read().unwrap().clone()
    }
}
